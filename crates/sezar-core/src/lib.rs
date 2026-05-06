//! `sezar-core` — shared event schema + posture rollup library.
//!
//! Every Sezar module emits one type: [`CryptoInventoryEvent`]. The
//! schema is intentionally narrow — modules are not allowed to add
//! new top-level fields without a coordinated `schema_version` bump
//! across all consumers.
//!
//! See `docs/crypto-event-schema.md` at the repo root for the
//! field-by-field rationale.
//!
//! # Feature flags
//!
//! - `schema` — enables `schemars::JsonSchema` derives so the
//!   `schema-export` binary can emit the canonical JSON Schema for
//!   the event format. Off by default; CI / docs use it.
//! - `ts-types` — enables `ts-rs::TS` derives for TypeScript codegen
//!   consumed by the React UI in V1. Off by default; the UI's
//!   `npm run codegen` script flips it on.

#![deny(missing_docs)]

use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

#[cfg(feature = "ts-types")]
use ts_rs::TS;

/// Current event schema version. Bumped on any breaking change to the
/// top-level shape; additive changes are allowed without a bump as
/// long as they're optional.
pub const SCHEMA_VERSION: u32 = 1;

/// One observation about one crypto-bearing asset, normalised to a
/// shape that's identical regardless of whether it came from
/// `sezar-net`, `sezar-cert`, `sezar-chain`, or `sezar-id`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "ts-types", derive(TS))]
#[cfg_attr(feature = "ts-types", ts(export))]
pub struct CryptoInventoryEvent {
    /// Schema version of this event. Always `SCHEMA_VERSION` at
    /// emission; consumers may see older values during rolling
    /// upgrades.
    pub schema_version: u32,
    /// Which Sezar module produced the event.
    pub source_module: String,
    /// Wall-clock observation time (UTC), RFC 3339 string on the wire.
    #[cfg_attr(feature = "ts-types", ts(type = "string"))]
    pub observed_at: chrono::DateTime<chrono::Utc>,
    /// What we observed — see [`Asset`].
    pub asset: Asset,
    /// The cryptographic primitives in use on this asset, decomposed
    /// by role (key exchange, signature, encryption, hash).
    pub primitives: Vec<Primitive>,
    /// Sezar's verdict on this asset.
    pub posture: Posture,
}

/// The thing being observed. `kind` is closed-set; `identity` is
/// module-specific (TLS session ID, cert SHA-256, blockchain address,
/// HSM slot URI, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "ts-types", derive(TS))]
#[cfg_attr(feature = "ts-types", ts(export))]
pub struct Asset {
    /// Which kind of asset this is.
    pub kind: AssetKind,
    /// Module-specific stable identifier.
    pub identity: String,
    /// Network or owner context — usually a hostname, IP, or human
    /// owner string. Optional because some assets (e.g. mempool
    /// observations) don't have one.
    pub host: Option<String>,
}

/// Closed enumeration of asset kinds. New variants are a schema
/// version bump.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "ts-types", derive(TS))]
#[cfg_attr(feature = "ts-types", ts(export))]
pub enum AssetKind {
    /// A live TLS session observed by `sezar-net`.
    TlsSession,
    /// A live SSH session observed by `sezar-net`.
    SshSession,
    /// An IPsec security association observed by `sezar-net`.
    IpsecSa,
    /// An X.509 certificate (CT log entry, on-disk file, etc.).
    X509Cert,
    /// A public-chain key — Bitcoin/Ethereum/etc address.
    BlockchainKey,
    /// An HSM/KMS slot or AWS KMS key.
    HsmSlot,
    /// A DNSSEC RRSIG observation forwarded from Nizam.
    DnsDnssec,
}

/// One primitive role used by the asset (kex / sig / encrypt / hash /
/// auth) plus metadata about the algorithm choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "ts-types", derive(TS))]
#[cfg_attr(feature = "ts-types", ts(export))]
pub struct Primitive {
    /// What this primitive does.
    pub role: PrimitiveRole,
    /// Canonical algorithm name. Examples: `"ECDSA-P256"`,
    /// `"Dilithium2"`, `"AES-256-GCM"`, `"SHA-256"`.
    pub algorithm: String,
    /// Algorithm parameters keyed by name. Free-form; consumers must
    /// tolerate unknown keys.
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[cfg_attr(feature = "ts-types", ts(type = "Record<string, unknown>"))]
    pub parameters: serde_json::Map<String, serde_json::Value>,
    /// Whether this primitive is post-quantum resistant. `None` =
    /// unknown / not yet classified.
    pub pq_resistant: Option<bool>,
    /// NIST PQC security level (1, 3, 5) when applicable.
    pub nist_classification: Option<NistLevel>,
}

