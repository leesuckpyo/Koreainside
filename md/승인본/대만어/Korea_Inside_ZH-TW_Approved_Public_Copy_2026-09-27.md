# Korea Inside — Taiwan Final Review Copy — 2026-09-27

**Status:** APPROVED PUBLIC COPY — CONTENT LOCKED  
**Locale:** Traditional Chinese (Taiwan) / `zh-TW`  
**Draft branch:** `zh-tw-draft`  
**English/Main protected baseline:** `8d84b5bd88eb3871d386521caf4012af2af57c4b`  
**Taiwan Draft commit:** `933de38a1411586dc29eccb00d620f6cd324314e`  
**Scope:** Editorial wording corrections only. No HTML implementation, routing, canonical, hreflang, sitemap, Git merge, or Production in this stage.

---

# 1. Editorial conclusion

The 58-page Taiwan Draft does **not** require a full retranslation.

Final editorial direction:

- preserve all facts, numbers, recommendations, trade-offs, structure, affiliate/tracking, images and JS behavior
- correct only confirmed semantic errors, Taiwan wording, terminology consistency, SEO wording, and Chinese typography
- do not weaken strong recommendations that correctly reflect the English source
- do not bulk-replace context-dependent terms such as `本國` or `地方`

Current state remains:

- Draft pages: **58/58**
- Inventory: **0 COMPLETE / 58 MISSING / 3 EXCLUDE**
- Approved Public Copy: **YES**
- CONTENT LOCKED: **YES**
- Production: **NO**

---

# 2. COMMON Golden Sample — apply identically to all 58 pages

These five COMMON/UI replacements are approved editorial directions for the final Taiwan copy.

| Current Draft | Final Review Copy |
|---|---|
| `旅行實用資訊` | `旅遊實用資訊` |
| `韓國旅行實用工具` | `韓國旅遊實用工具` |
| `旅行必備 App` | `旅遊必備 App` |
| `韓國旅行準備清單` | `韓國旅遊準備清單` |
| `由韓國編輯在當地撰寫、審閱，提供實用的韓國旅行資訊。` | `由韓國編輯在韓國撰寫與審閱，提供實用的韓國旅遊資訊。` |

**Important:** keep the language selector text as `繁中`. The adjacent small `語言` label is a separate DOM element and is not a mistranslation.

`在韓國製作` is acceptable and remains unchanged.

---

# 3. Confirmed page-specific semantic / Taiwan wording corrections

## 3.1 `rental-car.html`

Current:

`公車班次少，或行程串連好幾個小停靠點時，最能顯出租車價值。`

Final:

`公車班次少，或行程串連好幾個小停靠點時，這正是最適合租車的情況。`

Reason: English source says this is the strongest **rental-car** use case. `出租車` means taxi and is a real semantic error.

---

## 3.2 `payments.html`

Apply to both visible copy and FAQ schema where duplicated.

| Current | Final |
|---|---|
| `本國付款驗證` | `韓國本地付款驗證` |
| `本國付款流程` | `韓國本地付款流程` |
| `在韓國用外國卡操作 ATM` | `在韓國用外國卡提款` |

Do not change genuine traveler-home-country uses elsewhere.

---

## 3.3 `foreign-credit-cards-korea.html`

Apply consistently to body, table, visible FAQ and FAQ schema.

| Current | Final |
|---|---|
| `本國驗證流程` | `韓國本地驗證流程` |
| `本國驗證` | `韓國本地驗證` |
| `本國付款流程` | `韓國本地付款流程` |

English `domestic` refers to Korean domestic/local authentication, not the traveler's home country.

---

## 3.4 `wowpass.html`

Apply the same wording to schema FAQ, body, and visible FAQ.

Current concept:

`接受本國信用卡或簽帳金融卡的實體商家`

Final:

`接受韓國國內信用卡或簽帳金融卡付款的實體商家`

Recommended full sentence:

`官方指南說明，WOWPASS 付款餘額可在接受韓國國內信用卡或簽帳金融卡付款的實體商家使用。`

Body version keeps the existing additional clause about overseas cards.

Also change internal link text:

`在韓國用外國卡操作 ATM` → `在韓國用外國卡提款`

---

## 3.5 `apps.html`

Current:

`帳戶復原碼仍寄到原本的本國號碼。`

Final:

`帳戶復原碼仍可能寄到原本使用的門號。`

This preserves the warning that a data-only SIM/eSIM may not receive SMS sent to the user's original number.

---

## 3.6 `korea-atm-foreign-cards.html`

### Title

Current:

`在韓國用外國卡操作 ATM | Korea Inside`

Final:

`韓國 ATM 外國卡提款指南 | Korea Inside`

### H1

Current:

`在韓國用外國卡操作 ATM`

Final:

`在韓國用外國卡提款`

Meta description remains usable and does not require rewriting.

