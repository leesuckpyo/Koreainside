# Korea Inside — zh-TW Localization Audit + Correction — Batch 01H

- File: `Korea_Inside_ZH-TW_Localization_Audit_Correction_Batch01H_10pages_2026-09-30.md`
- Date: 2026-09-30
- Status: **REVIEW COPY — AWAITING USER APPROVAL**
- Scope: Current English Production ↔ current zh-TW Production direct comparison
- Work unit: **10 pages**
- Rule: **This correction MD is the final localization audit for these ten pages. After exact implementation, do not run another editorial localization audit unless a concrete defect appears.**

---

## 1. Batch scope

35. `zh-tw/k-beauty.html`
36. `zh-tw/korea-atm-foreign-cards.html`
37. `zh-tw/korea-esim-with-phone-number.html`
38. `zh-tw/korean-online-payments-foreigners.html`
39. `zh-tw/lotte-world-seoul.html`
40. `zh-tw/maps.html`
41. `zh-tw/myeongdong-travel-guide.html`
42. `zh-tw/payments.html`
43. `zh-tw/rental-car.html`
44. `zh-tw/seongsu-travel-guide.html`

Production basis checked directly:

- Current English Production and current zh-TW Production
- Facts, numbers, recommendation strength and Taiwan localization judged in one pass
- No page 45+ was inspected in this Batch
- This is a localization audit, not a new factual-refresh pass; current English Production remains the factual and recommendation Source of Truth

---

# 2. Final result

| Page | Result | Corrections |
|---|---|---:|
| `k-beauty.html` | FIX | 2 |
| `korea-atm-foreign-cards.html` | FIX | 1 |
| `korea-esim-with-phone-number.html` | FIX | 9 |
| `korean-online-payments-foreigners.html` | PASS | 0 |
| `lotte-world-seoul.html` | PASS | 0 |
| `maps.html` | PASS | 0 |
| `myeongdong-travel-guide.html` | FIX | 3 |
| `payments.html` | FIX | 3 |
| `rental-car.html` | PASS | 0 |
| `seongsu-travel-guide.html` | FIX | 3 |
| **TOTAL** |  | **21** |

The corrections below are the exact OLD → NEW instructions.

Do not paraphrase them during implementation.

---

# 3. Page 35 — `zh-tw/k-beauty.html`

## Result: FIX 2

### FIX 01 — hero paragraph coordination

OLD:

```text
想在同一趟旅行探索保養品、彩妝、個人色彩、美髮與頭皮護理與美容療程，韓國是很容易安排的地方。首爾從大型 Olive Young、獨立香氛店，到只接受預約的沙龍與診所，都找得到。
```

NEW:

```text
想在同一趟旅行探索保養品、彩妝、個人色彩、美髮與頭皮護理，以及美容療程，韓國是很容易安排的地方。首爾從大型 Olive Young、獨立香氛店，到只接受預約的沙龍與診所，都找得到。
```

Reason:
- Fixes the duplicated `與...與...` coordination.
- K-Beauty scope and recommendation unchanged.

### FIX 02 — accommodation recommendation

Context:
- H2: `以 K-Beauty 為重點的旅客，在首爾住哪裡？`

OLD:

```text
K-Beauty 若只是旅行的一部分，通常不必為它換飯店。店鋪與預約分散在首爾各處時，住市中心比較好安排；只有好幾項預約都集中在同一區，住在美妝服務密集的區域附近才更有道理。
```

NEW:

```text
K-Beauty 若只是旅行的一部分，通常不必為它換飯店。店鋪與預約分散在首爾各處時，住市中心比較好安排；只有好幾項預約都集中在同一區，住在美妝服務密集的區域附近才更合理。
```

Reason:
- Removes the direct `makes more sense → 更有道理` calque.
- Stay recommendation strength unchanged.

### Page 35 protection

- Previously established Taiwan terminology is already correctly present:
  - `個人色彩分析`
  - `美髮與頭皮護理`
  - `醫美診所`
  - `英文服務 / 英文諮詢`
- Do not reopen or bulk-replace those terms.
- Facts / product / clinic / shopping judgments: unchanged
- H1 1 / H2 5 / H3 12 — EN ↔ zh-TW parity
- Visible FAQ: 8
- FAQPage schema: intentionally 0
- External href parity: 2 / 2
- Affiliate-marked link parity: 2 / 2
- Internal zh-TW routing defect detected: 0

