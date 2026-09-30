# Korea Inside — zh-TW Localization Audit + Correction — Batch 01F

- File: `Korea_Inside_ZH-TW_Localization_Audit_Correction_Batch01F_4pages_2026-09-30.md`
- Date: 2026-09-30
- Status: **REVIEW COPY — AWAITING USER APPROVAL**
- Scope: Current English Production ↔ current zh-TW Production direct comparison
- Work unit: **4 pages**
- Rule: **This correction MD is the final localization audit for these four pages. After exact implementation, do not run another editorial localization audit unless a concrete defect appears.**

---

## 1. Batch scope

1. `zh-tw/dongdaemun-travel-guide.html`
2. `zh-tw/esim.html`
3. `zh-tw/foreign-credit-cards-korea.html`
4. `zh-tw/gangnam-travel-guide.html`

Production basis checked directly:

- English and zh-TW public pages: HTTP 200
- Current English Production used as the factual / numeric / recommendation basis
- Current zh-TW wording judged directly for Taiwan Traditional Chinese naturalness
- No page after these four was inspected in this Batch
- This is a localization audit, not a new factual-research pass. Existing current English Production facts and judgments remain the source basis.

---

# 2. Final result

| Page | Result | Corrections |
|---|---|---:|
| `dongdaemun-travel-guide.html` | FIX | 3 |
| `esim.html` | FIX | 7 |
| `foreign-credit-cards-korea.html` | FIX | 5 |
| `gangnam-travel-guide.html` | FIX | 8 |
| **TOTAL** |  | **23** |

The corrections below are the exact OLD → NEW strings.

Do not paraphrase them during implementation.

---

# 3. Page 1 — `zh-tw/dongdaemun-travel-guide.html`

## Result: FIX 3

### FIX 01 — Dynamic Decision Map route-line explanation

This exact wording currently appears in **two user-facing locations**:
1. the static map guide
2. the dynamic map status string inside inline JavaScript

Apply the same NEW wording to both locations.

OLD:

```text
路線連線表示行程順序，並非逐向導航。
```

NEW:

```text
地圖上的路線線條只表示行程規劃順序，不是逐步導航。
```

Reason:
- `逐向導航` is an unnatural calque for `turn-by-turn navigation`.
- Keeps the important limitation that the map line is planning sequence, not navigation.

### FIX 02 — guided Dongdaemun option

Context:
- H3: `東大門導覽選項`

OLD:

```text
目前合作平台提供東大門步行與市場導覽商品。
```

NEW:

```text
目前合作平台上有東大門步行與市場導覽行程可預訂。
```

Reason:
- `導覽商品` is overly commerce-literal in Taiwan travel copy.
- Preserves the current partner-inventory meaning.

### FIX 03 — jjimjilbang option

Context:
- H3: `深夜休息選項`

OLD:

```text
目前合作平台有東大門汗蒸幕商品，可選白天或夜間入場。
```

NEW:

```text
目前合作平台上有東大門汗蒸幕方案可預訂，可選白天或夜間入場。
```

Reason:
- Naturalizes `product` as a bookable travel option.
- No affiliate or availability claim is added.

### Page 1 protection

- Major factual mismatch: 0
- Numeric mismatch: 0
- Material recommendation drift: 0
- Large omission / invention: 0
- Structure: H1 1 / H2 23 / H3 62 — EN ↔ zh-TW parity
- FAQPage schema: 0 — unchanged
- Dynamic-map script count / quoted-string structure: aligned EN ↔ zh-TW
- Blocking English user-facing residue in the localized Dongdaemun map strings: not detected
- FIX 01 must remain identical in the static map guide and dynamic JS status message
- External-link parity against English: 2 / 2
- Affiliate-marked-link parity: 2 / 2
- `lang="zh-TW"`: correct
- self canonical: correct
- hreflang: `en / es / ja / zh-TW / x-default`
- main-content Taiwan routing defect detected: 0
- Existing raw `**...**` markers in the final four-question paragraph are inherited from current English Production and are **not reopened in this language-only Batch**

---

# 4. Page 2 — `zh-tw/esim.html`

## Result: FIX 7

### FIX 01 — Korean-number eSIM paragraph

Context:
- H3: `韓國門號 eSIM`

OLD:

```text
旅途中當地通話、SMS 或韓國聯絡號碼確實有用時，韓國門號 eSIM 更有道理。部分司機、預約、飯店或其他服務，需要韓國當地人直接聯絡你，就可能重要。
```

NEW:

```text
旅途中若確實需要當地通話、SMS 或韓國聯絡號碼，韓國門號 eSIM 會更適合。遇到司機、飯店、預約服務或其他韓國當地聯絡人需要直接找到你時，這類門號就很有用。
```

Reason:
- `更有道理` is a direct English calque.
- The second sentence currently attaches the need for contact awkwardly to `預約`.
- Same conditional recommendation is preserved.

### FIX 02 — physical SIM paragraph

Context:
- H3: `實體 SIM`

OLD:

```text
手機不支援 eSIM，或希望有人當面幫忙設定，實體 SIM 仍然合理。機場或門市領取可能比線上 eSIM 慢，但想親眼確認連上網再離開，人員支援可能值得多這一步。
```

NEW:

```text
手機不支援 eSIM，或希望有人當面幫忙設定時，實體 SIM 仍然合理。機場或門市領取可能比線上 eSIM 慢，但如果想在離開前確認已經連上網，人員協助可能值得多花這一步。
```

Reason:
- Repairs compressed translated syntax.
- Keeps the trade-off between slower pickup and staff assistance.

### FIX 03 — international roaming paragraph

Context:
- H3: `國際漫遊`

OLD:

```text
保留慣用號碼、避免再設另一張 SIM，比價格更重要時，漫遊最不複雜。手機繼續使用原電信商，極短旅程，或單純不想管理另一條門號的人，會很方便。
```

NEW:

```text
如果比起價格，你更重視保留慣用號碼，也不想另外設定一張 SIM，國際漫遊是最省事的選項。手機繼續使用原電信商，對極短程旅行，或單純不想管理另一條門號的人來說很方便。
```

Reason:
- `漫遊最不複雜` is unnatural Taiwan wording.
- Price-versus-convenience judgment unchanged.

### FIX 04 — sharing one connection

Context:
- H2: `同行每個人都需要一張 eSIM 嗎？`

OLD:

```text
不一定。兩人整天都在一起，且方案允許熱點，有時可以一支手機分享給另一支。短程旅行、很少分開，這樣可能就夠。
```

NEW:

```text
不一定。兩人整天都在一起，且方案允許熱點時，有時一支手機就能分享網路給另一支。短程旅行、很少分開，這樣可能就夠。
```

Reason:
- Restores the missing object `網路`.
- Meaning unchanged.

### FIX 05 — plan-selection paragraph

Context:
- H2: `準備比較實際 eSIM 方案了嗎？`

OLD:

```text
純上網 eSIM 若適合旅程，下一步是選方案，而不是技術。比較高速數據量、效期、能否開熱點、何時開始啟用，以及安裝失敗時怎麼處理。
```

NEW:

```text
純上網 eSIM 若適合旅程，下一步要比較的是方案，而不是 eSIM 技術本身。比較高速數據量、效期、能否開熱點、何時開始啟用，以及安裝失敗時怎麼處理。
```

Reason:
- Naturalizes `the next decision is the plan rather than the technology`.
- Page decision logic unchanged.

### FIX 06 — Korean local calls referent

Context:
- H3: `韓國 eSIM 與 SIM 方案`
- Change **only this exact visible phrase**; preserve the existing internal link and all carrier names.

OLD:

```text
國內通話
```

NEW:

```text
韓國當地通話
```

Reason:
- In English, `domestic calls` means calls within Korea.
- For a Taiwan traveler, `國內通話` can be read as Taiwan domestic calls.
- This is a referent clarification, not a factual change.

### FIX 07 — final setup recommendation

Context:
- H2: `韓國行動通訊該選哪種配置？`

OLD:

```text
無論買什麼，設定和方案同樣重要。了解效期何時開始、掌握原門號使用、離開機場前測試連線，也別把刪 eSIM 當第一個排錯步驟。起飛前稍作準備，比拖行李在入境大廳處理數據問題容易得多。
```

NEW:

```text
無論買什麼，設定和方案同樣重要。先弄清楚效期何時開始、確認原門號的使用設定，並在離開機場前測試連線；也別把刪除 eSIM 設定檔當成第一個排錯步驟。起飛前稍作準備，比拖著行李在入境大廳處理數據問題容易得多。
```

Reason:
- `掌握原門號使用` and `刪 eSIM` are compressed calques.
- Troubleshooting order and recommendation remain unchanged.

### Page 2 protection

- Major factual mismatch: 0
- Numeric mismatch: 0
- Material recommendation drift: 0
- Large omission / invention: 0
- Carrier / eSIM capability hierarchy: unchanged
- Structure: H1 1 / H2 17 / H3 14 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- Visible FAQ ↔ JSON-LD exact parity: 8/8
- No FAQ correction in this Batch
- `lang="zh-TW"`: correct
- self canonical: correct
- hreflang: `en / es / ja / zh-TW / x-default`
- main-content Taiwan routing defect detected: 0
- Affiliate / external href parity against English: unchanged

---

# 5. Page 3 — `zh-tw/foreign-credit-cards-korea.html`

## Result: FIX 5

### FIX 01 — broad card-acceptance paragraph

Context:
- H2: `你的外國卡在韓國能用嗎？`

OLD:

```text
通常可以。旅客常去的許多商家，尤其是飯店、百貨、連鎖店與成熟店家，刷卡是日常付款方式。但廣泛接受，不代表每張海外卡都能通用。商家設備、卡組織、發卡機構與交易驗證方式，都會影響結果。
```

NEW:

```text
通常可以。旅客常去的許多地方，尤其是飯店、百貨、連鎖店與一般店家，刷卡都是日常付款方式。但廣泛接受，不代表每張海外卡都能通用。商家設備、卡組織、發卡機構與交易驗證方式，都會影響結果。
```

Reason:
- `成熟店家` is an unnatural calque for `established shops`.
- Acceptance judgment is unchanged.

### FIX 02 — restaurant / café table cell

Context:
- Table row: `餐廳與咖啡廳`
- Column: `通常情況`

OLD:

```text
許多成熟店家的刷卡機可處理海外卡。
```

NEW:

```text
許多一般餐廳與咖啡廳的刷卡機都能處理海外卡。
```

Reason:
- Removes the same unnatural `成熟店家` wording.
- Stays within the English row context.

### FIX 03 — Korean online checkout paragraph

Context:
- H3: `韓國網路結帳是另一種問題`

OLD:

```text
海外卡即使現場能用，到了要求韓國電話、本地身分驗證或韓國本地付款流程的網站，仍可能卡住。全球版服務可能有不同結帳流程，但不是每個商家都提供。
```

NEW:

```text
海外卡即使現場能用，到了要求韓國電話、韓國本地身分驗證或韓國本地付款流程的網站，仍可能卡住。全球版服務可能有不同結帳流程，但不是每個商家都提供。
```

Reason:
- Makes the `local` referent explicit for a Taiwan reader.
- Does not change the English claim.

### FIX 04 — online-checkout FAQ answer

Context:
- FAQ question: `為什麼外國卡在店裡能用，網路上卻刷不過？`
- **Visible FAQ + FAQPage JSON-LD must be updated identically.**

OLD:

```text
現場刷卡機與韓國網站，可能使用不同付款及身分查核。即使同一張卡在商店能用，部分本地網路結帳仍要求韓國電話、本地身分驗證或韓國本地驗證流程。
```

NEW:

```text
現場刷卡機與韓國網站，可能使用不同付款及身分查核。即使同一張卡在商店能用，部分韓國本地網路結帳仍要求韓國電話、韓國本地身分驗證或韓國本地驗證流程。
```

Reason:
- Clarifies every `local` referent as Korean-local.
- Visible/schema meaning remains identical.

### FIX 05 — Visa support reference sentence

Context:
- H2: `官方參考資料`

OLD:

```text
Visa 消費者支援 指引持卡人向發卡機構詢問拒絕交易原因與旅遊通知要求，這些因卡公司而異。
```

NEW:

```text
Visa 消費者支援 指引持卡人向發卡機構詢問拒絕交易原因與旅遊通知要求，因為這些要求會依發卡機構而異。
```

Reason:
- Naturalizes the final clause.
- No source or factual claim changes.

### Page 3 protection

