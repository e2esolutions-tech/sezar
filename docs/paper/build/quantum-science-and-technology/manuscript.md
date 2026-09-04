---
title: "Three Axes of Quantum Risk: A Unified Observability Model for PQC, QKD, and Crypto-Agility"
author:
  - name: "Aleaddin Özer"
    affiliation: "E2E Solutions"
    role: "Chief System Engineer"
    email: "ozer@e2esolutions.tech"
    orcid: "0000-0001-9389-5357"
  - name: "Murat Aydos"
    affiliation: "Hacettepe University"
    role: "Associate Professor"
    orcid: "0000-0002-7570-9204"
date: 2026-08-29
keywords:
  - post-quantum cryptography
  - quantum key distribution
  - crypto-agility
  - observability
  - cryptographic posture
  - NIST PQC standardization
abstract: |
  The post-quantum migration has moved from research to
  operations. NIST has finalised FIPS 203/204/205, the NSA's
  CNSA 2.0 timetable runs to 2033, and Quantum Key Distribution
  is being deployed on high-assurance links under ETSI
  GS QKD 014. Yet most cryptographic-discovery tools answer a
  single question for each asset: is its primitive PQ-resistant?
  Two assets identical on that axis can have very different
  channel protections, and very different migration costs, and
  current tooling sees neither.

  We treat quantum-risk posture as a three-axis observation
  problem. Each asset is graded on its algorithmic resistance
  (A), the channel through which its keys are delivered (C),
  and how quickly its primitive can be replaced (G), and the
  three are folded into a deadline-adjusted score
  $q(\mathit{asset}, t)$ whose agility weight shrinks as the
  operator's deadline approaches. To make the model
  observable, we extend the open `crypto_inventory_event` v1
  schema with two new fields, build a reference
  implementation (ree0xQ, formerly *Sezar*) covering eBPF-based TLS observation,
  an ETSI GS QKD 014 collector with a working Key-Management
  -Entity emulator, and a static crypto-agility scanner, and
  evaluate the result against three reproducible studies: a
  Tranco-top-1k TLS handshake survey, a controlled
  emulator-based study of 13 induced QKD link/KME state
  transitions across five failure scenarios, and an
  11-project pilot against a hand-graded crypto-agility
  ground-truth corpus.

  The contributions are the model itself, the schema
  extension, the open emulator and replay corpus, a published
  agility-scoring rubric backed by a Semgrep rule pack, and
  an empirical baseline grading real-world systems on all
  three axes from a single open release.
---

# 1. Introduction

The cryptography deployed on the open internet today was designed under
assumptions that will not survive the next decade of computing.
Shor's algorithm on a sufficiently large fault-tolerant quantum
computer breaks RSA, finite-field Diffie–Hellman, and every
deployed-at-scale elliptic curve scheme in polynomial time
[@shor1997]. Grover's algorithm halves the effective key length of
symmetric primitives [@grover1996]. The exact arrival date of a
*cryptographically relevant quantum computer* (CRQC) remains
contested, but two operational facts are no longer in doubt. First,
adversaries can record encrypted traffic now and decrypt it later
once a CRQC is available — the *harvest-now, decrypt-later* threat
[@nsa-cnsa2; @nist-ir8547]. Second, the standards bodies have moved:
NIST finalized FIPS 203 (ML-KEM, née Kyber), FIPS 204 (ML-DSA, née
Dilithium), and FIPS 205 (SLH-DSA, née SPHINCS+) in 2024
[@fips203; @fips204; @fips205]. The U.S. NSA's CNSA 2.0 suite gives
calendar deadlines: software/firmware signing in PQC by 2025,
browsers and servers by 2030, network equipment by 2030, with
operating systems and other classes by 2033 [@nsa-cnsa2]. The U.K.,
EU, and ANSSI guidance follows similar curves [@ncsc-pqc; @anssi-pqc].

This places operators in an unfamiliar position. They must inventory,
classify, and migrate cryptographic assets across networks they did
not entirely design, on a deadline they did not set, using algorithms
that did not exist in deployable form three years ago. Tools for the
first step — inventory and classification — are appearing
[@cisco-pqc-discovery; @ibm-pqc-discovery; @microsoft-cryptotracker;
@cloudflare-pq-deploy], and standards bodies have begun publishing
guidance on which observables matter [@nist-ir8547; @bsi-migration].

In parallel, a hardware-rooted track has matured. Quantum Key
Distribution (QKD), proposed in 1984 [@bb84] and matured through the
SECOQC, Tokyo, and EuroQCI testbeds [@secoqc; @tokyo-qkd; @euroqci],
delivers symmetric key material through a channel whose security
reduces to physics rather than computational assumptions. ETSI's
*GS QKD 014* specification standardized the REST interface between
QKD key-management entities (KMEs) and the applications that consume
their keys [@etsi-qkd-014]. Commercial deployments exist in
finance, government, and metropolitan-fiber settings
[@idq-deployments; @toshiba-qkd; @qti-qkd].

Industrial and academic literature treats these two tracks — PQC and
QKD — as alternatives. Either you migrate your algorithms (PQC), or
you install QKD links on high-assurance segments, with the
implicit understanding that QKD's hardware cost and distance limits
make it inappropriate for general use [@nsa-qkd-position;
@anssi-qkd]. That framing collapses too soon. For an operator
trying to know what risks are present in their environment on
any given day, PQC and QKD are *complementary observables*, not
alternatives. An asset using ECDSA-P256 over a fiber link
protected by a Toshiba MU-QKD3 system is in a materially different
posture than an asset using ECDSA-P256 over an unprotected wide-area
link, even though their algorithms are identical. A unified
observability model must capture both.

A complete quantum-risk model has to include a third axis that
current tooling almost entirely ignores: *crypto-agility*. An asset using a classical algorithm today but whose
algorithm is selected at runtime by a server configuration field is
in a very different posture than an asset whose algorithm is
hard-coded into firmware that ships from a vendor and rotates only
on hardware refresh. The first is a configuration change; the second
is a hardware-replacement program. The NIST PQ-migration playbook
already names crypto-agility as a precondition for orderly migration
[@nist-ir8547; @rfc7696], yet no production observability tool we
have seen surfaces it as a first-class telemetry axis. Without it,
operators answering "are we PQ-ready?" answer a much easier
question than "can we *become* PQ-ready in time?"

We argue for, and implement, a **three-axis posture model**:

- **Axis A — Algorithmic Resistance.** Is the primitive itself
  quantum-resistant under standard assumptions?
- **Axis C — Channel Protection.** Is the key material protected by
  a quantum-secure delivery mechanism (QKD, hybrid PSK derived from
  QKD)?
- **Axis G — Migration Agility.** How quickly can axis A be changed
  on this asset — by configuration, library upgrade, or hardware
  replacement?

We collapse the three into a single deadline-adjusted quantum-risk
score $q(\mathit{asset}, t)$ suitable for dashboard display, alert
thresholds, and inter-organization comparison. We extend the open
ree0xQ `crypto_inventory_event v1` schema with the additional axes,
implement five agents that emit the same event shape, and
evaluate the system through three empirical studies any
practitioner can replicate on a Linux host with a public Internet
connection.

## 1.1 Contributions

The model side. We formalise the three axes — algorithmic
resistance, channel protection, migration agility — on
defined scales and define the deadline-adjusted scoring
function $q(\mathit{asset}, t)$. The agility weight shrinks
as the deadline approaches; the orthogonal `BLOCKED` flag
catches what $q$ alone does not. Details in §5.

The schema side. `crypto_inventory_event v1.1` adds two
top-level fields (`channel_protection`, `agility`) and two
new asset kinds (`qkd_link`, `qkd_kme`). The extensions are
strictly additive; v1.0 consumers ignore them safely. §6.

The implementation side. ree0xQ ships as five cooperating
agents (network, certificate, blockchain, key-management,
QKD), a shared rollup library, and a collector / dashboard.
A single Rust workspace hosts wire-level eBPF observation,
REST-based QKD telemetry collection, and static
crypto-agility analysis under one event shape. §7 walks
through the implementation choices that made the unified
shape possible.

A working QKD test rig. `ree0xq-qkd-kme-emulator` is an open
ETSI GS QKD 014 v1.1.1 Key Delivery implementation backed by
a synthetic key generator, configurable QBER, link
state-change replay scenarios, and a documented capture
format. Practitioners without QKD hardware can drive and
test QKD-aware software against it. §7.2, §8.2.

An auditable agility rubric. The five-level ordinal scale
(Negotiated / Configurable / Pinned / Locked / Frozen) is
derived from RFC 7696 and operationalised through a Semgrep
rule pack that runs against source code or installed
packages. The hand-graded ground-truth corpus of fifty
open-source server projects ships alongside the ruleset.
§7.3, §8.3.

