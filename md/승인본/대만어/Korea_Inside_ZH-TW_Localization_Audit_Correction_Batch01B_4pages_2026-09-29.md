# Korea Inside — zh-TW Localization Audit & Correction Batch 01B

- Date: 2026-09-29
- Language: Traditional Chinese (Taiwan) / `zh-TW`
- Scope: 4 pages
- Status: **REVIEW COPY — AWAITING USER APPROVAL**
- Source baseline: Current English Production on `main`
- Purpose: Direct EN → zh-TW localization audit and exact correction copy
- HTML implementation: **NOT STARTED**
- Git / stage / commit / push / Production: **0**

## Batch scope
1. `zh-tw/apple-pay-korea.html`
2. `zh-tw/apps.html`
3. `zh-tw/arex.html`
4. `zh-tw/arrival.html`

## 1. Batch-level audit result

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

# 2. `zh-tw/apple-pay-korea.html`

## Judgment
**PASS — no wording correction required**

The page preserves the English source correctly and the Taiwan wording is already natural enough for publication.

Confirmed:
- overseas Apple Pay card logic preserved
- Hyundai Card distinction preserved
- Apple Wallet T-money distinction preserved
- direct Wallet top-up vs MobileTmoney top-up distinction preserved
- foreign-card network conditions preserved
- iPhone / Apple Watch device rule preserved
- recommendation strength preserved
- FAQ wording aligned with the page judgment

Expressions such as `合理的做法`, `實際目標`, `海外卡`, `加值` are contextually natural here and should not be changed merely for stylistic variation.

**Approved correction count for this page: 0**

# 3. `zh-tw/apps.html`

## Judgment
**FIX — wording only**

### Correction 3.1
Context: taxi-app section introduction

**OLD**
`k.ride 為國際旅客設計；若已有帳戶、對介面熟悉，Kakao T 與 Uber Taxi 就更有意義。`

**NEW**
`k.ride 是為國際旅客設計的；如果已經有帳戶、也熟悉操作介面，Kakao T 或 Uber Taxi 也會更順手。`

Reason:
- `就更有意義` is a direct English-style construction.
- `也會更順手` preserves the same practical judgment.

### Correction 3.2
Context: food-delivery section heading

**OLD**
`Shuttle 能減少部分旅客使用障礙`

**NEW**
`Shuttle 能降低部分旅客的使用門檻`

Reason:
- `降低使用門檻` is more natural Taiwan usage for account/payment/access friction.

### Protected wording
Do not rewrite app names, payment conditions, Korean-phone-number conditions, local-address requirements, supported-card claims, or unrelated `實際` wording.

# 4. `zh-tw/arex.html`

## Judgment
**FIX — wording only**

### Correction 4.1
Context: KTX transfer

**OLD**
`若下一段要從首爾站搭 KTX，直達列車通常是合理的機場交通選擇。AREX 抵達時間不等於能搭上 KTX 的時間：還得離開位於深處的 AREX 月台、穿越車站，再到正確的國鐵月台。`

**NEW**
`若下一段要從首爾站搭 KTX，AREX 直達列車通常很適合。AREX 抵達時間不等於能搭上 KTX 的時間：還得離開位於深處的 AREX 月台、穿越車站，再到正確的國鐵月台。`

Reason:
- `合理的機場交通選擇` is formal translation language.
- `通常很適合` preserves the conditional recommendation.

### Correction 4.2
Context: planning around the last train

**OLD**
`將結果與表定落地時間比較，並保留合理的延誤餘裕。`

**NEW**
`將結果與表定落地時間比較，並預留適當的延誤緩衝時間。`

Reason:
- `合理的延誤餘裕` is stiff.
- New wording is clearer and more natural in Taiwan Chinese.

### Protected wording
Do not alter AREX Express / All-Stop distinctions, fares, times, terminal numbers, T-money rules, foreign-card support, City Airport Terminal conditions, or contextual `實際`.

# 5. `zh-tw/arrival.html`

## Judgment
**FIX — wording only**

The repeated `門到門路程` wording reads like a direct translation of English `door-to-door journey`.

### Correction 5.1
Context: link to Airport Transfer Guide

**OLD**
`如果只剩如何前往住宿處還沒決定，可以參考機場交通指南，以完整的門到門路程比較 AREX、機場巴士、計程車與預約接送。`

**NEW**
`如果只剩如何前往住宿處還沒決定，可以參考機場交通指南，直接比較 AREX、機場巴士、計程車與預約接送一路到住宿處的完整行程。`

### Correction 5.2
Context: FAQ / arrival-hall decision

**OLD**
`抵達入境大廳後再決定。適合的方式取決於實際住宿地點、抵達時間、行李量與同行人數。另有機場交通指南，以到住宿處的完整路程比較 AREX、機場巴士、計程車及預約接送。`

**NEW**
`抵達入境大廳後再決定。適合的方式取決於實際住宿地點、抵達時間、行李量與同行人數。另有機場交通指南，可比較 AREX、機場巴士、計程車及預約接送一路到住宿處的完整行程。`

### FAQ/schema rule
If this second sentence is mirrored in FAQPage JSON-LD:
- visible FAQ and JSON-LD must receive the same approved wording
- Question count must remain unchanged
- schema structure must not change

# 6. Exact implementation summary

| Page | Result | Corrections |
|---|---|---:|
| `apple-pay-korea.html` | PASS | 0 |
| `apps.html` | FIX | 2 |
| `arex.html` | FIX | 2 |
| `arrival.html` | FIX | 2 |
| **TOTAL** |  | **6** |

# 7. Implementation lock after approval

If the user approves this file:
- these 6 NEW strings become exact implementation copy
- Codex must not retranslate or embellish them
- Codex must not globally replace `合理`, `有意義`, `實際`, `門到門` or similar expressions elsewhere
- PASS page `apple-pay-korea.html` must remain untouched
- facts / numbers / prices / times / recommendation strength must remain unchanged
- HTML structure / classes / ids / data-* must remain unchanged
- affiliate / tracking must remain unchanged
- canonical / hreflang must remain unchanged unless separately authorized
- no additional localization audit is required for these four pages after exact implementation

Only mechanical implementation QA is required after implementation.

# 8. Workflow state

Completed 4-page units:
- Batch 01A: 4 pages
- Batch 01B: 4 pages

Next unit:
- Batch 01C: next 4 pages
