# Korea Inside — zh-TW Localization Audit & Correction Batch 01A

- Date: 2026-09-29
- Language: Traditional Chinese (Taiwan) / `zh-TW`
- Scope: 4 pages
- Status: **REVIEW COPY — AWAITING USER APPROVAL**
- Source baseline: Current English Production on `main`
- Purpose: Direct EN → zh-TW localization audit and exact correction copy
- HTML implementation: **NOT STARTED**
- Git / stage / commit / push / Production: **0**

## Batch scope

1. `zh-tw/accommodation.html`
2. `zh-tw/airport-bus.html`
3. `zh-tw/airport-transfer.html`
4. `zh-tw/airport.html`

---

# 1. Batch-level audit result

The four Taiwan pages were compared directly with their current English Production counterparts.

Checked together:

- page role
- factual meaning
- recommendation strength
- numbers / prices / times
- H1 / H2 / H3 structure
- section / FAQ structure
- Taiwan terminology
- literal-English / calque wording
- Mainland-Chinese residue
- internal localization routing
- canonical
- JSON-LD parse where present

## Result

- Major factual mismatch: **0**
- Major numeric mismatch: **0**
- Recommendation drift: **0**
- Large omission: **0**
- Large invention: **0**
- Structure mismatch: **0**
- Mainland-Chinese terminology defect: **0**
- Broken Taiwan internal routing detected: **0**
- Canonical defect detected: **0**
- JSON-LD parse error: **0**

Localization wording corrections required: **6**

---

# 2. `zh-tw/accommodation.html`

## Judgment

**FIX — wording only**

The page preserves the English facts, area ranking logic and recommendation strength correctly.  
Two expressions read like direct translations of English editorial phrasing rather than natural Taiwan travel copy.

### Correction 2.1

Context: Quick Answer

**OLD**

`多數第一次旅行，可以先從明洞考慮。重視夜生活可選弘大；帶重行李，首爾站與麻浦／孔德更方便；多數行程本來就在漢江以南，江南才更合理。`

**NEW**

`多數第一次旅行，可以先從明洞考慮。重視夜生活可選弘大；帶重行李，首爾站與麻浦／孔德更方便；如果多數行程本來就在漢江以南，住江南會比較順。`

Reason:
- `江南才更合理` is understandable but sounds translated.
- `住江南會比較順` expresses the same trip-fit judgment in natural Taiwan travel language.
- Recommendation strength is unchanged.

### Correction 2.2

Context: FAQ — airport-area hotel

**OLD**

`多數旅客住首爾市區比機場附近合適。抵達很晚、起飛很早，或只是短暫過夜轉機，進市區會增加不必要移動時，機場周邊飯店才更有意義。`

**NEW**

`多數旅客住首爾市區比機場附近合適。抵達很晚、起飛很早，或只是短暫過夜轉機，進市區會增加不必要移動時，這時才比較值得住機場附近。`

Reason:
- `飯店才更有意義` is a literal English-style construction.
- New wording preserves the conditional recommendation exactly.

### Protected wording

Do not rewrite other `合理`, `實際`, `直達 AREX` wording on this page unless separately approved.

---

# 3. `zh-tw/airport-bus.html`

## Judgment

**FIX — wording only**

Facts, operator distinctions, late-night logic and route-selection judgment are preserved correctly.  
Three sentences should be localized more naturally for Taiwan readers.

### Correction 3.1

Context: when airport bus is not the best choice

**OLD**

`巴士不會只因為直達就一定更好。如果最近的停靠站下車後仍很難走，AREX 對鐵路銜接方便的飯店可能更輕鬆。若比起價格更重視門到門，計程車更有意義；多人同行或有特殊行李，則可考慮預約車輛。完整取捨請參閱仁川機場交通指南。`

**NEW**

`巴士不會只因為直達就一定更好。如果最近的停靠站下車後仍很難走，AREX 對鐵路銜接方便的飯店可能更輕鬆。如果比起價格更在意一路直接到飯店門口，搭計程車通常更省事；多人同行或有特殊行李，則可考慮預約車輛。完整取捨請參閱仁川機場交通指南。`

