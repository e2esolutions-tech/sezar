# Sezar — Crypto-Posture Observability

> **Status:** pre-alpha. Repository scaffolding only. V1 (Network module)
> targeted for Q3 2026.

Sezar maps every cryptographic asset in your environment — TLS handshakes
on the wire, certificates on disk, signing keys in your HSM, **and even
crypto holdings on public blockchains** — to a single posture: how
quantum-resistant is each asset, and where do you have to migrate first?

It is **not** a DNS resolver, certificate authority, or blockchain node.
It is an observability layer that sits next to all of these and answers:

```
> Which of my crypto assets are still ECDSA / RSA-2048 / SHA-1?
> Which have already migrated to Dilithium / SPHINCS+ / hash-based?
> When the NIST 2030 deadline lands, what do I touch first?
```

## Why a separate product?

Sezar started life as backlog items on
[CortexDNS](https://github.com/e2esolutions-tech/cortexdns):

- *PQC algorithm tracking in DNSSEC visibility*
- *Network Crypto Posture roll-up*
- *Crypto-posture telemetry*

But these are not DNS problems. They span TLS termination, X.509 PKI,
SSH, IPsec, hardware tokens, and on-chain assets. Stuffing them into
Nizam (a single-binary DNS filter) would dilute that product. Spinning
them out as **Sezar** keeps both products honest:

- **Nizam** stays a focused DNS resolver + filter. It still does its
  share of crypto observability — DNSSEC validation, PQ-algorithm
  tagging on DNS responses — but only at the DNS layer.
- **Sezar** consumes those DNS-layer observations *plus* eBPF/agent
  feeds from every other crypto-bearing surface, normalises them into
  a single inventory, and answers the migration-readiness question.

## Module layout (planned)

| Module           | Purpose                                          | Data source                  |
|------------------|--------------------------------------------------|------------------------------|
| `sezar-net`      | TLS / SSH / IPsec ciphersuite observation        | eBPF + libpcap               |
| `sezar-cert`     | X.509 inventory, key sizes, signature algos      | CT logs, internal CA, host scan |
| `sezar-chain`    | Public-chain crypto: ECDSA/EdDSA/PQC adoption    | RPC / mempool sniffing       |
| `sezar-id`       | HSM / KMS / smart-card key inventory             | Vendor APIs                  |
| `sezar-core`     | Shared event schema + posture-rollup library     | (n/a — library)              |
| `sezar-server`   | Collector + REST API + dashboard backend         | (n/a — service)              |

V1 ships only `sezar-net` + `sezar-server` + a minimal UI. The rest are
stubs until later milestones.

## Architecture at a glance

```
┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│  sezar-net   │  │ sezar-cert   │  │ sezar-chain  │  │  sezar-id    │
│ (eBPF agent) │  │  (scanners)  │  │  (RPC/mp)    │  │ (HSM/KMS)    │
└──────┬───────┘  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘
       │                 │                 │                 │
       │     normalised  │   crypto_inventory_event v1       │
       │                 ▼                 ▼                 │
       │            ┌─────────────────────────────┐          │
       └───────────►│       sezar-server          │◄─────────┘
                    │  (axum collector + UI API)  │
                    └────────────┬────────────────┘
                                 │
                              ┌──┴──┐
                              │ DB  │  (Postgres for config,
                              │     │   columnar later for events)
                              └─────┘
                                 │
                          ┌──────┴──────┐
                          │  Sezar UI   │  posture dashboard,
                          │  (React)    │  migration roadmap
                          └─────────────┘
```

## Repository layout

```
sezar/
├── Cargo.toml                # workspace
├── crates/
│   ├── sezar-core/           # event schema + rollup library (V1)
│   ├── sezar-server/         # axum collector + REST API     (V1)
│   ├── sezar-net/            # TLS/SSH eBPF agent            (V1)
│   ├── sezar-cert/           # cert inventory                (V2)
│   ├── sezar-chain/          # blockchain crypto monitor     (V3)
│   └── sezar-id/             # HSM/KMS inventory             (V4)
├── docs/
│   ├── architecture.md
│   ├── crypto-event-schema.md
│   ├── module-net.md
│   ├── module-chain.md
│   └── posture-rollup.md
├── web/                      # React + Vite UI
└── docker-compose.yml
```

## License

MIT — same as Nizam, encourages downstream adoption + integrations.

## Related projects

- [CortexDNS](https://github.com/e2esolutions-tech/cortexdns) — DNS
  security platform. Will surface its DNSSEC observations through
  Sezar's collector once V1 lands.
- [Nizam](https://github.com/e2esolutions-tech/nizam) — DNS filter
  engine + DNSSEC validator. Same relationship as above.

## Getting involved

Roadmap + V1 backlog tracked in [ROADMAP.md](./ROADMAP.md) and the
GitHub Issues / Milestones for this repo. **No code yet** — the V1
crates contain only stubs. The smallest meaningful contribution today
is reading the architecture doc and filing an issue with feedback on
the event schema.
