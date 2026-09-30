# Korea Inside — zh-TW Localization Audit + Correction — Batch 01I

- File: `Korea_Inside_ZH-TW_Localization_Audit_Correction_Batch01I_10pages_2026-09-30.md`
- Date: 2026-09-30
- Status: **REVIEW COPY — AWAITING USER APPROVAL**
- Scope: Current English Production ↔ current zh-TW Production direct comparison
- Work unit: **10 pages**
- Rule: **This correction MD is the final localization audit for these ten pages. After exact implementation, do not run another editorial localization audit unless a concrete defect appears.**

---

## 1. Batch scope

45. `zh-tw/seoul-sky-guide.html`
46. `zh-tw/taste-korea.html`
47. `zh-tw/taxi.html`
48. `zh-tw/tmoney-vs-wowpass.html`
49. `zh-tw/tmoney.html`
50. `zh-tw/where-to-stay-in-dongdaemun.html`
51. `zh-tw/where-to-stay-in-gangnam.html`
52. `zh-tw/where-to-stay-in-hongdae.html`
53. `zh-tw/where-to-stay-in-insadong.html`
54. `zh-tw/where-to-stay-in-itaewon.html`

Production basis checked directly:

- Current English Production and current zh-TW Production
- Facts, numbers, recommendation strength and Taiwan localization judged in one pass
- No page 55+ was inspected in this Batch
- This is a localization audit, not a new factual-refresh pass; current English Production remains the factual and recommendation Source of Truth

---

# 2. Final result

| Page | Result | Corrections |
|---|---|---:|
| `seoul-sky-guide.html` | PASS | 0 |
| `taste-korea.html` | FIX | 2 |
| `taxi.html` | PASS | 0 |
| `tmoney-vs-wowpass.html` | FIX | 2 |
| `tmoney.html` | FIX | 1 |
| `where-to-stay-in-dongdaemun.html` | FIX | 1 |
| `where-to-stay-in-gangnam.html` | FIX | 5 |
| `where-to-stay-in-hongdae.html` | FIX | 4 |
| `where-to-stay-in-insadong.html` | FIX | 1 |
| `where-to-stay-in-itaewon.html` | FIX | 1 |
| **TOTAL** |  | **17** |

The corrections below are the exact OLD → NEW instructions.

Do not paraphrase them during implementation.

---

# 3. Page 45 — `zh-tw/seoul-sky-guide.html`

## Result: PASS

Direct comparison found no correction requiring inclusion in the final register.

Protected result:

- Ticket / visibility / sunset / Fast Pass / Sky Bridge decision logic: unchanged
- Facts / numbers: unchanged
- Recommendation strength: unchanged
- H1 1 / H2 21 / H3 25 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- FAQ exact parity: 8 / 8
- External href parity: 11 / 11
- Affiliate-marked link parity: 4 / 4
- Internal zh-TW routing defect detected: 0

---

# 4. Page 46 — `zh-tw/taste-korea.html`

## Result: FIX 2

The previously identified literal heading has **already been corrected in current Production**:

```text
理解在地料理，也要看它所在的地方
```

Do not reopen that heading.

### FIX 01 — regional-specialty wording

OLD:

```text
傳統韓食可以是正式的多道料理、市場早餐、一碗在地湯品、寺院料理、地方特色菜，或歷史街區裡的一餐。知名菜色清單只能說明其中一部分。
```

NEW:

```text
傳統韓食可以是正式的多道料理、市場早餐、一碗在地湯品、寺院料理、在地特色料理，或歷史街區裡的一餐。知名菜色清單只能說明其中一部分。
```

Reason:
- Aligns regional-food identity with established Taiwan editorial terminology.
- Food-category meaning unchanged.

### FIX 02 — Jeonju regional-identity wording

OLD:

```text
相較於夜生活或無窮無盡的選擇，喜歡地方特色與傳統文化的人，更容易感受到全州的魅力。韓屋村遊客不少，知名菜名也不保證吃到特別的一餐。不過，住一晚就能多留下一個夜晚和清晨，這是匆忙當日往返不容易感受到的。
```

NEW:

```text
相較於夜生活或無窮無盡的選擇，喜歡在地特色與傳統文化的人，更容易感受到全州的魅力。韓屋村遊客不少，知名菜名也不保證吃到特別的一餐。不過，住一晚就能多留下一個夜晚和清晨，這是匆忙當日往返不容易感受到的。
```

