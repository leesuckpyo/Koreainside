# Korea Inside — zh-TW Localization Audit + Correction — Batch 01G

- File: `Korea_Inside_ZH-TW_Localization_Audit_Correction_Batch01G_10pages_2026-09-30.md`
- Date: 2026-09-30
- Status: **REVIEW COPY — AWAITING USER APPROVAL**
- Scope: Current English Production ↔ current zh-TW Production direct comparison
- Work unit: **10 pages**
- Rule: **This correction MD is the final localization audit for these ten pages. After exact implementation, do not run another editorial localization audit unless a concrete defect appears.**

---

## 1. Batch scope

25. `zh-tw/gongdeok-mapo-seoul-guide.html`
26. `zh-tw/hongdae-travel-guide.html`
27. `zh-tw/hongdae-vs-myeongdong.html`
28. `zh-tw/hotels-near-gongdeok-station.html`
29. `zh-tw/hotels-near-seoul-station.html`
30. `zh-tw/incheon-airport-private-transfer.html`
31. `zh-tw/index.html`
32. `zh-tw/insadong-travel-guide.html`
33. `zh-tw/itaewon-travel-guide.html`
34. `zh-tw/jamsil-travel-guide.html`

Production basis checked directly:

- Current English Production and current zh-TW Production
- Facts, numbers, recommendation strength and Taiwan localization judged in one pass
- No page 35+ was inspected in this Batch
- This is a localization audit, not a factual-refresh pass; current English Production remains the factual and recommendation Source of Truth

---

# 2. Final result

| Page | Result | Corrections |
|---|---|---:|
| `gongdeok-mapo-seoul-guide.html` | FIX | 5 |
| `hongdae-travel-guide.html` | FIX | 1 |
| `hongdae-vs-myeongdong.html` | FIX | 3 |
| `hotels-near-gongdeok-station.html` | FIX | 3 |
| `hotels-near-seoul-station.html` | FIX | 3 |
| `incheon-airport-private-transfer.html` | FIX | 2 |
| `index.html` | FIX | 1 |
| `insadong-travel-guide.html` | FIX | 2 |
| `itaewon-travel-guide.html` | FIX | 3 |
| `jamsil-travel-guide.html` | FIX | 4 |
| **TOTAL** |  | **27** |

The corrections below are the exact OLD → NEW instructions.

Do not paraphrase them during implementation.

---

# 3. Page 25 — `zh-tw/gongdeok-mapo-seoul-guide.html`

## Result: FIX 5

### FIX 01 — English-language class wording

OLD:

```text
孔德附近目前可預約的課程，包括以英語進行的韓國傳統甜點製作體驗。
```

NEW:

```text
孔德附近目前可預約的課程，包括以英文進行的韓國傳統甜點製作體驗。
```

Reason:
- Taiwan-localized service/language wording should use `英文` here.
- Class availability and meaning unchanged.

### FIX 02 — repeat-visitor recommendation

OLD:

```text
對這類旅客，比起排滿的初訪行程，孔德更容易有安排的理由。
```

NEW:

```text
對這類旅客來說，孔德比在排得很滿的初訪行程中更值得安排。
```

Reason:
- `更容易有安排的理由` is an English-derived construction.
- Preserves the source judgment that Gongdeok is easier to justify for repeat visitors.

### FIX 03 — family heading

OLD:

```text
家庭也能享受，但別把晚間安排說得太精彩
```

NEW:

```text
家庭也能玩得愉快，但別把晚間行程說得過頭
```

Reason:
- Natural Taiwan heading for `Families Can Enjoy It — But Do Not Oversell the Evening`.
- Recommendation remains cautious rather than promotional.

### FIX 04 — FAQ/body typo and naturalness

OLD:

```text
美食、在地街區或已預約的體驗本來就在計畫中時，這裡就值得得多。
```

NEW:

```text
美食、在地街區或已預約的體驗本來就在計畫中時，這裡就更值得安排。
```

Reason:
- Fixes the clear `值得得多` error.
- Preserves the stronger conditional recommendation.

### FIX 05 — Gongdeok stay recommendation

OLD:

```text
重視 AREX、5 號線與 6 號線、在地餐廳，以及較安靜的晚間環境時，它可以是有力的選擇。
```

NEW:

```text
如果重視 AREX、5 號線與 6 號線、在地餐廳，以及較安靜的晚間環境，孔德會是很合適的選擇。
```

Reason:
- Removes the literal `strong choice → 有力的選擇` phrasing.
- Preserves the strength and conditions of the recommendation.

### Page 25 protection

- Facts / numbers: unchanged
- Recommendation order / strength: unchanged
- H1 1 / H2 19 / H3 56 — EN ↔ zh-TW parity
- FAQPage JSON-LD: 0
- External href parity: 5 / 5
- Affiliate-marked link parity: 5 / 5
- Internal zh-TW routing defect: 0
- H1 `data-guide-year="current"` marker: preserve exactly

---

# 4. Page 26 — `zh-tw/hongdae-travel-guide.html`

## Result: FIX 1

### FIX 01 — activity language wording

OLD:

```text
別以為首爾的活動一定以英文進行。目前弘大方案有英語帶領，也有多語影片引導等不同形式。
```

NEW:

```text
別以為首爾的活動一定以英文進行。目前弘大方案有英文帶領，也有多語影片引導等不同形式。
```

Reason:
- Aligns service-language terminology with Taiwan usage.
- Product conditions and recommendation unchanged.

### Page 26 protection

- Existing photo-booth terminology is already acceptable:
  - explanatory use: `四格拍貼機`
  - later concise uses: `拍貼機`
- Do not bulk-replace or expand those terms.
- Facts / numbers / current-event judgments: unchanged
- H1 1 / H2 16 / H3 67 — EN ↔ zh-TW parity
- FAQPage JSON-LD: 0
- External href parity: 6 / 6
- Affiliate-marked link parity: 5 / 5
- Internal zh-TW routing defect: 0
- H1 `data-guide-year="current"` marker: preserve exactly

---

# 5. Page 27 — `zh-tw/hongdae-vs-myeongdong.html`

## Result: FIX 3

### FIX 01 — shopping comparison table

OLD:

```text
韓系美妝與旅客容易逛的零售選擇更強。
```

NEW:

```text
韓系美妝與旅客容易逛的零售選擇更有優勢。
```

Reason:
- Removes literal `stronger → 更強` wording.
- Keeps Myeongdong's comparative advantage unchanged.

### FIX 02 — Myeongdong tourism-orientation FAQ answer

Context:
- FAQ question: `明洞會不會太觀光化？`
- This wording currently appears in visible FAQ + FAQPage JSON-LD.
- **Update both identically.**

OLD:

```text
明洞高度以旅客為主，帶來人潮，也帶來方便購物、多語服務與大量飯店選擇。覺得方便，還是少了生活感，取決於想從街區得到什麼。
```

NEW:

```text
明洞明顯以旅客為主，帶來人潮，也帶來方便購物、多語服務與大量飯店選擇。你覺得這是方便，還是少了生活感，取決於想從街區得到什麼。
```

Reason:
- `高度以旅客為主` is unnatural.
- Second sentence needs an explicit subject in Taiwan Chinese.
- Trade-off judgment remains unchanged.

### FIX 03 — final hotel-level decision paragraph

OLD:

```text
最後決定仍可能在飯店層級改變。明洞飯店若車站路線不好走，或弘大房間正好在最繁忙的夜生活街道上方，都可能比區域比較顯示的更不方便。區域選得差不多後，再比較實際考慮中的兩條飯店路線。
```

NEW:

```text
最後決定仍可能因實際飯店而改變。明洞飯店若車站路線不好走，或弘大房間正好在最繁忙的夜生活街道上方，都可能比區域比較看起來更不方便。區域方向大致確定後，再比較實際考慮中的兩間飯店路線。
```

Reason:
- Removes `飯店層級`, `區域比較顯示`, and `兩條飯店路線` calques.
- Preserves the protected hotel-level decision logic.

### Page 27 protection

- Facts / numbers: unchanged
- Area winner logic / recommendation conditions: unchanged
- H1 1 / H2 13 / H3 26 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- Current FAQ text parity: 8 / 8
- After FIX 02, visible FAQ and JSON-LD must remain exact mirrors
- External href parity: 30 / 30
- Affiliate-marked link parity: 30 / 30
- Internal zh-TW routing defect: 0

---

# 6. Page 28 — `zh-tw/hotels-near-gongdeok-station.html`

## Result: FIX 3

