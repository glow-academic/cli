// render.rs — pretty human-readable rendering of API search/list responses.
//
// The API exposes a consistent search envelope across artifacts: a plural
// collection array (``personas`` / ``scenarios`` / ``simulations`` /
// ``cohorts``), a ``total_count``, and per-field filter sections. Attempts
// diverge slightly (``data`` array + pagination fields). For the common
// "list things" commands we turn that JSON into a compact, colored table
// instead of dumping raw JSON.
//
// This only runs in ``OutputMode::Human``; ``--json`` still prints the raw
// pretty JSON untouched (see ``commands::glow::cmd_resource_action_ext``).
// Anything we don't recognize returns ``false`` so the caller falls back to
// the JSON dump — so an error response or an unexpected shape is never
// swallowed.

use colored::Colorize;
use serde_json::Value;
use tabled::builder::Builder;
use tabled::settings::Style;

/// Top-level entry: try every known renderer in turn, falling back to the
/// caller's raw-JSON dump (return ``false``) for anything unrecognized.
///
/// ``resource`` is the singular API path ("persona", "scenario", …) and
/// ``action`` the verb ("search" / "get" / "create" / …). Both flow in from
/// the dispatcher, so the ergonomic helpers (``glow personas search``) and the
/// generic body-driven path (``glow scenarios get --body …``) share this hook.
///
/// Ordering matters only in that each renderer guards on its own shape: a
/// renderer returns ``false`` (rather than printing garbage) when the payload
/// isn't what it expects, so unexpected/error responses are never swallowed.
pub fn render(resource: &str, action: &str, resp: &Value) -> bool {
    // 1. Search / list tables (guards on action + collection shape).
    if render_search(resource, action, resp) {
        return true;
    }
    // 2. Single-object detail views (``get``).
    if action == "get" {
        if resource == "attempt" && render_attempt_get(resp) {
            return true;
        }
        if render_get(resource, resp) {
            return true;
        }
    }
    // 3. System views (groups table / health summary), shape-detected.
    if resource == "system" && render_system(resp) {
        return true;
    }
    // 4. Write / ack status envelopes (create/update/delete/…/emulate/…).
    if render_status(resource, action, resp) {
        return true;
    }
    false
}

/// Render a known search/list response as a table. Returns ``true`` when we
/// handled it, ``false`` to fall back to the generic JSON dump.
///
/// ``resource`` is the singular API path ("persona", "scenario", …) and
/// ``action`` the verb ("search"). Both flow in from the dispatcher, so the
/// ergonomic helpers (``glow personas search``) and the generic body-driven
/// path (``glow scenarios search --body …``) are covered by the same hook.
pub fn render_search(resource: &str, action: &str, resp: &Value) -> bool {
    // ``list`` covers the ``glow simulations list`` sugar, which POSTs to
    // /simulation/search under the hood but arrives here as action="search"
    // anyway; keep both for safety.
    if !matches!(action, "search" | "list" | "history") {
        return false;
    }
    match resource {
        "persona" => render_personas(resp),
        "scenario" => render_scenarios(resp),
        "simulation" => render_simulations(resp),
        "cohort" => render_cohorts(resp),
        "attempt" => render_attempts(resp),
        // Every other artifact shares the envelope: a ``{resource}s`` array of
        // items with ``{resource}_id`` / ``name`` / ``description``. A generic
        // table covers documents/agents/models/fields/departments/rubrics/
        // evals/tools/providers/parameters/… without a bespoke renderer each.
        _ => render_generic_search(resource, resp),
    }
}

/// Fallback search renderer for artifacts without a tailored column set.
/// Reads the conventional ``{resource}s`` collection and shows the columns
/// every artifact has in common. Returns ``false`` (→ JSON) if the expected
/// collection key is absent, so unusual shapes aren't forced into a table.
fn render_generic_search(resource: &str, resp: &Value) -> bool {
    let collection = format!("{}s", resource);
    let Some(items) = resp.get(&collection).and_then(Value::as_array) else {
        return false;
    };
    let id_key = format!("{}_id", resource);
    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|it| {
            vec![
                bold_name(get_str(it, "name")),
                truncate(get_str(it, "description").unwrap_or(""), 44),
                date_part(get_str(it, "updated_at")),
                get_str(it, &id_key)
                    .or_else(|| get_str(it, "id"))
                    .unwrap_or("-")
                    .to_string(),
                badges(it),
            ]
        })
        .collect();
    print_table(
        &["NAME", "DESCRIPTION", "UPDATED", "ID", "FLAGS"],
        rows,
        resp,
        items.len(),
        resource,
        &format!("No {}s found.", resource),
    );
    true
}

// ── Per-resource renderers ───────────────────────────────────────

