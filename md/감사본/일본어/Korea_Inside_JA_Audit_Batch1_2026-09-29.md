# Korea Inside — Japanese Direct Audit Batch 1

- File: `Korea_Inside_JA_Audit_Batch1_2026-09-29.md`
- Date: 2026-09-29
- Status: **JA AUDIT 1/5 COMPLETE**
- Scope: English Production vs Japanese Production, direct GPT editorial audit
- Pages: 12
- Result: **8 PASS / 4 FIX**
- Structural parity: **12/12 PASS**
- Large omission/addition: **0**
- JSON-LD parse error: **0**
- Recommendation/fact distortion: **0**
- Approximate wording fixes: **18 locations**
- Rule: Pages already marked PASS in Batch 1 are not reopened during later audit batches unless a concrete defect is found.

---

## 1. Batch 1 scope — 12 pages

1. `ja/accommodation.html`
2. `ja/airport-bus.html`
3. `ja/airport-transfer.html`
4. `ja/airport.html`
5. `ja/apple-pay-korea.html`
6. `ja/apps.html`
7. `ja/arex.html`
8. `ja/arrival.html`
9. `ja/best-area-for-airport-access-seoul.html`
10. `ja/best-area-for-budget-travelers-seoul.html`
11. `ja/best-area-for-couples-seoul.html`
12. `ja/best-area-for-families-seoul.html`

---

## 2. PASS — 8 pages

- `airport-bus.html`
- `airport-transfer.html`
- `airport.html`
- `apple-pay-korea.html`
- `apps.html`
- `arex.html`
- `arrival.html`
- `best-area-for-budget-travelers-seoul.html`

PASS means English↔Japanese structure and section order align, with no major omission/addition, material recommendation change, blocking numeric/factual distortion, or blocking English residue detected at this audit stage.

---

## 3. FIX — 4 pages / about 18 wording locations

These are editorial-naturalness fixes, not structural rebuilds.

### A. `accommodation.html` — FIX

Approx. 9 wording locations. The recurring problem is literal transfer of English evaluative wording such as “strong / stronger / safest default / practical” into Japanese.

Flagged wording:

1. `ソウル駅や麻浦／孔徳が強くなります。`
2. `最も安全な基本候補になります。`
3. `旅行の実務を簡単にするエリアです。`
4. `特に強くなります。`
5. `明洞は今も強いです。`
6. `強い宿泊エリアです。`
7. `強い代替です。`
8. Mapo/Gongdeok context의 equivalent `強い代替` wording
9. `空港アクセス、KTX、大きな荷物が重要なら強い選択です。`

Correction principle:
- preserve the recommendation itself
- replace literal `強い / 強くなる / 安全な基本 / 実務` calques with natural Japanese suitability/decision language such as `向いています`, `使いやすくなります`, `有力な候補です`, `無難な第一候補です`, or context-specific equivalents
- preserve facts, hierarchy and traveler-fit judgment

### B. `best-area-for-airport-access-seoul.html` — FIX

Approx. 5 wording locations.

Flagged wording:

1. H2: `ソウル駅：空港以外の乗り換えも重なるときに強い`
2. `ソウル駅が最も強い選択になります。`
3. `明洞がより強い選択になるのは次のような場合です。`
4. `AREX直通列車、KTX、重い荷物を重視するならソウル駅が強いです。`
5. `その後の鉄道移動に強いです。`

Correction principle:
- preserve comparison and recommendation strength
- rewrite `強い / 最も強い / より強い` as natural Japanese such as `便利です`, `向いています`, `有力です`, or context-specific equivalents
- do not introduce a new winner or stronger recommendation than English

### C. `best-area-for-couples-seoul.html` — FIX

Approx. 3 wording locations.

Flagged wording:

1. table: `漢江より南側の移動に強い`
2. table: `ソウル南東部の予定に強い`
3. body: `旅程の中に江南を拠点にする明確な理由がある場合に強い選択肢です。`

Correction principle:
- replace literal `強い` with natural suitability/fit language such as `移動しやすい`, `予定が多い旅程に向く`, `有力な候補`, or equivalent
- preserve the original trade-off

Not treated as blocking defects in Batch 1 because they are contextually more natural:
- `旅行者向けの雰囲気が強い`
- similar descriptive uses where `強い` means “pronounced” rather than “better”

### D. `best-area-for-families-seoul.html` — FIX

1 wording location.

Flagged wording:

- `大きな荷物、空港アクセス、静かな夜を優先するなら麻浦／孔徳が強くなります。`

Correction principle:
- rewrite naturally without changing the recommendation, e.g. suitability/advantage wording such as `麻浦／孔徳がより使いやすくなります` or equivalent
- `予定変更に強い拠点` is natural Japanese and is **not** a defect
- safety-related wording was not treated as a blocking defect on this page

---

## 4. Common verification result

Across all 12 Batch 1 pages:

- English ↔ Japanese main-body structure: **12/12 aligned**
- section sequence: **aligned**
- large omitted sections: **0**
- large invented sections: **0**
- material recommendation distortion: **0**
- material numeric/factual error: **0**
- JSON-LD parse errors: **0**
- FAQ structure: consistent with English where applicable

Therefore this is a **wording cleanup batch**, not a source-sync rebuild.

---

## 5. Do not do yet

Do not:

- edit these 4 FIX pages yet while the remaining Japanese audit batches are still in progress, unless the user explicitly changes the workflow
- reopen the 8 PASS pages
- run all 58 pages again
- stage / commit / push / deploy
- alter facts, numbers, rankings, affiliate/tracking, images, canonical/hreflang, CSS/JS or common UI

---

## 6. Next audit

Next: **JA Audit 2/5 — 12 pages**

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

Audit only these 12 pages next.
