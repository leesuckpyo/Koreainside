# Korea Inside — Thai Infographic Batch 1 Source Extraction

## Document metadata

- Date: 2026-10-02
- Status: **SOURCE EXTRACTION COMPLETE**
- Thai copy status: **THAI LOCALIZATION NOT STARTED**
- Authority: current user instruction, `[THAI INFOGRAPHIC BATCH 1 — EXACT 5 ASSETS ONLY]`.
- Scope: source extraction for INF-001 through INF-005 only; one new Markdown file.
- Classification: LOCALIZE 5; P1 5; P2 0; source-asset REVIEW_REQUIRED 0.
- This is an English source record, not approved Thai Public Copy, asset-production approval, HTML implementation approval, or Production approval.
- No Thai translation or wording was authored. No image, HTML, CSS, JS, SEO, Inventory, staging, or release action was performed.

## Standards and evidence

- Existing audit: `md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md`, the INF-001–INF-005 entries only.
- Audit source paths, SHA-256 values, PNG formats, dimensions, and P1 priorities are preserved and reverified.
- The audit Embedded English field is a heading/label/key-callout summary; some best-for/warning sentences are not transcribed there. Therefore it cannot alone establish exact text coverage. All five unchanged English originals were visually inspected at native resolution to complete those fields.
- OCR was not used. English wording, spelling, accents, punctuation, capitalization, source recommendation pairing, and warning conditions are transcribed without rewriting.
- Previously read Project Base, Public Content Master Standard, Navigation Hub Architecture Standard, and Language Localization Standard remain byte-identical to the verified prior-task baseline.
- Latest repository handover: `md/감사본/일본어/Korea_Inside_Room_Handover_2026-09-29.md`; previously read and unchanged. Actual repository state takes precedence over historical handover counts.
- **Known input limitation:** `Korea_Inside_Thai_Localization_Standard.md` was not found at the repository root, in repository filename search, or under Downloads. Its contents were not reconstructed or claimed as read. This work applies the current explicit mechanical-extraction scope; Thai wording and typography approval remain outside this task.
- Existing Thai images/pages are historical user work. Their presence and references are recorded; Thai wording quality, asset approval, pixel QA, and whole-language completion were not re-audited.

## Protected Git baseline

- Branch: `main`
- HEAD: `d0ae52ad267f56926a6a55e413c3aaf7b704c0b6`
- Tracked/staged changes at start: 0.
- Existing untracked files: 26.
- Protected existing file count: 1095.
- Aggregate SHA-256 fingerprint: `53485aa54997fed24613fd50ede923294b884473b2032855ae253db1779c6430`.
- Fingerprint procedure: SHA-256 each tracked/untracked nonignored file; sort paths with JavaScript `localeCompare(path, "en")`; hash concatenated `path + NUL + SHA256 + LF` records. All pre-existing files are compared byte-for-byte after writing this document.
- Protection fingerprinting reads existing file bytes only to detect changes; it is not an infographic inventory or source-text audit outside this five-asset scope.
- The current working tree contains three existing Thai assets in this scope; they are not regenerated or overwritten.
- Expected final untracked count: 26 + 1 = 27.

## Five-asset summary

| ID | English source filename | Native PNG size | Priority | Exact text units | TRANSLATE | RETAIN | Thai page / current reference | Planned Thai filename |
|---|---|---|---|---:|---:|---:|---|---|
| INF-001 | `couples-stay-seoul-area-guide.png` | 1536 × 1024 | P1 | 26 | 17 | 9 | EXISTS; existing `-th.png` linked | `couples-stay-seoul-area-guide-th.png` |
| INF-002 | `family-stay-seoul-area-guide.png` | 1491 × 1055 | P1 | 33 | 24 | 9 | EXISTS; existing `-th.png` linked | `family-stay-seoul-area-guide-th.png` |
| INF-003 | `first-time-seoul-area-guide.png` | 1536 × 1024 | P1 | 28 | 18 | 10 | EXISTS; existing `-th.png` linked | `first-time-seoul-area-guide-th.png` |
| INF-004 | `luxury-stay-seoul-area-guide.png` | 1536 × 1024 | P1 | 29 | 18 | 11 | MISSING; no HTML created | `luxury-stay-seoul-area-guide-th.png` |
| INF-005 | `nightlife-stay-seoul-area-guide.png` | 1536 × 1024 | P1 | 14 | 8 | 6 | MISSING; no HTML created | `nightlife-stay-seoul-area-guide-th.png` |

- Total: **130 exact English occurrence-level text units** = **85 TRANSLATE** + **45 RETAIN**.
- All five source files exist; source SHA matches 5/5; format/dimensions match 5/5; planned Thai paths 5/5; duplicate planned paths 0.
- Full readable editorial/illustration text coverage: 100% for these five originals, based on direct visual transcription. Repeated labels are retained as separate source occurrences, but retained place names/monograms are excluded from the translation count.
- EN source pages: 5/5 exist, each with one matching source reference. Thai related pages: 3 exist; 2 missing. No existing Thai related page in this scope references the English original.
- ES/JA layout and filename precedents: 10/10 existing assets, all with matching source dimensions and an existing matching HTML reference. Their wording was not used as the Thai factual source.

## Extraction and classification rules

