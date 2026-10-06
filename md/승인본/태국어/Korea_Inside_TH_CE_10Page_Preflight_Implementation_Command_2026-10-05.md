# Korea Inside — Thai C+E 10-Page Preflight + Implementation Command

**Date:** 2026-10-05
**User approval:** C+E IMPLEMENTATION APPROVED
**Target branch:** `th-localization-2026-10-03`
**Current confirmed feature HEAD before this run:** `3be7ab4692f7fb2fbbee61ad83e5971282abe6d5`
**Forbidden:** main merge, Production deploy, Final Integration

## 1. Scope

### C — Payment Detail
1. `th/wowpass.html`
2. `th/tmoney-vs-wowpass.html`
3. `th/apple-pay-korea.html`
4. `th/foreign-credit-cards-korea.html`
5. `th/card-declined-korea.html`

### E — Final 5
6. `th/best-esim-for-korea.html`
7. `th/korea-esim-with-phone-number.html`
8. `th/checklist.html`
9. `th/taste-korea.html`
10. `th/index.html`

Process:
`READ-ONLY FULL RECONCILIATION → gap=0 only → exact implementation → QA → feature-branch commit/push`

If any missing approved value exists, finish the audit across all 10 pages and return **one consolidated all-gaps report**. Do not create partial Thai files.

## 2. Approved Sources

### C
- `Korea_Inside_TH_C_Payment_Detail_5Page_Approved_Public_Copy_2026-10-05.md`
  SHA-256 `c3da6215c285ed85346edf5e4d0e9aa01382b7eaa0e9c229a4a26eb936f3d53e`
- `Korea_Inside_TH_C_Payment_Detail_Approved_Manifest_2026-10-05.md`
  SHA-256 `b982b5786d675e1d661aa41f9c1122a443ce95a3aa311fa5692d26b13d801500`

### E
- `Korea_Inside_TH_E_Final_5Page_Approved_Public_Copy_2026-10-05.md`
  SHA-256 `eb63758950d2333ad3df780a3377781a6b4777db6ca6e915a0d8f41a9d257b2f`
- `Korea_Inside_TH_E_Final_Approved_Manifest_2026-10-05.md`
  SHA-256 `1f6a77203331e537df7599dcded83b8076107b7600596258e5e98a7ccbfce00e`

Verify all four SHAs before any write. If a SHA differs, STOP.

## 3. Git Safety

Repository: `C:\Projects\Koreainside`

Before write:
- confirm branch exactly `th-localization-2026-10-03`
- record HEAD and `git status --short`
- protect all existing user changes outside scope
- never use `git add .`, `git add -A`, `git restore`, `git reset`, `git clean`, `git stash`, or force push

Expected current HEAD:
`3be7ab4692f7fb2fbbee61ad83e5971282abe6d5`

If HEAD moved, inspect; never reset.

## 4. English Fingerprint Gate

### C
- `wowpass.html` → `b26cc850d1032055bc8a59d623381f6318c8b145`
- `tmoney-vs-wowpass.html` → `363b7d2eb19d6da9a94f1617357e596bc69b9289`
- `apple-pay-korea.html` → `33b4d8452a43880ee345fcb785311d03bc4e3836`
- `foreign-credit-cards-korea.html` → `573908d4be796b00fac5aa49c0faace9e6e1507f`
- `card-declined-korea.html` → `45e49d4be789fa09cc91099980f8d77eb59626cb`

### E
- `best-esim-for-korea.html` → `a2c364880d252ac6d215342b122891250ad73817`
- `korea-esim-with-phone-number.html` → `9bb8a40835a3333d9ca1786a5141029fe2933f5f`
- `checklist.html` → `fc796134b87b2cbef5c0d250b90fe003f90bccb6`
- `taste-korea.html` → `c6af3422bcd12605515e6fc2dce3f523dd9698cc`
- `index.html` → `ef1731be34fdc5d0ae5c27b7ed6c02dc190c648f`

These were rechecked immediately before issuing this command and were 10/10 MATCH.

## 5. Mandatory Read-Only Full Node Reconciliation

Before creating/modifying any of the 10 Thai targets, inventory every page-specific user-facing source target in source order.

Include:
- title/meta/OG/Twitter
- user-facing JSON-LD
- H1/H2/H3/H4
- body/direct text
- list text
- `<strong>` / `<span>`
- `<dt>` / `<dd>`
- table caption/th/td
- labels/helper text/badges
- CTA/button
- breadcrumb
- visible link text
- related-guide titles
- official-source titles and descriptions
- review/update-date labels
- notices/disclosures
- figure captions
- alt
- ARIA
- user-facing data-label/data-*
- user-facing inline JS strings
- FAQ visible and schema strings

For each page report:
- source target count
- approved target count
- mapped count
- unmapped source count
- orphan approved count
- ambiguous/duplicate mappings
- FAQ visible/schema
- alt
- ARIA
- user-facing data-label
- user-facing JSON-LD

### Anti-loop rule

If any unmapped target remains:
1. do not create/modify any C+E Thai target;
2. finish the audit across all 10;
3. return one list containing every remaining gap with file, line/context, element/type, exact English source;
4. STOP.

Do not return gaps 2–3 at a time. Do not partially implement safe pages.

Only if all 10 have:
- source fingerprints MATCH
- approved SHA MATCH
- coverage 100%
- unmapped 0
- orphan 0
- ambiguous 0

may implementation begin.

## 6. Approved English Fact Correction

One English correction is explicitly approved in `best-esim-for-korea.html`.

Replace exactly:
`On arrival with eSIM and roaming enabled; 30-day activation deadline`