fn render_personas(resp: &Value) -> bool {
    let Some(items) = resp.get("personas").and_then(Value::as_array) else {
        return false;
    };
    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|p| {
            vec![
                color_name(get_str(p, "name"), get_str(p, "color")),
                num(p, "num_scenarios"),
                num(p, "num_profiles"),
                date_part(get_str(p, "updated_at")),
                get_str(p, "persona_id").unwrap_or("-").to_string(),
                badges(p),
            ]
        })
        .collect();
    print_table(
        &["NAME", "SCN", "PROF", "UPDATED", "ID", "FLAGS"],
        rows,
        resp,
        items.len(),
        "persona",
        "No personas found.",
    );
    true
}

fn render_scenarios(resp: &Value) -> bool {
    let Some(items) = resp.get("scenarios").and_then(Value::as_array) else {
        return false;
    };
    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|s| {
            vec![
                bold_name(get_str(s, "name")),
                truncate(get_str(s, "problem_statement").unwrap_or(""), 44),
                num(s, "num_simulations"),
                date_part(get_str(s, "updated_at")),
                get_str(s, "scenario_id").unwrap_or("-").to_string(),
                badges(s),
            ]
        })
        .collect();
    print_table(
        &["NAME", "PROBLEM", "SIMS", "UPDATED", "ID", "FLAGS"],
        rows,
        resp,
        items.len(),
        "scenario",
        "No scenarios found.",
    );
    true
}

fn render_simulations(resp: &Value) -> bool {
    let Some(items) = resp.get("simulations").and_then(Value::as_array) else {
        return false;
    };
    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|s| {
            vec![
                bold_name(get_str(s, "name")),
                truncate(get_str(s, "description").unwrap_or(""), 44),
                num(s, "num_cohorts"),
                date_part(get_str(s, "updated_at")),
                get_str(s, "simulation_id").unwrap_or("-").to_string(),
                badges(s),
            ]
        })
        .collect();
    print_table(
        &["NAME", "DESCRIPTION", "COHORTS", "UPDATED", "ID", "FLAGS"],
        rows,
        resp,
        items.len(),
        "simulation",
        "No simulations found.",
    );
    true
}

fn render_cohorts(resp: &Value) -> bool {
    let Some(items) = resp.get("cohorts").and_then(Value::as_array) else {
        return false;
    };
    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|c| {
            vec![
                bold_name(get_str(c, "name")),
                num(c, "num_members"),
                num(c, "usage_count"),
                date_part(get_str(c, "updated_at")),
                get_str(c, "cohort_id").unwrap_or("-").to_string(),
                badges(c),
            ]
        })
        .collect();
    print_table(
        &["NAME", "MEMBERS", "USES", "UPDATED", "ID", "FLAGS"],
        rows,
        resp,
        items.len(),
        "cohort",
        "No cohorts found.",
    );
    true
}

fn render_attempts(resp: &Value) -> bool {
    // Attempts use the ``HistoryResponse`` envelope: ``data`` array instead
    // of a named collection, with its own per-item fields.
    let Some(items) = resp.get("data").and_then(Value::as_array) else {
        return false;
    };
    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|a| {
            let done = a.get("num_scenarios_completed").and_then(Value::as_u64);
            let total = a.get("num_scenarios").and_then(Value::as_u64);
            let progress = match (done, total) {
                (Some(d), Some(t)) => format!("{}/{}", d, t),
                _ => "-".to_string(),
            };
            vec![
                bold_name(get_str(a, "simulation_name")),
                get_str(a, "profile_name").unwrap_or("-").to_string(),
                progress,
                score_cell(a),
                date_part(get_str(a, "date")),
                get_str(a, "attempt_id").unwrap_or("-").to_string(),
            ]
        })
        .collect();
    print_table(
        &["SIMULATION", "PROFILE", "PROGRESS", "SCORE", "DATE", "ID"],
        rows,
        resp,
        items.len(),
        "attempt",
        "No attempts found.",
    );
    true
}

// ── Tier 1: write / ack status envelopes ─────────────────────────