Any internal link label that repeats `在韓國用外國卡操作 ATM` should use `在韓國用外國卡提款` for consistency.

---

## 3.7 `where-to-stay-in-gangnam.html`

Meta description and OG description:

`美容診所` → `醫美診所`

No recommendation or page-intent change.

---

# 4. K-Beauty terminology register

Apply only where the context is the K-Beauty/personal-color/beauty-service meaning below. Do not alter official brand or service names.

## 4.1 `k-beauty.html`

### Meta description

Current key terms:

`個人色彩診斷、美髮保養、診所諮詢`

Final:

`個人色彩分析、美髮與頭皮護理、醫美診所諮詢`

### Hero / lead / body

- `美髮保養` → `美髮與頭皮護理`
- `個人色彩診斷` → `個人色彩分析`
- H3 `頭髮與頭皮保養` → `美髮與頭皮護理`
- H3 `美容診所與諮詢` → `醫美診所與諮詢`
- `英語服務` → `英文服務`
- `英語支援` → `英文支援`

### FAQ

Current:

`美容診所需要預約嗎？`

Final:

`醫美診所需要預約嗎？`

Current:

`個人色彩診斷有英語服務嗎？`

Final:

`個人色彩分析有英文服務嗎？`

FAQ answer:

`英語諮詢` → `英文諮詢`

All eight FAQ answers remain present; no FAQ is missing.

---

## 4.2 `myeongdong-travel-guide.html`

Apply context-specific Taiwan wording:

- `個人色彩診斷` → `個人色彩分析`
- `基本色彩診斷` → `基本色彩分析`
- `英語協助` → `英文協助`
- `英語或翻譯協助` → `英文或翻譯協助`
- `以英語進行` → `以英文進行`

Tax-refund display formatting:

- `₩1 百萬` → `₩100萬`
- `₩5 百萬` → `₩500萬`

The factual thresholds remain unchanged.

---

## 4.3 `taste-korea.html` K-Beauty bridge

`個人色彩診斷` → `個人色彩分析`

---

# 5. Hongdae photo-booth terminology

File: `hongdae-travel-guide.html`

The Draft uses `拍照機` repeatedly. For Taiwan readers:

- first explanatory occurrence: `四格拍貼機`
- later concise occurrences: `拍貼機`
- existing `四格拍照機` → `四格拍貼機`

Exact affected Draft lines at commit `933de38a...`:

`489, 596, 622, 647, 648, 654, 671, 936, 1166, 1286, 1296`

English-service wording on this page:

- `以英語帶領` → `以英文帶領`
- `英語協助` → `英文協助`
- `以英語進行` → `以英文進行`

Keep the meaning, duration, booking judgment and affiliate placement unchanged.

---

# 6. Taste Korea — regional/local food terminology

File: `taste-korea.html`

On this page the listed `地方料理 / 地方美食` occurrences refer to regional/local food identity, so they should use Taiwan-natural `在地` wording.

Apply:

- `地方料理` → `在地料理` in the identified food-identity contexts
- `地方美食` → `在地美食` in the identified food-identity contexts
- image alt `傳統與地方料理` → `傳統與在地料理`
- H3 `傳統與地方料理` → `傳統與在地料理`

Special heading rewrite:

Current:

`認識地方料理，從周圍的地方開始`

Final:

`理解在地料理，也要看它所在的地方`

This is a contextual correction for `taste-korea.html`; it is **not** permission to bulk-replace every `地方` across the site.

Also:

`英語協助` → `英文協助`

---

# 7. Chinese punctuation spacing cleanup — exact audited set

Rule:

Remove only the unnecessary ASCII space immediately before Chinese punctuation (`，。；：？！`) in the audited occurrences. Do not normalize any other whitespace, Latin brand spacing, URLs or technical tokens.

The final sweep found **63 occurrences** in the Draft commit `933de38a...`.

## Exact locations

| File | Draft line(s) | Occurrences |
|---|---|---:|
| `accommodation.html` | 214, 679 | 2 |
| `airport-bus.html` | 114, 155, 256 | 3 |
| `airport-transfer.html` | 272, 296, 316, 341, 363 | 5 |
| `apps.html` | 391, 522 | 2 |
| `arex.html` | 136, 293, 350, 364, 372 | 5 |
| `arrival.html` | 61, 297, 325 | 3 |
| `best-area-for-airport-access-seoul.html` | 179 | 3 |
| `card-declined-korea.html` | 234 | 1 |
| `esim.html` | 410, 427 | 2 |
| `gangnam-travel-guide.html` | 208, 772 | 2 |
| `gongdeok-mapo-seoul-guide.html` | 782, 901 | 2 |
| `hongdae-travel-guide.html` | 634, 652, 1440 | 3 |
| `jamsil-travel-guide.html` | 647, 655 | 2 |
| `korea-esim-with-phone-number.html` | 486 | 1 |
| `lotte-world-seoul.html` | 428, 873, 907, 1063, 1182 | 5 |
| `maps.html` | 563, 573 | 2 |
| `rental-car.html` | 419 | 1 |
| `seongsu-travel-guide.html` | 1130, 1131 | 2 |
| `taxi.html` | 249, 262, 345, 377, 410 | 6 |
| `tmoney.html` | 113, 393 | 2 |
| `where-to-stay-in-dongdaemun.html` | 274, 358 | 2 |
| `where-to-stay-in-hongdae.html` | 319 | 1 |
| `where-to-stay-in-insadong.html` | 223 | 1 |
| `where-to-stay-in-itaewon.html` | 319 | 1 |
| `where-to-stay-in-jamsil.html` | 279 | 1 |
| `where-to-stay-in-seongsu.html` | 309 | 1 |
| `wowpass.html` | 192, 288 | 2 |
| **TOTAL** |  | **63** |

