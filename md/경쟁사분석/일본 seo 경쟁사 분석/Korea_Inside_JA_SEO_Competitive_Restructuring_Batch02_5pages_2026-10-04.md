# Korea Inside — Japanese SEO Competitive Restructuring Report
## Batch 2 — 5 Pages

**Date:** 2026-10-04  
**Status:** RESEARCH COMPLETE — SEO RESTRUCTURING REVIEW / NO IMPLEMENTATION  
**Language:** Japanese  
**Scope:** Existing Korea Inside Japanese 58-page program — Batch 2 / exactly 5 pages  
**Research basis:** Current Japanese Korea Inside source + Japanese-language current SERP sampling + direct Japanese competitor/editorial page review  
**Implementation:** HTML 0 / Git 0 / Production 0  

### Batch 2 Pages
1. `ja/insadong-travel-guide.html`
2. `ja/gangnam-travel-guide.html`
3. `ja/jamsil-travel-guide.html`
4. `ja/gongdeok-mapo-seoul-guide.html`
5. `ja/itaewon-travel-guide.html`

> **SERP caution:** Search-result order varies by time, location, device and personalization. This report does not claim absolute Google Japan ranking positions. “Direct Editorial Benchmark #1/#2” means the pages selected for direct structural comparison.

---

# 0. Executive Summary

Batch 2 confirms that Korea Inside should not apply one SEO repair pattern to every Japanese area guide.

Three different states appear:

### A. Strong current structure — protect it
- **Jamsil — PASS**
- **Insadong — P1**
- **Gangnam — P1**

These pages already solve the traveler’s actual planning problem more deeply than most sampled competitors.

### B. Search-facing wording needs clearer alignment
- **Itaewon — P1**

The body is strong, but the Japanese SERP is highly explicit around `梨泰院 観光`, `ハラール`, `漢南洞`, `ナイトライフ`, `アクセス`. Korea Inside already contains these topics; they need stronger surface visibility rather than a body rewrite.

### C. Japanese market intent may not match a generic area-guide framing
- **Gongdeok / Mapo — P0 Market Intent / Page-Role Review**

Japanese search results around 孔徳/麻浦 are heavily fragmented into:
- local food
- pork/trotter/pancake alleys
- airport access
- hotel base
- individual restaurants

There is much weaker evidence for a broad `孔徳 観光` head intent than for food/access/stay intent.

This does **not** mean the current Korea Inside page is weak. The page is unusually differentiated because it connects:
- food
- local evening
- airport access
- stay decision
- booked experiences
- route logic

But before changing title/H1 aggressively, Korea Inside should decide whether the representative Japanese intent remains an **Area Travel Guide** or should be positioned as a **local-food + practical-base guide**.

---

# 1. Current Korea Inside Japanese Source Baseline

## 1.1 Insadong

- File: `ja/insadong-travel-guide.html`
- SHA: `feac031b25beb62cfd0ff4e9b9583d2c397677f8`
- Title: `仁寺洞（インサドン）観光ガイド 2026｜伝統茶・工芸・街歩き | Korea Inside`
- Meta: `2026年の仁寺洞を、伝統茶、韓国工芸、サムジギル、曹渓寺、食事、景福宮・北村との組み合わせまで実用的に案内します。`
- H1: `仁寺洞観光ガイド 2026`

## 1.2 Gangnam

- File: `ja/gangnam-travel-guide.html`
- SHA: `e89bac5aa0f6b8342163b9100042e6a555b04389`
- Title: `江南（カンナム）観光ガイド 2026｜見どころ・1日ルート | Korea Inside`
- Meta: `ソウル江南を無理なく回る1日ルート。COEX、奉恩寺、狎鴎亭、清潭、江南駅の違いを比較し、ショッピング、K-pop、食事、宿泊の判断まで解説します。`
- H1: `江南観光ガイド 2026`
- Important current Japanese-source defect: one H2 remains in English/Japanese mixed form:
  - `Suggested Timing：フルルート`

## 1.3 Jamsil

- File: `ja/jamsil-travel-guide.html`
- SHA: `d5cf4dc23c09351610417c6320bcb99551838372`
- Title: `蚕室（チャムシル）観光ガイド 2026｜ロッテワールド・Seoul Sky・石村湖 | Korea Inside`
- Meta: `蚕室の1日を、ロッテワールド、Seoul Sky、石村湖、ソンリダンギル、コンサート、野球から選んで計画。自分の旅に合うルートを解説します。`
- H1: `蚕室観光ガイド 2026`

## 1.4 Gongdeok / Mapo

- File: `ja/gongdeok-mapo-seoul-guide.html`
- SHA: `89e9bffe2c54e41eb864256865c409dc37c43312`
- Title: `孔徳・麻浦ソウルガイド 2026｜市場・ローカルグルメ・夜の過ごし方 | Korea Inside`
- Meta: `孔徳市場、チョッパル、ジョン、麻浦の豚カルビ、体験、夜の散歩まで、孔徳・麻浦をローカルフード中心に実用的に案内。泊まる価値があるかも解説します。`
- H1: `孔徳・麻浦ソウルガイド 2026 ：市場・ローカルグルメ・夜`

## 1.5 Itaewon

- File: `ja/itaewon-travel-guide.html`
- SHA: `a513ff0874f1d6851504d702d7f496346086ca0a`
- Title: `梨泰院（イテウォン）ソウルガイド 2026｜漢南・グルメ・ナイトライフ | Korea Inside`
- Meta: `梨泰院をエリアと時間帯で計画。漢南・Leeum、国際色豊かな食事とハラール対応、経理団・解放村の坂道、ナイトライフまで、現実的な動線と宿泊判断を解説します。`
- H1: `梨泰院ソウルガイド 2026 ：漢南・グルメ・ナイトライフ`

---

# 2. PAGE 1 — Insadong

## 2.1 Search Intent

Primary Japanese intent:

> **仁寺洞をどう観光するか。何を見るか、何時間必要か、伝統茶・工芸・サムジギルをどう組み、景福宮・北村・益善洞とどうつなぐか。**

Japanese results commonly expose:
- `仁寺洞（インサドン）`
- `観光`
- `完全ガイド`
- `伝統茶`
- `工芸`
- `サムジギル`
- `アクセス`
- `何時間`
- `景福宮・北村`
- walking route

Korea Inside is already well aligned.

## 2.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `仁寺洞 観光`

Supporting:
- `仁寺洞 インサドン 観光`
- `仁寺洞 サムジギル`
- `仁寺洞 伝統茶`
- `仁寺洞 工芸`
- `仁寺洞 お土産`
- `仁寺洞 グルメ`
- `仁寺洞 何時間`
- `仁寺洞 アクセス`
- `安国駅 仁寺洞`
- `仁寺洞 北村`
- `仁寺洞 益善洞`

## 2.3 SERP Observation

Japanese editorial results tend to treat Insadong in one of two ways:

1. **Historic-Seoul cluster**
   - palace
   - Bukchon
   - Insadong
   - Jogyesa
   - Jongno

2. **Insadong standalone guide**
   - access
   - crafts
   - tea
   - Ssamzigil
   - galleries
   - food
   - 3-hour / half-day walking route

Korea Inside already combines both models while giving stronger “what not to overdo” guidance.

## 2.4 Direct Editorial Benchmark #1 — Korea Visit Guide

**URL:**  
https://www.koreavisitguide.com/ja/neighborhoods/jongno-insadong-travel-guide