/// Mutations (create/update/delete/duplicate/draft/archive/refresh) on every
/// artifact share the shape ``{ results: [{success, id, message, errors}],
/// idempotency_key }``. A handful of endpoints return small ack objects
/// instead. Turn both into a one-line-per-outcome summary. Returns ``false``
/// for anything that's neither, so the caller falls back to JSON.
fn render_status(resource: &str, action: &str, resp: &Value) -> bool {
    // Shared mutation envelope — detected by a ``results`` array whose items
    // carry a ``success`` flag (so we don't grab unrelated ``results`` keys).
    if let Some(results) = resp.get("results").and_then(Value::as_array) {
        if results.iter().any(|r| r.get("success").is_some()) {
            return render_mutation_results(resource, results, resp.get("idempotency_key"));
        }
    }

    // Small per-endpoint ack objects, matched by (resource, action) + a
    // sentinel field so an error payload on the same route isn't claimed.
    match (resource, action) {
        ("profile", "emulate") if resp.get("allowed").is_some() => {
            let allowed = resp.get("allowed").and_then(Value::as_bool) == Some(true);
            if allowed {
                let until = get_str(resp, "expires_at")
                    .map(|e| format!(" until {}", e))
                    .unwrap_or_default();
                println!("{} Emulating{}", "✓".green().bold(), until);
                if let Some(g) = get_str(resp, "grant_id") {
                    println!("  {} {}", "grant".dimmed(), g.dimmed());
                }
            } else {
                println!("{} Emulation not allowed", "✗".red().bold());
            }
            true
        }
        ("profile", "unemulate") if resp.get("ok").is_some() => {
            if resp.get("ok").and_then(Value::as_bool) == Some(true) {
                println!("{} Stopped emulation", "✓".green().bold());
            } else {
                let reason = get_str(resp, "reason").unwrap_or("unknown reason");
                println!("{} {}", "✗".red().bold(), reason);
            }
            true
        }
        ("attempt", "chat_message") | ("attempt", "message")
            if resp.get("message_id").is_some() || resp.get("content_ids").is_some() =>
        {
            let ok = resp.get("success").and_then(Value::as_bool) != Some(false);
            let mark = if ok {
                "✓".green().bold()
            } else {
                "✗".red().bold()
            };
            match get_str(resp, "message_id") {
                Some(id) => println!("{} Message sent {}", mark, id.dimmed()),
                None => println!("{} Message sent", mark),
            }
            true
        }
        (_, "problem") if resp.get("success").is_some() => {
            if resp.get("success").and_then(Value::as_bool) == Some(true) {
                println!("{} Problem reported", "✓".green().bold());
            } else {
                println!("{} Problem report failed", "✗".red().bold());
            }
            true
        }
        _ => false,
    }
}

/// One line per result item: a green ✓ / red ✗, the API's human message, a
/// dimmed id, and indented per-field validation errors. A multi-item batch
/// gets an ``N ok, M failed`` footer; a pending soft-call key is surfaced so
/// the user knows an ack is outstanding.
fn render_mutation_results(resource: &str, results: &[Value], idem: Option<&Value>) -> bool {
    let mut ok = 0usize;
    let mut failed = 0usize;
    for r in results {
        let success = r.get("success").and_then(Value::as_bool) == Some(true);
        let mark = if success {
            ok += 1;
            "✓".green().bold()
        } else {
            failed += 1;
            "✗".red().bold()
        };
        let message = get_str(r, "message").unwrap_or(if success { "ok" } else { "failed" });
        match get_str(r, "id") {
            Some(id) => println!("{} {}  {}", mark, message, id.dimmed()),
            None => println!("{} {}", mark, message),
        }
        if let Some(errors) = r.get("errors").and_then(Value::as_array) {
            for e in errors {
                println!("    {} {}", "↳".red().dimmed(), format_field_error(e));
            }
        }
    }

    if results.len() != 1 {
        let mut parts = vec![format!("{} ok", ok)];
        if failed > 0 {
            parts.push(format!("{} failed", failed));
        }
        println!(
            "\n{}",
            format!("{} {}s — {}", results.len(), resource, parts.join(", ")).dimmed()
        );
    }

    // A non-null idempotency_key on a write means a staged soft-call awaiting
    // ack; show it so the user can promote/reject it.
    if let Some(key) = idem.and_then(Value::as_str) {
        println!(
            "{}",
            format!(
                "soft-call key {} — ack with --idempotency_key {} --accept true",
                key, key
            )
            .dimmed()
        );
    }
    true
}

/// Render a single per-field validation error compactly. The shape varies, so
/// fall back to the raw JSON of the error when it isn't an obvious
/// ``{field, message}`` object.
fn format_field_error(e: &Value) -> String {
    let field = get_str(e, "field").or_else(|| get_str(e, "loc"));
    let msg = get_str(e, "message").or_else(|| get_str(e, "msg"));
    match (field, msg) {
        (Some(f), Some(m)) => format!("{}: {}", f, m),
        (None, Some(m)) => m.to_string(),
        _ => serde_json::to_string(e).unwrap_or_default(),
    }
}

// ── Tier 2: single-object detail views ───────────────────────────

/// A row in a detail panel: which response section to read, the value key
/// inside the selected item, and whether multiple selected items are joined.
struct DetailRow {
    label: &'static str,
    section: &'static str,
    value_key: &'static str,
    multi: bool,
}

const fn row(label: &'static str, section: &'static str, value_key: &'static str) -> DetailRow {
    DetailRow {
        label,
        section,
        value_key,
        multi: false,
    }
}
const fn multi(label: &'static str, section: &'static str, value_key: &'static str) -> DetailRow {
    DetailRow {
        label,
        section,
        value_key,
        multi: true,
    }
}

