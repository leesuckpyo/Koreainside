# Korea Inside — Thai SEO Batch 2 Final Codex Implementation Instruction

**Date:** 2026-10-03  
**Status:** FINAL IMPLEMENTATION INSTRUCTION  
**User approval:** Batch 2 CONTENT LOCK + HTML implementation approved  
**Target branch:** `th-localization-2026-10-03`  
**Reference Thai HEAD at approval time:** `01b93d4e30733427608ab862b8627357cc1a2d6a`  
**Authorized phase:** Approved Copy exact implementation → static QA  
**NOT authorized in this instruction:** stage / commit / push / merge / Production / final internal-link closure

---

# 0. Core rule

This is an **implementation task, not a translation or editorial task**.

Codex must implement the already-approved Thai public copy exactly.

Priority for wording:

1. Thai **Approved Public Copy — CONTENT LOCKED**
2. Page-specific implementation instructions in this document
3. Current English Production HTML for structure / facts / technical shell only
4. Thai Standard / Public Master / Navigation Standard

If English wording and Approved Thai wording differ stylistically, **Approved Thai wins**.

Do not change Approved Thai merely because English is phrased differently.

---

# 1. Required pre-checks before touching any file

## 1.1 Confirm branch

Must be on:

`th-localization-2026-10-03`

Reference HEAD at approval time:

`01b93d4e30733427608ab862b8627357cc1a2d6a`

If current HEAD differs:

- do **not** reset
- do **not** restore
- do **not** clean
- do **not** overwrite user work

Instead report the current HEAD and continue only if the difference does not conflict with the five-page scope.

## 1.2 Confirm the 5 Approved Copy files exist

Required wording authority:

1. `Korea_Inside_TH_Insadong_Travel_Guide_SEO_Localized_Approved_2026-10-03.md`
2. `Korea_Inside_TH_Itaewon_Travel_Guide_SEO_Localized_Approved_2026-10-03.md`
3. `Korea_Inside_TH_Myeongdong_Travel_Guide_SEO_Localized_Approved_2026-10-03.md`
4. `Korea_Inside_TH_Dongdaemun_Travel_Guide_SEO_Localized_Approved_2026-10-03.md`
5. `Korea_Inside_TH_Where_to_Stay_Dongdaemun_SEO_Localized_Approved_2026-10-03.md`

Lock manifest:

`Korea_Inside_TH_SEO_Batch2_CONTENT_LOCK_Manifest_2026-10-03.md`

If any one is missing, unreadable, duplicated with uncertain version, or does not say:

`APPROVED PUBLIC COPY — CONTENT LOCKED`

**STOP. Do not reconstruct or infer the wording.**

## 1.3 Verify Approved Copy hashes

Expected SHA-256:

- Insadong  
  `fb31a0359399401cdd884f1ab74a37376bd983f9b0d2a814c0d8a14741f92eec`

- Itaewon  
  `1996a64acdd14e828c5448435a5e7a99f52b0ac412e89557701955a03da5aa9c`

- Myeongdong  
  `573016620b49d1b81bbe38120246892c9095a0937635c2dead16b5658b1598a9`

- Dongdaemun Travel Guide  
  `a5c1f51183eec0f1441ac944c96b44e14651399be2ffd6cd8f1f630ff466656d`

- Where to Stay Dongdaemun  
  `793f64714f6d9332202d2071adf1bda768dbf1c1916dc898df3e51d64679793a`

If a hash differs:

**STOP and report the mismatch. Do not “use the latest-looking file.”**

## 1.4 Verify English source fingerprints

The Approved Copy was created against these English source blobs:

- `insadong-travel-guide.html`  
  `fe1bb184a53903c3f7d986cdfd93ffcff583187e`

- `itaewon-travel-guide.html`  
  `d39af2f8862350695b683caa75ee551091db7758`

- `myeongdong-travel-guide.html`  
  `ec7b3f41662424cd4d61d57667d81afd0fc51673`