- One unit is one independent heading, subtitle, recommendation sentence, row description, area label, best-for sentence, warning/callout, or readable illustrated sign. Wrapped lines within a unit are joined with one space; words/punctuation are unchanged. Raster pixels cannot certify invisible whitespace characters.
- Reading order is header → quick-match rows top-to-bottom, each row left-to-right → top-picks heading/introduction → cards left-to-right, then readable fields top-to-bottom within each card. Illustrated signs are placed inside their original card and labelled with their location. Nightlife has no QUICK MATCH or TOP PICKS sections; none are invented.
- Arrows, icons, separator lines, map geometry, artwork, and visual spacing are protected graphical elements, not invented text units.
- TRANSLATE: English editorial copy supplied for ChatGPT Thai localization only; Codex writes no Thai values.
- RETAIN: source entity labels and visual monogram occurrences, counted separately from editorial translation units.
- BRAND: an official brand/product token embedded in a unit (e.g. AREX, KTX, Lotte World); preserve its identity and spelling. Tagging a mixed unit does not exempt its surrounding English sentence from localization.
- PROPER_NOUN: a place/entity token; preserve the mapped entity and source relationship. Any future approved Thai place-name form belongs to ChatGPT/user review, not this extraction.
- NUMBER: preserve quantity or displayed numeric ordering, including Family “3”, Family “One-night”, and Nightlife prefixes “1.”–“6.”. Do not convert year/calendar/numerals or add facts.
- RECOMMENDATION: preserve description → area pairing and best-for meaning. FACT marks warning/condition/transport constraints that must not be changed.
- The illustrated `G` is recorded as a single-letter visual monogram, not identified as an official brand and not expanded into a guessed name.

## Scope and QA boundary

- Only the five designated English originals were visually inspected and semantically verified. No following infographic was validated, transcribed, generated, or modified.
- Initial documentation/index searches inadvertently surfaced some out-of-scope audit metadata and historical source/copy-gate excerpts. No out-of-scope image inspection or extraction followed. Strict zero incidental out-of-scope document access is therefore not claimed.
- No full 30-asset or 235-image inventory was performed.
- Browser QA and Thai rendered/pixel QA were not performed; this is source extraction and static document QA.
- Source path, SHA, format, dimensions, occurrence paths, target uniqueness, all unit IDs and source-order region metadata, Markdown whitespace, allowed new-file scope, existing-file fingerprints, HEAD, and empty Git index are checked.
- Only this Markdown document may be newly added to the repository. All existing source assets, existing Thai assets, HTML, shared CSS/JS, other Markdown, navigation, sitemap, and hreflang remain unchanged.

# INF-001 — Couples stay area guide

## SOURCE

- Source asset: `images/Accommodation/couples-stay-seoul-area-guide.png`
- Historical audit SHA-256: `184d0526b2c08d4cdc1c5c4b8a09591e1486a38180fb711caae1b40464a35a9c`
- Current SHA-256: `184d0526b2c08d4cdc1c5c4b8a09591e1486a38180fb711caae1b40464a35a9c`
- SHA comparison: MATCH.
- Format: PNG; original format preserved.
- Dimensions: 1536 × 1024 px; historical/current comparison MATCH.
- Priority: P1; unchanged from the existing ES/JA audit.
- Classification: LOCALIZE; readable Korea Inside editorial English is embedded in the original.
- English page: `best-area-for-couples-seoul.html` — EXISTS; 1 occurrence.
- English current src: `images/Accommodation/couples-stay-seoul-area-guide.png`
- Thai page: `th/best-area-for-couples-seoul.html` — EXISTS; local HTML present. No COMPLETE/Production status is changed.
- Thai current src: `../images/Accommodation/couples-stay-seoul-area-guide-th.png`
- English original currently used by Thai page: NO; existing Thai path is already linked.
- Existing planned Thai asset: YES; pre-existing user asset, byte-preserved. No rebuilding or replacement is authorized.
- Exact text coverage: 100% of readable source text; direct visual inspection, no OCR.
- Asset REVIEW_REQUIRED: NO.

## EXACT ENGLISH TEXT

- Exact source units: 26; TRANSLATE 17; RETAIN 9.
- The list and metadata table use the same continuous unit numbering; source position is preserved in the region column.

1. `Where Should Couples Stay in Seoul?`
2. `A quick area guide for different couple travel styles`
3. `Start with the atmosphere you want, then check transport, nighttime noise and the final hotel route.`
4. `QUICK MATCH`
5. `Nightlife and cafés`
6. `Hongdae`
7. `Design shops and local cafés`
8. `Seongsu`
9. `Traditional streets and quiet evenings`
10. `Insadong`
11. `Central sightseeing and shopping`
12. `Myeongdong`
13. `Premium shopping and southern Seoul`
14. `Gangnam`
15. `Lotte World and evening lake walks`
16. `Jamsil`
17. `TOP PICKS`
18. `Hongdae`
19. `Best for nightlife and cafés`
20. `Watch out: Busy streets and late-night noise`
21. `Seongsu`
22. `Best for design cafés and daytime walks`
23. `Watch out: Airport access is less direct`
24. `Insadong`
25. `Best for culture and quieter evenings`
26. `Watch out: Check elevator and taxi access`

### Unit metadata

| Unit ID | Action | Protection tags | Original visual region |
|---|---|---|---|
| `INF-001-U001` | TRANSLATE | PROPER_NOUN | header / heading |
| `INF-001-U002` | TRANSLATE | — | header / subtitle |
| `INF-001-U003` | TRANSLATE | RECOMMENDATION | header / recommendation sentence |
| `INF-001-U004` | TRANSLATE | — | quick-match / section heading |
| `INF-001-U005` | TRANSLATE | RECOMMENDATION | quick-match / row 1 / left |
| `INF-001-U006` | RETAIN | PROPER_NOUN | quick-match / row 1 / right |
| `INF-001-U007` | TRANSLATE | RECOMMENDATION | quick-match / row 2 / left |
| `INF-001-U008` | RETAIN | PROPER_NOUN | quick-match / row 2 / right |
| `INF-001-U009` | TRANSLATE | RECOMMENDATION | quick-match / row 3 / left |
| `INF-001-U010` | RETAIN | PROPER_NOUN | quick-match / row 3 / right |
| `INF-001-U011` | TRANSLATE | RECOMMENDATION | quick-match / row 4 / left |
| `INF-001-U012` | RETAIN | PROPER_NOUN | quick-match / row 4 / right |
| `INF-001-U013` | TRANSLATE | RECOMMENDATION, PROPER_NOUN | quick-match / row 5 / left |
| `INF-001-U014` | RETAIN | PROPER_NOUN | quick-match / row 5 / right |
| `INF-001-U015` | TRANSLATE | RECOMMENDATION, BRAND, PROPER_NOUN | quick-match / row 6 / left |
| `INF-001-U016` | RETAIN | PROPER_NOUN | quick-match / row 6 / right |
| `INF-001-U017` | TRANSLATE | — | top-picks / section heading |
| `INF-001-U018` | RETAIN | PROPER_NOUN | top-picks / left card / heading |
| `INF-001-U019` | TRANSLATE | RECOMMENDATION | top-picks / left card / best-for |
| `INF-001-U020` | TRANSLATE | FACT | top-picks / left card / warning |
| `INF-001-U021` | RETAIN | PROPER_NOUN | top-picks / middle card / heading |
| `INF-001-U022` | TRANSLATE | RECOMMENDATION | top-picks / middle card / best-for |
| `INF-001-U023` | TRANSLATE | FACT | top-picks / middle card / warning |
| `INF-001-U024` | RETAIN | PROPER_NOUN | top-picks / right card / heading |
| `INF-001-U025` | TRANSLATE | RECOMMENDATION | top-picks / right card / best-for |
| `INF-001-U026` | TRANSLATE | FACT | top-picks / right card / warning |

