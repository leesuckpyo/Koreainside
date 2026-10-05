# Korea Inside — Thai Infographic Remaining Source Extraction

**Status:** SOURCE EXTRACTION COMPLETE WITH REVIEW_REQUIRED — READY FOR THAI LOCALIZATION

**Work type:** SOURCE EXTRACTION ONLY

**Thai translation:** NOT STARTED

**Date:** 2026-10-02 (Asia/Seoul)

**Branch / HEAD at start:** `main` / `955a34145165cf117a6a7d2a56e5e17ce604a4b5`

**Execution authority:** Current user-approved remaining-25 one-shot instruction. This document records English source extraction, not Thai Public Copy, approval, image production, implementation or Production.

## 1. Inputs, authority and boundaries

The current user instruction takes precedence. Exactly 25 existing LOCALIZE entries are inherited from the ES/JA audit. No new whole-image inventory was created. INF-001–005 were neither re-extracted nor visually re-tested; their completed release state is inherited from the user instruction only. EXCLUDE entries are classification records only.

| Input | Actual path | Current role |
|---|---|---|
| Public Content Master | `Korea_Inside_Public_Content_Master_Standard.md` | Actual file Version 1.3; full document read |
| Navigation Hub Architecture | `Korea_Inside_Navigation_Hub_Architecture_Standard.md` | Actual file Version 1.0; full document read |
| Language Localization | `Korea_Inside_Language_Localization_Standard.md` | Actual file Version 2.0; full document read |
| Thai Localization | `Korea_Inside_Thai_Localization_Standard_v1.1.md` | ACTIVE / SPECIALIZED STANDARD, Version 1.1; full document read |
| Latest Room Handover | `md/감사본/일본어/Korea_Inside_Room_Handover_2026-09-29.md` | Latest matching date; full document read; historical context only |
| Source inventory / ES + JA precedent | `md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md` | Full document read; original 33 / final LOCALIZE 30 / EXCLUDE 3 |
| Completed Batch 1 extraction | `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | Full document read as prior-format/context reference; no re-extraction or edits |

Project Base documents PROJECT.md, AGENTS.md, docs/product-constitution.md, docs/business-operating-system.md and docs/standards-hub.md were read and applied. The latest handover's quoted standard versions differ from the current files (1.4/1.1/2.1 in the handover versus actual 1.3/1.0/2.0). Current files and this user's explicit source-extraction instruction govern; no historical next task or release action was resumed.

The audit checkpoint `2cf26a55861eb264e2103c29f9953b0632f3266c` records ES/JA local implementation complete and release actions pending at that historical checkpoint. Current verification here is restricted to these 25 IDs: all 50 corresponding ES/JA asset paths exist and their sibling HTML image references match. This is local precedent verification, not renewed Production or pixel QA.

Protected scope: existing approved copy, Thai pages/untracked files, INF-001–005, all existing images/HTML/CSS/JS, common navigation/footer, sitemap, hreflang, Git index, commits and Production. Exactly one new repository MD is authorized. No Thai wording is generated from the English source.

## 2. Extraction method and counting

Read raster assets at native resolution; read SVG text nodes directly. Visual reading order is top-to-bottom, with multi-panel sections read left-to-right; tables are read row-by-row across columns. Each actual repeated text occurrence remains a separate unit. Line wraps within one printed sentence/label are joined with one space; spelling, case, punctuation, numbers, brands and qualifiers are preserved. No inferred heading, clipped suffix or source typo correction is added.

Every unit has exactly one implementation mode (TRANSLATE or RETAIN) and zero or more identity/protection tags (BRAND, PROPER_NOUN, NUMBER, SYMBOL, RECOMMENDATION). These tags overlap and do not add extra units. TRANSLATE means future ChatGPT-approved wording is required, not permission for Codex translation. Proper names/brands/numeric values embedded in a TRANSLATE sentence remain protected.

Original photographed/app/product UI strings, search queries and readable place names are RETAIN. Existing English-selected app-language demonstrations must not be rewritten into unsupported Thai UI. Korean/Chinese/Japanese-only screen or map text is not English extraction; existing non-English source context and associations remain protected. Isolated Latin/numeric tokens within Korean labels are included where meaningful.

Non-text route arrows, pictograms, pins, QR artwork and interface control shapes are protected visual geometry, not expanded into invented text. Readable text-bearing glyphs and explicit numeric labels are counted; unclear glyphs and microtext are isolated under REVIEW_REQUIRED. QR patterns are not decoded.

VISIBLE_FRAGMENT / VISIBLE_TRUNCATION entries reproduce only readable visible text. EXACT_XML entries reproduce source SVG strings; the long bottom tip's pixel extent is separately REVIEW_REQUIRED. Candidate readings (10) are outside the exact-unit totals and must not be promoted to Public Copy without source confirmation. Unknown microtext has no fabricated unit count. Therefore no 100% fully resolved pixel-text-coverage claim is made for review-required assets.

## 3. Scope and QA summary

| Measure | Result |
|---|---:|
| Historical English-text inventory (inherited; not re-audited) | 33 |
| Historical LOCALIZE / EXCLUDE | 30 / 3 |
| Completed INF-001–005 protected; re-extraction / repeated visual QA | 5 / 0 / 0 |
| Remaining LOCALIZE target / documented asset states | 25 / 25 |
| Remaining P1 / P2 | 9 / 16 |
| Extraction without unresolved asset issue / REVIEW_REQUIRED assets | 11 / 14 |
| REVIEW_REQUIRED issue records | 18 |
| Source asset exists / unique source paths | 25 / 25 |
| Current SHA-256 compared with inherited audit / match | 25 / 25 |
| Native dimensions compared with inherited audit / match | 25 / 25 |
| Source format match | 25 / 25 |
| English rendered image occurrences / distinct English pages | 25 / 13 |
| Exact source units (readable fragments and exact SVG included) | 1326 |
| TRANSLATE / RETAIN | 622 / 704 |
| BRAND tagged units | 200 |
| PROPER_NOUN tagged units | 212 |
| NUMBER tagged units | 416 |
| SYMBOL tagged units | 209 |
| RECOMMENDATION tagged units | 65 |
| Visible fragment / truncation units | 8 |
| Exact SVG text nodes | 5 |
| Unconfirmed candidate readings excluded from exact totals | 10 |
| Planned Thai target paths / duplicate paths | 25 / 0 |
| Planned Thai paths already present / missing | 1 / 24 |
| Thai page EXISTS by asset occurrence / unique page | 1 / 1 |
| Thai page MISSING by asset occurrence / unique page | 24 / 12 |
| EXISTS — IMPLEMENTED (reference only) | 1 |
| EXISTS — IMAGE NOT YET LOCALIZED / NOT CURRENTLY USED | 0 / 0 |
| ES/JA corresponding assets / valid page references | 50 / 50 |
| Existing image / HTML / CSS/JS modification | 0 / 0 / 0 |
| Existing protected files changed / new authorized repository files | 0 / 1 |
| Stage / commit / push / deploy / Production actions | 0 / 0 / 0 / 0 / 0 |

Thai page existence is counted per target asset (25) and separately by unique page (13), so multi-image pages are not mistaken for multiple physical HTML pages. INF-006 already has a local Thai asset reference; this task does not re-test its pixels/wording or claim new implementation. Page absence alone is PAGE MISSING, not an asset failure.

## 4. Protection and EXCLUDE records

| IDs | State inherited from current instruction / audit | This task |
|---|---|---|
| INF-001, INF-002, INF-003, INF-004, INF-005 | Completed Thai Batch 1, release `955a34145165cf117a6a7d2a56e5e17ce604a4b5` | Protected; extraction 0, translation 0, image edits 0, repeat visual QA 0 |
| INF-028 | EXCLUDE / BRAND-PRODUCT VISUAL | Classification only; detailed text extraction 0 |
| INF-030 | EXCLUDE / BRAND-PRODUCT VISUAL | Classification only; detailed text extraction 0 |
| INF-032 | EXCLUDE / BRAND-PRODUCT VISUAL | Classification only; detailed text extraction 0 |

Repository-wide baseline: 1100 existing tracked/untracked files fingerprinted solely for preservation. This opaque byte-integrity check is not repeated Batch 1 content/visual QA. Initial tracked modified 0, staged 0, expanded untracked file count 26.

## 5. Remaining-25 inventory

| ID | Priority | Format / native size | Units (T / R) | Extraction state | Thai page state | Planned Thai asset |
|---|---|---|---:|---|---|---|
| INF-006 | P1 | WEBP 1672×941 | 5 (3 / 2) | EXTRACTION COMPLETE | EXISTS — IMPLEMENTED | `images/Accommodation/hongdae-vs-myeongdong-th.webp` |
| INF-007 | P2 | PNG 1448×1086 | 37 (25 / 12) | REVIEW_REQUIRED | PAGE MISSING | `images/airport/airport-bus-boarding-location-guide-th.png` |
| INF-008 | P2 | PNG 1536×1024 | 136 (35 / 101) | REVIEW_REQUIRED | PAGE MISSING | `images/airport/airport-bus-by-hotel-area-th.png` |
| INF-009 | P2 | PNG 1536×1024 | 69 (56 / 13) | EXTRACTION COMPLETE | PAGE MISSING | `images/airport/airport-bus-how-to-use-th.png` |
| INF-010 | P1 | PNG 1448×1086 | 42 (17 / 25) | EXTRACTION COMPLETE | PAGE MISSING | `images/airport/arex/arex-express-vs-all-stop-route-map-th.png` |
| INF-011 | P1 | PNG 1672×941 | 13 (2 / 11) | EXTRACTION COMPLETE | PAGE MISSING | `images/airport/arex/arex-hero-express-vs-all-stop-th.png` |
| INF-012 | P1 | PNG 768×1024 | 116 (83 / 33) | REVIEW_REQUIRED | PAGE MISSING | `images/airport/arex/arex-station-by-destination-th.png` |
| INF-013 | P2 | PNG 736×1024 | 73 (51 / 22) | REVIEW_REQUIRED | PAGE MISSING | `images/airport/arex/arex-terminal-1-2-directions-th.png` |
| INF-014 | P1 | PNG 1536×1024 | 118 (98 / 20) | EXTRACTION COMPLETE | PAGE MISSING | `images/airport/arex/arex-ticket-decision-guide-th.png` |
| INF-015 | P2 | PNG 1448×1086 | 29 (23 / 6) | EXTRACTION COMPLETE | PAGE MISSING | `images/airport/arex/when-not-to-use-arex-th.png` |
| INF-016 | P1 | WEBP 1402×1122 | 26 (19 / 7) | REVIEW_REQUIRED | PAGE MISSING | `images/airport-arrival-hall-first-30-minutes-infographic-th.webp` |
| INF-017 | P2 | PNG 1536×1024 | 77 (24 / 53) | REVIEW_REQUIRED | PAGE MISSING | `images/arrival/terminal-arrival-maps-th.png` |
| INF-018 | P1 | PNG 1663×946 | 17 (11 / 6) | EXTRACTION COMPLETE | PAGE MISSING | `images/esim/hero-esim-th.png` |
| INF-019 | P1 | WEBP 1536×600 | 15 (8 / 7) | EXTRACTION COMPLETE | PAGE MISSING | `images/esim/esim-korea-quick-decision-th.webp` |
| INF-020 | P1 | PNG 1457×1080 | 21 (6 / 15) | REVIEW_REQUIRED | PAGE MISSING | `images/home/arrival-guide-th.png` |
| INF-021 | P2 | WEBP 1672×941 | 25 (16 / 9) | EXTRACTION COMPLETE | PAGE MISSING | `images/hongdae/hongdae-at-a-glance-map-th.webp` |
| INF-022 | P2 | WEBP 1536×1024 | 19 (8 / 11) | REVIEW_REQUIRED | PAGE MISSING | `images/jamsil/jamsil-at-a-glance-map-th.webp` |
| INF-023 | P2 | WEBP 1376×768 | 18 (11 / 7) | EXTRACTION COMPLETE | PAGE MISSING | `images/seongsu/seongsu-at-a-glance-map-th.webp` |
| INF-024 | P2 | WEBP 1448×1086 | 71 (9 / 62) | EXTRACTION COMPLETE | PAGE MISSING | `images/naver-map-language-guide-th.webp` |
| INF-025 | P2 | WEBP 1448×1086 | 80 (23 / 57) | REVIEW_REQUIRED | PAGE MISSING | `images/naver-map-place-search-guide-th.webp` |
| INF-026 | P2 | WEBP 1448×1086 | 135 (28 / 107) | REVIEW_REQUIRED | PAGE MISSING | `images/naver-map-route-exit-bus-guide-th.webp` |
| INF-027 | P2 | PNG 1536×1024 | 73 (30 / 43) | REVIEW_REQUIRED | PAGE MISSING | `images/tmoney/tmoney-buy-recharge-use-th.png` |
| INF-029 | P2 | SVG 900×560 | 5 (5 / 0) | REVIEW_REQUIRED | PAGE MISSING | `images/tmoney/tmoney-recharge-machine-th.svg` |
| INF-031 | P2 | PNG 1536×1024 | 68 (19 / 49) | REVIEW_REQUIRED | PAGE MISSING | `images/wowpass/wowpass-guide-th.png` |
| INF-033 | P2 | PNG 1505×395 | 38 (12 / 26) | REVIEW_REQUIRED | PAGE MISSING | `images/wowpass/wowpass-use-flow-th.png` |

## 6. Asset source extraction

# INF-006 — Hongdae vs Myeongdong hero comparison

**Asset status:** EXTRACTION COMPLETE

## A. IDENTITY / SOURCE

- INF ID: `INF-006`
- Source filename: `hongdae-vs-myeongdong.webp`
- Source asset: `images/Accommodation/hongdae-vs-myeongdong.webp`
- Format: WEBP
- Native dimensions: 1672 × 941 px
- Current SHA-256: `a1bdbff1f785e12489fc30bd3009f8c46648d5f3aba48e5dc314c4b7b6707537`
- Inherited audit SHA-256: `a1bdbff1f785e12489fc30bd3009f8c46648d5f3aba48e5dc314c4b7b6707537`
- SHA comparison: MATCH
- Inherited audit dimensions: 1672 × 941 px; comparison: MATCH
- Priority: P1
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `hongdae-vs-myeongdong.html`, image reference line 387
- Usage location: Hero / comparison figure after The short answer card
- Existing wrapper: `<figure class="stay-hero-media" style="grid-column: 1 / -1; margin: 0;">`
- English alt (reference only, not an embedded image unit): `Editorial illustration comparing the evening atmosphere of Hongdae and Myeongdong in Seoul.`
- English caption (reference only): `Editorial illustration comparing the evening atmosphere of Hongdae and Myeongdong in Seoul.`
- Thai page: `th/hongdae-vs-myeongdong.html`
- Thai page state: EXISTS — IMPLEMENTED
- Thai page currently uses English source image: NO
- Current Thai image reference: `../images/Accommodation/hongdae-vs-myeongdong-th.webp` → `images/Accommodation/hongdae-vs-myeongdong-th.webp`; local file EXISTS
- Planned Thai filename: `hongdae-vs-myeongdong-th.webp`
- Planned Thai path: `images/Accommodation/hongdae-vs-myeongdong-th.webp`
- Planned path current state: EXISTS — existing user/local asset; unchanged
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/Accommodation/hongdae-vs-myeongdong-es.webp` — EXISTS
  - Sibling `es/hongdae-vs-myeongdong.html`; page EXISTS; image reference `../images/Accommodation/hongdae-vs-myeongdong-es.webp`
- JA precedent asset: `images/Accommodation/hongdae-vs-myeongdong-ja.webp` — EXISTS
  - Sibling `ja/hongdae-vs-myeongdong.html`; page EXISTS; image reference `../images/Accommodation/hongdae-vs-myeongdong-ja.webp`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-006-U001` — `Hongdae`
2. `INF-006-U002` — `vs`
3. `INF-006-U003` — `Myeongdong`
4. `INF-006-U004` — `Cafés • Creative Culture • Youthful Energy`
5. `INF-006-U005` — `Shopping • Central Location • Easy Access`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-006-U001 | RETAIN | PROPER_NOUN | comparison / left title | VISIBLE |
| INF-006-U002 | TRANSLATE | SYMBOL | comparison / center connector | VISIBLE |
| INF-006-U003 | RETAIN | PROPER_NOUN | comparison / right title | VISIBLE |
| INF-006-U004 | TRANSLATE | RECOMMENDATION, SYMBOL | comparison / left subtitle | VISIBLE |
| INF-006-U005 | TRANSLATE | RECOMMENDATION, SYMBOL | comparison / right subtitle | VISIBLE |

Exact units: 5; TRANSLATE 3; RETAIN 2.

## E. PROTECTION

- Preserve native 1672×941 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Keep Hongdae on the left and Myeongdong on the right; retain the center comparison connector and each area's exact three-part descriptor.

### Source-specific notes

- Natural storefront signs and photographic branding are outside the editorial text inventory, as in the inherited audit. Existing Thai asset/reference presence is recorded only; its wording and pixels are not re-audited.

### REVIEW_REQUIRED

NONE.

# INF-007 — Airport bus boarding location guide

**Asset status:** REVIEW_REQUIRED

## A. IDENTITY / SOURCE

- INF ID: `INF-007`
- Source filename: `airport-bus-boarding-location-guide.png`
- Source asset: `images/airport/airport-bus-boarding-location-guide.png`
- Format: PNG
- Native dimensions: 1448 × 1086 px
- Current SHA-256: `45f281d1e6a80f4fac1f36f12399324edf8526ec9d63771f63fa1a3529c7664d`
- Inherited audit SHA-256: `45f281d1e6a80f4fac1f36f12399324edf8526ec9d63771f63fa1a3529c7664d`
- SHA comparison: MATCH
- Inherited audit dimensions: 1448 × 1086 px; comparison: MATCH
- Priority: P2
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `airport-bus.html`, image reference line 223
- Usage location: Airport bus boarding chapter / Terminal 2 subsection
- Existing wrapper: `<figure class="airport-bus-media__figure airport-bus-media__figure--wide">`
- English alt (reference only, not an embedded image unit): `Incheon Airport bus boarding location guide for Terminal 1 and Terminal 2`
- English caption (reference only): `Visual guide to Terminal 1 and Terminal 2 bus boarding locations`
- Thai page: `th/airport-bus.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `airport-bus-boarding-location-guide-th.png`
- Planned Thai path: `images/airport/airport-bus-boarding-location-guide-th.png`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/airport/airport-bus-boarding-location-guide-es.png` — EXISTS
  - Sibling `es/airport-bus.html`; page EXISTS; image reference `../images/airport/airport-bus-boarding-location-guide-es.png`
- JA precedent asset: `images/airport/airport-bus-boarding-location-guide-ja.png` — EXISTS
  - Sibling `ja/airport-bus.html`; page EXISTS; image reference `../images/airport/airport-bus-boarding-location-guide-ja.png`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-007-U001` — `Korea Inside`
2. `INF-007-U002` — `Airport Bus Boarding Location Guide`
3. `INF-007-U003` — `INCHEON AIRPORT BUS NUMBER & BOARDING CHECK`
4. `INF-007-U004` — `START HERE`
5. `INF-007-U005` — `Check your bus number first. Match the route number and destination on the airport sign before you go to the boarding area.`
6. `INF-007-U006` — `TERMINAL 1`
7. `INF-007-U007` — `ARRINAL HALL`
8. `INF-007-U008` — `Follow airport bus signs after arrival. Go to the official bus stop area and check the route number on the platform sign.`
9. `INF-007-U009` — `TERMINAL 2`
10. `INF-007-U010` — `TRANSPORTATION CENTER`
11. `INF-007-U011` — `Use the official airport bus signs to reach the boarding area. Confirm the bus number and destination again before boarding.`
12. `INF-007-U012` — `STEP 1 — FIND THE NUMBER`
13. `INF-007-U013` — `Look for your route number, such as 6000-series, 6700-series, late-night buses, or regional routes.`
14. `INF-007-U014` — `6001`
15. `INF-007-U015` — `6002`
16. `INF-007-U016` — `6701`
17. `INF-007-U017` — `N6001`
18. `INF-007-U018` — `Examples only`
19. `INF-007-U019` — `STEP 2 — MATCH THE DESTINATION`
20. `INF-007-U020` — `Some bus numbers are similar. Always match both the route number and the destination name shown on the airport sign.`
21. `INF-007-U021` — `6001`
22. `INF-007-U022` — `Seoul Station`
23. `INF-007-U023` — `6701`
24. `INF-007-U024` — `Gangnam`
25. `INF-007-U025` — `STEP 3 — CHECK THE PLATFORM`
26. `INF-007-U026` — `At the boarding area, read the platform sign again. The platform may serve several routes, so confirm your bus before you wait.`
27. `INF-007-U027` — `STEP 4 — BOARD SAFELY`
28. `INF-007-U028` — `When the bus arrives, check the front display and ask the staff or driver if you are unsure.`
29. `INF-007-U029` — `COMMON ROUTE TYPES`
30. `INF-007-U030` — `• 6000-series: many Seoul hotel and district routes`
31. `INF-007-U031` — `• 6700-series: premium limousine routes`
32. `INF-007-U032` — `• Late-night buses: limited hours`
33. `INF-007-U033` — `• Regional buses: cities outside central Seoul`
34. `INF-007-U034` — `These are route families, not exact boarding assignments. Please check the airport sign.`
35. `INF-007-U035` — `IMPORTANT NOTE`
36. `INF-007-U036` — `Boarding locations can change. Always confirm the latest route number, destination, and platform on the official airport sign on the day of travel.`
37. `INF-007-U037` — `Korea Inside`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-007-U001 | RETAIN | BRAND | header / brand | VISIBLE |
| INF-007-U002 | TRANSLATE | — | header / title | VISIBLE |
| INF-007-U003 | TRANSLATE | PROPER_NOUN, SYMBOL | header / subtitle | VISIBLE |
| INF-007-U004 | TRANSLATE | — | upper left / heading | VISIBLE |
| INF-007-U005 | TRANSLATE | — | upper left / instruction | VISIBLE |
| INF-007-U006 | RETAIN | PROPER_NOUN, NUMBER | upper middle / terminal heading | VISIBLE |
| INF-007-U007 | TRANSLATE | — | upper middle / diagram sign | VISIBLE |
| INF-007-U008 | TRANSLATE | — | upper middle / instruction | VISIBLE |
| INF-007-U009 | RETAIN | PROPER_NOUN, NUMBER | upper right / terminal heading | VISIBLE |
| INF-007-U010 | TRANSLATE | — | upper right / diagram sign | VISIBLE |
| INF-007-U011 | TRANSLATE | — | upper right / instruction | VISIBLE |
| INF-007-U012 | TRANSLATE | NUMBER, SYMBOL | steps / card 1 / heading | VISIBLE |
| INF-007-U013 | TRANSLATE | NUMBER | steps / card 1 / instruction | VISIBLE |
| INF-007-U014 | RETAIN | NUMBER | steps / card 1 / example 1 | VISIBLE |
| INF-007-U015 | RETAIN | NUMBER | steps / card 1 / example 2 | VISIBLE |
| INF-007-U016 | RETAIN | NUMBER | steps / card 1 / example 3 | VISIBLE |
| INF-007-U017 | RETAIN | NUMBER | steps / card 1 / example 4 | VISIBLE |
| INF-007-U018 | TRANSLATE | — | steps / card 1 / qualifier | VISIBLE |
| INF-007-U019 | TRANSLATE | NUMBER, SYMBOL | steps / card 2 / heading | VISIBLE |
| INF-007-U020 | TRANSLATE | — | steps / card 2 / instruction | VISIBLE |
| INF-007-U021 | RETAIN | NUMBER | steps / card 2 / table row 1 / route | VISIBLE |
| INF-007-U022 | RETAIN | PROPER_NOUN | steps / card 2 / table row 1 / destination | VISIBLE |
| INF-007-U023 | RETAIN | NUMBER | steps / card 2 / table row 2 / route | VISIBLE |
| INF-007-U024 | RETAIN | PROPER_NOUN | steps / card 2 / table row 2 / destination | VISIBLE |
| INF-007-U025 | TRANSLATE | NUMBER, SYMBOL | steps / card 3 / heading | VISIBLE |
| INF-007-U026 | TRANSLATE | — | steps / card 3 / instruction | VISIBLE |
| INF-007-U027 | TRANSLATE | NUMBER, SYMBOL | steps / card 4 / heading | VISIBLE |
| INF-007-U028 | TRANSLATE | — | steps / card 4 / instruction | VISIBLE |
| INF-007-U029 | TRANSLATE | — | lower left / heading | VISIBLE |
| INF-007-U030 | TRANSLATE | NUMBER, PROPER_NOUN, SYMBOL | lower left / route family 1 | VISIBLE |
| INF-007-U031 | TRANSLATE | NUMBER, SYMBOL | lower left / route family 2 | VISIBLE |
| INF-007-U032 | TRANSLATE | SYMBOL | lower left / route family 3 | VISIBLE |
| INF-007-U033 | TRANSLATE | PROPER_NOUN, SYMBOL | lower left / route family 4 | VISIBLE |
| INF-007-U034 | TRANSLATE | — | lower middle / warning | VISIBLE |
| INF-007-U035 | TRANSLATE | — | lower right / heading | VISIBLE |
| INF-007-U036 | TRANSLATE | — | lower right / warning | VISIBLE |
| INF-007-U037 | RETAIN | BRAND | footer / brand | VISIBLE |

Exact units: 37; TRANSLATE 25; RETAIN 12.

## E. PROTECTION

- Preserve native 1448×1086 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Terminal 1 and Terminal 2 are separate diagrams; preserve arrows to each boarding area, numbered step order 1–4, and route-family versus actual-platform warning.
- Preserve examples 6001, 6002, 6701, N6001 and the example pairing 6001 → Seoul Station, 6701 → Gangnam.

### Source-specific notes

- 6000-series / 6700-series are route families, not exact boarding assignments. Source typo is not silently repaired.

### REVIEW_REQUIRED

1. **Terminal 1 / diagram sign** — The source sign visibly reads ARRINAL HALL, rather than the expected ARRIVAL HALL. Preserve the source spelling in extraction; editorial correction/localization approval is required before changing it.

# INF-008 — Airport bus by hotel area

**Asset status:** REVIEW_REQUIRED

## A. IDENTITY / SOURCE

- INF ID: `INF-008`
- Source filename: `airport-bus-by-hotel-area.png`
- Source asset: `images/airport/airport-bus-by-hotel-area.png`
- Format: PNG
- Native dimensions: 1536 × 1024 px
- Current SHA-256: `c85d8dac5f4cc3c865944fb03b0f30ee1efc8f6402b5b6d47ef33ee3e93ee680`
- Inherited audit SHA-256: `c85d8dac5f4cc3c865944fb03b0f30ee1efc8f6402b5b6d47ef33ee3e93ee680`
- SHA comparison: MATCH
- Inherited audit dimensions: 1536 × 1024 px; comparison: MATCH
- Priority: P2
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `airport-bus.html`, image reference line 166
- Usage location: Hotel destination and route-selection instructions
- Existing wrapper: `<figure class="airport-bus-media__figure airport-bus-media__figure--wide">`
- English alt (reference only, not an embedded image unit): `Airport bus selection guide by hotel area in Seoul and nearby regions`
- English caption (reference only): `Bus routes and boarding locations may change. Confirm the latest information on the official airport or bus operator website before travel.`
- Thai page: `th/airport-bus.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `airport-bus-by-hotel-area-th.png`
- Planned Thai path: `images/airport/airport-bus-by-hotel-area-th.png`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/airport/airport-bus-by-hotel-area-es.png` — EXISTS
  - Sibling `es/airport-bus.html`; page EXISTS; image reference `../images/airport/airport-bus-by-hotel-area-es.png`
