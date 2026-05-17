// resource.rs — Resource type definitions for generic CLI dispatch
//
// Each Glow resource maps to a URL slug: POST /v5/{slug}/{action}
// The macro generates the enum, slug mapping, and help text.

/// Generate a Resource enum with slug and description mappings.
macro_rules! define_resources {
    ( $( ($variant:ident, $slug:expr, $about:expr) ),* $(,)? ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Resource {
            $( $variant, )*
        }

        impl Resource {
            pub fn slug(&self) -> &'static str {
                match self {
                    $( Resource::$variant => $slug, )*
                }
            }

            pub fn from_slug(s: &str) -> Option<Self> {
                match s {
                    $( $slug => Some(Resource::$variant), )*
                    _ => None,
                }
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
                vec![ $( $slug, )* ]
            }
        }

        impl std::fmt::Display for Resource {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.slug())
            }
        }
    };
}

define_resources! {
    // Core content
    (Personas,     "persona",      "AI personas for simulation"),
    (Scenarios,    "scenarios",    "Simulation scenarios"),
    (Simulations,  "simulations",  "Simulation configurations"),
    (Documents,    "documents",    "Document management"),
    (Agents,       "agents",       "AI agents"),
    (Models,       "models",       "AI model configurations"),
    (Providers,    "providers",    "AI provider integrations"),
    (Parameters,   "parameters",   "Configuration parameters"),
    (Fields,       "fields",       "Custom fields"),
    (Tools,        "tools",        "Tool integrations"),

    // Organization
    (Departments,  "departments",  "Organizational departments"),
    (Cohorts,      "cohorts",      "User cohorts"),

    // Evaluation
    (Evals,        "evals",        "Evaluation configurations"),
    (Rubrics,      "rubrics",      "Grading rubrics"),
    (Benchmarks,   "benchmarks",   "Performance benchmarks"),

    // User management
    (Profiles,     "profiles",     "User profiles"),
    (Auths,        "auths",        "Authentication records"),
    (Settings,     "settings",     "Instance settings"),

    // Session/interactive
    (Attempt,      "attempt",      "Simulation attempts"),
    (Test,         "test",         "Test sessions"),
    (Chat,         "chat",         "Chat sessions"),

    // Read-only / view
    (Activity,     "activity",     "Activity logs"),
    (Record,       "record",       "Records"),
    (Dashboard,    "dashboard",    "Dashboard data"),
    (Leaderboard,  "leaderboard",  "Leaderboard data"),
    (Reports,      "reports",      "Report generation"),
    (Home,         "home",         "Home page data"),
    (Practice,     "practice",     "Practice sessions"),
    (Group,        "group",        "Group management"),
    (Pricing,      "pricing",      "Pricing information"),
    (Session,      "session",      "Session management"),
    (Invocation,   "invocation",   "AI invocation records"),
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
        msg.push_str(&format!("  {:15} {}\n", r.slug(), r.about()));
    }
    msg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_slug_roundtrip() {
        for r in Resource::all() {
            let slug = r.slug();
            let parsed = Resource::from_slug(slug).unwrap();
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
        assert!(Resource::all().len() >= 30); // we have 32 resources
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Resource::Personas), "persona");
        assert_eq!(format!("{}", Resource::Attempt), "attempt");
    }

    #[test]
    fn test_about_not_empty() {
        for r in Resource::all() {
            assert!(
                !r.about().is_empty(),
                "Resource {} has empty about",
                r.slug()
            );
        }
    }

    #[test]
    fn test_unknown_resource_error_message() {
        let msg = unknown_resource_error("foo");
        assert!(msg.contains("Unknown resource 'foo'"));
        assert!(msg.contains("persona"));
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