Reason:
- `regional identity` is more naturally expressed as `在地特色` in this Taiwan travel context.
- Jeonju recommendation unchanged.

### Page 46 protection

- Do **not** bulk-replace every `地方`.
- Existing `在地料理 / 在地美食 / 在地湯品` usage remains.
- Facts / city comparison / stay logic: unchanged
- H1 1 / H2 6 / H3 27 — EN ↔ zh-TW parity
- Visible FAQ: 8
- FAQPage schema: intentionally 0
- External href parity: 0 / 0
- Internal zh-TW routing defect detected: 0

---

# 5. Page 47 — `zh-tw/taxi.html`

## Result: PASS

Direct comparison found no correction requiring inclusion in the final register.

Protected result:

- k.ride / Kakao T / Uber role and app guidance: unchanged
- Fare / surcharge / airport / payment guidance: unchanged
- Major factual mismatch: 0
- Numeric mismatch: 0
- Recommendation drift: 0
- H1 1 / H2 13 / H3 6 — EN ↔ zh-TW parity
- FAQ: 10 visible / 10 FAQPage JSON-LD
- FAQ exact parity: 10 / 10
- External href parity: 8 / 8
- Internal zh-TW routing defect detected: 0

---

# 6. Page 48 — `zh-tw/tmoney-vs-wowpass.html`

## Result: FIX 2

### FIX 01 — WOWPASS decision heading

OLD:

```text
什麼時候 WOWPASS 更有意義
```

NEW:

```text
什麼情況更適合用 WOWPASS
```

Reason:
- Naturalizes `When WOWPASS makes more sense`.
- Does not strengthen WOWPASS over T-money.

### FIX 02 — WOWPASS decision paragraph

OLD:

```text
如果附加功能確實用得到，WOWPASS 就更有意義。
```

NEW:

```text
如果附加功能確實用得到，WOWPASS 就更適合。
```

Reason:
- Removes the direct `makes more sense → 更有意義` calque.
- Conditional recommendation unchanged.

### Page 48 protection

- Core conclusion remains:
  - most travelers do not need a separate T-money card if using WOWPASS
  - WOWPASS payment balance and T-money transit balance are separate
- T-money / WOWPASS feature hierarchy: unchanged
- H1 1 / H2 11 / H3 0 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- FAQ exact parity: 8 / 8
- External official href parity: 5 / 5
- Internal zh-TW routing defect detected: 0

---

# 7. Page 49 — `zh-tw/tmoney.html`

## Result: FIX 1

### FIX 01 — physical-card recharge table wording

Context:
- Comparison table
- Row: `儲值方式`
- Column: physical T-money

OLD:

```text
預設以現金在地鐵儲值機或合作便利商店儲值最穩妥
```

NEW:

```text
最穩妥的基本做法，是用現金在地鐵儲值機或合作便利商店儲值
```

Reason:
- Naturalizes `the safest default`.
- Cash-recharge recommendation and factual scope unchanged.

### Page 49 protection

- T-money price / recharge / refund / Apple Wallet / Android facts: unchanged
- `₩20,000–₩30,000`, `₩50,000`, device and date information: unchanged
- H1 1 / H2 16 / H3 7 — EN ↔ zh-TW parity
- FAQ: 12 visible / 12 FAQPage JSON-LD
- FAQ exact parity: 12 / 12
- External official href parity: 8 / 8
- Internal zh-TW routing defect detected: 0
- Infographic embedded-English localization is a separate visual track; do not mix it into this text correction Batch

---

# 8. Page 50 — `zh-tw/where-to-stay-in-dongdaemun.html`

## Result: FIX 1

### FIX 01 — Hotel Skypark Kingstown justification

OLD:

```text
購物不只是附近的娛樂，而是住宿期間預期反覆回來做的事，這間最容易有合理選擇理由。
```

NEW:

```text
如果購物不只是附近的娛樂，而是住宿期間會反覆回來做的事，這種情況下最值得考慮這間飯店。
```

Reason:
- Removes the literal `easiest to justify` construction.
- Preserves the conditional reason to choose this hotel.

### Page 50 protection

- Hotel order / editorial roles: unchanged
- Room sizes / bed layouts / station exits / airport buses: unchanged
- H1 1 / H2 8 / H3 14 — EN ↔ zh-TW parity
- Visible FAQ: 7
- FAQPage schema: intentionally 0
- External href parity: 24 / 24
- Affiliate-marked link parity: 24 / 24
- Internal zh-TW routing defect detected: 0
- H1 `data-guide-year="current"` marker: preserve exactly