/// Render ``glow <artifact> get`` as a key-value panel. The detail response is
/// a bundle of per-field resource arrays where each item has a ``selected``
/// flag; the "current value" of a field is the selected item(s). We headline
/// the selected name (tinted by the selected color for personas) and list the
/// rest. Returns ``false`` for unknown artifacts or non-detail payloads.
fn render_get(resource: &str, resp: &Value) -> bool {
    let spec: &[DetailRow] = match resource {
        "persona" => &[
            row("Description", "descriptions", "description"),
            row("Color", "colors", "hex_code"),
            // ``icons[].value`` is the raw SVG markup; ``name`` is the
            // human-readable icon identifier we actually want to show.
            row("Icon", "icons", "name"),
            row("Instructions", "instructions", "template"),
            multi("Departments", "departments", "name"),
        ],
        "scenario" => &[
            row("Description", "descriptions", "description"),
            row("Problem", "problem_statements", "problem_statement"),
            multi("Objectives", "objectives", "objective"),
            multi("Personas", "personas", "name"),
            multi("Documents", "documents", "name"),
            multi("Departments", "departments", "name"),
        ],
        "simulation" => &[
            row("Description", "descriptions", "description"),
            multi("Scenarios", "scenarios", "name"),
            multi("Departments", "departments", "name"),
        ],
        "cohort" => &[
            row("Description", "descriptions", "description"),
            multi("Simulations", "simulations", "name"),
            multi("Profiles", "profiles", "name"),
            multi("Departments", "departments", "name"),
        ],
        // Every other artifact's detail response shares the same
        // ``names``/``descriptions``/``departments`` sections, so show that
        // common subset rather than dumping JSON.
        _ => &[
            row("Description", "descriptions", "description"),
            multi("Departments", "departments", "name"),
        ],
    };

    // Explicit "not found" beats a misleading empty panel.
    if resp
        .get(format!("{}_exists", resource))
        .and_then(Value::as_bool)
        == Some(false)
    {
        println!("{}", format!("No {} found.", resource).dimmed());
        return true;
    }
    // A detail response always carries the ``names`` section; its absence
    // means this isn't a get payload (e.g. an error), so fall back to JSON.
    if resp.get("names").is_none() {
        return false;
    }

    let name = selected_one(resp, "names", "name").unwrap_or_else(|| "(unnamed)".to_string());
    let hex = selected_one(resp, "colors", "hex_code");
    println!("{}", color_name(Some(&name), hex.as_deref()));

    for f in spec {
        let value = if f.multi {
            let vals = selected_all(resp, f.section, f.value_key);
            if vals.is_empty() {
                continue;
            }
            // Cap long lists (e.g. a cohort's members) so one row can't run
            // away; show the count of the remainder.
            const CAP: usize = 6;
            if vals.len() > CAP {
                format!(
                    "{} {}",
                    vals[..CAP].join(", "),
                    format!("+{} more", vals.len() - CAP).dimmed()
                )
            } else {
                vals.join(", ")
            }
        } else {
            match selected_one(resp, f.section, f.value_key) {
                Some(v) if !v.is_empty() => v,
                _ => continue,
            }
        };
        // Color gets a truecolor swatch alongside the hex.
        let display = if f.value_key == "hex_code" {
            match parse_hex(&value) {
                Some((r, g, b)) => format!("{} {}", value, "███".truecolor(r, g, b)),
                None => value,
            }
        } else {
            truncate(&value, 100)
        };
        println!("  {:<13} {}", f.label.dimmed(), display);
    }

    if resp.get("can_edit").and_then(Value::as_bool) == Some(false) {
        println!("  {:<13} {}", "Editable".dimmed(), "no".dimmed());
    }
    true
}

// ── Tier 3: attempt detail + system views ────────────────────────

