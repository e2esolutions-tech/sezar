#!/usr/bin/env bash
# docs/paper/make-latex-zip.sh
#
# Build a self-contained LaTeX .zip of the extended manuscript for
# journals that require an editable LaTeX submission (npj QI, QST).
# The journal compiles the .tex themselves, so the output must be a
# clean standalone document with figures alongside.
#
# Pipeline: extended.md -> path-rewrite + author-block + unicode-fix
#           -> pandoc standalone .tex -> escaped-math fixup
#           -> .zip with figures/.
#
# A local xelatex compile-test runs at the end so we never ship a
# .tex that doesn't build. Requires: pandoc, xelatex (texlive),
# zip (or python3 fallback).
#
# Usage:
#   ./make-latex-zip.sh <bundle-dir>
#   # e.g. ./make-latex-zip.sh build/npj-quantum-information
#
# Output: <bundle-dir>/manuscript-latex.zip
#         (manuscript.tex + figures/*.pdf + references.bib)

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$ROOT/../.." && pwd)"
SRC="$ROOT/quantum-risk-observability-extended.md"
BIB="$ROOT/references.bib"
CSL="$ROOT/ieee.csl"

BUNDLE="${1:?usage: make-latex-zip.sh <bundle-dir>}"
BUNDLE_ABS="$(cd "$ROOT/../.." && cd "$(dirname "$BUNDLE")" 2>/dev/null && pwd)/$(basename "$BUNDLE")" || BUNDLE_ABS="$REPO_ROOT/docs/paper/$BUNDLE"
# Resolve bundle dir relative to docs/paper if not absolute
case "$BUNDLE" in
  /*) BUNDLE_ABS="$BUNDLE" ;;
  *)  BUNDLE_ABS="$ROOT/$BUNDLE" ;;
esac
mkdir -p "$BUNDLE_ABS"

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
mkdir -p "$WORK/figures"

echo "[latex-zip] staging figures..."
# Three conceptual figures live in docs/paper/figures/.
for f in three-axis-cube q-trajectory ree0xq-architecture; do
  cp "$ROOT/figures/$f.pdf" "$WORK/figures/"
done
# Four empirical figures live under studies/.
for src in \
  study1/plots/study1-tranco-distribution \
  study1/plots/study1-tranco-pq-kex \
  study2/plots/r3-hard-failure-timeline \
  study3/plots/study3-agreement-matrix ; do
  cp "$REPO_ROOT/studies/$src.pdf" "$WORK/figures/"
done

echo "[latex-zip] preparing markdown (flat figure paths + author block + unicode)..."
# Flatten figure paths so includegraphics finds them in figures/.
sed -E \
  -e 's|studies/study1/plots/|figures/|g' \
  -e 's|studies/study2/plots/|figures/|g' \
  -e 's|studies/study3/plots/|figures/|g' \
  "$SRC" > "$WORK/src.md"

# Author-block rewrite (extended variant) + bare-unicode → LaTeX math.
python3 - "$WORK/src.md" <<'PYEOF'
import re, sys
path = sys.argv[1]
src = open(path).read()

# Author block: title-page names with ORCIDs + plain-asterisk
# corresponding-author marker (renders in every font).
new_author = """author:
  - "Aleaddin Özer* (Chief System Engineer, E2E Solutions, ORCID 0000-0001-9389-5357)"
  - "Murat Aydos (Assoc. Prof., Hacettepe University, ORCID 0000-0002-7570-9204)"
  - "*Corresponding author: ozer@e2esolutions.tech"
"""
src = re.sub(r"^author:\n(?:[ -].*\n)+", new_author, src, count=1, flags=re.MULTILINE)

# Box-drawing glyphs (the §7.1 directory tree) → ASCII. Latin Modern
# Mono has no box-drawing coverage.
for u, a in {'─':'-','│':'|','├':'+','└':'+','┬':'+','┌':'+','┐':'+',
             '┘':'+','┤':'+','┴':'+','┼':'+'}.items():
    src = src.replace(u, a)

# Bare Greek + math symbols in prose → inline math. Protect code and
# existing math first so we don't double-wrap.
blocks = []
def stash(m):
    blocks.append(m.group(0)); return f"@@B{len(blocks)-1}@@"
src = re.sub(r'```[\s\S]*?```', stash, src)
src = re.sub(r'`[^`\n]+`', stash, src)
src = re.sub(r'\$\$[\s\S]*?\$\$', stash, src)
src = re.sub(r'\$[^$\n]+\$', stash, src)
for u, l in {'α':r'$\alpha$','β':r'$\beta$','γ':r'$\gamma$','δ':r'$\delta$',
             'τ':r'$\tau$','≈':r'$\approx$','≤':r'$\le$','≥':r'$\ge$',
             '→':r'$\to$','×':r'$\times$'}.items():
    src = src.replace(u, l)
for i, b in enumerate(blocks):
    src = src.replace(f"@@B{i}@@", b)

open(path, "w").write(src)
PYEOF

echo "[latex-zip] pandoc → standalone LaTeX..."
pandoc "$WORK/src.md" \
  --bibliography="$BIB" --citeproc --csl="$CSL" \
  --standalone \
  --metadata reference-section-title="References" \
  -V documentclass=article -V geometry:margin=1in -V fontsize=11pt \
  -o "$WORK/manuscript.tex"

echo "[latex-zip] fixing pandoc-escaped math fragments..."
# pandoc escapes the $\le$-style substitutions above as \$\le\$ when
# they sit flush against a digit or paren; unescape to \ensuremath.
python3 - "$WORK/manuscript.tex" <<'PYEOF'
import sys
path = sys.argv[1]
src = open(path).read()
for bad, good in {
    r'\$\le\$':r'\ensuremath{\le}', r'\$\ge\$':r'\ensuremath{\ge}',
    r'\$\approx\$':r'\ensuremath{\approx}', r'\$\to\$':r'\ensuremath{\to}',
    r'\$\times\$':r'\ensuremath{\times}', r'\$\alpha\$':r'\ensuremath{\alpha}',
    r'\$\beta\$':r'\ensuremath{\beta}', r'\$\gamma\$':r'\ensuremath{\gamma}',
    r'\$\delta\$':r'\ensuremath{\delta}', r'\$\tau\$':r'\ensuremath{\tau}',
}.items():
    src = src.replace(bad, good)

# Figure captions: the markdown carries a literal "**Figure N.**"
# prefix (so the WeasyPrint render is numbered), but LaTeX's
# \caption already auto-numbers as "Figure N:". Strip the literal
# prefix here so the LaTeX render is not double-numbered
# ("Figure 1: Figure 1. ...").
import re as _re
src = _re.sub(r'\\caption\{\\textbf\{Figure \d+\.\}\s*', r'\\caption{', src)

# Title-page layout: print the keyword list under the abstract and
# force the body to start on a fresh page, so page 1 is title +
# abstract + keywords only. Matches what a research-journal review
# PDF is expected to look like; the journal's own class file
# reproduces this at the typesetting stage.
keywords = ("post-quantum cryptography; quantum key distribution; "
            "crypto-agility; observability; cryptographic posture; "
            "NIST PQC standardization")
kw_block = (r"\end{abstract}" "\n"
            r"\medskip\noindent\textbf{Keywords:} " + keywords + "\n"
            r"\clearpage")
src = src.replace(r"\end{abstract}", kw_block, 1)

open(path, "w").write(src)
PYEOF

cp "$BIB" "$WORK/references.bib"

echo "[latex-zip] compile-test (xelatex, twice for refs)..."
( cd "$WORK"
  xelatex -interaction=nonstopmode manuscript.tex >/dev/null 2>&1 || true
  xelatex -interaction=nonstopmode manuscript.tex >compile.log 2>&1 || true )
if grep -qiE "^! |emergency stop|fatal error" "$WORK/compile.log"; then
  echo "[latex-zip] ERROR: LaTeX compile reported a fatal error:" >&2
  grep -iE "^! |emergency stop|fatal error" "$WORK/compile.log" | head >&2
  exit 1
fi
MISSING=$(grep -ciE "Missing character" "$WORK/compile.log" || true)
PAGES=$(pdfinfo "$WORK/manuscript.pdf" 2>/dev/null | awk '/Pages/{print $2}')
echo "[latex-zip] compile OK — ${PAGES} pages, ${MISSING} missing-char warnings"
if [ "${MISSING:-0}" -ne 0 ]; then
  echo "[latex-zip] WARNING: ${MISSING} missing-char warnings remain (font gaps)" >&2
fi

echo "[latex-zip] packaging zip..."
OUT="$BUNDLE_ABS/manuscript-latex.zip"
rm -f "$OUT"
( cd "$WORK"
  if command -v zip >/dev/null 2>&1; then
    zip -q -r "$OUT" manuscript.tex references.bib figures/
  else
    python3 -c "import zipfile,glob,os; z=zipfile.ZipFile('$OUT','w',zipfile.ZIP_DEFLATED); [z.write(f) for f in ['manuscript.tex','references.bib']+glob.glob('figures/*')]; z.close()"
  fi )
# Drop the compiled PDF next to the zip for a quick visual check.
cp "$WORK/manuscript.pdf" "$BUNDLE_ABS/manuscript-latex-preview.pdf"

echo "[latex-zip] wrote $OUT"
echo "[latex-zip] wrote $BUNDLE_ABS/manuscript-latex-preview.pdf (compile preview)"
unzip -l "$OUT"
