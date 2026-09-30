# Korea Inside — zh-TW Localization Audit + Correction — Batch 01D

- File: `Korea_Inside_ZH-TW_Localization_Audit_Correction_Batch01D_4pages_2026-09-29.md`
- Date: 2026-09-29
- Status: **REVIEW COPY — AWAITING USER APPROVAL**
- Scope: Current English Production ↔ current zh-TW Production direct comparison
- Work unit: **4 pages**
- Rule: **This correction MD is the final localization audit for these four pages. After exact implementation, do not run another editorial localization audit unless a concrete defect appears.**

---

## 1. Batch scope

1. `zh-tw/best-area-for-first-time-visitors-seoul.html`
2. `zh-tw/best-area-for-luxury-hotels-seoul.html`
3. `zh-tw/best-area-for-nightlife-seoul.html`
4. `zh-tw/best-area-for-shopping-seoul.html`

Production basis checked directly on 2026-09-29:

- English and zh-TW public pages: HTTP 200
- Current English wording used as factual / numeric / recommendation basis
- Current zh-TW wording judged directly for Taiwan Traditional Chinese naturalness
- No page after these four was inspected in this Batch

---

# 2. Final result

| Page | Result | Corrections |
|---|---|---:|
| `best-area-for-first-time-visitors-seoul.html` | FIX | 6 |
| `best-area-for-luxury-hotels-seoul.html` | FIX | 2 |
| `best-area-for-nightlife-seoul.html` | FIX | 9 |
| `best-area-for-shopping-seoul.html` | FIX | 11 |
| **TOTAL** |  | **28** |

The corrections below are the exact OLD → NEW strings.

Do not paraphrase them during implementation.

---

# 3. Page 1 — `zh-tw/best-area-for-first-time-visitors-seoul.html`

## Result: FIX 6

### FIX 01 — comparison table / Seoul Station daily travel

OLD:

```text
交通連接強
```

NEW:

```text
交通銜接方便
```

Reason:
- `交通連接強` is an English-derived calque.
- Preserves the source judgment that Seoul Station has strong transport connections.

### FIX 02 — comparison table / Seoul Station evening feel

OLD:

```text
實用為主，氣氛較少
```

NEW:

```text
偏重實用，街區氣氛較弱
```

Reason:
- Naturalizes `Practical rather than atmospheric`.
- Recommendation strength unchanged.

### FIX 03 — comparison table / Seoul Station trade-off

OLD:

```text
街區特色較少
```

NEW:

```text
街區特色較不鮮明
```

Reason:
- Natural Taiwan wording for `Less neighborhood character`.

### FIX 04 — luggage storage paragraph

Context:
- H3: `行李寄放`

OLD:

```text
入住前或退房後可寄放行李，能讓第一天與最後一天觀光輕鬆許多，尤其班機時間與飯店時間不吻合時。
```

NEW:

```text
入住前或退房後可寄放行李，能讓第一天與最後一天觀光輕鬆許多，尤其班機時間和飯店入住／退房時間對不上時。
```

Reason:
- `飯店時間` is vague and unnatural.
- Restores the intended check-in / check-out timing condition.

### FIX 05 — cheapest-hotel FAQ answer

Context:
- FAQ question: `應該選最便宜的飯店嗎？`
- **Visible FAQ + FAQPage JSON-LD must be updated identically.**

OLD:

```text
最便宜的房間不一定最划算。 位置不方便，可能增加交通時間、計程車費與辛苦的步行，短期初訪尤其有感。
```

NEW:

```text
最便宜的房間不一定最划算。位置不方便，可能增加交通時間、計程車費與辛苦的步行，短期初訪尤其有感。
```

Reason:
- Removes the incorrect ASCII space after Chinese punctuation.
- Meaning unchanged.

### FIX 06 — final recommendation

Context:
- H2: `初訪旅客該選首爾哪一區？`

OLD:

```text
明洞仍是許多初訪旅客各方面均衡、最容易安排的住宿區。夜生活與直達機場鐵路更重要時，弘大是較強的替代選擇；行李與抵達離開的移動，比晚間氣氛更值得關注時，首爾站或麻浦／孔德更合理。
```

NEW:

```text
明洞仍是許多初訪旅客各方面均衡、最容易安排的住宿區。夜生活與直達機場鐵路更重要時，弘大是更合適的替代選擇；行李與抵達離開的移動，比晚間氣氛更值得關注時，首爾站或麻浦／孔德更合理。
```

Reason:
- Removes the literal `stronger alternative → 較強的替代選擇` calque.
- Preserves the same conditional Hongdae recommendation.

### Page 1 protection

- Major factual mismatch: 0
- Numeric mismatch: 0
- Material recommendation drift: 0
- Large omission / invention: 0
- Structure: H1 1 / H2 10 / H3 31 — EN ↔ zh-TW parity
- FAQ: 14 visible / 14 FAQPage JSON-LD
- Visible FAQ ↔ JSON-LD text parity: 14/14 when inline link boundaries are normalized
- After FIX 05, visible FAQ and JSON-LD must remain exact mirrors
- `lang="zh-TW"`: correct
- self canonical: correct
- hreflang set: `en / es / ja / zh-TW / x-default`
- main-content Taiwan routing defect detected: 0

---

# 4. Page 2 — `zh-tw/best-area-for-luxury-hotels-seoul.html`

## Result: FIX 2

### FIX 01 — comparison table / Seoul Station-Namdaemun trade-off

OLD:

```text
出口複雜，街區氛圍較少
```

NEW:

```text
出口複雜，街區生活感較弱
```

Reason:
- `街區氛圍較少` is unnatural.
- Preserves `less neighborhood atmosphere`.

### FIX 02 — view-category paragraph

Context:
- H3: `景觀房型`

OLD:

```text
城市、河景與地標景觀通常對應特定房型，而不是飯店名稱本身。重要的是所訂房價方案附帶的確切文字。
```

NEW:

```text
城市、河景與地標景觀通常對應特定房型，而不是飯店名稱本身。真正重要的是實際預訂的房價方案如何標示。
```

Reason:
- Removes literal English wording.
- Preserves the instruction to check the exact booked rate/category wording.

### Page 2 protection

- Major factual mismatch: 0
- Numeric mismatch: 0
- Material recommendation drift: 0
- Large omission / invention: 0
- Structure: H1 1 / H2 10 / H3 40 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- Visible FAQ ↔ JSON-LD exact text parity: 8/8
- `lang="zh-TW"`: correct
- self canonical: correct
- hreflang set: `en / es / ja / zh-TW / x-default`
- main-content Taiwan routing defect detected: 0

---

# 5. Page 3 — `zh-tw/best-area-for-nightlife-seoul.html`

## Result: FIX 9

### FIX 01 — practical-answer paragraph

Context:
- H2: `夜生活行程，哪裡最適合住宿？`

OLD:

```text
夜生活只是旅行一部分，明洞、麻浦／孔德與首爾站更合理。以門外沒有酒吧作為取捨，換取更容易觀光、安靜睡眠、機場移動或行李便利。
```

NEW:

```text
夜生活只是旅行的一部分時，明洞、麻浦／孔德與首爾站更合理。這些區域犧牲飯店門口就有酒吧的便利，換來更容易觀光、較安靜的睡眠、機場移動或行李便利。
```

Reason:
- Current second sentence is a direct-translation structure and reads unnaturally.
- Preserves the English `trade doorstep bars for...` logic.

### FIX 02 — comparison table / Itaewon trade-off

OLD:

```text
對首爾其他行程需求幫助較少
```

NEW:

```text
較難兼顧首爾其他行程需求
```

Reason:
- Naturalizes `Less useful for other Seoul priorities`.

### FIX 03 — comparison table / Seoul Station late return

OLD:

```text
從市中心搭計程車實用
```

NEW:

```text
從市中心搭計程車回來還算方便
```

Reason:
- Restores a natural late-return sentence in Taiwan Chinese.
- Meaning unchanged.

### FIX 04 — Itaewon terrain paragraph

OLD:

```text
地形也要留意。坡道與小支巷，可能讓深夜最後走回飯店的路，比起初地圖上看起來更長。
```

NEW:

```text
地形也要留意。坡道與小支巷，可能讓深夜最後走回飯店的路，比一開始從地圖上看起來更長。
```

