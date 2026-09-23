# Korea Inside Maps + Apps 2 Spanish Localization Source - Batch 6

## Document metadata

- Date: 2026-09-23
- Purpose: Exact technical extraction of page-specific English strings for later approved Spanish localization.
- Scope: The two English Production/main source files `maps.html` and `apps.html`.
- No-language-work boundary: This document contains no translation, localization, grammar improvement, humanization, rewriting, summarization, expansion, or recommendation change.
- Exactness rule: Text is decoded as browser-visible text and HTML whitespace is normalized only; wording, spelling, punctuation, capitalization, numbers, and meaning remain unchanged.
- Common UI boundary: Global navigation, the language switcher, and the global footer are excluded because approved Spanish common UI strings already exist.
- User-facing `data-*` rule: A `data-*` value is extracted as its own ITEM only when CSS or JavaScript exposes it to users, including `content: attr(data-label)`; functional, tracking, analytics, affiliate, and event `data-*` values remain protected structure.
- Protection rule: Facts, numbers, recommendations, proper names, brands, products, stations/routes, prices, dates, operating conditions, URLs/tracking, functional data attributes, class/id values, image/srcset, CSS/JS, and schema structure must remain unchanged.

## Page index and source integrity

| Page | English file | SHA-256 | ITEM count |
|---:|---|---|---:|
| 1 | `maps.html` | `14f843c1cccfcb1c0b0e77ba9026920449189d11bf22376d802e08ea94944739` | 303 |
| 2 | `apps.html` | `b80f02a398c9b562baa3bfc8977961c25fe4081f92f6262ed3945fcbd185b734` | 246 |
| **Total** | **2 files** |  | **549** |

## EXCLUDE / protected structural records

- EXCLUDE - Shared global UI: `<header data-common-header>`, global navigation, language switcher, and global footer strings. Reason: approved Spanish common UI already exists and must not be duplicated in this page-specific source.
- EXCLUDE - Non-user-facing structure: HTML tags, schema keys/types, CSS, JavaScript, `class`, `id`, functional `data-*`, `src`, `srcset`, internal control attributes, and code-only values. Reason: preserve exactly; these are not localization strings.
- EXCLUDE - Decorative or empty alternative text and decorative `aria-hidden="true"` symbols. Reason: they do not expose page-specific English strings.
- PROTECTED - Link destinations, affiliate/tracking values, image paths/srcsets, IDs/classes, functional attributes, and JSON-LD structure remain byte-for-byte unchanged even when associated user-facing copy is localized later.
- PROTECTED - Source facts and recommendation judgments remain unchanged; each ITEM's `Protected tokens` field identifies exact-value names, numbers, products, brands, places, and related fixed tokens present in the English copy.

## Page extraction items


## PAGE - maps.html

- English source: `maps.html`
- Source SHA-256: `14f843c1cccfcb1c0b0e77ba9026920449189d11bf22376d802e08ea94944739`
- Extracted ITEM count: 303

### ITEM 001

- Page ITEM: 001 of 303
- File: `maps.html`
- Line/context: L6 - `html > head > meta[name="description"] @content`
- Element/type: Meta description
- Exact English:

  ```text
  Use Naver Map, KakaoMap and Google Maps in Korea. Learn how to search Korean places, check subway exits, follow bus routes and recover when English search fails.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`, `Google Maps`, `Google`, `Korea`

### ITEM 002

- Page ITEM: 002 of 303
- File: `maps.html`
- Line/context: L9 - `html > head > title`
- Element/type: Title
- Exact English:

  ```text
  Best Map App for Korea: Naver Map, KakaoMap & Google Maps | Korea Inside
  ```
- Protected tokens: `Korea`, `Naver Map`, `KakaoMap`, `Google Maps`, `Google`, `Korea Inside`

### ITEM 003

- Page ITEM: 003 of 303
- File: `maps.html`
- Line/context: L12 - `html > head > meta[property="og:title"] @content`
- Element/type: Open Graph title
- Exact English:

  ```text
  Best Map App for Korea: Naver Map, KakaoMap & Google Maps
  ```
- Protected tokens: `Korea`, `Naver Map`, `KakaoMap`, `Google Maps`, `Google`

### ITEM 004

- Page ITEM: 004 of 303
- File: `maps.html`
- Line/context: L13 - `html > head > meta[property="og:description"] @content`
- Element/type: Open Graph description
- Exact English:

  ```text
  Use Naver Map, KakaoMap and Google Maps in Korea. Learn how to search Korean places, check subway exits, follow bus routes and recover when English search fails.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`, `Google Maps`, `Google`, `Korea`

### ITEM 005

- Page ITEM: 005 of 303
- File: `maps.html`
- Line/context: L17 - `html > head > meta[name="twitter:title"] @content`
- Element/type: Twitter card title
- Exact English:

  ```text
  Best Map App for Korea: Naver Map, KakaoMap & Google Maps
  ```
- Protected tokens: `Korea`, `Naver Map`, `KakaoMap`, `Google Maps`, `Google`

### ITEM 006

- Page ITEM: 006 of 303
- File: `maps.html`
- Line/context: L18 - `html > head > meta[name="twitter:description"] @content`
- Element/type: Twitter card description
- Exact English:

  ```text
  Use Naver Map as your main navigation app, KakaoMap as a backup, and Google Maps for planning, saved places and reviews.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`, `Google Maps`, `Google`

### ITEM 007

- Page ITEM: 007 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[0].name`
- Element/type: JSON-LD user-facing name
- Exact English:

  ```text
  Best Map App for Korea: Naver Map, KakaoMap & Google Maps
  ```
- Protected tokens: `Korea`, `Naver Map`, `KakaoMap`, `Google Maps`, `Google`

### ITEM 008

- Page ITEM: 008 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[0].description`
- Element/type: JSON-LD user-facing description
- Exact English:

  ```text
  Use Naver Map, KakaoMap and Google Maps in Korea. Learn how to search Korean places, check subway exits, follow bus routes and recover when English search fails.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`, `Google Maps`, `Google`, `Korea`

### ITEM 009

- Page ITEM: 009 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[0].isPartOf.name`
- Element/type: JSON-LD user-facing name
- Exact English:

  ```text
  Korea Inside
  ```
- Protected tokens: `Korea Inside`, `Korea`

### ITEM 010

- Page ITEM: 010 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[1].itemListElement[0].name`
- Element/type: JSON-LD user-facing name
- Exact English:

  ```text
  Home
  ```
- Protected tokens: None identified in this item.

### ITEM 011

- Page ITEM: 011 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[1].itemListElement[1].name`
- Element/type: JSON-LD user-facing name
- Exact English:

  ```text
  Maps
  ```
- Protected tokens: None identified in this item.

### ITEM 012

- Page ITEM: 012 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[0].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  What is the best map app for Korea?
  ```
- Protected tokens: `Korea`

### ITEM 013

- Page ITEM: 013 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[0].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Naver Map is the best first choice for most travelers because it combines local place search with walking, subway and bus routes. Keep KakaoMap as a backup and Google Maps for planning, saved places and international reviews.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`, `Google Maps`, `Google`

### ITEM 014

- Page ITEM: 014 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[1].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Is Naver Map available in English?
  ```
- Protected tokens: `Naver Map`

### ITEM 015

- Page ITEM: 015 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[1].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Yes. In the app, open MY, then Settings, Language and English. On iPhone, the app may send you to Settings, NAVER Map and Preferred Language.
  ```
- Protected tokens: `iPhone`, `NAVER Map`, `NAVER`, `Preferred Language`

### ITEM 016

- Page ITEM: 016 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[2].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Does Google Maps work in Korea?
  ```
- Protected tokens: `Google Maps`, `Google`, `Korea`

### ITEM 017

- Page ITEM: 017 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[2].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Yes. It is useful for saved places, shared lists, international reviews and broad trip planning. Route and navigation availability can still vary, so confirm the exact route on your device and keep a local map app ready.
  ```
- Protected tokens: None identified in this item.

### ITEM 018

- Page ITEM: 018 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[3].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Should I install KakaoMap too?
  ```
- Protected tokens: `KakaoMap`

### ITEM 019

- Page ITEM: 019 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[3].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  It is a useful backup, not a requirement for every traveler. Install it if you want to cross-check a place, compare a walking route, verify bus information or open a Kakao place link.
  ```
- Protected tokens: `Kakao`

### ITEM 020

- Page ITEM: 020 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[4].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  What should I do when English search fails?
  ```
- Protected tokens: None identified in this item.

### ITEM 021

- Page ITEM: 021 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[4].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Find the official Korean place name or road address in your booking, the business website or another reliable listing. Paste it into Naver Map, then confirm the address, phone number and photos.
  ```
- Protected tokens: `Naver Map`

### ITEM 022

- Page ITEM: 022 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[5].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  How do I find the correct subway exit?
  ```
- Protected tokens: None identified in this item.

### ITEM 023

- Page ITEM: 023 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[5].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Open the route before leaving the train and confirm the exit number, the side of the road and the remaining walk. At a large station, also check whether you need an elevator and how far the indoor walk is.
  ```
- Protected tokens: None identified in this item.

### ITEM 024

- Page ITEM: 024 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[6].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  How do I avoid choosing the wrong bus stop?
  ```
- Protected tokens: None identified in this item.

### ITEM 025

- Page ITEM: 025 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[6].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Check the stop number or direction, the bus destination and the side of the road before boarding. Similar stop names can refer to opposite travel directions.
  ```
- Protected tokens: None identified in this item.

### ITEM 026

- Page ITEM: 026 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[7].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Can I use offline maps in Korea?
  ```
- Protected tokens: `Korea`

### ITEM 027

- Page ITEM: 027 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[7].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Google says offline map downloads are unavailable in some countries or regions, and offline transit, walking and cycling directions are unavailable. Check download availability on your device and keep mobile data for live navigation.
  ```
- Protected tokens: `Google`

### ITEM 028

- Page ITEM: 028 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[8].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  How do I save and share my hotel address?
  ```
- Protected tokens: None identified in this item.

### ITEM 029

- Page ITEM: 029 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[8].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Save the exact hotel listing in Naver Map, copy its Korean name and road address, and share the place link with your travel companions. Also keep the text in your booking confirmation.
  ```
- Protected tokens: `Naver Map`

### ITEM 030

- Page ITEM: 030 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[9].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Do I need mobile data for map apps in Korea?
  ```
- Protected tokens: `Korea`

### ITEM 031

- Page ITEM: 031 of 303
- File: `maps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[9].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Mobile data is strongly recommended for route changes, live transit information, place details and sharing. Arrange an eSIM, SIM or roaming plan before you depend on live navigation.
  ```
- Protected tokens: `eSIM`, `SIM`

### ITEM 032

- Page ITEM: 032 of 303
- File: `maps.html`
- Line/context: L217 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > div.maps-page-hero__copy > h1#maps-page-title`
- Element/type: H1 heading
- Exact English:

  ```text
  Best Map App for Korea: How to Use Naver Map, KakaoMap and Google Maps
  ```
- Protected tokens: `Korea`, `Naver Map`, `KakaoMap`, `Google Maps`, `Google`

### ITEM 033

- Page ITEM: 033 of 303
- File: `maps.html`
- Line/context: L218 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > div.maps-page-hero__copy > p.maps-page-hero__intro`
- Element/type: Hero / lead copy
- Exact English:

  ```text
  Use Naver Map as your main navigation app in Korea. Keep KakaoMap as a backup, and use Google Maps for saved places, reviews and global trip planning.
  ```
- Protected tokens: `Naver Map`, `Korea`, `KakaoMap`, `Google Maps`, `Google`

### ITEM 034

- Page ITEM: 034 of 303
- File: `maps.html`
- Line/context: L220 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > aside.maps-quick-answer @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Map app quick comparison
  ```
- Protected tokens: None identified in this item.

### ITEM 035

- Page ITEM: 035 of 303
- File: `maps.html`
- Line/context: L223 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > aside.maps-quick-answer > dl > div:nth-of-type(1) > dt`
- Element/type: Visible label
- Exact English:

  ```text
  Main navigation
  ```
- Protected tokens: None identified in this item.

### ITEM 036

- Page ITEM: 036 of 303
- File: `maps.html`
- Line/context: L224 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > aside.maps-quick-answer > dl > div:nth-of-type(1) > dd`
- Element/type: Visible description
- Exact English:

  ```text
  Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 037

- Page ITEM: 037 of 303
- File: `maps.html`
- Line/context: L227 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > aside.maps-quick-answer > dl > div:nth-of-type(2) > dt`
- Element/type: Visible label
- Exact English:

  ```text
  Backup routes and local cross-checks
  ```
- Protected tokens: None identified in this item.

### ITEM 038

- Page ITEM: 038 of 303
- File: `maps.html`
- Line/context: L228 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > aside.maps-quick-answer > dl > div:nth-of-type(2) > dd`
- Element/type: Visible description
- Exact English:

  ```text
  KakaoMap
  ```
- Protected tokens: `KakaoMap`

### ITEM 039

- Page ITEM: 039 of 303
- File: `maps.html`
- Line/context: L231 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > aside.maps-quick-answer > dl > div:nth-of-type(3) > dt`
- Element/type: Visible label
- Exact English:

  ```text
  Planning, saved places and international reviews
  ```
- Protected tokens: None identified in this item.

### ITEM 040

- Page ITEM: 040 of 303
- File: `maps.html`
- Line/context: L232 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > aside.maps-quick-answer > dl > div:nth-of-type(3) > dd`
- Element/type: Visible description
- Exact English:

  ```text
  Google Maps
  ```
- Protected tokens: `Google Maps`, `Google`

### ITEM 041

- Page ITEM: 041 of 303
- File: `maps.html`
- Line/context: L244 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-section__heading:nth-of-type(1) > div > h2#comparison-title`
- Element/type: H2 heading
- Exact English:

  ```text
  Korea Map Apps Compared
  ```
- Protected tokens: `Korea`

### ITEM 042

- Page ITEM: 042 of 303
- File: `maps.html`
- Line/context: L245 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-section__heading:nth-of-type(1) > div > p`
- Element/type: Body text
- Exact English:

  ```text
  Choose the app by the travel problem you need to solve, not by brand familiarity alone.
  ```
- Protected tokens: None identified in this item.

### ITEM 043

- Page ITEM: 043 of 303
- File: `maps.html`
- Line/context: L248 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Map apps by travel need
  ```
- Protected tokens: None identified in this item.

### ITEM 044

- Page ITEM: 044 of 303
- File: `maps.html`
- Line/context: L252 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > thead > tr > th:nth-of-type(1)`
- Element/type: Table header
- Exact English:

  ```text
  Travel need
  ```
- Protected tokens: None identified in this item.

### ITEM 045

- Page ITEM: 045 of 303
- File: `maps.html`
- Line/context: L253 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > thead > tr > th:nth-of-type(2)`
- Element/type: Table header
- Exact English:

  ```text
  Best first choice
  ```
- Protected tokens: None identified in this item.

### ITEM 046

- Page ITEM: 046 of 303
- File: `maps.html`
- Line/context: L254 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > thead > tr > th:nth-of-type(3)`
- Element/type: Table header
- Exact English:

  ```text
  Backup or secondary use
  ```
- Protected tokens: None identified in this item.

### ITEM 047

- Page ITEM: 047 of 303
- File: `maps.html`
- Line/context: L259 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(1) > th`
- Element/type: Table header
- Exact English:

  ```text
  Walking and local navigation
  ```
- Protected tokens: None identified in this item.

### ITEM 048

- Page ITEM: 048 of 303
- File: `maps.html`
- Line/context: L260 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(1) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 049

- Page ITEM: 049 of 303
- File: `maps.html`
- Line/context: L261 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(1) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  KakaoMap for an alternate route
  ```
- Protected tokens: `KakaoMap`

### ITEM 050

- Page ITEM: 050 of 303
- File: `maps.html`
- Line/context: L264 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(2) > th`
- Element/type: Table header
- Exact English:

  ```text
  Subway and bus routes
  ```
- Protected tokens: None identified in this item.

### ITEM 051

- Page ITEM: 051 of 303
- File: `maps.html`
- Line/context: L265 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(2) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 052

- Page ITEM: 052 of 303
- File: `maps.html`
- Line/context: L266 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(2) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  KakaoMap for a local cross-check
  ```
- Protected tokens: `KakaoMap`

### ITEM 053

- Page ITEM: 053 of 303
- File: `maps.html`
- Line/context: L269 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(3) > th`
- Element/type: Table header
- Exact English:

  ```text
  Finding the exact branch
  ```
- Protected tokens: None identified in this item.

### ITEM 054

- Page ITEM: 054 of 303
- File: `maps.html`
- Line/context: L270 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(3) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 055

- Page ITEM: 055 of 303
- File: `maps.html`
- Line/context: L271 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(3) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  KakaoMap to confirm the listing
  ```
- Protected tokens: `KakaoMap`

### ITEM 056

- Page ITEM: 056 of 303
- File: `maps.html`
- Line/context: L274 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(4) > th`
- Element/type: Table header
- Exact English:

  ```text
  Saved places and trip planning
  ```
- Protected tokens: None identified in this item.

### ITEM 057

- Page ITEM: 057 of 303
- File: `maps.html`
- Line/context: L275 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(4) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Google Maps
  ```
- Protected tokens: `Google Maps`, `Google`

### ITEM 058

- Page ITEM: 058 of 303
- File: `maps.html`
- Line/context: L276 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(4) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Naver Map for the final local listing
  ```
- Protected tokens: `Naver Map`

### ITEM 059

- Page ITEM: 059 of 303
- File: `maps.html`
- Line/context: L279 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(5) > th`
- Element/type: Table header
- Exact English:

  ```text
  English search failure
  ```
- Protected tokens: None identified in this item.

### ITEM 060

- Page ITEM: 060 of 303
- File: `maps.html`
- Line/context: L280 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(5) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Naver Map with the Korean name or address
  ```
- Protected tokens: `Naver Map`

### ITEM 061

- Page ITEM: 061 of 303
- File: `maps.html`
- Line/context: L281 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(5) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  KakaoMap to cross-check the result
  ```
- Protected tokens: `KakaoMap`

### ITEM 062

- Page ITEM: 062 of 303
- File: `maps.html`
- Line/context: L284 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(6) > th`
- Element/type: Table header
- Exact English:

  ```text
  International reviews
  ```
- Protected tokens: None identified in this item.

### ITEM 063