## RETAIN

- Source place/entity tokens and brand tokens: `Seoul`, `Hongdae`, `Seongsu`, `Insadong`, `Myeongdong`, `Gangnam`, `Jamsil`, `Lotte World`.
- Area-label occurrence count retained separately: 9.
- Numeric constraints: no explicit numeric price/time/distance label appears; do not invent one.
- Preserve native casing of illustrated entity labels where present. Keep descriptions and warnings as independent complete source units.

## THAI TARGET

- Planned asset path: `images/Accommodation/couples-stay-seoul-area-guide-th.png`
- Format/canvas: PNG, 1536 × 1024 px; same aspect ratio and dimensions.
- HTML future replacement target: `th/best-area-for-couples-seoul.html` — already references the planned path; this task makes no src/alt/caption change.
- Thai translation: **NOT STARTED in this source-extraction batch**.
- Existing Thai work: asset/page reference present; not replaced and not represented as newly translated or produced.
- Production/deployment: NOT AUTHORIZED.

## ES/JA PRODUCTION PRECEDENT

- ES: `images/Accommodation/couples-stay-seoul-area-guide-es.png` — exists; PNG 1536 × 1024; `es/best-area-for-couples-seoul.html` references `../images/Accommodation/couples-stay-seoul-area-guide-es.png`.
- JA: `images/Accommodation/couples-stay-seoul-area-guide-ja.png` — exists; PNG 1536 × 1024; `ja/best-area-for-couples-seoul.html` references `../images/Accommodation/couples-stay-seoul-area-guide-ja.png`.
- These are filename/layout/canvas precedents only. No ES/JA wording was translated into Thai.
- The historical scoped audit does not identify an editable production master/script for this asset. Existing flattened PNG precedents do not establish an editable-source workflow; no new production method is created here.

## PROTECTION

- Facts: preserve source warning and access/transport/room constraints; do not add claims.
- Recommendation: preserve quick-match pairings, top-picks order, best-for statements, and every watch-out sentence.
- Numbers: preserve all displayed values/order and quantity meaning.
- Layout: same canvas, aspect ratio, row/card count, arrows, icon positions, artwork placement, and map/illustration geometry.
- Dimensions/format: 1536 × 1024 PNG.
- Visual hierarchy: preserve heading/subtitle, decision rows, illustrated labels, and original top-picks presentation where present.
- Source image and every existing localized asset: byte-preserved. No pixel edit, generated PNG/WebP/SVG, redesign, HTML replacement, or common-system change.

# INF-002 — Family stay area guide

## SOURCE

- Source asset: `images/Accommodation/family-stay-seoul-area-guide.png`
- Historical audit SHA-256: `5b241eb23e123238e527694cf2d3e377dac5244b355d661ec180b4f2fd5442a1`
- Current SHA-256: `5b241eb23e123238e527694cf2d3e377dac5244b355d661ec180b4f2fd5442a1`
- SHA comparison: MATCH.
- Format: PNG; original format preserved.
- Dimensions: 1491 × 1055 px; historical/current comparison MATCH.
- Priority: P1; unchanged from the existing ES/JA audit.
- Classification: LOCALIZE; readable Korea Inside editorial English is embedded in the original.
- English page: `best-area-for-families-seoul.html` — EXISTS; 1 occurrence.
- English current src: `images/Accommodation/family-stay-seoul-area-guide.png`
- Thai page: `th/best-area-for-families-seoul.html` — EXISTS; local HTML present. No COMPLETE/Production status is changed.
- Thai current src: `../images/Accommodation/family-stay-seoul-area-guide-th.png`
- English original currently used by Thai page: NO; existing Thai path is already linked.
- Existing planned Thai asset: YES; pre-existing user asset, byte-preserved. No rebuilding or replacement is authorized.
- Exact text coverage: 100% of readable source text; direct visual inspection, no OCR.
- Asset REVIEW_REQUIRED: NO.

## EXACT ENGLISH TEXT

- Exact source units: 33; TRANSLATE 24; RETAIN 9.
- The list and metadata table use the same continuous unit numbering; source position is preserved in the region column.

1. `Where Should Your Family Stay in Seoul?`
2. `A quick area guide for families`
3. `Start with your main family priority, then choose the Seoul area that makes the trip easier.`
4. `QUICK MATCH`
5. `First family trip`
6. `Myeongdong`
7. `Best all-round base for central sightseeing, meals and easy planning.`
8. `Lotte World and family attractions`
9. `Jamsil`
10. `Best for family attractions, modern hotels and indoor activities.`
11. `Airport access and large luggage`
12. `Mapo / Gongdeok`
13. `Best for practical transfers, easier luggage handling and quieter nights.`
14. `Traditional culture and calmer evenings`
15. `Insadong`
16. `Best for palaces, traditional streets and a quieter cultural base.`
17. `One-night transit or KTX connection`
18. `Seoul Station`
19. `Best for airport rail, train connections and short stopovers.`
20. `Teenagers, cafés and nightlife`
21. `Hongdae`
22. `Best for older kids or teens who enjoy cafés, music and a lively atmosphere.`
23. `TOP PICKS`
24. `Top 3 family-friendly Seoul bases`
25. `Myeongdong`
26. `Best first family base`
27. `Watch out: Busy streets and some smaller rooms`
28. `Jamsil`
29. `Best for attractions`
30. `Watch out: Longer rides to northwest Seoul`
31. `Mapo / Gongdeok`
32. `Best for airport and luggage`
33. `Watch out: Check the exact station exit`

