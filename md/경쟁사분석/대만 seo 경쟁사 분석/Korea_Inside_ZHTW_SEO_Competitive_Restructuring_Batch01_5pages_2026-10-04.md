# Korea Inside — Taiwan Traditional Chinese SEO Competitive Restructuring
## Batch 01 — 5 Pages

**Date:** 2026-10-04  
**Status:** RESEARCH COMPLETE / EDITORIAL DECISION INPUT  
**Market:** Taiwan / Traditional Chinese (`zh-TW`)  
**Scope:** 5 pages, inventory order 1–5  
**Implementation:** Research only — HTML 0 / Git 0 / Production 0

---

# 0. Executive Summary

## Batch 01 conclusion

| No. | Page | Primary Taiwan intent | Final status | Main reason |
|---:|---|---|---|---|
| 1 | `index.html` | 韓國自由行 / 第一次去韓國 | **P0** | Search-facing promise is broad, but current home-page body behaves more like a starter hub than the comprehensive planning guide that dominates Taiwan SERP. Page role must be clarified before implementation. |
| 2 | `taste-korea.html` | 韓國美食 | **P0** | Current content is a differentiated food-travel decision hub, but Taiwan SERP for the head topic is dominated by “必吃 / 美食推薦 / 料理 / 餐廳” intent. Search surface and entry architecture are materially misaligned. |
| 3 | `k-beauty.html` | 韓國美妝 | **P1** | Body already matches the 2026 shift from product shopping to beauty experiences. Main gap is Taiwan search wording, first-screen framing, and contextual area links. |
| 4 | `hongdae-travel-guide.html` | 弘大攻略 | **P1** | Current body is stronger than list competitors on route choice, exits, time, weather, luggage/stay friction and traveler fit. Title/H1 do not fully use Taiwan SERP wording (`弘大攻略 / 弘大逛街 / 弘大一日遊`). |
| 5 | `myeongdong-travel-guide.html` | 明洞攻略 / 明洞逛街 | **P1** | Current body is highly differentiated and practical. Search surface should better capture `明洞逛街攻略`; a September-only current layer is stale on 2026-10-04 and needs controlled freshness maintenance. |

### Batch-level editorial finding

The main Taiwan issue is **not “the body is too short”**. In 3 of 5 pages, Korea Inside already has deeper decision support than direct competitors. The recurring weakness is that Taiwan competitors state the search problem very aggressively in Title/H1 using words such as:

- `自由行`
- `攻略`
- `逛街`
- `一日遊`
- `美食推薦`
- `必吃`
- `韓國美妝`
- `Olive Young`

Korea Inside often uses a more editorial label (`旅遊指南`, `品味韓國`, `體驗指南`) while the body itself is stronger. The first restructuring rule for Taiwan should therefore be:

> **Fix the search surface first when the body already solves the trip better. Do not destroy Korea Inside’s friction / decision architecture just to imitate giant Taiwan listicles.**

---

# 1. Research Basis and Limits

## Active source state confirmed

- Public Content Master: ACTIVE v1.4
- Navigation & Hub Architecture: ACTIVE v1.1
- Language Localization Standard: ACTIVE v2.1
- Taiwan Traditional Chinese Localization Standard: ACTIVE v1.1
- Taiwan SEO Room Handover: 2026-10-04
- Taiwan Master Inventory: **58 COMPLETE / 0 MISSING / 3 EXCLUDE**
- Repository `/zh-tw/`: **58 HTML pages**

## Batch 01 exact scope

1. `zh-tw/index.html`
2. `zh-tw/taste-korea.html`
3. `zh-tw/k-beauty.html`
4. `zh-tw/hongdae-travel-guide.html`
5. `zh-tw/myeongdong-travel-guide.html`

## SERP method

Research used live Traditional-Chinese queries and Taiwan-oriented editorial results on 2026-10-04. SERP positions vary by time, device, location and personalization, so this report does **not** claim any source is absolute Google #1/#2. The labels below mean editorial benchmark quality for the same search problem.

### Quantitative keyword limitation

A Taiwan Semrush batch keyword request was attempted, but the connected Semrush API returned `API UNITS BALANCE IS ZERO`. Therefore:

- search volume: **not claimed**
- keyword difficulty: **not claimed**
- traffic estimates: **not claimed**
- priority is based on live SERP wording, intent pattern, competitor architecture, and current Korea Inside structure.

---

# PAGE 1 — `index.html`

## A. Page Role

**Recommended role:** First-time Korea trip starter / decision hub.

The homepage should help a Taiwan traveler answer: **「第一次去韓國，我現在先決定什麼？」** and then route each next question to the correct Korea Inside detail page.

It should not become a 10,000-word Seoul itinerary page merely because `首爾自由行` is competitive.

## B. Search Intent

Taiwan SERP around first-trip / Korea-Seoul free travel is dominated by users trying to settle:

- first-trip itinerary shape
- entry / pre-trip preparation
- airport-to-city transport
- where to stay
- transport cards / payment
- mobile data
- major areas / attractions
- rough budget / trip length

The user wants a **planning sequence**, not just a site directory.

## C. Primary / Supporting Traditional-Chinese Keywords

**Primary candidate:** `韓國自由行`  
**Supporting:** `第一次去韓國`, `韓國自由行新手`, `韓國自由行攻略`, `首爾自由行`, `韓國旅遊準備`

Editorial note: if Korea Inside decides the homepage should remain strictly a first-trip starter rather than a full Korea free-travel guide, `第一次去韓國` should be emphasized strongly in H1/lead to narrow intent without abandoning the `韓國自由行` head topic.

## D. Taiwan SERP Observation

Dominant format:

- comprehensive guide / “懶人包”
- 5D4N / 5–8 day itinerary
- entry + transport + stay + food + shopping + attractions
- first-timer checklist
- strong 2026 freshness labeling

Observed Taiwan wording repeatedly includes `首爾自由行`, `韓國首爾自由行懶人包`, `第一次去首爾`, `行程`, `住宿區域`, `交通卡`, `換錢`, `必去景點`.

## E. Direct Editorial Benchmark #1

**Source:** 小不點看世界 / Paine  
**URL:** https://www.paine0602.com/seoul-travel/  
**Observed H1:** `2026韓國首爾自由行懶人包｜第一次去首爾五天四夜行程、住宿區域、交通卡、換錢與必去景點`

### Structure

- first-time quick direction
- pre-trip preparation
- entry rules
- money / payment
- T-money / WOWPASS / climate card
- Incheon Airport transport
- stay-area selection
- key Seoul areas
- 5D4N itinerary variants
- food
- FAQ / mistakes

### Strength

Very explicit first-timer query match. The table of contents tells the user immediately that the page can solve the entire trip-planning problem.

### Weakness vs Korea Inside

It is a broad “everything in one article” model. Korea Inside has a cleaner opportunity to route each decision to dedicated high-depth pages and avoid duplicating transport/payment/stay detail on the homepage.

## F. Direct Editorial Benchmark #2

**Source:** Klook Taiwan  
**URL:** https://www.klook.com/zh-TW/blog/seoul-korea-self-guided-tour/  
**Observed H1:** `2026首爾自由行》3種五天四夜行程推薦，30+種美食景點攻略`

### Structure

- travel preparation
- airport
- city transport
- multiple 5D4N itinerary models
- stay areas
- attractions / food / shopping
- booking-oriented action links

### Strength

Very strong query language and immediate itinerary utility. It combines editorial guide content with booking actions.

### Weakness vs Korea Inside

Commercial and list-heavy. It does not naturally provide Korea Inside’s cross-page decision layer for luggage, exact hotel area, payment failure, card choice, maps, arrival friction, and “what should I decide next?”

## G. Korea Inside Current Structure

**Current source SHA:** `b57194a0472ba9b11fcb9cb3e789885ffa28a89b`