Reason:
- Fixes malformed `比起初地圖`.

### FIX 05 — Gangnam spending-level paragraph

OLD:

```text
江南也通常比弘大休閒夜生活適合更高消費，所以選擇不只關乎位置，也關乎晚上的風格。
```

NEW:

```text
江南的夜生活通常也比弘大需要更高的消費預算，所以選擇不只關乎位置，也關乎晚上的風格。
```

Reason:
- Current word order is unnatural.
- Preserves the source judgment that Gangnam generally fits a higher spending level.

### FIX 06 — Seoul Station short-trip trade-off

OLD:

```text
短旅行中，一次晚歸比不上抵達、離開或後續移動方便時，這項取捨可能值得。
```

NEW:

```text
短旅行中，如果只有一次晚歸，而抵達、離開或後續移動更重要，這項取捨可能值得。
```

Reason:
- Current sentence is grammatically awkward.
- Preserves the source comparison between one late night and transport convenience.

### FIX 07 — exact-block / sleep paragraph

OLD:

```text
稍遠離最繁忙街道，常能享有相同晚間交通，又更輕鬆結束一天。
```

NEW:

```text
稍微遠離最繁忙的街道，常能保有同樣的夜生活便利，又能更輕鬆地結束一天。
```

Reason:
- English refers to evening/nightlife access, not `晚間交通`.
- Corrects a minor semantic wording mismatch.

### FIX 08 — Myeongdong FAQ answer

Context:
- FAQ question: `明洞適合夜生活嗎？`
- **Visible FAQ + FAQPage JSON-LD must be updated identically.**

OLD:

```text
夜生活只是旅行的一部分，明洞仍可以是好的住宿區。比起酒吧夜店就在飯店外，它更擅長市中心觀光與購物。
```

NEW:

```text
夜生活只是旅行的一部分時，明洞仍可以是很好的住宿區。比起飯店門外就有酒吧夜店，它更適合市中心觀光與購物。
```

Reason:
- `明洞更擅長` is unnatural personification.
- Recommendation strength and page logic unchanged.

### FIX 09 — related budget-guide description

Context:
- Related guide: `首爾預算型住宿區`
- `<span>` only

OLD:

```text
比較飯店性價比，也算進反覆深夜交通的實際費用。
```

NEW:

```text
比較飯店是否划算，也把反覆深夜交通的實際費用算進去。
```

Reason:
- Removes Mainland-oriented `性價比`.
- Preserves the English `hotel value + real cost of repeated late-night transport`.

### Page 3 protection

- Major factual mismatch: 0
- Numeric mismatch: 0
- Material recommendation drift: 0
- Large omission / invention: 0
- Minor semantic wording mismatch: 1 → corrected by FIX 07
- Mainland-oriented terminology defect: 1 → corrected by FIX 09
- Structure: H1 1 / H2 9 / H3 24 — EN ↔ zh-TW parity
- FAQ: 10 visible / 10 FAQPage JSON-LD
- Visible FAQ ↔ JSON-LD exact text parity: 10/10
- After FIX 08, visible FAQ and JSON-LD must remain exact mirrors
- `lang="zh-TW"`: correct
- self canonical: correct
- hreflang set: `en / es / ja / zh-TW / x-default`
- main-content Taiwan routing defect detected: 0

---

# 6. Page 4 — `zh-tw/best-area-for-shopping-seoul.html`

## Result: FIX 11

### FIX 01 — Hero / lead paragraph

OLD:

```text
購物旅行，買完後的位置也重要。提重袋子回飯店的簡單路線、白天有地方放行李，以及方便的機場交通，可能和靠近店家同樣實用。
```

NEW:

```text
購物旅行中，買完東西後怎麼回飯店也很重要。提著重袋子能輕鬆回飯店、白天有地方寄放行李，以及機場交通方便，可能和靠近店家同樣實用。
```

Reason:
- `買完後的位置也重要` is an English-derived structure.
- Preserves the hotel-route / luggage / airport logic.

### FIX 02 — hotel-practicality paragraph

Context:
- H3: `飯店本身仍然重要`

OLD:

```text
購物街區再理想，也無法補償難走的飯店路線、沒有寄放，或房間攤不開兩個行李箱。購物很多的旅行，位置與房間實用性要一起看。
```

