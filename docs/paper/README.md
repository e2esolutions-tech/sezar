# Paper: Three Axes of Quantum Risk

Working draft of the Sezar reference paper plus its build pipeline.

## Versions

- **[quantum-risk-observability.md](quantum-risk-observability.md)** — primary deliverable. IEEE S&P Magazine target, ~14 pages in WeasyPrint two-column rendering (~8 pages in IEEE 2-col format), 7 figures, IEEE-numbered bibliography.
- **[quantum-risk-observability-extended.md](quantum-risk-observability-extended.md)** — long version (~21 pages). Reserved for a future longer venue (IEEE Communications Surveys & Tutorials, ACSAC industry track) or as supplementary material.

## Authors

- **Aleaddin Özer**, Chief Information Officer, E2E Solutions. <aleaddinozer@hacettepe.edu.tr>. ORCID: [0000-0001-9389-5357](https://orcid.org/0000-0001-9389-5357).
- **Murat Aydos**, Associate Professor, Hacettepe University. ORCID: [0000-0002-7570-9204](https://orcid.org/0000-0002-7570-9204).

## Files

| Path | Purpose |
|---|---|
| [quantum-risk-observability.md](quantum-risk-observability.md) | Magazine-fit draft (primary) |
| [quantum-risk-observability-extended.md](quantum-risk-observability-extended.md) | Long-form draft (backup / future venue) |
| [methodology.md](methodology.md) | Executable empirical plan with exact commands, datasets, runtime, ethics, ~11-week effort estimate |
| [references.bib](references.bib) | BibTeX bibliography; all entries verified against live sources on 2026-05-13 |
| [ieee.csl](ieee.csl) | IEEE numeric citation style for pandoc citeproc |
| [paper.css](paper.css) | Print stylesheet for the WeasyPrint render |
| [build.sh](build.sh) | One-shot LaTeX-free PDF build pipeline (pandoc → HTML → WeasyPrint) |
| [build/](build/) | Generated PDFs + intermediate HTML and patched Markdown |
| [figures/](figures/) | Conceptual figures 1–3 (vector PDF + 200 DPI PNG) |
| [scripts/](scripts/) | Matplotlib regenerators for figures 1 and 2 |

## Figures

The paper carries seven figures in the magazine version:

| # | Source | Path |
|---|---|---|
| 1 | Three-axis cube (matplotlib mpl_toolkits.mplot3d) | `figures/three-axis-cube.{pdf,png}` |
| 2 | $q(t)$ trajectory (matplotlib) | `figures/q-trajectory.{pdf,png}` |
| 3 | Sezar reference architecture (matplotlib FancyBboxPatch) | `figures/sezar-architecture.{pdf,png}` |
| 4 | Study 1 classical-probe distribution (Tranco-1k) | `studies/study1/plots/study1-tranco-distribution.{pdf,png}` |
| 5 | Study 1 PQ-capable probe results (Tranco-1k) | `studies/study1/plots/study1-tranco-pq-kex.{pdf,png}` |
| 6 | Study 2 R3 link-health timeline | `studies/study2/plots/r3-hard-failure-timeline.{pdf,png}` |
| 7 | Study 3 agreement confusion matrix | `studies/study3/plots/study3-agreement-matrix.{pdf,png}` |

Paths in the table are repo-root-relative — the same form
the paper sources use. `build.sh` rewrites them to absolute
paths at render time, so writing them without a `../../`
prefix in markdown is the canonical convention.

Regenerate the conceptual figures (1–3) with the Python scripts under
[`scripts/`](scripts/); the empirical figures (4–7) are produced by the
`studies/*/analyse.py` scripts.

## Build → PDF (LaTeX-free)

Pipeline: pandoc renders markdown to HTML with citeproc; WeasyPrint
converts HTML to PDF using `paper.css` for the magazine layout.

Dependencies (all installable on a standard Linux box):

- `pandoc` ≥ 3.0
- `weasyprint` ≥ 60
- `python3` (only for the YAML-frontmatter rewrite in `build.sh`)
- `curl` (only on first run, to fetch `ieee.csl` if you replace it)

One-shot:

```bash
./build.sh                # builds both magazine + extended
./build.sh magazine       # magazine only
./build.sh extended       # extended only
```

Outputs land in `build/`:

```
build/
├── quantum-risk-observability.pdf            ← magazine (≈1 MB, 14 pp)
├── quantum-risk-observability.html
├── quantum-risk-observability.patched.md
├── quantum-risk-observability-extended.pdf   ← extended (≈200 KB, 21 pp)
├── quantum-risk-observability-extended.html
└── quantum-risk-observability-extended.patched.md
```

Inside `build.sh` we do four things between source markdown and the
final PDF:

1. **Patch figure refs**: `.pdf` → `.png` (publishers want vector PDFs,
   but WeasyPrint embeds rasters; the LaTeX submission keeps the
   vector paths intact in the source).
2. **Rewrite figure paths**: `figures/` → absolute path under `docs/paper/`;
   `studies/` → absolute path under repo root. WeasyPrint takes a
   single `--base-url`, so we resolve at sed time.
3. **Math fixup**: `\Bigl(/\Bigr)` → `\bigl(/\bigr)` because pandoc's
   HTML math renderer doesn't recognise the capitalised forms.
4. **YAML author rewrite**: the structured author block in the source
   (carrying affiliation/role/email/ORCID) is collapsed to two
   simple strings so pandoc's default HTML template renders names
   instead of `true`. Author bios in the body keep the full detail.

## LaTeX submission path

For a venue that wants a `.tex` source against the IEEE template
(IEEEtran, ACMart, etc.):

```bash
pandoc quantum-risk-observability.md \
  --bibliography=references.bib \
  --citeproc \
  --csl=ieee.csl \
  -t latex \
  -o paper.tex
```

Then drop `paper.tex` into the venue's IEEEtran skeleton and iterate
the figure / table macros against the venue's CSL spec.

## Worked-example numerics (verified, §3.1 of paper)

For default weights ($\alpha=0.5, \beta=0.2, \gamma_{\max}=0.3$),
$D=$ 2030-01-01, $H=5$ years:

| Asset                  | $A$  | $C$  | $G$  | $q$(2026-05-13) | $q$(2029-07-01) |
|------------------------|------|------|------|----------------:|----------------:|
| α modern-agile, no QKD | 0.51 | 0.00 | 0.75 | 0.544           | 0.620           |
| β modern-FIPS-locked   | 0.51 | 0.00 | 0.20 | 0.675           | 0.643           |
| γ legacy-pinned        | 0.12 | 0.00 | 0.50 | 0.816           | 0.897           |
| δ modern + QKD-PSK     | 0.51 | 0.70 | 0.75 | 0.392           | 0.428           |

`crates/sezar-server` runs unit tests
(`worked_example_alpha_q_matches_paper`,
`worked_example_delta_q_matches_paper`) that assert the
implementation reproduces α and δ to within 0.01.

## Status

- **Magazine draft v0.4** (2026-05-18). PDF render verified end-to-end.
  All seven figures embed correctly. Bibliography renders as IEEE
  numeric [1]–[28] with citeproc + `ieee.csl`. Two-column layout, page
  footers, page numbers — all working. Study 1 §5.1 rewritten on the
  Tranco-top-1k scan (n = 1,000, 724 responsive, 317 PQ = 43.8%);
  the 30-host curated pilot is retained as a sample-selection
  contrast.
- **Extended draft v0.3** (2026-05-18). Same PDF pipeline; §8.1.3
  rewritten with the Tranco-1k results table and pilot comparison;
  §8.1.1 methodology updated to describe the actual Python ssl +
  rustls-pq probe pair (the earlier zgrab2 plan was superseded
  before the scan ran); frontmatter author block aligned with the
  magazine version (`Aleaddin Özer` + `Murat Aydos`).
- **Authors confirmed.** Strings rendered correctly. ORCIDs landed
  (Aleaddin Özer 0000-0001-9389-5357 / Murat Aydos
  0000-0002-7570-9204).
- **References verified.** All 28 entries cross-checked against live
  sources; `microsoft-cryptotracker` previously identified as a
  fabricated name and replaced with the real Microsoft Security Blog
  post on cryptographic-posture management.

## Open decisions

- **Target venue confirmed:** IEEE S&P Magazine. The 14-page WeasyPrint
  render fits roughly 8 pages in the IEEE 2-col submission template.
- **ORCIDs:** landed 2026-05-21 — see Authors section above.
- **PQ-capable Study 1 scale-up — resolved (2026-05-13 scan).**
  Tranco-top-1k (snapshot 6G8PX, 2026-05-13) returns 317/724
  (43.8%) X25519MLKEM768 adoption on responsive hosts; raw
  captures and summary JSON in `studies/study1/`.
