# Korea Inside — Japanese SEO Competitive Restructuring Report
## Batch 6 — 5 Pages

**Date:** 2026-10-04  
**Status:** RESEARCH COMPLETE — SEO RESTRUCTURING REVIEW / NO IMPLEMENTATION  
**Language:** Japanese  
**Scope:** Existing Korea Inside Japanese 58-page program — Batch 6 / exactly 5 pages  
**Research basis:** Current Korea Inside Japanese source + current Japanese-language SERP sampling + direct Japanese competitor/editorial page review + current official hotel pages where useful  
**Implementation:** HTML 0 / Git 0 / Production 0  

### Batch 6 Pages
1. `ja/where-to-stay-in-myeongdong.html`
2. `ja/where-to-stay-in-hongdae.html`
3. `ja/hotels-near-seoul-station.html`
4. `ja/hotels-near-gongdeok-station.html`
5. `ja/where-to-stay-in-insadong.html`

> **SERP caution:** Search-result order varies by time, location, device and personalization. “Direct Editorial Benchmark #1/#2” identifies useful current Japanese comparison pages, not absolute Google Japan rank positions.

---

# 0. Executive Summary

Batch 6 is the first pure **Area Stay / Hotel Detail** batch.

The most important finding is that current Japanese hotel content is increasingly moving beyond generic “best hotels” lists and toward the same operational questions Korea Inside already prioritizes:

- which side of the station
- exact exit
- last walk with luggage
- whether a station is genuinely useful
- whether the area matches repeated daily routes
- room size
- bathroom layout
- family/group capacity
- airport transfer
- quietness vs nightlife

That validates the Korea Inside hotel-detail architecture.

## Final Batch 6 Classification

| Page | Final Status | Main Finding |
|---|---|---|
| Myeongdong Stay | **PASS** | Search intent and subarea/hotel structure are already excellent. KI is stronger than generic “hotel ranking” pages because it separates Myeongdong Station, central Myeongdong and Euljiro. |
| Hongdae Stay | **PASS** | Current page already matches the strongest Japanese competitor pattern: Hongdae Station vs Yeonnam vs Hapjeong + airport/noise/final-walk trade-offs. |
| Seoul Station Hotels | **PASS** | KTX/AREX intent is exact. KI correctly distinguishes “near Seoul Station” from actual platform-to-hotel convenience and avoids pretending the area suits everyone. |
| Gongdeok Station Hotels | **P1** | Search fit is strong and current hotel set is useful. Improve exact station-exit/elevator/final-walk evidence and strengthen Gongdeok-vs-Mapo practical distinction. |
| Insadong Stay | **P1** | Strong hotel/apartment structure, but Japanese market has additional interest in hanok-style stays, Japanese-comfort features and Jongno-side choices. These require fact-layer verification and careful role control, not blanket addition. |

No P0 page was identified in this Batch.

---

# 1. Current Korea Inside Japanese Source Baseline

## 1.1 Myeongdong Stay

- File: `ja/where-to-stay-in-myeongdong.html`
- SHA: `829e33a7ac01edaed8ce473a6e876e82c6e37e7e`
- Title: `明洞でどこに泊まる？おすすめエリアとホテル比較 2026 | Korea Inside`
- Meta: `明洞駅、乙支路、明洞中心部を比較し、地下鉄の使いやすさ、荷物、空港アクセス、家族旅行やグループ滞在に合わせて11軒のホテルから選びます。`
- H1: `明洞でどこに泊まる？ 2026`

Key current structure:
- Myeongdong Station side
- central Myeongdong
- Euljiro 1-ga / Sogong-dong
- Euljiro 3-ga side
- quick hotel table
- 11 hotel candidates
- simple sleeping base vs family/group room
- airport access
- repeated-route decision
- FAQ

## 1.2 Hongdae Stay

- File: `ja/where-to-stay-in-hongdae.html`
- SHA: `f7597b71ff66129373749631b7cebd7c0f3217e2`
- Title: `弘大でどこに泊まる？エリア・ホテル・立地ガイド 2026 | Korea Inside`
- Meta: `弘大の位置関係、弘大入口駅・延南・合井の違いを整理し、空港アクセス、荷物、ナイトライフ、静かな夜という条件から弘大の宿泊先を選びます。`
- H1: `弘大でどこに泊まる？エリア・ホテル・立地ガイド 2026`

Current structure:
- Hongdae geography
- Hongdae is not one block
- luggage changes walking-distance perception
- airport access vs final walk
- center-Seoul trade-off
- selection criteria
- 5–6 person stay
- hotel categories:
  - “Hongdae feel”
  - airport/AREX
  - lower-cost
  - quieter side
  - guesthouse
- 14 hotel/lodging candidates
- FAQ

## 1.3 Seoul Station Hotels

- File: `ja/hotels-near-seoul-station.html`
- SHA: `9ff57e215793e8b34e9af699f30846c3de0ee141`
- Title: `KTX・AREX利用に便利なソウル駅周辺ホテル比較 | Korea Inside`
- Meta: `KTX、AREX、早朝列車、出張、家族旅行に合わせてソウル駅周辺ホテルを比較。予約前に確認したい実用上の注意点まで整理します。`
- H1: `ソウル駅周辺ホテル 2026`

Current structure:
- should you stay near Seoul Station?
- who benefits
- why it is not for everyone
- what “near Seoul Station” actually means
- four subareas
- 8 hotel candidates
- KTX/AREX reality
- actual Korea travel use
- booking checks
- final verdict
- 6 FAQ

## 1.4 Gongdeok Station Hotels

- File: `ja/hotels-near-gongdeok-station.html`
- SHA: `7afc97af1a852f441d2873e760c8796ade04587c`
- Title: `孔徳駅周辺ホテル比較：麻浦でどこに泊まる？ | Korea Inside`
- Meta: `孔徳駅・麻浦駅周辺の7軒を、AREX、出張、長期滞在、家族旅行、落ち着いた夜という条件で比較します。`
- H1: `孔徳駅周辺ホテル 2026`

Current structure:
- what changes by staying in Gongdeok
- who should stay
- why travelers choose the area
- Gongdeok vs Mapo Station
- who should not stay
- seven hotels
- transport
- how to use Gongdeok in a real trip
- booking checks
- compare other Seoul areas
- final verdict
- FAQ

## 1.5 Insadong Stay

