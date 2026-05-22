// tests/cli_integration.rs — Binary-level integration tests
//
// These run the actual compiled binary and assert on stdout/stderr/exit codes.
// Like subprocess testing in Python (subprocess.run + assert) or execa in Node.
//
// Runs with: cargo test --test cli_integration

use assert_cmd::Command;
use predicates::prelude::*;

// ── Help & basic CLI structure ────────────────────────────────

#[test]
fn test_help_output() {
    Command::cargo_bin("glow")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Glow CLI"));
}

#[test]
fn test_glw_alias_works() {
    Command::cargo_bin("glw")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Glow CLI"));
}

#[test]
fn test_unknown_subcommand_shows_error() {
    Command::cargo_bin("glow")
        .unwrap()
        .arg("nonsense")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown resource 'nonsense'"))
        .stderr(predicate::str::contains("personas"));
}

// ── Per-resource media operations ────────────────────────────

#[test]
fn test_media_requires_action() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["documents", "file"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Expected a media action"));
}

#[test]
fn test_media_upload_requires_file() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["documents", "file", "upload"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--file"));
}

#[test]
fn test_media_download_requires_id() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["documents", "image", "download"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--id"));
}

#[test]
fn test_media_unknown_action_shows_error() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["documents", "file", "nonsense"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown media action"));
}