Note: line 345 of `taxi.html` and line 179 of `best-area-for-airport-access-seoul.html` contain multiple audited spaces; the table count records the actual number of occurrences, not just line count.

---

# 8. Deliberately retained wording — DO NOT OVER-CORRECT

The following were checked against the English source and should remain unless a later real defect appears.

## 8.1 Strong recommendation wording in Seongsu

`非常適合` is not automatically recommendation drift. The relevant English source uses `Very strong fit.` Preserve the strong recommendation level.

## 8.2 Myeongdong negative/conditional “best choice”

`但它不是每個人的最佳選擇。`

This correctly reflects English `It is not automatically the best choice for everyone.` Do not weaken or remove `最佳` mechanically.

## 8.3 Genuine home-country uses of `本國`

Keep when it actually means the traveler's home country, including examples such as:

- `本國駕照`
- `本國貨幣`

Only payment/authentication contexts where English `domestic` means Korea are corrected.

## 8.4 `在韓國製作`

Meaning is acceptable. Keep it.

## 8.5 FAQ completeness

K-Beauty visible FAQ: all 8 answers exist. No missing-answer correction is needed.

---

# 9. SEO Title / Meta / H1 conclusion

**58/58 reviewed.**

No broad SEO rewrite is required.

Confirmed selective SEO wording changes:

1. `korea-atm-foreign-cards.html`
   - Title → `韓國 ATM 外國卡提款指南 | Korea Inside`
   - H1 → `在韓國用外國卡提款`
2. `where-to-stay-in-gangnam.html`
   - Meta/OG `美容診所` → `醫美診所`
3. `k-beauty.html`
   - Meta terminology aligned to `個人色彩分析 / 美髮與頭皮護理 / 醫美診所諮詢`

No new keyword target or page-role change is introduced.

---

# 10. FAQ / schema consistency

Where the same wording exists in visible FAQ and JSON-LD FAQ schema, corrections must be applied identically.

Required synchronized sets:

- `payments.html` — Korean-local payment authentication wording
- `foreign-credit-cards-korea.html` — Korean-local authentication/payment wording
- `wowpass.html` — Korean domestic credit/debit merchant wording

Do not create new FAQPage schema where English does not have one.

---

# 11. Inline-JS wording

The Draft QA identified 117 localized user-facing inline-JS literals and 13 retained proper-name/symbol locations.

Editorial review found no additional semantic correction requiring a new inline-JS rewrite in this Final Review Copy.

Therefore:

- preserve all 117 localized literals unless they match an explicitly listed COMMON/page-specific correction above
- preserve all JS logic and structure
- no new JS behavior change

---

# 12. Final self-check before user approval

- 58/58 Draft pages accounted for
- COMMON/UI review complete
- terminology/typography review complete
- Stay/Hotel high-risk review complete
- Airport/Transport high-risk review complete
- Payments high-risk review complete
- SEO Title/Meta/H1 58/58 reviewed
- confirmed semantic errors registered
- FAQ/schema synchronized corrections identified
- inline-JS review accounted for
- punctuation-space exact audited set: **63**
- recommendation-strength drift introduced by Review Copy: **0**
- fact/number/date changes introduced by Review Copy: **0**
- affiliate/tracking changes: **0**
- structure/CSS/JS logic changes: **0**

---

# 13. Approval effect

If the user approves this Final Review Copy:

1. This wording becomes the basis for **APPROVED PUBLIC COPY — CONTENT LOCKED**.
2. Codex may then receive one exact implementation instruction.
3. Only after implementation + technical QA will the Production approval gate open.
4. Technical implementation will separately handle:
   - `lang="zh-TW"`
   - Taiwan self-canonical
   - reciprocal hreflang for actual Production siblings
   - `/zh-tw/` internal routing
   - sitemap
   - affiliate/tracking preservation
   - final routing QA
   - zh-TW localized infographic assets before Taiwan completion
5. Inventory remains **0 COMPLETE / 58 MISSING / 3 EXCLUDE** until Production QA is complete.

---

# END
