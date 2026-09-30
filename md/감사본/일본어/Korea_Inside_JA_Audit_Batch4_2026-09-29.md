# Korea Inside — Japanese Direct Audit Batch 4

- File: `Korea_Inside_JA_Audit_Batch4_2026-09-29.md`
- Date: 2026-09-29
- Status: **JA AUDIT 4/5 COMPLETE**
- Scope: Current English source vs Japanese source, direct GPT editorial/source-parity audit
- Pages: 11
- Result: **8 PASS / 3 FIX**
- Main-body structural parity: **11/11 PASS**
- Head target parity: **10/11 PASS**
- Large omitted/invented section: **0**
- Material recommendation distortion: **0**
- Material numeric/factual error: **0**
- JSON-LD parse error: **0**
- Visible FAQ / FAQPage question-count mismatch: **0**
- Raw Markdown residue: **0**
- Japanese sentence-end ASCII period defect: **0**
- Blocking English residue: **0**
- Blocking Hangul residue: **0**
- Source-position / rendering repair required: **1 page**
- Editorial-naturalness cleanup required: **2 additional pages**
- Rule: Pages marked PASS in Batch 4 are not reopened during later audit batches unless a concrete defect is found.

---

## 1. Batch 4 scope — 11 pages

37. `korea-esim-with-phone-number.html`
38. `korean-online-payments-foreigners.html`
39. `lotte-world-seoul.html`
40. `maps.html`
41. `myeongdong-travel-guide.html`
42. `payments.html`
43. `rental-car.html`
44. `seongsu-travel-guide.html`
45. `seoul-sky-guide.html`
46. `taste-korea.html`
47. `taxi.html`

---

## 2. PASS — 8 pages

- `korea-esim-with-phone-number.html`
- `korean-online-payments-foreigners.html`
- `maps.html`
- `payments.html`
- `rental-car.html`
- `seoul-sky-guide.html`
- `taste-korea.html`
- `taxi.html`

PASS means:
- English↔Japanese structure and page role align
- no large omission/addition
- no material recommendation/fact/number distortion
- no concrete source-position/rendering defect
- Japanese reads naturally enough that isolated words such as `意味があります`, `自動的`, `強み`, `役割`, or `合理的` do not justify reopening the page by themselves

### PASS notes

#### `korea-esim-with-phone-number.html`
One isolated sentence uses `追加設定に意味があります`, but the page as a whole does not repeat the English-calque pattern and reads naturally. Not treated as a defect.

#### `maps.html`
Text extraction appeared to add spaces around labels such as `やること ：`, but raw HTML uses inline `<span>` boundaries. No visible punctuation defect was confirmed.

#### `seoul-sky-guide.html`
Text extraction initially showed forms such as `Kore a`, `チケ ット`, `予 定`, but raw HTML confirmed these are `<strong>` boundaries and render contiguously in the browser. They are not visible word breaks.

Isolated uses of `合理的` / `自動的` are not frequent enough to constitute a template-style regression.

#### `taste-korea.html`
Uses such as `ソウルの強み`, `聖水の役割`, and similar wording are contextually natural Japanese, not mechanical comparative ranking language.

#### Intentional Korean text
- `payments.html`: Korean phrase for separate checks
- `taxi.html`: `빈차`
- `seoul-sky-guide.html`: `서울스카이`

These are deliberate Korean-language references, not localization residue.

---

## 3. FIX — 3 pages

### A. `lotte-world-seoul.html` — FIX

This is the only Batch 4 page with **source-position / rendering defects** and a **head source-target mismatch**.

#### 1) Head source-target mismatch

English source currently contains:

`<!-- Meta description awaits approved public copy. -->`

and **does not have a meta description target**.

Japanese currently adds:

`<meta name="description" content="ロッテワールドソウルの1Day・After4、マジックパス、人気アトラクション、待ち時間対策、子連れ、雨の日、チケット購入まで実用的に解説します。">`

Therefore:
- English meta-description target: 0
- Japanese meta-description target: 1

This is not allowed under the Japanese source-coverage rule. Do not silently retain or redesign it during repair; fix must follow the current approved English source state or a separately approved English meta change.

#### 2) Concrete `<br>` / source-position rendering defects

Raw Japanese HTML contains multiple `<br>` placements inside words or sentence units.

Representative locations:

- line ~444  
  `ロッテワールドを丸一日使</strong><br>う...`
  - splits `使う`

- line ~447  
  `...乗りたいも</strong><br>のを決める`
  - splits `もの`

- line ~937  
  `でき</strong><br>れば乗りたい`
  - splits `できれば`

- line ~939  
  `ル</strong><br>ートは変えない`
  - splits `ルート`

- lines ~1077–1081  
  `A</strong>tlant<br>is`  
  `Com<br>et Express`  
  `Gyro S<br>wing`  
  `Gyr</strong>o Drop`  
  `French Revolut</strong>ion`
  - visibly fragments attraction names and Japanese labels

- line ~1179  
  `Premium事</strong><br>前購入...`
  - splits `事前購入`

- line ~1180  
  `当日購入すべての</strong><br>Magic Pass...`
  - label/body boundary is mapped incorrectly

- line ~1397  
  `Lotte World Adve... Olympic-<br>ro`
  - attraction/address comparison block is fragmented

#### 3) Japanese wording cleanup

Representative English-calque/editorial wording also exists:

- `...場合には合理的です。`
- `...屋外エリアへ早めの時間を取る強い理由になります。`
- `Magic Passを選ぶ理由は強くなります。`

Do not mechanically replace every `強い`:
- `強い回転`
- `強いライド`
- `強い風`
- ride-intensity descriptions

are semantically appropriate and should remain where they describe physical intensity.

