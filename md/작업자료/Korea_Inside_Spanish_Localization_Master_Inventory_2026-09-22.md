# Korea Inside Spanish Localization Master Inventory

## Document Metadata

- Date: 2026-09-22
- Repository: `C:\Projects\Koreainside`
- Production: `https://www.getkoreainside.com/`
- Production commit verified: `70422bc850df115a4518ab26480e872b1e39bfb1`
- Scope: Current English public detail HTML pages and their same-filename Spanish siblings
- Status values: `COMPLETE`, `MISSING`, `EXCLUDE`

## Audit Basis

This inventory cross-checks three current-state sources:

1. Repository root English `*.html` files and tracked public paths
2. Local and Production `sitemap.xml` URLs
3. Production Global Navigation and anchor-based internal links across the English public pages

Audit results:

- Repository root English HTML files: 61
- English Production sitemap URLs: 60
- Spanish Production sitemap URLs: 43
- English sitemap HTTP 200: 60/60
- English localization-target detail pages: 57
- Target pages linked from Production Global Navigation: 41
- Target pages linked from Production Navigation or other internal anchors: 57/57
- Local sitemap versus Production sitemap difference: 0
- Spanish siblings meeting COMPLETE criteria: 43
- COMPLETE criteria verified: file exists, Production HTTP 200, Spanish self canonical, reciprocal `en` / `es` / `x-default`

The COMPLETE total is the 13-page cohort deployed from commit `b94220a`, the two earlier Dongdaemun Travel/Stay pilots, the five Stay Area/Hotel pages deployed from commit `31a0c25`, the six-page Stay 5 plus Hongdae Travel batch deployed from commit `43b6db5`, the six-page Stay 6 batch deployed from commit `7d92e37`, the six-page Airport 6 batch deployed from commit `f3af0ac`, and the five-page Discover + eSIM batch deployed from commit `70422bc` on 2026-09-23. The already deployed Spanish pages are inventory records only. This audit does not reopen or modify them.

## Master Inventory

Notes use these source markers:

- `Nav`: linked directly from the Production Global Navigation.
- `Internal`: linked from at least one Production English public page but not directly listed in the Global Navigation.
- Every non-EXCLUDE row is present in the English Production sitemap and returned HTTP 200 during this audit.