- File: `ja/where-to-stay-in-insadong.html`
- SHA: `9a2e540c7d1128997a0970a141b7bff86ada53f9`
- Title: `仁寺洞でどこに泊まる？ホテル・アパートメント比較 | Korea Inside`
- Meta: `安国、鐘閣、鍾路3街周辺から仁寺洞の宿泊先を選び、客室の広さ、家族向け寝具、アパートメント、空港到着時の動線まで確認します。`
- H1: `仁寺洞でどこに泊まる？ 2026`

Current structure:
- quick selection criteria
- Anguk / north
- central Insadong / Jonggak
- Jongno 3-ga / Ikseon-dong
- hotels
- apartment/residence with kitchen/laundry
- airport-to-hotel
- payment checks
- FAQ

---

# 2. PAGE 1 — Myeongdong Stay

## 2.1 Search Intent

Primary Japanese intent:

> **明洞に泊まるなら、明洞駅・乙支路入口・明洞中心部のどこを基準にホテルを選べばいい？**

Searchers also want:
- recommended hotels
- station proximity
- airport bus
- shopping
- family/group rooms
- Japanese-style comfort
- luggage
- late-night return

## 2.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `明洞 どこに泊まる`

Supporting:
- `明洞 ホテル おすすめ`
- `明洞駅 ホテル`
- `乙支路入口 ホテル`
- `明洞 ホテル 駅近`
- `明洞 家族 ホテル`
- `明洞 グループ ホテル`
- `明洞 空港アクセス ホテル`
- `明洞 ホテル 荷物`
- `明洞 日本語 ホテル`

The last term is a market-intent signal only. Do not add “Japanese support” facts without verification.

## 2.3 SERP Observation

Current Japanese content is becoming more sophisticated.

The strongest pages no longer treat all hotels labeled “Myeongdong” as the same location.

They split:
- Myeongdong Station
- Euljiro 1-ga
- Euljiro 3-ga
- Sogong/City Hall boundary
- Hoehyeon/Namsan side

and emphasize:
- station exit
- road crossing
- luggage
- daily return direction

This is almost exactly the current KI architecture.

## 2.4 Direct Editorial Benchmark #1 — SEOULIN

URL:
`https://jandkfoods.com/myeongdong-hotel-near-station/`

Observed title:
`明洞駅から徒歩5分以内のホテルはどこ？出口別に選ぶ`

Observed core structure:
1. do not trust “near Myeongdong Station” as one category
2. compare Myeongdong Station vs Euljiro 1-ga
3. compare specific exits
4. actual street crossings
5. luggage implications
6. verify hotel official address / nearest exit
7. hotel candidates by exit

### Strength
- excellent Japanese exact intent
- exit-specific
- current 2026 update
- official-source checking
- luggage friction

### Weakness vs KI
- intentionally narrower scope
- less family/group room logic
- less central-Myeongdong vs Sogong vs Euljiro 3-ga breadth
- less 11-hotel role comparison
- less airport/stay-family integration

## 2.5 Direct Editorial Benchmark #2 — KoriNavi

URL:
`https://korinavi.net/ja/guides/myeongdong-where-to-stay/`

Observed thesis:
> choose by the direction from which you return to the room every day, not by the Myeongdong label.

Observed area model:
1. Myeongdong Station side
2. Euljiro 1-ga side
3. Hoehyeon side
4. Namsan side

Key decision criteria:
- return direction
- suitcase route
- large roads
- pedestrian streets
- whether midday hotel returns are likely
- exact station / road crossing

### Strength
- close direct editorial competitor to KI
- real return-route logic
- area label is not enough
- avoids price-only hotel selection

### Weakness vs KI
- less hotel inventory
- less room/occupancy/group depth
- less explicit family/group comparison
- weaker airport-mode branching

## 2.6 Current Official Hotel Signal

Japanese official hotel pages confirm why exact-exit facts matter.

Examples:
- Sotetsu Fresa Inn Seoul Myeong-dong:
  - Myeongdong Station Exit 8: about 5 minutes
  - Euljiro 1-ga Exit 5: about 5 minutes
- Solaria Nishitetsu Seoul Myeongdong:
  - Myeongdong Station Exit 8: about 3 minutes
  - Euljiro 1-ga Exit 5: about 7 minutes

These are hotel-specific facts and should stay attached to verified hotel entries rather than broad area assumptions.

## 2.7 Korea Inside Actual Structure

KI already:
- splits four practical subareas
- provides an 11-hotel table
- differentiates simple sleeping base vs family/group stay
- handles airport access separately
- asks what route the traveler repeatedly uses
- links area choice to hotel choice

This is highly competitive.

## 2.8 1:1 SEO GAP

| Element | Competitors | Korea Inside | Judgment | Priority |
|---|---|---|---|---|
| Title | station/exit or “where to stay” | exact broad stay query | PASS |
| Meta | exit / station / hotels | station + luggage + airport + group | KI stronger |
| H1 | direct | direct | PASS |
| Intro / Quick Answer | station-side quick answer | first asks area | strong |
| H2 | exit/subarea | subarea + hotel + traveler type | KI broader |
| H3 | hotel entities | 11 verified candidates | strong |
| Keyword language | `ホテルおすすめ / 駅近` | natural `どこに泊まる` | strong |
| Intent | hotel + location | exact | PASS |
| Depth | narrow exit/location | full stay decision | KI stronger |
| Practical info | exit/road | luggage/airport/family/route | KI stronger |
| Decision support | high | exceptional | KEEP |
| Freshness | hotel facts | requires maintenance | maintenance |
| Entities | selected hotels | 11 | strong |
| Internal links | general Stay | full KI area/stay cluster | KI advantage |
| FAQ | variable | 5 | useful |
| CTA | hotel booking | contextual | strong |
| Media/map | station/exit | could help, not critical | P2 |
| Trust/source | official hotel pages | can verify per hotel | strong |
| Commercial usefulness | high | high | PASS |

## 2.9 KEEP

- current title/meta/H1
- Myeongdong Station side
- central Myeongdong
- Euljiro 1-ga/Sogong
- Euljiro 3-ga
- 11-hotel table
- family/group use
- airport access
- repeated-route logic
- FAQ

## 2.10 P0 / P1 / P2

### P0
None.

### P1
No structural change required.

Maintenance:
- exit
- airport bus stops
- room type/occupancy
- hotel operational changes

### P2
Potential subarea orientation visual if current text is not enough.

## 2.11 SEO Surface Gap

No urgent rewrite.

Current:
`明洞でどこに泊まる？おすすめエリアとホテル比較`

already bridges area + hotel intent very well.

