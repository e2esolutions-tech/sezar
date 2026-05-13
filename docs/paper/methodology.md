# Empirical Methodology — Executable Plan

This document accompanies the paper *Three Axes of Quantum Risk* and
specifies, in operational detail, how the three empirical studies are
executed. Every step is reproducible from a Linux host (Rocky Linux
10 or any modern distribution), a public Internet connection, and the
code released alongside the paper. No commercial scanning service or
QKD hardware is required.

The three studies map one-to-one to the paper's three axes:

| Study | Axis | What it measures                                          | What it produces                                |
|-------|------|-----------------------------------------------------------|-------------------------------------------------|
| 1     | A    | TLS algorithm distribution on Tranco-top-1k               | TLS scan corpus + `sezar-net`-emitted events    |
| 2     | C    | ETSI GS QKD 014 collector + SAE behavior under fault     | KME emulator + replay scripts + capture corpus  |
| 3     | G    | Crypto-agility of 50 widely deployed OSS server projects  | Semgrep ruleset + hand-graded ground truth      |

For each study we list the **inputs**, **steps**, **expected
outputs**, **runtime envelope**, and **ethical considerations**.

---

## Study 1 — Public-Web TLS Survey (Axis A)

### Inputs

- **Tranco-top-1k** target list. Obtained from `https://tranco-list.eu/`
  on the scan date; we pin to a specific list ID for reproducibility.
- `zgrab2` v0.1.x or later (single static Go binary).
- A scanning host with stable IPv4/IPv6 connectivity. Single host;
  no distributed scan needed at this scale.

### Steps

1. **Pin the target list.**
   ```bash
   TRANCO_ID=$(curl -s https://tranco-list.eu/api/lists/date/$(date +%Y-%m-%d) | jq -r .list_id)
   curl -s "https://tranco-list.eu/download/${TRANCO_ID}/1000" -o targets/tranco-1k-${TRANCO_ID}.csv
   ```

2. **Generate scan target file (one host per line).**
   ```bash
   awk -F, '{print $2}' targets/tranco-1k-${TRANCO_ID}.csv > targets/tranco-1k-${TRANCO_ID}.txt
   ```

3. **Run baseline scan (no PQ groups offered).**
   ```bash
   zgrab2 --senders 10 --output-file results/tranco-1k-baseline-${TRANCO_ID}.json \
     tls --port 443 \
     --custom-name "sezar-survey/1.0 (+https://e2esolutions.tech/sezar)" \
     --next-protos h2,http/1.1 \
     < targets/tranco-1k-${TRANCO_ID}.txt
   ```

4. **Run PQ-capable scan (X25519MLKEM768 offered alongside classical).**
   ```bash
   # Uses a sezar-net wrapper that issues a ClientHello with the
   # X25519MLKEM768 key share group code (0x11EC).
   ./bin/sezar-net-tls-probe \
     --groups x25519,x25519mlkem768 \
     --in targets/tranco-1k-${TRANCO_ID}.txt \
     --out results/tranco-1k-pq-${TRANCO_ID}.ndjson \
     --rate 10 \
     --identifier "sezar-survey/1.0 (+https://e2esolutions.tech/sezar)"
   ```

5. **Convert scan outputs to `crypto_inventory_event v1.1` and ingest.**
   ```bash
   ./bin/sezar-scan-to-events \
     --input results/tranco-1k-pq-${TRANCO_ID}.ndjson \
     --source-module sezar-net \
     --output-collector https://collector.local/v1/events
   ```

### Outputs

- `results/tranco-1k-baseline-${TRANCO_ID}.json` — raw zgrab2 capture.
- `results/tranco-1k-pq-${TRANCO_ID}.ndjson` — PQ-capable scan capture.
- ≤1000 `tls_session` events ingested into the collector.
- Aggregate report: percentage of hosts that (a) accept any TLS 1.3,
  (b) negotiate PQ hybrid when offered, (c) present an RSA/ECDSA
  certificate, (d) present any deprecated primitive.

