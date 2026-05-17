//! Compose-file materialization.
//!
//! Bundles a known-good docker-compose.yml at build time and writes it
//! verbatim to the instance dir on deploy. The template uses `${VAR}`
//! interpolation that compose itself resolves from the instance's `.env`
//! — so we don't do any string templating here, we just embed + write.
//!
//! Two stacks: `api` and `client`. Each lives in its own subdirectory
//! of the instance dir (`<instance>/api/` and `<instance>/client/`) so
//! they have independent `.env` + compose state and compose's
//! `COMPOSE_PROJECT_NAME` keeps the container names from colliding.
//!
//! The client stack also needs its nginx config + docker-gen upstream
//! template + initial seed file — those ship as separate templates.
//!
//! Upgrading the compose shape = cutting a new CLI release. There's no
//! runtime config to mutate; the user gets the bundled template
//! whenever the CLI is upgraded.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

const API_COMPOSE: &str = include_str!("templates/api-compose.yml");
const CLIENT_COMPOSE: &str = include_str!("templates/client-compose.yml");
const CLIENT_NGINX_CONF: &str = include_str!("templates/client-nginx.conf.template");
const CLIENT_UPSTREAMS_TMPL: &str = include_str!("templates/client-upstreams.tmpl");
const CLIENT_UPSTREAMS_SEED: &str = include_str!("templates/client-upstreams.conf");

/// Write the api stack files into `<instance>/api/`. Just the compose
/// file for now — the api stack's nginx config is baked into the api
/// image, not mounted from the host.
pub fn write_api_stack(instance_dir: &Path) -> Result<()> {
    let dir = instance_dir.join("api");
    fs::create_dir_all(&dir).with_context(|| format!("mkdir {}", dir.display()))?;
    fs::write(dir.join("docker-compose.yml"), API_COMPOSE)
        .with_context(|| format!("write api compose to {}", dir.display()))?;
    Ok(())
}

/// Write the client stack files into `<instance>/client/`:
///   - docker-compose.yml
///   - default.conf.template  (mounted into the client's nginx)
///   - upstreams.tmpl         (consumed by docker-gen)
///   - upstreams.conf         (seed; docker-gen overwrites at runtime)
pub fn write_client_stack(instance_dir: &Path) -> Result<()> {
    let dir = instance_dir.join("client");
    fs::create_dir_all(&dir).with_context(|| format!("mkdir {}", dir.display()))?;
    fs::write(dir.join("docker-compose.yml"), CLIENT_COMPOSE)
        .with_context(|| format!("write client compose to {}", dir.display()))?;
    fs::write(dir.join("default.conf.template"), CLIENT_NGINX_CONF)
        .with_context(|| "write client nginx config")?;
    fs::write(dir.join("upstreams.tmpl"), CLIENT_UPSTREAMS_TMPL)
        .with_context(|| "write client upstreams template")?;
    fs::write(dir.join("upstreams.conf"), CLIENT_UPSTREAMS_SEED)
        .with_context(|| "write client upstreams seed")?;
    Ok(())
}