### FIX 01 — Gongdeok vs Mapo transport comparison

OLD:

```text
孔德與麻浦在 5 號線上只差一站，但作為飯店落腳處，用法不同。孔德是較強的交通樞紐；飯店本身、家庭房選擇或漢江一側更重要時，麻浦就更值得看。
```

NEW:

```text
孔德與麻浦在 5 號線上只差一站，但作為飯店落腳處，用法不同。孔德的轉乘功能更完整；飯店本身、家庭房選擇或漢江一側更重要時，麻浦就更值得看。
```

Reason:
- Removes literal `stronger transport hub → 較強的交通樞紐`.
- Keeps Gongdeok's transport advantage intact.

### FIX 02 — first / final night scenario

OLD:

```text
韓國旅程第一晚或最後一晚，機場交通格外重要時，孔德也可能有道理。
```

NEW:

```text
韓國旅程的第一晚或最後一晚，如果機場交通特別重要，住孔德也可能很合適。
```

Reason:
- `孔德也可能有道理` is a direct English calque.
- Conditional stay recommendation unchanged.

### FIX 03 — final transport-value recommendation

OLD:

```text
如果只因機場列車停靠就選孔德，先比其他首爾行程。當交通連接能解決旅程不只一部分，這一帶才最合適。
```

NEW:

```text
如果只因機場列車停靠就選孔德，先看看首爾其他行程怎麼安排。當交通優勢能同時解決旅程中的不只一種移動需求時，這一帶才最合適。
```

Reason:
- Naturalizes the English decision logic.
- Preserves the rule that airport access alone is not enough reason to choose the area.

### Page 28 protection

- Hotel ranking: none introduced
- Hotel / room / bed / occupancy / size / station / exit facts: unchanged
- H1 1 / H2 12 / H3 18 — EN ↔ zh-TW parity
- FAQ: 5 visible / 5 FAQPage JSON-LD, exact parity 5 / 5
- External href parity: 20 / 20
- Affiliate-marked link parity: 20 / 20
- Internal zh-TW routing defect: 0

---

# 7. Page 29 — `zh-tw/hotels-near-seoul-station.html`

## Result: FIX 3

### FIX 01 — weak case for staying near the station

OLD:

```text
如果車站只在行程出現一次，首爾站附近飯店就比較難說服人。大多數日子若在宮殿、鐘路、明洞、聖水或弘大，住這裡可能只是用一個方便的搭車日，換來好幾天不必要的往返。
```

NEW:

```text
如果整趟行程只會用到首爾站一次，住在附近的理由就沒那麼充分。大多數日子若在宮殿、鐘路、明洞、聖水或弘大，住這裡可能只是用一個方便的搭車日，換來好幾天不必要的往返。
```

Reason:
- `飯店就比較難說服人` is a direct calque.
- Preserves the negative stay judgment.

### FIX 02 — Hotel Gracery role

OLD:

```text
只有當首爾站是旅程一部分，而不是住宿全部理由，Hotel Gracery 才適合列在這份指南。
```

NEW:

```text
只有當首爾站只是旅程的一部分，而不是選住宿的唯一理由時，Hotel Gracery 才適合列在這份指南。
```

Reason:
- Repairs the literal `part of the trip rather than the whole reason for the stay`.
- Hotel inclusion judgment unchanged.

### FIX 03 — final area recommendation

OLD:

```text
如果這些理由都不符合行程，可能根本不必住首爾站。明洞、弘大或鐘路，或許能讓首爾住宿更合適。
```

NEW:

```text
如果這些理由都不符合行程，可能根本不必住首爾站。住明洞、弘大或鐘路，或許會更符合整段首爾行程。
```

Reason:
- Naturalizes the final area-selection conclusion.
- Recommendation direction unchanged.

### Page 29 protection

- All hotel / KTX / AREX / station-side / exit / room-layout facts: unchanged
- H1 1 / H2 10 / H3 18 — EN ↔ zh-TW parity
- FAQ: 6 visible / 6 FAQPage JSON-LD, exact parity 6 / 6
- External href parity: 24 / 24
- Affiliate-marked link parity: 24 / 24
- Internal zh-TW routing defect: 0

---

# 8. Page 30 — `zh-tw/incheon-airport-private-transfer.html`

## Result: FIX 2