### Runtime envelope

≈3 minutes for the baseline scan, ≈4 minutes for the PQ-capable
scan, at 10 Hz with a 5-second per-host timeout. Single host.
Network egress ≈30 MB total.

### Ethical considerations

- One TCP connection per host. No protocol downgrade. No
  authentication attempts. No repeated probing.
- The Tranco list excludes adult content and is curated for
  research use.
- Connection identifies the scanner via SNI extension when the
  scanner's wrapper sets it, and via the SAE name; the
  identifier URL hosts an opt-out form (a static page on the
  E2E Solutions site explaining the survey and an email
  contact).
- We honor robots.txt on the HTTP-level resource at
  `https://<host>/robots.txt` where retrievable in ≤256 bytes
  within a 2-second timeout; if the file disallows scanning,
  the host is excluded from the corpus and noted in the audit
  log.
- Scan rate is intentionally low (10 Hz total, not per-host).
  No host receives more than one connection in the same scan
  campaign.

---

## Study 2 — ETSI GS QKD 014 Emulator and SAE Behavior (Axis C)

### Inputs

- `sezar-qkd-kme-emulator` (released with this paper). Pure Rust;
  no external dependencies beyond Tokio.
- Three SAEs configured against the emulators:
  - **strongSwan** 5.9+ with PSK rotation via a small shell
    helper that retrieves keys via `curl` from the ETSI 014
    `/enc_keys` endpoint and reloads connections.
  - **Wireguard** with `wg set` PSK rotation, similarly driven.
  - **Custom TLS endpoint**: a minimal Rust server that imports
    QKD-PSK material as the external PSK in TLS 1.3 hybrid PSK
    handshake.
- A single host (any modern Linux with namespaces).

### Steps

1. **Bring up the emulator topology.**
   ```bash
   ./bin/sezar-qkd-kme-emulator \
     --listen 127.0.0.1:11071 --kme-id KME-A --role master \
     --paired-kme KME-B,KME-C \
     --key-rate-bps 12000 --qber 0.018 &
   ./bin/sezar-qkd-kme-emulator \
     --listen 127.0.0.1:11072 --kme-id KME-B --role slave \
     --paired-kme KME-A &
   ./bin/sezar-qkd-kme-emulator \
     --listen 127.0.0.1:11073 --kme-id KME-C --role slave \
     --paired-kme KME-A &
   ```

2. **Configure SAEs against KME endpoints.** Example for the
   custom TLS endpoint:
   ```bash
   ./bin/sezar-test-sae-tls \
     --master-kme http://127.0.0.1:11071/api/v1 \
     --slave-sae SAE-TLS-B \
     --rotate-every 60s \
     --listen 0.0.0.0:8443 &
   ```

3. **Run the Sezar QKD collector.**
   ```bash
   ./bin/sezar-qkd \
     --kme http://127.0.0.1:11071/api/v1 \
     --kme http://127.0.0.1:11072/api/v1 \
     --kme http://127.0.0.1:11073/api/v1 \
     --collector https://collector.local/v1/events \
     --status-poll-interval 5s \
     --key-test-interval 60s
   ```

4. **Execute replay scenarios.** The emulator accepts replay
   files describing pre-recorded link state changes. Five
   scenarios ship with the release:

   ```bash
   for replay in r1-steady r2-degradation r3-hard-failure r4-stale-psk r5-bifurcated; do
     ./bin/sezar-qkd-replay \
       --emulator-control http://127.0.0.1:11071/control \
       --replay scenarios/${replay}.yaml \
       --capture-out captures/${replay}.ndjson
   done
   ```

5. **Cross-correlate emitted Sezar events with replay timeline.**
   The `sezar-scenario-eval` tool ingests both the emulator
   replay log and the collector's event log, and computes for
   each scenario:
   - Time-to-observation (replay event → emitted event).
   - Per-session classification correctness for `state` and
     `link_health`.
   - Posture-rollup correctness against the scenario-expected
     score curve.

