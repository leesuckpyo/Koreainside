# Korea Inside — zh-TW Localization Audit + Correction — Batch 01J

- File: `Korea_Inside_ZH-TW_Localization_Audit_Correction_Batch01J_4pages_2026-09-30.md`
- Date: 2026-09-30
- Status: **REVIEW COPY — AWAITING USER APPROVAL**
- Scope: Current English Production ↔ current zh-TW Production direct comparison
- Work unit: **Final 4 pages**
- Rule: **This correction MD is the final localization audit for these four pages. After exact implementation, do not run another editorial localization audit unless a concrete defect appears.**

---

## 1. Batch scope

55. `zh-tw/where-to-stay-in-jamsil.html`
56. `zh-tw/where-to-stay-in-myeongdong.html`
57. `zh-tw/where-to-stay-in-seongsu.html`
58. `zh-tw/wowpass.html`

Production basis checked directly:

- Current English Production and current zh-TW Production
- Facts, numbers, recommendation strength and Taiwan localization judged in one pass
- This is the final Taiwan page batch; no page remains after these four
- This is a localization audit, not a new factual-refresh pass; current English Production remains the factual and recommendation Source of Truth

---

# 2. Final result

| Page | Result | Corrections |
|---|---|---:|
| `where-to-stay-in-jamsil.html` | FIX | 6 |
| `where-to-stay-in-myeongdong.html` | FIX | 5 |
| `where-to-stay-in-seongsu.html` | FIX | 4 |
| `wowpass.html` | FIX | 4 |
| **TOTAL** |  | **19** |

The corrections below are the exact OLD → NEW instructions.

Do not paraphrase them during implementation.

---

# 3. Page 55 — `zh-tw/where-to-stay-in-jamsil.html`

## Result: FIX 6

### FIX 01 — hero recommendation

OLD:

```text
首爾這一側是旅程重點時，住蠶室有道理。會花不少時間在 Lotte World 的家庭、有棒球或演唱會票的旅客，以及計畫好幾天在松坡、江南或 COEX 活動的人，住這裡能省下很多來回。
```

NEW:

```text
首爾這一側是旅程重點時，住蠶室就很合適。會花不少時間在 Lotte World 的家庭、有棒球或演唱會票的旅客，以及計畫好幾天在松坡、江南或 COEX 活動的人，住這裡能省下很多來回。
```

Reason:
- Removes the direct `makes sense → 有道理` calque.
- Strong conditional recommendation unchanged.

### FIX 02 — Delight Hotel fit

OLD:

```text
想要大型國際飯店那種較可預期的設施，這不會是我們的第一選擇。芳荑洞地點確實用得到時，它更有說服力。
```

NEW:

```text
想要大型國際飯店那種較可預期的設施，這不會是我們的第一選擇。芳荑洞地點確實用得到時，它就更值得考慮。
```

Reason:
- Naturalizes `more convincing`.
- Hotel-fit judgment unchanged.

### FIX 03 — Hotel Lake fit

OLD:

```text
主要想找湖畔落腳處，又不介意較舊飯店，可以有道理。
```

NEW:

```text
主要想找湖畔落腳處，又不介意較舊飯店，Hotel Lake 也可以很合適。
```

Reason:
- Removes the direct `can make sense → 可以有道理` calque.
- Makes the referent explicit.

### FIX 04 — Lotte Hotel World FAQ answer

Context:
- FAQ question: `帶孩子去 Lotte World，蠶室哪間飯店最省事？`

OLD:

```text
這不代表它自動適合整趟首爾旅程。在蠶室以外活動的天數越多，這項位置優勢就越低。
```

NEW:

```text
這不代表它一定適合整趟首爾旅程。在蠶室以外活動的天數越多，這項位置優勢就越低。
```

Reason:
- Naturalizes `does not automatically make it right`.
- Negative limitation unchanged.

### FIX 05 — Sports Complex FAQ answer

Context:
- FAQ question: `去蠶室綜合運動場看棒球或演唱會，應該住哪裡？`

OLD:

```text
先看綜合運動場或蠶室新內一側，不要直接預設住蠶室站。
```

NEW:

```text
先看綜合運動場或蠶室新內一側，不要一開始就直接選蠶室站。
```