**Observed title:**  
`鍾路・仁寺洞ガイド：宮殿と韓屋街`

**Observed structure / stops:**
1. Jongno / Insadong location and access
2. compare Gyeongbokgung / Changdeokgung / Changgyeonggung
3. Bukchon
4. Insadong tea / galleries / traditional crafts
5. Jogyesa
6. Jongno 3-ga food
7. practical sightseeing advice
8. one-day historical Seoul framing

### Strength
- excellent topic-cluster architecture
- direct access/time answer
- strong palace/Bukchon linkage
- “one day” planning intent

### Weakness vs KI
- Insadong itself is one subsection of a wider cluster
- less detailed tea/craft/shopping decision logic
- weaker “how long to stop at Ssamzigil / tea” guidance

## 2.5 Direct Editorial Benchmark #2 — Koro Journal

**URL:**  
https://www.koreabycar.com/ja/journal/insadong-seoul-guide

**Observed title:**  
`仁寺洞（インサドン）完全ガイド：伝統工芸・街歩き・隠れた路地を探索`

**Observed structure / claims:**
- access from Anguk Exit 6 / Jonggak side
- sights
- food
- shopping
- alleys
- combine with Gyeongbokgung/Bukchon
- explicit time answer:
  - Insadong alone: about 3 hours
  - palace/Bukchon combination: one day

### Strength
- excellent literal Japanese search language
- strong access/time visibility
- clear walking intent
- named place/entity density

### Weakness vs KI
- more conventional “cover everything” guide
- weaker traveler-fit filtering
- weaker stopping rules
- less discussion of fatigue after palace/Bukchon

## 2.6 Korea Inside Actual Structure

Korea Inside already includes:
- what Insadong is actually good for
- 2h / 4–5h / full-day distinction
- Anguk vs south-side start
- Insadong-gil as route spine
- craft shopping by purpose
- luggage consideration for purchases
- Ssamzigil 30–40 min
- traditional tea 40–60 min
- real meal
- Jogyesa 20–30 min
- one next neighborhood only
- Gyeongbokgung/Bukchon before Insadong
- Ikseon-dong / Cheonggyecheon / Jongno onward options
- traveler-type fit
- stay decision

This is stronger than a generic attraction list.

## 2.7 1:1 SEO GAP

| Element | Japanese competitors | Korea Inside current | Gap / Action | Priority |
|---|---|---|---|---|
| Title | `仁寺洞（インサドン）完全ガイド/観光` | already `仁寺洞（インサドン）観光ガイド` | KEEP | PASS |
| Meta | sights/access/tea/crafts | tea/crafts/Ssamzigil/Jogyesa/nearby | strong | PASS |
| H1 | 観光/完全ガイド | `仁寺洞観光ガイド 2026` | strong | PASS |
| Intro / Quick Answer | time + area role | strong but narrative | compress top answer | P1 |
| H2 | access / time / spots | editorial | expose `何時間` more literally | P1 |
| H3 | entities | decision-specific | KI stronger | KEEP |
| Keyword language | direct | strong | minor only | P1 |
| Intent | sightseeing/walking | sightseeing + decision | KI stronger | KEEP |
| Depth | entity breadth | route + time + shopping/tea judgment | KI stronger | KEEP |
| Practical info | access/time | excellent time/route | stable entity facts optional | P1 |
| Decision support | medium | very strong | KEEP | KEEP |
| Freshness | mostly evergreen | mostly evergreen | low maintenance burden | PASS |
| Entities | high | sufficient/selective | current strategy good | KEEP |
| Internal links | historic cluster | KI 58P cluster | strengthen contextual historic cluster links | P1 |
| FAQ | sometimes present | none | source-level candidate only | P2 |
| CTA | limited | experiences/stay contextual | KEEP | KEEP |
| Media/map | entity/map | editorial imagery | orientation map optional, not mandatory | P2 |
| Trust/source | mixed | can verify official entities | strong enough | PASS |
| Commercial usefulness | low-medium | crafts/tea/experience/stay | useful | KEEP |

## 2.8 KEEP

- current Title / Meta / H1
- Anguk vs south-side decision
- 2h and 4–5h timing
- Insadong-gil as route spine
- craft-shopping purpose logic
- Ssamzigil time cap
- traditional tea as a real rest period
- food before tea when hungry
- Jogyesa as pace change
- Gyeongbokgung/Bukchon → Insadong order
- one next-neighborhood rule
- day-focused character
- traveler fit / non-fit

## 2.9 P0 / P1 / P2

### P0
None.

### P1
- top Quick Answer
- explicit `何時間` search wording
- stronger historic-cluster internal links
- stable access/entity facts where useful

### P2
- source-level FAQ
- orientation walking map only if it materially improves route comprehension

## 2.10 SEO Surface Gap

Title/meta/H1: **KEEP**

Potential H2:

Current:
`まず、実際に使える時間から考える`

Candidate:
`仁寺洞観光は何時間必要？ 2時間・半日・1日の使い分け`

Current:
`どこから始める？安国か南側か`

Candidate:
`仁寺洞はどこから歩く？ 安国駅か鍾路側か`

Do not mechanically rewrite all headings.

## 2.11 Content Gap

No material body-content gap.

Only optional improvements:
- explicit stable access block
- one-line current Ssamzigil/Jogyesa verification where useful
- clearer inbound/outbound historic-Seoul links

## 2.12 Japan-Specific Market Gap

Japanese users are repeatedly shown:
- `何時間`
- station/exit
- `完全ガイド`
- `伝統茶`
- `工芸`
- palace/Bukchon combination

KI already covers the substance. Search phrasing is the remaining gap.

## 2.13 Competitor-Only Content

- long attraction lists
- detailed parking
- many individual galleries/shops
- item-by-item souvenir lists

Not automatically necessary.

## 2.14 New Page Candidates

Backlog:
- `仁寺洞 伝統茶`
- `仁寺洞 工芸・お土産`
- `景福宮・北村・仁寺洞 1日コース`

Only after independent-intent review.

## 2.15 GEO / AI Search

Make these answers easy to extract:
- Insadong alone: how long
- best start point
- Ssamzigil: how long
- tea: how long
- palace/Bukchon order
- what to do after Insadong
- who does not need a long visit

## 2.16 Internal Links

### Outbound
- `/ja/where-to-stay-in-insadong.html`
- `/ja/best-area-for-first-time-visitors-seoul.html`
- `/ja/accommodation.html`
- relevant nearby historic-area pages when available
- `taste-korea.html` only where food/tea context is natural

### Inbound
- Insadong Stay
- first-time visitor decision
- accommodation hub
- nearby historic-cluster pages

## 2.17 Final Primary Keyword

**`仁寺洞 観光`**

## 2.18 Final Status

# **P1**

---

# 3. PAGE 2 — Gangnam

## 3.1 Search Intent

Primary Japanese intent:

> **江南で何を見るか、COEX・奉恩寺・狎鴎亭・清潭・江南駅をどう分け、半日か1日をどう使うか。**

Supporting intent:
- shopping
- K-pop
- K-beauty
- cafe
- food
- nightlife
- where to stay
- how long

This is an area where a major SEO danger exists:

> treating all “Gangnam” destinations as one walkable neighborhood.

Korea Inside handles this better than sampled competitors.

## 3.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `江南 観光`