- JA precedent asset: `images/airport/airport-bus-by-hotel-area-ja.png` — EXISTS
  - Sibling `ja/airport-bus.html`; page EXISTS; image reference `../images/airport/airport-bus-by-hotel-area-ja.png`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-008-U001` — `Korea Inside`
2. `INF-008-U002` — `Terminal 2 Ground Transportation Guide`
3. `INF-008-U003` — `INCHEON AIRPORT TERMINAL 2 ARRIVAL HALL (1F)`
4. `INF-008-U004` — `START HERE`
5. `INF-008-U005` — `• Exit customs on the Arrivals Hall (1F)`
6. `INF-008-U006` — `• Follow the “GROUND TRANSPORTATION” signs`
7. `INF-008-U007` — `• Choose your ride`
8. `INF-008-U008` — `• Pay at the counter or kiosk`
9. `INF-008-U009` — `• Enjoy a safe trip!`
10. `INF-008-U010` — `TERMINAL 2 ARRIVAL HALL (1F)`
11. `INF-008-U011` — `GATE`
12. `INF-008-U012` — `GATE`
13. `INF-008-U013` — `ARRIVAL HALL`
14. `INF-008-U014` — `4`
15. `INF-008-U015` — `Bus Stop`
16. `INF-008-U016` — `TAXI`
17. `INF-008-U017` — `Taxi Stand`
18. `INF-008-U018` — `Pick-up Point`
19. `INF-008-U019` — `Cross the zebra crossing to the outer curb (1F)`
20. `INF-008-U020` — `1`
21. `INF-008-U021` — `AIRPORT BUS`
22. `INF-008-U022` — `Take the airport limousine bus to major cities.`
23. `INF-008-U023` — `Ticket: ₩ 17,000 ~ 18,000 (varies by destination)`
24. `INF-008-U024` — `Buy at the ticket counter or kiosk.`
25. `INF-008-U025` — `Bus Stop 6 ~ 11`
26. `INF-008-U026` — `2`
27. `INF-008-U027` — `TAXI`
28. `INF-008-U028` — `Take a taxi to your destination.`
29. `INF-008-U029` — `Basic Fare: ₩ 4,800 (metered)`
30. `INF-008-U030` — `Taxi Stand 4D ~ 6D`
31. `INF-008-U031` — `3`
32. `INF-008-U032` — `AREX / TRAIN`
33. `INF-008-U033` — `Take the AREX Express or All Stop Train to Seoul Station.`
34. `INF-008-U034` — `Express: 43 min`
35. `INF-008-U035` — `All Stop: 56 min`
36. `INF-008-U036` — `B1F (Follow the signs)`
37. `INF-008-U037` — `4`
38. `INF-008-U038` — `PICK-UP`
39. `INF-008-U039` — `Meet your driver in the designated pick-up area.`
40. `INF-008-U040` — `Pick-up Point 4C, 5C`
41. `INF-008-U041` — `Traffic may vary. Allow extra time for your journey.`
42. `INF-008-U042` — `Korea Inside`
43. `INF-008-U043` — `Korea Inside`
44. `INF-008-U044` — `T1·T2`
45. `INF-008-U045` — `1 (1F)`
46. `INF-008-U046` — `6`
47. `INF-008-U047` — `7`
48. `INF-008-U048` — `8`
49. `INF-008-U049` — `9`
50. `INF-008-U050` — `10`
51. `INF-008-U051` — `11`
52. `INF-008-U052` — `6001`
53. `INF-008-U053` — `8A-1`
54. `INF-008-U054` — `6002`
55. `INF-008-U055` — `8A-2`
56. `INF-008-U056` — `6003`
57. `INF-008-U057` — `8B-1`
58. `INF-008-U058` — `6004`
59. `INF-008-U059` — `9A`
60. `INF-008-U060` — `6701`
61. `INF-008-U061` — `10B`
62. `INF-008-U062` — `6702`
63. `INF-008-U063` — `10A`
64. `INF-008-U064` — `6703`
65. `INF-008-U065` — `11B`
66. `INF-008-U066` — `6705`
67. `INF-008-U067` — `6A`
68. `INF-008-U068` — `2 (1F)`
69. `INF-008-U069` — `6`
70. `INF-008-U070` — `7`
71. `INF-008-U071` — `8`
72. `INF-008-U072` — `9`
73. `INF-008-U073` — `10`
74. `INF-008-U074` — `11`
75. `INF-008-U075` — `6001`
76. `INF-008-U076` — `6A-1`
77. `INF-008-U077` — `6002`
78. `INF-008-U078` — `6A-2`
79. `INF-008-U079` — `6003`
80. `INF-008-U080` — `7A`
81. `INF-008-U081` — `6004`
82. `INF-008-U082` — `8A`
83. `INF-008-U083` — `6701`
84. `INF-008-U084` — `9B`
85. `INF-008-U085` — `6702`
86. `INF-008-U086` — `10A`
87. `INF-008-U087` — `6703`
88. `INF-008-U088` — `11A`
89. `INF-008-U089` — `6705`
90. `INF-008-U090` — `6B`
91. `INF-008-U091` — `Korea Inside`
92. `INF-008-U092` — `Korea Inside`
93. `INF-008-U093` — `6004`
94. `INF-008-U094` — `T1`
95. `INF-008-U095` — `9A`
96. `INF-008-U096` — `9A`
97. `INF-008-U097` — `6001`
98. `INF-008-U098` — `T1`
99. `INF-008-U099` — `8A-1`
100. `INF-008-U100` — `T2`
101. `INF-008-U101` — `6002`
102. `INF-008-U102` — `T1`
103. `INF-008-U103` — `8A-2`
104. `INF-008-U104` — `T2`
105. `INF-008-U105` — `6003`
106. `INF-008-U106` — `T1`
107. `INF-008-U107` — `8B-1`
108. `INF-008-U108` — `TA`
109. `INF-008-U109` — `6702`
110. `INF-008-U110` — `T1`
111. `INF-008-U111` — `10A`
112. `INF-008-U112` — `T2`
113. `INF-008-U113` — `10A`
114. `INF-008-U114` — `6701`
115. `INF-008-U115` — `T1`
116. `INF-008-U116` — `10B`
117. `INF-008-U117` — `T2`
118. `INF-008-U118` — `9B`
119. `INF-008-U119` — `6702`
120. `INF-008-U120` — `T1`
121. `INF-008-U121` — `10A`
122. `INF-008-U122` — `T2`
123. `INF-008-U123` — `10A`
124. `INF-008-U124` — `6703`
125. `INF-008-U125` — `T1`
126. `INF-008-U126` — `11B`
127. `INF-008-U127` — `T2`
128. `INF-008-U128` — `11A`
129. `INF-008-U129` — `6705`
130. `INF-008-U130` — `T1`
131. `INF-008-U131` — `6A`
132. `INF-008-U132` — `T2`
133. `INF-008-U133` — `6B`
134. `INF-008-U134` — `Korea Inside`
135. `INF-008-U135` — `2024`
136. `INF-008-U136` — `7`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-008-U001 | RETAIN | BRAND | upper left panel / header brand | VISIBLE |
| INF-008-U002 | TRANSLATE | PROPER_NOUN, NUMBER | upper left panel / title | VISIBLE |
| INF-008-U003 | TRANSLATE | PROPER_NOUN, NUMBER | upper left panel / subtitle | VISIBLE |
| INF-008-U004 | TRANSLATE | — | upper left panel / first box heading | VISIBLE |
| INF-008-U005 | TRANSLATE | NUMBER, SYMBOL | upper left panel / first box item 1 | VISIBLE |
| INF-008-U006 | TRANSLATE | SYMBOL | upper left panel / first box item 2 | VISIBLE |
| INF-008-U007 | TRANSLATE | SYMBOL | upper left panel / first box item 3 | VISIBLE |
| INF-008-U008 | TRANSLATE | SYMBOL | upper left panel / first box item 4 | VISIBLE |
| INF-008-U009 | TRANSLATE | SYMBOL | upper left panel / first box item 5 | VISIBLE |
| INF-008-U010 | TRANSLATE | PROPER_NOUN, NUMBER | upper left panel / map heading | VISIBLE |
| INF-008-U011 | TRANSLATE | — | upper left panel / map left gate | VISIBLE |
| INF-008-U012 | TRANSLATE | — | upper left panel / map right gate | VISIBLE |
| INF-008-U013 | TRANSLATE | — | upper left panel / map center label | VISIBLE |
| INF-008-U014 | RETAIN | NUMBER | upper left panel / map gate number | VISIBLE |
| INF-008-U015 | TRANSLATE | — | upper left panel / map bus label | VISIBLE |
| INF-008-U016 | TRANSLATE | — | upper left panel / map taxi label | VISIBLE |
| INF-008-U017 | TRANSLATE | — | upper left panel / map taxi sublabel | VISIBLE |
| INF-008-U018 | TRANSLATE | — | upper left panel / map pickup sublabel | VISIBLE |
| INF-008-U019 | TRANSLATE | NUMBER | upper left panel / map instruction | VISIBLE |
| INF-008-U020 | RETAIN | NUMBER | upper left panel / transport card 1 badge | VISIBLE |
| INF-008-U021 | TRANSLATE | — | upper left panel / transport card 1 heading | VISIBLE |
| INF-008-U022 | TRANSLATE | — | upper left panel / transport card 1 instruction | VISIBLE |
| INF-008-U023 | TRANSLATE | NUMBER, SYMBOL | upper left panel / transport card 1 fare | VISIBLE |
| INF-008-U024 | TRANSLATE | — | upper left panel / transport card 1 purchase | VISIBLE |
| INF-008-U025 | TRANSLATE | NUMBER, SYMBOL | upper left panel / transport card 1 stop range | VISIBLE |
| INF-008-U026 | RETAIN | NUMBER | upper left panel / transport card 2 badge | VISIBLE |
| INF-008-U027 | TRANSLATE | — | upper left panel / transport card 2 heading | VISIBLE |
| INF-008-U028 | TRANSLATE | — | upper left panel / transport card 2 instruction | VISIBLE |
| INF-008-U029 | TRANSLATE | NUMBER, SYMBOL | upper left panel / transport card 2 fare | VISIBLE |
| INF-008-U030 | TRANSLATE | NUMBER, SYMBOL | upper left panel / transport card 2 stand range | VISIBLE |
| INF-008-U031 | RETAIN | NUMBER | upper left panel / transport card 3 badge | VISIBLE |
| INF-008-U032 | TRANSLATE | BRAND, SYMBOL | upper left panel / transport card 3 heading | VISIBLE |
| INF-008-U033 | TRANSLATE | BRAND, PROPER_NOUN | upper left panel / transport card 3 instruction | VISIBLE |
| INF-008-U034 | TRANSLATE | NUMBER | upper left panel / transport card 3 Express time | VISIBLE |
| INF-008-U035 | TRANSLATE | NUMBER | upper left panel / transport card 3 All Stop time | VISIBLE |
| INF-008-U036 | TRANSLATE | NUMBER | upper left panel / transport card 3 level | VISIBLE |
| INF-008-U037 | RETAIN | NUMBER | upper left panel / transport card 4 badge | VISIBLE |
| INF-008-U038 | TRANSLATE | — | upper left panel / transport card 4 heading | VISIBLE |
| INF-008-U039 | TRANSLATE | — | upper left panel / transport card 4 instruction | VISIBLE |
| INF-008-U040 | TRANSLATE | NUMBER | upper left panel / transport card 4 points | VISIBLE |
| INF-008-U041 | TRANSLATE | — | upper left panel / warning | VISIBLE |
| INF-008-U042 | RETAIN | BRAND | upper left panel / footer brand | VISIBLE |
| INF-008-U043 | RETAIN | BRAND | upper right panel / header brand | VISIBLE |
| INF-008-U044 | RETAIN | NUMBER, SYMBOL | upper right panel / code inside Korean title | VISIBLE |
| INF-008-U045 | RETAIN | NUMBER | upper right panel / Terminal 1 Korean heading numeric substring | VISIBLE |
| INF-008-U046 | RETAIN | NUMBER | upper right panel / Terminal 1 diagram exit 6 | VISIBLE |
| INF-008-U047 | RETAIN | NUMBER | upper right panel / Terminal 1 diagram exit 7 | VISIBLE |
| INF-008-U048 | RETAIN | NUMBER | upper right panel / Terminal 1 diagram exit 8 | VISIBLE |
| INF-008-U049 | RETAIN | NUMBER | upper right panel / Terminal 1 diagram exit 9 | VISIBLE |
| INF-008-U050 | RETAIN | NUMBER | upper right panel / Terminal 1 diagram exit 10 | VISIBLE |
| INF-008-U051 | RETAIN | NUMBER | upper right panel / Terminal 1 diagram exit 11 | VISIBLE |
| INF-008-U052 | RETAIN | NUMBER | upper right panel / Terminal 1 table / route 6001 | VISIBLE |
| INF-008-U053 | RETAIN | NUMBER | upper right panel / Terminal 1 table / 6001 platform | VISIBLE |
| INF-008-U054 | RETAIN | NUMBER | upper right panel / Terminal 1 table / route 6002 | VISIBLE |
| INF-008-U055 | RETAIN | NUMBER | upper right panel / Terminal 1 table / 6002 platform | VISIBLE |
| INF-008-U056 | RETAIN | NUMBER | upper right panel / Terminal 1 table / route 6003 | VISIBLE |
| INF-008-U057 | RETAIN | NUMBER | upper right panel / Terminal 1 table / 6003 platform | VISIBLE |
| INF-008-U058 | RETAIN | NUMBER | upper right panel / Terminal 1 table / route 6004 | VISIBLE |
| INF-008-U059 | RETAIN | NUMBER | upper right panel / Terminal 1 table / 6004 platform | VISIBLE |
| INF-008-U060 | RETAIN | NUMBER | upper right panel / Terminal 1 table / route 6701 | VISIBLE |
| INF-008-U061 | RETAIN | NUMBER | upper right panel / Terminal 1 table / 6701 platform | VISIBLE |
| INF-008-U062 | RETAIN | NUMBER | upper right panel / Terminal 1 table / route 6702 | VISIBLE |
| INF-008-U063 | RETAIN | NUMBER | upper right panel / Terminal 1 table / 6702 platform | VISIBLE |
| INF-008-U064 | RETAIN | NUMBER | upper right panel / Terminal 1 table / route 6703 | VISIBLE |
| INF-008-U065 | RETAIN | NUMBER | upper right panel / Terminal 1 table / 6703 platform | VISIBLE |
| INF-008-U066 | RETAIN | NUMBER | upper right panel / Terminal 1 table / route 6705 | VISIBLE |
| INF-008-U067 | RETAIN | NUMBER | upper right panel / Terminal 1 table / 6705 platform | VISIBLE |
| INF-008-U068 | RETAIN | NUMBER | upper right panel / Terminal 2 Korean heading numeric substring | VISIBLE |
| INF-008-U069 | RETAIN | NUMBER | upper right panel / Terminal 2 diagram exit 6 | VISIBLE |
| INF-008-U070 | RETAIN | NUMBER | upper right panel / Terminal 2 diagram exit 7 | VISIBLE |
| INF-008-U071 | RETAIN | NUMBER | upper right panel / Terminal 2 diagram exit 8 | VISIBLE |
| INF-008-U072 | RETAIN | NUMBER | upper right panel / Terminal 2 diagram exit 9 | VISIBLE |
| INF-008-U073 | RETAIN | NUMBER | upper right panel / Terminal 2 diagram exit 10 | VISIBLE |
| INF-008-U074 | RETAIN | NUMBER | upper right panel / Terminal 2 diagram exit 11 | VISIBLE |
| INF-008-U075 | RETAIN | NUMBER | upper right panel / Terminal 2 table / route 6001 | VISIBLE |
| INF-008-U076 | RETAIN | NUMBER | upper right panel / Terminal 2 table / 6001 platform | VISIBLE |
| INF-008-U077 | RETAIN | NUMBER | upper right panel / Terminal 2 table / route 6002 | VISIBLE |
| INF-008-U078 | RETAIN | NUMBER | upper right panel / Terminal 2 table / 6002 platform | VISIBLE |
| INF-008-U079 | RETAIN | NUMBER | upper right panel / Terminal 2 table / route 6003 | VISIBLE |
| INF-008-U080 | RETAIN | NUMBER | upper right panel / Terminal 2 table / 6003 platform | VISIBLE |
| INF-008-U081 | RETAIN | NUMBER | upper right panel / Terminal 2 table / route 6004 | VISIBLE |
| INF-008-U082 | RETAIN | NUMBER | upper right panel / Terminal 2 table / 6004 platform | VISIBLE |
| INF-008-U083 | RETAIN | NUMBER | upper right panel / Terminal 2 table / route 6701 | VISIBLE |
| INF-008-U084 | RETAIN | NUMBER | upper right panel / Terminal 2 table / 6701 platform | VISIBLE |
| INF-008-U085 | RETAIN | NUMBER | upper right panel / Terminal 2 table / route 6702 | VISIBLE |
| INF-008-U086 | RETAIN | NUMBER | upper right panel / Terminal 2 table / 6702 platform | VISIBLE |
| INF-008-U087 | RETAIN | NUMBER | upper right panel / Terminal 2 table / route 6703 | VISIBLE |
| INF-008-U088 | RETAIN | NUMBER | upper right panel / Terminal 2 table / 6703 platform | VISIBLE |
| INF-008-U089 | RETAIN | NUMBER | upper right panel / Terminal 2 table / route 6705 | VISIBLE |
| INF-008-U090 | RETAIN | NUMBER | upper right panel / Terminal 2 table / 6705 platform | VISIBLE |
| INF-008-U091 | RETAIN | BRAND | upper right panel / footer brand | VISIBLE |
| INF-008-U092 | RETAIN | BRAND | lower panel / header brand | VISIBLE |
| INF-008-U093 | RETAIN | NUMBER | lower panel / Myeongdong / City Hall / Namdaemun / route badge | VISIBLE |
| INF-008-U094 | RETAIN | NUMBER | lower panel / Myeongdong / City Hall / Namdaemun / platform cell 1 | VISIBLE |
| INF-008-U095 | RETAIN | NUMBER | lower panel / Myeongdong / City Hall / Namdaemun / platform cell 2 | VISIBLE |
| INF-008-U096 | RETAIN | NUMBER | lower panel / Myeongdong / City Hall / Namdaemun / platform cell 3 | VISIBLE |
| INF-008-U097 | RETAIN | NUMBER | lower panel / Seoul Station / Yongsan / route badge | VISIBLE |
| INF-008-U098 | RETAIN | NUMBER | lower panel / Seoul Station / Yongsan / platform cell 1 | VISIBLE |
| INF-008-U099 | RETAIN | NUMBER | lower panel / Seoul Station / Yongsan / platform cell 2 | VISIBLE |
| INF-008-U100 | RETAIN | NUMBER | lower panel / Seoul Station / Yongsan / platform cell 3 | VISIBLE |
| INF-008-U101 | RETAIN | NUMBER | lower panel / Gangnam / Yeoksam / Central City / route badge | VISIBLE |
| INF-008-U102 | RETAIN | NUMBER | lower panel / Gangnam / Yeoksam / Central City / platform cell 1 | VISIBLE |
| INF-008-U103 | RETAIN | NUMBER | lower panel / Gangnam / Yeoksam / Central City / platform cell 2 | VISIBLE |
| INF-008-U104 | RETAIN | NUMBER | lower panel / Gangnam / Yeoksam / Central City / platform cell 3 | VISIBLE |
| INF-008-U105 | RETAIN | NUMBER | lower panel / Jamsil / Songpa / Lotte World / route badge | VISIBLE |
| INF-008-U106 | RETAIN | NUMBER | lower panel / Jamsil / Songpa / Lotte World / platform cell 1 | VISIBLE |
| INF-008-U107 | RETAIN | NUMBER | lower panel / Jamsil / Songpa / Lotte World / platform cell 2 | VISIBLE |
| INF-008-U108 | RETAIN | NUMBER | lower panel / Jamsil / Songpa / Lotte World / platform cell 3 | VISIBLE |
| INF-008-U109 | RETAIN | NUMBER | lower panel / Hongdae / Hapjeong / Sinchon / route badge | VISIBLE |
| INF-008-U110 | RETAIN | NUMBER | lower panel / Hongdae / Hapjeong / Sinchon / platform cell 1 | VISIBLE |
| INF-008-U111 | RETAIN | NUMBER | lower panel / Hongdae / Hapjeong / Sinchon / platform cell 2 | VISIBLE |
| INF-008-U112 | RETAIN | NUMBER | lower panel / Hongdae / Hapjeong / Sinchon / platform cell 3 | VISIBLE |
| INF-008-U113 | RETAIN | NUMBER | lower panel / Hongdae / Hapjeong / Sinchon / platform cell 4 | VISIBLE |
| INF-008-U114 | RETAIN | NUMBER | lower panel / Dobong / Nowon / Uijeongbu / route badge | VISIBLE |
| INF-008-U115 | RETAIN | NUMBER | lower panel / Dobong / Nowon / Uijeongbu / platform cell 1 | VISIBLE |
| INF-008-U116 | RETAIN | NUMBER | lower panel / Dobong / Nowon / Uijeongbu / platform cell 2 | VISIBLE |
| INF-008-U117 | RETAIN | NUMBER | lower panel / Dobong / Nowon / Uijeongbu / platform cell 3 | VISIBLE |
| INF-008-U118 | RETAIN | NUMBER | lower panel / Dobong / Nowon / Uijeongbu / platform cell 4 | VISIBLE |
| INF-008-U119 | RETAIN | NUMBER | lower panel / Guri / Namyangju / route badge | VISIBLE |
| INF-008-U120 | RETAIN | NUMBER | lower panel / Guri / Namyangju / platform cell 1 | VISIBLE |
| INF-008-U121 | RETAIN | NUMBER | lower panel / Guri / Namyangju / platform cell 2 | VISIBLE |
| INF-008-U122 | RETAIN | NUMBER | lower panel / Guri / Namyangju / platform cell 3 | VISIBLE |
| INF-008-U123 | RETAIN | NUMBER | lower panel / Guri / Namyangju / platform cell 4 | VISIBLE |
| INF-008-U124 | RETAIN | NUMBER | lower panel / Hanam / Gwangju / Seongnam / route badge | VISIBLE |
| INF-008-U125 | RETAIN | NUMBER | lower panel / Hanam / Gwangju / Seongnam / platform cell 1 | VISIBLE |
| INF-008-U126 | RETAIN | NUMBER | lower panel / Hanam / Gwangju / Seongnam / platform cell 2 | VISIBLE |
| INF-008-U127 | RETAIN | NUMBER | lower panel / Hanam / Gwangju / Seongnam / platform cell 3 | VISIBLE |
| INF-008-U128 | RETAIN | NUMBER | lower panel / Hanam / Gwangju / Seongnam / platform cell 4 | VISIBLE |
| INF-008-U129 | RETAIN | NUMBER | lower panel / Bucheon / route badge | VISIBLE |
| INF-008-U130 | RETAIN | NUMBER | lower panel / Bucheon / platform cell 1 | VISIBLE |
| INF-008-U131 | RETAIN | NUMBER | lower panel / Bucheon / platform cell 2 | VISIBLE |
| INF-008-U132 | RETAIN | NUMBER | lower panel / Bucheon / platform cell 3 | VISIBLE |
| INF-008-U133 | RETAIN | NUMBER | lower panel / Bucheon / platform cell 4 | VISIBLE |
| INF-008-U134 | RETAIN | BRAND | lower panel / footer brand | VISIBLE |
| INF-008-U135 | RETAIN | NUMBER | lower panel / Korean reference date year | VISIBLE |
| INF-008-U136 | RETAIN | NUMBER | lower panel / Korean reference date month | VISIBLE |

Exact units: 136; TRANSLATE 35; RETAIN 101.

## E. PROTECTION

- Preserve native 1536×1024 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Keep upper-left Terminal 2 guide, upper-right Terminal 1/2 boarding tables and bottom Korean area decision panel in place.
- Terminal 1 table: 6001→8A-1, 6002→8A-2, 6003→8B-1, 6004→9A, 6701→10B, 6702→10A, 6703→11B, 6705→6A; Terminal 2: 6001→6A-1, 6002→6A-2, 6003→7A, 6004→8A, 6701→9B, 6702→10A, 6703→11A, 6705→6B.
- Korean destinations, bottom-panel arrows and recommendations remain as drawn; mismatched TA / missing terminal cells require review, not automatic correction.
- Preserve ₩17,000~18,000; ₩4,800; 43/56 min; 1F/B1F; stops6~11; taxi4D~6D; pickup4C,5C.

### Source-specific notes

- English editorial text, numeric/Latin-script route/platform labels and repeated Korea Inside branding are extracted. Korean-only headings/destination text are not translated; protect the existing Korean panels and their entity pairing.
- Historical asset filename says by-hotel-area while the English panel title is Terminal 2 Ground Transportation Guide; this mismatch is inherited, not a current SHA drift.
- Original lower panel includes Korean July 2024 reference date; retain 2024 and 7, no current-date replacement.
- Pickup heading case/glyphs remain uncertain; candidate excluded from exact table. Candidate readings are not counted as exact units: "Pick-UP" (upper left panel / map pickup label)

### REVIEW_REQUIRED

1. **upper left map / pickup label** — Upper-left pickup heading has irregular/case-ambiguous glyphs; candidate Pick-UP is not a verified exact unit. Obtain letter-level source confirmation without silently normalizing it.
2. **lower Korean decision panel / Jamsil column / terminal cell** — The source cell reads TA, not the T2 expected from context. Preserve TA as visible; do not silently correct the original.
3. **lower Korean decision panel / Myeongdong column** — Second platform row shows 9A without a terminal code; several other lower cells are blank. Do not invent terminal/platform values.
4. **Lower-panel illustrated hotel sign** — Tiny hotel sign lettering is not reliably legible; it is not reconstructed or counted as exact English.

Unconfirmed candidate readings (outside exact-unit totals; NOT Public Copy):

1. `Pick-UP` — upper left panel / map pickup label.

# INF-009 — Airport limousine bus step guide

**Asset status:** EXTRACTION COMPLETE

## A. IDENTITY / SOURCE

- INF ID: `INF-009`
- Source filename: `airport-bus-how-to-use.png`
- Source asset: `images/airport/airport-bus-how-to-use.png`
- Format: PNG
- Native dimensions: 1536 × 1024 px
- Current SHA-256: `dbd7c6e1cebad34b32fa29aba695eca172762e522f7848a32d9ee4389221c17b`
- Inherited audit SHA-256: `dbd7c6e1cebad34b32fa29aba695eca172762e522f7848a32d9ee4389221c17b`
- SHA comparison: MATCH
- Inherited audit dimensions: 1536 × 1024 px; comparison: MATCH
- Priority: P2
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `airport-bus.html`, image reference line 251
- Usage location: Airport bus journey/process instructions
- Existing wrapper: `<figure class="airport-bus-media__figure airport-bus-media__figure--wide">`
- English alt (reference only, not an embedded image unit): `Step-by-step guide to using the Incheon Airport limousine bus`
- English caption (reference only): `Airport limousine bus process from route check to hotel arrival`
- Thai page: `th/airport-bus.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `airport-bus-how-to-use-th.png`
- Planned Thai path: `images/airport/airport-bus-how-to-use-th.png`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/airport/airport-bus-how-to-use-es.png` — EXISTS
  - Sibling `es/airport-bus.html`; page EXISTS; image reference `../images/airport/airport-bus-how-to-use-es.png`
- JA precedent asset: `images/airport/airport-bus-how-to-use-ja.png` — EXISTS
  - Sibling `ja/airport-bus.html`; page EXISTS; image reference `../images/airport/airport-bus-how-to-use-ja.png`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-009-U001` — `Korea Inside`
2. `INF-009-U002` — `Airport Limousine Bus – How to Use (Step by Step)`
3. `INF-009-U003` — `1`
4. `INF-009-U004` — `Check Your Destination & Bus`
5. `INF-009-U005` — `• Find your final stop (hotel or nearby stop)`
6. `INF-009-U006` — `• Check bus number, route, and boarding location (T1 or T2)`
7. `INF-009-U007` — `• Use official website or airport signs`
8. `INF-009-U008` — `2`
9. `INF-009-U009` — `Go to the Bus Stop`
10. `INF-009-U010` — `BUS TICKET`
11. `INF-009-U011` — `• Follow “BUS / 리무진버스” signs`
12. `INF-009-U012` — `• Find your bus stop number`
13. `INF-009-U013` — `• T1: 1F 4A~6A, 8A~11A`
14. `INF-009-U014` — `• T2: B1 6~11`
15. `INF-009-U015` — `3`
16. `INF-009-U016` — `Buy Your Ticket`
17. `INF-009-U017` — `BUS TICKET`
18. `INF-009-U018` — `• Buy a ticket at the ticket booth or kiosk`
19. `INF-009-U019` — `• You can also pay with T-money card on the bus`
20. `INF-009-U020` — `• Keep your ticket until you get off`
21. `INF-009-U021` — `4`
22. `INF-009-U022` — `Board the Bus`
23. `INF-009-U023` — `• Check the bus number on the front of the bus`
24. `INF-009-U024` — `• Queue in order and board`
25. `INF-009-U025` — `• Put luggage in the storage compartment`
26. `INF-009-U026` — `5`
27. `INF-009-U027` — `Enjoy the Ride`
28. `INF-009-U028` — `• Relax and enjoy the ride`
29. `INF-009-U029` — `• Stops may vary depending on route`
30. `INF-009-U030` — `• Traffic conditions may affect arrival time`
31. `INF-009-U031` — `6`
32. `INF-009-U032` — `Get Off at Your Stop`
33. `INF-009-U033` — `6002`
34. `INF-009-U034` — `• Press the stop button before your stop`
35. `INF-009-U035` — `• Check your belongings before getting off`
36. `INF-009-U036` — `• Take your luggage`
37. `INF-009-U037` — `7`
38. `INF-009-U038` — `Walk to Your Accommodation`
39. `INF-009-U039` — `HOTEL`
40. `INF-009-U040` — `• Check the direction to your accommodation`
41. `INF-009-U041` — `• Most hotels are within 5–10 minutes walk from the bus stop`
42. `INF-009-U042` — `• Use maps app if needed`
43. `INF-009-U043` — `8`
44. `INF-009-U044` — `Arrive Safely!`
45. `INF-009-U045` — `• You’ve made it!`
46. `INF-009-U046` — `• Check in and enjoy your trip in Korea`
47. `INF-009-U047` — `• Thank you for using airport limousine bus!`
48. `INF-009-U048` — `TIP & INFO`
49. `INF-009-U049` — `Operating Hours`
50. `INF-009-U050` — `04:20 ~ 23:30 (Varies by route)`
51. `INF-009-U051` — `Luggage`
52. `INF-009-U052` — `1 piece of luggage per person is free (Additional fee may apply)`
53. `INF-009-U053` — `Payment`
54. `INF-009-U054` — `Cash, Credit Card, T-money, and other transportation cards`
55. `INF-009-U055` — `Traffic Notice`
56. `INF-009-U056` — `Allow extra time during rush hour and holidays`
57. `INF-009-U057` — `Children`
58. `INF-009-U058` — `Children under 6 years old ride free (no separate seat)`
59. `INF-009-U059` — `Where to Check More Information`
60. `INF-009-U060` — `Airport Limousine Official Website`
61. `INF-009-U061` — `https://airportlimousine.co.kr/en/`
62. `INF-009-U062` — `Incheon Airport Official Website`
63. `INF-009-U063` — `https://www.airport.kr/ap/en/`
64. `INF-009-U064` — `Information Desks`
65. `INF-009-U065` — `Located in Arrivals Hall (T1 1F, T2 1F)`
66. `INF-009-U066` — `Please check the latest information before your trip.`
67. `INF-009-U067` — `Bus routes and stop locations may change.`
68. `INF-009-U068` — `Korea Inside`
69. `INF-009-U069` — `Updated: July 2024`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-009-U001 | RETAIN | BRAND | header / brand | VISIBLE |
| INF-009-U002 | TRANSLATE | — | header / title | VISIBLE |
| INF-009-U003 | RETAIN | NUMBER | steps / card 1 / badge | VISIBLE |
| INF-009-U004 | TRANSLATE | SYMBOL | steps / card 1 / heading | VISIBLE |
| INF-009-U005 | TRANSLATE | SYMBOL | steps / card 1 / bullet 1 | VISIBLE |
| INF-009-U006 | TRANSLATE | SYMBOL, NUMBER | steps / card 1 / bullet 2 | VISIBLE |
| INF-009-U007 | TRANSLATE | SYMBOL | steps / card 1 / bullet 3 | VISIBLE |
| INF-009-U008 | RETAIN | NUMBER | steps / card 2 / badge | VISIBLE |
| INF-009-U009 | TRANSLATE | — | steps / card 2 / heading | VISIBLE |
| INF-009-U010 | TRANSLATE | — | steps / card 2 / illustrated sign | VISIBLE |
| INF-009-U011 | TRANSLATE | SYMBOL | steps / card 2 / bullet 1 | VISIBLE |
| INF-009-U012 | TRANSLATE | SYMBOL | steps / card 2 / bullet 2 | VISIBLE |
| INF-009-U013 | TRANSLATE | SYMBOL, NUMBER | steps / card 2 / bullet 3 | VISIBLE |
| INF-009-U014 | TRANSLATE | SYMBOL, NUMBER | steps / card 2 / bullet 4 | VISIBLE |
| INF-009-U015 | RETAIN | NUMBER | steps / card 3 / badge | VISIBLE |
| INF-009-U016 | TRANSLATE | — | steps / card 3 / heading | VISIBLE |
| INF-009-U017 | TRANSLATE | — | steps / card 3 / illustrated sign | VISIBLE |
| INF-009-U018 | TRANSLATE | SYMBOL | steps / card 3 / bullet 1 | VISIBLE |
| INF-009-U019 | TRANSLATE | SYMBOL, BRAND | steps / card 3 / bullet 2 | VISIBLE |
| INF-009-U020 | TRANSLATE | SYMBOL | steps / card 3 / bullet 3 | VISIBLE |
| INF-009-U021 | RETAIN | NUMBER | steps / card 4 / badge | VISIBLE |
| INF-009-U022 | TRANSLATE | — | steps / card 4 / heading | VISIBLE |
| INF-009-U023 | TRANSLATE | SYMBOL | steps / card 4 / bullet 1 | VISIBLE |
| INF-009-U024 | TRANSLATE | SYMBOL | steps / card 4 / bullet 2 | VISIBLE |
| INF-009-U025 | TRANSLATE | SYMBOL | steps / card 4 / bullet 3 | VISIBLE |
| INF-009-U026 | RETAIN | NUMBER | steps / card 5 / badge | VISIBLE |
| INF-009-U027 | TRANSLATE | — | steps / card 5 / heading | VISIBLE |
| INF-009-U028 | TRANSLATE | SYMBOL | steps / card 5 / bullet 1 | VISIBLE |
| INF-009-U029 | TRANSLATE | SYMBOL | steps / card 5 / bullet 2 | VISIBLE |
| INF-009-U030 | TRANSLATE | SYMBOL | steps / card 5 / bullet 3 | VISIBLE |
| INF-009-U031 | RETAIN | NUMBER | steps / card 6 / badge | VISIBLE |
| INF-009-U032 | TRANSLATE | — | steps / card 6 / heading | VISIBLE |
| INF-009-U033 | RETAIN | NUMBER | steps / card 6 / illustrated bus route | VISIBLE |
| INF-009-U034 | TRANSLATE | SYMBOL | steps / card 6 / bullet 1 | VISIBLE |
| INF-009-U035 | TRANSLATE | SYMBOL | steps / card 6 / bullet 2 | VISIBLE |
| INF-009-U036 | TRANSLATE | SYMBOL | steps / card 6 / bullet 3 | VISIBLE |
| INF-009-U037 | RETAIN | NUMBER | steps / card 7 / badge | VISIBLE |
| INF-009-U038 | TRANSLATE | — | steps / card 7 / heading | VISIBLE |
| INF-009-U039 | TRANSLATE | — | steps / card 7 / illustrated sign | VISIBLE |
| INF-009-U040 | TRANSLATE | SYMBOL | steps / card 7 / bullet 1 | VISIBLE |
| INF-009-U041 | TRANSLATE | SYMBOL, NUMBER | steps / card 7 / bullet 2 | VISIBLE |
| INF-009-U042 | TRANSLATE | SYMBOL | steps / card 7 / bullet 3 | VISIBLE |
| INF-009-U043 | RETAIN | NUMBER | steps / card 8 / badge | VISIBLE |
| INF-009-U044 | TRANSLATE | — | steps / card 8 / heading | VISIBLE |
| INF-009-U045 | TRANSLATE | SYMBOL | steps / card 8 / bullet 1 | VISIBLE |
| INF-009-U046 | TRANSLATE | SYMBOL | steps / card 8 / bullet 2 | VISIBLE |
| INF-009-U047 | TRANSLATE | SYMBOL | steps / card 8 / bullet 3 | VISIBLE |
| INF-009-U048 | TRANSLATE | SYMBOL | lower left / section heading | VISIBLE |
| INF-009-U049 | TRANSLATE | — | lower left / item 1 heading | VISIBLE |
| INF-009-U050 | TRANSLATE | NUMBER, SYMBOL | lower left / item 1 details | VISIBLE |
| INF-009-U051 | TRANSLATE | — | lower left / item 2 heading | VISIBLE |
| INF-009-U052 | TRANSLATE | NUMBER | lower left / item 2 details | VISIBLE |
| INF-009-U053 | TRANSLATE | — | lower left / item 3 heading | VISIBLE |
| INF-009-U054 | TRANSLATE | BRAND | lower left / item 3 details | VISIBLE |
| INF-009-U055 | TRANSLATE | — | lower left / item 4 heading | VISIBLE |
| INF-009-U056 | TRANSLATE | — | lower left / item 4 details | VISIBLE |
| INF-009-U057 | TRANSLATE | — | lower left / item 5 heading | VISIBLE |
| INF-009-U058 | TRANSLATE | NUMBER | lower left / item 5 details | VISIBLE |
| INF-009-U059 | TRANSLATE | — | lower right / heading | VISIBLE |
| INF-009-U060 | TRANSLATE | PROPER_NOUN | lower right / source 1 label | VISIBLE |
| INF-009-U061 | RETAIN | PROPER_NOUN | lower right / source 1 URL | VISIBLE |
| INF-009-U062 | TRANSLATE | PROPER_NOUN | lower right / source 2 label | VISIBLE |
| INF-009-U063 | RETAIN | PROPER_NOUN | lower right / source 2 URL | VISIBLE |
| INF-009-U064 | TRANSLATE | — | lower right / source 3 label | VISIBLE |
| INF-009-U065 | TRANSLATE | NUMBER | lower right / source 3 details | VISIBLE |
| INF-009-U066 | TRANSLATE | — | footer / warning line 1 | VISIBLE |
| INF-009-U067 | TRANSLATE | — | footer / warning line 2 | VISIBLE |
| INF-009-U068 | RETAIN | BRAND | footer / brand | VISIBLE |
| INF-009-U069 | TRANSLATE | NUMBER | footer / update marker | VISIBLE |

Exact units: 69; TRANSLATE 56; RETAIN 13.

## E. PROTECTION

- Preserve native 1536×1024 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Maintain eight-step order and all connecting arrows; duplicate BUS TICKET signs appear in cards2 and3.
- Preserve T1:1F4A~6A,8A~11A; T2:B1 6~11; route6002;5–10minutes;04:20~23:30;1piece;childrenunder6/no separate seat;July2024;both source URLs.
- Keep warnings and qualifiers on route-specific hours, stops, traffic, luggage fees and seat entitlement.

### Source-specific notes

- Korean subtitle and Korean portion of the mixed BUS / 리무진버스 sign are retained source content, not authored Thai.
- Original July 2024 update marker is transcribed as shown; no freshness claim or update is made.

### REVIEW_REQUIRED

NONE.

# INF-010 — AREX Express vs All-Stop route map

**Asset status:** EXTRACTION COMPLETE

## A. IDENTITY / SOURCE

- INF ID: `INF-010`
- Source filename: `arex-express-vs-all-stop-route-map.png`
- Source asset: `images/airport/arex/arex-express-vs-all-stop-route-map.png`
- Format: PNG
- Native dimensions: 1448 × 1086 px
- Current SHA-256: `101e302778f7540d2eb7a484a6aaa63fad68138ced4bb7e88d87223b46de177c`
- Inherited audit SHA-256: `101e302778f7540d2eb7a484a6aaa63fad68138ced4bb7e88d87223b46de177c`
- SHA comparison: MATCH
- Inherited audit dimensions: 1448 × 1086 px; comparison: MATCH
- Priority: P1
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `arex.html`, image reference line 210
- Usage location: Main AREX Stations and Stops / #stations
- Existing wrapper: `<figure class="arex-infographic">`
- English alt (reference only, not an embedded image unit): `AREX Express and All-Stop route map showing Incheon Airport terminals, major stations and Seoul Station`
- English caption (reference only): `AREX Express and All-Stop routes from Incheon Airport to Seoul`
- Thai page: `th/arex.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `arex-express-vs-all-stop-route-map-th.png`
- Planned Thai path: `images/airport/arex/arex-express-vs-all-stop-route-map-th.png`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/airport/arex/arex-express-vs-all-stop-route-map-es.png` — EXISTS
  - Sibling `es/arex.html`; page EXISTS; image reference `../images/airport/arex/arex-express-vs-all-stop-route-map-es.png`
- JA precedent asset: `images/airport/arex/arex-express-vs-all-stop-route-map-ja.png` — EXISTS
  - Sibling `ja/arex.html`; page EXISTS; image reference `../images/airport/arex/arex-express-vs-all-stop-route-map-ja.png`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-010-U001` — `Korea Inside`
2. `INF-010-U002` — `AREX Express vs All-Stop Route Map`
3. `INF-010-U003` — `Incheon Airport to Seoul`
4. `INF-010-U004` — `AREX Express`
5. `INF-010-U005` — `Faster, reserved-seat train`
6. `INF-010-U006` — `Terminal 2`
7. `INF-010-U007` — `Incheon Airport`
8. `INF-010-U008` — `Terminal 1`
9. `INF-010-U009` — `Incheon Airport`
10. `INF-010-U010` — `Seoul Station`
11. `INF-010-U011` — `FARE (ONE WAY)`
12. `INF-010-U012` — `Adult`
13. `INF-010-U013` — `₩13,000`
14. `INF-010-U014` — `Child`
15. `INF-010-U015` — `₩9,500`
16. `INF-010-U016` — `TO SEOUL STATION`
17. `INF-010-U017` — `51 min from Terminal 2`
18. `INF-010-U018` — `43 min from Terminal 1`
19. `INF-010-U019` — `Separate Express ticket required`
20. `INF-010-U020` — `AREX All-Stop`
21. `INF-010-U021` — `Local train with more stops`
22. `INF-010-U022` — `Terminal 2`
23. `INF-010-U023` — `Incheon Airport`
24. `INF-010-U024` — `Terminal 1`
25. `INF-010-U025` — `Incheon Airport`
26. `INF-010-U026` — `Gimpo Airport`
27. `INF-010-U027` — `Hongik University`
28. `INF-010-U028` — `Gongdeok`
29. `INF-010-U029` — `Seoul Station`
30. `INF-010-U030` — `FARE (ONE WAY)`
31. `INF-010-U031` — `T2 → Seoul Station`
32. `INF-010-U032` — `₩5,350`
33. `INF-010-U033` — `T1 → Seoul Station`
34. `INF-010-U034` — `₩4,750`
35. `INF-010-U035` — `TO SEOUL STATION`
36. `INF-010-U036` — `66 min from Terminal 2`
37. `INF-010-U037` — `59 min from Terminal 1`
38. `INF-010-U038` — `T`
39. `INF-010-U039` — `money`
40. `INF-010-U040` — `Use T-money or a single-use subway ticket`
41. `INF-010-U041` — `Check the train type, platform and departure display before boarding.`
42. `INF-010-U042` — `Korea Inside`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-010-U001 | RETAIN | BRAND | top brand | VISIBLE |
| INF-010-U002 | TRANSLATE | BRAND | heading | VISIBLE |
| INF-010-U003 | TRANSLATE | PROPER_NOUN | subtitle | VISIBLE |
| INF-010-U004 | RETAIN | BRAND | left panel heading | VISIBLE |
| INF-010-U005 | TRANSLATE | — | left panel subtitle | VISIBLE |
| INF-010-U006 | RETAIN | PROPER_NOUN, NUMBER | left route 1 | VISIBLE |
| INF-010-U007 | RETAIN | PROPER_NOUN | left route 1 sublabel | VISIBLE |
| INF-010-U008 | RETAIN | PROPER_NOUN, NUMBER | left route 2 | VISIBLE |
| INF-010-U009 | RETAIN | PROPER_NOUN | left route 2 sublabel | VISIBLE |
| INF-010-U010 | RETAIN | PROPER_NOUN | left route 3 | VISIBLE |
| INF-010-U011 | TRANSLATE | — | left fare | VISIBLE |
| INF-010-U012 | TRANSLATE | — | left adult label | VISIBLE |
| INF-010-U013 | RETAIN | NUMBER, SYMBOL | left adult fare | VISIBLE |
| INF-010-U014 | TRANSLATE | — | left child label | VISIBLE |
| INF-010-U015 | RETAIN | NUMBER, SYMBOL | left child fare | VISIBLE |
| INF-010-U016 | TRANSLATE | PROPER_NOUN | left time label | VISIBLE |
| INF-010-U017 | TRANSLATE | NUMBER, PROPER_NOUN | left time T2 | VISIBLE |
| INF-010-U018 | TRANSLATE | NUMBER, PROPER_NOUN | left time T1 | VISIBLE |
| INF-010-U019 | TRANSLATE | BRAND | left footer | VISIBLE |
| INF-010-U020 | RETAIN | BRAND | right panel heading | VISIBLE |
| INF-010-U021 | TRANSLATE | — | right panel subtitle | VISIBLE |
| INF-010-U022 | RETAIN | PROPER_NOUN, NUMBER | right route 1 | VISIBLE |
| INF-010-U023 | RETAIN | PROPER_NOUN | right route 1 sublabel | VISIBLE |
| INF-010-U024 | RETAIN | PROPER_NOUN, NUMBER | right route 2 | VISIBLE |
| INF-010-U025 | RETAIN | PROPER_NOUN | right route 2 sublabel | VISIBLE |
| INF-010-U026 | RETAIN | PROPER_NOUN | right route 3 | VISIBLE |
| INF-010-U027 | RETAIN | PROPER_NOUN | right route 4 | VISIBLE |
| INF-010-U028 | RETAIN | PROPER_NOUN | right route 5 | VISIBLE |
| INF-010-U029 | RETAIN | PROPER_NOUN | right route 6 | VISIBLE |
| INF-010-U030 | TRANSLATE | — | right fare | VISIBLE |
| INF-010-U031 | RETAIN | PROPER_NOUN, NUMBER, SYMBOL | right fare T2 label | VISIBLE |
| INF-010-U032 | RETAIN | NUMBER, SYMBOL | right fare T2 | VISIBLE |
| INF-010-U033 | RETAIN | PROPER_NOUN, NUMBER, SYMBOL | right fare T1 label | VISIBLE |
| INF-010-U034 | RETAIN | NUMBER, SYMBOL | right fare T1 | VISIBLE |
| INF-010-U035 | TRANSLATE | PROPER_NOUN | right time label | VISIBLE |
| INF-010-U036 | TRANSLATE | NUMBER, PROPER_NOUN | right time T2 | VISIBLE |
| INF-010-U037 | TRANSLATE | NUMBER, PROPER_NOUN | right time T1 | VISIBLE |
| INF-010-U038 | RETAIN | BRAND | right footer logo top | VISIBLE |
| INF-010-U039 | RETAIN | BRAND | right footer logo bottom | VISIBLE |
| INF-010-U040 | TRANSLATE | BRAND | right footer | VISIBLE |
| INF-010-U041 | TRANSLATE | — | bottom warning | VISIBLE |
| INF-010-U042 | RETAIN | BRAND | footer brand | VISIBLE |

Exact units: 42; TRANSLATE 17; RETAIN 25.

## E. PROTECTION

- Preserve native 1448×1086 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Express: Terminal 2 → Terminal 1 → Seoul Station; All-Stop additionally includes Gimpo Airport, Hongik University and Gongdeok.
- Adult ₩13,000; child ₩9,500; All-Stop T2 ₩5,350 / T1 ₩4,750.
- Express 51/43 min; All-Stop 66/59 min; separate Express ticket versus T-money/single-use ticket.

### Source-specific notes

- Read panels left-to-right; each route top-to-bottom/left-to-right. The illustrated T-money logo is split into T and money, with no printed hyphen between those glyphs.

### REVIEW_REQUIRED

NONE.

# INF-011 — AREX hero comparison

**Asset status:** EXTRACTION COMPLETE

## A. IDENTITY / SOURCE

- INF ID: `INF-011`
- Source filename: `arex-hero-express-vs-all-stop.png`
- Source asset: `images/airport/arex/arex-hero-express-vs-all-stop.png`
- Format: PNG
- Native dimensions: 1672 × 941 px
- Current SHA-256: `9e2aa0864a69d90ab4c72d20d62251a1fa4e20b89513acfc63b37e7ed4ff13a8`
- Inherited audit SHA-256: `9e2aa0864a69d90ab4c72d20d62251a1fa4e20b89513acfc63b37e7ed4ff13a8`
- SHA comparison: MATCH
- Inherited audit dimensions: 1672 × 941 px; comparison: MATCH
- Priority: P1
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `arex.html`, image reference line 90
- Usage location: AREX hero / .arex-hero
- Existing wrapper: `<figure class="arex-hero-visual">`
- English alt (reference only, not an embedded image unit): `AREX train traveling from Incheon Airport toward Seoul with Express and All-Stop route choices`
- English caption (reference only): NONE
- Thai page: `th/arex.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `arex-hero-express-vs-all-stop-th.png`
- Planned Thai path: `images/airport/arex/arex-hero-express-vs-all-stop-th.png`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/airport/arex/arex-hero-express-vs-all-stop-es.png` — EXISTS
  - Sibling `es/arex.html`; page EXISTS; image reference `../images/airport/arex/arex-hero-express-vs-all-stop-es.png`
- JA precedent asset: `images/airport/arex/arex-hero-express-vs-all-stop-ja.png` — EXISTS
  - Sibling `ja/arex.html`; page EXISTS; image reference `../images/airport/arex/arex-hero-express-vs-all-stop-ja.png`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-011-U001` — `AREX Express or All-Stop?`