### FIX 01 — value of paying extra

OLD:

```text
真正的問題不只是私人車輛比較貴，而是長途飛行後，額外花費能否值得省去行李、轉乘與最後一段路的麻煩。
```

NEW:

```text
真正的問題不只是私人車輛比較貴，而是長途飛行後，多花這筆錢是否值得用來省去搬行李、轉乘與最後一段路的麻煩。
```

Reason:
- `額外花費能否值得` is grammatically unnatural.
- Door-to-door value judgment unchanged.

### FIX 02 — final recommendation for solo / couple travelers

OLD:

```text
獨自或兩人同行、行李好處理，且抵達時間仍搭得到火車或機場巴士時，通常較難說明為何需要多花錢預訂私人車輛。
```

NEW:

```text
獨自或兩人同行、行李好處理，且抵達時間仍搭得到火車或機場巴士時，通常沒有太大理由多花錢預訂私人車輛。
```

Reason:
- Natural Taiwan phrasing for `hard to justify`.
- Recommendation strength remains negative / conditional.

### Page 30 protection

- Legal / vehicle / luggage / terminal / waiting-policy facts: unchanged
- Platform ordering: unchanged
- H1 1 / H2 11 / H3 10 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD, exact parity 8 / 8
- External href parity: 4 / 4
- Affiliate-marked links: 0 / 0
- Internal zh-TW routing defect: 0

---

# 9. Page 31 — `zh-tw/index.html`

## Result: FIX 1

### FIX 01 — first-hour anchor text

Context:
- H2: `抵達韓國後的第一個小時`
- The OLD string is the **visible anchor text only**. Preserve the following colon and the rest of the paragraph.

OLD:

```text
領完行李後，多數旅客需要的其實是同幾件事
```

NEW:

```text
領完行李後，多數旅客需要的其實就是幾件事
```

Reason:
- `同幾件事` is an unnatural rendering of `the same few things`.
- Link destination and surrounding paragraph remain unchanged.

### Page 31 protection

- Homepage intent / hierarchy: unchanged
- H1 1 / H2 3 / H3 5 — EN ↔ zh-TW parity
- FAQPage JSON-LD: 0
- External href parity: 1 / 1
- Affiliate-marked links: 0 / 0
- self canonical remains `https://www.getkoreainside.com/zh-tw/`
- Internal zh-TW routing defect: 0

---

# 10. Page 32 — `zh-tw/insadong-travel-guide.html`

## Result: FIX 2

### FIX 01 — Anguk starting-point recommendation

OLD:

```text
對許多第一次來的旅客，安國站是最方便的預設選擇；但更適合的起點，還是要看當天其他行程在哪裡。
```

NEW:

```text
對許多第一次來的旅客，安國站是最容易安排的起點；但更適合的起點，還是要看當天其他行程在哪裡。
```

Reason:
- Removes literal `default → 預設選擇`.
- Preserves Anguk as the easiest general first-visit starting point.

### FIX 02 — Ikseon-dong relationship

OLD:

```text
別把益善洞當成「更多的仁寺洞」。
```

NEW:

```text
別把益善洞當成仁寺洞的延伸版。
```

Reason:
- `more Insadong → 更多的仁寺洞` is not natural Taiwan Chinese.
- Keeps the source warning that Ikseon-dong should not be treated as simply more of the same neighborhood.

### Page 32 protection

- Facts / routes / time ranges / recommendation hierarchy: unchanged
- H1 1 / H2 14 / H3 74 — EN ↔ zh-TW parity
- FAQPage JSON-LD: 0
- External href parity: 0 / 0
- Internal zh-TW routing defect: 0
- H1 `data-guide-year="current"` marker: preserve exactly

---

# 11. Page 33 — `zh-tw/itaewon-travel-guide.html`

## Result: FIX 3

### FIX 01 — food as a reason to choose Itaewon

OLD:

```text
美食是比起首爾其他街區，選擇梨泰院最有力的理由之一。
```

NEW:

```text
美食是選擇梨泰院、而不是首爾其他街區的主要理由之一。
```

Reason:
- Repairs awkward comparative word order.
- Preserves the source's strong food-led rationale.

### FIX 02 — Gyeongnidan middle-ground sentence

Context:
- Section: `想安靜一點的夜晚，就去經理團`

OLD:

```text
因此，它是一個實用的折衷。
```

NEW:

```text
因此，這裡是兩種夜晚之間很實用的折衷選擇。
```

Reason:
- Naturalizes `a useful middle ground`.
- Preserves Gyeongnidan's role between a quiet evening and the main nightlife scene.

### FIX 03 — stay-vs-visit recommendation

OLD:

```text
如果好幾個晚上都會以這個街區為主、很在意晚歸，或漢南／梨泰院在旅程中反覆出現，住宿才更有意義。
```

NEW:

```text
如果好幾個晚上都會以這個街區為主、很在意晚歸，或旅程中會反覆到漢南／梨泰院，住在這裡才更合理。
```

Reason:
- Naturalizes the stay recommendation without changing its conditions.

### Page 33 protection

- Religious / halal / route / hill / nightlife judgments: unchanged
- H1 1 / H2 22 / H3 54 — EN ↔ zh-TW parity
- FAQPage JSON-LD: 0
- External href parity: 3 / 3
- Affiliate-marked link parity: 3 / 3
- Internal zh-TW routing defect: 0
- H1 `data-guide-year="current"` marker: preserve exactly

---

# 12. Page 34 — `zh-tw/jamsil-travel-guide.html`

## Result: FIX 4

### FIX 01 — Aquarium indoor-day comparison

OLD:

```text
雨、熱或冷讓你想改在室內活動時，比起承諾一整天主題樂園，水族館更容易配合蠶室其餘行程。
```

NEW:

```text
下雨、太熱或太冷，想改在室內活動時，水族館比排一整天主題樂園更容易和蠶室其他行程搭配。
```

Reason:
- `承諾一整天主題樂園` is a direct calque from `committing to a full theme-park day`.
- Indoor-option judgment unchanged.

### FIX 02 — Bangi / Songridan mood sentence

OLD:

```text
想想想要的氛圍。
```

NEW:

```text
想想你想要的氛圍。
```

Reason:
- Clear duplicated-word typo.

### FIX 03 — first-trip stay FAQ opening

Context:
- FAQ question: `第一次來首爾，應該住蠶室嗎？`
- This answer is mirrored in FAQPage JSON-LD.
- **Update visible FAQ + JSON-LD identically.**

OLD:

```text
通常不會直接把它當預設選擇。
```

NEW:

```text
通常不會把蠶室當成第一次首爾旅行的住宿首選。
```

Reason:
- Replaces the literal `not by default` wording with a clear Taiwan-natural answer.
- Recommendation remains explicitly non-default.

### FIX 04 — conditional Jamsil stay recommendation

Context:
- Same FAQ answer as FIX 03.
- **Update visible FAQ + JSON-LD identically.**

OLD:

```text
Lotte World、兒童活動、演唱會、運動賽事或首爾東部是旅程重點時，蠶室才是有力的住宿選項。
```

NEW:

```text
Lotte World、兒童活動、演唱會、運動賽事或首爾東部是旅程重點時，蠶室才會成為很合適的住宿據點。
```

Reason:
- Removes literal `strong base → 有力的住宿選項`.
- Preserves the strong conditional recommendation.

### Page 34 protection

- Lotte World / Aquarium / Seoul Sky / KSPO Dome / route / time / weather judgments: unchanged
- H1 1 / H2 23 / H3 37 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- Current zh-TW FAQ parity: 8 / 8
- After FIX 03–04, visible FAQ and JSON-LD must remain exact mirrors
- External href parity: 4 / 4
- Affiliate-marked link parity: 4 / 4
- Internal zh-TW routing defect: 0
- H1 `data-guide-year="current"` marker: preserve exactly

---

# 13. Batch 01G final audit checks

## Content

- Major factual mismatch: **0**
- Material numeric mismatch: **0**
- Material recommendation drift: **0**
- Large omission: **0**
- Large invention: **0**
- English page-role distortion: **0**

## Taiwan localization

- Exact correction items: **27**
- Clear grammar / typo / calque / Taiwan-naturalness fixes: **27**
- Recommendation strength changed by correction: **0**
- Known Taiwan terminology corrections:
  - `英語` → `英文` in two relevant service/language contexts
- Hongdae photo-booth terminology:
  - existing explanatory `四格拍貼機` / concise `拍貼機` pattern preserved