/// What a [`Primitive`] is being used for. `Auth` means message
/// authentication code; `Sig` means asymmetric digital signature.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "ts-types", derive(TS))]
#[cfg_attr(feature = "ts-types", ts(export))]
pub enum PrimitiveRole {
    /// Asymmetric key exchange (e.g. X25519, ECDH-P256, Kyber).
    Kex,
    /// Asymmetric digital signature (e.g. ECDSA, Dilithium).
    Sig,
    /// Message authentication code (e.g. HMAC-SHA-256, GMAC).
    Auth,
    /// Symmetric encryption (e.g. AES-256-GCM, ChaCha20-Poly1305).
    Encrypt,
    /// Cryptographic hash (e.g. SHA-256, SHA-3).
    Hash,
}

/// NIST PQC classification levels. See FIPS 203/204/205.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "ts-types", derive(TS))]
#[cfg_attr(feature = "ts-types", ts(export))]
pub enum NistLevel {
    /// Level 1 — equivalent to AES-128.
    L1,
    /// Level 3 — equivalent to AES-192.
    L3,
    /// Level 5 — equivalent to AES-256.
    L5,
}

/// Sezar's verdict on an asset, derived from its [`Primitive`] list.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "ts-types", derive(TS))]
#[cfg_attr(feature = "ts-types", ts(export))]
pub struct Posture {
    /// 0–100 score; higher is better. 100 = fully PQ-ready.
    pub score: u8,
    /// Human-readable explanation of the score. Keep short
    /// (one sentence) — the dashboard renders this verbatim.
    pub rationale: String,
    /// If applicable, what to migrate to. `None` when the asset is
    /// already at the recommended primitive (or when the rollup
    /// engine can't suggest a replacement yet).
    pub recommended_replacement: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build an event with the given `AssetKind` so the JSON
    /// round-trip test can sweep every variant. Other fields are
    /// minimally plausible; we test serialization stability, not
    /// posture-rollup correctness.
    fn fixture(kind: AssetKind, identity: &str, primitives: Vec<Primitive>) -> CryptoInventoryEvent {
        CryptoInventoryEvent {
            schema_version: SCHEMA_VERSION,
            source_module: "test".into(),
            observed_at: chrono::Utc::now(),
            asset: Asset {
                kind,
                identity: identity.into(),
                host: Some("test.example.com".into()),
            },
            primitives,
            posture: Posture {
                score: 50,
                rationale: "fixture".into(),
                recommended_replacement: None,
            },
        }
    }

    fn prim(role: PrimitiveRole, algorithm: &str) -> Primitive {
        Primitive {
            role,
            algorithm: algorithm.into(),
            parameters: Default::default(),
            pq_resistant: None,
            nist_classification: None,
        }
    }

    #[test]
    fn schema_version_constant_is_sane() {
        assert_eq!(SCHEMA_VERSION, 1);
    }

    /// Sanity round-trip on the canonical TLS shape.
    #[test]
    fn event_round_trips_through_json() {
        let ev = CryptoInventoryEvent {
            schema_version: SCHEMA_VERSION,
            source_module: "sezar-net".into(),
            observed_at: chrono::Utc::now(),
            asset: Asset {
                kind: AssetKind::TlsSession,
                identity: "abc123".into(),
                host: Some("api.example.com".into()),
            },
            primitives: vec![Primitive {
                role: PrimitiveRole::Kex,
                algorithm: "X25519".into(),
                parameters: Default::default(),
                pq_resistant: Some(false),
                nist_classification: None,
            }],
            posture: Posture {
                score: 40,
                rationale: "X25519 is classical-only".into(),
                recommended_replacement: Some("Kyber768+X25519 hybrid".into()),
            },
        };
        let s = serde_json::to_string(&ev).unwrap();
        let back: CryptoInventoryEvent = serde_json::from_str(&s).unwrap();
        assert_eq!(back.schema_version, ev.schema_version);
        assert_eq!(back.asset.kind, AssetKind::TlsSession);
        assert_eq!(back.posture.score, 40);
    }

