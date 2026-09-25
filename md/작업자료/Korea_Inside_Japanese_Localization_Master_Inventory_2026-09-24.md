# Korea Inside Japanese Localization Master Inventory

## Document Metadata

- Date: 2026-09-25
- Purpose: Track the Japanese localization status of the current public English page set.
- Scope: 58 localization-target pages plus 3 explicitly excluded public/root HTML files.
- Japanese folder: `/ja/`
- Japanese hreflang code: `ja`
- Current status: Production inventory; 35 Japanese pages are COMPLETE and 23 remain MISSING.
- Completion condition: Japanese `MISSING = 0`.

## Audit Basis

- The English root contains 61 public/root HTML files: 58 Japanese localization targets and 3 exclusions.
- All 58 target filenames exist under `/ja/`: 35 released Japanese pages and 23 unreleased local working copies.
- The remaining 23 `/ja/` working copies are byte-identical English copies. English-to-`ja` SHA-256 matches are 23/23 as of this update.
- A working copy is not a completed localization. Each remaining target stays `MISSING` until approved Japanese public copy is implemented, fully QA-verified, and released through the approved Production workflow.
- `ja/index.html`, `ja/dongdaemun-travel-guide.html`, `ja/where-to-stay-in-dongdaemun.html`, `ja/airport.html`, `ja/arrival.html`, `ja/airport-transfer.html`, `ja/arex.html`, `ja/airport-bus.html`, `ja/maps.html`, `ja/tmoney.html`, `ja/wowpass.html`, and `ja/tmoney-vs-wowpass.html` completed the approved Production workflow on 2026-09-24.
- `ja/taxi.html`, `ja/incheon-airport-private-transfer.html`, `ja/rental-car.html`, `ja/payments.html`, and `ja/korea-atm-foreign-cards.html` completed the approved Japanese Batch 4 Production workflow on 2026-09-25.
- `ja/foreign-credit-cards-korea.html`, `ja/card-declined-korea.html`, `ja/korean-online-payments-foreigners.html`, `ja/apple-pay-korea.html`, and `ja/checklist.html` completed the approved Japanese Batch 5 Production workflow on 2026-09-25.
- `ja/esim.html`, `ja/best-esim-for-korea.html`, `ja/korea-esim-with-phone-number.html`, and `ja/apps.html` completed the approved Japanese Batch 6 Production workflow on 2026-09-25.
- The remaining 23 `/ja/` working copies are not Production pages and are not linked as Japanese siblings.

## Status Definitions

- `COMPLETE`: Approved Japanese public copy has been exactly implemented and the page has completed the required QA and Production workflow.
- `MISSING`: Japanese localization is not complete. A local English working copy may exist.
- `EXCLUDE`: The file is outside the Japanese localization target set for the stated reason.

## Future URL and SEO Structure

- English pages remain at the root English URL.
- Released Japanese pages use `/ja/` after their approved Batch Production release.
- Japanese homepage URL: `https://www.getkoreainside.com/ja/`
- Japanese detail URL pattern: `https://www.getkoreainside.com/ja/FILENAME`
- Future multilingual SEO is applied per released Batch: self canonical and reciprocal `en`, `es`, `ja`, and `x-default` hreflang using only siblings that actually exist in Production.
- Japanese hreflang and sitemap entries exist only for the 35 released Japanese pages.

## Master Inventory

All `MISSING` rows below share this state: Japanese working copy exists; English exact copy; not localized; not Production.

