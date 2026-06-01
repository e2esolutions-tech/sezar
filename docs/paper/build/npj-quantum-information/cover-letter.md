# Cover Letter

**To:** Editorial Office, *npj Quantum Information* (Nature Portfolio)
**From:** Aleaddin Özer (Corresponding Author) · ORCID 0000-0001-9389-5357
**E-mail:** aleaddinozer@hacettepe.edu.tr
**Date:** 1 June 2026

Dear Editors,

We would like *npj Quantum Information* to consider our
manuscript *"Three Axes of Quantum Risk: A Unified
Observability Model for PQC, QKD, and Crypto-Agility"* as an
Original Research Article.

The motivation is operator-side. The post-quantum migration
is being driven now by NIST FIPS 203/204/205 and by the
NSA CNSA 2.0 schedule, but the cryptographic inventory tools
operators rely on still classify assets along one dimension:
PQ-resistant or not. We argue this is structurally
insufficient. An asset's posture also depends on the channel
through which its keys arrive (where QKD becomes a measured
property, not a separate research topic), and on how
quickly its primitive can be replaced before the deadline.
The paper formalises these as Axis A, Axis C, and Axis G,
combines them with the deadline horizon into a per-asset
score $q(\mathit{asset}, t)$, and ships an open reference
platform that observes all three.

What we think is interesting for *npj QI* readers

Quantum information research has shaped the underlying
primitives — both lattice KEMs and QKD links. The gap our
work fills is between the science and the deployment
posture: how do you measure what fraction of a real
infrastructure benefits from each, and how do you compare
two assets when one has a PQ KEM but no QKD link and the
other is classical but QKD-protected? Axis C is the part
most likely to interest QST/QI readers: a per-session
attribution of QKD-derived key material, validated on a
working ETSI GS QKD 014 emulator we release alongside
the paper, with 13 induced failure modes correctly
classified.

The empirical results are modest but reproducible. We
measured 43.8% hybrid PQ-KEM adoption (317 of 724
responsive hosts) on a Tranco-top-1k probe. We measured
91% agreement with hand-graded ground truth on an
eleven-project crypto-agility pilot drawn from a fifty-
project corpus. Every artifact — schema, implementation,
emulator, replay corpus, Semgrep ruleset, hand-graded
TSV, Tranco snapshot identifier — is open at
<https://github.com/e2esolutions-tech/sezar> under MIT.

A note on APC

Both authors are self-funding this work; we have no
external grant or industrial sponsor covering open-access
fees. If the journal can offer a discount on the Original
Research APC we would gratefully take it, but our intent
is to publish in *npj QI* regardless of the outcome of
that request.

## Submission disclosures

- **Originality.** The manuscript is not under
  consideration anywhere else and has not been published.
- **Authorship.** Both authors approved the submission.
  CRediT contributions are stated in §10 of the
  manuscript.
- **Competing interests.** None declared. E2E Solutions
  releases Sezar under MIT but earns no direct revenue
  from this publication.
- **Funding.** Self-funded by E2E Solutions and
  Hacettepe University.
- **Data and code.** See the Data and Code Availability
  block in §10 of the manuscript; everything is on
  GitHub under MIT.
- **Suggested reviewers.** Five names in
  `suggested-reviewers.md`, three of them first
  authors on cited references. No co-author or
  advisor relationship with the submitting authors.
- **Licence.** CC-BY 4.0 on acceptance.

Aleaddin Özer · Hacettepe University · E2E Solutions
Murat Aydos · Associate Professor, Hacettepe University · ORCID 0000-0002-7570-9204
