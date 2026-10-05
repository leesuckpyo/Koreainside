# Korea Inside — Thai SEO Batch 3 Structure & Source-Coverage Review

**Date:** 2026-10-04  
**Status:** SEO / LOCALIZATION DIRECTION REVIEW — NOT CONTENT LOCKED / NOT FOR PRODUCTION  
**Batch:** Thai SEO Batch 3 — 5 pages  
**Purpose:** Lock source structure, Thai search-facing direction, hierarchy delta and complete visible-string coverage before full Thai localization.

---

# 0. Batch 3 pages

1. `jamsil-travel-guide.html`
2. `where-to-stay-in-jamsil.html`
3. `seongsu-travel-guide.html`
4. `where-to-stay-in-seongsu.html`
5. `gongdeok-mapo-seoul-guide.html`

Target Thai pages:

1. `th/jamsil-travel-guide.html`
2. `th/where-to-stay-in-jamsil.html`
3. `th/seongsu-travel-guide.html`
4. `th/where-to-stay-in-seongsu.html`
5. `th/gongdeok-mapo-seoul-guide.html`

**Internal-link rule for this phase:** new Thai contextual internal links = **0**.  
Hub↔Detail reciprocal closure remains deferred until the planned Thai page set is complete.

---

# 1. Source lock and coverage method

Batch 2 exposed a weakness in counting only normal prose tags. Batch 3 therefore uses a wider source-coverage pass before localization.

The localization source inventory must cover:

- `h1`–`h4`
- `p`
- `li`
- `th` / `td`
- `dt` / `dd`
- `summary`
- `figcaption`
- standalone CTA / anchor text
- `small`
- `button`
- `legend`
- page-specific `aria-label`
- page-specific image `alt`
- page-specific user-visible JSON-LD text
- page-specific inline-JavaScript strings if any are rendered into the UI

Machine values remain protected and are not translated merely because they appear in HTML.

---

# 2. Page 1 — Jamsil Travel Guide

**File:** `jamsil-travel-guide.html`  
**Target:** `th/jamsil-travel-guide.html`  
**English source blob SHA:** `fb53eb3c9e9a7e014b4e30735acf8133aa47a32d`

## 2.1 Source structure

- MAIN visible units: **390**
- H1: **1**
- H2: **23**
- H3: **37**
- H4: **0**
- visible FAQ: **8**
- FAQPage JSON-LD questions: **8**
- standalone affiliate disclosures: **4**
- page-specific image alt targets: **8**
- meaningful standalone anchor/CTA text targets: **8**
- page-specific ARIA targets identified in main: **0**

The four affiliate disclosures use:

`Affiliate link — Korea Inside may earn a commission at no extra cost to you.`

These must be explicitly mapped in the Thai Review Copy and must not be left for Codex to infer.

## 2.2 Thai SEO direction

Primary Thai intent:

- `เที่ยวจัมซิล`
- `จัมซิล โซล`
- `Lotte World`
- `Seoul Sky`
- `ทะเลสาบซอกชน`

Supporting intent:

- `Songridan-gil`
- `Olympic Park`
- คอนเสิร์ต
- เบสบอล
- พักจัมซิล
- โรงแรมใกล้ Lotte World

The page should remain a **decision-first Jamsil area guide**, not become a generic attraction list.

Core decision model to preserve:

- If Lotte World is the reason for coming, give it the day.
- If the theme park is not the priority, Jamsil can work as a lake / Seoul Sky / Songridan-gil / event day.
- Jamsil Station is not the correct default for every sports or concert venue.
- Weather and visibility should change the plan.
- Staying in Jamsil makes more sense when eastern-Seoul attractions, children, concerts or sports repeat across several days.

## 2.3 Proposed Thai search-facing copy

### `<title>`

`เที่ยวจัมซิล 2026: Lotte World, Seoul Sky + ทะเลสาบซอกชน | Korea Inside`

### Meta description

`เที่ยวจัมซิลแบบเลือกเส้นทางให้ถูก วางแผน Lotte World, Seoul Sky, ทะเลสาบซอกชน, Songridan-gil, Olympic Park รวมคอนเสิร์ตและเบสบอล`

### H1