### Unit metadata

| Unit ID | Action | Protection tags | Original visual region |
|---|---|---|---|
| `INF-002-U001` | TRANSLATE | PROPER_NOUN | header / heading |
| `INF-002-U002` | TRANSLATE | — | header / subtitle |
| `INF-002-U003` | TRANSLATE | RECOMMENDATION, PROPER_NOUN | header / recommendation sentence |
| `INF-002-U004` | TRANSLATE | — | quick-match / section heading |
| `INF-002-U005` | TRANSLATE | RECOMMENDATION | quick-match / row 1 / left |
| `INF-002-U006` | RETAIN | PROPER_NOUN | quick-match / row 1 / area |
| `INF-002-U007` | TRANSLATE | RECOMMENDATION | quick-match / row 1 / right |
| `INF-002-U008` | TRANSLATE | RECOMMENDATION, BRAND, PROPER_NOUN | quick-match / row 2 / left |
| `INF-002-U009` | RETAIN | PROPER_NOUN | quick-match / row 2 / area |
| `INF-002-U010` | TRANSLATE | RECOMMENDATION | quick-match / row 2 / right |
| `INF-002-U011` | TRANSLATE | RECOMMENDATION | quick-match / row 3 / left |
| `INF-002-U012` | RETAIN | PROPER_NOUN | quick-match / row 3 / area |
| `INF-002-U013` | TRANSLATE | RECOMMENDATION | quick-match / row 3 / right |
| `INF-002-U014` | TRANSLATE | RECOMMENDATION | quick-match / row 4 / left |
| `INF-002-U015` | RETAIN | PROPER_NOUN | quick-match / row 4 / area |
| `INF-002-U016` | TRANSLATE | RECOMMENDATION | quick-match / row 4 / right |
| `INF-002-U017` | TRANSLATE | RECOMMENDATION, BRAND, NUMBER | quick-match / row 5 / left |
| `INF-002-U018` | RETAIN | PROPER_NOUN | quick-match / row 5 / area |
| `INF-002-U019` | TRANSLATE | RECOMMENDATION | quick-match / row 5 / right |
| `INF-002-U020` | TRANSLATE | RECOMMENDATION | quick-match / row 6 / left |
| `INF-002-U021` | RETAIN | PROPER_NOUN | quick-match / row 6 / area |
| `INF-002-U022` | TRANSLATE | RECOMMENDATION | quick-match / row 6 / right |
| `INF-002-U023` | TRANSLATE | — | top-picks / section heading |
| `INF-002-U024` | TRANSLATE | NUMBER, PROPER_NOUN | top-picks / introduction heading |
| `INF-002-U025` | RETAIN | PROPER_NOUN | top-picks / left card / heading |
| `INF-002-U026` | TRANSLATE | RECOMMENDATION | top-picks / left card / best-for |
| `INF-002-U027` | TRANSLATE | FACT | top-picks / left card / warning |
| `INF-002-U028` | RETAIN | PROPER_NOUN | top-picks / middle card / heading |
| `INF-002-U029` | TRANSLATE | RECOMMENDATION | top-picks / middle card / best-for |
| `INF-002-U030` | TRANSLATE | FACT, PROPER_NOUN | top-picks / middle card / warning |
| `INF-002-U031` | RETAIN | PROPER_NOUN | top-picks / right card / heading |
| `INF-002-U032` | TRANSLATE | RECOMMENDATION | top-picks / right card / best-for |
| `INF-002-U033` | TRANSLATE | FACT | top-picks / right card / warning |

## RETAIN

- Source place/entity tokens and brand tokens: `Seoul`, `Myeongdong`, `Jamsil`, `Mapo / Gongdeok`, `Insadong`, `Seoul Station`, `Hongdae`, `Lotte World`, `KTX`.
- Area-label occurrence count retained separately: 9.
- Numeric constraints: preserve `3` in the top-picks heading and one-night transit meaning.
- Preserve native casing of illustrated entity labels where present. Keep descriptions and warnings as independent complete source units.

## THAI TARGET

- Planned asset path: `images/Accommodation/family-stay-seoul-area-guide-th.png`
- Format/canvas: PNG, 1491 × 1055 px; same aspect ratio and dimensions.
- HTML future replacement target: `th/best-area-for-families-seoul.html` — already references the planned path; this task makes no src/alt/caption change.
- Thai translation: **NOT STARTED in this source-extraction batch**.
- Existing Thai work: asset/page reference present; not replaced and not represented as newly translated or produced.
- Production/deployment: NOT AUTHORIZED.

## ES/JA PRODUCTION PRECEDENT

- ES: `images/Accommodation/family-stay-seoul-area-guide-es.png` — exists; PNG 1491 × 1055; `es/best-area-for-families-seoul.html` references `../images/Accommodation/family-stay-seoul-area-guide-es.png`.
- JA: `images/Accommodation/family-stay-seoul-area-guide-ja.png` — exists; PNG 1491 × 1055; `ja/best-area-for-families-seoul.html` references `../images/Accommodation/family-stay-seoul-area-guide-ja.png`.
- These are filename/layout/canvas precedents only. No ES/JA wording was translated into Thai.
- The historical scoped audit does not identify an editable production master/script for this asset. Existing flattened PNG precedents do not establish an editable-source workflow; no new production method is created here.

## PROTECTION