2. `INF-011-U002` — `Choose by destination, luggage and transfer.`
3. `INF-011-U003` — `Express`
4. `INF-011-U004` — `Terminal 2`
5. `INF-011-U005` — `Terminal 1`
6. `INF-011-U006` — `Seoul Station`
7. `INF-011-U007` — `All-Stop`
8. `INF-011-U008` — `Terminal 2`
9. `INF-011-U009` — `Terminal 1`
10. `INF-011-U010` — `Gimpo Airport`
11. `INF-011-U011` — `Hongik University`
12. `INF-011-U012` — `Gongdeok`
13. `INF-011-U013` — `Seoul Station`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-011-U001 | TRANSLATE | BRAND | heading | VISIBLE |
| INF-011-U002 | TRANSLATE | — | subtitle | VISIBLE |
| INF-011-U003 | RETAIN | BRAND | upper route badge | VISIBLE |
| INF-011-U004 | RETAIN | PROPER_NOUN, NUMBER | upper route station | VISIBLE |
| INF-011-U005 | RETAIN | PROPER_NOUN, NUMBER | upper route station | VISIBLE |
| INF-011-U006 | RETAIN | PROPER_NOUN | upper route station | VISIBLE |
| INF-011-U007 | RETAIN | BRAND | lower route badge | VISIBLE |
| INF-011-U008 | RETAIN | PROPER_NOUN, NUMBER | lower route station | VISIBLE |
| INF-011-U009 | RETAIN | PROPER_NOUN, NUMBER | lower route station | VISIBLE |
| INF-011-U010 | RETAIN | PROPER_NOUN | lower route station | VISIBLE |
| INF-011-U011 | RETAIN | PROPER_NOUN | lower route station | VISIBLE |
| INF-011-U012 | RETAIN | PROPER_NOUN | lower route station | VISIBLE |
| INF-011-U013 | RETAIN | PROPER_NOUN | lower route station | VISIBLE |

Exact units: 13; TRANSLATE 2; RETAIN 11.

## E. PROTECTION

- Preserve native 1672×941 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Express direct-route diagram and All-Stop intermediate-station order; luggage/destination/transfer decision logic.

### Source-specific notes

- Station labels are retained; headline/subtitle are editorial TRANSLATE. No Korea Inside wordmark is visible.

### REVIEW_REQUIRED

NONE.

# INF-012 — AREX station by destination

**Asset status:** REVIEW_REQUIRED

## A. IDENTITY / SOURCE

- INF ID: `INF-012`
- Source filename: `arex-station-by-destination.png`
- Source asset: `images/airport/arex/arex-station-by-destination.png`
- Format: PNG
- Native dimensions: 768 × 1024 px
- Current SHA-256: `79df52d85319a62198e7e3dfbbb4d9c9bd829484754c5733abc6fa3bdde82606`
- Inherited audit SHA-256: `79df52d85319a62198e7e3dfbbb4d9c9bd829484754c5733abc6fa3bdde82606`
- SHA comparison: MATCH
- Inherited audit dimensions: 768 × 1024 px; comparison: MATCH
- Priority: P1
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `arex.html`, image reference line 249
- Usage location: Where Are You Going After AREX? / #hotel-area
- Existing wrapper: `<figure class="arex-infographic">`
- English alt (reference only, not an embedded image unit): `Guide to choosing an AREX train and station for Seoul Station, Hongdae, Gongdeok, Myeongdong, Gangnam and other destinations`
- English caption (reference only): `AREX train and station options for major Seoul destinations`
- Thai page: `th/arex.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `arex-station-by-destination-th.png`
- Planned Thai path: `images/airport/arex/arex-station-by-destination-th.png`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/airport/arex/arex-station-by-destination-es.png` — EXISTS
  - Sibling `es/arex.html`; page EXISTS; image reference `../images/airport/arex/arex-station-by-destination-es.png`
- JA precedent asset: `images/airport/arex/arex-station-by-destination-ja.png` — EXISTS
  - Sibling `ja/arex.html`; page EXISTS; image reference `../images/airport/arex/arex-station-by-destination-ja.png`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-012-U001` — `Which AREX Station Should You Use?`
2. `INF-012-U002` — `Find the best AREX train and station for your destination in Seoul`
3. `INF-012-U003` — `Korea`
4. `INF-012-U004` — `Insid`
5. `INF-012-U005` — `Express`
6. `INF-012-U006` — `(Direct to Seoul Station)`
7. `INF-012-U007` — `All-Stop`
8. `INF-012-U008` — `(Every Station)`
9. `INF-012-U009` — `Transfer Needed`
10. `INF-012-U010` — `Better with Large Luggage`
11. `INF-012-U011` — `(Bus/Taxi Recommended)`
12. `INF-012-U012` — `Best Choice`
13. `INF-012-U013` — `DESTINATION`
14. `INF-012-U014` — `BEST AREX TRAIN`
15. `INF-012-U015` — `GET OFF AT`
16. `INF-012-U016` — `TRANSFER & CONNECTIONS`
17. `INF-012-U017` — `BEST FOR`
18. `INF-012-U018` — `OUR RECOMMENDATION`
19. `INF-012-U019` — `Seoul Station`
20. `INF-012-U020` — `KTX, City Center`
21. `INF-012-U021` — `EXPRESS`
22. `INF-012-U022` — `(or All-Stop)`
23. `INF-012-U023` — `Seoul Station`
24. `INF-012-U024` — `(Last Stop)`
25. `INF-012-U025` — `KTX, Line 1, 4`
26. `INF-012-U026` — `Airport Bus, Taxi`
27. `INF-012-U027` — `KTX travelers, business travelers, first-time visitors`
28. `INF-012-U028` — `BEST CHOICE`
29. `INF-012-U029` — `Fastest and most convenient`
30. `INF-012-U030` — `Hongdae`
31. `INF-012-U031` — `(Hongik Univ.)`
32. `INF-012-U032` — `Shopping, Youth`
33. `INF-012-U033` — `ALL-STOP`
34. `INF-012-U034` — `Hongik University`
35. `INF-012-U035` — `(6th Stop)`
36. `INF-012-U036` — `Line 2`
37. `INF-012-U037` — `(2 stops to Gangnam)`
38. `INF-012-U038` — `Young travelers, students, nightlife`
39. `INF-012-U039` — `BEST CHOICE`
40. `INF-012-U040` — `Direct, easy, and affordable`
41. `INF-012-U041` — `Gongdeok`
42. `INF-012-U042` — `Mapo, Yeouido, Gov’t Offices`
43. `INF-012-U043` — `ALL-STOP`
44. `INF-012-U044` — `Gongdeok`
45. `INF-012-U045` — `(8th Stop)`
46. `INF-012-U046` — `Line 5, 6`
47. `INF-012-U047` — `Gyeongui-Jungang Line`
48. `INF-012-U048` — `Airport Bus`
49. `INF-012-U049` — `Business travelers, Mapo/Yeouido area, City Hall`
50. `INF-012-U050` — `BEST CHOICE`
51. `INF-012-U051` — `Very convenient location`
52. `INF-012-U052` — `Myeongdong`
53. `INF-012-U053` — `Shopping, Sightseeing`
54. `INF-012-U054` — `EXPRESS`
55. `INF-012-U055` — `(or All-Stop)`
56. `INF-012-U056` — `Seoul Station`
57. `INF-012-U057` — `(Last Stop)`
58. `INF-012-U058` — `→ Walk or take Subway (Line 4)`
59. `INF-012-U059` — `Line 4`
60. `INF-012-U060` — `(1 stop)`
61. `INF-012-U061` — `Sightseeing, shopping, first-timers`
62. `INF-012-U062` — `BOTH OK`
63. `INF-012-U063` — `Express is faster overall`
64. `INF-012-U064` — `Gangnam`
65. `INF-012-U065` — `Business, Luxury Area`
66. `INF-012-U066` — `ALL-STOP`
67. `INF-012-U067` — `Hongik University`
68. `INF-012-U068` — `(6th Stop)`
69. `INF-012-U069` — `Line 2`
70. `INF-012-U070` — `(Direct to Gangnam)`
71. `INF-012-U071` — `Shopping, business, clinics, COEX`
72. `INF-012-U072` — `BOTH OK`
73. `INF-012-U073` — `Short transfer to Line 2`
74. `INF-012-U074` — `Jamsil`
75. `INF-012-U075` — `Lotte World, Sports`
76. `INF-012-U076` — `ALL-STOP`
77. `INF-012-U077` — `Hongik University`
78. `INF-012-U078` — `(6th Stop)`
79. `INF-012-U079` — `Line 2`
80. `INF-012-U080` — `(To Jamsil Station)`
81. `INF-012-U081` — `Families, Lotte World, events`
82. `INF-012-U082` — `BOTH OK`
83. `INF-012-U083` — `Transfer to Line 2 at Hongdae`
84. `INF-012-U084` — `Gimpo Airport`
85. `INF-012-U085` — `Domestic Flights`
86. `INF-012-U086` — `ALL-STOP`
87. `INF-012-U087` — `Gimpo Airport`
88. `INF-012-U088` — `(Last Stop)`
89. `INF-012-U089` — `Walk`
90. `INF-012-U090` — `Domestic travelers, connecting flights`
91. `INF-012-U091` — `BEST CHOICE`
92. `INF-012-U092` — `Direct and convenient`
93. `INF-012-U093` — `EXPRESS vs ALL-STOP AT A GLANCE`
94. `INF-012-U094` — `EXPRESS`
95. `INF-012-U095` — `(Direct)`
96. `INF-012-U096` — `Incheon Airport T1 → Seoul Station 43 min`
97. `INF-012-U097` — `Incheon Airport T2 → Seoul Station ≈ 51 min`
98. `INF-012-U098` — `• Fastest to Seoul Station`
99. `INF-012-U099` — `• Reserved seats`
100. `INF-012-U100` — `• Slightly higher fare`
101. `INF-012-U101` — `₩13,000`
102. `INF-012-U102` — `ALL-STOP`
103. `INF-012-U103` — `(Every Station)`
104. `INF-012-U104` — `T1 → Seoul Station 59 min`
105. `INF-012-U105` — `T2 → Seoul Station ≈ 66 min`
106. `INF-012-U106` — `• Stops at every station`
107. `INF-012-U107` — `• Lower fare`
108. `INF-012-U108` — `• single-journey ticket`
109. `INF-012-U109` — `₩4,750 (T1)`
110. `INF-012-U110` — `₩5,350 (T2)`
111. `INF-012-U111` — `TIPS`
112. `INF-012-U112` — `Both trains use the same station at T1 and T2. Check the train type before boarding.`
113. `INF-012-U113` — `If your hotel is near a station on Line 2, Hongdae is a good transfer point.`
114. `INF-012-U114` — `Traveling with large luggage or a group? Compare airport bus or taxi.`
115. `INF-012-U115` — `Travel times and fares are approximate and may change. Check the latest schedule on the AREX official website.`
116. `INF-012-U116` — `Korea Inside`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-012-U001 | TRANSLATE | BRAND | heading | VISIBLE |
| INF-012-U002 | TRANSLATE | BRAND, PROPER_NOUN | subtitle | VISIBLE |
| INF-012-U003 | RETAIN | BRAND | top-right clipped brand upper | VISIBLE_FRAGMENT |
| INF-012-U004 | RETAIN | BRAND | top-right clipped brand lower | VISIBLE_FRAGMENT |
| INF-012-U005 | RETAIN | BRAND | legend | VISIBLE |
| INF-012-U006 | TRANSLATE | PROPER_NOUN | legend | VISIBLE |
| INF-012-U007 | RETAIN | BRAND | legend | VISIBLE |
| INF-012-U008 | TRANSLATE | — | legend | VISIBLE |
| INF-012-U009 | TRANSLATE | — | legend | VISIBLE |
| INF-012-U010 | TRANSLATE | — | legend | VISIBLE |
| INF-012-U011 | TRANSLATE | RECOMMENDATION | legend | VISIBLE |
| INF-012-U012 | TRANSLATE | RECOMMENDATION | legend | VISIBLE |
| INF-012-U013 | TRANSLATE | — | table header | VISIBLE |
| INF-012-U014 | TRANSLATE | BRAND | table header | VISIBLE |
| INF-012-U015 | TRANSLATE | — | table header | VISIBLE |
| INF-012-U016 | TRANSLATE | SYMBOL | table header | VISIBLE |
| INF-012-U017 | TRANSLATE | — | table header | VISIBLE |
| INF-012-U018 | TRANSLATE | RECOMMENDATION | table header | VISIBLE |
| INF-012-U019 | RETAIN | PROPER_NOUN | table row 1, label 1 | VISIBLE |
| INF-012-U020 | TRANSLATE | BRAND | table row 1, label 2 | VISIBLE |
| INF-012-U021 | RETAIN | BRAND | table row 1, label 3 | VISIBLE |
| INF-012-U022 | TRANSLATE | BRAND | table row 1, label 4 | VISIBLE |
| INF-012-U023 | RETAIN | PROPER_NOUN | table row 1, label 5 | VISIBLE |
| INF-012-U024 | TRANSLATE | — | table row 1, label 6 | VISIBLE |
| INF-012-U025 | TRANSLATE | BRAND, NUMBER | table row 1, label 7 | VISIBLE |
| INF-012-U026 | TRANSLATE | — | table row 1, label 8 | VISIBLE |
| INF-012-U027 | TRANSLATE | BRAND | table row 1, label 9 | VISIBLE |
| INF-012-U028 | TRANSLATE | RECOMMENDATION | table row 1, label 10 | VISIBLE |
| INF-012-U029 | TRANSLATE | RECOMMENDATION | table row 1, label 11 | VISIBLE |
| INF-012-U030 | RETAIN | PROPER_NOUN | table row 2, label 1 | VISIBLE |
| INF-012-U031 | RETAIN | PROPER_NOUN | table row 2, label 2 | VISIBLE |
| INF-012-U032 | TRANSLATE | — | table row 2, label 3 | VISIBLE |
| INF-012-U033 | RETAIN | BRAND | table row 2, label 4 | VISIBLE |
| INF-012-U034 | RETAIN | PROPER_NOUN | table row 2, label 5 | VISIBLE |
| INF-012-U035 | TRANSLATE | NUMBER | table row 2, label 6 | VISIBLE |
| INF-012-U036 | TRANSLATE | NUMBER | table row 2, label 7 | VISIBLE |
| INF-012-U037 | TRANSLATE | NUMBER | table row 2, label 8 | VISIBLE |
| INF-012-U038 | TRANSLATE | — | table row 2, label 9 | VISIBLE |
| INF-012-U039 | TRANSLATE | RECOMMENDATION | table row 2, label 10 | VISIBLE |
| INF-012-U040 | TRANSLATE | RECOMMENDATION | table row 2, label 11 | VISIBLE |
| INF-012-U041 | RETAIN | PROPER_NOUN | table row 3, label 1 | VISIBLE |
| INF-012-U042 | TRANSLATE | — | table row 3, label 2 | VISIBLE |
| INF-012-U043 | RETAIN | BRAND | table row 3, label 3 | VISIBLE |
| INF-012-U044 | RETAIN | PROPER_NOUN | table row 3, label 4 | VISIBLE |
| INF-012-U045 | TRANSLATE | NUMBER | table row 3, label 5 | VISIBLE |
| INF-012-U046 | TRANSLATE | NUMBER | table row 3, label 6 | VISIBLE |
| INF-012-U047 | RETAIN | PROPER_NOUN | table row 3, label 7 | VISIBLE |
| INF-012-U048 | TRANSLATE | — | table row 3, label 8 | VISIBLE |
| INF-012-U049 | TRANSLATE | — | table row 3, label 9 | VISIBLE |
| INF-012-U050 | TRANSLATE | RECOMMENDATION | table row 3, label 10 | VISIBLE |
| INF-012-U051 | TRANSLATE | RECOMMENDATION | table row 3, label 11 | VISIBLE |
| INF-012-U052 | RETAIN | PROPER_NOUN | table row 4, label 1 | VISIBLE |
| INF-012-U053 | TRANSLATE | — | table row 4, label 2 | VISIBLE |
| INF-012-U054 | RETAIN | BRAND | table row 4, label 3 | VISIBLE |
| INF-012-U055 | TRANSLATE | BRAND | table row 4, label 4 | VISIBLE |
| INF-012-U056 | RETAIN | PROPER_NOUN | table row 4, label 5 | VISIBLE |
| INF-012-U057 | TRANSLATE | — | table row 4, label 6 | VISIBLE |
| INF-012-U058 | TRANSLATE | NUMBER, SYMBOL | table row 4, label 7 | VISIBLE |
| INF-012-U059 | TRANSLATE | NUMBER | table row 4, label 8 | VISIBLE |
| INF-012-U060 | TRANSLATE | NUMBER | table row 4, label 9 | VISIBLE |
| INF-012-U061 | TRANSLATE | — | table row 4, label 10 | VISIBLE |
| INF-012-U062 | TRANSLATE | RECOMMENDATION | table row 4, label 11 | VISIBLE |
| INF-012-U063 | TRANSLATE | BRAND, RECOMMENDATION | table row 4, label 12 | VISIBLE |
| INF-012-U064 | RETAIN | PROPER_NOUN | table row 5, label 1 | VISIBLE |
| INF-012-U065 | TRANSLATE | — | table row 5, label 2 | VISIBLE |
| INF-012-U066 | RETAIN | BRAND | table row 5, label 3 | VISIBLE |
| INF-012-U067 | RETAIN | PROPER_NOUN | table row 5, label 4 | VISIBLE |
| INF-012-U068 | TRANSLATE | NUMBER | table row 5, label 5 | VISIBLE |
| INF-012-U069 | TRANSLATE | NUMBER | table row 5, label 6 | VISIBLE |
| INF-012-U070 | TRANSLATE | — | table row 5, label 7 | VISIBLE |
| INF-012-U071 | TRANSLATE | — | table row 5, label 8 | VISIBLE |
| INF-012-U072 | TRANSLATE | RECOMMENDATION | table row 5, label 9 | VISIBLE |
| INF-012-U073 | TRANSLATE | NUMBER, RECOMMENDATION | table row 5, label 10 | VISIBLE |
| INF-012-U074 | RETAIN | PROPER_NOUN | table row 6, label 1 | VISIBLE |
| INF-012-U075 | TRANSLATE | — | table row 6, label 2 | VISIBLE |
| INF-012-U076 | RETAIN | BRAND | table row 6, label 3 | VISIBLE |
| INF-012-U077 | RETAIN | PROPER_NOUN | table row 6, label 4 | VISIBLE |
| INF-012-U078 | TRANSLATE | NUMBER | table row 6, label 5 | VISIBLE |
| INF-012-U079 | TRANSLATE | NUMBER | table row 6, label 6 | VISIBLE |
| INF-012-U080 | TRANSLATE | — | table row 6, label 7 | VISIBLE |
| INF-012-U081 | TRANSLATE | — | table row 6, label 8 | VISIBLE |
| INF-012-U082 | TRANSLATE | RECOMMENDATION | table row 6, label 9 | VISIBLE |
| INF-012-U083 | TRANSLATE | NUMBER, RECOMMENDATION | table row 6, label 10 | VISIBLE |
| INF-012-U084 | RETAIN | PROPER_NOUN | table row 7, label 1 | VISIBLE |
| INF-012-U085 | TRANSLATE | — | table row 7, label 2 | VISIBLE |
| INF-012-U086 | RETAIN | BRAND | table row 7, label 3 | VISIBLE |
| INF-012-U087 | RETAIN | PROPER_NOUN | table row 7, label 4 | VISIBLE |
| INF-012-U088 | TRANSLATE | — | table row 7, label 5 | VISIBLE |
| INF-012-U089 | TRANSLATE | — | table row 7, label 6 | VISIBLE |
| INF-012-U090 | TRANSLATE | — | table row 7, label 7 | VISIBLE |
| INF-012-U091 | TRANSLATE | RECOMMENDATION | table row 7, label 8 | VISIBLE |
| INF-012-U092 | TRANSLATE | RECOMMENDATION | table row 7, label 9 | VISIBLE |
| INF-012-U093 | TRANSLATE | BRAND | bottom comparison heading | VISIBLE |
| INF-012-U094 | RETAIN | BRAND | bottom comparison | VISIBLE |
| INF-012-U095 | TRANSLATE | — | bottom comparison | VISIBLE |
| INF-012-U096 | TRANSLATE | PROPER_NOUN, NUMBER, SYMBOL | bottom comparison | VISIBLE |
| INF-012-U097 | TRANSLATE | PROPER_NOUN, NUMBER, SYMBOL | bottom comparison | VISIBLE |
| INF-012-U098 | TRANSLATE | PROPER_NOUN, RECOMMENDATION, SYMBOL | bottom comparison | VISIBLE |
| INF-012-U099 | TRANSLATE | SYMBOL | bottom comparison | VISIBLE |
| INF-012-U100 | TRANSLATE | SYMBOL | bottom comparison | VISIBLE |
| INF-012-U101 | RETAIN | NUMBER, SYMBOL | bottom comparison | VISIBLE |
| INF-012-U102 | RETAIN | BRAND | bottom comparison | VISIBLE |
| INF-012-U103 | TRANSLATE | — | bottom comparison | VISIBLE |
| INF-012-U104 | TRANSLATE | PROPER_NOUN, NUMBER, SYMBOL | bottom comparison | VISIBLE |
| INF-012-U105 | TRANSLATE | PROPER_NOUN, NUMBER, SYMBOL | bottom comparison | VISIBLE |
| INF-012-U106 | TRANSLATE | SYMBOL | bottom comparison | VISIBLE |
| INF-012-U107 | TRANSLATE | SYMBOL | bottom comparison | VISIBLE |
| INF-012-U108 | TRANSLATE | SYMBOL | bottom comparison | VISIBLE |
| INF-012-U109 | RETAIN | NUMBER, SYMBOL | bottom comparison | VISIBLE |
| INF-012-U110 | RETAIN | NUMBER, SYMBOL | bottom comparison | VISIBLE |
| INF-012-U111 | TRANSLATE | — | tips heading | VISIBLE |
| INF-012-U112 | TRANSLATE | NUMBER, RECOMMENDATION | bottom tips/footer | VISIBLE |
| INF-012-U113 | TRANSLATE | NUMBER, RECOMMENDATION | bottom tips/footer | VISIBLE |
| INF-012-U114 | TRANSLATE | RECOMMENDATION | bottom tips/footer | VISIBLE |
| INF-012-U115 | TRANSLATE | BRAND | bottom tips/footer | VISIBLE |
| INF-012-U116 | RETAIN | BRAND | footer brand | VISIBLE |

Exact units: 116; TRANSLATE 83; RETAIN 33.

## E. PROTECTION

- Preserve native 768×1024 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Seven destination rows and their six-column associations, BEST CHOICE/BOTH OK judgments and hotel-area recommendation order.
- Source stop ordinals (6th, 8th, Last Stop), Line 2 transfer statements and all times/fares are immutable extraction data, not newly verified travel facts.

### Source-specific notes

- Table is transcribed row-by-row, left-to-right across six columns. Source train-stop ordinals and transfer statements are extracted without factual correction.
- Top-right visible brand fragments are Korea / Insid; complete unclipped Korea Inside occurs in footer.

### REVIEW_REQUIRED

1. **Top-right Korea Inside wordmark** — The wordmark is clipped by the right canvas boundary. Visible fragments are recorded; do not invent cropped glyphs or silently reconstruct the brand without review.

# INF-013 — AREX terminal directions

**Asset status:** REVIEW_REQUIRED

## A. IDENTITY / SOURCE

- INF ID: `INF-013`
- Source filename: `arex-terminal-1-2-directions.png`
- Source asset: `images/airport/arex/arex-terminal-1-2-directions.png`
- Format: PNG
- Native dimensions: 736 × 1024 px
- Current SHA-256: `66775d9edefd211995cfa6b0a9e8d6d3c10058b39270e78f503417048896daa0`
- Inherited audit SHA-256: `66775d9edefd211995cfa6b0a9e8d6d3c10058b39270e78f503417048896daa0`
- SHA comparison: MATCH
- Inherited audit dimensions: 736 × 1024 px; comparison: MATCH
- Priority: P2
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `arex.html`, image reference line 293
- Usage location: AREX terminal access / T1–T2 platform directions
- Existing wrapper: `<figure class="arex-infographic">`
- English alt (reference only, not an embedded image unit): `Step-by-step directions from Incheon Airport Terminal 1 and Terminal 2 arrival halls to the AREX platforms`
- English caption (reference only): `How to reach the AREX station from Terminal 1 and Terminal 2`
- Thai page: `th/arex.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `arex-terminal-1-2-directions-th.png`
- Planned Thai path: `images/airport/arex/arex-terminal-1-2-directions-th.png`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/airport/arex/arex-terminal-1-2-directions-es.png` — EXISTS
  - Sibling `es/arex.html`; page EXISTS; image reference `../images/airport/arex/arex-terminal-1-2-directions-es.png`
- JA precedent asset: `images/airport/arex/arex-terminal-1-2-directions-ja.png` — EXISTS
  - Sibling `ja/arex.html`; page EXISTS; image reference `../images/airport/arex/arex-terminal-1-2-directions-ja.png`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-013-U001` — `How to Find AREX at Incheon Airport (T1 & T2)`
2. `INF-013-U002` — `Step-by-Step Directions from Arrival to Platform`
3. `INF-013-U003` — `Korea Inside`
4. `INF-013-U004` — `Express Train`
5. `INF-013-U005` — `(Direct to Seoul Station)`
6. `INF-013-U006` — `All-Stop Train`
7. `INF-013-U007` — `(Every Station)`
8. `INF-013-U008` — `Follow the signs:`
9. `INF-013-U009` — `“Airport Railroad” 공항철도`
10. `INF-013-U010` — `TERMINAL 1`
11. `INF-013-U011` — `From Arrival Hall to Platform`
12. `INF-013-U012` — `Allow extra time with luggage.`
13. `INF-013-U013` — `1`
14. `INF-013-U014` — `Arrival Hall (1F)`
15. `INF-013-U015` — `Complete immigration, baggage claim, and customs.`
16. `INF-013-U016` — `2`
17. `INF-013-U017` — `Follow the Signs`
18. `INF-013-U018` — `Look for the blue “Airport Railroad” signs.`
19. `INF-013-U019` — `Airport Railroad`
20. `INF-013-U020` — `3`
21. `INF-013-U021` — `Go to Transportation Center (B1F)`
22. `INF-013-U022` — `Take the escalator or elevator down to B1 (Transportation Center).`
23. `INF-013-U023` — `Transportation Center`
24. `INF-013-U024` — `B1`
25. `INF-013-U025` — `4`
26. `INF-013-U026` — `Buy Your Ticket`
27. `INF-013-U027` — `Purchase tickets at the machines or ticket counters.`
28. `INF-013-U028` — `Express Train`
29. `INF-013-U029` — `One-way ticket`
30. `INF-013-U030` — `(seat reservation included)`
31. `INF-013-U031` — `All-Stop Train`
32. `INF-013-U032` — `T-money card or single-journey ticket`
33. `INF-013-U033` — `5`
34. `INF-013-U034` — `Go to the Platforms`
35. `INF-013-U035` — `Follow the signs and take the escalator or elevator down to the platform.`
36. `INF-013-U036` — `6`
37. `INF-013-U037` — `Board the Train`
38. `INF-013-U038` — `Check the train type (Express or All-Stop) and platform on the electronic board.`
39. `INF-013-U039` — `TERMINAL 2`
40. `INF-013-U040` — `From Arrival Hall to Platform`
41. `INF-013-U041` — `Allow extra time with luggage.`
42. `INF-013-U042` — `1`
43. `INF-013-U043` — `Arrival Hall (1F)`
44. `INF-013-U044` — `Complete immigration, baggage claim, and customs.`
45. `INF-013-U045` — `2`
46. `INF-013-U046` — `Follow the Signs`
47. `INF-013-U047` — `Look for the blue “Airport Railroad” signs.`
48. `INF-013-U048` — `Airport Railroad`
49. `INF-013-U049` — `3`
50. `INF-013-U050` — `Go to Transportation Center (B1F)`
51. `INF-013-U051` — `Take the escalator or elevator down to B1 (Transportation Center).`
52. `INF-013-U052` — `Transportation Center`
53. `INF-013-U053` — `B1`
54. `INF-013-U054` — `4`
55. `INF-013-U055` — `Buy Your Ticket`
56. `INF-013-U056` — `Purchase tickets at the machines or ticket counters.`
57. `INF-013-U057` — `Express Train`
58. `INF-013-U058` — `One-way ticket`
59. `INF-013-U059` — `(seat reservation included)`
60. `INF-013-U060` — `All-Stop Train`
61. `INF-013-U061` — `T-money card or single-journey ticket`
62. `INF-013-U062` — `5`
63. `INF-013-U063` — `Go to the Platforms`
64. `INF-013-U064` — `Follow the signs and take the escalator or elevator down to the platform.`
65. `INF-013-U065` — `6`
66. `INF-013-U066` — `Board the Train`
67. `INF-013-U067` — `Check the train type (Express or All-Stop) and platform on the electronic board.`
68. `INF-013-U068` — `IMPORTANT NOTES`
69. `INF-013-U069` — `Each terminal has one AREX station. Express and All-Stop trains have separate ticket gates and boarding areas. Follow the signs and check the electronic board.`
70. `INF-013-U070` — `Platform information can change. Always confirm the correct platform and train type before boarding.`
71. `INF-013-U071` — `Ticket types and fares are shown on the left.`
72. `INF-013-U072` — `Source: AREX Official Website (www.arex.or.kr) | Incheon Airport Official Website (www.airport.kr)`
73. `INF-013-U073` — `Last Updated: July 26, 2026`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-013-U001 | TRANSLATE | BRAND, PROPER_NOUN, NUMBER, SYMBOL | heading | VISIBLE |
| INF-013-U002 | TRANSLATE | — | subtitle | VISIBLE |
| INF-013-U003 | RETAIN | BRAND | top brand | VISIBLE |
| INF-013-U004 | TRANSLATE | BRAND | legend | VISIBLE |
| INF-013-U005 | TRANSLATE | PROPER_NOUN | legend | VISIBLE |
| INF-013-U006 | TRANSLATE | BRAND | legend | VISIBLE |
| INF-013-U007 | TRANSLATE | — | legend | VISIBLE |
| INF-013-U008 | TRANSLATE | — | legend | VISIBLE |
| INF-013-U009 | RETAIN | PROPER_NOUN, SYMBOL | legend | VISIBLE |
| INF-013-U010 | RETAIN | PROPER_NOUN, NUMBER | T1 sidebar | VISIBLE |
| INF-013-U011 | TRANSLATE | — | T1 sidebar | VISIBLE |
| INF-013-U012 | TRANSLATE | — | T1 sidebar | VISIBLE |
| INF-013-U013 | RETAIN | NUMBER | T1 step badge 1 | VISIBLE |
| INF-013-U014 | TRANSLATE | NUMBER, PROPER_NOUN | T1 step 1 | VISIBLE |
| INF-013-U015 | TRANSLATE | — | T1 step 1 | VISIBLE |
| INF-013-U016 | RETAIN | NUMBER | T1 step badge 2 | VISIBLE |
| INF-013-U017 | TRANSLATE | — | T1 step 2 | VISIBLE |
| INF-013-U018 | TRANSLATE | PROPER_NOUN, SYMBOL | T1 step 2 | VISIBLE |
| INF-013-U019 | RETAIN | PROPER_NOUN | T1 step 2 photo sign | VISIBLE |
| INF-013-U020 | RETAIN | NUMBER | T1 step badge 3 | VISIBLE |
| INF-013-U021 | TRANSLATE | NUMBER, PROPER_NOUN | T1 step 3 | VISIBLE |
| INF-013-U022 | TRANSLATE | NUMBER, PROPER_NOUN | T1 step 3 | VISIBLE |
| INF-013-U023 | RETAIN | PROPER_NOUN | T1 step 3 photo sign | VISIBLE |
| INF-013-U024 | RETAIN | NUMBER | T1 step 3 photo sign | VISIBLE |
| INF-013-U025 | RETAIN | NUMBER | T1 step badge 4 | VISIBLE |
| INF-013-U026 | TRANSLATE | — | T1 step 4 | VISIBLE |
| INF-013-U027 | TRANSLATE | — | T1 step 4 | VISIBLE |
| INF-013-U028 | TRANSLATE | BRAND | T1 step 4 ticket key | VISIBLE |
| INF-013-U029 | TRANSLATE | — | T1 step 4 ticket key | VISIBLE |
| INF-013-U030 | TRANSLATE | — | T1 step 4 ticket key | VISIBLE |
| INF-013-U031 | TRANSLATE | BRAND | T1 step 4 ticket key | VISIBLE |
| INF-013-U032 | TRANSLATE | BRAND | T1 step 4 ticket key | VISIBLE |
| INF-013-U033 | RETAIN | NUMBER | T1 step badge 5 | VISIBLE |
| INF-013-U034 | TRANSLATE | — | T1 step 5 | VISIBLE |
| INF-013-U035 | TRANSLATE | — | T1 step 5 | VISIBLE |
| INF-013-U036 | RETAIN | NUMBER | T1 step badge 6 | VISIBLE |
| INF-013-U037 | TRANSLATE | — | T1 step 6 | VISIBLE |
| INF-013-U038 | TRANSLATE | BRAND | T1 step 6 | VISIBLE |
| INF-013-U039 | RETAIN | PROPER_NOUN, NUMBER | T2 sidebar | VISIBLE |
| INF-013-U040 | TRANSLATE | — | T2 sidebar | VISIBLE |
| INF-013-U041 | TRANSLATE | — | T2 sidebar | VISIBLE |
| INF-013-U042 | RETAIN | NUMBER | T2 step badge 1 | VISIBLE |
| INF-013-U043 | TRANSLATE | NUMBER, PROPER_NOUN | T2 step 1 | VISIBLE |
| INF-013-U044 | TRANSLATE | — | T2 step 1 | VISIBLE |
| INF-013-U045 | RETAIN | NUMBER | T2 step badge 2 | VISIBLE |
| INF-013-U046 | TRANSLATE | — | T2 step 2 | VISIBLE |
| INF-013-U047 | TRANSLATE | PROPER_NOUN, SYMBOL | T2 step 2 | VISIBLE |
| INF-013-U048 | RETAIN | PROPER_NOUN | T2 step 2 photo sign | VISIBLE |
| INF-013-U049 | RETAIN | NUMBER | T2 step badge 3 | VISIBLE |
| INF-013-U050 | TRANSLATE | NUMBER, PROPER_NOUN | T2 step 3 | VISIBLE |
| INF-013-U051 | TRANSLATE | NUMBER, PROPER_NOUN | T2 step 3 | VISIBLE |
| INF-013-U052 | RETAIN | PROPER_NOUN | T2 step 3 photo sign | VISIBLE |
| INF-013-U053 | RETAIN | NUMBER | T2 step 3 photo sign | VISIBLE |
| INF-013-U054 | RETAIN | NUMBER | T2 step badge 4 | VISIBLE |
| INF-013-U055 | TRANSLATE | — | T2 step 4 | VISIBLE |
| INF-013-U056 | TRANSLATE | — | T2 step 4 | VISIBLE |
| INF-013-U057 | TRANSLATE | BRAND | T2 step 4 ticket key | VISIBLE |
| INF-013-U058 | TRANSLATE | — | T2 step 4 ticket key | VISIBLE |
| INF-013-U059 | TRANSLATE | — | T2 step 4 ticket key | VISIBLE |
| INF-013-U060 | TRANSLATE | BRAND | T2 step 4 ticket key | VISIBLE |
| INF-013-U061 | TRANSLATE | BRAND | T2 step 4 ticket key | VISIBLE |
| INF-013-U062 | RETAIN | NUMBER | T2 step badge 5 | VISIBLE |
| INF-013-U063 | TRANSLATE | — | T2 step 5 | VISIBLE |
| INF-013-U064 | TRANSLATE | — | T2 step 5 | VISIBLE |
| INF-013-U065 | RETAIN | NUMBER | T2 step badge 6 | VISIBLE |
| INF-013-U066 | TRANSLATE | — | T2 step 6 | VISIBLE |
| INF-013-U067 | TRANSLATE | BRAND | T2 step 6 | VISIBLE |
| INF-013-U068 | TRANSLATE | — | bottom notes / footer | VISIBLE |
| INF-013-U069 | TRANSLATE | BRAND, PROPER_NOUN | bottom notes / footer | VISIBLE |
| INF-013-U070 | TRANSLATE | — | bottom notes / footer | VISIBLE |
| INF-013-U071 | TRANSLATE | — | bottom notes / footer | VISIBLE |
| INF-013-U072 | TRANSLATE | BRAND, PROPER_NOUN | bottom notes / footer | VISIBLE |
| INF-013-U073 | TRANSLATE | NUMBER | bottom notes / footer | VISIBLE |

