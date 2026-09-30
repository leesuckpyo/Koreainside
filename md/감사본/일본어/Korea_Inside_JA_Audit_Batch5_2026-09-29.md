# Korea Inside — Japanese Direct Audit Batch 5

- File: `Korea_Inside_JA_Audit_Batch5_2026-09-29.md`
- Date: 2026-09-29
- Status: **JA AUDIT 5/5 COMPLETE**
- Scope: Current English source vs Japanese source, direct GPT editorial/source-parity audit
- Pages: 11
- Result: **9 PASS / 2 FIX**
- Main-body structural parity: **11/11 PASS**
- Head target parity: **11/11 PASS**
- Large omitted/invented section: **0**
- Material recommendation distortion: **0**
- Material numeric/factual error: **0**
- JSON-LD parse error: **0**
- Visible FAQ / English schema parity error: **0**
- Raw Markdown residue: **0**
- Japanese sentence-end ASCII period defect: **0**
- Blocking English residue: **0**
- Blocking Hangul residue: **0**
- Source-position / rendering repair required: **0**
- Fix type: Japanese editorial-naturalness only
- Rule: Pages marked PASS in Batch 5 are closed unless a concrete defect is later identified.

---

## 1. Batch 5 scope — 11 pages

48. `tmoney-vs-wowpass.html`
49. `tmoney.html`
50. `where-to-stay-in-dongdaemun.html`
51. `where-to-stay-in-gangnam.html`
52. `where-to-stay-in-hongdae.html`
53. `where-to-stay-in-insadong.html`
54. `where-to-stay-in-itaewon.html`
55. `where-to-stay-in-jamsil.html`
56. `where-to-stay-in-myeongdong.html`
57. `where-to-stay-in-seongsu.html`
58. `wowpass.html`

---

## 2. PASS — 9 pages

- `tmoney-vs-wowpass.html`
- `tmoney.html`
- `where-to-stay-in-hongdae.html`
- `where-to-stay-in-insadong.html`
- `where-to-stay-in-itaewon.html`
- `where-to-stay-in-jamsil.html`
- `where-to-stay-in-myeongdong.html`
- `where-to-stay-in-seongsu.html`
- `wowpass.html`

### PASS judgment rule

Isolated uses of `強み`, `意味があります`, `自動的`, etc. are not treated as defects by themselves. They were kept closed where:
- the sentence reads naturally in Japanese,
- the expression is semantically literal rather than an English evaluation template,
- the same pattern is not mechanically repeated across the page,
- facts / recommendations / traveler-fit remain aligned with English.

### Notes

#### `tmoney-vs-wowpass.html`
`自動的` is used literally for balance behavior and FAQ logic, not as ranking-template language.

#### `tmoney.html`
`自動的に割引運賃になるとは限りません` is natural, literal rule wording.

#### `where-to-stay-in-hongdae.html`
`強み` is mostly used as ordinary “advantage” wording. One `RYSEがより強い候補` line is slightly English-shaped, but not enough by itself to reopen an otherwise natural page.

#### `where-to-stay-in-insadong.html`
No recurring English-calque evaluation pattern detected.

#### `where-to-stay-in-itaewon.html`
`自動的につながる` etc. describes a literal booking condition. No template regression.

#### `where-to-stay-in-jamsil.html`
`強みが最も生きます` and similar isolated language remains understandable and natural enough in context.

#### `where-to-stay-in-myeongdong.html`
Korean NAVER Map search strings such as:
- `르메르디앙 서울 명동`
- `나인트리 바이 파르나스 서울 명동 II`

are intentional search aids, not untranslated residue.

#### `where-to-stay-in-seongsu.html`
Repeated `泊まる意味` wording is contextually natural decision language, not a broad strong/stronger template.

#### `wowpass.html`
`自動的` is used literally for balance/payment behavior and is appropriate.

---

## 3. FIX — 2 pages

### A. `where-to-stay-in-dongdaemun.html` — FIX

Structure, facts, hotel order, room data and recommendation hierarchy are correct.