Reason:
- Removes the literal `defaulting to Jamsil Station`.
- Recommendation order unchanged.

### FIX 06 — SIGNIEL FAQ answer

Context:
- FAQ question: `只為了去 Lotte World，住 SIGNIEL 值得嗎？`

OLD:

```text
若目的是方便去 Lotte World，Lotte Hotel World 更直接解決需求。想付費體驗的是住在 Lotte World Tower 與高樓層飯店本身，SIGNIEL 才更有道理。
```

NEW:

```text
若目的是方便去 Lotte World，Lotte Hotel World 更直接解決需求。想付費體驗的是住在 Lotte World Tower 與高樓層飯店本身時，SIGNIEL 才更合適。
```

Reason:
- Naturalizes `makes more sense`.
- Strong conditional SIGNIEL recommendation unchanged.

### Page 55 protection

- Hotel order / roles: unchanged
- Room sizes / bed layouts / occupancy / airport bus numbers: unchanged
- H1 1 / H2 6 / H3 14 — EN ↔ zh-TW parity
- Visible FAQ: 7
- FAQPage schema: intentionally 0
- External href parity: 21 / 21
- Affiliate-marked link parity: 21 / 21
- Numeric multiset parity EN ↔ zh-TW: PASS
- Internal zh-TW routing defect detected: 0
- H1 `data-guide-year="current"` marker: preserve exactly

---

# 4. Page 56 — `zh-tw/where-to-stay-in-myeongdong.html`

## Result: FIX 5

### FIX 01 — hero default choice

OLD:

```text
對大多數初訪旅客，明洞站一側是最省事的預設選擇。主要購物街好辨認，4 號線近，白天回飯店也方便。
```

NEW:

```text
對大多數初訪旅客，明洞站一側是最容易安排的住宿選擇。主要購物街好辨認，4 號線近，白天回飯店也方便。
```

Reason:
- Removes the literal `easiest default`.
- Recommendation strength unchanged.

### FIX 02 — Euljiro 1-ga / Sogong-dong fit

OLD:

```text
好幾天都會往市廳、光化門或鐘路走時，這一側就有意義。住乙支路 1 街，可把 2 號線納入日常動線，不必只因飯店以明洞住宿作宣傳，就一直回明洞站搭車。
```

NEW:

```text
好幾天都會往市廳、光化門或鐘路走時，這一側就更合適。住乙支路 1 街，可把 2 號線納入日常動線，不必只因飯店以明洞住宿作宣傳，就一直回明洞站搭車。
```

Reason:
- Naturalizes `this side starts to make sense`.
- Area-selection logic unchanged.

### FIX 03 — Hotel28 fit

OLD:

```text
如果比起出門就是明洞站，更在意住在購物與餐廳街道裡，Hotel28 Myeongdong 更有道理。飯店位於明洞 7 街，離乙支路 1 街站約 300 公尺。ibis Ambassador Myeongdong 的 6015 機場巴士站也只離飯店約 80 公尺，讓這個中心位置有出乎意料實用的抵達路線。
```

NEW:

```text
如果比起出門就是明洞站，更在意住在購物與餐廳街道裡，Hotel28 Myeongdong 更合適。飯店位於明洞 7 街，離乙支路 1 街站約 300 公尺。ibis Ambassador Myeongdong 的 6015 機場巴士站也只離飯店約 80 公尺，讓這個中心位置有出乎意料實用的抵達路線。
```

Reason:
- Removes the direct `makes more sense → 更有道理` calque.
- Hotel-fit judgment unchanged.

### FIX 04 — L7 final decision paragraph

OLD:

```text
對多數初訪旅客，可以先看明洞站旁的兩間飯店。想兼顧地點與較完整的住宿體驗，L7 有道理；Skypark III 則更集中在車站便利。
```

NEW:

```text
對多數初訪旅客，可以先看明洞站旁的兩間飯店。想兼顧地點與較完整的住宿體驗，L7 更合適；Skypark III 則更集中在車站便利。
```

Reason:
- Naturalizes `L7 makes sense`.
- Recommendation hierarchy unchanged.

### FIX 05 — Myeongdong Station vs Euljiro FAQ answer

Context:
- FAQ question: `明洞站和乙支路，住哪裡比較好？`
- This answer currently appears in **visible FAQ + FAQPage JSON-LD**.
- Update both identically.