---

# 9. Page 51 — `zh-tw/where-to-stay-in-gangnam.html`

## Result: FIX 5

### FIX 01 — north-of-river mismatch

OLD:

```text
如果旅程大多在江北，就比較沒那麼有說服力。每天主要逛景福宮、仁寺洞、明洞、弘大與首爾站，住江南代表反覆橫跨首爾。地鐵讓這些路程可行，但不會讓交通時間消失。
```

NEW:

```text
如果旅程大多在江北，住江南就沒那麼合適。每天主要逛景福宮、仁寺洞、明洞、弘大與首爾站，住江南代表反覆橫跨首爾。地鐵讓這些路程可行，但不會讓交通時間消失。
```

Reason:
- `less convincing → 沒那麼有說服力` is an English-derived evaluation phrase.
- Negative stay judgment unchanged.

### FIX 02 — quick-choice Ocloud wording

Context:
- Decision card: `會在新論峴玩到晚：`
- Replace only the sentence after the label.

OLD:

```text
比起晚餐或喝酒後再回三成，Ocloud 更有道理。
```

NEW:

```text
比起晚餐或喝酒後再回三成，Ocloud 更合適。
```

Reason:
- Naturalizes `makes more sense`.
- Hotel recommendation strength unchanged.

### FIX 03 — Ocloud / COEX mismatch

OLD:

```text
如果每天都在 COEX，Ocloud 就較難說服人。這種情況，住三成能減少更多路程。
```

NEW:

```text
如果每天都在 COEX，Ocloud 就較不適合。這種情況，住三成能減少更多路程。
```

Reason:
- Removes direct `harder to justify` calque.
- Same negative hotel-fit judgment retained.

### FIX 04 — Grand InterContinental price logic

OLD:

```text
有用到地點或飯店本身，房價才合理。如果一週都在別處觀光，只短暫去一次 COEX，每晚為這份便利付費就較難說服自己。
```

NEW:

```text
有用到地點或飯店本身，房價才合理。如果一週都在別處觀光，只短暫去一次 COEX，每晚為這份便利付費，就比較難說值得。
```

Reason:
- `harder to defend → 較難說服自己` is unnatural.
- Price-value judgment unchanged.

### FIX 05 — voco / Sinsa-Dosan fit

OLD:

```text
新沙與島山本來就在計畫裡，voco 最有道理。
```

NEW:

```text
新沙與島山本來就在計畫裡，voco 最合適。
```

Reason:
- Naturalizes `makes the most sense`.
- Strong hotel-fit judgment retained.

### Page 51 protection

- `醫美診所` in Meta is already correct; preserve
- Hotel order / zone logic: unchanged
- Room sizes / bed layouts / airport bus numbers: unchanged
- H1 1 / H2 9 / H3 17 — EN ↔ zh-TW parity
- Visible FAQ: 8
- FAQPage schema: intentionally 0
- External href parity: 30 / 30
- Affiliate-marked link parity: 30 / 30
- Internal zh-TW routing defect detected: 0
- H1 `data-guide-year="current"` marker: preserve exactly

---

# 10. Page 52 — `zh-tw/where-to-stay-in-hongdae.html`

## Result: FIX 4

### FIX 01 — hero default choice

OLD:

```text
對大多數已決定住弘大的初訪旅客，弘大入口站附近是最簡單的預設選擇。AREX 普通列車從仁川機場直達，弘大主要街道也容易前往。
```

NEW:

```text
對大多數已決定住弘大的初訪旅客，弘大入口站附近是最容易安排的住宿選擇。AREX 普通列車從仁川機場直達，弘大主要街道也容易前往。
```

Reason:
- Removes literal `simplest default`.
- Recommendation strength unchanged.

### FIX 02 — strongest reason to stay in Hongdae

OLD:

```text
住弘大最有力的理由，是白天主要觀光行程結束之後的生活。
```

NEW:

```text
住弘大最重要的理由，是白天主要觀光行程結束之後的生活。
```

Reason:
- Naturalizes `strongest reason`.
- Preserves strongest / primary rationale.

### FIX 03 — L7 value argument

OLD:

```text
透過弘大入口站往返機場仍很簡單，但這不是多花錢住 L7 的主要理由。更有說服力的是：完整服務的飯店，就在預計反覆活動的區域中心。淺眠、寧願犧牲便利換較安靜街道的旅客，應往離中心遠一點的地方找。
```

