// resource.rs — Resource type definitions for generic CLI dispatch
//
// Each Glow resource has two surface forms:
//   * ``cli_name`` — what the user types (always plural — ``glow personas
//     search``, ``glow scenarios get``).
//   * ``api_path`` — what the CLI sends on the wire. The frozen Glow API
//     exposes routes at ``POST /{artifact}/{operation}`` where artifact
//     is singular (``/persona/search``, ``/scenario/get``). So the CLI
//     maps plural→singular at dispatch time.
//
// View artifacts (dashboard, home, leaderboard, practice, reports,
// activity, health, sessions, groups, pricing) and nested-op artifacts
// (chat, benchmark, invocation) live *inside* a parent artifact's URL
// space (``/attempt/dashboard``, ``/system/health``, ``/test/benchmark``).
// They're intentionally NOT in this macro — they'll surface as
// subcommands of their parent artifact in Phase 4 (e.g. ``glow attempts
// dashboard``, ``glow system health``, ``glow tests benchmark``).

/// Generate a Resource enum with cli_name → api_path → description mappings.
macro_rules! define_resources {
    ( $( ($variant:ident, $cli_name:expr, $api_path:expr, $about:expr) ),* $(,)? ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Resource {
            $( $variant, )*
        }

        impl Resource {
            /// The plural user-facing name (what the user types).
            pub fn cli_name(&self) -> &'static str {
                match self {
                    $( Resource::$variant => $cli_name, )*
                }
            }

            /// The singular API path segment (what the CLI sends on the wire).
            pub fn api_path(&self) -> &'static str {
                match self {
                    $( Resource::$variant => $api_path, )*
                }
            }

            /// Backwards-compatible alias for ``cli_name`` — kept so the
            /// ``Display`` impl + a few internal callers don't churn.
            /// New code should prefer ``cli_name`` / ``api_path`` explicitly.
            #[allow(dead_code)]
            pub fn slug(&self) -> &'static str {
                self.cli_name()
            }

            /// Resolve by the user-facing name (what shows up on argv).
            pub fn from_cli_name(s: &str) -> Option<Self> {
                match s {
                    $( $cli_name => Some(Resource::$variant), )*
                    _ => None,
                }
            }

            /// Backwards-compatible alias for ``from_cli_name`` — kept so
            /// existing callers (dispatch.rs, completions) don't churn.
            pub fn from_slug(s: &str) -> Option<Self> {
                Self::from_cli_name(s)
            }

            pub fn about(&self) -> &'static str {
                match self {
                    $( Resource::$variant => $about, )*
                }
            }

            pub fn all() -> &'static [Resource] {
                &[ $( Resource::$variant, )* ]
            }

            #[allow(dead_code)]
            pub fn all_slugs() -> Vec<&'static str> {
                vec![ $( $cli_name, )* ]
            }
        }

        impl std::fmt::Display for Resource {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.cli_name())
            }
        }
    };
}

define_resources! {
    // ── Core content ──────────────────────────────────────────
    (Personas,     "personas",     "persona",     "AI personas for simulation"),
    (Scenarios,    "scenarios",    "scenario",    "Simulation scenarios"),
    (Simulations,  "simulations",  "simulation",  "Simulation configurations"),
    (Documents,    "documents",    "document",    "Document management"),
    (Agents,       "agents",       "agent",       "AI agents"),
    (Models,       "models",       "model",       "AI model configurations"),
    (Providers,    "providers",    "provider",    "AI provider integrations"),
    (Parameters,   "parameters",   "parameter",   "Configuration parameters"),
    (Fields,       "fields",       "field",       "Custom fields"),
    (Tools,        "tools",        "tool",        "Tool integrations"),

    // ── Organization ──────────────────────────────────────────
    (Departments,  "departments",  "department",  "Organizational departments"),
    (Cohorts,      "cohorts",      "cohort",      "User cohorts"),

    // ── Evaluation ────────────────────────────────────────────
    (Evals,        "evals",        "eval",        "Evaluation configurations"),
    (Rubrics,      "rubrics",      "rubric",      "Grading rubrics"),

    // ── User management ───────────────────────────────────────
    (Profiles,     "profiles",     "profile",     "User profiles"),
    (Auths,        "auths",        "auth",        "Authentication records"),
    (Settings,     "settings",     "setting",     "Instance settings"),

    // ── Session / interactive (top-level CRUD on the API) ─────
    (Attempts,     "attempts",     "attempt",     "Simulation attempts"),
    (Tests,        "tests",        "test",        "Test sessions"),

    // ── System namespace ──────────────────────────────────────
    // Not a CRUD artifact — system/<op> is a flat 1-level-per-op
    // surface (POST /system/activity, /system/health, /system/sessions,
    // /system/groups, /system/pricing, /system/{audio,image,file,
    // video,text,call}_download, /system/file_preview, /system/watch,
    // /system/export, /system/refresh, /system/resolve, /system/title,
    // /system/generations, /system/context, /system/generate,
    // /system/problem). Adding it as a Resource lets all 17 system
    // ops reach the generic dispatch — ``glow system activity`` etc.
    // (Pre-cleanup B these were a scattered set of top-level Clap
    // commands and a Groups subcommand group; consolidated here.)
    (System,       "system",       "system",      "System views + cross-artifact ops (activity, health, sessions, groups, pricing, downloads, ...)"),
}