The issue is a small cluster of English-derived evaluative `strong` phrasing.

Representative wording:

1. `ホテルまで移す理由はそれほど強くありません。`
2. `夜の買い物に強いという利点`
3. `...夜遅くまで買い物する旅行に強い理由です。`
4. `Summitも空港からの到着に強く...`

Not treated as defects:
- `存在感が強くなり`
- `これはSotetsuとは別の強みです`
- `自動的に安いわけではありません`

These are natural contextual Japanese.

#### Correction principle

Preserve:
- DDP / Dongdaemun Station / west-side area logic
- hotel ordering and traveler-fit
- airport bus numbers / exits / walking distances
- room sizes, beds, occupancy
- affiliate links

Rewrite only the literal `strong` evaluation frames into concrete Japanese such as:
- `泊まる必要性は高くありません`
- `夜遅い買い物に便利`
- `この条件がそろうため使いやすい`
- `空港から到着しやすい`

Do not change recommendation strength.

---

### B. `where-to-stay-in-gangnam.html` — FIX

Structure and factual layer are correct.

This page has a denser cluster of English-derived abstract evaluation language.

Representative wording:

1. `Ocloudのほうが合理的です。`
2. `新論峴は夜の動きも強いエリアです。`
3. `こちらのほうが拠点として強いです。`
4. `立地の意味が出ます。`
5. `AC Hotelは...最も強いホテルです。`
6. `高級ブティックなら狎鴎亭・清潭のほうが強く...`

Not treated as defects:
- `高級だから自動的に『江南で一番』というわけではありません`
- `ファミリー客室だから自動的に眺めが良い...`
These are natural literal uses of `自動的`.

#### Correction principle

Preserve:
- Gangnam Station / Sinnonhyeon / Yeoksam
- Samseong / COEX
- Apgujeong / Cheongdam
- Sinsa / Dosan
- airport bus numbers and stops
- hotel facts / rooms / facilities / traveler-fit
- existing hotel order and recommendation hierarchy

Replace:
- `合理的`
- `強い拠点`
- `最も強いホテル`
- `意味が出る`
- repeated comparative `強い`

with concrete route/fit language:
- `戻りやすい`
- `この旅程に合う`
- `駅三を使う予定なら特に便利`
- `狎鴎亭・清潭中心の旅程ならこちらが使いやすい`

Do not upgrade or downgrade any hotel.

---

## 4. Common verification result

Across all 11 Batch 5 pages:

### Structure
- major English ↔ Japanese element counts: **11/11 aligned**
- head target parity: **11/11**
- large section omission: **0**
- invented section: **0**
- suspicious `<br>` word fragmentation: **0**

### Content
- material recommendation distortion: **0**
- material factual/number error: **0**
- hotel order distortion: **0**
- traveler-fit distortion: **0**

### FAQ / JSON-LD
- parse error: **0**
- English/Japanese schema presence parity: **PASS**
- visible FAQ/schema mismatch where schema exists: **0**
- JSON-LD English residue: **0**

### Regression guard
- raw Markdown: **0**
- Japanese sentence-end ASCII period: **0**
- blocking English residue: **0**
- blocking Hangul residue: **0**

### Numeric scan
Differences detected mechanically were unit-localization forms such as:
- `24 → 24時間`
- `100 → 100㎡`
- `150 → 150m`
- `18 → 18歳`

No material numeric omission was found.

---

## 5. Batch 5 classification

- **9 PASS**
- **2 wording-only FIX**
- source-position repair: **0**
- head-target repair: **0**

This Batch requires no structural rebuild.

---

## 6. Audit completion

With Batch 5 complete:

> **JA DIRECT AUDIT 5/5 COMPLETE — 58/58 PAGES AUDITED**

Do not reopen PASS pages during the correction phase unless a concrete defect is identified.

Next workflow:
1. consolidate all FIX pages
2. create correction wording for FIX pages only
3. user approves correction set
4. exact implementation
5. scoped QA
6. Production only after explicit approval