- **Title:** `第一次去韓國自由行指南 | Korea Inside`
- **Meta:** `第一次到韓國旅行的實用指南，涵蓋住宿、機場抵達、eSIM、交通、付款、地圖，以及旅途中各種選擇。`
- **H1:** `第一次去韓國，從這裡開始`

### H2

1. `從這趟旅行最期待的事開始。`
2. `先決定幾件事，抵達當天就輕鬆許多`
3. `抵達韓國後的第一個小時`

### H3

- `韓國美食`
- `韓系美妝 K-Beauty`
- `住在哪裡，影響的不只是飯店本身`
- `在需要用手機之前，先把上網安排好`
- `把機場交通規劃到你真正要住的飯店`

### Existing strengths

- very strong same-language internal-link network
- airport / eSIM / payment / map / stay / area-guide sibling coverage exists
- first-hour framing is distinctive
- page does not drown the user in a giant attraction list

## H. 1:1 SEO GAP Table

| Element | Competitor pattern | Korea Inside | Gap / judgment | Priority |
|---|---|---|---|---|
| Title | `首爾自由行 / 韓國首爾自由行 / 新手 / 行程 / 懶人包` | `第一次去韓國自由行指南` | Good core term, but weaker 2026 / planning problem signal | P1 |
| Meta | names itinerary, entry, stay, transport, budget | names stay, airport, eSIM, transport, payment, maps | Good practical scope, but does not promise trip-planning sequence | P1 |
| H1 | query-heavy, often `首爾自由行` | `第一次去韓國，從這裡開始` | Human-friendly but under-expresses the target query | P0 |
| Intro / Quick Answer | “first trip: do these things / here is itinerary” | editorial starter framing | Needs a compact planning order before discovery modules | P0 |
| H2 | full planning lifecycle | only 3 top sections | Current architecture reads like homepage starter, not SERP-complete guide | P0 |
| H3 | many exact planning tasks | 5 core decisions | Strong decisions, insufficient search-surface breadth | P1 |
| keyword language | `自由行`, `攻略`, `行程`, `懶人包`, `新手` | `第一次去韓國`, `自由行指南` | Taiwan query vocabulary underused | P1 |
| intent | comprehensive first-trip planning | starter hub / next-question routing | Material role ambiguity | P0 |
| depth | one-page comprehensive | distributed into sibling detail pages | Distributed depth is a strength if hub routing is explicit | KEEP |
| practical info | broad entry/transport/stay/budget | strongest in linked detail pages | Homepage should summarize decision, not duplicate all facts | P1 |
| decision support | usually generic recommendation | decision-first architecture | Korea Inside advantage | KEEP |
| freshness | year/date prominent | title no year | Current year could be useful if maintained automatically/appropriately | P1 |
| entities | many attractions/services | major service/topic entities | Could expose more destination/arrival/stay entities in contextual hub copy | P1 |
| internal links | many article/product links | excellent 58-page language network | Quantity already strong; contextual “next question” labeling can improve | P1 |
| FAQ | often strong first-timer FAQ | none detected in current page | Add only if it captures real first-trip decision questions | P2 |
| CTA | commercial booking deep | editorial routing | Good balance; do not turn homepage into OTA | KEEP |
| media / map | itinerary maps / area visuals common | not central | A high-level first-trip decision map could help, but not required | P2 |
| trust / source | blogger experience / platform updates | Korea Inside editorial system | Add visible checked/official-source cues only to volatile modules | P1 |
| commercial usefulness | direct booking | routes to service/stay detail | Strong funnel when contextual links are made more explicit | KEEP |

## I. KEEP

- Homepage as a **decision gateway**, not a copied 5-day itinerary article.
- Existing links to accommodation, airport, eSIM, payment, maps and area guides.
- “first hour after arrival” practical framing.
- Korea Inside’s modular architecture: answer → next question → detail page.

## J. P0 / P1 / P2

### P0

**Resolve page role before implementation.** Two valid directions exist, but they should not be mixed accidentally:

1. Recommended: keep homepage as `第一次去韓國 / 韓國自由行新手` starter hub and make the planning sequence unmistakable; or
2. expand it into a full `韓國自由行` guide — which would create large duplication and is not preferred.

Recommendation: **Direction 1.**

### P1

- put `韓國自由行` / `新手` language more clearly in H1/first screen
- add a compact “先決定什麼” sequence
- expose entry/checklist and itinerary-next-step links more clearly
- add current-year freshness only where maintainable

### P2

- FAQ for first trip
- simple orientation / planning graphic

## K. SEO Surface Gap

**Direction only — not approved public copy:**

- Title direction: `韓國自由行新手攻略 2026｜第一次去韓國先從住宿、交通與付款開始`
- H1 direction: `第一次去韓國自由行：新手先從這裡開始`
- First screen: default order such as `住宿區域 → 入境/抵達 → 上網 → 機場交通 → 交通卡/付款 → 地圖 → 第一天`

Do not promise a full 5D4N itinerary unless one is actually provided.

## L. Content Gap

The material gap is **planning sequence**, not word count. A Taiwan first-timer should understand in 15–30 seconds what to settle first and which Korea Inside detail page answers each question.

## M. Taiwan-Specific Market Gap

Taiwan competitor pages frequently make Taiwan-origin concerns explicit:

- Taiwan passport / entry status
- TWD / Korean won handling
- Taiwan-friendly payment framing
- eSIM / roaming choice
- first Seoul trip route

Korea Inside should only surface these when officially verified and maintainable. Do not copy blogger-specific current rule claims into evergreen homepage copy.

## N. Competitor-Only Content

Do **not** automatically copy:

- giant 5D4N attraction schedules
- personal “I stayed / I tested” experience
- large affiliate coupon blocks
- volatile currency / entry claims without primary-source verification

## O. New Page Candidate

**NEW PAGE CANDIDATE — `首爾自由行 5天4夜 / 5–7天行程`**

Reason: itinerary intent is distinct from the homepage’s first-trip decision hub, has enough depth, and can link bidirectionally with area guides, stay, airport and transport pages.

## P. GEO / AI Search

AI-extractable answers should include:

- 第一次去韓國，最先決定什麼？
- 住宿區域要先選，還是先排行程？
- 抵達韓國後第一個小時要做什麼？
- eSIM、T-money、付款、地圖哪個要出發前處理？
- 什麼情況下應該先看首爾住宿 / 機場交通 / 支付頁？

## Q. Internal Links

### Outbound

Keep and strengthen contextual links to:

- accommodation
- arrival / airport / airport transfer
- eSIM
- T-money / WOWPASS / payment
- maps/apps/checklist
- area guides

### Inbound

Priority inbound links from:

- `checklist.html`
- `arrival.html`
- `airport.html`
- `accommodation.html`
- main area guides where a first-timer “start here” bridge is natural

### Cluster logic

`第一次去韓國` → `住宿 / 抵達 / 上網 / 支付 / 地圖` → specific detail → area / stay / booking action.

## R. Final Primary Keyword

**`韓國自由行`**  
Secondary framing: **`第一次去韓國`**

## S. Final Status

# **P0**

Editorial role clarification required before implementation. Recommended outcome: **first-time Korea decision hub, not itinerary clone.**

---

# PAGE 2 — `taste-korea.html`

## A. Page Role

Korea-wide food-travel discovery hub: help the traveler decide **what kind of Korean food experience they want and where it fits geographically and temporally**.

## B. Search Intent

For Taiwan users, the head food SERP strongly leans toward:

- `韓國美食`
- `韓國必吃`
- `首爾美食推薦`
- named dishes
- named restaurants
- neighborhood restaurant lists

Users first ask **“what should I eat?”** before they ask **“which city or neighborhood changes the food experience?”**

## C. Primary / Supporting Traditional-Chinese Keywords

**Primary:** `韓國美食`  
**Supporting:** `韓國必吃`, `韓國美食推薦`, `韓國料理`, `韓國美食攻略`, `首爾美食`

## D. Taiwan SERP Observation

Dominant format:

- must-eat dish list
- restaurant recommendation list
- area-based restaurant list
- TOP10 / TOP25 / TOP49
- sometimes official food hub / food encyclopedia