`เที่ยวจัมซิล 2026: Lotte World, Seoul Sky และทะเลสาบซอกชน`

## 2.4 Heading hierarchy decision

Current H2: **23**

Keep the travel-guide structure, but demote only the three hotel/stay bridge headings:

1. `The station side matters more when you sleep here.`
2. `For a late concert, the trip back can matter more than the hotel brand.`
3. `If Jamsil belongs in several days of the trip, compare the hotel location before you book.`

Their visible copy and existing links remain.

Expected substantive editorial H2 after restructuring: **20**

No body deletion.  
No route-order change.  
No recommendation-strength change.

## 2.5 Existing link rule

Preserve existing targets only.

Existing page relationships already include the Jamsil stay sibling and current affiliate ticket / spa targets.

Do not add Lotte World / Seoul Sky / Accommodation hub reciprocal links yet. That belongs to final Thai internal-link closure.

---

# 3. Page 2 — Where to Stay in Jamsil

**File:** `where-to-stay-in-jamsil.html`  
**Target:** `th/where-to-stay-in-jamsil.html`  
**English source blob SHA:** `266377cf3574d8d14a9ed17d9357176a33497f68`

## 3.1 Source structure

- MAIN visible units: **129**
- H1: **1**
- H2: **6**
- H3: **14**
- visible FAQ: **7**
- FAQPage JSON-LD: **none**
- hotels: **7**
- affiliate links: **21**
- booking-strip ARIA labels: **28**
- page-specific image alts: **0**

Hotels, source order:

1. Lotte Hotel World
2. Sofitel Ambassador Seoul Hotel & Serviced Residences
3. Rosana Hotel
4. Delight Hotel Jamsil
5. Seoul Sangju Hotel
6. Hotel Lake
7. SIGNIEL Seoul

Every booking-strip `aria-label` must be explicitly localized in the Review Copy before implementation.

## 3.2 Thai SEO direction

Primary Thai intent:

- `พักจัมซิลไหนดี`
- `โรงแรมจัมซิล`
- `โรงแรมใกล้ Lotte World`
- `ที่พักใกล้ทะเลสาบซอกชน`

The page must remain **area/location decision first**, not a hotel ranking.

Core decision model:

- Jamsil Station / Lotte World side for repeated Lotte access.
- Seokchon Lake side for lake / room / serviced-residence value.
- Sports Complex side for baseball and some concert trips.
- Exact airport-bus stop, final walk, room configuration and occupancy matter more than a generic “near Jamsil” label.
- A Jamsil hotel is not automatically the best base for the whole Seoul trip.

## 3.3 Proposed Thai search-facing copy

### `<title>`

`พักจัมซิลไหนดี 2026: โรงแรมใกล้ Lotte World + ทะเลสาบซอกชน | Korea Inside`

### Meta description

`เลือกที่พักจัมซิลสำหรับ Lotte World, ทะเลสาบซอกชน, เบสบอลหรือคอนเสิร์ต เปรียบเทียบทำเล ห้องพัก กระเป๋า และการเดินทางจากสนามบินอินชอน`

### H1

`พักจัมซิลไหนดี 2026: เลือกโรงแรมให้เหมาะกับ Lotte World และแผนเที่ยว`

## 3.4 Heading hierarchy decision

Current H2: **6**

Keep all six H2s.

No hierarchy rewrite is necessary.

## 3.5 Protected hotel logic

Preserve exact:

- hotel order
- room sizes
- bed configuration
- permitted occupancy
- genuine triple/family distinctions
- luggage handling/storage conditions
- exact station/side logic
- Airport Bus 6705A / 6006 distinctions where present
- Lotte World vs Seokchon Lake vs Sports Complex decision logic
- affiliate URLs and tracking

Affiliate URL count must remain **21/21**.

New internal links in this phase: **0**

---

# 4. Page 3 — Seongsu Travel Guide

**File:** `seongsu-travel-guide.html`  
**Target:** `th/seongsu-travel-guide.html`  
**English source blob SHA:** `e170a14a132207b7d3516320939171cbd1bc56e6`

## 4.1 Source structure