- Facts: preserve source warning and access/transport/room constraints; do not add claims.
- Recommendation: preserve quick-match pairings, top-picks order, best-for statements, and every watch-out sentence.
- Numbers: preserve all displayed values/order and quantity meaning.
- Layout: same canvas, aspect ratio, row/card count, arrows, icon positions, artwork placement, and map/illustration geometry.
- Dimensions/format: 1491 × 1055 PNG.
- Visual hierarchy: preserve heading/subtitle, decision rows, illustrated labels, and original top-picks presentation where present.
- Source image and every existing localized asset: byte-preserved. No pixel edit, generated PNG/WebP/SVG, redesign, HTML replacement, or common-system change.

# INF-003 — First-time visitor stay area guide

## SOURCE

- Source asset: `images/Accommodation/first-time-seoul-area-guide.png`
- Historical audit SHA-256: `2e958e12213927ce0a4a15588c525875ec14323eaf04486ae4e468784d988421`
- Current SHA-256: `2e958e12213927ce0a4a15588c525875ec14323eaf04486ae4e468784d988421`
- SHA comparison: MATCH.
- Format: PNG; original format preserved.
- Dimensions: 1536 × 1024 px; historical/current comparison MATCH.
- Priority: P1; unchanged from the existing ES/JA audit.
- Classification: LOCALIZE; readable Korea Inside editorial English is embedded in the original.
- English page: `best-area-for-first-time-visitors-seoul.html` — EXISTS; 1 occurrence.
- English current src: `images/Accommodation/first-time-seoul-area-guide.png`
- Thai page: `th/best-area-for-first-time-visitors-seoul.html` — EXISTS; local HTML present. No COMPLETE/Production status is changed.
- Thai current src: `../images/Accommodation/first-time-seoul-area-guide-th.png`
- English original currently used by Thai page: NO; existing Thai path is already linked.
- Existing planned Thai asset: YES; pre-existing user asset, byte-preserved. No rebuilding or replacement is authorized.
- Exact text coverage: 100% of readable source text; direct visual inspection, no OCR.
- Asset REVIEW_REQUIRED: NO.

## EXACT ENGLISH TEXT

- Exact source units: 28; TRANSLATE 18; RETAIN 10.
- The list and metadata table use the same continuous unit numbering; source position is preserved in the region column.

1. `Where Should First-Time Visitors Stay in Seoul?`
2. `A quick area guide for your first trip`
3. `QUICK MATCH`
4. `Central sightseeing and easy planning`
5. `Myeongdong`
6. `Cafés, nightlife and direct AREX`
7. `Hongdae`
8. `Airport rail, KTX and large luggage`
9. `Seoul Station`
10. `Airport access and quieter nights`
11. `Mapo / Gongdeok`
12. `Palaces and traditional streets`
13. `Insadong`
14. `Lotte World and southern Seoul`
15. `Jamsil`
16. `TOP PICKS`
17. `SHOP`
18. `Myeongdong`
19. `Best all-round first base`
20. `Watch out: Busy streets and some smaller rooms`
21. `Hongdae`
22. `Best for cafés and nightlife`
23. `COFFEE`
24. `Watch out: Check distance from the busiest streets`
25. `Seoul Station`
26. `Best for airport rail and luggage`
27. `SEOUL STATION`
28. `Watch out: Verify the exact exit and walking route`

### Unit metadata

| Unit ID | Action | Protection tags | Original visual region |
|---|---|---|---|
| `INF-003-U001` | TRANSLATE | PROPER_NOUN | header / heading |
| `INF-003-U002` | TRANSLATE | — | header / subtitle |
| `INF-003-U003` | TRANSLATE | — | quick-match / section heading |
| `INF-003-U004` | TRANSLATE | RECOMMENDATION | quick-match / row 1 / left |
| `INF-003-U005` | RETAIN | PROPER_NOUN | quick-match / row 1 / right |
| `INF-003-U006` | TRANSLATE | RECOMMENDATION, BRAND | quick-match / row 2 / left |
| `INF-003-U007` | RETAIN | PROPER_NOUN | quick-match / row 2 / right |
| `INF-003-U008` | TRANSLATE | RECOMMENDATION, BRAND | quick-match / row 3 / left |
| `INF-003-U009` | RETAIN | PROPER_NOUN | quick-match / row 3 / right |
| `INF-003-U010` | TRANSLATE | RECOMMENDATION | quick-match / row 4 / left |
| `INF-003-U011` | RETAIN | PROPER_NOUN | quick-match / row 4 / right |
| `INF-003-U012` | TRANSLATE | RECOMMENDATION | quick-match / row 5 / left |
| `INF-003-U013` | RETAIN | PROPER_NOUN | quick-match / row 5 / right |
| `INF-003-U014` | TRANSLATE | RECOMMENDATION, BRAND, PROPER_NOUN | quick-match / row 6 / left |
| `INF-003-U015` | RETAIN | PROPER_NOUN | quick-match / row 6 / right |
| `INF-003-U016` | TRANSLATE | — | top-picks / section heading |
| `INF-003-U017` | TRANSLATE | — | top-picks / left card / vertical illustrated sign |
| `INF-003-U018` | RETAIN | PROPER_NOUN | top-picks / left card / heading |
| `INF-003-U019` | TRANSLATE | RECOMMENDATION | top-picks / left card / best-for |
| `INF-003-U020` | TRANSLATE | FACT | top-picks / left card / warning |
| `INF-003-U021` | RETAIN | PROPER_NOUN | top-picks / middle card / heading |
| `INF-003-U022` | TRANSLATE | RECOMMENDATION | top-picks / middle card / best-for |
| `INF-003-U023` | TRANSLATE | — | top-picks / middle card / illustrated sign |
| `INF-003-U024` | TRANSLATE | FACT | top-picks / middle card / warning |
| `INF-003-U025` | RETAIN | PROPER_NOUN | top-picks / right card / heading |
| `INF-003-U026` | TRANSLATE | RECOMMENDATION | top-picks / right card / best-for |
| `INF-003-U027` | RETAIN | PROPER_NOUN | top-picks / right card / illustrated station sign |
| `INF-003-U028` | TRANSLATE | FACT | top-picks / right card / warning |