Exact units: 73; TRANSLATE 51; RETAIN 22.

## E. PROTECTION

- Preserve native 736×1024 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- T1/T2, 1F/B1F/B1 levels; steps 1–6 repeated twice; gate/ticket/train distinctions; official source URLs and July 26, 2026 date.

### Source-specific notes

- T1 and T2 each have the same six-step editorial sequence; all repeated editorial occurrences are independently enumerated.
- Korean-only terminal names/sign lettering are source non-English content, retained as visual source context; no Thai wording authored.

### REVIEW_REQUIRED

1. **T1/T2 embedded photographs: ticket-machine screens and distant platform signs** — Microtext is below reliable legibility at native 736×1024. Legible Airport Railroad / Transportation Center / B1 signage is included; unreadable screen/sign glyphs are not guessed. These photograph/UI details require source-master or pixel review before Thai production.

# INF-014 — AREX ticket decision guide

**Asset status:** EXTRACTION COMPLETE

## A. IDENTITY / SOURCE

- INF ID: `INF-014`
- Source filename: `arex-ticket-decision-guide.png`
- Source asset: `images/airport/arex/arex-ticket-decision-guide.png`
- Format: PNG
- Native dimensions: 1536 × 1024 px
- Current SHA-256: `bad56057f050c4b3f293535acf5a4a8a9bebf13ef8533651aeafb0fdc8c8c7bb`
- Inherited audit SHA-256: `bad56057f050c4b3f293535acf5a4a8a9bebf13ef8533651aeafb0fdc8c8c7bb`
- SHA comparison: MATCH
- Inherited audit dimensions: 1536 × 1024 px; comparison: MATCH
- Priority: P1
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `arex.html`, image reference line 307
- Usage location: Tickets for Express and All-Stop / #tickets
- Existing wrapper: `<figure class="arex-infographic">`
- English alt (reference only, not an embedded image unit): `AREX ticket decision guide for Express reservations, station tickets, T-money and single-use cards`
- English caption (reference only): `AREX ticket types for Express and All-Stop trains`
- Thai page: `th/arex.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `arex-ticket-decision-guide-th.png`
- Planned Thai path: `images/airport/arex/arex-ticket-decision-guide-th.png`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/airport/arex/arex-ticket-decision-guide-es.png` — EXISTS
  - Sibling `es/arex.html`; page EXISTS; image reference `../images/airport/arex/arex-ticket-decision-guide-es.png`
- JA precedent asset: `images/airport/arex/arex-ticket-decision-guide-ja.png` — EXISTS
  - Sibling `ja/arex.html`; page EXISTS; image reference `../images/airport/arex/arex-ticket-decision-guide-ja.png`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-014-U001` — `AREX Ticket Decision Guide`
2. `INF-014-U002` — `Answer a few simple questions to choose the best way to travel.`
3. `INF-014-U003` — `Two Trains. Different Tickets.`
4. `INF-014-U004` — `• Express Train → reserved seats, not available with T-money`
5. `INF-014-U005` — `• All-Stop Train → can use T-money or single-journey ticket`
6. `INF-014-U006` — `LEGEND`
7. `INF-014-U007` — `Express Train (Direct to Seoul Station)`
8. `INF-014-U008` — `All-Stop Train (Every Station)`
9. `INF-014-U009` — `START HERE`
10. `INF-014-U010` — `Where are you going?`
11. `INF-014-U011` — `1`
12. `INF-014-U012` — `Is your final destination Seoul Station?`
13. `INF-014-U013` — `YES`
14. `INF-014-U014` — `NO`
15. `INF-014-U015` — `2`
16. `INF-014-U016` — `Do you want a reserved seat?`
17. `INF-014-U017` — `YES`
18. `INF-014-U018` — `NO`
19. `INF-014-U019` — `3`
20. `INF-014-U020` — `Do you have T-money card?`
21. `INF-014-U021` — `T`
22. `INF-014-U022` — `money`
23. `INF-014-U023` — `YES`
24. `INF-014-U024` — `NO`
25. `INF-014-U025` — `4`
26. `INF-014-U026` — `Traveling with large luggage or with family/elderly?`
27. `INF-014-U027` — `YES`
28. `INF-014-U028` — `NO`
29. `INF-014-U029` — `5`
30. `INF-014-U030` — `Arriving late at night (may miss the last train)?`
31. `INF-014-U031` — `YES`
32. `INF-014-U032` — `NO`
33. `INF-014-U033` — `EXPRESS TRAIN`
34. `INF-014-U034` — `(Online Reservation)`
35. `INF-014-U035` — `Best for travelers going to Seoul Station who want fast, comfortable travel with a reserved seat.`
36. `INF-014-U036` — `How to Buy`
37. `INF-014-U037` — `Buy online in advance`
38. `INF-014-U038` — `Pick your seat`
39. `INF-014-U039` — `QR ticket (mobile)`
40. `INF-014-U040` — `Show QR at gate`
41. `INF-014-U041` — `Pay`
42. `INF-014-U042` — `Credit/Debit Card (International cards OK)`
43. `INF-014-U043` — `EXPRESS TRAIN`
44. `INF-014-U044` — `(At Station)`
45. `INF-014-U045` — `Buy at the station on the day of travel. Reserved seat available if seats remain.`
46. `INF-014-U046` — `How to Buy`
47. `INF-014-U047` — `Use ticket machines or ticket counters`
48. `INF-014-U048` — `Choose time & seat`
49. `INF-014-U049` — `Pay and get ticket`
50. `INF-014-U050` — `Pay`
51. `INF-014-U051` — `Credit/Debit Card or Cash (KRW)`
52. `INF-014-U052` — `ALL-STOP TRAIN`
53. `INF-014-U053` — `(T-money)`
54. `INF-014-U054` — `Use your T-money card. Tap in and tap out. Cheapest and most convenient for multiple stops.`
55. `INF-014-U055` — `How to Use`
56. `INF-014-U056` — `Tap in at gate`
57. `INF-014-U057` — `Tap out at your destination`
58. `INF-014-U058` — `Fare is calculated automatically by distance`
59. `INF-014-U059` — `Pay`
60. `INF-014-U060` — `T`
61. `INF-014-U061` — `money`
62. `INF-014-U062` — `T-money Card (Balance required)`
63. `INF-014-U063` — `ALL-STOP TRAIN`
64. `INF-014-U064` — `(Single-Journey Ticket)`
65. `INF-014-U065` — `Buy a single-journey ticket at the station.`
66. `INF-014-U066` — `How to Buy`
67. `INF-014-U067` — `Use ticket machines or ticket counters`
68. `INF-014-U068` — `Select your destination`
69. `INF-014-U069` — `Get ticket and go`
70. `INF-014-U070` — `Pay`
71. `INF-014-U071` — `Credit/Debit Card or Cash (KRW)`
72. `INF-014-U072` — `EXPRESS TRAIN`
73. `INF-014-U073` — `(Recommended)`
74. `INF-014-U074` — `More space, wider seats, and luggage racks. Better for families, seniors, or large luggage.`
75. `INF-014-U075` — `How to Buy`
76. `INF-014-U076` — `Reserve online or buy at the station`
77. `INF-014-U077` — `Choose seat`
78. `INF-014-U078` — `Board and enjoy`
79. `INF-014-U079` — `Pay`
80. `INF-014-U080` — `Credit/Debit Card (International cards OK)`
81. `INF-014-U081` — `CHECK SCHEDULE FIRST`
82. `INF-014-U082` — `If you might miss the last AREX train, consider Airport Bus or Taxi.`
83. `INF-014-U083` — `What to Do`
84. `INF-014-U084` — `Check last train time (on official website)`
85. `INF-014-U085` — `If too late, use Airport Bus or Taxi instead`
86. `INF-014-U086` — `Useful Links`
87. `INF-014-U087` — `airport-bus.html`
88. `INF-014-U088` — `taxi.html`
89. `INF-014-U089` — `TICKET TYPES OVERVIEW`
90. `INF-014-U090` — `Express Train`
91. `INF-014-U091` — `• Reserved seat`
92. `INF-014-U092` — `• Direct to Seoul Station`
93. `INF-014-U093` — `• Not available with T-money`
94. `INF-014-U094` — `• Online reservation available`
95. `INF-014-U095` — `All-Stop Train`
96. `INF-014-U096` — `• Stops at every station`
97. `INF-014-U097` — `• Use T-money or single ticket`
98. `INF-014-U098` — `• Lower fare`
99. `INF-014-U099` — `• Great for multiple destinations`
100. `INF-014-U100` — `PAYMENT METHODS`
101. `INF-014-U101` — `Credit/Debit Card`
102. `INF-014-U102` — `(Visa, Mastercard, JCB, American Express)`
103. `INF-014-U103` — `Cash (KRW)`
104. `INF-014-U104` — `(At ticket counter only)`
105. `INF-014-U105` — `T`
106. `INF-014-U106` — `money`
107. `INF-014-U107` — `T-money Card`
108. `INF-014-U108` — `(For All-Stop Train only)`
109. `INF-014-U109` — `GOOD TO KNOW`
110. `INF-014-U110` — `Children 6–12 get discounted fares. Under 6 ride free.`
111. `INF-014-U111` — `Bring your passport if you reserved online.`
112. `INF-014-U112` — `Keep your single-journey ticket until you exit.`
113. `INF-014-U113` — `Schedules and fares may change. Check the official website.`
114. `INF-014-U114` — `OFFICIAL WEBSITE`
115. `INF-014-U115` — `www.airportrailroad.com`
116. `INF-014-U116` — `Check schedules, fares, and ticket information.`
117. `INF-014-U117` — `Source: AREX Official Website (www.airportrailroad.com) | Incheon Airport Official Website (www.airport.kr)`
118. `INF-014-U118` — `Last Updated: July 26, 2026`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-014-U001 | TRANSLATE | BRAND | heading | VISIBLE |
| INF-014-U002 | TRANSLATE | RECOMMENDATION | subtitle | VISIBLE |
| INF-014-U003 | TRANSLATE | — | top note | VISIBLE |
| INF-014-U004 | TRANSLATE | BRAND, SYMBOL | top note | VISIBLE |
| INF-014-U005 | TRANSLATE | BRAND, SYMBOL | top note | VISIBLE |
| INF-014-U006 | TRANSLATE | — | legend | VISIBLE |
| INF-014-U007 | TRANSLATE | BRAND, PROPER_NOUN | legend | VISIBLE |
| INF-014-U008 | TRANSLATE | BRAND, PROPER_NOUN | legend | VISIBLE |
| INF-014-U009 | TRANSLATE | — | start card | VISIBLE |
| INF-014-U010 | TRANSLATE | — | start card | VISIBLE |
| INF-014-U011 | RETAIN | NUMBER | question 1 badge | VISIBLE |
| INF-014-U012 | TRANSLATE | PROPER_NOUN | question 1 | VISIBLE |
| INF-014-U013 | TRANSLATE | — | question 1 left branch | VISIBLE |
| INF-014-U014 | TRANSLATE | — | question 1 right branch | VISIBLE |
| INF-014-U015 | RETAIN | NUMBER | question 2 badge | VISIBLE |
| INF-014-U016 | TRANSLATE | — | question 2 | VISIBLE |
| INF-014-U017 | TRANSLATE | — | question 2 left branch | VISIBLE |
| INF-014-U018 | TRANSLATE | — | question 2 right branch | VISIBLE |
| INF-014-U019 | RETAIN | NUMBER | question 3 badge | VISIBLE |
| INF-014-U020 | TRANSLATE | BRAND | question 3 | VISIBLE |
| INF-014-U021 | RETAIN | BRAND | question 3 logo top | VISIBLE |
| INF-014-U022 | RETAIN | BRAND | question 3 logo bottom | VISIBLE |
| INF-014-U023 | TRANSLATE | — | question 3 left branch | VISIBLE |
| INF-014-U024 | TRANSLATE | — | question 3 right branch | VISIBLE |
| INF-014-U025 | RETAIN | NUMBER | question 4 badge | VISIBLE |
| INF-014-U026 | TRANSLATE | — | question 4 | VISIBLE |
| INF-014-U027 | TRANSLATE | — | question 4 left branch | VISIBLE |
| INF-014-U028 | TRANSLATE | — | question 4 right branch | VISIBLE |
| INF-014-U029 | RETAIN | NUMBER | question 5 badge | VISIBLE |
| INF-014-U030 | TRANSLATE | — | question 5 | VISIBLE |
| INF-014-U031 | TRANSLATE | — | question 5 left branch | VISIBLE |
| INF-014-U032 | TRANSLATE | — | question 5 right branch | VISIBLE |
| INF-014-U033 | RETAIN | BRAND | result card 1, text 1 | VISIBLE |
| INF-014-U034 | TRANSLATE | — | result card 1, text 2 | VISIBLE |
| INF-014-U035 | TRANSLATE | PROPER_NOUN, RECOMMENDATION | result card 1, text 3 | VISIBLE |
| INF-014-U036 | TRANSLATE | — | result card 1, text 4 | VISIBLE |
| INF-014-U037 | TRANSLATE | — | result card 1, text 5 | VISIBLE |
| INF-014-U038 | TRANSLATE | — | result card 1, text 6 | VISIBLE |
| INF-014-U039 | TRANSLATE | — | result card 1, text 7 | VISIBLE |
| INF-014-U040 | TRANSLATE | — | result card 1, text 8 | VISIBLE |
| INF-014-U041 | TRANSLATE | — | result card 1, text 9 | VISIBLE |
| INF-014-U042 | TRANSLATE | — | result card 1, text 10 | VISIBLE |
| INF-014-U043 | RETAIN | BRAND | result card 2, text 1 | VISIBLE |
| INF-014-U044 | TRANSLATE | — | result card 2, text 2 | VISIBLE |
| INF-014-U045 | TRANSLATE | — | result card 2, text 3 | VISIBLE |
| INF-014-U046 | TRANSLATE | — | result card 2, text 4 | VISIBLE |
| INF-014-U047 | TRANSLATE | — | result card 2, text 5 | VISIBLE |
| INF-014-U048 | TRANSLATE | SYMBOL | result card 2, text 6 | VISIBLE |
| INF-014-U049 | TRANSLATE | — | result card 2, text 7 | VISIBLE |
| INF-014-U050 | TRANSLATE | — | result card 2, text 8 | VISIBLE |
| INF-014-U051 | TRANSLATE | — | result card 2, text 9 | VISIBLE |
| INF-014-U052 | RETAIN | BRAND | result card 3, text 1 | VISIBLE |
| INF-014-U053 | TRANSLATE | BRAND | result card 3, text 2 | VISIBLE |
| INF-014-U054 | TRANSLATE | BRAND, RECOMMENDATION | result card 3, text 3 | VISIBLE |
| INF-014-U055 | TRANSLATE | — | result card 3, text 4 | VISIBLE |
| INF-014-U056 | TRANSLATE | — | result card 3, text 5 | VISIBLE |
| INF-014-U057 | TRANSLATE | — | result card 3, text 6 | VISIBLE |
| INF-014-U058 | TRANSLATE | — | result card 3, text 7 | VISIBLE |
| INF-014-U059 | TRANSLATE | — | result card 3, text 8 | VISIBLE |
| INF-014-U060 | RETAIN | BRAND | result card 3, text 9 | VISIBLE |
| INF-014-U061 | RETAIN | BRAND | result card 3, text 10 | VISIBLE |
| INF-014-U062 | TRANSLATE | BRAND | result card 3, text 11 | VISIBLE |
| INF-014-U063 | RETAIN | BRAND | result card 4, text 1 | VISIBLE |
| INF-014-U064 | TRANSLATE | — | result card 4, text 2 | VISIBLE |
| INF-014-U065 | TRANSLATE | — | result card 4, text 3 | VISIBLE |
| INF-014-U066 | TRANSLATE | — | result card 4, text 4 | VISIBLE |
| INF-014-U067 | TRANSLATE | — | result card 4, text 5 | VISIBLE |
| INF-014-U068 | TRANSLATE | — | result card 4, text 6 | VISIBLE |
| INF-014-U069 | TRANSLATE | — | result card 4, text 7 | VISIBLE |
| INF-014-U070 | TRANSLATE | — | result card 4, text 8 | VISIBLE |
| INF-014-U071 | TRANSLATE | — | result card 4, text 9 | VISIBLE |
| INF-014-U072 | RETAIN | BRAND | result card 5, text 1 | VISIBLE |
| INF-014-U073 | TRANSLATE | RECOMMENDATION | result card 5, text 2 | VISIBLE |
| INF-014-U074 | TRANSLATE | RECOMMENDATION | result card 5, text 3 | VISIBLE |
| INF-014-U075 | TRANSLATE | — | result card 5, text 4 | VISIBLE |
| INF-014-U076 | TRANSLATE | — | result card 5, text 5 | VISIBLE |
| INF-014-U077 | TRANSLATE | — | result card 5, text 6 | VISIBLE |
| INF-014-U078 | TRANSLATE | — | result card 5, text 7 | VISIBLE |
| INF-014-U079 | TRANSLATE | — | result card 5, text 8 | VISIBLE |
| INF-014-U080 | TRANSLATE | — | result card 5, text 9 | VISIBLE |
| INF-014-U081 | TRANSLATE | — | result card 6, text 1 | VISIBLE |
| INF-014-U082 | TRANSLATE | BRAND, RECOMMENDATION | result card 6, text 2 | VISIBLE |
| INF-014-U083 | TRANSLATE | — | result card 6, text 3 | VISIBLE |
| INF-014-U084 | TRANSLATE | — | result card 6, text 4 | VISIBLE |
| INF-014-U085 | TRANSLATE | RECOMMENDATION | result card 6, text 5 | VISIBLE |
| INF-014-U086 | TRANSLATE | — | result card 6, text 6 | VISIBLE |
| INF-014-U087 | RETAIN | — | result card 6, text 7 | VISIBLE |
| INF-014-U088 | RETAIN | — | result card 6, text 8 | VISIBLE |
| INF-014-U089 | TRANSLATE | — | bottom left heading | VISIBLE |
| INF-014-U090 | TRANSLATE | BRAND | bottom left Express | VISIBLE |
| INF-014-U091 | TRANSLATE | SYMBOL | bottom left Express | VISIBLE |
| INF-014-U092 | TRANSLATE | SYMBOL | bottom left Express | VISIBLE |
| INF-014-U093 | TRANSLATE | BRAND, SYMBOL | bottom left Express | VISIBLE |
| INF-014-U094 | TRANSLATE | SYMBOL | bottom left Express | VISIBLE |
| INF-014-U095 | TRANSLATE | BRAND | bottom left All-Stop | VISIBLE |
| INF-014-U096 | TRANSLATE | SYMBOL | bottom left All-Stop | VISIBLE |
| INF-014-U097 | TRANSLATE | BRAND, SYMBOL | bottom left All-Stop | VISIBLE |
| INF-014-U098 | TRANSLATE | SYMBOL | bottom left All-Stop | VISIBLE |
| INF-014-U099 | TRANSLATE | SYMBOL, RECOMMENDATION | bottom left All-Stop | VISIBLE |
| INF-014-U100 | TRANSLATE | — | bottom payment | VISIBLE |
| INF-014-U101 | TRANSLATE | — | bottom payment card | VISIBLE |
| INF-014-U102 | RETAIN | BRAND | bottom payment card | VISIBLE |
| INF-014-U103 | TRANSLATE | — | bottom payment cash | VISIBLE |
| INF-014-U104 | TRANSLATE | — | bottom payment cash | VISIBLE |
| INF-014-U105 | RETAIN | BRAND | bottom payment logo top | VISIBLE |
| INF-014-U106 | RETAIN | BRAND | bottom payment logo bottom | VISIBLE |
| INF-014-U107 | TRANSLATE | BRAND | bottom payment T-money | VISIBLE |
| INF-014-U108 | TRANSLATE | BRAND | bottom payment T-money | VISIBLE |
| INF-014-U109 | TRANSLATE | — | bottom tips | VISIBLE |
| INF-014-U110 | TRANSLATE | NUMBER, SYMBOL | bottom tips | VISIBLE |
| INF-014-U111 | TRANSLATE | — | bottom tips | VISIBLE |
| INF-014-U112 | TRANSLATE | — | bottom tips | VISIBLE |
| INF-014-U113 | TRANSLATE | — | bottom tips | VISIBLE |
| INF-014-U114 | TRANSLATE | — | bottom official | VISIBLE |
| INF-014-U115 | RETAIN | — | bottom official | VISIBLE |
| INF-014-U116 | TRANSLATE | — | bottom official | VISIBLE |
| INF-014-U117 | TRANSLATE | BRAND, SYMBOL | footer left | VISIBLE |
| INF-014-U118 | TRANSLATE | NUMBER | footer right | VISIBLE |

Exact units: 118; TRANSLATE 98; RETAIN 20.

## E. PROTECTION

- Preserve native 1536×1024 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Five questions, six result cards and all YES/NO arrow connections.
- Express reserved seats and no T-money; All-Stop T-money/single-journey ticket; online/station purchase distinctions; late-night alternative recommendation.
- Children 6–12 discount / under 6 free; July 26, 2026; URLs and payment brand list.

### Source-specific notes

- Read top notes/legend, question strip 1–5, six decision-result cards left-to-right, then lower summary panels. Repeated YES/NO, buy/pay labels, payment descriptions and logos are separate occurrences.
- Flow connectors are protected graph relationships; do not reinterpret the diagram's branching or silently fix editorial logic.

### REVIEW_REQUIRED

NONE.

# INF-015 — When not to use AREX

**Asset status:** EXTRACTION COMPLETE

## A. IDENTITY / SOURCE

- INF ID: `INF-015`
- Source filename: `when-not-to-use-arex.png`
- Source asset: `images/airport/arex/when-not-to-use-arex.png`
- Format: PNG
- Native dimensions: 1448 × 1086 px
- Current SHA-256: `7fcdaaa24938685cfc5b7f7201d4453a17e515bfa807b44a241f1ae6eb96959d`
- Inherited audit SHA-256: `7fcdaaa24938685cfc5b7f7201d4453a17e515bfa807b44a241f1ae6eb96959d`
- SHA comparison: MATCH
- Inherited audit dimensions: 1448 × 1086 px; comparison: MATCH
- Priority: P2
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `arex.html`, image reference line 355
- Usage location: Luggage, Families and Accessibility / between train and hotel
- Existing wrapper: `<figure class="arex-infographic">`
- English alt (reference only, not an embedded image unit): `Situations when travelers should consider an airport bus, taxi or private pickup instead of AREX`
- English caption (reference only): `When an airport bus, taxi or private pickup may be easier than AREX`
- Thai page: `th/arex.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `when-not-to-use-arex-th.png`
- Planned Thai path: `images/airport/arex/when-not-to-use-arex-th.png`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/airport/arex/when-not-to-use-arex-es.png` — EXISTS
  - Sibling `es/arex.html`; page EXISTS; image reference `../images/airport/arex/when-not-to-use-arex-es.png`
- JA precedent asset: `images/airport/arex/when-not-to-use-arex-ja.png` — EXISTS
  - Sibling `ja/arex.html`; page EXISTS; image reference `../images/airport/arex/when-not-to-use-arex-ja.png`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-015-U001` — `Korea Inside`
2. `INF-015-U002` — `When Should You NOT Use AREX?`
3. `INF-015-U003` — `Incheon Airport to Seoul · When another option may be better`
4. `INF-015-U004` — `1`
5. `INF-015-U005` — `Heavy luggage or a stroller`
6. `INF-015-U006` — `AREX is still possible, but multiple escalators, long station walks and a later subway transfer can be tiring. A direct airport bus or private pickup is often easier.`
7. `INF-015-U007` — `2`
8. `INF-015-U008` — `Your hotel is in Gangnam or Jamsil`
9. `INF-015-U009` — `AREX usually means Seoul Station or Gongdeok first, then another subway ride. If you want fewer transfers, compare an airport bus or taxi instead.`
10. `INF-015-U010` — `3`
11. `INF-015-U011` — `You arrive very late`
12. `INF-015-U012` — `AREX does not run all night. If your flight lands close to the last train, immigration and baggage claim may make you miss it. Check the latest train time before choosing AREX.`
13. `INF-015-U013` — `4`
14. `INF-015-U014` — `You are traveling with family or older adults`
15. `INF-015-U015` — `The train can be efficient, but long walks, stairs and crowded subway transfers may be stressful for children, parents or seniors. A direct bus or private ride may be more comfortable.`
16. `INF-015-U016` — `5`
17. `INF-015-U017` — `HOTEL`
18. `INF-015-U018` — `A bus stop is near your hotel`
19. `INF-015-U019` — `If an airport limousine bus drops you close to the hotel entrance, it may save time and effort even if the train is faster on paper.`
20. `INF-015-U020` — `BETTER ALTERNATIVES`
21. `INF-015-U021` — `Airport Bus`
22. `INF-015-U022` — `Best for direct hotel-area access`
23. `INF-015-U023` — `TAXI`
24. `INF-015-U024` — `Taxi`
25. `INF-015-U025` — `Best for late arrival or door-to-door convenience`
26. `INF-015-U026` — `Private Pickup`
27. `INF-015-U027` — `Best for families, groups and lots of luggage`
28. `INF-015-U028` — `AREX is still one of the best options for Seoul Station, Hongdae, Gongdeok and Gimpo Airport — but it is not always the easiest choice.`
29. `INF-015-U029` — `Always check the latest train times and route details before travel.`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-015-U001 | RETAIN | BRAND | top brand | VISIBLE |
| INF-015-U002 | TRANSLATE | BRAND | heading | VISIBLE |
| INF-015-U003 | TRANSLATE | PROPER_NOUN, SYMBOL, RECOMMENDATION | subtitle | VISIBLE |
| INF-015-U004 | RETAIN | NUMBER | scenario 1 badge | VISIBLE |
| INF-015-U005 | TRANSLATE | — | scenario 1 | VISIBLE |
| INF-015-U006 | TRANSLATE | BRAND, RECOMMENDATION | scenario 1 | VISIBLE |
| INF-015-U007 | RETAIN | NUMBER | scenario 2 badge | VISIBLE |
| INF-015-U008 | TRANSLATE | PROPER_NOUN | scenario 2 | VISIBLE |
| INF-015-U009 | TRANSLATE | BRAND, PROPER_NOUN, RECOMMENDATION | scenario 2 | VISIBLE |
| INF-015-U010 | RETAIN | NUMBER | scenario 3 badge | VISIBLE |
| INF-015-U011 | TRANSLATE | — | scenario 3 | VISIBLE |
| INF-015-U012 | TRANSLATE | BRAND, RECOMMENDATION | scenario 3 | VISIBLE |
| INF-015-U013 | RETAIN | NUMBER | scenario 4 badge | VISIBLE |
| INF-015-U014 | TRANSLATE | — | scenario 4 | VISIBLE |
| INF-015-U015 | TRANSLATE | RECOMMENDATION | scenario 4 | VISIBLE |
| INF-015-U016 | RETAIN | NUMBER | scenario 5 badge | VISIBLE |
| INF-015-U017 | TRANSLATE | — | scenario 5 icon | VISIBLE |
| INF-015-U018 | TRANSLATE | — | scenario 5 | VISIBLE |
| INF-015-U019 | TRANSLATE | RECOMMENDATION | scenario 5 | VISIBLE |
| INF-015-U020 | TRANSLATE | RECOMMENDATION | alternative heading | VISIBLE |
| INF-015-U021 | TRANSLATE | — | alternatives / footer | VISIBLE |
| INF-015-U022 | TRANSLATE | RECOMMENDATION | alternatives / footer | VISIBLE |
| INF-015-U023 | TRANSLATE | — | alternative taxi icon | VISIBLE |
| INF-015-U024 | TRANSLATE | — | alternatives / footer | VISIBLE |
| INF-015-U025 | TRANSLATE | RECOMMENDATION | alternatives / footer | VISIBLE |
| INF-015-U026 | TRANSLATE | — | alternatives / footer | VISIBLE |
| INF-015-U027 | TRANSLATE | RECOMMENDATION | alternatives / footer | VISIBLE |
| INF-015-U028 | TRANSLATE | RECOMMENDATION, BRAND | alternatives / footer | VISIBLE |
| INF-015-U029 | TRANSLATE | RECOMMENDATION | alternatives / footer | VISIBLE |

Exact units: 29; TRANSLATE 23; RETAIN 6.

## E. PROTECTION

- Preserve native 1448×1086 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Five contraindication scenarios and relative recommendation strength (still possible/often easier/may be more comfortable).
- Gangnam/Jamsil transfer examples; late train risk; Seoul Station/Hongdae/Gongdeok/Gimpo Airport remain qualified as best options.
- Airport Bus / Taxi / Private Pickup three alternative associations.

### Source-specific notes

- Five scenario panels read left-to-right; small HOTEL and TAXI icon labels included.

### REVIEW_REQUIRED

NONE.

# INF-016 — Arrival hall first 30 minutes

**Asset status:** REVIEW_REQUIRED

## A. IDENTITY / SOURCE

- INF ID: `INF-016`
- Source filename: `airport-arrival-hall-first-30-minutes-infographic.webp`
- Source asset: `images/airport-arrival-hall-first-30-minutes-infographic.webp`
- Format: WEBP
- Native dimensions: 1402 × 1122 px
- Current SHA-256: `b8fb577669046fd7945d74b46fa3969a0924d258fb39bf7abf8dc12f20318ab8`
- Inherited audit SHA-256: `b8fb577669046fd7945d74b46fa3969a0924d258fb39bf7abf8dc12f20318ab8`
- SHA comparison: MATCH
- Inherited audit dimensions: 1402 × 1122 px; comparison: MATCH
- Priority: P1
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `airport.html`, image reference line 100
- Usage location: Airport page hero / .airport-page-hero__photo
- Existing wrapper: `<figure class="airport-page-hero__photo">`
- English alt (reference only, not an embedded image unit): `Illustrated first 30 minutes in the Incheon Airport arrival hall: connect, save your address, check payment and choose transport`
- English caption (reference only): NONE
- Thai page: `th/airport.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `airport-arrival-hall-first-30-minutes-infographic-th.webp`
- Planned Thai path: `images/airport-arrival-hall-first-30-minutes-infographic-th.webp`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/airport-arrival-hall-first-30-minutes-infographic-es.webp` — EXISTS
  - Sibling `es/airport.html`; page EXISTS; image reference `../images/airport-arrival-hall-first-30-minutes-infographic-es.webp`
- JA precedent asset: `images/airport-arrival-hall-first-30-minutes-infographic-ja.webp` — EXISTS
  - Sibling `ja/airport.html`; page EXISTS; image reference `../images/airport-arrival-hall-first-30-minutes-infographic-ja.webp`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-016-U001` — `YOUR FIRST 30 MINUTES`
2. `INF-016-U002` — `IN THE ARRIVAL HALL`
3. `INF-016-U003` — `After immigration and customs, use this Incheon Airport arrival hall guide to connect, save your address, check payment and choose transport.`
4. `INF-016-U004` — `Arrivals`
5. `INF-016-U005` — `Baggage Claim`
6. `INF-016-U006` — `Customs`
7. `INF-016-U007` — `AREX`
8. `INF-016-U008` — `Bus`
9. `INF-016-U009` — `Taxi`
10. `INF-016-U010` — `P`
11. `INF-016-U011` — `Parking`
12. `INF-016-U012` — `INFORMATION`
13. `INF-016-U013` — `FREE Wi-Fi`
14. `INF-016-U014` — `01`
15. `INF-016-U015` — `CONNECT`
16. `INF-016-U016` — `Check your eSIM or Wi-Fi.`
17. `INF-016-U017` — `02`
18. `INF-016-U018` — `SAVE YOUR ADDRESS`
19. `INF-016-U019` — `Keep your hotel address in Korean ready.`
20. `INF-016-U020` — `03`
21. `INF-016-U021` — `₩`
22. `INF-016-U022` — `CHECK PAYMENT`
23. `INF-016-U023` — `Confirm one working payment method.`
24. `INF-016-U024` — `04`
25. `INF-016-U025` — `CHOOSE TRANSPORT`
26. `INF-016-U026` — `AREX · Bus · Taxi · Pre-booked Transfer · Rental Car`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-016-U001 | TRANSLATE | NUMBER | eyebrow | VISIBLE |
| INF-016-U002 | TRANSLATE | — | heading | VISIBLE |
| INF-016-U003 | TRANSLATE | PROPER_NOUN | intro | VISIBLE |
| INF-016-U004 | TRANSLATE | — | main sign left 1 | VISIBLE |
| INF-016-U005 | TRANSLATE | — | main sign left 2 | VISIBLE |
| INF-016-U006 | TRANSLATE | — | main sign left 3 | VISIBLE |
| INF-016-U007 | RETAIN | BRAND | main sign right 1 | VISIBLE |
| INF-016-U008 | TRANSLATE | — | main sign right 2 | VISIBLE |
| INF-016-U009 | TRANSLATE | — | main sign right 3 | VISIBLE |
| INF-016-U010 | RETAIN | SYMBOL | main sign parking icon | VISIBLE |
| INF-016-U011 | TRANSLATE | — | main sign right 4 | VISIBLE |
| INF-016-U012 | TRANSLATE | — | background information desk | VISIBLE |
| INF-016-U013 | TRANSLATE | — | foreground pillar | VISIBLE |
| INF-016-U014 | RETAIN | NUMBER | step 1 badge | VISIBLE |
| INF-016-U015 | TRANSLATE | — | step 1 heading | VISIBLE |
| INF-016-U016 | TRANSLATE | — | step 1 body | VISIBLE |
| INF-016-U017 | RETAIN | NUMBER | step 2 badge | VISIBLE |
| INF-016-U018 | TRANSLATE | — | step 2 heading | VISIBLE |
| INF-016-U019 | TRANSLATE | — | step 2 body | VISIBLE |
| INF-016-U020 | RETAIN | NUMBER | step 3 badge | VISIBLE |
| INF-016-U021 | RETAIN | SYMBOL | step 3 currency icon | VISIBLE |
| INF-016-U022 | TRANSLATE | — | step 3 heading | VISIBLE |
| INF-016-U023 | TRANSLATE | — | step 3 body | VISIBLE |
| INF-016-U024 | RETAIN | NUMBER | step 4 badge | VISIBLE |
| INF-016-U025 | TRANSLATE | — | step 4 heading | VISIBLE |
| INF-016-U026 | TRANSLATE | BRAND, SYMBOL | step 4 options | VISIBLE |

Exact units: 26; TRANSLATE 19; RETAIN 7.

## E. PROTECTION

- Preserve native 1402×1122 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- After immigration/customs; connect → save Korean hotel address → payment → transport four-step sequence.
- AREX / Bus / Taxi / Pre-booked Transfer / Rental Car options; keep one working payment method.

### Source-specific notes

- Illustrated arrivals/baggage/customs/transport/information/free Wi-Fi signs are legible and included. Korean signage remains source-language context.
- Parking P and currency ₩ icon glyphs are retained; heading's 30 minutes and numbered 01–04 remain exact.

### REVIEW_REQUIRED

1. **Illustrated background gate sign and right-hand terminal-map kiosk** — Tiny synthetic-looking background lettering is not reliably readable at native resolution. All legible main signage and editorial English are extracted; the unreadable glyphs require source/pixel review before production.

# INF-017 — Terminal arrival maps

**Asset status:** REVIEW_REQUIRED

## A. IDENTITY / SOURCE

- INF ID: `INF-017`
- Source filename: `terminal-arrival-maps.png`
- Source asset: `images/arrival/terminal-arrival-maps.png`
- Format: PNG
- Native dimensions: 1536 × 1024 px
- Current SHA-256: `6ef80718a20a726ab5f73ddae0821284d43a887b7f2dbfd6f23f963e058735dd`
- Inherited audit SHA-256: `6ef80718a20a726ab5f73ddae0821284d43a887b7f2dbfd6f23f963e058735dd`
- SHA comparison: MATCH
- Inherited audit dimensions: 1536 × 1024 px; comparison: MATCH
- Priority: P2
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `arrival.html`, image reference line 284
- Usage location: Terminal orientation / .arrival-terminal-map
- Existing wrapper: `<figure class="arrival-terminal-map">`
- English alt (reference only, not an embedded image unit): `Combined Incheon Airport Terminal 1 and Terminal 2 arrival maps showing immigration, baggage claim, customs, arrival halls, transport and service locations`
- English caption (reference only): `Terminal 1 and Terminal 2 arrival maps. Use them for orientation; live airport signs and flight information take priority.
              Open the terminal map at full size ↗`
- Thai page: `th/arrival.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `terminal-arrival-maps-th.png`
- Planned Thai path: `images/arrival/terminal-arrival-maps-th.png`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/arrival/terminal-arrival-maps-es.png` — EXISTS
  - Sibling `es/arrival.html`; page EXISTS; image reference `../images/arrival/terminal-arrival-maps-es.png`
- JA precedent asset: `images/arrival/terminal-arrival-maps-ja.png` — EXISTS
  - Sibling `ja/arrival.html`; page EXISTS; image reference `../images/arrival/terminal-arrival-maps-ja.png`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-017-U001` — `Terminal 1 Arrival Map`
