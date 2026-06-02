# Cover Letter

**To:** Editorial Office, *npj Quantum Information* (Nature Portfolio)\
**From:** Aleaddin Özer (Corresponding Author) · ORCID 0000-0001-9389-5357\
**E-mail:** ozer@e2esolutions.tech\
**Date:** 2 June 2026

Dear Editors,

Please consider our manuscript, *"Three Axes of Quantum
Risk: A Unified Observability Model for PQC, QKD, and
Crypto-Agility,"* as an Original Research Article for
*npj Quantum Information*.

The work started from a practical frustration. NIST has
finalised its post-quantum standards and the NSA has put
calendar deadlines on the migration, so operators are now
being asked to inventory their cryptography and act on it.
The tools they have for that job report a single bit per
asset: PQ-resistant, or not. In practice that bit hides
most of what an operator needs to decide where to spend a
migration budget. Two TLS terminators can both read
"classical," yet one is a configuration change away from a
hybrid key exchange while the other is locked to a FIPS
firmware build that will not move for eighteen months.
And the single-bit view has nothing at all to say about
quantum key distribution, which protects the channel rather
than the primitive and is being deployed on exactly the
high-assurance links where the migration matters most.

We make the case that quantum-risk posture is a three-axis
measurement, not a one-axis label. The axes are algorithmic
resistance, the protection on the channel that carries the
key material, and how quickly the primitive can actually be
replaced. We combine them with the operator's own deadline
into a single score and — this is the part we care about —
we build the instrumentation that observes all three on
real systems, and we run it.

We think the channel axis is where the manuscript speaks
most directly to this journal. QKD is usually studied as a
physical-layer or protocol question; here it appears as a
measured property of a deployed link, attributed per
session, and validated on an open ETSI GS QKD 014 emulator
that reproduces the link and key-management failure modes a
real deployment would see. We drove thirteen such failures
and the channel-state classification was correct on every
one, including the case where one key-management endpoint
fails while its healthy pair keeps delivering keys — a
situation link-level telemetry alone cannot resolve. For a
readership that has built the QKD systems being deployed, we
hope the question "how would an operator actually observe
this in production?" is a useful one to put on the table.

The empirical results are deliberately modest and fully
reproducible. On the Tranco top-1000 we measured 43.8% of
responsive hosts negotiating a hybrid PQ key exchange when
offered one. On an eleven-project crypto-agility pilot we
reached 91% agreement with hand-graded ground truth. None
of the studies needs QKD hardware or a commercial scanner;
they run on one Linux host. Everything — the schema, the
implementation, the emulator and its replay corpus, the
agility rule pack, the hand-graded data, and the Tranco
snapshot identifier — is released under the MIT License at
<https://github.com/e2esolutions-tech/sezar>.

On open-access fees: both authors are self-funding this
work, with no grant or industrial sponsor behind it. We
would be grateful for any waiver or discount the journal
can extend, and we are submitting in the hope of publishing
with *npj Quantum Information* either way.

The usual declarations: the manuscript is original and not
under consideration elsewhere; both authors approved it and
their contributions are stated in the back matter; we
disclose one competing interest — the corresponding author
is Chief System Engineer of E2E Solutions, which develops
the open-source Sezar implementation described here, though
the company earns no direct revenue from publication; the
work was self-funded; and the data and code availability
statement points to the open repository above. We suggest
five reviewers in the accompanying file, three of them
first authors on work we cite, none with any co-author or
advisory tie to us. On
acceptance we would publish under CC-BY 4.0.

Thank you for your time and for considering the manuscript.

Yours sincerely,

Aleaddin Özer\
E2E Solutions · Hacettepe University\
ORCID 0000-0001-9389-5357

Murat Aydos\
Associate Professor, Hacettepe University\
ORCID 0000-0002-7570-9204