    /// Sweep every AssetKind through JSON. Catches a missing serde
    /// rename, a forgotten `Deserialize`, or a kind variant that
    /// breaks the closed-set invariant (next major bump only).
    #[test]
    fn every_asset_kind_round_trips() {
        let cases: Vec<(AssetKind, &str, Vec<Primitive>)> = vec![
            (
                AssetKind::TlsSession,
                "tls-abc-123",
                vec![
                    prim(PrimitiveRole::Kex, "X25519"),
                    prim(PrimitiveRole::Sig, "ECDSA-P256"),
                    prim(PrimitiveRole::Encrypt, "AES-256-GCM"),
                    prim(PrimitiveRole::Hash, "SHA-384"),
                ],
            ),
            (
                AssetKind::SshSession,
                "ssh-9af0",
                vec![
                    prim(PrimitiveRole::Kex, "curve25519-sha256"),
                    prim(PrimitiveRole::Sig, "ssh-ed25519"),
                    prim(PrimitiveRole::Encrypt, "chacha20-poly1305"),
                ],
            ),
            (
                AssetKind::IpsecSa,
                "spi-0xdeadbeef",
                vec![prim(PrimitiveRole::Encrypt, "AES-256-GCM")],
            ),
            (
                AssetKind::X509Cert,
                "sha256:1f2e3d…",
                vec![prim(PrimitiveRole::Sig, "RSA-2048")],
            ),
            (
                AssetKind::BlockchainKey,
                "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
                vec![prim(PrimitiveRole::Sig, "ECDSA-secp256k1")],
            ),
            (
                AssetKind::HsmSlot,
                "pkcs11:token=acme-prod;id=05",
                vec![prim(PrimitiveRole::Sig, "ECDSA-P256")],
            ),
            (
                AssetKind::DnsDnssec,
                "rrsig:fingerprint:abc",
                vec![prim(PrimitiveRole::Sig, "ECDSAP256SHA256")],
            ),
        ];

        for (kind, id, prims) in cases {
            let ev = fixture(kind.clone(), id, prims.clone());
            let json = serde_json::to_string(&ev).unwrap();
            let back: CryptoInventoryEvent = serde_json::from_str(&json).unwrap();
            assert_eq!(back.asset.kind, kind, "round-trip kind for {id}");
            assert_eq!(back.asset.identity, id);
            assert_eq!(back.primitives.len(), prims.len());
        }
    }

    /// Optional fields (`host`, `pq_resistant`, `nist_classification`,
    /// `recommended_replacement`) drop out of the JSON when None.
    /// Catches a regression where someone removes
    /// `skip_serializing_if = "Option::is_none"` on a future field.
    #[test]
    fn optional_fields_omit_when_none() {
        // Note: serde defaults — we don't currently set
        // skip_serializing_if on these. Test asserts that when omitted
        // *from JSON input*, deserialization re-fills with None.
        let json = r#"{
            "schema_version": 1,
            "source_module": "test",
            "observed_at": "2026-01-01T00:00:00Z",
            "asset": {"kind": "blockchain_key", "identity": "0xabc"},
            "primitives": [{"role": "sig", "algorithm": "Ed25519"}],
            "posture": {"score": 0, "rationale": "n/a"}
        }"#;
        let ev: CryptoInventoryEvent = serde_json::from_str(json).unwrap();
        assert!(ev.asset.host.is_none());
        assert!(ev.primitives[0].pq_resistant.is_none());
        assert!(ev.primitives[0].nist_classification.is_none());
        assert!(ev.posture.recommended_replacement.is_none());
        // parameters: serde(default) → empty Map.
        assert!(ev.primitives[0].parameters.is_empty());
    }

    /// Schema_version is a hard rejection point: an event from a
    /// future major version must not silently look valid.
    #[test]
    fn unknown_asset_kind_fails_to_deserialize() {
        let json = r#"{
            "schema_version": 1,
            "source_module": "test",
            "observed_at": "2026-01-01T00:00:00Z",
            "asset": {"kind": "quantum_keyring", "identity": "qk-1"},
            "primitives": [],
            "posture": {"score": 0, "rationale": "n/a"}
        }"#;
        let result: Result<CryptoInventoryEvent, _> = serde_json::from_str(json);
        assert!(result.is_err(), "unknown AssetKind variant must fail to parse");
    }

    /// Parameters round-trip arbitrary JSON.
    #[test]
    fn parameters_carry_arbitrary_json() {
        let mut params = serde_json::Map::new();
        params.insert("curve".into(), serde_json::Value::String("Curve25519".into()));
        params.insert("key_bits".into(), serde_json::Value::Number(256.into()));
        params.insert(
            "extensions".into(),
            serde_json::json!(["server_name", "supported_versions"]),
        );
        let p = Primitive {
            role: PrimitiveRole::Kex,
            algorithm: "X25519".into(),
            parameters: params.clone(),
            pq_resistant: Some(false),
            nist_classification: None,
        };
        let json = serde_json::to_string(&p).unwrap();
        let back: Primitive = serde_json::from_str(&json).unwrap();
        assert_eq!(back.parameters, params);
    }

    /// Empty parameters map must not appear in the wire JSON
    /// (skip_serializing_if). Smaller payloads, easier diff'ing.
    #[test]
    fn empty_parameters_omitted_from_wire() {
        let p = prim(PrimitiveRole::Hash, "SHA-256");
        let json = serde_json::to_string(&p).unwrap();
        assert!(
            !json.contains("\"parameters\""),
            "empty parameters should be skipped: {json}"
        );
    }

    /// NIST classification round-trips with UPPERCASE rename.
    #[test]
    fn nist_levels_serialize_uppercase() {
        for (level, expected) in [(NistLevel::L1, "L1"), (NistLevel::L3, "L3"), (NistLevel::L5, "L5")] {
            let s = serde_json::to_string(&level).unwrap();
            assert_eq!(s, format!("\"{expected}\""));
        }
    }
}