NEW:

```text
購物街區再理想，也無法補償難走的飯店路線、沒有行李寄放服務，或房間攤不開兩個行李箱。購物很多的旅行，位置與房間實用性要一起看。
```

Reason:
- Restores the missing object after `沒有寄放`.
- Meaning unchanged.

### FIX 03 — comparison table / Dongdaemun trade-off

OLD:

```text
零售與批發購買條件不同
```

NEW:

```text
零售與批發的購物方式不同
```

Reason:
- English refers to varying retail/wholesale access, not booking/purchase conditions.
- Corrects a small semantic shift.

### FIX 04 — comparison table / Seongsu trade-off

OLD:

```text
不太適合當全首爾通用住宿區
```

NEW:

```text
不太適合作為遊遍首爾的住宿據點
```

Reason:
- `全首爾通用住宿區` is a literal calque.
- Preserves the source judgment that Seongsu is weak as an all-Seoul base.

### FIX 05 — Gangnam airport paragraph

OLD:

```text
機場往返也比弘大或孔德不直接。購物多的旅行，離開日最有感，因為箱子很可能比抵達時重。
```

NEW:

```text
機場往返也不如弘大或孔德直接。購物多的旅行，離開日最有感，因為箱子很可能比抵達時重。
```

Reason:
- Fixes broken comparative grammar.
- Meaning unchanged.

### FIX 06 — Dongdaemun hotel-base paragraph

OLD:

```text
只計畫一晚東大門的旅客，住市中心、從別區過來可能更簡單。深夜服飾購物確實是此行主因之一時，才更有理由住這裡。
```

NEW:

```text
只計畫一個晚上逛東大門的旅客，住在市中心、從別區過來可能更簡單。只有當深夜服飾購物確實是這趟旅行的主要目的之一時，才更有理由住這裡。
```

Reason:
- `一晚東大門` is ambiguous and unnatural.
- Restores the English meaning: one Dongdaemun evening, not necessarily one hotel night.

### FIX 07 — Gangnam shopping-cluster paragraph

Context:
- H3: `百貨與高級購物`

OLD:

```text
高級購物日，把江南當成幾個獨立商圈，而非一個可全程步行的區域更好。COEX 與三成適合一種安排，狎鷗亭與清潭則更適合另一天。
```

NEW:

```text
安排高級購物日時，把江南視為幾個獨立商圈，而不是一個可以全程步行的區域會更實際。COEX 與三成適合安排成一種行程，狎鷗亭與清潭則適合另一種。
```

Reason:
- Removes English-derived sentence order.
- Preserves the source distinction between Gangnam shopping clusters.

### FIX 08 — Dongdaemun retail mistake

OLD:

```text
東大門有不同購物類型，不是每棟都以相同方式服務隨意逛逛的個別旅客。
```

NEW:

```text
東大門有不同購物類型，不是每棟大樓都同樣適合一般散客。
```

Reason:
- Natural Taiwan wording for `casual individual visitors`.
- No change to the retail/wholesale judgment.

### FIX 09 — tax-refund mistake

OLD:

```text
資格與流程可能不同。退稅期待應依當前規則與實際店家，而不是對韓國購物的籠統想法。
```

NEW:

```text
退稅資格與流程可能不同。是否能退稅，應以當前規則與實際店家為準，而不是憑對韓國購物的籠統印象。
```

Reason:
- `退稅期待` is unnatural.
- Preserves the warning to rely on current rules and the actual store.

### FIX 10 — luggage FAQ answer

Context:
- FAQ question: `帶購物袋與行李，哪種飯店位置最方便？`
- **Visible FAQ + FAQPage JSON-LD must be updated identically.**

OLD:

```text
購物旅行中，飯店最後一段路比起初想像更重要。電梯、簡單地鐵出口、行李寄放與隨時放回購物袋的便利，能讓一天輕鬆許多。
```

NEW:

```text
購物旅行中，飯店最後一段路比一開始想像的更重要。電梯、簡單的地鐵出口、行李寄放，以及能隨時把購物袋放回飯店的便利，都能讓一天輕鬆許多。
```