- Page ITEM: 063 of 303
- File: `maps.html`
- Line/context: L285 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(6) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Google Maps
  ```
- Protected tokens: `Google Maps`, `Google`

### ITEM 064

- Page ITEM: 064 of 303
- File: `maps.html`
- Line/context: L286 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(6) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Naver Map for local details and routing
  ```
- Protected tokens: `Naver Map`

### ITEM 065

- Page ITEM: 065 of 303
- File: `maps.html`
- Line/context: L289 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(7) > th`
- Element/type: Table header
- Exact English:

  ```text
  Driving
  ```
- Protected tokens: None identified in this item.

### ITEM 066

- Page ITEM: 066 of 303
- File: `maps.html`
- Line/context: L290 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(7) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 067

- Page ITEM: 067 of 303
- File: `maps.html`
- Line/context: L291 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(7) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  KakaoMap; verify Google Maps on your device
  ```
- Protected tokens: `KakaoMap`, `Google Maps`, `Google`

### ITEM 068

- Page ITEM: 068 of 303
- File: `maps.html`
- Line/context: L294 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(8) > th`
- Element/type: Table header
- Exact English:

  ```text
  Offline use
  ```
- Protected tokens: None identified in this item.

### ITEM 069

- Page ITEM: 069 of 303
- File: `maps.html`
- Line/context: L295 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(8) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Check Google Maps download availability
  ```
- Protected tokens: `Google Maps`, `Google`

### ITEM 070

- Page ITEM: 070 of 303
- File: `maps.html`
- Line/context: L296 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(8) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Keep mobile data and saved Korean addresses
  ```
- Protected tokens: None identified in this item.

### ITEM 071

- Page ITEM: 071 of 303
- File: `maps.html`
- Line/context: L306 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > div.maps-section__heading > div > h2#set-up-title`
- Element/type: H2 heading
- Exact English:

  ```text
  Set Up Your Maps Before You Arrive
  ```
- Protected tokens: None identified in this item.

### ITEM 072

- Page ITEM: 072 of 303
- File: `maps.html`
- Line/context: L307 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > div.maps-section__heading > div > p`
- Element/type: Body text
- Exact English:

  ```text
  Prepare the apps, language and one test route while you still have reliable Wi-Fi.
  ```
- Protected tokens: `Wi-Fi`

### ITEM 073

- Page ITEM: 073 of 303
- File: `maps.html`
- Line/context: L311 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Install Naver Map Action Download the official app and open it once. If it fails Search the store for “NAVER Maps, Navigation” and confirm the developer is NAVER Corp.
  ```
- Protected tokens: `Naver Map`, `NAVER`, `NAVER Corp.`, `NAVER Corp`

### ITEM 074

- Page ITEM: 074 of 303
- File: `maps.html`
- Line/context: L313 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(1) > div > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Install Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 075

- Page ITEM: 075 of 303
- File: `maps.html`
- Line/context: L314 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(1) > div > p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Action Download the official app and open it once.
  ```
- Protected tokens: None identified in this item.

### ITEM 076

- Page ITEM: 076 of 303
- File: `maps.html`
- Line/context: L315 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(1) > div > p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  If it fails Search the store for “NAVER Maps, Navigation” and confirm the developer is NAVER Corp.
  ```
- Protected tokens: `NAVER`, `NAVER Corp.`, `NAVER Corp`

### ITEM 077

- Page ITEM: 077 of 303
- File: `maps.html`
- Line/context: L318 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Install KakaoMap as a backup Action Keep it ready for alternate routes and local cross-checks. If it fails Continue with Naver Map and open the official KakaoMap web service when needed.
  ```
- Protected tokens: `KakaoMap`, `Naver Map`

### ITEM 078

- Page ITEM: 078 of 303
- File: `maps.html`
- Line/context: L320 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(2) > div > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Install KakaoMap as a backup
  ```
- Protected tokens: `KakaoMap`

### ITEM 079

- Page ITEM: 079 of 303
- File: `maps.html`
- Line/context: L321 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(2) > div > p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Action Keep it ready for alternate routes and local cross-checks.
  ```
- Protected tokens: None identified in this item.

### ITEM 080

- Page ITEM: 080 of 303
- File: `maps.html`
- Line/context: L322 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(2) > div > p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  If it fails Continue with Naver Map and open the official KakaoMap web service when needed.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`

### ITEM 081

- Page ITEM: 081 of 303
- File: `maps.html`
- Line/context: L325 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Keep Google Maps Action Retain your saved places, shared lists and familiar trip plan. If it fails Copy the place name or address into a local map app for the final route.
  ```
- Protected tokens: `Google Maps`, `Google`

### ITEM 082

- Page ITEM: 082 of 303
- File: `maps.html`
- Line/context: L327 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(3) > div > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Keep Google Maps
  ```
- Protected tokens: `Google Maps`, `Google`

### ITEM 083

- Page ITEM: 083 of 303
- File: `maps.html`
- Line/context: L328 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(3) > div > p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Action Retain your saved places, shared lists and familiar trip plan.
  ```
- Protected tokens: None identified in this item.

### ITEM 084

- Page ITEM: 084 of 303
- File: `maps.html`
- Line/context: L329 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(3) > div > p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  If it fails Copy the place name or address into a local map app for the final route.
  ```
- Protected tokens: None identified in this item.

### ITEM 085

- Page ITEM: 085 of 303
- File: `maps.html`
- Line/context: L332 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Allow location access Action Allow location while using each app so the starting point is accurate. If it fails Open the phone’s privacy or app permission settings and enable precise location.
  ```
- Protected tokens: None identified in this item.

### ITEM 086

- Page ITEM: 086 of 303
- File: `maps.html`
- Line/context: L334 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(4) > div > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Allow location access
  ```
- Protected tokens: None identified in this item.

### ITEM 087

- Page ITEM: 087 of 303
- File: `maps.html`
- Line/context: L335 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(4) > div > p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Action Allow location while using each app so the starting point is accurate.
  ```
- Protected tokens: None identified in this item.

### ITEM 088

- Page ITEM: 088 of 303
- File: `maps.html`
- Line/context: L336 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(4) > div > p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  If it fails Open the phone’s privacy or app permission settings and enable precise location.
  ```
- Protected tokens: None identified in this item.

### ITEM 089

- Page ITEM: 089 of 303
- File: `maps.html`
- Line/context: L339 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  Change Naver Map to English Action Open MY, Settings, Language, then choose English. If it fails On iPhone, use Settings, NAVER Map, Preferred Language.
  ```
- Protected tokens: `Naver Map`, `iPhone`, `NAVER Map`, `NAVER`, `Preferred Language`

### ITEM 090

- Page ITEM: 090 of 303
- File: `maps.html`
- Line/context: L341 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(5) > div > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Change Naver Map to English
  ```
- Protected tokens: `Naver Map`

### ITEM 091

- Page ITEM: 091 of 303
- File: `maps.html`
- Line/context: L342 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(5) > div > p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Action Open MY, Settings, Language, then choose English.
  ```
- Protected tokens: None identified in this item.

### ITEM 092

- Page ITEM: 092 of 303
- File: `maps.html`
- Line/context: L343 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(5) > div > p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  If it fails On iPhone, use Settings, NAVER Map, Preferred Language.
  ```
- Protected tokens: `iPhone`, `NAVER Map`, `NAVER`, `Preferred Language`

### ITEM 093

- Page ITEM: 093 of 303
- File: `maps.html`
- Line/context: L346 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  Save your hotel’s Korean name and address Action Save the exact listing, Korean road address and phone number. If it fails Copy the Korean text from your hotel or booking confirmation.
  ```
- Protected tokens: None identified in this item.

### ITEM 094

- Page ITEM: 094 of 303
- File: `maps.html`
- Line/context: L348 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(6) > div > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Save your hotel’s Korean name and address
  ```
- Protected tokens: None identified in this item.

### ITEM 095

- Page ITEM: 095 of 303
- File: `maps.html`
- Line/context: L349 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(6) > div > p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Action Save the exact listing, Korean road address and phone number.
  ```
- Protected tokens: None identified in this item.

### ITEM 096

- Page ITEM: 096 of 303
- File: `maps.html`
- Line/context: L350 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(6) > div > p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  If it fails Copy the Korean text from your hotel or booking confirmation.
  ```
- Protected tokens: None identified in this item.

### ITEM 097

- Page ITEM: 097 of 303
- File: `maps.html`
- Line/context: L353 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(7)`
- Element/type: List text
- Exact English:

  ```text
  Test one airport-to-hotel route Action Set the correct airport terminal and compare transit with the final walk. If it fails Save a screenshot and verify the route again after landing.
  ```
- Protected tokens: None identified in this item.

### ITEM 098

- Page ITEM: 098 of 303
- File: `maps.html`
- Line/context: L355 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(7) > div > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Test one airport-to-hotel route
  ```
- Protected tokens: None identified in this item.

### ITEM 099

- Page ITEM: 099 of 303
- File: `maps.html`
- Line/context: L356 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(7) > div > p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Action Set the correct airport terminal and compare transit with the final walk.
  ```
- Protected tokens: None identified in this item.

### ITEM 100

- Page ITEM: 100 of 303
- File: `maps.html`
- Line/context: L357 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(7) > div > p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  If it fails Save a screenshot and verify the route again after landing.
  ```
- Protected tokens: None identified in this item.

### ITEM 101

- Page ITEM: 101 of 303
- File: `maps.html`
- Line/context: L361 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > p.maps-context-link`
- Element/type: Body text
- Exact English:

  ```text
  Live routes work best with mobile data. Prepare an eSIM for Korea , compare eSIM options , review the essential apps guide , and add these steps to your Korea travel checklist .
  ```
- Protected tokens: `eSIM`, `Korea`

### ITEM 102

- Page ITEM: 102 of 303
- File: `maps.html`
- Line/context: L367 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-section__heading:nth-of-type(1) > div > h2#naver-map-title`
- Element/type: H2 heading
- Exact English:

  ```text
  How to Use Naver Map in English
  ```
- Protected tokens: `Naver Map`

### ITEM 103

- Page ITEM: 103 of 303
- File: `maps.html`
- Line/context: L368 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-section__heading:nth-of-type(1) > div > p`
- Element/type: Body text
- Exact English:

  ```text
  Use this sequence from setup to the final subway exit or bus stop.
  ```
- Protected tokens: None identified in this item.

### ITEM 104

- Page ITEM: 104 of 303
- File: `maps.html`
- Line/context: L373 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(1) > h3#naver-language-title`
- Element/type: H3 heading
- Exact English:

  ```text
  A. Change the app language to English
  ```
- Protected tokens: None identified in this item.

### ITEM 105

- Page ITEM: 105 of 303
- File: `maps.html`
- Line/context: L376 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(1) > div.maps-setting-paths > div:nth-of-type(1) > p.maps-small-label:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Android
  ```
- Protected tokens: `Android`

### ITEM 106

- Page ITEM: 106 of 303
- File: `maps.html`
- Line/context: L377 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(1) > div.maps-setting-paths > div:nth-of-type(1) > p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  MY → Settings → Language → English
  ```
- Protected tokens: None identified in this item.

### ITEM 107

- Page ITEM: 107 of 303
- File: `maps.html`
- Line/context: L380 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(1) > div.maps-setting-paths > div:nth-of-type(2) > p.maps-small-label:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  iPhone
  ```
- Protected tokens: `iPhone`

### ITEM 108

- Page ITEM: 108 of 303
- File: `maps.html`
- Line/context: L381 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(1) > div.maps-setting-paths > div:nth-of-type(2) > p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  MY → Settings → Language
  ```
- Protected tokens: None identified in this item.

### ITEM 109

- Page ITEM: 109 of 303
- File: `maps.html`
- Line/context: L382 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(1) > div.maps-setting-paths > div:nth-of-type(2) > p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  If needed: iPhone Settings → NAVER Map → Preferred Language → English
  ```
- Protected tokens: `iPhone`, `NAVER Map`, `NAVER`, `Preferred Language`

### ITEM 110

- Page ITEM: 110 of 303
- File: `maps.html`
- Line/context: L386 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(1) > figure.maps-guide-figure > img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  How to change NAVER Map to English using the app settings
  ```
- Protected tokens: `NAVER Map`, `NAVER`

### ITEM 111

- Page ITEM: 111 of 303
- File: `maps.html`
- Line/context: L390 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(2) > h3#naver-search-title`
- Element/type: H3 heading
- Exact English:

  ```text
  B. Search for a place
  ```
- Protected tokens: None identified in this item.

### ITEM 112

- Page ITEM: 112 of 303
- File: `maps.html`
- Line/context: L391 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(2) > p`
- Element/type: Body text
- Exact English:

  ```text
  Try increasingly specific information. One failed English search does not mean the place is missing.
  ```
- Protected tokens: None identified in this item.

### ITEM 113

- Page ITEM: 113 of 303
- File: `maps.html`
- Line/context: L393 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  English place name
  ```
- Protected tokens: None identified in this item.

### ITEM 114

- Page ITEM: 114 of 303
- File: `maps.html`
- Line/context: L394 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  English road address
  ```
- Protected tokens: None identified in this item.

### ITEM 115

- Page ITEM: 115 of 303
- File: `maps.html`
- Line/context: L395 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Korean place name
  ```
- Protected tokens: None identified in this item.

### ITEM 116

- Page ITEM: 116 of 303
- File: `maps.html`
- Line/context: L396 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Korean road address
  ```
- Protected tokens: None identified in this item.

### ITEM 117

- Page ITEM: 117 of 303
- File: `maps.html`
- Line/context: L397 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  Phone number
  ```
- Protected tokens: None identified in this item.

### ITEM 118

- Page ITEM: 118 of 303
- File: `maps.html`
- Line/context: L401 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > h3#naver-branch-title`
- Element/type: H3 heading
- Exact English:

  ```text
  C. Confirm the correct branch
  ```
- Protected tokens: None identified in this item.

### ITEM 119

- Page ITEM: 119 of 303
- File: `maps.html`
- Line/context: L402 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > p`
- Element/type: Body text
- Exact English:

  ```text
  Hotels, cafés and restaurants can share a name across several neighborhoods. Check the listing before you start the route.
  ```
- Protected tokens: None identified in this item.

### ITEM 120

- Page ITEM: 120 of 303
- File: `maps.html`
- Line/context: L404 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > ul.maps-check-list > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Neighborhood or district
  ```
- Protected tokens: None identified in this item.

### ITEM 121

- Page ITEM: 121 of 303
- File: `maps.html`
- Line/context: L405 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > ul.maps-check-list > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Korean road address
  ```
- Protected tokens: None identified in this item.

### ITEM 122

- Page ITEM: 122 of 303
- File: `maps.html`
- Line/context: L406 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > ul.maps-check-list > li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Phone number
  ```
- Protected tokens: None identified in this item.

### ITEM 123

- Page ITEM: 123 of 303
- File: `maps.html`
- Line/context: L407 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > ul.maps-check-list > li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Photos
  ```
- Protected tokens: None identified in this item.

### ITEM 124

- Page ITEM: 124 of 303
- File: `maps.html`
- Line/context: L408 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > ul.maps-check-list > li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  Nearest subway station
  ```
- Protected tokens: None identified in this item.

### ITEM 125

- Page ITEM: 125 of 303
- File: `maps.html`
- Line/context: L409 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > ul.maps-check-list > li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  Distance from the station
  ```
- Protected tokens: None identified in this item.

### ITEM 126

- Page ITEM: 126 of 303
- File: `maps.html`
- Line/context: L410 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > ul.maps-check-list > li:nth-of-type(7)`
- Element/type: List text
- Exact English:

  ```text
  Opening information
  ```
- Protected tokens: None identified in this item.

### ITEM 127

- Page ITEM: 127 of 303
- File: `maps.html`
- Line/context: L413 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > figure.maps-guide-figure > img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  How to search NAVER Map and compare similar place results
  ```
- Protected tokens: `NAVER Map`, `NAVER`

### ITEM 128

- Page ITEM: 128 of 303
- File: `maps.html`
- Line/context: L417 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(4) > h3#naver-directions-title`
- Element/type: H3 heading
- Exact English:

  ```text
  D. Get directions
  ```
- Protected tokens: None identified in this item.

### ITEM 129

- Page ITEM: 129 of 303
- File: `maps.html`
- Line/context: L419 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(4) > ul > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Set your current location or another starting point.
  ```
- Protected tokens: None identified in this item.

### ITEM 130

- Page ITEM: 130 of 303
- File: `maps.html`
- Line/context: L420 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(4) > ul > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Select the confirmed destination.
  ```
- Protected tokens: None identified in this item.

### ITEM 131

- Page ITEM: 131 of 303
- File: `maps.html`
- Line/context: L421 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(4) > ul > li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Choose transit or walking.
  ```
- Protected tokens: None identified in this item.

### ITEM 132

- Page ITEM: 132 of 303
- File: `maps.html`
- Line/context: L422 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(4) > ul > li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Compare travel time, transfers and the final walking distance.
  ```
- Protected tokens: None identified in this item.

### ITEM 133

- Page ITEM: 133 of 303
- File: `maps.html`
- Line/context: L423 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(4) > ul > li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  Save the route before leaving Wi-Fi.
  ```
- Protected tokens: `Wi-Fi`

### ITEM 134

- Page ITEM: 134 of 303
- File: `maps.html`
- Line/context: L427 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(5) > h3#naver-exits-title`
- Element/type: H3 heading
- Exact English:

  ```text
  E. Check subway exits
  ```
- Protected tokens: None identified in this item.

### ITEM 135

- Page ITEM: 135 of 303
- File: `maps.html`
- Line/context: L429 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(5) > ul > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  A station name alone is not enough.
  ```
- Protected tokens: None identified in this item.

### ITEM 136

- Page ITEM: 136 of 303
- File: `maps.html`
- Line/context: L430 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(5) > ul > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Confirm the exit number before leaving the train.
  ```
- Protected tokens: None identified in this item.

### ITEM 137

- Page ITEM: 137 of 303
- File: `maps.html`
- Line/context: L431 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(5) > ul > li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Check which side of the road the exit reaches.
  ```
- Protected tokens: None identified in this item.

### ITEM 138

- Page ITEM: 138 of 303
- File: `maps.html`
- Line/context: L432 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(5) > ul > li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Expect a long indoor walk at some large stations.
  ```
- Protected tokens: None identified in this item.

### ITEM 139

- Page ITEM: 139 of 303
- File: `maps.html`
- Line/context: L436 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(6) > h3#naver-bus-title`
- Element/type: H3 heading
- Exact English:

  ```text
  F. Check bus stops
  ```
- Protected tokens: None identified in this item.

### ITEM 140