| Category | English filename | English Production URL | Spanish filename | Spanish Production URL | Status: COMPLETE / MISSING / EXCLUDE | Notes |
|---|---|---|---|---|---|---|
| Discover | `taste-korea.html` | `https://www.getkoreainside.com/taste-korea.html` | `es/taste-korea.html` | `https://www.getkoreainside.com/es/taste-korea.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Discover | `k-beauty.html` | `https://www.getkoreainside.com/k-beauty.html` | `es/k-beauty.html` | `https://www.getkoreainside.com/es/k-beauty.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Travel — Area | `hongdae-travel-guide.html` | `https://www.getkoreainside.com/hongdae-travel-guide.html` | `es/hongdae-travel-guide.html` | `https://www.getkoreainside.com/es/hongdae-travel-guide.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Travel — Area | `myeongdong-travel-guide.html` | `https://www.getkoreainside.com/myeongdong-travel-guide.html` | `es/myeongdong-travel-guide.html` | `https://www.getkoreainside.com/es/myeongdong-travel-guide.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Travel — Area | `seongsu-travel-guide.html` | `https://www.getkoreainside.com/seongsu-travel-guide.html` | `es/seongsu-travel-guide.html` | `https://www.getkoreainside.com/es/seongsu-travel-guide.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Travel — Area | `insadong-travel-guide.html` | `https://www.getkoreainside.com/insadong-travel-guide.html` | `es/insadong-travel-guide.html` | `https://www.getkoreainside.com/es/insadong-travel-guide.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Travel — Area | `gangnam-travel-guide.html` | `https://www.getkoreainside.com/gangnam-travel-guide.html` | `es/gangnam-travel-guide.html` | `https://www.getkoreainside.com/es/gangnam-travel-guide.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Travel — Area | `jamsil-travel-guide.html` | `https://www.getkoreainside.com/jamsil-travel-guide.html` | `es/jamsil-travel-guide.html` | `https://www.getkoreainside.com/es/jamsil-travel-guide.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Travel — Area | `gongdeok-mapo-seoul-guide.html` | `https://www.getkoreainside.com/gongdeok-mapo-seoul-guide.html` | `es/gongdeok-mapo-seoul-guide.html` | `https://www.getkoreainside.com/es/gongdeok-mapo-seoul-guide.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Travel — Area | `itaewon-travel-guide.html` | `https://www.getkoreainside.com/itaewon-travel-guide.html` | `es/itaewon-travel-guide.html` | `https://www.getkoreainside.com/es/itaewon-travel-guide.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Travel — Area | `dongdaemun-travel-guide.html` | `https://www.getkoreainside.com/dongdaemun-travel-guide.html` | `es/dongdaemun-travel-guide.html` | `https://www.getkoreainside.com/es/dongdaemun-travel-guide.html` | COMPLETE | Nav; Spanish Golden Sample; Production HTTP 200 and canonical/hreflang PASS. |
| Travel — Attraction | `lotte-world-seoul.html` | `https://www.getkoreainside.com/lotte-world-seoul.html` | `es/lotte-world-seoul.html` | `https://www.getkoreainside.com/es/lotte-world-seoul.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Travel — Attraction | `seoul-sky-guide.html` | `https://www.getkoreainside.com/seoul-sky-guide.html` | `es/seoul-sky-guide.html` | `https://www.getkoreainside.com/es/seoul-sky-guide.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Hub / Decision / Comparison | `accommodation.html` | `https://www.getkoreainside.com/accommodation.html` | `es/accommodation.html` | `https://www.getkoreainside.com/es/accommodation.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Hub / Decision / Comparison | `hongdae-vs-myeongdong.html` | `https://www.getkoreainside.com/hongdae-vs-myeongdong.html` | `es/hongdae-vs-myeongdong.html` | `https://www.getkoreainside.com/es/hongdae-vs-myeongdong.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Hub / Decision / Comparison | `best-area-for-first-time-visitors-seoul.html` | `https://www.getkoreainside.com/best-area-for-first-time-visitors-seoul.html` | `es/best-area-for-first-time-visitors-seoul.html` | `https://www.getkoreainside.com/es/best-area-for-first-time-visitors-seoul.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Hub / Decision / Comparison | `best-area-for-families-seoul.html` | `https://www.getkoreainside.com/best-area-for-families-seoul.html` | `es/best-area-for-families-seoul.html` | `https://www.getkoreainside.com/es/best-area-for-families-seoul.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Hub / Decision / Comparison | `best-area-for-solo-travelers-seoul.html` | `https://www.getkoreainside.com/best-area-for-solo-travelers-seoul.html` | `es/best-area-for-solo-travelers-seoul.html` | `https://www.getkoreainside.com/es/best-area-for-solo-travelers-seoul.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Hub / Decision / Comparison | `best-area-for-couples-seoul.html` | `https://www.getkoreainside.com/best-area-for-couples-seoul.html` | `es/best-area-for-couples-seoul.html` | `https://www.getkoreainside.com/es/best-area-for-couples-seoul.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Hub / Decision / Comparison | `best-area-for-budget-travelers-seoul.html` | `https://www.getkoreainside.com/best-area-for-budget-travelers-seoul.html` | `es/best-area-for-budget-travelers-seoul.html` | `https://www.getkoreainside.com/es/best-area-for-budget-travelers-seoul.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Hub / Decision / Comparison | `best-area-for-shopping-seoul.html` | `https://www.getkoreainside.com/best-area-for-shopping-seoul.html` | `es/best-area-for-shopping-seoul.html` | `https://www.getkoreainside.com/es/best-area-for-shopping-seoul.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Hub / Decision / Comparison | `best-area-for-nightlife-seoul.html` | `https://www.getkoreainside.com/best-area-for-nightlife-seoul.html` | `es/best-area-for-nightlife-seoul.html` | `https://www.getkoreainside.com/es/best-area-for-nightlife-seoul.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Hub / Decision / Comparison | `best-area-for-luxury-hotels-seoul.html` | `https://www.getkoreainside.com/best-area-for-luxury-hotels-seoul.html` | `es/best-area-for-luxury-hotels-seoul.html` | `https://www.getkoreainside.com/es/best-area-for-luxury-hotels-seoul.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Hub / Decision / Comparison | `best-area-for-airport-access-seoul.html` | `https://www.getkoreainside.com/best-area-for-airport-access-seoul.html` | `es/best-area-for-airport-access-seoul.html` | `https://www.getkoreainside.com/es/best-area-for-airport-access-seoul.html` | COMPLETE | Internal; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Area / Hotel Detail | `where-to-stay-in-myeongdong.html` | `https://www.getkoreainside.com/where-to-stay-in-myeongdong.html` | `es/where-to-stay-in-myeongdong.html` | `https://www.getkoreainside.com/es/where-to-stay-in-myeongdong.html` | COMPLETE | Internal; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Area / Hotel Detail | `where-to-stay-in-hongdae.html` | `https://www.getkoreainside.com/where-to-stay-in-hongdae.html` | `es/where-to-stay-in-hongdae.html` | `https://www.getkoreainside.com/es/where-to-stay-in-hongdae.html` | COMPLETE | Internal; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Area / Hotel Detail | `hotels-near-seoul-station.html` | `https://www.getkoreainside.com/hotels-near-seoul-station.html` | `es/hotels-near-seoul-station.html` | `https://www.getkoreainside.com/es/hotels-near-seoul-station.html` | COMPLETE | Internal; hotel detail page; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Area / Hotel Detail | `hotels-near-gongdeok-station.html` | `https://www.getkoreainside.com/hotels-near-gongdeok-station.html` | `es/hotels-near-gongdeok-station.html` | `https://www.getkoreainside.com/es/hotels-near-gongdeok-station.html` | COMPLETE | Internal; hotel detail page; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Area / Hotel Detail | `where-to-stay-in-insadong.html` | `https://www.getkoreainside.com/where-to-stay-in-insadong.html` | `es/where-to-stay-in-insadong.html` | `https://www.getkoreainside.com/es/where-to-stay-in-insadong.html` | COMPLETE | Internal; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Area / Hotel Detail | `where-to-stay-in-jamsil.html` | `https://www.getkoreainside.com/where-to-stay-in-jamsil.html` | `es/where-to-stay-in-jamsil.html` | `https://www.getkoreainside.com/es/where-to-stay-in-jamsil.html` | COMPLETE | Internal; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Area / Hotel Detail | `where-to-stay-in-gangnam.html` | `https://www.getkoreainside.com/where-to-stay-in-gangnam.html` | `es/where-to-stay-in-gangnam.html` | `https://www.getkoreainside.com/es/where-to-stay-in-gangnam.html` | COMPLETE | Internal; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Area / Hotel Detail | `where-to-stay-in-dongdaemun.html` | `https://www.getkoreainside.com/where-to-stay-in-dongdaemun.html` | `es/where-to-stay-in-dongdaemun.html` | `https://www.getkoreainside.com/es/where-to-stay-in-dongdaemun.html` | COMPLETE | Internal; Spanish Stay Golden Sample; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Area / Hotel Detail | `where-to-stay-in-seongsu.html` | `https://www.getkoreainside.com/where-to-stay-in-seongsu.html` | `es/where-to-stay-in-seongsu.html` | `https://www.getkoreainside.com/es/where-to-stay-in-seongsu.html` | COMPLETE | Internal; Production HTTP 200 and canonical/hreflang PASS. |
| Stay — Area / Hotel Detail | `where-to-stay-in-itaewon.html` | `https://www.getkoreainside.com/where-to-stay-in-itaewon.html` | `es/where-to-stay-in-itaewon.html` | `https://www.getkoreainside.com/es/where-to-stay-in-itaewon.html` | COMPLETE | Internal; Production HTTP 200 and canonical/hreflang PASS. |
| eSIM | `esim.html` | `https://www.getkoreainside.com/esim.html` | `es/esim.html` | `https://www.getkoreainside.com/es/esim.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| eSIM | `best-esim-for-korea.html` | `https://www.getkoreainside.com/best-esim-for-korea.html` | `es/best-esim-for-korea.html` | `https://www.getkoreainside.com/es/best-esim-for-korea.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| eSIM | `korea-esim-with-phone-number.html` | `https://www.getkoreainside.com/korea-esim-with-phone-number.html` | `es/korea-esim-with-phone-number.html` | `https://www.getkoreainside.com/es/korea-esim-with-phone-number.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Airport | `airport.html` | `https://www.getkoreainside.com/airport.html` | `es/airport.html` | `https://www.getkoreainside.com/es/airport.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Airport | `arrival.html` | `https://www.getkoreainside.com/arrival.html` | `es/arrival.html` | `https://www.getkoreainside.com/es/arrival.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Airport | `airport-transfer.html` | `https://www.getkoreainside.com/airport-transfer.html` | `es/airport-transfer.html` | `https://www.getkoreainside.com/es/airport-transfer.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Airport | `arex.html` | `https://www.getkoreainside.com/arex.html` | `es/arex.html` | `https://www.getkoreainside.com/es/arex.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Airport | `airport-bus.html` | `https://www.getkoreainside.com/airport-bus.html` | `es/airport-bus.html` | `https://www.getkoreainside.com/es/airport-bus.html` | COMPLETE | Nav; Production HTTP 200 and canonical/hreflang PASS. |
| Maps | `maps.html` | `https://www.getkoreainside.com/maps.html` | `es/maps.html` | `https://www.getkoreainside.com/es/maps.html` | MISSING | Nav; Spanish sibling absent. |
| Transport | `tmoney.html` | `https://www.getkoreainside.com/tmoney.html` | `es/tmoney.html` | `https://www.getkoreainside.com/es/tmoney.html` | MISSING | Nav; Spanish sibling absent. |
| Transport | `wowpass.html` | `https://www.getkoreainside.com/wowpass.html` | `es/wowpass.html` | `https://www.getkoreainside.com/es/wowpass.html` | MISSING | Nav; Spanish sibling absent. |
| Transport | `tmoney-vs-wowpass.html` | `https://www.getkoreainside.com/tmoney-vs-wowpass.html` | `es/tmoney-vs-wowpass.html` | `https://www.getkoreainside.com/es/tmoney-vs-wowpass.html` | MISSING | Nav; comparison page; Spanish sibling absent. |
| Transport | `taxi.html` | `https://www.getkoreainside.com/taxi.html` | `es/taxi.html` | `https://www.getkoreainside.com/es/taxi.html` | MISSING | Nav; Spanish sibling absent. |
| Transport | `incheon-airport-private-transfer.html` | `https://www.getkoreainside.com/incheon-airport-private-transfer.html` | `es/incheon-airport-private-transfer.html` | `https://www.getkoreainside.com/es/incheon-airport-private-transfer.html` | COMPLETE | Nav; service/affiliate transport page; Production HTTP 200 and canonical/hreflang PASS. |
| Transport | `rental-car.html` | `https://www.getkoreainside.com/rental-car.html` | `es/rental-car.html` | `https://www.getkoreainside.com/es/rental-car.html` | MISSING | Nav; Spanish sibling absent. |
| Apps | `apps.html` | `https://www.getkoreainside.com/apps.html` | `es/apps.html` | `https://www.getkoreainside.com/es/apps.html` | MISSING | Nav; Spanish sibling absent. |
| Travel Tips | `checklist.html` | `https://www.getkoreainside.com/checklist.html` | `es/checklist.html` | `https://www.getkoreainside.com/es/checklist.html` | MISSING | Nav; Spanish sibling absent. |
| Travel Tips | `payments.html` | `https://www.getkoreainside.com/payments.html` | `es/payments.html` | `https://www.getkoreainside.com/es/payments.html` | MISSING | Nav; payment hub; Spanish sibling absent. |
| Travel Tips | `foreign-credit-cards-korea.html` | `https://www.getkoreainside.com/foreign-credit-cards-korea.html` | `es/foreign-credit-cards-korea.html` | `https://www.getkoreainside.com/es/foreign-credit-cards-korea.html` | MISSING | Internal; Spanish sibling absent. |
| Travel Tips | `card-declined-korea.html` | `https://www.getkoreainside.com/card-declined-korea.html` | `es/card-declined-korea.html` | `https://www.getkoreainside.com/es/card-declined-korea.html` | MISSING | Internal; Spanish sibling absent. |
| Travel Tips | `korean-online-payments-foreigners.html` | `https://www.getkoreainside.com/korean-online-payments-foreigners.html` | `es/korean-online-payments-foreigners.html` | `https://www.getkoreainside.com/es/korean-online-payments-foreigners.html` | MISSING | Internal; Spanish sibling absent. |
| Travel Tips | `korea-atm-foreign-cards.html` | `https://www.getkoreainside.com/korea-atm-foreign-cards.html` | `es/korea-atm-foreign-cards.html` | `https://www.getkoreainside.com/es/korea-atm-foreign-cards.html` | MISSING | Internal; Spanish sibling absent. |
| Travel Tips | `apple-pay-korea.html` | `https://www.getkoreainside.com/apple-pay-korea.html` | `es/apple-pay-korea.html` | `https://www.getkoreainside.com/es/apple-pay-korea.html` | MISSING | Internal; Spanish sibling absent. |
| Home / Scope Exclusion | `index.html` | `https://www.getkoreainside.com/` | — | — | EXCLUDE | Public homepage, not an English detail page under the approved inventory scope; requires a separate homepage localization decision. |
| Legal | `affiliate-disclosure.html` | `https://www.getkoreainside.com/affiliate-disclosure.html` | — | — | EXCLUDE | Public legal/disclosure page; explicitly outside public-detail localization scope. |
| Legal | `privacy.html` | `https://www.getkoreainside.com/privacy.html` | — | — | EXCLUDE | Public legal/privacy page; explicitly outside public-detail localization scope. |
| Technical | `fo-verify.html` | `https://www.getkoreainside.com/fo-verify.html` | — | — | EXCLUDE | Website-ownership verification file; no canonical, no sitemap entry, no localization role. |

