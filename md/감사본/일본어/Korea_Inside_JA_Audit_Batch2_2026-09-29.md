# Korea Inside — Japanese Direct Audit Batch 2

- File: `Korea_Inside_JA_Audit_Batch2_2026-09-29.md`
- Date: 2026-09-29
- Status: **JA AUDIT 2/5 COMPLETE**
- Scope: English Production/source parity vs Japanese Production/source, direct GPT editorial audit
- Pages: 12
- Result: **4 PASS / 8 FIX**
- Structural parity: **12/12 PASS**
- Large omission/addition: **0**
- JSON-LD parse error: **0**
- Visible FAQ / JSON-LD count mismatch: **0**
- Material recommendation/fact distortion: **0**
- Material numeric/factual error: **0**
- Blocking English/Hangul residue: **0**
- Fix type: editorial-naturalness / Japanese calque cleanup only
- Rule: Pages marked PASS in Batch 2 are not reopened during later audit batches unless a concrete defect is found.

---

## 1. Batch 2 scope — 12 pages

13. `best-area-for-first-time-visitors-seoul.html`
14. `best-area-for-luxury-hotels-seoul.html`
15. `best-area-for-nightlife-seoul.html`
16. `best-area-for-shopping-seoul.html`
17. `best-area-for-solo-travelers-seoul.html`
18. `best-esim-for-korea.html`
19. `card-declined-korea.html`
20. `checklist.html`
21. `dongdaemun-travel-guide.html`
22. `esim.html`
23. `foreign-credit-cards-korea.html`
24. `gangnam-travel-guide.html`

---

## 2. PASS — 4 pages

- `best-esim-for-korea.html`
- `checklist.html`
- `esim.html`
- `foreign-credit-cards-korea.html`

PASS means English↔Japanese structure and section order align, with no major omission/addition, material recommendation change, blocking numeric/factual distortion, blocking English residue, or concrete Japanese-naturalness defect requiring reopening at this audit stage.

Isolated expressions such as `意味があります` are not treated as defects by themselves when the surrounding Japanese is natural and the expression is not functioning as a repeated English-calque template.

---

## 3. FIX — 8 pages

These are editorial-naturalness fixes, not structural rebuilds.

### A. `best-area-for-first-time-visitors-seoul.html` — FIX

Main flagged wording:

1. `ソウル駅や麻浦／孔徳が強くなります。`
2. `ソウル市内での分泊が合理的なのは...`
3. table: `交通接続が強い`
4. table: `雰囲気より実用性`
5. table: `南側では強い`
6. `...興味がある旅行者に強いです。`
7. `...実際の旅程に理由がある場合に強い拠点です。`
8. FAQ: `...蚕室も強いです。`
9. final: `...到着・出発の実務が重要なら...`

The visible FAQ wording has the corresponding JSON-LD mirror and must remain meaning-equivalent/exact as required by the current structure.

Correction principle:
- preserve the existing recommendation hierarchy
- replace literal `強い / 強くなる / 合理的 / 実用性 / 実務` with natural Japanese fit, convenience, route, or decision wording
- do not introduce a new winner or strengthen/soften the English judgment

### B. `best-area-for-luxury-hotels-seoul.html` — FIX

Main flagged wording:

1. H2: `高級ホテルに泊まるなら、どのエリアが意味を持つ？`
2. `...高い宿泊費を払う意味が出てきます。`
3. table: `ソウル南東部の予定に強い`
4. table: `乗り換えや移動の多い旅に強い`
5. table: `鍾路・王宮方面の動線に強い`
6. table: `食事中心の滞在に強い`
7. `江南は高級ホテル拠点として最も理由を作りやすいエリアです。`
8. `...場合に最も強い選択です。`
9. `...蚕室の高い宿泊費に意味が出ます。`
10. FAQ: `...総合候補としては江南が強く...`
11. final: `江南が最も理由を作りやすい高級ホテル拠点`
12. final: `...賢い高級滞在になることもあります。`

Correction principle:
- preserve the luxury-area comparison and conditional judgments
- rewrite English-derived `makes sense / strong / strongest / easiest case to justify / smart` framing into natural Japanese
- keep the page area-first, not a hotel ranking

### C. `best-area-for-nightlife-seoul.html` — FIX

Main flagged wording:

1. `...ナイトライフが旅行の一部にすぎない場合に意味が出ます。`
2. `...ここに泊まる意味が出ます。`
3. `明洞はソウルで最も強いナイトライフエリアではありませんが...`
4. `...合理的なトレードオフにできます。`
5. FAQ: `...場合に最も意味があります。`
6. FAQ: `...中心部観光や買い物に強いエリアです。`
7. related-card wording: `夜の活気に強い西側の拠点`
8. final: `...江南がより強くなります。`

Not treated as a defect:
- descriptive `国際色がより強い` where `強い` means “more pronounced”

Correction principle:
- preserve nightlife hierarchy and trade-offs
- replace comparative English-calque phrasing with natural Japanese suitability/priority wording
- do not change which traveler type each area fits

