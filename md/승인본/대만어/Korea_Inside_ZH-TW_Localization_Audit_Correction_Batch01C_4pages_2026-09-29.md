# Korea Inside — zh-TW Localization Audit + Correction — Batch 01C

- File: `Korea_Inside_ZH-TW_Localization_Audit_Correction_Batch01C_4pages_2026-09-29.md`
- Date: 2026-09-29
- Status: **REVIEW COPY — AWAITING USER APPROVAL**
- Scope: Current English Production ↔ current zh-TW Production direct comparison
- Work unit: **4 pages**
- Rule: **This correction MD is the final localization audit for these four pages. After exact implementation, do not run another editorial localization audit unless a concrete defect appears.**

---

## 1. Batch scope

1. `zh-tw/best-area-for-airport-access-seoul.html`
2. `zh-tw/best-area-for-budget-travelers-seoul.html`
3. `zh-tw/best-area-for-couples-seoul.html`
4. `zh-tw/best-area-for-families-seoul.html`

Production basis checked directly on 2026-09-29:

- English and zh-TW public page: HTTP 200
- Current English wording used as the factual / recommendation basis
- Current zh-TW wording judged directly for Taiwan Traditional Chinese naturalness
- No future pages were inspected

---

# 2. Final result

| Page | Result | Corrections |
|---|---|---:|
| `best-area-for-airport-access-seoul.html` | FIX | 2 |
| `best-area-for-budget-travelers-seoul.html` | FIX | 5 |
| `best-area-for-couples-seoul.html` | FIX | 5 |
| `best-area-for-families-seoul.html` | FIX | 7 |
| **TOTAL** |  | **19** |

The corrections below are the exact approved-candidate OLD → NEW strings.

Do not paraphrase them during implementation.

---

# 3. Page 1 — `zh-tw/best-area-for-airport-access-seoul.html`

## Result: FIX 2

### FIX 01 — AREX / rail decision paragraph

Context:
- H2: `AREX、機場巴士，還是計程車？`
- H3: `鐵路銜接確實簡單時，選 AREX`

OLD:

```text
弘大、孔德與首爾站最有理由選鐵路，因為普通列車可到這三站，不必先轉其他首爾地鐵線。首爾站另有直達列車。
```

NEW:

```text
弘大、孔德與首爾站最適合優先考慮鐵路，因為 AREX 普通列車可直達這三站，不必先轉乘其他首爾地鐵線。首爾站另有直達列車。
```

Reason:
- `最有理由選鐵路` is a direct-translation calque.
- Adds `AREX` explicitly without changing the English fact or recommendation.
- Recommendation strength remains unchanged.

### FIX 02 — private transfer paragraph

Context:
- H3: `重視抵達安排的確定性時，考慮私人接送`

OLD:

```text
接機會合、兒童出行、長輩同行或大量行李等需求，重要到讓你想在起飛前就安排好抵達方式時，預約接送會更實用。
```

NEW:

```text
如果帶孩子或長輩同行、行李很多，或需要事先安排接機會合方式，而且希望起飛前就把抵達流程確定下來，預約接送會更實用。
```

Reason:
- Removes stiff English-derived sentence structure.
- Preserves the original conditional recommendation.

### Page 1 protection

- Facts: unchanged
- Numbers: unchanged
- Recommendation order / strength: unchanged
- H1 / H2 / H3 structure: unchanged
- FAQ: 7 visible / FAQPage JSON-LD 0 — unchanged
- Canonical / hreflang: no correction required
- Internal Taiwan routing: no correction required

---

# 4. Page 2 — `zh-tw/best-area-for-budget-travelers-seoul.html`

## Result: FIX 5

### FIX 01 — `<title>`

OLD:

```text
省預算去首爾住哪裡：總花費與便利性地圖｜Korea Inside
```

NEW:

```text
首爾省錢住哪區？總花費與便利性比較｜Korea Inside
```

Reason:
- `省預算去首爾住哪裡` is unnatural Taiwan search-facing wording.
- `比較` matches the actual decision role more naturally than the literal title construction.
- Page role and recommendation logic are unchanged.

### FIX 02 — H1

Preserve the existing current-year marker exactly.

OLD:

```html
<h1>省預算去首爾住哪裡 <span data-guide-year="current">2026</span></h1>
```

NEW:

```html
<h1>首爾省錢住哪區？ <span data-guide-year="current">2026</span></h1>
```

Reason:
- Natural Taiwan Traditional Chinese question form.
- `data-guide-year="current"` must remain unchanged.

### FIX 03 — quick-decision label

Context:
- H2: `首爾哪一區適合預算旅行？`
- `<dt>`