| Category | English filename | English Production URL | Japanese filename | Japanese future URL | Status | Notes |
|---|---|---|---|---|---|---|
| Home | `index.html` | `https://www.getkoreainside.com/` | `ja/index.html` | `https://www.getkoreainside.com/ja/` | COMPLETE | Approved Japanese homepage released to Production on 2026-09-24. |
| Discover | `taste-korea.html` | `https://www.getkoreainside.com/taste-korea.html` | `ja/taste-korea.html` | `https://www.getkoreainside.com/ja/taste-korea.html` | MISSING | Common working-copy state applies. |
| Discover | `k-beauty.html` | `https://www.getkoreainside.com/k-beauty.html` | `ja/k-beauty.html` | `https://www.getkoreainside.com/ja/k-beauty.html` | MISSING | Common working-copy state applies. |
| Travel — Area | `hongdae-travel-guide.html` | `https://www.getkoreainside.com/hongdae-travel-guide.html` | `ja/hongdae-travel-guide.html` | `https://www.getkoreainside.com/ja/hongdae-travel-guide.html` | MISSING | Common working-copy state applies. |
| Travel — Area | `myeongdong-travel-guide.html` | `https://www.getkoreainside.com/myeongdong-travel-guide.html` | `ja/myeongdong-travel-guide.html` | `https://www.getkoreainside.com/ja/myeongdong-travel-guide.html` | COMPLETE | Japanese Batch 7 released to Production on 2026-09-25. |
| Travel — Area | `seongsu-travel-guide.html` | `https://www.getkoreainside.com/seongsu-travel-guide.html` | `ja/seongsu-travel-guide.html` | `https://www.getkoreainside.com/ja/seongsu-travel-guide.html` | COMPLETE | Japanese Batch 7 released to Production on 2026-09-25. |
| Travel — Area | `insadong-travel-guide.html` | `https://www.getkoreainside.com/insadong-travel-guide.html` | `ja/insadong-travel-guide.html` | `https://www.getkoreainside.com/ja/insadong-travel-guide.html` | COMPLETE | Japanese Batch 7 released to Production on 2026-09-25. |
| Travel — Area | `gangnam-travel-guide.html` | `https://www.getkoreainside.com/gangnam-travel-guide.html` | `ja/gangnam-travel-guide.html` | `https://www.getkoreainside.com/ja/gangnam-travel-guide.html` | COMPLETE | Japanese Batch 7 released to Production on 2026-09-25. |
| Travel — Area | `jamsil-travel-guide.html` | `https://www.getkoreainside.com/jamsil-travel-guide.html` | `ja/jamsil-travel-guide.html` | `https://www.getkoreainside.com/ja/jamsil-travel-guide.html` | COMPLETE | Japanese Batch 7 released to Production on 2026-09-25. |
| Travel — Area | `gongdeok-mapo-seoul-guide.html` | `https://www.getkoreainside.com/gongdeok-mapo-seoul-guide.html` | `ja/gongdeok-mapo-seoul-guide.html` | `https://www.getkoreainside.com/ja/gongdeok-mapo-seoul-guide.html` | COMPLETE | Japanese Batch 8 released to Production on 2026-09-25. |
| Travel — Area | `itaewon-travel-guide.html` | `https://www.getkoreainside.com/itaewon-travel-guide.html` | `ja/itaewon-travel-guide.html` | `https://www.getkoreainside.com/ja/itaewon-travel-guide.html` | COMPLETE | Japanese Batch 8 released to Production on 2026-09-25. |
| Travel — Area | `dongdaemun-travel-guide.html` | `https://www.getkoreainside.com/dongdaemun-travel-guide.html` | `ja/dongdaemun-travel-guide.html` | `https://www.getkoreainside.com/ja/dongdaemun-travel-guide.html` | COMPLETE | Golden Sample Batch 1 released to Production on 2026-09-24. |
| Travel — Attraction | `lotte-world-seoul.html` | `https://www.getkoreainside.com/lotte-world-seoul.html` | `ja/lotte-world-seoul.html` | `https://www.getkoreainside.com/ja/lotte-world-seoul.html` | COMPLETE | Japanese Batch 8 released to Production on 2026-09-25. |
| Travel — Attraction | `seoul-sky-guide.html` | `https://www.getkoreainside.com/seoul-sky-guide.html` | `ja/seoul-sky-guide.html` | `https://www.getkoreainside.com/ja/seoul-sky-guide.html` | COMPLETE | Japanese Batch 8 released to Production on 2026-09-25. |
| Stay — Hub / Decision / Comparison | `accommodation.html` | `https://www.getkoreainside.com/accommodation.html` | `ja/accommodation.html` | `https://www.getkoreainside.com/ja/accommodation.html` | MISSING | Common working-copy state applies. |
| Stay — Hub / Decision / Comparison | `hongdae-vs-myeongdong.html` | `https://www.getkoreainside.com/hongdae-vs-myeongdong.html` | `ja/hongdae-vs-myeongdong.html` | `https://www.getkoreainside.com/ja/hongdae-vs-myeongdong.html` | MISSING | Common working-copy state applies. |
| Stay — Hub / Decision / Comparison | `best-area-for-first-time-visitors-seoul.html` | `https://www.getkoreainside.com/best-area-for-first-time-visitors-seoul.html` | `ja/best-area-for-first-time-visitors-seoul.html` | `https://www.getkoreainside.com/ja/best-area-for-first-time-visitors-seoul.html` | MISSING | Common working-copy state applies. |
| Stay — Hub / Decision / Comparison | `best-area-for-families-seoul.html` | `https://www.getkoreainside.com/best-area-for-families-seoul.html` | `ja/best-area-for-families-seoul.html` | `https://www.getkoreainside.com/ja/best-area-for-families-seoul.html` | MISSING | Common working-copy state applies. |
| Stay — Hub / Decision / Comparison | `best-area-for-solo-travelers-seoul.html` | `https://www.getkoreainside.com/best-area-for-solo-travelers-seoul.html` | `ja/best-area-for-solo-travelers-seoul.html` | `https://www.getkoreainside.com/ja/best-area-for-solo-travelers-seoul.html` | MISSING | Common working-copy state applies. |
| Stay — Hub / Decision / Comparison | `best-area-for-couples-seoul.html` | `https://www.getkoreainside.com/best-area-for-couples-seoul.html` | `ja/best-area-for-couples-seoul.html` | `https://www.getkoreainside.com/ja/best-area-for-couples-seoul.html` | MISSING | Common working-copy state applies. |
| Stay — Hub / Decision / Comparison | `best-area-for-budget-travelers-seoul.html` | `https://www.getkoreainside.com/best-area-for-budget-travelers-seoul.html` | `ja/best-area-for-budget-travelers-seoul.html` | `https://www.getkoreainside.com/ja/best-area-for-budget-travelers-seoul.html` | MISSING | Common working-copy state applies. |
| Stay — Hub / Decision / Comparison | `best-area-for-shopping-seoul.html` | `https://www.getkoreainside.com/best-area-for-shopping-seoul.html` | `ja/best-area-for-shopping-seoul.html` | `https://www.getkoreainside.com/ja/best-area-for-shopping-seoul.html` | MISSING | Common working-copy state applies. |
| Stay — Hub / Decision / Comparison | `best-area-for-nightlife-seoul.html` | `https://www.getkoreainside.com/best-area-for-nightlife-seoul.html` | `ja/best-area-for-nightlife-seoul.html` | `https://www.getkoreainside.com/ja/best-area-for-nightlife-seoul.html` | MISSING | Common working-copy state applies. |
| Stay — Hub / Decision / Comparison | `best-area-for-luxury-hotels-seoul.html` | `https://www.getkoreainside.com/best-area-for-luxury-hotels-seoul.html` | `ja/best-area-for-luxury-hotels-seoul.html` | `https://www.getkoreainside.com/ja/best-area-for-luxury-hotels-seoul.html` | MISSING | Common working-copy state applies. |
| Stay — Hub / Decision / Comparison | `best-area-for-airport-access-seoul.html` | `https://www.getkoreainside.com/best-area-for-airport-access-seoul.html` | `ja/best-area-for-airport-access-seoul.html` | `https://www.getkoreainside.com/ja/best-area-for-airport-access-seoul.html` | MISSING | Common working-copy state applies. |
| Stay — Area / Hotel Detail | `where-to-stay-in-myeongdong.html` | `https://www.getkoreainside.com/where-to-stay-in-myeongdong.html` | `ja/where-to-stay-in-myeongdong.html` | `https://www.getkoreainside.com/ja/where-to-stay-in-myeongdong.html` | MISSING | Common working-copy state applies. |
| Stay — Area / Hotel Detail | `where-to-stay-in-hongdae.html` | `https://www.getkoreainside.com/where-to-stay-in-hongdae.html` | `ja/where-to-stay-in-hongdae.html` | `https://www.getkoreainside.com/ja/where-to-stay-in-hongdae.html` | MISSING | Common working-copy state applies. |
| Stay — Area / Hotel Detail | `hotels-near-seoul-station.html` | `https://www.getkoreainside.com/hotels-near-seoul-station.html` | `ja/hotels-near-seoul-station.html` | `https://www.getkoreainside.com/ja/hotels-near-seoul-station.html` | MISSING | Common working-copy state applies. |
| Stay — Area / Hotel Detail | `hotels-near-gongdeok-station.html` | `https://www.getkoreainside.com/hotels-near-gongdeok-station.html` | `ja/hotels-near-gongdeok-station.html` | `https://www.getkoreainside.com/ja/hotels-near-gongdeok-station.html` | MISSING | Common working-copy state applies. |
| Stay — Area / Hotel Detail | `where-to-stay-in-insadong.html` | `https://www.getkoreainside.com/where-to-stay-in-insadong.html` | `ja/where-to-stay-in-insadong.html` | `https://www.getkoreainside.com/ja/where-to-stay-in-insadong.html` | MISSING | Common working-copy state applies. |
| Stay — Area / Hotel Detail | `where-to-stay-in-jamsil.html` | `https://www.getkoreainside.com/where-to-stay-in-jamsil.html` | `ja/where-to-stay-in-jamsil.html` | `https://www.getkoreainside.com/ja/where-to-stay-in-jamsil.html` | MISSING | Common working-copy state applies. |
| Stay — Area / Hotel Detail | `where-to-stay-in-gangnam.html` | `https://www.getkoreainside.com/where-to-stay-in-gangnam.html` | `ja/where-to-stay-in-gangnam.html` | `https://www.getkoreainside.com/ja/where-to-stay-in-gangnam.html` | MISSING | Common working-copy state applies. |
| Stay — Area / Hotel Detail | `where-to-stay-in-dongdaemun.html` | `https://www.getkoreainside.com/where-to-stay-in-dongdaemun.html` | `ja/where-to-stay-in-dongdaemun.html` | `https://www.getkoreainside.com/ja/where-to-stay-in-dongdaemun.html` | COMPLETE | Golden Sample Batch 1 released to Production on 2026-09-24. |
| Stay — Area / Hotel Detail | `where-to-stay-in-seongsu.html` | `https://www.getkoreainside.com/where-to-stay-in-seongsu.html` | `ja/where-to-stay-in-seongsu.html` | `https://www.getkoreainside.com/ja/where-to-stay-in-seongsu.html` | MISSING | Common working-copy state applies. |
| Stay — Area / Hotel Detail | `where-to-stay-in-itaewon.html` | `https://www.getkoreainside.com/where-to-stay-in-itaewon.html` | `ja/where-to-stay-in-itaewon.html` | `https://www.getkoreainside.com/ja/where-to-stay-in-itaewon.html` | MISSING | Common working-copy state applies. |
| eSIM | `esim.html` | `https://www.getkoreainside.com/esim.html` | `ja/esim.html` | `https://www.getkoreainside.com/ja/esim.html` | COMPLETE | Japanese Batch 6 released to Production on 2026-09-25. |
| eSIM | `best-esim-for-korea.html` | `https://www.getkoreainside.com/best-esim-for-korea.html` | `ja/best-esim-for-korea.html` | `https://www.getkoreainside.com/ja/best-esim-for-korea.html` | COMPLETE | Japanese Batch 6 released to Production on 2026-09-25. |
| eSIM | `korea-esim-with-phone-number.html` | `https://www.getkoreainside.com/korea-esim-with-phone-number.html` | `ja/korea-esim-with-phone-number.html` | `https://www.getkoreainside.com/ja/korea-esim-with-phone-number.html` | COMPLETE | Japanese Batch 6 released to Production on 2026-09-25. |
| Airport | `airport.html` | `https://www.getkoreainside.com/airport.html` | `ja/airport.html` | `https://www.getkoreainside.com/ja/airport.html` | COMPLETE | Japanese Airport Batch 2 released to Production on 2026-09-24. |
| Airport | `arrival.html` | `https://www.getkoreainside.com/arrival.html` | `ja/arrival.html` | `https://www.getkoreainside.com/ja/arrival.html` | COMPLETE | Japanese Arrival Batch 2 released to Production on 2026-09-24. |
| Airport | `airport-transfer.html` | `https://www.getkoreainside.com/airport-transfer.html` | `ja/airport-transfer.html` | `https://www.getkoreainside.com/ja/airport-transfer.html` | COMPLETE | Japanese Airport Transfer Batch 2 released to Production on 2026-09-24. |
| Airport | `arex.html` | `https://www.getkoreainside.com/arex.html` | `ja/arex.html` | `https://www.getkoreainside.com/ja/arex.html` | COMPLETE | Japanese AREX Batch 2 released to Production on 2026-09-24. |
| Airport | `airport-bus.html` | `https://www.getkoreainside.com/airport-bus.html` | `ja/airport-bus.html` | `https://www.getkoreainside.com/ja/airport-bus.html` | COMPLETE | Japanese Batch 3 released to Production on 2026-09-24. |
| Maps | `maps.html` | `https://www.getkoreainside.com/maps.html` | `ja/maps.html` | `https://www.getkoreainside.com/ja/maps.html` | COMPLETE | Japanese Batch 3 released to Production on 2026-09-24. |
| Transport | `tmoney.html` | `https://www.getkoreainside.com/tmoney.html` | `ja/tmoney.html` | `https://www.getkoreainside.com/ja/tmoney.html` | COMPLETE | Japanese Batch 3 released to Production on 2026-09-24. |
| Transport | `wowpass.html` | `https://www.getkoreainside.com/wowpass.html` | `ja/wowpass.html` | `https://www.getkoreainside.com/ja/wowpass.html` | COMPLETE | Japanese Batch 3 released to Production on 2026-09-24. |
| Transport | `tmoney-vs-wowpass.html` | `https://www.getkoreainside.com/tmoney-vs-wowpass.html` | `ja/tmoney-vs-wowpass.html` | `https://www.getkoreainside.com/ja/tmoney-vs-wowpass.html` | COMPLETE | Japanese Batch 3 released to Production on 2026-09-24. |
| Transport | `taxi.html` | `https://www.getkoreainside.com/taxi.html` | `ja/taxi.html` | `https://www.getkoreainside.com/ja/taxi.html` | COMPLETE | Japanese Batch 4 released to Production on 2026-09-25. |
| Transport | `incheon-airport-private-transfer.html` | `https://www.getkoreainside.com/incheon-airport-private-transfer.html` | `ja/incheon-airport-private-transfer.html` | `https://www.getkoreainside.com/ja/incheon-airport-private-transfer.html` | COMPLETE | Japanese Batch 4 released to Production on 2026-09-25. |
| Transport | `rental-car.html` | `https://www.getkoreainside.com/rental-car.html` | `ja/rental-car.html` | `https://www.getkoreainside.com/ja/rental-car.html` | COMPLETE | Japanese Batch 4 released to Production on 2026-09-25. |
| Apps | `apps.html` | `https://www.getkoreainside.com/apps.html` | `ja/apps.html` | `https://www.getkoreainside.com/ja/apps.html` | COMPLETE | Japanese Batch 6 released to Production on 2026-09-25. |
| Travel Tips | `checklist.html` | `https://www.getkoreainside.com/checklist.html` | `ja/checklist.html` | `https://www.getkoreainside.com/ja/checklist.html` | COMPLETE | Japanese Batch 5 released to Production on 2026-09-25. |
| Travel Tips | `payments.html` | `https://www.getkoreainside.com/payments.html` | `ja/payments.html` | `https://www.getkoreainside.com/ja/payments.html` | COMPLETE | Japanese Batch 4 released to Production on 2026-09-25. |
| Travel Tips | `foreign-credit-cards-korea.html` | `https://www.getkoreainside.com/foreign-credit-cards-korea.html` | `ja/foreign-credit-cards-korea.html` | `https://www.getkoreainside.com/ja/foreign-credit-cards-korea.html` | COMPLETE | Japanese Batch 5 released to Production on 2026-09-25. |
| Travel Tips | `card-declined-korea.html` | `https://www.getkoreainside.com/card-declined-korea.html` | `ja/card-declined-korea.html` | `https://www.getkoreainside.com/ja/card-declined-korea.html` | COMPLETE | Japanese Batch 5 released to Production on 2026-09-25. |
| Travel Tips | `korean-online-payments-foreigners.html` | `https://www.getkoreainside.com/korean-online-payments-foreigners.html` | `ja/korean-online-payments-foreigners.html` | `https://www.getkoreainside.com/ja/korean-online-payments-foreigners.html` | COMPLETE | Japanese Batch 5 released to Production on 2026-09-25. |
| Travel Tips | `korea-atm-foreign-cards.html` | `https://www.getkoreainside.com/korea-atm-foreign-cards.html` | `ja/korea-atm-foreign-cards.html` | `https://www.getkoreainside.com/ja/korea-atm-foreign-cards.html` | COMPLETE | Japanese Batch 4 released to Production on 2026-09-25. |
| Travel Tips | `apple-pay-korea.html` | `https://www.getkoreainside.com/apple-pay-korea.html` | `ja/apple-pay-korea.html` | `https://www.getkoreainside.com/ja/apple-pay-korea.html` | COMPLETE | Japanese Batch 5 released to Production on 2026-09-25. |
| Legal | `affiliate-disclosure.html` | `https://www.getkoreainside.com/affiliate-disclosure.html` | — | — | EXCLUDE | Legal / disclosure page. |
| Legal | `privacy.html` | `https://www.getkoreainside.com/privacy.html` | — | — | EXCLUDE | Legal / privacy page. |
| Technical | `fo-verify.html` | `https://www.getkoreainside.com/fo-verify.html` | — | — | EXCLUDE | Technical verification file. |

## Aggregate Counts

### Overall

| Measure | Count |
|---|---:|
| Japanese localization target pages | 58 |
| Japanese COMPLETE | 35 |
| Japanese MISSING | 23 |
| Public EXCLUDE | 3 |

### By Category

| Category | Target pages | COMPLETE | MISSING |
|---|---:|---:|---:|
| Home | 1 | 1 | 0 |
| Discover | 2 | 0 | 2 |
| Travel — Area | 9 | 8 | 1 |
| Travel — Attraction | 2 | 2 | 0 |
| Stay — Hub / Decision / Comparison | 11 | 0 | 11 |
| Stay — Area / Hotel Detail | 10 | 1 | 9 |
| eSIM | 3 | 3 | 0 |
| Airport | 5 | 5 | 0 |
| Maps | 1 | 1 | 0 |
| Transport | 6 | 6 | 0 |
| Apps | 1 | 1 | 0 |
| Travel Tips | 7 | 7 | 0 |
| **Total** | **58** | **35** | **23** |

## Reconciliation Check

- Root public/root HTML: 61
- Japanese localization targets: 58
- Public exclusions: 3
- Reconciliation: `58 + 3 = 61`
- Current completion condition is not met: `MISSING = 23`.