- Major factual mismatch: 0
- Numeric mismatch: 0
- Material recommendation / payment guidance drift: 0
- Large omission / invention: 0
- Structure: H1 1 / H2 9 / H3 6 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- Current visible FAQ ↔ JSON-LD exact parity: 8/8
- After FIX 04, visible FAQ and JSON-LD must remain exact mirrors
- External official-reference href parity against English: 7 / 7
- `lang="zh-TW"`: correct
- self canonical: correct
- hreflang: `en / es / ja / zh-TW / x-default`
- main-content Taiwan routing defect detected: 0

---

# 6. Page 4 — `zh-tw/gangnam-travel-guide.html`

## Result: FIX 8

### FIX 01 — Gangnam optional wording

The same sentence is used in:
- the early decision section
- the visible FAQ answer
- the FAQPage JSON-LD answer

Apply the same NEW wording to all three locations.

OLD:

```text
江南可以不排。
```

NEW:

```text
江南可以不排進行程。
```

Reason:
- Current wording is understandable but incomplete in Taiwan Chinese.
- Preserves the English judgment `Gangnam is optional.`

### FIX 02 — Route 1 explanation

Context:
- H2: `路線 1 — 初訪江南`

OLD:

```text
這條路線走得通，因為每一站都改變當天的體驗，不是重複同類景點。
```

NEW:

```text
這條路線之所以走得順，是因為每一站都會改變當天的體驗，而不是重複同類景點。
```

Reason:
- Naturalizes `This route works because...`.
- Route judgment unchanged.

### FIX 03 — COEX lunch paragraph

OLD:

```text
有收藏的餐廳可以去；沒有的話，就找方便的地方吃，讓路線繼續前進。
```

NEW:

```text
如果已經存好想去的餐廳，就照計畫去；沒有的話，找方便的地方吃即可，讓行程順著走下去。
```

Reason:
- `有收藏的餐廳可以去` is a literal UI-style calque.
- Preserves the advice not to turn lunch into an unnecessary detour.

### FIX 04 — Gangnam Station evening paragraph

OLD:

```text
一間咖啡廳、一場包廂唱歌，或再走走小街，可能就夠了。大家都累了，不必勉強待到深夜。
```

NEW:

```text
去一間咖啡廳、唱個歌，或再走走小街，可能就夠了。大家都累了，不必勉強待到深夜。
```

Reason:
- `一場包廂唱歌` is not natural Taiwan wording.
- Evening recommendation unchanged.

### FIX 05 — K-Beauty service category

Context:
- H2: `韓系美妝與固定預約`
- List item only

OLD:

```text
預約制造型或美妝服務
```

NEW:

```text
預約制的造型或美妝服務
```

Reason:
- Repairs a grammatical omission.

### FIX 06 — fast-changing Apgujeong / Dosan retail

Context:
- H3: `狎鷗亭／島山快閃店與旗艦店`

OLD:

```text
短期零售與品牌聯名活動，來去很快。
```

NEW:

```text
快閃店與品牌聯名活動變動很快。
```

Reason:
- `短期零售` is an English-derived phrase for temporary retail / pop-ups.
- Preserves the time-sensitive nature of the content.

### FIX 07 — stay-or-visit recommendation

Context:
- H2: `該住江南，還是只來逛？`

OLD:

```text
行程多在首爾南部與東南部時，江南可以是很好的據點；但不是第一次來的自動預設選擇。
```

NEW:

```text
行程多在首爾南部與東南部時，江南可以是很好的據點；但第一次來首爾時，並不是理所當然的住宿首選。
```

Reason:
- `自動預設選擇` is a direct calque.
- Preserves the strong non-default judgment.

### FIX 08 — stay-in-Gangnam condition list

Context:
- H3: `以下幾項符合時，可以住江南`

OLD:

```text
旅程結合江南與多個東側地點，而不是主要逛歷史悠久的首爾市中心
```

NEW:

```text
旅程會串連江南與多個東側地點，而不是主要安排首爾歷史市中心的景點
```

Reason:
- Current wording is grammatically stiff and attaches `歷史悠久` to the city center unnaturally.
- Same hotel-fit condition is preserved.

### Page 4 protection

