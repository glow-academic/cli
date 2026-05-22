//! Schema-migration trigger.
//!
//! The migration logic itself lives in the api image — `/app/database/`
//! has `migrate/add/v{version}_{N}_{name}.sql` and `migrate/remove/...`
//! plus an in-image python runner (`database.scripts.migrate`). The CLI's
//! job is just to invoke that runner inside the running container at the
//! right moments in the deploy timeline:
//!
//!   - **add migrations**: BEFORE we promote the new color (so the new
//!     server boots against a schema it's compatible with).
//!   - **remove migrations**: AFTER the new color is stable (so destructive
//!     drops don't break the old color while it's still serving).
//!
//! Both are idempotent — the runner records applied migrations in
//! `migrations.applied` and skips ones that already ran.
//!
//! We used to invoke `make migrate-docker TYPE=add|remove` here, but the
//! runtime image has no `make` and the host script the Makefile target
//! delegated to was a docker-compose-orchestrator (wrong shape for an
//! exec-into-container call). The python module replaces both, using the
//! same asyncpg connection the api itself uses.

use anyhow::Result;
use std::path::Path;

use crate::deploy::runtime;

/// Run pre-deploy additive migrations against the currently-running DB.
/// Safe to call multiple times; runner skips applied entries.
pub fn run_add(project_dir: &Path, project_name: &str, server_service: &str) -> Result<()> {
    println!("  · applying add migrations (idempotent)");
    runtime::exec_capture(
        project_dir,
        project_name,
        server_service,
        &["python", "-m", "database.scripts.migrate", "add"],
    )?;
    Ok(())
}

/// Run post-promotion destructive migrations. Caller invokes ONLY after
/// the new color has cleared its grace window.
pub fn run_remove(project_dir: &Path, project_name: &str, server_service: &str) -> Result<()> {
    println!("  · applying remove migrations (post-stable, destructive)");
    runtime::exec_capture(
        project_dir,
        project_name,
        server_service,
        &["python", "-m", "database.scripts.migrate", "remove"],
    )?;
    Ok(())
}