## RETAIN

- Source place/entity tokens and brand tokens: `Seoul`, `Myeongdong`, `Hongdae`, `Seoul Station`, `Mapo / Gongdeok`, `Insadong`, `Jamsil`, `AREX`, `KTX`, `Lotte World`, `SEOUL STATION`.
- Area-label occurrence count retained separately: 10.
- Numeric constraints: no explicit numeric price/time/distance label appears; do not invent one.
- Preserve native casing of illustrated entity labels where present. Keep descriptions and warnings as independent complete source units.

## THAI TARGET

- Planned asset path: `images/Accommodation/first-time-seoul-area-guide-th.png`
- Format/canvas: PNG, 1536 × 1024 px; same aspect ratio and dimensions.
- HTML future replacement target: `th/best-area-for-first-time-visitors-seoul.html` — already references the planned path; this task makes no src/alt/caption change.
- Thai translation: **NOT STARTED in this source-extraction batch**.
- Existing Thai work: asset/page reference present; not replaced and not represented as newly translated or produced.
- Production/deployment: NOT AUTHORIZED.

## ES/JA PRODUCTION PRECEDENT

- ES: `images/Accommodation/first-time-seoul-area-guide-es.png` — exists; PNG 1536 × 1024; `es/best-area-for-first-time-visitors-seoul.html` references `../images/Accommodation/first-time-seoul-area-guide-es.png`.
- JA: `images/Accommodation/first-time-seoul-area-guide-ja.png` — exists; PNG 1536 × 1024; `ja/best-area-for-first-time-visitors-seoul.html` references `../images/Accommodation/first-time-seoul-area-guide-ja.png`.
- These are filename/layout/canvas precedents only. No ES/JA wording was translated into Thai.
- The historical scoped audit does not identify an editable production master/script for this asset. Existing flattened PNG precedents do not establish an editable-source workflow; no new production method is created here.

## PROTECTION

- Facts: preserve source warning and access/transport/room constraints; do not add claims.
- Recommendation: preserve quick-match pairings, top-picks order, best-for statements, and every watch-out sentence.
- Numbers: preserve all displayed values/order and quantity meaning.
- Layout: same canvas, aspect ratio, row/card count, arrows, icon positions, artwork placement, and map/illustration geometry.
- Dimensions/format: 1536 × 1024 PNG.
- Visual hierarchy: preserve heading/subtitle, decision rows, illustrated labels, and original top-picks presentation where present.
- Source image and every existing localized asset: byte-preserved. No pixel edit, generated PNG/WebP/SVG, redesign, HTML replacement, or common-system change.

# INF-004 — Luxury stay area guide

## SOURCE

- Source asset: `images/Accommodation/luxury-stay-seoul-area-guide.png`
- Historical audit SHA-256: `a88d4eeb4a1678a82136a1bdb8893859e99544c41152c884990735e5ad364935`
- Current SHA-256: `a88d4eeb4a1678a82136a1bdb8893859e99544c41152c884990735e5ad364935`
- SHA comparison: MATCH.
- Format: PNG; original format preserved.
- Dimensions: 1536 × 1024 px; historical/current comparison MATCH.
- Priority: P1; unchanged from the existing ES/JA audit.
- Classification: LOCALIZE; readable Korea Inside editorial English is embedded in the original.
- English page: `best-area-for-luxury-hotels-seoul.html` — EXISTS; 1 occurrence.
- English current src: `images/Accommodation/luxury-stay-seoul-area-guide.png`
- Thai page: `th/best-area-for-luxury-hotels-seoul.html` — MISSING; no working copy is created.
- Thai current src: NONE — Thai page does not exist.
- English original currently used by Thai page: NOT APPLICABLE — page missing.
- Existing planned Thai asset: NO; future asset only, no file created.
- Exact text coverage: 100% of readable source text; direct visual inspection, no OCR.
- Asset REVIEW_REQUIRED: NO.

## EXACT ENGLISH TEXT

- Exact source units: 29; TRANSLATE 18; RETAIN 11.
- The list and metadata table use the same continuous unit numbering; source position is preserved in the region column.

1. `Where Should Luxury Travelers Stay in Seoul?`
2. `A quick area guide for premium stays`
3. `QUICK MATCH`
4. `Luxury shopping and fine dining`
5. `Gangnam`
6. `Lotte World and modern comfort`
7. `Jamsil`
8. `First trip and central sightseeing`
9. `Myeongdong`
10. `Airport rail and large luggage`
11. `Seoul Station / Namdaemun`
12. `Palaces and quiet cultural stays`
13. `Insadong`
14. `International dining and nightlife`
15. `Itaewon`
16. `TOP PICKS`
17. `Gangnam`
18. `DEPARTMENT STORE`
19. `G`
20. `Best for shopping, dining and business`
21. `Watch out: Longer routes to palaces and airport rail`
22. `Jamsil`
23. `Best for modern comfort and attractions`
24. `Watch out: Farther from central historic sights`
25. `Myeongdong`
26. `Myeongdong`
27. `Best for first-time central convenience`
28. `DUTY FREE`
29. `Watch out: Busy streets and less privacy`

### Unit metadata