---

# 4. Page 36 — `zh-tw/korea-atm-foreign-cards.html`

## Result: FIX 1

The previously identified literal Title/H1 defect is **already corrected in current Production**:

- Title: `韓國 ATM 外國卡提款指南 | Korea Inside`
- H1: `在韓國用外國卡提款`

Do not reopen those strings.

### FIX 01 — ATM FAQ spacing

Context:
- FAQ question: `韓國 ATM 拒絕我的卡時，該怎麼辦？`
- This exact answer appears in **visible FAQ + FAQPage JSON-LD**.
- Update both identically.

OLD:

```text
換一台明確支援你卡片網路的 ATM。如果多台相容 ATM都拒絕這張卡，請查看發卡機構的 App，或聯絡發卡機構，詢問海外提款、額度與帳戶限制。
```

NEW:

```text
換一台明確支援你卡片網路的 ATM。如果多台相容 ATM 都拒絕這張卡，請查看發卡機構的 App，或聯絡發卡機構，詢問海外提款、額度與帳戶限制。
```

Reason:
- Restores spacing around the Latin technical token `ATM`.
- Meaning unchanged.

### Page 36 protection

- Title/H1 SEO correction already present: preserve
- `本國幣別` is correct here because it means the cardholder's **home currency**; do not replace it
- Facts / PIN / DCC / fee guidance: unchanged
- H1 1 / H2 9 / H3 0 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- Current FAQ parity: 8 / 8
- After FIX 01, visible FAQ and JSON-LD must remain exact mirrors
- External official-reference href parity: 6 / 6
- Internal zh-TW routing defect detected: 0

---

# 5. Page 37 — `zh-tw/korea-esim-with-phone-number.html`

## Result: FIX 9

### FIX 01 — H1 + BreadcrumbList JSON-LD name

This exact OLD string appears in:
1. H1
2. BreadcrumbList JSON-LD user-facing name

Update those two locations identically.

OLD:

```text
有韓國門號的韓國 eSIM
```

NEW:

```text
韓國 eSIM：附韓國門號
```

Reason:
- Removes the awkward repetition of `韓國`.
- Keeps both Korea eSIM intent and the Korean-phone-number distinction.
- Do not change the visible breadcrumb text `有門號的韓國 eSIM`.

### FIX 02 — hero recommendation

OLD:

```text
韓國當地確實有人需要打電話、傳簡訊給你，或預約、當地聯絡需要韓國號碼時，旅遊 eSIM 值得考慮。若只要地圖、訊息、翻譯與上網，一般數據 eSIM 通常更簡單。
```

NEW:

```text
如果在韓國確實有人需要打電話或傳簡訊給你，或你需要韓國門號來預約、作為當地聯絡方式，韓國旅遊 eSIM 就值得考慮。若只要地圖、訊息、翻譯與上網，一般數據 eSIM 通常更簡單。
```

Reason:
- Repairs the broken `預約、當地聯絡需要...` structure.
- Makes clear that this recommendation is for a Korean tourist eSIM rather than any generic travel eSIM.
- Recommendation remains conditional.

### FIX 03 — domestic-call referent

Context:
- H2: `只需要接電話、收簡訊，還是也要撥出與傳送？`

OLD:

```text
有些旅客只需韓國號碼，讓飯店、餐廳或司機聯絡；另一些需要自行撥打國內電話或發 SMS。需求不同，即使同一家電信商，也可能對應不同產品。
```

NEW:

```text
有些旅客只需韓國號碼，讓飯店、餐廳或司機聯絡；另一些需要自行撥打韓國當地電話或發 SMS。需求不同，即使同一家電信商，也可能對應不同產品。
```

Reason:
- English `domestic calls` means calls within Korea.
- `國內電話` can be misread by a Taiwan traveler as Taiwan domestic calls.

### FIX 04 — Data + Voice scenario

Context:
- Step `02 Data + Voice`
- Replace the paragraph text only.

OLD:

```text
預期自行撥國內電話或發 SMS，可通話產品更合適。但可能多了驗證、機場程序或獨立通話餘額，額外功能伴隨更多設定。
```

NEW:

```text
預期自行撥打韓國當地電話或發 SMS 時，可通話產品更合適。但可能多了驗證、機場程序或獨立通話餘額，額外功能也伴隨更多設定。
```

Reason:
- Clarifies the Korean domestic-call referent.
- Repairs compressed sentence grammar.

### FIX 05 — LG U+ limitation paragraph

OLD:

```text
最大限制在號碼不能變成什麼。LG U+ 區分簡單預約 SMS 用途，與銀行、政府身分驗證；已安裝或啟用 eSIM，也有嚴格刪除與補發限制。
```

NEW:

```text
最大的限制，是有這個號碼仍不等於具備韓國居民身分驗證資格。LG U+ 區分簡單預約 SMS 用途，與銀行、政府身分驗證；已安裝或啟用 eSIM，也有嚴格刪除與補發限制。
```

Reason:
- Current first sentence is grammatically unnatural and incomplete.
- Restores the source distinction between having a number and resident identity verification.

### FIX 06 — data-oriented product wording

Context:
- H3: `需要撥電話，卻買純上網`

OLD:

```text
數據為主產品提供的號碼，不自動包含撥出服務。找可通話產品，也查是否需要獨立餘額。
```

NEW:

```text
以數據為主的產品所提供的號碼，不會自動包含撥出服務。應選可通話產品，也要確認是否需要獨立餘額。
```

Reason:
- Repairs missing particles and compressed syntax.
- Product-function warning unchanged.

### FIX 07 — deleting vs disabling line

Context:
- H3: `刪除已安裝的 eSIM`

OLD:

```text
三家都公布刪除或補發限制。排錯時，關門號比刪除安全，下一步應找電信商支援。
```

NEW:

```text
三家都公布刪除或補發限制。排錯時，先關閉 eSIM 門號比刪除設定檔安全，下一步應聯絡電信商支援。
```

Reason:
- `關門號` can sound like cancelling the phone number.
- Source means turning the line off while troubleshooting.

### FIX 08 — unnecessary setup burden

Context:
- H3: `純上網較簡單，卻選韓國電信商方案`

OLD:

```text
地圖、通訊與 App 通話不需要韓國號碼。為這些用途加上電信商驗證與通話規則，可能增加工作，沒有實際價值。
```

NEW:

```text
地圖、通訊與 App 通話不需要韓國號碼。為這些用途加上電信商驗證與通話規則，反而可能增加設定負擔，卻沒有實際價值。
```

Reason:
- `增加工作` is a direct calque from `create work`.
- Decision judgment unchanged.

### FIX 09 — final Korean-number recommendation

Context:
- H2: `韓國門號 eSIM 何時值得額外設定`

OLD:

```text
只需要上網，先看一般旅遊 eSIM。接收當地電話或簡訊、自行撥國內電話，或有韓國聯絡號碼，確實會改變旅程時，韓國旅遊 eSIM 才值得多做設定。
```

NEW:

```text
只需要上網，先看一般旅遊 eSIM。只有在接收當地電話或簡訊、自行撥打韓國當地電話，或擁有韓國聯絡號碼確實會改變旅程時，韓國旅遊 eSIM 才值得多做設定。
```

Reason:
- Clarifies `domestic calls`.
- Repairs conditional sentence structure.
- Recommendation strength unchanged.

### Page 37 protection

- Carrier order: unchanged
- SK Telecom / KT / LG U+ facts and numbers: unchanged
- `010`, calls, SMS, activation, passport, reissue and identity-verification distinctions: unchanged
- H1 1 / H2 19 / H3 25 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- Current FAQ parity: 8 / 8
- No FAQ correction in this Batch
- External official/provider href parity: 14 / 14
- Internal zh-TW routing defect detected: 0

---

# 6. Page 38 — `zh-tw/korean-online-payments-foreigners.html`

## Result: PASS

Direct comparison found no correction requiring inclusion in the final register.

Protected result:

- Major factual mismatch: 0
- Numeric mismatch: 0
- Material payment-flow distortion: 0
- Large omission / invention: 0
- Taiwan localization blocking defect: 0
- H1 1 / H2 9 / H3 0 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- FAQ exact parity: 8 / 8
- External official-reference href parity: 5 / 5
- Internal zh-TW routing defect detected: 0

The existing uses of `本地` are sufficiently anchored to Korean services / Korean checkout context on this page and are not treated as an independent mistranslation.

