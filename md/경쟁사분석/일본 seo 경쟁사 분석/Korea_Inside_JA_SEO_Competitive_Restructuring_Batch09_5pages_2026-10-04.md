# Korea Inside — Japanese SEO Competitive Restructuring Report
## Batch 9 — 5 Pages

**Date:** 2026-10-04  
**Status:** RESEARCH COMPLETE — SEO RESTRUCTURING REVIEW / NO IMPLEMENTATION  
**Language:** Japanese  
**Scope:** Existing Korea Inside Japanese 58-page program — Batch 9 / exactly 5 pages  
**Research basis:** Current Korea Inside Japanese source + current Japanese-language SERP sampling + direct Japanese competitor/editorial page review + official current transport / map / T-money sources where time-sensitive facts mattered  
**Implementation:** HTML 0 / Git 0 / Production 0  

### Batch 9 Pages
1. `ja/airport-transfer.html`
2. `ja/arex.html`
3. `ja/airport-bus.html`
4. `ja/maps.html`
5. `ja/tmoney.html`

> **SERP caution:** Search-result order varies by time, location, device and personalization. “Direct Editorial Benchmark #1/#2” identifies useful current Japanese comparison pages, not absolute Google Japan ranking positions.

---

# 0. Executive Summary

Batch 9 covers Korea travel infrastructure that is both **high-search-volume and highly time-sensitive**.

The strongest overall finding is:

> **Korea Inside’s information architecture is already very strong. The risk is not lack of depth; it is factual drift in fares, timetables, payment rules, Google Maps functionality and mobile T-money conditions.**

The five pages should therefore be treated differently from evergreen area guides.

## Final Batch 9 Classification

| Page | Final Status | Main Finding |
|---|---|---|
| Airport Transfer | **PASS** | Current door-to-door comparison is exactly aligned with the strongest Japanese 2026 competitors. Preserve “hotel location + luggage + terminal exit time” logic. |
| AREX | **PASS** | One of KI’s strongest transport pages. Exact direct-vs-all-stop comparison, current fare logic, T-money distinction and destination-based choice are highly competitive. |
| Airport Bus | **PASS** | Strongest differentiator is choosing the stop from the hotel backwards and treating operators/payment/baggage rules separately. This is better than a simple route-number list. |
| Maps | **P1 — Freshness Watch** | Current structure is excellent. Google Maps is in a 2026 transition period after conditional high-precision map-data export approval, so “what Google can/can’t do” needs periodic revalidation. |
| T-money | **P1 — Mobile / Apple Wallet Freshness** | Physical-card guidance is strong. Apple Wallet/mobile T-money is now a major Japanese search concern and needs clearly separated current rules for Wallet funding, Mobile Tmoney app funding, tourist usability and physical-card fallback. |

No P0 structural page was identified in Batch 9.

---

# 1. Current Korea Inside Japanese Source Baseline

## 1.1 Airport Transfer

- File: `ja/airport-transfer.html`
- SHA: `b1b630d7ce1728f37d61866222857b2321ca5cd4`
- Title: `仁川空港からソウル市内への行き方：AREX・バス・タクシー・送迎比較 | Korea Inside`
- Meta: `仁川空港からソウル市内への行き方を、AREX、空港バス、タクシー、コールバン、貸切送迎で比較。現在の運賃、荷物、深夜到着時の選び方までまとめています。`
- H1: `仁川空港からソウル市内へ： どの移動手段が合う？`

Core:
- full route to hotel
- comparison table
- AREX
- airport bus
- taxi
- call van / prebooked private transfer
- deep-night arrival
- T1/T2 differences
- traveler-type decision
- 10 FAQ
- official information

## 1.2 AREX

- File: `ja/arex.html`
- SHA: `f19b59a798ce0c9121dac41e635a4cf4716c7e3e`
- Title: `仁川空港AREXガイド：直通列車・一般列車（各駅停車）の料金・所要時間 | Korea Inside`
- Meta: `仁川空港からソウルへ向かうAREX直通列車と一般列車（各駅停車）を、料金、所要時間、停車駅、乗車券、T-money、荷物、深夜到着時の選び方まで比較します。`
- H1: `仁川空港からソウルへAREXで移動：直通列車 vs 一般列車（各駅停車）`

Core:
- direct vs all-stop
- fast comparison
- fare
- actual time saved
- stops
- after getting off
- T1/T2 station finding
- ticketing
- Seoul Station as intermediate point
- luggage/family/accessibility
- last train
- common mistakes
- 12 FAQ

## 1.3 Airport Bus

- File: `ja/airport-bus.html`
- SHA: `e8cbdca74c5c84ffc9aba03bca170b6c07303ca2`
- Title: `仁川空港バスガイド：路線・チケット・乗り方 | Korea Inside`
- Meta: `ソウル、京畿道、韓国各都市へ向かう仁川空港バスの選び方。T1/T2の路線・チケット・乗り場、荷物、深夜バス、空港へ戻る方法まで解説します。`
- H1: `仁川空港バスガイド`

Core:
- choose stop before bus number
- choose route based on post-stop walk
- operator structure
- T1/T2 ticketing
- luggage/family/deep night
- troubleshooting
- return to airport
- pre-departure check
- FAQ
- official info

## 1.4 Maps

- File: `ja/maps.html`
- SHA: `2aa984b6e0ea33524b3ebc3e5beb7adf5ef89443`
- Title: `韓国で使う地図アプリ：NAVER Map・KakaoMap・Google Maps | Korea Inside`
- Meta: `韓国でNAVER Map、KakaoMap、Google Mapsを使う方法。韓国語の場所検索、地下鉄出口、バス経路、英語検索で見つからないときの対処まで解説します。`
- H1: `韓国で使う地図アプリ：NAVER Map・KakaoMap・Google Mapsの使い方`

Current quick answer:
- main navigation = NAVER Map
- backup/local cross-check = KakaoMap
- planning/saved places/global reviews = Google Maps

## 1.5 T-money

- File: `ja/tmoney.html`
- SHA: `707979ea76f533a2c38ea4a66e2d67b9891e5147`
- Title: `韓国でのT-moneyの使い方 | 旅行者向け交通カードガイド`
- Meta: `韓国でT-moneyを使う方法を解説。カード型T-moneyの購入・チャージ、Apple Walletのチャージ条件、WOWPASSやClimate Cardとの違いまで確認できます。`
- H1: `韓国でのT-moneyの使い方`