Supporting:
- `江南 カンナム 観光`
- `江南 見どころ`
- `江南 1日`
- `江南 半日`
- `江南 COEX`
- `江南 狎鴎亭`
- `江南 清潭`
- `江南 ショッピング`
- `江南 K-pop`
- `江南 Kビューティー`
- `江南 何時間`

## 3.3 SERP Observation

Japanese competitors strongly foreground:
- `観光完全ガイド`
- COEX
- Apgujeong
- Cheongdam
- shopping
- cafe
- K-pop
- model course
- recommended 10 spots

The listicle model is highly visible.

Korea Inside’s major advantage is recognizing Gangnam as multiple clusters rather than one continuous walking route.

## 3.4 Direct Editorial Benchmark #1 — K-ELITE

**URL:**  
https://k-elitevvip.com/ja/info/blog/gangnam-sightseeing-guide

**Observed title:**  
`江南観光ガイド2026|COEX・狎鴎亭・清潭・グルメ`

**Observed framing:**
- current 2026 guide
- COEX / Starfield Library
- Bongeunsa / Seonjeongneung
- Apgujeong / Cheongdam
- Gangnam Station
- shopping
- food
- nightlife
- model-course logic
- area differences

### Strength
- excellent direct Japanese search wording
- top entities in title
- up-to-date framing
- clear area/category breadth

### Weakness vs KI
- less explicit “do not combine both major Gangnam routes” rule
- weaker stay-location consequences
- less route-friction analysis

## 3.5 Direct Editorial Benchmark #2 — Traveloka Japan

**URL:**  
https://www.traveloka.com/ja-jp/explore/culinary/gangnam_tourism/592909

**Observed title:**  
`江南（カンナム）観光完全ガイド2026：ファッション・カフェ・グルメ・K-POPまで！最新おすすめスポット10選とモデルコース`

**Observed structure:**
1. `江南（カンナム）観光の魅力とは？`
2. `おすすめ観光スポット10選`
3. COEX
4. Starfield Library
5. Bongeunsa
6. Garosu-gil
7. Apgujeong Rodeo
8. Gangnam Station
9. Seonjeongneung
10. K-Star Road
11. Goto Mall
12. Banpo Bridge
13. half-day model course
14. full-day model course
15. FAQ

### Strength
- high keyword/entity coverage
- clear model-course intent
- FAQ
- very scan-friendly

### Weakness vs KI
- geography becomes over-broad
- mixes COEX, Garosu-gil, Apgujeong, Gangnam Station and Banpo in a way that can create inefficient movement
- “10 spots” architecture prioritizes coverage over travel friction
- commercial inventory interrupts editorial flow

## 3.6 Korea Inside Actual Structure

Current KI:
- defines what Gangnam actually means
- asks “which Gangnam?”
- Route 1:
  - Bongeunsa
  - COEX / Starfield Library
  - Seonjeongneung
  - Gangnam Station
- Route 2:
  - Dosan Park
  - Apgujeong Rodeo
  - K-Star Road
  - Cheongdam
- explicitly says most travelers do not need both routes in one day
- 3–4h short version
- COEX Aquarium branch
- Gangnam Station evening role
- shopping by area
- K-pop
- K-beauty fixed appointments
- food by area
- four planning mistakes
- stay-or-visit decision
- FAQ

This is strategically stronger.

## 3.7 1:1 SEO GAP

| Element | Japanese competitors | Korea Inside current | Gap / Action | Priority |
|---|---|---|---|---|
| Title | COEX/Apgujeong/Cheongdam often explicit | `見どころ・1日ルート` | already strong; entity test possible | PASS/P2 |
| Meta | entities + activity | excellent entity coverage | KEEP | PASS |
| H1 | 江南観光 | same | KEEP | PASS |
| Intro / Quick Answer | broad attraction promise | clear geography warning | KI stronger | KEEP |
| H2 | direct attractions/model courses | route/decision based | strong | KEEP |
| H3 | entity list | entity + decision | KI stronger | KEEP |
| Keyword language | direct | strong | minor only | P1 |
| Intent | attractions/list | area selection + route | KI stronger | KEEP |
| Depth | broad | deep | KI stronger | KEEP |
| Practical info | hours/entities | travel-friction strong | stable fact blocks optional | P1 |
| Decision support | medium | exceptional | KEEP | KEEP |
| Freshness | 2026 updated | check-current section exists | KEEP | P1 |
| Entities | very high | high/selective | current strategy good | KEEP |
| Internal links | mixed | strong KI cluster | strengthen K-beauty/Stay links | P1 |
| FAQ | yes | 6 FAQ | strong | PASS |
| CTA | travel booking | contextual COEX/experience | KEEP | KEEP |
| Media/map | list imagery | route imagery | orientation map could help geography | P2 |
| Trust/source | mixed | explicit official current checks | strong | PASS |
| Commercial usefulness | high | high | strong | KEEP |

## 3.8 Critical Current Defect

Current H2:

`Suggested Timing：フルルート`

This is an obvious Japanese-page English residual.

It is not a strategy question.

**Recommended exact direction:** replace with natural Japanese equivalent during the eventual approved implementation stage.

Candidate:
`所要時間の目安：フルルート`

This should be treated as a **P0 micro-fix / QA defect**, even though the overall page strategy is P1.

## 3.9 KEEP

- current title/meta/H1
- “which Gangnam?” concept
- two-route architecture
- do-not-do-both rule
- COEX time-budget decision
- correct stations by subarea
- Gangnam Station as evening zone rather than universal starting point
- shopping by area
- K-pop as optional layer
- K-beauty appointment burden
- stay-or-visit decision
- FAQ

## 3.10 P0 / P1 / P2

### P0 micro-fix
- `Suggested Timing` English residual

### P1
- stronger stable practical facts where useful
- K-Beauty / Stay contextual linking
- freshness/current-check layer
- test selected exact query phrasing

### P2
- orientation map for separated Gangnam clusters
- title entity experiment only after Search Console evidence

## 3.11 SEO Surface Gap

Current title/meta/H1: **KEEP**

Potential H2:

Current:
`まず「どの江南」に行くか決める`

KEEP — this is differentiation.

Potential supplementary wording in lead/Quick Answer:
`江南は1つの徒歩観光エリアではありません。`

This answer should remain near the top because it directly solves the core planning mistake.

## 3.12 Content Gap

No major content gap.

Potential additions:
- compact area-to-station mapping
- current major attraction hours only if stable/verified
- current popup/event layer when relevant

## 3.13 Japan-Specific Market Gap

Japanese competitors emphasize:
- `おすすめスポット10選`
- cafe
- shopping
- K-pop
- model course

KI should not imitate the 10-list structure.

Its advantage is:
- choose a Gangnam cluster first
- protect transit time
- choose the correct station
- avoid unnecessary cross-district movement

## 3.14 Competitor-Only Content

- Goto Mall
- Banpo Bridge in generic Gangnam route
- long café lists
- “women in their 20s–40s” demographic framing
- broad “must visit” entity lists

Not automatic gaps.

## 3.15 New Page Candidates

Backlog:
- `COEX 観光ガイド`
- `狎鴎亭・清潭 ショッピング`
- `江南 Kビューティー`
- `江南 K-pop`

Only after independent-intent review.

## 3.16 GEO / AI Search

Extractable answers:
- Gangnam is not one walkable neighborhood
- first-time Route 1
- fashion/K-pop Route 2
- 3–4 hours vs 6–8 hours
- which station for COEX / Apgujeong
- whether first-time short Seoul trips need Gangnam
- when staying in Gangnam makes sense