- Page ITEM: 140 of 303
- File: `maps.html`
- Line/context: L438 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(6) > ul > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Two stops can have similar names.
  ```
- Protected tokens: None identified in this item.

### ITEM 141

- Page ITEM: 141 of 303
- File: `maps.html`
- Line/context: L439 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(6) > ul > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Confirm the stop number or direction.
  ```
- Protected tokens: None identified in this item.

### ITEM 142

- Page ITEM: 142 of 303
- File: `maps.html`
- Line/context: L440 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(6) > ul > li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Check the bus destination before boarding.
  ```
- Protected tokens: None identified in this item.

### ITEM 143

- Page ITEM: 143 of 303
- File: `maps.html`
- Line/context: L441 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(6) > ul > li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Review the final walking segment.
  ```
- Protected tokens: None identified in this item.

### ITEM 144

- Page ITEM: 144 of 303
- File: `maps.html`
- Line/context: L444 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(6) > figure.maps-guide-figure > img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  How to compare NAVER Map routes and confirm boarding and get-off points
  ```
- Protected tokens: `NAVER Map`, `NAVER`

### ITEM 145

- Page ITEM: 145 of 303
- File: `maps.html`
- Line/context: L448 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(7) > h3#naver-save-title`
- Element/type: H3 heading
- Exact English:

  ```text
  G. Save and share a place
  ```
- Protected tokens: None identified in this item.

### ITEM 146

- Page ITEM: 146 of 303
- File: `maps.html`
- Line/context: L450 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(7) > ul > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Save your hotel and exact airport terminal.
  ```
- Protected tokens: None identified in this item.

### ITEM 147

- Page ITEM: 147 of 303
- File: `maps.html`
- Line/context: L451 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(7) > ul > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Save restaurants only after confirming the branch.
  ```
- Protected tokens: None identified in this item.

### ITEM 148

- Page ITEM: 148 of 303
- File: `maps.html`
- Line/context: L452 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(7) > ul > li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Copy the Korean address as text.
  ```
- Protected tokens: None identified in this item.

### ITEM 149

- Page ITEM: 149 of 303
- File: `maps.html`
- Line/context: L453 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(7) > ul > li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Share the exact place link with travel companions.
  ```
- Protected tokens: None identified in this item.

### ITEM 150

- Page ITEM: 150 of 303
- File: `maps.html`
- Line/context: L462 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-section__heading:nth-of-type(1) > div > h2#search-recovery-title`
- Element/type: H2 heading
- Exact English:

  ```text
  What to Do When English Search Fails
  ```
- Protected tokens: None identified in this item.

### ITEM 151

- Page ITEM: 151 of 303
- File: `maps.html`
- Line/context: L463 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-section__heading:nth-of-type(1) > div > p`
- Element/type: Body text
- Exact English:

  ```text
  Recover the result by switching from a translated name to the exact Korean listing information.
  ```
- Protected tokens: None identified in this item.

### ITEM 152

- Page ITEM: 152 of 303
- File: `maps.html`
- Line/context: L466 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > ol.maps-recovery-flow @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  English search recovery flow
  ```
- Protected tokens: None identified in this item.

### ITEM 153

- Page ITEM: 153 of 303
- File: `maps.html`
- Line/context: L467 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > ol.maps-recovery-flow > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  English name fails
  ```
- Protected tokens: None identified in this item.

### ITEM 154

- Page ITEM: 154 of 303
- File: `maps.html`
- Line/context: L468 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > ol.maps-recovery-flow > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Find the official Korean name
  ```
- Protected tokens: None identified in this item.

### ITEM 155

- Page ITEM: 155 of 303
- File: `maps.html`
- Line/context: L469 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > ol.maps-recovery-flow > li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Copy the Korean text
  ```
- Protected tokens: None identified in this item.

### ITEM 156

- Page ITEM: 156 of 303
- File: `maps.html`
- Line/context: L470 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > ol.maps-recovery-flow > li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Paste it into Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 157

- Page ITEM: 157 of 303
- File: `maps.html`
- Line/context: L471 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > ol.maps-recovery-flow > li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  Confirm address, phone and photos
  ```
- Protected tokens: None identified in this item.

### ITEM 158

- Page ITEM: 158 of 303
- File: `maps.html`
- Line/context: L472 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > ol.maps-recovery-flow > li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  Cross-check in KakaoMap if needed
  ```
- Protected tokens: `KakaoMap`

### ITEM 159

- Page ITEM: 159 of 303
- File: `maps.html`
- Line/context: L476 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(1) > h3#korean-name-source-title`
- Element/type: H3 heading
- Exact English:

  ```text
  Where to find the Korean name
  ```
- Protected tokens: None identified in this item.

### ITEM 160

- Page ITEM: 160 of 303
- File: `maps.html`
- Line/context: L478 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(1) > ul > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Hotel or booking confirmation
  ```
- Protected tokens: None identified in this item.

### ITEM 161

- Page ITEM: 161 of 303
- File: `maps.html`
- Line/context: L479 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(1) > ul > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Official business website
  ```
- Protected tokens: None identified in this item.

### ITEM 162

- Page ITEM: 162 of 303
- File: `maps.html`
- Line/context: L480 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(1) > ul > li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Official Instagram profile
  ```
- Protected tokens: `Instagram`

### ITEM 163

- Page ITEM: 163 of 303
- File: `maps.html`
- Line/context: L481 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(1) > ul > li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Google search result
  ```
- Protected tokens: `Google`

### ITEM 164

- Page ITEM: 164 of 303
- File: `maps.html`
- Line/context: L482 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(1) > ul > li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  Korean road address
  ```
- Protected tokens: None identified in this item.

### ITEM 165

- Page ITEM: 165 of 303
- File: `maps.html`
- Line/context: L483 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(1) > ul > li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  Phone number
  ```
- Protected tokens: None identified in this item.

### ITEM 166

- Page ITEM: 166 of 303
- File: `maps.html`
- Line/context: L487 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(2) > h3#search-failure-title`
- Element/type: H3 heading
- Exact English:

  ```text
  Why a result can still be wrong
  ```
- Protected tokens: None identified in this item.

### ITEM 167

- Page ITEM: 167 of 303
- File: `maps.html`
- Line/context: L489 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  The booking-site name differs from the local registered name.
  ```
- Protected tokens: None identified in this item.

### ITEM 168

- Page ITEM: 168 of 303
- File: `maps.html`
- Line/context: L490 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Another branch uses the same name.
  ```
- Protected tokens: None identified in this item.

### ITEM 169

- Page ITEM: 169 of 303
- File: `maps.html`
- Line/context: L491 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  The restaurant is registered under a building name.
  ```
- Protected tokens: None identified in this item.

### ITEM 170

- Page ITEM: 170 of 303
- File: `maps.html`
- Line/context: L492 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  The business moved, closed or is a temporary pop-up.
  ```
- Protected tokens: None identified in this item.

### ITEM 171

- Page ITEM: 171 of 303
- File: `maps.html`
- Line/context: L493 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  The shop is inside an underground mall.
  ```
- Protected tokens: None identified in this item.

### ITEM 172

- Page ITEM: 172 of 303
- File: `maps.html`
- Line/context: L494 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  The English spelling varies.
  ```
- Protected tokens: None identified in this item.

### ITEM 173

- Page ITEM: 173 of 303
- File: `maps.html`
- Line/context: L503 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > div.maps-section__heading > div > h2#kakaomap-title`
- Element/type: H2 heading
- Exact English:

  ```text
  When to Use KakaoMap as a Backup
  ```
- Protected tokens: `KakaoMap`

### ITEM 174

- Page ITEM: 174 of 303
- File: `maps.html`
- Line/context: L504 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > div.maps-section__heading > div > p`
- Element/type: Body text
- Exact English:

  ```text
  KakaoMap adds a second local result when confirmation matters. It does not need to replace your main app.
  ```
- Protected tokens: `KakaoMap`

### ITEM 175

- Page ITEM: 175 of 303
- File: `maps.html`
- Line/context: L508 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > ul.maps-editorial-list > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Naver search results are unclear.
  ```
- Protected tokens: None identified in this item.

### ITEM 176

- Page ITEM: 176 of 303
- File: `maps.html`
- Line/context: L509 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > ul.maps-editorial-list > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  You want to compare an alternate walking route.
  ```
- Protected tokens: None identified in this item.

### ITEM 177

- Page ITEM: 177 of 303
- File: `maps.html`
- Line/context: L510 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > ul.maps-editorial-list > li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  You need to cross-check bus information.
  ```
- Protected tokens: None identified in this item.

### ITEM 178

- Page ITEM: 178 of 303
- File: `maps.html`
- Line/context: L511 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > ul.maps-editorial-list > li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  A local sends you a Kakao place link.
  ```
- Protected tokens: `Kakao`

### ITEM 179

- Page ITEM: 179 of 303
- File: `maps.html`
- Line/context: L512 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > ul.maps-editorial-list > li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  You want to confirm a place name or address.
  ```
- Protected tokens: None identified in this item.

### ITEM 180

- Page ITEM: 180 of 303
- File: `maps.html`
- Line/context: L513 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > ul.maps-editorial-list > li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  You want another route before a late-night trip.
  ```
- Protected tokens: None identified in this item.

### ITEM 181

- Page ITEM: 181 of 303
- File: `maps.html`
- Line/context: L515 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > p.maps-conclusion-line`
- Element/type: Body text
- Exact English:

  ```text
  Naver Map first → KakaoMap when confirmation or another route is needed.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`

### ITEM 182

- Page ITEM: 182 of 303
- File: `maps.html`
- Line/context: L521 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > div.maps-section__heading > div > h2#google-maps-title`
- Element/type: H2 heading
- Exact English:

  ```text
  What Google Maps Can and Cannot Do in Korea
  ```
- Protected tokens: `Google Maps`, `Google`, `Korea`

### ITEM 183

- Page ITEM: 183 of 303
- File: `maps.html`
- Line/context: L522 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > div.maps-section__heading > div > p`
- Element/type: Body text
- Exact English:

  ```text
  Keep Google Maps, but separate today’s available features from possible future improvements.
  ```
- Protected tokens: `Google Maps`, `Google`

### ITEM 184

- Page ITEM: 184 of 303
- File: `maps.html`
- Line/context: L525 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  South Korea approved Google’s export of 1:5,000 map data in February 2026 under strict security conditions. This may support future improvements, but rollout timing and feature availability can still vary by device, route and region.
  ```
- Protected tokens: `South Korea`, `Korea`, `Google`, `1:5,000`, `February 2026`

### ITEM 185

- Page ITEM: 185 of 303
- File: `maps.html`
- Line/context: L526 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > h3:nth-of-type(1)`
- Element/type: H3 heading
- Exact English:

  ```text
  Useful roles right now
  ```
- Protected tokens: None identified in this item.

### ITEM 186

- Page ITEM: 186 of 303
- File: `maps.html`
- Line/context: L528 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul.maps-check-list:nth-of-type(1) > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Save places before the trip.
  ```
- Protected tokens: None identified in this item.

### ITEM 187

- Page ITEM: 187 of 303
- File: `maps.html`
- Line/context: L529 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul.maps-check-list:nth-of-type(1) > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Organize shared lists.
  ```
- Protected tokens: None identified in this item.

### ITEM 188

- Page ITEM: 188 of 303
- File: `maps.html`
- Line/context: L530 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul.maps-check-list:nth-of-type(1) > li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Read international reviews.
  ```
- Protected tokens: None identified in this item.

### ITEM 189

- Page ITEM: 189 of 303
- File: `maps.html`
- Line/context: L531 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul.maps-check-list:nth-of-type(1) > li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Understand citywide location relationships.
  ```
- Protected tokens: None identified in this item.

### ITEM 190

- Page ITEM: 190 of 303
- File: `maps.html`
- Line/context: L532 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul.maps-check-list:nth-of-type(1) > li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  Share a familiar Google location.
  ```
- Protected tokens: `Google`

### ITEM 191

- Page ITEM: 191 of 303
- File: `maps.html`
- Line/context: L533 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul.maps-check-list:nth-of-type(1) > li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  Check features currently available on your device.
  ```
- Protected tokens: None identified in this item.

### ITEM 192

- Page ITEM: 192 of 303
- File: `maps.html`
- Line/context: L535 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > p.maps-caution:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Important Do not assume that every walking, driving or navigation feature is already fully available. Confirm the route on your device and keep Naver Map or KakaoMap ready.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`

### ITEM 193

- Page ITEM: 193 of 303
- File: `maps.html`
- Line/context: L536 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > h3:nth-of-type(2)`
- Element/type: H3 heading
- Exact English:

  ```text
  Google offline maps
  ```
- Protected tokens: `Google`

### ITEM 194

- Page ITEM: 194 of 303
- File: `maps.html`
- Line/context: L538 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul:nth-of-type(2) > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Some countries or regions may not allow a map download.
  ```
- Protected tokens: None identified in this item.

### ITEM 195

- Page ITEM: 195 of 303
- File: `maps.html`
- Line/context: L539 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul:nth-of-type(2) > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Offline transit, walking and cycling directions are unavailable.
  ```
- Protected tokens: None identified in this item.

### ITEM 196

- Page ITEM: 196 of 303
- File: `maps.html`
- Line/context: L540 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul:nth-of-type(2) > li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Mobile data remains important for live navigation and route changes.
  ```
- Protected tokens: None identified in this item.

### ITEM 197

- Page ITEM: 197 of 303
- File: `maps.html`
- Line/context: L547 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > div.maps-section__heading > div > h2#travel-scenarios-title`
- Element/type: H2 heading
- Exact English:

  ```text
  Use the Right Map in Real Travel Situations
  ```
- Protected tokens: None identified in this item.

### ITEM 198

- Page ITEM: 198 of 303
- File: `maps.html`
- Line/context: L548 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > div.maps-section__heading > div > p`
- Element/type: Body text
- Exact English:

  ```text
  Start with the information that can change the route: terminal, branch, exit, stop direction or operating time.
  ```
- Protected tokens: None identified in this item.

### ITEM 199

- Page ITEM: 199 of 303
- File: `maps.html`
- Line/context: L552 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Incheon Airport to your hotel Save the exact terminal and the hotel’s Korean address. Compare airport rail, bus and the final walk. Use the Airport Transfer Guide to compare the full journey. Check the Incheon Airport Guide and Arrival Guide if terminal or arrival steps affect the plan.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 200

- Page ITEM: 200 of 303
- File: `maps.html`
- Line/context: L554 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(1) > div > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Incheon Airport to your hotel
  ```
- Protected tokens: `Incheon Airport`

### ITEM 201

- Page ITEM: 201 of 303
- File: `maps.html`
- Line/context: L556 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(1) > div > ul > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Save the exact terminal and the hotel’s Korean address.
  ```
- Protected tokens: None identified in this item.

### ITEM 202

- Page ITEM: 202 of 303
- File: `maps.html`
- Line/context: L557 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(1) > div > ul > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Compare airport rail, bus and the final walk.
  ```
- Protected tokens: None identified in this item.

### ITEM 203

- Page ITEM: 203 of 303
- File: `maps.html`
- Line/context: L558 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(1) > div > ul > li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Use the Airport Transfer Guide to compare the full journey.
  ```
- Protected tokens: None identified in this item.

### ITEM 204

- Page ITEM: 204 of 303
- File: `maps.html`
- Line/context: L559 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(1) > div > ul > li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Check the Incheon Airport Guide and Arrival Guide if terminal or arrival steps affect the plan.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 205

- Page ITEM: 205 of 303
- File: `maps.html`
- Line/context: L563 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Hotel to a restaurant Search the Korean restaurant name. Confirm the branch, opening information and final walk. Use the Accommodation Guide when hotel location or station access is the real problem.
  ```
- Protected tokens: None identified in this item.

### ITEM 206

- Page ITEM: 206 of 303
- File: `maps.html`
- Line/context: L565 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(2) > div > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Hotel to a restaurant
  ```
- Protected tokens: None identified in this item.

### ITEM 207

- Page ITEM: 207 of 303
- File: `maps.html`
- Line/context: L567 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(2) > div > ul > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Search the Korean restaurant name.
  ```
- Protected tokens: None identified in this item.

### ITEM 208

- Page ITEM: 208 of 303
- File: `maps.html`
- Line/context: L568 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(2) > div > ul > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Confirm the branch, opening information and final walk.
  ```
- Protected tokens: None identified in this item.

### ITEM 209

- Page ITEM: 209 of 303
- File: `maps.html`
- Line/context: L569 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(2) > div > ul > li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Use the Accommodation Guide when hotel location or station access is the real problem.
  ```
- Protected tokens: None identified in this item.

### ITEM 210

- Page ITEM: 210 of 303
- File: `maps.html`
- Line/context: L573 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Finding the correct subway exit Check the exit before leaving the train. Review the street direction and whether you need an elevator.
  ```
- Protected tokens: None identified in this item.

### ITEM 211

- Page ITEM: 211 of 303
- File: `maps.html`
- Line/context: L575 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(3) > div > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Finding the correct subway exit
  ```
- Protected tokens: None identified in this item.

### ITEM 212

- Page ITEM: 212 of 303
- File: `maps.html`
- Line/context: L577 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(3) > div > ul > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Check the exit before leaving the train.
  ```
- Protected tokens: None identified in this item.

### ITEM 213

- Page ITEM: 213 of 303
- File: `maps.html`
- Line/context: L578 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(3) > div > ul > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Review the street direction and whether you need an elevator.
  ```
- Protected tokens: None identified in this item.

### ITEM 214

- Page ITEM: 214 of 303
- File: `maps.html`
- Line/context: L582 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Taking a local bus Confirm the stop direction, bus number and destination. Watch the final stop and walking segment.
  ```
- Protected tokens: None identified in this item.

### ITEM 215

- Page ITEM: 215 of 303
- File: `maps.html`
- Line/context: L584 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(4) > div > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Taking a local bus
  ```
- Protected tokens: None identified in this item.

### ITEM 216

- Page ITEM: 216 of 303
- File: `maps.html`
- Line/context: L586 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(4) > div > ul > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Confirm the stop direction, bus number and destination.
  ```
- Protected tokens: None identified in this item.

### ITEM 217

- Page ITEM: 217 of 303
- File: `maps.html`
- Line/context: L587 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(4) > div > ul > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Watch the final stop and walking segment.
  ```
- Protected tokens: None identified in this item.

### ITEM 218