OLD:

```text
想讓初訪簡單一些，或常搭 4 號線，明洞站是較省事的預設選擇。如果行程更常用到 2 號線，或多往市廳、鐘路及首爾其他地區移動，乙支路可能更合適。
```

NEW:

```text
想讓初訪簡單一些，或常搭 4 號線，明洞站是較容易安排的選擇。如果行程更常用到 2 號線，或多往市廳、鐘路及首爾其他地區移動，乙支路可能更合適。
```

Reason:
- Removes the literal `easier default`.
- Area comparison unchanged.

### Page 56 protection

- `但它不是每個人的最佳選擇` logic remains valid and should not be mechanically rewritten
- Hotel order / room size / bed / occupancy / airport bus / station / exit facts: unchanged
- H1 1 / H2 13 / H3 15 — EN ↔ zh-TW parity
- FAQ: 5 visible / 5 FAQPage JSON-LD
- Current FAQ parity: 5 / 5
- After FIX 05, visible FAQ and JSON-LD must remain exact mirrors
- External href parity: 33 / 33
- Affiliate-marked link parity: 33 / 33
- Numeric multiset parity EN ↔ zh-TW: PASS
- Internal zh-TW routing defect detected: 0
- H1 `data-guide-year="current"` marker: preserve exactly

---

# 5. Page 57 — `zh-tw/where-to-stay-in-seongsu.html`

## Result: FIX 4

### FIX 01 — hero opening sentence

OLD:

```text
時尚店、美妝空間、快閃店與咖啡廳，是最想體驗的首爾生活時，就住聖水。沿武場街周邊，舊工業建築與商店、咖啡去處共處同一條街。房間在附近，就能放下買的東西、休息一下再出門，不必把整個街區塞進一趟排得很緊的行程。
```

NEW:

```text
如果時尚店、美妝空間、快閃店與咖啡廳，正是你最想體驗的首爾生活，就住聖水。沿武場街周邊，舊工業建築與商店、咖啡去處共處同一條街。房間在附近，就能放下買的東西、休息一下再出門，不必把整個街區塞進一趟排得很緊的行程。
```

Reason:
- Current opening grammar is broken.
- Strong stay recommendation preserved exactly.

### FIX 02 — location-justification sentence

Context:
- Hero second paragraph
- Replace only the first sentence; preserve the following bold sentence unchanged.

OLD:

```text
不需要每天都待在這裡，位置才有理由。
```

NEW:

```text
不必每天都待在聖水，才值得住這裡。
```

Reason:
- Naturalizes `You do not need to spend every day here to justify the location`.
- Preserves the important nuance that a Seongsu stay does not require every day to be spent there.

### FIX 03 — strongest reason to stay

OLD:

```text
最有力的住宿理由，是在意的地方都靠近房間。早上逛店，午餐前放下購物袋，再回先前來不及逛的店。這份彈性，可能比前往只去一次的地方省幾分鐘更有價值。
```

NEW:

```text
住聖水最重要的理由，是在意的地方都靠近房間。早上逛店，午餐前放下購物袋，再回先前來不及逛的店。這份彈性，可能比前往只去一次的地方省幾分鐘更有價值。
```

Reason:
- Naturalizes `the strongest reason to stay`.
- Primary stay rationale preserved.

### FIX 04 — ONJAE family caution

OLD:

```text
願意接受這些安排，以換取合適床位配置的旅客，可考慮 ONJAE。但行李多的家庭抵達，這不是我們的預設選擇。
```

NEW:

```text
願意接受這些安排，以換取合適床位配置的旅客，可考慮 ONJAE。但對帶很多行李抵達的家庭，我們不會把它當成首選。
```

Reason:
- Removes literal `not our default`.
- Family-arrival caution unchanged.

### Page 57 protection

- Hotel / small-stay roles: unchanged
- Room sizes / beds / paid luggage storage / no-elevator / check-in facts: unchanged
- H1 1 / H2 7 / H3 15 — EN ↔ zh-TW parity
- Visible FAQ: 5
- FAQPage schema: intentionally 0
- External href parity: 11 / 11
- Affiliate-marked link parity: 11 / 11
- Numeric multiset parity EN ↔ zh-TW: PASS
- Internal zh-TW routing defect detected: 0
- H1 `data-guide-year="current"` marker: preserve exactly
- In FIX 02, preserve the existing `<strong>` around `不住聖水，也能好好逛聖水。`