OLD:

```text
機場往返日更輕鬆
```

NEW:

```text
往返機場的日子更輕鬆
```

Reason:
- Fixes compressed translated word order.

### FIX 04 — comparison-table wording

Context:
- Area: `新村`
- Column: `適合情況`

OLD:

```text
重視日常性價比的長住
```

NEW:

```text
重視日常花費的長住旅客
```

Reason:
- `性價比` is not the preferred Taiwan-localized wording here.
- Preserves the English meaning: longer stays focused on daily value.

### FIX 05 — hostel FAQ answer

Context:
- FAQ question: `青年旅館一定比平價飯店便宜嗎？`
- **Visible FAQ + FAQPage JSON-LD must be updated identically.**

OLD:

```text
不一定適合每位旅客。熱門日期的青年旅館私人房，可能接近飯店價格；兩人同行，有時基本飯店房間反而更划算。
```

NEW:

```text
不一定。熱門日期的青年旅館私人房，可能接近飯店價格；兩人同行，有時基本飯店房間反而更划算。
```

Reason:
- The English answer means “not necessarily cheaper.”
- Current zh-TW changes that into “not suitable for every traveler,” which is a minor semantic mismatch.
- NEW restores the original meaning without changing the recommendation.

### Page 2 protection

- Major factual mismatch: 0
- Numeric mismatch: 0
- Material recommendation drift: 0
- Minor semantic mismatch: 1 → corrected above
- Mainland-oriented terminology defect: 1 → corrected above
- Structure: H1 1 / H2 13 / H3 20 — EN ↔ zh-TW parity
- FAQ: 12 visible / 12 FAQPage JSON-LD
- Current visible FAQ ↔ JSON-LD exact parity: 12/12
- After FIX 05, visible FAQ and JSON-LD must remain exact mirrors
- Canonical / hreflang: no correction required
- Main-content internal Taiwan routing: no correction required

---

# 5. Page 3 — `zh-tw/best-area-for-couples-seoul.html`

## Result: FIX 5

### FIX 01 — room-value paragraph

Context:
- H3: `喜歡回去的房間，可能值得多花錢`

OLD:

```text
比起只找最低每晚房價，情侶常能從回去住得舒服的房間得到更多價值。在首爾玩了好幾個充實的一天後，床鋪尺寸、可用空間與方便的每日路線，可能更重要。
```

NEW:

```text
比起只找最低每晚房價，情侶通常更值得選一間每天回去都住得舒服的房間。在首爾連續玩了幾個整天後，床鋪尺寸、可用空間與方便的每日動線，可能更重要。
```

Reason:
- `好幾個充實的一天` is grammatically broken.
- Removes English-derived phrasing while preserving the same hotel-decision logic.

### FIX 02 — area comparison lead

Context:
- H2: `首爾情侶住宿區快速比較`

OLD:

```text
最大差異常在觀光結束後：想附近有夜生活、安靜散步、交通更容易，還是不排固定行程也值得花時間的街區。
```

NEW:

```text
最大差異常在觀光結束後：你可能會在意附近有沒有夜生活、能不能安靜散步、交通是否方便，或這個街區在沒有固定行程時，是否仍值得慢慢待著。
```

Reason:
- `不排固定行程` is malformed and reverses the intended expression.
- NEW preserves all four English decision factors.

### FIX 03 — Myeongdong trade-off paragraph

Context:
- H3: `明洞`

OLD:

```text
街區繁忙、以旅客為主，較少親密小街區或住宅感。比起日常氣氛，更重視便利的情侶，通常覺得這項取捨值得，尤其短期初訪時。
```

NEW:

```text
街區繁忙、以旅客為主，比較少小街區的親密感或住宅區氛圍。比起日常氣氛，更重視便利的情侶，通常會覺得這項取捨值得，尤其是短期初訪。
```

Reason:
- Fixes unnatural noun stacking while preserving the same Myeongdong trade-off.

### FIX 04 — luggage-storage paragraph

Context:
- H3: `行李寄放`

OLD:

```text
入住前或退房後寄放，能讓第一天與最後一天輕鬆許多，尤其班機時間讓飯店與機場行程之間差好幾小時時。
```

NEW:

```text
入住前或退房後能寄放行李，會讓第一天與最後一天輕鬆許多，尤其班機時間和飯店入住／退房時間之間有好幾個小時空檔時。
```

Reason:
- Current wording is incomplete and unnatural.
- NEW restores the intended practical time-gap condition.

### FIX 05 — laundry paragraph

Context:
- H3: `洗衣`

OLD:

```text
長天數旅行有洗衣設施很實用，能減少兩人要打包的衣物。附近自助洗衣店，可能和館內洗衣機同樣方便。
```

NEW:

```text
旅行天數較長時，有洗衣設施很實用，也能減少兩人需要打包的衣物。附近的自助洗衣店，可能和館內洗衣機同樣方便。
```

Reason:
- `長天數旅行` is unnatural Taiwan Chinese.
- Meaning and recommendation remain unchanged.

### Page 3 protection

- Major factual mismatch: 0
- Numeric mismatch: 0
- Material recommendation drift: 0
- Large omission / invention: 0
- Structure: H1 1 / H2 10 / H3 29 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- Current visible FAQ ↔ JSON-LD exact parity: 8/8
- Canonical / hreflang: no correction required
- Main-content internal Taiwan routing: no correction required

---

# 6. Page 4 — `zh-tw/best-area-for-families-seoul.html`

## Result: FIX 7

### FIX 01 — nightlife / bedtime paragraph

Context:
- H3: `白天熱鬧的街區，睡前感受可能不同`

OLD:

```text
白天有趣的熱鬧街區，到了就寢時間可能很累。住在繁忙購物或夜生活區附近的家庭，通常選安靜支巷，比直接住在最熱鬧晚間活動上方更舒服。
```

NEW:

```text
白天覺得有趣的熱鬧街區，到了孩子睡覺時間可能就顯得太吵。住在繁忙購物區或夜生活區附近時，選安靜的側街，通常比直接住在最熱鬧街段上方更舒服。
```

Reason:
- Current sentence attaches `很累` to the neighborhood and uses the unnatural phrase `晚間活動上方`.
- NEW preserves the original family-sleep judgment.

### FIX 02 — Seoul Station table trade-off

Context:
- Area: `首爾站`
- Column: `主要取捨`

OLD:

```text
街區氣氛較少
```

NEW:

```text
街區生活感較弱
```

Reason:
- Natural Taiwan wording for “less neighborhood atmosphere.”
- Recommendation strength unchanged.

### FIX 03 — laundry paragraph

Context:
- H3: `洗衣`

OLD:

```text
長天數家庭旅行中，洗衣比訂房時想像更有用。住客洗衣間或附近自助洗衣店，能減少需要打包的衣物，帶年幼孩子尤其實用。
```

NEW:

```text
家庭旅行天數較長時，洗衣設施會比訂房時想像得更實用。住客洗衣間或附近自助洗衣店，能減少需要打包的衣物，帶年幼孩子時尤其有幫助。
```

Reason:
- Fixes unnatural `長天數家庭旅行中`.
- Preserves the practical family-booking point.

### FIX 04 — luggage-storage paragraph

Context:
- H3: `行李寄放`

OLD:

```text
入住前或退房後可寄放行李，能讓第一天與最後一天輕鬆許多。飯店時間與機場或鐵路行程之間差好幾小時，尤其實用。
```

NEW:

```text
入住前或退房後可寄放行李，能讓第一天與最後一天輕鬆許多。當入住／退房時間和前往機場或搭火車之間有好幾個小時空檔時，尤其實用。
```

Reason:
- `飯店時間` is not natural or sufficiently precise.
- NEW restores the intended timing condition.

### FIX 05 — airport-access FAQ answer

Context:
- FAQ question: `家庭往返機場，哪一區最方便？`
- **Visible FAQ + FAQPage JSON-LD must be updated identically.**

OLD:

```text
弘大、孔德與首爾站有直達 AREX，機場交通特別容易掌握。其他區域透過機場巴士或計程車也可能方便，不必讓機場交通決定整趟住宿。
```

NEW:

```text
AREX 可直達弘大、孔德與首爾站，往返機場的路線特別容易掌握。其他區域搭機場巴士或計程車也可能很方便，不必讓機場交通決定整段住宿安排。
```

Reason:
- Naturalizes word order without changing the airport-access recommendation.

### FIX 06 — subway / taxi FAQ answer

Context:
- FAQ question: `家庭從機場應該搭地鐵還是計程車？`
- **Visible FAQ + FAQPage JSON-LD must be updated identically.**

OLD:

```text
飯店車站路線簡單、家人能應付行李時，鐵路通常更划算。搭晚班機、帶嬰兒車或好幾個大包，或出站後步行不方便時，計程車可能值得多花錢。
```

NEW:

```text
飯店與車站之間的動線簡單、家人也能應付行李時，鐵路通常更划算。搭晚班機抵達、帶嬰兒車或好幾個大件行李，或出站後步行不方便時，多花一些搭計程車可能更值得。
```

