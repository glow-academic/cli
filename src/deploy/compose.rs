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

use crate::deploy::config::Topology;

const API_COMPOSE: &str = include_str!("templates/api-compose.yml");
const API_NGINX_CONF: &str = include_str!("templates/api-nginx.conf.template");
const API_UPSTREAMS_TMPL: &str = include_str!("templates/api-upstreams.tmpl");
const API_UPSTREAMS_SEED: &str = include_str!("templates/api-upstreams.conf");
const CLIENT_COMPOSE: &str = include_str!("templates/client-compose.yml");
const CLIENT_NGINX_CONF: &str = include_str!("templates/client-nginx.conf.template");
const CLIENT_UPSTREAMS_TMPL: &str = include_str!("templates/client-upstreams.tmpl");
const CLIENT_UPSTREAMS_SEED: &str = include_str!("templates/client-upstreams.conf");

/// Write the api stack files into `<instance>/api/`:
///   - docker-compose.yml
///   - default.conf.template  (mounted into the api's nginx)
///   - upstreams.tmpl         (consumed by docker-gen)
///   - upstreams.conf         (seed; docker-gen overwrites at runtime)
///   - docker-compose.override.yml  (assembled from fragments — see
///     `assemble_api_override`. Written only when at least one fragment
///     applies, else any stale override is removed.)
pub fn write_api_stack(instance_dir: &Path, topology: Topology) -> Result<()> {
    let dir = instance_dir.join("api");
    fs::create_dir_all(&dir).with_context(|| format!("mkdir {}", dir.display()))?;
    // docker-gen writes generated/upstreams.conf via rename, which
    // requires a directory mount (not a single-file mount).
    let generated = dir.join("generated");
    fs::create_dir_all(&generated).with_context(|| format!("mkdir {}", generated.display()))?;
    fs::write(dir.join("docker-compose.yml"), API_COMPOSE)
        .with_context(|| format!("write api compose to {}", dir.display()))?;
    fs::write(dir.join("default.conf.template"), API_NGINX_CONF)
        .with_context(|| "write api nginx config")?;
    fs::write(dir.join("upstreams.tmpl"), API_UPSTREAMS_TMPL)
        .with_context(|| "write api upstreams template")?;
    fs::write(generated.join("upstreams.conf"), API_UPSTREAMS_SEED)
        .with_context(|| "write api upstreams seed")?;

    // The seed loader (database/seeds/config.py) prefers a gitignored
    // `glow-deploy.local.yaml` over the committed `glow-deploy.yaml`.
    // The base compose only mounts `glow-deploy.yaml`; if the operator
    // dropped a `glow-deploy.local.yaml` in the instance dir we must
    // bind it in too, or the loader never sees it. We can't add the
    // mount unconditionally in the base template: a bind of a missing
    // host path materializes an empty dir inside the container, and the
    // loader would then `yaml.safe_load` a directory and crash.
    let has_local_overlay = instance_dir.join("glow-deploy.local.yaml").exists();

    let override_path = dir.join("docker-compose.override.yml");
    match assemble_api_override(topology, has_local_overlay) {
        Some(body) => fs::write(&override_path, body).with_context(|| {
            format!("write api compose override to {}", override_path.display())
        })?,
        // No fragment applies — make sure no stale override lingers.
        None => {
            let _ = fs::remove_file(&override_path);
        }
    }
    Ok(())
}

/// Assemble the api stack's `docker-compose.override.yml` body from the
/// fragments that apply, or `None` when none do.
///
/// Compose loads exactly ONE override file and deep-merges it onto the
/// base, so both concerns (the public-port binding and the
/// `glow-deploy.local.yaml` mount) must live in the same file. List-valued
/// keys like `ports` / `volumes` are appended to the base service's lists,
/// so the local-overlay mount stacks on top of the base
/// `glow-deploy.yaml` mount rather than replacing it.
///
/// Fragments:
///   - public api port — for `api_only` / `exposed` topologies (the api
///     is publicly reachable; airgapped stays purely internal).
///   - local-overlay mount — when a `glow-deploy.local.yaml` exists in the
///     instance dir, bind it read-only into db-init at
///     `/app/glow-deploy.local.yaml` (the path the seed loader prefers).
///     Durable across `--reseed`: the file lives in the instance dir, the
///     mount is re-emitted on every deploy.
fn assemble_api_override(topology: Topology, has_local_overlay: bool) -> Option<String> {
    let mut services = String::new();

    if matches!(topology, Topology::ApiOnly | Topology::Exposed) {
        services.push_str(
            "  nginx:\n    \
                 ports:\n      \
                   - \"${API_HTTP_PORT:-127.0.0.1:18081}:80\"\n",
        );
    }

    if has_local_overlay {
        // Path is relative to the api stack dir (`<instance>/api/`), so
        // `..` reaches the instance dir — mirrors the base mount of
        // `../glow-deploy.yaml`.
        services.push_str(
            "  db-init:\n    \
                 volumes:\n      \
                   - ../glow-deploy.local.yaml:/app/glow-deploy.local.yaml:ro\n",
        );
    }

    if services.is_empty() {
        return None;
    }
    Some(format!(
        "# Auto-generated by glow-cli — do not edit.\nservices:\n{services}"
    ))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn override_none_for_airgapped_without_overlay() {
        assert!(assemble_api_override(Topology::Airgapped, false).is_none());
    }

    #[test]
    fn override_has_port_for_exposed_without_overlay() {
        let body = assemble_api_override(Topology::Exposed, false).unwrap();
        assert!(body.contains("nginx:"));
        assert!(body.contains("API_HTTP_PORT"));
        assert!(!body.contains("glow-deploy.local.yaml"));
    }

    #[test]
    fn override_mounts_local_overlay_into_db_init() {
        let body = assemble_api_override(Topology::Airgapped, true).unwrap();
        assert!(body.contains("db-init:"));
        assert!(body.contains(
            "../glow-deploy.local.yaml:/app/glow-deploy.local.yaml:ro"
        ));
        // Airgapped → no public port fragment.
        assert!(!body.contains("API_HTTP_PORT"));
    }

    #[test]
    fn override_combines_port_and_overlay() {
        let body = assemble_api_override(Topology::ApiOnly, true).unwrap();
        assert!(body.contains("nginx:"));
        assert!(body.contains("API_HTTP_PORT"));
        assert!(body.contains("db-init:"));
        assert!(body.contains("glow-deploy.local.yaml"));
        // Single top-level `services:` key — both fragments nest under it.
        assert_eq!(body.matches("\nservices:\n").count(), 1);
    }
}
