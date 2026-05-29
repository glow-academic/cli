//! Local backup CRUD — list / create / restore / delete.
//!
//! Backups live under `~/.glow/instances/<name>/backups/`. Create =
//! `pg_dump` inside the DB container piped to gzip; restore = stop the
//! servers, drop+recreate the DB, gunzip + psql in, restart the servers.
//!
//! v1.0.0 keeps this dead simple — no encryption, no remote upload,
//! no scheduling. The user owns the files and can rsync them anywhere.

use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::deploy::instance::Instance;
use crate::deploy::runtime;

#[derive(Debug)]
pub struct BackupEntry {
    pub name: String,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub mtime: SystemTime,
}

pub fn list(instance_name: &str) -> Result<Vec<BackupEntry>> {
    let i = Instance::open(instance_name)?;
    let dir = i.backups_dir();
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for e in fs::read_dir(&dir)? {
        let e = e?;
        let meta = e.metadata()?;
        if !meta.is_file() {
            continue;
        }
        let name = e.file_name().to_string_lossy().to_string();
        if !name.ends_with(".sql.gz") {
            continue;
        }
        out.push(BackupEntry {
            name,
            path: e.path(),
            size_bytes: meta.len(),
            mtime: meta.modified()?,
        });
    }
    // Most recent first.
    out.sort_by_key(|b| std::cmp::Reverse(b.mtime));
    Ok(out)
}

/// Create a fresh backup tagged with a user-supplied label (or "manual"
/// by default). Filename: `manual-<label>-<timestamp>.sql.gz`.
pub fn create(instance_name: &str, label: Option<&str>) -> Result<PathBuf> {
    runtime::assert_docker_available()?;
    let i = Instance::open(instance_name)?;
    // The database lives in the API stack — project `glow-<name>-api`,
    // compose under `<instance>/api/` — not the bare `glow-<name>` / instance
    // root. Use the api stack's dir + project name or compose can't find the
    // `database` service ("service database is not running").
    let project_name = i.api_project_name();
    let api_dir = i.api_dir();
    let label = label.unwrap_or("manual").replace(['/', ' ', ':'], "-");
    let ts = compact_now();
    let filename = format!("manual-{label}-{ts}.sql.gz");
    let target = i.backups_dir().join(&filename);
    fs::create_dir_all(i.backups_dir()).ok();

    println!("· dumping database (pg_dump | gzip) — this may take a minute");
    let dump = runtime::exec_capture(
        &api_dir,
        &project_name,
        "database",
        &[
            "sh",
            "-c",
            "pg_dump --exclude-schema=keycloak -U $POSTGRES_USER $POSTGRES_DB | gzip",
        ],
    )?;
    fs::write(&target, dump).with_context(|| format!("write backup: {}", target.display()))?;

    println!("✓ backup written: {}", target.display());
    Ok(target)
}

pub fn delete(instance_name: &str, name: &str) -> Result<()> {
    let i = Instance::open(instance_name)?;
    let target = i.backups_dir().join(name);
    if !target.exists() {
        return Err(anyhow!("backup not found: {}", target.display()));
    }
    fs::remove_file(&target).with_context(|| format!("delete {}", target.display()))?;
    println!("✓ deleted {}", target.display());
    Ok(())
}

/// Restore a backup — destructive. Drops the DB, recreates it, streams
/// the gzipped dump in via psql.
///
/// The restored container will need a server restart on top to clear
/// its asyncpg pool; we trigger that by stopping + starting whichever
/// color is currently active (per the state file).
pub fn restore(instance_name: &str, name: &str) -> Result<()> {
    runtime::assert_docker_available()?;
    let i = Instance::open(instance_name)?;
    // API-stack project + dir (see create() — the database is in glow-<name>-api).
    let project_name = i.api_project_name();
    let api_dir = i.api_dir();
    let source = i.backups_dir().join(name);
    if !source.exists() {
        return Err(anyhow!(
            "backup not found: {} (run `glow backup list` to see options)",
            source.display()
        ));
    }

    println!("· copying backup into database container");
    // Two-step: cp into container, then exec the restore inside. We
    // can't pipe gigabytes through `docker exec -i` cleanly across
    // backup sizes that vary by 1000x, so the cp path is safer.
    let in_container_path = format!("/tmp/restore-{}", name);
    let cp_status = std::process::Command::new("docker")
        .args([
            "cp",
            source.to_str().unwrap(),
            &format!("{project_name}-database-1:{in_container_path}"),
        ])
        .status()?;
    if !cp_status.success() {
        return Err(anyhow!("docker cp failed (exit {:?})", cp_status.code()));
    }

    println!("· dropping + recreating database");
    // The API servers keep live pools open against the target DB, so a bare
    // DROP DATABASE fails with "is being accessed by other users". Terminate
    // those sessions first, then DROP ... WITH (FORCE) (Postgres 13+) so any
    // racing reconnect is severed as part of the drop.
    runtime::exec_capture(
        &api_dir,
        &project_name,
        "database",
        &[
            "sh",
            "-c",
            "psql -U $POSTGRES_USER -d postgres -c \
               \"SELECT pg_terminate_backend(pid) FROM pg_stat_activity \
                 WHERE datname = '$POSTGRES_DB' AND pid <> pg_backend_pid();\" \
             && psql -U $POSTGRES_USER -d postgres -c \"DROP DATABASE IF EXISTS $POSTGRES_DB WITH (FORCE);\" \
             && psql -U $POSTGRES_USER -d postgres -c \"CREATE DATABASE $POSTGRES_DB;\"",
        ],
    )?;

    println!("· streaming backup into database");
    runtime::exec_capture(
        &api_dir,
        &project_name,
        "database",
        &[
            "sh",
            "-c",
            &format!("gunzip -c {in_container_path} | psql -U $POSTGRES_USER -d $POSTGRES_DB"),
        ],
    )?;
    // Cleanup the in-container file.
    let _ = runtime::exec_capture(
        &api_dir,
        &project_name,
        "database",
        &["rm", "-f", &in_container_path],
    );

    println!("· restarting servers to clear connection pools");
    // Best-effort restart of both colors so whichever is live gets a
    // fresh pool. compose restart is no-op for not-running services.
    let _ = runtime::up(&api_dir, &project_name, &["server-blue", "server-green"]);

    println!("✓ restored from {}", source.display());
    Ok(())
}

fn compact_now() -> String {
    let n = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{n}")
}