## 3.17 Internal Links

### Outbound
- `/ja/where-to-stay-in-gangnam.html`
- `/ja/k-beauty.html`
- `/ja/best-area-for-shopping-seoul.html`
- `/ja/best-area-for-luxury-hotels-seoul.html`
- `/ja/accommodation.html`
- `/ja/jamsil-travel-guide.html` only where southeast-Seoul comparison is useful

### Inbound
- Gangnam Stay
- K-Beauty
- Shopping
- Luxury Hotels
- accommodation hub

## 3.18 Final Primary Keyword

**`江南 観光`**

## 3.19 Final Status

# **P1**
plus one **P0 micro-fix** for the English residual H2.

---

# 4. PAGE 3 — Jamsil

## 4.1 Search Intent

Primary Japanese intent:

> **蚕室で何をするか。ロッテワールド、ロッテワールドタワー、Seoul Sky、石村湖、ソンリダンギルをどう組み合わせるか。**

Secondary:
- family
- aquarium
- concert
- baseball
- weather
- half-day / full-day
- stay

Current Korea Inside Japanese page is highly aligned.

## 4.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `蚕室 観光`

Supporting:
- `蚕室 チャムシル 観光`
- `蚕室 ロッテワールド`
- `蚕室 ロッテワールドタワー`
- `蚕室 Seoul Sky`
- `蚕室 石村湖`
- `蚕室 ソンリダンギル`
- `蚕室 子連れ`
- `蚕室 コンサート`
- `蚕室 野球`
- `蚕室 何時間`

## 4.3 SERP Observation

Competitors commonly organize Jamsil around:
- Lotte World Tower
- Seoul Sky
- Aquarium
- Lotte World
- Seokchon Lake
- Songridan-gil
- mall
- half-day / one-day route

Korea Inside adds two major layers competitors often underdevelop:
1. **event branch** — Olympic Park / KSPO Dome / baseball
2. **decision architecture** — choose the type of Jamsil day before adding destinations

## 4.4 Direct Editorial Benchmark #1 — Korea Visit Guide

**URL:**  
https://koreavisitguide.vercel.app/ja/neighborhoods/jamsil-lotte-world-tower-guide

**Observed title:**  
`蚕室&ロッテワールドタワー完全ガイド`

**Observed structure / framing:**
- location and access
- Lotte World Tower
- Seoul Sky
- Aquarium
- mall
- Seokchon Lake
- practical half-day/one-day framing
- weather-independent indoor/outdoor combination
- prices near the top

### Strength
- immediate entity/price/access answer
- strong scanability
- direct `完全ガイド` intent
- good first-time orientation

### Weakness vs KI
- narrower Lotte cluster
- less event-day differentiation
- less route-priority / fatigue / “do not combine everything” guidance

## 4.5 Direct Editorial Benchmark #2 — K Village MODULY

**URL:**  
https://kasioda.com/guide/jamsilkorea/

**Observed title:**  
`ロッテワールドだけじゃない！蚕室（チャムシル）エリア完全ガイド`

**Observed structure:**
1. Lotte World Mall
2. Lotte World
3. Lotte World Tower
4. Seokchon Lake
5. lake café street
6. Bangi food street
7. summary

### Strength
- clear entity discovery
- directly combats “Jamsil = only Lotte World”
- easy to scan

### Weakness vs KI
- primarily a six-place list
- less choice logic
- weaker concert/baseball branch
- weaker weather/family/stay decision

## 4.6 Supporting Official Evidence

Current Japanese official tourism sources reinforce Jamsil’s core entity set:
- Lotte World Tower / Seoul Sky
- Seokchon Lake
- Songridan-gil
- Olympic Park
- Jamsil events/sports

This supports the current KI cluster model.

## 4.7 Korea Inside Actual Structure

Current page includes:
- choose what kind of Jamsil day
- at-a-glance area structure
- correct side of Jamsil Station
- Lotte World full-day branch
- queue / Magic Pass context
- non-theme-park route
- Aquarium / mall / lake / Songridan-gil
- weather
- Olympic Park / KSPO Dome
- Jamsil Sports Complex
- food-area choices
- rest
- family logic
- common mistakes
- stay or visit
- traveler types
- FAQ

This is stronger than the benchmark list architecture.

## 4.8 1:1 SEO GAP

| Element | Japanese competitors | Korea Inside current | Gap / Action | Priority |
|---|---|---|---|---|
| Title | Jamsil + Lotte entities | same + Seoul Sky/Seokchon | excellent | PASS |
| Meta | entity list/route | route-choice oriented | excellent | PASS |
| H1 | Jamsil guide | same | PASS |
| Intro / Quick Answer | entity-first | decision-first and direct | KI stronger | PASS |
| H2 | attraction sections | activity-day branches | KI stronger | PASS |
| H3 | place details | decision and friction | KI stronger | PASS |
| Keyword language | direct | direct | strong | PASS |
| Intent | Lotte cluster | Lotte + event + stay | broader and coherent | PASS |
| Depth | medium-high | very high | KI stronger | PASS |
| Practical info | access/prices | station side, route, event station | strong | PASS |
| Decision support | medium | exceptional | KEEP | PASS |
| Freshness | price/entity facts | current check section | maintain | P1 maintenance |
| Entities | high | high | PASS |
| Internal links | attraction cluster | Jamsil/Lotte/Seoul Sky/Stay | strong | PASS |
| FAQ | sometimes | 8 FAQ | strong | PASS |
| CTA | tickets | contextual | KEEP | PASS |
| Media/map | entity maps | orientation map already present | strong | PASS |
| Trust/source | mixed/official | official-fact framework | maintain | PASS |
| Commercial usefulness | high | high | strong | PASS |

## 4.9 KEEP

Nearly all current architecture:
- title/meta/H1
- choose-the-day-type lead
- station-side distinction
- Lotte World as full-day
- non-theme-park branch
- Seokchon Lake as real walk
- Songridan-gil role
- weather decisions
- KSPO Dome / sports branch
- food choice by convenience vs leaving the complex
- family energy logic
- mistakes
- stay decision
- FAQ
- orientation map

## 4.10 P0 / P1 / P2

### P0
None.

### P1 maintenance only
- revalidate volatile ticket/event/venue facts
- keep event-station facts current

### P2
- no structural work currently justified

## 4.11 SEO Surface Gap

No urgent search-facing rewrite recommended.

Potential low-priority test only:
- whether `ロッテワールドタワー` should appear in title instead of or alongside `Seoul Sky`.

Do not change without Search Console/CTR evidence because current title already covers the representative destination set.

## 4.12 Content Gap

No material strategic content gap.

Potential competitor-only ideas such as more cafés or more food-street entries do not improve the page role enough to justify expansion.

## 4.13 Japan-Specific Market Gap

No major gap.

Japanese users want recognizable Jamsil entities. Current page already supplies them and adds stronger decision logic.

## 4.14 Competitor-Only Content

- longer café lists
- more individual restaurants
- current ticket prices above the fold
- more shopping-mall detail

These are optional, not required.

## 4.15 New Page Candidates

Backlog:
- `蚕室 ロッテワールドタワー`
- `ソンリダンギル`
- `蚕室 コンサート / KSPO Dome`

But the existing Lotte World and Seoul Sky pages already cover much of the likely independent demand. Cannibalization review is mandatory.

