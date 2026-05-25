# Suggested Reviewers — npj Quantum Information submission

npj Quantum Information requires authors to suggest 3–5
reviewers from outside the authors' institutions. The list
below covers the three subfields of the manuscript: PQ-TLS
deployment + measurement (Stebila, Wiggers, Durumeric), QKD
network engineering (Pacher), and crypto-agility / IETF
standards (Housley). Three of the five are first authors on
references we cite directly.

The corresponding author (Aleaddin Özer) and second author
(Murat Aydos) have no co-author, advisor-student, or
recent-grant relationships with any of the five names below.

---

## 1. Douglas Stebila

- **Affiliation.** Associate Professor (Faculty / Associate
  Chair, Graduate Studies), Department of Combinatorics &
  Optimization, University of Waterloo. Cross-appointed to
  the David R. Cheriton School of Computer Science and the
  Institute for Quantum Computing.
- **E-mail.** dstebila@uwaterloo.ca
- **ORCID.** Listed on the ORCID registry; not surfaced on
  the UWaterloo contact page used to verify the e-mail.
- **Expertise.** Internet cryptography protocol security
  (TLS, SSH), practical quantum-resistant cryptosystems,
  Open Quantum Safe co-founder.
- **Why this paper.** OQS ecosystem (liboqs, oqs-provider)
  is the substrate on which our Study 1 PQ-TLS
  measurements were generated. Stebila is the natural
  reviewer for the methodology of the Tranco-top-1k
  X25519MLKEM768 probe and the agility scoring's TLS
  surface.

## 2. Thom Wiggers

- **Affiliation.** Senior Cryptography Researcher, PQShield.
- **E-mail.** thom@thomwiggers.nl
- **Expertise.** Post-quantum cryptographic protocol design;
  KEMTLS, hybrid-PSK TLS, hash-based signatures. IETF
  PLANTS Working Group co-chair.
- **Why this paper.** First author on the cited IETF
  Internet-Draft `hybrid-tls-psk` underlying our §5.2
  Axis C definition of `qkd_hybrid_psk` channels.
  Best-placed to evaluate the schema's representation of
  the hybrid PQ + QKD-PSK pattern and the QKD-PSK §8.2
  emulator study.

## 3. Zakir Durumeric

- **Affiliation.** Assistant Professor of Computer Science,
  Stanford University. Co-Founder and Chief Scientist of
  Censys.
- **E-mail.** zakir@cs.stanford.edu
- **ORCID.** 0000-0002-9647-4192
- **Expertise.** Large-scale empirical Internet measurement;
  ZMap creator, Censys co-founder; foundational HTTPS/TLS
  deployment measurement.
- **Why this paper.** First author on the cited reference
  for the Internet-wide HTTPS scan methodology that our
  Tranco-top-1k probe is patterned on. Best-placed to
  audit the sampling, ethics, and non-response handling
  of Study 1.

## 4. Christoph Pacher

- **Affiliation.** Senior Scientist, Optical Quantum
  Technologies group, Center for Digital Safety & Security,
  AIT Austrian Institute of Technology (Vienna).
- **E-mail.** christoph.pacher@ait.ac.at
- **Expertise.** Quantum cryptography and QKD —
  information reconciliation, finite-key security analysis,
  continuous-variable QKD engineering; ETSI QKD
  standardisation contributor.
- **Why this paper.** Cited as the lead engineering author
  on the SECOQC Vienna QKD network. Best-placed to assess
  the realism of our ETSI GS QKD 014 emulator (§7.2,
  §8.2), the QBER / key-rate variables we expose on the
  per-link `channel_protection` block, and our induced
  failure-mode catalogue against actual deployed-network
  experience.

## 5. Russ Housley

- **Affiliation.** Founder, Vigil Security, LLC. Former
  Chair of the IETF (2007–2013) and of the IAB.
- **E-mail.** housley@vigilsec.com
- **Expertise.** Long-time IETF Security Area Director;
  prolific author of TLS, S/MIME, X.509, and CMS RFCs;
  active on post-quantum migration of PKIX and CMS.
- **Why this paper.** Author of RFC 7696, *Guidelines for
  Cryptographic Algorithm Agility*, cited as the
  operational source for our Axis G (Migration Agility).
  Best-placed to evaluate whether the five-level
  ordinal rubric (Negotiated / Configurable / Pinned /
  Locked / Frozen) maps faithfully to the agility
  characteristics RFC 7696 enumerates.

---

## Backup reviewers

In case any of the above declines or has an undisclosed
conflict, two backup names from adjacent subfields:

- **Sofía Celi** — Brave / Cloudflare (last known); active
  on TLS post-quantum deployment, hybrid PSK protocols.
- **Matthew Green** — Johns Hopkins University; applied
  cryptography, TLS security analysis, public-facing
  cryptographic education.

---

## Reviewers we ask you not to consider

None.

Submitting authors disclose that no editorial decisions
should be deferred to:

- Co-authors on any of the cited references not listed
  above.
- Anyone currently employed by E2E Solutions, Hacettepe
  University, or institutions hosting collaborative grants
  with the submitting authors.

The editor retains full discretion over reviewer
assignment.