- Page ITEM: 218 of 303
- File: `maps.html`
- Line/context: L591 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  Late-night return Confirm the last usable transit option. Cross-check a taxi pickup point and keep the Taxi Guide ready.
  ```
- Protected tokens: None identified in this item.

### ITEM 219

- Page ITEM: 219 of 303
- File: `maps.html`
- Line/context: L593 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(5) > div > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Late-night return
  ```
- Protected tokens: None identified in this item.

### ITEM 220

- Page ITEM: 220 of 303
- File: `maps.html`
- Line/context: L595 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(5) > div > ul > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Confirm the last usable transit option.
  ```
- Protected tokens: None identified in this item.

### ITEM 221

- Page ITEM: 221 of 303
- File: `maps.html`
- Line/context: L596 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(5) > div > ul > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Cross-check a taxi pickup point and keep the Taxi Guide ready.
  ```
- Protected tokens: None identified in this item.

### ITEM 222

- Page ITEM: 222 of 303
- File: `maps.html`
- Line/context: L600 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  Sharing a meeting point Share the exact place listing and Korean address or name. Do not send only a neighborhood name.
  ```
- Protected tokens: None identified in this item.

### ITEM 223

- Page ITEM: 223 of 303
- File: `maps.html`
- Line/context: L602 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(6) > div > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Sharing a meeting point
  ```
- Protected tokens: None identified in this item.

### ITEM 224

- Page ITEM: 224 of 303
- File: `maps.html`
- Line/context: L604 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(6) > div > ul > li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Share the exact place listing and Korean address or name.
  ```
- Protected tokens: None identified in this item.

### ITEM 225

- Page ITEM: 225 of 303
- File: `maps.html`
- Line/context: L605 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(6) > div > ul > li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Do not send only a neighborhood name.
  ```
- Protected tokens: None identified in this item.

### ITEM 226

- Page ITEM: 226 of 303
- File: `maps.html`
- Line/context: L615 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > div.maps-section__heading > div > h2#map-mistakes-title`
- Element/type: H2 heading
- Exact English:

  ```text
  Common Map Mistakes to Avoid
  ```
- Protected tokens: None identified in this item.

### ITEM 227

- Page ITEM: 227 of 303
- File: `maps.html`
- Line/context: L616 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > div.maps-section__heading > div > p`
- Element/type: Body text
- Exact English:

  ```text
  Small checks prevent the most common wrong-place and wrong-direction problems.
  ```
- Protected tokens: None identified in this item.

### ITEM 228

- Page ITEM: 228 of 303
- File: `maps.html`
- Line/context: L620 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(1) > strong :: text()[1]`
- Element/type: Visible list label
- Exact English:

  ```text
  Searching only in English
  ```
- Protected tokens: None identified in this item.

### ITEM 229

- Page ITEM: 229 of 303
- File: `maps.html`
- Line/context: L620 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(1) > span :: text()[1]`
- Element/type: Visible list description
- Exact English:

  ```text
  Keep the Korean name, road address and phone number ready.
  ```
- Protected tokens: None identified in this item.

### ITEM 230

- Page ITEM: 230 of 303
- File: `maps.html`
- Line/context: L621 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(2) > strong :: text()[1]`
- Element/type: Visible list label
- Exact English:

  ```text
  Choosing the wrong branch
  ```
- Protected tokens: None identified in this item.

### ITEM 231

- Page ITEM: 231 of 303
- File: `maps.html`
- Line/context: L621 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(2) > span :: text()[1]`
- Element/type: Visible list description
- Exact English:

  ```text
  Match the neighborhood, photos and nearest station.
  ```
- Protected tokens: None identified in this item.

### ITEM 232

- Page ITEM: 232 of 303
- File: `maps.html`
- Line/context: L622 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(3) > strong :: text()[1]`
- Element/type: Visible list label
- Exact English:

  ```text
  Ignoring the subway exit
  ```
- Protected tokens: None identified in this item.

### ITEM 233

- Page ITEM: 233 of 303
- File: `maps.html`
- Line/context: L622 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(3) > span :: text()[1]`
- Element/type: Visible list description
- Exact English:

  ```text
  Confirm the exit before you leave the train.
  ```
- Protected tokens: None identified in this item.

### ITEM 234

- Page ITEM: 234 of 303
- File: `maps.html`
- Line/context: L623 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(4) > strong :: text()[1]`
- Element/type: Visible list label
- Exact English:

  ```text
  Using scheduled arrival instead of actual departure
  ```
- Protected tokens: None identified in this item.

### ITEM 235

- Page ITEM: 235 of 303
- File: `maps.html`
- Line/context: L623 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(4) > span :: text()[1]`
- Element/type: Visible list description
- Exact English:

  ```text
  Set the route for the time you will really leave.
  ```
- Protected tokens: None identified in this item.

### ITEM 236

- Page ITEM: 236 of 303
- File: `maps.html`
- Line/context: L624 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(5) > strong :: text()[1]`
- Element/type: Visible list label
- Exact English:

  ```text
  Comparing only total travel time
  ```
- Protected tokens: None identified in this item.

### ITEM 237

- Page ITEM: 237 of 303
- File: `maps.html`
- Line/context: L624 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(5) > span :: text()[1]`
- Element/type: Visible list description
- Exact English:

  ```text
  Check transfers, waiting and reliability too.
  ```
- Protected tokens: None identified in this item.

### ITEM 238

- Page ITEM: 238 of 303
- File: `maps.html`
- Line/context: L625 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(6) > strong :: text()[1]`
- Element/type: Visible list label
- Exact English:

  ```text
  Ignoring the final walk with luggage
  ```
- Protected tokens: None identified in this item.

### ITEM 239

- Page ITEM: 239 of 303
- File: `maps.html`
- Line/context: L625 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(6) > span :: text()[1]`
- Element/type: Visible list description
- Exact English:

  ```text
  A short train trip can still end with a difficult walk.
  ```
- Protected tokens: None identified in this item.

### ITEM 240

- Page ITEM: 240 of 303
- File: `maps.html`
- Line/context: L626 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(7) > strong :: text()[1]`
- Element/type: Visible list label
- Exact English:

  ```text
  Selecting the wrong side of a bus stop
  ```
- Protected tokens: None identified in this item.

### ITEM 241

- Page ITEM: 241 of 303
- File: `maps.html`
- Line/context: L626 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(7) > span :: text()[1]`
- Element/type: Visible list description
- Exact English:

  ```text
  Verify the direction or stop number before boarding.
  ```
- Protected tokens: None identified in this item.

### ITEM 242

- Page ITEM: 242 of 303
- File: `maps.html`
- Line/context: L627 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(8) > strong :: text()[1]`
- Element/type: Visible list label
- Exact English:

  ```text
  Relying on one app without a backup
  ```
- Protected tokens: None identified in this item.

### ITEM 243

- Page ITEM: 243 of 303
- File: `maps.html`
- Line/context: L627 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(8) > span :: text()[1]`
- Element/type: Visible list description
- Exact English:

  ```text
  Use KakaoMap or another listing when a result is unclear.
  ```
- Protected tokens: `KakaoMap`

### ITEM 244

- Page ITEM: 244 of 303
- File: `maps.html`
- Line/context: L628 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(9) > strong :: text()[1]`
- Element/type: Visible list label
- Exact English:

  ```text
  Leaving the airport without mobile data
  ```
- Protected tokens: None identified in this item.

### ITEM 245

- Page ITEM: 245 of 303
- File: `maps.html`
- Line/context: L628 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(9) > span :: text()[1]`
- Element/type: Visible list description
- Exact English:

  ```text
  Activate your eSIM, SIM or roaming plan before live navigation.
  ```
- Protected tokens: `eSIM`, `SIM`

### ITEM 246

- Page ITEM: 246 of 303
- File: `maps.html`
- Line/context: L635 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-section__heading:nth-of-type(1) > div > h2#download-title`
- Element/type: H2 heading
- Exact English:

  ```text
  Download the Map Apps Before Your Trip
  ```
- Protected tokens: None identified in this item.

### ITEM 247

- Page ITEM: 247 of 303
- File: `maps.html`
- Line/context: L636 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-section__heading:nth-of-type(1) > div > p`
- Element/type: Body text
- Exact English:

  ```text
  Use only the official store listing or service website.
  ```
- Protected tokens: None identified in this item.

### ITEM 248

- Page ITEM: 248 of 303
- File: `maps.html`
- Line/context: L642 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(1) > div:nth-of-type(1) > h3#download-naver-title`
- Element/type: H3 heading
- Exact English:

  ```text
  Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 249

- Page ITEM: 249 of 303
- File: `maps.html`
- Line/context: L643 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(1) > div:nth-of-type(1) > p`
- Element/type: Body text
- Exact English:

  ```text
  Main navigation
  ```
- Protected tokens: None identified in this item.

### ITEM 250

- Page ITEM: 250 of 303
- File: `maps.html`
- Line/context: L646 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(1) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(1)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  App Store
  ```
- Protected tokens: `App Store`

### ITEM 251

- Page ITEM: 251 of 303
- File: `maps.html`
- Line/context: L647 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(1) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(2)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Google Play
  ```
- Protected tokens: `Google Play`, `Google`

### ITEM 252

- Page ITEM: 252 of 303
- File: `maps.html`
- Line/context: L648 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(1) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(3)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Official website
  ```
- Protected tokens: None identified in this item.

### ITEM 253

- Page ITEM: 253 of 303
- File: `maps.html`
- Line/context: L653 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(2) > div:nth-of-type(1) > h3#download-kakao-title`
- Element/type: H3 heading
- Exact English:

  ```text
  KakaoMap
  ```
- Protected tokens: `KakaoMap`

### ITEM 254

- Page ITEM: 254 of 303
- File: `maps.html`
- Line/context: L654 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(2) > div:nth-of-type(1) > p`
- Element/type: Body text
- Exact English:

  ```text
  Backup local map
  ```
- Protected tokens: None identified in this item.

### ITEM 255

- Page ITEM: 255 of 303
- File: `maps.html`
- Line/context: L657 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(2) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(1)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  App Store
  ```
- Protected tokens: `App Store`

### ITEM 256

- Page ITEM: 256 of 303
- File: `maps.html`
- Line/context: L658 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(2) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(2)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Google Play
  ```
- Protected tokens: `Google Play`, `Google`

### ITEM 257

- Page ITEM: 257 of 303
- File: `maps.html`
- Line/context: L659 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(2) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(3)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Official website
  ```
- Protected tokens: None identified in this item.

### ITEM 258

- Page ITEM: 258 of 303
- File: `maps.html`
- Line/context: L664 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(3) > div:nth-of-type(1) > h3#download-google-title`
- Element/type: H3 heading
- Exact English:

  ```text
  Google Maps
  ```
- Protected tokens: `Google Maps`, `Google`

### ITEM 259

- Page ITEM: 259 of 303
- File: `maps.html`
- Line/context: L665 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(3) > div:nth-of-type(1) > p`
- Element/type: Body text
- Exact English:

  ```text
  Planning and saved places
  ```
- Protected tokens: None identified in this item.

### ITEM 260

- Page ITEM: 260 of 303
- File: `maps.html`
- Line/context: L668 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(3) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(1)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  App Store
  ```
- Protected tokens: `App Store`

### ITEM 261

- Page ITEM: 261 of 303
- File: `maps.html`
- Line/context: L669 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(3) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(2)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Google Play
  ```
- Protected tokens: `Google Play`, `Google`

### ITEM 262

- Page ITEM: 262 of 303
- File: `maps.html`
- Line/context: L670 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(3) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(3)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Official website
  ```
- Protected tokens: None identified in this item.

### ITEM 263

- Page ITEM: 263 of 303
- File: `maps.html`
- Line/context: L679 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > div.maps-section__heading > div > h2#faq-title`
- Element/type: H2 heading
- Exact English:

  ```text
  Frequently Asked Questions
  ```
- Protected tokens: None identified in this item.

### ITEM 264

- Page ITEM: 264 of 303
- File: `maps.html`
- Line/context: L680 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > div.maps-section__heading > div > p`
- Element/type: Body text
- Exact English:

  ```text
  Short answers to the map problems travelers most often need to solve.
  ```
- Protected tokens: None identified in this item.

### ITEM 265

- Page ITEM: 265 of 303
- File: `maps.html`
- Line/context: L684 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(1) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What is the best map app for Korea?
  ```
- Protected tokens: `Korea`

### ITEM 266

- Page ITEM: 266 of 303
- File: `maps.html`
- Line/context: L685 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(1) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Naver Map is the best first choice for most travelers because it combines local place search with walking, subway and bus routes. Keep KakaoMap as a backup and Google Maps for planning, saved places and international reviews.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`, `Google Maps`, `Google`

### ITEM 267

- Page ITEM: 267 of 303
- File: `maps.html`
- Line/context: L688 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(2) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Is Naver Map available in English?
  ```
- Protected tokens: `Naver Map`

### ITEM 268

- Page ITEM: 268 of 303
- File: `maps.html`
- Line/context: L689 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(2) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Yes. In the app, open MY, then Settings, Language and English. On iPhone, the app may send you to Settings, NAVER Map and Preferred Language.
  ```
- Protected tokens: `iPhone`, `NAVER Map`, `NAVER`, `Preferred Language`

### ITEM 269

- Page ITEM: 269 of 303
- File: `maps.html`
- Line/context: L692 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(3) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Does Google Maps work in Korea?
  ```
- Protected tokens: `Google Maps`, `Google`, `Korea`

### ITEM 270

- Page ITEM: 270 of 303
- File: `maps.html`
- Line/context: L693 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(3) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Yes. It is useful for saved places, shared lists, international reviews and broad trip planning. Route and navigation availability can still vary, so confirm the exact route on your device and keep a local map app ready.
  ```
- Protected tokens: None identified in this item.

### ITEM 271

- Page ITEM: 271 of 303
- File: `maps.html`
- Line/context: L696 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(4) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Should I install KakaoMap too?
  ```
- Protected tokens: `KakaoMap`

### ITEM 272

- Page ITEM: 272 of 303
- File: `maps.html`
- Line/context: L697 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(4) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  It is a useful backup, not a requirement for every traveler. Install it if you want to cross-check a place, compare a walking route, verify bus information or open a Kakao place link.
  ```
- Protected tokens: `Kakao`

### ITEM 273

- Page ITEM: 273 of 303
- File: `maps.html`
- Line/context: L700 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(5) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What should I do when English search fails?
  ```
- Protected tokens: None identified in this item.

### ITEM 274

- Page ITEM: 274 of 303
- File: `maps.html`
- Line/context: L701 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(5) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Find the official Korean place name or road address in your booking, the business website or another reliable listing. Paste it into Naver Map, then confirm the address, phone number and photos.
  ```
- Protected tokens: `Naver Map`

### ITEM 275

- Page ITEM: 275 of 303
- File: `maps.html`
- Line/context: L704 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(6) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  How do I find the correct subway exit?
  ```
- Protected tokens: None identified in this item.

### ITEM 276

- Page ITEM: 276 of 303
- File: `maps.html`
- Line/context: L705 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(6) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Open the route before leaving the train and confirm the exit number, the side of the road and the remaining walk. At a large station, also check whether you need an elevator and how far the indoor walk is.
  ```
- Protected tokens: None identified in this item.

### ITEM 277

- Page ITEM: 277 of 303
- File: `maps.html`
- Line/context: L708 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(7) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  How do I avoid choosing the wrong bus stop?
  ```
- Protected tokens: None identified in this item.

### ITEM 278

- Page ITEM: 278 of 303
- File: `maps.html`
- Line/context: L709 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(7) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Check the stop number or direction, the bus destination and the side of the road before boarding. Similar stop names can refer to opposite travel directions.
  ```
- Protected tokens: None identified in this item.

### ITEM 279

- Page ITEM: 279 of 303
- File: `maps.html`
- Line/context: L712 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(8) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Can I use offline maps in Korea?
  ```
- Protected tokens: `Korea`

### ITEM 280

- Page ITEM: 280 of 303
- File: `maps.html`
- Line/context: L713 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(8) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Google says offline map downloads are unavailable in some countries or regions, and offline transit, walking and cycling directions are unavailable. Check download availability on your device and keep mobile data for live navigation.
  ```
- Protected tokens: `Google`

### ITEM 281

- Page ITEM: 281 of 303
- File: `maps.html`
- Line/context: L716 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(9) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  How do I save and share my hotel address?
  ```
- Protected tokens: None identified in this item.

### ITEM 282

- Page ITEM: 282 of 303
- File: `maps.html`
- Line/context: L717 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(9) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Save the exact hotel listing in Naver Map, copy its Korean name and road address, and share the place link with your travel companions. Also keep the text in your booking confirmation.
  ```
- Protected tokens: `Naver Map`

### ITEM 283

- Page ITEM: 283 of 303
- File: `maps.html`
- Line/context: L720 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(10) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Do I need mobile data for map apps in Korea?
  ```
- Protected tokens: `Korea`

### ITEM 284

- Page ITEM: 284 of 303
- File: `maps.html`
- Line/context: L721 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(10) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Mobile data is strongly recommended for route changes, live transit information, place details and sharing. Arrange an eSIM, SIM or roaming plan before you depend on live navigation.
  ```
- Protected tokens: `eSIM`, `SIM`

### ITEM 285

- Page ITEM: 285 of 303
- File: `maps.html`
- Line/context: L728 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > div.maps-section__heading > div > h2#sources-title`
- Element/type: H2 heading
- Exact English:

  ```text
  Official Sources
  ```
- Protected tokens: None identified in this item.

### ITEM 286

- Page ITEM: 286 of 303
- File: `maps.html`
- Line/context: L729 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > div.maps-section__heading > div > p`
- Element/type: Body text
- Exact English:

  ```text
  Provider documentation supports the product instructions below. Korea Inside’s app order and backup advice are editorial recommendations.
  ```
- Protected tokens: `Korea Inside`, `Korea`

### ITEM 287

- Page ITEM: 287 of 303
- File: `maps.html`
- Line/context: L733 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(1) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  NAVER Map Help: app language settings
  ```
- Protected tokens: `NAVER Map Help`, `NAVER Map`, `NAVER`

### ITEM 288

- Page ITEM: 288 of 303
- File: `maps.html`
- Line/context: L733 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(1) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official Android and iPhone paths for changing the app language.
  ```
- Protected tokens: `Android`, `iPhone`

### ITEM 289

- Page ITEM: 289 of 303
- File: `maps.html`
- Line/context: L734 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(2) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  Google Maps Help: get directions and show routes
  ```
- Protected tokens: `Google Maps Help`, `Google Maps`, `Google`

### ITEM 290

