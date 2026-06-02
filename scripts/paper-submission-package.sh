#!/usr/bin/env bash
# scripts/paper-submission-package.sh — bundle the paper
# for venue submission.
#
# Builds a fresh PDF, copies the source Markdown,
# references.bib, every figure (PDF + PNG), and a
# generated cover letter + submission checklist into a
# self-contained staging dir under `dist/paper/`, then
# zips the dir for upload.
#
# Usage:
#   scripts/paper-submission-package.sh           # magazine
#   scripts/paper-submission-package.sh magazine
#   scripts/paper-submission-package.sh extended
#   scripts/paper-submission-package.sh both      # both bundles
#
# Each bundle is named `<variant>-<YYYY-MM-DD>` and the zip
# lands as `dist/paper/<variant>-<YYYY-MM-DD>.zip`.
#
# Defaults to the magazine variant (IEEE S&P Magazine
# target per docs/paper/README.md).

set -euo pipefail

VARIANT="${1:-magazine}"
case "$VARIANT" in
  magazine|extended|both) ;;
  *) echo "usage: $0 [magazine|extended|both]" >&2; exit 1 ;;
esac

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

DATE="$(date -u +%Y-%m-%d)"
PAPER_DIR="$ROOT/docs/paper"
OUT_ROOT="$ROOT/dist/paper"
mkdir -p "$OUT_ROOT"

if [[ -t 1 ]]; then
  R=$'\033[31m'; G=$'\033[32m'; Y=$'\033[33m'; B=$'\033[34m'; N=$'\033[0m'
else
  R=""; G=""; Y=""; B=""; N=""
fi
step()  { printf "%s[paper-pkg]%s %s\n" "$B" "$N" "$*"; }
ok()    { printf "  %s✓%s %s\n" "$G" "$N" "$*"; }
fail()  { printf "  %s✗%s %s\n" "$R" "$N" "$*" >&2; exit 1; }

# ----- build PDFs ------------------------------------------------------------

step "rebuilding paper PDFs via build.sh"
( cd "$PAPER_DIR" && ./build.sh ) >/dev/null
ok "PDFs rebuilt"

# ----- per-variant bundler ---------------------------------------------------

bundle_one() {
  local variant="$1"
  local name pdf
  case "$variant" in
    magazine)
      name="quantum-risk-observability"
      pdf="$PAPER_DIR/build/quantum-risk-observability.pdf"
      venue="IEEE Security & Privacy Magazine"
      ;;
    extended)
      name="quantum-risk-observability-extended"
      pdf="$PAPER_DIR/build/quantum-risk-observability-extended.pdf"
      venue="ACM IMC / NDSS / USENIX Security (extended track)"
      ;;
  esac

  local stage="$OUT_ROOT/${variant}-${DATE}"
  step "staging $variant bundle at ${stage#$ROOT/}"
  rm -rf "$stage"
  mkdir -p "$stage/figures"

  # Main PDF + source Markdown + references.
  install -m 0644 "$pdf" "$stage/"
  install -m 0644 "$PAPER_DIR/$name.md" "$stage/"
  install -m 0644 "$PAPER_DIR/references.bib" "$stage/"
  install -m 0644 "$PAPER_DIR/ieee.csl" "$stage/" 2>/dev/null || true

  # Conceptual figures 1-3 (paper/figures/).
  for fig in three-axis-cube q-trajectory sezar-architecture; do
    for ext in pdf png; do
      [ -f "$PAPER_DIR/figures/$fig.$ext" ] && \
        install -m 0644 "$PAPER_DIR/figures/$fig.$ext" "$stage/figures/"
    done
  done

  # Empirical figures 4-7 (studies/).
  for src in \
    "studies/study1/plots/study1-tranco-distribution" \
    "studies/study1/plots/study1-tranco-pq-kex" \
    "studies/study2/plots/r3-hard-failure-timeline" \
    "studies/study3/plots/study3-agreement-matrix"; do
    for ext in pdf png; do
      [ -f "$ROOT/$src.$ext" ] && \
        install -m 0644 "$ROOT/$src.$ext" "$stage/figures/"
    done
  done

  # LaTeX-template-ready source — pandoc → .tex. Authors who
  # need IEEEtran-style submission paste this into the
  # venue's template; the substitute does the bibliography
  # via citeproc so the .tex is mostly self-contained.
  if command -v pandoc >/dev/null 2>&1; then
    ( cd "$stage" && \
      pandoc "$name.md" \
        --bibliography=references.bib \
        --citeproc \
        --csl=ieee.csl \
        -t latex -s -o "$name.tex" \
        2>/dev/null ) && ok "pandoc → $name.tex" || true
  fi

  # Cover letter template. Operators fill in editor + venue
  # specifics before submission; the template's content is
  # invariant across venues.
  cat > "$stage/cover-letter.md" <<EOF
# Cover letter — $venue

**Date:** $DATE
**Manuscript title:** Three Axes of Quantum Risk: Why "PQ-Ready" Is Not Enough
**Variant:** $variant ($(grep -E "^# 1\.|^## " "$stage/$name.md" 2>/dev/null | head -1 | sed 's/^# *//; s/^## *//'))
**Corresponding author:** Aleaddin Özer (ORCID 0000-0001-9389-5357), Chief Information Officer, E2E Solutions <info@e2esolutions.tech>
**Co-author:** Murat Aydos (ORCID 0000-0002-7570-9204), Associate Professor, Hacettepe University

