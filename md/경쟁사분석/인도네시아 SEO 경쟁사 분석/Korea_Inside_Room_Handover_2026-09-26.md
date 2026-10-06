# Korea Inside — Room Handover
**Date:** 2026-09-26  
**Status:** ACTIVE PROJECT HANDOVER  
**Revision:** 2026-09-26 — Home-laptop recovery + Batch 9 hreflang follow-up + Batch 10 wording approval  
**Primary purpose:** Preserve exact Korea Inside working context when moving to a new chat room.  
**Priority:** In the next room, read this file after the required Standards and before doing any new work.

---

# 0. Mandatory startup order in the next room

At the start of the new Korea Inside room:

1. Read in full:
   - `Korea_Inside_Public_Content_Master_Standard.md` — current ACTIVE v1.4
   - `Korea_Inside_Navigation_Hub_Architecture_Standard.md` — current ACTIVE v1.1

2. Because the active work is multilingual / Japanese, also read in full:
   - `Korea_Inside_Language_Localization_Standard.md` — current ACTIVE v2.1
   - `Korea_Inside_Japanese_Localization_Standard.md` — current ACTIVE v1.1

3. Then read this Handover in full:
   - `Korea_Inside_Room_Handover_2026-09-26.md`

4. Check whether a newer `Korea_Inside_Room_Handover_*.md` exists.
   - If a newer valid ACTIVE Handover exists, read the newest one and use it instead.

5. If the next task is a Travel Guide production task, also read the current ACTIVE `Korea_Inside_Travel_Guide_Production_Playbook.md` and the applicable Active Family Design Standard.

6. If the current Stay Decision Family has an ACTIVE Family Design Standard, apply it before using this Handover as state context.

Conflict order for the current Japanese work:
**Current user instruction > Public Content Master > Navigation Standard > applicable common Specialized Standard / Playbook > Language Localization Standard > Japanese Localization Standard > Active Family Design Standard > latest Handover > Master Inventory > Research Master > Approved Public Copy / CONTENT LOCKED > current Production HTML**

Handover is a state document. It does not override a newer active Standard.

Do not redesign from memory or guesswork.

---

# 1. Core Korea Inside role separation — fixed

This is the working method that has proven the most stable.

## ChatGPT
Responsible for:
- public copy
- localization
- Japanese naturalness
- Humanization
- editorial judgment
- search-intent interpretation
- recommendation wording
- final localized Review Copy creation

ChatGPT must directly read and judge the actual language.

## User
Responsible for:
- final approval
- final decision

The user does not want to manually re-audit the entire project each time.  
ChatGPT should first produce a complete result, then ask for approval.

## Codex
Responsible only for:
- technical extraction
- exact implementation of approved copy
- HTML/CSS technical application
- canonical / hreflang / sitemap
- internal links
- QA
- Git
- Production

Codex must NOT:
- translate
- localize
- humanize
- rewrite grammar
- add/delete/merge public copy
- alter recommendations
- alter hotel/area ranking
- infer missing wording

---

# 2. Normal localization workflow — fixed

Use this workflow for Batch 10 and later:

**English Production HTML**  
→ **Codex technical Source MD extraction only**  
→ **ChatGPT direct human localization**  
→ **Localized Review MD**  
→ **ChatGPT self-check for completeness**  
→ **User final wording approval ONCE**  
→ **Approved Public Copy — CONTENT LOCKED**  
→ **Codex exact implementation**  
→ **Technical QA**  
→ **Production approval ONCE**  
→ **commit / push / Vercel Production / public QA / Inventory COMPLETE**

Important:
- Do NOT repeat the Batch 8/9 pattern of many intermediate approval loops.
- Do NOT make Codex the content completeness judge.
- Completeness checking before user approval is ChatGPT's responsibility.
- Codex may still perform technical exactness QA after approval, but not editorial judgment.

---

# 3. Protection rules — fixed

Without explicit user approval, do NOT modify:
- common header
- navigation
- footer
- `common.js`
- mobile hamburger
- shared `style.css`

Preserve:
- facts
- numbers
- prices
- times
- recommendation judgments
- hotel / area rankings
- affiliate URLs
- tracking
- HTML structure
- section order
- class / id / data-*
- images / srcset
- schema structure
- CSS / JS functional logic