## 4.16 GEO / AI Search

Current page already provides strong answer units:
- three types of Jamsil day
- correct Jamsil Station side
- when Lotte World consumes the day
- what to do without the theme park
- when to use Olympic Park station
- how weather changes Seoul Sky
- family/stay decision

## 4.17 Internal Links

### Outbound
- `/ja/lotte-world-seoul.html`
- `/ja/seoul-sky-guide.html`
- `/ja/where-to-stay-in-jamsil.html`
- `/ja/best-area-for-families-seoul.html`
- `/ja/accommodation.html`

### Inbound
- Lotte World
- Seoul Sky
- Jamsil Stay
- Families
- accommodation hub

## 4.18 Final Primary Keyword

**`蚕室 観光`**

## 4.19 Final Status

# **PASS**

---

# 5. PAGE 4 — Gongdeok / Mapo

## 5.1 Search Intent

This page is the most important strategic finding in Batch 2.

Japanese current search results do **not** show a strong unified “Gongdeok sightseeing” content field comparable to Hongdae, Myeongdong, Insadong or Jamsil.

Instead, demand is fragmented across:

- `孔徳 グルメ`
- `孔徳市場`
- `孔徳 チョッパル`
- `麻浦 グルメ`
- `麻浦 カルメギサル`
- `孔徳 空港鉄道`
- `孔徳 ホテル`
- `麻浦 ホテル`
- individual restaurant queries

This creates both a weakness and an opportunity.

## 5.2 Current Page Role vs Japanese Market Intent

### Current KI role
A broad practical area guide:
- market
- local food
- experiences
- evening walk
- airport/transport
- stay decision

### Observed Japanese intent
Mostly:
- food
- airport access
- hotel base
- individual restaurant/entity discovery

### Conflict

Japanese SERP does not strongly validate a broad `孔徳 観光` head term.

Therefore any major title/H1 change should be treated as **page-role review**, not routine SEO wording cleanup.

## 5.3 Primary / Supporting Keywords

**Provisional Primary Keyword:** `孔徳 グルメ`

Alternative cluster candidate:
- `孔徳 麻浦 グルメ`

Supporting:
- `孔徳市場`
- `孔徳 チョッパル`
- `孔徳 チヂミ`
- `麻浦 豚カルビ`
- `麻浦 グルメ`
- `孔徳 空港鉄道`
- `孔徳 ホテル`
- `孔徳 何がある`
- `孔徳 観光`
- `麻浦 観光`

**Important:** This primary keyword is a research recommendation, not an approved page-role change.

## 5.4 SERP Observation

The sampled Japanese landscape is much less saturated with broad area guides.

This means:
- there is no need to imitate a dominant `孔徳観光完全ガイド` format
- KI may have a chance to own a differentiated combined intent
- but the search-facing promise should reflect what Japanese travelers actually search

The strongest recurring topics are:
1. local food alleys
2. local restaurants
3. airport transport convenience
4. hotel/base convenience
5. low-tourist/local atmosphere

## 5.5 Direct Editorial Benchmark #1 — Travel.jp

**URL:**  
https://www.travel.co.jp/guide/article/36816/

**Observed title:**  
`穴場かも？アクセス便利！ソウル 孔徳・麻浦のおいしい名店4選`

**Observed framing:**
- Gongdeok/Mapo as convenient base
- AREX / lines 5 and 6
- close to Hongdae / central Seoul
- local-business-district feel
- food as the main attraction
- four restaurant/entity recommendations
- pork / local meal focus

### Strength
- matches real Japanese “why would I go there?” intent
- food and access promise is immediate
- concrete named entities

### Weakness vs KI
- older editorial piece
- narrow restaurant-list role
- no full afternoon/evening route
- limited traveler-fit logic
- little current experience/stay decision depth

## 5.6 Direct Editorial Benchmark #2 — Minfor

**URL:**  
https://minfor.jp/tourism/1394/

**Article:** `ソウルで行くべき穴場SPOT3選`

**Observed Gongdeok section:**
- location
- strong transport convenience
- Lines 5/6, AREX, Gyeongui–Jungang
- direct airport access
- local restaurants/cafes
- “not a large-department-store tourist zone” positioning

### Strength
- explains Gongdeok’s role quickly
- access is highly visible
- “local / convenient / less touristy” concept is clear

### Weakness vs KI
- Gongdeok is only one subsection
- little food-route structure
- no Mapo differentiation
- no evening decision
- no stay trade-off beyond convenience

## 5.7 Supporting Entity / Topic Benchmarks

Current Japanese results also show strong content around:
- Gongdeok jokbal alley
- Mapo pancake alley
- Mapo galmaegisal street
- individual local restaurants
- airport convenience
- hotels around Gongdeok/Mapo

This confirms that **food + base utility** is the strongest Japanese semantic cluster.

## 5.8 Korea Inside Actual Structure

KI currently offers:
- why Gongdeok/Mapo is worth knowing
- Gongdeok Market first
- old Mapo food
- Korean BBQ
- Pyongyang naengmyeon
- desserts / makgeolli / making experience
- head spa / spa / massage
- Gyeongui Line Forest Park
- local evening
- how much time
- whether to stay
- traveler fit / skip conditions
- half-day + evening route
- Gongdeok vs Mapo starting point
- Incheon Airport
- luggage
- ordering mistakes
- FAQ-like question block

This is unusually deep compared with the fragmented Japanese competitor field.

## 5.9 1:1 SEO GAP

| Element | Japanese competitors | Korea Inside current | Gap / Action | Priority |
|---|---|---|---|---|
| Title | food/access/hidden-area framing | broad `ソウルガイド` + food/night | role may be too broad for search | **P0** |
| Meta | food + access | food + experience + stay | strong but may need tighter intent | **P0/P1** |
| H1 | no dominant broad benchmark | broad area guide | role review required | **P0** |
| Intro / Quick Answer | “convenient + local food” | editorial reason-to-go | should state default purpose faster | P0/P1 |
| H2 | restaurant/entity topics | decision/route | KI stronger | KEEP |
| H3 | restaurant entities | food + activity decisions | strong | KEEP |
| Keyword language | `グルメ`, `孔徳市場`, `空港鉄道` | some present, less title-level | major gap | **P0** |
| Intent | fragmented food/access/stay | integrated area guide | strategic differentiation but search mismatch | **P0** |
| Depth | fragmented | very deep combined guide | KI stronger | KEEP |
| Practical info | access/entities | route/access/stay strong | strong | KEEP |
| Decision support | weak-medium | very strong | KEEP | KEEP |
| Freshness | restaurant-level variable | broader evergreen | entity freshness needed | P1 |
| Entities | restaurant-heavy | selected experiences/foods | could strengthen permanent food entities | P1 |
| Internal links | weak | broad KI cluster | major advantage | KEEP/P1 |
| FAQ | scattered | question-like section | strong enough | KEEP |
| CTA | weak / restaurant | experiences/spa/stay | differentiated | KEEP |
| Media/map | entity location | editorial | food/access orientation map could help | P2 |
| Trust/source | mixed | official food/transport facts possible | strengthen | P1 |
| Commercial usefulness | restaurant/stay | stay + experiences + food | strong potential | KEEP |

## 5.10 KEEP

- Gongdeok vs Mapo distinction
- market and local-food identity
- one good meal is enough
- not a checklist area
- local evening / no fake nightlife positioning
- AREX convenience but “AREX alone is not a reason to stay”
- luggage/arrival-day reality
- traveler fit
- stay decision
- food-experience mapping