## Repository-only Non-public HTML

These files were separated from the public-content inventory and are not counted in COMPLETE, MISSING, or the public EXCLUDE total above:

- `admin/accommodation-analyzer.html`
- `admin/category-map.html`
- `korea-inside-admin/src/index.html`
- `_CleanTemp/**/*.html`

Reasons:

- `admin/` and `korea-inside-admin/` are administrator prototypes/app sources and are excluded by `.vercelignore`.
- `_CleanTemp/` is an untracked, user-owned temporary copy tree, not part of Production commit `31a0c25`; it remains protected and untouched.

## Aggregate Counts

### Overall

| Measure | Count |
|---|---:|
| English public localization-target detail pages | 57 |
| Spanish COMPLETE | 43 |
| Spanish MISSING | 14 |
| Public/root EXCLUDE | 4 |

### By Category

| Category | Total target pages | COMPLETE | MISSING |
|---|---:|---:|---:|
| Discover | 2 | 2 | 0 |
| Travel | 11 | 11 | 0 |
| Stay | 21 | 21 | 0 |
| eSIM | 3 | 3 | 0 |
| Airport | 5 | 5 | 0 |
| Maps | 1 | 0 | 1 |
| Transport | 6 | 1 | 5 |
| Apps | 1 | 0 | 1 |
| Travel Tips | 7 | 0 | 7 |
| **Total** | **57** | **43** | **14** |

