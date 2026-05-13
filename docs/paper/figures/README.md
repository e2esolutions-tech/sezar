# Figures — Specifications

This document specifies the three figures referenced in
`quantum-risk-observability.md`. Each figure should be produced
as a vector PDF (or SVG with PDF export) and placed in this
directory under the listed filename. The Markdown source already
references them with the correct paths.

Recommended toolchain:
- **TikZ / pgfplots** if you intend to integrate cleanly with a
  LaTeX submission template (IEEEtran, acmart). Renders identical
  on every reviewer's machine.
- **Matplotlib** (Python) for `q-trajectory.pdf` — the math is
  already in `scripts/q-trajectory.py` (see below).
- **draw.io / Excalidraw → PDF** for `sezar-architecture.pdf`
  — fastest path; export to vector.

Style guidance for IEEE S&P Magazine:
- Single column width ≈ 3.5 in, double column width ≈ 7.25 in.
- Sans-serif labels (the body text is serif; figures cleaner in
  sans).
- ≥10 pt font in the published size — many figures lose
  legibility because authors size for the source PDF, not the
  rendered column.
- Avoid red+green as the only colour distinction (colour-blind
  readers); use shape + colour together.
- B&W readable: every figure should still read on grayscale.

---

## Figure 1 — `three-axis-cube.pdf`

**Caption** (final): "The three-axis quantum-risk space. Each
cryptographic asset occupies a point in (A, C, G); colour
encodes the deadline-adjusted score q. The four worked-example
assets (α, β, γ, δ) are plotted at their observed coordinates.
The shaded region near the A=0, G=0 corner is the BLOCKED-flagged
volume requiring out-of-band remediation."

**Content**:
- 3D cube with axes labelled A (algorithmic resistance), C
  (channel protection), G (migration agility). Each axis
  spans [0, 1].
- Background gradient (or contour shading) encoding $q$ at
  $\tau=0.27$ (2026-05-13): green near (1,1,1), red near
  (0,0,0).
- Four data points labelled with Greek letters and a one-line
  legend:
  - α at (0.51, 0.0, 0.75) — medium-green
  - β at (0.51, 0.0, 0.20) — orange
  - γ at (0.12, 0.0, 0.50) — red
  - δ at (0.51, 0.70, 0.75) — green
- A hatched / shaded sub-volume in the {A ≤ 0.30} ∧ {G ≤ 0.20}
  octant labelled `BLOCKED`.
- Camera angle: standard 3D isometric, slight tilt to show
  all three axes legibly.

**Suggested implementation**: TikZ's 3D primitives, or
matplotlib's mpl_toolkits.mplot3d. A TikZ template skeleton:

```latex
\begin{tikzpicture}[scale=1.2]
  \begin{axis}[
    view={30}{30},
    xlabel={$A$ — algorithmic resistance},
    ylabel={$C$ — channel protection},
    zlabel={$G$ — migration agility},
    xmin=0, xmax=1, ymin=0, ymax=1, zmin=0, zmax=1,
  ]
  \addplot3[only marks, mark=*, mark size=4pt] coordinates {
    (0.51, 0.00, 0.75) (0.51, 0.00, 0.20)
    (0.12, 0.00, 0.50) (0.51, 0.70, 0.75)};
  % labels, BLOCKED shading, etc.
  \end{axis}
\end{tikzpicture}
```

---

## Figure 2 — `q-trajectory.pdf`

**Caption** (final): "Trajectory of q(t) for the four
worked-example assets from 2026 to the deadline at 2030-01-01,
holding observables fixed. The legacy-pinned asset (γ) climbs
steepest; the modern-agile asset (α) enters the must-migrate
band as its agility weight erodes. The locked-but-modern asset
(β) remains high throughout — the BLOCKED flag, not the
prioritization score, is what surfaces it for action."

**Content**:
- X-axis: date, 2026-01-01 to 2030-01-01.
- Y-axis: $q \in [0, 1]$.
- Four labelled lines (α, β, γ, δ) drawn from the formula in §3.
- Horizontal dashed lines at $q=0.3$ ("plan migration") and
  $q=0.6$ ("must migrate") with text labels.