/// Render ``glow attempts get`` — the nested attempt bundle. Header (sim +
/// who + status), score line, then a table of the attempt's chats.
fn render_attempt_get(resp: &Value) -> bool {
    if resp.get("attempt_exists").and_then(Value::as_bool) == Some(false) {
        println!("{}", "No attempt found.".dimmed());
        return true;
    }
    // ``attempt`` is the anchor object; without it this isn't the detail shape.
    let Some(attempt) = resp.get("attempt") else {
        return false;
    };

    let sim_name = resp
        .get("simulation")
        .and_then(|s| get_str(s, "name"))
        .unwrap_or("(simulation)");
    println!("{}", sim_name.bold());

    if let Some(who) = get_str(attempt, "profile_name") {
        println!("  {:<13} {}", "Profile".dimmed(), who);
    }
    let status = if resp.get("is_active").and_then(Value::as_bool) == Some(true) {
        "active".yellow().to_string()
    } else if resp.get("show_results").and_then(Value::as_bool) == Some(true) {
        "complete".green().to_string()
    } else {
        "—".dimmed().to_string()
    };
    println!("  {:<13} {}", "Status".dimmed(), status);
    if let Some(t) = resp.get("timer").and_then(|t| get_str(t, "formatted")) {
        println!("  {:<13} {}", "Time".dimmed(), t);
    }

    if let Some(agg) = resp.get("aggregated_results") {
        let passed = agg.get("passed").and_then(Value::as_bool);
        let pct = agg.get("percentage").and_then(Value::as_f64);
        let done = agg.get("chats_completed").and_then(Value::as_u64);
        let total = agg.get("total_chats").and_then(Value::as_u64);
        let mut score = match pct {
            Some(p) => format!("{:.0}%", p),
            None => "—".to_string(),
        };
        score = match passed {
            Some(true) => score.green().to_string(),
            Some(false) => score.red().to_string(),
            None => score,
        };
        let chats = match (done, total) {
            (Some(d), Some(t)) => format!("  ({}/{} chats)", d, t),
            _ => String::new(),
        };
        println!("  {:<13} {}{}", "Score".dimmed(), score, chats.dimmed());
    }

    // Chat list table.
    if let Some(chats) = resp
        .get("entries")
        .and_then(|e| e.get("attempt_chat"))
        .and_then(Value::as_array)
    {
        if !chats.is_empty() {
            let rows: Vec<Vec<String>> = chats
                .iter()
                .map(|c| {
                    let pos = c
                        .get("position")
                        .and_then(Value::as_u64)
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "-".to_string());
                    let done = if c.get("completed").and_then(Value::as_bool) == Some(true) {
                        "✓".green().to_string()
                    } else {
                        "·".dimmed().to_string()
                    };
                    let grade = c.get("grade");
                    let score = grade
                        .and_then(|g| g.get("score"))
                        .and_then(Value::as_f64)
                        .map(|s| format!("{}", s))
                        .unwrap_or_else(|| "-".to_string());
                    let passed = match grade.and_then(|g| g.get("passed")).and_then(Value::as_bool)
                    {
                        Some(true) => "pass".green().to_string(),
                        Some(false) => "fail".red().to_string(),
                        None => "-".dimmed().to_string(),
                    };
                    let id = get_str(c, "id").unwrap_or("-").to_string();
                    vec![pos, done, score, passed, id]
                })
                .collect();
            println!();
            let mut builder = Builder::default();
            builder.push_record(
                ["#", "DONE", "SCORE", "RESULT", "CHAT ID"]
                    .iter()
                    .map(|h| h.dimmed().to_string()),
            );
            for r in rows {
                builder.push_record(r);
            }
            let mut table = builder.build();
            table.with(Style::blank());
            println!("{}", table);
        }
    }
    true
}

/// System views: ``groups`` (generation/cost groups) renders as a table;
/// ``health`` as a compact count summary. Both are shape-detected so a
/// generic ``system`` action that returns something else falls back to JSON.
fn render_system(resp: &Value) -> bool {
    // Single group detail (``system group <id>``): top-level ``group_id`` +
    // ``runs`` array, and no ``data`` collection (which marks the list).
    if resp.get("data").is_none() {
        if let (Some(gid), Some(runs)) = (
            resp.get("group_id").and_then(Value::as_str),
            resp.get("runs").and_then(Value::as_array),
        ) {
            let name = get_str(resp, "name").filter(|s| !s.is_empty());
            println!("{}", bold_name(name.or(Some("(unnamed group)"))));
            println!("  {:<10} {}", "ID".dimmed(), gid);
            println!("  {:<10} {}", "Runs".dimmed(), runs.len());
            for (label, key) in [
                ("Agents", "agents"),
                ("Models", "models"),
                ("Profiles", "profiles"),
            ] {
                // These sub-resources may be arrays of strings or of objects
                // with a ``name``; render whichever we can, skip otherwise.
                if let Some(arr) = resp.get(key).and_then(Value::as_array) {
                    let names: Vec<String> = arr
                        .iter()
                        .filter_map(|v| {
                            v.as_str()
                                .map(str::to_string)
                                .or_else(|| get_str(v, "name").map(str::to_string))
                        })
                        .collect();
                    if !names.is_empty() {
                        println!("  {:<10} {}", label.dimmed(), names.join(", "));
                    }
                }
            }
            return true;
        }
    }

    // Groups/pricing: ``data`` array whose items carry ``group_id``.
    if let Some(items) = resp.get("data").and_then(Value::as_array) {
        if items.iter().any(|i| i.get("group_id").is_some()) {
            let rows: Vec<Vec<String>> = items
                .iter()
                .map(|g| {
                    vec![
                        bold_name(get_str(g, "group_name")),
                        num(g, "run_count"),
                        cost_cell(g.get("total_cost")),
                        date_part(get_str(g, "first_run_at")),
                        date_part(get_str(g, "last_run_at")),
                        get_str(g, "group_id").unwrap_or("-").to_string(),
                    ]
                })
                .collect();
            print_table(
                &["GROUP", "RUNS", "COST", "FIRST", "LAST", "ID"],
                rows,
                resp,
                items.len(),
                "group",
                "No groups found.",
            );
            return true;
        }
    }

    // Health: ``views.service_hourly`` + ``total_count``.
    if let Some(views) = resp.get("views") {
        let services = views
            .get("service_hourly")
            .and_then(Value::as_array)
            .map(|a| a.len())
            .unwrap_or(0);
        let rows = resp.get("total_count").and_then(Value::as_u64).unwrap_or(0);
        println!(
            "{} system reachable — {} metric rows, {} service samples",
            "✓".green().bold(),
            rows,
            services
        );
        return true;
    }
    false
}

