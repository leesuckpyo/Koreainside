# Korea Inside — Thai Stay / Area SEO Closure Decision Lock

**Date:** 2026-10-04  
**Status:** SEO CLOSURE DIRECTION LOCKED — IMPLEMENTATION NOT YET APPLIED  
**Branch:** `th-localization-2026-10-03`  
**Baseline HEAD:** `09d45073528a046db657d539df81ed16373e060d`  
**Purpose:** Apply the Thai competitor/SERP findings immediately to the already-localized Stay / Area cluster so the fixes are not deferred or forgotten.

---

# 1. Decision

The Thai competitor/SERP analysis does **not** justify a broad content rewrite.

Current Thai Title / Meta / H1 direction is already decision-first and natural enough across the existing Stay / Area set.

The immediate SEO gap is structural:

1. Thai contextual links still route to English-root pages even when the Thai sibling already exists.
2. Some older Thai pages still use inconsistent Jamsil / Mapo Thai spellings.
3. `hongdae-vs-myeongdong.html` has a Hongdae body guide link but lacks the equivalent Myeongdong Travel Guide body link.
4. Existing Thai pages should distribute relevance through Hub → Decision → Area → Stay paths before Batch 4 begins.

Therefore the closure is **link + spelling + one missing contextual link**, not a body rewrite.

---

# 2. Scope

Audit population:

- current 13 Thai working pages
- Thai Batch 1
- Thai Batch 2
- Thai Batch 3

Current Thai HTML population audited: **28 pages**

Pages with a current closure change identified: **21 pages**

No change is required merely to make all 28 files different.

---

# 3. Canonical Thai spelling normalization

## Jamsil

Canonical:

`จัมซิล`

Replace user-visible old variants:

- `จัมชิล`
- `ชัมชิล`

with:

`จัมซิล`

Apply only to user-facing Thai strings where they refer to Jamsil:

- body
- heading
- CTA / anchor text
- breadcrumb
- alt / ARIA
- visible table / labels
- title / meta / H1 if present

Do not change machine identifiers, URLs, hotel/brand official names, tracking values, classes, ids or data-* merely because they contain English `jamsil`.

Known affected pages include:

- `th/accommodation.html`
- `th/best-area-for-airport-access-seoul.html`
- `th/best-area-for-couples-seoul.html`
- `th/best-area-for-families-seoul.html`
- `th/best-area-for-first-time-visitors-seoul.html`
- `th/best-area-for-luxury-hotels-seoul.html`
- `th/best-area-for-shopping-seoul.html`
- `th/gangnam-travel-guide.html`
- `th/where-to-stay-in-myeongdong.html`

Final visible old-variant target:

- `จัมชิล` = 0
- `ชัมชิล` = 0

## Mapo

Canonical:

`มาโป`

Replace user-visible old variant:

`มาโพ`

with:

`มาโป`

Known affected pages include:

- `th/accommodation.html`
- `th/best-area-for-airport-access-seoul.html`
- `th/best-area-for-budget-travelers-seoul.html`
- `th/best-area-for-couples-seoul.html`
- `th/best-area-for-families-seoul.html`
- `th/best-area-for-first-time-visitors-seoul.html`
- `th/best-area-for-nightlife-seoul.html`
- `th/best-area-for-solo-travelers-seoul.html`

Final visible old-variant target:

`มาโพ` = 0

---

# 4. Thai contextual-link closure rule

For user-facing contextual links inside `<main>`:

> If the target Thai sibling already exists on the current Thai feature branch, the Thai page must link to the Thai sibling, not back to the English root.

Examples of incorrect current patterns:

- `../where-to-stay-in-jamsil.html`
- `../where-to-stay-in-hongdae.html`
- `/where-to-stay-in-myeongdong.html`
- `/best-area-for-first-time-visitors-seoul.html`

when the corresponding `th/...` target already exists.

Preferred same-language target:

- same-directory relative target such as `where-to-stay-in-jamsil.html`
- preserve existing `#fragment` exactly when applicable

Do not convert:

- image paths such as `../images/...`
- future Thai pages that do not exist yet
- common header/navigation/footer links in this closure
- external / affiliate links

Do not add links just to increase link count.

---

# 5. Hub / Decision pages requiring link closure

## `th/accommodation.html`

This is the highest-priority closure page.

Convert contextual body links to already-existing Thai siblings for:

- Hongdae vs Myeongdong
- first-time visitors
- families
- solo travelers
- couples
- budget travelers
- shopping
- nightlife
- luxury hotels
- Where to Stay Hongdae
- Where to Stay Myeongdong
- Where to Stay Gangnam
- Where to Stay Insadong
- Where to Stay Dongdaemun
- Where to Stay Jamsil
- Where to Stay Seongsu
- Where to Stay Itaewon

Preserve English targets for Thai pages that do **not** yet exist, including future station / transport detail pages.

No body rewrite.

## `th/best-area-for-first-time-visitors-seoul.html`

Convert contextual body links to existing Thai siblings, including:

- Myeongdong Stay
- Hongdae Stay
- Insadong Stay
- Jamsil Stay
- Gangnam Stay
- Dongdaemun Stay
- Hongdae vs Myeongdong
- Shopping
- Families
- Budget
- Solo

Preserve recommendation meaning and anchor wording except canonical spelling correction.

## Other Stay Decision pages

Where an existing Thai sibling is currently reached through `../...` English-root routing, convert the target to the Thai sibling.