Protect existing user working-tree changes.

Never use unless explicitly authorized:
- `git add .`
- `git add -A`
- `git restore`
- `git reset`
- `git clean`

Current known working-tree protection after home-laptop recovery:
- the **18 Japanese MISSING working copies** under `/ja/` are currently exact English copies and untracked
- English ↔ Japanese working-copy file-size match: **18/18**
- English ↔ Japanese working-copy SHA-256 exact match: **18/18**
- Batch 10 implementation scope is only the 6 Stay Decision working copies
- the other 12 MISSING working copies must remain untouched during Batch 10
- existing COMPLETE Japanese 40 pages remain protected
- Batch 10 approved localization MD is protected as the wording Source of Truth
- any other user tracked/untracked file outside current scope remains protected

---

# 4. Current Japanese Production state

Latest confirmed Japanese Inventory before Batch 10 Production:
- **COMPLETE: 40**
- **MISSING: 18**
- **EXCLUDE: 3**

This remains the correct Inventory state until Batch 10 completes Production QA. Working-copy existence does not make a page COMPLETE.

Latest confirmed Production / Git state after the Batch 9 hreflang follow-up:
- HEAD: `020456f37287fa32e44dc49bbd91258da911fd13`
- origin/main: `020456f37287fa32e44dc49bbd91258da911fd13`
- ahead/behind: **0 / 0**
- staged: **0**
- Vercel Production: **READY**
- Batch 9 English reciprocal Japanese hreflang follow-up: **5/5 PASS**
- public English/Japanese URL checks for the 5 corrected pairs: **HTTP 200**

Working tree is intentionally not clean because the 18 recovered Japanese MISSING working copies are untracked.

---

# 5. Japanese Batch 8 — DONE / CONTENT LOCKED / Production COMPLETE

Pages:
1. `ja/gongdeok-mapo-seoul-guide.html`
2. `ja/itaewon-travel-guide.html`
3. `ja/lotte-world-seoul.html`
4. `ja/seoul-sky-guide.html`

Final coverage:
- total user-visible strings: **2,044**
- covered: **2,044 / 2,044**
- UNMAPPED: **0**
- page-specific English residue: **0**
- structure/fact/number/recommendation mismatch: **0**
- affiliate/tracking/image/srcset mismatch: **0**
- Lotte World FAQ/schema: **8/8**
- Seoul Sky FAQ/schema: **8/8**
- broken/future `/ja/` links: **0**
- reciprocal EN/ES/JA/x-default: **12/12 PASS**
- sitemap: JA 4 pages each exactly once
- public HTTP EN/ES/JA: **12/12 = 200**
- public internal links/assets: **67 checked, 0 errors**
- public JA HTML exact match: **4/4**

Commits:
- implementation/SEO: `fac951c5436e639d99e56bb234d3d95ac7bf5093`
- Inventory: `9f0766ab425346a902f2b60c2db7af06827d39bd`
- final Production SHA: `9f0766ab425346a902f2b60c2db7af06827d39bd`

Inventory after Batch 8:
- COMPLETE 35
- MISSING 23
- EXCLUDE 3

Batch 8 is locked. Do not reopen unless user explicitly orders it.

---

# 6. Japanese Batch 9 — DONE / CONTENT LOCKED / Production COMPLETE

Pages:
1. `ja/accommodation.html`
2. `ja/hongdae-vs-myeongdong.html`
3. `ja/best-area-for-first-time-visitors-seoul.html`
4. `ja/best-area-for-families-seoul.html`
5. `ja/best-area-for-solo-travelers-seoul.html`

Final Approved Public Copy:
- `Korea_Inside_JA_Stay_Decision_Batch9_Approved_Public_Copy_2026-09-25_v2.md`

Final Approved SHA-256:
`50d21272e14e28a7873b0ecadd628739b69bdc010a59a92b8b20a1db4c85f227`