// ── Media types ──────────────────────────────────────────────

/// Media types that can be nested under any resource
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    File,
    Image,
    Text,
    Audio,
    Video,
}

impl MediaType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "file" => Some(MediaType::File),
            "image" => Some(MediaType::Image),
            "text" => Some(MediaType::Text),
            "audio" => Some(MediaType::Audio),
            "video" => Some(MediaType::Video),
            _ => None,
        }
    }

    pub fn slug(&self) -> &'static str {
        match self {
            MediaType::File => "file",
            MediaType::Image => "image",
            MediaType::Text => "text",
            MediaType::Audio => "audio",
            MediaType::Video => "video",
        }
    }

    #[allow(dead_code)]
    pub fn all_slugs() -> &'static [&'static str] {
        &["file", "image", "text", "audio", "video"]
    }
}

/// Format a helpful error message listing all valid resources
pub fn unknown_resource_error(given: &str) -> String {
    let mut msg = format!("Unknown resource '{}'. Available resources:\n", given);
    for r in Resource::all() {
        msg.push_str(&format!("  {:15} {}\n", r.cli_name(), r.about()));
    }
    msg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cli_name_roundtrip() {
        for r in Resource::all() {
            let name = r.cli_name();
            let parsed = Resource::from_cli_name(name).unwrap();
            assert_eq!(*r, parsed);
        }
    }

    #[test]
    fn test_from_slug_roundtrip_backcompat() {
        for r in Resource::all() {
            let parsed = Resource::from_slug(r.slug()).unwrap();
            assert_eq!(*r, parsed);
        }
    }

    #[test]
    fn test_from_slug_unknown() {
        assert!(Resource::from_slug("nonexistent").is_none());
    }

    #[test]
    fn test_all_slugs_count() {
        assert_eq!(Resource::all().len(), Resource::all_slugs().len());
        // 19 CRUD resources + System namespace = 20 after Cleanup B.
        assert!(Resource::all().len() >= 20);
    }

    #[test]
    fn test_display_uses_plural_cli_name() {
        assert_eq!(format!("{}", Resource::Personas), "personas");
        assert_eq!(format!("{}", Resource::Attempts), "attempts");
        assert_eq!(format!("{}", Resource::Tests), "tests");
    }

    #[test]
    fn test_cli_to_api_mapping() {
        // The whole point of Phase 1: plural CLI → singular API path.
        assert_eq!(Resource::Personas.cli_name(), "personas");
        assert_eq!(Resource::Personas.api_path(), "persona");
        assert_eq!(Resource::Scenarios.api_path(), "scenario");
        assert_eq!(Resource::Attempts.api_path(), "attempt");
        assert_eq!(Resource::Tests.api_path(), "test");
    }

    #[test]
    fn test_about_not_empty() {
        for r in Resource::all() {
            assert!(
                !r.about().is_empty(),
                "Resource {} has empty about",
                r.cli_name()
            );
        }
    }

    #[test]
    fn test_unknown_resource_error_message() {
        let msg = unknown_resource_error("foo");
        assert!(msg.contains("Unknown resource 'foo'"));
        assert!(msg.contains("personas"));
        assert!(msg.contains("scenarios"));
    }

    #[test]
    fn test_media_type_from_str() {
        assert_eq!(MediaType::from_str("file"), Some(MediaType::File));
        assert_eq!(MediaType::from_str("image"), Some(MediaType::Image));
        assert_eq!(MediaType::from_str("text"), Some(MediaType::Text));
        assert_eq!(MediaType::from_str("audio"), Some(MediaType::Audio));
        assert_eq!(MediaType::from_str("video"), Some(MediaType::Video));
        assert_eq!(MediaType::from_str("unknown"), None);
    }

    #[test]
    fn test_media_type_slug_roundtrip() {
        for slug in MediaType::all_slugs() {
            let mt = MediaType::from_str(slug).unwrap();
            assert_eq!(mt.slug(), *slug);
        }
    }
}