Pages include:

- `best-area-for-airport-access-seoul.html`
- `best-area-for-budget-travelers-seoul.html`
- `best-area-for-couples-seoul.html`
- `best-area-for-families-seoul.html`
- `best-area-for-luxury-hotels-seoul.html`
- `best-area-for-nightlife-seoul.html`
- `best-area-for-shopping-seoul.html`
- `best-area-for-solo-travelers-seoul.html`

Do not alter recommendation order, hotel/area judgment or body structure.

---

# 6. Area ↔ Stay reciprocal closure

These eight pairs must resolve Thai-to-Thai in contextual body links:

1. Gangnam Guide ↔ Where to Stay in Gangnam
2. Hongdae Guide ↔ Where to Stay in Hongdae
3. Insadong Guide ↔ Where to Stay in Insadong
4. Itaewon Guide ↔ Where to Stay in Itaewon
5. Myeongdong Guide ↔ Where to Stay in Myeongdong
6. Dongdaemun Guide ↔ Where to Stay in Dongdaemun
7. Jamsil Guide ↔ Where to Stay in Jamsil
8. Seongsu Guide ↔ Where to Stay in Seongsu

Current audit shows the Stay → Area side is generally already Thai-local.

The main remaining defect is that multiple Area → Stay links still use `../...`, which leaves `/th/` and opens the English root page.

Fix those contextual targets.

Where a natural link to `th/accommodation.html` already exists as an English-root target, close it to the Thai hub.

No common navigation changes.

---

# 7. `hongdae-vs-myeongdong.html`

Current body already has:

- Hongdae Travel Guide link
- Hongdae Stay link
- Myeongdong Stay link

Required closure:

1. Convert all existing body links to existing Thai siblings.
2. Add the missing natural Myeongdong Travel Guide body link in the Myeongdong decision section.

Approved anchor direction:

`อ่านคู่มือเที่ยวเมียงดง`

Target:

`myeongdong-travel-guide.html`

Do not turn this into a new content section.

Do not change the comparison recommendation.

---

# 8. Pages with direct Area / Stay English-root target corrections

Known pages:

- `th/dongdaemun-travel-guide.html`
- `th/gangnam-travel-guide.html`
- `th/gongdeok-mapo-seoul-guide.html`
- `th/hongdae-travel-guide.html`
- `th/hongdae-vs-myeongdong.html`
- `th/insadong-travel-guide.html`
- `th/itaewon-travel-guide.html`
- `th/jamsil-travel-guide.html`
- `th/myeongdong-travel-guide.html`
- `th/seongsu-travel-guide.html`

Preserve anchor meaning and section location.

Only change the target path unless the anchor contains an approved spelling correction.

---

# 9. No search-facing rewrite unless required by spelling consistency

Current Stay / Area Title / Meta / H1 direction is retained.

Examples already aligned with the Thai search model:

- `ที่พักโซล 2026: พักย่านไหนดี?`
- `เที่ยวโซลครั้งแรก พักย่านไหนดี?`
- `เที่ยวโซลคนเดียว พักย่านไหนดี?`
- `พักโซลกับเด็ก ย่านไหนดี?`
- `ฮงแด vs เมียงดง: พักย่านไหนดีในโซล?`

Do not reopen locked copy just to chase larger generic keywords.

Broad `เที่ยวเกาหลี` / `ที่เที่ยวเกาหลี` intent belongs to later Hub / Topic work, not Stay Detail pages.

---

# 10. Competitor-analysis application

Apply the research in these ways now:

- Hub → detail authority flow
- Thai-native decision language
- practical decision paths
- exact contextual links
- current-year cue only where already maintained
- affiliate CTA only at matched action points

Do not copy:

- social-hype titles
- OTA-style sales-first structure
- thin synonym pages
- unverified transport/hours/prices
- generic keyword stuffing

---

# 11. Protected

Do not modify in this closure:

- common header
- common navigation
- footer
- `common.js`
- shared `style.css`
- sitemap
- language switcher
- affiliate URLs / tracking
- hotel order
- recommendation order / strength
- room / bed / occupancy facts
- images / srcset
- schema structure except where a real technical defect is separately approved
- Batch 3 Seongsu event-status hotfix content

Do not touch main / Production.

---

# 12. QA gates

After implementation, verify:

1. user-visible `จัมชิล` = 0 across current Thai 28
2. user-visible `ชัมชิล` = 0 across current Thai 28
3. user-visible `มาโพ` = 0 across current Thai 28
4. every contextual `<main>` link whose Thai sibling exists stays inside `/th/`
5. no conversion of future/nonexistent Thai targets
6. Area ↔ Stay 8 pairs reciprocal Thai-to-Thai PASS
7. `hongdae-vs-myeongdong.html`:
   - Hongdae Guide Thai target PASS
   - Myeongdong Guide Thai target PASS
   - Hongdae Stay Thai target PASS
   - Myeongdong Stay Thai target PASS
8. affiliate href/tracking drift = 0
9. body recommendation drift = 0
10. common protected files changed = 0
11. main / Production changed = 0

---

# 13. Sequence lock

Current sequence:

`Batch 3 Seongsu event-status hotfix COMPLETE`
→ **Stay / Area SEO Closure NOW**
→ Batch 4 Airport Core
→ later clusters
→ 58/58 Final Integration

Do not defer this closure until the end.

**THAI STAY / AREA SEO CLOSURE — DIRECTION LOCKED**