### Outputs

- Five `.ndjson` captures, one per replay.
- A per-scenario report containing time-to-observation
  histograms, classification confusion matrices, and
  posture-rollup correctness curves.
- The released emulator and replay scenarios become a
  community artifact for testing other SAEs.

### Runtime envelope

R1 (steady-state) is 24 hours by default; can be shortened to
30 minutes for fast validation. R2 runs 4 hours. R3, R4, R5
run ≤30 minutes each. Total study elapsed time ≈30 hours when
run sequentially; ≈4 hours when parallelized across separate
hosts.

### Ethical considerations

All keys generated by the emulator are synthetic. No traffic
leaves the test host (loopback only). When operators integrate
real KMEs they assume responsibility for the SAE authentication
and the security of the KME network.

---

## Study 3 — Crypto-Agility Audit of Fifty OSS Server Projects (Axis G)

### Inputs

- `sezar-agility` v0.3+ with the published ruleset `rules/v1`.
- Source repositories for the corpus (pinned to specific
  commits). Full corpus listed in
  `corpus/oss-50-v1.csv`.
- Rocky Linux 10 host with the corresponding packages
  installed at default versions, for the binary-side audit.
- Two human reviewers (paper authors) for ground-truth grading.

### Steps

1. **Clone corpus at pinned commits.**
   ```bash
   ./bin/sezar-corpus-fetch \
     --list corpus/oss-50-v1.csv \
     --out corpus/sources/
   # Produces corpus/sources/<project>/ each at a recorded SHA.
   ```

2. **Run agility scan on each repository.**
   ```bash
   for proj in corpus/sources/*; do
     ./bin/sezar-agility scan \
       --target "$proj" \
       --rules rules/v1 \
       --output-events corpus/results/$(basename $proj).events.json
   done
   ```

3. **Run agility scan on each installed package on the host.**
   ```bash
   ./bin/sezar-agility scan-host \
     --packages corpus/oss-50-v1.csv \
     --rules rules/v1 \
     --output-events corpus/results-host/
   ```

4. **Hand-grade the corpus.** Each reviewer independently
   assigns one of the five levels per project using the rubric.
   Inter-rater agreement is computed via Cohen's κ.
   Disagreements are resolved by joint review with reference
   to source and runtime behavior. The final grade is recorded
   in `corpus/oss-50-v1.csv` as `ground_truth_level`.

5. **Compute scanner vs. ground-truth metrics.** Per-project
   confusion matrix, overall accuracy, per-evidence
   precision/recall, false-negative and false-positive rate
   per agility level.

### Outputs

- `corpus/oss-50-v1.csv` enriched with hand-graded levels and
  reviewer notes. Public dataset (MIT licensed).
- `corpus/results/` — per-project scanner output (events).
- `corpus/results-host/` — per-package scanner output from
  installed binaries.
- A confusion matrix and aggregate metrics report.
- Published Semgrep ruleset (`rules/v1`) with documented
  patterns and per-pattern evidentiary meaning.

### Runtime envelope

Source scan: ≈45 minutes for the full corpus on a 16-core host.
Host-side scan: ≈10 minutes for installed packages. Hand
grading: ≈8 hours per reviewer for the full corpus (the
limiting factor; budget accordingly).

### Ethical considerations

The corpus consists of OSS projects under open licenses; no
private code is processed. Hand grades are stored as
reviewer-attributed notes in the public CSV and reflect
reviewer judgment, not vendor statements.

---

## Cross-Study Synthesis (§8.4)

The synthesis is the intersection of the three corpora — TLS
servers in the Tranco-top-1k that are also in the OSS-50
corpus. The expected intersection is small (≈10–25 hosts), but
sufficient to demonstrate the unified $q$ score on a real
deployed surface.