## 2.12 Content Gap

No major body gap.

Potential future verification:
- exact elevator/escalator exits where luggage is a major decision
- selected room-size/occupancy facts

## 2.13 Japan-Specific Market Gap

Japanese SERP repeatedly pushes:
- 日本語対応
- バスタブ
- 日系ホテル
- 駅近

These are valid user concerns but hotel-specific.

Do not add them globally without current official verification.

## 2.14 Competitor-Only Content

- Japanese-support ranking
- live hotel prices
- “best” ranking
- current coupon offers

Not core.

## 2.15 New Page Candidate

None required.

The current Myeongdong Stay page already covers the representative intent.

## 2.16 GEO / AI Search

High-value extractable answers:
- Myeongdong Station vs Euljiro 1-ga
- shopping-heavy stay
- central sightseeing
- family/group
- airport transfer
- luggage
- exact route used repeatedly

## 2.17 Internal Links

Outbound:
- Myeongdong Travel Guide
- Accommodation
- Hongdae vs Myeongdong
- First-Time
- Shopping
- K-Beauty
- Airport Bus / Airport Access

Inbound:
- Myeongdong Travel Guide
- Accommodation
- First-Time
- Shopping
- Hongdae vs Myeongdong
- K-Beauty

## 2.18 Final Primary Keyword

**`明洞 どこに泊まる`**

## 2.19 Final Status

# **PASS**

---

# 3. PAGE 2 — Hongdae Stay

## 3.1 Search Intent

Primary Japanese intent:

> **弘大に泊まるなら、弘大入口駅の近く・延南洞・合井のどこが、自分の夜・空港・静けさ・荷物に合う？**

This is a real subarea stay decision.

## 3.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `弘大 どこに泊まる`

Supporting:
- `弘大 ホテル おすすめ`
- `弘大入口駅 ホテル`
- `弘大 延南洞 ホテル`
- `弘大 合井 ホテル`
- `弘大 ホテル AREX`
- `弘大 静か ホテル`
- `弘大 ナイトライフ ホテル`
- `弘大 家族 ホテル`
- `弘大 グループ ホテル`

## 3.3 SERP Observation

Recent Japanese competitor pages explicitly distinguish:
- Hongik University Station
- Yeonnam
- Hapjeong

The main trade-off:
- station/AREX convenience
- atmosphere
- quietness
- nightlife
- final walking distance

This validates the exact KI structure.

## 3.4 Direct Editorial Benchmark #1 — limkyo998

URL:
`https://note.com/limch998_kyo/n/n8358247aea63`

Observed title:
`弘大に泊まるのはアリ？延南洞・合井と比べた正直な話〖2026年版〗`

Observed structure:
1. quick conclusion
2. Hongdae is actually three different stay environments
3. Hongik University Station:
   - convenience
   - nightlife
   - noise
4. Yeonnam:
   - cafes/walking
   - calmer evening
   - station distance
5. Hapjeong:
   - wider routes
   - Han River / Line 6
   - quieter than central Hongdae
6. who each suits

### Strength
- exact subarea search intent
- direct Japanese wording
- nightlife/noise trade-off
- not just a hotel list

### Weakness vs KI
- less exact luggage/final-walk structure
- less group-room/hotel inventory
- less airport-to-hotel distinction
- less room/occupancy comparison

## 3.5 Direct Editorial Benchmark #2 — KoriNavi

URL:
`https://krinavi.com/ja/guides/hongdae-stay-guide/`

Observed thesis:
> Hongdae Station, Yeonnam and Hapjeong should be divided by what you are willing to give up.

Subarea logic:
- station/commercial core:
  - airport and late food convenience
  - trade some quiet
- Yeonnam:
  - walking/cafes/slower night
  - trade station-door immediacy
- Hapjeong:
  - Mangwon/Han River/Line 6
  - trade simple “AREX next to hotel” logic

### Strength
- decision-first
- real trade-offs
- luggage / rain / road crossing
- strong current framework

### Weakness vs KI
- fewer hotels
- less 5–6 person room logic
- less hotel-category segmentation
- weaker exact hotel-room comparisons

## 3.6 Korea Inside Actual Structure

KI already:
- explains Hongdae geography
- says Hongdae is not one block
- explains why map-distance changes with luggage
- separates airport access from final walk
- explains center-Seoul travel trade-off
- compares hotel by trip purpose
- has 5–6 person stay logic
- groups hotels by:
  - Hongdae atmosphere
  - airport/AREX
  - cost
  - quieter street
  - guesthouse
- includes 14 lodging entities
- FAQ

## 3.7 1:1 SEO GAP

| Element | Competitors | Korea Inside | Judgment | Priority |
|---|---|---|---|---|
| Title | `弘大に泊まる / 延南 / 合井` | `どこに泊まる / エリア・ホテル・立地` | excellent | PASS |
| Meta | subarea/noise | subarea + airport + luggage + nightlife | KI stronger |
| H1 | stay/subarea | exact | PASS |
| Intro / Quick Answer | 3 subareas | geography first | strong |
| H2 | subareas | stronger functional grouping | KI stronger |
| H3 | limited hotel entities | 14 hotel/lodging candidates | KI stronger |
| Keyword language | direct | direct | PASS |
| Intent | exact | exact | PASS |
| Depth | area trade-off | area + hotel + group + airport | KI stronger |
| Practical info | noise/walk | luggage/final walk/airport | KI stronger |
| Decision support | high | exceptional | KEEP |
| Freshness | hotel details | maintenance needed | maintenance |
| Entities | moderate | high | strong |
| Internal links | limited | KI full cluster | advantage |
| FAQ | some | 5 | sufficient |
| CTA | hotel | contextual | strong |
| Media/map | useful | optional | P2 |
| Trust/source | editorial | hotel facts can be verified | strong |
| Commercial usefulness | high | high | PASS |

## 3.8 KEEP

- current title/meta/H1
- Hongdae is not one block
- station/Yeonnam/Hapjeong differences
- luggage changes perceived distance
- airport access ≠ hotel-door access
- center-Seoul trade-off
- group stay logic
- quiet vs nightlife hotel categories
- guesthouses
- 14 lodging candidates
- FAQ

## 3.9 P0 / P1 / P2

### P0
None.

### P1
No structural change.

Maintenance:
- exact station exits
- operational status
- room capacity
- airport/AREX facts

### P2
Possible visual subarea map if not already sufficiently clear.

## 3.10 SEO Surface Gap

No urgent rewrite.

