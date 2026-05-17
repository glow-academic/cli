use axum::{
    extract::Json,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde_json::json;
use std::process::Command;

/// GET / — version info
async fn root() -> impl IntoResponse {
    Json(json!({
        "name": "glow",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "Glow CLI — manage your platform",
    }))
}

/// GET /cli-spec.json — full CLI tree as JSON
async fn cli_spec() -> impl IntoResponse {
    Json(crate::build_cli_spec())
}

/// POST /exec — run a CLI command and return the result
///
/// Request: { "args": ["personas", "list"] }
/// Response: { "exit_code": 0, "stdout": "...", "stderr": "..." }
async fn exec(Json(body): Json<serde_json::Value>) -> impl IntoResponse {
    let args = body
        .get("args")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let bin = std::env::current_exe().unwrap_or_else(|_| "glow".into());

    match Command::new(&bin).args(&args).output() {
        Ok(output) => Json(json!({
            "exit_code": output.status.code().unwrap_or(-1),
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
        })),
        Err(e) => Json(json!({
            "exit_code": -1,
            "stdout": "",
            "stderr": format!("Failed to execute: {e}"),
        })),
    }
}

/// Start the CLI dev server.
pub async fn run(port: u16) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(root))
        .route("/cli-spec.json", get(cli_spec))
        .route("/exec", post(exec));

    let addr = format!("0.0.0.0:{port}");
    eprintln!("Glow CLI server listening on http://localhost:{port}");
    eprintln!("  GET  /              → version info");
    eprintln!("  GET  /cli-spec.json → CLI spec (for docs generation)");
    eprintln!("  POST /exec          → run a command");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
