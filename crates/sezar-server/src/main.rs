//! `sezar-server` — collector + REST API + dashboard backend.
//!
//! V1 lands the smallest possible HTTP surface:
//!
//!   POST /v1/events           — ingest one or many events
//!   GET  /v1/inventory        — current asset list
//!   GET  /v1/posture          — org-level rollup
//!   GET  /healthz             — liveness probe
//!
//! Storage and authentication are stubbed in this initial commit;
//! Postgres + mTLS bootstrap land in V1 issues #SEZ-2 and #SEZ-6.

use anyhow::Result;
use axum::{
    extract::Json,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use sezar_core::CryptoInventoryEvent;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    info!(version = env!("CARGO_PKG_VERSION"), "Starting sezar-server (V1 stub)");

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/v1/events", post(ingest_event))
        .route("/v1/inventory", get(inventory_stub))
        .route("/v1/posture", get(posture_stub));

    let addr = "0.0.0.0:8090";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(%addr, "sezar-server listening");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn healthz() -> &'static str {
    "ok"
}

/// Stub: accept the event, log it, drop it on the floor. V1 issue
/// #SEZ-2 wires Postgres persistence behind this handler.
async fn ingest_event(Json(ev): Json<CryptoInventoryEvent>) -> StatusCode {
    if ev.schema_version != sezar_core::SCHEMA_VERSION {
        warn!(
            got = ev.schema_version,
            expected = sezar_core::SCHEMA_VERSION,
            "rejecting event with unsupported schema_version"
        );
        return StatusCode::UNPROCESSABLE_ENTITY;
    }
    info!(
        module = %ev.source_module,
        asset_kind = ?ev.asset.kind,
        score = ev.posture.score,
        "event ingested (stub — not persisted yet)"
    );
    StatusCode::ACCEPTED
}

async fn inventory_stub() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "assets": [],
        "note": "V1 stub — storage lands in #SEZ-2",
    }))
}

async fn posture_stub() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "org_score": null,
        "by_kind": {},
        "note": "V1 stub — rollup engine lands in #SEZ-4",
    }))
}