2. `INF-017-U002` — `eSIM/SIM`
3. `INF-017-U003` — `1–50`
4. `INF-017-U004` — `Gates 1–50`
5. `INF-017-U005` — `101–132`
6. `INF-017-U006` — `Gates 101–132`
7. `INF-017-U007` — `3F`
8. `INF-017-U008` — `12`
9. `INF-017-U009` — `11`
10. `INF-017-U010` — `9`
11. `INF-017-U011` — `7`
12. `INF-017-U012` — `5`
13. `INF-017-U013` — `14`
14. `INF-017-U014` — `17`
15. `INF-017-U015` — `3`
16. `INF-017-U016` — `20`
17. `INF-017-U017` — `E`
18. `INF-017-U018` — `1F`
19. `INF-017-U019` — `D`
20. `INF-017-U020` — `10`
21. `INF-017-U021` — `10`
22. `INF-017-U022` — `1`
23. `INF-017-U023` — `12`
24. `INF-017-U024` — `1`
25. `INF-017-U025` — `6`
26. `INF-017-U026` — `1`
27. `INF-017-U027` — `1`
28. `INF-017-U028` — `eSIM/SIM`
29. `INF-017-U029` — `CU`
30. `INF-017-U030` — `ATM`
31. `INF-017-U031` — `P`
32. `INF-017-U032` — `1F H`
33. `INF-017-U033` — `1`
34. `INF-017-U034` — `1`
35. `INF-017-U035` — `6, 7, 8`
36. `INF-017-U036` — `12, 13`
37. `INF-017-U037` — `P`
38. `INF-017-U038` — `A06`
39. `INF-017-U039` — `eSIM/SIM`
40. `INF-017-U040` — `CU`
41. `INF-017-U041` — `CU, GS25`
42. `INF-017-U042` — `ATM`
43. `INF-017-U043` — `Terminal 2 Arrival Map`
44. `INF-017-U044` — `eSIM/SIM`
45. `INF-017-U045` — `201–230`
46. `INF-017-U046` — `Gates 201–230`
47. `INF-017-U047` — `231–270`
48. `INF-017-U048` — `Gates 231–270`
49. `INF-017-U049` — `20`
50. `INF-017-U050` — `17`
51. `INF-017-U051` — `15`
52. `INF-017-U052` — `10`
53. `INF-017-U053` — `8`
54. `INF-017-U054` — `6`
55. `INF-017-U055` — `2`
56. `INF-017-U056` — `Quarantine Information`
57. `INF-017-U057` — `A`
58. `INF-017-U058` — `(Information)`
59. `INF-017-U059` — `(Medical Center)`
60. `INF-017-U060` — `(Restrooms)`
61. `INF-017-U061` — `(Currency Exchange)`
62. `INF-017-U062` — `(Restrooms)`
63. `INF-017-U063` — `A`
64. `INF-017-U064` — `(Arrivals Hall A)`
65. `INF-017-U065` — `P`
66. `INF-017-U066` — `(Bus Ticket Counters)`
67. `INF-017-U067` — `(Bus Stops)`
68. `INF-017-U068` — `(Taxi Stands)`
69. `INF-017-U069` — `(Short-term Parking)`
70. `INF-017-U070` — `P`
71. `INF-017-U071` — `(Information)`
72. `INF-017-U072` — `(Medical Center)`
73. `INF-017-U073` — `(Restrooms)`
74. `INF-017-U074` — `(Currency Exchange)`
75. `INF-017-U075` — `(Bus Stops)`
76. `INF-017-U076` — `(Taxi Stands)`
77. `INF-017-U077` — `(Short-term Parking)`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-017-U001 | TRANSLATE | PROPER_NOUN, NUMBER | T1 heading | VISIBLE |
| INF-017-U002 | RETAIN | — | T1 Korean subtitle Latin token | VISIBLE |
| INF-017-U003 | RETAIN | NUMBER, SYMBOL | T1 gate Korean label left | VISIBLE |
| INF-017-U004 | TRANSLATE | NUMBER, SYMBOL | T1 English gate left | VISIBLE |
| INF-017-U005 | RETAIN | NUMBER, SYMBOL | T1 gate Korean label right | VISIBLE |
| INF-017-U006 | TRANSLATE | NUMBER, SYMBOL | T1 English gate right | VISIBLE |
| INF-017-U007 | RETAIN | NUMBER | T1 departure floor | VISIBLE |
| INF-017-U008 | RETAIN | NUMBER | T1 blue gate | VISIBLE |
| INF-017-U009 | RETAIN | NUMBER | T1 blue gate | VISIBLE |
| INF-017-U010 | RETAIN | NUMBER | T1 blue gate | VISIBLE |
| INF-017-U011 | RETAIN | NUMBER | T1 blue gate | VISIBLE |
| INF-017-U012 | RETAIN | NUMBER | T1 blue gate | VISIBLE |
| INF-017-U013 | RETAIN | NUMBER | T1 blue gate | VISIBLE |
| INF-017-U014 | RETAIN | NUMBER | T1 blue gate | VISIBLE |
| INF-017-U015 | RETAIN | NUMBER | T1 blue gate | VISIBLE |
| INF-017-U016 | RETAIN | NUMBER | T1 blue gate | VISIBLE |
| INF-017-U017 | RETAIN | PROPER_NOUN | T1 Korean arrival-hall label | VISIBLE |
| INF-017-U018 | RETAIN | NUMBER | T1 immigration floor | VISIBLE |
| INF-017-U019 | RETAIN | PROPER_NOUN | T1 Korean arrival-hall label | VISIBLE |
| INF-017-U020 | RETAIN | NUMBER | T1 entry label first numeric token | VISIBLE |
| INF-017-U021 | RETAIN | NUMBER | T1 entry label second numeric token | VISIBLE |
| INF-017-U022 | RETAIN | NUMBER | T1 left-middle entry floor token | VISIBLE |
| INF-017-U023 | RETAIN | NUMBER | T1 left-middle entry number | VISIBLE |
| INF-017-U024 | RETAIN | NUMBER | T1 right-middle entry floor token | VISIBLE |
| INF-017-U025 | RETAIN | NUMBER | T1 right-middle entry number | VISIBLE |
| INF-017-U026 | RETAIN | NUMBER | T1 right entry floor token | VISIBLE |
| INF-017-U027 | RETAIN | NUMBER | T1 right entry number | VISIBLE |
| INF-017-U028 | RETAIN | NUMBER | T1 facility | VISIBLE |
| INF-017-U029 | RETAIN | BRAND | T1 convenience-store logo | VISIBLE |
| INF-017-U030 | RETAIN | NUMBER | T1 facility | VISIBLE |
| INF-017-U031 | RETAIN | SYMBOL | T1 parking left icon | VISIBLE |
| INF-017-U032 | RETAIN | NUMBER, PROPER_NOUN | T1 parking left sublabel | VISIBLE |
| INF-017-U033 | RETAIN | NUMBER | T1 terminal name numeric token | VISIBLE |
| INF-017-U034 | RETAIN | NUMBER | T1 terminal west floor token | VISIBLE |
| INF-017-U035 | RETAIN | NUMBER, SYMBOL | T1 terminal west entry numbers | VISIBLE |
| INF-017-U036 | RETAIN | NUMBER, SYMBOL | T1 terminal west entry numbers | VISIBLE |
| INF-017-U037 | RETAIN | SYMBOL | T1 parking right icon | VISIBLE |
| INF-017-U038 | RETAIN | PROPER_NOUN, NUMBER | T1 parking right sublabel | VISIBLE |
| INF-017-U039 | RETAIN | NUMBER | T1 bottom legend | VISIBLE |
| INF-017-U040 | RETAIN | BRAND | T1 bottom legend logo | VISIBLE |
| INF-017-U041 | RETAIN | BRAND, SYMBOL, NUMBER | T1 bottom legend brand list | VISIBLE |
| INF-017-U042 | RETAIN | NUMBER | T1 bottom legend | VISIBLE |
| INF-017-U043 | TRANSLATE | PROPER_NOUN, NUMBER | T2 heading | VISIBLE |
| INF-017-U044 | RETAIN | — | T2 Korean subtitle Latin token | VISIBLE |
| INF-017-U045 | RETAIN | NUMBER, SYMBOL | T2 gate Korean label left | VISIBLE |
| INF-017-U046 | TRANSLATE | NUMBER, SYMBOL | T2 English gate left | VISIBLE |
| INF-017-U047 | RETAIN | NUMBER, SYMBOL | T2 gate Korean label right | VISIBLE |
| INF-017-U048 | TRANSLATE | NUMBER, SYMBOL | T2 English gate right | VISIBLE |
| INF-017-U049 | RETAIN | NUMBER | T2 blue gate | VISIBLE |
| INF-017-U050 | RETAIN | NUMBER | T2 blue gate | VISIBLE |
| INF-017-U051 | RETAIN | NUMBER | T2 blue gate | VISIBLE |
| INF-017-U052 | RETAIN | NUMBER | T2 blue gate | VISIBLE |
| INF-017-U053 | RETAIN | NUMBER | T2 blue gate | VISIBLE |
| INF-017-U054 | RETAIN | NUMBER | T2 blue gate | VISIBLE |
| INF-017-U055 | RETAIN | NUMBER | T2 blue gate | VISIBLE |
| INF-017-U056 | TRANSLATE | — | T2 central label | VISIBLE |
| INF-017-U057 | RETAIN | PROPER_NOUN | T2 arrival hall icon | VISIBLE |
| INF-017-U058 | TRANSLATE | — | T2 facility | VISIBLE |
| INF-017-U059 | TRANSLATE | — | T2 facility | VISIBLE |
| INF-017-U060 | TRANSLATE | — | T2 facility first | VISIBLE |
| INF-017-U061 | TRANSLATE | — | T2 facility | VISIBLE |
| INF-017-U062 | TRANSLATE | — | T2 facility second | VISIBLE |
| INF-017-U063 | RETAIN | PROPER_NOUN | T2 Korean arrival-hall label | VISIBLE |
| INF-017-U064 | TRANSLATE | PROPER_NOUN | T2 facility | VISIBLE |
| INF-017-U065 | RETAIN | SYMBOL | T2 parking icon | VISIBLE |
| INF-017-U066 | TRANSLATE | — | T2 transport | VISIBLE |
| INF-017-U067 | TRANSLATE | — | T2 transport | VISIBLE |
| INF-017-U068 | TRANSLATE | — | T2 transport | VISIBLE |
| INF-017-U069 | TRANSLATE | — | T2 transport | VISIBLE |
| INF-017-U070 | RETAIN | SYMBOL | T2 bottom parking icon | VISIBLE |
| INF-017-U071 | TRANSLATE | — | T2 bottom legend | VISIBLE |
| INF-017-U072 | TRANSLATE | — | T2 bottom legend | VISIBLE |
| INF-017-U073 | TRANSLATE | — | T2 bottom legend | VISIBLE |
| INF-017-U074 | TRANSLATE | — | T2 bottom legend | VISIBLE |
| INF-017-U075 | TRANSLATE | — | T2 bottom legend | VISIBLE |
| INF-017-U076 | TRANSLATE | — | T2 bottom legend | VISIBLE |
| INF-017-U077 | TRANSLATE | — | T2 bottom legend | VISIBLE |

Exact units: 77; TRANSLATE 24; RETAIN 53.

## E. PROTECTION

- Preserve native 1536×1024 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- T1/T2 map geometry, gates, facilities, arrows, floor levels, legend order and all parking/entry associations.
- T1 Gates 1–50 / 101–132; T2 Gates 201–230 / 231–270. Source legend CU, GS25 and ATM retained.

### Source-specific notes

- Read full Terminal 1 panel then full Terminal 2 panel. Korean-only strings are not translated here; Latin substrings and numeric tokens embedded in them are included as RETAIN.
- Source map floor/entry/parking labels and Korean legend context remain protected; no missing English counterpart is invented.
- Two ambiguous badge candidates are isolated for review. Candidate readings are not counted as exact units: "23" (T1 uncertain rightmost blue gate); "17-" (T2 uncertain pale badge)

### REVIEW_REQUIRED

1. **T1 rightmost blue gate badge and T2 far-left pale badge** — T1 badge appears 23 but stylized digits are unclear; T2 pale badge appears 17- with uncertain trailing mark. Candidate readings are separately marked; verify from source master before production.

Unconfirmed candidate readings (outside exact-unit totals; NOT Public Copy):

1. `23` — T1 uncertain rightmost blue gate.
2. `17-` — T2 uncertain pale badge.

# INF-018 — Homepage eSIM hero visual

**Asset status:** EXTRACTION COMPLETE

## A. IDENTITY / SOURCE

- INF ID: `INF-018`
- Source filename: `hero-esim.png`
- Source asset: `images/esim/hero-esim.png`
- Format: PNG
- Native dimensions: 1663 × 946 px
- Current SHA-256: `ff4a3af40f390adfd66951f0e9127d158b6c8570df323958a14bb57c89d6b7e8`
- Inherited audit SHA-256: `ff4a3af40f390adfd66951f0e9127d158b6c8570df323958a14bb57c89d6b7e8`
- SHA comparison: MATCH
- Inherited audit dimensions: 1663 × 946 px; comparison: MATCH
- Priority: P1
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `index.html`, image reference line 229
- Usage location: Homepage journey row: Have your phone ready before you need it
- Existing wrapper: No figure; existing image/card wrapper
- English alt (reference only, not an embedded image unit): `Mobile data setup for staying connected in Korea`
- English caption (reference only): NONE
- Thai page: `th/index.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `hero-esim-th.png`
- Planned Thai path: `images/esim/hero-esim-th.png`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/esim/hero-esim-es.png` — EXISTS
  - Sibling `es/index.html`; page EXISTS; image reference `../images/esim/hero-esim-es.png`
- JA precedent asset: `images/esim/hero-esim-ja.png` — EXISTS
  - Sibling `ja/index.html`; page EXISTS; image reference `../images/esim/hero-esim-ja.png`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-018-U001` — `eSIM for Korea`
2. `INF-018-U002` — `Stay Connected in Korea`
3. `INF-018-U003` — `Easy eSIM setup. Fast internet.`
4. `INF-018-U004` — `Stay connected wherever you go.`
5. `INF-018-U005` — `High Speed Data`
6. `INF-018-U006` — `Voice Calls Available`
7. `INF-018-U007` — `SMS Supported`
8. `INF-018-U008` — `Check Compatibility`
9. `INF-018-U009` — `›`
10. `INF-018-U010` — `Compare Options`
11. `INF-018-U011` — `›`
12. `INF-018-U012` — `9:41`
13. `INF-018-U013` — `KOREA`
14. `INF-018-U014` — `eSIM`
15. `INF-018-U015` — `Scan QR code to install eSIM`
16. `INF-018-U016` — `eSIM`
17. `INF-018-U017` — `Connected!`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-018-U001 | TRANSLATE | PROPER_NOUN | top pill | VISIBLE |
| INF-018-U002 | TRANSLATE | PROPER_NOUN | heading | VISIBLE |
| INF-018-U003 | TRANSLATE | — | subtitle line 1 | VISIBLE |
| INF-018-U004 | TRANSLATE | — | subtitle line 2 | VISIBLE |
| INF-018-U005 | TRANSLATE | — | feature 1 | VISIBLE |
| INF-018-U006 | TRANSLATE | — | feature 2 | VISIBLE |
| INF-018-U007 | TRANSLATE | — | feature 3 | VISIBLE |
| INF-018-U008 | TRANSLATE | — | CTA 1 | VISIBLE |
| INF-018-U009 | RETAIN | SYMBOL | CTA 1 chevron | VISIBLE |
| INF-018-U010 | TRANSLATE | — | CTA 2 | VISIBLE |
| INF-018-U011 | RETAIN | SYMBOL | CTA 2 chevron | VISIBLE |
| INF-018-U012 | RETAIN | NUMBER, SYMBOL | phone status clock | VISIBLE |
| INF-018-U013 | RETAIN | PROPER_NOUN | phone country | VISIBLE |
| INF-018-U014 | RETAIN | — | phone heading | VISIBLE |
| INF-018-U015 | TRANSLATE | — | phone instruction | VISIBLE |
| INF-018-U016 | RETAIN | — | detached chip | VISIBLE |
| INF-018-U017 | TRANSLATE | — | phone status | VISIBLE |

Exact units: 17; TRANSLATE 11; RETAIN 6.

## E. PROTECTION

- Preserve native 1663×946 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Phone compatibility/compare-options CTA purposes; high speed data / voice calls / SMS claims are copied as source, not validated or strengthened.
- 9:41 illustrative clock; QR/install/Connected states; imagery and hierarchy.

### Source-specific notes

- Phone status-bar time, on-device copy and detached eSIM chip label included. QR pattern is a retained visual element, not decoded or replaced.

### REVIEW_REQUIRED

NONE.

# INF-019 — eSIM quick decision flow

**Asset status:** EXTRACTION COMPLETE

## A. IDENTITY / SOURCE

- INF ID: `INF-019`
- Source filename: `esim-korea-quick-decision.webp`
- Source asset: `images/esim/esim-korea-quick-decision.webp`
- Format: WEBP
- Native dimensions: 1536 × 600 px
- Current SHA-256: `2fda5c641e8febb1599054e7fd9ad7044224e8acf430a245139ea29304b037e7`
- Inherited audit SHA-256: `2fda5c641e8febb1599054e7fd9ad7044224e8acf430a245139ea29304b037e7`
- SHA comparison: MATCH
- Inherited audit dimensions: 1536 × 600 px; comparison: MATCH
- Priority: P1
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `esim.html`, image reference line 361
- Usage location: eSIM/SIM/roaming comparison / .esim-infographic
- Existing wrapper: `<figure class="esim-infographic">`
- English alt (reference only, not an embedded image unit): `Korea eSIM quick decision guide comparing phone eSIM support, data-only use, Korean phone number needs, physical SIM, and roaming.`
- English caption (reference only): `Use this quick decision path as a starting point, then confirm the plan's included services and activation rules before buying.`
- Thai page: `th/esim.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `esim-korea-quick-decision-th.webp`
- Planned Thai path: `images/esim/esim-korea-quick-decision-th.webp`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/esim/esim-korea-quick-decision-es.webp` — EXISTS
  - Sibling `es/esim.html`; page EXISTS; image reference `../images/esim/esim-korea-quick-decision-es.webp`
- JA precedent asset: `images/esim/esim-korea-quick-decision-ja.webp` — EXISTS
  - Sibling `ja/esim.html`; page EXISTS; image reference `../images/esim/esim-korea-quick-decision-ja.webp`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-019-U001` — `1`
2. `INF-019-U002` — `PHONE SUPPORTS ESIM?`
3. `INF-019-U003` — `No → Physical SIM`
4. `INF-019-U004` — `→`
5. `INF-019-U005` — `2`
6. `INF-019-U006` — `DATA ONLY?`
7. `INF-019-U007` — `Yes → Data-only travel eSIM`
8. `INF-019-U008` — `→`
9. `INF-019-U009` — `3`
10. `INF-019-U010` — `NEED A KOREAN NUMBER?`
11. `INF-019-U011` — `Yes → Korean carrier tourist eSIM or SIM`
12. `INF-019-U012` — `→`
13. `INF-019-U013` — `4`
14. `INF-019-U014` — `ROAMING INSTEAD?`
15. `INF-019-U015` — `Choose → Roaming`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-019-U001 | RETAIN | NUMBER | card 1 badge | VISIBLE |
| INF-019-U002 | TRANSLATE | — | card 1 question | VISIBLE |
| INF-019-U003 | TRANSLATE | RECOMMENDATION, SYMBOL | card 1 result | VISIBLE |
| INF-019-U004 | RETAIN | SYMBOL | connector 1 to 2 | VISIBLE |
| INF-019-U005 | RETAIN | NUMBER | card 2 badge | VISIBLE |
| INF-019-U006 | TRANSLATE | — | card 2 question | VISIBLE |
| INF-019-U007 | TRANSLATE | RECOMMENDATION, SYMBOL | card 2 result | VISIBLE |
| INF-019-U008 | RETAIN | SYMBOL | connector 2 to 3 | VISIBLE |
| INF-019-U009 | RETAIN | NUMBER | card 3 badge | VISIBLE |
| INF-019-U010 | TRANSLATE | — | card 3 question | VISIBLE |
| INF-019-U011 | TRANSLATE | RECOMMENDATION, SYMBOL | card 3 result | VISIBLE |
| INF-019-U012 | RETAIN | SYMBOL | connector 3 to 4 | VISIBLE |
| INF-019-U013 | RETAIN | NUMBER | card 4 badge | VISIBLE |
| INF-019-U014 | TRANSLATE | — | card 4 question | VISIBLE |
| INF-019-U015 | TRANSLATE | RECOMMENDATION, SYMBOL | card 4 result | VISIBLE |

Exact units: 15; TRANSLATE 8; RETAIN 7.

## E. PROTECTION

- Preserve native 1536×600 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- No phone eSIM support → Physical SIM; data-only → travel eSIM; Korean number → Korean carrier tourist eSIM or SIM; choose roaming → Roaming.
- Question/result order and 1–4 labels; do not invent new telecom claims.

### Source-specific notes

- Four decision cards left-to-right; three inter-card arrow glyphs are retained visual connectors.

### REVIEW_REQUIRED

NONE.

# INF-020 — Homepage arrival/navigation/payment guide

**Asset status:** REVIEW_REQUIRED

## A. IDENTITY / SOURCE

- INF ID: `INF-020`
- Source filename: `arrival-guide.png`
- Source asset: `images/home/arrival-guide.png`
- Format: PNG
- Native dimensions: 1457 × 1080 px
- Current SHA-256: `3ac066f417301e78c6ada98552f22a4e085e916b55702370a836a333aab2d57c`
- Inherited audit SHA-256: `3ac066f417301e78c6ada98552f22a4e085e916b55702370a836a333aab2d57c`
- SHA comparison: MATCH
- Inherited audit dimensions: 1457 × 1080 px; comparison: MATCH
- Priority: P1
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `index.html`, image reference line 245
- Usage location: Homepage arrival/navigation/payment journey row
- Existing wrapper: No figure; existing image/card wrapper
- English alt (reference only, not an embedded image unit): `Airport arrival guide for first steps after landing in Korea`
- English caption (reference only): NONE
- Thai page: `th/index.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `arrival-guide-th.png`
- Planned Thai path: `images/home/arrival-guide-th.png`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/home/arrival-guide-es.png` — EXISTS
  - Sibling `es/index.html`; page EXISTS; image reference `../images/home/arrival-guide-es.png`
- JA precedent asset: `images/home/arrival-guide-ja.png` — EXISTS
  - Sibling `ja/index.html`; page EXISTS; image reference `../images/home/arrival-guide-ja.png`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-020-U001` — `Airport arrival`
2. `INF-020-U002` — `Step by step guide from landing to the city`
3. `INF-020-U003` — `10:29`
4. `INF-020-U004` — `Search here`
5. `INF-020-U005` — `Gyeongbokgung Palace`
6. `INF-020-U006` — `Myeong-dong`
7. `INF-020-U007` — `N Seoul Tower`
8. `INF-020-U008` — `Local maps`
9. `INF-020-U009` — `Apps, navigation tips and must-know info`
10. `INF-020-U010` — `CARD`
11. `INF-020-U011` — `Tmoney`
12. `INF-020-U012` — `T-money`
13. `INF-020-U013` — `How to buy, top up and use in Korea`
14. `INF-020-U014` — `WOWPASS`
15. `INF-020-U015` — `W`
16. `INF-020-U016` — `WOWPASS`
17. `INF-020-U017` — `T`
18. `INF-020-U018` — `Tmoney`
19. `INF-020-U019` — `W`
20. `INF-020-U020` — `WOWPASS`
21. `INF-020-U021` — `Payment, balance, benefits and usage guide`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-020-U001 | TRANSLATE | — | upper-left caption heading | VISIBLE |
| INF-020-U002 | TRANSLATE | — | upper-left caption body | VISIBLE |
| INF-020-U003 | RETAIN | NUMBER, SYMBOL | upper-right phone clock | VISIBLE |
| INF-020-U004 | RETAIN | — | upper-right phone search UI | VISIBLE |
| INF-020-U005 | RETAIN | PROPER_NOUN | upper-right map label | VISIBLE |
| INF-020-U006 | RETAIN | PROPER_NOUN | upper-right map label | VISIBLE |
| INF-020-U007 | RETAIN | PROPER_NOUN | upper-right map label | VISIBLE |
| INF-020-U008 | TRANSLATE | — | upper-right caption heading | VISIBLE |
| INF-020-U009 | TRANSLATE | — | upper-right caption body | VISIBLE |
| INF-020-U010 | RETAIN | — | lower-left machine label | VISIBLE |
| INF-020-U011 | RETAIN | BRAND | lower-left photographed card logo | VISIBLE |
| INF-020-U012 | RETAIN | BRAND | lower-left caption heading | VISIBLE |
| INF-020-U013 | TRANSLATE | PROPER_NOUN | lower-left caption body | VISIBLE |
| INF-020-U014 | RETAIN | BRAND | lower-right background kiosk logo | VISIBLE |
| INF-020-U015 | RETAIN | BRAND | lower-right photographed card logo | VISIBLE |
| INF-020-U016 | RETAIN | BRAND | lower-right photographed card label | VISIBLE |
| INF-020-U017 | RETAIN | BRAND | lower-right card Tmoney mark icon | VISIBLE |
| INF-020-U018 | RETAIN | BRAND | lower-right card Tmoney mark text | VISIBLE |
| INF-020-U019 | RETAIN | BRAND | lower-right caption logo | VISIBLE |
| INF-020-U020 | RETAIN | BRAND | lower-right caption heading | VISIBLE |
| INF-020-U021 | TRANSLATE | — | lower-right caption body | VISIBLE |

Exact units: 21; TRANSLATE 6; RETAIN 15.

## E. PROTECTION

- Preserve native 1457×1080 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Four feature roles: airport arrival / local maps / T-money / WOWPASS; payment card branding and source photographed UI.
- Map place spellings Gyeongbokgung Palace / Myeong-dong / N Seoul Tower; illustrative 10:29; current navigation not edited.

### Source-specific notes

- Read four panels in row-major order; include legible phone map UI and place names. Preserve photographed product/logo English as RETAIN; editorial captions remain TRANSLATE.
- Original card spelling Tmoney (no hyphen) differs from editorial T-money; both are retained exactly by occurrence. No QR/card verification or brand replacement.

### REVIEW_REQUIRED

1. **Lower-left photographed card purple badge / machine screen; lower-right blurred kiosk UI** — Background microtext is not reliably readable. Readable Tmoney/WOWPASS/CARD marks and main captions are included; do not reconstruct blurred or synthetic glyphs.

# INF-021 — Hongdae at-a-glance map

**Asset status:** EXTRACTION COMPLETE

## A. IDENTITY / SOURCE

- INF ID: `INF-021`
- Source filename: `hongdae-at-a-glance-map.webp`
- Source asset: `images/hongdae/hongdae-at-a-glance-map.webp`
- Format: WEBP
- Native dimensions: 1672 × 941 px
- Current SHA-256: `0c3ef2afcf52b2bef155443b5684a1a8400337a4ae01ee8b3c06f17a66544b88`
- Inherited audit SHA-256: `0c3ef2afcf52b2bef155443b5684a1a8400337a4ae01ee8b3c06f17a66544b88`
- SHA comparison: MATCH
- Inherited audit dimensions: 1672 × 941 px; comparison: MATCH
- Priority: P2
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `hongdae-travel-guide.html`, image reference line 515
- Usage location: Standalone #hongdae-at-a-glance-map figure
- Existing wrapper: `<figure class="container" id="hongdae-at-a-glance-map">`
- English alt (reference only, not an embedded image unit): `Hongdae at a Glance map showing Yeonnam, central Hongdae, Sangsu, Hapjeong and Mangwon.`
- English caption (reference only): NONE
- Thai page: `th/hongdae-travel-guide.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `hongdae-at-a-glance-map-th.webp`
- Planned Thai path: `images/hongdae/hongdae-at-a-glance-map-th.webp`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/hongdae/hongdae-at-a-glance-map-es.webp` — EXISTS
  - Sibling `es/hongdae-travel-guide.html`; page EXISTS; image reference `../images/hongdae/hongdae-at-a-glance-map-es.webp`
- JA precedent asset: `images/hongdae/hongdae-at-a-glance-map-ja.webp` — EXISTS
  - Sibling `ja/hongdae-travel-guide.html`; page EXISTS; image reference `../images/hongdae/hongdae-at-a-glance-map-ja.webp`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-021-U001` — `Hongdae at a Glance`
2. `INF-021-U002` — `A simple guide to Yeonnam, central Hongdae, shopping, nightlife, and nearby neighborhoods`
3. `INF-021-U003` — `WEST`
4. `INF-021-U004` — `NORTH`
5. `INF-021-U005` — `EAST`
6. `INF-021-U006` — `Gyeongui Line Forest Park`
7. `INF-021-U007` — `Mangwon Station`
8. `INF-021-U008` — `(Line 6)`
9. `INF-021-U009` — `Yeonnam Cafés`
10. `INF-021-U010` — `Mangwon Market & Local Eats`
11. `INF-021-U011` — `Hongik Univ. Station`
12. `INF-021-U012` — `(Line 2 · AREX · Gyeongui-Jungang)`
13. `INF-021-U013` — `Central Hongdae Shopping & Cafés`
14. `INF-021-U014` — `Seogyo-dong`
15. `INF-021-U015` — `Hapjeong Station`
16. `INF-021-U016` — `(Lines 2 & 6)`
17. `INF-021-U017` — `Hapjeong Food & Stay`
18. `INF-021-U018` — `Red Road Busking & Nightlife`
19. `INF-021-U019` — `Mapo-gu`
20. `INF-021-U020` — `Sangsu Station`
21. `INF-021-U021` — `(Line 6)`
22. `INF-021-U022` — `Seogang Bridge`
23. `INF-021-U023` — `Sangsu Indie Cafés & Bars`
24. `INF-021-U024` — `Han River`
25. `INF-021-U025` — `Editorial map — not to scale`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-021-U001 | TRANSLATE | PROPER_NOUN | heading | VISIBLE |
| INF-021-U002 | TRANSLATE | PROPER_NOUN | subtitle | VISIBLE |
| INF-021-U003 | TRANSLATE | — | orientation left | VISIBLE |
| INF-021-U004 | TRANSLATE | — | orientation upper | VISIBLE |
| INF-021-U005 | TRANSLATE | — | orientation right | VISIBLE |
| INF-021-U006 | RETAIN | PROPER_NOUN | map upper park | VISIBLE |
| INF-021-U007 | RETAIN | PROPER_NOUN | map northwest station | VISIBLE |
| INF-021-U008 | TRANSLATE | NUMBER | map northwest line | VISIBLE |
| INF-021-U009 | TRANSLATE | PROPER_NOUN | map northeast café area | VISIBLE |
| INF-021-U010 | TRANSLATE | PROPER_NOUN, SYMBOL | map west area | VISIBLE |
| INF-021-U011 | RETAIN | PROPER_NOUN | map central station | VISIBLE |
| INF-021-U012 | TRANSLATE | PROPER_NOUN, BRAND, NUMBER, SYMBOL | map central transfer | VISIBLE |
| INF-021-U013 | TRANSLATE | PROPER_NOUN, SYMBOL | map east central area | VISIBLE |
| INF-021-U014 | RETAIN | PROPER_NOUN | map east background | VISIBLE |
| INF-021-U015 | RETAIN | PROPER_NOUN | map southwest station | VISIBLE |
| INF-021-U016 | TRANSLATE | NUMBER, SYMBOL | map southwest lines | VISIBLE |
| INF-021-U017 | TRANSLATE | PROPER_NOUN, SYMBOL | map southwest area | VISIBLE |
| INF-021-U018 | TRANSLATE | PROPER_NOUN, SYMBOL | map southeast area | VISIBLE |
| INF-021-U019 | RETAIN | PROPER_NOUN | map east background | VISIBLE |
| INF-021-U020 | RETAIN | PROPER_NOUN | map southeast station | VISIBLE |
| INF-021-U021 | TRANSLATE | NUMBER | map southeast line | VISIBLE |
| INF-021-U022 | RETAIN | PROPER_NOUN | map southwest bridge | VISIBLE |
| INF-021-U023 | TRANSLATE | PROPER_NOUN, SYMBOL | map southeast area | VISIBLE |
| INF-021-U024 | RETAIN | PROPER_NOUN | map south river | VISIBLE |
| INF-021-U025 | TRANSLATE | SYMBOL | footer | VISIBLE |

Exact units: 25; TRANSLATE 16; RETAIN 9.

## E. PROTECTION

- Preserve native 1672×941 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- North/west/east; Han River and Seogang Bridge; four station labels, line numbers and dotted walking relationships.
- Yeonnam/central Hongdae/Red Road/Hapjeong/Sangsu/Mangwon role associations; footer not-to-scale caveat.

### Source-specific notes

- Editorial area map; only drawn text is extracted. Preserve proper names within TRANSLATE labels; line numbers/transfer links remain fixed.
- Orientation/map geometry are source editorial schematic, explicitly not to scale.

### REVIEW_REQUIRED

NONE.

# INF-022 — Jamsil orientation map

**Asset status:** REVIEW_REQUIRED

## A. IDENTITY / SOURCE

- INF ID: `INF-022`
- Source filename: `jamsil-at-a-glance-map.webp`
- Source asset: `images/jamsil/jamsil-at-a-glance-map.webp`
- Format: WEBP
- Native dimensions: 1536 × 1024 px
- Current SHA-256: `8ab5f95de4abf41422b9b33f456f06b7a193d64fd2f6d1d40122a2d663555eb2`
- Inherited audit SHA-256: `8ab5f95de4abf41422b9b33f456f06b7a193d64fd2f6d1d40122a2d663555eb2`
- SHA comparison: MATCH
- Inherited audit dimensions: 1536 × 1024 px; comparison: MATCH
- Priority: P2
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `jamsil-travel-guide.html`, image reference line 406
- Usage location: Jamsil at a Glance / #jamsil-at-a-glance-map
- Existing wrapper: `<figure class="container" id="jamsil-at-a-glance-map">`
- English alt (reference only, not an embedded image unit): `Orientation map of Jamsil showing the Lotte World and Seokchon Lake cluster between Jamsil Sports Complex and Olympic Park.`
- English caption (reference only): `Jamsil works best as one main cluster with separate west and east branches. Orientation map — not to scale.`
- Thai page: `th/jamsil-travel-guide.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `jamsil-at-a-glance-map-th.webp`
- Planned Thai path: `images/jamsil/jamsil-at-a-glance-map-th.webp`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/jamsil/jamsil-at-a-glance-map-es.webp` — EXISTS
  - Sibling `es/jamsil-travel-guide.html`; page EXISTS; image reference `../images/jamsil/jamsil-at-a-glance-map-es.webp`
- JA precedent asset: `images/jamsil/jamsil-at-a-glance-map-ja.webp` — EXISTS
  - Sibling `ja/jamsil-travel-guide.html`; page EXISTS; image reference `../images/jamsil/jamsil-at-a-glance-map-ja.webp`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-022-U001` — `WEST`
