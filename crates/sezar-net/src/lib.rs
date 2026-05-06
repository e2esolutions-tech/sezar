//! `sezar-net` — TLS / SSH / IPsec observation via eBPF.
//!
//! V1 ships TLS only. Hooks `sk_msg` for client/server hello frames,
//! parses the ciphersuite list + extensions, emits one
//! [`CryptoInventoryEvent`] per session (deduplicated on
//! `(client_ip, server_ip:port, hash(client_random))`).
//!
//! This crate is a typed stub at the moment — actual eBPF wiring
//! (probably `aya`) lands in repo issue `#SEZ-3`.

#![deny(missing_docs)]

use sezar_core::{Asset, AssetKind, CryptoInventoryEvent, Posture, Primitive, PrimitiveRole};

/// Module identifier emitted as `source_module` on every event.
pub const MODULE_NAME: &str = "sezar-net";

/// Sketched event constructor. Real version pulls from the eBPF ring
/// buffer; this one builds a synthetic event for tests + integration
/// scaffolding.
pub fn fake_tls_observation(host: impl Into<String>) -> CryptoInventoryEvent {
    CryptoInventoryEvent {
        schema_version: sezar_core::SCHEMA_VERSION,
        source_module: MODULE_NAME.into(),
        observed_at: chrono::Utc::now(),
        asset: Asset {
            kind: AssetKind::TlsSession,
            identity: format!("synthetic-{}", uuid_lite()),
            host: Some(host.into()),
        },
        primitives: vec![
            Primitive {
                role: PrimitiveRole::Kex,
                algorithm: "X25519".into(),
                parameters: Default::default(),
                pq_resistant: Some(false),
                nist_classification: None,
            },
            Primitive {
                role: PrimitiveRole::Sig,
                algorithm: "ECDSA-P256".into(),
                parameters: Default::default(),
                pq_resistant: Some(false),
                nist_classification: None,
            },
            Primitive {
                role: PrimitiveRole::Encrypt,
                algorithm: "AES-256-GCM".into(),
                parameters: Default::default(),
                pq_resistant: Some(true), // symmetric resists Grover
                nist_classification: None,
            },
            Primitive {
                role: PrimitiveRole::Hash,
                algorithm: "SHA-384".into(),
                parameters: Default::default(),
                pq_resistant: Some(true),
                nist_classification: None,
            },
        ],
        posture: Posture {
            score: 40,
            rationale: "X25519 + ECDSA-P256 are classical-only".into(),
            recommended_replacement: Some("Kyber768 + ML-DSA-65 hybrid".into()),
        },
    }
}

fn uuid_lite() -> String {
    // Avoid the uuid dep on this stub crate; deterministic-enough for
    // a placeholder.
    let now = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{now:x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_observation_validates_against_core_schema() {
        let ev = fake_tls_observation("example.com");
        assert_eq!(ev.source_module, MODULE_NAME);
        assert_eq!(ev.asset.kind, AssetKind::TlsSession);
        // Round-trip through JSON to make sure no skip_serializing trips us up.
        let s = serde_json::to_string(&ev).unwrap();
        assert!(s.contains("X25519"));
    }
}