An empirical baseline. We report A, C, G scores for the
Tranco-top-1k over TLS, for a controlled multi-KME ETSI 014
testbed exercising representative failure modes, and for
the 11-project pilot drawn from the 50-project agility
corpus. Scaling the pilot to the full corpus is the
remaining mechanical step. §8.

## 1.2 Non-goals

This paper does not propose a new PQC primitive, a new QKD
protocol, or a new key-management protocol. It does not argue that
QKD should replace PQC, or vice versa; we take the standards-body
position that hybrid deployments are the realistic operating mode
and instrument both observables. We do not claim to inventory all
deployed cryptography on Earth; our empirical work is bounded by
the corpora we publish. We do not address adversary modeling
beyond accepting the harvest-now/decrypt-later assumption.

## 1.3 Roadmap

§2 reviews the standards landscape (PQC, QKD, crypto-agility) at the
level required to make the paper self-contained. §3 surveys related
observability work and identifies the gap. §4 states the threat
model and operator assumptions. §5 defines the three-axis posture
model. §6 specifies the event schema. §7 presents the ree0xQ
reference implementation. §8 reports the empirical evaluation. §9
discusses limitations, ethics, and deployment guidance. §10
concludes.

---

# 2. Background

## 2.1 Post-quantum cryptography standardization

NIST initiated its post-quantum cryptography standardization process
in 2016 [@nist-pqc-call] and finalized the first three production
standards in August 2024 [@fips203; @fips204; @fips205]. These are:

- **FIPS 203 — ML-KEM** (formerly CRYSTALS-Kyber), a module-lattice
  key-encapsulation mechanism at three parameter sets corresponding
  to NIST security categories 1, 3, and 5.
- **FIPS 204 — ML-DSA** (formerly CRYSTALS-Dilithium), a
  module-lattice digital signature scheme also at three parameter
  sets.
- **FIPS 205 — SLH-DSA** (formerly SPHINCS+), a stateless
  hash-based digital signature scheme, with both small and fast
  parameter sets across SHA2 and SHAKE hashes.

A fourth standard for a fast falcon-style lattice signature
(FN-DSA, formerly Falcon) is in advanced draft [@nist-fn-dsa]. NIST
has additionally signaled that further KEM candidates will be
standardized to diversify mathematical assumptions
[@nist-pqc-onramp]. The U.S. NSA's *Commercial National Security
Algorithm Suite 2.0* (CNSA 2.0) identifies ML-KEM-1024, ML-DSA-87,
SLH-DSA-SHA2-256s, AES-256-GCM, and SHA-384 / SHA-512 as the
required suite for national-security systems by milestones falling
between 2025 and 2033 depending on equipment class [@nsa-cnsa2].
The German BSI, French ANSSI, and U.K. NCSC have published
equivalent guidance with broadly similar timelines and a shared
recommendation to deploy *hybrid* (classical + PQC) key exchange
during the transition window [@bsi-migration; @anssi-pqc; @ncsc-pqc].

The deployment story is mixed. Chrome and Firefox have shipped
X25519MLKEM768 hybrid key exchange in TLS 1.3 since 2024
[@chrome-pq-tls; @firefox-pq-tls]. Cloudflare reported
roughly two percent of TLS 1.3 connections to its edge
were post-quantum-protected in early 2024
[@cloudflare-pq-deploy]; by late 2025 the majority of
human-initiated traffic to the same edge was hybrid-PQ
[@pq-tls-measurement].
Linux kernel and OpenSSH support is staged across recent releases.
Certificate authorities have piloted ML-DSA signing in test
hierarchies but have not yet issued production-trust-anchor PQ
certificates [@digicert-pq]. The pattern is consistent across
operator surveys: the algorithms exist, library support is
materializing, but the long tail of embedded, appliance, and legacy
software remains unaddressed [@cisco-pqc-discovery; @ibm-pqc-discovery].

## 2.2 Quantum key distribution and ETSI GS QKD 014

Quantum key distribution, in its prepare-and-measure form, was
introduced by Bennett and Brassard in 1984 [@bb84] and has been
deployed in operational testbeds for two decades
[@secoqc; @tokyo-qkd]. A QKD system distributes symmetric key
material between two endpoints using single-photon (or weak-coherent)
quantum states, with security guarantees derived from the no-cloning
theorem and the disturbance of measurement. Modern systems achieve
metropolitan ranges (≤100 km dark fiber) at key rates from kilobits
to megabits per second [@toshiba-qkd; @idq-deployments]. Trusted-node
relaying, satellite QKD [@micius], and twin-field protocols
[@twin-field] extend the reach, though at the cost of additional
assumptions.

QKD's controversial status in the policy community deserves brief
treatment because it informs our observability design choices. The
U.S. NSA has stated that QKD is not a replacement for PQC for
national-security systems, citing implementation-attack surface,
limited authentication coverage, and operational complexity
[@nsa-qkd-position]. The French ANSSI takes a similar position
[@anssi-qkd]. In contrast, the U.K., German, Italian, and Chinese
research and standards communities continue to invest in QKD
infrastructure, including the EU's EuroQCI program [@euroqci] and
multiple national QKD networks. The pragmatic operator view, which
this paper adopts, is that QKD will be deployed on a subset of
high-assurance links (financial inter-datacenter, intelligence,
government inter-site) regardless of which policy community is
"correct," and that the deployment will be heterogeneous,
vendor-specific, and operationally opaque without explicit
observability tooling.

ETSI's *GS QKD 014 v1.1.1* specification (2019) standardizes the
REST interface between a Key Management Entity (KME) — the device
that holds key material produced by the underlying QKD link — and
the applications consuming those keys (Secure Application Entities,
SAEs) [@etsi-qkd-014]. The interface defines three operations:

- `GET /api/v1/keys/{slave_SAE_ID}/status` returns KME and link
  status: current key rate, number of stored keys, supported key
  sizes.
- `GET /api/v1/keys/{slave_SAE_ID}/enc_keys` returns one or more
  fresh keys from the master KME, identified by UUIDs.
- `POST /api/v1/keys/{master_SAE_ID}/dec_keys` retrieves the
  matching keys at the slave KME, by UUID.

The specification is deliberately silent on what an SAE does with
the keys; in practice, SAEs use them as pre-shared keys (PSK) for
IPsec, MACsec, or — increasingly — as the symmetric secret in a
hybrid TLS PSK mode [@hybrid-tls-psk; @nist-sp-1800-38a]. From an
observability standpoint, ETSI 014 gives us a stable, vendor-neutral
surface on which to observe QKD-protected channels: link status,
key rate, request volume, error rates, and (with cooperating SAE
instrumentation) which downstream sessions consume the keys.

## 2.3 Crypto-agility

Crypto-agility — the property that allows a system to change
cryptographic primitives without invasive redesign — was codified
operationally by RFC 7696 [@rfc7696] and is repeatedly named in
PQ-migration guidance as a precondition for orderly transition
[@nist-ir8547; @bsi-migration; @ncsc-pqc]. The literature describes
crypto-agility along three dimensions: *protocol* agility (TLS 1.3
negotiates ciphersuites; IPsec IKEv2 negotiates transforms), *code*
agility (a library exposes algorithm choice as a parameter, not a
recompile-time decision), and *deployment* agility (operators can
roll out a new algorithm without firmware replacement or
trust-anchor reissuance).

In practice, agility varies dramatically across asset classes. A
modern Web server typically negotiates its TLS ciphersuite on every
handshake; a hardware HSM may lock its supported key types at the
firmware level; embedded devices in industrial or medical settings
often have a fixed algorithm chosen at the silicon level. The
emergence of FIPS 140-3 validation, with its requirement that
algorithm changes trigger revalidation, introduces a second axis of
*locked* status that any regulated environment has to plan
around [@fips140-3].

Despite repeated standards-body emphasis on crypto-agility, the
practitioner picture is poor. No widely deployed inventory tool we
have seen reports crypto-agility as a first-class field. The
question "can this asset migrate?" is repeatedly answered
informally — by tribal knowledge, vendor questionnaires, or
case-by-case engineering audits.

---

# 3. Related work

We organize related work along three threads — PQC discovery
tooling, QKD telemetry, and crypto-agility analysis — and conclude
with the gap statement that motivates the present paper.

## 3.1 PQC discovery and migration tooling

Cisco's PQC Discovery service [@cisco-pqc-discovery] is, to our
knowledge, the most operationally mature PQ inventory tool deployed
today. It combines passive network observation (NetFlow-derived
context with optional inline TLS handshake inspection) with active
endpoint scanning and a managed-service operator dashboard. The
analytic surface is algorithm-class oriented: assets are labeled
*PQ-ready*, *classical*, or *unknown* based on observed handshakes
and reported algorithm support. The service does not surface
channel-protection or agility as separate dimensions.

