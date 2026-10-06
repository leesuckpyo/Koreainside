# Korea Inside — Japanese Heading Correction Approved Copy

**Date:** 2026-10-07  
**Market:** Japan / Japanese (`ja`)  
**Status:** APPROVED CORRECTION COPY — CORRECTED FINAL  
**Supersedes:** earlier same-date heading correction draft with incorrect H1/H2 summary counts  
**Production baseline:** `c386710dc99d12282aedcfc8e861d59da8c2f81d`

## 0. Correct Scope

This correction does **not** reopen the 58 Japanese pages for general rewriting.

### Approved heading mapping total
- Total approved mappings: **29**
- H1 mappings: **5**
- H2 mappings: **24**

### Already satisfied in current Production
The following **4 H1 mappings are already in NEW state** on Production and are **verification-only**.
Do not edit them again.

1. `ja/accommodation.html`
2. `ja/airport-transfer.html`
3. `ja/dongdaemun-travel-guide.html`
4. `ja/k-beauty.html`

### Actual implementation scope
- Files to modify: **13**
- Actual replacements to apply: **25**
- H1 replacements to apply: **1**
- H2 replacements to apply: **24**

### Protected
Do not change:
- any heading not listed below
- Title / Meta / OG
- body copy
- FAQ wording
- JSON-LD
- canonical / hreflang / sitemap
- affiliate URLs / tracking
- images / srcset
- CSS / JS
- common header / navigation / footer
- `common.js`
- shared `style.css`
- any other language
- existing user working-tree changes

Preserve all HTML structure, section order, class, id, data-* attributes, and existing `data-guide-year="current"` behavior.

---

# 1. Verification-Only — Already Correct in Production

These 4 mappings remain part of the approved 29-heading set, but current Production already matches NEW.

## 1.1 `ja/accommodation.html`

EXPECTED CURRENT H1:
`ソウルでどこに泊まる？（2026）`

Verification:
- current visible H1 must match the approved NEW form
- preserve current-year automation
- **no edit if already matched**

## 1.2 `ja/airport-transfer.html`

EXPECTED CURRENT H1:
`仁川空港からソウル市内へ：どの移動手段が合う？`

Verification:
- **no edit if already matched**

## 1.3 `ja/dongdaemun-travel-guide.html`

EXPECTED CURRENT H1:
`ソウル・東大門（トンデムン）観光ガイド 2026：DDP・市場・ナイトショッピング`

Verification:
- preserve current-year marker if present
- **no edit if already matched**

## 1.4 `ja/k-beauty.html`

EXPECTED CURRENT H1:
`韓国Kビューティーガイド：買い物・美容体験・ソウルのエリア選び`

Verification:
- **no edit if already matched**

---

# 2. Actual Implementation — 13 Files / 25 Replacements

## 2.1 `ja/apps.html`

### H2

OLD:
`自国のアカウントが使えるうちに壊れやすい設定を終える`

NEW:
`日本のアカウントが使えるうちに、つまずきやすい設定を済ませる`

---

## 2.2 `ja/best-area-for-first-time-visitors-seoul.html`

### H2-1

OLD:
`初回旅行向けソウル宿泊エリア：ひと目で比較`

NEW:
`初めてのソウル旅行向け宿泊エリア：ひと目で比較`

### H2-2

OLD:
`初めてホテルを予約する前に確認したいこと`

NEW:
`初めてのソウル旅行でホテルを予約する前に確認したいこと`

### H2-3

OLD:
`初回旅行でよくあるホテル予約ミス`

NEW:
`初めてのソウル旅行でよくあるホテル予約ミス`

### H2-4

OLD:
`初回ソウル旅行のホテルを比較する`

NEW:
`初めてのソウル旅行でホテルを比較する`

### H2-5

OLD:
`初回旅行なら結局どこ？`

NEW:
`初めてのソウル旅行なら結局どこ？`

---

## 2.3 `ja/best-area-for-nightlife-seoul.html`

### H2

OLD:
`翌朝も気に入れる場所が、ナイトライフの良い拠点`

NEW:
`翌朝も過ごしやすい場所が、ナイトライフの良い拠点`

---

## 2.4 `ja/best-area-for-shopping-seoul.html`

### H2

OLD:
`買い物袋を持ってソウルを横断しないための買い物日のまとめ方`

NEW:
`買い物袋を持ってソウルを横断しないための、買い物日の組み方`

---

## 2.5 `ja/gongdeok-mapo-seoul-guide.html`

### H1 — Page-role correction

ACTUAL OLD IN CURRENT PRODUCTION:
`孔徳（コンドク）・麻浦（マポ）ガイド 2026：ローカルグルメ・市場・空港アクセス`

NEW:
`孔徳（コンドク）・麻浦（マポ）ガイド 2026：ローカルグルメ・市場・夜の過ごし方`

Reason:
The English page role is `Food, Markets & Local Evenings`. `空港アクセス` shifted the H1 away from the actual page role.

Implementation:
- preserve the existing year automation marker if present
- change only `空港アクセス` → `夜の過ごし方`
- do not alter surrounding H1 structure

### H2-1

OLD:
`昔ながらの麻浦グルメを使う`

NEW:
`昔ながらの麻浦グルメを味わう`

### H2-2

OLD:
`孔徳と麻浦は一方向の夜にすると使いやすい`

NEW:
`孔徳と麻浦は、夜に一方向で巡ると使いやすい`