Final implementation QA:
- approved mapping: **1,616 / 1,616**
- UNMAPPED: **0**
- position mismatch: **0**
- related-guide/card additional mappings: **11/11**
- FAQ visible/schema: **53/53**
- page-specific alt: **40/40**
- exact `aria-label`: **47/47**
- user-facing `data-label`: **63/63**
- BreadcrumbList name: **10/10**
- Accommodation summary `<strong>`: **4/4**
- page-specific English residue: **0**
- HTML tag order mismatch: **0**
- class/id mismatch: **0**
- functional data-* mismatch: **0**
- fact/number/recommendation mismatch: **0**
- affiliate/external href mismatch: **0**
- image/srcset mismatch: **0**
- broken internal link: **0**
- future `/ja/` link: **0**
- H1 one per page
- `lang="ja"`: **5/5**
- self canonical: **5/5**
- en/es/ja/x-default hreflang: **5/5**
- `git diff --check`: PASS

Production commits:
- implementation: `868718ce1f9389395dfd06d530955b8997f54dc3`
- asset path fix: `71c7802345710cd24626b3052dc1347cc663ed0f`
- Inventory: `0addea7fbd29fb7957e2b4ee1624dcd5b188afae`
- Batch 9 localization/Inventory Production SHA: `0addea7fbd29fb7957e2b4ee1624dcd5b188afae`

Post-release technical follow-up on 2026-09-26:
- missing reciprocal `hreflang="ja"` on the 5 English Batch 9 pages was corrected
- body/public copy changes: **0**
- correction commit: `020456f37287fa32e44dc49bbd91258da911fd13`
- HEAD = origin/main = correction commit
- ahead/behind: **0 / 0**
- Vercel: **READY**
- English and Japanese public URL checks: **5/5 pairs HTTP 200**
- Production `ja` hreflang QA: **5/5 PASS**

Inventory after Batch 9 remains:
- COMPLETE **40**
- MISSING **18**
- EXCLUDE **3**

Batch 9 is locked. The hreflang correction was a technical SEO follow-up and does not reopen Batch 9 public copy.

---

# 7. Important lesson from Batch 8 / 9

A major problem occurred because the process became too defensive:

Review → supplement → approve → preflight → supplement → approve → another preflight → implement

This created repeated STOP / approval loops and consumed hours.

The corrected rule for Batch 10 onward:

- ChatGPT must finish the content before asking the user.
- ChatGPT must self-check completeness before the user sees the final Review Copy.
- User wording approval should normally be **one approval**.
- Production should normally be **one separate approval**.
- Codex is not the content editor.

---

# 8. Japanese Batch 10 — ACTIVE

## Scope: six remaining Stay Decision Family pages

1. `best-area-for-couples-seoul.html`
2. `best-area-for-budget-travelers-seoul.html`
3. `best-area-for-shopping-seoul.html`
4. `best-area-for-nightlife-seoul.html`
5. `best-area-for-luxury-hotels-seoul.html`
6. `best-area-for-airport-access-seoul.html`

Current English Git blob SHAs used for the Batch 10 localization basis:
- couples: `260dbb9c04d80724e95c000850b8bf62d3193a5a`
- budget: `a4b4cca51f81170355252c753412db3110d0e823`
- shopping: `6cd60535dcc9a86ebc4291742b82d1a7eff85f6f`
- nightlife: `ceb3a619ef85f587d36c37e898622e58d18f2760`
- luxury: `59140c9b2c2fa6b211fb910dadc3c013c7f698d3`
- airport access: `1407a7f42dae3dc07d89db042502b106e61d44da`

These six fingerprints were verified during Batch 10 preparation. Codex must recheck them before exact implementation and STOP on drift.

Home-laptop recovery completed before Batch 10 implementation:
- all **18** Japanese MISSING working copies were regenerated as filesystem-level exact copies of current English source
- source exists: **18/18**
- destination exists: **18/18**
- file-size match: **18/18**
- SHA-256 exact match: **18/18**
- FAIL: **0**
- all 18 remain untracked until their approved Production Batch

Batch 10 may modify only the 6 files in this scope. The other 12 recovered working copies remain protected.

---

# 9. Batch 10 Research / Knowledge state

## Existing Research Master
`Korea_Inside_JA_Stay_Decision_Batch10_Research_Master_2026-09-25.md`

State:
**RESEARCH COMPLETE — BASIS ALREADY USED FOR BATCH 10 LOCALIZATION**