IBM's *Quantum-Safe Discover* [@ibm-pqc-discovery] and Microsoft's
internal *CryptoTracker* tooling [@microsoft-cryptotracker] follow
similar patterns: enumerate cryptographic uses across an
environment, classify by algorithm against a PQC-readiness
yardstick, prioritize migration. Cloudflare's published reports
[@cloudflare-pq-deploy] focus on edge-observable TLS handshake
mixtures and provide one of the few public datasets on PQ adoption
on the open internet.

The academic literature includes systematic measurements of TLS
deployment hygiene (key sizes, cipher choices, validation behavior)
[@durumeric-tls; @holz-tls; @felt-tls] and more recent measurements
of hybrid PQ deployment in the wild [@pq-tls-measurement]. NIST
IR 8547 [@nist-ir8547] provides the most authoritative migration
playbook and includes a discovery section, but stops short of
prescribing telemetry semantics.

Common across this thread: algorithm-class is treated as the
primary axis, hybrid/QKD is at most a footnote, and agility is
discussed in prose but not surfaced as a measured field.

## 3.2 QKD telemetry

QKD telemetry has been studied primarily from the perspective of
the QKD operator — the entity running the physical link and the
KMEs. The ETSI ISG-QKD has produced multiple documents on
operational and security testing
[@etsi-qkd-002; @etsi-qkd-008; @etsi-qkd-011]; the SECOQC
[@secoqc] and Tokyo [@tokyo-qkd] testbeds published detailed
operational data. The EuroQCI program [@euroqci] is in the process
of standardizing inter-domain QKD telemetry.

This thread, however, is *internal* to the QKD operator. For
an operator using QKD keys to protect application traffic — the
SAE/application perspective — the public literature is much
thinner. ETSI GS QKD 014 [@etsi-qkd-014]
standardizes the SAE-facing interface; we are not aware of an
existing open-source SAE-side observability platform that consumes
014 telemetry and integrates it with a broader cryptographic
inventory.

## 3.3 Crypto-agility analysis

RFC 7696 [@rfc7696] is the canonical statement. Subsequent
work has analyzed agility properties of individual protocols
(TLS, IPsec, S/MIME) and proposed agility frameworks for
specific domains (industrial control, embedded), though
without operationalising them as a continuous telemetry
input.

In the practitioner space, NCC Group has published audit reports
on cryptographic-agility deficiencies in deployed products
[@ncc-group-crypto-audits]. The OWASP and CIS communities maintain
checklists [@owasp-crypto], but none of this work, to our
knowledge, has been operationalized as a continuous telemetry
input feeding a cryptographic-posture dashboard.

## 3.4 Gap statement

Each thread is mature in isolation. What is missing — and
what ree0xQ fills — is a tool that treats algorithmic
resistance, channel protection, and migration agility as
three independent telemetry axes, rolls them into a single
deadline-adjusted score from a shared event schema, and is
reproducible without commercial tooling.

---

# 4. Threat model and operator assumptions

We assume a *Q-day* adversary: a sufficiently capable
fault-tolerant quantum computer capable of executing Shor's
algorithm on RSA-2048-class moduli and elliptic-curve groups of
practical size. The adversary's *capability date* is unknown but
treated as a moving deadline; the standards-body consensus places
the planning horizon between 2030 and 2040 [@nist-ir8547;
@nsa-cnsa2; @ncsc-pqc]. Following the harvest-now/decrypt-later
threat model [@nsa-cnsa2], we assume that an adversary records
classical-encrypted traffic *today* and decrypts it *later* once
the CRQC is available. Consequently, traffic encrypted with
classical-only KEMs is at risk *now*, not at Q-day.

We assume an operator who:

- Controls or has visibility into the cryptographic surfaces of
  their environment (TLS termination points, certificate inventory,
  application source code, host configuration, QKD links if any).
- Operates under a calendar deadline (NIST CNSA 2.0 milestones,
  regulatory mandates, or contractual obligations) that is fixed,
  even if individual subdeadlines are uncertain.
- Cannot replace all cryptography at once; migration is staged
  over months to years.
- Tolerates and benefits from continuous telemetry rather than
  periodic audits.

We assume an adversary who:

- Records traffic now, decrypts later.
- Cannot break PQ-secure primitives at standard parameter sizes
  (we accept the NIST standardization process's hardness
  assumptions).
- Cannot break the no-cloning theorem (we accept the standard QKD
  security argument); but may exploit implementation-level QKD
  vulnerabilities — detector blinding, side channels, classical
  authentication failure — and so QKD protection is observed
  *probabilistically* by the SAE, not guaranteed.
- May exploit non-cryptographic vulnerabilities; this paper does
  not address those.

The threat model justifies treating agility as a *risk multiplier*:
an asset with low algorithmic resistance but high agility may not
be at risk in *real* terms because it can be migrated before the
deadline horizon. An asset with low resistance and low agility
(e.g., a hardware appliance with hard-coded ECDSA on a five-year
refresh cycle) is at *significantly higher* real risk, even if
its observed primitive is identical to the agile case.

---

# 5. The three-axis quantum-risk posture model

We define three independent axes — *A*, *C*, *G* — and a unified
deadline-adjusted quantum-risk score $q(\mathit{asset}, t)$.
Figure 1 plots the space these axes span, with the four
worked-example assets at their observed coordinates.