---

# 6. Page 58 — `zh-tw/wowpass.html`

## Result: FIX 4

The previously identified domestic-card referent defect is **already corrected in current Production**.

Current correct wording includes:

```text
接受韓國國內信用卡或簽帳金融卡付款的實體商家
```

Do not restore `本國信用卡`.

### FIX 01 — hero WOWPASS decision

OLD:

```text
若只需要大眾運輸，一般 T-money 卡通常較簡單。還想要韓國付款餘額、支援的外幣儲值、現金提領，或 App 卡片管理功能時，WOWPASS 才更有意義。
```

NEW:

```text
若只需要大眾運輸，一般 T-money 卡通常較簡單。還想要韓國付款餘額、支援的外幣儲值、現金提領，或 App 卡片管理功能時，WOWPASS 才更適合。
```

Reason:
- Naturalizes `starts to make more sense`.
- Conditional product recommendation unchanged.

### FIX 02 — WOWPASS withdrawal vs Korean ATM access

Context:
- H2: `提領韓元`
- The existing internal link to `korea-atm-foreign-cards.html` must be preserved.
- Change the linked visible text from `韓國國內 ATM` to `韓國 ATM`.

OLD visible paragraph:

```text
重要差別在於，這是透過 WOWPASS 系統提領 WOWPASS 餘額，不等於擁有韓國銀行帳戶，能不受限制地使用韓國國內 ATM。
```

NEW visible paragraph:

```text
重要差別在於，這是透過 WOWPASS 系統提領 WOWPASS 餘額，不等於擁有韓國銀行帳戶，也不代表能不受限制地使用韓國 ATM。
```

Reason:
- Removes the compressed clause and redundant `韓國國內`.
- Preserves the factual distinction between WOWPASS withdrawal and unrestricted bank-account ATM access.

### FIX 03 — short-trip feature judgment

OLD:

```text
只在首爾待三天、只需要搭地鐵的旅客，不會因功能更多就自動受益。
```

NEW:

```text
只在首爾待三天、只需要搭地鐵的旅客，不會只因功能更多就一定更方便。
```

Reason:
- Naturalizes `does not automatically benefit from more features`.
- Negative recommendation unchanged.

### FIX 04 — FAQ payment balance vs T-money balance

Context:
- FAQ question: `儲進 WOWPASS 的錢可以搭地鐵嗎？`
- This answer currently appears in **visible FAQ + FAQPage JSON-LD**.
- Update both identically.

OLD:

```text
不會自動通用。WOWPASS 付款餘額與 T-money 餘額是分開的。要靠這張卡搭車前，先確認交通餘額。
```

NEW:

```text
不會。WOWPASS 付款餘額與 T-money 餘額是分開的。要靠這張卡搭車前，先確認交通餘額。
```

Reason:
- `不會自動通用` is unnecessary calque-like wording.
- The two-balance distinction remains explicit and unchanged.

### Page 58 protection

- `韓國國內信用卡或簽帳金融卡` is correct in the merchant-acceptance context; preserve
- WOWPASS / T-money two-balance architecture: unchanged
- Card price / withdrawal / fee / currencies / airport pickup facts: unchanged
- H1 1 / H2 16 / H3 7 — EN ↔ zh-TW parity
- FAQ: 13 visible / 13 FAQPage JSON-LD
- Current FAQ parity: 13 / 13
- After FIX 04, visible FAQ and JSON-LD must remain exact mirrors
- External official href parity: 4 / 4
- Affiliate-marked links: 0 / 0
- Numeric multiset parity EN ↔ zh-TW: PASS
- Internal zh-TW routing defect detected: 0
- WOWPASS / T-money infographic embedded-English localization remains a separate visual-localization track

---

# 7. Batch 01J final audit checks

## Content

- Major factual mismatch: **0**
- Material numeric mismatch: **0**
- Numeric multiset parity EN ↔ zh-TW: **4/4 pages**
- Material recommendation drift: **0**
- Large omission: **0**
- Large invention: **0**
- English page-role distortion: **0**

## Taiwan localization