Core:
- physical / iPhone / Android decision
- purchase
- initial top-up
- physical-card top-up
- subway/bus use
- non-transit use
- Apple Wallet
- Android mobile option
- youth
- low balance
- refund
- common failures
- T-money vs WOWPASS
- 12 FAQ
- official sources

---

# 2. PAGE 1 — Airport Transfer

## 2.1 Search Intent

Primary Japanese intent:

> **仁川空港からソウルのホテルへ、AREX・空港バス・タクシー・送迎のどれで行けばいい？**

The actual problem is not:
> “Which transport is fastest?”

It is:
> **Which mode produces the easiest full journey to my hotel, with my luggage, at my arrival time?**

## 2.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `仁川空港 ソウル 行き方`

Supporting:
- `仁川空港 ソウル AREX`
- `仁川空港 ソウル バス`
- `仁川空港 ソウル タクシー`
- `仁川空港 ソウル 送迎`
- `仁川空港 明洞 行き方`
- `仁川空港 弘大 行き方`
- `仁川空港 江南 行き方`
- `仁川空港 深夜 ソウル`
- `仁川空港 荷物 タクシー`
- `仁川空港 AREX バス どっち`

## 2.3 SERP Observation

Japanese 2026 competitors strongly converge on the same selection factors:

- hotel area
- luggage
- arrival time
- group size
- cost
- direct vs transfer
- late-night availability

This validates the current KI architecture.

The strongest competitor pages also move away from simplistic:
- `AREX is fastest`
- `bus is easiest`

and instead calculate the full route.

## 2.4 Direct Editorial Benchmark #1 — Seoul Window

URL:
`https://www.seoulwindow.com/ja/guides/incheon-airport-to-seoul-compared/`

Observed title:
`仁川空港→ソウルへの移動手段を比較：AREX・リムジンバス・プライベート送迎・タクシー、どれが安い？`

Observed current structure:
1. 2026 current update
2. how many modes
3. cost/time table
4. group size / luggage comparison
5. private transfer vs taxi
6. late-night actual options
7. baggage-delivery combination
8. numeric cost comparison

### Strength
- highly current
- quantitative
- group-size math
- luggage variable
- late-night reality

### Weakness vs KI
- can become price/calculation heavy
- hotel entrance and final walk are less central
- broader commercial/private-transfer emphasis
- less destination-specific Hongdae vs Seoul Station decision logic

## 2.5 Direct Editorial Benchmark #2 — Koripin

URL:
`https://koripin.com/guide/incheon-airport/`

Observed title:
`〖2026年最新〗仁川空港からソウル市内へ｜明洞・ソウル駅への空港鉄道・リムジンバス・タクシーの料金と選び方`

Observed structure:
- first comparison table
- AREX general
- AREX express
- limousine bus
- taxi
- destination-specific recommendation
- current official fare reference
- travel-day verification

### Strength
- direct Japanese query
- clear price/time
- area-based selection
- current 2026 date

### Weakness vs KI
- narrower mode set
- less call-van/private-transfer distinction
- less terminal-exit-time logic
- less complete luggage vehicle-type decision

## 2.6 Korea Inside Actual Structure

KI already:
- begins with complete route to accommodation
- compares five modes
- says Express AREX is for Seoul Station, not Hongdae
- makes bus value dependent on stop proximity
- differentiates taxi vehicle needs by luggage
- separates call van and private transfer
- uses terminal exit time, not landing time, for deep-night decisions
- distinguishes T1/T2
- 10 FAQ

This is highly competitive.

## 2.7 1:1 SEO GAP

| Element | Competitors | Korea Inside | Judgment | Priority |
|---|---|---|---|---|
| Title | exact transfer mode comparison | exact + private transfer | excellent | PASS |
| Meta | fare/time/modes | modes + current fare + luggage + late arrival | excellent |
| H1 | “how to get to Seoul” | “which mode fits?” | strong |
| Intro / Quick Answer | numeric table | whole-route principle | KI stronger |
| H2 | modes | same + decision logic | KI stronger |
| H3 | costs/locations | mode-specific use | strong |
| Keyword language | direct | direct | PASS |
| Intent | exact | exact | PASS |
| Depth | price/time | destination/friction | complementary |
| Practical info | fares/time | luggage/terminal/vehicle | KI stronger |
| Decision support | high | exceptional | KEEP |
| Freshness | fares/times | high maintenance | maintenance |
| Entities | modes | all major modes | strong |
| Internal links | transport only | AREX/bus/taxi/private/stay | KI advantage |
| FAQ | yes | 10 | strong |
| CTA | booking often high | contextual | strong |
| Trust/source | official fare references | official source section | strong |
| Commercial usefulness | high | high | PASS |

## 2.8 KEEP

- current title/meta/H1
- hotel door-to-door logic
- AREX direct vs all-stop destination choice
- stop-near-hotel logic for bus
- luggage/vehicle type
- taxi
- call van
- private transfer
- terminal exit time
- T1/T2
- FAQ

## 2.9 P0 / P1 / P2

### P0
None.

### P1
No strategic change.

Maintenance:
- fares
- late-night routes
- taxi surcharges
- bus stops
- private-transfer conditions

### P2
No structural expansion required.

## 2.10 SEO Surface Gap

No urgent rewrite.

The page is already very close to the strongest 2026 Japanese decision model.

## 2.11 Content Gap

No material gap.

## 2.12 Japan-Specific Market Gap

Japanese competitors show strong demand for:
- exact yen conversion
- cheap vs fast
- late-night
- family/group

KI can show KRW as source truth and avoid over-maintaining exchange-rate conversions.

## 2.13 Competitor-Only Content

- per-person private-transfer break-even calculator
- luggage delivery
- live exchange-rate conversions

Useful P2 ideas, not necessary core.

## 2.14 New Page Candidate

None.

Existing:
- AREX
- Airport Bus
- Taxi
- Private Transfer
already decompose the intent.

## 2.15 GEO / AI Search

Answer:
- cheapest
- fastest to Seoul Station
- best for Hongdae
- best for Myeongdong with luggage
- family/group
- deep-night
- T1/T2
- bus vs AREX
- taxi vs private transfer