Do not reopen the Research merely because the room or computer changed. Recheck only if a factual conflict or time-sensitive requirement appears.

## Localization Knowledge Master
`Korea_Inside_JA_Stay_Decision_Batch10_Localization_Knowledge_Master_2026-09-26.md`

State:
**LOCALIZATION KNOWLEDGE COMPLETE / LOCKED FOR THE APPROVED BATCH 10 COPY**

Purpose:
- Research Master interpretation
- current English Production basis
- current Japanese Production family style
- Japanese search-language direction

This Knowledge Master informed the completed Batch 10 human localization. It is not itself the public-copy Source of Truth.

---

# 10. Batch 10 Japanese search-intent / editorial direction

Current Japanese Stay family style already in Production:
- `ソウルでどこに泊まる？`
- `初めてのソウル旅行はどこに泊まる？`
- `子連れソウル旅行はどこに泊まる？`
- `ソウル一人旅はどこに泊まる？`

Batch 10 must inherit:
- decision-first
- practical
- natural Japanese
- conditional judgment
- clear trade-offs
- no personal-experience fabrication
- no OTA-like copy
- no mechanical literal translation
- no repetitive `おすすめです`
- no universal “best” claims when English is conditional

Core Batch 10 model:
**traveler purpose → area choice → practical friction → hotel booking checks**

---

# 11. Batch 10 page-by-page SEO / localization direction

## A. Couples

Title:
`カップルのソウル旅行はどこに泊まる？おすすめエリア比較 | Korea Inside`

Meta:
`カップルのソウル旅行におすすめの宿泊エリアを比較。弘大、聖水、仁寺洞、明洞、江南、蚕室を、雰囲気、ナイトライフ、交通、予算、静かさで選びます。`

H1:
`カップルのソウル旅行はどこに泊まる？ 2026`

Editorial idea:
Do NOT make this a “romantic hotel” article.

Core:
- time together between sights
- morning coffee
- dinner nearby
- evening walk
- active vs quiet night
- room worth returning to
- two travelers + two suitcases
- shared daily route

Protected judgments:
- 弘大 = cafés / nightlife / active evenings
- 聖水 = slower design / café / pop-up days
- 仁寺洞 = culture / traditional streets / quieter evenings
- 明洞 = practical first-trip base
- 江南 = only when south-Seoul plans justify it
- 蚕室 = Lotte / lake / southeastern Seoul
- 梨泰院 = international dining / bars + hill friction
- 麻浦／孔徳 = calmer + direct AREX from 孔徳

---

## B. Budget

Title:
`ソウルで安く泊まるなら？予算重視のおすすめエリア比較 | Korea Inside`

Meta:
`ソウルの予算重視の宿泊エリアを、宿泊費だけでなく空港アクセス、地下鉄、荷物、ランドリー、深夜移動まで含めて比較します。`

H1:
`ソウルで安く泊まるなら？予算重視の宿泊エリア 2026`

Core idea:
**安い客室 ≠ 安い旅行**

Must preserve total-cost framework:
- final OTA total
- airport transfer
- station / exit
- elevator
- outdoor walk
- hills / stairs
- late taxi
- breakfast
- laundry
- luggage storage
- usable room size
- extra bed / second room
- weekend pricing
- cancellation terms

Do not invent prices.

---

## C. Shopping

Title:
`ソウルで買い物するならどこに泊まる？おすすめエリア比較 | Korea Inside`

Meta:
`明洞、江南、弘大、東大門、聖水、蚕室を、Kビューティー、ファッション、百貨店、夜の買い物、荷物、空港アクセスで比較します。`

H1:
`ソウルで買い物するならどこに泊まる？ 2026`

Core:
**何を買うか → 何度そのエリアへ行くか → 買った物をどうホテルへ戻すか**

Protected judgments:
- 明洞 = easiest first shopping base / K-beauty
- 江南 = department stores / premium; COEX・三成 and 狎鴎亭・清潭 are separate clusters
- 弘大 = young fashion / goods / vintage + active evenings
- 東大門 = late fashion; retail vs wholesale/building differences matter
- 聖水 = pop-ups / newer brands; not automatically best hotel base
- 蚕室 = large indoor malls + family-friendly / Lotte complex

