# Korea Inside — Japanese Direct Audit Batch 3

- File: `Korea_Inside_JA_Audit_Batch3_2026-09-29.md`
- Date: 2026-09-29
- Status: **JA AUDIT 3/5 COMPLETE**
- Scope: Current English main source vs Japanese main source, direct GPT editorial/source-parity audit
- Pages: 12
- Result: **4 PASS / 8 FIX**
- HTML structural parity by element counts: **12/12 PASS**
- Large omitted/invented section: **0**
- Material recommendation distortion: **0**
- Material numeric/factual error: **0**
- JSON-LD parse error: **0**
- Visible FAQ / FAQPage question-count mismatch: **0**
- Raw Markdown residue: **0**
- Japanese sentence-end ASCII period defect: **0**
- Blocking English residue: **0**
- Hangul residue: **0 blocking**  
  - Hongdae contains Korean exhibition titles/provenance names only; these are proper names, not untranslated residue.
- Small visible source-parity addition: **1 page**
- Japanese source-position / rendering fragmentation: **4 additional pages**
- Rule: Pages marked PASS in Batch 3 are not reopened during later audit batches unless a concrete defect is found.

---

## 1. Batch 3 scope — 12 pages

25. `gongdeok-mapo-seoul-guide.html`
26. `hongdae-travel-guide.html`
27. `hongdae-vs-myeongdong.html`
28. `hotels-near-gongdeok-station.html`
29. `hotels-near-seoul-station.html`
30. `incheon-airport-private-transfer.html`
31. `index.html`
32. `insadong-travel-guide.html`
33. `itaewon-travel-guide.html`
34. `jamsil-travel-guide.html`
35. `k-beauty.html`
36. `korea-atm-foreign-cards.html`

---

## 2. PASS — 4 pages

- `incheon-airport-private-transfer.html`
- `index.html`
- `k-beauty.html`
- `korea-atm-foreign-cards.html`

PASS means:
- English↔Japanese structural role and section sequence align
- no material omission/addition
- no recommendation/factual/numeric distortion
- no concrete Japanese-naturalness defect requiring reopening at this audit stage
- FAQ/schema structure remains consistent with English where applicable

### PASS verification notes

#### `index.html`
Text extraction initially appeared to show `同じです ：通信...`, but raw HTML is:

`<a>...同じです</a>：通信...`

The browser-visible Japanese has no extra space. This is **not** a defect.

#### `k-beauty.html`
Text extraction initially appeared to show `韓国の Kビューティー`, but raw HTML is:

`韓国の<span>Kビューティー</span>`

The browser-visible H1 is `韓国のKビューティー`. This is **not** a defect.

Visible FAQ exists without FAQPage schema; English uses the same structure, so this is not a parity defect.

---

## 3. FIX — 8 pages

### A. `gongdeok-mapo-seoul-guide.html` — FIX

This page has both editorial-calque issues and a real source-position/rendering defect.

#### 1) Japanese wording

Flagged examples:

- around line 749: `寝る場所として強い旅行者もいます。`
- around line 779: `孔徳が強いのは...`
- around line 828: `孔徳・麻浦が最も強いのは...`

These are English-derived `strong / strongest` evaluation frames. Preserve the judgment but rewrite as natural Japanese suitability/fit language.

#### 2) Source-position / `<br>` fragmentation — concrete defect

English source:

`jeon and drinks ...`  
`or jokbal ...`  
`or pork galbi ...`  
`or ... seolleongtang or naengmyeon`

Japanese implementation around line 551 currently breaks Japanese words across `<br>` positions:

- `...夕<strong>食にするまたは</strong>麻<br>浦...`
- `...豚カルビま</strong>た<br>は...`

This produces visible breaks inside:
- `麻浦`
- `または`

Additional fragmented route blocks occur around:
- lines 702–710
- lines 1001–1003
- line 1039

Examples include:
- `麻<br>浦`
- `ホテ<br>ル`
- `ローカル夕<br>食`
- `終わ<br>り`

#### Correction principle

- preserve facts, route order, affiliate placements and HTML component structure
- rebuild the Japanese wording around the existing inline/`<br>` positions so Japanese words are never split
- do not add or remove route choices
- replace repeated `強い` calques naturally

---

### B. `hongdae-travel-guide.html` — FIX

No structural/source-node break was found. This is an editorial-naturalness cleanup.

Flagged examples:

- line ~493: `...一度訪れるほうが合理的です。`
- line ~972: `これは弘大に宿泊する強い理由のひとつです。`
- line ~1050: `過ごしやすい天気の日には強い追加先になります。`
- line ~1051: `...弱い追加先です。`
- line ~1251: `どれも合理的に見えます。`

The page also contains Korean exhibition titles such as `《입영 전야》`, `《임시휴먼》`, `《유령들의 사회》`; these are proper exhibition names and are **not** English/Korean residue defects.