The current title is already better than most generic `弘大おすすめホテル` list pages because it signals:
- area
- hotel
- location

## 3.11 Content Gap

No major gap.

Potential future:
- exact late-night noise/street block facts only if verifiable and maintainable.

## 3.12 Japan-Specific Market Gap

Japanese users repeatedly search:
- quiet
- AREX
- airport
- nightlife
- station
- Yeonnam

KI already addresses the underlying decisions.

## 3.13 Competitor-Only Content

- live prices
- rank order
- “Japanese support” labels
- nightlife popularity ranking

Not required.

## 3.14 New Page Candidate

None required.

Current page is a strong representative URL.

## 3.15 GEO / AI Search

Extractable answers:
- Hongik University Station vs Yeonnam vs Hapjeong
- airport access
- nightlife
- quiet nights
- family/group
- 5–6 adults
- map distance vs luggage reality

## 3.16 Internal Links

Outbound:
- Hongdae Travel Guide
- Accommodation
- Hongdae vs Myeongdong
- Nightlife
- Solo
- Couples
- Airport Access / AREX

Inbound:
- Hongdae Travel Guide
- Accommodation
- Hongdae vs Myeongdong
- First-Time
- Nightlife
- Solo/Couples

## 3.17 Final Primary Keyword

**`弘大 どこに泊まる`**

## 3.18 Final Status

# **PASS**

---

# 4. PAGE 3 — Seoul Station Hotels

## 4.1 Search Intent

Primary Japanese intent:

> **KTX・AREX・早朝列車・大きな荷物を使う旅行なら、ソウル駅周辺のどのホテルが本当に便利？**

This is a transport-purpose hotel page.

It should not pretend Seoul Station is the best base for everyone.

## 4.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `ソウル駅 ホテル`

Supporting:
- `ソウル駅 周辺 ホテル`
- `ソウル駅 ホテル おすすめ`
- `KTX ソウル駅 ホテル`
- `AREX ソウル駅 ホテル`
- `ソウル駅 早朝 KTX ホテル`
- `ソウル駅 荷物 ホテル`
- `ソウル駅 家族 ホテル`
- `ソウル駅 明洞 ホテル 比較`

## 4.3 SERP Observation

The Japanese market has many generic “20 best hotels” pages.

The stronger recent content increasingly emphasizes:
- platform-to-hotel route
- station side
- underground passages
- hills
- luggage
- early KTX
- airport train
- whether you actually use Seoul Station repeatedly

This exactly supports the KI page role.

## 4.4 Direct Editorial Benchmark #1 — TravelBook

URL:
`https://www.travelbook.co.jp/topic/54728`

Observed title:
`ソウル駅周辺のおすすめホテル20選：「失敗しない」神宿＆エリア別攻略`

Observed structure:
- Seoul Station as transport hub
- AREX / KTX
- station complexity
- hills
- weather
- station-connected / nearby hotels
- Namdaemun / Myeongdong-side hotels
- Japanese-style comfort
- hotel ranking
- room / bathroom features
- OTA booking

### Strength
- exact high-volume hotel intent
- many hotel entities
- luggage/station complexity visible
- KTX/AREX obvious

### Weakness / Risks vs KI
- hyperbolic “best / absolute winner” framing
- many Japanese-support/facility claims require hotel-level verification
- live hotel inventory dominates
- “Seoul Station is smartest base” can be overgeneralized
- less disciplined “who should not stay here?” logic

## 4.5 Direct Editorial Benchmark #2 — KRINAVI / Four Points Seoul Station

URL:
`https://krinavi.com/ja/guides/four-points-josun-seoul-station-korea-guide/`

Observed title:
`Four Pointsソウル駅は泊まる価値ある？KTX・AREXと部屋の狭さを判断`

Observed structure:
1. 30-second decision
2. use Seoul Station repeatedly?
3. underground connection still includes walking
4. large luggage / room-size check
5. room direction/noise
6. if Seoul Station is used once, area value drops
7. official hotel access
8. exact room/booking conditions

### Strength
- real journey fit
- platform-to-hotel interpretation
- room-size trade-off
- official hotel evidence
- avoids station-name simplification

### Weakness vs KI
- single-hotel page
- not a broad eight-hotel area comparison
- less full area segmentation

## 4.6 Supporting Benchmark — Seoul Window

General Seoul Stay comparison identifies Seoul Station as strong when:
- AREX
- KTX
- regional travel
matter, but weaker as a universal sightseeing base.

This supports KI’s “not for everyone” section.

## 4.7 Korea Inside Actual Structure

Current KI:
- whether to stay around Seoul Station
- when it truly fits
- when another area is better
- what “near Seoul Station” means
- subareas:
  - station immediate
  - west / Exit 15 side
  - Seoullo / Namdaemun side
  - City Hall boundary
- eight hotels
- KTX platform reality
- AREX Express vs all-stop distinction
- City Airport Terminal potential value
- actual itinerary use
- booking checks
- final judgment
- 6 FAQ

This is structurally excellent.

## 4.8 1:1 SEO GAP

| Element | Competitors | Korea Inside | Judgment | Priority |
|---|---|---|---|---|
| Title | `ソウル駅ホテルおすすめ` | `KTX・AREX利用に便利` | highly specific and strong | PASS |
| Meta | hotel list | KTX/AREX/family/business | strong |
| H1 | station hotels | exact | PASS |
| Intro / Quick Answer | station is convenient | asks whether you should stay | KI stronger |
| H2 | hotel ranking | fit / subarea / travel use | KI stronger |
| H3 | hotel entities | 8 + transport reality | strong |
| Keyword language | `おすすめ` | purpose-specific | strong |
| Intent | hotel list | transport-purpose hotel decision | differentiated |
| Depth | many hotels | decision depth | KI stronger |
| Practical info | access/room | platform/station side/transfer | KI stronger |
| Decision support | medium-high | exceptional | KEEP |
| Freshness | hotels/transport | maintenance required | maintenance |
| Entities | 20 hotels | 8 curated | appropriate |
| Internal links | booking-heavy | airport/stay/transport cluster | KI advantage |
| FAQ | variable | 6 | strong |
| CTA | booking | contextual | strong |
| Media/map | could help | optional | P2 |
| Trust/source | varied | can verify official access | strong |
| Commercial usefulness | high | high | PASS |

## 4.9 KEEP

- current title/meta/H1
- not for everyone
- “near station” is not one location
- four subareas
- 8 hotels
- KTX reality
- AREX two train types
- early train / regional travel
- room and luggage checks
- final verdict
- FAQ