2. `INF-022-U002` — `CENTER — MAIN CLUSTER`
3. `INF-022-U003` — `EAST`
4. `INF-022-U004` — `Jamsil Station`
5. `INF-022-U005` — `Lines 2 / 8`
6. `INF-022-U006` — `Jamsil Sports Complex`
7. `INF-022-U007` — `Sports Complex Station`
8. `INF-022-U008` — `Lines 2 / 9`
9. `INF-022-U009` — `Lotte World Adventure`
10. `INF-022-U010` — `Lotte World Tower / Mall`
11. `INF-022-U011` — `Lotte World Aquarium`
12. `INF-022-U012` — `Olympic Park`
13. `INF-022-U013` — `KSPO Dome`
14. `INF-022-U014` — `Olympic Park Station`
15. `INF-022-U015` — `Lines 5 / 9`
16. `INF-022-U016` — `Seokchon Lake`
17. `INF-022-U017` — `Songridan-gil`
18. `INF-022-U018` — `Seokchon Lake east side`
19. `INF-022-U019` — `Orientation map — not to scale`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-022-U001 | TRANSLATE | — | orientation west | VISIBLE |
| INF-022-U002 | TRANSLATE | SYMBOL | orientation center | VISIBLE |
| INF-022-U003 | TRANSLATE | — | orientation east | VISIBLE |
| INF-022-U004 | RETAIN | PROPER_NOUN | center station | VISIBLE |
| INF-022-U005 | TRANSLATE | NUMBER, SYMBOL | center lines | VISIBLE |
| INF-022-U006 | RETAIN | PROPER_NOUN | west place | VISIBLE |
| INF-022-U007 | RETAIN | PROPER_NOUN | west station | VISIBLE |
| INF-022-U008 | TRANSLATE | NUMBER, SYMBOL | west lines | VISIBLE |
| INF-022-U009 | RETAIN | PROPER_NOUN, BRAND | center left attraction | VISIBLE |
| INF-022-U010 | RETAIN | PROPER_NOUN, BRAND, SYMBOL | center right attraction | VISIBLE |
| INF-022-U011 | RETAIN | PROPER_NOUN, BRAND | center right attraction | VISIBLE |
| INF-022-U012 | RETAIN | PROPER_NOUN | east place | VISIBLE |
| INF-022-U013 | RETAIN | PROPER_NOUN | east place | VISIBLE |
| INF-022-U014 | RETAIN | PROPER_NOUN | east station | VISIBLE |
| INF-022-U015 | TRANSLATE | NUMBER, SYMBOL | east lines | VISIBLE |
| INF-022-U016 | RETAIN | PROPER_NOUN | center lake | VISIBLE |
| INF-022-U017 | RETAIN | PROPER_NOUN | center east street | VISIBLE |
| INF-022-U018 | TRANSLATE | PROPER_NOUN | street location subtitle | VISIBLE |
| INF-022-U019 | TRANSLATE | SYMBOL | footer | VISIBLE |

Exact units: 19; TRANSLATE 8; RETAIN 11.

## E. PROTECTION

- Preserve native 1536×1024 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Sports Complex Lines 2 / 9; Jamsil Lines 2 / 8; Olympic Park Lines 5 / 9.
- West/central/east associations, Lotte attraction grouping, Songridan-gil on Seokchon Lake east side and not-to-scale footer.

### Source-specific notes

- Three geographic groups: west → main cluster → east. No separate title/subtitle is printed; filename is not an embedded heading.

### REVIEW_REQUIRED

1. **Central mall building illustration above Lotte World Tower / Mall caption** — A tiny façade wordmark is not reliably legible; no hidden brand letters are invented. All main editorial map labels are legible and extracted.

# INF-023 — Seongsu at-a-glance map

**Asset status:** EXTRACTION COMPLETE

## A. IDENTITY / SOURCE

- INF ID: `INF-023`
- Source filename: `seongsu-at-a-glance-map.webp`
- Source asset: `images/seongsu/seongsu-at-a-glance-map.webp`
- Format: WEBP
- Native dimensions: 1376 × 768 px
- Current SHA-256: `02810e427bb7f812b50338c70fb733e169fca444f713bb1b2c51bd60f79a8f04`
- Inherited audit SHA-256: `02810e427bb7f812b50338c70fb733e169fca444f713bb1b2c51bd60f79a8f04`
- SHA comparison: MATCH
- Inherited audit dimensions: 1376 × 768 px; comparison: MATCH
- Priority: P2
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `seongsu-travel-guide.html`, image reference line 385
- Usage location: Standalone #seongsu-at-a-glance-map figure
- Existing wrapper: `<figure class="container" id="seongsu-at-a-glance-map">`
- English alt (reference only, not an embedded image unit): `Illustrated Seongsu map showing Seoul Forest, Seoul Forest Station, Ttukseom Station, Seongsu Station, Yeonmujang-gil, pop-ups, cafés, and hands-on beauty areas.`
- English caption (reference only): NONE
- Thai page: `th/seongsu-travel-guide.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `seongsu-at-a-glance-map-th.webp`
- Planned Thai path: `images/seongsu/seongsu-at-a-glance-map-th.webp`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/seongsu/seongsu-at-a-glance-map-es.webp` — EXISTS
  - Sibling `es/seongsu-travel-guide.html`; page EXISTS; image reference `../images/seongsu/seongsu-at-a-glance-map-es.webp`
- JA precedent asset: `images/seongsu/seongsu-at-a-glance-map-ja.webp` — EXISTS
  - Sibling `ja/seongsu-travel-guide.html`; page EXISTS; image reference `../images/seongsu/seongsu-at-a-glance-map-ja.webp`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-023-U001` — `Seongsu at a Glance`
2. `INF-023-U002` — `A simple guide to pop-ups, cafés, hands-on beauty, and Seoul Forest`
3. `INF-023-U003` — `WEST`
4. `INF-023-U004` — `NORTH`
5. `INF-023-U005` — `EAST`
6. `INF-023-U006` — `Seoul Forest Station`
7. `INF-023-U007` — `(Suin-Bundang Line)`
8. `INF-023-U008` — `Ttukseom Station`
9. `INF-023-U009` — `(Line 2)`
10. `INF-023-U010` — `Seongsu Station`
11. `INF-023-U011` — `(Line 2)`
12. `INF-023-U012` — `Pop-Ups & Flagships`
13. `INF-023-U013` — `Yeonmujang-gil`
14. `INF-023-U014` — `Seoul Forest`
15. `INF-023-U015` — `Cafés & Industrial Alleys`
16. `INF-023-U016` — `Hands-On Beauty`
17. `INF-023-U017` — `Editorial map — not to scale`
18. `INF-023-U018` — `Han River`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-023-U001 | TRANSLATE | PROPER_NOUN | heading | VISIBLE |
| INF-023-U002 | TRANSLATE | PROPER_NOUN | subtitle | VISIBLE |
| INF-023-U003 | TRANSLATE | — | orientation | VISIBLE |
| INF-023-U004 | TRANSLATE | — | orientation | VISIBLE |
| INF-023-U005 | TRANSLATE | — | orientation | VISIBLE |
| INF-023-U006 | RETAIN | PROPER_NOUN | west station | VISIBLE |
| INF-023-U007 | RETAIN | PROPER_NOUN | west line | VISIBLE |
| INF-023-U008 | RETAIN | PROPER_NOUN | middle station | VISIBLE |
| INF-023-U009 | TRANSLATE | NUMBER | middle line | VISIBLE |
| INF-023-U010 | RETAIN | PROPER_NOUN | east station | VISIBLE |
| INF-023-U011 | TRANSLATE | NUMBER | east line | VISIBLE |
| INF-023-U012 | TRANSLATE | SYMBOL | east activity | VISIBLE |
| INF-023-U013 | RETAIN | PROPER_NOUN | central street | VISIBLE |
| INF-023-U014 | RETAIN | PROPER_NOUN | west park | VISIBLE |
| INF-023-U015 | TRANSLATE | SYMBOL | central activity | VISIBLE |
| INF-023-U016 | TRANSLATE | — | east activity | VISIBLE |
| INF-023-U017 | TRANSLATE | SYMBOL | footer | VISIBLE |
| INF-023-U018 | RETAIN | PROPER_NOUN | south river | VISIBLE |

Exact units: 18; TRANSLATE 11; RETAIN 7.

## E. PROTECTION

- Preserve native 1376×768 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- West/north/east, Seoul Forest Station Suin-Bundang / Ttukseom Line 2 / Seongsu Line 2.
- Yeonmujang-gil, cafés/industrial alleys, pop-ups/flagships and hands-on beauty placement; Han River; not-to-scale footer.

### Source-specific notes

- All text is editorial schematic-map lettering; proper names within labels are protected. No extra caption inferred.

### REVIEW_REQUIRED

NONE.

# INF-024 — NAVER Map language guide

**Asset status:** EXTRACTION COMPLETE

## A. IDENTITY / SOURCE

- INF ID: `INF-024`
- Source filename: `naver-map-language-guide.webp`
- Source asset: `images/naver-map-language-guide.webp`
- Format: WEBP
- Native dimensions: 1448 × 1086 px
- Current SHA-256: `cfb535c19d1d67f8f8d512df3f24d001a32fcf1b436727ed5fb9a5e5fec280d1`
- Inherited audit SHA-256: `cfb535c19d1d67f8f8d512df3f24d001a32fcf1b436727ed5fb9a5e5fec280d1`
- SHA comparison: MATCH
- Inherited audit dimensions: 1448 × 1086 px; comparison: MATCH
- Priority: P2
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `maps.html`, image reference line 393
- Usage location: NAVER app-language instructions / .maps-guide-figure
- Existing wrapper: `<figure class="maps-guide-figure">`
- English alt (reference only, not an embedded image unit): `How to change NAVER Map to English using the app settings`
- English caption (reference only): NONE
- Thai page: `th/maps.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `naver-map-language-guide-th.webp`
- Planned Thai path: `images/naver-map-language-guide-th.webp`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/naver-map-language-guide-es.webp` — EXISTS
  - Sibling `es/maps.html`; page EXISTS; image reference `../images/naver-map-language-guide-es.webp`
- JA precedent asset: `images/naver-map-language-guide-ja.webp` — EXISTS
  - Sibling `ja/maps.html`; page EXISTS; image reference `../images/naver-map-language-guide-ja.webp`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-024-U001` — `How to Change NAVER Map to English`
2. `INF-024-U002` — `Use the app settings before your trip so menus are easier to read.`
3. `INF-024-U003` — `Open the profile panel`
4. `INF-024-U004` — `SKT`
5. `INF-024-U005` — `1:37`
6. `INF-024-U006` — `33`
7. `INF-024-U007` — `Please log in.`
8. `INF-024-U008` — `Commute`
9. `INF-024-U009` — `Favorites`
10. `INF-024-U010` — `Bus`
11. `INF-024-U011` — `Subway`
12. `INF-024-U012` — `N`
13. `INF-024-U013` — `Booking`
14. `INF-024-U014` — `Order`
15. `INF-024-U015` — `Reviews`
16. `INF-024-U016` — `Coupons`
17. `INF-024-U017` — `Directions`
18. `INF-024-U018` — `Navigation`
19. `INF-024-U019` — `Subway`
20. `INF-024-U020` — `Book train tickets`
21. `INF-024-U021` — `My timeline`
22. `INF-024-U022` — `1`
23. `INF-024-U023` — `Open the profile panel`
24. `INF-024-U024` — `→`
25. `INF-024-U025` — `Go to Language/언어`
26. `INF-024-U026` — `SKT`
27. `INF-024-U027` — `1:35`
28. `INF-024-U028` — `34`
29. `INF-024-U029` — `Settings`
30. `INF-024-U030` — `Maps & Directions`
31. `INF-024-U031` — `Map settings`
32. `INF-024-U032` — `Driving navigation`
33. `INF-024-U033` — `Transit directions`
34. `INF-024-U034` — `Walking Directions`
35. `INF-024-U035` — `Use my location as start`
36. `INF-024-U036` — `Manage mobility data`
37. `INF-024-U037` — `App & Display`
38. `INF-024-U038` — `Language/언어`
39. `INF-024-U039` — `English`
40. `INF-024-U040` — `Display theme`
41. `INF-024-U041` — `Dark`
42. `INF-024-U042` — `Open with`
43. `INF-024-U043` — `Default`
44. `INF-024-U044` — `Keep screen on`
45. `INF-024-U045` — `i`
46. `INF-024-U046` — `Auto-rotate screen`
47. `INF-024-U047` — `i`
48. `INF-024-U048` — `Go to Language/언어`
49. `INF-024-U049` — `→`
50. `INF-024-U050` — `Select English and tap OK`
51. `INF-024-U051` — `SKT`
52. `INF-024-U052` — `1:35`
53. `INF-024-U053` — `34`
54. `INF-024-U054` — `Settings`
55. `INF-024-U055` — `Maps & Directions`
56. `INF-024-U056` — `Map settings`
57. `INF-024-U057` — `Language`
58. `INF-024-U058` — `Use the system language`
59. `INF-024-U059` — `English`
60. `INF-024-U060` — `Cancel`
61. `INF-024-U061` — `OK`
62. `INF-024-U062` — `Display theme`
63. `INF-024-U063` — `Dark`
64. `INF-024-U064` — `Open with`
65. `INF-024-U065` — `Default`
66. `INF-024-U066` — `Keep screen on`
67. `INF-024-U067` — `i`
68. `INF-024-U068` — `Auto-rotate screen`
69. `INF-024-U069` — `i`
70. `INF-024-U070` — `Select English and tap OK`
71. `INF-024-U071` — `Menu labels may vary slightly by app version and device.`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-024-U001 | TRANSLATE | BRAND | heading | VISIBLE |
| INF-024-U002 | TRANSLATE | — | subtitle | VISIBLE |
| INF-024-U003 | TRANSLATE | — | panel 1 top caption | VISIBLE |
| INF-024-U004 | RETAIN | BRAND | panel 1 screenshot 1 | VISIBLE |
| INF-024-U005 | RETAIN | NUMBER, SYMBOL | panel 1 screenshot 2 | VISIBLE |
| INF-024-U006 | RETAIN | NUMBER, SYMBOL | panel 1 screenshot 3 | VISIBLE |
| INF-024-U007 | RETAIN | — | panel 1 screenshot 4 | VISIBLE |
| INF-024-U008 | RETAIN | — | panel 1 screenshot 5 | VISIBLE |
| INF-024-U009 | RETAIN | — | panel 1 screenshot 6 | VISIBLE |
| INF-024-U010 | RETAIN | — | panel 1 screenshot 7 | VISIBLE |
| INF-024-U011 | RETAIN | — | panel 1 screenshot 8 | VISIBLE |
| INF-024-U012 | RETAIN | BRAND | panel 1 screenshot 9 | VISIBLE |
| INF-024-U013 | RETAIN | — | panel 1 screenshot 10 | VISIBLE |
| INF-024-U014 | RETAIN | — | panel 1 screenshot 11 | VISIBLE |
| INF-024-U015 | RETAIN | — | panel 1 screenshot 12 | VISIBLE |
| INF-024-U016 | RETAIN | — | panel 1 screenshot 13 | VISIBLE |
| INF-024-U017 | RETAIN | — | panel 1 screenshot 14 | VISIBLE |
| INF-024-U018 | RETAIN | — | panel 1 screenshot 15 | VISIBLE |
| INF-024-U019 | RETAIN | — | panel 1 screenshot 16 | VISIBLE |
| INF-024-U020 | RETAIN | — | panel 1 screenshot 17 | VISIBLE |
| INF-024-U021 | RETAIN | — | panel 1 screenshot 18 | VISIBLE |
| INF-024-U022 | RETAIN | NUMBER, SYMBOL | panel 1 screenshot 19 | VISIBLE |
| INF-024-U023 | TRANSLATE | — | panel 1 bottom caption | VISIBLE |
| INF-024-U024 | RETAIN | SYMBOL | panel connector 1 | VISIBLE |
| INF-024-U025 | TRANSLATE | — | panel 2 top caption | VISIBLE |
| INF-024-U026 | RETAIN | BRAND | panel 2 screenshot 1 | VISIBLE |
| INF-024-U027 | RETAIN | NUMBER, SYMBOL | panel 2 screenshot 2 | VISIBLE |
| INF-024-U028 | RETAIN | NUMBER, SYMBOL | panel 2 screenshot 3 | VISIBLE |
| INF-024-U029 | RETAIN | — | panel 2 screenshot 4 | VISIBLE |
| INF-024-U030 | RETAIN | SYMBOL | panel 2 screenshot 5 | VISIBLE |
| INF-024-U031 | RETAIN | — | panel 2 screenshot 6 | VISIBLE |
| INF-024-U032 | RETAIN | — | panel 2 screenshot 7 | VISIBLE |
| INF-024-U033 | RETAIN | — | panel 2 screenshot 8 | VISIBLE |
| INF-024-U034 | RETAIN | — | panel 2 screenshot 9 | VISIBLE |
| INF-024-U035 | RETAIN | — | panel 2 screenshot 10 | VISIBLE |
| INF-024-U036 | RETAIN | — | panel 2 screenshot 11 | VISIBLE |
| INF-024-U037 | RETAIN | SYMBOL | panel 2 screenshot 12 | VISIBLE |
| INF-024-U038 | RETAIN | — | panel 2 screenshot 13 | VISIBLE |
| INF-024-U039 | RETAIN | — | panel 2 screenshot 14 | VISIBLE |
| INF-024-U040 | RETAIN | — | panel 2 screenshot 15 | VISIBLE |
| INF-024-U041 | RETAIN | — | panel 2 screenshot 16 | VISIBLE |
| INF-024-U042 | RETAIN | — | panel 2 screenshot 17 | VISIBLE |
| INF-024-U043 | RETAIN | — | panel 2 screenshot 18 | VISIBLE |
| INF-024-U044 | RETAIN | — | panel 2 screenshot 19 | VISIBLE |
| INF-024-U045 | RETAIN | SYMBOL | panel 2 screenshot 20 | VISIBLE |
| INF-024-U046 | RETAIN | — | panel 2 screenshot 21 | VISIBLE |
| INF-024-U047 | RETAIN | SYMBOL | panel 2 screenshot 22 | VISIBLE |
| INF-024-U048 | TRANSLATE | — | panel 2 bottom caption | VISIBLE |
| INF-024-U049 | RETAIN | SYMBOL | panel connector 2 | VISIBLE |
| INF-024-U050 | TRANSLATE | — | panel 3 top caption | VISIBLE |
| INF-024-U051 | RETAIN | BRAND | panel 3 screenshot 1 | VISIBLE |
| INF-024-U052 | RETAIN | NUMBER, SYMBOL | panel 3 screenshot 2 | VISIBLE |
| INF-024-U053 | RETAIN | NUMBER, SYMBOL | panel 3 screenshot 3 | VISIBLE |
| INF-024-U054 | RETAIN | — | panel 3 screenshot 4 | VISIBLE |
| INF-024-U055 | RETAIN | SYMBOL | panel 3 screenshot 5 | VISIBLE |
| INF-024-U056 | RETAIN | — | panel 3 screenshot 6 | VISIBLE |
| INF-024-U057 | RETAIN | — | panel 3 screenshot 7 | VISIBLE |
| INF-024-U058 | RETAIN | — | panel 3 screenshot 8 | VISIBLE |
| INF-024-U059 | RETAIN | — | panel 3 screenshot 9 | VISIBLE |
| INF-024-U060 | RETAIN | — | panel 3 screenshot 10 | VISIBLE |
| INF-024-U061 | RETAIN | — | panel 3 screenshot 11 | VISIBLE |
| INF-024-U062 | RETAIN | — | panel 3 screenshot 12 | VISIBLE |
| INF-024-U063 | RETAIN | — | panel 3 screenshot 13 | VISIBLE |
| INF-024-U064 | RETAIN | — | panel 3 screenshot 14 | VISIBLE |
| INF-024-U065 | RETAIN | — | panel 3 screenshot 15 | VISIBLE |
| INF-024-U066 | RETAIN | — | panel 3 screenshot 16 | VISIBLE |
| INF-024-U067 | RETAIN | SYMBOL | panel 3 screenshot 17 | VISIBLE |
| INF-024-U068 | RETAIN | — | panel 3 screenshot 18 | VISIBLE |
| INF-024-U069 | RETAIN | SYMBOL | panel 3 screenshot 19 | VISIBLE |
| INF-024-U070 | TRANSLATE | — | panel 3 bottom caption | VISIBLE |
| INF-024-U071 | TRANSLATE | — | bottom caveat | VISIBLE |

Exact units: 71; TRANSLATE 9; RETAIN 62.

## E. PROTECTION

- Preserve native 1448×1086 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Change NAVER Map to English (not Thai) / settings path / selected English and OK action.
- Three-panel action order; SKT/time/battery illustrative data; UI wording/case (Walking Directions, Dark, Default).

### Source-specific notes

- NAVER screenshot UI labels, English selected-language state and English option are RETAIN exactly; do not translate UI screenshots into a language the source app does not show.
- Korean/Chinese/Japanese-only menu options remain source-screen context, not English localization units. Literal mixed Language/언어 labels are retained when inside the screenshot; editorial instructions TRANSLATE.
- UI glyphs (close X, chevrons, information i, arrows) are preserved as interface/connector symbols.
- Status-bar notification icons, toggles, chevrons and non-text icon shapes are retained visual elements, not additional editorial English. Panel 3 App & Display is occluded and is not reconstructed.

### REVIEW_REQUIRED

NONE.

# INF-025 — NAVER Map place-search guide

**Asset status:** REVIEW_REQUIRED

## A. IDENTITY / SOURCE

- INF ID: `INF-025`
- Source filename: `naver-map-place-search-guide.webp`
- Source asset: `images/naver-map-place-search-guide.webp`
- Format: WEBP
- Native dimensions: 1448 × 1086 px
- Current SHA-256: `c46af1a31417440f3964113188c305c7ae166267a36f8af481eb230a04180abc`
- Inherited audit SHA-256: `c46af1a31417440f3964113188c305c7ae166267a36f8af481eb230a04180abc`
- SHA comparison: MATCH
- Inherited audit dimensions: 1448 × 1086 px; comparison: MATCH
- Priority: P2
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `maps.html`, image reference line 420
- Usage location: NAVER place-result comparison / .maps-guide-figure
- Existing wrapper: `<figure class="maps-guide-figure">`
- English alt (reference only, not an embedded image unit): `How to search NAVER Map and compare similar place results`
- English caption (reference only): NONE
- Thai page: `th/maps.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `naver-map-place-search-guide-th.webp`
- Planned Thai path: `images/naver-map-place-search-guide-th.webp`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/naver-map-place-search-guide-es.webp` — EXISTS
  - Sibling `es/maps.html`; page EXISTS; image reference `../images/naver-map-place-search-guide-es.webp`
- JA precedent asset: `images/naver-map-place-search-guide-ja.webp` — EXISTS
  - Sibling `ja/maps.html`; page EXISTS; image reference `../images/naver-map-place-search-guide-ja.webp`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-025-U001` — `How to Search and Confirm the Right Place`
2. `INF-025-U002` — `Compare similar results before you choose a station, branch or entrance.`
3. `INF-025-U003` — `1. Type the English name`
4. `INF-025-U004` — `SKT`
5. `INF-025-U005` — `1:45`
6. `INF-025-U006` — `31`
7. `INF-025-U007` — `seoul station`
8. `INF-025-U008` — `seoulstationline1`
9. `INF-025-U009` — `Seoul station (High-Speed Train)`
10. `INF-025-U010` — `43-205 Dongja-dong Yongsan-gu Seoul`
11. `INF-025-U011` — `KTX,SRT stations`
12. `INF-025-U012` — `Seoul Station station Line1`
13. `INF-025-U013` — `73-6 Namdaemunno 5(o)-ga Jung-gu Seoul`
14. `INF-025-U014` — `Metropolitan Line 1`
15. `INF-025-U015` — `Seoul Station station Airport Railroad`
16. `INF-025-U016` — `43-205 Dongja-dong Yongsan-gu Seoul`
17. `INF-025-U017` — `Airport train`
18. `INF-025-U018` — `Seoul-forest station SuinBundang Line`
19. `INF-025-U019` — `656-436 Seongsu-dong 1(il)-ga`
20. `INF-025-U020` — `Seongdong-gu Seoul`
21. `INF-025-U021` — `Suin-bundang line`
22. `INF-025-U022` — `Type the English name`
23. `INF-025-U023` — `→`
24. `INF-025-U024` — `2. Compare similar results`
25. `INF-025-U025` — `SKT`
26. `INF-025-U026` — `1:46`
27. `INF-025-U027` — `31`
28. `INF-025-U028` — `seoul station`
29. `INF-025-U029` — `Places`
30. `INF-025-U030` — `Buses`
31. `INF-025-U031` — `Stops`
32. `INF-025-U032` — `Bongrae BBQ`
33. `INF-025-U033` — `Ongsimi Seoul Station Branch`
34. `INF-025-U034` — `FOCALPOINT`
35. `INF-025-U035` — `Syugaseukeol Seoul Station Branch`
36. `INF-025-U036` — `Seoul station (Hi`
37. `INF-025-U037` — `Speed Train)`
38. `INF-025-U038` — `Seoul Station station Airport Railroad`
39. `INF-025-U039` — `Seoul Station station Line4`
40. `INF-025-U040` — `UPPERLINE`
41. `INF-025-U041` — `GS Caltex`
42. `INF-025-U042` — `GS칼텍스`
43. `INF-025-U043` — `matsudo seoul`
44. `INF-025-U044` — `Map centered`
45. `INF-025-U045` — `Relevance`
46. `INF-025-U046` — `Seoul Station station Airport Railroad`
47. `INF-025-U047` — `Subway`
48. `INF-025-U048` — `Yongsan-gu Seoul`
49. `INF-025-U049` — `Call`
50. `INF-025-U050` — `Get Directions`
51. `INF-025-U051` — `Seoul station (High-Speed Train)`
52. `INF-025-U052` — `KTX,SRT stations`
53. `INF-025-U053` — `Open · Closes at 24:00`
54. `INF-025-U054` — `Yongsan-gu Seoul`
55. `INF-025-U055` — `Call`
56. `INF-025-U056` — `Get Directions`
57. `INF-025-U057` — `Seoul Station station Line1`
58. `INF-025-U058` — `Subway`
59. `INF-025-U059` — `Jung-gu Seoul`
60. `INF-025-U060` — `Compare similar results`
61. `INF-025-U061` — `→`
62. `INF-025-U062` — `Check before you choose`
63. `INF-025-U063` — `Match the line or place type`
64. `INF-025-U064` — `Check whether it’s Subway, KTX/SRT, Airport Railroad, etc.`
65. `INF-025-U065` — `Compare the station name carefully`
66. `INF-025-U066` — `Names can be very similar. Check each result.`
67. `INF-025-U067` — `Use the Korean name or address if needed`
68. `INF-025-U068` — `It helps you find the exact place or branch.`
69. `INF-025-U069` — `Do not rely on one result only`
70. `INF-025-U070` — `Always compare a few options before you decide.`
71. `INF-025-U071` — `If English search fails`
72. `INF-025-U072` — `Find the Korean name`
73. `INF-025-U073` — `Search in Korean or ask locally.`
74. `INF-025-U074` — `→`
75. `INF-025-U075` — `Paste it into NAVER Map`
76. `INF-025-U076` — `Open NAVER Map and paste the name.`
77. `INF-025-U077` — `→`
78. `INF-025-U078` — `Compare the results again`
79. `INF-025-U079` — `Review the options and choose carefully.`
80. `INF-025-U080` — `Always confirm the exact station or branch before you go.`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-025-U001 | TRANSLATE | — | heading | VISIBLE |
| INF-025-U002 | TRANSLATE | RECOMMENDATION | subtitle | VISIBLE |
| INF-025-U003 | TRANSLATE | NUMBER | panel 1 top | VISIBLE |
| INF-025-U004 | RETAIN | BRAND | panel 1 screenshot 1 | VISIBLE |
| INF-025-U005 | RETAIN | NUMBER | panel 1 screenshot 2 | VISIBLE |
| INF-025-U006 | RETAIN | NUMBER | panel 1 screenshot 3 | VISIBLE |
| INF-025-U007 | RETAIN | PROPER_NOUN | panel 1 screenshot 4 | VISIBLE |
| INF-025-U008 | RETAIN | NUMBER, PROPER_NOUN | panel 1 screenshot 5 | VISIBLE |
| INF-025-U009 | RETAIN | PROPER_NOUN | panel 1 screenshot 6 | VISIBLE |
| INF-025-U010 | RETAIN | NUMBER, PROPER_NOUN | panel 1 screenshot 7 | VISIBLE |
| INF-025-U011 | RETAIN | BRAND | panel 1 screenshot 8 | VISIBLE |
| INF-025-U012 | RETAIN | NUMBER, PROPER_NOUN | panel 1 screenshot 9 | VISIBLE |
| INF-025-U013 | RETAIN | NUMBER, PROPER_NOUN | panel 1 screenshot 10 | VISIBLE |
| INF-025-U014 | RETAIN | NUMBER | panel 1 screenshot 11 | VISIBLE |
| INF-025-U015 | RETAIN | PROPER_NOUN | panel 1 screenshot 12 | VISIBLE |
| INF-025-U016 | RETAIN | NUMBER, PROPER_NOUN | panel 1 screenshot 13 | VISIBLE |
| INF-025-U017 | RETAIN | — | panel 1 screenshot 14 | VISIBLE |
| INF-025-U018 | RETAIN | PROPER_NOUN | panel 1 screenshot 15 | VISIBLE |
| INF-025-U019 | RETAIN | NUMBER, PROPER_NOUN | panel 1 screenshot 16 | VISIBLE |
| INF-025-U020 | RETAIN | PROPER_NOUN | panel 1 screenshot 17 | VISIBLE |
| INF-025-U021 | RETAIN | PROPER_NOUN | panel 1 screenshot 18 | VISIBLE |
| INF-025-U022 | TRANSLATE | — | connector 1 caption | VISIBLE |
| INF-025-U023 | RETAIN | SYMBOL | connector 1 arrow | VISIBLE |
| INF-025-U024 | TRANSLATE | NUMBER | panel 2 top | VISIBLE |
| INF-025-U025 | RETAIN | BRAND | panel 2 screenshot 1 | VISIBLE |
| INF-025-U026 | RETAIN | NUMBER, SYMBOL | panel 2 screenshot 2 | VISIBLE |
| INF-025-U027 | RETAIN | NUMBER | panel 2 screenshot 3 | VISIBLE |
| INF-025-U028 | RETAIN | PROPER_NOUN | panel 2 screenshot 4 | VISIBLE |
| INF-025-U029 | RETAIN | — | panel 2 screenshot 5 | VISIBLE |
| INF-025-U030 | RETAIN | — | panel 2 screenshot 6 | VISIBLE |
| INF-025-U031 | RETAIN | — | panel 2 screenshot 7 | VISIBLE |
| INF-025-U032 | RETAIN | BRAND, PROPER_NOUN | panel 2 screenshot 8 | VISIBLE |
| INF-025-U033 | RETAIN | PROPER_NOUN | panel 2 screenshot 9 | VISIBLE |
| INF-025-U034 | RETAIN | BRAND | panel 2 screenshot 10 | VISIBLE |
| INF-025-U035 | RETAIN | BRAND, PROPER_NOUN | panel 2 screenshot 11 | VISIBLE |
| INF-025-U036 | RETAIN | PROPER_NOUN | panel 2 screenshot 15 | VISIBLE_FRAGMENT |
| INF-025-U037 | RETAIN | — | panel 2 screenshot 16 | VISIBLE_FRAGMENT |
| INF-025-U038 | RETAIN | PROPER_NOUN | panel 2 screenshot 17 | VISIBLE |
| INF-025-U039 | RETAIN | NUMBER, PROPER_NOUN | panel 2 screenshot 18 | VISIBLE |
| INF-025-U040 | RETAIN | BRAND | panel 2 screenshot 19 | VISIBLE |
| INF-025-U041 | RETAIN | BRAND | panel 2 screenshot 20 | VISIBLE |
| INF-025-U042 | RETAIN | BRAND | panel 2 screenshot 21 | VISIBLE |
| INF-025-U043 | RETAIN | BRAND, PROPER_NOUN | panel 2 screenshot 22 | VISIBLE |
| INF-025-U044 | RETAIN | — | panel 2 screenshot 23 | VISIBLE |
| INF-025-U045 | RETAIN | — | panel 2 screenshot 24 | VISIBLE |
| INF-025-U046 | RETAIN | PROPER_NOUN | panel 2 screenshot 25 | VISIBLE |
| INF-025-U047 | RETAIN | — | panel 2 screenshot 26 | VISIBLE |
| INF-025-U048 | RETAIN | PROPER_NOUN | panel 2 screenshot 27 | VISIBLE |
| INF-025-U049 | RETAIN | — | panel 2 screenshot 28 | VISIBLE |
| INF-025-U050 | RETAIN | — | panel 2 screenshot 29 | VISIBLE |
| INF-025-U051 | RETAIN | PROPER_NOUN | panel 2 screenshot 30 | VISIBLE |
| INF-025-U052 | RETAIN | BRAND | panel 2 screenshot 31 | VISIBLE |
| INF-025-U053 | RETAIN | NUMBER, SYMBOL | panel 2 screenshot 32 | VISIBLE |
| INF-025-U054 | RETAIN | PROPER_NOUN | panel 2 screenshot 33 | VISIBLE |
| INF-025-U055 | RETAIN | — | panel 2 screenshot 34 | VISIBLE |
| INF-025-U056 | RETAIN | — | panel 2 screenshot 35 | VISIBLE |
| INF-025-U057 | RETAIN | NUMBER, PROPER_NOUN | panel 2 screenshot 36 | VISIBLE |
| INF-025-U058 | RETAIN | — | panel 2 screenshot 37 | VISIBLE |
| INF-025-U059 | RETAIN | PROPER_NOUN | panel 2 screenshot 38 | VISIBLE |
| INF-025-U060 | TRANSLATE | — | connector 2 caption | VISIBLE |
| INF-025-U061 | RETAIN | SYMBOL | connector 2 arrow | VISIBLE |
| INF-025-U062 | TRANSLATE | RECOMMENDATION | right checklist | VISIBLE |
| INF-025-U063 | TRANSLATE | — | right checklist | VISIBLE |
| INF-025-U064 | TRANSLATE | BRAND | right checklist | VISIBLE |
| INF-025-U065 | TRANSLATE | RECOMMENDATION | right checklist | VISIBLE |
| INF-025-U066 | TRANSLATE | RECOMMENDATION | right checklist | VISIBLE |
| INF-025-U067 | TRANSLATE | RECOMMENDATION | right checklist | VISIBLE |
| INF-025-U068 | TRANSLATE | — | right checklist | VISIBLE |
| INF-025-U069 | TRANSLATE | RECOMMENDATION | right checklist | VISIBLE |
| INF-025-U070 | TRANSLATE | — | right checklist | VISIBLE |
| INF-025-U071 | TRANSLATE | — | lower fallback / footer | VISIBLE |
| INF-025-U072 | TRANSLATE | — | lower fallback / footer | VISIBLE |
| INF-025-U073 | TRANSLATE | — | lower fallback / footer | VISIBLE |
| INF-025-U074 | RETAIN | SYMBOL | lower fallback / footer | VISIBLE |
| INF-025-U075 | TRANSLATE | BRAND | lower fallback / footer | VISIBLE |
| INF-025-U076 | TRANSLATE | BRAND | lower fallback / footer | VISIBLE |
| INF-025-U077 | RETAIN | SYMBOL | lower fallback / footer | VISIBLE |
| INF-025-U078 | TRANSLATE | — | lower fallback / footer | VISIBLE |
| INF-025-U079 | TRANSLATE | RECOMMENDATION | lower fallback / footer | VISIBLE |
| INF-025-U080 | TRANSLATE | RECOMMENDATION | lower fallback / footer | VISIBLE |

Exact units: 80; TRANSLATE 23; RETAIN 57.

## E. PROTECTION

- Preserve native 1448×1086 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Compare line/place type, station name and Korean name/address; do not rely on one result; Korean-name fallback → paste into NAVER Map → compare again.
- Result order, source addresses, visible 24:00, 1:45/1:46, battery 31, line numbers, branch names and map associations.

### Source-specific notes

- Search query and all app UI/result/address/brand strings are RETAIN; editorial instructions are TRANSLATE. Screenshot UI is not approved Thai copy.
- Preserve exact original spacing/case differences: seoul station, seoulstationline1, Seoul station, Seoul Station, KTX,SRT.
- Occluded/truncated map candidates are isolated for review; hidden suffixes are not inferred. Candidate readings are not counted as exact units: "Seoul St." (panel 2 screenshot 12); "Seoul St." (panel 2 screenshot 13); "station G" (panel 2 screenshot 14)

### REVIEW_REQUIRED

1. **Panel 1 lowest result row clipped by phone frame; panel 2 map labels obscured by pins/controls** — Only fully legible UI strings and visible readable fragments are enumerated. Clipped/occluded result and map-name suffixes require original screenshot/master review; no expanded place/branch names are guessed.

Unconfirmed candidate readings (outside exact-unit totals; NOT Public Copy):

1. `Seoul St.` — panel 2 screenshot 12.
2. `Seoul St.` — panel 2 screenshot 13.
3. `station G` — panel 2 screenshot 14.

# INF-026 — NAVER Map route, exit, and bus guide

**Asset status:** REVIEW_REQUIRED

## A. IDENTITY / SOURCE

- INF ID: `INF-026`
- Source filename: `naver-map-route-exit-bus-guide.webp`
- Source asset: `images/naver-map-route-exit-bus-guide.webp`
- Format: WEBP
- Native dimensions: 1448 × 1086 px
- Current SHA-256: `417d8f50875ce1a768d107901f6fd63318d34bdd13a445d91d69ae6a0026c3ab`
- Inherited audit SHA-256: `417d8f50875ce1a768d107901f6fd63318d34bdd13a445d91d69ae6a0026c3ab`
- SHA comparison: MATCH
- Inherited audit dimensions: 1448 × 1086 px; comparison: MATCH
- Priority: P2
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `maps.html`, image reference line 451
- Usage location: F. Check bus stops / .maps-guide-figure
- Existing wrapper: `<figure class="maps-guide-figure">`
- English alt (reference only, not an embedded image unit): `How to compare NAVER Map routes and confirm boarding and get-off points`
- English caption (reference only): NONE
- Thai page: `th/maps.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `naver-map-route-exit-bus-guide-th.webp`
- Planned Thai path: `images/naver-map-route-exit-bus-guide-th.webp`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/naver-map-route-exit-bus-guide-es.webp` — EXISTS
  - Sibling `es/maps.html`; page EXISTS; image reference `../images/naver-map-route-exit-bus-guide-es.webp`
- JA precedent asset: `images/naver-map-route-exit-bus-guide-ja.webp` — EXISTS
  - Sibling `ja/maps.html`; page EXISTS; image reference `../images/naver-map-route-exit-bus-guide-ja.webp`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-026-U001` — `How to Check Routes, Subway Exits and Bus Stops`
