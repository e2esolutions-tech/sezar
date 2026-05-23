# QST Submission Checklist

**Target venue:** *Quantum Science and Technology* — IOP
Publishing
**Submission system:** Paperpal Preflight + IOP ScholarOne
**Date prepared:** 2026-05-24
**Manuscript word count:** ~9,220 words (within QST 8–15k
target range)

## Files in this bundle

| File | Purpose | Required by IOP? |
|---|---|---|
| `manuscript.pdf` | Main manuscript, single-column PDF (21 pp) | yes (initial submission) |
| `manuscript.md` | Canonical authoring source (Markdown) | optional |
| `manuscript.tex` | Pandoc-generated LaTeX (for accept-stage reformat to `iopart.cls`) | optional pre-acceptance |
| `cover-letter.pdf` | Cover letter PDF | yes |
| `cover-letter.md` | Cover letter source | optional |
| `references.bib` | BibTeX bibliography (47 entries) | optional pre-acceptance |
| `figures/` | 7 figures × (PDF + PNG) | yes |
| `MANIFEST.txt` | SHA-256 checksums | optional but operator-friendly |

## Pre-submission operator checks

Tick each box before opening the Paperpal preflight URL.

### Document content
- [ ] Cover letter text reviewed; corresponding author name + ORCID correct
- [ ] Manuscript PDF opens cleanly; no missing fonts, broken figures
- [ ] All seven figures embedded + numbered consistently (Fig. 1–7)
- [ ] Abstract length ~250 words (verify in the PDF)
- [ ] Keywords list present and venue-appropriate

### IOP-mandatory blocks
- [x] Author Contributions block present
- [x] Competing Interests declaration present
- [x] Funding statement present
- [x] Data and Code Availability statement present
- [x] Corresponding Author marker present
- [x] Author Bios with ORCID hyperlinks

### Cross-checks
- [x] No `[to be provided]`, `[to be added]`, `TBD`, `TODO`,
      `FIXME`, `XXX` placeholders anywhere
- [x] Every `[@key]` citation resolves to a `references.bib`
      entry
- [x] Numerical results consistent between abstract and body
      (43.8%, 317/724, 13/13, 91%, 10/11, 27.6%, 6G8PX,
      2026-05-13, α=0.544, δ=0.392)
- [x] Cloudflare claims correctly cite the 2024 post for
      "~2%" and the 2025 post for "39% top-100k sites" /
      ">50% human-initiated"
- [x] ORCIDs in both the title block and Author Bios

### Style / Turnitin readiness
- [x] style-tell sweep (27 style-template phrases) clean
- [x] No mechanical "first, second, third" enumerations on
      un-ordered items
- [x] No abstract-to-conclusion verbatim recycling
- [x] No four-time recital of the same tricolon

### Post-acceptance work (deferred — only if accepted)
- [ ] Reformat references with IOP-Numeric CSL instead of IEEE
- [ ] Convert `manuscript.tex` to the IOP `iopart.cls` template
- [ ] Convert figures to IOP's preferred 300 dpi / 1200 dpi line
      art specifications

## Paperpal preflight URL

<https://preflight.paperpal.com/partner/iop/QST/v2>

Upload `manuscript.pdf` as the primary document. Attach
`cover-letter.pdf` when prompted. Figures can be uploaded
individually or via the ZIP option (zip the `figures/`
directory if the portal expects it).

## After preflight feedback

Paperpal preflight returns a structured report. Common
checks it runs:
- Reference format consistency (IEEE numeric → may flag for
  IOP-Numeric — see "Post-acceptance work" above)
- Figure resolution / colorspace
- Statistical reporting style
- Affiliations / ORCID / corresponding-author markup

If preflight returns blocking issues, fix them in
`manuscript.md`, rebuild via
`cd /home/tempu/sezar/docs/paper && ./build.sh extended`,
re-copy to `quantum-science-and-technology/manuscript.pdf`,
and re-upload.