No invented tax-refund thresholds.

---

## D. Nightlife

Title:
`ソウルで夜遊びするならどこに泊まる？ナイトライフのエリア比較 | Korea Inside`

Meta:
`弘大、梨泰院、江南、明洞、麻浦・孔徳、ソウル駅を、バー・クラブ、深夜の帰りやすさ、騒音、予算、空港アクセスで比較します。`

H1:
`ソウルで夜遊びするならどこに泊まる？ 2026`

Core:
**夜が終わったあと、ホテルまでどう戻るか。そして翌朝もその立地で困らないか。**

Use:
- `夜遊び` in title/H1/search intent
- `ナイトライフ` naturally in body
- `夜の過ごし方` for broader contexts

Protected judgments:
- 弘大 = easiest broad nightlife base
- 梨泰院 = international pubs / social nights
- 江南 = polished / higher-budget south-Seoul nightlife
- 明洞 = not nightlife-first; useful when day sightseeing dominates
- 麻浦／孔徳 = Hongdae access + quieter sleep + airport convenience
- ソウル駅 = transport/luggage choice, not nightlife choice

No universal safe/unsafe ranking.

---

## E. Luxury

Title:
`ソウルの高級ホテルはどのエリア？ラグジュアリー宿泊エリア比較 | Korea Inside`

Meta:
`江南、蚕室、明洞、ソウル駅・南大門、仁寺洞、梨泰院を、高級ホテル、ダイニング、買い物、空港、静かさ、観光で比較します。`

H1:
`ソウルで高級ホテルに泊まるならどのエリア？ 2026`

Core:
**高いホテル代が、実際の旅の移動や不便をどれだけ減らすか**

Still area-first, not luxury hotel ranking.

Protected room/rate checks:
- exact room category
- bed configuration / occupancy
- room size / luggage
- view category
- breakfast
- club lounge
- spa / pool / fitness access conditions
- soundproofing / room direction
- airport / taxi arrival
- early / late check-in
- luggage storage
- restaurant reservations
- taxes / service charges / deposits / cancellation / payment timing

---

## F. Airport Access

Title:
`仁川空港アクセスが便利なソウル宿泊エリア比較 | Korea Inside`

Meta:
`弘大、孔徳、ソウル駅、明洞を、仁川空港アクセス、AREX、空港バス、荷物、深夜到着とソウル観光の動きやすさで比較します。`

H1:
`仁川空港アクセスが便利なソウルの宿泊エリア 2026`

Core:
Do NOT simplify to “direct train = best.”

Decision path:
**空港ターミナル → 鉄道／バス／タクシー → 駅・停留所 → ホテル入口**

Protected facts:
- 弘大: AREX一般列車 direct to 弘大入口駅; Line 2; large station/final walk matters
- 孔徳: AREX一般列車 direct; Lines 5/6; Gyeongui–Jungang; airport limousine reaches area; calmer evenings than Hongdae
- ソウル駅: AREX直通列車 + AREX一般列車; KTX; large station friction remains
- 明洞: no direct AREX stop; rail needs connection; airport limousine may be easier for the right hotel
- late arrival: regular rail/daytime buses eventually stop; late-night buses exist; current timetable must be checked for actual date
- family/group: luggage + children + stroller + group size can change rail vs bus/taxi/private transfer value

Schema rule:
- visible FAQ: **7**
- FAQPage JSON-LD: **0**
- do NOT add FAQPage schema

---

# 12. Batch 10 structure baseline

| Page | H1 | H2 | H3 | Visible FAQ | FAQPage JSON-LD | Page-specific alt |
|---|---:|---:|---:|---:|---:|---:|
| Couples | 1 | 10 | 29 | 8 | 8 | 9 |
| Budget | 1 | 13 | 20 | 12 | 12 | 3 |
| Shopping | 1 | 11 | 29 | 8 | 8 | 6 |
| Nightlife | 1 | 9 | 24 | 10 | 10 | 7 |
| Luxury | 1 | 10 | 40 | 8 | 8 | 7 |
| Airport Access | 1 | 11 | 11 | 7 | 0 | 0 |

Note:
The earlier higher alt counts included the common header logo alt.
Current Source Extraction correctly separates page-specific alt from COMMON UI.

