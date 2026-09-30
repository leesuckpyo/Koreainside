# Korea Inside — zh-TW Localization Audit + Correction — Batch 01E

- File: `Korea_Inside_ZH-TW_Localization_Audit_Correction_Batch01E_4pages_2026-09-30.md`
- Date: 2026-09-30
- Status: **REVIEW COPY — AWAITING USER APPROVAL**
- Scope: Current English Production ↔ current zh-TW Production direct comparison
- Work unit: **4 pages**
- Rule: **This correction MD is the final localization audit for these four pages. After exact implementation, do not run another editorial localization audit unless a concrete defect appears.**

---

## 1. Batch scope

1. `zh-tw/best-area-for-solo-travelers-seoul.html`
2. `zh-tw/best-esim-for-korea.html`
3. `zh-tw/card-declined-korea.html`
4. `zh-tw/checklist.html`

Production basis checked directly:

- English and zh-TW public pages: HTTP 200
- Current English Production used as the factual / numeric / recommendation basis
- Current zh-TW wording judged directly for Taiwan Traditional Chinese naturalness
- No page after these four was inspected in this Batch
- Time-sensitive eSIM provider conditions were **not re-researched or changed** in this localization audit; the current English Production remains the locked factual basis for this task

---

# 2. Final result

| Page | Result | Corrections |
|---|---|---:|
| `best-area-for-solo-travelers-seoul.html` | FIX | 6 |
| `best-esim-for-korea.html` | FIX | 13 |
| `card-declined-korea.html` | FIX | 2 |
| `checklist.html` | FIX | 3 |
| **TOTAL** |  | **24** |

The corrections below are the exact OLD → NEW strings.

Do not paraphrase them during implementation.

---

# 3. Page 1 — `zh-tw/best-area-for-solo-travelers-seoul.html`

## Result: FIX 6

### FIX 01 — hero practical-detail paragraph

OLD:

```text
獨自旅行，小細節可能比團體旅行更重要：從地鐵走回飯店、玩了一整天後容易找到餐食，以及疲累或拖行李時仍然好理解的路線。
```

NEW:

```text
獨自旅行時，小細節可能比團體旅行更重要：從地鐵走回飯店的路、玩了一整天後是否容易找到吃的，以及疲累或拖行李時仍容易掌握的路線。
```

Reason:
- `仍然好理解的路線` is a direct-translation expression.
- Preserves the source emphasis on simple return routes, food and luggage friction.

### FIX 02 — quick decision / Hongdae

OLD:

```text
咖啡廳、夜生活與直達 AREX 為主的活躍獨旅行程，弘大更適合。
```

NEW:

```text
如果想把咖啡廳、夜生活與直達 AREX 都放進較活躍的獨旅行程，弘大更適合。
```

Reason:
- Repairs unnatural noun stacking.
- Recommendation strength remains unchanged.

### FIX 03 — Seoul Station body paragraph

OLD:

```text
AREX、鐵路與幾條地鐵連接，能簡化那些無人幫忙提行李時最容易覺得累的移動。
```

NEW:

```text
AREX、鐵路與多條地鐵線，能簡化那些沒有人幫忙提行李時最容易覺得累的移動。
```

Reason:
- `幾條地鐵連接` is not natural Taiwan Chinese.
- Preserves the transport judgment.

### FIX 04 — comparison table / Seoul Station trade-off

OLD:

```text
街區氣氛較少
```

NEW:

```text
街區生活感較弱
```

Reason:
- Naturalizes `Less neighborhood atmosphere`.

### FIX 05 — cheapest-price mistake

Context:
- H3: `只選最便宜的價格`

OLD:

```text
低房價若增加長距離車站步行、反覆轉乘或昂貴深夜計程車，就可能失去吸引力。獨旅選稍微方便的位置，值得多付一點也不一定。
```

NEW:

```text
低房價若換來很長的車站步行、反覆轉乘或昂貴的深夜計程車，就可能失去吸引力。獨旅時，稍微方便一點的位置有時值得多付一些。
```

Reason:
- Current second sentence is grammatically broken.
- Restores the English total-convenience judgment.

### FIX 06 — luggage FAQ answer