- Vertical dashed line at 2029-07-01 marking the worked-example
  $t_2$ checkpoint.
- A small marker on β indicating `BLOCKED` (e.g., a hashed-pattern
  segment along the line, or a separate small icon and legend
  entry).
- Legend in upper-left or upper-right depending on overlap.

**Reference implementation** (see `scripts/q-trajectory.py`):
```python
from datetime import date, timedelta
import matplotlib.pyplot as plt
import numpy as np

D = date(2030, 1, 1); H = 5.0

def tau(t): return max(0.0, min(1.0, 1.0 - (D-t).days/365.25/H))

def q(A, C, G, t):
    tv = tau(t)
    g = 0.3 * (1 - tv); s = 0.5 + 0.2 + g
    a, b, gn = 0.5/s, 0.2/s, g/s
    return 1.0 - (a*A + b*C + gn*G)

assets = [("α modern-agile",      0.51, 0.00, 0.75),
          ("β modern-locked",     0.51, 0.00, 0.20),
          ("γ legacy-pinned",     0.12, 0.00, 0.50),
          ("δ modern-QKD",        0.51, 0.70, 0.75)]

dates = [date(2026,1,1) + timedelta(days=d) for d in range(0, 365*4, 14)]
fig, ax = plt.subplots(figsize=(7, 4))
for label, A, C, G in assets:
    ax.plot(dates, [q(A, C, G, t) for t in dates], label=label, linewidth=2)
ax.axhline(0.6, ls='--', color='gray'); ax.axhline(0.3, ls='--', color='gray')
ax.set_ylabel("q(asset, t)"); ax.set_ylim(0, 1)
ax.legend(loc='upper left'); plt.tight_layout()
plt.savefig("q-trajectory.pdf")
```

---

## Figure 3 — `sezar-architecture.pdf`

**Caption** (final): "Sezar reference architecture. Five agents
(sezar-net, sezar-qkd, sezar-agility, plus sezar-cert and
sezar-chain/sezar-id in later phases) emit
crypto_inventory_event records into sezar-server. The shared
sezar-core library hosts the schema, the classification table,
and the deadline-adjusted rollup. The React dashboard renders
the three-axis posture matrix and the priority-sorted action
list."

**Content**:
- Five agent boxes at the top of the figure, labelled with the
  primary data source for each:
  - `sezar-net` — eBPF (TLS / SSH / IPsec wire)
  - `sezar-qkd` — ETSI GS QKD 014 (KME REST API)
  - `sezar-agility` — Semgrep over source / installed packages
  - `sezar-cert` (V2) — CT logs + host scan
  - `sezar-chain` (V3) / `sezar-id` (V4) — chain RPC + KMS APIs
- A single arrow from each agent merging into a labelled bus
  `crypto_inventory_event v1.1` flowing down to a `sezar-server`
  box (axum collector + REST API + DB).
- An adjacent box `sezar-core` (shared library) connected to all
  agents (showing it computes posture client-side and is
  re-used).
- Below `sezar-server`: a Postgres + columnar DB pair.
- Right of `sezar-server`: a React dashboard box, with three
  small sub-panel labels: "three-axis matrix", "priority queue",
  "BLOCKED inventory".
- Dashed arrows from `sezar-server` to dashboard.

**Suggested implementation**: draw.io or Excalidraw, export as
vector PDF. Keep boxes rectangular, monospace labels inside
boxes, sans-serif for axis-style annotations.

---

## Notes for the figure designer

Once figures are finalised, replace the `.pdf` placeholders in
this directory with the rendered output. The Markdown references
(`![…](figures/<filename>.pdf)`) will resolve automatically when
pandoc / LaTeX renders the paper.

If submitting to a venue that requires LaTeX source rather than
Markdown, the easiest path is to keep these PDFs as the figure
artifacts and `\includegraphics{figures/…}` them from the
generated LaTeX.