## 4.10 P0 / P1 / P2

### P0
None.

### P1
No structural change required.

Maintenance:
- City Airport Terminal conditions
- KTX/AREX current operations
- exact hotel access
- hotel room/facility conditions

### P2
Potential station-side route graphic for luggage.

## 4.11 SEO Surface Gap

No urgent rewrite.

Current title deliberately qualifies **why** Seoul Station hotels matter:
`KTX・AREX利用に便利`.

That is more useful than generic `おすすめ20選`.

## 4.12 Content Gap

No major gap.

A hotel count greater than eight is not inherently useful.

## 4.13 Japan-Specific Market Gap

Japanese SERP frequently values:
- Japanese hotel chains
- bathtub
- Japanese support
- washlet
- direct connection

These should only be added as hotel-specific verified facts.

## 4.14 Competitor-Only Content

- 20-hotel inventory
- live prices
- unverified Japanese-support strength
- “best hotel” superlatives

Not required.

## 4.15 New Page Candidate

No urgent candidate.

Potential hotel-level detail:
- Four Points by Sheraton Josun Seoul Station
only if independent demand and hotel-detail strategy justify it.

## 4.16 GEO / AI Search

Extractable answers:
- who should stay near Seoul Station
- who should not
- early KTX
- AREX
- large luggage
- station side
- Four Points / Hotel Manu / Gracery role differences
- “near station” actual meaning

## 4.17 Internal Links

Outbound:
- Accommodation
- Airport Access
- AREX
- First-Time
- Family
- Budget
- Myeongdong / Namdaemun related area pages if appropriate

Inbound:
- Accommodation
- Airport Access
- First-Time
- Family
- Budget
- Airport/AREX pages

## 4.18 Final Primary Keyword

**`ソウル駅 ホテル`**

## 4.19 Final Status

# **PASS**

---

# 5. PAGE 4 — Gongdeok Station Hotels

## 5.1 Search Intent

Primary Japanese intent:

> **孔徳駅に泊まるとAREXと複数路線は便利そうだが、弘大や明洞ではなく孔徳を選ぶだけの価値がある？どのホテルが条件に合う？**

This is less about “best hotel ranking” and more about whether the transport hub actually matches the trip.

## 5.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `孔徳 ホテル`

Supporting:
- `孔徳駅 ホテル`
- `孔徳 ホテル おすすめ`
- `孔徳 AREX ホテル`
- `麻浦 ホテル`
- `孔徳 麻浦 ホテル`
- `孔徳 家族 ホテル`
- `孔徳 長期滞在 ホテル`
- `孔徳 弘大 どっち`
- `孔徳 空港アクセス`

## 5.3 SERP Observation

Current Japanese search content around Gongdeok/Mapo is split across:
- area comparison
- hotel lists
- specific Japanese-brand hotel pages
- AREX access
- Gongdeok vs Mapo Station

The strongest practical differentiator is **exact station/exit/elevator route**.

## 5.4 Direct Editorial Benchmark #1 — Seoul Window

URL:
`https://www.seoulwindow.com/ja/guides/seoul-hotels-yeouido-mapo/`

Observed title:
`汝矣島・麻浦・孔徳・新村ホテル比較：空港鉄道で乗り換え0回のエリアと漢江公園までの徒歩時間`

Observed structure:
- exact station-to-hotel distances
- current displayed prices
- Gongdeok / Mapo / Yeouido / Sinchon comparison
- airport access
- subway line differences
- hotel table
- walking distance
- uncertainty marked when values cannot be verified

### Strength
- quantitative
- exact walking distances
- strong current access data
- clear station differences
- no need to infer missing values

### Weakness vs KI
- price table is volatile
- wider regional comparison dilutes detailed hotel-use cases
- less family/long-stay/room-condition reasoning

## 5.5 Direct Editorial Benchmark #2 — SEOULIN

URL:
`https://jandkfoods.com/mapo-gongdeok-area-guide/`

Observed title:
`孔徳（コンドク）・麻浦は泊まる価値ある？4路線駅を検証`

Observed key points:
- Gongdeok is a four-line hub
- AREX Express does **not** stop; all-stop train does
- tourists often see airport access but do not know the area
- evaluates whether transport convenience is actually worth staying for
- distinguishes Gongdeok and Mapo roles

### Strength
- exact “is it worth staying?” intent
- correct AREX distinction
- current update
- strong place-role explanation

### Weakness vs KI
- area-guide level, not seven-hotel comparison
- less room/facility/luggage detail
- less exact hotel-by-purpose segmentation

## 5.6 Current Official Hotel Signal — Roynet Hotel Seoul Mapo

Official Japanese page currently states:
- Gongdeok and Mapo stations: about 4 minutes
- Incheon Airport → Gongdeok by AREX all-stop: about 50 minutes
- Gimpo → Gongdeok: about 20 minutes
- nearest Gongdeok Exit 1 involves stairs
- Exit 9 is recommended if elevator access is needed
- Mapo Exit 2 has escalator access
- rooms are 23㎡ or larger

This is exactly the type of hotel-specific practical evidence that can strengthen KI.

## 5.7 Korea Inside Actual Structure

Current KI:
- what changes by staying in Gongdeok
- who should / should not stay
- reasons travelers stay
- Gongdeok vs Mapo Station
- seven hotel candidates:
  - GLAD Mapo
  - LOTTE City Hotel Mapo
  - Roynet Hotel Seoul Mapo
  - Shilla Stay Mapo
  - Seoul Garden Hotel
  - Hotel Naru Seoul MGallery Ambassador
  - Gongdeok Stay Masil
- AREX all-stop distinction
- four-line value only when actually used
- Gongdeok Station is large
- Mapo Station is simpler
- real trip use
- booking checks
- other Seoul areas
- final verdict
- FAQ

## 5.8 1:1 SEO GAP

