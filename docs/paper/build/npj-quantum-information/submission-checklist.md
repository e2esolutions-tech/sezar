# npj Quantum Information — Submission Checklist

**Target venue:** *npj Quantum Information* — Nature Portfolio
**Submission system:** Editorial Manager via
<https://www.nature.com/npjqi/>
**Article type:** Article (Original Research)
**Date prepared:** 2026-05-25
**Manuscript word count:** ~9,220 words (no hard limit; npj
QI "strongly recommends" concise writing)

## Files in this bundle

| File | Purpose | Required at submission? |
|---|---|---|
| `manuscript.pdf` | Main manuscript, embedded figures (Nature accepts inline placement at initial submission) | yes |
| `manuscript.md` | Canonical authoring source | optional |
| `manuscript.tex` | Pandoc-generated LaTeX (acceptance stage will require the Nature Portfolio LaTeX template; this file gives a starting point) | optional pre-acceptance |
| `cover-letter.md` | Cover letter source | optional |
| `suggested-reviewers.md` | Five-name reviewer list per Nature Portfolio policy | **yes — Nature mandatory** |
| `references.bib` | BibTeX bibliography (47 entries) | optional pre-acceptance |
| `figures/` | Seven figures, PDF + PNG per figure | yes (Nature accepts embedded at initial submission) |
| `MANIFEST.txt` | SHA-256 over every file | optional |

## Pre-submission operator checks

### Account + portal
- [ ] Open or sign in to a Nature Editorial Manager account
- [ ] Confirm corresponding author ORCID linked to the account
  (npj QI mandates this for the corresponding author)

### Document content
- [ ] Cover letter customised — corresponding author + ORCID + APC
      licence preference (CC-BY 4.0)
- [ ] Manuscript PDF opens cleanly; figures embedded
- [ ] Abstract within journal's recommended length (npj QI does
      not publish a hard limit; treat ~250 words as advisory)
- [ ] All seven figures numbered and captioned consistently
      (Fig. 1–7)
- [ ] No "[to be provided]" or "[to be added]" placeholders
      anywhere

### Mandatory Nature Portfolio submission blocks
- [x] Author Contributions — CRediT taxonomy
- [x] Competing Interests declaration
- [x] Funding statement
- [x] Data and Code Availability statement
- [x] Corresponding Author marker on the title page
- [x] ORCIDs on the title page

### Nature-specific items
- [x] Suggested reviewers — 5 names in `suggested-reviewers.md`
- [ ] Reporting summary / editorial-policy checklist —
      Nature Portfolio encourages but does not always require
      at initial submission. The Editorial Manager portal
      surfaces this if mandatory for the article type.
- [ ] APC waiver / discount request — if seeking a discount
      from the £3,090 / $4,390 / €3,690 fee, submit the
      request at the point of manuscript submission per
      Nature policy. Self-funded status (no external grants)
      is the relevant operator argument.

### Cross-checks
- [x] No placeholder text remaining
- [x] Every `[@key]` citation resolves to a `references.bib`
      entry
- [x] Numerical results consistent (43.8%, 317/724, 13/13,
      91%, 10/11, 27.6%, Tranco snapshot 6G8PX, dated
      2026-05-13, α=0.544, δ=0.392)
- [x] Cloudflare claims correctly cite the 2024 post for
      "~2%" and the 2025 post for "39% top-100k sites" /
      ">50% human-initiated"
- [x] Reference style: currently IEEE numeric `[N]`. Nature
      uses numeric superscript `^N`. The journal accepts the
      current style at initial submission; reformatting is
      an acceptance-stage step.

### Style / Turnitin readiness
- [x] style-tell sweep clean (27 style-template phrases checked)
- [x] Heading case: sentence case (Nature Portfolio convention)
- [x] Internet / Web → internet / web (modern style)
- [x] Double-space cleanup applied

### Post-acceptance work (deferred — only if accepted)
- [ ] Reformat against Nature Portfolio LaTeX template
      (`nature-template.cls` / Overleaf "Springer Nature
      LaTeX Template")
- [ ] Switch reference style to Nature numeric superscript
- [ ] Re-render figures to Nature 300 dpi colour + 600 dpi
      line-art specifications, eps or tiff (Nature's preferred
      formats)
- [ ] CRediT taxonomy assignments verified once more against
      what each author actually did during revisions
- [ ] Editorial summary / "highlights" if the editor
      requests one at revision

## Submission flow

1. Sign in to Editorial Manager at
   <https://www.nature.com/npjqi/>
2. Start a new submission, select **Article** as the
   manuscript type
3. Upload `manuscript.pdf` as the primary document
4. Upload `cover-letter.md` (or its rendered PDF) as the cover
   letter — Editorial Manager accepts both
5. In the suggested reviewers section, enter all five names
   from `suggested-reviewers.md` (the portal surfaces fields
   for name + affiliation + e-mail per reviewer)
6. Tick the data + code availability acknowledgements
7. Choose licence (CC-BY 4.0 recommended for broad reuse)
8. Submit; Editorial Manager assigns a manuscript ID; an
   acknowledgement e-mail follows within 24 hours

## After editorial feedback

If the editor returns "transferred / declined" within a few
days (Nature's editorial-board pre-screen), iterate the
abstract framing and resubmit to *QST (IOP)* using the
`docs/paper/build/quantum-science-and-technology/` bundle.
Withdrawing here and submitting elsewhere is the correct
sequence — never overlap.

If the manuscript enters peer review and reviewers return
revision requests, fix them in `manuscript.md`, rebuild via
`cd /home/tempu/sezar/docs/paper && ./build.sh extended`,
re-copy to
`npj-quantum-information/manuscript.pdf`, and re-upload.