- Page ITEM: 290 of 303
- File: `maps.html`
- Line/context: L734 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(2) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official guidance on route modes and feature availability.
  ```
- Protected tokens: None identified in this item.

### ITEM 291

- Page ITEM: 291 of 303
- File: `maps.html`
- Line/context: L735 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(3) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  Google Maps Help: download areas and navigate offline
  ```
- Protected tokens: `Google Maps Help`, `Google Maps`, `Google`

### ITEM 292

- Page ITEM: 292 of 303
- File: `maps.html`
- Line/context: L735 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(3) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official download limits and unavailable offline route modes.
  ```
- Protected tokens: None identified in this item.

### ITEM 293

- Page ITEM: 293 of 303
- File: `maps.html`
- Line/context: L736 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(4) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  Republic of Korea policy briefing: Google 1:5,000 map export approval
  ```
- Protected tokens: `Republic of Korea policy briefing`, `Republic of Korea`, `Korea`, `Google`, `1:5,000`

### ITEM 294

- Page ITEM: 294 of 303
- File: `maps.html`
- Line/context: L736 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(4) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Government decision and the security conditions that apply before data export.
  ```
- Protected tokens: None identified in this item.

### ITEM 295

- Page ITEM: 295 of 303
- File: `maps.html`
- Line/context: L737 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(5) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  Kakao Corp: KakaoMap official feature documentation
  ```
- Protected tokens: `Kakao Corp`, `Kakao`, `KakaoMap`

### ITEM 296

- Page ITEM: 296 of 303
- File: `maps.html`
- Line/context: L737 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(5) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official route, navigation and public-transport feature overview.
  ```
- Protected tokens: None identified in this item.

### ITEM 297

- Page ITEM: 297 of 303
- File: `maps.html`
- Line/context: L739 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(6) > span.maps-source-group-title:nth-of-type(1) :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official app store pages
  ```
- Protected tokens: None identified in this item.

### ITEM 298

- Page ITEM: 298 of 303
- File: `maps.html`
- Line/context: L741 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(6) > span.maps-source-links:nth-of-type(2) > a:nth-of-type(1)`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  Naver Map for iPhone
  ```
- Protected tokens: `Naver Map`, `iPhone`

### ITEM 299

- Page ITEM: 299 of 303
- File: `maps.html`
- Line/context: L742 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(6) > span.maps-source-links:nth-of-type(2) > a:nth-of-type(2)`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  Naver Map for Android
  ```
- Protected tokens: `Naver Map`, `Android`

### ITEM 300

- Page ITEM: 300 of 303
- File: `maps.html`
- Line/context: L743 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(6) > span.maps-source-links:nth-of-type(2) > a:nth-of-type(3)`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  KakaoMap for iPhone
  ```
- Protected tokens: `KakaoMap`, `iPhone`

### ITEM 301

- Page ITEM: 301 of 303
- File: `maps.html`
- Line/context: L744 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(6) > span.maps-source-links:nth-of-type(2) > a:nth-of-type(4)`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  KakaoMap for Android
  ```
- Protected tokens: `KakaoMap`, `Android`

### ITEM 302

- Page ITEM: 302 of 303
- File: `maps.html`
- Line/context: L745 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(6) > span.maps-source-links:nth-of-type(2) > a:nth-of-type(5)`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  Google Maps for iPhone
  ```
- Protected tokens: `Google Maps`, `Google`, `iPhone`

### ITEM 303

- Page ITEM: 303 of 303
- File: `maps.html`
- Line/context: L746 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(6) > span.maps-source-links:nth-of-type(2) > a:nth-of-type(6)`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  Google Maps for Android
  ```
- Protected tokens: `Google Maps`, `Google`, `Android`


## PAGE - apps.html

- English source: `apps.html`
- Source SHA-256: `b80f02a398c9b562baa3bfc8977961c25fe4081f92f6262ed3945fcbd185b734`
- Extracted ITEM count: 246

### ITEM 304

- Page ITEM: 001 of 246
- File: `apps.html`
- Line/context: L6 - `html > head > meta[name="description"] @content`
- Element/type: Meta description
- Exact English:

  ```text
  The essential apps for Korea travel, including Naver Map, Papago, k.ride, restaurant booking, food delivery, payments, trains and emergency help for foreign visitors.
  ```
- Protected tokens: `Korea`, `Naver Map`, `Papago`, `k.ride`

### ITEM 305

- Page ITEM: 002 of 246
- File: `apps.html`
- Line/context: L9 - `html > head > title`
- Element/type: Title
- Exact English:

  ```text
  Best Apps for Korea Travel: Maps, Taxis & Translation | Korea Inside
  ```
- Protected tokens: `Korea`, `Korea Inside`

### ITEM 306

- Page ITEM: 003 of 246
- File: `apps.html`
- Line/context: L12 - `html > head > meta[property="og:title"] @content`
- Element/type: Open Graph title
- Exact English:

  ```text
  Best Apps for Korea Travel: Maps, Taxis & Translation
  ```
- Protected tokens: `Korea`

### ITEM 307

- Page ITEM: 004 of 246
- File: `apps.html`
- Line/context: L13 - `html > head > meta[property="og:description"] @content`
- Element/type: Open Graph description
- Exact English:

  ```text
  Choose the essential Korea travel apps for maps, translation, taxis, restaurants, delivery, payments, trains and emergency help.
  ```
- Protected tokens: `Korea`

### ITEM 308

- Page ITEM: 005 of 246
- File: `apps.html`
- Line/context: L17 - `html > head > meta[name="twitter:title"] @content`
- Element/type: Twitter card title
- Exact English:

  ```text
  Best Apps for Korea Travel: Maps, Taxis & Translation
  ```
- Protected tokens: `Korea`

### ITEM 309

- Page ITEM: 006 of 246
- File: `apps.html`
- Line/context: L18 - `html > head > meta[name="twitter:description"] @content`
- Element/type: Twitter card description
- Exact English:

  ```text
  Choose the essential Korea travel apps for maps, translation, taxis, restaurants, delivery, payments, trains and emergency help.
  ```
- Protected tokens: `Korea`

### ITEM 310

- Page ITEM: 007 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[0].name`
- Element/type: JSON-LD user-facing name
- Exact English:

  ```text
  Best Apps for Korea Travel: Maps, Taxis & Translation | Korea Inside
  ```
- Protected tokens: `Korea`, `Korea Inside`

### ITEM 311

- Page ITEM: 008 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[0].description`
- Element/type: JSON-LD user-facing description
- Exact English:

  ```text
  The essential apps for Korea travel, including Naver Map, Papago, k.ride, restaurant booking, food delivery, payments, trains and emergency help for foreign visitors.
  ```
- Protected tokens: `Korea`, `Naver Map`, `Papago`, `k.ride`

### ITEM 312

- Page ITEM: 009 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[0].isPartOf.name`
- Element/type: JSON-LD user-facing name
- Exact English:

  ```text
  Korea Inside
  ```
- Protected tokens: `Korea Inside`, `Korea`

### ITEM 313

- Page ITEM: 010 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[1].itemListElement[0].name`
- Element/type: JSON-LD user-facing name
- Exact English:

  ```text
  Home
  ```
- Protected tokens: None identified in this item.

### ITEM 314

- Page ITEM: 011 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[1].itemListElement[1].name`
- Element/type: JSON-LD user-facing name
- Exact English:

  ```text
  Apps
  ```
- Protected tokens: None identified in this item.

### ITEM 315

- Page ITEM: 012 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[0].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  What apps should I install before traveling to Korea?
  ```
- Protected tokens: `Korea`

### ITEM 316

- Page ITEM: 013 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[0].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Most first-time visitors should install Naver Map, Papago and k.ride before traveling to Korea. Add Catchtable, Shuttle Delivery, KakaoTalk, payment or rail apps only when your plans need them.
  ```
- Protected tokens: `Naver Map`, `Papago`, `k.ride`, `Korea`, `Catchtable`, `Shuttle Delivery`, `Shuttle`, `KakaoTalk`

### ITEM 317

- Page ITEM: 014 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[1].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  What is the best map app for Korea?
  ```
- Protected tokens: `Korea`

### ITEM 318

- Page ITEM: 015 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[1].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Naver Map is the best default map app for most visitors because it provides detailed local search and routes for subway, bus, walking and driving. Keep KakaoMap as an optional cross-check.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`

### ITEM 319

- Page ITEM: 016 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[2].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Does Google Maps work in Korea?
  ```
- Protected tokens: `Google Maps`, `Google`, `Korea`

### ITEM 320

- Page ITEM: 017 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[2].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Google Maps works for saved places and basic orientation, but local search and route details can be less dependable in Korea. Use Naver Map as your main navigation app and check a Korean place name or address when search fails.
  ```
- Protected tokens: `Google Maps`, `Google`, `Korea`, `Naver Map`

### ITEM 321

- Page ITEM: 018 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[3].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  What is the best translation app for Korea?
  ```
- Protected tokens: `Korea`

### ITEM 322

- Page ITEM: 019 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[3].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Papago is the best first translation app for many Korea trips. It supports text, image, voice, conversation and offline translation, although names, slang and menu context still need judgment.
  ```
- Protected tokens: `Papago`, `Korea`

### ITEM 323

- Page ITEM: 020 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[4].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  What taxi app is easiest for foreign visitors?
  ```
- Protected tokens: None identified in this item.

### ITEM 324

- Page ITEM: 021 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[4].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  k.ride is the easiest default for many foreign visitors because it is designed for international travelers and supports multilingual destination search, translated driver chat and overseas-issued card registration. Kakao T and Uber Taxi remain useful alternatives.
  ```
- Protected tokens: `k.ride`, `Kakao T`, `Kakao`, `Uber Taxi`

### ITEM 325

- Page ITEM: 022 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[5].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Do I need KakaoTalk as a tourist?
  ```
- Protected tokens: `KakaoTalk`

### ITEM 326

- Page ITEM: 023 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[5].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  You need KakaoTalk only if you expect to message Korean friends, hosts, tour operators or local businesses. Travelers with no local contacts can usually skip it.
  ```
- Protected tokens: `KakaoTalk`

### ITEM 327

- Page ITEM: 024 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[6].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  What app should I use to book restaurants?
  ```
- Protected tokens: None identified in this item.

### ITEM 328

- Page ITEM: 025 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[6].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Use Catchtable when a restaurant accepts reservations or waitlist registration through the service. Confirm the branch, date, party size, deposit and cancellation terms before booking.
  ```
- Protected tokens: `Catchtable`

### ITEM 329

- Page ITEM: 026 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[7].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Can tourists order food delivery in Korea?
  ```
- Protected tokens: `Korea`

### ITEM 330

- Page ITEM: 027 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[7].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Tourists can order food delivery in some areas. Shuttle Delivery is the simplest first option for many foreign visitors because it supports international users and cards, but coverage varies; hotel help, pickup and direct restaurant visits are reliable backups.
  ```
- Protected tokens: `Shuttle Delivery`, `Shuttle`

### ITEM 331

- Page ITEM: 028 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[8].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Do Korean apps require a Korean phone number?
  ```
- Protected tokens: None identified in this item.

### ITEM 332

- Page ITEM: 029 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[8].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Some Korean apps work with a foreign number, while others require Korean SMS, identity or payment verification for specific features. Test sign-in and recovery before departure and never assume a data-only eSIM provides a Korean phone number.
  ```
- Protected tokens: `SMS`, `eSIM`

### ITEM 333

- Page ITEM: 030 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[9].name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Which apps should I set up before my flight?
  ```
- Protected tokens: None identified in this item.

### ITEM 334

- Page ITEM: 031 of 246
- File: `apps.html`
- Line/context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[9].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Before your flight, set up Naver Map, Papago and k.ride, then test login, language, permissions and payment. Save your hotel's Korean name, road address and phone number, plus screenshots of important bookings.
  ```
- Protected tokens: `Naver Map`, `Papago`, `k.ride`

### ITEM 335

- Page ITEM: 032 of 246
- File: `apps.html`
- Line/context: L217 - `html > body > main > section.apps-hero > div.container > div.apps-hero__grid > div.apps-hero__copy:nth-of-type(1) > h1#apps-title`
- Element/type: H1 heading
- Exact English:

  ```text
  Best Apps for Korea Travel: What to Install Before You Arrive
  ```
- Protected tokens: `Korea`

### ITEM 336

- Page ITEM: 033 of 246
- File: `apps.html`
- Line/context: L218 - `html > body > main > section.apps-hero > div.container > div.apps-hero__grid > div.apps-hero__copy:nth-of-type(1) > p.apps-hero__intro`
- Element/type: Hero / lead copy
- Exact English:

  ```text
  You do not need a phone full of Korean apps for a short trip. Naver Map handles most local navigation, Papago helps when Korean text or speech becomes the obstacle, and k.ride covers the moments when a taxi is easier than public transport.
  ```
- Protected tokens: `Naver Map`, `Papago`, `k.ride`

### ITEM 337

- Page ITEM: 034 of 246
- File: `apps.html`
- Line/context: L220 - `html > body > main > section.apps-hero > div.container > div.apps-hero__grid > div.apps-hero__answer:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Apps to install before landing
  ```
- Protected tokens: None identified in this item.

### ITEM 338

- Page ITEM: 035 of 246
- File: `apps.html`
- Line/context: L221 - `html > body > main > section.apps-hero > div.container > div.apps-hero__grid > div.apps-hero__answer:nth-of-type(2) > p`
- Element/type: Hero / lead copy
- Exact English:

  ```text
  KakaoTalk matters when a local contact expects to use it. Restaurant, delivery, payment, rail and safety apps can wait until the itinerary gives them a specific job.
  ```
- Protected tokens: `KakaoTalk`

### ITEM 339

- Page ITEM: 036 of 246
- File: `apps.html`
- Line/context: L231 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > header.apps-section__heading > h2#apps-at-a-glance-title`
- Element/type: H2 heading
- Exact English:

  ```text
  Apps Worth Knowing Before Korea
  ```
- Protected tokens: `Korea`

### ITEM 340

- Page ITEM: 037 of 246
- File: `apps.html`
- Line/context: L232 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Exact English:

  ```text
  “Install before arrival” means complete the basic setup and test the feature you intend to use, not merely download the app.
  ```
- Protected tokens: None identified in this item.

### ITEM 341

- Page ITEM: 038 of 246
- File: `apps.html`
- Line/context: L234 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Korea travel apps comparison
  ```
- Protected tokens: `Korea`

### ITEM 342

- Page ITEM: 039 of 246
- File: `apps.html`
- Line/context: L238 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > thead > tr > th:nth-of-type(1)`
- Element/type: Table header
- Exact English:

  ```text
  App
  ```
- Protected tokens: None identified in this item.

### ITEM 343

- Page ITEM: 040 of 246
- File: `apps.html`
- Line/context: L239 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > thead > tr > th:nth-of-type(2)`
- Element/type: Table header
- Exact English:

  ```text
  What it helps with
  ```
- Protected tokens: None identified in this item.

### ITEM 344

- Page ITEM: 041 of 246
- File: `apps.html`
- Line/context: L240 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > thead > tr > th:nth-of-type(3)`
- Element/type: Table header
- Exact English:

  ```text
  When setup matters
  ```
- Protected tokens: None identified in this item.

### ITEM 345

- Page ITEM: 042 of 246
- File: `apps.html`
- Line/context: L241 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > thead > tr > th:nth-of-type(4)`
- Element/type: Table header
- Exact English:

  ```text
  What can get in the way
  ```
- Protected tokens: None identified in this item.

### ITEM 346

- Page ITEM: 043 of 246
- File: `apps.html`
- Line/context: L246 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(1) > th`
- Element/type: Table header
- Exact English:

  ```text
  Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 347

- Page ITEM: 044 of 246
- File: `apps.html`
- Line/context: L247 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(1) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Local search and routes
  ```
- Protected tokens: None identified in this item.

### ITEM 348

- Page ITEM: 045 of 246
- File: `apps.html`
- Line/context: L248 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(1) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Yes
  ```
- Protected tokens: None identified in this item.

### ITEM 349

- Page ITEM: 046 of 246
- File: `apps.html`
- Line/context: L249 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(1) > td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  English searches can miss a place that is listed under its Korean name.
  ```
- Protected tokens: None identified in this item.

### ITEM 350

- Page ITEM: 047 of 246
- File: `apps.html`
- Line/context: L252 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(2) > th`
- Element/type: Table header
- Exact English:

  ```text
  Papago
  ```
- Protected tokens: `Papago`

### ITEM 351

- Page ITEM: 048 of 246
- File: `apps.html`
- Line/context: L253 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(2) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Menus, signs and short conversations
  ```
- Protected tokens: None identified in this item.

### ITEM 352

- Page ITEM: 049 of 246
- File: `apps.html`
- Line/context: L254 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(2) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Yes
  ```
- Protected tokens: None identified in this item.

### ITEM 353

- Page ITEM: 050 of 246
- File: `apps.html`
- Line/context: L255 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(2) > td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  Machine translation can miss names, slang and context.
  ```
- Protected tokens: None identified in this item.

### ITEM 354

- Page ITEM: 051 of 246
- File: `apps.html`
- Line/context: L258 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(3) > th`
- Element/type: Table header
- Exact English:

  ```text
  k.ride
  ```
- Protected tokens: `k.ride`

### ITEM 355

- Page ITEM: 052 of 246
- File: `apps.html`
- Line/context: L259 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(3) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Taxi calls for international visitors
  ```
- Protected tokens: None identified in this item.

### ITEM 356

- Page ITEM: 053 of 246
- File: `apps.html`
- Line/context: L260 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(3) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Yes
  ```
- Protected tokens: None identified in this item.

### ITEM 357

- Page ITEM: 054 of 246
- File: `apps.html`
- Line/context: L261 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(3) > td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  Vehicle availability and payment success can vary by place and time.
  ```
- Protected tokens: None identified in this item.

### ITEM 358

- Page ITEM: 055 of 246
- File: `apps.html`
- Line/context: L264 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(4) > th`
- Element/type: Table header
- Exact English:

  ```text
  KakaoTalk
  ```
- Protected tokens: `KakaoTalk`

### ITEM 359

- Page ITEM: 056 of 246
- File: `apps.html`
- Line/context: L265 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(4) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Messaging Korean contacts
  ```
- Protected tokens: None identified in this item.

### ITEM 360

- Page ITEM: 057 of 246
- File: `apps.html`
- Line/context: L266 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(4) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Only if needed
  ```
- Protected tokens: None identified in this item.

### ITEM 361

- Page ITEM: 058 of 246
- File: `apps.html`
- Line/context: L267 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(4) > td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  Phone verification and account recovery need advance testing.
  ```
