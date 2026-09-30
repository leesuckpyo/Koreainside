# Korea Inside — Japanese Direct Audit Master Summary

- File: `Korea_Inside_JA_Direct_Audit_Master_Summary_2026-09-29.md`
- Date: 2026-09-29
- Status: **JA DIRECT AUDIT COMPLETE — 5/5**
- Total Japanese pages audited: **58**
- Final result: **33 PASS / 25 FIX**
- Audit mode: English source ↔ Japanese source direct GPT audit
- During audit: file edit 0 / stage 0 / commit 0 / push 0 / deploy 0

---

# 1. Final scorecard

| Batch | Pages | PASS | FIX |
|---|---:|---:|---:|
| Audit 1 | 12 | 8 | 4 |
| Audit 2 | 12 | 4 | 8 |
| Audit 3 | 12 | 4 | 8 |
| Audit 4 | 11 | 8 | 3 |
| Audit 5 | 11 | 9 | 2 |
| **TOTAL** | **58** | **33** | **25** |

---

# 2. Site-wide audit findings

Across the 58-page direct audit:

- major English ↔ Japanese body structure: **58/58 aligned**
- large omitted section: **0**
- large invented section: **0**
- material recommendation distortion: **0**
- material factual/number distortion: **0**
- JSON-LD parse errors: **0**
- blocking raw Markdown regression: **0**
- blocking Japanese ASCII-period regression: **0**
- blocking untranslated English/Korean residue: **0**

The Japanese site does **not** require a 58-page rebuild.

The correction scope is limited to the concrete 25 FIX pages.

---

# 3. FIX classification

## A. Wording-only / Japanese-naturalness FIX — 19 pages

### Batch 1
1. `accommodation.html`
2. `best-area-for-airport-access-seoul.html`
3. `best-area-for-couples-seoul.html`
4. `best-area-for-families-seoul.html`

### Batch 2
5. `best-area-for-first-time-visitors-seoul.html`
6. `best-area-for-luxury-hotels-seoul.html`
7. `best-area-for-nightlife-seoul.html`
8. `best-area-for-shopping-seoul.html`
9. `best-area-for-solo-travelers-seoul.html`
10. `card-declined-korea.html`
11. `dongdaemun-travel-guide.html`
12. `gangnam-travel-guide.html`

### Batch 3 — wording-only subset
13. `hongdae-travel-guide.html`
14. `hotels-near-seoul-station.html`
15. `insadong-travel-guide.html`

### Batch 4 — wording-only subset
16. `myeongdong-travel-guide.html`
17. `seongsu-travel-guide.html`

### Batch 5
18. `where-to-stay-in-dongdaemun.html`
19. `where-to-stay-in-gangnam.html`

Main recurring issue:
- literal transfer of English evaluative scaffolding such as:
  - strong / stronger / strongest
  - makes sense
  - rational
  - automatic default
  - practical / practical base
  - role
  - reason becomes stronger
- rendered in Japanese as repeated:
  - `強い / 強くなる`
  - `合理的`
  - `意味が出る`
  - `自動的`
  - `役割`
  - other abstract editorial frames

The facts and recommendations are generally correct. These are targeted Humanization/localization corrections.

---

## B. Source-position / rendering / head-parity repair — 6 pages

These pages require more than prose polishing.

### Batch 3
1. `gongdeok-mapo-seoul-guide.html`
   - `<br>` positions split Japanese words/routes such as `麻浦`, `または`, `ホテル`
   - wording cleanup also required

2. `hongdae-vs-myeongdong.html`
   - image-description phrase incorrectly appended to visible short-answer paragraph
   - alt/caption itself remains in correct image locations
   - wording cleanup also required

3. `hotels-near-gongdeok-station.html`
   - stray `。` / `場合。` fragments outside scenario-title paragraph nodes
   - wording cleanup also required

4. `itaewon-travel-guide.html`
   - evening choice block breaks `解放村 / ナイトライフ / クラブ` across inline/`<br>` positions
   - wording cleanup also required

5. `jamsil-travel-guide.html`
   - weather labels (`雨の日 / 晴れの日 / 暑い夏の午後 / 霞が強い日`) shifted into wrong paragraph positions
   - wording cleanup also required

### Batch 4
6. `lotte-world-seoul.html`
   - multiple visible `<br>` fragmentation defects:
     - `使う`
     - `もの`
     - `できれば`
     - `ルート`
     - `Atlantis`
     - `Comet Express`
     - `Gyro Swing`
     - `French Revolution`
     - `事前購入`
     - others
   - Japanese-only meta description target exists while current English source has no approved meta-description target
   - wording cleanup also required

---

# 4. PASS protection

**33 PASS pages are closed.**

Do not reopen them merely because:
- another expression is possible
- wording could be slightly prettier
- one isolated `強み`, `意味があります`, `自動的`, etc. exists
- a new room does not remember the prior work

Reopen only for:
- concrete Japanese sentence breakage
- factual error
- user direction
- newly identified structural/source-position defect
- visible FAQ/schema mismatch
- major official terminology/search-intent problem

---

# 5. Correction phase — recommended execution order

Do not correct all 25 pages blindly in one uncontrolled rewrite.

## Phase 1 — source-position / rendering repairs first
6 pages:
1. `gongdeok-mapo-seoul-guide.html`
2. `hongdae-vs-myeongdong.html`
3. `hotels-near-gongdeok-station.html`
4. `itaewon-travel-guide.html`
5. `jamsil-travel-guide.html`
6. `lotte-world-seoul.html`

Reason:
- these contain actual visible mapping/rendering issues
- wording corrections should be written around the correct node boundaries

## Phase 2 — wording-only correction set
19 pages:
- replace only concrete audited calques
- preserve facts, numbers, order, recommendations and HTML structure
- visible FAQ/schema mirrors must remain aligned

## Phase 3 — approval and implementation
- ChatGPT prepares correction copy
- user approves
- Codex exact implementation only
- static QA
- explicit Production approval
- scoped stage/commit/push/deploy
- public Japanese URL QA

---

# 6. What is NOT required

Do not:
- re-audit all 58 pages
- rewrite all Japanese copy
- change English approved copy
- change recommendation hierarchy
- change hotel/area ordering
- change facts or numbers
- change affiliate URLs/tracking
- change common header/navigation/footer/common.js/style.css without explicit approval

---

# 7. Final state

> **Japanese direct audit is complete.**

> **58 pages audited = 33 PASS + 25 FIX.**

> **The Japanese site is structurally sound overall.**

> **Correction is targeted: 19 wording-only pages + 6 source-position/head-repair pages.**

Next active task:
**prepare the Japanese correction set for the 25 FIX pages only.**