- `dongdaemun-travel-guide.html`  
  `4538a2bd87f29b13596bd91191f3957672159e49`

- `where-to-stay-in-dongdaemun.html`  
  `bbf0cab56bf92dd727747a552a0fd79bee51f002`

If the current English source blob differs:

- do not silently remap
- do not merge the new English wording
- do not regenerate Thai
- **STOP and report source drift**

---

# 2. Exact authorized output files

Codex may create/modify only these five Thai HTML files:

1. `th/insadong-travel-guide.html`
2. `th/itaewon-travel-guide.html`
3. `th/myeongdong-travel-guide.html`
4. `th/dongdaemun-travel-guide.html`
5. `th/where-to-stay-in-dongdaemun.html`

Everything else is protected unless this instruction explicitly says otherwise.

---

# 3. Absolute protected scope

Do not modify:

- any existing 13 Thai working-copy pages
- any Thai SEO Batch 1 page
- `common.js`
- shared `style.css`
- common header source
- common navigation source
- common footer source
- mobile hamburger logic
- sitemap
- English HTML
- ES / JA / ZH-TW / FR / DE HTML
- image binaries
- image source files
- infographic assets
- Approved MDs
- Review MDs
- Handover
- Inventory
- Standard files
- unrelated working-tree files

If implementation appears to require a protected file:

**STOP and report.**

---

# 4. Thai page-shell implementation rules

Use the current English Production page as structural source, but follow the already-established Thai page-shell convention from the Thai Golden Sample / Batch 1.

Required:

- `<html lang="th">`
- Thai canonical under `/th/`
- correct Thai-relative CSS/JS/image paths
- preserve source classes / ids / data-* / JS hooks
- preserve source section order unless the Approved Copy explicitly authorizes a heading-level absorption/demotion
- preserve images and srcset
- preserve affiliate URLs and tracking
- preserve schema structure
- localize user-visible schema text only where represented in Approved Copy
- preserve functionality

Do **not** modify English or other-language reciprocal hreflang files in this step.

Do **not** perform final language-switcher / common.js integration in this step.

---

# 5. Global content rules

Preserve exactly:

- facts
- numbers
- prices
- dates
- times
- distances
- route sequence
- recommendation strength
- who-it-fits / who-it-does-not-fit judgments
- hotel decision logic
- room / bed / occupancy logic
- station / exit / final-walk logic
- luggage friction
- airport transport logic
- brand / hotel / place names
- affiliate URL / tracking

Codex must not:

- translate on its own
- retranslate
- rewrite Thai
- improve Thai grammar
- “make it more natural”
- shorten
- expand
- merge paragraphs
- split paragraphs
- add Humanization
- change recommendation strength
- change area ranking
- add new examples
- remove trade-offs
- restore English wording because it looks “closer to source”

---

# 6. Internal-link rule for this phase

New contextual Thai internal links: **0**

Do not perform:

- Hub ↔ Detail reciprocal closure
- cross-cluster linking
- authority-flow tuning
- orphan-link closure
- future `/th/` link creation
- anchor-text optimization beyond Approved Copy

Preserve only the existing href targets represented by the source / Approved Copy.

Final Thai internal-link closure happens later, after planned Thai coverage is complete.

---

# 7. Page 1 — Insadong Travel Guide

Target:

`th/insadong-travel-guide.html`

Approved source:

`Korea_Inside_TH_Insadong_Travel_Guide_SEO_Localized_Approved_2026-10-03.md`

Approved English source coverage:

**620 / 620**

## Required implementation

Apply exact Approved:

- title
- meta description
- H1
- H2/H3
- full body
- CTA
- breadcrumb visible text
- alt
- captions
- ARIA / user-visible attributes represented in Approved Copy
- existing link text

## Approved SEO hierarchy delta

Demote only:

`Considering Insadong as your base?`

into the existing Stay bridge/card treatment.

Do not delete its visible text or its link.

Target substantive H2 count:

**13**

## Forbidden