- Protected tokens: None identified in this item.

### ITEM 362

- Page ITEM: 059 of 246
- File: `apps.html`
- Line/context: L270 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(5) > th`
- Element/type: Table header
- Exact English:

  ```text
  Catchtable
  ```
- Protected tokens: `Catchtable`

### ITEM 363

- Page ITEM: 060 of 246
- File: `apps.html`
- Line/context: L271 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(5) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Restaurant reservations and waitlists
  ```
- Protected tokens: None identified in this item.

### ITEM 364

- Page ITEM: 061 of 246
- File: `apps.html`
- Line/context: L272 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(5) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  If dining is a priority
  ```
- Protected tokens: None identified in this item.

### ITEM 365

- Page ITEM: 062 of 246
- File: `apps.html`
- Line/context: L273 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(5) > td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  Not every restaurant or time slot is available.
  ```
- Protected tokens: None identified in this item.

### ITEM 366

- Page ITEM: 063 of 246
- File: `apps.html`
- Line/context: L276 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(6) > th`
- Element/type: Table header
- Exact English:

  ```text
  Shuttle Delivery
  ```
- Protected tokens: `Shuttle Delivery`, `Shuttle`

### ITEM 367

- Page ITEM: 064 of 246
- File: `apps.html`
- Line/context: L277 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(6) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Foreigner-friendly food delivery
  ```
- Protected tokens: None identified in this item.

### ITEM 368

- Page ITEM: 065 of 246
- File: `apps.html`
- Line/context: L278 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(6) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  If you plan to order
  ```
- Protected tokens: None identified in this item.

### ITEM 369

- Page ITEM: 066 of 246
- File: `apps.html`
- Line/context: L279 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(6) > td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  Restaurant choice and delivery coverage are location-dependent.
  ```
- Protected tokens: None identified in this item.

### ITEM 370

- Page ITEM: 067 of 246
- File: `apps.html`
- Line/context: L282 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(7) > th`
- Element/type: Table header
- Exact English:

  ```text
  WOWPASS
  ```
- Protected tokens: `WOWPASS`

### ITEM 371

- Page ITEM: 068 of 246
- File: `apps.html`
- Line/context: L283 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(7) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Prepaid payment backup and card management
  ```
- Protected tokens: None identified in this item.

### ITEM 372

- Page ITEM: 069 of 246
- File: `apps.html`
- Line/context: L284 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(7) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  If it fits your payment plan
  ```
- Protected tokens: None identified in this item.

### ITEM 373

- Page ITEM: 070 of 246
- File: `apps.html`
- Line/context: L285 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(7) > td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  Payment and T-money balances are separate.
  ```
- Protected tokens: `T-money`

### ITEM 374

- Page ITEM: 071 of 246
- File: `apps.html`
- Line/context: L288 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(8) > th`
- Element/type: Table header
- Exact English:

  ```text
  Mobile Tmoney
  ```
- Protected tokens: `Mobile Tmoney`, `Tmoney`

### ITEM 375

- Page ITEM: 072 of 246
- File: `apps.html`
- Line/context: L289 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(8) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Phone-based transit payment
  ```
- Protected tokens: None identified in this item.

### ITEM 376

- Page ITEM: 073 of 246
- File: `apps.html`
- Line/context: L290 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(8) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Only after a compatibility check
  ```
- Protected tokens: None identified in this item.

### ITEM 377

- Page ITEM: 074 of 246
- File: `apps.html`
- Line/context: L291 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(8) > td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  Phone, wallet, NFC and top-up requirements vary by device.
  ```
- Protected tokens: `NFC`

### ITEM 378

- Page ITEM: 075 of 246
- File: `apps.html`
- Line/context: L294 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(9) > th`
- Element/type: Table header
- Exact English:

  ```text
  Emergency Ready
  ```
- Protected tokens: `Emergency Ready`

### ITEM 379

- Page ITEM: 076 of 246
- File: `apps.html`
- Line/context: L295 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(9) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Official alerts and nearby safety information
  ```
- Protected tokens: None identified in this item.

### ITEM 380

- Page ITEM: 077 of 246
- File: `apps.html`
- Line/context: L296 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(9) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Recommended
  ```
- Protected tokens: None identified in this item.

### ITEM 381

- Page ITEM: 078 of 246
- File: `apps.html`
- Line/context: L297 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(9) > td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  Location and notification permissions are needed for full value.
  ```
- Protected tokens: None identified in this item.

### ITEM 382

- Page ITEM: 079 of 246
- File: `apps.html`
- Line/context: L300 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(10) > th`
- Element/type: Table header
- Exact English:

  ```text
  VisitKorea
  ```
- Protected tokens: `VisitKorea`

### ITEM 383

- Page ITEM: 080 of 246
- File: `apps.html`
- Line/context: L301 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(10) > td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Official travel information and planning
  ```
- Protected tokens: None identified in this item.

### ITEM 384

- Page ITEM: 081 of 246
- File: `apps.html`
- Line/context: L302 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(10) > td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Optional
  ```
- Protected tokens: None identified in this item.

### ITEM 385

- Page ITEM: 082 of 246
- File: `apps.html`
- Line/context: L303 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(10) > td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  It complements rather than replaces a dedicated map app.
  ```
- Protected tokens: None identified in this item.

### ITEM 386

- Page ITEM: 083 of 246
- File: `apps.html`
- Line/context: L314 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > header.apps-section__heading > h2#naver-map-title`
- Element/type: H2 heading
- Exact English:

  ```text
  When An English Search Does Not Find The Place
  ```
- Protected tokens: None identified in this item.

### ITEM 387

- Page ITEM: 084 of 246
- File: `apps.html`
- Line/context: L315 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Exact English:

  ```text
  Naver Map is the practical starting point for local routes and listings. Its map and interface support Korean, English, Japanese and Chinese, but the exact Korean name or address often matters more than another English spelling.
  ```
- Protected tokens: `Naver Map`

### ITEM 388

- Page ITEM: 085 of 246
- File: `apps.html`
- Line/context: L319 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Routes and local details live together
  ```
- Protected tokens: None identified in this item.

### ITEM 389

- Page ITEM: 086 of 246
- File: `apps.html`
- Line/context: L320 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > p`
- Element/type: Body text
- Exact English:

  ```text
  Subway, bus, walking and driving routes sit alongside business hours, entrances and nearby branches. Important places and bookmarks remain easier to retrieve when they are saved before leaving reliable Wi-Fi.
  ```
- Protected tokens: `Wi-Fi`

### ITEM 390

- Page ITEM: 087 of 246
- File: `apps.html`
- Line/context: L323 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  A failed search needs better source text
  ```
- Protected tokens: None identified in this item.

### ITEM 391

- Page ITEM: 088 of 246
- File: `apps.html`
- Line/context: L324 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > p`
- Element/type: Body text
- Exact English:

  ```text
  An empty English result does not mean the place is missing. The exact Korean name, road address or phone number from a booking can recover the listing; the pin and storefront photos then distinguish one branch from another.
  ```
- Protected tokens: None identified in this item.

### ITEM 392

- Page ITEM: 089 of 246
- File: `apps.html`
- Line/context: L327 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(3) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  KakaoMap is a second local reference
  ```
- Protected tokens: `KakaoMap`

### ITEM 393

- Page ITEM: 090 of 246
- File: `apps.html`
- Line/context: L328 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(3) > p`
- Element/type: Body text
- Exact English:

  ```text
  An unclear pin, branch or walking route is where KakaoMap can help. The same Korean name or road address works better than repeatedly changing the English spelling, and the phone number and storefront photos provide a final cross-check.
  ```
- Protected tokens: `KakaoMap`

### ITEM 394

- Page ITEM: 091 of 246
- File: `apps.html`
- Line/context: L331 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > p.apps-context-note:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Search recovery: English place name → Korean place name → Korean road address → phone number. For route modes and more detailed map tactics, use the Korea maps guide .
  ```
- Protected tokens: `Korea`

### ITEM 395

- Page ITEM: 092 of 246
- File: `apps.html`
- Line/context: L332 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > p.apps-download-links:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Naver Map downloads
  ```
- Protected tokens: `Naver Map`

### ITEM 396

- Page ITEM: 093 of 246
- File: `apps.html`
- Line/context: L333 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(1)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Naver Map on the App Store
  ```
- Protected tokens: `Naver Map`, `App Store`

### ITEM 397

- Page ITEM: 094 of 246
- File: `apps.html`
- Line/context: L334 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(2)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Naver Map on Google Play
  ```
- Protected tokens: `Naver Map`, `Google Play`, `Google`

### ITEM 398

- Page ITEM: 095 of 246
- File: `apps.html`
- Line/context: L336 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > p.apps-download-links:nth-of-type(3) @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  KakaoMap official page
  ```
- Protected tokens: `KakaoMap`

### ITEM 399

- Page ITEM: 096 of 246
- File: `apps.html`
- Line/context: L337 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > p.apps-download-links:nth-of-type(3) > a`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  KakaoMap official service page
  ```
- Protected tokens: `KakaoMap`

### ITEM 400

- Page ITEM: 097 of 246
- File: `apps.html`
- Line/context: L345 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > header.apps-section__heading > h2#papago-title`
- Element/type: H2 heading
- Exact English:

  ```text
  Menus, Signs And Short Exchanges Need Different Translation Modes
  ```
- Protected tokens: None identified in this item.

### ITEM 401

- Page ITEM: 098 of 246
- File: `apps.html`
- Line/context: L346 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Exact English:

  ```text
  Papago supports text, image, voice, conversation and offline translation, so the useful part is matching the input method to what is in front of you.
  ```
- Protected tokens: `Papago`

### ITEM 402

- Page ITEM: 099 of 246
- File: `apps.html`
- Line/context: L350 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  The camera handles text you cannot copy
  ```
- Protected tokens: None identified in this item.

### ITEM 403

- Page ITEM: 100 of 246
- File: `apps.html`
- Line/context: L351 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > p`
- Element/type: Body text
- Exact English:

  ```text
  Image translation can read a restaurant menu, medicine label, kiosk, notice or screenshot. Voice or conversation mode is more natural during a short exchange where typing would slow both people down.
  ```
- Protected tokens: None identified in this item.

### ITEM 404

- Page ITEM: 101 of 246
- File: `apps.html`
- Line/context: L354 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Offline support is not identical across features
  ```
- Protected tokens: None identified in this item.

### ITEM 405

- Page ITEM: 102 of 246
- File: `apps.html`
- Line/context: L355 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > p`
- Element/type: Body text
- Exact English:

  ```text
  The required language data needs to be downloaded and tested before departure. Offline availability varies by language and feature, while key addresses and booking details are safer when saved separately.
  ```
- Protected tokens: None identified in this item.

### ITEM 406

- Page ITEM: 103 of 246
- File: `apps.html`
- Line/context: L358 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > p.apps-context-note:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Translation still needs judgment. Names, slang, dietary terms and menu-specific meaning can be wrong even when the sentence looks fluent. Show the original Korean alongside the translation when accuracy matters.
  ```
- Protected tokens: None identified in this item.

### ITEM 407

- Page ITEM: 104 of 246
- File: `apps.html`
- Line/context: L359 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > p.apps-download-links:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Papago downloads
  ```
- Protected tokens: `Papago`

### ITEM 408

- Page ITEM: 105 of 246
- File: `apps.html`
- Line/context: L360 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(1)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Papago on the App Store
  ```
- Protected tokens: `Papago`, `App Store`

### ITEM 409

- Page ITEM: 106 of 246
- File: `apps.html`
- Line/context: L361 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(2)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Papago on Google Play
  ```
- Protected tokens: `Papago`, `Google Play`, `Google`

### ITEM 410

- Page ITEM: 107 of 246
- File: `apps.html`
- Line/context: L362 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(3)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Papago on the web
  ```
- Protected tokens: `Papago`

### ITEM 411

- Page ITEM: 108 of 246
- File: `apps.html`
- Line/context: L370 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > header.apps-section__heading > h2#taxi-apps-title`
- Element/type: H2 heading
- Exact English:

  ```text
  The Taxi App Has To Work Before The Late-Night Ride
  ```
- Protected tokens: None identified in this item.

### ITEM 412

- Page ITEM: 109 of 246
- File: `apps.html`
- Line/context: L371 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Exact English:

  ```text
  k.ride was built for international travelers, while Kakao T and Uber Taxi are more relevant when an existing account already makes one of them familiar.
  ```
- Protected tokens: `k.ride`, `Kakao T`, `Kakao`, `Uber Taxi`

### ITEM 413

- Page ITEM: 110 of 246
- File: `apps.html`
- Line/context: L375 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  k.ride removes several visitor barriers
  ```
- Protected tokens: `k.ride`

### ITEM 414

- Page ITEM: 111 of 246
- File: `apps.html`
- Line/context: L376 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > p`
- Element/type: Body text
- Exact English:

  ```text
  Kakao Mobility built k.ride for international travelers. It supports a multilingual interface, destination search and driver-chat translation across many languages, plus registration of overseas-issued cards. Test sign-in and payment before the trip.
  ```
- Protected tokens: `Kakao Mobility`, `Kakao`, `k.ride`

### ITEM 415

- Page ITEM: 112 of 246
- File: `apps.html`
- Line/context: L379 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Kakao T suits existing Kakao users
  ```
- Protected tokens: `Kakao T`, `Kakao`

### ITEM 416

- Page ITEM: 113 of 246
- File: `apps.html`
- Line/context: L380 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > p`
- Element/type: Body text
- Exact English:

  ```text
  It is a strong local service for someone already comfortable with the Kakao ecosystem. Account, language and payment conditions can vary, so do not make first use depend on a late-night ride.
  ```
- Protected tokens: `Kakao`

### ITEM 417

- Page ITEM: 114 of 246
- File: `apps.html`
- Line/context: L383 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(3) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Uber Taxi is a familiar alternative
  ```
- Protected tokens: `Uber Taxi`

### ITEM 418

- Page ITEM: 115 of 246
- File: `apps.html`
- Line/context: L384 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(3) > p`
- Element/type: Body text
- Exact English:

  ```text
  It can be convenient when you already have an Uber account and prefer its interface. Available vehicle types, dispatch and payment options still vary by location and time.
  ```
- Protected tokens: None identified in this item.

### ITEM 419

- Page ITEM: 116 of 246
- File: `apps.html`
- Line/context: L387 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > p.apps-context-note:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Save your destination in Korean and keep a second payment method even when in-app payment is set. See the complete Korea taxi guide for fares, pickup checks and late-night strategy. If you prefer a prearranged ride after a long flight, compare the options in the airport transfer guide .
  ```
- Protected tokens: `Korea`

### ITEM 420

- Page ITEM: 117 of 246
- File: `apps.html`
- Line/context: L388 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > p.apps-download-links:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Taxi app downloads
  ```
- Protected tokens: None identified in this item.

### ITEM 421

- Page ITEM: 118 of 246
- File: `apps.html`
- Line/context: L389 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(1)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  k.ride on the App Store
  ```
- Protected tokens: `k.ride`, `App Store`

### ITEM 422

- Page ITEM: 119 of 246
- File: `apps.html`
- Line/context: L390 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(2)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  k.ride on Google Play
  ```
- Protected tokens: `k.ride`, `Google Play`, `Google`

### ITEM 423

- Page ITEM: 120 of 246
- File: `apps.html`
- Line/context: L391 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(3)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  k.ride official guide
  ```
- Protected tokens: `k.ride`

### ITEM 424

- Page ITEM: 121 of 246
- File: `apps.html`
- Line/context: L399 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > header.apps-section__heading > h2#kakaotalk-title`
- Element/type: H2 heading
- Exact English:

  ```text
  Local Contacts And Restaurant Bookings Are Trip-Dependent
  ```
- Protected tokens: None identified in this item.

### ITEM 425

- Page ITEM: 122 of 246
- File: `apps.html`
- Line/context: L400 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Exact English:

  ```text
  KakaoTalk and Catchtable become relevant because of a particular person, business or reservation—not simply because they are popular Korean apps.
  ```
- Protected tokens: `KakaoTalk`, `Catchtable`

### ITEM 426

- Page ITEM: 123 of 246
- File: `apps.html`
- Line/context: L404 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  KakaoTalk follows the contact
  ```
- Protected tokens: `KakaoTalk`

### ITEM 427

- Page ITEM: 124 of 246
- File: `apps.html`
- Line/context: L405 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > p`
- Element/type: Body text
- Exact English:

  ```text
  Korean friends, accommodation hosts, tour operators and local businesses may expect to communicate there. Open Chat, Voice Talk and Face Talk can support coordination when those contacts use them; a short independent trip with no local contacts may never need the app.
  ```
- Protected tokens: `Open Chat`, `Voice Talk`, `Face Talk`

### ITEM 428

- Page ITEM: 125 of 246
- File: `apps.html`
- Line/context: L408 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Account access needs to survive the trip
  ```
- Protected tokens: None identified in this item.

### ITEM 429

- Page ITEM: 126 of 246
- File: `apps.html`
- Line/context: L409 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > p`
- Element/type: Body text
- Exact English:

  ```text
  SMS verification and account recovery are worth testing before departure when KakaoTalk will carry an important conversation. Losing access abroad can matter more than any feature inside the app.
  ```
- Protected tokens: `SMS`, `KakaoTalk`

### ITEM 430

- Page ITEM: 127 of 246
- File: `apps.html`
- Line/context: L412 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > div.apps-editorial-list > div#restaurant-reservations.apps-editorial-row:nth-of-type(3) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Catchtable depends on the restaurant
  ```
- Protected tokens: `Catchtable`

### ITEM 431

- Page ITEM: 128 of 246
- File: `apps.html`
- Line/context: L413 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > div.apps-editorial-list > div#restaurant-reservations.apps-editorial-row:nth-of-type(3) > p`
- Element/type: Body text
- Exact English:

  ```text
  The global service supports discovery, reviews, reservations and waitlists for participating venues. Similar English names can point to different districts or branches, so the Korean address, neighborhood and photos need to match before confirming.
  ```
- Protected tokens: None identified in this item.

### ITEM 432

- Page ITEM: 129 of 246
- File: `apps.html`
- Line/context: L416 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(4) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  The booking terms matter after a table appears
  ```
- Protected tokens: None identified in this item.

### ITEM 433

- Page ITEM: 130 of 246
- File: `apps.html`
- Line/context: L417 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(4) > p`
- Element/type: Body text
- Exact English:

  ```text
  Party size, deposit, cancellation terms, arrival time and whether the listing offers a reservation or only a waitlist can change the value of an available slot.
  ```