## 2.16 Internal Links

Outbound:
- AREX
- Airport Bus
- Taxi
- Private Transfer
- Airport
- Arrival
- Airport Access Stay
- Maps

Inbound:
- Airport
- Arrival
- Airport Access Stay
- Home
- area Stay pages

## 2.17 Final Primary Keyword

**`仁川空港 ソウル 行き方`**

## 2.18 Final Status

# **PASS**

---

# 3. PAGE 2 — AREX

## 3.1 Search Intent

Primary Japanese intent:

> **仁川空港からAREXに乗るなら、直通列車と一般列車のどちら？ 料金・時間・停車駅・T-money・チケットはどう違う？**

This is a clean, specific utility query.

## 3.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `仁川空港 AREX`

Supporting:
- `AREX 直通 一般 違い`
- `AREX 料金`
- `AREX 所要時間`
- `AREX 弘大`
- `AREX ソウル駅`
- `AREX T-money`
- `AREX チケット`
- `AREX 乗り方`
- `AREX 終電`
- `AREX T1 T2`
- `AREX 直通列車 13000`

## 3.3 SERP Observation

Current Japanese competitors are highly standardized:
- Express vs All-stop table
- adult fare
- T1/T2 time
- Hongdae stops only on all-stop
- T-money only on all-stop
- reserved vs commuter seating
- purchase method
- last train

Korea Inside already covers all of this plus what happens **after** the station.

## 3.4 Direct Editorial Benchmark #1 — KOREA-LOGUE

URL:
`https://korealogue.net/2026/08/12/incheon-airport-arex-guide/`

Observed title:
`〖2026年最新〗仁川空港AREX完全ガイド｜直通・一般列車の料金・乗り場・乗り方`

Observed current comparison:
- Express:
  - Seoul Station only
  - 13,000 KRW adult current sale fare
  - reserved seat
- All-stop:
  - T1→Seoul Station 4,750 KRW
  - T2→5,350 KRW
  - Hongik/Gongdeok stops
  - transit card
- station finding
- ticket purchase
- NAVER Map hotel route

### Strength
- exact current fares
- current date
- very easy Japanese scan
- good station/ticket detail

### Weakness vs KI
- less destination-after-Seoul-Station logic
- less family/accessibility/luggage nuance
- less explicit common-mistake system

## 3.5 Direct Editorial Benchmark #2 — UtilKorea

URL:
`https://utilkorea.com/ja/korea/travel/incheon-airport-to-seoul-arex`

Observed current structure:
- direct numeric comparison
- Express vs all-stop
- current fare/time
- frequency
- ticket type
- how much time saved
- official-source references
- last train

### Strength
- quantitative
- frequency differences
- transparent source/date
- clean decision table

### Weakness vs KI
- more “train product” focused
- less hotel-area continuation
- less “Seoul Station is often not the final destination” reasoning

## 3.6 Current 2026 Fare Layer

Current Japanese sources and KI current source agree on the critical high-level structure:
- Express current sale fare: **13,000 KRW adult**
- All-stop:
  - T1 → Seoul Station: **4,750 KRW**
  - T2 → Seoul Station: **5,350 KRW**
- Express:
  - no T-money
  - separate ticket
  - reserved seats
- All-stop:
  - T-money/transit card
  - commuter-style seating
  - Hongdae/Gongdeok intermediate stops

These are time-sensitive and must continue to be checked against AREX current official information.

## 3.7 Korea Inside Actual Structure

Current KI already includes:
- first decision
- quick comparison
- current fares
- actual time saving
- major stations
- destination after AREX
- T1/T2 station finding
- ticket types
- Seoul Station as intermediate point
- family/luggage/accessibility
- last-train back-calculation
- mistakes
- 12 FAQ
- official sources

This is excellent.

## 3.8 1:1 SEO GAP

| Element | Competitors | Korea Inside | Judgment | Priority |
|---|---|---|---|---|
| Title | exact AREX/direct/all-stop/fare | exact | PASS |
| Meta | fare/time/stops/ticket | same + luggage/late arrival | KI stronger |
| H1 | direct vs all-stop | exact | PASS |
| Intro / Quick Answer | table | decision first | strong |
| H2 | fare/stops/tickets | all plus after-arrival route | KI stronger |
| H3 | numeric details | full station/route detail | strong |
| Keyword language | exact | exact | PASS |
| Intent | exact | exact | PASS |
| Depth | product utility | product + traveler route | KI stronger |
| Practical info | excellent | excellent | PASS |
| Decision support | high | exceptional | KEEP |
| Freshness | fares/timetable | high maintenance | maintenance |
| Entities | stations | stations + destination links | strong |
| Internal links | airport transfer | full airport/stay cluster | KI advantage |
| FAQ | yes | 12 | strong |
| CTA | ticket booking | neutral/current | strong |
| Trust/source | official | official-source section | strong |
| Commercial usefulness | high | high | PASS |

## 3.9 KEEP

- Title/meta/H1
- Express vs all-stop
- fare
- time saved
- Hongdae/Gongdeok
- T-money distinction
- ticketing
- T1/T2
- Seoul Station is often not final destination
- luggage/family
- last train
- FAQ

## 3.10 P0 / P1 / P2

### P0
None.

### P1
No structural work.

Maintenance:
- fare
- timetable
- ticket channels
- City Airport Terminal conditions
- payment methods

### P2
No major expansion.

## 3.11 SEO Surface Gap

No urgent rewrite.

Current page is already appearing as a strong exact-answer source.

## 3.12 Content Gap

No strategic gap.

## 3.13 Japan-Specific Market Gap

Japanese users often see third-party discounted Express prices.

KI should keep:
- official/base/current sale fare
- ticket-channel differences
separate from temporary OTA discounts.

## 3.14 Competitor-Only Content

- coupon codes
- OTA discounts
- detailed train-count statistics

Not required as core.

## 3.15 New Page Candidate

None.

## 3.16 GEO / AI Search

Answer:
- Express vs all-stop
- current fare
- T-money
- Hongdae
- Seoul Station
- T1/T2
- large luggage
- last train
- airport bus alternative

## 3.17 Internal Links

Outbound:
- Airport Transfer
- Airport
- Airport Bus
- T-money
- Airport Access Stay
- Seoul Station / Hongdae / Gongdeok Stay