Reason:
- Fixes malformed `比起初想像`.
- Naturalizes the final list without changing the hotel-selection logic.

### FIX 11 — K-Beauty related-guide description

Context:
- Related guide: `韓系美妝指南`
- `<span>` only

OLD:

```text
規劃產品、購物期待與實用美妝停留。
```

NEW:

```text
規劃想買的產品、購物期待與實際可安排的美妝行程。
```

Reason:
- `實用美妝停留` is a direct English calque.
- Keeps the related-guide role without inventing a new recommendation.

### Page 4 protection

- Major factual mismatch: 0
- Numeric mismatch: 0
- Material recommendation drift: 0
- Large omission / invention: 0
- Minor semantic wording mismatch: 2 → corrected by FIX 03 and FIX 06
- Structure: H1 1 / H2 11 / H3 29 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- Visible FAQ ↔ JSON-LD exact text parity: 8/8
- After FIX 10, visible FAQ and JSON-LD must remain exact mirrors
- `lang="zh-TW"`: correct
- self canonical: correct
- hreflang set: `en / es / ja / zh-TW / x-default`
- main-content Taiwan routing defect detected: 0

---

# 7. Batch 01D final audit checks

## Content

- Major factual mismatch: **0**
- Numeric mismatch: **0**
- Material recommendation drift: **0**
- Large omission: **0**
- Large invention: **0**
- English page-role distortion: **0**

## Taiwan localization

- Exact corrections: **28**
- Minor semantic wording mismatches: **3**
- Mainland-oriented terminology defects: **1** (`性價比`)
- Simplified-Chinese character contamination detected: **0**
- Recommendation softening / strengthening caused by localization: **0**

## Structure / schema

- `best-area-for-first-time-visitors-seoul.html`
  - H1 1 / H2 10 / H3 31 / visible FAQ 14 / FAQPage 14
- `best-area-for-luxury-hotels-seoul.html`
  - H1 1 / H2 10 / H3 40 / visible FAQ 8 / FAQPage 8
- `best-area-for-nightlife-seoul.html`
  - H1 1 / H2 9 / H3 24 / visible FAQ 10 / FAQPage 10
- `best-area-for-shopping-seoul.html`
  - H1 1 / H2 11 / H3 29 / visible FAQ 8 / FAQPage 8

EN ↔ zh-TW structure mismatch: **0**

Current visible FAQ ↔ FAQPage JSON-LD text parity after normalizing inline-link boundaries:

- First-Time Visitors: **14/14**
- Luxury Hotels: **8/8**
- Nightlife: **10/10**
- Shopping: **8/8**

JSON-LD parse error: **0**

## Technical protection check

- `lang="zh-TW"`: **4/4**
- self canonical: **4/4**
- hreflang set present (`en / es / ja / zh-TW / x-default`): **4/4**
- non-zh-TW main-content internal routing defect detected: **0**
- external / affiliate href mismatch against English in current main content: **0**
- HTML implementation performed in this audit: **0**
- stage / commit / push / Production performed: **0**

---

# 8. Exact implementation rule for later Codex work

When implementation is eventually requested:

1. Use **this file as the exact correction Source of Truth for Batch 01D**.
2. Apply only the 28 OLD → NEW corrections above.
3. Do not rewrite any other zh-TW wording.
4. Do not change facts, numbers, recommendation order or recommendation strength.
5. Do not change section order, classes, IDs, `data-*`, images, CSS, JS, affiliate/tracking, canonical or hreflang unless a separate approved technical instruction explicitly requires it.
6. Preserve all current-year automation markers exactly.
7. For First-Time FIX 05, Nightlife FIX 08 and Shopping FIX 10, update **visible FAQ + FAQPage JSON-LD with exactly the same NEW wording**.
8. Preserve all other FAQ/schema wording unchanged.
9. If any OLD string is missing or appears in an unexpected location, STOP instead of guessing.
10. After exact implementation, perform only mechanical QA. Do **not** repeat the localization audit.

---

# 9. Batch state

**Batch 01D = REVIEW COPY — AWAITING USER APPROVAL**

After user approval:
- wording becomes **APPROVED PUBLIC COPY — CONTENT LOCKED**
- later Codex implementation must be exact
- no second editorial localization audit