#### Correction principle

- preserve all ride names, height restrictions, prices, times, recommendation hierarchy, Magic Pass logic, ticket/affiliate links, images and schema
- restore label/body units according to the current English source
- never split Japanese words or attraction names with `<br>`
- remove the Japanese-only meta-description target unless a separate approved English meta source is created
- naturalize only evaluative calques; do not weaken ride-intensity wording

---

### B. `myeongdong-travel-guide.html` — FIX

Main structure is aligned. No source-position/rendering defect was confirmed.

The issue is **repeated abstract English editorial framing**. Individual uses can be valid, but the density across the page makes parts of the Japanese read like translated editorial scaffolding rather than native travel prose.

Representative flagged wording:

- line ~456: `買う目的があると明洞は強い`
- line ~509: `...1日使う意味があります。`
- line ~644: `ブランド直営店にも役割がある`
- line ~646: `...サービスを確認する意味があります。`
- line ~700: `明洞餃子には明確な役割がある`
- line ~702: `役割が分かりやすい店です。`
- line ~793: `高いコースを自動的に選ばない`
- line ~849: `明洞との役割分担`
- line ~862: `自動的に行くべき場所ではありません。`
- line ~866: `...より強い理由になります。`
- line ~873: `...より合理的です。`
- line ~1109: `その便利さには意味があります。`
- line ~1123: `拠点としては強いです。`
- line ~1142 / ~1260–1261: repeated `役割` framing

Not treated as defects:
- `観光客向けの要素が強い`
- `強い雨`
- other descriptive uses where `強い` means “pronounced/intense,” not “better”

#### Correction principle

- preserve the same Myeongdong recommendation, shopping/K-beauty logic, NANTA/Namsan/Euljiro trade-offs and stay judgment
- replace repeated `strong / role / makes sense / automatic / rational` editorial abstractions with concrete Japanese action/fit wording
- do not change facts, times, businesses, route sequence or affiliate placements

---

### C. `seongsu-travel-guide.html` — FIX

Main structure is aligned. No source-position/rendering defect was confirmed.

The page has the highest Batch 4 concentration of English-derived abstract editorial wording after Lotte World.

Representative flagged wording:

- line ~307: `ファッションとKビューティーが特に強い`
- line ~310: `...商品を買うだけ以上の意味があります。`
- line ~323: `半日でも十分強い体験`
- line ~325: `丸一日も合理的です。`
- line ~515: `時間を使う意味がある`
- line ~517: `自動的に価値があるわけではありません。`
- line ~530: `...5件回るより強い一日です。`
- line ~555 / ~558 / ~567: repeated `意味がある`
- line ~617: `最初の1軒として強いです。`
- line ~649: `自動的に両方へ行く必要はありません。`
- line ~691: `カフェが一日の中で何の役割を持つか`
- line ~752: `このエリアを見る意味があります。`
- line ~819: `ソウルの森の基本的な役割`
- line ~860 / ~864: repeated `役割`
- line ~982: `自動的に両方入れないでください。`
- line ~1028: `聖水は十分役割を果たしています。`
- line ~1039: `泊まる意味があるのは...`
- line ~1054: `...ときに強い街です。`
- line ~1056: `自動的に丸一日ではありません。`

Not treated as defects:
- `強い寒さ`
- `存在感が強い`
- other literal intensity/pronounced-character uses

#### Correction principle

- preserve Seongsu’s current editorial judgment: half-day is often enough, full day only for strong fashion/K-beauty/design interest, pop-ups are optional/current, Seoul Forest depends on weather and interest, staying in Seongsu is conditional
- rewrite abstract `strong / makes sense / role / automatic / rational` scaffolding into natural Japanese travel-language
- do not add/remove pop-ups, brands, routes, stay decisions or time-sensitive facts

---

## 4. Common verification result

Across all 11 Batch 4 pages:

### Main-body structure
- English ↔ Japanese major element counts: **11/11 aligned**
- section sequence: aligned
- large omitted sections: **0**
- large invented sections: **0**

### Head / source target
- head parity: **10/11**
- exception: `lotte-world-seoul.html`
  - English meta description target 0
  - Japanese meta description target 1

### Content
- material recommendation distortion: **0**
- material numeric/factual error: **0**
- place/brand/ride hierarchy distortion: **0**

### FAQ / Schema
- JSON-LD parse errors: **0**
- visible FAQ / FAQPage count mismatch: **0**
- JSON-LD English residue: **0**

### Regression guard
- raw Markdown residue: **0**
- Japanese sentence-end ASCII period: **0**
- blocking English residue: **0**
- blocking Hangul residue: **0**

### Numeric scan
Mechanical differences were unit-localization differences, for example:
- `15 → 15日`
- `541 → 541m`
- `10 → 10時`
- `90 → 90分`

No material numeric omission was found.

---

## 5. Batch 4 classification

This Batch contains:

- **8 PASS pages**
- **2 wording-cleanup pages**
  - Myeongdong
  - Seongsu
- **1 source-position/head-parity repair page**
  - Lotte World

Therefore Batch 4 is **not** a full source-sync rebuild.

The Lotte World page requires focused implementation repair in addition to Japanese wording cleanup.

---

## 6. Do not do yet

Do not:

- edit the 3 FIX pages while JA Audit 5/5 remains, unless the user explicitly changes the workflow
- reopen the 8 PASS pages
- reopen PASS pages from Audit 1–3
- run all 58 pages again
- stage / commit / push / deploy
- alter facts, numbers, recommendation hierarchy, affiliate/tracking, images, canonical/hreflang, CSS/JS or common UI

---

## 7. Next audit

Next: **JA Audit 5/5 — 11 pages**

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

Audit only these 11 pages next.