NEW:

```text
透過弘大入口站往返機場仍很簡單，但這不是多花錢住 L7 的主要理由。更值得多花錢的理由是：完整服務的飯店，就在預計反覆活動的區域中心。淺眠、寧願犧牲便利換較安靜街道的旅客，應往離中心遠一點的地方找。
```

Reason:
- Replaces the direct `stronger case → 更有說服力`.
- Keeps the reason for paying more for L7 explicit.

### FIX 04 — Localstitch fit

OLD:

```text
較長的旅程，或每天有一部分時間要工作，這種安排確實有價值。主要想要寬敞常規飯店客房的人，就比較難被說服。西橋的位置也離開弘大中心最繁忙的地方，但不保證夜晚安靜。
```

NEW:

```text
較長的旅程，或每天有一部分時間要工作，這種安排確實有價值。主要想要寬敞常規飯店客房的人，就比較不適合。西橋的位置也離開弘大中心最繁忙的地方，但不保證夜晚安靜。
```

Reason:
- Naturalizes `much less convincing`.
- Negative fit judgment retained.

### Page 52 protection

- Area / station / airport / noise logic: unchanged
- Hotel order / bed / room / age / airport details: unchanged
- H1 1 / H2 16 / H3 14 — EN ↔ zh-TW parity
- FAQ: 5 visible / 5 FAQPage JSON-LD
- FAQ exact parity: 5 / 5
- External href parity: 39 / 39
- Affiliate-marked link parity: 39 / 39
- zh-TW main-content routing defect detected: 0
- Current zh-TW correctly routes the family-area link to the localized sibling; preserve
- H1 `data-guide-year="current"` marker: preserve exactly

---

# 11. Page 53 — `zh-tw/where-to-stay-in-insadong.html`

## Result: FIX 1

### FIX 01 — AMID hotel-fit wording

OLD:

```text
別只因房型叫「Deluxe」就選。看實際準備訂的類別照片與格局。仁寺洞的鐘閣一側，或三張獨立床對旅程重要時，AMID 更有說服力；別假設每間房都特別大。
```

NEW:

```text
別只因房型叫「Deluxe」就選。看實際準備訂的類別照片與格局。仁寺洞的鐘閣一側，或三張獨立床對旅程重要時，AMID 更值得考慮；別假設每間房都特別大。
```

Reason:
- Naturalizes `more convincing`.
- Hotel-fit judgment unchanged.

### Page 53 protection

- Hotel order / room sizes / bed layouts / station logic: unchanged
- H1 1 / H2 7 / H3 10 — EN ↔ zh-TW parity
- Visible FAQ: 4
- FAQPage schema: intentionally 0
- External href parity: 21 / 21
- Affiliate-marked link parity: 21 / 21
- Internal zh-TW routing defect detected: 0
- H1 `data-guide-year="current"` marker: preserve exactly

---

# 12. Page 54 — `zh-tw/where-to-stay-in-itaewon.html`

## Result: FIX 1

### FIX 01 — Imperial Palace family recommendation wording

OLD:

```text
成人團體的需求符合 Deluxe 類別，我們會比較 Imperial Palace。年齡資訊仍不一致時，它不是我們的預設家庭推薦。
```

NEW:

```text
成人團體的需求符合 Deluxe 類別時，可以比較 Imperial Palace。年齡資訊仍不一致時，我們不會把它當成家庭住宿的優先推薦。
```

Reason:
- Naturalizes `not our default family recommendation`.
- Preserves the caution caused by inconsistent age information.
- Does not upgrade or downgrade the adult-group recommendation.

### Page 54 protection

- Hamilton / Imperial Palace / Grand Hyatt / airport / hill / noise logic: unchanged
- Hotel order / room details / age caution: unchanged
- H1 1 / H2 8 / H3 20 — EN ↔ zh-TW parity
- Visible FAQ: 7
- FAQPage schema: intentionally 0
- External href parity: 18 / 18
- Affiliate-marked link parity: 18 / 18
- Internal zh-TW routing defect detected: 0
- H1 `data-guide-year="current"` marker: preserve exactly

---

# 13. Batch 01I final audit checks

## Content

- Major factual mismatch: **0**
- Material numeric mismatch: **0**
- Numeric multiset parity EN ↔ zh-TW: **10/10 pages**
- Material recommendation drift: **0**
- Large omission: **0**
- Large invention: **0**
- English page-role distortion: **0**

