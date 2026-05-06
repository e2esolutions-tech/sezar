//! `sezar-chain` — public-chain crypto monitor.
//!
//! Reserved crate. Real implementation begins in V3 — see
//! `ROADMAP.md`. Initial chains will be Bitcoin (secp256k1 / ECDSA),
//! Ethereum L1 (secp256k1), and one PQ-native reference chain (QRL or
//! similar) to prove the schema doesn't fall over on hash-based
//! signatures.

/// Module identifier emitted as `source_module` on every event once
/// V3 lands.
pub const MODULE_NAME: &str = "sezar-chain";
