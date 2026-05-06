//! `sezar-core` — shared event schema + posture rollup library.
//!
//! Every Sezar module emits one type: [`CryptoInventoryEvent`]. The
//! schema is intentionally narrow — modules are not allowed to add
//! new top-level fields without a coordinated `schema_version` bump
//! across all consumers.
//!
//! See `docs/crypto-event-schema.md` at the repo root for the
//! field-by-field rationale.

#![deny(missing_docs)]

use serde::{Deserialize, Serialize};

/// Current event schema version. Bumped on any breaking change to the
/// top-level shape; additive changes are allowed without a bump as
/// long as they're optional.
pub const SCHEMA_VERSION: u32 = 1;

/// One observation about one crypto-bearing asset, normalised to a
/// shape that's identical regardless of whether it came from
/// `sezar-net`, `sezar-cert`, `sezar-chain`, or `sezar-id`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoInventoryEvent {
    /// Schema version of this event. Always `SCHEMA_VERSION` at
    /// emission; consumers may see older values during rolling
    /// upgrades.
    pub schema_version: u32,
    /// Which Sezar module produced the event.
    pub source_module: String,
    /// Wall-clock observation time (UTC).
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
pub struct Primitive {
    /// What this primitive does.
    pub role: PrimitiveRole,
    /// Canonical algorithm name. Examples: `"ECDSA-P256"`,
    /// `"Dilithium2"`, `"AES-256-GCM"`, `"SHA-256"`.
    pub algorithm: String,
    /// Algorithm parameters keyed by name. Free-form; consumers must
    /// tolerate unknown keys.
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
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

    #[test]
    fn schema_version_constant_is_sane() {
        assert_eq!(SCHEMA_VERSION, 1);
    }

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
}