A useful non-list pattern also exists: Korea-wide dish guides that tell first-timers what the food is and how/when to eat it. This is closer to Korea Inside’s potential moat than a restaurant catalog.

## E. Direct Editorial Benchmark #1

**Source:** 波比看世界  
**URL:** https://bobbytravel.tw/seoul-food/  
**Observed H1:** `〖2026首爾美食推薦〗韓國首爾必吃美食攻略！東大門、明洞、弘大等激推彙整`

### Structure

- large master list / Top list
- neighborhood grouping: Dongdaemun, Myeongdong, Hongdae, etc.
- individual restaurant capsules
- practical notes / personal eating experience

### Strength

Excellent exact-match search language and large entity coverage.

### Weakness vs Korea Inside

Restaurant inventory ages quickly. It does not naturally solve “what kind of meal fits this day, city, time, solo/group condition?” as well as Korea Inside can.

## F. Direct Editorial Benchmark #2

**Source:** BringYou  
**URL:** https://www.bring-you.info/zh-tw/seoul-must-eat  
**Observed H1:** `〖2026首爾美食推薦〗明洞、弘大、東大門必吃餐廳TOP25｜烤肉、炸雞、醬蟹攻略`

### Structure

- familiar Korean dish anchors
- neighborhood sections
- restaurant recommendations
- queues / price / solo dining / local popularity signals

### Strength

Strong mapping from `what to eat` → `where to eat`.

### Weakness vs Korea Inside

Still list-led. Korea Inside can differentiate through meal timing, solo/group friction, neighborhood route fit, market-vs-restaurant choice, and city-level differences.

## G. Korea Inside Current Structure

**Current source SHA:** `79dc343820479fac6a0030fed85a2016e5db0fa4`

- **Title:** `品味韓國：美食體驗、城市與住宿地點 | Korea Inside`
- **Meta:** `從市場、烤肉、咖啡廳、在地料理、海鮮到料理課，找到適合這趟旅行的韓國美食體驗，再比較哪些城市與街區能讓行程更順。`
- **H1:** `品味 韓國`

### H2

1. `你期待怎樣的韓國美食旅行？`
2. `不同場景，有不同的用餐節奏。`
3. `讓美食參與路線規劃。`
4. `在首爾，街區會改變美食旅行的樣子。`
5. `美食旅行常見問題`
6. `規劃韓國旅行的其他部分`

### Major H3 themes

- 市場與街頭小吃
- 烤肉與深夜用餐
- 咖啡廳與潮流空間
- 傳統與在地料理
- 海鮮與沿海美食
- 料理課與導覽體驗
- 首爾 / 釜山 / 全州 / 濟州
- 鍾路 / 麻浦 / 弘大與延南 / 乙支路 / 聖水 / 明洞

### Existing FAQ

8 visible questions, including first food experience, city choice, Gwangjang Market, solo barbecue, where to stay for food, Jeonju day trip vs overnight, cooking class/tour, dietary restrictions.

## H. 1:1 SEO GAP Table

| Element | Competitor pattern | Korea Inside | Gap / judgment | Priority |
|---|---|---|---|---|
| Title | `首爾美食推薦`, `韓國必吃`, dish names | `品味韓國：美食體驗、城市與住宿地點` | Editorially elegant but weak head-query match | P0 |
| Meta | restaurant/dish/area promise | experience/city/stay promise | Differentiated but should mention `韓國美食`/`必吃` question more directly | P1 |
| H1 | exact `首爾美食推薦 / 必吃` | `品味 韓國` | Major search-facing mismatch | P0 |
| Intro / Quick Answer | list immediately answers what to eat | starts from trip experience type | Needs first answer to “what should I eat / how should I choose?” | P0 |
| H2 | dish / neighborhood / restaurant categories | experience / rhythm / route / neighborhood | Korea Inside architecture is stronger for travel decisions | KEEP |
| H3 | named restaurants and dishes | markets, BBQ, cafes, local food, seafood, classes, cities/areas | Needs clearer canonical dish/entity anchors before experience branching | P1 |
| keyword language | `美食推薦`, `必吃`, `餐廳`, dish names | `美食體驗`, `美食旅行` | Taiwan head wording underused | P0 |
| intent | what/where to eat | how food shapes the trip | Partial intent mismatch at entry point | P0 |
| depth | broad restaurant inventory | travel-context depth | Different strengths; do not copy list volume | KEEP |
| practical info | addresses, prices, queues | route / solo / timing / city fit | Korea Inside moat | KEEP |
| decision support | which restaurant | which food experience / city / neighborhood | Stronger editorial model | KEEP |
| freshness | restaurant lists require updates | evergreen architecture | Better maintainability, but key named entities should be current | KEEP |
| entities | large restaurant entity set | cities/areas/experience types | Missing a compact set of canonical Korean dish entities | P1 |
| internal links | restaurant/area articles | accommodation, K-beauty, eSIM, site network | Add contextual area-guide links where food section names the area | P1 |
| FAQ | often practical | 8 strong FAQs | Strong | KEEP |
| CTA | restaurant/booking/affiliate | low-pressure onward planning | Fine; experience CTA can be contextual | KEEP |
| media / map | restaurant maps common | no food orientation map central | Optional city/area food map only if it clarifies geography | P2 |
| trust / source | firsthand + list updates | editorial architecture | Official food-source references can strengthen canonical dish/fact sections | P1 |
| commercial usefulness | restaurant/tour clicks | cooking class / routing opportunity | Can monetize experiences without becoming restaurant catalog | P1 |

## I. KEEP

- food as part of the actual itinerary, not a detached Top 50 list
- market / BBQ / café / traditional food / seafood / class segmentation
- Seoul / Busan / Jeonju / Jeju distinction
- meal timing and traveler-condition logic
- existing 8-question FAQ

## J. P0 / P1 / P2

### P0

Search-facing entry must change from brand-like `品味韓國` to a page that clearly answers the `韓國美食` head problem.

### P1

- add a compact “第一次去韓國，先吃什麼？” decision layer
- introduce a small set of canonical dish categories/entities without becoming a listicle
- make city/area H3s contextually link to relevant Korea Inside area guides

### P2

- visual map of food styles by city/area
- structured “meal situation → best direction” summary

## K. SEO Surface Gap

**Direction only:**

- Title direction: `韓國美食攻略 2026｜必吃料理、市場、烤肉與城市怎麼選`
- H1 direction: `韓國美食攻略：第一次去韓國該吃什麼、去哪裡吃？`
- First screen: `經典韓食 / 市場小吃 / 烤肉 / 咖啡 / 海鮮 / 料理體驗` → who it suits → where it fits.

## L. Content Gap

The page lacks a strong **canonical “what to eat” layer** before the existing excellent “how food fits the trip” layer.

The solution is not 50 restaurants. A concise foundational set of Korean food categories/dishes can satisfy head intent, then hand off to the current decision architecture.

## M. Taiwan-Specific Market Gap

Taiwan SERP heavily uses:

- `必吃`
- `美食推薦`
- `首爾美食`
- recognizable dish names

Taiwan readers also commonly plan food by shopping district / transit area. Korea Inside should localize the query language but preserve the stronger route logic.

## N. Competitor-Only Content

Do not automatically copy:

- giant restaurant counts
- `Top 25 / Top 49` just for volume
- subjective “必吃” claims without editorial basis
- fast-aging restaurant prices/opening hours without maintenance capacity

## O. New Page Candidate

**NEW PAGE CANDIDATE — `首爾美食指南 / 首爾美食推薦`**

Reason: `首爾美食` is a distinct city-level search intent from a Korea-wide food hub. A Seoul detail page can own neighborhoods, solo dining, late-night food, markets, meal timing and internal links to area guides without forcing `taste-korea.html` to become a Seoul restaurant catalog.

## P. GEO / AI Search

Extractable answers:

- 第一次去韓國最值得先吃哪幾類料理？
- 一個人吃韓式烤肉可行嗎？
- 市場小吃和坐下來吃一餐，怎麼選？
- 首爾、釜山、全州、濟州的美食旅行有什麼差別？
- 哪種美食體驗值得預約？

## Q. Internal Links

### Outbound

Contextual links from city/area sections to:

- Hongdae / Myeongdong / Seongsu / Gongdeok-Mapo area guides
- accommodation where repeated food trips affect stay choice
- future Seoul food detail if approved

### Inbound

- area travel guides’ food sections → `taste-korea.html` or future city food detail
- homepage → food hub
- relevant itinerary/detail pages → food hub

### Cluster logic

`韓國美食` hub → `城市 / 區域` → `實際餐型 / 用餐情境` → `area guide / future food detail / experience action`.

## R. Final Primary Keyword

# **`韓國美食`**

## S. Final Status

# **P0**

Body concept should be protected; search-facing page role must be re-anchored to the real Taiwan head intent.

---

# PAGE 3 — `k-beauty.html`

## A. Page Role

Korea / Seoul K-Beauty travel decision guide covering shopping **and** bookable beauty experiences.

## B. Search Intent

Taiwan users enter through several overlapping intents:

- Korean cosmetics / skincare shopping
- Olive Young
- product selection / “必買”
- personal color analysis
- makeup / hair / scalp services
- dermatology / medical beauty
- where in Seoul to do each activity

2026 official and news coverage also shows a broader **beauty tourism / experience** layer, not product shopping alone.

## C. Primary / Supporting Traditional-Chinese Keywords

**Primary:** `韓國美妝`  
**Supporting:** `K-Beauty`, `Olive Young`, `韓國美妝攻略`, `個人色彩`, `韓國美容體驗`, `首爾美妝`

`韓國醫美` is relevant as a supporting entity but should not become the page’s sole identity.

## D. Taiwan SERP Observation

Mixed SERP:

- shopping / Olive Young guides
- product lists
- skincare explanation
- personal color / makeup / scalp experience articles
- medical/wellness tourism
- official K-Beauty festival material

This is favorable to Korea Inside because the current page already spans shopping + experiences + areas + repurchase.

## E. Direct Editorial Benchmark #1

**Source:** Seoul Window  
**URL:** https://www.seoulwindow.com/zh-tw/guides/korea-beauty-skincare-guide/  
**Observed H1:** `韓國美妝保養全攻略 2026：Olive Young怎麼買、成分怎麼看、韓妞真正在用什麼`

### Structure

- quick-answer bullets
- industry / ingredient understanding
- what people use
- where to buy
- Olive Young store choice
- personal color / hair services
- climate/skin context

### Strength

Excellent `韓國美妝 + Olive Young` search wording and strong first-screen utility.

### Weakness vs Korea Inside

More product/industry heavy. Korea Inside is better positioned to integrate shopping, reservations, neighborhood choice, trip schedule, and stay location.

## F. Direct Editorial Benchmark #2

**Source:** ToYouPick  
**URL:** https://toyoupick.com/zh/stories/glowcation-seoul-kbeauty-2026.html  
**Observed H1:** `只是去首爾買化妝品？K-Beauty 旅行其實已經不太一樣了`

### Structure

- shopping is no longer the only K-Beauty trip
- personal color
- makeup
- scalp
- dermatology / skin experience
- “glowcation” travel-trend framing

### Strength

Very close to Korea Inside’s experiential thesis and timely for 2026.

### Weakness vs Korea Inside

Less comprehensive on area-by-area decision, stay-location implication, repurchase and end-to-end trip planning.

### Supporting official / market evidence

- 2026 Korea Beauty Festival includes K-Beauty, beauty medicine, wellness, styling and bookable experiences.
- Taiwan CNA coverage in July 2026 specifically reported makeup experiences and color diagnosis as part of the growth of Korean beauty tourism.

## G. Korea Inside Current Structure

**Current source SHA:** `c8a158c1ba902f3273d7ceebfe6db231035cf792`

- **Title:** `韓國 K-Beauty：體驗、首爾各區與回購指南 | Korea Inside`
- **Meta:** `依照你的需求、預約安排與回國後補貨方式，挑選 K-Beauty 購物、個人色彩分析、美髮與頭皮護理、醫美診所諮詢，以及適合前往的首爾區域。`
- **H1:** `韓系美妝 K-Beauty 韓國體驗指南`

### H2

1. `K-Beauty 不只是購物行程`
2. `在首爾哪裡探索 K-Beauty`
3. `在韓國買，還是之後再回購？`
4. `K-Beauty 行程常見問題`
5. `以 K-Beauty 為重點的旅客，在首爾住哪裡？`

### H3

- 保養品購物
- 彩妝與個人色彩
- 美髮與頭皮護理
- 醫美診所與諮詢
- 香氛與生活美妝
- 明洞 / 聖水 / 弘大 / 江南 / 狎鷗亭與清潭
- Olive Young online-store / repurchase section
- stay-location implication

### FAQ

8 visible questions.

## H. 1:1 SEO GAP Table

| Element | Competitor pattern | Korea Inside | Gap / judgment | Priority |
|---|---|---|---|---|
| Title | `韓國美妝`, `Olive Young`, 2026 | `韓國 K-Beauty：體驗、首爾各區與回購指南` | Strong concept, could lead with Taiwan head phrase `韓國美妝` | P1 |
| Meta | product/shopping or experience specifics | already names shopping, personal color, hair/scalp, clinic, areas | Very strong | KEEP |
| H1 | clear query / hook | `韓系美妝 K-Beauty 韓國體驗指南` | Awkward word order; should sound native Taiwan | P1 |
| Intro / Quick Answer | strong “what should I do?” bullet answer | experience thesis exists | Add compact shopping-vs-experience split | P1 |
| H2 | shopping/ingredients/services | shopping + area + repurchase + stay | Korea Inside broader travel architecture | KEEP |
| H3 | product/service entities | service types + areas | Strong; Olive Young can be surfaced more clearly without domination | P1 |
| keyword language | `韓國美妝`, `Olive Young`, `個人色彩` | `K-Beauty`, `韓系美妝`, experience terms | Taiwan query vocabulary partly underused | P1 |
| intent | mixed shopping + experience | mixed shopping + experience | Match | KEEP |
| depth | product depth or single experience | travel decision depth | Differentiated | KEEP |
| practical info | store/service details | scheduling + area + repurchase | Strong | KEEP |
| decision support | what to buy/do | what to buy/do + where + trip fit | Korea Inside advantage | KEEP |
| freshness | 2026 trends/products | structurally evergreen | Need current-source layer for promotions/availability only | P1 |
| entities | Olive Young, ingredients, studios | beauty categories + Seoul areas | Add selective high-value entity exposure | P1 |
| internal links | mostly commerce/service | nav links + accommodation | Add contextual area-guide links from each area H3 | P1 |
| FAQ | practical | 8 strong FAQ | Strong | KEEP |
| CTA | product/service booking | can map experience to relevant action | Good opportunity | P1 |
| media / map | product imagery / guide visual | not central | Optional “where each K-Beauty type fits” map | P2 |
| trust / source | data / trend sourcing | editorial | Official KBF / provider source layer can strengthen volatile claims | P1 |
| commercial usefulness | shopping / services | multiple matching experience opportunities | High, if relevant-section principle is maintained | KEEP |

## I. KEEP

- `K-Beauty 不只是購物行程` thesis
- shopping + personal color + hair/scalp + clinic + fragrance split
- area-by-area choice
- repurchase logic
- stay-location implication
- no unsupported “Taiwan traveler favorite” claims

## J. P0 / P1 / P2

### P1

- rewrite search surface into native Taiwan wording
- put `韓國美妝` before the more global `K-Beauty` label
- give first-screen choice between shopping and booked experiences
- add contextual links from Myeongdong / Seongsu / Hongdae / Gangnam area sections

### P2

- area/experience orientation graphic
- dedicated Olive Young detail only if maintenance and distinct intent justify it later