- no new Insadong recommendations
- no new hotel links
- no new Bukchon / Gyeongbokgung / Ikseon-dong links
- no body rewriting
- no recommendation-strength changes

---

# 8. Page 2 — Itaewon Travel Guide

Target:

`th/itaewon-travel-guide.html`

Approved source:

`Korea_Inside_TH_Itaewon_Travel_Guide_SEO_Localized_Approved_2026-10-03.md`

Approved English source coverage:

**331 / 331**

## Required implementation

Apply exact Approved:

- title
- meta
- H1
- H2/H3/H4
- full body
- CTA
- captions
- alt
- existing links
- affiliate-visible text

## Approved hierarchy delta

### A. Final Recommendation

`Final Recommendation`

must not remain a separate substantive H2.

Its body must remain visible and be absorbed under:

`The Korea Inside Default`

Do not delete or rewrite that body.

### B. Stay bridge

Final:

`Planning to stay in Itaewon?`

must be demoted to Stay bridge/card treatment.

Keep visible copy and link.

Target substantive H2 count:

**20**

## Existing CTA / affiliate protection

Preserve all existing targets and tracking exactly, including:

- Creatrip Hannam beauty
- Creatrip Kyochon Pilbang
- Klook Itaewon Pub Crawl
- Stay guide links

No affiliate substitution.

---

# 9. Page 3 — Myeongdong Travel Guide

Target:

`th/myeongdong-travel-guide.html`

Approved source:

`Korea_Inside_TH_Myeongdong_Travel_Guide_SEO_Localized_Approved_2026-10-03.md`

Approved English source coverage:

**557 / 557 by translation or explicit disposition**

## Required implementation

Apply exact Approved:

- title
- meta
- H1
- H2/H3/H4
- full evergreen body
- CTA
- captions
- alt
- existing links
- current layer

## Approved hierarchy delta

Demote these three Stay / hotel bridge H2s only:

1. `Your station choice matters even more when it is your hotel.`
2. `If you expect to shop heavily, a nearby hotel changes the afternoon.`
3. `You do not need to spend all day here for staying here to make sense.`

Keep all visible copy.

Target substantive H2 count:

**14**

## Current Layer — critical

Do **not** implement the September Current Layer as active October copy.

Do **not** reintroduce the expired Olive Young × Sanrio September campaign.

Implement only the Approved October CURRENT replacement layer.

Do not independently research and add new October items.

If the approved current layer has clearly become invalid before implementation:

**STOP and report. Do not rewrite.**

---

# 10. Page 4 — Dongdaemun Travel Guide

Target:

`th/dongdaemun-travel-guide.html`

Approved source:

`Korea_Inside_TH_Dongdaemun_Travel_Guide_SEO_Localized_Approved_2026-10-03.md`

Approved source coverage:

**396 / 396**

## Required implementation

Apply exact Approved:

- title
- meta
- H1
- headings
- full body
- map-visible UI text represented by Approved Copy
- CTA
- alt / caption
- existing links
- affiliate-visible text

## Approved hierarchy delta

### A. Late-night rest bridge

`If You Are Still Shopping Late: Do You Need a Break?`

must not remain a substantive SEO H2.

Demote it to the optional late-night activity / bridge treatment.

Preserve:

- all body copy
- jjimjilbang CTA
- affiliate URL

### B. Final Recommendation

`Final Recommendation`

must not remain a separate substantive H2.

Absorb its body into:

`The Korea Inside Default`

Do not delete body copy.

Target substantive H2 count:

**20**

## Dynamic Decision Map protection

Preserve exactly:

- NAVER dynamic-map integration
- map container
- map ids
- classes
- data attributes
- route logic
- marker logic
- route A/B/C behavior
- JavaScript hooks
- fallback content

Do not rewrite map JavaScript.

Only replace approved user-visible strings.

If map implementation requires touching a protected global file:

**STOP and report.**

---

# 11. Page 5 — Where to Stay in Dongdaemun

Target:

`th/where-to-stay-in-dongdaemun.html`

Approved source:

`Korea_Inside_TH_Where_to_Stay_Dongdaemun_SEO_Localized_Approved_2026-10-03.md`

Approved source coverage:

**140 / 140**

## Required implementation

Apply exact Approved:

- title
- meta
- H1
- all H2/H3
- body
- FAQ
- CTA
- hotel decision text

All substantive H2s remain:

**8**

Do not demote or merge any H2 on this page.

## Hotel order — exact

Preserve this order:

1. Sotetsu Hotels The Splaisir Seoul Dongdaemun
2. Novotel Ambassador Seoul Dongdaemun Hotels & Residences
3. Hotel Skypark Kingstown Dongdaemun
4. Nine Tree by Parnas Seoul Dongdaemun
5. Toyoko Inn Seoul Dongdaemun II
6. The Summit Hotel Seoul Dongdaemun
7. JW Marriott Dongdaemun Square Seoul
8. Mangrove Dongdaemun

## Decision logic — exact

Preserve:

- room size
- bed type
- occupancy
- triple / family / quadruple distinctions
- elevator logic
- station exit
- final walk
- luggage route
- airport-bus logic
- kitchen / washing-machine logic
- residence vs standard hotel distinction
- coliving vs hotel distinction
- no-window basement warning
- longer-stay trade-offs

Do not rank these hotels 1–8.

Do not introduce star-rating or review-score logic.

## Affiliate lock

Expected affiliate URLs:

**24 / 24**

Preserve exact:

- Expedia
- Trip.com
- Agoda

Do not change:

- destination URL
- query parameters
- affiliate code
- tracking
- visible OTA label

---

# 12. Schema / FAQ rules

For every page:

- preserve source schema structure
- user-visible FAQ and FAQ schema must match where FAQ schema exists
- do not add FAQ schema where source/Approved Copy does not have it
- do not generate new FAQ questions
- do not translate hidden technical values that should remain machine values
- localize only user-visible schema text represented by Approved Copy

QA must explicitly state per page:

- visible FAQ count
- schema FAQ count
- parity PASS / N/A

---

# 13. Accessibility / attributes

Preserve or implement Approved Thai for:

- alt
- caption
- breadcrumb visible text
- user-visible ARIA labels
- buttons
- CTA labels
- summary / details visible text

Do not alter:

- ids
- data attributes
- machine-readable values
- functional selectors

---

# 14. Static QA — mandatory

After implementation, run static QA on all five pages.

## 14.1 Approved coverage

Verify:

- Insadong: `620/620`
- Itaewon: `331/331`
- Myeongdong: `557/557 disposition`
- Dongdaemun: `396/396`
- Where to Stay Dongdaemun: `140/140`

No unmapped approved public string.

## 14.2 Search-facing exactness

For each page:

- title exact
- meta exact
- H1 exact

## 14.3 Heading counts

Must report:

- Insadong substantive H2 = **13**
- Itaewon substantive H2 = **20**
- Myeongdong substantive H2 = **14**
- Dongdaemun substantive H2 = **20**
- Where to Stay Dongdaemun substantive H2 = **8**

## 14.4 Content integrity

Must be:

- missing approved Thai = 0
- unauthorized added Thai = 0
- accidental English body leakage = 0
- fact mismatch = 0
- number mismatch = 0
- time mismatch = 0
- distance mismatch = 0
- recommendation-strength mismatch = 0
- route-order mismatch = 0
- hotel-order mismatch = 0

## 14.5 Link integrity

Must be:

- new contextual Thai links = 0
- future `/th/` links = 0
- existing href target changes = 0 unless explicitly required by Approved Copy
- broken links = 0

## 14.6 Affiliate integrity

Must be:

- affiliate URL changes = 0
- tracking changes = 0
- Dongdaemun Stay affiliate URLs = 24/24 exact

## 14.7 Asset integrity

Must be:

- broken image path = 0
- broken srcset = 0
- path-case mismatch = 0
- image binary changes = 0

## 14.8 Technical structure

Must be:

- class changes outside required localized shell = 0
- id changes = 0
- data-* changes = 0
- JS logic changes = 0
- common-file changes = 0
- Dongdaemun map functionality preserved

---

# 15. Working-tree protection

Before and after implementation:

- run `git status --short --untracked-files=all`
- record all pre-existing changes
- do not overwrite them
- do not clean them
- do not restore them
- do not stage them

Only the five authorized HTML files may appear as new task changes.

If any unrelated file changes during the task:

**STOP and report before doing anything else.**

---

# 16. Forbidden Git operations

Do not use:

- `git add .`
- `git add -A`
- `git restore`
- `git reset`
- `git clean`
- `git stash`
- force push
- force checkout over user changes

This instruction does not authorize any staging.

---

# 17. Git / Production boundary

This command authorizes only:

**5 Thai HTML implementation + static QA**

At completion:

- stage = **0**
- commit = **0**
- push = **0**
- merge = **0**
- Vercel Production = **0**
- sitemap change = **0**
- common.js change = **0**
- style.css change = **0**

Wait for explicit user approval before the next phase.

---

# 18. STOP conditions

STOP without improvising if:

1. any Approved MD is missing
2. any Approved MD hash mismatches
3. English source fingerprint mismatches
4. Approved mapping is ambiguous
5. Approved Thai and source structure cannot be reconciled without rewriting
6. an affiliate URL cannot be matched
7. a required map function would require global-file changes
8. a protected working-tree change conflicts with the target
9. an Approved current fact is clearly expired/invalid
10. implementation would require adding content not approved by ChatGPT/user

Do not fix these with new translation or editorial judgment.

---

# 19. Required completion report

Return exactly this structure.

## Pre-check

- Branch:
- HEAD:
- Approved files 5/5:
- Approved SHA-256 5/5:
- English source fingerprint 5/5:
- Existing working-tree changes protected:

## Page 1 — Insadong

- File:
- Approved coverage:
- Title:
- Meta:
- H1:
- Substantive H2:
- Body mapping:
- FAQ/schema:
- Existing links:
- New internal links:
- Broken links:
- Broken assets:
- English leakage:

## Page 2 — Itaewon

Same fields.

Additionally:
- Final Recommendation absorbed:
- Stay bridge demoted:

## Page 3 — Myeongdong

Same fields.

Additionally:
- October CURRENT layer:
- Expired September active copy:
- Olive Young × Sanrio September active copy:

Expected last two values:

`0`

## Page 4 — Dongdaemun

Same fields.

Additionally:
- Late-night rest bridge:
- Final Recommendation absorbed:
- Dynamic Decision Map:
- Map JS logic changed:

Expected Map JS logic changed:

`0`

## Page 5 — Where to Stay Dongdaemun

Same fields.

Additionally:
- Hotels:
- Affiliate URLs:
- Affiliate/tracking changes:
- Hotel order:
- Room/bed/occupancy logic:

Expected:

- Hotels = `8/8`
- Affiliate URLs = `24/24`
- Affiliate/tracking changes = `0`

## Final scope report

- Modified task files:
- Expected task files: 5
- Out-of-scope changed files:
- Protected files changed:
- Approved MD changed:
- New contextual internal links:
- Affiliate/tracking changes:
- Image binary changes:
- User pre-existing changes preserved:
- stage:
- commit:
- push:
- merge:
- Production:

Required final values:

- Out-of-scope changed files = `0`
- Protected files changed = `0`
- Approved MD changed = `0`
- New contextual internal links = `0`
- Affiliate/tracking changes = `0`
- Image binary changes = `0`
- stage = `0`
- commit = `0`
- push = `0`
- merge = `0`
- Production = `0`

---

# 20. Final instruction to Codex

Do not optimize beyond this instruction.

Do not make editorial judgments.

Do not reopen CONTENT LOCK.

Do not compare Thai wording to English and “correct” the Thai simply because the expressions differ.

Implement the five approved pages exactly, run static QA, report, and stop.