| Element | Competitors | Korea Inside | Judgment | Priority |
|---|---|---|---|---|
| Title | Gongdeok/Mapo hotels / worth staying | exact hotel comparison | strong |
| Meta | access/price | AREX/business/long stay/family | strong |
| H1 | hotel/area | exact | PASS |
| Intro / Quick Answer | is it worth staying? | what changes | strong |
| H2 | station differences | same + hotel selection | KI stronger |
| H3 | walking distances | seven hotels + transport facts | strong |
| Keyword language | `ホテル / AREX` | direct | PASS |
| Intent | area/hotel/access | exact | PASS |
| Depth | quantitative | decision + hotel roles | KI stronger |
| Practical info | walking/price | transport/area/hotel | exact exit could improve | **P1** |
| Decision support | high | exceptional | KEEP |
| Freshness | price/access | maintenance needed | maintenance |
| Entities | hotel table | seven | appropriate |
| Internal links | regional | KI full cluster | advantage |
| FAQ | variable | 5 | sufficient |
| CTA | booking | contextual | strong |
| Media/map | useful | optional | P2 |
| Trust/source | quantitative/current | can strengthen official hotel facts | P1 |
| Commercial usefulness | high | high | strong |

## 5.9 KEEP

- current title/meta/H1
- why Gongdeok matters
- Gongdeok vs Mapo
- AREX all-stop only
- four-line utility
- “large station” friction
- “Mapo is simpler” logic
- seven hotels
- family/business/long stay distinctions
- who should not stay
- FAQ

## 5.10 P0 / P1 / P2

### P0
None.

### P1
Strengthen hotel-specific practical facts where officially verifiable:
- exit
- elevator/escalator
- actual final walk
- room size / bathroom where decision-relevant

Especially useful for:
- Roynet
- GLAD
- Lotte City
- Seoul Garden / Mapo-side candidates

Also make the station-size/final-walk consequence more visible in the top answer.

### P2
Station-area mini map:
- Gongdeok
- Mapo
- hotel positions
- elevator-friendly exits

## 5.11 SEO Surface Gap

No urgent title/H1 rewrite.

Potential top-answer emphasis:
> Gongdeok is strongest when airport access and multiple subway directions matter repeatedly; Mapo can be simpler when the trip mainly needs Line 5 / river-side access.

## 5.12 Content Gap

Not missing hotel count.

High-value gap = **verified micro-friction**:
- stairs
- elevator exit
- road crossing
- exact walk
- room size for luggage

## 5.13 Japan-Specific Market Gap

Japanese users place high value on:
- elevator
- bathtub
- separate bath/toilet
- Japanese-brand hotel
- airport access

Only current hotel-specific facts should be added.

## 5.14 Competitor-Only Content

- live hotel prices
- broad “best cost performance” rankings
- Japanese-language-service claims

Not core without verification.

## 5.15 New Page Candidate

None required.

Potential future:
- Gongdeok vs Hongdae Stay comparison
if independent search demand is demonstrated.

## 5.16 GEO / AI Search

Extractable answers:
- why stay in Gongdeok
- Gongdeok vs Mapo
- AREX type
- family / business / long stay
- who should skip it
- elevator/final walk
- exact hotel roles

## 5.17 Internal Links

Outbound:
- Gongdeok/Mapo Travel Guide
- Accommodation
- Airport Access
- AREX
- First-Time
- Budget
- Families
- Hongdae vs Myeongdong / Hongdae stay when comparison is natural

Inbound:
- Gongdeok/Mapo Travel Guide
- Accommodation
- Airport Access
- Budget
- Families
- AREX

## 5.18 Final Primary Keyword

**`孔徳 ホテル`**

## 5.19 Final Status

# **P1**

---

# 6. PAGE 5 — Insadong Stay

## 6.1 Search Intent

Primary Japanese intent:

> **仁寺洞・安国・鍾路3街のどこに泊まれば、古宮・北村・益善洞・夜の帰路・荷物が扱いやすい？ホテルとアパートメントはどちらがいい？**

Additional Japanese search concerns:
- Japanese support
- bathtub
- family room
- residence
- hanok guesthouse
- quiet stay
- airport bus
- traditional atmosphere

## 6.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `仁寺洞 どこに泊まる`

Supporting:
- `仁寺洞 ホテル おすすめ`
- `仁寺洞 ホテル`
- `安国 ホテル`
- `鍾路3街 ホテル`
- `仁寺洞 家族 ホテル`
- `仁寺洞 レジデンス`
- `仁寺洞 アパートメント`
- `仁寺洞 韓屋 宿泊`
- `仁寺洞 空港アクセス`
- `仁寺洞 日本語 ホテル`

Again, `日本語` is a demand signal, not a fact to add automatically.

## 6.3 SERP Observation

The Japanese Insadong/Jongno hotel field is broader than the current “hotel/apartment” frame.

The recurring models are:
1. modern city hotels near palaces/Insadong
2. Jongno 3-ga lower-cost hotels
3. residence/apartment
4. hanok guesthouse/traditional stay
5. Japanese-support/bathtub-oriented hotel lists

Korea Inside already covers 1–3 strongly.

The meaningful open question is whether **hanok stay** should be:
- added as a conditional subsection
or
- handled as a separate page/entity cluster.

## 6.4 Direct Editorial Benchmark #1 — KRINAVI

URL:
`https://krinavi.com/ja/guides/jongno-stay-guide/`

Observed title:
`鍾路・仁寺洞のホテルはどこに泊まる？「守りたい朝」と3つの夜で選ぶ`

Observed thesis:
- palace/Bukchon/Insadong mornings can justify Jongno
- three actual evening return directions must also work
- broad “Jongno/Insadong” booking results include materially different addresses
- choose by morning value and night return

### Strength
- strong behavior-based stay logic
- morning vs night
- subarea nuance
- avoids generic historic-atmosphere recommendation

### Weakness vs KI
- broader Jongno scope
- less explicit apartment/kitchen/laundry
- less family room/bed decision depth
- fewer hotel entities

## 6.5 Direct Editorial Benchmark #2 — 今ここは (Now Yeogi)

URL:
`https://nowyeogi.com/ja/posts/jongno-hotels-2026-09/`

Observed title:
`鍾路の宿：韓屋ゲストハウスとホテルを予算別に (2026)`

Observed structure:
1. palace-side modern hotel vs 100-year hanok room
2. choose side:
   - Gwanghwamun
   - Insadong/Anguk
   - Jongno 3-ga
   - Bukchon/Samcheong
3. hotel vs hanok
4. budget ranges
5. walking / hill
6. stay-style differences

### Strength
- unique hanok/hotel decision
- price and area clarity
- good historical-stay intent
- strong subarea differentiation

### Weakness / Risk vs KI
- price bands are volatile
- broader Jongno than the KI Insadong page
- hanok operational conditions vary substantially
- less apartment/family-long-stay detail

## 6.6 Supporting Benchmark — TravelBook / Hotel Rankings