![**Figure 1.** The three-axis quantum-risk space. Each
cryptographic asset occupies a point in $(A, C, G)$; colour
encodes the deadline-adjusted score $q$. The four worked-example
assets (α, β, γ, δ) are plotted at their observed coordinates.
The shaded region near the $A=0, G=0$ corner is the
`BLOCKED`-flagged volume requiring out-of-band remediation.](figures/three-axis-cube.pdf){#fig:cube width=85%}

## 5.1 Axis A — Algorithmic resistance

Axis $A$ scores the cryptographic primitives observed on an asset
on the scale $[0, 1]$, where 0 corresponds to a primitive that is
quantum-trivial (a deprecated or known-broken algorithm) and 1
corresponds to a primitive that is quantum-resistant under the
standard NIST hardness assumptions.

Following the existing ree0xQ V1 rollup (defined in
`docs/posture-rollup.md` in the ree0xQ repository), we classify
each primitive into one of five categories (Table 1):

| Category | $a$ value | Examples |
|--------------|-----------|----------|
| `pq` | 1.0 | ML-KEM-{512,768,1024}, ML-DSA-{44,65,87}, SLH-DSA-*, AES-256-GCM, SHA-256 |
| `pq_hybrid` | 0.9 | X25519+ML-KEM-768, ECDH-P256+ML-KEM-768 |
| `classical` | 0.3 | X25519, Ed25519, RSA-2048, ECDSA-P256, AES-128-GCM |
| `unknown` | 0.4 | Anything not in the classification table |
| `deprecated` | 0.0 | SHA-1, MD5, RSA-1024, RC4, 3DES, DH-1024 |

: Axis A — primitive categories and their $a$ values.

For an asset with observed primitives $\{p_i\}$ at roles $\{r_i\}$
with role weights $w_{r_i}$ summing to 1, the axis-A score is

$$
A(\mathit{asset}) = \sum_i w_{r_i} \cdot a(p_i)
$$

Role weights are calibrated to harvest-now/decrypt-later risk:
$w_\text{sig}=0.40, w_\text{kex}=0.30, w_\text{encrypt}=0.20,
w_\text{hash}=0.10$ for assets exhibiting all four roles, with
re-normalization when only a subset is present. The schema
also defines a fifth role, `auth` — the MAC primitive in
protocols where it is independent of the AEAD (SSH, IPsec-AH,
TLS 1.2 ciphersuites with separate HMAC). For those assets we
fold the `auth` weight into $w_\text{encrypt}$ at observation
time; the role exists in the schema so non-TLS-1.3 protocols
can be represented without forcing AEAD semantics onto them.

## 5.2 Axis C — Channel protection

Axis $C$ scores the channel through which key material reaches the
endpoint, on the scale $[0, 1]$, where 0 corresponds to a channel
with no quantum-secure key delivery and 1 corresponds to a channel
deriving its session key entirely from QKD-delivered material.
Three categorical states cover the realistic deployment options (Table 2):

| State | $c$ value | Description |
|------------------|-----------|-------------|
| `classical` | 0.0 | Session key derived solely from the negotiated KEM. |
| `qkd_hybrid_psk` | 0.7 | Session key derived from QKD-PSK XOR negotiated KEM (NIST SP 1800-38A pattern, ETSI 014 SAE). |
| `qkd_only` | 1.0 | Session key derived from QKD material alone (rare; MACsec-style transport). |

: Axis C — channel-protection states and their $c$ values.

Sub-states allow partial credit when telemetry indicates a QKD
link is *degraded* — high QBER, sustained KME unavailability, low
key rate — but the SAE has not failed over to classical. We
discount the score in proportion to the observed degradation;
formal degradation thresholds are deferred to the implementation
section (§7.2).

A subtle but important observation: the SAE may *think* it is
operating in QKD-hybrid mode while the underlying KME is failing
gracefully to a classical fallback. ree0xQ's role is to observe
both layers and surface the discrepancy. We define $c$ on observed
ETSI 014 status, not on SAE-reported intent.

## 5.3 Axis G — Migration agility

Axis $G$ scores the asset's ability to migrate its primitives on
the scale $[0, 1]$, where 0 corresponds to an asset whose
primitives can be changed only by physical replacement and 1
corresponds to an asset that negotiates primitives on every
session. We define five ordinal levels (Table 3):

| Level | $g$ value | Definition (observable signature) |
|----------------|-----------|------------------------------------|
| `negotiated` | 1.0 | Algorithm selected per-session by protocol negotiation. (TLS 1.3 server, modern SSH server, IKEv2 responder.) |
| `configurable` | 0.75 | Algorithm fixed per-deployment but changeable by configuration without code change. (Library config file, environment variable.) |
| `pinned` | 0.50 | Algorithm fixed in code; changeable by software upgrade. (Hard-coded algorithm name in application source.) |
| `locked` | 0.20 | Algorithm fixed in firmware or by FIPS/compliance binding; changeable only by vendor update or revalidation cycle. (Embedded firmware crypto, FIPS 140-3 validated module under tested-configuration constraint.) |
| `frozen` | 0.0 | Algorithm fixed in silicon, ROM, or otherwise unchangeable without hardware replacement. (TPM 1.2 with hard-coded RSA-2048; smart-card hard-wired ECDSA-P256.) |

: Axis G — migration-agility levels and their observable signatures.

The classification is derived from static analysis of the asset's
implementation surface plus, where available, vendor declarations
of FIPS validation scope. We detail the scanning methodology in
§7.3 and the scoring rubric in §8.3.

## 5.4 Unified deadline-adjusted quantum risk

The three axes are independent observables. To collapse them into
a single posture metric we adopt the operator's deadline as a
fourth input.

Let $D$ be the operator-configured deadline (e.g., 2030-01-01 for
NSA CNSA 2.0 browser/server class). Let $t$ be the current date.
Define the *deadline tension* $\tau(t) = \max(0,
\min(1, 1 - (D-t)/H))$, where $H$ is a horizon constant (we use
five years by default). When $D$ is far in the future, $\tau \to 0$
and agility forgives lower algorithmic resistance. As $t \to D$,
$\tau \to 1$ and agility no longer compensates because there is
insufficient time to migrate.

We define the quantum-risk score as

$$
q(\mathit{asset}, t) = 1 - \Bigl( \alpha \cdot A + \beta \cdot C +
  \gamma(\tau) \cdot G \Bigr)
$$

with $\alpha + \beta + \gamma(\tau) = 1$. Default weights:
$\alpha = 0.5, \beta = 0.2, \gamma(\tau) = 0.3 \cdot (1-\tau)$,
re-normalized when $\gamma$ shrinks. The agility weight is the
only weight that shrinks with deadline tension: an asset with
high agility but classical algorithms looks safe today (because
$\gamma$ is large) and looks increasingly unsafe as the deadline
approaches (because $\gamma$ shrinks toward zero). This is the
intended behavior: agility is forgiving *now*, not *at the
deadline*. Figure 2 traces $q(t)$ for the four worked-example
assets from today to the deadline, holding the observables
fixed.

![**Figure 2.** Trajectory of $q(t)$ for the four worked-example
assets from 2026 to the deadline at 2030-01-01, holding
observables fixed. The legacy-pinned asset (γ) climbs steepest;
the modern-agile asset (α) rises across the $q > 0.6$
must-migrate threshold as its agility weight erodes. The
locked-but-modern asset (β) remains high throughout — the
`BLOCKED` flag, not the prioritization score, is what surfaces
it for action.](figures/q-trajectory.pdf){#fig:trajectory width=85%}

Asset-class weights $w_k$ further weight the asset's contribution
to the org-wide posture, with `blockchain_key` weighted higher
than `tls_session` (a forged signature against a public-chain key
is permanent; a forged session is ephemeral).

## 5.5 Interpretation and bounds

The score $q \in [0, 1]$, with 0 = posture is fully aligned with
the deadline and 1 = posture is maximally exposed. Operators
typically configure alert thresholds at $q > 0.6$ ("must migrate")
and $q > 0.3$ ("plan migration").

Two edge cases warrant note. First, a fully PQ asset on a
non-QKD-protected channel scores $q = 1 - (0.5 \cdot 1 + 0.2 \cdot
0 + 0.3 \cdot g)$, which is approximately 0.2 for a fully agile
asset and 0.5 for a fully frozen asset, both before deadline
tension. This is intentional: QKD is a *bonus* on PQ-protected
sessions, not a requirement. Second, a fully classical asset on
a QKD-protected channel ($A=0.3$, $C=0.7$, $g=0$) scores
$q = 1 - (0.5 \cdot 0.3 + 0.2 \cdot 0.7 + 0.3 \cdot 0)
\approx 0.71$, down from 0.85 on the same asset over a
classical channel — QKD partially compensates for classical
algorithms, reflecting the real-world deployment of QKD on
high-assurance links.

---

# 6. Event schema extensions

We extend the ree0xQ `crypto_inventory_event v1` schema (defined
in `docs/crypto-event-schema.md` in the ree0xQ repository) with
additive, non-breaking fields. Existing consumers that ignore
unknown top-level fields continue to function; consumers that
opt in to v1.1 gain access to channel-protection and agility
observables.

## 6.1 New top-level fields

```json
{
  "schema_version": 1,
  "schema_minor": 1,
  "source_module": "ree0xq-net",
  "observed_at": "2026-08-15T11:42:03.421Z",
  "asset": { ... },
  "primitives": [ ... ],
  "channel_protection": { ... },   // NEW in 1.1
  "agility": { ... },              // NEW in 1.1
  "posture": { ... }
}
```

`schema_minor` is the first additive use of a minor-version
field. Consumers must accept any `schema_minor` ≥ their compiled
value and treat unknown top-level fields as opaque.

## 6.2 `channel_protection`

```json
{
  "state": "qkd_hybrid_psk",
  "kme_endpoint": "https://kme-1.dc.example/api/v1",
  "key_id_observed": "9c45e0a2-...",
  "psk_age_seconds": 47,
  "link_qber": 0.018,
  "link_key_rate_bps": 12480,
  "link_health": "ok",
  "degraded_reason": null
}
```

Table 4 lists these fields.

| Field | Type | Notes |
|----------------------|--------|-------|
| `state` | enum | `classical` / `qkd_hybrid_psk` / `qkd_only`. |
| `kme_endpoint` | string | ETSI 014 base URL. Omitted when state = `classical`. |
| `key_id_observed` | string | UUID of the consumed key, when reported by the cooperating SAE. |
| `psk_age_seconds` | int | Age of the PSK when the session began. |
| `link_qber` | float | Quantum bit error rate (0–1). |
| `link_key_rate_bps` | int | Average key generation rate over the prior minute. |
| `link_health` | enum | `ok` / `degraded` / `failed`. |
| `degraded_reason` | string | One-sentence reason when degraded; `null` otherwise. |

: Fields of the `channel_protection` block.

The fields are populated by `ree0xq-qkd`, which polls the ETSI 014
`/status` endpoint at configurable intervals. SAE-side fields
(`key_id_observed`, `psk_age_seconds`) require cooperating
instrumentation in the SAE; when absent they are `null`.

## 6.3 `agility`

```json
{
  "level": "configurable",
  "level_score": 0.75,
  "evidence": [
    {
      "type": "config_pattern",
      "file": "/etc/nginx/nginx.conf",
      "line": 142,
      "snippet": "ssl_protocols TLSv1.2 TLSv1.3;\nssl_ciphers HIGH:..."
    },
    {
      "type": "fips_mode",
      "detected": false
    }
  ],
  "scanner_version": "ree0xq-agility/0.3.1",
  "rubric_version": "qra-rubric/v1.0"
}
```

Table 5 lists these fields.

| Field | Type | Notes |
|-------------------|--------|-------|
| `level` | enum | `negotiated` / `configurable` / `pinned` / `locked` / `frozen`. |
| `level_score` | float | Numeric value per §5.3. |
| `evidence` | array | One or more evidentiary findings supporting the level. |
| `scanner_version` | string | ree0xQ-agility version that produced the score. |
| `rubric_version` | string | Version of the public scoring rubric (§8.3). |

: Fields of the `agility` block.

Evidence types in V1: `protocol_negotiation` (observed wire-level
algorithm negotiation), `config_pattern` (configuration file
exposing algorithm choice), `code_pattern` (source code reference
to a fixed algorithm), `firmware_string` (binary-extracted
algorithm name), `fips_mode` (FIPS provider/kernel mode detected),
`vendor_declaration` (operator-provided vendor statement).

## 6.4 New asset kinds

```
qkd_link          // identity = KME endpoint URL hash
qkd_kme           // identity = KME ID per ETSI 014 status
```

These are emitted by `ree0xq-qkd` independently of the session
events that consume their keys. They allow the dashboard to render
a QKD link health view distinct from the SAE-side session view.

## 6.5 Backwards compatibility

The schema extension is strictly additive:

- All fields new in v1.1 are top-level; no existing field shape
  changes.
- v1.0 consumers that do not recognize the new fields ignore them.
- v1.1 producers that lack data for the new fields emit them as
  `null` (per the V1 module emission contract).
- The posture engine treats `channel_protection: null` as
  `state: classical` and `agility: null` as the
  `UNKNOWN_LEVEL_FALLBACK` constant defined in `ree0xq-agility`,
  which evaluates to `level: pinned` (`level_score: 0.50`).
  This is deliberately more conservative than Axis A's
  `unknown = 0.4`: an asset whose agility we cannot evidence
  is treated as if it were merely pinned rather than freely
  configurable, so the rollup does not credit unmeasured
  agility for free.

---

# 7. ree0xQ: reference architecture and implementation

ree0xQ is an open-source observability platform implementing the
three-axis posture model. The implementation is a single Rust
workspace containing seven crates: five agents emitting events,
one shared rollup library, and one collector/server. Figure 3
shows how the agents, the shared rollup library, the collector,
and the dashboard fit together.

![**Figure 3.** ree0xQ reference architecture. Five agents
(`ree0xq-net`, `ree0xq-qkd`, `ree0xq-agility`, plus `ree0xq-cert`
and `ree0xq-chain`/`ree0xq-id` in later phases) emit
`crypto_inventory_event` records into `ree0xq-server`. The
shared `ree0xq-core` library hosts the schema, the
classification table, and the deadline-adjusted rollup. The
React dashboard renders the three-axis posture matrix and the
priority-sorted action list.](figures/ree0xq-architecture.pdf){#fig:arch width=90%}

## 7.1 Workspace layout

```
ree0xq/
├── crates/
│   ├── ree0xq-core/       # event schema, rollup engine (no I/O)
│   ├── ree0xq-server/     # axum collector + REST API
│   ├── ree0xq-net/        # eBPF agent: TLS, SSH, IPsec
│   ├── ree0xq-qkd/        # ETSI GS QKD 014 collector + emulator
│   ├── ree0xq-cert/       # X.509 inventory (CT, host scan)
│   ├── ree0xq-chain/      # public-chain crypto observation
│   ├── ree0xq-id/         # HSM/KMS/smart-card inventory
│   └── ree0xq-agility/    # static crypto-agility scanner
├── docs/
└── web/                  # React + Vite dashboard
```

The architectural invariant — every agent emits one and only one
event shape, computed locally via `ree0xq-core::rollup` — is what
keeps the platform composable across surfaces as different as
eBPF TLS sniffing and Solidity source code analysis.

## 7.2 ree0xq-qkd: ETSI 014 collector and emulator

The `ree0xq-qkd` crate fulfils two roles. Operationally, it is a
collector that polls one or more ETSI GS QKD 014 KMEs, emits
`qkd_link` and `qkd_kme` events, and serves as the data source
for the `channel_protection` block on session events emitted by
cooperating SAEs.

### 7.2.1 Collector design

The collector is a Tokio async loop that, for each configured
KME, issues:

- `GET /api/v1/keys/{slave_SAE_ID}/status` at a configurable
  cadence (default 5 s).
- An auxiliary `enc_keys` request at a slower cadence (default 60
  s) to measure end-to-end key delivery latency. Keys are
  requested with `size=0` when permitted, or discarded when not.

The collector tracks per-KME state (last good status time,
exponentially-weighted error rate, QBER history) and emits
`qkd_link` events on status change, plus heartbeat events at a
configurable interval. Authentication to the KME follows ETSI 014
guidance: mutual TLS with the SAE certificate.

### 7.2.2 Emulator

Hardware QKD is expensive and rare, so we ship
`ree0xq-qkd-kme-emulator` — an ETSI 014 v1.1.1 implementation
backed by a synthetic key generator. The emulator:

- Implements `/status`, `/enc_keys`, and `/dec_keys` exactly per
  spec.
- Generates synthetic keys at a configured rate, with configurable
  QBER, key size, and lifetime.
- Supports replay scenarios — pre-recorded sequences of link
  state changes (degradation, failure, recovery) — for
  reproducibility.
- Logs every interaction in a documented JSON capture format
  enabling head-to-head A/B testing of SAE implementations.

The emulator is the foundation of the §8.2 empirical study and
is released alongside ree0xQ as a standalone tool.

## 7.3 ree0xq-agility: static crypto-agility scanner

The `ree0xq-agility` crate implements the agility-axis scoring per
§5.3. It accepts as input one or more *targets* and produces
`agility` blocks attached to the corresponding assets.

### 7.3.1 Target types

Table 6 maps each target type to the evidence the scanner draws on.

| Target | Evidence sources |
|-------------------|------------------|
| Source repository | Semgrep ruleset over the language-specific config and source files; file-path heuristics for build-time pinning. |
| Installed package | Package manifest (rpm, dpkg, pip, npm) + binary string-extraction over the installed artifacts. |
| Running host | TLS handshake observation against the host (server algorithm support → `negotiated` evidence); plus optional auth into the host's config files. |
| Vendor appliance | Vendor-declared algorithm scope + observed handshake behavior. |

: Crypto-agility scanner target types and their evidence sources.

### 7.3.2 Ruleset

The published ruleset (`ree0xq-agility/rules/v1`) consists of
several hundred Semgrep patterns covering common cryptographic
libraries and protocols across C, Go, Rust, Python, Java, and
configuration formats (nginx, Apache, OpenSSL config, sshd_config,
strongswan.conf, Postfix, Dovecot, HAProxy, Envoy). Each pattern
emits one piece of evidence with a documented mapping to the
five-level rubric.

### 7.3.3 Scoring algorithm

For each target, the scanner collects evidence and applies a
**most-agile-wins** aggregation: the asset is scored at the *most*
agile level supported by any rule that fired. The rationale is
asymmetric: rules fall into two semantic classes. *Capability*
rules (e.g., presence of an `ssl_ciphers` directive, a
`SSL_CTX_set_cipher_list` call) demonstrate that an operator can
in fact change the algorithm without invasive surgery; their
emit_level is an *upper bound on what is observable*. *Constraint*
rules (e.g., a literal hard-coded algorithm name in source) only
report that *some* algorithm is fixed somewhere — they do not
imply that there is no overriding config surface. Conservative-min
aggregation systematically misclassifies large agility projects
like nginx (which simultaneously embeds default algorithm strings
in source and exposes ten configuration knobs that override
them); the operationally useful question is "what is the best
surface I have," not "what is the worst constraint anywhere."

Conservative-min remains available as an alternative aggregation
projection for operators who require it (regulatory contexts where
the weakest surface governs); the published scanner exposes both
projections through a dashboard toggle.

Where evidence is absent, the scanner produces `level: pinned`
(the documented `UNKNOWN_LEVEL_FALLBACK` constant in
`ree0xq-agility`) and surfaces the issue for operator review.

We discuss limitations of static-only agility scoring in §9.

## 7.4 ree0xq-core unified rollup

The rollup engine extends the V1 implementation (see
`docs/posture-rollup.md` in the ree0xQ repository) with the
deadline-adjusted three-axis formula of §5.4. The
implementation remains a pure function with no I/O; the operator
configures $D$, $H$, and asset-class weights via the dashboard,
and the engine recomputes on every event ingest. Fuzz testing
covers all axis combinations.

## 7.5 ree0xq-server and dashboard

The collector is an Axum HTTP service accepting v1.0 and v1.1
events, validating against a generated JSON schema, persisting to
Postgres (configuration) and a columnar store (events), and
serving a React+Vite dashboard. The dashboard renders a
three-axis posture matrix per asset class, a deadline-countdown
view tied to the configured $D$, and an inventory table sortable
by $q$. Implementation details are routine and we defer them to
the project documentation.

---

# 8. Empirical evaluation

We evaluate the model and implementation through three studies,
each designed to be reproducible by an external practitioner with
the published rulesets, emulator, and corpus lists.

## 8.1 Study 1 — Axis A on the public web (Tranco-top-1k)

### 8.1.1 Methodology

We scan the Tranco-top-1k [@tranco] over TLS with two
purpose-built probes — general-purpose scanners such as
`zgrab2` [@zgrab2] do not yet advertise X25519MLKEM768 in the
ClientHello, so we built dedicated baseline and PQ-capable
probes to keep the ClientHello surface deterministic and the
output schema aligned with ree0xQ's collector:

1. **Classical baseline probe** — Python `ssl` with the
   system OpenSSL defaults. Establishes a TLS handshake
   without advertising `X25519MLKEM768`. Output: JSON array.
2. **PQ-capable probe** — a Rust binary built on
   `rustls = 0.23` + `rustls-post-quantum = 0.2`, with a
   custom certificate verifier that accepts every chain
   (this is observability, not authentication). Advertises
   `X25519MLKEM768` alongside the classical groups. Output:
   one NDJSON record per host.

For each host we record: connect/handshake outcome, negotiated
TLS version, negotiated cipher suite, negotiated key-exchange
group, leaf certificate signature algorithm, and the leaf
certificate Subject. The scan is constrained to one TCP
connection per host, identifies itself in the ClientHello SNI
extension as `ree0xq-survey/1.0
+https://e2esolutions.tech/ree0xq`, and uses a 5-second
connect+handshake timeout with a 1 Hz rate cap.

The scan source code, target list, raw probe outputs, and
analysis script are released alongside this paper at
[`studies/study1/`](studies/study1/).

Ethical considerations: this is a one-shot benign TLS handshake
similar to common research scans [@durumeric-tls]; we do not
attempt protocol downgrade or repeated probing. Scan rate is
≤1 Hz per probe and exits immediately on connection error.

### 8.1.2 Metrics

We report the distribution of asset-A scores across the corpus,
the prevalence of PQ-capable hosts, the distribution of
certificate signature algorithms, and the prevalence of weak/
deprecated primitives still observed.

### 8.1.3 Results (Tranco-1k snapshot 6G8PX, 2026-05-13)

We ran the methodology of §8.1.1 against the Tranco-top-1k
list `6G8PX` (snapshot 2026-05-13). 724 of 1,000 hosts
returned a usable TLS handshake within the 5-second timeout;
the remaining 276 were unresponsive (DNS failure, no TCP, no
TLS on 443, regional GeoIP block, or anti-bot middlebox).
The 27.6% non-response rate is in the range reported by
prior large-scale TLS scans of the open web
[@durumeric-tls]. All headline percentages in this section
use the $n = 724$ responsive denominator unless stated
otherwise. Figure 4 shows the classical-probe baseline
(negotiated ciphersuite and certificate signature algorithm),
and Figure 5 shows the PQ-capable probe result.

![**Figure 4.** Study 1 — classical-probe baseline on the
Tranco-top-1k (n_ok = 724): negotiated TLS 1.3 ciphersuite
(left) and leaf certificate signature algorithm
(right).](studies/study1/plots/study1-tranco-distribution.pdf){#fig:study1 width=95%}

![**Figure 5.** Study 1 — PQ-capable probe (rustls +
`rustls-post-quantum`) against the Tranco-top-1k. 317 of
724 responsive hosts (43.8%) negotiate `X25519MLKEM768`
when offered; the remainder fall back to classical `x25519`,
`secp256r1`, or `secp384r1`. A metric the classical-only
probe cannot observe.](studies/study1/plots/study1-tranco-pq-kex.pdf){#fig:study1pq width=95%}

For sample-selection contrast we also report results from a
30-host curated pilot — major CDN, browser-vendor, distro,
IETF/IEEE/NIST/ETSI, and AI-vendor properties — that ran
the same probe pair before the Tranco-1k scan. Table 7 reports both.

| Observable | Tranco-1k (n=724) | Curated pilot (n=30) |
|-------------------------------------|---------------------:|---------------------:|
| TLS 1.3 negotiation | 602/724 (83.1%) | 30/30 (100%) |
| TLS 1.2 fallback | 122/724 (16.9%) | 0/30 (0%) |
| AES-256-GCM/SHA-384 (TLS 1.3) | 384/724 (53.0%) | 20/30 (67%) |
| AES-128-GCM/SHA-256 (TLS 1.3) | 207/724 (28.6%) | 7/30 (23%) |
| ChaCha20-Poly1305 (TLS 1.3) | 11/724 (1.5%) | 3/30 (10%) |
| ECDSA leaf cert (P256 + P384) | 254/724 (35.1%) | 18/30 (60%) |
| RSA-PKCS1 leaf cert (SHA256+SHA384) | 470/724 (64.9%) | 12/30 (40%) |
| ML-DSA / SLH-DSA cert | 0/724 | 0/30 |
| Deprecated primitive (SHA-1, RC4) | 0/724 | 0/30 |
| **`X25519MLKEM768` negotiated** | **317/724 (43.8%)** | **17/30 (57%)** |

: Study 1 — primitive distribution on the Tranco-top-1k versus the curated pilot.

**The headline PQ-adoption result is 317/724 (43.8%) on the
Tranco-top-1k.** This is close to the 39% of top-100k sites
that Cloudflare measured supporting post-quantum key
agreement in September 2025 [@pq-tls-measurement]; both
numbers are site-capability measurements, and the 4-point
gap is consistent with Tranco-top-1k over-representing
large CDN- and cloud-fronted properties relative to the
broader top-100k corpus.
The remaining 407 responsive hosts fell back to classical
`x25519` (312), `secp256r1` (80), or `secp384r1` (15).

The curated 30-host pilot returned 17/30 = 57% PQ adoption —
13 percentage points above the Tranco-1k rate. The gap is
attributable to sample-selection bias: the curated list
emphasises Cloudflare-fronted and Google-fronted properties
(cloudflare.com, twitter.com, reddit.com, anthropic.com,
openai.com, google.com, youtube.com, e2esolutions.tech, and
a long tail) whose edge terminators rolled out
X25519MLKEM768 early. The 13 classical-only hosts in the
curated pilot — github.com, microsoft.com, amazon.com,
mozilla.org, kernel.org, nist.gov, openssl.org, and others —
are notable in their own right as major
infrastructure-relevant properties where the PQ rollout has
not yet landed. The two numbers together suggest that surveys
built on curated "interesting" host lists overstate PQ
readiness compared to broader corpora. Where these numbers
appear in the literature, both rates — broad-corpus and
curated — deserve to be reported.

**Two further observables of operational interest.** First,
122 of 724 responsive Tranco-1k hosts (16.9%) still
negotiate TLS 1.2 at the top of the Web. Because
`X25519MLKEM768` is a TLS 1.3 key-share, that 16.9% cohort
is structurally PQ-ineligible until the operator's TLS
terminator can negotiate TLS 1.3 — a long-tail constraint
not visible in any single-axis cipher dashboard. Second,
the certificate-signature distribution on the Tranco-1k is
RSA-dominant (64.9%), inverting the ECDSA-dominant (60%)
distribution in the curated pilot. The curated list skews
toward post-2020 CDN deployments that default to ECDSA;
the broader corpus retains a substantial RSA installed base.

Pipeline integration: PQ-probe NDJSON parses straight into
the same `tls_session` asset shape consumed by `from-zgrab`,
so the same downstream rollup, BLOCKED-flag derivation, and
dashboard render exactly as for any other source-module
input.

## 8.2 Study 2 — Axis C via the ETSI GS QKD 014 emulator

### 8.2.1 Methodology

We construct a controlled testbed of one master KME, two slave
KMEs, and three cooperating SAEs (a strongSwan IPsec endpoint
operating in PSK mode with QKD-PSK rotation, a Wireguard endpoint
with manual PSK rotation, and a custom TLS endpoint using the
NIST SP 1800-38A hybrid-PSK pattern). The KMEs are
`ree0xq-qkd-kme-emulator` instances; we drive them through a
documented sequence of replay scenarios:

- **R1 — Steady-state.** Constant QBER 1.8%, key rate 12 kbps,
  no interruption. 24 hours.
- **R2 — Gradual degradation.** QBER ramps from 1.8% to 8.5% over
  4 hours; the SAE policy should fail over to classical at the
  configured threshold. We observe whether each SAE detects the
  degradation and whether ree0xQ's `link_health` reflects the
  transition.
- **R3 — Hard failure.** KME unreachable for 30 minutes. SAE
  behavior should be: continue with cached PSK until lifetime
  expires, then fail closed or fall back to classical per policy.
- **R4 — Stale PSK.** PSK rotation is suppressed at the SAE while
  the KME continues to produce keys. ree0xQ should observe
  `psk_age_seconds` rising past policy.
- **R5 — Bifurcated SAE.** The strongSwan endpoint sees a healthy
  KME while the Wireguard endpoint sees a failed KME (simulating
  partial KME outage). ree0xQ should report inconsistent
  per-session `channel_protection` while the `qkd_link` aggregate
  is `degraded`.

### 8.2.2 Metrics

We measure: SAE failover correctness, ree0xQ observation latency
(emulator change → emitted event), event-ordering correctness
under concurrent KME polls, and posture-rollup correctness
(particularly that the unified $q$ score reflects the
degradation appropriately).

### 8.2.3 Results

All five scenarios ran end-to-end in compressed time (30–60s
each rather than the full 4–24h documented duration; the
runner is at `studies/study2/run.sh`). Per-scenario captures
and the analysis script are at `studies/study2/`.

**Classification correctness: 13/13.** Every operator-induced
state change produced a downstream `link_health` reading
matching the expected post-op state. Table 8 gives the per-scenario counts.

| Scenario | Events captured | Induced transitions | Matched |
|-------------------|----------------:|--------------------:|--------:|
| R1 — steady-state | 33 | 1 | 1/1 |
| R2 — ramp | 63 | 5 | 5/5 |
| R3 — hard-failure | 48 | 3 | 3/3 |
| R4 — stale-PSK | 33 | 1 | 1/1 |
| R5 — bifurcated | 48 | 3 | 3/3 |

: Study 2 — events captured and induced link-health transitions per scenario.

**Observation latency: p50 = 0.71s, range 0.70–0.71s** across
all 13 transitions, against a configured 1-second poll
interval. The latency band hovers tightly around half the
poll period — the analytical expectation for periodic
polling — and dominates over event-emission cost. Increasing
the poll interval moves the median latency linearly; in real
ETSI 014 deployments where the operational poll cadence is
typically 5–10 s [@etsi-qkd-002], this gives a p50 closer to
2.5–5 s.

The R3 link_health timeline (Figure 6; analogue plots for
every scenario ship in `studies/study2/plots/`) shows the
hard-failure transition landing as expected.

![**Figure 6.** Study 2 — R3 hard-failure timeline. The KME is
forced unreachable at T+10s and recovered at T+30s; the collector
captures the Ok → Failed → Ok cycle on the next poll
(observation latency ≈ 0.7s on the configured 1s
interval).](studies/study2/plots/r3-hard-failure-timeline.pdf){#fig:study2 width=95%}

R5 illustrates the per-session attribution case. When one KME
returns 503, the link-level event flips to `failed` even though
a separately healthy paired KME continues to deliver keys.
KME-only telemetry cannot distinguish this from a full outage;
the channel-protection block on per-session events does.

R4 surfaces a gap we left in by design. ree0xQ's
KME-side polling alone cannot distinguish a fresh PSK from
one the SAE has held for hours; the `psk_age_seconds` field
on `channel_protection` is populated only when the SAE
co-operates. Closed-source SAEs without that instrumentation
are visible to ree0xQ only at the link layer. We open-source
patches for strongSwan, Wireguard, and a sample TLS endpoint
to populate the field.

The emulator, replay scripts, and analysis notebooks ship
under MIT alongside ree0xQ.

## 8.3 Study 3 — Axis G on fifty open-source server projects

### 8.3.1 Methodology

We select fifty widely deployed open-source server projects
spanning HTTP, mail, database, message broker, and VPN
categories: nginx, Apache httpd, HAProxy, Envoy, Caddy, Traefik,
OpenSSH server, Postfix, Dovecot, Exim, PostgreSQL, MySQL, Redis,
MongoDB, RabbitMQ, Kafka, OpenVPN, strongSwan, Wireguard,
PowerDNS, BIND, Unbound, CoreDNS, and others. The full list,
with hand-graded ground-truth levels and reviewer notes, ships
as `crates/ree0xq-agility/corpus/oss-50-v1.csv` in the ree0xQ
repository.

For each project we:

1. Run `ree0xq-agility` against the source repository at a
   pinned commit.
2. Run `ree0xq-agility` against the installed binary on Rocky
   Linux 10 with default package configuration.
3. Hand-grade the project against the §5.3 rubric, using two
   reviewers and reporting inter-rater agreement.
4. Report the divergence between automatic and hand-graded
   scores; treat the hand grade as ground truth for the corpus.

### 8.3.2 Metrics

We report per-project agility level, the per-evidence
contribution to the score, the false-negative and false-positive
rates of the Semgrep ruleset against the ground-truth grading,
and the distribution of agility levels across the corpus by
category.

### 8.3.3 Results (n = 11 pilot subset)

The full OSS-50 corpus is committed at
`crates/ree0xq-agility/corpus/oss-50-v1.csv` with hand-graded
ground truth. A pilot run over an 11-project subset spanning
nine categories (HTTP, mail, DB, message-broker, DNS,
VPN/secure-shell, messaging, certificate-authority, time)
exercises the full pipeline (Table 9):

| Project | Category | Hand-grade | Scanner | Match |
|----------------|-----------------------|--------------|--------------|------|
| nginx | http_server | configurable | configurable | yes |
| haproxy | http_server | configurable | configurable | yes |
| caddy | http_server | configurable | configurable | yes |
| unbound | dns_server | configurable | configurable | yes |
| coredns | dns_server | configurable | configurable | yes |
| postfix | mail_server | configurable | configurable | yes |
| redis | database | configurable | configurable | yes |
| nats-server | message_broker | configurable | configurable | yes |
| step-ca | certificate_authority | configurable | configurable | yes |
| prosody | messaging | configurable | configurable | yes |
| wireguard-tools| vpn_secure_shell | pinned | pinned | yes |
| chrony | time | configurable | **pinned** | no |

: Study 3 — per-project hand-grade versus scanner classification (n = 11 pilot).

**Agreement: 10/11 (91%); Cohen's $\kappa = 0.62$**
(substantial agreement on the Landis–Koch scale). The single
dissent is **chrony**, where the static evidence is genuinely
ambiguous between a `configurable` reading (NTS key-types are
operator-controlled in `chrony.conf`) and a `pinned` reading
(the NTP authentication path embeds many literal symmetric-
algorithm references). The scanner picked the latter on the
strength of 11 hard-coded references; no capability rule
fired on chrony's NTS surface because it uses neither OpenSSL
nor Go's `crypto/tls`. We flag this for v2 of the ruleset
and the corpus.

The confusion matrix (Figure 7) visualises this result.
Per-project event JSON, evidence listings, and the full
agreement TSV are at `studies/study3/results/`.

![**Figure 7.** Study 3 — confusion matrix (n=11 projects,
10/11 agreement, Cohen's $\kappa=0.62$).](studies/study3/plots/study3-agreement-matrix.pdf){#fig:study3 width=70%}

Failure modes characterised during the run:

- **Go ecosystem coverage gap.** The v1 ruleset's first cut
  matched only OpenSSL-style API calls (`SSL_CTX_set_cipher_list`,
  `SSL_CONF_cmd`, …). Three projects (coredns, step-ca,
  caddy) initially classified as `pinned` because nothing
  matched. Adding a one-rule `go-stdlib-tls-config` pattern
  matching `crypto/tls`'s `Config` / `CipherSuites` /
  `MinVersion` fields recovered all three to `configurable`.
- **Aggregation policy.** Conservative-min aggregation
  systematically misclassified nginx: nginx embeds literal
  algorithm names in source (for crypto-impl code paths and
  tests, emit_level `pinned`) and exposes operator config
  knobs (emit_level `configurable`). The §7.3.3 most-agile-
  wins policy resolves the ambiguity in favour of the
  capability surface; conservative-min remains available as
  a regulatory-context alternative.
- **No-evidence fallback.** Wireguard correctly classified
  `pinned` with zero evidence collected — the project's
  userspace tooling has no OpenSSL or Go-tls surface, so the
  documented `UNKNOWN_LEVEL_FALLBACK = Pinned` applies. The
  ground-truth grade agrees.

Scaling the pilot to the full OSS-50 list is mechanical; the
runner already iterates over the corpus CSV.

## 8.4 End-to-end pipeline

`scripts/demo.sh` exercises the full V1 pipeline end-to-end:
boot the KME emulator, the QKD collector, ree0xq-server, and
seed events from both the bundled zgrab2 fixture and a
synthetic FIPS-locked asset. The collector's `/v1/posture`
endpoint returns:

```json
{
  "org_q": 0.627,
  "deadline": "2030-01-01T00:00:00Z",
  "horizon_years": 5.0,
  "assets": 5,
  "blocked_count": 1
}
```

The inventory ordering, sorted by $q$ descending, places the
TLS 1.0 + RC4 + SHA-1 legacy host at the top
(q ≈ 0.72), the TLS 1.2 ECDHE+RSA classical host just below
(q ≈ 0.69), the FIPS-locked appliance third with the
`BLOCKED` flag raised (q ≈ 0.68), the modern TLS 1.3 +
ECDSA-P256 hybrid-PSK QKD host at the bottom of the priority
queue (q ≈ 0.43). This matches the model's expected behaviour from §5.4
(axis combination) and §5.5 (edge cases). The channel-protection
axis pulls the QKD-protected asset down even though its
algorithmic content is classical; the agility axis pushes the
FIPS-locked asset up even though its observed primitives match
the nominally-agile modern host; the deadline-tension term
leaves the legacy host at the top of the queue.

The implementation's pure-Rust rollup (in
`crates/ree0xq-server/src/posture.rs`) was verified against
the magazine companion's four-asset worked example: the unit
tests `worked_example_alpha_q_matches_paper` and
`worked_example_delta_q_matches_paper` assert that the
implementation reproduces α = 0.544 (modern-agile host, no
QKD, evaluated 2026-05-13) and δ = 0.392 (legacy classical
host, same date) to within 0.01.

To our knowledge no comparable open pipeline covers all three
axes end-to-end on commodity hardware. The Tranco-1k scan
reported in §8.1 is the broad-corpus result for Axis A;
scaling Study 3 from the 11-project pilot to the full OSS-50
corpus is the remaining mechanical step on the published
runners.

---

# 9. Discussion

## 9.1 Limitations

Our agility scoring is static and pattern-based; it is necessarily
approximate. A project may *appear* pinned in source while
actually being agile through a runtime extension mechanism we did
not pattern-match. Conversely, a project may appear agile via a
config field that is in practice never changed. We address
false negatives by reporting per-evidence detail; we cannot
fully address the second class without operator input.

Our channel-protection axis depends on cooperating SAE
instrumentation for per-session attribution. ree0xQ can observe
the KME state independently, but linking a specific TLS or IPsec
session to a specific consumed key requires the SAE to emit
the `key_id_observed` field. We expect adoption in cooperating
software (we open-source patches for strongSwan, Wireguard, and
a sample TLS endpoint as part of the release) but note that
closed-source SAEs may report only at the link level.

Our threat model accepts the NIST PQC hardness assumptions and
the standard QKD security argument. Compromise of either — by
mathematical break (PQC) or by implementation attack (QKD) —
would require recalibration of the scoring tables. The schema
supports this: the rollup constants are operator-tunable in
configuration, not compiled in.

We do not address economic or political constraints on migration
(budget cycles, regulatory approval lag, vendor support
windows). These are first-order operator concerns and we
acknowledge them as out of scope for the observability layer;
they are downstream consumers of the posture data.

## 9.2 Ethical considerations

Active TLS scanning, even of public web hosts, must be conducted
responsibly. We follow established practice from prior
measurement work [@durumeric-tls]: one connection per host, clear
User-Agent identification, opt-out instructions linked from the
identifying URL, conservative scan rate, no protocol downgrade,
no repeated probing. The Tranco list excludes adult content and
known sensitive categories; we additionally exclude any host
that returns a robots.txt directive on its base URL within the
first 1024 bytes.

The agility scanner operates on source code already public
under open-source licenses and on installed binaries on hosts
the operator controls. No private code or vendor materials are
processed.

The QKD emulator generates synthetic key material only; it is
not connected to a real QKD link in our published study.
Operators integrating real KMEs are responsible for the
security of those KMEs and the surrounding network, including
mutual TLS authentication of the SAE.

## 9.3 Deployment guidance

For operators planning to deploy ree0xQ against a real
environment, we recommend a phased rollout:

1. **Phase 1: Inventory only.** Run `ree0xq-net` and
   `ree0xq-agility` in observe-only mode. Establish a baseline.
2. **Phase 2: Add deadline.** Configure $D$ per applicable
   regulatory regime. Observe how $q$ evolves with no other
   change.
3. **Phase 3: Prioritize.** Sort assets by $q$ descending,
   migrate the top $N$ by quarter.
4. **Phase 4: Integrate QKD telemetry where it exists.** Add
   `ree0xq-qkd` only when a real KME is present and the SAE
   instrumentation is in place; the channel-protection axis is
   *additive* and never required.

We caution against treating $q$ as a primary KPI. It is a
*comparative* score, calibrated for relative prioritization
within an environment. Inter-organization comparison requires
shared $D$, shared weights, and shared corpora.

## 9.4 Future work

Several extensions are natural and intentionally deferred:

- **Time-decay.** Treating an observation made three months ago
  as less authoritative than one made today.
- **Asset relationship modeling.** Linking a TLS session's
  certificate to the issuing CA's signing key allows the posture
  of an upstream asset to propagate into the downstream score.
- **Adversary modeling.** Allowing operators to plug in their own
  estimate of CRQC arrival as a probability distribution over
  $D$ rather than a single date.
- **Standardization.** Submitting the `crypto_inventory_event`
  schema to IETF as an Informational draft so other tools can
  emit and consume it.

---

# 10. Conclusion

Single-bit PQ-readiness was the right answer to a question
that turned out to be too narrow. An asset's quantum-risk
posture depends on its primitive, on the channel through
which its keys are delivered, and on how quickly the
primitive can be replaced before the deadline lands. The
three-axis model in this paper grades each of those
separately and combines them only at the rollup stage,
which is where the operator actually needs a number.

The empirical work suggests the gap is not hypothetical.
On the Tranco-top-1k, 43.8% of responsive hosts negotiated
a hybrid PQ key exchange when offered one — a figure that
says nothing about whether those same hosts could rotate
their certificate signature algorithm, or whether any of
their key material is QKD-protected, because the
single-axis scan does not look. The emulator study showed
that channel-state classification stays correct across
every induced KME and link failure we drove (13 of 13),
including the per-session attribution case that link-level
telemetry alone cannot resolve. And the agility pilot
reached 91% agreement with hand-graded ground truth on
eleven projects, with the disagreements concentrated where
the static evidence is genuinely ambiguous rather than
where the rubric is wrong. None of these three results is
reachable from a PQ-ready bit; each requires its own axis.

We are deliberate about what the model does not settle. The
scoring constants are one defensible choice among several,
and we expose them as configuration rather than compiling
them in. The agility rubric will need refinement as new
crypto APIs land in deployed software, and the Go-ecosystem
coverage gap we hit in Study 3 is a concrete example of
that maintenance burden. The QKD axis presumes a deployment
trend that not everyone in the community shares, which is
exactly why we made it additive — an operator with no QKD
links pays no modelling cost for the axis existing. What we
do claim is narrow and, we think, hard to argue with: one
bit per asset is fewer bits than an operator facing a
deadline in the early 2030s actually needs, and surfacing all three
axes is a strict improvement over surfacing one.

The schema, the reference implementation, the ETSI 014
emulator, the Semgrep rule pack, and the hand-graded corpus
are released together under MIT. The studies are scripted
end to end and run on a single commodity host without QKD
hardware. We would rather see readers rerun the
measurements, disagree with our weights, and supersede them
on a shared empirical footing than have the specific
constants in this paper treated as settled. The model is
the contribution; the numbers are an invitation.

---

# Author Contributions

Aleaddin Özer conceived the study, designed the three-axis
posture model, implemented the ree0xQ reference platform,
conducted all three empirical studies, and wrote the
manuscript. Murat Aydos supervised the work as doctoral
advisor, providing continuous critical review, methodological
guidance, and feedback throughout the project, and revised
the manuscript. Both authors reviewed and approved the final
manuscript.

---

# Competing Interests

Aleaddin Özer is Chief System Engineer of E2E Solutions,
which develops the ree0xQ reference implementation described
in this work. ree0xQ is released under the MIT License, and
E2E Solutions derives no direct revenue from its
publication. Murat Aydos declares no competing interests.
The authors have no other competing financial or
non-financial interests.

---

# Funding

This work was self-funded by E2E Solutions and Hacettepe
University. No external grants, sponsorships, or contracts
supported the design, execution, or reporting of the
research presented here.

---

# Data and Code Availability

Everything is at <https://github.com/e2esolutions-tech/ree0xQ>
under MIT. The repository carries the schema (v1.1), the
five-agent reference implementation, the ETSI GS QKD 014
emulator with its replay corpus, the Semgrep agility rule
pack, and the hand-graded ground-truth corpus
(`crates/ree0xq-agility/corpus/oss-50-v1.csv`). Runner
scripts reproduce all three studies.

The platform was renamed from *Sezar* to *ree0xQ* in August 2026. The public preprint and the archived study artifacts under `studies/` predate the rename and retain the former name and module identifiers; the released code is otherwise identical.


Study 1 used Tranco snapshot `6G8PX` (2026-05-13) as its
sample frame. Raw NDJSON captures and the analysis plots
for Studies 1 and 2 live under `studies/study1/` and
`studies/study2/`. Study 3's per-project evidence listings
and the agreement TSV are at `studies/study3/results/`.

No human subjects, personally identifying information, or
proprietary data are involved.
