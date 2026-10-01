# Korea Inside Taiwan Traditional Chinese Localization Master Inventory

## Document Metadata

- Date: 2026-10-01
- Initial inventory snapshot: 2026-09-27; the historical Pilot and setup records are preserved below.
- Status: ACTIVE INVENTORY
- Language: Traditional Chinese (Taiwan / zh-TW)
- Purpose: Track Taiwan Traditional Chinese localization status for the current public English page set.
- Scope: 58 localization-target pages plus 3 explicitly excluded public/root HTML files.
- Taiwan Production folder: `/zh-tw/`
- Taiwan hreflang code: `zh-TW`
- Current status: 58/58 PRODUCTION COMPLETE; 58 COMPLETE / 0 MISSING / 3 EXCLUDE. Current technical release and public QA verified on 2026-10-01.
- Completion condition: Taiwan `MISSING = 0`.

## Operating Basis

- Current English Production is the factual, structural, and recommendation-judgment source.
- `Korea_Inside_Taiwan_Localization_Standard.md` is the Taiwan-specific execution standard.
- Historical preparation included the user-approved Taiwan Codex DRAFT exception; final public wording followed ChatGPT editorial review and user approval.
- ChatGPT performs the full editorial audit and final relocalization before any public copy becomes approved.
- A Codex draft, local HTML copy, draft branch, or Pilot does not make a page COMPLETE.
- `COMPLETE` requires Approved Public Copy / CONTENT LOCKED, exact implementation, technical QA, Production release, and public QA.
- Normal internal routing for a published Taiwan sibling uses `/zh-tw/FILENAME`.
- If a Taiwan sibling does not exist yet, English fallback is permitted until that Taiwan page is released.

## Historical Pilot State — 2026-09-27

- `where-to-stay-in-myeongdong.html` completed a Codex Taiwan localization Pilot on 2026-09-27.
- Pilot coverage: 415 / 415 localized ITEMs.
- At that stage the Pilot was a DRAFT and did not change the Inventory status.
- At that stage `where-to-stay-in-myeongdong.html` was `MISSING` pending final ChatGPT review, user approval, implementation and Production QA. Its current status is COMPLETE following the release verification below.

## Technical Release Verification — 2026-10-01

- Current release commit: `ab8640e14338fc89056ceab0dde623bef7966acc` — `Fix Japanese and Taiwan release integration gaps`.
- Git-triggered Vercel Production deployment: `dpl_5oBqsrWcR5a8VcCb3JVtnxy9yqxL`, READY; deployed SHA matches the release commit.
- Public QA completed: 2026-10-01 21:32:45 KST (2026-10-01T12:32:45Z).
- Earlier implementation records: approved Taiwan localization commit `97c9d9a3a79d0ba172c4071924b079fc86ea1e4e` and sitemap integration commit `6d327d40c2758d6545b7f6bb0958dd4f8ee5a86a`, both dated 2026-09-27; approved zh-TW corrections commit `c46393946e917864557dd6827dea1cd4f37e9d44`, dated 2026-09-30. These commit dates are not asserted as exact initial Production publication timestamps.
- The 2026-10-01 date records technical integration completion and public QA, not the first publication of Taiwan pages.
- Taiwan public HTTP: 58/58 HTTP 200; unexpected redirects, 404 and 5xx: 0.
- Taiwan lang=zh-TW, self canonical and exactly one H1: 58/58; noindex: 0.
- Taiwan meta descriptions: 58/58, with no empty or duplicate tags. The two new Lotte World / Seoul Sky descriptions received final user approval on 2026-10-01 and match the approved copy exactly.
- Current six-language reciprocal hreflang: 2,436/2,436 entries across 348 corresponding pages, including `en`, `es`, `ja`, `zh-TW`, `fr`, `de` and `x-default`; duplicate, missing and wrong-page targets: 0.
- Taiwan sitemap: 58/58 canonical URLs, homepage `/zh-tw/` exactly once, with no duplicate or missing target entries; existing Taiwan sitemap entries were preserved.
- Taiwan preload fix: 29 imagesrcset candidates across 11 pages resolve to existing root image files and return HTTP 200. Candidate order, width descriptors, imagesizes and ordinary img/srcset values are preserved.
- FAQ / JSON-LD: 49 JSON-LD blocks parse successfully; 38 FAQPage pages, question parity 327/327 and answer parity 327/327. The 89 visible FAQs on pages without FAQPage remain without newly added schema.
- Internal links: same-language English fallback residue 0; all 123 distinct internal public targets checked across JA and zh-TW return HTTP 200.
- Body copy, visible FAQ, JSON-LD copy, headings, internal hrefs, affiliate URLs, CID/subid/campaign parameters, tracking and provider order are unchanged by the technical release.
- Production source matches the release commit for all 348 HTML pages and sitemap.xml. Staged scope was exactly 233 approved files; staged diff fingerprint and whitespace checks passed.
- Known QA limitation: Browser interaction QA NOT RUN because no connected browser was available. Static HTML and HTTP verification do not establish visual rendering, menu interaction or measured performance.

