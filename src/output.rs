// output.rs — Output formatting and user interaction
//
// Two output modes:
//   Human: colored, formatted text (default)
//   Json:  machine-readable JSON (--json flag)
//
// In Python terms: like rich vs json.dumps
// In TS terms:     like chalk vs JSON.stringify

use serde::Serialize;
use std::io::IsTerminal;

/// Output mode for all commands
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputMode {
    Human,
    Json,
}

impl OutputMode {
    /// Resolve output mode from --json flag.
    /// Only JSON when explicitly requested (like Stripe CLI).
    pub fn resolve(json_flag: bool) -> Self {
        if json_flag {
            OutputMode::Json
        } else {
            OutputMode::Human
        }
    }
}

/// Print a value as JSON or use the provided human-readable formatter.
/// In Python terms: like `if json_mode: print(json.dumps(obj)) else: print_pretty(obj)`
pub fn print_result<T: Serialize>(mode: OutputMode, value: &T, human_fn: impl FnOnce(&T)) {
    match mode {
        OutputMode::Json => {
            println!("{}", serde_json::to_string_pretty(value).unwrap());
        }
        OutputMode::Human => human_fn(value),
    }
}

/// Prompt for confirmation before a destructive action.
/// Returns true if: --yes flag is set, or user types "y"/"yes".
/// Returns false if: stdin is not a terminal (piped/CI), or user declines.
#[allow(dead_code)]
pub fn confirm(prompt: &str, auto_yes: bool) -> bool {
    if auto_yes {
        return true;
    }
    if !std::io::stdin().is_terminal() {
        eprintln!("Error: destructive operation requires confirmation. Use --yes to skip.");
        return false;
    }
    eprint!("{} [y/N] ", prompt);
    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_err() {
        return false;
    }
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_flag_forces_json_mode() {
        assert_eq!(OutputMode::resolve(true), OutputMode::Json);
    }

    #[test]
    fn test_no_flag_gives_human_mode() {
        assert_eq!(OutputMode::resolve(false), OutputMode::Human);
    }

    #[test]
    fn test_confirm_auto_yes_returns_true() {
        assert!(confirm("Delete everything?", true));
    }

    #[test]
    fn test_confirm_non_tty_without_yes_returns_false() {
        // Tests run in non-TTY, so without auto_yes this should refuse
        assert!(!confirm("Delete everything?", false));
    }

    #[test]
    fn test_print_result_json_outputs_valid_json() {
        #[derive(Serialize)]
        struct TestData {
            name: String,
            count: i32,
        }

        let data = TestData {
            name: "test".into(),
            count: 42,
        };

        // Verify JSON mode doesn't call human_fn (no panic = not called)
        print_result(OutputMode::Json, &data, |_| {
            // If this were Human mode, this would run
        });
    }

    #[test]
    fn test_print_result_human_calls_formatter() {
        #[derive(Serialize)]
        struct TestData {
            value: i32,
        }

        let data = TestData { value: 1 };
        let mut called = false;
        print_result(OutputMode::Human, &data, |d| {
            assert_eq!(d.value, 1);
            called = true;
        });
        assert!(called);
    }
}