## 5.11 P0 / P1 / P2

### P0 — Market Intent / Page-Role Review
Before implementation, decide:

**Option A — retain Area Guide role**
- strengthen `孔徳・麻浦`
- explicitly promise `グルメ・空港アクセス・ローカルな夜`
- use broad guide as a differentiated hub

**Option B — reposition toward food-led area guide**
- make `孔徳・麻浦グルメ` the primary search promise
- keep transport/stay as supporting decisions

No page-role change should be implemented without explicit approval.

### P1
- stable food-alley entities
- official transport/source cues
- contextual links to Stay / AREX / food hub
- entity freshness

### P2
- orientation map: Gongdeok Market → Mapo food → Gyeongui Line evening
- future Food Detail pages

## 5.12 SEO Surface Candidates

### Candidate A — Keep Area Guide Role
`孔徳（コンドク）・麻浦（マポ）ガイド 2026｜ローカルグルメ・市場・空港アクセス | Korea Inside`

### Candidate B — Food-Led Search Role
`孔徳・麻浦グルメガイド 2026｜市場・チョッパル・豚カルビ・ローカルな夜 | Korea Inside`

Candidate B is more aligned with the sampled Japanese SERP, but changes the search-facing page role more materially.

### Candidate H1 A
`孔徳・麻浦ガイド 2026：ローカルグルメ・市場・空港アクセス`

### Candidate H1 B
`孔徳・麻浦グルメガイド 2026`

**No implementation recommendation until page-role approval.**

## 5.13 Content Gap

No large body-content gap.

Useful verified layers:
- Gongdeok jokbal/pancake alleys
- Mapo galmaegisal
- permanent food anchors
- precise transport role
- solo-ordering limitations

## 5.14 Japan-Specific Market Gap

The Japanese market appears to understand Gongdeok primarily as:
- good food
- local/less-touristy Seoul
- airport-convenient base

That should be visible immediately.

## 5.15 Competitor-Only Content

- restaurant-by-restaurant menus
- exact prices
- many individual reviews
- large hotel lists

Not necessary in the Travel Guide body.

## 5.16 New Page Candidates

Strong backlog:
- `孔徳市場 グルメ`
- `孔徳 チョッパル・チヂミ`
- `麻浦 豚カルビ`
- `孔徳 空港アクセス`
- `孔徳・麻浦 グルメ`

The strongest candidate is likely **孔徳・麻浦グルメ**, but cannibalization with the current guide and future Taste Korea architecture must be reviewed first.

## 5.17 GEO / AI Search

The page should clearly answer:
- why go to Gongdeok/Mapo
- is it sightseeing or food?
- Gongdeok Market vs Mapo dinner
- how many hours
- Gongdeok vs Mapo start
- is AREX convenience enough to stay
- who should skip it on a short first trip

## 5.18 Internal Links

### Outbound
- `/ja/accommodation.html`
- `/ja/arex.html`
- `/ja/best-area-for-airport-access-seoul.html`
- `/ja/best-area-for-solo-travelers-seoul.html`
- `/ja/best-area-for-couples-seoul.html`
- `/ja/taste-korea.html`
- Hongdae only where west-Seoul route logic is natural

### Inbound
- accommodation hub
- airport-access stay guide
- solo/couples area decision
- AREX
- Taste Korea / future food cluster

## 5.19 Final Primary Keyword

**Provisional:** `孔徳 グルメ`

## 5.20 Final Status

# **P0 — Market Intent / Page-Role Review**

---

# 6. PAGE 5 — Itaewon

## 6.1 Search Intent

Primary Japanese intent:

> **梨泰院で何をするか。漢南、Leeum、国際料理、ハラール、経理団、解放村、ナイトライフをどう使い分けるか。**

Japanese search pages strongly foreground:
- `梨泰院（イテウォン）`
- `観光`
- `グルメ`
- `ハラール`
- `ナイトライフ`
- `漢南洞`
- `アクセス`
- `治安`
- `坂`
- `梨泰院クラス`

KI already covers the important modern planning questions without reducing the district to nightlife or drama locations.

## 6.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `梨泰院 観光`

Supporting:
- `梨泰院 イテウォン 観光`
- `梨泰院 グルメ`
- `梨泰院 ハラール`
- `梨泰院 ナイトライフ`
- `梨泰院 漢南洞`
- `梨泰院 Leeum`
- `梨泰院 経理団`
- `梨泰院 解放村`
- `梨泰院 アクセス`
- `梨泰院 何時間`
- `梨泰院 一人旅`

## 6.3 SERP Observation

The current Japanese result field is broad and sometimes outdated.

The strongest current pages emphasize:
- international food
- halal
- nightlife
- Leeum
- Hannam
- slopes
- current district change
- operating-status verification

This aligns strongly with KI’s existing body.

## 6.4 Direct Editorial Benchmark #1 — Korea Visit Guide

**URL:**  
https://www.koreavisitguide.com/ja/neighborhoods/itaewon-halal-food-nightlife-guide

**Observed title:**  
`梨泰院(イテウォン)完全ガイド`

**Current-data signal:**  
Information checked August 2026.

**Observed framing:**
- why Itaewon differs
- Seoul Central Mosque / halal restaurant area
- world food
- nightlife
- club street
- quieter Gyeongridan bars
- access via Itaewon / Noksapyeong
- warning that shops change quickly
- verify hours and halal status

### Strength
- exact Japanese head keyword
- excellent halal/currentness signal
- direct access
- strong modern Itaewon positioning

### Weakness vs KI
- less complete east-to-west route logic
- weaker Hannam → central Itaewon → evening transition
- weaker stay decision
- less “you may stop after dinner” permission

## 6.5 Direct Editorial Benchmark #2 — IVisitKorea

**URL:**  
https://www.ivisitkorea.com/ja/things-to-do-in-itaewon/

**Observed title:**  
`梨泰院で楽しめる地元ならではのアクティビティ12選：観光スポット、グルメ、ナイトライフ`

**Current update:** August 2026.

**Observed framing:**
- Leeum Museum
- world food street
- antique shopping
- nightlife
- Korean BBQ / halal restaurants
- rooftop bars
- Namsan/N Seoul Tower walks
- tours / pub crawl / cooking/food experience
- price range for experiences

### Strength
- highly direct `things to do` query coverage
- named entities
- activity/product actionability
- clear commercial value

### Weakness vs KI
- list architecture
- less route pruning
- weaker slope/friction integration
- weaker explanation of who can skip Itaewon

## 6.6 Supporting Current Benchmarks

Other recent Japanese pages emphasize:
- Hannam as art/design/fashion district
- halal-food current verification
- Itaewon’s changing post-2022 commercial structure
- steep hills as a practical issue

These reinforce KI’s current strategy.

## 6.7 Korea Inside Actual Structure

Current KI includes:
- why Itaewon is still worth visiting
- four subareas
- Hannam/Hangangjin first
- Leeum
- central Itaewon
- food by purpose
- Central Mosque / halal
- Gyeongridan
- Haebangchon
- four evening choices
- half-day + night
- full day
- traveler fit
- stay or visit
- 500m / slope friction
- taxi use
- airport-access stay issue
- practical mistakes
- booking conditions
- FAQ-like question block
- Hongdae vs Itaewon nightlife decision

This is a strong modern area-guide structure.