## Status Definitions

- `COMPLETE`: Final Taiwan public copy is approved, exactly implemented, Production-released, and QA-verified.
- `MISSING`: Taiwan localization is not yet Production-complete. Draft work may exist.
- `EXCLUDE`: Outside the Taiwan localization target set for the stated reason.

## Master Inventory

| Category | English filename | English Production URL | Taiwan filename | Taiwan Production URL | Status | Notes |
|---|---|---|---|---|---|---|
| Home | `index.html` | `https://www.getkoreainside.com/` | `zh-tw/index.html` | `https://www.getkoreainside.com/zh-tw/` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Discover | `taste-korea.html` | `https://www.getkoreainside.com/taste-korea.html` | `zh-tw/taste-korea.html` | `https://www.getkoreainside.com/zh-tw/taste-korea.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Discover | `k-beauty.html` | `https://www.getkoreainside.com/k-beauty.html` | `zh-tw/k-beauty.html` | `https://www.getkoreainside.com/zh-tw/k-beauty.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel — Area | `hongdae-travel-guide.html` | `https://www.getkoreainside.com/hongdae-travel-guide.html` | `zh-tw/hongdae-travel-guide.html` | `https://www.getkoreainside.com/zh-tw/hongdae-travel-guide.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel — Area | `myeongdong-travel-guide.html` | `https://www.getkoreainside.com/myeongdong-travel-guide.html` | `zh-tw/myeongdong-travel-guide.html` | `https://www.getkoreainside.com/zh-tw/myeongdong-travel-guide.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel — Area | `seongsu-travel-guide.html` | `https://www.getkoreainside.com/seongsu-travel-guide.html` | `zh-tw/seongsu-travel-guide.html` | `https://www.getkoreainside.com/zh-tw/seongsu-travel-guide.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel — Area | `insadong-travel-guide.html` | `https://www.getkoreainside.com/insadong-travel-guide.html` | `zh-tw/insadong-travel-guide.html` | `https://www.getkoreainside.com/zh-tw/insadong-travel-guide.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel — Area | `gangnam-travel-guide.html` | `https://www.getkoreainside.com/gangnam-travel-guide.html` | `zh-tw/gangnam-travel-guide.html` | `https://www.getkoreainside.com/zh-tw/gangnam-travel-guide.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel — Area | `jamsil-travel-guide.html` | `https://www.getkoreainside.com/jamsil-travel-guide.html` | `zh-tw/jamsil-travel-guide.html` | `https://www.getkoreainside.com/zh-tw/jamsil-travel-guide.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel — Area | `gongdeok-mapo-seoul-guide.html` | `https://www.getkoreainside.com/gongdeok-mapo-seoul-guide.html` | `zh-tw/gongdeok-mapo-seoul-guide.html` | `https://www.getkoreainside.com/zh-tw/gongdeok-mapo-seoul-guide.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel — Area | `itaewon-travel-guide.html` | `https://www.getkoreainside.com/itaewon-travel-guide.html` | `zh-tw/itaewon-travel-guide.html` | `https://www.getkoreainside.com/zh-tw/itaewon-travel-guide.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel — Area | `dongdaemun-travel-guide.html` | `https://www.getkoreainside.com/dongdaemun-travel-guide.html` | `zh-tw/dongdaemun-travel-guide.html` | `https://www.getkoreainside.com/zh-tw/dongdaemun-travel-guide.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel — Attraction | `lotte-world-seoul.html` | `https://www.getkoreainside.com/lotte-world-seoul.html` | `zh-tw/lotte-world-seoul.html` | `https://www.getkoreainside.com/zh-tw/lotte-world-seoul.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel — Attraction | `seoul-sky-guide.html` | `https://www.getkoreainside.com/seoul-sky-guide.html` | `zh-tw/seoul-sky-guide.html` | `https://www.getkoreainside.com/zh-tw/seoul-sky-guide.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Hub / Decision / Comparison | `accommodation.html` | `https://www.getkoreainside.com/accommodation.html` | `zh-tw/accommodation.html` | `https://www.getkoreainside.com/zh-tw/accommodation.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Hub / Decision / Comparison | `hongdae-vs-myeongdong.html` | `https://www.getkoreainside.com/hongdae-vs-myeongdong.html` | `zh-tw/hongdae-vs-myeongdong.html` | `https://www.getkoreainside.com/zh-tw/hongdae-vs-myeongdong.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Hub / Decision / Comparison | `best-area-for-first-time-visitors-seoul.html` | `https://www.getkoreainside.com/best-area-for-first-time-visitors-seoul.html` | `zh-tw/best-area-for-first-time-visitors-seoul.html` | `https://www.getkoreainside.com/zh-tw/best-area-for-first-time-visitors-seoul.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Hub / Decision / Comparison | `best-area-for-families-seoul.html` | `https://www.getkoreainside.com/best-area-for-families-seoul.html` | `zh-tw/best-area-for-families-seoul.html` | `https://www.getkoreainside.com/zh-tw/best-area-for-families-seoul.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Hub / Decision / Comparison | `best-area-for-solo-travelers-seoul.html` | `https://www.getkoreainside.com/best-area-for-solo-travelers-seoul.html` | `zh-tw/best-area-for-solo-travelers-seoul.html` | `https://www.getkoreainside.com/zh-tw/best-area-for-solo-travelers-seoul.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Hub / Decision / Comparison | `best-area-for-couples-seoul.html` | `https://www.getkoreainside.com/best-area-for-couples-seoul.html` | `zh-tw/best-area-for-couples-seoul.html` | `https://www.getkoreainside.com/zh-tw/best-area-for-couples-seoul.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Hub / Decision / Comparison | `best-area-for-budget-travelers-seoul.html` | `https://www.getkoreainside.com/best-area-for-budget-travelers-seoul.html` | `zh-tw/best-area-for-budget-travelers-seoul.html` | `https://www.getkoreainside.com/zh-tw/best-area-for-budget-travelers-seoul.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Hub / Decision / Comparison | `best-area-for-shopping-seoul.html` | `https://www.getkoreainside.com/best-area-for-shopping-seoul.html` | `zh-tw/best-area-for-shopping-seoul.html` | `https://www.getkoreainside.com/zh-tw/best-area-for-shopping-seoul.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Hub / Decision / Comparison | `best-area-for-nightlife-seoul.html` | `https://www.getkoreainside.com/best-area-for-nightlife-seoul.html` | `zh-tw/best-area-for-nightlife-seoul.html` | `https://www.getkoreainside.com/zh-tw/best-area-for-nightlife-seoul.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Hub / Decision / Comparison | `best-area-for-luxury-hotels-seoul.html` | `https://www.getkoreainside.com/best-area-for-luxury-hotels-seoul.html` | `zh-tw/best-area-for-luxury-hotels-seoul.html` | `https://www.getkoreainside.com/zh-tw/best-area-for-luxury-hotels-seoul.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Hub / Decision / Comparison | `best-area-for-airport-access-seoul.html` | `https://www.getkoreainside.com/best-area-for-airport-access-seoul.html` | `zh-tw/best-area-for-airport-access-seoul.html` | `https://www.getkoreainside.com/zh-tw/best-area-for-airport-access-seoul.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Area / Hotel Detail | `where-to-stay-in-myeongdong.html` | `https://www.getkoreainside.com/where-to-stay-in-myeongdong.html` | `zh-tw/where-to-stay-in-myeongdong.html` | `https://www.getkoreainside.com/zh-tw/where-to-stay-in-myeongdong.html` | COMPLETE | Historical Codex Pilot on 2026-09-27; current Production verified in the 2026-10-01 technical release QA. |
| Stay — Area / Hotel Detail | `where-to-stay-in-hongdae.html` | `https://www.getkoreainside.com/where-to-stay-in-hongdae.html` | `zh-tw/where-to-stay-in-hongdae.html` | `https://www.getkoreainside.com/zh-tw/where-to-stay-in-hongdae.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Area / Hotel Detail | `hotels-near-seoul-station.html` | `https://www.getkoreainside.com/hotels-near-seoul-station.html` | `zh-tw/hotels-near-seoul-station.html` | `https://www.getkoreainside.com/zh-tw/hotels-near-seoul-station.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Area / Hotel Detail | `hotels-near-gongdeok-station.html` | `https://www.getkoreainside.com/hotels-near-gongdeok-station.html` | `zh-tw/hotels-near-gongdeok-station.html` | `https://www.getkoreainside.com/zh-tw/hotels-near-gongdeok-station.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Area / Hotel Detail | `where-to-stay-in-insadong.html` | `https://www.getkoreainside.com/where-to-stay-in-insadong.html` | `zh-tw/where-to-stay-in-insadong.html` | `https://www.getkoreainside.com/zh-tw/where-to-stay-in-insadong.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Area / Hotel Detail | `where-to-stay-in-jamsil.html` | `https://www.getkoreainside.com/where-to-stay-in-jamsil.html` | `zh-tw/where-to-stay-in-jamsil.html` | `https://www.getkoreainside.com/zh-tw/where-to-stay-in-jamsil.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Area / Hotel Detail | `where-to-stay-in-gangnam.html` | `https://www.getkoreainside.com/where-to-stay-in-gangnam.html` | `zh-tw/where-to-stay-in-gangnam.html` | `https://www.getkoreainside.com/zh-tw/where-to-stay-in-gangnam.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Area / Hotel Detail | `where-to-stay-in-dongdaemun.html` | `https://www.getkoreainside.com/where-to-stay-in-dongdaemun.html` | `zh-tw/where-to-stay-in-dongdaemun.html` | `https://www.getkoreainside.com/zh-tw/where-to-stay-in-dongdaemun.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Area / Hotel Detail | `where-to-stay-in-seongsu.html` | `https://www.getkoreainside.com/where-to-stay-in-seongsu.html` | `zh-tw/where-to-stay-in-seongsu.html` | `https://www.getkoreainside.com/zh-tw/where-to-stay-in-seongsu.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Stay — Area / Hotel Detail | `where-to-stay-in-itaewon.html` | `https://www.getkoreainside.com/where-to-stay-in-itaewon.html` | `zh-tw/where-to-stay-in-itaewon.html` | `https://www.getkoreainside.com/zh-tw/where-to-stay-in-itaewon.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| eSIM | `esim.html` | `https://www.getkoreainside.com/esim.html` | `zh-tw/esim.html` | `https://www.getkoreainside.com/zh-tw/esim.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| eSIM | `best-esim-for-korea.html` | `https://www.getkoreainside.com/best-esim-for-korea.html` | `zh-tw/best-esim-for-korea.html` | `https://www.getkoreainside.com/zh-tw/best-esim-for-korea.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| eSIM | `korea-esim-with-phone-number.html` | `https://www.getkoreainside.com/korea-esim-with-phone-number.html` | `zh-tw/korea-esim-with-phone-number.html` | `https://www.getkoreainside.com/zh-tw/korea-esim-with-phone-number.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Airport | `airport.html` | `https://www.getkoreainside.com/airport.html` | `zh-tw/airport.html` | `https://www.getkoreainside.com/zh-tw/airport.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Airport | `arrival.html` | `https://www.getkoreainside.com/arrival.html` | `zh-tw/arrival.html` | `https://www.getkoreainside.com/zh-tw/arrival.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Airport | `airport-transfer.html` | `https://www.getkoreainside.com/airport-transfer.html` | `zh-tw/airport-transfer.html` | `https://www.getkoreainside.com/zh-tw/airport-transfer.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Airport | `arex.html` | `https://www.getkoreainside.com/arex.html` | `zh-tw/arex.html` | `https://www.getkoreainside.com/zh-tw/arex.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Airport | `airport-bus.html` | `https://www.getkoreainside.com/airport-bus.html` | `zh-tw/airport-bus.html` | `https://www.getkoreainside.com/zh-tw/airport-bus.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Maps | `maps.html` | `https://www.getkoreainside.com/maps.html` | `zh-tw/maps.html` | `https://www.getkoreainside.com/zh-tw/maps.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Transport | `tmoney.html` | `https://www.getkoreainside.com/tmoney.html` | `zh-tw/tmoney.html` | `https://www.getkoreainside.com/zh-tw/tmoney.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Transport | `wowpass.html` | `https://www.getkoreainside.com/wowpass.html` | `zh-tw/wowpass.html` | `https://www.getkoreainside.com/zh-tw/wowpass.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Transport | `tmoney-vs-wowpass.html` | `https://www.getkoreainside.com/tmoney-vs-wowpass.html` | `zh-tw/tmoney-vs-wowpass.html` | `https://www.getkoreainside.com/zh-tw/tmoney-vs-wowpass.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Transport | `taxi.html` | `https://www.getkoreainside.com/taxi.html` | `zh-tw/taxi.html` | `https://www.getkoreainside.com/zh-tw/taxi.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Transport | `incheon-airport-private-transfer.html` | `https://www.getkoreainside.com/incheon-airport-private-transfer.html` | `zh-tw/incheon-airport-private-transfer.html` | `https://www.getkoreainside.com/zh-tw/incheon-airport-private-transfer.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Transport | `rental-car.html` | `https://www.getkoreainside.com/rental-car.html` | `zh-tw/rental-car.html` | `https://www.getkoreainside.com/zh-tw/rental-car.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Apps | `apps.html` | `https://www.getkoreainside.com/apps.html` | `zh-tw/apps.html` | `https://www.getkoreainside.com/zh-tw/apps.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel Tips | `checklist.html` | `https://www.getkoreainside.com/checklist.html` | `zh-tw/checklist.html` | `https://www.getkoreainside.com/zh-tw/checklist.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel Tips | `payments.html` | `https://www.getkoreainside.com/payments.html` | `zh-tw/payments.html` | `https://www.getkoreainside.com/zh-tw/payments.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel Tips | `foreign-credit-cards-korea.html` | `https://www.getkoreainside.com/foreign-credit-cards-korea.html` | `zh-tw/foreign-credit-cards-korea.html` | `https://www.getkoreainside.com/zh-tw/foreign-credit-cards-korea.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel Tips | `card-declined-korea.html` | `https://www.getkoreainside.com/card-declined-korea.html` | `zh-tw/card-declined-korea.html` | `https://www.getkoreainside.com/zh-tw/card-declined-korea.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel Tips | `korean-online-payments-foreigners.html` | `https://www.getkoreainside.com/korean-online-payments-foreigners.html` | `zh-tw/korean-online-payments-foreigners.html` | `https://www.getkoreainside.com/zh-tw/korean-online-payments-foreigners.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel Tips | `korea-atm-foreign-cards.html` | `https://www.getkoreainside.com/korea-atm-foreign-cards.html` | `zh-tw/korea-atm-foreign-cards.html` | `https://www.getkoreainside.com/zh-tw/korea-atm-foreign-cards.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Travel Tips | `apple-pay-korea.html` | `https://www.getkoreainside.com/apple-pay-korea.html` | `zh-tw/apple-pay-korea.html` | `https://www.getkoreainside.com/zh-tw/apple-pay-korea.html` | COMPLETE | Production verified in the 2026-10-01 technical release QA. |
| Legal | `affiliate-disclosure.html` | `https://www.getkoreainside.com/affiliate-disclosure.html` | — | — | EXCLUDE | Legal / disclosure page. |
| Legal | `privacy.html` | `https://www.getkoreainside.com/privacy.html` | — | — | EXCLUDE | Legal / privacy page. |
| Technical | `fo-verify.html` | `https://www.getkoreainside.com/fo-verify.html` | — | — | EXCLUDE | Technical verification file. |