with:
`On arrival with eSIM and roaming enabled; 180-day activation window`

Also replace exactly:
`Provider conditions checked: August 15, 2026.`

with:
`Provider conditions checked: October 5, 2026.`

Approved Thai corresponding value:
`เริ่มใช้เมื่อถึงปลายทางและเปิด eSIM กับ roaming; มีช่วงเวลาเปิดใช้งาน 180 วัน`

Fingerprint gate uses the pre-correction blob. Apply these two English replacements only after read-only reconciliation passes. No other English public-copy change is authorized.

## 7. Exact Implementation Rules

After reconciliation PASS:
- create each Thai sibling from current English structure
- exact approved Thai wording only
- no Codex translation, rewriting, grammar improvement, summarization, node merge, or recommendation change
- no Thai-specific fact additions
- preserve section order
- reuse approved Thai common UI
- do not modify common header/navigation/footer, `common.js`, mobile hamburger, shared `style.css`
- preserve href, affiliate/tracking, images/srcset, classes/IDs, functional data-*, CSS/JS logic, schema structure
- new contextual Thai links = 0
- keep approved English fallback internal hrefs

## 8. Thai Technical Identity

All 10:
- `<html lang="th">`
- Thai self canonical
- `hreflang="th"` self
- current real sibling hreflang set
- `x-default` to English source

Homepage special case:
- file is `th/index.html`
- canonical must be `https://www.getkoreainside.com/th/`
- not `/th/index.html`

No sitemap/common.js/language-switcher/final-integration work.

## 9. Structural Gate

| Page | H1/H2/H3/H4 | Visible FAQ | FAQPage |
|---|---:|---:|---:|
| wowpass | 1/16/7/0 | 13 | 13 |
| tmoney-vs-wowpass | 1/11/0/0 | 8 | 8 |
| apple-pay-korea | 1/11/0/0 | 8 | 8 |
| foreign-credit-cards-korea | 1/9/6/0 | 8 | 8 |
| card-declined-korea | 1/9/0/0 | 8 | 8 |
| best-esim-for-korea | 1/17/41/0 | 8 | 8 |
| korea-esim-with-phone-number | 1/19/25/0 | 8 | 8 |
| checklist | 1/12/3/0 | 8 | 8 |
| taste-korea | 1/6/27/0 | 8 | 0 |
| index | 1/3/5/0 | 0 | 0 |

For FAQPage pages, visible Thai Q/A must exactly equal schema Thai Q/A in count/order/wording.

`taste-korea.html`: visible questions 8, FAQPage 0. Do not add schema.
`index.html`: FAQ 0/0.

## 10. QA

Required:
- approved mapping 100%
- missing/orphan mapping 0
- English editorial residue 0 except approved/protected brands/proper terms
- unapproved other-language residue 0
- fact/number/date/price/recommendation mismatch 0
- structure mismatch 0
- FAQ/schema mismatch 0
- href mismatch 0
- affiliate/tracking mismatch 0
- image/srcset/path-case mismatch 0
- class/id/data-* mismatch 0
- JSON-LD parse error 0
- duplicate H1/canonical 0
- new contextual Thai links 0
- `git diff --check` PASS
- polite particles `ครับ / ค่ะ / นะครับ / นะคะ` 0 unless explicitly approved
- no THB conversion
- Arabic numerals/Gregorian years preserved

If browser unavailable, report `BROWSER QA NOT RUN`; do not invent PASS.

## 11. Stage / Commit / Push

Only after reconciliation and QA PASS.

Allowed staged scope:
- 10 Thai HTML files in Section 1
- plus English `best-esim-for-korea.html` for the exact 2 approved corrections only

Maximum: 11 files.

Do not stage approved MDs.

Require:
- unexpected staged files 0
- `git diff --cached --check` PASS
- explicit staged manifest review

Commit message:
`Implement Thai payment and final guide pages`

Push only:
`origin/th-localization-2026-10-03`

Then verify local HEAD = remote feature HEAD and ahead/behind 0/0.

Do not merge main. Do not deploy Production.

## 12. Expected State

Before run: Thai HTML `43/58`.

After C+E success:
- Thai HTML `53/58`
- missing `5/58`

Remaining D:
- `th/korea-atm-foreign-cards.html`
- `th/korean-online-payments-foreigners.html`
- `th/apps.html`
- `th/maps.html`
- `th/esim.html`

Production COMPLETE remains 0/58.

## 13. Completion Report

Report:
1. repo/branch
2. start/end HEAD
3. C Approved Copy SHA
4. C Manifest SHA
5. E Approved Copy SHA
6. E Manifest SHA
7. fingerprints 10/10
8. source node count per page
9. mapped/unmapped/orphan/ambiguous per page
10. total approved coverage
11. Thai files 10/10
12. English best-eSIM correction 2/2
13. heading structure
14. FAQ visible/schema
15. lang=th
16. canonical, including homepage `/th/`
17. hreflang
18. residue scans
19. polite-particle scan
20. fact/number/date/recommendation mismatch
21. href/affiliate/tracking mismatch
22. image/srcset/path-case errors
23. class/id/data-* mismatch
24. JSON-LD errors
25. contextual Thai links 0
26. browser QA or limitation
27. git diff check
28. staged manifest
29. cached diff check
30. commit SHA/message
31. push result
32. HEAD/remote/ahead-behind
33. scope overrun 0
34. protected user changes
35. main 0
36. Production 0
37. Final Integration 0
38. Thai HTML 53/58
39. missing 5/58

If Phase 1 finds gaps, return one **ALL-GAPS REPORT** and STOP before any writes.