---

## 2.6 `ja/hotels-near-seoul-station.html`

### H2

OLD:
`ソウル駅が本当に向いているのは？`

NEW:
`ソウル駅周辺が本当に向いているのは誰？`

---

## 2.7 `ja/itaewon-travel-guide.html`

### H2-1

OLD:
`梨泰院では「理由のある食事」をする`

NEW:
`梨泰院だからこそ食べたい一食を選ぶ`

### H2-2

OLD:
`自分が本当に欲しい梨泰院の夜を選ぶ`

NEW:
`自分に合う梨泰院の夜の過ごし方を選ぶ`

---

## 2.8 `ja/jamsil-travel-guide.html`

### H2-1

OLD:
`まず「どの蚕室の日」にするか決める`

NEW:
`まず蚕室で何をする日か決める`

### H2-2

OLD:
`ルート1：ロッテワールドに一日を渡す`

NEW:
`ルート1：一日をロッテワールドに使う`

### H2-3

OLD:
`「Lotteの中」から出たくなったらソンリダンギル`

NEW:
`ロッテエリアを離れたくなったらソンリダンギルへ`

### H2-4

OLD:
`蚕室総合運動場は別の支線`

NEW:
`蚕室総合運動場は別ルートで考える`

---

## 2.9 `ja/korean-online-payments-foreigners.html`

### H2

OLD:
`決済トラブルの出発点が別の場所にあることもある`

NEW:
`決済トラブルの原因が別の場所にあることもある`

---

## 2.10 `ja/lotte-world-seoul.html`

### H2-1

OLD:
`ライド優先戦略：あとで取り戻しにくいものを先に守る`

NEW:
`ライド優先戦略：あとで乗るのが難しくなるライドを先に優先する`

### H2-2

OLD:
`Magic Pass 2026：価値はある？どのtierを買う？`

NEW:
`Magic Pass 2026：価値はある？どのタイプを買う？`

### H2-3

OLD:
`フルのロッテワールド日に一緒に入れないほうがいいもの`

NEW:
`ロッテワールドに一日使う日に、詰め込まないほうがいい予定`

---

## 2.11 `ja/myeongdong-travel-guide.html`

### H2

OLD:
`ホテルになると、どの駅に近いかがさらに重要です。`

NEW:
`明洞に泊まるなら、どの駅に近いかがさらに重要です。`

---

## 2.12 `ja/seongsu-travel-guide.html`

### H2

OLD:
`泊まるなら、どの駅に近いかの意味が変わります。`

NEW:
`聖水に泊まるなら、最寄り駅の選び方が変わります。`

---

## 2.13 `ja/where-to-stay-in-seongsu.html`

### H2

OLD:
`聖水に泊まる？それとも日帰りで行く？`

NEW:
`聖水に泊まる？それとも観光だけ？`

---

# 3. Final Scope Summary

## Approved mapping set
- Total approved mappings: **29**
- H1 mappings: **5**
- H2 mappings: **24**

## Verification-only already satisfied
- Files: **4**
- H1 mappings already NEW: **4**
- Expected edits: **0**

Verification-only files:
1. `ja/accommodation.html`
2. `ja/airport-transfer.html`
3. `ja/dongdaemun-travel-guide.html`
4. `ja/k-beauty.html`

## Actual implementation
- Files to modify: **13**
- Actual replacements: **25**
- H1 replacements: **1**
- H2 replacements: **24**

Actual modified-file manifest:
1. `ja/apps.html`
2. `ja/best-area-for-first-time-visitors-seoul.html`
3. `ja/best-area-for-nightlife-seoul.html`
4. `ja/best-area-for-shopping-seoul.html`
5. `ja/gongdeok-mapo-seoul-guide.html`
6. `ja/hotels-near-seoul-station.html`
7. `ja/itaewon-travel-guide.html`
8. `ja/jamsil-travel-guide.html`
9. `ja/korean-online-payments-foreigners.html`
10. `ja/lotte-world-seoul.html`
11. `ja/myeongdong-travel-guide.html`
12. `ja/seongsu-travel-guide.html`
13. `ja/where-to-stay-in-seongsu.html`

---

# 4. Required Local QA

## Mapping
- approved mappings verified: **29/29**
- verification-only H1 NEW state: **4/4**
- actual replacements applied: **25/25**
- actual H1 replacement: **1/1**
- actual H2 replacements: **24/24**
- pending OLD strings after implementation: **0**

## Scope
- modified files: **exactly 13**
- verification-only files modified: **0**
- out-of-scope modified files: **0**

## Structure
- Japanese HTML inventory: **58**
- H1 exactly one per page: **58/58**
- H2 counts unchanged from Production
- tag order drift: **0**
- class / id / data-* drift: **0**

## Protected content
- body copy change: **0**
- Title / Meta / OG change: **0**
- FAQ / JSON-LD change: **0**
- facts / numbers / dates change: **0**
- recommendation / ranking change: **0**
- affiliate / tracking change: **0**
- image / srcset change: **0**
- common files change: **0**
- other languages change: **0**

## Technical
- `git diff --check`: **PASS**
- staged files: **0**
- commit: **0**
- push: **0**
- Production: **0**

Final local verdict:
`JAPANESE HEADING HOTFIX — LOCAL QA PASS`

If any mapping, file count, or current Production source differs from this corrected final document, STOP and report without guessing.