Reason:
- Fixes compressed translation syntax.
- Preserves the rail-versus-taxi conditional judgment.

### FIX 07 — cheapest-hotel FAQ answer

Context:
- FAQ question: `家庭應該訂最便宜的飯店嗎？`
- **Visible FAQ + FAQPage JSON-LD must be updated identically.**

OLD:

```text
最低每晚房價，不一定代表家庭總住宿成本最低。位置困難可能增加計程車、交通時間與每天不便；稍貴的房間，可能有更多空間與更容易走的路線。
```

NEW:

```text
最低每晚房價，不一定代表家庭整段住宿的總成本最低。位置不方便，可能增加計程車費、交通時間與每天的移動負擔；稍貴的房間，反而可能有更多空間與更好走的路線。
```

Reason:
- `位置困難` is an unnatural collocation.
- NEW preserves the total-cost judgment and hotel-selection logic.

### Page 4 protection

- Major factual mismatch: 0
- Numeric mismatch: 0
- Material recommendation drift: 0
- Large omission / invention: 0
- Structure: H1 1 / H2 9 / H3 27 — EN ↔ zh-TW parity
- FAQ: 13 visible / 13 FAQPage JSON-LD
- Current visible FAQ ↔ JSON-LD exact parity: 13/13
- After FIX 05–07, each visible FAQ answer and its JSON-LD answer must remain an exact mirror
- Canonical / hreflang: no correction required
- Main-content internal Taiwan routing: no correction required

---

# 7. Batch 01C final audit checks

## Content

- Major factual mismatch: **0**
- Numeric mismatch: **0**
- Material recommendation drift: **0**
- Minor semantic mismatch: **1** → corrected
- Large omission: **0**
- Large invention: **0**
- English page-role distortion: **0**

## Taiwan localization

- Blocking grammar / calque / unnatural Taiwan wording locations: **18**
- Minor semantic mismatch locations: **1**
- Total corrections: **19**
- Mainland-oriented terminology defect: **1** (`性價比`) → corrected
- Simplified-Chinese character contamination detected: **0**
- Recommendation softening / strengthening caused by localization: **0**

## Structure / schema

- `best-area-for-airport-access-seoul.html`
  - H1 1 / H2 11 / H3 11 / visible FAQ 7 / FAQPage 0
- `best-area-for-budget-travelers-seoul.html`
  - H1 1 / H2 13 / H3 20 / visible FAQ 12 / FAQPage 12
- `best-area-for-couples-seoul.html`
  - H1 1 / H2 10 / H3 29 / visible FAQ 8 / FAQPage 8
- `best-area-for-families-seoul.html`
  - H1 1 / H2 9 / H3 27 / visible FAQ 13 / FAQPage 13

EN ↔ zh-TW structure mismatch: **0**

Current FAQ visible ↔ JSON-LD exact parity:
- Budget: **12/12**
- Couples: **8/8**
- Families: **13/13**
- Airport Access: FAQPage schema intentionally **0**

JSON-LD parse error: **0**

## Technical protection check

- `lang="zh-TW"`: **4/4**
- self canonical: **4/4**
- hreflang set present (`en / es / ja / zh-TW / x-default`): **4/4**
- broken non-zh-TW main-content internal link detected: **0**
- external / affiliate href mismatch against English in current main content: **0**
- HTML implementation performed in this audit: **0**
- stage / commit / push / Production performed: **0**

---

# 8. Exact implementation rule for later Codex work

When implementation is eventually requested:

1. Use **this file as the exact correction Source of Truth for Batch 01C**.
2. Apply only the 19 OLD → NEW corrections above.
3. Do not rewrite any other zh-TW wording.
4. Do not change facts, numbers, recommendation order or recommendation strength.
5. Do not change section order, classes, IDs, `data-*`, images, CSS, JS, affiliate/tracking, canonical or hreflang unless a separate approved technical instruction explicitly requires it.
6. Preserve the H1 `data-guide-year="current"` marker exactly.
7. For Budget FIX 05 and Families FIX 05–07, update **visible FAQ + FAQPage JSON-LD with exactly the same NEW wording**.
8. Airport Access must remain **visible FAQ 7 / FAQPage JSON-LD 0**.
9. If any OLD string is missing or appears in an unexpected location, STOP instead of guessing.
10. After exact implementation, perform only mechanical QA. Do **not** repeat the localization audit.

---

# 9. Batch state

**Batch 01C = REVIEW COPY — AWAITING USER APPROVAL**

After user approval:
- wording becomes **APPROVED PUBLIC COPY — CONTENT LOCKED**
- later Codex implementation must be exact
- no second editorial localization audit