## K. SEO Surface Gap

**Direction only:**

- Title direction: `韓國美妝 K-Beauty 攻略 2026｜Olive Young、個人色彩與首爾美容體驗`
- H1 direction: `韓國美妝 K-Beauty 旅行指南：購物、個人色彩與美容體驗`
- Lead: “只是買產品 / 想先做個人色彩 / 想排美髮頭皮 / 想諮詢醫美” → route by trip need.

## L. Content Gap

No major body gap. The main missing layer is a stronger **first-screen decision tree** and contextual area links.

## M. Taiwan-Specific Market Gap

Taiwan users strongly recognize `Olive Young` and `韓國美妝`, while 2026 market coverage is expanding toward personal color and experience tourism. Korea Inside should bridge those two intents:

`買什麼` → `要不要做體驗` → `在哪區做` → `如何排進行程`.

Any Taiwan-specific pricing, card, tax-refund or cross-border repurchase claims require separate official verification before production copy.

## N. Competitor-Only Content

Do not copy:

- “50 must-buy products” volume
- trend products that age monthly
- unsupported price-comparison claims against Taiwan
- medical claims or clinic ranking without a separate evidence standard

## O. New Page Candidate

**HOLD / validate later:** `Olive Young 韓國購物攻略` is a distinct high-maintenance intent, but should only become a new page if Korea Inside can maintain store / sale / refund / inventory behavior without cannibalizing the main K-Beauty hub.

## P. GEO / AI Search

AI answer units:

- 第一次去韓國買美妝，先去明洞還是弘大？
- 個人色彩要排在購物前還是後？
- K-Beauty 旅行除了 Olive Young 還能做什麼？
- 哪些體驗通常需要預約？
- 如果美妝行程很多天，住哪個區域比較合理？

## Q. Internal Links

### Outbound

Contextual body links from area H3s to:

- Myeongdong Guide
- Seongsu Guide
- Hongdae Guide
- Gangnam Guide
- accommodation decision hub

### Inbound

- Myeongdong / Seongsu / Hongdae / Gangnam shopping/experience sections → K-Beauty hub
- homepage Discover module → K-Beauty

### Cluster logic

`韓國美妝` hub → `shopping / personal color / hair / clinic` → `area guide` → `stay / booking action`.

## R. Final Primary Keyword

# **`韓國美妝`**

## S. Final Status

# **P1**

Surgical SEO-surface and internal-link improvement; protect the body architecture.

---

# PAGE 4 — `hongdae-travel-guide.html`

## A. Page Role

Broad Hongdae area guide that answers **how to use Hongdae in a real day**, including where to start, how long to stay, Yeonnam/Mangwon/Hapjeong/Sangsu extensions, food, experiences, nightlife, weather and whether staying nearby matters.

## B. Search Intent

Taiwan SERP users commonly ask:

- 弘大怎麼逛？
- 弘大有什麼好玩？
- 弘大逛街從哪裡開始？
- 弘大一日遊怎麼排？
- 弘大 / 延南洞 / 望遠怎麼串？
- 弘大美食 / 住宿 / 夜生活

## C. Primary / Supporting Traditional-Chinese Keywords

**Primary:** `弘大攻略`  
**Supporting:** `弘大逛街`, `弘大景點`, `弘大一日遊`, `弘大商圈`, `弘大美食`, `延南洞`

## D. Taiwan SERP Observation

Dominant format:

- `弘大商圈攻略`
- shopping map
- “必去景點 / 必吃美食” lists
- one-day itinerary
- transport
- stay recommendations
- extension to Yeonnam / Mangwon

Competitors explicitly answer “怎麼逛 / 從哪裡開始最順？” — the exact problem Korea Inside’s body already solves better than most listicles.

## E. Direct Editorial Benchmark #1

**Source:** BringYou  
**URL:** https://www.bring-you.info/zh-tw/hongdae  
**Observed H1:** `〖2026弘大商圈攻略〗逛街地圖、必吃美食、必去景點全整理`

### Structure

- intro / why Hongdae
- transport
- shopping map / four zones
- attractions
- Yeonnam / Mangwon extension
- experiences
- food
- lodging
- one-day route
- “值得嗎?”

### Strength

Excellent Taiwan query match. Clear map/list/route promise and strong entity coverage.

### Weakness vs Korea Inside

More “places to cover.” Korea Inside is materially stronger on:

- 3號出口 vs 9號出口 decision
- how long to stay in Yeonnam
- when to stop shopping
- meal timing
- paid-experience selection
- nightlife intensity choice
- traveler type
- bad-weather route
- stay-location effect

## F. Direct Editorial Benchmark #2

**Source:** Klook Taiwan  
**URL:** https://www.klook.com/zh-TW/blog/hongdae/  
**Observed H1:** `〖弘大一日遊攻略〗10個逛街景點、體驗活動、美食住宿全蒐集`

### Structure

- Hongdae intro
- airport / city transport
- 10 attractions
- experiences
- food
- lodging
- day-trip planning / booking actions

### Strength

Very direct `弘大一日遊` intent and strong commercial execution.

### Weakness vs Korea Inside

List / booking density is higher than travel judgment. Transport and operating details are volatile and must be independently verified rather than copied.

## G. Korea Inside Current Structure

**Current source SHA:** `fa03aa0d52d5528cfb660e33dddf9eb538a66304`

- **Title:** `弘大旅遊指南 2026 | Korea Inside`
- **Meta:** `以實用路線、街區選擇與旅行取捨，規劃 2026 年弘大行程：從延南、熱鬧的弘大核心區，到美食、購物與夜晚安排。`
- **H1:** `弘大旅遊指南 2026`

### H2 — 16 sections

- why travelers choose Hongdae
- stay bridge
- Exit 3 vs Exit 9
- Yeonnam timing
- Hongdae core by interest
- hotel-location effect
- food
- paid experiences
- after-dinner / nightlife
- stay-nearby decision
- Mangwon / Hapjeong / Sangsu extension
- route selection
- traveler types
- bad weather
- Sep–Oct 2026 current events
- final stay bridge

### H3 — key differentiators

- Exit 3 / Exit 9 start choice
- 2–2.5 hour Yeonnam guidance
- K-pop vs fashion shopping
- pop-up volatility
- photo booths
- shopping bag friction
- meal timing / solo-ordering rules
- K-pop dance / perfume / ring-making decision
- busking / live music / karaoke / club-bar intensity
- Mangwon market / Mangridan / Han River / Hapjeong / Sangsu
- 3–4 hour / half-day / full-day variants
- solo / couple / family / friends
- rain / heat route changes

## H. 1:1 SEO GAP Table

| Element | Competitor pattern | Korea Inside | Gap / judgment | Priority |
|---|---|---|---|---|
| Title | `弘大商圈攻略`, `弘大一日遊攻略`, `逛街地圖` | `弘大旅遊指南 2026` | Generic translation-like SEO surface | P1 |
| Meta | shopping, food, attractions, route | route, neighborhoods, trade-offs, food, shopping, night | Strong; add `逛街 / 一日遊` if natural | P1 |
| H1 | exact Taiwan query phrase | `弘大旅遊指南 2026` | Should say `弘大攻略` naturally | P1 |
| Intro / Quick Answer | “how to explore / where to start” | body solves it, but search surface could expose answer faster | Add default route answer above fold | P1 |
| H2 | transport/list/food/stay/route | route-first decision architecture | Korea Inside stronger | KEEP |
| H3 | store/attraction entities | micro-friction / choice conditions | Korea Inside moat | KEEP |
| keyword language | `攻略 / 逛街 / 一日遊 / 商圈` | `旅遊指南 / 行程` | Taiwan wording gap | P1 |
| intent | explore / shop / one day | explore / route / stay / night | Strong match | KEEP |
| depth | large lists | very high decision depth | Korea Inside stronger | KEEP |
| practical info | map, transport, stores | exit, time, weather, bags, meal, night | Korea Inside stronger | KEEP |
| decision support | which places | how to build the day | Major advantage | KEEP |
| freshness | 2026 updated pages | current Sep–Oct event layer | Good, but expired September items need state-aware maintenance | P1 |
| entities | shops / landmarks | route zones + experiences + nearby areas | Could add selective stable entities, not giant store list | P2 |
| internal links | many product/article links | Stay + comparison + global cluster | Add contextual transport/map/payment links at real need points | P1 |
| FAQ | sometimes implicit | no `<summary>` FAQ detected | Optional; body already answers many questions | P2 |
| CTA | strong booking | contextual experience logic | Korea Inside can monetize without overloading | KEEP |
| media / map | shopping map strongly featured | existing at-a-glance map asset | Protect and make orientation role clear | KEEP |
| trust / source | blogger/platform | current-event facts need official source | Maintain official current-event source layer | P1 |
| commercial usefulness | high | high via matching experiences/stay | Strong if context-specific | KEEP |