Context:
- FAQ question: `帶行李獨旅，哪一區最好？`
- **Visible FAQ + FAQPage JSON-LD must be updated identically.**

OLD:

```text
首爾站與孔德的機場及鐵路連接直接，帶大行李尤其實用。弘大飯店若從弘大入口站出來路線簡單，也很合適。
```

NEW:

```text
首爾站與孔德的機場和鐵路交通都很直接，帶大行李尤其實用。弘大飯店若從弘大入口站出來的路線簡單，也很合適。
```

Reason:
- Fixes compressed translation syntax.
- Recommendation and transport meaning unchanged.

### Page 1 protection

- Major factual mismatch: 0
- Numeric mismatch: 0
- Material recommendation drift: 0
- Large omission / invention: 0
- Structure: H1 1 / H2 8 / H3 19 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- Current visible FAQ ↔ JSON-LD exact parity: 8/8
- After FIX 06, visible FAQ and JSON-LD must remain exact mirrors
- `lang="zh-TW"`: correct
- self canonical: correct
- hreflang: `en / es / ja / zh-TW / x-default`
- main-content Taiwan routing defect detected: 0

---

# 4. Page 2 — `zh-tw/best-esim-for-korea.html`

## Result: FIX 13

### FIX 01 — hero / Korean-number decision

OLD:

```text
影片看得多、整天上傳照片或分享網路給筆電，就先看真正有多少全速數據、超過後會怎樣。韓國門號又是另一種決定，只有當地通話、SMS 或韓國聯絡號碼確實有用才重要。
```

NEW:

```text
如果常看影片、整天上傳照片，或會把網路分享給筆電，先看真正有多少全速數據，以及超過後會怎樣。是否需要韓國門號則是另一個決定，只有當地通話、SMS 或韓國聯絡號碼確實有用時才重要。
```

Reason:
- Removes literal English sentence structure.
- Korean-number recommendation remains conditional exactly as in English.

### FIX 02 — start-with-phone-use paragraph

Context:
- H2: `從實際使用手機的方式開始`

OLD:

```text
一開始，業者名字沒有手機在旅程會做什麼重要。
```

NEW:

```text
一開始，比起業者名稱，更重要的是你在旅途中實際會怎麼用手機。
```

Reason:
- Current sentence is grammatically broken.
- Restores the English decision-first meaning.

### FIX 03 — regional-plan trade-off

OLD:

```text
網路、數據與效期條件適用整個套裝，不只韓國，因此方便的代價，是較少針對單一國家的控制。
```

NEW:

```text
網路、數據與效期條件適用整個套裝，不只韓國，因此雖然方便，針對單一國家調整方案的彈性也比較少。
```

Reason:
- `較少針對單一國家的控制` is a direct calque.
- Preserves the regional-plan trade-off.

### FIX 04 — hotspot flexibility

OLD:

```text
分享能讓手機方案成為筆電或旅伴的網路，但前提是所選產品允許，也沒有不顯眼的獨立共享額度限制。
```

NEW:

```text
網路分享能讓手機方案供筆電或旅伴上網，但前提是所選產品允許，也要確認是否另有共享數據上限。
```

Reason:
- Removes the unnatural `不顯眼的獨立共享額度限制`.
- Preserves the requirement to check a separate tethering/shared-data cap.

### FIX 05 — policy clarity

OLD:

```text
付款前應看得到確切產品的重要限制。效期、降速或復原規則若每個套裝不同，業者層級的籠統宣稱就比較沒用。
```

NEW:

```text
付款前應看得到確切產品的重要限制。效期、降速或復原規則若每個套裝不同，業者整體的一般性說法參考價值就比較低。
```

Reason:
- `業者層級的籠統宣稱` is an English-derived construction.
- Meaning unchanged.

### FIX 06 — Ubigi future-plan wording

OLD:

```text
Smartstart 系統也適合想出發前安裝、又不立即開始方案的旅客：抵達目的地、eSIM 連線後才開始算效期。取捨是各方案條件仍不同；定量、無限與未來韓國產品要分別確認，別假設 7 天無限規則適用所有產品。
```

NEW:

```text
Smartstart 系統也適合想出發前安裝、又不立即開始方案的旅客：抵達目的地、eSIM 連線後才開始算效期。取捨是各方案條件仍不同；定量、無限，以及之後推出的韓國方案都要分別確認，別假設 7 天無限規則適用所有產品。
```

Reason:
- `未來韓國產品` is unnatural.
- Preserves the warning that future / other Korea products must be checked separately.

### FIX 07 — troubleshooting / deletion

OLD:

```text
不要把刪 eSIM 設定檔當成第一個排錯步驟。有些無法單純用同一 QR 碼重裝，先聯絡業者或依復原說明操作。
```

NEW:

```text
不要把刪除 eSIM 設定檔當成第一個排錯步驟。有些 eSIM 無法直接用同一個 QR 碼重新安裝，應先聯絡業者或依照復原說明操作。
```

Reason:
- Restores the missing subject and natural Taiwan wording.
- Recovery warning unchanged.

### FIX 08 — heavy video / hotspot scenario

OLD:

```text
03 大量影片或熱點使用者 大量影片與熱點，讓全速額度與分享規則成為決策核心。較大定量，有時比比預期更早降速的無限方案更可掌握。
```

NEW:

```text
03 大量影片或熱點使用者 大量影片與熱點，讓全速額度與分享規則成為決策核心。較大的定量方案，有時比預期更早降速的無限方案更容易掌握。
```

Reason:
- Removes the duplicated `比比`.
- Restores natural wording without changing the plan judgment.

### FIX 09 — remote-worker scenario

OLD:

```text
04 遠端工作者 遠端工作不只要標示額度大。熱點規則、加購不中斷與備用連線都重要，因為旅遊 eSIM 不應被當成保證可用的辦公室網路。
```

NEW:

```text
04 遠端工作者 遠端工作不只要標示額度大。熱點規則、加購後能否順利銜接，以及備用連線都很重要，因為旅遊 eSIM 不應被當成保證可用的辦公室網路。
```

Reason:
- Naturalizes `top-up continuity`.
- No change to the remote-work warning.

### FIX 10 — deleting installed eSIM

Context:
- H3: `安裝後刪掉 eSIM`

OLD:

```text
同一 QR 碼可能無法再裝，刪除會把暫時連線問題變成補發問題。支援人員確認復原選項時，先關閉門號較安全。
```

NEW:

```text
同一個 QR 碼可能無法再次安裝，刪除設定檔會把暫時的連線問題變成補發問題。支援人員確認復原方式時，先把這張 eSIM 關閉會比較安全。
```

Reason:
- `關閉門號` can imply cancelling a phone number rather than disabling the eSIM line.
- Preserves the source instruction to turn the line off rather than delete it.

### FIX 11 — Korean identity-verification paragraph

Context:
- H3: `期待完整韓國身分驗證`

OLD:

```text
門號能接電話或訊息，不代表能作為韓國居民身分。號碼持有、SMS 接收、App 驗證與身分驗證，仍是不同能力。
```

NEW:

```text
有門號能接電話或訊息，不代表它能用來完成韓國居民身分驗證。持有門號、接收 SMS、App 驗證與身分驗證，仍是不同的能力。
```

Reason:
- Current first sentence is semantically incomplete.
- Preserves the distinction among number ownership, SMS, app verification and resident identity verification.

### FIX 12 — local mobile service

Context:
- H3: `需要當地行動通訊服務`

OLD:

```text
國內通話、SMS 或韓國聯絡號碼，超出一般旅遊上網方案的範圍。韓國電信商選項可能更適合，但號碼持有、SMS 接收、App 驗證與居民身分驗證，仍要分別確認。
```

NEW:

```text
韓國當地通話、SMS 或韓國聯絡號碼，超出一般旅遊上網方案的範圍。韓國電信商選項可能更適合，但號碼持有、SMS 接收、App 驗證與居民身分驗證，仍要分別確認。
```

Reason:
- English `Domestic calls` refers to local Korean calls in this context.
- `國內通話` can be ambiguous for a Taiwan traveler.
- This is a referent clarification, not a fact change.

### FIX 13 — Ubigi provider CTA description

OLD:

```text
可重用設定檔，加上目前查核的 25GB 全速、7 天無限設計，讓預期整趟用量較高的旅客值得比較 Ubigi。
```

