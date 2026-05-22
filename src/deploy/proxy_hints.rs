//! Print reverse-proxy configuration snippets after a successful deploy.
//!
//! The CLI publishes exactly one host port (the client nginx in airgapped /
//! exposed, the api nginx in api_only). Anything in front of that port —
//! TLS termination, the public hostname, redirects — is the user's existing
//! reverse proxy's job. We don't try to manage it; we just print the
//! one-liner they need to add.
//!
//! Convergent pattern in self-hosted land (Plausible / Vaultwarden /
//! Mattermost / Outline): publish one port, document the proxy mapping,
//! stay out of the user's network config.
//!
//! `public_url` is the URL the user typed in `glow-deploy.yaml`
//! (`https://glow.example.com`) — the proxy frontend.
//! `published` is the host port spec from the env (`127.0.0.1:18080`,
//! `0.0.0.0:18080`, or just `18080`). We pass it through verbatim so
//! snippets reflect exactly what got bound.

use std::fmt::Write as _;

/// Print Caddy / Traefik (file provider) / nginx snippets to stdout.
/// `public_url` may be empty when topology is api_only and api_origin
/// isn't set — in that case we still print the snippets with a
/// placeholder so the user has the shape.
pub fn print_proxy_hints(public_url: &str, published: &str) {
    let host = host_from_url(public_url);
    let upstream = upstream_url(published);
    let mut out = String::new();
    let _ = writeln!(
        out,
        "\nReverse-proxy snippet (point your existing proxy at the stack):\n\
         \n\
         ── Caddyfile ────────────────────────────────────────────────\n\
         {host} {{\n    \
             reverse_proxy {upstream}\n\
         }}\n\
         \n\
         ── Traefik (file provider, e.g. /etc/traefik/dynamic/glow.yaml) ─\n\
         http:\n  \
           routers:\n    \
             glow:\n      \
               rule: \"Host(`{host}`)\"\n      \
               service: glow\n      \
               entryPoints: [websecure]\n      \
               tls:\n        \
                 certResolver: letsencrypt\n  \
           services:\n    \
             glow:\n      \
               loadBalancer:\n        \
                 servers:\n          \
                   - url: \"http://{upstream}\"\n\
         \n\
         ── nginx ────────────────────────────────────────────────────\n\
         server {{\n    \
             listen 443 ssl;\n    \
             server_name {host};\n    \
             # ssl_certificate ...;\n    \
             # ssl_certificate_key ...;\n    \
             location / {{\n        \
                 proxy_pass http://{upstream};\n        \
                 proxy_set_header Host $host;\n        \
                 proxy_set_header X-Forwarded-Proto $scheme;\n        \
                 proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;\n        \
                 proxy_http_version 1.1;\n        \
                 proxy_set_header Upgrade $http_upgrade;\n        \
                 proxy_set_header Connection \"upgrade\";\n    \
             }}\n\
         }}\n"
    );
    print!("{out}");
}

/// Extract host[:port] from a URL like `https://glow.example.com`.
/// Falls back to the input or a placeholder.
fn host_from_url(url: &str) -> String {
    if url.is_empty() {
        return "<your-domain.example.com>".into();
    }
    let after_scheme = url.split_once("://").map(|(_, rest)| rest).unwrap_or(url);
    after_scheme
        .trim_end_matches('/')
        .split('/')
        .next()
        .unwrap_or(after_scheme)
        .to_string()
}

/// Turn a `published` spec (`127.0.0.1:18080`, `0.0.0.0:18080`, `18080`)
/// into an `host:port` form suitable for a proxy upstream URL.
/// Bare-port input gets `127.0.0.1` prepended — same-host proxy by default.
fn upstream_url(published: &str) -> String {
    if published.contains(':') {
        published.to_string()
    } else {
        format!("127.0.0.1:{published}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_strips_scheme_and_path() {
        assert_eq!(
            host_from_url("https://glow.example.com"),
            "glow.example.com"
        );
        assert_eq!(
            host_from_url("https://glow.example.com/"),
            "glow.example.com"
        );
        assert_eq!(host_from_url("http://localhost:7070"), "localhost:7070");
    }

    #[test]
    fn host_handles_empty() {
        assert_eq!(host_from_url(""), "<your-domain.example.com>");
    }

    #[test]
    fn upstream_keeps_ip_prefix() {
        assert_eq!(upstream_url("127.0.0.1:18080"), "127.0.0.1:18080");
        assert_eq!(upstream_url("0.0.0.0:18080"), "0.0.0.0:18080");
    }

    #[test]
    fn upstream_adds_localhost_to_bare_port() {
        assert_eq!(upstream_url("18080"), "127.0.0.1:18080");
    }
}