---

# 13. Batch 10 Source Extraction — COMPLETE

Created by Codex:
`md/작업자료/Korea_Inside_JA_Stay_Decision_Batch10_Source_Extraction_2026-09-26.md`

Purpose:
technical English source-position inventory for later human Japanese localization.

Codex boundaries used:
- no translation
- no localization
- no grammar changes
- no summarizing
- no merging
- no rewriting
- no HTML implementation

Exactness rule:
Each value has:
- source file
- line
- element/type
- section/heading context
- exact English
- exact Source target

COMMON UI REUSE is separated.

## Extraction result

| Page | Page-specific ITEM | FAQ visible / JSON-LD | Page-specific alt | aria | data-label |
|---|---:|---:|---:|---:|---:|
| couples | 202 | 8/8 | 9 | 0 | 0 |
| budget travelers | 259 | 12/12 | 3 | 2 | 36 |
| shopping | 249 | 8/8 | 6 | 0 | 36 |
| nightlife | 189 | 10/10 | 7 | 0 | 0 |
| luxury hotels | 220 | 8/8 | 7 | 1 | 0 |
| airport access | 148 | 7/0 | 0 | 0 | 0 |
| **TOTAL** | **1,267** | **53/46** | **32** | **3** | **72** |

COMMON UI REUSE:
- **83 positions per page**
- **498 total positions**

Alt reconciliation:
- page-specific alt = 32
- common header logo alt = 6
- total structural alt positions = 38

Technical extraction QA:
- English fingerprint **6/6 MATCH**
- ITEM numbering gaps: **0**
- COMMON numbering gaps: **0**
- duplicate source target: **0**
- unlisted title/meta/H1-H3/body/link/FAQ/JSON-LD/alt/ARIA/data-label: **0**
- Airport Access FAQPage schema remains **0**
- HTML implementation: **0**
- stage/commit/push/deploy: **0**
- existing working-tree changes protected

This Source MD is the exact source basis for Batch 10 human localization.

---

# 14. Batch 10 current status — IMPORTANT

**Current Batch 10 state:**

- Research Master: **COMPLETE**
- Localization Knowledge Master: **COMPLETE**
- Source Extraction MD: **COMPLETE**
- Source Extraction coverage: **1,267 page-specific ITEMs + 498 COMMON UI reuse positions**
- Japanese Localized Review Copy: **CREATED**
- Localized file saved by user under Japanese approved-MD storage:
  - `md/승인본/일본어/Korea_Inside_JA_Stay_Decision_Batch10_Localized_2026-09-26.md`
- generated Review Copy SHA-256 used in this room: `ddb832df694d6814dcf4ae1268782e3e24bac8c7336988e1ec511f7efee24a25`
- localized coverage: **1,267 / 1,267**
- missing Japanese values: **0**
- ITEM numbering gaps: **0**
- user final wording approval: **COMPLETE**
- wording state: **APPROVED PUBLIC COPY — CONTENT LOCKED by user approval**
- stored MD management metadata update to the approved status: **pending Codex completion report unless already performed**
- Codex exact-implementation instruction: **ISSUED**
- Codex implementation report: **NOT YET RECEIVED in this handover state**
- JA HTML implementation: **NOT YET CONFIRMED**
- Production approval: **NOT YET**
- Production deploy: **NOT YET**
- Inventory: **40 COMPLETE / 18 MISSING / 3 EXCLUDE** until Batch 10 Production QA is complete

Do NOT relocalize Batch 10. Do NOT ask for wording approval again. The wording is locked.

---

# 15. Exact next action in the new room

The exact next action is to continue from the already-approved Batch 10 implementation stage.

1. First receive and review the Codex completion report for the already-issued Batch 10 exact-implementation instruction.
2. If Codex has not actually run that instruction yet, run it using the approved Batch 10 localization MD as the single wording Source of Truth.
3. Codex must:
   - confirm the approved MD path
   - update only management metadata needed to reflect `APPROVED PUBLIC COPY — CONTENT LOCKED` without changing any Japanese value
   - recheck the 6 English source fingerprints
   - implement the 1,267 page-specific Japanese values exactly
   - reuse the 498 locked Japanese common UI positions
   - preserve facts, structure, affiliate/tracking, images, CSS/JS and schema structure
   - keep Airport Access visible FAQ **7 / FAQPage 0**
   - perform local technical QA
   - **STOP before stage / commit / push / Production**