## 6.8 1:1 SEO GAP

| Element | Japanese competitors | Korea Inside current | Gap / Action | Priority |
|---|---|---|---|---|
| Title | `梨泰院 完全ガイド/観光` | `梨泰院ソウルガイド` + Hannam/food/nightlife | add `観光` consideration | P1 |
| Meta | food/halal/nightlife/access | excellent route + halal + hills | KEEP | PASS |
| H1 | `梨泰院観光` | `梨泰院ソウルガイド` | search wording can be stronger | P1 |
| Intro / Quick Answer | direct area summary | strong | KEEP/P1 |
| H2 | halal/nightlife/access | decision-first | strong | KEEP |
| H3 | named entities | decision/trade-off | KI stronger | KEEP |
| Keyword language | very literal | rich but less head-term explicit | P1 |
| Intent | activities/list | area/time decision | KI stronger | KEEP |
| Depth | entity/action | deeper route and friction | KI stronger | KEEP |
| Practical info | access/entity | hills/taxi/booking strong | strong | PASS |
| Decision support | medium | exceptional | KEEP | KEEP |
| Freshness | explicit check dates | current facts mixed evergreen | checked-date layer useful | P1 |
| Entities | high | selective | strong | KEEP/P1 |
| Internal links | moderate | broad cluster | strengthen K-Beauty/Nightlife/Stay | P1 |
| FAQ | often | question block without dedicated schema | source-level review only | P2 |
| CTA | tours | contextual pub crawl/food/K-beauty | strong | KEEP |
| Media/map | venue images | strong area imagery | route map could help slopes/subareas | P2 |
| Trust/source | mixed | halal/status caution strong | major advantage | KEEP |
| Commercial usefulness | high | high and better contextualized | KEEP | KEEP |

## 6.9 KEEP

- Hannam/Hangangjin → central Itaewon route
- Leeum decision
- food by actual purpose
- mosque/halal verification warning
- Gyeongridan as quieter evening
- Haebangchon slope trade-off
- four different night endings
- half-day + night default
- who should skip it
- 500m last-walk insight
- selective taxi use
- stay decision
- do-not-force-nightlife rule

## 6.10 P0 / P1 / P2

### P0
None.

### P1
- Title/H1 search wording review
- explicit `観光`
- checked/current layer
- stronger inbound/outbound contextual links
- maybe `何時間` surface wording

### P2
- orientation map by subarea / slope
- source-level FAQ structure review

## 6.11 SEO Surface Candidates

### Candidate Title
`梨泰院（イテウォン）観光ガイド 2026｜漢南・ハラール・グルメ・夜 | Korea Inside`

Alternative if “nightlife” remains more important:
`梨泰院（イテウォン）観光ガイド 2026｜漢南・グルメ・ナイトライフ | Korea Inside`

### Candidate H1
`梨泰院（イテウォン）観光ガイド 2026`

This improves head-term clarity without changing the body role.

### Candidate time H2
Current:
`梨泰院の半日＋夜ルート`

This is already strong and should remain.

A separate `何時間` H2 is not required if the lead/route summary answers it clearly.

## 6.12 Content Gap

No major body gap.

Optional:
- current Hannam anchor entities
- current halal-status verification sources
- current event/nightlife conditions where relevant

## 6.13 Japan-Specific Market Gap

Japanese competitors sometimes still overemphasize:
- `梨泰院クラス`
- generic “international town”
- old shopping narratives

KI is more current by treating Itaewon as:
- Hannam art/design
- international/halal food
- multiple types of evening
- hills and movement friction

This differentiation should be retained.

## 6.14 Competitor-Only Content

- long nightlife list
- generic safety rankings
- drama-location list
- many restaurant names
- specific bar/club rankings

Not necessary as evergreen core.

## 6.15 New Page Candidates

Backlog:
- `梨泰院 ハラールフード`
- `漢南洞 観光・ショッピング`
- `梨泰院 ナイトライフ`
- `解放村・経理団 夜`

Strongest likely candidates:
- halal
- Hannam
- nightlife

Separate intent validation required.

## 6.16 GEO / AI Search

Top answers should make clear:
- Itaewon is not one street
- start in Hannam if art/design matters
- halal/mosque value
- Gyeongridan vs Haebangchon vs central nightlife
- half-day + evening default
- hills/500m friction
- who should skip Itaewon on short first trips

## 6.17 Internal Links

### Outbound
- `/ja/where-to-stay-in-itaewon.html`
- `/ja/best-area-for-nightlife-seoul.html`
- `/ja/best-area-for-couples-seoul.html`
- `/ja/best-area-for-solo-travelers-seoul.html`
- `/ja/k-beauty.html`
- `/ja/accommodation.html`
- `/ja/hongdae-travel-guide.html` for nightlife comparison

### Inbound
- Itaewon Stay
- Nightlife area decision
- Couples
- Solo
- accommodation hub
- K-Beauty if Hannam appointments are discussed
- Hongdae where nightlife comparison is relevant

## 6.18 Final Primary Keyword

**`梨泰院 観光`**

## 6.19 Final Status

# **P1**

---

# 7. Cross-Page Findings

## 7.1 Batch 2 Is Not a “More Keywords” Problem

Insadong, Gangnam and Jamsil already have strong Japanese title/H1 alignment.

The improvement target is mostly:
- first-answer extraction
- `何時間`
- stable entity/access facts
- currentness
- contextual internal links

A title rewrite on every page would create unnecessary churn.

## 7.2 The Strongest Korea Inside Advantage Is Spatial Decision-Making

This Batch shows the moat clearly.

### Gangnam
Competitors often mix:
- COEX
- Garosu-gil
- Apgujeong
- Gangnam Station
- Banpo

into one “best spots” route.

KI instead says:
> choose which Gangnam first.

### Jamsil
Competitors often list:
- Lotte World
- Tower
- Aquarium
- lake
- food street

KI instead says:
> choose what kind of Jamsil day this is.

### Itaewon
Competitors often list:
- food
- clubs
- Leeum
- shopping

KI instead says:
> Hannam → central Itaewon → choose how the night ends.

This decision architecture should remain untouched.

## 7.3 Japanese `何時間` Intent Is Repeating

Across:
- Insadong
- Gangnam
- Jamsil
- Itaewon

users benefit from clear time answers.

Korea Inside often already has the numbers in the body.

SEO improvement:
> make them more visible in lead / selected H2 / FAQ where structurally allowed.

## 7.4 `観光` Is Usually Right — But Not Always

For:
- Insadong
- Gangnam
- Jamsil
- Itaewon

`観光` is a strong Japanese search-facing term.

For:
- Gongdeok / Mapo

the SERP appears much more food/access/stay driven.

This is exactly why all 58 pages should not use one Japanese SEO template.

## 7.5 Low Competition Can Reveal a Page-Role Opportunity

Gongdeok is valuable precisely because competitors are fragmented.

Korea Inside can potentially own a more complete journey:
> airport-friendly base → local food → evening → stay decision

But the search-facing promise must be chosen carefully.

## 7.6 Listicle Entity Volume Is Not the Target

Competitors win scanability with:
- 10 spots
- 12 things to do
- 4 restaurants
- long entity inventories

KI should borrow:
- entity clarity
- search language
- practical facts

but not the “more entries = better page” model.

## 7.7 GEO / AI Search Structure

Best answer-unit pattern:

> **Default → Alternative → Exception → Time → Starting point → Friction → Next action**

