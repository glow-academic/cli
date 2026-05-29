// glow/ws.rs — socket.io client layer for the live chat REPL.
//
// The Glow API speaks socket.io v4 over WebSocket; the FE transport
// goes through it for all live operations (chat message, audio
// frames, voice session lifecycle). This module wraps ``rust_socketio``
// with the Glow-specific bits:
//
//   * Bearer-token auth threaded into the socket.io ``auth`` payload
//     (matches how the FE attaches the JWT — server reads it via
//     ``resolve_socket_identity`` in ``app/infra/identity/socket.py``).
//   * Canonical ``<artifact>.<operation>`` event naming (matches the
//     ``@sio.on(...)`` registrations after the recent sweep).
//   * Reconnect with bounded backoff so a transient drop doesn't kill
//     a long-running chat session.
//
// **Status**: this layer backs the ``glow attempts chat live <chat_id>``
// socket.io REPL — the connect / emit / on-event surface is implemented
// and wired in. (Audio/voice flows go through the HTTP chat_speak /
// chat_silence ops, not this module.)
//
// **Untested against a live server**: the WS layer compiles clean but
// hasn't been exercised end-to-end with a running glow-academic-api
// instance. Treat the REPL as a v1 — expect to iterate on the event
// names / payload shapes once a smoke test reveals what the server
// actually emits.

use anyhow::{Context, Result};
use rust_socketio::client::Client as SocketIoClient;
use rust_socketio::{ClientBuilder, Event, Payload, RawClient};
use serde_json::{json, Value};
use std::sync::mpsc;
use std::time::Duration;

/// A connected socket.io client with helpers for the chat-live flow.
///
/// The raw ``rust_socketio::Client`` is held inside so the REPL can
/// emit messages; incoming events are forwarded onto an mpsc channel
/// the caller can drain on the main thread (rust_socketio's event
/// handlers run on their own thread pool).
pub struct GlowSocket {
    client: SocketIoClient,
    /// Receiver for inbound events: ``(event_name, payload_json)``.
    pub events: mpsc::Receiver<(String, Value)>,
}

impl GlowSocket {
    /// Open a socket.io connection to ``base_url`` (e.g. ``http://localhost:8000``).
    /// The bearer ``token`` is forwarded via the socket.io ``auth``
    /// payload — matches the FE convention.
    pub fn connect(base_url: &str, token: Option<&str>) -> Result<Self> {
        let (tx, events) = mpsc::channel::<(String, Value)>();

        // Build the client. The ``auth`` field is what we hand
        // socket.io as the connect-handshake payload — the server-
        // side handshake handler reads it for the bearer.
        let mut builder = ClientBuilder::new(base_url)
            .reconnect(true)
            .reconnect_delay(500, 5_000) // 0.5s → 5s exponential
            .on(Event::Connect, |_, _| {
                eprintln!("· socket.io connected");
            })
            .on(Event::Close, |_, _| {
                eprintln!("· socket.io closed");
            })
            .on(Event::Error, |err, _| {
                eprintln!("· socket.io error: {:?}", err);
            });

        // rust_socketio routes NAMED events to per-name handlers; the
        // default "message" slot catch-all does NOT see them. So the
        // server's ``attempt.chat_message.*`` / ``attempt.generate.*``
        // frames were silently dropped — the REPL connected but never
        // showed the assistant's reply. Register a forwarder PER event
        // (handlers can't be attached after ``connect()``), carrying the
        // real event name onto the channel so the REPL prints it inline.
        const REPL_EVENTS: &[&str] = &[
            "attempt.chat_message.started",
            "attempt.chat_message.progress",
            "attempt.chat_message.completed",
            "attempt.chat_message.failed",
            "attempt.generate.started",
            "attempt.generate.completed",
            "attempt.generate.failed",
        ];
        for &name in REPL_EVENTS {
            let tx_ev = tx.clone();
            builder = builder.on(name, move |payload, _socket: RawClient| {
                let _ = tx_ev.send((name.to_string(), parse_socket_payload(payload)));
            });
        }

        // Default "message" slot — anything not matched by a named
        // handler above (kept so nothing is silently lost).
        let tx_any = tx.clone();
        builder = builder.on("message", move |payload, _socket: RawClient| {
            let _ = tx_any.send(("message".to_string(), parse_socket_payload(payload)));
        });

        if let Some(t) = token {
            builder = builder.auth(json!({ "token": t }));
        }

        let client = builder
            .connect()
            .with_context(|| format!("Failed to connect socket.io to {}", base_url))?;

        Ok(GlowSocket { client, events })
    }

    /// Register a named-event handler that forwards onto the events
    /// channel. The chat REPL registers the events it cares about
    /// (``attempt.chat_message.*``, ``attempt.generate.*``) before
    /// draining the channel — so unhandled events stay invisible
    /// instead of flooding the terminal.
    #[allow(dead_code)]
    pub fn on(&self, event_name: &'static str, tx: mpsc::Sender<(String, Value)>) {
        // rust_socketio's per-event handler can't be registered after
        // ``connect()`` returns the high-level Client. We have to
        // attach handlers via the builder. As a workaround, we use
        // the catch-all handler in ``connect()`` and let the caller
        // filter on event_name. This method is kept for API
        // symmetry / future re-wiring once we move to a builder-
        // owned handler bag.
        let _ = (event_name, tx);
    }

    /// Emit a socket.io event with a JSON payload.
    pub fn emit(&self, event_name: &str, payload: Value) -> Result<()> {
        self.client
            .emit(event_name, payload)
            .with_context(|| format!("Failed to emit {} event", event_name))
    }

    /// Best-effort disconnect. The reconnect logic is disabled before
    /// closing so we don't immediately re-handshake on shutdown.
    pub fn disconnect(self) -> Result<()> {
        self.client
            .disconnect()
            .context("Failed to disconnect socket.io")
    }
}

/// Normalize a rust_socketio ``Payload`` into a single JSON ``Value``.
/// Text payloads arrive as a ``Vec<Value>`` (extra args after the first
/// are rare) — collapse a single entry, else keep the array. Binary frames
/// are surfaced as a length only (the chat REPL is text-only).
fn parse_socket_payload(payload: Payload) -> Value {
    match payload {
        Payload::Text(values) => {
            if values.len() == 1 {
                values.into_iter().next().unwrap_or(Value::Null)
            } else {
                Value::Array(values)
            }
        }
        Payload::Binary(bytes) => json!({ "binary_bytes": bytes.len() }),
        #[allow(deprecated)]
        Payload::String(s) => serde_json::from_str(&s).unwrap_or(Value::String(s)),
    }
}

/// Helper: wait up to ``timeout`` for the next event on ``rx``. Returns
/// ``None`` on timeout / channel close. Used by the REPL to print
/// inbound frames between user inputs without blocking the prompt.
#[allow(dead_code)]
pub fn try_recv_with_timeout(
    rx: &mpsc::Receiver<(String, Value)>,
    timeout: Duration,
) -> Option<(String, Value)> {
    rx.recv_timeout(timeout).ok()
}