4. After the Codex report, ChatGPT reviews the report.
5. Only then ask the user for the single **Production approval**.

Do not return to Research, Source Extraction, localization drafting, or wording approval unless a real source drift or approved-copy defect is reported.

---

# 16. Normal approval policy from this point

Batch 10 wording approval is already complete.

Do NOT ask again for:
- research approval
- preflight approval
- mapping approval
- localization wording approval
- supplement approval

The next normal user gate is:

**Production approval ONCE — after Codex exact implementation + local technical QA report passes review.**

Only break this flow if there is a real source fingerprint drift, approved-copy defect, technical conflict, or the user explicitly requests staged review.

---

# 17. Git / Production safety baseline

Latest confirmed Production baseline before Batch 10 implementation:
- current Production / Git SHA: `020456f37287fa32e44dc49bbd91258da911fd13`
- HEAD = origin/main
- ahead/behind: **0 / 0**
- staged: **0**
- Vercel: **READY**

Current working-tree expectation:
- 18 recovered `/ja/` MISSING working copies are untracked exact English copies
- no tracked modification was present at the completion of the 18/18 SHA recovery audit
- Batch 10 implementation is allowed to modify only its 6 Japanese working copies plus the approved Batch 10 MD management status within the approved instruction
- the other 12 untracked MISSING working copies remain protected

Before any Git action:
- inspect actual `git status --short` again
- verify scope
- stage only explicitly approved files
- never stage everything

Production must not run until the user explicitly approves it after reviewing the Codex implementation/QA report.

---

# 18. Working style expected by user

User preference:
- command → result → confirmation
- concise but exact
- facts vs assumptions should be clear internally
- no unnecessary re-explaining
- do not make the user re-check work ChatGPT should have completed itself
- preserve context across rooms
- if a room is getting full, create a handover before context loss becomes a risk

For Korea Inside specifically:
- user wants to focus on final decisions
- ChatGPT should carry the editorial burden
- Codex should carry implementation burden

---

# 19. DO NOT FORGET

1. Batch 8 = DONE LOCKED.
2. Batch 9 = DONE LOCKED; later English reciprocal `ja` hreflang follow-up completed in commit `020456f37287fa32e44dc49bbd91258da911fd13`.
3. Inventory before Batch 10 Production = **40 COMPLETE / 18 MISSING / 3 EXCLUDE**.
4. All 18 MISSING `/ja/` working copies were recovered and are **18/18 SHA-identical English copies**.
5. Batch 10 is active with 6 Stay Decision pages.
6. Batch 10 Source Extraction = **1,267 page-specific ITEMs + 498 common UI reuse positions**.
7. Batch 10 human Japanese localization = **1,267/1,267 complete**.
8. User final wording approval = **COMPLETE**.
9. Batch 10 wording = **CONTENT LOCKED**. Do not retranslate or rewrite it.
10. Codex exact-implementation instruction has already been issued; the next evidence needed is the **Codex completion report**.
11. Do not stage / commit / push / deploy Batch 10 until its implementation report is reviewed and the user gives explicit Production approval.
12. Do not ask Codex to translate.
13. Protect the 12 non-Batch-10 untracked working copies and all other user changes.

---

# 20. Short restart message for the next room

After reading the required current Standards + this Handover, report briefly:

- Public Content Master v1.4 read
- Navigation Standard v1.1 read
- Language Localization Standard v2.1 read
- Japanese Localization Standard v1.1 read
- latest Handover read
- Batch 8/9 locked
- current Inventory = 40/18/3
- 18 MISSING working copies recovered, SHA exact 18/18
- Batch 10 localization 1,267/1,267 complete and user-approved / CONTENT LOCKED
- next action = review the Batch 10 Codex exact-implementation report, or run the already-approved implementation instruction if it has not yet run
- no Production until explicit user approval after QA report
- protected working-tree changes will remain untouched

Do not recreate Batch 10 localization. Continue only within the user's instructed scope.

---