## I. KEEP

Protect nearly all body architecture, especially:

- Exit 3 vs Exit 9
- Yeonnam timing
- shopping stop rule / bag friction
- meal scheduling
- experience booking decision
- nightlife intensity choice
- Mangwon/Hapjeong/Sangsu extension logic
- traveler-type routes
- weather fallback
- stay bridge

This is the Korea Inside moat. Do not replace it with “10 must-visit places.”

## J. P0 / P1 / P2

### P1

- SEO surface: `弘大攻略 / 弘大逛街 / 弘大一日遊`
- a first-screen default route answer
- refresh current-event layer as September items expire
- add contextual next-question links beyond navigation

### P2

- FAQ only for high-value questions not already extractable
- selectively improve stable entity coverage

## K. SEO Surface Gap

**Direction only:**

- Title direction: `弘大攻略 2026｜逛街、美食、延南洞與一日路線怎麼排`
- H1 direction: `弘大攻略 2026：從延南洞到弘大商圈怎麼逛最順`
- First-screen answer: first-timer default route + shorter alternative + nightlife exception.

## L. Content Gap

No major body gap. The gap is **query naming + answer extraction**.

## M. Taiwan-Specific Market Gap

Taiwan pages frequently frame Hongdae as:

- `逛街`
- `商圈`
- `一日遊`
- `必吃 / 必買`

Korea Inside should use that language to get the click, then deliver a better route than the listicle promise.

Do not add Taiwan-dollar price comparisons unless maintained and sourced.

## N. Competitor-Only Content

Do not copy:

- giant store inventories
- fixed “all shops open X–Y” claims
- volatile promotional products
- transport snippets without official verification
- “must visit” inflation

## O. New Page Candidate

No new page required from this Batch. `弘大一日遊` is currently close enough to the existing page role; splitting now risks cannibalization unless query data later proves a separate itinerary intent.

## P. GEO / AI Search

Extractable answers:

- 第一次去弘大，3 號出口還是 9 號出口？
- 弘大要留幾小時？
- 延南洞和弘大怎麼排最順？
- 弘大晚上不去夜店還能做什麼？
- 望遠市場要不要和弘大排同一天？
- 下雨時弘大怎麼改行程？
- 什麼情況適合住弘大？

## Q. Internal Links

### Outbound

- Hongdae Stay Guide
- Hongdae vs Myeongdong
- maps/apps at route-planning friction points
- airport/AREX where arrival/stay context naturally arises
- payment/T-money where shopping or transit creates the next question

### Inbound

- homepage
- accommodation / traveler-type stay pages
- airport-access stay page
- future Seoul itinerary/detail pages

### Cluster logic

`弘大攻略` → `where to start / route / food / night` → `stay decision / airport / payment / map`.

## R. Final Primary Keyword

# **`弘大攻略`**

## S. Final Status

# **P1**

Protect the body; improve the Taiwan search surface and freshness layer.

---

# PAGE 5 — `myeongdong-travel-guide.html`

## A. Page Role

Myeongdong area guide that explains **when Myeongdong is actually useful, how long to spend, which station side to use, how to stop shopping from consuming the day, what to eat, and which one evening extension to choose**.

## B. Search Intent

Taiwan users commonly search:

- 明洞逛街攻略
- 明洞商圈
- 明洞必買 / 必逛
- 明洞美食 / 必吃
- 明洞換錢
- 明洞景點
- 明洞住宿
- 明洞怎麼逛 / 地圖

## C. Primary / Supporting Traditional-Chinese Keywords

**Primary:** `明洞攻略`  
**Supporting:** `明洞逛街`, `明洞商圈`, `明洞必買`, `明洞美食`, `明洞景點`, `明洞換錢`

## D. Taiwan SERP Observation

Dominant format:

- shopping map
- must-buy / must-eat list
- currency exchange
- transport
- attractions
- accommodation
- nearby Namsan / Namdaemun
- FAQ on tax refund / luggage

Korea Inside has a much stronger anti-overplanning / “what Myeongdong is actually good for” thesis, but the Title/H1 should speak the Taiwan search language more clearly.

## E. Direct Editorial Benchmark #1

**Source:** BringYou  
**URL:** https://www.bring-you.info/zh-tw/myeongdong  
**Observed H1:** `〖2026首爾明洞逛街攻略〗換錢所、必吃美食、必買衣服、伴手禮購物指南`

### Structure

- transport
- accommodation
- money exchange
- food / night market
- attractions / shows / cathedral
- beauty/fashion shopping
- department stores
- Namsan / Namdaemun extension

### Strength

Excellent match to Taiwan search vocabulary and practical shopping questions.

### Weakness vs Korea Inside

It treats more items as things to cover. Korea Inside is stronger at:

- how much time Myeongdong deserves
- Myeongdong Station vs Euljiro 1-ga
- when to stop shopping
- shopping-bag friction
- meal vs street-snack decision
- one extension only (Namsan / Namdaemun / Euljiro)
- “stay here ≠ spend all day here”

## F. Direct Editorial Benchmark #2

**Source:** FunTime Taiwan  
**URL:** https://www.funtime.com.tw/blog/funtime/myeongdong-guide  
**Observed H1:** `〖明洞逛街地圖〗首爾明洞必買必逛！美食、景點、購物一次看`

### Structure

- airport to Myeongdong
- exchange
- attractions
- food
- shopping
- FAQ: tax refund, luggage storage

### Strength

Strong practical task coverage and `逛街地圖` intent.

### Weakness vs Korea Inside

List-centric and less opinionated about what not to do. Korea Inside’s time-budgeting and stop rules are materially more useful.

## G. Korea Inside Current Structure

**Current source SHA:** `471169603f2369d2638eb4b769312a6af6f395fd`

- **Title:** `明洞旅遊指南 2026 | 景點、美食與購物`
- **Meta:** `以美妝、購物、美食、路線與晚間活動的實用建議，規劃 2026 年明洞行程，串連南山、南大門、乙支路與 NANTA。`
- **H1:** `明洞旅遊指南 2026`

### H2 — 17 sections

- why Myeongdong is useful
- time available
- Myeongdong Station vs Euljiro 1-ga
- hotel/station effect
- shopping without losing half a day
- 2026 K-Beauty shopping
- hotel-nearby / bags effect
- street food vs meal
- cathedral
- bookable experiences
- choose one extension: Namdaemun / Namsan / Euljiro
- bad weather
- routes
- “don’t spend all day here; staying here can be rational”
- who fits / who can move on
- current September 2026 info
- default route

### High-value H3 logic

- 2h / 3–5h / full-day / stay-in-Myeongdong branches
- Myeongdong Station vs Euljiro 1-ga
- shopping intent types
- Olive Young / personal color / tax refund prep
- bag friction
- street food vs restaurant
- Myeongdong Kyoja / Hadongkwan role difference
- NANTA
- Namdaemun vs Namsan vs Euljiro extension
- rain route
- traveler types

## H. 1:1 SEO GAP Table