#### Correction principle

- preserve the same routes, timing, stay/visit judgment and traveler-fit logic
- rewrite `合理的 / 強い追加先 / 弱い追加先 / 強い理由` into normal Japanese travel-guide wording
- do not change the underlying recommendation

---

### C. `hongdae-vs-myeongdong.html` — FIX

This page has both editorial-calque issues and one small visible source-parity addition.

#### 1) Small visible source-parity addition

English short-answer paragraph ends at:

`...rather than Hongdae or Myeongdong as a whole.`

Japanese paragraph currently adds:

`ソウルの弘大と明洞の夜の雰囲気を比較した編集用イラスト`

at the end of the answer paragraph.

That same text then correctly appears again as:
- image `alt`
- figure caption

Therefore the paragraph contains **one unintended extra visible phrase** not present in English body copy.

#### 2) Japanese wording

Flagged examples:

- line ~380: `...弘大が強くなります。`
- line ~381 and FAQ mirror: `自動的な勝者はいません。`
- line ~478: `通常は明洞のほうが合理的です。`
- table: `Kビューティーと旅行者向けの買い物に強い。`
- FAQ: `弘大が明確に強いです。`

#### Correction principle

- remove only the unintended body-copy image-description addition
- keep alt/caption in their correct locations
- preserve the current recommendation hierarchy
- rewrite `strong / automatic winner / rational` calques naturally
- visible FAQ and JSON-LD mirror must remain aligned

---

### D. `hotels-near-gongdeok-station.html` — FIX

This page has both wording issues and a source-position implementation defect.

#### 1) Text fragments outside the intended scenario-title `<p>`

In the scenario list, raw Japanese HTML currently includes:

- `</p>。<p>` after `汝矣島とソウル中心部に分かれる出張`
- `</p>場合。<p>` after `弘大に行くが弘大には泊まらない`
- `</p>。<p>` after `空港前後の初日・最終泊`

The English source contains no equivalent stray fragments.

These create awkward visible punctuation/phrase fragments between title and body.

#### 2) Japanese wording

Flagged examples:

- line ~363: `孔徳を選ぶ理由が強くなります。`
- line ~394: `交通ハブとしては孔徳が強く...`
- line ~484: `...実用性がはっきりします。`
- line ~587: `ホテルの役割判断`
- line ~639: `孔徳を選ぶ意味があります。`

#### Correction principle

- restore each scenario title/body boundary cleanly
- remove the stray `。` / `場合。` fragments from outside the title nodes
- preserve hotel facts, room sizes, station/exits, traveler-fit judgments and hotel ordering
- naturalize literal evaluation wording only

---

### E. `hotels-near-seoul-station.html` — FIX

Editorial-naturalness cleanup only; no structural/source-position break found.

Flagged examples:

- line ~484: `ソウル駅を自動的な第一候補にはしません。`
- line ~530: `ソウル駅が旅程の中で実際に役割を持つとき、このエリアに泊まる意味が生まれます。`
- line ~865: `すべてのホテルでAREXが自動的に最適とは限りません。`

#### Correction principle

Preserve the current judgment:
- Seoul Station is not the default for every first trip
- it becomes useful when KTX / AREX / early departures / onward travel actually matter

Rewrite the English-derived `automatic default / has a role / automatically optimal` framing as natural Japanese.

---

### F. `insadong-travel-guide.html` — FIX

No source-position break found. The issue is repeated English-derived editorial evaluation language.

Representative flagged wording:

- line ~246: `...歴史地区ルートの中に入れるとさらに強くなります。`
- line ~260: `...ペースを落としたいときに強い街です。`
- line ~361: `...のほうが強い一日です。`
- line ~393: `...決めるほうが合理的です。`
- line ~617: `...仁寺洞の時間を強くします。`
- line ~671: `ベジタリアン旅行者には意外に強い選択肢がある`
- line ~920: `...横道へ入る使い方が最も強いです。`
- line ~995: `...歴史地区の一日に入れると強いです。`
- line ~1043: `仁寺洞は昼〜早い夕方が強いです。`

#### Correction principle

- preserve the page’s slower daytime/culture/tea/craft judgment
- replace repeated `strong / stronger / rational` framing with Japanese timing/fit/route language
- do not change the page’s advice to move elsewhere for nightlife

---

### G. `itaewon-travel-guide.html` — FIX

This page has repeated calques and one clear source-position/rendering failure.

#### 1) Evening-route block is visibly broken

English source:

- `Quiet: Gyeongnidan`
- `Neighborhood: Haebangchon`
- `Nightlife: central Itaewon bars / pub crawl / clubs`
- `Finished: go back to the hotel`

Japanese raw HTML around lines 786–789 currently splits words across `<strong>` / `<br>` boundaries:

- `街を歩く：解</strong>放村ナイト<br>`
- `<strong>ライフ：</strong>中央梨泰院...／ク<br>`
- `<strong>ラブもう</strong>十分：ホテルへ戻る`

This visibly breaks:
- `解放村`
- `ナイトライフ`
- `クラブ`

and merges the labels/content incorrectly.

#### 2) Japanese wording

Representative examples:

- line ~485: `最も強いスタート地点です。`
- line ~508: `漢南は昼に強く...`
- line ~581: `食はかなり強いです。`
- repeated headings around lines 823–835: `強く合う — ...`
- line ~895: `短いタクシーが合理的です。`
- line ~964: `昼の強い理由`
- line ~1018: `強いルートは...`

#### Correction principle

- restore the four evening choices as four coherent Japanese labels/lines
- preserve the east-to-west time-of-day logic
- rewrite repeated `strong / rational` editorial calques naturally
- do not change Muslim-traveler, food, Hannam, nightlife or hill-friction judgments

---

### H. `jamsil-travel-guide.html` — FIX

This page has a concrete weather-block source-position failure plus repeated English-derived evaluation wording.

#### 1) Weather block mapping defect

English source:

- `Rainy day:` → Aquarium → Mall...
- `Clear day:` → Aquarium or Mall → Lake...
- `Hot summer afternoon:` → stay indoors...
- `Hazy day:` → do not build the schedule around the observatory...

Japanese currently maps the labels into the wrong nodes:

- line ~675 ends the introductory paragraph with `雨の日`
- line ~676 ends the next paragraph with `晴れの日`
- line ~678 combines `暑い夏の午後` directly with the instruction without label punctuation
- line ~679 combines `霞が強い日` directly with the instruction

The Japanese route logic is understandable, but the visible source-position formatting is broken.

#### 2) Japanese wording

Representative examples:

- JSON-LD / FAQ: `通常は自動的な標準にはしません。`
- line ~557: `...場合に最も意味があります。`
- line ~774: `...グループには強いです。`
- line ~901: `強い宿泊エリアです。`
- line ~903: `自動的な拠点としては弱くなります。`
- line ~945: `蚕室は最も強くなります。`

#### Correction principle

- restore the four weather labels to their corresponding paragraphs
- preserve Jamsil’s family / Lotte / event / southeastern-Seoul decision logic
- rewrite `automatic default / strong base / strongest` framing naturally
- visible FAQ / JSON-LD wording must remain aligned after correction

---

## 4. Structural / source-parity findings

### HTML structural-count mismatch

**0 pages**

Across all 12 pages:
- H1/H2/H3 and major element counts align
- large section omission: 0
- large invented section: 0
- JSON-LD parse error: 0

### Source-position / visible-rendering mismatch — 5 pages

1. `gongdeok-mapo-seoul-guide.html`
   - Japanese words/routes split by existing `<br>` positions; source-node wording placement needs repair.

2. `hongdae-vs-myeongdong.html`
   - one extra image-description phrase appended to visible short-answer body copy.

3. `hotels-near-gongdeok-station.html`
   - stray `。` / `場合。` fragments outside scenario-title paragraphs.

4. `itaewon-travel-guide.html`
   - evening route labels split `解放村 / ナイトライフ / クラブ`.

5. `jamsil-travel-guide.html`
   - weather labels shifted into preceding/following paragraphs.

These are **not** large page rebuilds. The approved page structure and recommendation logic can remain intact.

---

## 5. Common verification result

Across all 12 Batch 3 pages:

- English ↔ Japanese major structure: **12/12 aligned**
- material recommendation distortion: **0**
- material numeric/factual error: **0**
- JSON-LD parse errors: **0**
- visible FAQ / FAQPage count mismatch: **0**
- raw Markdown residue: **0**
- blocking English residue: **0**
- blocking Korean residue: **0**
- large omitted section: **0**
- large invented section: **0**

Mechanical numeric differences were unit-localization differences such as:
- `220 → 220m`
- `90 → 90分`
- `24 → 24時間`
- `1.2 → 1.2km`

not missing facts.

Therefore Batch 3 is primarily:
- **Japanese editorial-naturalness cleanup**
plus
- **focused source-position/rendering repair on 5 pages**

It is **not** a full source-sync rebuild.

---

## 6. Do not do yet

Do not:

- edit these 8 FIX pages while JA Audit 4/5 and 5/5 remain, unless the user explicitly changes the workflow
- reopen the 4 PASS pages
- reopen PASS pages from Batch 1 or Batch 2
- run all 58 pages again
- stage / commit / push / deploy
- alter facts, numbers, recommendation hierarchy, affiliate/tracking, images, canonical/hreflang, CSS/JS or common UI

---

## 7. Next audit

Next: **JA Audit 4/5 — 11 pages**

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

Audit only these 11 pages next.