---

# 7. Page 39 — `zh-tw/lotte-world-seoul.html`

## Result: PASS

Direct comparison found no correction requiring inclusion in the final register.

Protected result:

- Ticket types / prices / dates: unchanged
- Magic Pass judgment and order: unchanged
- Ride names / height rules / route order: unchanged
- Major factual mismatch: 0
- Numeric mismatch: 0
- Recommendation drift: 0
- H1 1 / H2 20 / H3 67 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- FAQ exact parity: 8 / 8
- External href parity: 5 / 5
- Affiliate-marked link parity: 5 / 5
- Internal zh-TW routing defect detected: 0
- Preserve nested current-year marker:
  - `<span class="lotte-world-guide-year">`
  - `<span data-guide-year="current">2026</span>`

---

# 8. Page 40 — `zh-tw/maps.html`

## Result: PASS

Direct comparison found no correction requiring inclusion in the final register.

Protected result:

- Naver Map / KakaoMap / Google Maps role hierarchy: unchanged
- Google Maps limitation / future-improvement wording: unchanged
- Major factual mismatch: 0
- Numeric mismatch: 0
- Recommendation drift: 0
- H1 1 / H2 11 / H3 27 — EN ↔ zh-TW parity
- FAQ: 10 visible / 10 FAQPage JSON-LD
- FAQ exact parity: 10 / 10
- External official / app-store href parity: 20 / 20
- Internal zh-TW routing defect detected: 0

---

# 9. Page 41 — `zh-tw/myeongdong-travel-guide.html`

## Result: FIX 3

### FIX 01 — tourist-information language wording

OLD:

```text
會賢站 5 號出口附近，也有提供英語、日語與中文協助的官方旅遊諮詢中心。
```

NEW:

```text
會賢站 5 號出口附近，也有提供英文、日語與中文協助的官方旅遊諮詢中心。
```

Reason:
- Taiwan localization uses `英文` for service-language wording here.
- Service availability unchanged.

### FIX 02 — Namsan is not automatic

OLD:

```text
但不是自動必選。
```

NEW:

```text
但不代表一定要排進行程。
```

Reason:
- Removes the direct `automatic choice → 自動必選` calque.
- Preserves the conditional Namsan recommendation.

### FIX 03 — clear-weather reason for Namsan

OLD:

```text
比起只因首爾清單上有南山，晴朗的傍晚或夜晚，是更有力的理由。
```

NEW:

```text
比起只因首爾清單上有南山，晴朗的傍晚或夜晚，才是更值得前往的理由。
```

Reason:
- Removes literal `stronger reason → 更有力的理由`.
- Preserves the stronger clear-weather condition.

### Page 41 protection

- Previously identified tax-refund display formatting is already corrected:
  - `₩100萬`
  - `₩500萬`
- Do not reopen the tax thresholds.
- Facts / routes / NANTA / Namsan / shopping judgments: unchanged
- H1 1 / H2 17 / H3 69 — EN ↔ zh-TW parity
- FAQPage JSON-LD: 0
- External href parity: 3 / 3
- Affiliate-marked link parity: 3 / 3
- Internal zh-TW routing defect detected: 0
- H1 `data-guide-year="current"` marker: preserve exactly

---

# 10. Page 42 — `zh-tw/payments.html`

## Result: FIX 3

### FIX 01 — local-verification heading

OLD:

```text
網路結帳可能要求本地驗證資料
```

NEW:

```text
網路結帳可能要求韓國本地驗證資料
```

Reason:
- English `local credentials` refers specifically to Korean local credentials.
- Removes possible Taiwan-home-country ambiguity.

### FIX 02 — Korean-local checkout paragraph

OLD:

```text
海外卡即使能在現場付款，遇到要求本地驗證、韓國電話驗證或韓國本地付款流程的韓國網站，仍可能失敗。
```

NEW:

```text
海外卡即使能在現場付款，遇到要求韓國本地驗證、韓國電話驗證或韓國本地付款流程的韓國網站，仍可能失敗。
```

Reason:
- Clarifies the referent of `local authentication`.
- No payment-policy fact change.

### FIX 03 — mobile-wallet paragraph

OLD:

```text
Apple Pay 可在參與支援的 NFC 商家使用，但錢包內卡片與商家設備仍須相容。Kakao Pay、Naver Pay 等韓國服務，可能要求本地身分、電話或付款設定，因此不是短期旅客能穩定依賴的預設方式。其他行動錢包則依卡片、裝置與服務設定而定。
```

NEW:

```text
Apple Pay 可在支援 NFC 的商家使用，但錢包內卡片與商家設備仍須相容。Kakao Pay、Naver Pay 等韓國服務，可能要求韓國本地的身分驗證、電話或付款設定，因此不是短期旅客能穩定依賴的預設方式。其他行動錢包則依卡片、裝置與服務設定而定。
```

Reason:
- Removes the awkward `參與支援的 NFC 商家`.
- Clarifies that `local identity` means Korean-local identity / account setup.
- Mobile-wallet recommendation unchanged.

### Page 42 protection

- Current visible FAQ already uses explicit Korean-local wording where needed:
  - `韓國本地付款驗證`
- Do not alter valid `home currency` / traveler-home-country contexts elsewhere.
- Facts / payment-method hierarchy: unchanged
- H1 1 / H2 6 / H3 8 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- FAQ exact parity: 8 / 8
- External href parity: 0 / 0
- Internal zh-TW routing defect detected: 0

---

# 11. Page 43 — `zh-tw/rental-car.html`

## Result: PASS

The previously confirmed semantic error is **already corrected in current Production**.

Current correct wording:

```text
公車班次少，或行程串連好幾個小停靠點時，這正是最適合租車的情況。
```

Do not restore the old incorrect `出租車` wording.

Protected result:

- `本國駕照` is valid because it refers to the traveler's home-country licence; preserve
- IDP / licence / insurance / airport / toll / Hi-Pass guidance: unchanged
- Major factual mismatch: 0
- Numeric mismatch: 0
- Recommendation drift: 0
- H1 1 / H2 16 / H3 28 — EN ↔ zh-TW parity
- FAQ: 10 visible / 10 FAQPage JSON-LD
- FAQ exact parity: 10 / 10
- External official-reference href parity: 8 / 8
- Internal zh-TW routing defect detected: 0

---

# 12. Page 44 — `zh-tw/seongsu-travel-guide.html`

## Result: FIX 3

### FIX 01 — Seongsu Station default start

OLD:

```text
對多數第一次來、想逛街、看快閃店、美妝與工業街道的旅客而言，聖水站是比較方便的預設起點。
```

NEW:

```text
對多數第一次來、想逛街、看快閃店、美妝與工業街道的旅客而言，聖水站是最容易安排的起點。
```

Reason:
- Removes literal `easier default → 比較方便的預設起點`.
- Starting-point recommendation strength unchanged.

### FIX 02 — English-friendly personal color service

OLD:

```text
聖水有可使用英語的個人色彩分析選項，位置也方便串進街區行程。如果分析結果能實際影響你當天稍後挑選的色調、衣服或彩妝，這項體驗就最有價值。
```

NEW:

```text
聖水有提供英文服務的個人色彩分析選項，位置也方便串進街區行程。如果分析結果能實際影響你當天稍後挑選的色調、衣服或彩妝，這項體驗就最有價值。
```

Reason:
- Natural Taiwan service-language wording.
- Personal-color recommendation unchanged.

### FIX 03 — skincare-making heading

OLD:

```text
比起再逛一家化妝品店，動手做保養品更有聖水的特色
```

NEW:

```text
比起再逛一家化妝品店，動手做保養品更能體現聖水的特色
```

Reason:
- Naturalizes `fits Seongsu better`.
- Same editorial judgment retained.

### Page 44 protection

- Existing `非常適合` occurrences were checked against English `Very strong fit` and are **not defects**.
- Do not weaken them.
- Facts / pop-up / K-Beauty / café / Seoul Forest judgments: unchanged
- H1 1 / H2 17 / H3 67 — EN ↔ zh-TW parity
- FAQPage JSON-LD: 0
- External href parity: 3 / 3
- Internal zh-TW routing defect detected: 0
- H1 `data-guide-year="current"` marker: preserve exactly

---

# 13. Batch 01H final audit checks

## Content

- Major factual mismatch: **0**
- Material numeric mismatch: **0**
- Material recommendation drift: **0**
- Large omission: **0**
- Large invention: **0**
- English page-role distortion: **0**