Procedure:

1. Identify hosts in `tranco-1k` whose server header or TLS ALPN
   identifies a project from `oss-50-v1.csv`. (Conservative
   identification; ambiguous matches dropped.)
2. For each match, combine the Study-1 `sezar-net` event with
   the Study-3 `sezar-agility` event into a single asset record
   via the collector's deduplication on
   `(asset.kind, asset.identity)` plus the additional
   `agility` block attribution by host.
3. Compute $q$ under default weights with $D=$ 2030-01-01.
4. Identify the top decile by $q$ as illustrative deployment
   examples. Anonymize hostnames in the publication; release
   non-anonymized data only under the operator's consent.
5. Forward-project the deadline tension over five years
   (synthetic; held-fixed observables) to demonstrate the
   agility weight's deadline-sensitivity.

---

## Reproducibility Bill of Materials

To replicate the studies exactly, an external party needs:

- The Sezar source repository at the paper's release tag.
- The `corpus/oss-50-v1.csv` corpus list with pinned commits.
- The `rules/v1` Semgrep ruleset (in the repository).
- The `scenarios/` ETSI 014 emulator replay files (in the
  repository).
- The Tranco list ID used on the scan date (recorded in the
  paper's data appendix).
- One Linux host with ≥16 GB RAM and ≥40 GB free disk space.

All artifacts are MIT-licensed. We will mirror the corpus
CSV and the Tranco list snapshot in an academic archive
(Zenodo) and assign a DOI for camera-ready citation.

---

## Risks to Reproducibility

1. **Internet-state drift.** The Tranco list rebuilds nightly;
   PQ adoption on the open web is rising. We pin the list ID
   used; subsequent re-runs by external parties will produce
   different headline numbers but the *methodology* and the
   *Sezar-side* metrics remain identical.
2. **OSS corpus drift.** We pin to commit SHAs; reviewers
   re-running the agility scan on the same SHAs will see the
   same scanner output. Re-runs on `main` may differ as
   projects evolve.
3. **Emulator divergence from real KMEs.** Our emulator is
   spec-faithful but is not a vendor-certified ETSI 014
   implementation. We document interoperability with at least
   one publicly-described vendor SAE behavior; users
   integrating real KMEs should replay our scenarios against
   the real hardware to confirm parity.
4. **Subjective agility grades.** The ground-truth grading has
   irreducible subjectivity. We mitigate by publishing
   reviewer notes and the rubric used. We expect that some
   levels will shift in v2 of the rubric.

---

## What This Plan Deliberately Excludes

- **No vendor partnerships required.** We have considered and
  rejected building the empirical case on private vendor data
  for both ethical reproducibility reasons and scope.
- **No commercial scanning service.** We rely on `zgrab2` and
  our own probe. The user retains end-to-end control of the
  scan corpus and the published data.
- **No claims about closed-source enterprise environments.**
  The studies measure observability surfaces we can publish.
  Sezar is deployable into closed environments, but we make
  no headline claims about those environments in this paper.

---

## Estimated Total Effort (calendar)

| Activity                                        | Effort        |
|-------------------------------------------------|---------------|
| Implement `sezar-qkd-kme-emulator` + replay     | 2 weeks       |
| Implement `sezar-net-tls-probe` + ingest tools  | 1 week        |
| Implement `sezar-agility` MVP + `rules/v1`      | 3 weeks       |
| Bring up SAEs and run Study 2 scenarios         | 1 week        |
| Run Study 1 scan + analysis                     | 2 days        |
| Run Study 3 source + host scans                 | 2 days        |
| Hand-grade 50 OSS projects (per reviewer)       | 1 week        |
| Draft analysis, plots, paper revision           | 2 weeks       |
| Buffer                                          | 1 week        |
| **Total**                                       | **~11 weeks** |

Plan assumes a single full-time researcher; doubles roughly
inversely with additional contributors.