/// Format a money value (the API sends ``total_cost`` as a number or a
/// stringified Decimal) as ``$N.NN``.
fn cost_cell(v: Option<&Value>) -> String {
    let amount = match v {
        Some(Value::Number(n)) => n.as_f64(),
        Some(Value::String(s)) => s.parse::<f64>().ok(),
        _ => None,
    };
    match amount {
        Some(a) => format!("${:.2}", a),
        None => "-".dimmed().to_string(),
    }
}

/// First ``value_key`` among the ``selected: true`` items of ``section``.
fn selected_one(resp: &Value, section: &str, value_key: &str) -> Option<String> {
    selected_all(resp, section, value_key).into_iter().next()
}

/// All ``value_key`` strings among the ``selected: true`` items of ``section``.
fn selected_all(resp: &Value, section: &str, value_key: &str) -> Vec<String> {
    resp.get(section)
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter(|it| it.get("selected").and_then(Value::as_bool) == Some(true))
                .filter_map(|it| get_str(it, value_key).map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

// ── Shared table printing ────────────────────────────────────────

/// Build + print the table, then a dimmed "N of M <noun>s" footer.
/// ``shown`` is the number of rows we have in hand; ``total_count`` (when the
/// API supplies it) is the unpaginated total.
fn print_table(
    headers: &[&str],
    rows: Vec<Vec<String>>,
    resp: &Value,
    shown: usize,
    noun: &str,
    empty_msg: &str,
) {
    if rows.is_empty() {
        println!("{}", empty_msg.dimmed());
        return;
    }

    let mut builder = Builder::default();
    // Dim the header so the data rows stand out; the ``ansi`` feature keeps
    // the escape codes out of the column-width math.
    builder.push_record(headers.iter().map(|h| h.dimmed().to_string()));
    for row in rows {
        builder.push_record(row);
    }
    let mut table = builder.build();
    // ``blank`` = space-separated columns, no borders — closest to a plain
    // ``ls``-style listing.
    table.with(Style::blank());
    println!("{}", table);

    let total = resp
        .get("total_count")
        .and_then(Value::as_u64)
        .map(|t| t as usize)
        .unwrap_or(shown);
    let footer = if total > shown {
        format!("{} of {} {}s", shown, total, noun)
    } else {
        format!("{} {}{}", shown, noun, if shown == 1 { "" } else { "s" })
    };
    println!("\n{}", footer.dimmed());
}

// ── Cell formatters ──────────────────────────────────────────────

/// Tint a name with the artifact's own hex ``color`` (truecolor), bold.
/// Falls back to plain bold when the color is missing/unparseable.
fn color_name(name: Option<&str>, hex: Option<&str>) -> String {
    let name = name.unwrap_or("(unnamed)");
    match hex.and_then(parse_hex) {
        Some((r, g, b)) => name.truecolor(r, g, b).bold().to_string(),
        None => name.bold().to_string(),
    }
}

fn bold_name(name: Option<&str>) -> String {
    name.unwrap_or("(unnamed)").bold().to_string()
}

/// Colored status badges shared across artifacts. Each check is guarded on
/// the field being present + truthy, so resources that lack a field simply
/// skip its badge.
fn badges(item: &Value) -> String {
    let mut out: Vec<String> = Vec::new();
    if item.get("generated").and_then(Value::as_bool) == Some(true) {
        out.push("✨AI".green().to_string());
    }
    // ``mcp`` on most artifacts; agents spell it ``is_mcp``.
    if item.get("mcp").and_then(Value::as_bool) == Some(true)
        || item.get("is_mcp").and_then(Value::as_bool) == Some(true)
    {
        out.push("🔌MCP".cyan().to_string());
    }
    if item.get("is_practice").and_then(Value::as_bool) == Some(true) {
        out.push("practice".blue().to_string());
    }
    let has_pending = item.get("pending_status").and_then(Value::as_str) == Some("pending")
        || item
            .get("pending_operation")
            .and_then(Value::as_str)
            .is_some_and(|s| !s.is_empty());
    if has_pending {
        out.push("draft".yellow().to_string());
    }
    if item.get("is_inactive").and_then(Value::as_bool) == Some(true) {
        out.push("inactive".red().dimmed().to_string());
    }
    out.join(" ")
}

/// Attempt score cell: prefer ``pass_pct`` (rendered as N%), else the raw
/// ``score``, else a dash. Tinted by ``score_status`` when present.
fn score_cell(a: &Value) -> String {
    let text = if let Some(pct) = a.get("pass_pct").and_then(Value::as_f64) {
        format!("{:.0}%", pct)
    } else if let Some(score) = a.get("score").and_then(Value::as_f64) {
        // Trim a trailing ``.0`` so whole numbers read cleanly.
        if score.fract() == 0.0 {
            format!("{}", score as i64)
        } else {
            format!("{}", score)
        }
    } else {
        return "-".dimmed().to_string();
    };
    match a.get("score_status").and_then(Value::as_str) {
        Some("pass") | Some("passed") => text.green().to_string(),
        Some("fail") | Some("failed") => text.red().to_string(),
        _ => text,
    }
}

// ── Small value helpers ──────────────────────────────────────────

fn get_str<'a>(v: &'a Value, key: &str) -> Option<&'a str> {
    v.get(key).and_then(Value::as_str)
}