## Aggregate Counts

### Overall

| Measure | Count |
|---|---:|
| Taiwan localization target pages | 58 |
| Taiwan COMPLETE | 58 |
| Taiwan MISSING | 0 |
| Public EXCLUDE | 3 |

### By Category

| Category | Target pages | COMPLETE | MISSING |
|---|---:|---:|---:|
| Home | 1 | 1 | 0 |
| Discover | 2 | 2 | 0 |
| Travel — Area | 9 | 9 | 0 |
| Travel — Attraction | 2 | 2 | 0 |
| Stay — Hub / Decision / Comparison | 11 | 11 | 0 |
| Stay — Area / Hotel Detail | 10 | 10 | 0 |
| eSIM | 3 | 3 | 0 |
| Airport | 5 | 5 | 0 |
| Maps | 1 | 1 | 0 |
| Transport | 6 | 6 | 0 |
| Apps | 1 | 1 | 0 |
| Travel Tips | 7 | 7 | 0 |
| **Total** | **58** | **58** | **0** |

## Reconciliation Check

- Root public/root HTML: 61
- Taiwan localization targets: 58
- Public exclusions: 3
- Reconciliation: `58 + 3 = 61`
- Current completion condition is met: `MISSING = 0`.

## Historical Initial Taiwan Setup State — 2026-09-27

The following is the initial setup snapshot before the approved implementation. It is retained as history and does not describe the current Production-complete state.

- Taiwan Standard: created; repository-root canonical filename is `Korea_Inside_Taiwan_Localization_Standard.md`.
- Working documents: existing `md/작업자료/` directly; no Taiwan-language subfolder.
- Approved Public Copy folder: `md/승인본/대만어/`.
- Production folder: `zh-tw/`.
- Taiwan COMMON UI Golden Sample: NOT YET LOCKED.
- Public language-selector label: NOT YET LOCKED.
- Full 58-page Codex draft: NOT YET STARTED.
- Production publication: NOT YET STARTED.