This matches KI’s existing editorial strength.

---

# 8. Japan SEO Restructuring Rules — Batch 2 Additions

## Rule 11 — Area Geography Can Be the Differentiator

Where competitors flatten a broad district into one list, KI should explicitly explain subareas and transit friction.

## Rule 12 — `何時間` Should Be Extractable

If time guidance already exists, surface it without creating duplicate paragraphs.

## Rule 13 — Page Role Can Differ by Japanese Market

A broad English/other-language Area Guide is not automatically best represented by the same Japanese head keyword.

If SERP intent diverges:
- preserve facts/body
- report the role conflict
- get user approval before changing the representative search intent

## Rule 14 — Sparse SERP Competition Is Not Permission to Invent Demand

Gongdeok’s weak broad tourism field is a signal to evaluate carefully, not a reason to claim a large `孔徳 観光` demand.

## Rule 15 — Fix Obvious Language Residue Separately From Strategy

Example:
- Gangnam `Suggested Timing`

This is a QA defect, not evidence that the whole page requires restructuring.

---

# 9. P0 Action List

## Gongdeok / Mapo
- decide representative Japanese page role
- evaluate Area Guide vs Food-Led Area Guide
- choose primary query family only after role approval
- then revise Title/Meta/H1

## Gangnam — Micro P0
- remove English residual `Suggested Timing`

No other Batch 2 page needs P0 structural work.

---

# 10. P1 Action List

## Insadong
- top answer
- `何時間` surface
- historic-cluster internal links

## Gangnam
- practical/entity source layer
- current checks
- K-Beauty/Stay links

## Itaewon
- Title/H1 `観光` alignment
- current check cues
- internal link strengthening

## Cross-page
- answer time earlier
- maintain exact place-name/readings
- use current-source cues for volatile business/event facts

---

# 11. P2 Action List

## Jamsil
No current structural need.

Across Batch 2:
- orientation maps only if they materially improve geography
- FAQ additions only as source-level decisions
- detail pages remain backlog until independent intent is verified

---

# 12. New Page Backlog

| Candidate | Why it surfaced | Gate |
|---|---|---|
| 仁寺洞 伝統茶 | clear entity/category intent | unique content depth |
| 仁寺洞 工芸・お土産 | strong shopping sub-intent | avoid thin list |
| 景福宮・北村・仁寺洞 1日コース | strong route intent | existing page overlap |
| COEX 観光 | strong entity | current guide overlap |
| 狎鴎亭・清潭 ショッピング | distinct cluster | freshness |
| 江南 Kビューティー | market intent | K-Beauty hub overlap |
| 孔徳・麻浦 グルメ | strongest Gongdeok/Mapo candidate | cannibalization with current guide |
| 孔徳市場 グルメ | specific entity intent | content depth |
| 麻浦 豚カルビ | food-detail intent | Taste Korea architecture |
| 孔徳 空港アクセス | strong utility intent | AREX/stay overlap |
| 梨泰院 ハラールフード | strong specific intent | current verification burden |
| 漢南洞 観光・ショッピング | distinct modern intent | Itaewon guide overlap |
| 梨泰院 ナイトライフ | strong intent | nightlife Stay page relation |
| 解放村・経理団 夜 | niche route intent | search demand confirmation |

No page is approved for production by this report.

---

# 13. Final Recommendation

## 13.1 Insadong
**P1**

Current page is already structurally strong. Improve time/search extraction rather than rewriting.

## 13.2 Gangnam
**P1 + P0 micro-fix**

The current strategy is better than list-based competitors because it respects Gangnam’s geography. Fix the English residual and preserve the two-route model.

## 13.3 Jamsil
**PASS**

Protect the page. It already exceeds competitor structures in decision support, event logic, family planning and route discipline.

## 13.4 Gongdeok / Mapo
**P0 Market Intent / Page-Role Review**

This is the most important result of Batch 2.

Do not simply insert `観光`.

Decide whether the Japanese representative intent should be:
- broad practical area guide
or
- food-led area guide with airport/stay support.

## 13.5 Itaewon
**P1**

Body is strong. Make `梨泰院 観光` more explicit in search-facing surfaces and maintain current halal/Hannam/nightlife verification.

---

# 14. Source Register

## Korea Inside Japanese source
- `ja/insadong-travel-guide.html`
- `ja/gangnam-travel-guide.html`
- `ja/jamsil-travel-guide.html`
- `ja/gongdeok-mapo-seoul-guide.html`
- `ja/itaewon-travel-guide.html`

## Insadong benchmarks
- Korea Visit Guide  
  https://www.koreavisitguide.com/ja/neighborhoods/jongno-insadong-travel-guide
- Koro Journal  
  https://www.koreabycar.com/ja/journal/insadong-seoul-guide
- Inside Seoul  
  https://insideseoul.app/ja/guides/insadong-essential-guide
- VISITKOREA / official entity references  
  https://japanese.visitkorea.or.kr/

## Gangnam benchmarks
- K-ELITE  
  https://k-elitevvip.com/ja/info/blog/gangnam-sightseeing-guide
- Traveloka Japan  
  https://www.traveloka.com/ja-jp/explore/culinary/gangnam_tourism/592909
- Gangnam official Japanese Hallyu guide  
  https://visitgangnam.net/images/guidebooks/2026-hallyu-guide-ja.pdf

## Jamsil benchmarks
- Korea Visit Guide  
  https://koreavisitguide.vercel.app/ja/neighborhoods/jamsil-lotte-world-tower-guide
- K Village MODULY  
  https://kasioda.com/guide/jamsilkorea/
- HaniSeoul  
  https://www.haniseoul.com/ja/travels/korea/lotte-world-mall-guide-for-foreigners
- VISIT SEOUL / VISITKOREA official references

## Gongdeok / Mapo benchmarks
- Travel.jp  
  https://www.travel.co.jp/guide/article/36816/
- Minfor  
  https://minfor.jp/tourism/1394/
- NOL World food-street reference  
  https://world.nol.com/ja/content/articles/4ed8f3c5-e771-4043-b35d-2b071ec83ca7
- VISITKOREA Mapo pancake-alley / food references
- Konest Gongdeok/Mapo entity pages

## Itaewon benchmarks
- Korea Visit Guide  
  https://www.koreavisitguide.com/ja/neighborhoods/itaewon-halal-food-nightlife-guide
- IVisitKorea  
  https://www.ivisitkorea.com/ja/things-to-do-in-itaewon/
- Now Yeogi  
  https://nowyeogi.com/ja/posts/itaewon-guide-2026-09/
- VISITKOREA / Visit Seoul official references

---

# 15. Batch Close

- Japanese pages analyzed: **5 / 5**
- Current Korea Inside Japanese source checked: **5 / 5**
- Japanese SERP intent review: **5 / 5**
- Direct competitor/editorial comparison: **5 / 5**
- 1:1 SEO GAP tables: **5 / 5**
- Internal-link analysis: **5 / 5**
- GEO / AI Search review: **5 / 5**
- New Page Candidate backlog: **updated**
- HTML changes: **0**
- Git changes: **0**
- Production changes: **0**

## Final Batch 2 Classification

1. Insadong — **P1**
2. Gangnam — **P1 + P0 micro-fix**
3. Jamsil — **PASS**
4. Gongdeok / Mapo — **P0 Market Intent / Page-Role Review**
5. Itaewon — **P1**

**Batch 2 research: COMPLETE**