- Blocking generic `酒店` residue in these ten pages: **0**
- Blocking `性價比` residue in these ten pages: **0**

## Structure / schema

| Page | H1 | H2 | H3 | Visible FAQ | FAQPage |
|---|---:|---:|---:|---:|---:|
| Gongdeok / Mapo | 1 | 19 | 56 | 0 | 0 |
| Hongdae | 1 | 16 | 67 | 0 | 0 |
| Hongdae vs Myeongdong | 1 | 13 | 26 | 8 | 8 |
| Hotels near Gongdeok | 1 | 12 | 18 | 5 | 5 |
| Hotels near Seoul Station | 1 | 10 | 18 | 6 | 6 |
| Incheon Private Transfer | 1 | 11 | 10 | 8 | 8 |
| Index | 1 | 3 | 5 | 0 | 0 |
| Insadong | 1 | 14 | 74 | 0 | 0 |
| Itaewon | 1 | 22 | 54 | 0 | 0 |
| Jamsil | 1 | 23 | 37 | 8 | 8 |

EN ↔ zh-TW structural mismatch: **0**

Current zh-TW FAQ parity before implementation:
- Hongdae vs Myeongdong: **8/8**
- Hotels near Gongdeok: **5/5**
- Hotels near Seoul Station: **6/6**
- Incheon Private Transfer: **8/8**
- Jamsil: **8/8**

JSON-LD parse error: **0**

## Link / affiliate protection

All ten pages:
- `lang="zh-TW"`: correct
- self canonical: correct
- hreflang set present: `en / es / ja / zh-TW / x-default`
- non-zh-TW main-content routing defect detected: **0**

External href parity EN ↔ zh-TW:

- Gongdeok / Mapo: **5 / 5**
- Hongdae: **6 / 6**
- Hongdae vs Myeongdong: **30 / 30**
- Hotels near Gongdeok: **20 / 20**
- Hotels near Seoul Station: **24 / 24**
- Incheon Private Transfer: **4 / 4**
- Index: **1 / 1**
- Insadong: **0 / 0**
- Itaewon: **3 / 3**
- Jamsil: **4 / 4**

Affiliate-marked link parity:

- Gongdeok / Mapo: **5 / 5**
- Hongdae: **5 / 5**
- Hongdae vs Myeongdong: **30 / 30**
- Hotels near Gongdeok: **20 / 20**
- Hotels near Seoul Station: **24 / 24**
- Incheon Private Transfer: **0 / 0**
- Index: **0 / 0**
- Insadong: **0 / 0**
- Itaewon: **3 / 3**
- Jamsil: **4 / 4**

HTML implementation performed in this audit: **0**  
stage / commit / push / Production performed: **0**

---

# 14. Exact implementation rule for later Codex work

When implementation is eventually requested:

1. Use **this file as the exact correction Source of Truth for Batch 01G**.
2. Apply only the **27 correction items** above.
3. Do not rewrite any other zh-TW wording.
4. Do not change facts, numbers, dates, route order, hotel-selection logic, recommendation order or recommendation strength.
5. Do not independently refresh current events, hotel conditions, ticket rules, transport schedules or affiliate conditions during this language-only implementation.
6. Do not change classes, IDs, `data-*`, images, CSS, JS logic, affiliate URLs, tracking, canonical or hreflang unless separately approved.
7. Preserve all `data-guide-year="current"` markers exactly.
8. `hongdae-vs-myeongdong.html` FIX 02:
   - visible FAQ + FAQPage JSON-LD must use exactly the same NEW wording.
9. `jamsil-travel-guide.html` FIX 03–04:
   - visible FAQ + FAQPage JSON-LD must use exactly the same NEW wording.
10. `index.html` FIX 01:
   - replace only the anchor's visible text.
   - preserve its existing href and the colon immediately after the closing `</a>`.
11. Preserve Hongdae photo-booth terminology pattern; do not bulk-replace.
12. If any OLD string is missing or occurs in an unexpected location, STOP instead of guessing.
13. After exact implementation, perform only mechanical QA. Do **not** repeat the localization audit.

---

# 15. Batch state

**Batch 01G = REVIEW COPY — AWAITING USER APPROVAL**

After user approval:
- wording becomes **APPROVED PUBLIC COPY — CONTENT LOCKED**
- later Codex implementation must be exact
- no second editorial localization audit