- Protected tokens: None identified in this item.

### ITEM 434

- Page ITEM: 131 of 246
- File: `apps.html`
- Line/context: L420 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > p.apps-download-links:nth-of-type(1) @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  KakaoTalk official page
  ```
- Protected tokens: `KakaoTalk`

### ITEM 435

- Page ITEM: 132 of 246
- File: `apps.html`
- Line/context: L421 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > p.apps-download-links:nth-of-type(1) > a`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  KakaoTalk official service page
  ```
- Protected tokens: `KakaoTalk`

### ITEM 436

- Page ITEM: 133 of 246
- File: `apps.html`
- Line/context: L423 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > p.apps-download-links:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Catchtable official page
  ```
- Protected tokens: `Catchtable`

### ITEM 437

- Page ITEM: 134 of 246
- File: `apps.html`
- Line/context: L424 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > p.apps-download-links:nth-of-type(2) > a`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Open CATCHTABLE Global
  ```
- Protected tokens: `CATCHTABLE Global`, `CATCHTABLE`

### ITEM 438

- Page ITEM: 135 of 246
- File: `apps.html`
- Line/context: L432 - `html > body > main > article.apps-guide > section#food-delivery.apps-section.apps-section--soft:nth-of-type(6) > div.container > header.apps-section__heading > h2#food-delivery-title`
- Element/type: H2 heading
- Exact English:

  ```text
  Delivery Is Where App Setup Gets Harder
  ```
- Protected tokens: None identified in this item.

### ITEM 439

- Page ITEM: 136 of 246
- File: `apps.html`
- Line/context: L433 - `html > body > main > article.apps-guide > section#food-delivery.apps-section.apps-section--soft:nth-of-type(6) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Exact English:

  ```text
  The difficult part is often not finding food. Phone verification, the delivery-address format, payment acceptance and coverage can stop an order after the menu has already been chosen.
  ```
- Protected tokens: None identified in this item.

### ITEM 440

- Page ITEM: 137 of 246
- File: `apps.html`
- Line/context: L437 - `html > body > main > article.apps-guide > section#food-delivery.apps-section.apps-section--soft:nth-of-type(6) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Shuttle reduces some visitor friction
  ```
- Protected tokens: `Shuttle`

### ITEM 441

- Page ITEM: 138 of 246
- File: `apps.html`
- Line/context: L438 - `html > body > main > article.apps-guide > section#food-delivery.apps-section.apps-section--soft:nth-of-type(6) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > p`
- Element/type: Body text
- Exact English:

  ```text
  Shuttle supports international users, multilingual ordering and overseas payment methods without requiring a Korean phone number for standard sign-up. Its restaurant choice and coverage are narrower than the largest domestic platforms, which makes the actual delivery address the first practical constraint.
  ```
- Protected tokens: `Shuttle`

### ITEM 442

- Page ITEM: 139 of 246
- File: `apps.html`
- Line/context: L441 - `html > body > main > article.apps-guide > section#food-delivery.apps-section.apps-section--soft:nth-of-type(6) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Domestic platforms ask more of the account
  ```
- Protected tokens: None identified in this item.

### ITEM 443

- Page ITEM: 140 of 246
- File: `apps.html`
- Line/context: L442 - `html > body > main > article.apps-guide > section#food-delivery.apps-section.apps-section--soft:nth-of-type(6) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > p`
- Element/type: Body text
- Exact English:

  ```text
  Baemin and Coupang Eats can make more sense during a longer stay or when local address entry, account setup and payment troubleshooting are manageable. Baemin offers multilingual support through the core order flow, but that does not guarantee every menu, card or delivery location. When setup fails, hotel assistance, pickup, a direct restaurant visit or a convenience store keeps the problem from taking over the evening.
  ```
- Protected tokens: `Baemin`, `Coupang Eats`

### ITEM 444

- Page ITEM: 141 of 246
- File: `apps.html`
- Line/context: L445 - `html > body > main > article.apps-guide > section#food-delivery.apps-section.apps-section--soft:nth-of-type(6) > div.container > p.apps-download-links @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Shuttle Delivery official page
  ```
- Protected tokens: `Shuttle Delivery`, `Shuttle`

### ITEM 445

- Page ITEM: 142 of 246
- File: `apps.html`
- Line/context: L446 - `html > body > main > article.apps-guide > section#food-delivery.apps-section.apps-section--soft:nth-of-type(6) > div.container > p.apps-download-links > a`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Open Shuttle Delivery
  ```
- Protected tokens: `Shuttle Delivery`, `Shuttle`

### ITEM 446

- Page ITEM: 143 of 246
- File: `apps.html`
- Line/context: L454 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > header.apps-section__heading > h2#payment-transit-apps-title`
- Element/type: H2 heading
- Exact English:

  ```text
  Payment, Rail And Official Services Depend On The Trip
  ```
- Protected tokens: None identified in this item.

### ITEM 447

- Page ITEM: 144 of 246
- File: `apps.html`
- Line/context: L455 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Exact English:

  ```text
  These services solve different problems. They belong on the phone only when the payment method, intercity route, planning style or safety need gives them a clear purpose.
  ```
- Protected tokens: None identified in this item.

### ITEM 448

- Page ITEM: 145 of 246
- File: `apps.html`
- Line/context: L459 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list:nth-of-type(1) > div.apps-editorial-row:nth-of-type(1) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  WOWPASS manages a card with two balances
  ```
- Protected tokens: `WOWPASS`

### ITEM 449

- Page ITEM: 146 of 246
- File: `apps.html`
- Line/context: L460 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list:nth-of-type(1) > div.apps-editorial-row:nth-of-type(1) > p`
- Element/type: Body text
- Exact English:

  ```text
  The app handles the prepaid payment card, transactions and security features. The payment balance and embedded T-money transit balance remain separate, so the WOWPASS guide is useful before the first top-up.
  ```
- Protected tokens: `T-money`, `WOWPASS`

### ITEM 450

- Page ITEM: 147 of 246
- File: `apps.html`
- Line/context: L463 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list:nth-of-type(1) > div.apps-editorial-row:nth-of-type(2) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Mobile Tmoney depends on the device and funding path
  ```
- Protected tokens: `Mobile Tmoney`, `Tmoney`

### ITEM 451

- Page ITEM: 148 of 246
- File: `apps.html`
- Line/context: L464 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list:nth-of-type(1) > div.apps-editorial-row:nth-of-type(2) > p`
- Element/type: Body text
- Exact English:

  ```text
  Mobile transit payment is available on compatible phones and wallets, including supported Apple devices and Android setups. Device, NFC, wallet and top-up conditions matter; if any part is unclear, a physical card is the lower-friction choice. Use the T-money guide for setup and use.
  ```
- Protected tokens: `Android`, `NFC`, `T-money`

### ITEM 452

- Page ITEM: 149 of 246
- File: `apps.html`
- Line/context: L467 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-context-note:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  A transit card and a broader payment service do different jobs. The Korea payments guide explains where each one fits without requiring both apps by default.
  ```
- Protected tokens: `Korea`

### ITEM 453

- Page ITEM: 150 of 246
- File: `apps.html`
- Line/context: L468 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Payment app official pages
  ```
- Protected tokens: None identified in this item.

### ITEM 454

- Page ITEM: 151 of 246
- File: `apps.html`
- Line/context: L469 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(1)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  WOWPASS official site
  ```
- Protected tokens: `WOWPASS`

### ITEM 455

- Page ITEM: 152 of 246
- File: `apps.html`
- Line/context: L470 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(2)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Tmoney mobile services
  ```
- Protected tokens: `Tmoney`

### ITEM 456

- Page ITEM: 153 of 246
- File: `apps.html`
- Line/context: L474 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list.apps-editorial-list--continued:nth-of-type(2) > div#train-apps.apps-editorial-row > h3#train-apps-title`
- Element/type: H3 heading
- Exact English:

  ```text
  KORAIL bookings can stay in the browser
  ```
- Protected tokens: `KORAIL`

### ITEM 457

- Page ITEM: 154 of 246
- File: `apps.html`
- Line/context: L475 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list.apps-editorial-list--continued:nth-of-type(2) > div#train-apps.apps-editorial-row > p`
- Element/type: Body text
- Exact English:

  ```text
  The English reservation site lets foreign visitors search and book KTX and other KORAIL-operated trains without installing an app. A card enrolled for 3-D Secure, the exact passenger name and a saved confirmation matter more than KorailTalk for an occasional rail journey. KORAIL and SRT remain separate systems.
  ```
- Protected tokens: `KTX`, `KORAIL`, `3-D Secure`, `KorailTalk`, `SRT`

### ITEM 458

- Page ITEM: 155 of 246
- File: `apps.html`
- Line/context: L478 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(3) @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  KORAIL official booking
  ```
- Protected tokens: `KORAIL`

### ITEM 459

- Page ITEM: 156 of 246
- File: `apps.html`
- Line/context: L479 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(3) > a`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Book on the official KORAIL site
  ```
- Protected tokens: `KORAIL`

### ITEM 460

- Page ITEM: 157 of 246
- File: `apps.html`
- Line/context: L483 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list.apps-editorial-list--continued:nth-of-type(3) > div#visitkorea.apps-editorial-row > h3#visitkorea-title`
- Element/type: H3 heading
- Exact English:

  ```text
  VisitKorea is an official reference, not another map
  ```
- Protected tokens: `VisitKorea`

### ITEM 461

- Page ITEM: 158 of 246
- File: `apps.html`
- Line/context: L484 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list.apps-editorial-list--continued:nth-of-type(3) > div#visitkorea.apps-editorial-row > p`
- Element/type: Body text
- Exact English:

  ```text
  The Korea Tourism Organization platform covers attractions, food, accommodation, festivals, travel basics, suggested itineraries and planning support. The app adds saved plans or notifications, while the website is enough when only an official reference is needed.
  ```
- Protected tokens: `Korea Tourism Organization`, `Korea`

### ITEM 462

- Page ITEM: 159 of 246
- File: `apps.html`
- Line/context: L487 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(4) @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  VisitKorea official app information
  ```
- Protected tokens: `VisitKorea`

### ITEM 463

- Page ITEM: 160 of 246
- File: `apps.html`
- Line/context: L488 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(4) > a`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  VisitKorea app information
  ```
- Protected tokens: `VisitKorea`

### ITEM 464

- Page ITEM: 161 of 246
- File: `apps.html`
- Line/context: L492 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list.apps-editorial-list--continued:nth-of-type(4) > div#emergency-ready.apps-editorial-row > h3#emergency-ready-title`
- Element/type: H3 heading
- Exact English:

  ```text
  Emergency Ready keeps official help close
  ```
- Protected tokens: `Emergency Ready`

### ITEM 465

- Page ITEM: 162 of 246
- File: `apps.html`
- Line/context: L493 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list.apps-editorial-list--continued:nth-of-type(4) > div#emergency-ready.apps-editorial-row > p`
- Element/type: Body text
- Exact English:

  ```text
  The Ministry of the Interior and Safety app provides multilingual disaster alerts, nearby shelters and emergency facilities, embassy information and official response guidance. Location and notification access let those features work when they are needed.
  ```
- Protected tokens: `Ministry of the Interior and Safety`

### ITEM 466

- Page ITEM: 163 of 246
- File: `apps.html`
- Line/context: L496 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-context-note:nth-of-type(5)`
- Element/type: Body text
- Exact English:

  ```text
  Know the difference: 1330 is a travel-information and interpretation helpline, not an emergency dispatch number. Call 112 for police or 119 for fire and medical emergencies.
  ```
- Protected tokens: `112`, `119`

### ITEM 467

- Page ITEM: 164 of 246
- File: `apps.html`
- Line/context: L497 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(6) @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Emergency Ready official information
  ```
- Protected tokens: `Emergency Ready`

### ITEM 468

- Page ITEM: 165 of 246
- File: `apps.html`
- Line/context: L498 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(6) > a:nth-of-type(1)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Emergency Ready on Google Play
  ```
- Protected tokens: `Emergency Ready`, `Google Play`, `Google`

### ITEM 469

- Page ITEM: 166 of 246
- File: `apps.html`
- Line/context: L499 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(6) > a:nth-of-type(2)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Emergency Ready official overview
  ```
- Protected tokens: `Emergency Ready`

### ITEM 470

- Page ITEM: 167 of 246
- File: `apps.html`
- Line/context: L507 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > header.apps-section__heading > h2#setup-before-you-fly-title`
- Element/type: H2 heading
- Exact English:

  ```text
  Finish The Fragile Setup While Home Accounts Still Work
  ```
- Protected tokens: None identified in this item.

### ITEM 471

- Page ITEM: 168 of 246
- File: `apps.html`
- Line/context: L508 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Exact English:

  ```text
  A familiar network keeps the usual phone number, cards, email and password manager within reach while login, permissions and payment are tested.
  ```
- Protected tokens: None identified in this item.

### ITEM 472

- Page ITEM: 169 of 246
- File: `apps.html`
- Line/context: L511 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(1) > strong :: text()[1]`
- Element/type: Visible list label
- Exact English:

  ```text
  Finish account access
  ```
- Protected tokens: None identified in this item.

### ITEM 473

- Page ITEM: 170 of 246
- File: `apps.html`
- Line/context: L511 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(1) > span :: text()[1]`
- Element/type: Visible list description
- Exact English:

  ```text
  Install only the apps with a job on the itinerary, set the language, then sign out and back in once so a forgotten password or blocked social login appears before the trip.
  ```
- Protected tokens: None identified in this item.

### ITEM 474

- Page ITEM: 171 of 246
- File: `apps.html`
- Line/context: L512 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(2) > strong :: text()[1]`
- Element/type: Visible list label
- Exact English:

  ```text
  Test the permissions you need
  ```
- Protected tokens: None identified in this item.

### ITEM 475

- Page ITEM: 172 of 246
- File: `apps.html`
- Line/context: L512 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(2) > span :: text()[1]`
- Element/type: Visible list description
- Exact English:

  ```text
  Give map, taxi and safety apps the minimum location access required, and test Papago camera translation on a screenshot.
  ```
- Protected tokens: `Papago`

### ITEM 476

- Page ITEM: 173 of 246
- File: `apps.html`
- Line/context: L513 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(3) > strong :: text()[1]`
- Element/type: Visible list label
- Exact English:

  ```text
  Test taxi payment
  ```
- Protected tokens: None identified in this item.

### ITEM 477

- Page ITEM: 174 of 246
- File: `apps.html`
- Line/context: L513 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(3) > span :: text()[1]`
- Element/type: Visible list description
- Exact English:

  ```text
  Add the intended payment method to k.ride while another card or cash remains available if the first charge fails.
  ```
- Protected tokens: `k.ride`

### ITEM 478

- Page ITEM: 175 of 246
- File: `apps.html`
- Line/context: L514 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(4) > strong :: text()[1]`
- Element/type: Visible list label
- Exact English:

  ```text
  Keep the hotel identifiable
  ```
- Protected tokens: None identified in this item.

### ITEM 479

- Page ITEM: 176 of 246
- File: `apps.html`
- Line/context: L514 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(4) > span :: text()[1]`
- Element/type: Visible list description
- Exact English:

  ```text
  Save its exact Korean name, road address and phone number together in notes so a map result, driver or hotel desk can confirm the same place.
  ```
- Protected tokens: None identified in this item.

### ITEM 480

- Page ITEM: 177 of 246
- File: `apps.html`
- Line/context: L515 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(5) > strong :: text()[1]`
- Element/type: Visible list label
- Exact English:

  ```text
  Screenshot important bookings
  ```
- Protected tokens: None identified in this item.

### ITEM 481

- Page ITEM: 178 of 246
- File: `apps.html`
- Line/context: L515 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(5) > span :: text()[1]`
- Element/type: Visible list description
- Exact English:

  ```text
  Save train, restaurant, airport and accommodation details for use without a live account session.
  ```
- Protected tokens: None identified in this item.

### ITEM 482

- Page ITEM: 179 of 246
- File: `apps.html`
- Line/context: L516 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(6) > strong :: text()[1]`
- Element/type: Visible list label
- Exact English:

  ```text
  Confirm data and SMS access
  ```
- Protected tokens: `SMS`

### ITEM 483

- Page ITEM: 180 of 246
- File: `apps.html`
- Line/context: L516 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(6) > span > a`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Korea eSIM
  ```
- Protected tokens: `Korea`, `eSIM`

### ITEM 484

- Page ITEM: 181 of 246
- File: `apps.html`
- Line/context: L518 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > p.apps-context-note`
- Element/type: Body text
- Exact English:

  ```text
  Keep these steps with your wider Korea travel checklist so app setup is finished before airport day.
  ```
- Protected tokens: `Korea`

### ITEM 485

- Page ITEM: 182 of 246
- File: `apps.html`
- Line/context: L525 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > header.apps-section__heading > h2#common-app-problems-title`
- Element/type: H2 heading
- Exact English:

  ```text
  Where Korean Apps Can Get Difficult
  ```
- Protected tokens: None identified in this item.

### ITEM 486

- Page ITEM: 183 of 246
- File: `apps.html`
- Line/context: L526 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Exact English:

  ```text
  The same obstacles can affect maps, taxis, delivery and messaging: account access, payment rules, Korean address fields and the loss of a live connection.
  ```
- Protected tokens: None identified in this item.

### ITEM 487

- Page ITEM: 184 of 246
- File: `apps.html`
- Line/context: L530 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div#real-travel-situations.apps-editorial-row:nth-of-type(1) > h3#real-travel-situations-title`
- Element/type: H3 heading
- Exact English:

  ```text
  English names can hide the correct branch
  ```
- Protected tokens: None identified in this item.

### ITEM 488

- Page ITEM: 185 of 246
- File: `apps.html`
- Line/context: L531 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div#real-travel-situations.apps-editorial-row:nth-of-type(1) > p`
- Element/type: Body text
- Exact English:

  ```text
  A Korean place name, road address or phone number usually gives a local search more to work with. District names, storefront photos and the booking image then help separate one branch from another.
  ```
- Protected tokens: None identified in this item.

### ITEM 489

- Page ITEM: 186 of 246
- File: `apps.html`
- Line/context: L534 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  SMS failure can become an account-recovery problem
  ```
- Protected tokens: `SMS`

### ITEM 490

- Page ITEM: 187 of 246
- File: `apps.html`
- Line/context: L535 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > p`
- Element/type: Body text
- Exact English:

  ```text
  The country code, roaming status and access to the original number all affect verification. Email, social, passport or trusted-device recovery only helps when the service officially offers that route.
  ```