2. `INF-026-U002` — `Use the exact place result, compare route types, and confirm where to get on and get off.`
3. `INF-026-U003` — `Search the exact place`
4. `INF-026-U004` — `Terminal 1 vs bus stop vs nearby stores`
5. `INF-026-U005` — `incheon airport t1`
6. `INF-026-U006` — `incheonairportt1`
7. `INF-026-U007` — `Incheon Airport (Terminal 1)`
8. `INF-026-U008` — `2851 Unseo-dong Yeongjong-gu Incheon`
9. `INF-026-U009` — `146km · Rent a car · Reviews 14`
10. `INF-026-U010` — `Incheon Int’l Airport T1 Bus Stop`
11. `INF-026-U011` — `2851 Unseo-dong Yeongjong-gu Incheon`
12. `INF-026-U012` — `146km · Bus, Stop`
13. `INF-026-U013` — `Starbucks Incheon International Airport T1 Air 4F Branch`
14. `INF-026-U014` — `2840 Unseo-dong Yeongjong-gu Incheon`
15. `INF-026-U015` — `146km · Cafe · Reviews 637`
16. `INF-026-U016` — `SHAKE SHACK INCHEON AIRPORT T1`
17. `INF-026-U017` — `2840 Unseo-dong Yeongjong-gu Incheon`
18. `INF-026-U018` — `146km · hamburger · Reviews 999+`
19. `INF-026-U019` — `Airport Railroad vs Line 2 vs other places`
20. `INF-026-U020` — `hongikuniv.station`
21. `INF-026-U021` — `Places`
22. `INF-026-U022` — `Stops`
23. `INF-026-U023` — `On the map`
24. `INF-026-U024` — `Relevance`
25. `INF-026-U025` — `Hongik Univ. station Airport Railroad`
26. `INF-026-U026` — `137km · Subway`
27. `INF-026-U027` — `172-9 Donggyo-dong Mapo-gu Seoul`
28. `INF-026-U028` — `1599-7788`
29. `INF-026-U029` — `11 entrance(s)`
30. `INF-026-U030` — `Hongik Univ. station Line2`
31. `INF-026-U031` — `137km · Subway`
32. `INF-026-U032` — `165 Donggyo-dong Mapo-gu Seoul`
33. `INF-026-U033` — `02-6110-2391`
34. `INF-026-U034` — `10 entrance(s)`
35. `INF-026-U035` — `Hongik Univ. station Gyeongui-Jungang Line`
36. `INF-026-U036` — `137km · Subway`
37. `INF-026-U037` — `190-66 Donggyo-dong Mapo-gu Seoul`
38. `INF-026-U038` — `1588-7788`
39. `INF-026-U039` — `Confirm the line, place type, and Korean address before you continue.`
40. `INF-026-U040` — `Compare the route options`
41. `INF-026-U041` — `Incheon Airport (Terminal 1)`
42. `INF-026-U042` — `Hongik Univ. station Airport Railroad`
43. `INF-026-U043` — `Entrances`
44. `INF-026-U044` — `Entrance`
45. `INF-026-U045` — `59min`
46. `INF-026-U046` — `All`
47. `INF-026-U047` — `Bus 2`
48. `INF-026-U048` — `Subway 1`
49. `INF-026-U049` — `Bus+Subway 2`
50. `INF-026-U050` — `Dep. Today 13:52`
51. `INF-026-U051` — `Best route, Include stairs`
52. `INF-026-U052` — `Best`
53. `INF-026-U053` — `59min`
54. `INF-026-U054` — `1:58 PM - 2:57 PM`
55. `INF-026-U055` — `₩4,650`
56. `INF-026-U056` — `4m`
57. `INF-026-U057` — `53m`
58. `INF-026-U058` — `Airport`
59. `INF-026-U059` — `Incheon Int’l Airport Terminal...`
60. `INF-026-U060` — `Real`
61. `INF-026-U061` — `Time`
62. `INF-026-U062` — `1min`
63. `INF-026-U063` — `Seoul Station bound | Incheon Int’l Airport C...`
64. `INF-026-U064` — `Get off`
65. `INF-026-U065` — `Hongik Univ. Station`
66. `INF-026-U066` — `GO`
67. `INF-026-U067` — `Fastest · Short transfer · Less walk`
68. `INF-026-U068` — `57min`
69. `INF-026-U069` — `1:54 PM - 2:51 PM`
70. `INF-026-U070` — `₩17,000`
71. `INF-026-U071` — `56m`
72. `INF-026-U072` — `11`
73. `INF-026-U073` — `Airport`
74. `INF-026-U074` — `Incheon Airport Arrival Lobby (1st Fl...`
75. `INF-026-U075` — `ETA`
76. `INF-026-U076` — `Bus 6011`
77. `INF-026-U077` — `1h 9min`
78. `INF-026-U078` — `2:02 PM - 3:11 PM`
79. `INF-026-U079` — `₩6,600`
80. `INF-026-U080` — `6011`
81. `INF-026-U081` — `1h 9m`
82. `INF-026-U082` — `Best route`
83. `INF-026-U083` — `Fastest route`
84. `INF-026-U084` — `Another bus option`
85. `INF-026-U085` — `Check total time, fare, and transfer type — not just the first result.`
86. `INF-026-U086` — `Open the route details`
87. `INF-026-U087` — `Fastest bus option`
88. `INF-026-U088` — `6002`
89. `INF-026-U089` — `Airport`
90. `INF-026-U090` — `No ETA`
91. `INF-026-U091` — `Past timetable`
92. `INF-026-U092` — `View more`
93. `INF-026-U093` — `Ride 3 stop(s)`
94. `INF-026-U094` — `56min`
95. `INF-026-U095` — `Get off at Hongdae Entrance`
96. `INF-026-U096` — `14801`
97. `INF-026-U097` — `Walk 54m · 1min`
98. `INF-026-U098` — `Hongik Univ. Station Seoul Metropolitan Area Airport Railroad`
99. `INF-026-U099` — `9`
100. `INF-026-U100` — `57min`
101. `INF-026-U101` — `Arr. at 2:51 PM`
102. `INF-026-U102` — `Preview`
103. `INF-026-U103` — `GO`
104. `INF-026-U104` — `Boarding point`
105. `INF-026-U105` — `Get off stop`
106. `INF-026-U106` — `Final walk`
107. `INF-026-U107` — `Best AREX option`
108. `INF-026-U108` — `Airport Railroad Get on at Incheon Int’l Airport Terminal 1 Station`
109. `INF-026-U109` — `14:03`
110. `INF-026-U110` — `Incheon Int’l Airport Cargo Termi...`
111. `INF-026-U111` — `Fast arrival: 1-2, 5-4`
112. `INF-026-U112` — `Real`
113. `INF-026-U113` — `Time`
114. `INF-026-U114` — `14:03`
115. `INF-026-U115` — `Seoul Station bound`
116. `INF-026-U116` — `Ride 10 stop(s)`
117. `INF-026-U117` — `53min`
118. `INF-026-U118` — `Get off at Hongik Univ. Station`
119. `INF-026-U119` — `11`
120. `INF-026-U120` — `Door on Left`
121. `INF-026-U121` — `59min`
122. `INF-026-U122` — `Arr. at 2:57 PM`
123. `INF-026-U123` — `Preview`
124. `INF-026-U124` — `GO`
125. `INF-026-U125` — `Boarding point`
126. `INF-026-U126` — `Get off stop`
127. `INF-026-U127` — `Correct place`
128. `INF-026-U128` — `Choose the exact place result.`
129. `INF-026-U129` — `Route type`
130. `INF-026-U130` — `Pick the route type that fits you.`
131. `INF-026-U131` — `Get-off stop`
132. `INF-026-U132` — `Confirm where to get off.`
133. `INF-026-U133` — `Final walk`
134. `INF-026-U134` — `Check the last walk to your destination.`
135. `INF-026-U135` — `Screens can vary by app version and device.`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-026-U001 | TRANSLATE | — | heading | VISIBLE |
| INF-026-U002 | TRANSLATE | RECOMMENDATION | subtitle | VISIBLE |
| INF-026-U003 | TRANSLATE | — | panel 1 heading | VISIBLE |
| INF-026-U004 | TRANSLATE | NUMBER | panel 1 upper callout | VISIBLE |
| INF-026-U005 | RETAIN | PROPER_NOUN, NUMBER | panel 1 upper screenshot 1 | VISIBLE |
| INF-026-U006 | RETAIN | PROPER_NOUN, NUMBER | panel 1 upper screenshot 2 | VISIBLE |
| INF-026-U007 | RETAIN | PROPER_NOUN, NUMBER | panel 1 upper screenshot 3 | VISIBLE |
| INF-026-U008 | RETAIN | PROPER_NOUN, NUMBER | panel 1 upper screenshot 4 | VISIBLE |
| INF-026-U009 | RETAIN | NUMBER, SYMBOL | panel 1 upper screenshot 5 | VISIBLE |
| INF-026-U010 | RETAIN | PROPER_NOUN, NUMBER | panel 1 upper screenshot 6 | VISIBLE |
| INF-026-U011 | RETAIN | PROPER_NOUN, NUMBER | panel 1 upper screenshot 7 | VISIBLE |
| INF-026-U012 | RETAIN | NUMBER, SYMBOL | panel 1 upper screenshot 8 | VISIBLE |
| INF-026-U013 | RETAIN | BRAND, PROPER_NOUN, NUMBER | panel 1 upper screenshot 9 | VISIBLE |
| INF-026-U014 | RETAIN | PROPER_NOUN, NUMBER | panel 1 upper screenshot 10 | VISIBLE |
| INF-026-U015 | RETAIN | NUMBER, SYMBOL | panel 1 upper screenshot 11 | VISIBLE |
| INF-026-U016 | RETAIN | BRAND, NUMBER | panel 1 upper screenshot 12 | VISIBLE |
| INF-026-U017 | RETAIN | PROPER_NOUN, NUMBER | panel 1 upper screenshot 13 | VISIBLE |
| INF-026-U018 | RETAIN | NUMBER, SYMBOL | panel 1 upper screenshot 14 | VISIBLE |
| INF-026-U019 | TRANSLATE | PROPER_NOUN, NUMBER | panel 1 lower callout | VISIBLE |
| INF-026-U020 | RETAIN | PROPER_NOUN | panel 1 lower screenshot 1 | VISIBLE |
| INF-026-U021 | RETAIN | — | panel 1 lower screenshot 2 | VISIBLE |
| INF-026-U022 | RETAIN | — | panel 1 lower screenshot 3 | VISIBLE |
| INF-026-U023 | RETAIN | — | panel 1 lower screenshot 4 | VISIBLE |
| INF-026-U024 | RETAIN | — | panel 1 lower screenshot 5 | VISIBLE |
| INF-026-U025 | RETAIN | PROPER_NOUN | panel 1 lower screenshot 6 | VISIBLE |
| INF-026-U026 | RETAIN | NUMBER, SYMBOL | panel 1 lower screenshot 7 | VISIBLE |
| INF-026-U027 | RETAIN | PROPER_NOUN, NUMBER | panel 1 lower screenshot 8 | VISIBLE |
| INF-026-U028 | RETAIN | NUMBER | panel 1 lower screenshot 9 | VISIBLE |
| INF-026-U029 | RETAIN | NUMBER | panel 1 lower screenshot 10 | VISIBLE |
| INF-026-U030 | RETAIN | PROPER_NOUN, NUMBER | panel 1 lower screenshot 11 | VISIBLE |
| INF-026-U031 | RETAIN | NUMBER, SYMBOL | panel 1 lower screenshot 12 | VISIBLE |
| INF-026-U032 | RETAIN | PROPER_NOUN, NUMBER | panel 1 lower screenshot 13 | VISIBLE |
| INF-026-U033 | RETAIN | NUMBER | panel 1 lower screenshot 14 | VISIBLE |
| INF-026-U034 | RETAIN | NUMBER | panel 1 lower screenshot 15 | VISIBLE |
| INF-026-U035 | RETAIN | PROPER_NOUN | panel 1 lower screenshot 16 | VISIBLE |
| INF-026-U036 | RETAIN | NUMBER, SYMBOL | panel 1 lower screenshot 17 | VISIBLE |
| INF-026-U037 | RETAIN | PROPER_NOUN, NUMBER | panel 1 lower screenshot 18 | VISIBLE |
| INF-026-U038 | RETAIN | NUMBER | panel 1 lower screenshot 19 | VISIBLE |
| INF-026-U039 | TRANSLATE | RECOMMENDATION | panel 1 bottom | VISIBLE |
| INF-026-U040 | TRANSLATE | — | panel 2 heading | VISIBLE |
| INF-026-U041 | RETAIN | PROPER_NOUN, NUMBER | panel 2 screenshot 1 | VISIBLE |
| INF-026-U042 | RETAIN | PROPER_NOUN | panel 2 screenshot 2 | VISIBLE |
| INF-026-U043 | RETAIN | — | panel 2 screenshot 3 | VISIBLE |
| INF-026-U044 | RETAIN | — | panel 2 screenshot 4 | VISIBLE |
| INF-026-U045 | RETAIN | NUMBER | panel 2 screenshot 5 | VISIBLE |
| INF-026-U046 | RETAIN | — | panel 2 screenshot 6 | VISIBLE |
| INF-026-U047 | RETAIN | NUMBER | panel 2 screenshot 7 | VISIBLE |
| INF-026-U048 | RETAIN | NUMBER | panel 2 screenshot 8 | VISIBLE |
| INF-026-U049 | RETAIN | NUMBER, SYMBOL | panel 2 screenshot 9 | VISIBLE |
| INF-026-U050 | RETAIN | NUMBER | panel 2 screenshot 10 | VISIBLE |
| INF-026-U051 | RETAIN | — | panel 2 screenshot 11 | VISIBLE |
| INF-026-U052 | RETAIN | — | panel 2 screenshot 12 | VISIBLE |
| INF-026-U053 | RETAIN | NUMBER | panel 2 screenshot 13 | VISIBLE |
| INF-026-U054 | RETAIN | NUMBER | panel 2 screenshot 14 | VISIBLE |
| INF-026-U055 | RETAIN | NUMBER, SYMBOL | panel 2 screenshot 15 | VISIBLE |
| INF-026-U056 | RETAIN | NUMBER | panel 2 screenshot 16 | VISIBLE |
| INF-026-U057 | RETAIN | NUMBER | panel 2 screenshot 17 | VISIBLE |
| INF-026-U058 | RETAIN | — | panel 2 screenshot 18 | VISIBLE |
| INF-026-U059 | RETAIN | PROPER_NOUN | panel 2 screenshot 19 | VISIBLE_TRUNCATION |
| INF-026-U060 | RETAIN | — | panel 2 screenshot 20 | VISIBLE |
| INF-026-U061 | RETAIN | — | panel 2 screenshot 21 | VISIBLE |
| INF-026-U062 | RETAIN | NUMBER | panel 2 screenshot 22 | VISIBLE |
| INF-026-U063 | RETAIN | PROPER_NOUN, SYMBOL | panel 2 screenshot 23 | VISIBLE_TRUNCATION |
| INF-026-U064 | RETAIN | — | panel 2 screenshot 24 | VISIBLE |
| INF-026-U065 | RETAIN | PROPER_NOUN | panel 2 screenshot 25 | VISIBLE |
| INF-026-U066 | RETAIN | — | panel 2 screenshot 26 | VISIBLE |
| INF-026-U067 | RETAIN | SYMBOL | panel 2 screenshot 27 | VISIBLE |
| INF-026-U068 | RETAIN | NUMBER | panel 2 screenshot 28 | VISIBLE |
| INF-026-U069 | RETAIN | NUMBER | panel 2 screenshot 29 | VISIBLE |
| INF-026-U070 | RETAIN | NUMBER, SYMBOL | panel 2 screenshot 30 | VISIBLE |
| INF-026-U071 | RETAIN | NUMBER | panel 2 screenshot 31 | VISIBLE |
| INF-026-U072 | RETAIN | NUMBER | panel 2 screenshot 32 | VISIBLE |
| INF-026-U073 | RETAIN | — | panel 2 screenshot 33 | VISIBLE |
| INF-026-U074 | RETAIN | PROPER_NOUN, NUMBER | panel 2 screenshot 34 | VISIBLE_TRUNCATION |
| INF-026-U075 | RETAIN | — | panel 2 screenshot 35 | VISIBLE |
| INF-026-U076 | RETAIN | NUMBER | panel 2 screenshot 36 | VISIBLE |
| INF-026-U077 | RETAIN | NUMBER | panel 2 screenshot 37 | VISIBLE |
| INF-026-U078 | RETAIN | NUMBER | panel 2 screenshot 38 | VISIBLE |
| INF-026-U079 | RETAIN | NUMBER, SYMBOL | panel 2 screenshot 39 | VISIBLE |
| INF-026-U080 | RETAIN | NUMBER | panel 2 screenshot 40 | VISIBLE |
| INF-026-U081 | RETAIN | NUMBER | panel 2 screenshot 41 | VISIBLE |
| INF-026-U082 | TRANSLATE | RECOMMENDATION | panel 2 editorial callout/footer | VISIBLE |
| INF-026-U083 | TRANSLATE | RECOMMENDATION | panel 2 editorial callout/footer | VISIBLE |
| INF-026-U084 | TRANSLATE | RECOMMENDATION | panel 2 editorial callout/footer | VISIBLE |
| INF-026-U085 | TRANSLATE | RECOMMENDATION, SYMBOL | panel 2 editorial callout/footer | VISIBLE |
| INF-026-U086 | TRANSLATE | — | panel 3 heading | VISIBLE |
| INF-026-U087 | TRANSLATE | RECOMMENDATION | panel 3 upper subtitle | VISIBLE |
| INF-026-U088 | RETAIN | NUMBER | panel 3 bus screenshot 1 | VISIBLE |
| INF-026-U089 | RETAIN | — | panel 3 bus screenshot 2 | VISIBLE |
| INF-026-U090 | RETAIN | — | panel 3 bus screenshot 3 | VISIBLE |
| INF-026-U091 | RETAIN | — | panel 3 bus screenshot 4 | VISIBLE |
| INF-026-U092 | RETAIN | — | panel 3 bus screenshot 5 | VISIBLE |
| INF-026-U093 | RETAIN | NUMBER | panel 3 bus screenshot 6 | VISIBLE |
| INF-026-U094 | RETAIN | NUMBER | panel 3 bus screenshot 7 | VISIBLE |
| INF-026-U095 | RETAIN | PROPER_NOUN | panel 3 bus screenshot 8 | VISIBLE |
| INF-026-U096 | RETAIN | NUMBER | panel 3 bus screenshot 9 | VISIBLE |
| INF-026-U097 | RETAIN | NUMBER, SYMBOL | panel 3 bus screenshot 10 | VISIBLE |
| INF-026-U098 | RETAIN | PROPER_NOUN | panel 3 bus screenshot 11 | VISIBLE |
| INF-026-U099 | RETAIN | NUMBER | panel 3 bus screenshot 12 | VISIBLE |
| INF-026-U100 | RETAIN | NUMBER | panel 3 bus screenshot 13 | VISIBLE |
| INF-026-U101 | RETAIN | NUMBER, SYMBOL | panel 3 bus screenshot 14 | VISIBLE |
| INF-026-U102 | RETAIN | — | panel 3 bus screenshot 15 | VISIBLE |
| INF-026-U103 | RETAIN | — | panel 3 bus screenshot 16 | VISIBLE |
| INF-026-U104 | TRANSLATE | — | panel 3 bus callout | VISIBLE |
| INF-026-U105 | TRANSLATE | — | panel 3 bus callout | VISIBLE |
| INF-026-U106 | TRANSLATE | — | panel 3 bus callout | VISIBLE |
| INF-026-U107 | TRANSLATE | BRAND, RECOMMENDATION | panel 3 lower subtitle | VISIBLE |
| INF-026-U108 | RETAIN | PROPER_NOUN, NUMBER | panel 3 AREX screenshot 1 | VISIBLE |
| INF-026-U109 | RETAIN | NUMBER, SYMBOL | panel 3 AREX screenshot 2 | VISIBLE |
| INF-026-U110 | RETAIN | PROPER_NOUN | panel 3 AREX screenshot 3 | VISIBLE_TRUNCATION |
| INF-026-U111 | RETAIN | NUMBER, SYMBOL | panel 3 AREX screenshot 4 | VISIBLE |
| INF-026-U112 | RETAIN | — | panel 3 AREX screenshot 5 | VISIBLE |
| INF-026-U113 | RETAIN | — | panel 3 AREX screenshot 6 | VISIBLE |
| INF-026-U114 | RETAIN | NUMBER, SYMBOL | panel 3 AREX screenshot 7 | VISIBLE |
| INF-026-U115 | RETAIN | PROPER_NOUN | panel 3 AREX screenshot 8 | VISIBLE |
| INF-026-U116 | RETAIN | NUMBER | panel 3 AREX screenshot 9 | VISIBLE |
| INF-026-U117 | RETAIN | NUMBER | panel 3 AREX screenshot 10 | VISIBLE |
| INF-026-U118 | RETAIN | PROPER_NOUN | panel 3 AREX screenshot 11 | VISIBLE |
| INF-026-U119 | RETAIN | NUMBER | panel 3 AREX screenshot 12 | VISIBLE |
| INF-026-U120 | RETAIN | — | panel 3 AREX screenshot 13 | VISIBLE |
| INF-026-U121 | RETAIN | NUMBER | panel 3 AREX screenshot 14 | VISIBLE |
| INF-026-U122 | RETAIN | NUMBER, SYMBOL | panel 3 AREX screenshot 15 | VISIBLE |
| INF-026-U123 | RETAIN | — | panel 3 AREX screenshot 16 | VISIBLE |
| INF-026-U124 | RETAIN | — | panel 3 AREX screenshot 17 | VISIBLE |
| INF-026-U125 | TRANSLATE | — | panel 3 AREX callout | VISIBLE |
| INF-026-U126 | TRANSLATE | — | panel 3 AREX callout | VISIBLE |
| INF-026-U127 | TRANSLATE | — | bottom legend / caveat | VISIBLE |
| INF-026-U128 | TRANSLATE | — | bottom legend / caveat | VISIBLE |
| INF-026-U129 | TRANSLATE | — | bottom legend / caveat | VISIBLE |
| INF-026-U130 | TRANSLATE | — | bottom legend / caveat | VISIBLE |
| INF-026-U131 | TRANSLATE | — | bottom legend / caveat | VISIBLE |
| INF-026-U132 | TRANSLATE | — | bottom legend / caveat | VISIBLE |
| INF-026-U133 | TRANSLATE | — | bottom legend / caveat | VISIBLE |
| INF-026-U134 | TRANSLATE | — | bottom legend / caveat | VISIBLE |
| INF-026-U135 | TRANSLATE | — | bottom legend / caveat | VISIBLE |

Exact units: 135; TRANSLATE 28; RETAIN 107.

## E. PROTECTION

- Preserve native 1448×1086 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Exact-place verification → compare route choices → inspect boarding/get-off/final-walk details.
- Incheon T1 vs bus-stop/store distinctions; Hongik AREX vs Line2/Gyeongui-Jungang; bus 6002/6011; exits 9/11; all three option times/fares and source addresses/phone numbers.
- Source numeric/time/fare claims not updated or corrected; bottom app/device variation caveat.

### Source-specific notes

- All screenshot result names, addresses, phone numbers, time/fare/route/exit values and UI labels are RETAIN. Editorial panel headings, callouts and footer explanations TRANSLATE.
- Original app values are illustrative source data, not current schedules/fares. Korean-only ad copy is preserved as screenshot context, not translated here.
- Screenshot ellipses below are literal visible truncation, not editorial abbreviation. Icons and highlighted route/exit connections remain unchanged.

### REVIEW_REQUIRED

1. **App screenshots: ellipsized result names / route labels; lower-left phone frame cropped** — Visible ellipses and readable fragments are preserved. Hidden trailing names and text beyond screen edges are not restored; use source screenshot/master review for clipped UI before localized production.

# INF-027 — T-money buy, recharge, and use guide

**Asset status:** REVIEW_REQUIRED

## A. IDENTITY / SOURCE

- INF ID: `INF-027`
- Source filename: `tmoney-buy-recharge-use.png`
- Source asset: `images/tmoney/tmoney-buy-recharge-use.png`
- Format: PNG
- Native dimensions: 1536 × 1024 px
- Current SHA-256: `c3f70b0c29d169512cb085eec1cc29fbcde78ee81055ce837eeb882b588b86d3`
- Inherited audit SHA-256: `c3f70b0c29d169512cb085eec1cc29fbcde78ee81055ce837eeb882b588b86d3`
- SHA comparison: MATCH
- Inherited audit dimensions: 1536 × 1024 px; comparison: MATCH
- Priority: P2
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `tmoney.html`, image reference line 273
- Usage location: T-money purchase and recharge visual band / .tmoney-visual-spread
- Existing wrapper: `<figure class="tmoney-editorial-figure">`
- English alt (reference only, not an embedded image unit): `Infographic showing how to buy, recharge and use T-money in Korea`
- English caption (reference only): NONE
- Thai page: `th/tmoney.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `tmoney-buy-recharge-use-th.png`
- Planned Thai path: `images/tmoney/tmoney-buy-recharge-use-th.png`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/tmoney/tmoney-buy-recharge-use-es.png` — EXISTS
  - Sibling `es/tmoney.html`; page EXISTS; image reference `../images/tmoney/tmoney-buy-recharge-use-es.png`
- JA precedent asset: `images/tmoney/tmoney-buy-recharge-use-ja.png` — EXISTS
  - Sibling `ja/tmoney.html`; page EXISTS; image reference `../images/tmoney/tmoney-buy-recharge-use-ja.png`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-027-U001` — `How to Buy & Recharge T-money in Korea`
2. `INF-027-U002` — `It’s easy! You can buy T-money at convenience stores or subway stations, and recharge it anytime.`
3. `INF-027-U003` — `TIP`
4. `INF-027-U004` — `If you plan to use public transportation, getting a T-money card is cheaper and more convenient than single-journey tickets.`
5. `INF-027-U005` — `1. How to Buy`
6. `INF-027-U006` — `GS25`
7. `INF-027-U007` — `T`
8. `INF-027-U008` — `money`
9. `INF-027-U009` — `T`
10. `INF-027-U010` — `money`
11. `INF-027-U011` — `Convenience Stores`
12. `INF-027-U012` — `Buy at CU, GS25, 7-Eleven and other stores.`
13. `INF-027-U013` — `Price: ₩2,500~₩4,000`
14. `INF-027-U014` — `Transportation Card`
15. `INF-027-U015` — `T`
16. `INF-027-U016` — `money`
17. `INF-027-U017` — `Subway Station Kiosk`
18. `INF-027-U018` — `Vending machines are available in most subway stations.`
19. `INF-027-U019` — `Price: ₩2,500~₩4,000`
20. `INF-027-U020` — `Tmoney`
21. `INF-027-U021` — `Transportation Card`
22. `INF-027-U022` — `AREX`
23. `INF-027-U023` — `You can also buy T-money at Incheon and Gimpo Airport.`
24. `INF-027-U024` — `Price: ₩3,000~₩4,000`
25. `INF-027-U025` — `2. How to Recharge`
26. `INF-027-U026` — `1`
27. `INF-027-U027` — `Place your card on the reader.`
28. `INF-027-U028` — `Card Reload Device`
29. `INF-027-U029` — `T`
30. `INF-027-U030` — `money`
31. `INF-027-U031` — `Card Reload Device`
32. `INF-027-U032` — `2`
33. `INF-027-U033` — `Select the amount to recharge.`
34. `INF-027-U034` — `Please select the amount.`
35. `INF-027-U035` — `1,000`
36. `INF-027-U036` — `2,000`
37. `INF-027-U037` — `3,000`
38. `INF-027-U038` — `5,000`
39. `INF-027-U039` — `10,000`
40. `INF-027-U040` — `20,000`
41. `INF-027-U041` — `30,000`
42. `INF-027-U042` — `30,000`
43. `INF-027-U043` — `50,000`
44. `INF-027-U044` — `Previous`
45. `INF-027-U045` — `To the beginning`
46. `INF-027-U046` — `3`
47. `INF-027-U047` — `Pay in cash or by card.`
48. `INF-027-U048` — `Cash`
49. `INF-027-U049` — `T`
50. `INF-027-U050` — `money`
51. `INF-027-U051` — `3. How to Use`
52. `INF-027-U052` — `Subway (Enter & Exit)`
53. `INF-027-U053` — `Tap your card when you enter and exit the station. (Missing the exit tap will charge the maximum fare.)`
54. `INF-027-U054` — `Bus (Board & Get Off)`
55. `INF-027-U055` — `1,450`
56. `INF-027-U056` — `Tmoney`
57. `INF-027-U057` — `Tap when boarding and tap again when getting off.`
58. `INF-027-U058` — `Transfer Discount`
59. `INF-027-U059` — `↔`
60. `INF-027-U060` — `30min`
61. `INF-027-U061` — `Free transfers within 30 minutes between bus and subway.`
62. `INF-027-U062` — `Taxi`
63. `INF-027-U063` — `4,900`
64. `INF-027-U064` — `Tmoney`
65. `INF-027-U065` — `Many taxis accept T-money. Ask the driver first: “T-money gayo?” (Can I use T-money?)`
66. `INF-027-U066` — `Check Your Balance`
67. `INF-027-U067` — `Tap your card on any subway gate or recharge machine to check your balance.`
68. `INF-027-U068` — `Balance`
69. `INF-027-U069` — `₩ 12,350`
70. `INF-027-U070` — `Refund`
71. `INF-027-U071` — `Get a refund for the remaining balance (minus a small fee of around ₩500) at subway customer centers or convenience stores.`
72. `INF-027-U072` — `Tmoney`
73. `INF-027-U073` — `Information`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-027-U001 | TRANSLATE | BRAND, PROPER_NOUN, SYMBOL | heading | VISIBLE |
| INF-027-U002 | TRANSLATE | BRAND | subtitle | VISIBLE |
| INF-027-U003 | TRANSLATE | — | top tip heading | VISIBLE |
| INF-027-U004 | TRANSLATE | BRAND, RECOMMENDATION | top tip body | VISIBLE |
| INF-027-U005 | TRANSLATE | NUMBER | buy heading | VISIBLE |
| INF-027-U006 | RETAIN | BRAND, NUMBER | buy card 1 text 1 | VISIBLE |
| INF-027-U007 | RETAIN | BRAND | buy card 1 text 2 | VISIBLE |
| INF-027-U008 | RETAIN | BRAND | buy card 1 text 3 | VISIBLE |
| INF-027-U009 | RETAIN | BRAND | buy card 1 text 4 | VISIBLE |
| INF-027-U010 | RETAIN | BRAND | buy card 1 text 5 | VISIBLE |
| INF-027-U011 | TRANSLATE | — | buy card 1 text 6 | VISIBLE |
| INF-027-U012 | TRANSLATE | BRAND, NUMBER | buy card 1 text 7 | VISIBLE |
| INF-027-U013 | TRANSLATE | NUMBER, SYMBOL | buy card 1 text 8 | VISIBLE |
| INF-027-U014 | RETAIN | — | buy card 2 text 1 | VISIBLE |
| INF-027-U015 | RETAIN | BRAND | buy card 2 text 2 | VISIBLE |
| INF-027-U016 | RETAIN | BRAND | buy card 2 text 3 | VISIBLE |
| INF-027-U017 | TRANSLATE | — | buy card 2 text 4 | VISIBLE |
| INF-027-U018 | TRANSLATE | — | buy card 2 text 5 | VISIBLE |
| INF-027-U019 | TRANSLATE | NUMBER, SYMBOL | buy card 2 text 6 | VISIBLE |
| INF-027-U020 | RETAIN | BRAND | buy card 3 text 1 | VISIBLE |
| INF-027-U021 | RETAIN | — | buy card 3 text 2 | VISIBLE |
| INF-027-U022 | RETAIN | BRAND | buy card 3 text 3 | VISIBLE |
| INF-027-U023 | TRANSLATE | BRAND, PROPER_NOUN | buy card 3 text 5 | VISIBLE |
| INF-027-U024 | TRANSLATE | NUMBER, SYMBOL | buy card 3 text 6 | VISIBLE |
| INF-027-U025 | TRANSLATE | NUMBER | recharge heading | VISIBLE |
| INF-027-U026 | RETAIN | NUMBER | recharge step 1 badge | VISIBLE |
| INF-027-U027 | TRANSLATE | — | recharge step 1 | VISIBLE |
| INF-027-U028 | RETAIN | — | recharge step 1 photo | VISIBLE |
| INF-027-U029 | RETAIN | BRAND | recharge step 1 photo | VISIBLE |
| INF-027-U030 | RETAIN | BRAND | recharge step 1 photo | VISIBLE |
| INF-027-U031 | RETAIN | — | recharge step 1 photo | VISIBLE |
| INF-027-U032 | RETAIN | NUMBER | recharge step 2 badge | VISIBLE |
| INF-027-U033 | TRANSLATE | — | recharge step 2 | VISIBLE |
| INF-027-U034 | RETAIN | — | recharge step 2 photo UI | VISIBLE |
| INF-027-U035 | RETAIN | NUMBER | recharge step 2 photo UI | VISIBLE |
| INF-027-U036 | RETAIN | NUMBER | recharge step 2 photo UI | VISIBLE |
| INF-027-U037 | RETAIN | NUMBER | recharge step 2 photo UI | VISIBLE |
| INF-027-U038 | RETAIN | NUMBER | recharge step 2 photo UI | VISIBLE |
| INF-027-U039 | RETAIN | NUMBER | recharge step 2 photo UI | VISIBLE |
| INF-027-U040 | RETAIN | NUMBER | recharge step 2 photo UI | VISIBLE |
| INF-027-U041 | RETAIN | NUMBER | recharge step 2 photo UI | VISIBLE |
| INF-027-U042 | RETAIN | NUMBER | recharge step 2 photo UI | VISIBLE |
| INF-027-U043 | RETAIN | NUMBER | recharge step 2 photo UI | VISIBLE |
| INF-027-U044 | RETAIN | — | recharge step 2 photo UI | VISIBLE |
| INF-027-U045 | RETAIN | — | recharge step 2 photo UI | VISIBLE |
| INF-027-U046 | RETAIN | NUMBER | recharge step 3 badge | VISIBLE |
| INF-027-U047 | TRANSLATE | — | recharge step 3 | VISIBLE |
| INF-027-U048 | RETAIN | — | recharge step 3 photo UI | VISIBLE |
| INF-027-U049 | RETAIN | BRAND | recharge step 3 photo UI | VISIBLE |
| INF-027-U050 | RETAIN | BRAND | recharge step 3 photo UI | VISIBLE |
| INF-027-U051 | TRANSLATE | NUMBER | use heading | VISIBLE |
| INF-027-U052 | TRANSLATE | SYMBOL | use card 1 text 1 | VISIBLE |
| INF-027-U053 | TRANSLATE | — | use card 1 text 2 | VISIBLE |
| INF-027-U054 | TRANSLATE | SYMBOL | use card 2 text 1 | VISIBLE |
| INF-027-U055 | RETAIN | NUMBER | use card 2 text 2 | VISIBLE |
| INF-027-U056 | RETAIN | BRAND | use card 2 text 3 | VISIBLE |
| INF-027-U057 | TRANSLATE | — | use card 2 text 4 | VISIBLE |
| INF-027-U058 | TRANSLATE | — | use card 3 text 1 | VISIBLE |
| INF-027-U059 | RETAIN | SYMBOL | use card 3 text 2 | VISIBLE |
| INF-027-U060 | RETAIN | NUMBER | use card 3 text 3 | VISIBLE |
| INF-027-U061 | TRANSLATE | NUMBER | use card 3 text 4 | VISIBLE |
| INF-027-U062 | TRANSLATE | — | use card 4 text 1 | VISIBLE |
| INF-027-U063 | RETAIN | NUMBER | use card 4 text 2 | VISIBLE |
| INF-027-U064 | RETAIN | BRAND | use card 4 text 3 | VISIBLE |
| INF-027-U065 | TRANSLATE | BRAND | use card 4 text 4 | VISIBLE |
| INF-027-U066 | TRANSLATE | — | balance heading | VISIBLE |
| INF-027-U067 | TRANSLATE | — | balance body | VISIBLE |
| INF-027-U068 | RETAIN | — | balance photo UI | VISIBLE |
| INF-027-U069 | RETAIN | NUMBER, SYMBOL | balance photo UI | VISIBLE |
| INF-027-U070 | TRANSLATE | — | refund heading | VISIBLE |
| INF-027-U071 | TRANSLATE | NUMBER, SYMBOL | refund body | VISIBLE |
| INF-027-U072 | RETAIN | BRAND | refund photo brand | VISIBLE |
| INF-027-U073 | RETAIN | — | refund photo sign | VISIBLE |

Exact units: 73; TRANSLATE 30; RETAIN 43.

## E. PROTECTION

- Preserve native 1536×1024 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Prices ₩2,500~₩4,000 (two occurrences), ₩3,000~₩4,000; transfer 30min/30 minutes; refund around ₩500.
- Source cash-or-card payment statement (not newly fact-checked), tap-in/out maximum fare warning, taxi ask-first qualifier.
- Machine amounts 1,000/2,000/3,000/5,000/10,000/20,000/30,000/30,000/50,000; reader 1,450 / 4,900; balance ₩12,350.

### Source-specific notes

- Three sections buy/recharge/use and balance/refund panels. Embedded photographed brand/UI English is RETAIN; explanatory editorial text TRANSLATE.
- Recharge grid includes 30,000 twice exactly as printed (row 3 col 1 and col 2); do not silently replace with an expected amount.
- Korean photo labels remain intact; English Card Reload Device / Cash / Balance / Information and numeric examples are extracted.
- Unexpected buy-card heading candidate is isolated for spelling review. Candidate readings are not counted as exact units: "Airbway Counters" (buy card 3 text 4)

### REVIEW_REQUIRED

1. **Photo UI: tiny subway kiosk/tap-reader labels and yellow cash-machine stickers** — Microtext and garbled-looking sticker letters are not reliably legible. Legible amounts/brands/button labels are included; no unseen payment restrictions or cash sticker wording is invented.
2. **Buy third card heading** — Source visibly reads “Airbway Counters” (unexpected spelling). Candidate is preserved without correcting to Airport/Airway; confirm letter-level spelling against master before Thai production.

Unconfirmed candidate readings (outside exact-unit totals; NOT Public Copy):

1. `Airbway Counters` — buy card 3 text 4.

# INF-029 — T-money recharge machine SVG

**Asset status:** REVIEW_REQUIRED

## A. IDENTITY / SOURCE

- INF ID: `INF-029`
- Source filename: `tmoney-recharge-machine.svg`
- Source asset: `images/tmoney/tmoney-recharge-machine.svg`
- Format: SVG
- Native dimensions: 900 × 560 px
- Current SHA-256: `22d48c8f650ae4a66eaab2d5632036e0849e9404b54b66d6c14e940ffb335ab9`
- Inherited audit SHA-256: `22d48c8f650ae4a66eaab2d5632036e0849e9404b54b66d6c14e940ffb335ab9`
- SHA comparison: MATCH
- Inherited audit dimensions: 900 × 560 px; comparison: MATCH
- Priority: P2
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `tmoney.html`, image reference line 276
- Usage location: T-money purchase and recharge visual band / .tmoney-visual-spread
- Existing wrapper: `<figure class="tmoney-editorial-figure">`
- English alt (reference only, not an embedded image unit): `T-money recharge machine guide`
- English caption (reference only): NONE
- Thai page: `th/tmoney.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `tmoney-recharge-machine-th.svg`
- Planned Thai path: `images/tmoney/tmoney-recharge-machine-th.svg`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/tmoney/tmoney-recharge-machine-es.svg` — EXISTS
  - Sibling `es/tmoney.html`; page EXISTS; image reference `../images/tmoney/tmoney-recharge-machine-es.svg`
- JA precedent asset: `images/tmoney/tmoney-recharge-machine-ja.svg` — EXISTS
  - Sibling `ja/tmoney.html`; page EXISTS; image reference `../images/tmoney/tmoney-recharge-machine-ja.svg`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-029-U001` — `T-money Recharge Machine`
