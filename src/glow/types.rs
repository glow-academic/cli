// glow/types.rs — Re-exports from auto-generated types + hand-written additions
//
// types_gen.rs is auto-generated from the glow-api OpenAPI spec.
// Add aliases or custom types here.

// ``PINNED_API_VERSION`` removed in Cleanup E — only check_api_version
// (also removed) read it. Canonical pinned version lives in
// ``api-versions.json`` (rewritten by ``make sync-types``).

#[allow(unused_imports)]
pub use super::api::latest::*;

// ``HealthResponse`` removed in Cleanup E along with cmd_health /
// GlowClient::health. The OpenAPI-generated ``HealthResponse`` in
// api/latest.rs covers the real /system/health artifact when needed.