Inbound:
- Airport Transfer
- Airport
- Stay pages
- Home/arrival

## 3.18 Final Primary Keyword

**`仁川空港 AREX`**

## 3.19 Final Status

# **PASS**

---

# 4. PAGE 3 — Airport Bus

## 4.1 Search Intent

Primary Japanese intent:

> **仁川空港から空港リムジンバスに乗るには、どの番号・停留所・チケット・乗り場を選べばいい？**

The hard part is not memorizing bus numbers.

It is:
- which stop is nearest the hotel
- which operator runs it
- T1/T2 boarding rules
- ticket/payment rule
- luggage rule
- late-night route

## 4.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `仁川空港 リムジンバス`

Supporting:
- `仁川空港 空港バス`
- `仁川空港 バス 乗り方`
- `仁川空港 バス チケット`
- `仁川空港 バス T1 T2`
- `仁川空港 明洞 バス`
- `仁川空港 江南 バス`
- `仁川空港 東大門 バス`
- `仁川空港 深夜バス`
- `空港リムジン 荷物`
- `空港バス T-money`

## 4.3 SERP Observation

Japanese competitor structure usually starts:
1. which bus number?
2. ticket purchase
3. boarding location
4. fare
5. baggage

Korea Inside uses a better traveler-first model:

> **hotel → usable stop → operator/route → ticket → boarding**

This is a meaningful differentiation.

## 4.4 Direct Editorial Benchmark #1 — KOREA-LOGUE

URL:
`https://korealogue.net/2026/08/12/incheon-airport-limousine-bus-guide/`

Observed title:
`〖2026年最新〗仁川空港リムジンバス完全ガイド｜バス番号の調べ方・乗り場・料金・乗り方`

Observed structure:
- find bus before leaving Japan
- use NAVER Map around hotel
- identify stop
- identify bus number
- T1/T2 ticket purchase
- boarding
- luggage
- fare
- destination examples

### Strength
- very direct
- current 2026
- good pre-trip sequence
- practical for first-time users

### Weakness vs KI
- route-number centric after stop lookup
- less operator-rule distinction
- less city-vs-Gyeonggi-vs-intercity architecture
- less return-to-airport logic

## 4.5 Direct Editorial Benchmark #2 — K Travel Way

URL:
`https://www.ktravelway.com/ja/airport-transfers/airport-limousine-bus-guide/`

Observed current highlights:
- T2 requires pre-purchase for city-bound routes
- T1 rules can differ by operator/route
- return-direction payment rules differ
- baggage limits vary
- climate-type passes are not universal bus payment
- current source verification date

### Strength
- operator/payment nuance
- current verification
- recognizes that airport-wide universal payment rules are misleading

### Weakness vs KI
- compressed guide
- less hotel/stop geography
- less family/deep-night troubleshooting
- less full outbound/return journey planning

## 4.6 Current Official Airport Structure

Incheon Airport current bus pages show:
- multiple operators
- separate route groups
- separate T1/T2 boarding zones
- Seoul / Gyeonggi / regional / Incheon / late-night categories
- ticket offices and vending machines
- schedules may change without notice
- late-night Seoul routes differ by terminal

This strongly supports KI’s current “operator-specific, not one universal bus system” framing.

## 4.7 Korea Inside Actual Structure

KI already:
- chooses stop before bus number
- judges post-stop walk
- separates:
  - Seoul
  - Incheon/Gyeonggi
  - other Korean cities
  - deep-night
- names multiple Seoul bus operators
- separates T1/T2 purchase/boarding
- says payment is operator-specific
- handles baggage/family/group
- troubleshooting
- return trip
- FAQ

This is a very strong current page.

## 4.8 1:1 SEO GAP

| Element | Competitors | Korea Inside | Judgment | Priority |
|---|---|---|---|---|
| Title | route/ticket/how-to | exact | PASS |
| Meta | route/ticket/platform | T1/T2 + luggage + late-night | KI stronger |
| H1 | airport bus guide | exact | PASS |
| Intro / Quick Answer | find bus number | find stop from hotel | KI stronger |
| H2 | ticket/platform | stop/operator/payment/return | KI stronger |
| H3 | bus routes | categories/operators/troubleshooting | strong |
| Keyword language | direct | direct | PASS |
| Intent | exact | exact | PASS |
| Depth | strong | very strong | KI stronger |
| Practical info | excellent | excellent | PASS |
| Decision support | high | exceptional | KEEP |
| Freshness | very high | high maintenance | maintenance |
| Entities | bus numbers/operators | operators + modes | strong |
| Internal links | transport | airport/stay/maps cluster | KI advantage |
| FAQ | yes | 10 | strong |
| CTA | low | neutral | appropriate |
| Trust/source | current official | official sources | strong |
| Commercial usefulness | medium | decision utility high | PASS |

## 4.9 KEEP

- Title/meta/H1
- stop before bus number
- last walk
- operator distinction
- Seoul vs Gyeonggi vs regional
- T1/T2
- payment not universal
- baggage
- family
- deep night
- troubleshooting
- airport return
- FAQ

## 4.10 P0 / P1 / P2

### P0
None.

### P1
No strategic change.

Maintenance:
- operator list
- T1/T2 boarding positions
- payment
- baggage
- deep-night route
- return ticket conditions

### P2
No structural expansion required.

## 4.11 SEO Surface Gap

No urgent rewrite.

## 4.12 Content Gap

No material gap.

## 4.13 Japan-Specific Market Gap

Japanese guides often oversimplify:
- “T-money can be used”
- “credit card can be used”

KI should preserve operator-specific wording.

## 4.14 Competitor-Only Content

- every bus number table
- every hotel stop
- current timetable copied into page

These are too volatile for evergreen body.

## 4.15 New Page Candidate

No new page needed.

Hotel-area bus details can be contextual modules on Stay pages if justified.

## 4.16 GEO / AI Search

Answer:
- how to pick bus
- T1 vs T2
- ticket location
- payment
- luggage
- deep night
- hotel stop
- return airport
- bus vs AREX

## 4.17 Internal Links

Outbound:
- Airport Transfer
- AREX
- Airport
- Airport Access Stay
- Maps
- area Stay details