2. `INF-029-U002` — `Choose English • Place card • Add cash • Check balance`
3. `INF-029-U003` — `CARD AREA`
4. `INF-029-U004` — `CASH SLOT`
5. `INF-029-U005` — `Tip: If a machine does not accept foreign cards, use cash or recharge at a convenience store.`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-029-U001 | TRANSLATE | BRAND | SVG text x450 y145 | EXACT_XML |
| INF-029-U002 | TRANSLATE | SYMBOL | SVG text x450 y178 | EXACT_XML |
| INF-029-U003 | TRANSLATE | — | SVG text x280 y294 | EXACT_XML |
| INF-029-U004 | TRANSLATE | — | SVG text x620 y294 | EXACT_XML |
| INF-029-U005 | TRANSLATE | RECOMMENDATION | SVG text x620 y455 | EXACT_XML — VISIBLE EXTENT REVIEW_REQUIRED |

Exact units: 5; TRANSLATE 5; RETAIN 0.

## E. PROTECTION

- Preserve native 900×560 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- 900×560 canvas, CARD AREA/CASH SLOT regions, four-step English/card/cash/balance sequence, foreign-card conditional convenience-store fallback.
- Original machine schematic gradients/colors/positions/visual hierarchy; no current HTML or SVG edited.

### Source-specific notes

- Exact UTF-8 SVG text nodes extracted directly from source XML, in document/visual order. No raster OCR or rendering required.
- SVG tip is a single text node at x=620, font-size=20; text can extend past native 900 px canvas. Exact XML string included in full; future layout must retain source dimensions and fit approved copy without omission.

### REVIEW_REQUIRED

1. **SVG bottom tip at x=620, y=455 / canvas 900×560** — Complete text is exact XML, but the single long centered text node has no wrapping and may extend outside the 900-pixel canvas. Pixel-visible coverage is not established for the tip; inspect rendering/layout before production.

# INF-031 — WOWPASS composite guide

**Asset status:** REVIEW_REQUIRED

## A. IDENTITY / SOURCE

- INF ID: `INF-031`
- Source filename: `wowpass-guide.png`
- Source asset: `images/wowpass/wowpass-guide.png`
- Format: PNG
- Native dimensions: 1536 × 1024 px
- Current SHA-256: `7f14f670bf1b23316f7108a710de40796f92cb1094cccfddda9ab72d3957fe60`
- Inherited audit SHA-256: `7f14f670bf1b23316f7108a710de40796f92cb1094cccfddda9ab72d3957fe60`
- SHA comparison: MATCH
- Inherited audit dimensions: 1536 × 1024 px; comparison: MATCH
- Priority: P2
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `wowpass.html`, image reference line 117
- Usage location: WOWPASS lead visual / .wowpass-lead-visual
- Existing wrapper: `<figure class="wowpass-editorial-figure wowpass-editorial-figure--wide">`
- English alt (reference only, not an embedded image unit): `WOWPASS card, machine and usage flow guide for foreign visitors in Korea`
- English caption (reference only): `The payment side and the transportation side share the same piece of plastic, but they do not share the same balance.`
- Thai page: `th/wowpass.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `wowpass-guide-th.png`
- Planned Thai path: `images/wowpass/wowpass-guide-th.png`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/wowpass/wowpass-guide-es.png` — EXISTS
  - Sibling `es/wowpass.html`; page EXISTS; image reference `../images/wowpass/wowpass-guide-es.png`
- JA precedent asset: `images/wowpass/wowpass-guide-ja.png` — EXISTS
  - Sibling `ja/wowpass.html`; page EXISTS; image reference `../images/wowpass/wowpass-guide-ja.png`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-031-U001` — `wowpass-card.png`
2. `INF-031-U002` — `W`
3. `INF-031-U003` — `WOWPASS`
4. `INF-031-U004` — `T`
5. `INF-031-U005` — `Tmoney`
6. `INF-031-U006` — `PREPAID CARD`
7. `INF-031-U007` — `WOWPASS All-in-One Prepaid Card for Travelers`
8. `INF-031-U008` — `Transportation`
9. `INF-031-U009` — `Use on subway, bus, AREX and more`
10. `INF-031-U010` — `Payments`
11. `INF-031-U011` — `Pay at stores, cafés, and convenience stores`
12. `INF-031-U012` — `$ ↔ ₩`
13. `INF-031-U013` — `Currency Exchange`
14. `INF-031-U014` — `Exchange foreign currency and use in Korea`
15. `INF-031-U015` — `wowpass-machine.png`
16. `INF-031-U016` — `WOWPASS`
17. `INF-031-U017` — `WOWPASS`
18. `INF-031-U018` — `ALL-IN-ONE PREPAID CARD`
19. `INF-031-U019` — `TOUCH TO START`
20. `INF-031-U020` — `ENGLISH`
21. `INF-031-U021` — `WOWPASS CARD SALES & TOP-UP`
22. `INF-031-U022` — `RECEIPT`
23. `INF-031-U023` — `CARD`
24. `INF-031-U024` — `CASH (KRW)`
25. `INF-031-U025` — `W`
26. `INF-031-U026` — `WOWPASS`
27. `INF-031-U027` — `All-in-One Prepaid Card for Travelers in Korea`
28. `INF-031-U028` — `$ ↔ ₩`
29. `INF-031-U029` — `wowpass-use-flow.png`
30. `INF-031-U030` — `1`
31. `INF-031-U031` — `Get Card`
32. `INF-031-U032` — `W`
33. `INF-031-U033` — `WOWPASS`
34. `INF-031-U034` — `T`
35. `INF-031-U035` — `Tmoney`
36. `INF-031-U036` — `PREPAID CARD`
37. `INF-031-U037` — `W`
38. `INF-031-U038` — `Get your WOWPASS card at airport machines or partner locations.`
39. `INF-031-U039` — `→`
40. `INF-031-U040` — `2`
41. `INF-031-U041` — `Load Money / Exchange Currency`
42. `INF-031-U042` — `$`
43. `INF-031-U043` — `€`
44. `INF-031-U044` — `¥`
45. `INF-031-U045` — `→`
46. `INF-031-U046` — `₩`
47. `INF-031-U047` — `Load Korean won (KRW) or exchange foreign currency onto your card.`
48. `INF-031-U048` — `→`
49. `INF-031-U049` — `3`
50. `INF-031-U050` — `Pay`
51. `INF-031-U051` — `W`
52. `INF-031-U052` — `WOWPASS`
53. `INF-031-U053` — `T`
54. `INF-031-U054` — `Tmoney`
55. `INF-031-U055` — `Use your card to pay at stores, cafés and convenience stores.`
56. `INF-031-U056` — `→`
57. `INF-031-U057` — `4`
58. `INF-031-U058` — `Use Transportation`
59. `INF-031-U059` — `Tap your card on subway, bus, AREX and other transportation.`
60. `INF-031-U060` — `→`
61. `INF-031-U061` — `5`
62. `INF-031-U062` — `Check Balance / Refund`
63. `INF-031-U063` — `BALANCE`
64. `INF-031-U064` — `W 30,000`
65. `INF-031-U065` — `W`
66. `INF-031-U066` — `Check balance in the app or at machines and get a refund if needed.`
67. `INF-031-U067` — `TIP`
68. `INF-031-U068` — `You can top up, check balance and get a refund at WOWPASS machines.`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-031-U001 | RETAIN | — | upper-left filename | VISIBLE |
| INF-031-U002 | RETAIN | BRAND | card logo | VISIBLE |
| INF-031-U003 | RETAIN | BRAND | card brand | VISIBLE |
| INF-031-U004 | RETAIN | BRAND | card Tmoney logo glyph | VISIBLE |
| INF-031-U005 | RETAIN | BRAND | card Tmoney logo text | VISIBLE |
| INF-031-U006 | RETAIN | — | card product inscription | VISIBLE |
| INF-031-U007 | TRANSLATE | BRAND | benefits heading | VISIBLE |
| INF-031-U008 | TRANSLATE | — | benefit 1 heading | VISIBLE |
| INF-031-U009 | TRANSLATE | BRAND | benefit 1 body | VISIBLE |
| INF-031-U010 | TRANSLATE | — | benefit 2 heading | VISIBLE |
| INF-031-U011 | TRANSLATE | — | benefit 2 body | VISIBLE |
| INF-031-U012 | RETAIN | SYMBOL | benefit 3 icon | VISIBLE |
| INF-031-U013 | TRANSLATE | — | benefit 3 heading | VISIBLE |
| INF-031-U014 | TRANSLATE | PROPER_NOUN | benefit 3 body | VISIBLE |
| INF-031-U015 | RETAIN | — | upper-right filename | VISIBLE |
| INF-031-U016 | RETAIN | BRAND | machine top brand | VISIBLE |
| INF-031-U017 | RETAIN | BRAND | machine screen brand | VISIBLE |
| INF-031-U018 | RETAIN | — | machine screen product inscription | VISIBLE |
| INF-031-U019 | RETAIN | — | machine screen button | VISIBLE |
| INF-031-U020 | RETAIN | — | machine screen language button | VISIBLE |
| INF-031-U021 | RETAIN | BRAND, SYMBOL | machine side inscription | VISIBLE |
| INF-031-U022 | RETAIN | — | machine side slot label | VISIBLE |
| INF-031-U023 | RETAIN | — | machine side slot label | VISIBLE |
| INF-031-U024 | RETAIN | — | machine side slot label | VISIBLE |
| INF-031-U025 | RETAIN | BRAND | machine lower brand logo | VISIBLE |
| INF-031-U026 | RETAIN | BRAND | banner brand | VISIBLE |
| INF-031-U027 | RETAIN | PROPER_NOUN | banner product inscription | VISIBLE |
| INF-031-U028 | RETAIN | SYMBOL | banner currency icon | VISIBLE |
| INF-031-U029 | RETAIN | — | lower filename | VISIBLE |
| INF-031-U030 | RETAIN | NUMBER | flow 1 badge | VISIBLE |
| INF-031-U031 | TRANSLATE | — | flow 1 text 1 | VISIBLE |
| INF-031-U032 | RETAIN | BRAND | flow 1 text 2 | VISIBLE |
| INF-031-U033 | RETAIN | BRAND | flow 1 text 3 | VISIBLE |
| INF-031-U034 | RETAIN | BRAND | flow 1 text 4 | VISIBLE |
| INF-031-U035 | RETAIN | BRAND | flow 1 text 5 | VISIBLE |
| INF-031-U036 | RETAIN | — | flow 1 text 6 | VISIBLE |
| INF-031-U037 | RETAIN | BRAND | flow 1 text 7 | VISIBLE |
| INF-031-U038 | TRANSLATE | BRAND | flow 1 text 8 | VISIBLE |
| INF-031-U039 | RETAIN | SYMBOL | flow connector 1 | VISIBLE |
| INF-031-U040 | RETAIN | NUMBER | flow 2 badge | VISIBLE |
| INF-031-U041 | TRANSLATE | — | flow 2 text 1 | VISIBLE |
| INF-031-U042 | RETAIN | SYMBOL | flow 2 text 2 | VISIBLE |
| INF-031-U043 | RETAIN | SYMBOL | flow 2 text 3 | VISIBLE |
| INF-031-U044 | RETAIN | SYMBOL | flow 2 text 4 | VISIBLE |
| INF-031-U045 | RETAIN | SYMBOL | flow 2 text 5 | VISIBLE |
| INF-031-U046 | RETAIN | SYMBOL | flow 2 text 6 | VISIBLE |
| INF-031-U047 | TRANSLATE | — | flow 2 text 7 | VISIBLE |
| INF-031-U048 | RETAIN | SYMBOL | flow connector 2 | VISIBLE |
| INF-031-U049 | RETAIN | NUMBER | flow 3 badge | VISIBLE |
| INF-031-U050 | TRANSLATE | — | flow 3 text 1 | VISIBLE |
| INF-031-U051 | RETAIN | BRAND | flow 3 text 2 | VISIBLE |
| INF-031-U052 | RETAIN | BRAND | flow 3 text 3 | VISIBLE |
| INF-031-U053 | RETAIN | BRAND | flow 3 text 4 | VISIBLE |
| INF-031-U054 | RETAIN | BRAND | flow 3 text 5 | VISIBLE |
| INF-031-U055 | TRANSLATE | — | flow 3 text 6 | VISIBLE |
| INF-031-U056 | RETAIN | SYMBOL | flow connector 3 | VISIBLE |
| INF-031-U057 | RETAIN | NUMBER | flow 4 badge | VISIBLE |
| INF-031-U058 | TRANSLATE | — | flow 4 text 1 | VISIBLE |
| INF-031-U059 | TRANSLATE | BRAND | flow 4 text 2 | VISIBLE |
| INF-031-U060 | RETAIN | SYMBOL | flow connector 4 | VISIBLE |
| INF-031-U061 | RETAIN | NUMBER | flow 5 badge | VISIBLE |
| INF-031-U062 | TRANSLATE | — | flow 5 text 1 | VISIBLE |
| INF-031-U063 | RETAIN | — | flow 5 text 2 | VISIBLE |
| INF-031-U064 | RETAIN | NUMBER | flow 5 text 3 | VISIBLE |
| INF-031-U065 | RETAIN | BRAND | flow 5 text 4 | VISIBLE |
| INF-031-U066 | TRANSLATE | — | flow 5 text 5 | VISIBLE |
| INF-031-U067 | TRANSLATE | — | footer tip label | VISIBLE |
| INF-031-U068 | TRANSLATE | BRAND | footer tip | VISIBLE |

Exact units: 68; TRANSLATE 19; RETAIN 49.

## E. PROTECTION

- Preserve native 1536×1024 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Get card → load KRW/exchange → pay → use transport → balance/refund sequence and source claims.
- WOWPASS versus Tmoney marks; separate currency exchange/payment/transit functions; airport/partner-location and app/machine refund claims remain source wording.
- Five step numbers; dollar/euro/yen/won glyphs; W 30,000 illustrative balance.

### Source-specific notes

- Source asset is a composite that prints three technical filenames; these filenames are RETAIN, not invented output files.
- Read upper-left card/editorial benefits; upper-right photographed machine/screen/banner; lower five-step flow and tip. Replicas have separate readable brand occurrences; no presumed hidden copy added.
- Source has “PREPAID CARD”, ALL-IN-ONE PREPAID CARD, phone W 30,000; currency/logo glyphs preserved exactly by context.
- Tiny machine-screen icon-label candidates are isolated for spelling review. Candidate readings are not counted as exact units: "TRANSPORTATION" (machine screen icon label); "PAYMENT" (machine screen icon label); "CURRENCY EXCHANGE" (machine screen icon label)

### REVIEW_REQUIRED

1. **Machine-screen tiny transport/payment/exchange labels and miniature device/card replicas in bottom flow** — Main readable English/logo occurrences are extracted. Tiny replicas and background signs cannot be verified letter-for-letter at native resolution; inspect source master before localized production. Preserve photographed product/device brand UI.

Unconfirmed candidate readings (outside exact-unit totals; NOT Public Copy):

1. `TRANSPORTATION` — machine screen icon label.
2. `PAYMENT` — machine screen icon label.
3. `CURRENCY EXCHANGE` — machine screen icon label.

# INF-033 — WOWPASS use flow

**Asset status:** REVIEW_REQUIRED

## A. IDENTITY / SOURCE

- INF ID: `INF-033`
- Source filename: `wowpass-use-flow.png`
- Source asset: `images/wowpass/wowpass-use-flow.png`
- Format: PNG
- Native dimensions: 1505 × 395 px
- Current SHA-256: `8699eac1b04ea13b43d9b319a3e877629b4af85fa09eb7c70ca1e4ff952414a7`
- Inherited audit SHA-256: `8699eac1b04ea13b43d9b319a3e877629b4af85fa09eb7c70ca1e4ff952414a7`
- SHA comparison: MATCH
- Inherited audit dimensions: 1505 × 395 px; comparison: MATCH
- Priority: P2
- Classification: LOCALIZE

## B. OCCURRENCE / THAI TARGET

- English page: `wowpass.html`, image reference line 182
- Usage location: Loading Money and Exchanging Foreign Currency / #wowpass-loading-title
- Existing wrapper: `<figure class="wowpass-editorial-figure wowpass-editorial-figure--flow">`
- English alt (reference only, not an embedded image unit): `How to get, load and use WOWPASS in Korea`
- English caption (reference only): `There are two different ideas here that are easy to blur together: exchanging foreign cash at a WOWPASS machine and adding value through the app.`
- Thai page: `th/wowpass.html`
- Thai page state: PAGE MISSING
- Thai page currently uses English source image: N/A — PAGE MISSING
- Planned Thai filename: `wowpass-use-flow-th.png`
- Planned Thai path: `images/wowpass/wowpass-use-flow-th.png`
- Planned path current state: MISSING — future asset only; not generated
- Thai translation in this extraction: NOT STARTED
- Source-path / SHA / dimension drift from inherited audit: 0
- ES precedent asset: `images/wowpass/wowpass-use-flow-es.png` — EXISTS
  - Sibling `es/wowpass.html`; page EXISTS; image reference `../images/wowpass/wowpass-use-flow-es.png`
- JA precedent asset: `images/wowpass/wowpass-use-flow-ja.png` — EXISTS
  - Sibling `ja/wowpass.html`; page EXISTS; image reference `../images/wowpass/wowpass-use-flow-ja.png`

## C. EXACT ENGLISH TEXT

Reading-order positions and mode/tag classifications are in D. Every repeated printed occurrence is numbered separately. Verbatim source strings below are the wording source; layout line breaks within a unit are normalized to spaces.

1. `INF-033-U001` — `1`
2. `INF-033-U002` — `Get Card`
3. `INF-033-U003` — `W`
4. `INF-033-U004` — `WOWPASS`
5. `INF-033-U005` — `T`
6. `INF-033-U006` — `Tmoney`
7. `INF-033-U007` — `W`
8. `INF-033-U008` — `Get your WOWPASS card at airport machines or partner locations.`
9. `INF-033-U009` — `→`
10. `INF-033-U010` — `2`
11. `INF-033-U011` — `Load Money / Exchange Currency`
12. `INF-033-U012` — `$`
13. `INF-033-U013` — `€`
14. `INF-033-U014` — `¥`
15. `INF-033-U015` — `→`
16. `INF-033-U016` — `₩`
17. `INF-033-U017` — `Load Korean won (KRW) or exchange foreign currency onto your card.`
18. `INF-033-U018` — `→`
19. `INF-033-U019` — `3`
20. `INF-033-U020` — `Pay`
21. `INF-033-U021` — `W`
22. `INF-033-U022` — `WOWPASS`
23. `INF-033-U023` — `T`
24. `INF-033-U024` — `Tmoney`
25. `INF-033-U025` — `Use your card to pay at stores, cafés and convenience stores.`
26. `INF-033-U026` — `→`
27. `INF-033-U027` — `4`
28. `INF-033-U028` — `Use Transportation`
29. `INF-033-U029` — `Tap your card on subway, bus, AREX and other transportation.`
30. `INF-033-U030` — `→`
31. `INF-033-U031` — `5`
32. `INF-033-U032` — `Check Balance / Refund`
33. `INF-033-U033` — `BALANCE`
34. `INF-033-U034` — `W 30,000`
35. `INF-033-U035` — `W`
36. `INF-033-U036` — `Check balance in the app or at machines and get a refund if needed.`
37. `INF-033-U037` — `TIP`
38. `INF-033-U038` — `You can top up, check balance and get a refund at WOWPASS machines.`

## D. UNIT CLASSIFICATION

| Unit | Mode | Tags | Visual position | Evidence |
|---|---|---|---|---|
| INF-033-U001 | RETAIN | NUMBER | step 1 badge | VISIBLE |
| INF-033-U002 | TRANSLATE | — | step 1 text 1 | VISIBLE |
| INF-033-U003 | RETAIN | BRAND | step 1 text 2 | VISIBLE |
| INF-033-U004 | RETAIN | BRAND | step 1 text 3 | VISIBLE |
| INF-033-U005 | RETAIN | BRAND | step 1 text 4 | VISIBLE |
| INF-033-U006 | RETAIN | BRAND | step 1 text 5 | VISIBLE |
| INF-033-U007 | RETAIN | BRAND | step 1 text 6 | VISIBLE |
| INF-033-U008 | TRANSLATE | BRAND | step 1 text 7 | VISIBLE |
| INF-033-U009 | RETAIN | SYMBOL | step connector 1 | VISIBLE |
| INF-033-U010 | RETAIN | NUMBER | step 2 badge | VISIBLE |
| INF-033-U011 | TRANSLATE | — | step 2 text 1 | VISIBLE |
| INF-033-U012 | RETAIN | SYMBOL | step 2 text 2 | VISIBLE |
| INF-033-U013 | RETAIN | SYMBOL | step 2 text 3 | VISIBLE |
| INF-033-U014 | RETAIN | SYMBOL | step 2 text 4 | VISIBLE |
| INF-033-U015 | RETAIN | SYMBOL | step 2 text 5 | VISIBLE |
| INF-033-U016 | RETAIN | SYMBOL | step 2 text 6 | VISIBLE |
| INF-033-U017 | TRANSLATE | — | step 2 text 7 | VISIBLE |
| INF-033-U018 | RETAIN | SYMBOL | step connector 2 | VISIBLE |
| INF-033-U019 | RETAIN | NUMBER | step 3 badge | VISIBLE |
| INF-033-U020 | TRANSLATE | — | step 3 text 1 | VISIBLE |
| INF-033-U021 | RETAIN | BRAND | step 3 text 2 | VISIBLE |
| INF-033-U022 | RETAIN | BRAND | step 3 text 3 | VISIBLE |
| INF-033-U023 | RETAIN | BRAND | step 3 text 4 | VISIBLE |
| INF-033-U024 | RETAIN | BRAND | step 3 text 5 | VISIBLE |
| INF-033-U025 | TRANSLATE | — | step 3 text 6 | VISIBLE |
| INF-033-U026 | RETAIN | SYMBOL | step connector 3 | VISIBLE |
| INF-033-U027 | RETAIN | NUMBER | step 4 badge | VISIBLE |
| INF-033-U028 | TRANSLATE | — | step 4 text 1 | VISIBLE |
| INF-033-U029 | TRANSLATE | BRAND | step 4 text 2 | VISIBLE |
| INF-033-U030 | RETAIN | SYMBOL | step connector 4 | VISIBLE |
| INF-033-U031 | RETAIN | NUMBER | step 5 badge | VISIBLE |
| INF-033-U032 | TRANSLATE | — | step 5 text 1 | VISIBLE |
| INF-033-U033 | RETAIN | — | step 5 text 2 | VISIBLE |
| INF-033-U034 | RETAIN | NUMBER | step 5 text 3 | VISIBLE |
| INF-033-U035 | RETAIN | BRAND | step 5 text 4 | VISIBLE |
| INF-033-U036 | TRANSLATE | — | step 5 text 5 | VISIBLE |
| INF-033-U037 | TRANSLATE | — | footer tip label | VISIBLE |
| INF-033-U038 | TRANSLATE | BRAND | footer tip | VISIBLE |

Exact units: 38; TRANSLATE 12; RETAIN 26.

## E. PROTECTION

- Preserve native 1505×395 dimensions, source format, visual hierarchy, panel/card/table order, route/arrow geometry, numbers, names, brands, warnings and recommendation strength.
- No Thai wording, factual update, editorial correction or new recommendation is authorized in this stage.
- Five steps and four inter-step arrows plus currency-conversion arrow; money flow/payment/transit/balance/refund distinctions.
- Source brands, airport/partner locations, Korean won (KRW), example balance 30,000; no new recommendation.

### Source-specific notes

- Standalone five-step flow is a separate physical asset; repeated captions are extracted independently, not treated as duplicate targets.
- ₩ glyph in step 2 and W 30,000 illustrative phone balance are different source representations; preserve each. Product/device logo text is RETAIN.

### REVIEW_REQUIRED

1. **Step 1 miniature machine/card and step 3 card micro-inscriptions** — Miniature screen/product inscriptions are below reliable letter-level legibility at native 1505×395. Readable WOWPASS/Tmoney/W/logo marks and main captions are included; no microtext inferred from INF-031 despite similar design.

## 7. REVIEW_REQUIRED — consolidated list

| ID | Region | Reason |
|---|---|---|
| INF-007 | Terminal 1 / diagram sign | The source sign visibly reads ARRINAL HALL, rather than the expected ARRIVAL HALL. Preserve the source spelling in extraction; editorial correction/localization approval is required before changing it. |
| INF-008 | upper left map / pickup label | Upper-left pickup heading has irregular/case-ambiguous glyphs; candidate Pick-UP is not a verified exact unit. Obtain letter-level source confirmation without silently normalizing it. |
| INF-008 | lower Korean decision panel / Jamsil column / terminal cell | The source cell reads TA, not the T2 expected from context. Preserve TA as visible; do not silently correct the original. |
| INF-008 | lower Korean decision panel / Myeongdong column | Second platform row shows 9A without a terminal code; several other lower cells are blank. Do not invent terminal/platform values. |
| INF-008 | Lower-panel illustrated hotel sign | Tiny hotel sign lettering is not reliably legible; it is not reconstructed or counted as exact English. |
| INF-012 | Top-right Korea Inside wordmark | The wordmark is clipped by the right canvas boundary. Visible fragments are recorded; do not invent cropped glyphs or silently reconstruct the brand without review. |
| INF-013 | T1/T2 embedded photographs: ticket-machine screens and distant platform signs | Microtext is below reliable legibility at native 736×1024. Legible Airport Railroad / Transportation Center / B1 signage is included; unreadable screen/sign glyphs are not guessed. These photograph/UI details require source-master or pixel review before Thai production. |
| INF-016 | Illustrated background gate sign and right-hand terminal-map kiosk | Tiny synthetic-looking background lettering is not reliably readable at native resolution. All legible main signage and editorial English are extracted; the unreadable glyphs require source/pixel review before production. |
| INF-017 | T1 rightmost blue gate badge and T2 far-left pale badge | T1 badge appears 23 but stylized digits are unclear; T2 pale badge appears 17- with uncertain trailing mark. Candidate readings are separately marked; verify from source master before production. |
| INF-020 | Lower-left photographed card purple badge / machine screen; lower-right blurred kiosk UI | Background microtext is not reliably readable. Readable Tmoney/WOWPASS/CARD marks and main captions are included; do not reconstruct blurred or synthetic glyphs. |
| INF-022 | Central mall building illustration above Lotte World Tower / Mall caption | A tiny façade wordmark is not reliably legible; no hidden brand letters are invented. All main editorial map labels are legible and extracted. |
| INF-025 | Panel 1 lowest result row clipped by phone frame; panel 2 map labels obscured by pins/controls | Only fully legible UI strings and visible readable fragments are enumerated. Clipped/occluded result and map-name suffixes require original screenshot/master review; no expanded place/branch names are guessed. |
| INF-026 | App screenshots: ellipsized result names / route labels; lower-left phone frame cropped | Visible ellipses and readable fragments are preserved. Hidden trailing names and text beyond screen edges are not restored; use source screenshot/master review for clipped UI before localized production. |
| INF-027 | Photo UI: tiny subway kiosk/tap-reader labels and yellow cash-machine stickers | Microtext and garbled-looking sticker letters are not reliably legible. Legible amounts/brands/button labels are included; no unseen payment restrictions or cash sticker wording is invented. |
| INF-027 | Buy third card heading | Source visibly reads “Airbway Counters” (unexpected spelling). Candidate is preserved without correcting to Airport/Airway; confirm letter-level spelling against master before Thai production. |
| INF-029 | SVG bottom tip at x=620, y=455 / canvas 900×560 | Complete text is exact XML, but the single long centered text node has no wrapping and may extend outside the 900-pixel canvas. Pixel-visible coverage is not established for the tip; inspect rendering/layout before production. |
| INF-031 | Machine-screen tiny transport/payment/exchange labels and miniature device/card replicas in bottom flow | Main readable English/logo occurrences are extracted. Tiny replicas and background signs cannot be verified letter-for-letter at native resolution; inspect source master before localized production. Preserve photographed product/device brand UI. |
| INF-033 | Step 1 miniature machine/card and step 3 card micro-inscriptions | Miniature screen/product inscriptions are below reliable letter-level legibility at native 1505×395. Readable WOWPASS/Tmoney/W/logo marks and main captions are included; no microtext inferred from INF-031 despite similar design. |

Review-required assets: 14 / 25; issue records: 18. Remaining 11 assets have extraction complete with no unresolved source-reading issue. All 25 assets have identity, occurrence, classified known-source units, target paths and protection records. Unknown details must be resolved without rewriting verified source units.

## 8. Thai page status list

| English page | Thai page | Current state | Asset IDs |
|---|---|---|---|
| `hongdae-vs-myeongdong.html` | `th/hongdae-vs-myeongdong.html` | EXISTS — IMPLEMENTED | INF-006 |
| `airport-bus.html` | `th/airport-bus.html` | PAGE MISSING | INF-007, INF-008, INF-009 |
| `arex.html` | `th/arex.html` | PAGE MISSING | INF-010, INF-011, INF-012, INF-013, INF-014, INF-015 |
| `airport.html` | `th/airport.html` | PAGE MISSING | INF-016 |
| `arrival.html` | `th/arrival.html` | PAGE MISSING | INF-017 |
| `index.html` | `th/index.html` | PAGE MISSING | INF-018, INF-020 |
| `esim.html` | `th/esim.html` | PAGE MISSING | INF-019 |
| `hongdae-travel-guide.html` | `th/hongdae-travel-guide.html` | PAGE MISSING | INF-021 |
| `jamsil-travel-guide.html` | `th/jamsil-travel-guide.html` | PAGE MISSING | INF-022 |
| `seongsu-travel-guide.html` | `th/seongsu-travel-guide.html` | PAGE MISSING | INF-023 |
| `maps.html` | `th/maps.html` | PAGE MISSING | INF-024, INF-025, INF-026 |
| `tmoney.html` | `th/tmoney.html` | PAGE MISSING | INF-027, INF-029 |
| `wowpass.html` | `th/wowpass.html` | PAGE MISSING | INF-031, INF-033 |

No Thai pages are created; missing pages are recorded only. Future asset filenames preserve the original extension. Existing English fallback links, CSS/JS, loading/decoding/fetchpriority, captions, alt, ARIA, srcset/picture structure, affiliate URLs and tracking remain untouched.

## 9. Final static QA and handoff

QA Level 1 — scoped document/source integrity verification only. No browser screenshots, Production HTTP checks or localized-image QA were authorized or performed.

- 25 inherited LOCALIZE IDs documented exactly once; EXCLUDE 028/030/032 have no detailed units.
- 25 source paths exist; 25 current SHA-256 values equal the audit; 25 dimensions and source formats equal the audit.
- Unit numbering is continuous within each asset; TRANSLATE + RETAIN = 1326; every tag belongs to the approved seven-category vocabulary (mode plus identity/protection tags).
- All readable extracted strings are assigned positions/classifications; uncertain candidates and unreadable regions are explicit REVIEW_REQUIRED. No invented words or 100% resolved coverage assertion.
- 25 unique planned Thai paths preserve file extensions; duplicate target paths = 0.
- Existing Thai HTML source-reference state recorded; 24 asset occurrences point to missing future Thai pages, without creating them.
- Current ES/JA local source references verified for only these 25 IDs; no new release/Production claim.
- Existing 1100 protected files byte-unchanged; intended repository delta is this MD only.
- Existing images/HTML/CSS/JS/approved MD changes = 0; stage/commit/push/deploy/Production actions = 0.
- `git diff --check` = PASS; untracked new MD also checked with `git diff --no-index --check -- NUL <new-md>` = PASS.

Next role: ChatGPT prepares Thai localized Review Copy from confirmed TRANSLATE units, retaining protected identifiers/values and resolving REVIEW_REQUIRED source details. This source-extraction MD is work material and is never an approved Thai wording Source of Truth.

**Final execution state:** THAI INFOGRAPHIC REMAINING SOURCE EXTRACTION COMPLETE WITH REVIEW_REQUIRED — NORMAL ITEMS COMPLETE

**STOP:** Authorized source-extraction task ends here. No automatic translation, image creation, HTML implementation, Git action or Production continuation.