- Protected tokens: None identified in this item.

### ITEM 491

- Page ITEM: 188 of 246
- File: `apps.html`
- Line/context: L538 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(3) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  A foreign card can fail after setup succeeds
  ```
- Protected tokens: None identified in this item.

### ITEM 492

- Page ITEM: 189 of 246
- File: `apps.html`
- Line/context: L539 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(3) > p`
- Element/type: Body text
- Exact English:

  ```text
  Repeated charges do not change a merchant's payment rules. Another prepared card, cash or an in-person payment route keeps one rejected transaction from blocking the next step.
  ```
- Protected tokens: None identified in this item.

### ITEM 493

- Page ITEM: 190 of 246
- File: `apps.html`
- Line/context: L542 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(4) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Korean addresses have to match the service format
  ```
- Protected tokens: None identified in this item.

### ITEM 494

- Page ITEM: 191 of 246
- File: `apps.html`
- Line/context: L543 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(4) > p`
- Element/type: Body text
- Exact English:

  ```text
  English additions can prevent an address field from accepting an otherwise correct location. The full Korean road address and the accommodation's delivery instructions are more reliable, while unavailable coverage calls for pickup, hotel help or a direct visit.
  ```
- Protected tokens: None identified in this item.

### ITEM 495

- Page ITEM: 192 of 246
- File: `apps.html`
- Line/context: L546 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(5) > h3`
- Element/type: H3 heading
- Exact English:

  ```text
  Permissions and language settings can quietly reset
  ```
- Protected tokens: None identified in this item.

### ITEM 496

- Page ITEM: 193 of 246
- File: `apps.html`
- Line/context: L547 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(5) > p`
- Element/type: Body text
- Exact English:

  ```text
  App and phone language settings may both matter. Location-dependent features also need the minimum permission required, followed by a fresh look at the pickup marker before confirmation.
  ```
- Protected tokens: None identified in this item.

### ITEM 497

- Page ITEM: 194 of 246
- File: `apps.html`
- Line/context: L550 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div#common-mistakes.apps-editorial-row:nth-of-type(6) > h3#common-mistakes-title`
- Element/type: H3 heading
- Exact English:

  ```text
  Screenshots carry the trip when live access disappears
  ```
- Protected tokens: None identified in this item.

### ITEM 498

- Page ITEM: 195 of 246
- File: `apps.html`
- Line/context: L551 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div#common-mistakes.apps-editorial-row:nth-of-type(6) > p`
- Element/type: Body text
- Exact English:

  ```text
  Saved bookings, Korean addresses and return-route details remain usable through a lost session or data interruption. Trusted accommodation or station Wi-Fi can help restore a roaming, SIM or eSIM connection without making live access the only plan.
  ```
- Protected tokens: `Wi-Fi`, `SIM`, `eSIM`

### ITEM 499

- Page ITEM: 196 of 246
- File: `apps.html`
- Line/context: L560 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > header.apps-section__heading > h2#faq-title`
- Element/type: H2 heading
- Exact English:

  ```text
  Frequently Asked Questions
  ```
- Protected tokens: None identified in this item.

### ITEM 500

- Page ITEM: 197 of 246
- File: `apps.html`
- Line/context: L564 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(1) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What apps should I install before traveling to Korea?
  ```
- Protected tokens: `Korea`

### ITEM 501

- Page ITEM: 198 of 246
- File: `apps.html`
- Line/context: L565 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(1) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Most first-time visitors should install Naver Map, Papago and k.ride before traveling to Korea. Add Catchtable, Shuttle Delivery, KakaoTalk, payment or rail apps only when your plans need them.
  ```
- Protected tokens: `Naver Map`, `Papago`, `k.ride`, `Korea`, `Catchtable`, `Shuttle Delivery`, `Shuttle`, `KakaoTalk`

### ITEM 502

- Page ITEM: 199 of 246
- File: `apps.html`
- Line/context: L568 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(2) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What is the best map app for Korea?
  ```
- Protected tokens: `Korea`

### ITEM 503

- Page ITEM: 200 of 246
- File: `apps.html`
- Line/context: L569 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(2) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Naver Map is the best default map app for most visitors because it provides detailed local search and routes for subway, bus, walking and driving. Keep KakaoMap as an optional cross-check.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`

### ITEM 504

- Page ITEM: 201 of 246
- File: `apps.html`
- Line/context: L572 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(3) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Does Google Maps work in Korea?
  ```
- Protected tokens: `Google Maps`, `Google`, `Korea`

### ITEM 505

- Page ITEM: 202 of 246
- File: `apps.html`
- Line/context: L573 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(3) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Google Maps works for saved places and basic orientation, but local search and route details can be less dependable in Korea. Use Naver Map as your main navigation app and check a Korean place name or address when search fails.
  ```
- Protected tokens: `Google Maps`, `Google`, `Korea`, `Naver Map`

### ITEM 506

- Page ITEM: 203 of 246
- File: `apps.html`
- Line/context: L576 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(4) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What is the best translation app for Korea?
  ```
- Protected tokens: `Korea`

### ITEM 507

- Page ITEM: 204 of 246
- File: `apps.html`
- Line/context: L577 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(4) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Papago is the best first translation app for many Korea trips. It supports text, image, voice, conversation and offline translation, although names, slang and menu context still need judgment.
  ```
- Protected tokens: `Papago`, `Korea`

### ITEM 508

- Page ITEM: 205 of 246
- File: `apps.html`
- Line/context: L580 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(5) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What taxi app is easiest for foreign visitors?
  ```
- Protected tokens: None identified in this item.

### ITEM 509

- Page ITEM: 206 of 246
- File: `apps.html`
- Line/context: L581 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(5) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  k.ride is the easiest default for many foreign visitors because it is designed for international travelers and supports multilingual destination search, translated driver chat and overseas-issued card registration. Kakao T and Uber Taxi remain useful alternatives.
  ```
- Protected tokens: `k.ride`, `Kakao T`, `Kakao`, `Uber Taxi`

### ITEM 510

- Page ITEM: 207 of 246
- File: `apps.html`
- Line/context: L584 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(6) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Do I need KakaoTalk as a tourist?
  ```
- Protected tokens: `KakaoTalk`

### ITEM 511

- Page ITEM: 208 of 246
- File: `apps.html`
- Line/context: L585 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(6) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  You need KakaoTalk only if you expect to message Korean friends, hosts, tour operators or local businesses. Travelers with no local contacts can usually skip it.
  ```
- Protected tokens: `KakaoTalk`

### ITEM 512

- Page ITEM: 209 of 246
- File: `apps.html`
- Line/context: L588 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(7) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What app should I use to book restaurants?
  ```
- Protected tokens: None identified in this item.

### ITEM 513

- Page ITEM: 210 of 246
- File: `apps.html`
- Line/context: L589 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(7) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Use Catchtable when a restaurant accepts reservations or waitlist registration through the service. Confirm the branch, date, party size, deposit and cancellation terms before booking.
  ```
- Protected tokens: `Catchtable`

### ITEM 514

- Page ITEM: 211 of 246
- File: `apps.html`
- Line/context: L592 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(8) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Can tourists order food delivery in Korea?
  ```
- Protected tokens: `Korea`

### ITEM 515

- Page ITEM: 212 of 246
- File: `apps.html`
- Line/context: L593 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(8) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Tourists can order food delivery in some areas. Shuttle Delivery is the simplest first option for many foreign visitors because it supports international users and cards, but coverage varies; hotel help, pickup and direct restaurant visits are reliable backups.
  ```
- Protected tokens: `Shuttle Delivery`, `Shuttle`

### ITEM 516

- Page ITEM: 213 of 246
- File: `apps.html`
- Line/context: L596 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(9) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Do Korean apps require a Korean phone number?
  ```
- Protected tokens: None identified in this item.

### ITEM 517

- Page ITEM: 214 of 246
- File: `apps.html`
- Line/context: L597 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(9) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Some Korean apps work with a foreign number, while others require Korean SMS, identity or payment verification for specific features. Test sign-in and recovery before departure and never assume a data-only eSIM provides a Korean phone number.
  ```
- Protected tokens: `SMS`, `eSIM`

### ITEM 518

- Page ITEM: 215 of 246
- File: `apps.html`
- Line/context: L600 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(10) > summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Which apps should I set up before my flight?
  ```
- Protected tokens: None identified in this item.

### ITEM 519

- Page ITEM: 216 of 246
- File: `apps.html`
- Line/context: L601 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(10) > p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Before your flight, set up Naver Map, Papago and k.ride, then test login, language, permissions and payment. Save your hotel's Korean name, road address and phone number, plus screenshots of important bookings.
  ```
- Protected tokens: `Naver Map`, `Papago`, `k.ride`

### ITEM 520

- Page ITEM: 217 of 246
- File: `apps.html`
- Line/context: L610 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > header.apps-section__heading > h2#official-sources-title`
- Element/type: H2 heading
- Exact English:

  ```text
  Official Sources
  ```
- Protected tokens: None identified in this item.

### ITEM 521

- Page ITEM: 218 of 246
- File: `apps.html`
- Line/context: L611 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Exact English:

  ```text
  App features, verification and payment conditions can change. Confirm a critical feature in the official service before making it part of your trip plan.
  ```
- Protected tokens: None identified in this item.

### ITEM 522

- Page ITEM: 219 of 246
- File: `apps.html`
- Line/context: L615 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(1) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  VISITKOREA — Helpful Apps & Resources
  ```
- Protected tokens: `VISITKOREA`

### ITEM 523

- Page ITEM: 220 of 246
- File: `apps.html`
- Line/context: L616 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(1) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official Korea Tourism Organization overview of visitor-facing map, transport, delivery and safety tools.
  ```
- Protected tokens: `Korea Tourism Organization`, `Korea`

### ITEM 524

- Page ITEM: 221 of 246
- File: `apps.html`
- Line/context: L619 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(2) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  NAVER Map official help — supported languages
  ```
- Protected tokens: `NAVER Map`, `NAVER`

### ITEM 525

- Page ITEM: 222 of 246
- File: `apps.html`
- Line/context: L620 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(2) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official interface and map-language information.
  ```
- Protected tokens: None identified in this item.

### ITEM 526

- Page ITEM: 223 of 246
- File: `apps.html`
- Line/context: L623 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(3) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  NAVER Map official app page
  ```
- Protected tokens: `NAVER Map`, `NAVER`

### ITEM 527

- Page ITEM: 224 of 246
- File: `apps.html`
- Line/context: L624 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(3) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Current route, search, transit, navigation and save features from NAVER Corp.
  ```
- Protected tokens: `NAVER Corp.`, `NAVER Corp`, `NAVER`

### ITEM 528

- Page ITEM: 225 of 246
- File: `apps.html`
- Line/context: L627 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(4) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  Papago official app page
  ```
- Protected tokens: `Papago`

### ITEM 529

- Page ITEM: 226 of 246
- File: `apps.html`
- Line/context: L628 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(4) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Current text, image, voice, conversation and offline translation features from NAVER Corp.
  ```
- Protected tokens: `NAVER Corp.`, `NAVER Corp`, `NAVER`

### ITEM 530

- Page ITEM: 227 of 246
- File: `apps.html`
- Line/context: L631 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(5) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  Kakao Mobility — k.ride
  ```
- Protected tokens: `Kakao Mobility`, `Kakao`, `k.ride`

### ITEM 531

- Page ITEM: 228 of 246
- File: `apps.html`
- Line/context: L632 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(5) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official visitor-focused taxi features, supported languages and app downloads.
  ```
- Protected tokens: None identified in this item.

### ITEM 532

- Page ITEM: 229 of 246
- File: `apps.html`
- Line/context: L635 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(6) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  CATCHTABLE Global
  ```
- Protected tokens: `CATCHTABLE Global`, `CATCHTABLE`

### ITEM 533

- Page ITEM: 230 of 246
- File: `apps.html`
- Line/context: L636 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(6) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official restaurant discovery, reservation and waitlist service.
  ```
- Protected tokens: None identified in this item.

### ITEM 534

- Page ITEM: 231 of 246
- File: `apps.html`
- Line/context: L639 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(7) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  Shuttle Delivery
  ```
- Protected tokens: `Shuttle Delivery`, `Shuttle`

### ITEM 535

- Page ITEM: 232 of 246
- File: `apps.html`
- Line/context: L640 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(7) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official ordering service and current delivery availability for international users.
  ```
- Protected tokens: None identified in this item.

### ITEM 536

- Page ITEM: 233 of 246
- File: `apps.html`
- Line/context: L643 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(8) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  Baemin — multilingual ordering experience
  ```
- Protected tokens: `Baemin`

### ITEM 537

- Page ITEM: 234 of 246
- File: `apps.html`
- Line/context: L644 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(8) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official Woowa Brothers engineering overview of the multilingual core order flow.
  ```
- Protected tokens: `Woowa Brothers`

### ITEM 538

- Page ITEM: 235 of 246
- File: `apps.html`
- Line/context: L647 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(9) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  WOWPASS official guide
  ```
- Protected tokens: `WOWPASS`

### ITEM 539

- Page ITEM: 236 of 246
- File: `apps.html`
- Line/context: L648 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(9) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official payment, top-up and separate T-money balance instructions.
  ```
- Protected tokens: `T-money`

### ITEM 540

- Page ITEM: 237 of 246
- File: `apps.html`
- Line/context: L651 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(10) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  Tmoney — mobile transit card support
  ```
- Protected tokens: `Tmoney`

### ITEM 541

- Page ITEM: 238 of 246
- File: `apps.html`
- Line/context: L652 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(10) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official mobile Tmoney availability and supported-device guidance.
  ```
- Protected tokens: `Tmoney`

### ITEM 542

- Page ITEM: 239 of 246
- File: `apps.html`
- Line/context: L655 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(11) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  KORAIL official English reservation
  ```
- Protected tokens: `KORAIL`

### ITEM 543

- Page ITEM: 240 of 246
- File: `apps.html`
- Line/context: L656 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(11) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official booking channel for KTX and other KORAIL-operated trains.
  ```
- Protected tokens: `KTX`, `KORAIL`

### ITEM 544

- Page ITEM: 241 of 246
- File: `apps.html`
- Line/context: L659 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(12) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  VISITKOREA mobile app
  ```
- Protected tokens: `VISITKOREA`

### ITEM 545

- Page ITEM: 242 of 246
- File: `apps.html`
- Line/context: L660 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(12) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official travel information, planner and itinerary features from the Korea Tourism Organization.
  ```
- Protected tokens: `Korea Tourism Organization`, `Korea`

### ITEM 546

- Page ITEM: 243 of 246
- File: `apps.html`
- Line/context: L663 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(13) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  Ministry of the Interior and Safety — Emergency Ready
  ```
- Protected tokens: `Ministry of the Interior and Safety`, `Emergency Ready`

### ITEM 547

- Page ITEM: 244 of 246
- File: `apps.html`
- Line/context: L664 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(13) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official overview of alerts, shelters, emergency facilities, embassy information and safety guidance.
  ```
- Protected tokens: None identified in this item.

### ITEM 548

- Page ITEM: 245 of 246
- File: `apps.html`
- Line/context: L667 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(14) > a`
- Element/type: Official-source visible link text
- Exact English:

  ```text
  VISITKOREA — 1330 Korea Travel Helpline
  ```
- Protected tokens: `VISITKOREA`, `1330 Korea Travel Helpline`, `Korea`

### ITEM 549

- Page ITEM: 246 of 246
- File: `apps.html`
- Line/context: L668 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(14) > span :: text()[1]`
- Element/type: Official-source description
- Exact English:

  ```text
  Official travel-information and interpretation support channel, distinct from police and fire or medical dispatch.
  ```
- Protected tokens: None identified in this item.

## Independent omission audit

After the primary extraction, both English files were independently traversed again from the beginning. Every non-empty page-specific visible text target inside `<main>` was reconciled against metadata, JSON-LD user-facing values, headings, FAQ, tables, link/CTA text, alternative text, ARIA labels, title attributes, and CSS/JavaScript-exposed `data-*` values. Composite UI rows were split where separate child elements carry separate visible labels, descriptions, or links; inline links inside a continuous sentence remain represented by that sentence target.

| English file | ITEMs | title/meta/H1-H4 missing | body/CTA/button/link/FAQ missing | table text missing | alt/ARIA missing | JSON-LD missing | user-facing `data-*` missing | final unlisted English |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `maps.html` | 303 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `apps.html` | 246 | 0 | 0 | 0 | 0 | 0 | 0 | 1 supplemental correction |
| **Total** | **549** | **0** | **0** | **0** | **0** | **0** | **0** | **1 supplemental correction** |

### Audit evidence totals

- Continuous Batch ITEM numbering: `ITEM 001` through `ITEM 549`; gaps `0`; duplicate source targets `0`.
- Page ITEM numbering: `maps.html` `001-303`; `apps.html` `001-246`; gaps `0`; duplicates `0`.
- Source-integrity SHA-256 mismatches: `0`.
- H1 count: `maps.html=1`, `apps.html=1`; pages outside the required count: `0`.
- H1-H4 user-facing headings audited: `76`; missing `0`.
- Body/CTA/button/link/FAQ targets: visible FAQ `40`, standalone visible links `53`; missing `0`.
- Table header/cell targets audited: `71`; missing `0`.
- JSON-LD user-facing values audited: `50`; missing `0`.
- Page-specific non-empty alt values audited: `3`; missing `0`.
- Page-specific literal ARIA labels audited: `16`; missing `0`. Structural ARIA ID references remain protected.
- User-facing title attributes audited: `0`; missing `0`.
- CSS exposure check: shared `style.css` contains `content: attr(data-label)` rules, but neither target file contains a page-specific user-facing `data-label` or other CSS/JavaScript-exposed `data-*` value. Targets `0`; missing `0`.
- Page-specific CSS/JS-generated user-facing English literals in the two target files: `0`; missing `0`.
- Protected-token record errors: `0`; every recorded token was rechecked as an exact substring of its ITEM.
- Final page-specific user-visible English strings omitted from the numbered ITEM extraction: `1`; recorded below as a supplemental correction without changing the Batch ITEM count.

## Post-extraction supplemental source correction

This correction does not change the continuous Batch numbering or the total of 549 ITEMs.

- File: `apps.html`
- Line/context: L516 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(6) > span :: mixed text around a`
- Element/type: Visible list description
- Exact English:

  ```text
  A roaming plan, SIM or Korea eSIM may be data-only while an account still sends recovery codes to the home number.
  ```
- Protected tokens: `SIM`, `Korea`, `eSIM`