Inbound:
- Airport Transfer
- Airport
- area Stay pages
- First-Time/Family
- Home

## 4.18 Final Primary Keyword

**`仁川空港 リムジンバス`**

## 4.19 Final Status

# **PASS**

---

# 5. PAGE 4 — Maps

## 5.1 Search Intent

Primary Japanese intent:

> **韓国旅行ではGoogle Mapsだけで足りる？ NAVER MapとKakaoMapのどれを使えばいい？**

Supporting problems:
- Japanese-language UI
- Korean-name search
- station exits
- bus stop direction
- walking routes
- saved places
- offline use
- restaurant/store discovery

## 5.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `韓国 地図アプリ`

Supporting:
- `韓国 NAVER Map`
- `韓国 ネイバーマップ`
- `韓国 Google Maps 使える`
- `NAVER Map 日本語`
- `KakaoMap 日本語`
- `韓国 地図 アプリ おすすめ`
- `韓国 徒歩 ナビ`
- `韓国 地下鉄 出口 アプリ`
- `韓国 バス 地図`
- `NAVER Map 使い方`

## 5.3 SERP Observation

Current Japanese consensus is:
- NAVER Map = primary local navigation
- KakaoMap = second local map / route check
- Google Maps = familiar planning/saved list/review tool

This is exactly the current KI quick answer.

The new issue is **Google Maps transition risk**.

In 2026:
- Korea approved high-precision map-data export to Google under conditions
- this does not mean full local navigation immediately became equivalent to Naver/Kakao
- current September 2026 checks still show incomplete/full rollout issues

Therefore, static claims such as:
> “Google Maps cannot do X in Korea”

need periodic rechecking.

KI’s current wording is already cautious:
> functionality can vary; test actual route and keep a local map app.

That is the right strategy.

## 5.4 Direct Editorial Benchmark #1 — HaniSeoul

URL:
`https://www.haniseoul.com/ja/travels/korea/map-app-korea`

Observed title:
`韓国地図アプリ比較：Naver Map・KakaoMap・Google Mapsの使い方`

Observed structure:
1. why Google differs
2. purpose-based comparison
3. Naver
4. Kakao
5. Google
6. Korean-address search
7. subway exits
8. bus stops
9. taxi
10. latest foreigner Naver verification feature

### Strength
- exact same core keyword
- current functionality
- practical address/entrance/search detail
- current platform features

### Weakness / Risk vs KI
- some new platform-feature claims are highly volatile
- account/reservation/payment features can change faster than navigation
- can drift into app-service features beyond travel routing

## 5.5 Direct Editorial Benchmark #2 — bento.travel

URL:
`https://www.bento.travel/ja/south-korea/navigation`

Observed title:
`韓国のおすすめナビゲーションアプリ（2026年） - Naver Map`

Observed answer:
- Naver Map essential/default
- KakaoMap secondary
- use Korean names/addresses
- route/navigation focus

### Strength
- extremely concise
- high extractability
- easy “one app” answer

### Weakness vs KI
- less troubleshooting
- less Google saved-list use
- less subway-exit/bus-stop detail
- less actual travel-scene switching

## 5.6 Current Official Naver Language Fact

NAVER official help currently lists:
- Korean
- English
- Simplified Chinese
- Japanese

for app language settings, with OS/version conditions.

That supports the Japanese-user recommendation.

## 5.7 Korea Inside Actual Structure

KI already:
- compares all three apps
- gives one job to each
- pre-trip preparation
- Naver Map language setup
- search
- branch verification
- route
- subway exit
- bus stop
- saved/share
- Korean-name fallback
- Kakao backup
- Google strengths/limitations
- scenario-based switching
- common mistakes
- FAQ

This is excellent.

## 5.8 1:1 SEO GAP

| Element | Competitors | Korea Inside | Judgment | Priority |
|---|---|---|---|---|
| Title | `地図アプリ比較 / Naver` | exact three-app title | excellent |
| Meta | route/search | Korean names/exits/bus/English issue | KI stronger |
| H1 | exact | exact | PASS |
| Intro / Quick Answer | Naver first | Naver/Kakao/Google roles | excellent |
| H2 | app-by-app | app + use cases | KI stronger |
| H3 | settings/search | very detailed | strong |
| Keyword language | direct | direct | PASS |
| Intent | exact | exact | PASS |
| Depth | medium/high | very high | KI stronger |
| Practical info | route/search | exits/bus/meeting/hotel | KI stronger |
| Decision support | high | exceptional | KEEP |
| Freshness | Google platform transition | **high watch** | **P1** |
| Entities | three apps | exact | strong |
| Internal links | app pages | airport/stay/apps/eSIM | KI advantage |
| FAQ | some | 10 | strong |
| CTA | app store links | practical | strong |
| Trust/source | mixed | official Naver + current checks | strong |
| Commercial usefulness | low | utility high | PASS |

## 5.9 KEEP

- title/meta/H1
- Naver primary
- Kakao backup
- Google planning/saved-place role
- Korean-name/address fallback
- branch check
- exit
- bus-stop direction
- sharing
- pre-trip test
- FAQ

## 5.10 P0 / P1 / P2

### P0
None.

### P1 — Freshness Watch

Recheck periodically:
- Google walking directions
- Google driving/navigation
- local place database quality
- rollout after 2026 map-data-export decision
- Naver/Kakao language support
- app-account/reservation features

Do not rewrite based on policy announcement alone; update only when actual user-facing functionality changes.

### P2
No structural expansion.

## 5.11 SEO Surface Gap

No title/H1 rewrite required.

## 5.12 Content Gap

No material gap.

The main risk is outdated categorical claims.

## 5.13 Japan-Specific Market Gap

Japanese users strongly search:
- `日本語`
- Google Maps compatibility
- Naver Map how-to

KI already answers these.

## 5.14 Competitor-Only Content

- account verification
- restaurant reservation/payment
- app-specific local rewards

These are secondary to navigation and should only be added if verified and relevant.

## 5.15 New Page Candidate

No new page required.

## 5.16 GEO / AI Search

Answer:
- one app to install
- Google Maps enough?
- Naver Japanese support
- Kakao needed?
- Korean-name search
- correct exit
- correct bus stop
- offline use
- data connection

## 5.17 Internal Links