- Major factual mismatch: 0
- Numeric mismatch: 0
- Material recommendation drift: 0
- Large omission / invention: 0
- Route order: unchanged
- Attraction / station / area names: unchanged
- Affiliate CTA order and URLs: unchanged
- Structure: H1 1 / H2 17 / H3 35 — EN ↔ zh-TW parity
- FAQ: 6 visible / 6 FAQPage JSON-LD
- Current zh-TW visible FAQ ↔ JSON-LD exact parity: 6/6
- FIX 01 must remain synchronized across early body copy + visible FAQ + FAQPage JSON-LD
- External / affiliate href parity against English: 3 / 3
- Affiliate-marked-link parity: 3 / 3
- `lang="zh-TW"`: correct
- self canonical: correct
- hreflang: `en / es / ja / zh-TW / x-default`
- main-content Taiwan routing defect detected: 0

---

# 7. Batch 01F final audit checks

## Content

- Major factual mismatch: **0**
- Numeric mismatch: **0**
- Material recommendation / decision-order drift: **0**
- Large omission: **0**
- Large invention: **0**
- English page-role distortion: **0**

## Taiwan localization

- Exact correction items: **23**
- Clear grammar / calque / Taiwan-naturalness fixes: **20**
- Local-referent clarification items: **3**
- Recommendation softening / strengthening caused by localization: **0**
- Blocking Simplified-Chinese residue detected: **0**

## Structure / schema

- `dongdaemun-travel-guide.html`
  - H1 1 / H2 23 / H3 62 / FAQPage 0
- `esim.html`
  - H1 1 / H2 17 / H3 14 / visible FAQ 8 / FAQPage 8
- `foreign-credit-cards-korea.html`
  - H1 1 / H2 9 / H3 6 / visible FAQ 8 / FAQPage 8
- `gangnam-travel-guide.html`
  - H1 1 / H2 17 / H3 35 / visible FAQ 6 / FAQPage 6

EN ↔ zh-TW main structure mismatch: **0**

Current zh-TW visible FAQ ↔ FAQPage JSON-LD exact parity:
- eSIM: **8/8**
- Foreign Credit Cards: **8/8**
- Gangnam: **6/6**
- Dongdaemun: FAQPage intentionally **0**

JSON-LD parse error: **0**

## Link / technical protection

- `lang="zh-TW"`: **4/4**
- self canonical: **4/4**
- hreflang set present (`en / es / ja / zh-TW / x-default`): **4/4**
- non-zh-TW main-content internal routing defect detected: **0**
- Dongdaemun external href parity: **2/2**
- Dongdaemun affiliate-marked link parity: **2/2**
- Foreign Credit Cards external href parity: **7/7**
- Gangnam external href parity: **3/3**
- Gangnam affiliate-marked link parity: **3/3**
- HTML implementation performed in this audit: **0**
- stage / commit / push / Production performed: **0**

---

# 8. Exact implementation rule for later Codex work

When implementation is eventually requested:

1. Use **this file as the exact correction Source of Truth for Batch 01F**.
2. Apply only the 23 correction items above.
3. Do not rewrite any other zh-TW wording.
4. Do not change facts, numbers, route order, recommendation order or recommendation strength.
5. Do not independently refresh current hours, events, provider conditions or payment policies during this language-only correction implementation.
6. Do not change classes, IDs, `data-*`, map logic, images, CSS, shared JS, affiliate URLs, tracking, canonical or hreflang unless a separate approved technical instruction explicitly requires it.
7. Dongdaemun FIX 01: update both the static map guide and the matching inline-JS user-facing status string.
8. Foreign Credit Cards FIX 04: update visible FAQ + FAQPage JSON-LD with exactly the same NEW wording.
9. Gangnam FIX 01: update all three occurrences — early body decision, visible FAQ, FAQPage JSON-LD.
10. Preserve all current-year automation markers exactly.
11. If any OLD string is missing or appears in an unexpected location, STOP instead of guessing.
12. After exact implementation, perform only mechanical QA. Do **not** repeat the localization audit.

---

# 9. Batch state

**Batch 01F = REVIEW COPY — AWAITING USER APPROVAL**

After user approval:
- wording becomes **APPROVED PUBLIC COPY — CONTENT LOCKED**
- later Codex implementation must be exact
- no second editorial localization audit