/// Integer count cell — the number when present, a dimmed dash otherwise so
/// "API didn't compute it" reads differently from a real zero.
fn num(v: &Value, key: &str) -> String {
    match v.get(key).and_then(Value::as_u64) {
        Some(n) => n.to_string(),
        None => "-".dimmed().to_string(),
    }
}

/// Take the date part of an ISO-8601 timestamp ("2026-05-20T14:…" →
/// "2026-05-20"). Avoids pulling in a datetime crate just for display.
fn date_part(ts: Option<&str>) -> String {
    match ts {
        Some(s) => s.split('T').next().unwrap_or(s).to_string(),
        None => "-".to_string(),
    }
}

/// Truncate to ``max`` chars (by char, not byte), appending "…" when cut.
fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let cut: String = s.chars().take(max.saturating_sub(1)).collect();
        format!("{}…", cut.trim_end())
    }
}

/// Parse ``#RRGGBB`` (or ``RRGGBB``) into an RGB triple.
fn parse_hex(hex: &str) -> Option<(u8, u8, u8)> {
    let h = hex.trim().trim_start_matches('#');
    if h.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&h[0..2], 16).ok()?;
    let g = u8::from_str_radix(&h[2..4], 16).ok()?;
    let b = u8::from_str_radix(&h[4..6], 16).ok()?;
    Some((r, g, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_hex_handles_with_and_without_hash() {
        assert_eq!(parse_hex("#ff8800"), Some((255, 136, 0)));
        assert_eq!(parse_hex("00ff00"), Some((0, 255, 0)));
        assert_eq!(parse_hex("nope"), None);
        assert_eq!(parse_hex("#fff"), None); // shorthand not supported
    }

    #[test]
    fn date_part_strips_time() {
        assert_eq!(date_part(Some("2026-05-20T14:30:00Z")), "2026-05-20");
        assert_eq!(date_part(Some("2026-05-20")), "2026-05-20");
        assert_eq!(date_part(None), "-");
    }

    #[test]
    fn truncate_respects_char_boundary() {
        assert_eq!(truncate("short", 10), "short");
        assert_eq!(truncate("abcdefghij", 5), "abcd…");
    }

    #[test]
    fn render_search_only_claims_known_shapes() {
        let personas = json!({ "personas": [], "total_count": 0 });
        // Empty but recognized → handled (prints the empty message).
        assert!(render_search("persona", "search", &personas));
        // Unknown resource → not handled.
        assert!(!render_search("widget", "search", &personas));
        // Non-list action → not handled.
        assert!(!render_search("persona", "get", &personas));
        // Recognized resource but wrong shape (no ``personas`` key) → fall
        // back to JSON so error payloads aren't swallowed.
        assert!(!render_search(
            "persona",
            "search",
            &json!({ "detail": "boom" })
        ));
    }

    #[test]
    fn render_generic_search_covers_other_artifacts() {
        // Artifacts without a bespoke renderer use the ``{resource}s`` +
        // ``{resource}_id`` convention.
        let docs = json!({
            "documents": [{ "document_id": "d1", "name": "Policy", "description": "A doc" }],
            "total_count": 3
        });
        assert!(render_search("document", "search", &docs));
        // agents spell MCP as ``is_mcp`` — exercised via the generic path.
        let agents = json!({
            "agents": [{ "agent_id": "a1", "name": "Bot", "is_mcp": true }],
            "total_count": 1
        });
        assert!(render_search("agent", "search", &agents));
        // Missing the conventional collection → fall back to JSON.
        assert!(!render_search(
            "document",
            "search",
            &json!({ "detail": "boom" })
        ));
    }

    #[test]
    fn render_personas_handles_full_row() {
        let resp = json!({
            "personas": [{
                "persona_id": "fa2b1c9e-0000-0000-0000-000000000000",
                "name": "Dr. Sarah Chen",
                "color": "#1188ee",
                "num_scenarios": 12,
                "num_profiles": 340,
                "generated": true,
                "mcp": true,
                "updated_at": "2026-05-20T10:00:00Z"
            }],
            "total_count": 12
        });
        assert!(render_search("persona", "search", &resp));
    }

    // ── Tier 1: status / ack ─────────────────────────────────────

    #[test]
    fn render_status_claims_mutation_envelope() {
        let ok = json!({
            "results": [{ "success": true, "id": "abc", "message": "Persona created" }],
            "idempotency_key": null
        });
        assert!(render_status("persona", "create", &ok));

        let failed = json!({
            "results": [{
                "success": false,
                "message": "Validation failed",
                "errors": [{ "field": "name", "message": "required" }]
            }]
        });
        assert!(render_status("persona", "update", &failed));
    }

    #[test]
    fn render_status_claims_acks_but_not_unknown() {
        assert!(render_status(
            "profile",
            "emulate",
            &json!({ "allowed": true, "grant_id": "g1", "expires_at": "2026-06-01T00:00:00Z" })
        ));
        assert!(render_status(
            "profile",
            "unemulate",
            &json!({ "ok": true })
        ));
        assert!(render_status(
            "attempt",
            "chat_message",
            &json!({ "success": true, "message_id": "m1", "content_ids": [] })
        ));
        // No results array, no recognized ack shape → fall back to JSON.
        assert!(!render_status(
            "persona",
            "context",
            &json!({ "foo": "bar" })
        ));
        // A ``results`` key that isn't the mutation envelope is ignored.
        assert!(!render_status(
            "system",
            "resolve",
            &json!({ "results": [1, 2, 3] })
        ));
    }

    // ── Tier 2: detail views ─────────────────────────────────────

    #[test]
    fn selected_helpers_pick_selected_items() {
        let resp = json!({
            "names": [
                { "name": "Old", "selected": false },
                { "name": "Current", "selected": true }
            ],
            "departments": [
                { "name": "A", "selected": true },
                { "name": "B", "selected": true },
                { "name": "C", "selected": false }
            ]
        });
        assert_eq!(
            selected_one(&resp, "names", "name").as_deref(),
            Some("Current")
        );
        assert_eq!(selected_all(&resp, "departments", "name"), vec!["A", "B"]);
        assert!(selected_one(&resp, "missing", "name").is_none());
    }

    #[test]
    fn render_get_handles_detail_and_rejects_non_detail() {
        let resp = json!({
            "persona_exists": true,
            "can_edit": true,
            "names": [{ "name": "Dr. Chen", "selected": true }],
            "descriptions": [{ "description": "An oncologist", "selected": true }],
            "colors": [{ "hex_code": "#1188ee", "selected": true }],
            "departments": [{ "name": "Oncology", "selected": true }]
        });
        assert!(render_get("persona", &resp));
        // exists=false → handled with a "not found" line.
        assert!(render_get("persona", &json!({ "persona_exists": false })));
        // Unknown artifact but a detail shape (has ``names``) → generic panel.
        assert!(render_get("widget", &resp));
        // No ``names`` section (e.g. an error body) → fall back to JSON, even
        // for an unknown artifact.
        assert!(!render_get("persona", &json!({ "detail": "boom" })));
        assert!(!render_get("widget", &json!({ "detail": "boom" })));
    }

    // ── Tier 3: attempt + system ─────────────────────────────────

    #[test]
    fn render_attempt_get_handles_bundle() {
        let resp = json!({
            "attempt_exists": true,
            "attempt": { "profile_name": "TA Johnson" },
            "simulation": { "name": "FERPA Training" },
            "is_active": false,
            "show_results": true,
            "aggregated_results": { "passed": true, "percentage": 80.0, "chats_completed": 1, "total_chats": 1 },
            "entries": { "attempt_chat": [{ "position": 1, "completed": true, "grade": { "score": 8.0, "passed": true }, "id": "c1" }] }
        });
        assert!(render_attempt_get(&resp));
        assert!(render_attempt_get(&json!({ "attempt_exists": false })));
        // No ``attempt`` anchor → not handled.
        assert!(!render_attempt_get(&json!({ "detail": "boom" })));
    }

    #[test]
    fn render_system_groups_and_health() {
        let groups = json!({
            "data": [{ "group_id": "g1", "group_name": "Run A", "run_count": 4, "total_cost": "1.25", "first_run_at": "2026-05-25T00:00:00Z", "last_run_at": "2026-05-25T01:00:00Z" }],
            "total_count": 9
        });
        assert!(render_system(&groups));
        let health = json!({ "views": { "service_hourly": [] }, "total_count": 0 });
        assert!(render_system(&health));
        // Single group detail: top-level group_id + runs, no ``data``.
        assert!(render_system(
            &json!({ "group_id": "g1", "name": "Run A", "runs": [] })
        ));
        // Neither shape → fall back to JSON.
        assert!(!render_system(&json!({ "foo": 1 })));
    }

    #[test]
    fn render_routes_by_action() {
        // Top-level dispatcher picks the right renderer.
        assert!(render(
            "persona",
            "search",
            &json!({ "personas": [], "total_count": 0 })
        ));
        assert!(render(
            "persona",
            "create",
            &json!({ "results": [{ "success": true, "message": "ok" }] })
        ));
        assert!(render(
            "system",
            "groups",
            &json!({ "data": [{ "group_id": "g1" }] })
        ));
        // Unrecognized → false so the caller dumps JSON.
        assert!(!render(
            "persona",
            "weird_action",
            &json!({ "anything": 1 })
        ));
    }

    #[test]
    fn cost_cell_formats_number_and_string() {
        assert_eq!(cost_cell(Some(&json!(1.5))), "$1.50");
        assert_eq!(cost_cell(Some(&json!("2.345"))), "$2.35");
        // Missing → dimmed dash (strip ANSI for the assert).
        assert!(cost_cell(None).contains('-'));
    }
}