Outbound:
- Apps
- eSIM
- Airport
- Airport Transfer
- Taxi
- T-money
- Area Guides

Inbound:
- Home
- Airport/Arrival
- Apps
- Stay/Area guides at route-friction points

## 5.18 Final Primary Keyword

**`韓国 地図アプリ`**

## 5.19 Final Status

# **P1 — Freshness Watch**

---

# 6. PAGE 5 — T-money

## 6.1 Search Intent

Primary Japanese intent:

> **韓国旅行でT-moneyをどう買って、どうチャージして、地下鉄・バスでどう使う？ iPhoneでも使える？ WOWPASSと何が違う？**

In 2026, this intent has expanded significantly because of:
- Apple Wallet T-money
- Mobile Tmoney app
- Climate Card short-term products
- tourist-specific Tmoney products
- WOWPASS/NAMANE comparison

The physical-card basics are still important, but no longer the whole query.

## 6.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `韓国 T-money 使い方`

Supporting:
- `T-money 買い方`
- `T-money チャージ`
- `T-money 現金`
- `T-money Apple Pay`
- `T-money iPhone`
- `T-money Apple Wallet`
- `T-money Android`
- `T-money WOWPASS どっち`
- `T-money 払い戻し`
- `T-money 子供`
- `韓国 交通カード`

## 6.3 SERP Observation

Traditional Japanese T-money pages still use:
- buy at convenience store
- cash top-up
- tap in/out
- refund

But current 2026 pages increasingly include:
- iPhone / Apple Wallet
- mobile T-money
- WOWPASS
- Climate Card
- tourist cards
- foreign-card funding conditions

This creates a new SEO expectation.

## 6.4 Direct Editorial Benchmark #1 — Koripin

URL:
`https://koripin.com/guide/tmoney/`

Observed title:
`〖2026年最新〗T-moneyカード（ティーマネー）の買い方・チャージ・払い戻し｜ソウル旅行の交通カード`

Observed structure:
1. what T-money is
2. where to buy
3. cash top-up
4. subway/bus
5. transfer rule
6. refund
7. practical screenshots
8. travel-use sequence

### Strength
- exact beginner intent
- very straightforward
- current date
- physical-card workflow is easy to follow

### Weakness vs KI
- mobile/iPhone decision is less central
- less WOWPASS/Climate Card decision
- less Android/mobile-option architecture
- less “which format should I use?” first answer

## 6.5 Direct Editorial Benchmark #2 — Korea Travel Guide.com

URL:
`https://kortravelguide.com/wowpass/`

Observed current title:
`WOWPASSとT-moneyの違いは？どっちがいい？残高・チャージ・使い方を比較`

Observed 2026 structure:
- T-money vs WOWPASS
- separate balances
- physical T-money
- mobile T-money
- Climate Card
- iPhone
- payment roles
- which traveler needs which card

### Strength
- reflects current card ecosystem
- explains that roles differ
- highlights separate WOWPASS/T-money balances
- recognizes mobile T-money

### Weakness vs KI
- primarily a comparison page
- not a complete T-money setup/use/refund guide
- some card/product conditions are volatile

## 6.6 Supporting Current Competitor Signal — Trip2KR

Current page:
`T-money vs WOWPASS vs NAMANE (2026)`

Its quick answer treats:
- physical T-money
- Apple Wallet T-money
- WOWPASS
- NAMANE

as competing choices.

This confirms mobile/iPhone intent is now prominent.

## 6.7 Current Official Apple / Tmoney Facts

### Apple Wallet Tmoney
Apple’s current Korea transit information confirms:
- prepaid Tmoney can be added to Apple Wallet
- compatible iPhone / Apple Watch required
- Express Transit supported
- direct Wallet top-up/purchase uses supported payment conditions
- Apple’s launch terms state Korean-issued credit/debit cards are required for adding money through Apple Wallet

### Mobile Tmoney app
Tmoney’s 2026 guide says:
- iOS Mobile Tmoney app provides another top-up route
- Naver Pay was added as an iOS top-up method in September 2026
- this is a distinct funding path from Wallet’s direct card funding

Important Japan-traveler conclusion:
> **Do not imply that every Japanese traveler can automatically top up Apple Wallet Tmoney with a Japanese-issued card.**

Whether a tourist can use a particular app/payment route depends on the payment/account conditions.

### Physical card
VISITKOREA still supports the core tourist workflow:
- buy at convenience stores / designated points
- top up
- tap on/off
- transfer discount
- refund conditions

So physical T-money remains the simplest universal fallback.

## 6.8 Korea Inside Current Fact Layer

Current Japanese page already says:
- physical top-up with cash is the most reliable default for foreign travelers
- do not assume all foreign credit cards work in standard physical-card top-up
- Apple Wallet T-money is supported
- direct Wallet funding conditions currently require Korean-issued card per Apple
- Android travel app offers separate mobile options
- WOWPASS solves a different payment problem
- initial load should not be unnecessarily large

This is fundamentally strong.

## 6.9 1:1 SEO GAP

| Element | Competitors | Korea Inside | Judgment | Priority |
|---|---|---|---|---|
| Title | buy/top-up/refund | broad `使い方` | strong |
| Meta | physical + mobile growing | physical + Apple + WOWPASS/Climate | strong |
| H1 | exact | exact | PASS |
| Intro / Quick Answer | physical card default | format choice | KI stronger |
| H2 | buy/top-up/use | physical/iPhone/Android/refund/compare | KI stronger |
| H3 | screenshots | troubleshooting | complementary |
| Keyword language | Apple Pay increasingly visible | Apple Wallet H2 | strong |
| Intent | broad travel card | broad | exact |
| Depth | physical card | ecosystem | KI stronger |
| Practical info | buy/top-up | mobile conditions + fallback | KI stronger |
| Decision support | medium | exceptional | KEEP |
| Freshness | **very high** | Apple/mobile conditions evolve | **P1** |
| Entities | T-money/WOWPASS | plus Climate Card/Android | strong |
| Internal links | comparison | WOWPASS/comparison/payments | KI advantage |
| FAQ | yes | 12 | strong |
| CTA | card/provider | neutral | trust advantage |
| Trust/source | blogs vary | Apple/Tmoney/VisitKorea possible | strengthen |
| Commercial usefulness | medium/high | high utility | strong |