## Taiwan localization

- Exact correction items: **17**
- FIX pages: **7**
- PASS pages: **3**
- Main correction types:
  - regional / local Taiwan terminology
  - `makes sense / convincing / strongest reason / default` calque cleanup
  - one T-money table wording cleanup
- Recommendation strength changed by correction: **0**

## Structure / schema

| Page | H1 | H2 | H3 | Visible FAQ | FAQPage |
|---|---:|---:|---:|---:|---:|
| Seoul Sky | 1 | 21 | 25 | 8 | 8 |
| Taste Korea | 1 | 6 | 27 | 8 | 0 |
| Taxi | 1 | 13 | 6 | 10 | 10 |
| T-money vs WOWPASS | 1 | 11 | 0 | 8 | 8 |
| T-money | 1 | 16 | 7 | 12 | 12 |
| Stay Dongdaemun | 1 | 8 | 14 | 7 | 0 |
| Stay Gangnam | 1 | 9 | 17 | 8 | 0 |
| Stay Hongdae | 1 | 16 | 14 | 5 | 5 |
| Stay Insadong | 1 | 7 | 10 | 4 | 0 |
| Stay Itaewon | 1 | 8 | 20 | 7 | 0 |

EN ↔ zh-TW structural mismatch: **0**

Current visible FAQ ↔ FAQPage JSON-LD exact parity:

- Seoul Sky: **8/8**
- Taxi: **10/10**
- T-money vs WOWPASS: **8/8**
- T-money: **12/12**
- Stay Hongdae: **5/5**

Taste Korea and the Dongdaemun / Gangnam / Insadong / Itaewon stay pages intentionally have visible FAQ without FAQPage schema.

JSON-LD parse error: **0**

## Link / affiliate / routing protection

All ten pages:

- `lang="zh-TW"`: correct
- self canonical: correct
- hreflang set present: `en / es / ja / zh-TW / x-default`
- non-zh-TW main-content routing defect detected in zh-TW pages: **0**

External href parity EN ↔ zh-TW:

- Seoul Sky: **11 / 11**
- Taste Korea: **0 / 0**
- Taxi: **8 / 8**
- T-money vs WOWPASS: **5 / 5**
- T-money: **8 / 8**
- Stay Dongdaemun: **24 / 24**
- Stay Gangnam: **30 / 30**
- Stay Hongdae: **39 / 39**
- Stay Insadong: **21 / 21**
- Stay Itaewon: **18 / 18**

Affiliate-marked link parity where present:

- Seoul Sky: **4 / 4**
- Stay Dongdaemun: **24 / 24**
- Stay Gangnam: **30 / 30**
- Stay Hongdae: **39 / 39**
- Stay Insadong: **21 / 21**
- Stay Itaewon: **18 / 18**

HTML implementation performed in this audit: **0**  
stage / commit / push / Production performed: **0**

---

# 14. Exact implementation rule for later Codex work

When implementation is eventually requested:

1. Use **this file as the exact correction Source of Truth for Batch 01I**.
2. Apply only the **17 correction items** above.
3. Do not rewrite PASS pages.
4. Do not rewrite any other zh-TW wording on FIX pages.
5. Do not change facts, numbers, dates, hotel order, room configuration, transport details, recommendation order or recommendation strength.
6. Do not independently refresh Seoul Sky rules, taxi fares, T-money rules, hotel conditions, airport buses or other time-sensitive data during this language-only implementation.
7. Do not change classes, IDs, `data-*`, images, CSS, JS logic, affiliate URLs, tracking, canonical or hreflang unless separately approved.
8. Preserve all `data-guide-year="current"` markers exactly.
9. Do not bulk-replace `地方`; only apply the two exact Taste Korea corrections above.
10. Do not alter FAQ/schema wording because none of the 17 corrections is an FAQ-answer correction.
11. Preserve T-money infographic assets; embedded-English visual localization is handled on the separate visual-localization track.
12. If any OLD string is missing or occurs in an unexpected location, STOP instead of guessing.
13. After exact implementation, perform only mechanical QA. Do **not** repeat the localization audit.

---

# 15. Batch state

**Batch 01I = REVIEW COPY — AWAITING USER APPROVAL**

After user approval:
- wording becomes **APPROVED PUBLIC COPY — CONTENT LOCKED**
- later Codex implementation must be exact
- no second editorial localization audit
