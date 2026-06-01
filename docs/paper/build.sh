#!/usr/bin/env bash
# docs/paper/build.sh
#
# Render the Sezar paper to PDF via the LaTeX-free pipeline:
#
#   .md  --pandoc--->  .html  --weasyprint-->  .pdf
#
# Citation rendering: pandoc + citeproc + ieee.csl.
# Figures: source markdown references .pdf vector figures (the
# preferred form for a real LaTeX submission); for HTML/PDF
# rendering we substitute the .png siblings each plot script writes.
#
# Usage:
#   ./build.sh                # builds both magazine + extended
#   ./build.sh magazine       # builds just the magazine version
#   ./build.sh extended       # builds just the extended version

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$ROOT/../.." && pwd)"
BUILD="$ROOT/build"
mkdir -p "$BUILD"

CSS="$ROOT/paper.css"
BIB="$ROOT/references.bib"
CSL="$ROOT/ieee.csl"

build_one() {
  local kind="$1"
  local md_in
  case "$kind" in
    magazine)
      md_in="$ROOT/quantum-risk-observability.md"
      ;;
    extended)
      md_in="$ROOT/quantum-risk-observability-extended.md"
      ;;
    *)
      echo "unknown kind: $kind" >&2
      return 1
      ;;
  esac
  local stem
  stem="$(basename "${md_in%.md}")"
  local md_patched="$BUILD/$stem.patched.md"
  local html_out="$BUILD/$stem.html"
  local pdf_out="$BUILD/$stem.pdf"

  echo "[paper] preparing $stem..."

  # 1) Patch the markdown for HTML / WeasyPrint rendering:
  #    (a) figure refs .pdf → .png (the LaTeX source keeps .pdf;
  #        for HTML we use the .png siblings each plot script writes)
  #    (b) `figures/...` paths get the paper dir prefix so they
  #        resolve from the build directory
  #    (c) `studies/...` paths get an absolute prefix so they
  #        resolve from anywhere (figures live two levels up)
  #    (d) math escapes \Bigl( / \Bigr) → \bigl/\bigr since
  #        pandoc's HTML math renderer doesn't recognise the
  #        capitalised forms
  sed -E \
    -e 's|(figures/[A-Za-z0-9._-]+)\.pdf|\1.png|g' \
    -e 's|(studies/[A-Za-z0-9./_-]+)\.pdf|\1.png|g' \
    -e "s|]\(figures/|](${ROOT}/figures/|g" \
    -e "s|]\(studies/|](${REPO_ROOT}/studies/|g" \
    -e 's|\\Bigl\(|\\bigl(|g' \
    -e 's|\\Bigr\)|\\bigr)|g' \
    "$md_in" > "$md_patched"

  # Rewrite YAML structured author block to simple strings so pandoc's
  # default HTML template renders authors instead of "true".
  #
  # The magazine version keeps the title page minimal and carries
  # ORCIDs in the Author Bios section. The extended version drops
  # Author Bios entirely (research-journal convention) and folds
  # the ORCIDs into the title page author lines.
  python3 - "$md_patched" <<'PYEOF'
import re, sys
path = sys.argv[1]
src = open(path).read()
if "extended" in path:
    new_author = """author:
  - "Aleaddin Özer✱ (CIO, E2E Solutions, ORCID 0000-0001-9389-5357)"
  - "Murat Aydos (Assoc. Prof., Hacettepe University, ORCID 0000-0002-7570-9204)"
  - "✱ Corresponding author: ozer@e2esolutions.tech"
"""
else:
    new_author = """author:
  - "Aleaddin Özer✱ (CIO, E2E Solutions)"
  - "Murat Aydos (Assoc. Prof., Hacettepe University)"
  - "✱ Corresponding author: ozer@e2esolutions.tech"
"""
src = re.sub(
    r"^author:\n(?:[ -].*\n)+",
    new_author,
    src,
    count=1,
    flags=re.MULTILINE,
)
open(path, "w").write(src)
PYEOF

  # 2) Pandoc → standalone HTML with citeproc + CSS link.
  pandoc "$md_patched" \
    --from=markdown+yaml_metadata_block+pipe_tables+footnotes+raw_html \
    --to=html5 \
    --standalone \
    --citeproc \
    --bibliography="$BIB" \
    --csl="$CSL" \
    --metadata link-citations=true \
    --metadata reference-section-title="References" \
    --css="$CSS" \
    --resource-path="$ROOT:$REPO_ROOT" \
    --output="$html_out"

  # 3) WeasyPrint → PDF. `--base-url=$REPO_ROOT` so relative figure
  #    references like `studies/study1/plots/...png` resolve.
  weasyprint \
    --base-url="$REPO_ROOT" \
    --presentational-hints \
    "$html_out" \
    "$pdf_out"

  echo "[paper] wrote $pdf_out"
}

if [[ $# -eq 0 ]]; then
  build_one magazine
  build_one extended
else
  for kind in "$@"; do
    build_one "$kind"
  done
fi

ls -lah "$BUILD"/*.pdf 2>/dev/null