## 6.10 KEEP

- title/meta/H1
- physical vs iPhone vs Android
- buy
- initial load
- physical top-up
- tap in/out
- non-transit use
- child/youth
- low balance
- refund
- common problems
- WOWPASS comparison
- 12 FAQ

## 6.11 P0 / P1 / P2

### P0
None.

### P1 — Mobile / Apple Wallet Freshness

Rework/verify the Apple/mobile section as a **funding-path matrix**:

#### Physical T-money
- purchase
- cash top-up
- broad fallback

#### Apple Wallet direct
- compatible device
- Express Transit
- direct Wallet funding condition
- whether Japanese-issued card works: do not assume yes

#### Mobile Tmoney app
- separate iOS funding methods
- current Naver Pay path
- traveler account/payment eligibility must be checked

#### Android
- supported NFC/app/device conditions

Also recheck:
- Climate Card
- Tmoney Travel Card / Travel Card+
- discontinued legacy tourist-card names
- refund locations/thresholds

### P2
A simple comparison visual could help:
`Physical / iPhone / Android / WOWPASS / Climate Card`

## 6.12 SEO Surface Recommendation

Current title can remain.

Potential H2 candidate:
`iPhoneのT-money：Apple WalletとMobile Tmoneyアプリの違い`

This is clearer than treating Apple Wallet as one single funding system.

## 6.13 Content Gap

No broad content gap.

Main gap = current **mobile funding/eligibility clarity**.

## 6.14 Japan-Specific Market Gap

Japanese travelers disproportionately care about:
- iPhone
- Suica-like usability
- Japanese-issued card
- physical card unnecessary?
- WOWPASS both needed?
- cash top-up inconvenience

KI should directly answer these without promising unsupported card compatibility.

## 6.15 Competitor-Only Content

- referral codes
- promotional discounts
- one “best card” claim
- app-payment claims without tourist eligibility verification

Not core.

## 6.16 New Page Candidate

No new page required.

Existing:
- T-money
- WOWPASS
- T-money vs WOWPASS

already form the correct cluster.

## 6.17 GEO / AI Search

Answer:
- where to buy
- cash top-up
- foreign card
- Apple Wallet
- Japanese-issued card question
- Android
- tap off bus
- taxi/store use
- refund
- WOWPASS
- Climate Card

## 6.18 Internal Links

Outbound:
- WOWPASS
- T-money vs WOWPASS
- Payments
- Airport
- Maps
- AREX
- Airport Bus
- Apps

Inbound:
- Home
- Airport
- AREX
- Airport Bus
- Maps
- WOWPASS
- T-money vs WOWPASS
- Checklist

## 6.19 Final Primary Keyword

**`韓国 T-money 使い方`**

## 6.20 Final Status

# **P1 — Mobile / Apple Wallet Freshness**

---

# 7. Cross-Page Findings

## 7.1 Korea Inside’s Transport Architecture Is Already Highly Competitive

The five URLs are well separated:

### Airport Transfer
Which mode?

### AREX
Which train?

### Airport Bus
Which stop/route/operator?

### Maps
Which navigation app?

### T-money
Which transit-card format / how to use it?

This prevents one massive transport page from becoming stale and unreadable.

## 7.2 Door-to-Door Is the Strongest Cross-Page Principle

For airport transport:

> airport → mode → station/stop → exit → walk → hotel entrance

For maps:

> station → exit → street side → building entrance

For T-money:

> card/app → top-up → gate/bus → transfer → refund

The page should always solve the traveler’s **complete action**, not only name the service.

## 7.3 Time-Sensitive Utility Pages Need “Current Fact Layer” Governance

High-risk facts:
- AREX fare/timetable
- bus boarding positions
- deep-night routes
- payment rules
- Google Maps capabilities
- Apple Wallet funding
- mobile-app eligibility
- Climate Card rules

These should be checked independently from the editorial body.

## 7.4 Search-Facing Copy Is Already Mostly Strong

Unlike several Travel Guides in earlier batches, these utility pages generally already contain the exact Japanese terms users search.

The improvement priority is therefore:
1. freshness
2. conditions/exceptions
3. official-source accuracy
4. not more keywords

## 7.5 Japanese Mobile-Payment Context Changes Faster Than Physical Transport

Physical T-money:
- stable mental model

Mobile/Apple:
- fast-moving
- payment-method dependent
- account/region/device dependent

Do not make mobile T-money the universal default for foreign visitors unless the exact funding path is verified.

## 7.6 Google Maps Is Now a Watch Item, Not a Fixed Limitation

The 2026 map-data policy change means:
- old static explanations can become obsolete
- current user-facing functionality, not the policy announcement itself, should control the page

KI’s cautious wording is appropriate.

---

# 8. Japan SEO Restructuring Rules — Batch 9 Additions

## Rule 46 — Utility Pages Need a Fact-Refresh Layer Separate From Editorial Judgment

Editorial structure can stay locked while:
- fares
- routes
- platform/payment functions
change.

## Rule 47 — Transport Comparison Must End at the Hotel Door

Do not compare modes only to the city center.

## Rule 48 — “Supports T-money” Is Not Enough

For any transport/service:
- ticket or tap?
- T-money or separate ticket?
- operator-specific?
- terminal-specific?
- direction-specific?

## Rule 49 — Mobile Payment Must Specify Funding Path

Do not write:
`Apple WalletでT-moneyを使える`

and stop there.

Explain:
- card creation
- direct Wallet top-up
- app top-up
- payment eligibility
- tourist fallback

## Rule 50 — Map-App Advice Must Track Actual Feature Rollout

Policy change ≠ immediate app capability.

Use current functional verification.

---

# 9. P0 Action List

**None in Batch 9.**

No page requires page-role or representative-query redesign.

---

# 10. P1 Action List

## Maps
- periodically recheck actual Google Maps walking/driving behavior
- maintain Naver/Kakao language/support facts
- avoid frozen “Google cannot” wording

## T-money
- Apple Wallet vs Mobile Tmoney funding matrix
- current Japanese-tourist practicality
- current Travel Card / Climate Card naming and rules
- official source refresh

## All transport pages
- maintain current fares/timetables/payment rules

---

# 11. P2 Action List

- T-money physical/mobile/card comparison visual
- airport transport luggage/group calculator
- no additional structural pages required