### D. `best-area-for-shopping-seoul.html` — FIX

Main flagged wording:

1. `...江南がより強い拠点です。`
2. `...買い物の性格を優先するときに強くなる選択です。`
3. table: `ソウル全体を回る拠点としては弱め`
4. `江南に泊まる意味があります。`
5. `...宿泊拠点として強くなります。`
6. final: `...江南がより強くなります。`
7. final: `...宿泊地として意味が出ます。`

Not treated as defects:
- contextual descriptive uses such as `旅行者向けの色が強く`
- ordinary noun uses such as `買い物の強み`

Correction principle:
- preserve shopping-type distinctions and area hierarchy
- express fit naturally instead of strong/weak/makes-sense calques

### E. `best-area-for-solo-travelers-seoul.html` — FIX

Main flagged wording:

1. `...ソウル駅や麻浦／孔徳が強くなります。`
2. `...到着日を簡単にすることに強みがあります。`
3. `...到着・出発の簡単さを優先するなら強いです。`
4. `...明確な南側の理由があるときに強い拠点です。`
5. FAQ: `...初めての一人旅の拠点として簡単です。`

The FAQ wording also has a JSON-LD mirror.

Correction principle:
- preserve recommendation strength
- replace literal `strong` framing and unnatural `拠点として簡単` with native Japanese traveler-fit wording

### F. `card-declined-korea.html` — FIX

One clear wording defect:

- `問題がカードについて回るのか、場所について回るのかを見る。`

This is an English-calque diagnostic phrase and is not natural Japanese.

Correction principle:
- preserve the diagnostic logic: determine whether failure follows the same card across locations, or occurs only at a specific place/terminal
- rewrite naturally in Japanese without changing the troubleshooting sequence

### G. `dongdaemun-travel-guide.html` — FIX

Small localized cleanup only.

Flagged wording:

1. H1 typography: `ソウル・東大門（トンデムン）観光ガイド 2026 ：DDP・市場・ナイトショッピング`
   - unnatural space before the full-width colon
2. `その時間に行く意味がある場所だけに絞り込めます。`
3. `卸売： 他の店へ商品を供給する売り手と買い手の商取引の場で買う。`

Correction principle:
- preserve the six-area orientation, route logic and retail/wholesale distinction
- clean Japanese typography and literal phrasing only
- do not restructure the guide

### H. `gangnam-travel-guide.html` — FIX

This page has the largest concentration of literal editorial abstractions in Batch 2.

Flagged wording includes:

1. `目的地型カフェ`
2. `江南の価値は最も強くなります。`
3. `...意味があるルートです。`
4. `...昼食を取れる実用的な場所です。`
5. `目的地型ショッピング`
6. H3: `K-Star Roadは「ファンの層」として使う`
7. `一部のK-pop関心には強いエリアです。`
8. `...狎鴎亭で終えても合理的です。`
9. `...島山／狎鴎亭側のほうが役割が明確です。`
10. `現在のリテール文化が旅の一部ならこちらが強いです。`
11. another `目的地型カフェ`
12. `到着前に「どのCOEX」を使うか決めてください。`
13. `初回旅行の自動的な標準ではありません。`

Correction principle:
- preserve both approved routes, exclusion logic, time allocation and traveler-fit judgments
- remove literal English editorial jargon (`destination café/shopping`, `fan layer`, `automatic default`, `rational`, `strong`)
- rewrite as normal Japanese travel-guide prose
- do not add or remove attractions, route stops or recommendations

---

## 4. Common verification result

Across all 12 Batch 2 pages:

- English ↔ Japanese main-body structure: **12/12 aligned**
- H1/H2/H3 and main structural element counts: **aligned**
- section sequence: **aligned**
- large omitted sections: **0**
- large invented sections: **0**
- material recommendation distortion: **0**
- material numeric/factual error: **0**
- JSON-LD parse errors: **0**
- visible FAQ / JSON-LD question-count mismatch: **0**
- blocking Japanese sentence-end ASCII period: **0**
- raw Markdown residue: **0**
- Hangul residue: **0**
- blocking page-specific English residue: **0**

Numeric differences detected by mechanical scanning were unit-localization differences such as `7 days → 7日`, `20 minutes → 20分`, not missing facts.

Therefore Batch 2 is a **Japanese wording cleanup batch**, not a source-sync or structural rebuild.

---

## 5. Do not do yet

Do not:

- edit these 8 FIX pages while the remaining Japanese audit batches are still in progress, unless the user explicitly changes the workflow
- reopen the 4 PASS pages
- reopen JA Audit 1 PASS pages
- run all 58 pages again
- stage / commit / push / deploy
- alter facts, numbers, recommendation hierarchy, affiliate/tracking, images, canonical/hreflang, CSS/JS or common UI

---

## 6. Next audit

Next: **JA Audit 3/5 — 12 pages**

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

Audit only these 12 pages next.