Current Japanese hotel-list pages emphasize:
- Japanese support
- bathtub
- Jongno 3-ga / Anguk distance
- “quiet/safe” claims
- 20-hotel rankings

These are commercially useful but fact-heavy and volatile.

## 6.7 Korea Inside Actual Structure

Current KI:
- quick selection criteria
- three practical subareas:
  - Anguk / north
  - central Insadong / Jonggak
  - Jongno 3-ga / Ikseon-dong
- hotel list:
  - Nine Tree by Parnas Seoul Insadong
  - Hotel Sunbee Insadong
  - AMID Hotel Seoul
  - Dormy Inn EXPRESS Seoul Insadong
  - Moxy Seoul Insadong
- apartment/residence:
  - Orakai Insadong Suites
  - Somerset Palace Seoul
- airport-to-hotel route
- payment checks
- FAQ on:
  - four adults
  - one-week apartment stay
  - “near station” with luggage
  - late check-in

This is practical and distinct.

## 6.8 1:1 SEO GAP

| Element | Competitors | Korea Inside | Judgment | Priority |
|---|---|---|---|---|
| Title | hotel / Jongno / hanok | hotel + apartment | strong but narrower | P1 |
| Meta | area/hotel | subarea + room + family + airport | strong |
| H1 | hotel/stay | `仁寺洞でどこに泊まる` | strong |
| Intro / Quick Answer | morning/evening or hanok/hotel | selection criteria | good |
| H2 | subareas / hanok | subareas / hotel / apartment | possible hanok gap | **P1** |
| H3 | hotel/hanok entities | 7 hotels/residences | good |
| Keyword language | `ホテル / 韓屋` | `ホテル / アパートメント` | add only if role supports | P1 |
| Intent | historic stay | hotel/apartment decision | aligned |
| Depth | broad stay types | stronger practical room/stay conditions | complementary |
| Practical info | price/area | family/room/airport | KI strong |
| Decision support | high | strong | KEEP |
| Freshness | price/operation | maintenance needed | maintenance |
| Entities | many hotels/hanok | selected 7 | reasonable |
| Internal links | hotel booking | KI cluster | advantage |
| FAQ | variable | 4 | could be richer but not required | P2 |
| CTA | booking | contextual | strong |
| Media/map | helpful | optional | P2 |
| Trust/source | hotel-list claims vary | current facts can be verified | strong |
| Commercial usefulness | high | high | P1 |

## 6.9 KEEP

- current title/meta/H1
- Anguk / central Insadong / Jongno 3-ga split
- hotels vs apartment/residence
- family/room-size logic
- kitchen/laundry
- airport route
- payment checks
- current hotel entities
- FAQ

## 6.10 P0 / P1 / P2

### P0
None.

### P1
Evaluate whether a conditional **hanok stay** subsection is justified.

If included:
- explain who it suits
- hills/luggage
- bathroom/floor bedding
- check-in
- privacy
- exact property facts
- do not romanticize “traditional stay”

Also strengthen:
- exact station/exit/final walk
- verified hotel room/family configurations

### P2
- FAQ expansion only if real search questions justify it
- historic-stay orientation map

## 6.11 SEO Surface Recommendation

Current title/H1 can remain.

Possible supporting wording in lead/H2:
`ホテル・レジデンス・韓屋をどう使い分ける？`

Only if source-level expansion is approved.

Do not change the page title to promise hanok unless real inventory and facts are added.

## 6.12 Content Gap

Most meaningful possible gap:
**traditional/hanok stay choice**

But this should not be inserted simply because competitors rank for it.

First check:
- enough verified properties?
- room/bathroom/luggage conditions?
- page-role fit?
- whether a separate Detail is cleaner?

## 6.13 Japan-Specific Market Gap

Japanese travelers often value:
- bathtub
- Japanese support
- traditional atmosphere
- hanok
- family room
- kitchen/laundry

KI already handles room/family/apartment better than typical listicles.

Bathtub/Japanese support/hanok require property-level fact validation.

## 6.14 Competitor-Only Content

- Japanese-support hotel ranking
- daily prices
- generic safety claims
- “bathtub guaranteed” generalizations
- long 20-hotel list

Not core without verification.

## 6.15 New Page Candidate

Potential:
**`ソウル 韓屋 宿泊` / `仁寺洞・北村 韓屋ステイ`**

This may be better than bloating the current hotel/apartment page if independent Japanese demand is strong.

## 6.16 GEO / AI Search

Extractable answers:
- Anguk vs Jonggak vs Jongno 3-ga
- family/group
- apartment for week-long stay
- kitchen/laundry
- airport arrival
- station-near does not always mean luggage-easy
- hotel vs possible hanok stay

## 6.17 Internal Links

Outbound:
- Insadong Travel Guide
- Accommodation
- First-Time
- Families
- Couples
- nearby historic-area guides
- Airport Bus / Airport Access where relevant

Inbound:
- Insadong Travel Guide
- Accommodation
- First-Time
- Families
- Couples
- historic-area cluster

## 6.18 Final Primary Keyword

**`仁寺洞 どこに泊まる`**

## 6.19 Final Status

# **P1**

---

# 7. Cross-Page Findings

## 7.1 Area Stay Pages Are Becoming More Competitive Than Generic Hotel Lists

The strongest new Japanese pages increasingly use:
- station side
- exit
- route
- luggage
- exact hotel entrance
- repeated daily use

This validates KI’s long-standing Humanization direction.

## 7.2 “Near the Station” Is Not a Sufficient Hotel Fact

Batch 6 confirms a recurring rule:

> **station name → exit → elevator/stairs → road crossing → hotel entrance**

must be treated as one complete access question.

This matters especially in:
- Myeongdong
- Seoul Station
- Gongdeok
- Insadong

## 7.3 Room Conditions Become More Important at Detail Level

Once the user has chosen the area, SEO usefulness shifts from generic attractions toward:
- occupancy
- bed configuration
- room size
- bathroom
- luggage
- laundry
- late check-in
- exact booking plan

This is where KI should continue to differentiate from listicles.

## 7.4 Japanese-Market Comfort Signals Must Stay Property-Specific

Competitors repeatedly use:
- 日本語対応
- 日系
- バスタブ
- 洗い場
- ウォシュレット
- 日本式コンセント

These are real Japanese concerns.

But they are not safe as broad area claims.

Use only when:
1. current official/property source confirms
2. exact room/category condition is known
3. current fact layer can be maintained

## 7.5 Hotel Count Is Not a Quality Metric