## Taiwan localization

- Exact correction items: **21**
- FIX pages: **6**
- PASS pages: **4**
- Korean-domestic/local referent clarifications: included only where ambiguity remained
- Previously fixed items were not re-added
- Recommendation softening / strengthening caused by correction: **0**

## Structure / schema

| Page | H1 | H2 | H3 | Visible FAQ | FAQPage |
|---|---:|---:|---:|---:|---:|
| K-Beauty | 1 | 5 | 12 | 8 | 0 |
| Korea ATM | 1 | 9 | 0 | 8 | 8 |
| Korea eSIM with Phone Number | 1 | 19 | 25 | 8 | 8 |
| Korean Online Payments | 1 | 9 | 0 | 8 | 8 |
| Lotte World | 1 | 20 | 67 | 8 | 8 |
| Maps | 1 | 11 | 27 | 10 | 10 |
| Myeongdong | 1 | 17 | 69 | 0 | 0 |
| Payments | 1 | 6 | 8 | 8 | 8 |
| Rental Car | 1 | 16 | 28 | 10 | 10 |
| Seongsu | 1 | 17 | 67 | 0 | 0 |

EN ↔ zh-TW structural mismatch: **0**

Current visible FAQ ↔ FAQPage JSON-LD exact parity:

- Korea ATM: **8/8**
- Korea eSIM with Phone Number: **8/8**
- Korean Online Payments: **8/8**
- Lotte World: **8/8**
- Maps: **10/10**
- Payments: **8/8**
- Rental Car: **10/10**

K-Beauty intentionally has visible FAQ 8 / FAQPage 0.

JSON-LD parse error: **0**

## Link / affiliate / routing protection

All ten pages:

- `lang="zh-TW"`: correct
- self canonical: correct
- hreflang set present: `en / es / ja / zh-TW / x-default`
- non-zh-TW main-content routing defect detected: **0**

External href parity EN ↔ zh-TW:

- K-Beauty: **2 / 2**
- Korea ATM: **6 / 6**
- Korea eSIM with Phone Number: **14 / 14**
- Korean Online Payments: **5 / 5**
- Lotte World: **5 / 5**
- Maps: **20 / 20**
- Myeongdong: **3 / 3**
- Payments: **0 / 0**
- Rental Car: **8 / 8**
- Seongsu: **3 / 3**

Affiliate-marked link parity where present:

- K-Beauty: **2 / 2**
- Lotte World: **5 / 5**
- Myeongdong: **3 / 3**

HTML implementation performed in this audit: **0**  
stage / commit / push / Production performed: **0**

---

# 14. Exact implementation rule for later Codex work

When implementation is eventually requested:

1. Use **this file as the exact correction Source of Truth for Batch 01H**.
2. Apply only the **21 correction items** above.
3. Do not rewrite PASS pages.
4. Do not rewrite any other zh-TW wording on FIX pages.
5. Do not change facts, numbers, dates, provider conditions, attraction rules, route order, recommendation order or recommendation strength.
6. Do not independently refresh current ticket prices, event schedules, provider policies, Google Maps functionality, payment policies, rental conditions or other time-sensitive data during this language-only implementation.
7. Do not change classes, IDs, `data-*`, images, CSS, JS logic, affiliate URLs, tracking, canonical or hreflang unless separately approved.
8. Preserve all current-year automation markers exactly.
9. Korea ATM FIX 01:
   - update visible FAQ + FAQPage JSON-LD identically.
10. Korea eSIM FIX 01:
   - update H1 + BreadcrumbList JSON-LD user-facing name.
   - do not change visible breadcrumb `有門號的韓國 eSIM`.
11. Do not change any other FAQ/schema strings.
12. Preserve Seongsu `非常適合` recommendation strength.
13. Preserve valid `本國駕照` and `本國幣別` home-country meanings.
14. If any OLD string is missing or occurs in an unexpected location, STOP instead of guessing.
15. After exact implementation, perform only mechanical QA. Do **not** repeat the localization audit.

---

# 15. Batch state

**Batch 01H = REVIEW COPY — AWAITING USER APPROVAL**

After user approval:
- wording becomes **APPROVED PUBLIC COPY — CONTENT LOCKED**
- later Codex implementation must be exact
- no second editorial localization audit
