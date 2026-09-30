# Korea Inside — Japanese Airport Access Wording Correction Supplement

- File: `Korea_Inside_JA_Airport_Access_Wording_Correction_SUPPLEMENT_APPROVED_2026-09-29.md`
- Date: 2026-09-29
- Status: **APPROVED / CONTENT LOCKED FOR IMPLEMENTATION**
- User approval: **APPROVED 2026-09-29**
- Scope: `ja/best-area-for-airport-access-seoul.html` only
- Purpose: Resolve the single BLOCKED page from the 19-page wording-only implementation batch
- HTML implementation: **NOT STARTED**
- Git / stage / commit / push / deploy: **0**

## 1. Why this supplement exists

The previously approved wording correction required a visible FAQ correction and incorrectly stated that the same correction must also be mirrored into FAQPage JSON-LD.

Current source verification shows:

- visible FAQ: **exists**
- FAQPage JSON-LD: **does not exist**
- this page intentionally has visible FAQ without FAQPage schema
- adding FAQPage schema is **not approved**
- schema structure must remain unchanged

Therefore:

> Apply the approved wording correction to the visible FAQ only.  
> Do **not** add FAQPage JSON-LD.  
> Do **not** create or modify schema structure.

This supplement overrides only the FAQ-mirror instruction for this one page. All five previously approved wording corrections remain unchanged.

---

# 2. Exact implementation scope — 5 corrections

## 2.1 Seoul Station H2

Current:

`ソウル駅：空港以外の乗り換えも重なるときに強い`

Replace:

`ソウル駅：空港以外の乗り換えも重なるときに便利`

---

## 2.2 Seoul Station opening judgment

Current:

`空港移動と別の交通上の課題が重なると、ソウル駅が最も強い選択になります。`

Replace:

`空港移動と別の交通上の課題が重なると、ソウル駅が最も有力な候補になります。`

---

## 2.3 Myeongdong judgment

Current:

`明洞がより強い選択になるのは次のような場合です。`

Replace:

`明洞のほうが使いやすくなるのは次のような場合です。`

---

## 2.4 Visible FAQ — first question answer

Question:

`仁川空港アクセスが最も便利なソウルのエリアはどこですか？`

Within that visible FAQ answer, current:

`AREX直通列車、KTX、重い荷物を重視するならソウル駅が強いです。`

Replace:

`AREX直通列車、KTX、重い荷物を重視するならソウル駅が特に便利です。`

Keep the rest of that FAQ answer unchanged:

`孔徳はAREX一般列車の直通アクセスと落ち着いた夜を両立できます。実際のホテル近くに空港リムジンバスが停まるなら、明洞のほうが楽な場合もあります。`

### Important schema rule

- visible FAQ only
- FAQPage JSON-LD addition: **FORBIDDEN**
- Question count: unchanged
- existing schema blocks: unchanged

---

## 2.5 Visible FAQ — Hongdae vs Seoul Station answer

Question:

`空港アクセスなら弘大とソウル駅のどちらが便利ですか？`

Current:

`ソウル駅にはAREX直通列車とKTXがあるため、その後の鉄道移動に強いです。`

Replace:

`ソウル駅にはAREX直通列車とKTXがあるため、その後の鉄道移動にも便利です。`

Keep the following sentence unchanged:

`弘大はAREX一般列車が直通し、ホテル周辺にレストラン、カフェ、夜の活気がより多くあります。`

---

# 3. Protected wording

Do not alter the following contextual wording merely because it contains `強い`, `自動的`, `合理的`, or `意味`:

- `トレードオフ` labels or body wording not explicitly listed above
- `鉄道が直通だからといって空港からホテル入口までの移動が自動的に最も楽になるわけではない`
- `AREXが直通という理由だけで...自動的に弘大へ移すことはできません`
- taxi / transfer wording not explicitly listed above
- final-night decision wording not explicitly listed above

This supplement is not a new audit.

---

# 4. Technical protection

Do not change:

- facts
- numbers
- routes
- airport transport information
- hotel-area recommendation hierarchy
- section order
- FAQ count
- schema structure
- title/meta outside the listed H2
- class / id / data-*
- canonical / hreflang
- images / srcset
- affiliate / tracking
- CSS / JS
- common navigation/header/footer

---

# 5. Existing working-tree protection

Before implementation, there are already approved Japanese modifications from:

- 6-page Source-Position Correction
- 18 completed Wording-Only Correction pages

These existing diffs must remain untouched.

The airport-access file may appear as `.M` in `git status` while its content diff is currently zero due to stat/line-ending metadata. Do not use:

- `git restore`
- `git checkout --`
- `git reset`
- `git clean`

to normalize that state.

Apply only the five approved content changes above.

---

# 6. QA after implementation

Required checks:

1. `ja/best-area-for-airport-access-seoul.html` is the only newly changed content file.
2. All five OLD strings above are absent.
3. All five NEW strings above are present.
4. Visible FAQ still contains the same number of questions.
5. FAQPage JSON-LD remains absent.
6. Existing JSON-LD blocks, if any, parse successfully.
7. Facts / numbers / transport details unchanged.
8. canonical / hreflang unchanged.
9. affiliate / tracking / images / class / id unchanged.
10. Existing 24 approved Japanese content diffs remain unchanged.
11. `git diff --check` PASS.
12. stage / commit / push / Production remain unexecuted.

---

# 7. Completion state

After exact implementation and static QA:

> **Japanese 25 FIX pages implementation complete**
>
> - 6 source-position/head-repair pages: complete
> - 19 wording-only pages: complete
> - Airport-access visible FAQ exception: resolved
> - 33 PASS pages remain closed
> - no Production action yet

STOP after static QA and report to the user.
