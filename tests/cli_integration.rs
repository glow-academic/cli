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

// ── Top-level Glow instance commands ─────────────────────────

#[test]
fn test_health_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["health", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("health"));
}

#[test]
fn test_login_placeholder() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["login"])
        .assert()
        .success()
        .stdout(predicate::str::contains("TODO: instance login"));
}

#[test]
fn test_logout_placeholder() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["logout"])
        .assert()
        .success()
        .stdout(predicate::str::contains("TODO: instance logout"));
}

// ── Top-level identity & streaming commands ─────────────────

#[test]
fn test_context_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["context", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("context"));
}

#[test]
fn test_emulate_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["emulate", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("TARGET_PROFILE_ID"));
}

#[test]
fn test_unemulate_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["unemulate", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("emulat"));
}

#[test]
fn test_generate_requires_group_id() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["generate"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("GROUP_ID"));
}

#[test]
fn test_stream_requires_artifact_and_operation() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["stream"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--artifact"))
        .stderr(predicate::str::contains("--operation"));
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

// ── Instance management ─────────────────────────────────────

#[test]
fn test_instances_list_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["instances", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("configured"));
}

#[test]
fn test_instances_alias() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["inst", "--help"])
        .assert()
        .success();
}

#[test]
fn test_instances_add_requires_url() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["instances", "add", "test-inst"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--url"));
}

#[test]
fn test_use_unknown_instance_fails() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["use", "nonexistent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown instance"));
}

// ── Instance-level: connect, disconnect, problem ──────────

#[test]
fn test_connect_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["connect", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("stream session"));
}

#[test]
fn test_disconnect_requires_sid() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["disconnect"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("SID"));
}

#[test]
fn test_problem_requires_kind_and_message() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["problem"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--kind"));
}

#[test]
fn test_stream_accepts_types_and_limit() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["stream", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--types"))
        .stdout(predicate::str::contains("--limit"));
}