- MAIN visible units: **598**
- H1: **1**
- H2: **17**
- H3: **63**
- H4: **2**
- visible FAQ: **0**
- FAQPage JSON-LD: **none**
- standalone affiliate disclosures: **3**
- page-specific image alt targets: **6**
- meaningful standalone anchor/CTA text targets: **7**
- page-specific ARIA targets identified in main: **0**

The three affiliate disclosures must be explicitly mapped.

## 4.2 Thai SEO direction

Primary Thai intent:

- `เที่ยวซองซู`
- `ซองซู โซล`
- `ซองซู คาเฟ่`
- `ซองซู ป๊อปอัพ`
- `ช้อปปิ้งซองซู`

Supporting intent:

- `Seoul Forest`
- `Seongsu Station`
- `Yeonmujang-gil`
- `K-Beauty`
- ร้านแฟลกชิป
- คาเฟ่
- แฟชั่น
- personal color
- perfume experience

Thai search results strongly associate Seongsu with cafés, pop-ups, fashion, beauty and the creative/industrial neighborhood identity. The page should answer **how to build a route without chasing every trend**.

Core Korea Inside difference:

**Build the day around permanent neighborhood value and one or two real interests; use current pop-ups to modify the route, not to control it.**

## 4.3 Proposed Thai search-facing copy

### `<title>`

`เที่ยวซองซู 2026: คาเฟ่ ป๊อปอัพ ช้อปปิ้ง K-Beauty + Seoul Forest | Korea Inside`

### Meta description

`เที่ยวซองซูแบบไม่ไล่ตามทุกกระแส เลือก Seongsu Station หรือ Seoul Forest วางเส้นทางคาเฟ่ ป๊อปอัพ แฟชั่น K-Beauty และร้านถาวรที่คุ้มเวลา`

### H1

`เที่ยวซองซู 2026: ป๊อปอัพ คาเฟ่ ช้อปปิ้ง K-Beauty และ Seoul Forest`

## 4.4 Heading hierarchy decision

Current H2: **17**

Demote only these three hotel/stay bridge headings:

1. `The station matters differently when you sleep here.`
2. `A nearby hotel matters more when Seongsu is part of several days, not one afternoon.`
3. `Staying here only makes sense if the location improves the rest of your Seoul trip.`

Visible copy and links remain.

Expected substantive editorial H2 after restructuring: **14**

The Current Layer remains an H2 and is refreshed rather than removed.

## 4.5 October 2026 CURRENT replacement

The English source still contains:

`What's Current in Seongsu: September 2026`

and September-only items.

These must not become active Thai October copy.

### Official current item 1 — Seoul International Garden Show

Keep and refresh:

- 2026 Seoul International Garden Show
- Seoul Forest / Seongsu
- runs through **October 27, 2026**
- this belongs in the Current Layer, not the evergreen route

Official Seoul sources still confirm the event and the October 27 end date.

### Official current item 2 — Creative X Seongsu

Add to the October Current Layer:

- `2026 Creative X Seongsu`
- **October 5–11, 2026**
- venues across S-Factory, PUBG Seongsu, Seoul Forest, Yeonmujang-gil and Seongsu-dong
- program times vary
- use only as an optional current event when the traveler’s exact dates match

This is supported by the Seoul Culture Portal.

### Remove / supersede from active October copy

- Spotify House Seoul — September 10–13
- Ma:nyo Soybean Mill pop-up — September 9–21
- BYREDO FUTURE MEMORIES Seoul — September 11–20
- September checked-date wording

### Preserve evergreen current-layer rule

Keep the editorial rule:

- check exact dates
- care about the brand / artist / experience first
- do not join a queue just because a queue exists
- one meaningful pop-up is enough
- permanent Seongsu value must survive after the temporary event closes

No unverified October pop-up should be hard-coded simply to make the list longer.

## 4.6 Existing link rule

Existing stay links and three affiliate experience links are preserved.

No new Thai reciprocal link is added in this phase.

---

# 5. Page 4 — Where to Stay in Seongsu

**File:** `where-to-stay-in-seongsu.html`  
**Target:** `th/where-to-stay-in-seongsu.html`  
**English source blob SHA:** `e0d8619a7ec6786abe46ebf7ae93b6088a67bddb`