Dear Editor,

We submit the attached manuscript, "Three Axes of Quantum
Risk: Why \`PQ-Ready\` Is Not Enough," for consideration at
*$venue*.

The paper argues that current PQC discovery tooling treats
algorithmic resistance as the only observable, and that
this single-axis framing hides operationally significant
differences between assets that look identical on the
wire. We propose a three-axis posture model
(algorithmic resistance A, channel protection C, migration
agility G), combine them with a deadline-adjusted
quantum-risk score, and validate the framework through
three reproducible empirical studies:

1. A Tranco-top-1k TLS handshake survey (n = 1,000;
   724 responsive; 43.8% PQ-KEM adoption observed when
   the client advertises X25519MLKEM768).
2. A controlled ETSI GS QKD 014 KME emulator study
   (5 replay scenarios; 13/13 induced transitions
   correctly classified; observation latency p50 = 0.71 s
   against a 1 s poll interval).
3. A static crypto-agility audit of 11 widely deployed
   open-source server projects (10/11 agreement with a
   hand-graded ground truth; Cohen's κ = 0.62).

All artefacts described — schema, reference implementation,
ETSI 014 KME emulator, Semgrep agility ruleset, and the
hand-graded OSS corpus — are released under MIT at
<https://github.com/e2esolutions-tech/sezar> and run on a
single Linux host with no QKD hardware and no commercial
scanner.

The work has not been published elsewhere and is not
under consideration at any other venue.

Sincerely,
Aleaddin Özer
Chief Information Officer, E2E Solutions
EOF

  # Submission checklist.
  cat > "$stage/submission-checklist.md" <<EOF
# Submission checklist — $venue

Generated $DATE by scripts/paper-submission-package.sh.

## Files in this bundle

| File | Purpose |
|---|---|
| $name.pdf | Manuscript (final rendered PDF; $(pdfinfo "$stage/$name.pdf" 2>/dev/null | awk '/^Pages/ {print $2}') pages) |
| $name.md | Source Markdown |
| $name.tex | LaTeX source (pandoc-rendered for IEEEtran templates) |
| references.bib | BibTeX bibliography (28 entries, all live-source-verified) |
| ieee.csl | IEEE numeric CSL style (used by citeproc for the PDF render) |
| figures/three-axis-cube.{pdf,png} | Figure 1 — three-axis cube |
| figures/q-trajectory.{pdf,png} | Figure 2 — q(t) trajectory |
| figures/sezar-architecture.{pdf,png} | Figure 3 — reference architecture |
| figures/study1-tranco-distribution.{pdf,png} | Figure 4 — Study 1 baseline distribution |
| figures/study1-tranco-pq-kex.{pdf,png} | Figure 5 — Study 1 PQ adoption |
| figures/r3-hard-failure-timeline.{pdf,png} | Figure 6 — Study 2 R3 timeline |
| figures/study3-agreement-matrix.{pdf,png} | Figure 7 — Study 3 agreement matrix |
| cover-letter.md | Editor cover letter template (operator fills in venue specifics) |
| MANIFEST.txt | SHA-256 checksums of every file in this bundle |

## Pre-submission checks

- [ ] Cover letter customised (editor name, manuscript-tracking ID if revision)
- [ ] Author photos + bios attached if venue requires (the magazine PDF already includes 60-word bios)
- [ ] Conflict-of-interest declaration prepared if venue requires
- [ ] Funding-disclosure statement matches grants list
- [ ] Page count fits venue limit ($venue: 8 pages IEEE 2-col for magazine; ~21 pages full for IMC/NDSS extended)
- [ ] No-AI-attribution rule honoured: the manuscript text contains no AI-assistant mentions, no co-author trailers, no machine-generated boilerplate

## Suggested venues this variant fits

$venue

EOF

  # MANIFEST with SHA-256 over every file.
  ( cd "$stage" && find . -type f ! -name MANIFEST.txt -print0 \
        | sort -z \
        | xargs -0 sha256sum > MANIFEST.txt )

  # Zip. Prefer the `zip` binary when present (smaller
  # archives, deterministic ordering); fall back to
  # python3's stdlib zipfile module so the script works
  # on a stock Linux install.
  local archive="$OUT_ROOT/${variant}-${DATE}.zip"
  rm -f "$archive"
  if command -v zip >/dev/null 2>&1; then
    ( cd "$OUT_ROOT" && zip -qr "${variant}-${DATE}.zip" "${variant}-${DATE}" )
  else
    ( cd "$OUT_ROOT" && \
      python3 -m zipfile -c "${variant}-${DATE}.zip" "${variant}-${DATE}" )
  fi

  ok "bundle ready: ${archive#$ROOT/}"
  local size; size=$(du -h "$archive" | awk '{print $1}')
  local files; files=$(python3 -c "import zipfile,sys; print(len(zipfile.ZipFile(sys.argv[1]).namelist()))" "$archive")
  printf "    %s, %d files\n" "$size" "$files"
}

case "$VARIANT" in
  both)
    bundle_one magazine
    bundle_one extended
    ;;
  *)
    bundle_one "$VARIANT"
    ;;
esac

step "done"
echo
echo "  Upload candidate(s) under: $OUT_ROOT/"
echo "  Tip: open the staging dir to tweak the cover letter before zipping the final"
echo "       submission, or re-run with one bundle changed."