NEW:

```text
如果預期整趟旅行的數據用量較高，Ubigi 值得比較；它可重複使用設定檔，而目前查核的 7 天無限方案提供 25GB 全速數據。
```

Reason:
- Repairs compressed translated word order.
- Preserves the current English facts and the conditional Ubigi comparison judgment.

### Page 2 protection

- Provider order: unchanged
- Provider facts / numbers / dates: unchanged
- `25GB / 2Mbps`, `3GB / 24 hours / 1Mbps`, validity periods and activation rules: unchanged
- No independent factual refresh performed in this localization Batch
- Recommendation hierarchy: unchanged
- Structure: H1 1 / H2 17 / H3 41 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- Visible FAQ ↔ JSON-LD exact parity: 8/8
- Affiliate / external href parity against English: **15/15**
- Affiliate-marked link count parity: **6/6**
- `lang="zh-TW"`: correct
- self canonical: correct
- hreflang: `en / es / ja / zh-TW / x-default`
- main-content Taiwan routing defect detected: 0

---

# 5. Page 3 — `zh-tw/card-declined-korea.html`

## Result: FIX 2

### FIX 01 — stores vs Korean websites

Context:
- H2: `先看付款是在哪裡失敗`

OLD:

```text
如果卡片在實體店能用，卻在韓國網站或 App 失敗，卡片本身可能沒有問題。線上結帳可能另有驗證或韓國本地身分驗證要求。
```

NEW:

```text
如果卡片在實體店能用，卻在韓國網站或 App 失敗，卡片本身可能沒有問題。線上結帳可能另有驗證或韓國本地驗證要求。
```

Reason:
- English says `local verification requirements`, not specifically resident / identity verification.
- Removes an unsupported semantic narrowing.

### FIX 02 — online-payment FAQ answer

Context:
- FAQ question: `為什麼我的卡在實體店能刷，韓國網站卻刷不過？`
- **Visible FAQ + FAQPage JSON-LD must be updated identically.**

OLD:

```text
線上結帳可能要求一般實體刷卡不需要的驗證，或韓國本地身分驗證。因此，實體店刷得過，不代表韓國網站也會接受同一筆付款。
```

NEW:

```text
線上結帳可能要求一般實體刷卡不需要的驗證，或韓國本地驗證流程。因此，實體店刷得過，不代表韓國網站也會接受同一筆付款。
```

Reason:
- Same semantic correction as FIX 01.
- Does not claim the local verification is necessarily identity verification.

### Page 3 protection

- Major factual mismatch: 0
- Numeric mismatch: 0
- Recommendation / troubleshooting-order drift: 0
- Large omission / invention: 0
- Minor semantic over-specification: 2 → corrected
- Structure: H1 1 / H2 9 / H3 0 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- Current visible FAQ ↔ JSON-LD exact parity: 8/8
- After FIX 02, visible FAQ and JSON-LD must remain exact mirrors
- External official-reference href parity against English: **7/7**
- `lang="zh-TW"`: correct
- self canonical: correct
- hreflang: `en / es / ja / zh-TW / x-default`
- main-content Taiwan routing defect detected: 0

---

# 6. Page 4 — `zh-tw/checklist.html`

## Result: FIX 3

### FIX 01 — offline-information heading

OLD:

```text
重要資訊也要存到可能失效的 App 以外
```

NEW:

```text
重要資訊不要只放在可能失效的 App 裡
```

Reason:
- Current heading is understandable but strongly English-derived.
- Preserves the same offline-backup instruction.

### FIX 02 — day-before-flight summary

Context:
- H2: `搭機前一天，再走過這份清單`

OLD:

```text
以下七項都準備好，實用的基本事項就有照顧到。
```

NEW:

```text
以下七項都準備好，基本的實用準備就算齊全了。
```

Reason:
- `就有照顧到` is a direct calque from `the practical basics are covered`.
- Meaning unchanged.

### FIX 03 — changing-information paragraph

Context:
- H2: `接近出發時，再確認可能變動的資訊`

OLD:

```text
入境規定、交通服務與公共聯絡資料可能變動。
```

NEW:

```text
入境規定、交通服務與公共服務聯絡資訊都可能變動。
```