- Exact correction items: **19**
- FIX pages: **4**
- PASS pages: **0**
- Main correction types:
  - `makes sense / convincing / default / strongest reason` calque cleanup
  - one broken Seongsu hero sentence
  - one WOWPASS ATM-access sentence cleanup
- Recommendation strength changed by correction: **0**

## Structure / schema

| Page | H1 | H2 | H3 | Visible FAQ | FAQPage |
|---|---:|---:|---:|---:|---:|
| Stay Jamsil | 1 | 6 | 14 | 7 | 0 |
| Stay Myeongdong | 1 | 13 | 15 | 5 | 5 |
| Stay Seongsu | 1 | 7 | 15 | 5 | 0 |
| WOWPASS | 1 | 16 | 7 | 13 | 13 |

EN ↔ zh-TW structural mismatch: **0**

Current visible FAQ ↔ FAQPage JSON-LD exact parity:

- Stay Myeongdong: **5/5**
- WOWPASS: **13/13**

Jamsil and Seongsu intentionally have visible FAQ without FAQPage schema.

JSON-LD parse error: **0**

## Link / affiliate / routing protection

All four pages:

- `lang="zh-TW"`: correct
- self canonical: correct
- hreflang set present: `en / es / ja / zh-TW / x-default`
- non-zh-TW main-content routing defect detected: **0**

External href parity EN ↔ zh-TW:

- Stay Jamsil: **21 / 21**
- Stay Myeongdong: **33 / 33**
- Stay Seongsu: **11 / 11**
- WOWPASS: **4 / 4**

Affiliate-marked link parity:

- Stay Jamsil: **21 / 21**
- Stay Myeongdong: **33 / 33**
- Stay Seongsu: **11 / 11**
- WOWPASS: **0 / 0**

HTML implementation performed in this audit: **0**  
stage / commit / push / Production performed: **0**

---

# 8. Taiwan 58-page audit completion state

With Batch 01J complete:

- Taiwan zh-TW page audit: **58 / 58 COMPLETE**
- Correction-MD audit coverage: **58 / 58**
- Pages left to audit: **0**
- Second editorial audit after implementation: **NOT REQUIRED**
- Next phase, only after user direction:
  - exact implementation from Batch correction MDs
  - mechanical QA only
  - no re-localization / no second editorial pass

Core lock rule remains:

> **Correction MD creation itself is the final localization audit.**

---

# 9. Exact implementation rule for later Codex work

When implementation is eventually requested:

1. Use **this file as the exact correction Source of Truth for Batch 01J**.
2. Apply only the **19 correction items** above.
3. Do not rewrite any other zh-TW wording.
4. Do not change facts, numbers, dates, hotel order, room configuration, transport details, recommendation order or recommendation strength.
5. Do not independently refresh hotel conditions, WOWPASS pricing, withdrawal limits, airport pickup rules or other time-sensitive data during this language-only implementation.
6. Do not change classes, IDs, `data-*`, images, CSS, JS logic, affiliate URLs, tracking, canonical or hreflang unless separately approved.
7. Preserve all `data-guide-year="current"` markers exactly.
8. Myeongdong FIX 05:
   - visible FAQ + FAQPage JSON-LD must use exactly the same NEW wording.
9. WOWPASS FIX 02:
   - preserve the existing internal link to `/zh-tw/korea-atm-foreign-cards.html`
   - update its visible anchor text to `韓國 ATM`
10. WOWPASS FIX 04:
   - visible FAQ + FAQPage JSON-LD must use exactly the same NEW wording.
11. Seongsu FIX 02:
   - change only the first sentence
   - preserve the existing `<strong>` wrapper and text for `不住聖水，也能好好逛聖水。`
12. Preserve valid Korean-domestic merchant wording in WOWPASS.
13. If any OLD string is missing or occurs in an unexpected location, STOP instead of guessing.
14. After exact implementation, perform only mechanical QA. Do **not** repeat the localization audit.

---

# 10. Batch state

**Batch 01J = REVIEW COPY — AWAITING USER APPROVAL**

Taiwan audit state:

**58 / 58 PAGES COMPLETE**

After user approval:
- wording becomes **APPROVED PUBLIC COPY — CONTENT LOCKED**
- later Codex implementation must be exact
- no second editorial localization audit