## 5.1 Source structure

- MAIN visible units: **89**
- H1: **1**
- H2: **7**
- H3: **10**
- visible FAQ: **5**
- FAQPage JSON-LD: **none**
- accommodation products: **4**
- affiliate links: **11**
- booking-strip ARIA labels: **15**
- page-specific image alts: **0**

Accommodation order:

1. Hotel POCO Seongsu
2. ONJAE STAY SEONGSU
3. Stay BUT SEONGSU
4. Seoul Forest Stay

The 15 booking `aria-label` values must be explicitly localized in the Review Copy.

## 5.2 Thai SEO direction

Primary Thai intent:

- `พักซองซูไหนดี`
- `ที่พักซองซู`
- `โรงแรมซองซู`
- `Seongsu Station`
- `Seoul Forest`

Core decision model:

- Seongsu Station / Yeonmujang-gil when the commercial core is the priority.
- Ttukseom when location trade-offs work better.
- Seoul Forest side for park access and a different neighborhood feel.
- Do not move hotels merely to spend one afternoon in Seongsu.
- Small stays require closer scrutiny of stairs, bathrooms, check-in, beds and permitted occupancy.
- A nearby room does not guarantee entry to a temporary pop-up.
- Luggage storage and the final airport day must be checked rather than assumed.

## 5.3 Proposed Thai search-facing copy

### `<title>`

`พักซองซูไหนดี 2026: Seongsu Station หรือ Seoul Forest | Korea Inside`

### Meta description

`ตัดสินใจว่าควรพักซองซูหรือมาเที่ยวแบบไปกลับ เปรียบเทียบที่พักใกล้ Seongsu Station, Ttukseom และ Seoul Forest พร้อมห้องพัก กระเป๋า และสนามบิน`

### H1

`พักซองซูไหนดี 2026: Seongsu Station, Ttukseom หรือ Seoul Forest`

## 5.4 Heading hierarchy decision

Current H2: **7**

Keep all seven H2s.

No hierarchy rewrite.

## 5.5 Affiliate / accessibility lock

Affiliate links must remain **11/11 exact**.

Booking ARIA must be explicitly mapped **15/15** before Codex implementation.

New internal links in this phase: **0**

---

# 6. Page 5 — Gongdeok & Mapo Seoul Guide

**File:** `gongdeok-mapo-seoul-guide.html`  
**Target:** `th/gongdeok-mapo-seoul-guide.html`  
**English source blob SHA:** `0ebc580e927a3e2d6c8d436f75b60ff5cef905ed`

## 6.1 Source structure

- MAIN visible units: **430**
- H1: **1**
- H2: **19**
- H3: **53**
- H4: **0**
- visible FAQ: **0**
- FAQPage JSON-LD: **none**
- standalone affiliate disclosures: **5**
- page-specific image alt targets: **4**
- meaningful standalone anchor/CTA text targets: **9**
- page-specific ARIA targets identified in main: **0**

The five affiliate disclosures must be explicitly mapped.

## 6.2 Thai SEO direction

Primary Thai intent:

- `เที่ยวกงด็อก`
- `กงด็อก โซล`
- `มาโป โซล`
- `ตลาดกงด็อก`
- `ของกินมาโป`

Supporting intent:

- Gongdeok Market
- Mapo pork galbi
- makgeolli
- Korean dessert
- head spa / massage
- Gyeongui Line Forest Park
- AREX
- พักกงด็อก

Thai-language web results use the spelling **กงด็อก** for Gongdeok, including Thai pages for businesses and station-area accommodation.

This page should not pretend Gongdeok/Mapo is a must-see sightseeing district.

Core editorial position:

**Come because food, a booked experience, airport-connected convenience or a calmer local evening already gives you a reason. Do not manufacture a checklist of attractions.**

## 6.3 Proposed Thai search-facing copy

### `<title>`

`เที่ยวกงด็อก–มาโป 2026: ตลาด ของกินเกาหลี + ค่ำคืนโลคัล | Korea Inside`

### Meta description

`เที่ยวกงด็อกและมาโปแบบเน้นของกิน เริ่ม Gongdeok Market ต่อหมูย่างมาโป ประสบการณ์อาหาร สปา และค่ำคืนสบายๆ พร้อมคำแนะนำว่าควรพักย่านนี้ไหม`