| Unit ID | Action | Protection tags | Original visual region |
|---|---|---|---|
| `INF-004-U001` | TRANSLATE | PROPER_NOUN | header / heading |
| `INF-004-U002` | TRANSLATE | — | header / subtitle |
| `INF-004-U003` | TRANSLATE | — | quick-match / section heading |
| `INF-004-U004` | TRANSLATE | RECOMMENDATION | quick-match / row 1 / left |
| `INF-004-U005` | RETAIN | PROPER_NOUN | quick-match / row 1 / right |
| `INF-004-U006` | TRANSLATE | RECOMMENDATION, BRAND, PROPER_NOUN | quick-match / row 2 / left |
| `INF-004-U007` | RETAIN | PROPER_NOUN | quick-match / row 2 / right |
| `INF-004-U008` | TRANSLATE | RECOMMENDATION | quick-match / row 3 / left |
| `INF-004-U009` | RETAIN | PROPER_NOUN | quick-match / row 3 / right |
| `INF-004-U010` | TRANSLATE | RECOMMENDATION | quick-match / row 4 / left |
| `INF-004-U011` | RETAIN | PROPER_NOUN | quick-match / row 4 / right |
| `INF-004-U012` | TRANSLATE | RECOMMENDATION | quick-match / row 5 / left |
| `INF-004-U013` | RETAIN | PROPER_NOUN | quick-match / row 5 / right |
| `INF-004-U014` | TRANSLATE | RECOMMENDATION | quick-match / row 6 / left |
| `INF-004-U015` | RETAIN | PROPER_NOUN | quick-match / row 6 / right |
| `INF-004-U016` | TRANSLATE | — | top-picks / section heading |
| `INF-004-U017` | RETAIN | PROPER_NOUN | top-picks / left card / heading |
| `INF-004-U018` | TRANSLATE | — | top-picks / left card / illustrated storefront sign |
| `INF-004-U019` | RETAIN | — | top-picks / left card / decorative single-letter monogram |
| `INF-004-U020` | TRANSLATE | RECOMMENDATION | top-picks / left card / best-for |
| `INF-004-U021` | TRANSLATE | FACT | top-picks / left card / warning |
| `INF-004-U022` | RETAIN | PROPER_NOUN | top-picks / middle card / heading |
| `INF-004-U023` | TRANSLATE | RECOMMENDATION | top-picks / middle card / best-for |
| `INF-004-U024` | TRANSLATE | FACT | top-picks / middle card / warning |
| `INF-004-U025` | RETAIN | PROPER_NOUN | top-picks / right card / heading |
| `INF-004-U026` | RETAIN | PROPER_NOUN | top-picks / right card / vertical illustrated place sign |
| `INF-004-U027` | TRANSLATE | RECOMMENDATION | top-picks / right card / best-for |
| `INF-004-U028` | TRANSLATE | — | top-picks / right card / illustrated storefront sign |
| `INF-004-U029` | TRANSLATE | FACT | top-picks / right card / warning |

## RETAIN

- Source place/entity tokens and brand tokens: `Seoul`, `Gangnam`, `Jamsil`, `Myeongdong`, `Seoul Station / Namdaemun`, `Insadong`, `Itaewon`, `Lotte World`, `G`.
- Area-label occurrence count retained separately: 11.
- Numeric constraints: no explicit numeric price/time/distance label appears; do not invent one.
- Preserve native casing of illustrated entity labels where present. Keep descriptions and warnings as independent complete source units.

## THAI TARGET

- Planned asset path: `images/Accommodation/luxury-stay-seoul-area-guide-th.png`
- Format/canvas: PNG, 1536 × 1024 px; same aspect ratio and dimensions.
- HTML future replacement target: `th/best-area-for-luxury-hotels-seoul.html` — future approved Thai page only; this task creates no HTML.
- Thai translation: **NOT STARTED in this source-extraction batch**.
- Existing Thai work: no target asset currently exists.
- Production/deployment: NOT AUTHORIZED.

## ES/JA PRODUCTION PRECEDENT

- ES: `images/Accommodation/luxury-stay-seoul-area-guide-es.png` — exists; PNG 1536 × 1024; `es/best-area-for-luxury-hotels-seoul.html` references `../images/Accommodation/luxury-stay-seoul-area-guide-es.png`.
- JA: `images/Accommodation/luxury-stay-seoul-area-guide-ja.png` — exists; PNG 1536 × 1024; `ja/best-area-for-luxury-hotels-seoul.html` references `../images/Accommodation/luxury-stay-seoul-area-guide-ja.png`.
- These are filename/layout/canvas precedents only. No ES/JA wording was translated into Thai.
- The historical scoped audit does not identify an editable production master/script for this asset. Existing flattened PNG precedents do not establish an editable-source workflow; no new production method is created here.

## PROTECTION

- Facts: preserve source warning and access/transport/room constraints; do not add claims.
- Recommendation: preserve quick-match pairings, top-picks order, best-for statements, and every watch-out sentence.
- Numbers: preserve all displayed values/order and quantity meaning.
- Layout: same canvas, aspect ratio, row/card count, arrows, icon positions, artwork placement, and map/illustration geometry.
- Dimensions/format: 1536 × 1024 PNG.
- Visual hierarchy: preserve heading/subtitle, decision rows, illustrated labels, and original top-picks presentation where present.
- Source image and every existing localized asset: byte-preserved. No pixel edit, generated PNG/WebP/SVG, redesign, HTML replacement, or common-system change.

# INF-005 — Nightlife stay area guide

## SOURCE

- Source asset: `images/Accommodation/nightlife-stay-seoul-area-guide.png`
- Historical audit SHA-256: `ae765cc088069bc9bd156b6e8f9b45e564d08ad6d4e460cfb39249401c2b5a2f`
- Current SHA-256: `ae765cc088069bc9bd156b6e8f9b45e564d08ad6d4e460cfb39249401c2b5a2f`
- SHA comparison: MATCH.
- Format: PNG; original format preserved.
- Dimensions: 1536 × 1024 px; historical/current comparison MATCH.
- Priority: P1; unchanged from the existing ES/JA audit.
- Classification: LOCALIZE; readable Korea Inside editorial English is embedded in the original.
- English page: `best-area-for-nightlife-seoul.html` — EXISTS; 1 occurrence.
- English current src: `images/Accommodation/nightlife-stay-seoul-area-guide.png`
- Thai page: `th/best-area-for-nightlife-seoul.html` — MISSING; no working copy is created.
- Thai current src: NONE — Thai page does not exist.
- English original currently used by Thai page: NOT APPLICABLE — page missing.
- Existing planned Thai asset: NO; future asset only, no file created.
- Exact text coverage: 100% of readable source text; direct visual inspection, no OCR.
- Asset REVIEW_REQUIRED: NO.