A 20-hotel page can be weaker than a 7-hotel page if:
- roles overlap
- differences are not explained
- room facts are stale
- only price/rating separates them

KI should keep curated lists.

## 7.6 Exact Official Access Data Is High-Value Content

Official hotel pages can provide:
- exact exit
- stairs
- elevator
- escalator
- actual minutes
- airport route

These are especially useful because they directly reduce travel friction.

---

# 8. Japan SEO Restructuring Rules — Batch 6 Additions

## Rule 31 — Area Stay Detail Should Answer “Where Inside the Area?”

Area Guide:
> Why visit?

Area Stay Detail:
> Which side/station/block should I sleep in?

Hotel:
> Which exact room/product?

Do not merge these roles.

## Rule 32 — Every “Station-Near” Claim Should Be Read as a Route

`駅徒歩3分` alone is insufficient.

Check:
- exit
- stairs/elevator
- crossing
- hill
- indoor/outdoor
- luggage condition

## Rule 33 — Japanese Comfort Features Are Hotel-Level Data

Do not apply:
- Japanese support
- bathtub
- washlet
- Japanese outlets
to the whole area.

## Rule 34 — Curated Hotel Sets Are Better Than Unmaintainable Rankings

Use enough properties to represent:
- trip type
- budget
- group size
- transport
- room type

not arbitrary Top 20 counts.

## Rule 35 — Traditional Stay Is a Separate Product Type

Hanok should not be inserted as decoration.

If added:
- explain practical trade-offs
- verify property conditions
- consider a separate page if intent is large enough

---

# 9. P0 Action List

**None in Batch 6.**

No page needs an urgent representative-query or page-role rewrite.

---

# 10. P1 Action List

## Gongdeok Station Hotels
- exact station exit/elevator/final-walk facts
- official hotel evidence
- sharpen Gongdeok vs Mapo practical difference

## Insadong Stay
- evaluate hanok stay as a conditional option
- strengthen exact station/final-walk facts
- verify family/room/property details

## Cross-page
- maintain hotel facts
- use official access pages where possible
- keep Japanese comfort facts property-specific

---

# 11. P2 Action List

- Myeongdong subarea map
- Hongdae subarea map
- Seoul Station platform-to-hotel route visual
- Gongdeok/Mapo hotel-location map
- Insadong historic-stay map
- hanok detail page research

---

# 12. New Page Backlog

| Candidate | Why it surfaced | Gate |
|---|---|---|
| ソウル 韓屋 宿泊 | distinct Japanese historical-stay intent | verified property depth |
| 仁寺洞・北村 韓屋ステイ | clear local intent | scope/cannibalization |
| Gongdeok vs Hongdae Stay | airport/access choice can be independent | demand validation |
| Four Points Seoul Station hotel detail | entity-level demand | hotel-detail strategy |

No candidate is approved for production.

---

# 13. Final Recommendation

## Myeongdong Stay
**PASS**

Protect current four-subarea + 11-hotel structure.

## Hongdae Stay
**PASS**

Protect the Hongdae Station / Yeonnam / Hapjeong trade-off architecture.

## Seoul Station Hotels
**PASS**

Keep KTX/AREX purpose-first positioning and “not for everyone” judgment.

## Gongdeok Station Hotels
**P1**

Add verified micro-friction: exits, elevators, actual walk, room/luggage practicality.

## Insadong Stay
**P1**

Current hotel/apartment structure is strong. Evaluate hanok as a real stay product, not decorative culture copy.

---

# 14. Source Register

## Korea Inside current Japanese source
- `ja/where-to-stay-in-myeongdong.html`
- `ja/where-to-stay-in-hongdae.html`
- `ja/hotels-near-seoul-station.html`
- `ja/hotels-near-gongdeok-station.html`
- `ja/where-to-stay-in-insadong.html`

## Myeongdong benchmarks
- SEOULIN  
  https://jandkfoods.com/myeongdong-hotel-near-station/
- KoriNavi  
  https://korinavi.net/ja/guides/myeongdong-where-to-stay/
- Sotetsu Fresa Inn Seoul Myeong-dong official  
  https://www.sotetsu-hotels.com/fresa-inn/myeong-dong/
- Solaria Nishitetsu Hotel Seoul Myeongdong official  
  https://solaria-seoul.nnr-h.com/

## Hongdae benchmarks
- limkyo998  
  https://note.com/limch998_kyo/n/n8358247aea63
- KoriNavi  
  https://krinavi.com/ja/guides/hongdae-stay-guide/

## Seoul Station benchmarks
- TravelBook  
  https://www.travelbook.co.jp/topic/54728
- KoriNavi / Four Points Seoul Station  
  https://krinavi.com/ja/guides/four-points-josun-seoul-station-korea-guide/
- Seoul Window general area comparison  
  https://www.seoulwindow.com/ja/guides/seoul-hotels-by-neighborhood/

## Gongdeok / Mapo benchmarks
- Seoul Window  
  https://www.seoulwindow.com/ja/guides/seoul-hotels-yeouido-mapo/
- SEOULIN  
  https://jandkfoods.com/mapo-gongdeok-area-guide/
- Roynet Hotel Seoul Mapo official  
  https://www.daiwaroynet.jp/seoulmapo/
- KoriNavi / GLAD Mapo  
  https://krinavi.com/ja/guides/glad-mapo-korea-guide/

## Insadong / Jongno benchmarks
- KoriNavi  
  https://krinavi.com/ja/guides/jongno-stay-guide/
- 今ここは / Now Yeogi  
  https://nowyeogi.com/ja/posts/jongno-hotels-2026-09/
- TravelBook  
  https://www.travelbook.co.jp/topic/54739/

---

# 15. Batch Close

- Japanese pages analyzed: **5 / 5**
- Current Korea Inside Japanese source checked: **5 / 5**
- Japanese SERP intent review: **5 / 5**
- Direct competitor/editorial comparison: **5 / 5**
- Official hotel fact checks used where useful
- 1:1 SEO GAP tables: **5 / 5**
- Internal-link analysis: **5 / 5**
- GEO / AI Search review: **5 / 5**
- New Page Candidate backlog: **updated**
- HTML changes: **0**
- Git changes: **0**
- Production changes: **0**

## Final Batch 6 Classification

1. Myeongdong Stay — **PASS**
2. Hongdae Stay — **PASS**
3. Seoul Station Hotels — **PASS**
4. Gongdeok Station Hotels — **P1**
5. Insadong Stay — **P1**

**Batch 6 research: COMPLETE**