### H1

`เที่ยวกงด็อก–มาโป 2026: ตลาด อาหารท้องถิ่น และค่ำคืนแบบโซล`

## 6.4 Heading hierarchy decision

Current H2: **19**

SEO audit classification: **P2 / KEEP**

No structural rewrite is required at this stage.

Therefore:

- H2 remains **19**
- do not merge `Final Recommendation`
- do not demote the stay-decision sections
- do not reorder the route
- no body deletion

The page already has a coherent food / experience / stay / route decision structure. Optimization should concentrate on Thai search-facing copy and natural localization, not needless hierarchy churn.

## 6.5 Existing links

Existing contextual links include first-time stay decision, accommodation hub and solo traveler decision pages.

Preserve current targets.

Do not add the future Gongdeok hotel-detail reciprocal link yet.

New Thai internal links in this phase: **0**

---

# 7. Batch 3 structure decision summary

| Page | Audit priority | Current H2 | Approved direction for Review draft | Expected substantive H2 |
|---|---|---:|---|---:|
| Jamsil Travel Guide | P1 | 23 | demote 3 stay bridges | 20 |
| Where to Stay Jamsil | P0 | 6 | keep structure | 6 |
| Seongsu Travel Guide | P1 | 17 | demote 3 stay bridges + October Current refresh | 14 |
| Where to Stay Seongsu | P0 | 7 | keep structure | 7 |
| Gongdeok & Mapo | P2 | 19 | KEEP; search-facing localization only | 19 |

The P0 classification on the two Stay Detail pages is primarily an **internal-link architecture issue**, not a content-depth problem. Because final Thai reciprocal linking is intentionally deferred, no new link is added during this localization batch.

---

# 8. Current-research notes used for this review

## Seongsu

Current official Seoul information confirms:

- 2026 Seoul International Garden Show spans Seoul Forest / Seongsu and runs through **October 27, 2026**.
- 2026 Creative X Seongsu runs **October 5–11, 2026** across multiple Seongsu venues.

Current Thai/English search results also reinforce that Seongsu search intent centers on:

- cafés
- pop-ups
- shopping
- K-beauty
- Seoul Forest
- rapidly changing temporary events

Third-party current listings were used only as supporting market observation; the proposed hard-coded October Current Layer above uses official Seoul sources.

## Jamsil

Current official Seoul tourism still presents Seoul Sky as a major Jamsil attraction and continues to emphasize current operating information. This supports the existing source rule to check visibility and current schedules rather than hard-code a fixed decision regardless of weather/date.

## Thai naming

Current Thai pages/search results support:

- `จัมซิล` — Jamsil
- `ซองซู` — Seongsu
- `กงด็อก` — Gongdeok

These forms are used as the default Thai search-facing spellings in Batch 3.

---

# 9. Full-localization production rule for the next step

The next ChatGPT step is to create **five full Thai Localized Review Copy MDs**.

Before any page is sent to Codex, each Review Copy must explicitly cover all page-specific user-visible sources, including the categories that caused Batch 2 STOPs:

- ordinary MAIN source units
- standalone affiliate disclosure strings
- CTA/anchor text
- image alt
- booking ARIA
- visible FAQ
- FAQ schema when present
- Current Layer replacement/disposition
- any other page-specific rendered string found in HTML

Expected special coverage:

- Jamsil: MAIN 390 + affiliate disclosures 4 + image alts 8 + CTA/anchor text + FAQ/schema parity
- Jamsil Stay: MAIN 129 + booking ARIA 28 + affiliate 21 exact
- Seongsu: MAIN 598 + affiliate disclosures 3 + image alts 6 + CTA/anchor text + October CURRENT replacement
- Seongsu Stay: MAIN 89 + booking ARIA 15 + affiliate 11 exact
- Gongdeok/Mapo: MAIN 430 + affiliate disclosures 5 + image alts 4 + CTA/anchor text

No HTML / Git / Production action is authorized by this Review document.

---

**THAI SEO BATCH 3 — SOURCE LOCK & STRUCTURE REVIEW COMPLETE**