## EXACT ENGLISH TEXT

- Exact source units: 14; TRANSLATE 8; RETAIN 6.
- The list and metadata table use the same continuous unit numbering; source position is preserved in the region column.

1. `Where Should Nightlife Travelers Stay in Seoul?`
2. `A quick area guide for nights out and easier returns`
3. `1. Clubs, live music and youthful energy`
4. `Hongdae`
5. `2. International bars and social nights`
6. `Itaewon`
7. `3. Upscale clubs and late dinners`
8. `Gangnam`
9. `4. Central sightseeing with occasional nights out`
10. `Myeongdong`
11. `5. Quieter sleep with easy Hongdae access`
12. `Mapo / Gongdeok`
13. `6. Airport rail and early departures`
14. `Seoul Station`

### Unit metadata

| Unit ID | Action | Protection tags | Original visual region |
|---|---|---|---|
| `INF-005-U001` | TRANSLATE | PROPER_NOUN | header / heading |
| `INF-005-U002` | TRANSLATE | — | header / subtitle |
| `INF-005-U003` | TRANSLATE | RECOMMENDATION, NUMBER | decision / row 1 / left |
| `INF-005-U004` | RETAIN | PROPER_NOUN | decision / row 1 / right |
| `INF-005-U005` | TRANSLATE | RECOMMENDATION, NUMBER | decision / row 2 / left |
| `INF-005-U006` | RETAIN | PROPER_NOUN | decision / row 2 / right |
| `INF-005-U007` | TRANSLATE | RECOMMENDATION, NUMBER | decision / row 3 / left |
| `INF-005-U008` | RETAIN | PROPER_NOUN | decision / row 3 / right |
| `INF-005-U009` | TRANSLATE | RECOMMENDATION, NUMBER | decision / row 4 / left |
| `INF-005-U010` | RETAIN | PROPER_NOUN | decision / row 4 / right |
| `INF-005-U011` | TRANSLATE | RECOMMENDATION, NUMBER, PROPER_NOUN | decision / row 5 / left |
| `INF-005-U012` | RETAIN | PROPER_NOUN | decision / row 5 / right |
| `INF-005-U013` | TRANSLATE | RECOMMENDATION, NUMBER | decision / row 6 / left |
| `INF-005-U014` | RETAIN | PROPER_NOUN | decision / row 6 / right |

## RETAIN

- Source place/entity tokens and brand tokens: `Seoul`, `Hongdae`, `Itaewon`, `Gangnam`, `Myeongdong`, `Mapo / Gongdeok`, `Seoul Station`.
- Area-label occurrence count retained separately: 6.
- Numeric constraints: preserve exact prefixes `1.` through `6.` and six original description/area pairings.
- Preserve native casing of illustrated entity labels where present. Keep descriptions and warnings as independent complete source units.

## THAI TARGET

- Planned asset path: `images/Accommodation/nightlife-stay-seoul-area-guide-th.png`
- Format/canvas: PNG, 1536 × 1024 px; same aspect ratio and dimensions.
- HTML future replacement target: `th/best-area-for-nightlife-seoul.html` — future approved Thai page only; this task creates no HTML.
- Thai translation: **NOT STARTED in this source-extraction batch**.
- Existing Thai work: no target asset currently exists.
- Production/deployment: NOT AUTHORIZED.

## ES/JA PRODUCTION PRECEDENT

- ES: `images/Accommodation/nightlife-stay-seoul-area-guide-es.png` — exists; PNG 1536 × 1024; `es/best-area-for-nightlife-seoul.html` references `../images/Accommodation/nightlife-stay-seoul-area-guide-es.png`.
- JA: `images/Accommodation/nightlife-stay-seoul-area-guide-ja.png` — exists; PNG 1536 × 1024; `ja/best-area-for-nightlife-seoul.html` references `../images/Accommodation/nightlife-stay-seoul-area-guide-ja.png`.
- These are filename/layout/canvas precedents only. No ES/JA wording was translated into Thai.
- The historical scoped audit does not identify an editable production master/script for this asset. Existing flattened PNG precedents do not establish an editable-source workflow; no new production method is created here.

## PROTECTION

- Facts: preserve source warning and access/transport/room constraints; do not add claims.
- Recommendation: preserve quick-match pairings, top-picks order, best-for statements, and every watch-out sentence.
- Numbers: preserve all displayed values/order and quantity meaning.
- Layout: same canvas, aspect ratio, row/card count, arrows, icon positions, artwork placement, and map/illustration geometry.
- Dimensions/format: 1536 × 1024 PNG.
- Visual hierarchy: preserve heading/subtitle, decision rows, illustrated labels, and original top-picks presentation where present.
- Source image and every existing localized asset: byte-preserved. No pixel edit, generated PNG/WebP/SVG, redesign, HTML replacement, or common-system change.

## Batch completion totals

- Target sources: 5/5; source existence: 5/5; SHA match: 5/5; format/dimension match: 5/5.
- Exact English text units: 130; Thai localization units: 85; RETAIN-only occurrences: 45.
- Unit numbering gaps: 0; duplicates within an INF: 0; source region order recorded for every unit.
- Planned Thai paths: 5/5; duplicate target paths: 0; existing target assets: 3; future missing targets: 2.
- Thai related page existence: 3/5; missing: Luxury and Nightlife. No Thai page is created.
- Asset REVIEW_REQUIRED: 0. Known input limitation: missing Thai Standard; incidental out-of-scope documentation search results disclosed above.
- Existing file modifications: 0; existing image modifications: 0; Thai HTML modifications: 0; CSS/JS modifications: 0.
- Source extraction document is left untracked. Stage/commit/push/merge/deploy/Production: 0.
- Next steps are deferred: ChatGPT Thai wording, user review/approval, content lock, asset production, pixel/HTML/browser QA, and any Git/release action. No next infographic batch is started.

**THAI INFOGRAPHIC BATCH 1 SOURCE EXTRACTION COMPLETE — READY FOR CHATGPT THAI LOCALIZATION**
