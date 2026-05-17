//! Schema-migration trigger.
//!
//! The migration logic itself lives in the api image — `/app/database/`
//! has `migrate/add/v{version}_{N}_{name}.sql` and `migrate/remove/...`
//! plus a runner (`make migrate-docker TYPE=add|remove`). The CLI's job
//! is just to invoke that runner inside the running container at the
//! right moments in the deploy timeline:
//!
//!   - **add migrations**: BEFORE we promote the new color (so the new
//!     server boots against a schema it's compatible with).
//!   - **remove migrations**: AFTER the new color is stable (so destructive
//!     drops don't break the old color while it's still serving).
//!
//! Both are idempotent — the runner records applied migrations in
//! `migrations.applied` and skips ones that already ran.

use anyhow::Result;
use std::path::Path;

use crate::deploy::runtime;

/// Run pre-deploy additive migrations against the currently-running DB.
/// Safe to call multiple times; runner skips applied entries.
pub fn run_add(project_dir: &Path, project_name: &str, server_service: &str) -> Result<()> {
    println!("  · applying add migrations (idempotent)");
    // The api image has `make migrate-docker TYPE=add` wired to the
    // python runner; running through make gives us a stable interface
    // even as the runner internals evolve.
    runtime::exec_capture(
        project_dir,
        project_name,
        server_service,
        &["make", "migrate-docker", "TYPE=add"],
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
        &["make", "migrate-docker", "TYPE=remove"],
    )?;
    Ok(())
}
