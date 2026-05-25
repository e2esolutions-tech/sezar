# Cover Letter

**To:** Editorial Office, *npj Quantum Information*
(Nature Portfolio, Springer Nature)
**Date:** 25 May 2026
**From:** Aleaddin Özer (Corresponding Author),
Chief Information Officer, E2E Solutions
<info@e2esolutions.tech> · ORCID 0000-0001-9389-5357

Dear Editors,

Please find attached our manuscript, *"Three Axes of Quantum
Risk: A Unified Observability Model for PQC, QKD, and
Crypto-Agility,"* which we are submitting as an Original
Research Article to *npj Quantum Information*.

The post-quantum transition is now an operational problem
rather than a research-only one. NIST has finalised FIPS
203, 204, and 205; the U.S. NSA's CNSA 2.0 timetable
mandates broad migration between 2030 and 2035; and a
parallel hardware track — Quantum Key Distribution
governed by ETSI GS QKD 014 — offers an orthogonal
channel-level control on high-assurance links. Yet the
tools operators use to inventory their cryptographic
posture continue to answer a single question: *"is this
asset PQ-ready?"* That framing collapses two assets with
very different migration costs into one bucket and is
silent on the channel through which their key material
travels.

This manuscript treats quantum-risk posture as a
three-axis problem — algorithmic resistance (*A*),
channel protection (*C*), and migration agility (*G*) —
and combines the three with the operator's deadline
horizon into a single deadline-adjusted score
$q(\text{asset}, t)$. The contribution is in three
layers:

1. A formal three-axis posture model with explicit
   scoring rubrics, a deadline-tension term that shrinks
   the agility weight as the migration deadline
   approaches, and a worked four-asset example.
2. An open observability platform (Sezar) implementing
   the model end to end — eBPF-based TLS observation,
   an ETSI GS QKD 014 collector with a reusable
   Key-Management-Entity emulator, a static
   crypto-agility scanner, and a unified posture rollup.
   All under MIT at
   <https://github.com/e2esolutions-tech/sezar>.
3. Three reproducible empirical studies establishing a
   first baseline on real corpora — a Tranco-top-1k
   TLS-handshake survey (43.8% hybrid PQ-KEM adoption on
   317 of 724 responsive hosts), a controlled ETSI 014
   emulator study characterising 13 induced KME/link
   failure modes (13/13 classification correctness), and
   a crypto-agility pilot with 91% agreement against
   hand-graded ground truth on an eleven-project subset
   of a fifty-project corpus.

The work sits within *npj Quantum Information*'s scope
on three counts. The QKD axis treats channel-level
quantum-secure key delivery as a first-class observable
alongside the algorithmic PQC track, rather than as a
separate concern — directly engaging quantum-information
science's deployment frontier. The reproducibility of
all three studies on commodity Linux without QKD
hardware addresses the journal's emphasis on open,
verifiable research. And the cross-disciplinary
framing — security engineering meeting quantum
information — fits Nature Portfolio's interest in work
that draws practitioners from adjacent fields into the
quantum-information conversation.

## Submission disclosures

- **Originality.** The work has not been published or
  submitted elsewhere.
- **Authorship.** All authors have read and approved
  this submission. CRediT contributions are stated in
  the manuscript.
- **Competing interests.** None declared. E2E Solutions
  develops the Sezar reference implementation released
  under the MIT License but derives no direct commercial
  revenue from its publication.
- **Funding.** Self-funded by E2E Solutions and
  Hacettepe University; no external grants supported
  this work.
- **Data and code availability.** A repository-wide
  release at <https://github.com/e2esolutions-tech/sezar>
  contains the schema, the implementation, the ETSI 014
  emulator and replay corpus, the Semgrep agility
  ruleset, the hand-graded ground-truth corpus, and the
  Tranco snapshot identifier used for Study 1.
- **Suggested reviewers.** Five names are listed in the
  accompanying `suggested-reviewers.md` file, covering
  PQ-TLS deployment (Stebila, Wiggers, Durumeric), QKD
  network engineering (Pacher), and crypto-agility / IETF
  standards (Housley). Three of the five are first
  authors on cited references. None has a co-author,
  advisor-student, or recent-grant relationship with
  either submitting author.
- **Licence.** Subject to acceptance, we will publish
  under CC-BY 4.0.

We thank the editors and the assigned reviewers for
their consideration.

Sincerely,

Aleaddin Özer (Corresponding Author)
E2E Solutions
ORCID 0000-0001-9389-5357

Murat Aydos
Hacettepe University
ORCID 0000-0002-7570-9204