### Stay Detail

| Stay group | Total | COMPLETE | MISSING |
|---|---:|---:|---:|
| Stay Hub / Decision / Comparison | 11 | 11 | 0 |
| Area Stay / Hotel Detail | 10 | 10 | 0 |
| **Stay total** | **21** | **21** | **0** |

Confirmed COMPLETE Area Stay pages:

- Dongdaemun
- Myeongdong
- Seongsu
- Insadong
- Gangnam
- Hongdae
- Jamsil
- Itaewon
- Seoul Station
- Gongdeok Station

### Travel Detail

| Travel group | Total | COMPLETE | MISSING |
|---|---:|---:|---:|
| Seoul Area Travel Guides | 9 | 9 | 0 |
| Attractions / Places | 2 | 2 | 0 |
| **Travel total** | **11** | **11** | **0** |

All 11 Travel pages are COMPLETE. This result is based on the actual repository and Production state, not an inferred page list.

## Recommended Batch Plan

The remaining plan covers all 14 MISSING pages exactly once. It follows HTML-first localization and uses focused batches where practical.

| Order | Batch | Pages | Count | Reason |
|---:|---|---|---:|---|
| 1 | Maps + Apps | `maps.html`, `apps.html` | 2 | Completes two high-utility Navigation groups. |
| 2 | Transport remainder | `tmoney.html`, `wowpass.html`, `tmoney-vs-wowpass.html`, `taxi.html`, `rental-car.html` | 5 | Keeps transport facts and comparisons in one focused QA batch. |
| 3 | Travel Tips / Payments | `checklist.html`, `payments.html`, `foreign-credit-cards-korea.html`, `card-declined-korea.html`, `korean-online-payments-foreigners.html`, `korea-atm-foreign-cards.html`, `apple-pay-korea.html` | 7 | Completes the payment/troubleshooting cluster and its internal-link network. |
|  | **Total** |  | **14** | All remaining MISSING detail pages covered. |

## Batch Implementation Contract Principles

The future batches should use these fixed rules:

- HTML-first localization: copy the approved English HTML structure to `/es/` with the same filename.
- Preserve structure, shared CSS/JS/images, affiliate URLs, tracking attributes, numeric facts and interactive behavior.
- Localize only user-visible English copy after user approval; Codex does not independently translate or rewrite public copy.
- Use Spanish internal links only when the sibling actually exists; otherwise keep the English fallback.
- Do not create future Spanish 404 links.
- Do not use repeated large Review MD generation as the default production workflow.
- Treat page omission prevention and batch-level inventory reconciliation as release blockers.
- Keep the existing 43 COMPLETE Spanish pages unchanged unless a separately approved correction is required.

## Reconciliation Check

- Target total: `57`
- COMPLETE + MISSING: `43 + 14 = 57`
- Category target totals: `2 + 11 + 21 + 3 + 5 + 1 + 6 + 1 + 7 = 57`
- Batch page totals: `2 + 5 + 7 = 14`
- Stay total: `21 = 21 COMPLETE + 0 MISSING`
- Travel total: `11 = 11 COMPLETE + 0 MISSING`