Reason:
- `公共聯絡資料` is vague and unnatural in this context.
- Preserves the warning to recheck public-service contact details.

### Page 4 protection

- Emergency numbers: unchanged
- `112 / 119 / 1330 / 1339 / +82-2-1330`: unchanged
- Review / update dates: unchanged
- Major factual mismatch: 0
- Numeric mismatch: 0
- Recommendation / checklist-order drift: 0
- Large omission / invention: 0
- Structure: H1 1 / H2 12 / H3 3 — EN ↔ zh-TW parity
- FAQ: 8 visible / 8 FAQPage JSON-LD
- Visible FAQ ↔ JSON-LD exact parity: 8/8
- External official-reference href parity against English: **1/1**
- `lang="zh-TW"`: correct
- self canonical: correct
- hreflang: `en / es / ja / zh-TW / x-default`
- main-content Taiwan routing defect detected: 0

---

# 7. Batch 01E final audit checks

## Content

- Major factual mismatch: **0**
- Numeric mismatch: **0**
- Material recommendation / decision-order drift: **0**
- Large omission: **0**
- Large invention: **0**
- English page-role distortion: **0**

## Taiwan localization

- Exact corrections: **24**
- Clear grammar / calque / sentence-quality fixes: **21**
- Minor semantic over-specification / referent fixes: **3**
- Simplified-Chinese / obvious Mainland-only terminology residue detected in this Batch: **0**
- Recommendation softening / strengthening caused by localization: **0**

## Structure / schema

- `best-area-for-solo-travelers-seoul.html`
  - H1 1 / H2 8 / H3 19 / visible FAQ 8 / FAQPage 8
- `best-esim-for-korea.html`
  - H1 1 / H2 17 / H3 41 / visible FAQ 8 / FAQPage 8
- `card-declined-korea.html`
  - H1 1 / H2 9 / H3 0 / visible FAQ 8 / FAQPage 8
- `checklist.html`
  - H1 1 / H2 12 / H3 3 / visible FAQ 8 / FAQPage 8

EN ↔ zh-TW structure mismatch: **0**

Visible FAQ ↔ FAQPage JSON-LD exact parity:

- Solo Travelers: **8/8**
- Best eSIM: **8/8**
- Card Declined: **8/8**
- Checklist: **8/8**

JSON-LD parse error: **0**

## Technical protection check

- `lang="zh-TW"`: **4/4**
- self canonical: **4/4**
- hreflang set present (`en / es / ja / zh-TW / x-default`): **4/4**
- non-zh-TW main-content internal routing defect detected: **0**
- Best eSIM external href parity: **15/15**
- Best eSIM affiliate-marked link parity: **6/6**
- Card Declined external href parity: **7/7**
- Checklist external href parity: **1/1**
- HTML implementation performed in this audit: **0**
- stage / commit / push / Production performed: **0**

---

# 8. Exact implementation rule for later Codex work

When implementation is eventually requested:

1. Use **this file as the exact correction Source of Truth for Batch 01E**.
2. Apply only the 24 OLD → NEW corrections above.
3. Do not rewrite any other zh-TW wording.
4. Do not change facts, numbers, provider conditions, recommendation order or recommendation strength.
5. Do not independently update time-sensitive eSIM conditions during this language-only correction implementation.
6. Do not change section order, classes, IDs, `data-*`, images, CSS, JS, affiliate URLs, tracking, canonical or hreflang unless a separate approved technical instruction explicitly requires it.
7. Solo FIX 06 and Card Declined FIX 02: update **visible FAQ + FAQPage JSON-LD with exactly the same NEW wording**.
8. Preserve all other FAQ/schema wording unchanged.
9. Preserve Best eSIM provider and affiliate link order exactly.
10. If any OLD string is missing or appears in an unexpected location, STOP instead of guessing.
11. After exact implementation, perform only mechanical QA. Do **not** repeat the localization audit.

---

# 9. Batch state

**Batch 01E = REVIEW COPY — AWAITING USER APPROVAL**

After user approval:
- wording becomes **APPROVED PUBLIC COPY — CONTENT LOCKED**
- later Codex implementation must be exact
- no second editorial localization audit