| Element | Competitor pattern | Korea Inside | Gap / judgment | Priority |
|---|---|---|---|---|
| Title | `明洞逛街攻略`, `必買`, `美食`, `換錢` | `明洞旅遊指南 2026 | 景點、美食與購物` | Strong topics, but `旅遊指南` is weaker than Taiwan `逛街攻略` wording | P1 |
| Meta | shopping/exchange/food/transport | beauty/shopping/food/routes/evening | Strong; could mention `逛街` / station choice | P1 |
| H1 | `明洞逛街攻略` | `明洞旅遊指南 2026` | Search wording gap | P1 |
| Intro / Quick Answer | must-buy/must-eat overview | “Myeongdong is useful for specific things; do not overstay” | Korea Inside more distinctive; make quick route answer more extractable | P1 |
| H2 | exchange/food/shopping/attractions | decision-first time/route/shopping/meal/extension | Korea Inside stronger | KEEP |
| H3 | shops/restaurants | route and friction logic + selected entities | Strong | KEEP |
| keyword language | `逛街`, `商圈`, `必買`, `換錢` | `旅遊指南`, `購物` | Add Taiwan terms naturally | P1 |
| intent | shopping + practical visit | same plus stay/route judgment | Strong match | KEEP |
| depth | high list/entity volume | high decision depth | Korea Inside moat | KEEP |
| practical info | exchange/airport/refund/luggage | station/route/time/bags/meal/weather/stay | Missing a concise payment/exchange bridge | P1 |
| decision support | where to shop/eat | whether/how long/which side/when to stop | Korea Inside stronger | KEEP |
| freshness | updated 2026 | H2 says `2026 年九月` | **Stale as of 2026-10-04**; urgent current-layer refresh | P1-URGENT |
| entities | many stores / money changers | stable shopping/food/experience entities | Do not chase every shop; selective stable entities are enough | KEEP |
| internal links | broad | Stay Guide + global cluster | Add contextual payments/ATM/cards and K-Beauty links | P1 |
| FAQ | refund/luggage FAQs common | no `<summary>` FAQ detected | Could answer 4–6 high-friction questions if not duplicative | P2 |
| CTA | OTA/tickets | selected experience/stay bridge | Strong if context-matched | KEEP |
| media / map | shopping map common | route explanation is strong; map not central | Orientation map would materially help station/route decision | P2 |
| trust / source | frequent updated list | current events + official facts | Keep volatile facts tied to official/current sources | P1 |
| commercial usefulness | high shopping/hotel/tickets | K-Beauty, NANTA, stay, nearby attractions | Strong; preserve relevant-section mapping | KEEP |

## I. KEEP

Protect:

- Myeongdong is useful, but not necessarily all-day
- 2h / half-day / stay-base variants
- Myeongdong Station vs Euljiro 1-ga
- shopping intention and stop rule
- shopping-bag friction
- street food vs proper meal
- Myeongdong Kyoja vs Hadongkwan role distinction
- choose **one** extension: Namdaemun / Namsan / Euljiro
- weather adjustments
- “staying here can make more sense than sightseeing here all day”

## J. P0 / P1 / P2

### P1 — urgent freshness

The `明洞近期資訊：2026 年九月` section is stale on **2026-10-04**. This should be refreshed or neutralized in the current-information layer before/with SEO implementation. Do not rewrite the evergreen body because of it.

### P1 — search surface

- use `明洞攻略 / 明洞逛街` naturally
- add concise payment/exchange/tax-refund decision bridge to dedicated Korea Inside pages / official sources rather than duplicating money-changer lists
- contextual K-Beauty link from beauty-shopping section

### P2

- orientation map: Myeongdong Station / Euljiro 1-ga / cathedral / shopping core / direction to Namdaemun-Namsan-Euljiro
- focused FAQ if it adds extractable answers

## K. SEO Surface Gap

**Direction only:**

- Title direction: `明洞攻略 2026｜逛街、美食、K-Beauty、景點與半日路線`
- H1 direction: `明洞攻略 2026：逛街、美食與半天怎麼排最順`
- First screen: `2 小時 / 半天 / 住明洞` three-way quick choice.

## L. Content Gap

Not a list-volume gap. The material gap is a **small Taiwan practical module** around:

- exchange/payment choice
- tax refund / passport prep
- luggage handling

Most of this should link to dedicated Korea Inside payment/card/ATM content and official sources, not become volatile shop lists.

## M. Taiwan-Specific Market Gap

Taiwan SERP strongly associates Myeongdong with:

- `換錢`
- `必買`
- `退稅`
- `行李寄放`
- `美妝`

Korea Inside should acknowledge those intents at the right decision point while avoiding unsupported “best exchange rate” or exact shop-ranking claims.

## N. Competitor-Only Content

Do not copy:

- long money-changer rankings
- real-time exchange-rate claims
- giant brand/store inventories
- fixed “must-buy” lists
- volatile price thresholds without official current verification

## O. New Page Candidate

No new page required from this Batch. Payment/exchange/ATM questions already fit existing Korea Inside service pages and should be solved by internal linking, not a duplicate Myeongdong money page.

## P. GEO / AI Search

Extractable answers:

- 明洞要留 2 小時、半天還是一整天？
- 明洞站和乙支路 1 街，從哪一站開始？
- 明洞最適合買什麼？
- 明洞街頭小吃能不能當一餐？
- 南大門、南山、乙支路只能選一個時，怎麼選？
- 什麼人適合住明洞？
- 買很多東西時，飯店位置為什麼會改變行程？

## Q. Internal Links

### Outbound

- Myeongdong Stay Guide
- K-Beauty
- payments
- foreign cards / card-declined / ATM where context fits
- airport bus / airport transfer for stay-arrival context
- maps/apps when navigation is discussed

### Inbound

- homepage
- accommodation / first-time stay page
- K-Beauty hub
- future Seoul itinerary page

### Cluster logic

`明洞攻略` → `time / station / shopping / meal / evening` → `K-Beauty / stay / payment / airport`.

## R. Final Primary Keyword

# **`明洞攻略`**

Secondary high-value phrase: `明洞逛街`

## S. Final Status

# **P1**

Protect the body; improve Taiwan query wording and fix the stale September current layer.

---

# 7. Cross-Page Findings

## 7.1 Taiwan SERP is more explicit than current Korea Inside search surfaces

The recurring Taiwan SERP vocabulary is highly task-oriented:

- `攻略`
- `自由行`
- `逛街`
- `一日遊`
- `美食推薦`
- `必吃`
- `必買`

Korea Inside often has a better body but a softer label such as `旅遊指南` or a branded/editorial phrase such as `品味韓國`.

**Rule:** Taiwan Title/H1 can be more direct without making the body more generic.

## 7.2 List volume is not Korea Inside’s deficit

Competitors frequently win the click with:

- TOP 10 / TOP 25 / TOP 49
- many restaurant/store entities
- “必去 / 必買 / 必吃” wording

Korea Inside should not answer by copying the volume. Its defensible advantage is:

- when to stop
- where to start
- how long to stay
- luggage / shopping-bag friction
- station / exit choice
- route sequencing
- who should skip the area
- where the hotel changes the day
- alternative for weather / traveler type

## 7.3 Discover pages need stronger head-query anchoring

`taste-korea.html` and, to a lesser degree, `k-beauty.html` show the risk of using an editorial concept before the query is clear.

- `品味韓國` does not say `韓國美食` strongly enough.
- `韓系美妝 K-Beauty 韓國體驗指南` is understandable but not native search-facing Taiwan wording.

## 7.4 Current / event layers need month-state discipline

- Hongdae: `2026 年九月至十月` remains partly relevant on 2026-10-04 but September-only entries need ended/current state handling.
- Myeongdong: `2026 年九月` is stale on 2026-10-04.

The evergreen architecture should stay locked while current modules refresh separately.

## 7.5 Global navigation links are not enough for topic-cluster SEO

All five pages have strong same-language navigation coverage, but contextual internal links should be added where the user’s **next question actually arises**.

Examples:

- K-Beauty “明洞” section → Myeongdong Guide
- Myeongdong beauty shopping → K-Beauty
- Hongdae route/stay choice → Hongdae Stay + airport/maps as needed
- Taste Korea neighborhood section → relevant Area Guide

---

# 8. Taiwan SEO Restructuring Rules — Batch 01 Additions

1. **Taiwan head wording first:** when the body is already strong, fix Title/H1/lead before rewriting sections.
2. **Use `攻略 / 自由行 / 逛街 / 美食 / 美妝` where they describe the real page role.** Do not hide the query behind editorial branding.
3. **Do not imitate giant listicles.** Entity volume is not a substitute for decision structure.
4. **Broad Discover hubs need one explicit head problem.** “What should I eat?” before “how food shapes the trip”; “shopping or experience?” before area detail.
5. **Contextual links count more than menu presence.** Global navigation does not replace a next-question link inside the relevant paragraph.
6. **Time-sensitive layers remain separate from evergreen body architecture.** Month-expired modules should be refreshed without reopening locked evergreen judgments.
7. **Taiwan-market specifics need primary verification.** Payment, tax refund, entry rules, phone/eSIM, prices and promotions cannot be imported from competitor copy.
8. **GEO/AI answers must be extractable.** Each page should expose default choice, alternative, exception and the main friction in compact text.
9. **Search-surface directness must not become recommendation inflation.** `攻略` is fine; unsupported `最佳 / 必住 / 一定要` is not.

---

# 9. P0 Action List

## `index.html`

- Decide and lock role: **first-time Korea decision hub**.
- Rebuild Title/H1/lead around `韓國自由行 + 第一次去韓國` without pretending the page is a full Seoul itinerary.
- Add an explicit planning sequence and route each step to existing detail pages.

## `taste-korea.html`

- Re-anchor Title/H1/intro to `韓國美食`.
- Add a compact “what to eat first” canonical food layer.
- Preserve city / neighborhood / meal-context architecture.
- Do not convert into Top-restaurant inventory.

---

# 10. P1 Action List

## `k-beauty.html`

- Lead with `韓國美妝` wording.
- Naturalize H1.
- Add shopping-vs-experience quick choice.
- Add contextual area-guide links.

## `hongdae-travel-guide.html`

- Title/H1: `弘大攻略` / `弘大逛街` language.
- Surface the default first-time route sooner.
- Maintain Sep/Oct event state.
- Add next-question service links where friction occurs.

## `myeongdong-travel-guide.html`

- Title/H1: `明洞攻略` / `明洞逛街` language.
- Refresh September current layer immediately.
- Add concise exchange/payment/refund bridge via existing service pages and official sources.
- Add contextual K-Beauty link.

---

# 11. P2 Action List

- Home: lightweight trip-planning orientation graphic.
- Taste Korea: optional city/food-style orientation map.
- K-Beauty: optional area × experience orientation visual.
- Hongdae: FAQ only if it adds new extractable answers beyond the strong body.
- Myeongdong: orientation map for two stations + shopping core + adjacent extensions.

---

# 12. New Page Backlog

## NEW PAGE CANDIDATE 1 — Seoul itinerary

**Intent:** `首爾自由行 5天4夜 / 5–7天行程`  
**Why distinct:** itinerary sequencing is materially different from the first-time homepage decision hub.  
**Cannibalization risk:** manageable if homepage owns “start planning” and itinerary page owns day-by-day route.  
**Status:** RESEARCH CANDIDATE — no Production approval.

## NEW PAGE CANDIDATE 2 — Seoul food guide

**Intent:** `首爾美食 / 首爾美食推薦`  
**Why distinct:** current `taste-korea.html` is Korea-wide and experience-oriented. Seoul food can own neighborhoods, markets, solo dining, late-night meals and meal timing.  
**Cannibalization risk:** low if Korea-wide hub → Seoul food detail relationship is explicit.  
**Status:** RESEARCH CANDIDATE — no Production approval.

## HOLD — Olive Young shopping guide

Strong distinct query, but high maintenance. Validate only after Taiwan keyword/traffic data and maintenance capacity are available.

---

# 13. Final Recommendation

### Do not start by rewriting all five bodies.

Recommended editorial order after the full Taiwan research program is complete:

1. **P0 role decisions:** `index.html`, `taste-korea.html`
2. **search-surface P1:** K-Beauty, Hongdae, Myeongdong
3. **freshness correction:** Myeongdong September current layer; Hongdae current-event state
4. **contextual internal-link closure**
5. **optional P2 visuals / FAQ only where they improve a real decision**

Batch 01 confirms the Japanese research lesson applies strongly to Taiwan:

> **The best Taiwan SEO change is often not “write more.” It is “name the search problem correctly, answer it immediately, then preserve the Korea Inside decision depth competitors lack.”**

---

# 14. Source Register

## Korea Inside current source

Repository: `leesuckpyo/Koreainside`, branch `main`, current `zh-tw/` files retrieved 2026-10-04.

- https://www.getkoreainside.com/zh-tw/
- https://www.getkoreainside.com/zh-tw/taste-korea.html
- https://www.getkoreainside.com/zh-tw/k-beauty.html
- https://www.getkoreainside.com/zh-tw/hongdae-travel-guide.html
- https://www.getkoreainside.com/zh-tw/myeongdong-travel-guide.html

## First-trip / Seoul free-travel benchmarks

- Paine — https://www.paine0602.com/seoul-travel/
- BringYou — https://www.bring-you.info/zh-tw/seoul-travel-guide
- Klook Taiwan — https://www.klook.com/zh-TW/blog/seoul-korea-self-guided-tour/
- 豌豆 — https://wandou.tw/south-korea-travel/

## Food benchmarks / supporting source

- 波比看世界 — https://bobbytravel.tw/seoul-food/
- BringYou — https://www.bring-you.info/zh-tw/seoul-must-eat
- VISITKOREA Traditional Chinese food hub — https://big5chinese.visitkorea.or.kr/svc/thingsToDo/foodTrip/special_main.do?lang=LABNDR__TDCSE
- Seoul city food content — https://tchinese.seoul.go.kr/

## K-Beauty benchmarks / official supporting sources

- Seoul Window — https://www.seoulwindow.com/zh-tw/guides/korea-beauty-skincare-guide/
- ToYouPick — https://toyoupick.com/zh/stories/glowcation-seoul-kbeauty-2026.html
- Korea Beauty Festival / Visit Korea Committee — https://vkc.or.kr/tc/main-project/korea-beauty-festival/
- VISITKOREA KBF — https://big5chinese.visitkorea.or.kr/
- Taiwan CNA K-Beauty tourism report — https://www.cna.com.tw/news/ahel/202607160307.aspx

## Hongdae benchmarks

- BringYou — https://www.bring-you.info/zh-tw/hongdae
- Klook Taiwan — https://www.klook.com/zh-TW/blog/hongdae/

## Myeongdong benchmarks

- BringYou — https://www.bring-you.info/zh-tw/myeongdong
- FunTime Taiwan — https://www.funtime.com.tw/blog/funtime/myeongdong-guide
- 波比看世界 — https://bobbytravel.tw/myeongdong/
- Klook Taiwan — https://www.klook.com/zh-TW/blog/myeong-dong/

## Transport verification support

- AREX official — https://www.arex.or.kr/main.do?langCd=en

---

# 15. Batch Close

- Taiwan target inventory confirmed: **58 pages**
- Batch 01 researched: **5 / 5**
- Cumulative Taiwan SEO research: **5 / 58**
- Duplicate pages researched: **0**
- HTML changes: **0**
- Git stage: **0**
- Commit: **0**
- Push: **0**
- Production changes: **0**

**Next inventory pages for Batch 02 (not yet researched):**

6. `seongsu-travel-guide.html`
7. `insadong-travel-guide.html`
8. `gangnam-travel-guide.html`
9. `jamsil-travel-guide.html`
10. `gongdeok-mapo-seoul-guide.html`