---

# 12. New Page Backlog

No strong new URL is required from Batch 9.

Possible research-only candidates:
- `韓国旅行 eSIM vs ローミング` from prior Batch
- no additional airport/transport URL needed because the current cluster is already well decomposed

---

# 13. Final Recommendation

## Airport Transfer
**PASS**

Protect the whole-route selection model.

## AREX
**PASS**

Protect the current direct-vs-all-stop structure and maintain current fares.

## Airport Bus
**PASS**

Protect hotel-stop-first and operator-specific rules.

## Maps
**P1 Freshness Watch**

Structure is excellent; monitor Google Maps rollout and local-app features.

## T-money
**P1 Mobile / Apple Wallet Freshness**

Physical-card guidance is strong. Clarify the rapidly evolving mobile funding paths for Japanese travelers.

---

# 14. Source Register

## Korea Inside current Japanese source
- `ja/airport-transfer.html`
- `ja/arex.html`
- `ja/airport-bus.html`
- `ja/maps.html`
- `ja/tmoney.html`

## Airport Transfer benchmarks
- Seoul Window  
  https://www.seoulwindow.com/ja/guides/incheon-airport-to-seoul-compared/
- Koripin  
  https://koripin.com/guide/incheon-airport/
- Korea Playlist  
  https://korplaylist.com/ja/travel/incheon/incheon-airport-to-seoul-arex-bus-taxi-guide/
- HaniSeoul  
  https://www.haniseoul.com/ja/travels/korea/incheon-airport-to-seoul

## AREX benchmarks / current sources
- KOREA-LOGUE  
  https://korealogue.net/2026/08/12/incheon-airport-arex-guide/
- UtilKorea  
  https://utilkorea.com/ja/korea/travel/incheon-airport-to-seoul-arex
- Creatrip  
  https://creatrip.com/ja/news/5708
- Korea Inside current indexed page  
  https://www.getkoreainside.com/ja/arex.html

## Airport Bus benchmarks / official
- KOREA-LOGUE  
  https://korealogue.net/2026/08/12/incheon-airport-limousine-bus-guide/
- K Travel Way  
  https://www.ktravelway.com/ja/airport-transfers/airport-limousine-bus-guide/
- Incheon Airport current bus information  
  https://www.airport.kr/ap_ja/1888/subview.do
- Airport Bus current timetable  
  https://www.airportbus.or.kr/ja/bus/schedule.php
- Incheon Airport late-night routes  
  https://business.airport.kr/ap_ja/1894/subview.do

## Maps benchmarks / current sources
- HaniSeoul  
  https://www.haniseoul.com/ja/travels/korea/map-app-korea
- bento.travel  
  https://www.bento.travel/ja/south-korea/navigation
- NAVER official language help  
  https://help.naver.com/service/5637/contents/8275
- current functional-verification benchmark  
  https://korea-travel-101.guide/blog/naver-map-vs-kakaomap-vs-google-maps-korea

## T-money benchmarks / official
- Koripin  
  https://koripin.com/guide/tmoney/
- Korea Travel Guide.com  
  https://kortravelguide.com/wowpass/
- Tmoney official iPhone / Apple Pay guide  
  https://blog.tmoney.co.kr/en/guide/iphone-transit-card-how-to-use-tmoney-with-apple-pay/
- Apple Tmoney launch/current conditions  
  https://www.apple.com/kr/newsroom/2025/07/apple-and-tmoney-introduce-tmoney-for-apple-pay-on-iphone-and-apple-watch/
- Apple transit page  
  https://www.apple.com/kr/apple-pay/transit/
- VISITKOREA Tmoney guide  
  https://japanese1.visitkorea.or.kr/jpn/TRP/TR_JPN_9_1_1.jsp
- Tmoney foreign-traveler product page  
  https://eng.tmoney.co.kr/aeb/biz/bridge/foreignTravel.dev

---

# 15. Current Fact Notes Used for Freshness Review

These notes do not authorize public-copy changes by themselves.

## AREX
Current 2026 Japanese source checks align around:
- Express sale fare: 13,000 KRW adult
- All-stop:
  - T1 → Seoul Station: 4,750 KRW
  - T2 → Seoul Station: 5,350 KRW
- Express requires separate ticket and does not use standard T-money tap-in
- All-stop supports transit-card use and intermediate stops including Hongdae/Gongdeok

Recheck before implementation.

## Airport Bus
Current Incheon Airport pages:
- separate multiple operators
- list T1/T2 boarding zones
- distinguish Seoul/Gyeonggi/regional/late-night
- warn that schedules can change
- current payment/ticket rules can differ by operator/terminal/direction

Do not create one universal payment claim.

## Maps
Current 2026 state:
- Naver Map remains the safest default for local walking/transit/place search
- KakaoMap remains a useful second local map
- Google Maps remains useful for planning/saved places/global reviews
- Google’s future Korea navigation capabilities are in transition after a 2026 policy change and should be verified by actual app functionality, not assumed

## T-money
Current verified high-level:
- physical T-money remains broadly useful
- physical-card cash top-up remains the most reliable tourist default
- Apple Wallet Tmoney is supported on compatible devices
- Apple’s direct Wallet funding conditions remain important
- Tmoney’s iOS app now exposes additional funding paths such as Naver Pay
- app/payment eligibility is not the same as “all foreign cards work”
- physical-card fallback should remain visible

---

# 16. Batch Close

- Japanese pages analyzed: **5 / 5**
- Current Korea Inside Japanese source checked: **5 / 5**
- Japanese SERP intent review: **5 / 5**
- Direct competitor/editorial comparison: **5 / 5**
- Current official transport/map/T-money fact layer checked where needed
- 1:1 SEO GAP tables: **5 / 5**
- Internal-link analysis: **5 / 5**
- GEO / AI Search review: **5 / 5**
- New Page Candidate backlog: **reviewed**
- HTML changes: **0**
- Git changes: **0**
- Production changes: **0**

## Final Batch 9 Classification

1. Airport Transfer — **PASS**
2. AREX — **PASS**
3. Airport Bus — **PASS**
4. Maps — **P1 Freshness Watch**
5. T-money — **P1 Mobile / Apple Wallet Freshness**

**Batch 9 research: COMPLETE**