Reason:
- `門到門，計程車更有意義` reads as an English calque.
- New wording turns the same trade-off into practical Taiwan travel language.

### Correction 3.2

Context: arrival-day payment backup

**OLD**

`另外準備一張付款卡與一些韓元，仍是抵達當天合理的備案。實際路線的售票機或人工櫃檯，才是最後確認依據。`

**NEW**

`抵達當天，另外準備一張付款卡和一些韓元，仍是比較穩妥的備案。實際路線的售票機或人工櫃檯，才是最後確認依據。`

Reason:
- `合理的備案` is understandable but stiff.
- `比較穩妥的備案` is more natural in Taiwan usage without changing the advice.

### Correction 3.3

Context: missed bus / next departure

**OLD**

`放棄這條路線前，先看下一班。等 20 分鐘可能仍比把行李搬到鐵路區輕鬆；若要等很久，AREX 或計程車可能更合理。`

**NEW**

`放棄這條路線前，先看下一班。等 20 分鐘可能仍比把行李搬到鐵路區輕鬆；如果下一班要等很久，改搭 AREX 或計程車可能更省事。`

Reason:
- `可能更合理` is translated editorial language.
- New wording describes the actual traveler decision directly.

### Protected wording

Do not globally replace:
- `機場利木津`
- `直達`
- `實際`
- other occurrences of `合理`

Some are contextual or tied to service naming and are not defects.

---

# 4. `zh-tw/airport-transfer.html`

## Judgment

**PASS — no wording correction required**

Direct comparison found:

- facts preserved
- fares preserved
- terminal numbers preserved
- taxi surcharge information preserved
- AREX times and prices preserved
- recommendation / trade-off strength preserved
- luggage-capacity judgment preserved
- Call Van vs private-transfer distinction preserved
- Taiwan wording acceptable

No correction should be made merely because expressions such as `門到門` or `實際` appear.  
In this page they are understandable and fit the transport context.

**Approved correction count for this page: 0**

---

# 5. `zh-tw/airport.html`

## Judgment

**FIX — wording only**

Facts and page structure are aligned with English.  
One sentence contains a clear literal translation.

### Correction 5.1

Context: boundary between Arrival Guide and Airport Guide

**OLD**

`若還在找轉機、入境審查、行李領取或海關說明，先看入境流程指南。本頁只從進入公共入境大廳、準備處理旅程最初實務步驟後開始。`

**NEW**

`若還在找轉機、入境審查、行李領取或海關說明，先看入境流程指南。本頁從進入公共入境大廳、準備處理抵達後的交通與其他實際事項開始。`

Reason:
- `旅程最初實務步驟` is unnatural Taiwan Chinese and directly reflects English `initial practical steps`.
- New wording explains the actual page boundary more naturally without adding a new fact.

---

# 6. Exact implementation summary

| Page | Result | Corrections |
|---|---|---:|
| `accommodation.html` | FIX | 2 |
| `airport-bus.html` | FIX | 3 |
| `airport-transfer.html` | PASS | 0 |
| `airport.html` | FIX | 1 |
| **TOTAL** |  | **6** |

---

# 7. Implementation lock after approval

If the user approves this file:

- these 6 NEW strings become exact implementation copy
- Codex must not retranslate them
- Codex must not globally replace similar words elsewhere
- PASS page `airport-transfer.html` must remain untouched
- facts / numbers / recommendation hierarchy must not change
- HTML structure / classes / ids / data-* must not change
- affiliate / tracking must not change
- canonical / hreflang must not change unless separately authorized
- no additional localization audit is required for these four pages after exact implementation

Only mechanical implementation QA is required after implementation.

---

# 8. Workflow state

This file covers only the first 4-page unit.

Next units are intentionally separate:

- Batch 01B: next 4 pages
- Batch 01C: next 4 pages

Do not wait for 12 pages before saving confirmed work.
