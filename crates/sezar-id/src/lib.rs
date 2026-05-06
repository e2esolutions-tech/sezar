//! `sezar-id` — HSM / KMS / smart-card inventory.
//!
//! Reserved crate. Real implementation begins in V4 — see
//! `ROADMAP.md`. Adapters: PKCS#11, AWS KMS, GCP KMS, Azure Key
//! Vault, YubiHSM 2, PIV-compatible smart cards.

/// Module identifier emitted as `source_module` on every event once
/// V4 lands.
pub const MODULE_NAME: &str = "sezar-id";
