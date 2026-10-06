# Korea Inside — Japanese SEO Competitive Restructuring Report
## Batch 8 — 5 Pages

**Date:** 2026-10-04  
**Status:** RESEARCH COMPLETE — SEO RESTRUCTURING REVIEW / NO IMPLEMENTATION  
**Language:** Japanese  
**Scope:** Existing Korea Inside Japanese 58-page program — Batch 8 / exactly 5 pages  
**Research basis:** Current Korea Inside Japanese source + current Japanese-language SERP sampling + direct Japanese competitor/editorial page review + official telecom/airport/immigration sources where time-sensitive facts mattered  
**Implementation:** HTML 0 / Git 0 / Production 0  

### Batch 8 Pages
1. `ja/esim.html`
2. `ja/best-esim-for-korea.html`
3. `ja/korea-esim-with-phone-number.html`
4. `ja/airport.html`
5. `ja/arrival.html`

> **SERP caution:** Search-result order varies by time, location, device and personalization. “Direct Editorial Benchmark #1/#2” identifies useful current Japanese comparison pages, not absolute Google Japan ranking positions.

---

# 0. Executive Summary

Batch 8 contains two different problem families:

1. **Connectivity decision**
   - eSIM basics
   - best eSIM comparison
   - Korean phone-number eSIM

2. **Arrival decision**
   - after reaching the public arrivals hall
   - immigration / baggage / customs flow before reaching the public hall

Korea Inside is strongest when it separates these problems instead of writing one giant “Korea arrival guide.”

## Final Batch 8 Classification

| Page | Final Status | Main Finding |
|---|---|---|
| eSIM Guide | **PASS** | Exact `韓国 eSIM 使い方` intent, plus SIM/roaming comparison, setup and troubleshooting. Strong role separation from the product-comparison page. |
| Best eSIM for Korea | **P0 — Japan Market Intent / Product Set Review** | Title/structure are strong, but current Japanese SERP compares a materially different provider/product set from KI’s Ubigi/Saily/Airalo focus. Product selection should be reviewed for Japan before SEO implementation. |
| Korea eSIM with Phone Number | **P1 — Freshness / Trust** | Search intent and structure are excellent. Maintain a carrier-by-carrier current matrix for 010 number, receive/send calls/SMS, airport verification and identity-verification limitations. |
| Airport / First 30 Minutes | **PASS** | Strong neutral post-arrivals-hall decision page. Competitors often push SIM/T-money/transport in a fixed order; KI better separates what is actually necessary for the first trip leg. |
| Arrival Guide | **P1 — Search-facing + Regulatory Freshness** | Flow is strong and correct. Strengthen `e-Arrival Card / K-ETA / 入国審査` visibility and maintain the 2026 official regulatory state without hard-coding stale future rules. |

The largest strategic finding is **Best eSIM for Korea**.

Japanese search intent is not merely a translated version of English-language global eSIM shopping. Current Japanese results heavily compare:
- Japan-facing sellers
- Korean carrier-backed products
- 010-number availability
- unlimited-data conditions
- Japanese support
- purchase channel
- short-trip price

Korea Inside’s current Ubigi / Saily / Airalo comparison may still be valid editorially, but it is **not sufficiently representative of the Japanese market without a deliberate market-specific product-selection review**.

---

# 1. Current Korea Inside Japanese Source Baseline

## 1.1 eSIM Guide

- File: `ja/esim.html`
- SHA: `e7b7010ca4de08e468ed044adcdaf3263677adb7`
- Title: `韓国でeSIMを使う方法 | Korea Inside`
- Meta: `韓国旅行向けのデータeSIM、韓国電話番号付きeSIM、物理SIM、海外ローミングを比較。対応端末、本人確認の制限、よくある接続トラブル、選び方を解説します。`
- H1: `韓国でeSIMを使う方法`

Core current structure:
- simplest configuration
- decide what is needed
- device compatibility
- keep home-country number
- compare eSIM / phone-number eSIM / physical SIM / roaming
- data use
- group use
- link to actual eSIM comparison
- whether Korean number is needed
- install/activate
- first five minutes after arrival
- connection troubleshooting
- common mistakes
- post-trip cleanup
- FAQ

## 1.2 Best eSIM for Korea

- File: `ja/best-esim-for-korea.html`
- SHA: `4c5c88b8669e882d82c4995ddd7e03c2de7b181a`
- Title: `韓国旅行におすすめのeSIM比較：データプランの選び方 | Korea Inside`
- Meta: `韓国旅行向けeSIMを、データ容量、有効期間、テザリング、開通方法、ネットワーク情報、返金条件で比較。韓国電話番号付きプランが必要なケースも解説します。`
- H1: `韓国旅行におすすめのeSIMを比較`

Current provider focus:
- Ubigi
- Saily
- Airalo

Current comparison criteria:
- fixed vs unlimited vs regional
- actual high-speed allowance
- validity
- activation
- tethering
- top-up
- support/refund/reissue
- conditions
- user type

## 1.3 Phone-Number eSIM

- File: `ja/korea-esim-with-phone-number.html`
- SHA: `ea9fa2834380d07153baadee28205ace7fc74880`
- Title: `韓国電話番号付きeSIM比較：SKT・KT・LG U+ | Korea Inside`
- Meta: `SK Telecom、KT、LG U+の旅行者向けeSIMを、韓国010番号、通話、SMS、開通方法、本人確認の制限で比較。データ専用eSIMで十分なケースも解説します。`
- H1: `韓国電話番号付きeSIMを比較`

Core:
- decide what the number is for
- receiving vs sending
- what 010 can do
- data-only vs data+voice
- online vs airport counter
- carrier comparison
- when number actually works
- SMS code vs Korean identity verification
- passport conditions
- SKT / KT / LG U+
- mistakes
- data-only alternative
- FAQ

## 1.4 Airport — Public Arrival Hall

- File: `ja/airport.html`
- SHA: `3b0af98ec96dc119414630fc01d01a6ef3c6db91`
- Title: `仁川空港 到着後ガイド：到着ロビーで最初の30分にやること | Korea Inside`
- Meta: `仁川空港到着後、入国審査と税関を終えて到着ロビーに出たら、通信、宿泊先の住所、支払い手段、空港からの移動方法を確認します。`
- H1: `仁川空港の到着ロビーで最初の30分にやること`

Core:
- data connection
- save hotel destination in locally understandable form
- backup payment
- look at full route, not first train/bus only
- terminal map
- redirect to Arrival Guide if still before baggage/customs
- AREX
- bus
- taxi
- private transfer
- rental car
- save practical data before leaving
- troubleshoot data/card/transport/deep-night/address
- FAQ

## 1.5 Arrival — Immigration to Public Hall

- File: `ja/arrival.html`
- SHA: `787f6c901e907213ffcb871192dab46dbd5ae5d5`
- Title: `仁川空港 到着ガイド：入国審査・手荷物受取・税関 | Korea Inside`
- Meta: `仁川空港の到着ガイド。利用ターミナルの確認、入国審査、K-ETAと電子入国申告書（e-Arrival Card）、手荷物受取、税関、一般到着ロビーまでの流れをまとめています。`
- H1: `仁川空港に到着したら何をする？`

Core:
- verify actual terminal
- entering Korea vs transferring
- arrival signs
- immigration
- baggage claim
- customs
- public arrival hall
- T1 vs T2
- FAQ
- official sources

---

# 2. PAGE 1 — eSIM Guide

## 2.1 Search Intent

Primary Japanese intent:

> **韓国旅行でeSIMをどう使う？ そもそもeSIM・SIM・ローミングのどれが自分に合う？**

This is a **how-to / architecture** query.

It is different from:
- “which eSIM company is best?”
- “which phone-number eSIM should I buy?”

The current URL separation is correct.

## 2.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `韓国 eSIM 使い方`

Supporting:
- `韓国 eSIM`
- `韓国旅行 eSIM`
- `韓国 eSIM 設定`
- `韓国 eSIM 開通`
- `韓国 eSIM いつ設定`
- `韓国 eSIM 電話番号`
- `韓国 eSIM SIM どっち`
- `韓国 eSIM ローミング 比較`
- `韓国 eSIM 繋がらない`
- `韓国 eSIM 日本の番号`
- `韓国 eSIM 削除`

## 2.3 SERP Observation

Current Japanese results strongly cluster around:
- recommendation/ranking
- price
- unlimited data
- phone number
- setup
- roaming comparison
- physical SIM comparison

This creates a risk:
a generic eSIM article can become a duplicated product-comparison page.

Korea Inside correctly avoids that by using this URL for:
- architecture
- setup
- troubleshooting
- “do I need a phone number?”

## 2.4 Direct Editorial Benchmark #1 — tabicle

URL:
`https://tabicle.com/korea-esim/`

Observed title:
`韓国旅行のeSIMおすすめ〖2026〗料金の目安・データ量・設定の注意点`

Observed structure:
1. short-trip recommendations
2. eSIM vs rental Wi-Fi vs SIM vs carrier roaming
3. data-use guidance
4. provider examples
5. pricing
6. setup cautions
7. current-condition disclaimer

### Strength
- broad Japanese search intent
- communication-method comparison
- setup and data-use guidance
- very understandable first answer

### Weakness vs KI
- recommendation/product layer overlaps the basic guide
- more price/provider driven
- less phone-number / identity-verification nuance
- less troubleshooting depth

## 2.5 Direct Editorial Benchmark #2 — eSIM韓国ナビ

URL:
`https://esim-kankoku.com/korea-esim-vs-localsim/`

Observed title:
`韓国旅行の通信はeSIM・ローミング・現地SIMどれが安い？徹底比較`

Observed structure:
- eSIM
- Japanese-carrier roaming
- local SIM
- 3-day/5-day cost
- which type fits which traveler
- phone-number cases
- decision checklist

### Strength
- exact comparison intent
- easy “which method?” answer
- Japanese carrier context

### Weakness vs KI
- cost-centric
- local-number and verification subtleties are lighter
- less installation/troubleshooting logic

## 2.6 Official Telecom Signal — LG U+

Current LG U+ official Japanese content confirms high-value facts that support the KI page structure:
- Data Only vs Data+Voice are materially different
- Data+Voice has a 010 number and supports calls/SMS
- some SMS verification can work, but bank/government identity verification does not
- eSIM deletion can prevent recovery
- device/eSIM compatibility matters
- installation and first data use/activation conditions matter

These are exactly the kinds of facts that belong in an operational guide.

## 2.7 Korea Inside Actual Structure

KI already provides:
- eSIM / phone-number eSIM / SIM / roaming comparison
- device compatibility
- dual-SIM/home-number retention
- data-usage scenarios
- group decision
- phone-number need
- install vs activate distinction
- arrival check
- troubleshooting
- “do not delete” warning
- FAQ
- next-page links

This is stronger than typical recommendation-first pages for the **how-to intent**.

## 2.8 1:1 SEO GAP

| Element | Competitors | Korea Inside | Judgment | Priority |
|---|---|---|---|---|
| Title | `おすすめ / 設定 / 比較` | exact `使う方法` | correct for role | PASS |
| Meta | price/data/setup | architecture + limitations + troubleshooting | KI stronger for how-to |
| H1 | recommendation or setup | exact how-to | PASS |
| Intro / Quick Answer | provider recommendation | simple setup architecture | differentiated |
| H2 | provider/data/price | need/device/method/install/trouble | KI stronger |
| H3 | provider specifics | usage scenarios/mistakes | strong |
| Keyword language | `eSIM おすすめ` dominant | `eSIM 使い方` | correct separate intent |
| Intent | mixed | cleanly scoped | KI advantage |
| Depth | product depth | operational depth | complementary |
| Practical info | prices/setup | setup/troubleshooting/phone-number | KI stronger |
| Decision support | medium | high | KEEP |
| Freshness | product prices | product-neutral | KI advantage |
| Entities | providers | method types | correct |
| Internal links | shopping pages | Best eSIM + phone-number + apps/maps | KI advantage |
| FAQ | yes | 8 | strong |
| CTA | affiliate-first | guide-first | KEEP |
| Media | screenshots often useful | optional | P2 |
| Trust/source | commercial | carrier facts can be official | KI advantage |
| Commercial usefulness | high | routes to comparison page | strong |

## 2.9 KEEP

- current title/meta/H1
- method comparison
- device compatibility
- keep home number
- data-use guidance
- group decision
- phone-number decision
- install vs activate
- arrival test
- troubleshooting
- common mistakes
- do-not-delete warning
- FAQ

## 2.10 P0 / P1 / P2

### P0
None.

### P1
No structural change required.

Maintenance:
- phone-number/SMS/identity facts
- current carrier activation policies

### P2
Device-specific setup screenshots only if maintenance burden is acceptable.

## 2.11 SEO Surface Gap

No urgent rewrite.

The current page is correctly differentiated from the Best eSIM page.

## 2.12 Content Gap

No major content gap.

Potential future micro-module:
- “Japan carrier roaming may already be enough” as a decision branch, if current Japanese-carrier facts are verified separately.

## 2.13 Japan-Specific Market Gap

Japanese users have stronger awareness of:
- ahamo / Rakuten / carrier roaming
- Japanese-language support

These are valid market considerations, but should only be added with current carrier terms and not as assumptions.

## 2.14 Competitor-Only Content

- live 3/5/7-day prices
- ranking table
- Japanese-carrier roaming fee table

Those belong in current comparison content, not the evergreen setup hub.

## 2.15 New Page Candidate

Potential:
`韓国旅行 eSIM vs ローミング`

Only if Search Console shows strong independent demand and the current guide becomes overloaded.

## 2.16 GEO / AI Search

The page should remain the canonical answer for:
- eSIM vs SIM vs roaming
- install timing
- activation timing
- keep Japanese number
- Korean phone number need
- troubleshooting
- delete/reinstall risk

## 2.17 Internal Links

Outbound:
- `/ja/best-esim-for-korea.html`
- `/ja/korea-esim-with-phone-number.html`
- `/ja/apps.html`
- `/ja/maps.html`
- `/ja/airport.html`
- `/ja/arrival.html`

Inbound:
- homepage
- arrival/airport
- best eSIM
- phone-number eSIM
- apps/maps/checklist

## 2.18 Final Primary Keyword

**`韓国 eSIM 使い方`**

## 2.19 Final Status

# **PASS**

---

# 3. PAGE 2 — Best eSIM for Korea

## 3.1 Search Intent

Primary Japanese intent:

> **韓国旅行で使うeSIMはどれを買えばいい？**

Unlike the general eSIM guide, this is a **shopping/comparison query**.

Japanese users expect:
- actual providers/products
- 3/5/7-day price
- data allowance
- “unlimited” definition
- local network
- phone number
- tethering
- setup
- Japanese support
- refund
- purchase channel

## 3.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `韓国 eSIM おすすめ`

Supporting:
- `韓国 eSIM 比較`
- `韓国 eSIM おすすめ 2026`
- `韓国 eSIM 安い`
- `韓国 eSIM 無制限`
- `韓国 eSIM 電話番号付き`
- `韓国 eSIM SKT`
- `韓国 eSIM LG U+`
- `韓国 eSIM KT`
- `韓国 eSIM 日本語`
- `韓国 eSIM 3日`
- `韓国 eSIM 5日`
- `韓国 eSIM 7日`

## 3.3 SERP Observation

Current Japanese SERP is strongly product-specific.

Visible 2026 comparison sets include:
- KLP SIM
- コネスト SKT / LG U+
- World eSIM
- eSIM square
- KKday
- trifa
- Glocal eSIM
- 地球の歩き方eSIM
- Holafly
- Airalo
- Saily
- Korean-carrier-direct products

This provider field is materially different from the current Korea Inside Japanese page’s main comparison set:
- Ubigi
- Saily
- Airalo

That is not a small keyword gap.

It is a **market-product-set gap**.

## 3.4 Direct Editorial Benchmark #1 — mybest

URL:
`https://my-best.com/23488`

Observed title:
`〖徹底比較〗韓国で使えるeSIM・SIMカードのおすすめ人気ランキング〖2026年9月〗`

Current comparison:
- 15 products
- 3/4/7-day pricing
- actual speed tests
- data restriction tests
- setup ease
- network
- phone number
- support
- tethering
- 5G
- provider ranking

### Strength
- exceptional Japanese shopping-intent match
- current date
- real Japanese purchase environment
- measurable comparison
- large provider coverage

### Weakness vs KI
- ranking methodology defines “best” narrowly
- some test results are from 2025 and environment-specific
- product abundance can overwhelm actual traveler needs
- more shopping tool than travel decision guide

## 3.5 Direct Editorial Benchmark #2 — 価格.com

URL:
`https://kakaku.com/mobile_data/world-wifi/world-esim/search.html?we_area=korea`

Observed current state:
- 2026-10-04 current update
- 459 Korea eSIM plans
- filters:
  - days
  - total data
  - unlimited
- sort:
  - popularity
  - lowest price

### Strength
- near-real-time shopping inventory
- unbeatable breadth
- price/filter utility

### Weakness vs KI
- not editorial guidance
- no itinerary/activation/identity context
- cannot explain which product conditions matter most for a traveler

## 3.6 Supporting Benchmark — Saily

Current Saily Japanese comparison page explicitly compares:
- Saily
- Holafly
- Airalo
- Korean-carrier-backed eSIM

and includes:
- plan type
- setup
- phone-number option
- security features
- price positioning

This illustrates the competitive expectation:
even a provider-owned page acknowledges a broader product field.

## 3.7 Korea Inside Actual Structure

Current KI:
- starts from smartphone use
- plan type
- comparison criteria
- unlimited-definition warning
- provider comparison
- price is not the only criterion
- current plan differences
- setup failures
- traveler-type matching
- purchase checklist
- common mistakes
- when travel eSIM is not suitable
- Ubigi / Saily / Airalo detail
- FAQ

The **decision architecture is good**.

The problem is **representativeness of the Japanese product set**.

## 3.8 1:1 SEO GAP

| Element | Japanese competitors | Korea Inside | Gap / Action | Priority |
|---|---|---|---|---|
| Title | exact `おすすめ / 比較 / ranking` | exact | PASS |
| Meta | provider/price/network | decision criteria | strong |
| H1 | exact | exact | PASS |
| Intro / Quick Answer | winner/shortlist | usage-first | editorially strong |
| H2 | provider/ranking | conditions + providers | structure strong |
| H3 | Japan-market products | Ubigi/Saily/Airalo | product-set mismatch | **P0** |
| Keyword language | exact | exact | PASS |
| Intent | purchase comparison | purchase comparison | exact |
| Depth | very broad inventory | deep decision framework | complementary |
| Practical info | price/speed/network | conditions/refund/activation | KI strong |
| Decision support | shopping-oriented | traveler-oriented | KI advantage |
| Freshness | frequent updates | current plan section exists | high maintenance |
| Entities | many Japanese-market products | 3 global providers | **major gap** |
| Internal links | purchase-only | setup/phone-number guides | KI advantage |
| FAQ | yes | 8 | strong |
| CTA | many stores | three provider links | market coverage issue |
| Media/table | rich product tables | likely simpler | P1 after product review |
| Trust/source | tests/official sources | needs current official plan checks | P0/P1 |
| Commercial usefulness | very high | currently narrower for JP market | **P0** |

## 3.9 KEEP

- current title/meta/H1
- usage-first approach
- fixed/unlimited/regional distinction
- “unlimited” skepticism
- validity
- tethering
- top-up
- refund/support
- activation
- purchase checklist
- mistakes
- phone-number branch
- FAQ

## 3.10 P0 / P1 / P2

### P0 — Japan Market Product Set Review

Before implementation, decide which provider universe the Japanese page should compare.

Questions:
1. Should Japanese buyers see Japan-facing sellers?
2. Should local SKT/KT/LG U+ direct products appear?
3. Should global providers remain?
4. Which options are actually purchasable/supportable from Japan?
5. What is the affiliate/business relationship?
6. Which provider set can be maintained monthly?

Do **not** simply replace Ubigi/Saily/Airalo automatically.

### P1
After product-set approval:
- comparison matrix
- 3/5/7-day or typical-trip scenario
- Japanese purchase/support
- current network/phone-number/tethering facts
- checked date

### P2
Price filter / calculator only if maintenance is automatable.

## 3.11 SEO Surface Recommendation

Current Title/H1 can remain.

The urgent issue is **content/entity selection**, not wording.

Potential first answer format after product review:

- easiest data-only
- lowest-friction app setup
- heavy-data/unlimited
- Korean number needed
- local-carrier support
- Japan-facing purchase/support

## 3.12 Content Gap

The main content gap is:
**Japanese market product coverage.**

This is more important than adding words.

## 3.13 Japan-Specific Market Gap

Japanese buyers encounter:
- Japanese-language checkout/support
- Japan-based sellers
- local-carrier products sold through Japanese channels
- short 2/3/4/7-day travel durations
- phone-number bundled plans

A translated global-provider shortlist may underperform even if editorial logic is good.

## 3.14 Competitor-Only Content

- 15-product rankings
- 459-plan filter inventory
- speed-score league tables
- live cheapest-price sorting

KI should not copy scale it cannot maintain.

## 3.15 New Page Candidate

No new page required.

The existing Best eSIM URL is the correct place for this query if the product set is corrected.

## 3.16 GEO / AI Search

The page should answer:
- best data-only option
- best heavy-data option
- best for tethering
- best if Korean number is needed
- what “unlimited” really means
- which conditions matter more than price
- how to choose by trip length

## 3.17 Internal Links

Outbound:
- eSIM Guide
- Phone-Number eSIM
- Apps
- Maps
- Checklist
- Arrival/Airport

Inbound:
- Home
- eSIM Guide
- Checklist
- Airport/Arrival

## 3.18 Final Primary Keyword

**`韓国 eSIM おすすめ`**

## 3.19 Final Status

# **P0 — Japan Market Intent / Product Set Review**

---

# 4. PAGE 3 — Korea eSIM with Phone Number

## 4.1 Search Intent

Primary Japanese intent:

> **韓国の010番号付きeSIMは必要？ 通話・SMS・SMS認証・本人認証はどこまでできる？**

This is a distinct, high-confusion query.

## 4.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `韓国 eSIM 電話番号付き`

Supporting:
- `韓国 eSIM 010`
- `韓国 eSIM SMS`
- `韓国 eSIM 通話`
- `韓国 eSIM SMS認証`
- `韓国 eSIM 本人認証`
- `韓国 eSIM SKT`
- `韓国 eSIM KT`
- `韓国 eSIM LG U+`
- `韓国 電話番号 旅行者`
- `韓国 eSIM 電話番号 必要`

## 4.3 SERP Observation

Current Japanese competitor content correctly separates:
- number exists
- call receiving
- call sending
- SMS receiving
- SMS sending
- SMS code
- Korean resident identity verification

That distinction is essential.

The current KI page already uses almost the exact same decision model.

## 4.4 Direct Editorial Benchmark #1 — 韓国eSIM比較ナビ

URL:
`https://kankoku-esim.com/phone-number-included/`

Observed title:
`韓国eSIMの電話番号付きを比較！010番号・通話・SMSの違いを解説`

Observed first decision:
- receive-focused
- send/call-focused
- identity-verification purpose

Observed structure:
- 010 number
- call/SMS capability
- verification
- provider comparison
- use case

### Strength
- excellent exact Japanese query match
- separates capability types
- direct current purchase comparison

### Weakness vs KI
- reseller/product-centric
- identity-verification explanation can depend on exact product conditions
- less carrier-direct official architecture

## 4.5 Direct Editorial Benchmark #2 — ちょぴログ

URL:
`https://www.rinablog.org/korea-esim-phone-number/`

Observed title:
`韓国eSIMに電話番号は必要？SKTとLG U+の電話番号付きプランを比較`

Observed structure:
- data-only limitations
- whether Korean number is needed
- SKT vs LG U+
- price/plan conditions
- recommendation by use case

### Strength
- direct Japanese question
- narrow useful comparison
- product decision clarity

### Weakness vs KI
- narrower carrier coverage
- less detailed identity-verification separation
- less online-vs-airport activation logic

## 4.6 Current Official Carrier Facts

### KT official Japanese page
Current KT traveler eSIM page states:
- 010 number is provided
- data-only and data/call/SMS products differ
- voice/SMS receiving can require entry/identity check
- sending requires relevant voice product / charge
- airport roaming-center verification is relevant
- eSIM QR can be one-use and deletion can be unrecoverable
- KT tourist SIM cannot perform Korean resident personal authentication

### LG U+ official Japanese content
Current LG U+ guidance distinguishes:
- Data Only
- Data+Voice
- 010 number
- calls/SMS
- SMS verification
- bank/government identity verification not supported

### SK Telecom official site
Current SK Telecom product navigation includes:
- LTE/5G eSIM Data, Call, SMS
- data-only eSIM
- traveler-specific product options

These confirm the KI page’s three-carrier structure remains appropriate, but the exact matrix is volatile.

## 4.7 Korea Inside Actual Structure

KI already:
- asks why the number is needed
- separates receiving vs sending
- separates 010 number vs identity verification
- compares data-only vs voice
- online vs airport
- compares SKT / KT / LG U+
- explains passport conditions
- includes mistakes
- routes data-only users away from unnecessary complexity
- 8 FAQ

This is excellent.

## 4.8 1:1 SEO GAP

| Element | Competitors | Korea Inside | Judgment | Priority |
|---|---|---|---|---|
| Title | exact 010/phone-number comparison | exact + 3 carriers | excellent |
| Meta | call/SMS | call/SMS/activation/identity | KI stronger |
| H1 | exact | exact | PASS |
| Intro / Quick Answer | receive/send/auth | “what do you need it for?” | excellent |
| H2 | capability split | same + carrier/airport | KI stronger |
| H3 | products/providers | carrier differences | strong |
| Keyword language | exact | exact | PASS |
| Intent | exact | exact | PASS |
| Depth | product matrix | decision + limitations | KI stronger |
| Practical info | plans/prices | activation/passport/identity | KI stronger |
| Decision support | high | exceptional | KEEP |
| Freshness | very high | carrier policy changes | **P1** |
| Entities | providers | SKT/KT/LG U+ | strong |
| Internal links | product pages | eSIM basics/best eSIM | KI advantage |
| FAQ | yes | 8 | strong |
| CTA | reseller-heavy | neutral carrier comparison | trust advantage |
| Trust/source | mixed | official carrier pages available | strengthen |
| Commercial usefulness | high | high | strong |

## 4.9 KEEP

- current title/meta/H1
- need-first logic
- receive vs send
- 010 vs Korean identity verification
- data-only vs voice
- online vs airport
- SKT/KT/LG U+
- passport/entry confirmation
- mistakes
- data-only alternative
- FAQ

## 4.10 P0 / P1 / P2

### P0
None.

### P1 — Freshness / Trust
Maintain a current matrix:
- 010 number
- incoming voice
- outgoing voice
- incoming SMS
- outgoing SMS
- SMS code
- resident identity verification
- online activation
- airport verification
- passport purchase limit
- QR/reissue/deletion conditions

Use official carrier sources.

### P2
No structural expansion needed.

## 4.11 SEO Surface Gap

No urgent title/H1 rewrite.

The current title is more complete than many competitors.

## 4.12 Content Gap

No strategic gap.

The risk is **fact drift**, not missing explanation.

## 4.13 Japan-Specific Market Gap

Japanese travelers often buy phone-number eSIM for:
- restaurants
- beauty appointments
- delivery/taxi apps
- SMS codes

The page should continue distinguishing simple contact use from resident identity systems.

## 4.14 Competitor-Only Content

- reseller price tables
- temporary promo codes
- seller-specific “Japanese support”
- ranking by cheapest package

Not core.

## 4.15 New Page Candidate

None.

The current URL correctly owns this problem.

## 4.16 GEO / AI Search

Answer:
- do I need Korean number?
- does it start 010?
- can I receive/send calls?
- can I receive/send SMS?
- can I receive an app code?
- does that equal Korean identity verification?
- which carrier requires airport confirmation?
- when data-only is enough

## 4.17 Internal Links

Outbound:
- eSIM Guide
- Best eSIM
- Apps
- K-Beauty where appointment/contact use is relevant
- Arrival/Airport

Inbound:
- eSIM Guide
- Best eSIM
- Apps / K-Beauty practical sections

## 4.18 Final Primary Keyword

**`韓国 eSIM 電話番号付き`**

## 4.19 Final Status

# **P1 — Freshness / Trust**

---

# 5. PAGE 4 — Airport: First 30 Minutes in Public Arrival Hall

## 5.1 Page Role

This page begins **after**:
- immigration
- baggage claim
- customs

That separation is important.

Correct role:

> **Public arrival hall → communication / destination / payment / ground transport decision**

It should not repeat the Arrival Guide’s immigration content.

## 5.2 Search Intent

Primary Japanese intent:

**`仁川空港 到着後 やること`**

Supporting:
- `仁川空港 到着後`
- `仁川空港 到着ロビー`
- `仁川空港 eSIM`
- `仁川空港 T-money`
- `仁川空港 AREX`
- `仁川空港 リムジンバス`
- `仁川空港 タクシー`
- `仁川空港 深夜 到着`
- `仁川空港 両替`
- `仁川空港 ホテル 移動`

## 5.3 SERP Observation

Competitor flow usually becomes:

1. SIM/eSIM
2. T-money
3. exchange cash
4. AREX/bus/taxi

This is useful but often too fixed.

Korea Inside’s stronger distinction:
- if transport does not need T-money immediately, buying it does not have to be first
- first confirm data
- save destination
- have payment backup
- confirm the **full route to the hotel**

That is a more robust arrival protocol.

## 5.4 Direct Editorial Benchmark #1 — LG U+ official travel blog

URL:
`https://www.lguplus.com/korea-sim/jpn/pc/support/blog/incheon_airport_arrival_guide`

Observed title:
`仁川空港到着後にやるべき4つのこと, 入国初日ガイド`

Updated:
- 2026-10-01

Observed structure:
1. connect data at roaming center/eSIM
2. buy T-money
3. exchange money
4. move by AREX/bus
5. terminal counter details
6. current price/location facts

### Strength
- highly current
- exact search query
- official telecom-counter information
- actionable terminal details

### Weakness vs KI
- carrier-owned content
- pushes LG U+ product
- fixed order can imply every traveler needs the same purchases
- less backup-payment / destination-save logic
- less “full hotel route” decision

## 5.5 Direct Editorial Benchmark #2 — airportguide.jp

URL:
`https://www.airportguide.jp/icn-airport-guide/`

Observed structure:
- terminal basics
- arrival
- communications
- T-money/payment
- currency
- AREX/bus/taxi
- departure
- facilities

Arrival section explicitly lists:
- SIM/eSIM/Wi-Fi
- T-money
- small cash exchange
- city transfer

### Strength
- independent airport-guide format
- task checklist
- broad airport context
- useful Japanese labels

### Weakness vs KI
- more airport encyclopedia than decision guide
- less troubleshooting
- less hotel-address/local-language preparation
- less conditional order

## 5.6 Korea Inside Actual Structure

Current KI:
- first 30 minutes only
- data test with airport Wi-Fi off
- save hotel destination in Korean/local format
- backup payment
- full route, not first segment
- terminal map
- clear redirect to Arrival Guide if pre-customs
- transport options
- rental car
- save important info
- troubleshoot:
  - eSIM
  - foreign card
  - transport location
  - late arrival
  - driver cannot identify hotel
- 6 FAQ

This is strongly differentiated.

## 5.7 1:1 SEO GAP

| Element | Competitors | Korea Inside | Judgment | Priority |
|---|---|---|---|---|
| Title | exact `到着後やること` | exact + first 30 min | excellent |
| Meta | SIM/Tmoney/transport | data/address/payment/transport | KI stronger |
| H1 | arrival checklist | exact | PASS |
| Intro / Quick Answer | fixed sequence | conditional first tasks | KI stronger |
| H2 | purchase/service tasks | decision + troubleshooting | KI stronger |
| H3 | counters/fares | practical failures | complementary |
| Keyword language | exact | exact | PASS |
| Intent | first-hour checklist | exact | PASS |
| Depth | airport services | decision/trouble resolution | KI stronger |
| Practical info | counters/prices | neutral route logic | strong |
| Decision support | medium | exceptional | KEEP |
| Freshness | service counters/fares | official links needed | maintenance |
| Entities | services | AREX/bus/taxi/private | strong |
| Internal links | limited | arrival/esim/payments/transport | KI advantage |
| FAQ | some | 6 | strong |
| CTA | carrier product | neutral | trust advantage |
| Trust/source | official/commercial | official airport references | strong |
| Commercial usefulness | service sales | transfer/service decision | strong |

## 5.8 KEEP

- current title/meta/H1
- page starts after customs
- four first tasks
- mobile-data test
- Korean/local destination info
- payment backup
- full route
- T-money is conditional, not mandatory first purchase
- transport choice
- troubleshooting
- deep-night arrival
- FAQ

## 5.9 P0 / P1 / P2

### P0
None.

### P1
No structural change required.

Maintenance:
- terminal services
- transport counters
- late-night transport
- T-money/payment facts

### P2
Small terminal checklist visual only if maintainable.

## 5.10 SEO Surface Gap

No urgent rewrite.

The “first 30 minutes” framing is distinctive and highly useful.

## 5.11 Content Gap

No major gap.

Do not add immigration flow here; that belongs to Arrival.

## 5.12 Japan-Specific Market Gap

Japanese competitors often present:
- exchange cash
- T-money
as universal first steps.

KI should continue using conditional guidance based on actual first transport and payment needs.

## 5.13 Competitor-Only Content

- carrier booth locations
- carrier promotion
- fixed T-money purchase order
- exact transfer prices

These should be linked/current facts, not the page’s editorial core.

## 5.14 New Page Candidate

None.

Current airport + arrival + transport cluster is sufficient.

## 5.15 GEO / AI Search

Answer:
- first thing after public arrival hall
- test eSIM
- T-money now or later?
- save hotel address
- payment backup
- AREX/bus/taxi
- deep-night arrival
- what to do if card/eSIM/transport fails

## 5.16 Internal Links

Outbound:
- Arrival
- eSIM
- Payments
- T-money
- AREX
- Airport Bus
- Taxi
- Private Transfer
- Rental Car
- Maps

Inbound:
- Home
- Arrival
- Airport Transfer
- eSIM
- T-money/Payments
- Stay/Airport Access pages

## 5.17 Final Primary Keyword

**`仁川空港 到着後 やること`**

## 5.18 Final Status

# **PASS**

---

# 6. PAGE 5 — Arrival: Immigration, Baggage and Customs

## 6.1 Page Role

Correct role:

> **Aircraft → immigration → baggage → customs → public arrivals hall**

This must remain separate from:
- after-arrival shopping/services
- Seoul transfer
- airport transport comparison

## 6.2 Search Intent

Primary Japanese intent:

**`仁川空港 到着 流れ`**

Supporting:
- `仁川空港 入国審査`
- `韓国 入国 e-Arrival Card`
- `韓国 K-ETA 2026`
- `仁川空港 手荷物受取`
- `仁川空港 税関`
- `仁川空港 ターミナル`
- `仁川空港 T1 T2`
- `仁川空港 乗り継ぎ`
- `韓国 入国 流れ`
- `韓国 電子入国申告書`

## 6.3 SERP Observation

Current Japanese arrival content often combines:
- entry documents
- immigration
- baggage
- customs
- SIM
- T-money
- transfer

Korea Inside’s split architecture is cleaner:
- Arrival = border/baggage flow
- Airport = public-hall first 30 minutes
- Transfer = city transport

The main SEO gap is that the Japanese market now has highly visible **e-Arrival Card / K-ETA** regulatory intent.

## 6.4 Direct Editorial Benchmark #1 — KOREA-LOGUE

URL:
`https://korealogue.net/2026/08/11/incheon-airport-seoul-transport-guide/`

Observed framing:
- `仁川空港に着いたらどうする？`
- aircraft
- immigration
- luggage
- public arrival hall
- AREX/bus/taxi
- NAVER Map
- hotel route

### Strength
- full sequence
- first-time traveler language
- practical city-transfer continuation
- current 2026

### Weakness vs KI
- combines multiple page roles
- transport can overshadow entry flow
- fixed time estimates can vary greatly
- less explicit transfer-passenger branch

## 6.5 Direct Editorial Benchmark #2 — KoreaPlus

URL:
`https://koreaplus-lifes.com/guide/ja/airport`

Observed title:
`仁川空港：到着後の最初の1時間`

Observed structure:
- entry → baggage → customs
- SIM/eSIM + cash
- AREX / bus / taxi
- price snippets
- “check official operations on travel day”

### Strength
- concise
- current-source warning
- price/action scanability

### Weakness vs KI
- mixes pre-public-hall and post-public-hall tasks
- less detailed terminal/transfer separation
- weaker information architecture

## 6.6 Official 2026 Immigration / Airport Facts

### Incheon Airport official Japanese flow
Current official airport guidance shows the arrival process through:
- arrival
- declarations/guidance
- immigration
- baggage claim
- public arrival hall

It also warns travelers to verify the actual terminal because codeshare/airline conditions can change.

### e-Arrival Card
Current official Korean e-government Japanese page states:
- no fee
- can be submitted from 3 days before arrival
- eligibility depends on visa/K-ETA/residence status
- official site is the only official e-Arrival Card site
- fake paid sites exist

### K-ETA in 2026
The official K-ETA notice states:
- the temporary exemption for currently exempt countries/regions is extended through **2026-12-31**
- eligible travelers may still apply voluntarily
- one benefit of valid K-ETA is no separate arrival card submission

Important:
The page should not hard-code that all Japanese passport holders always need or never need K-ETA outside the verified date/state. The rule is time-sensitive.

## 6.7 Korea Inside Actual Structure

Current KI:
- terminal verification
- entering Korea vs transfer
- follow Arrivals
- immigration
- baggage
- customs
- public hall
- T1 vs T2
- FAQ
- official info

Meta already includes:
- K-ETA
- e-Arrival Card

The structure is correct.

## 6.8 1:1 SEO GAP

| Element | Competitors | Korea Inside | Gap / Action | Priority |
|---|---|---|---|---|
| Title | arrival/immigration | exact | PASS |
| Meta | entry docs + flow | K-ETA/e-Arrival + baggage/customs | strong |
| H1 | “what to do after arrival?” | exact | PASS |
| Intro / Quick Answer | full sequence | clear sequence | PASS |
| H2 | often mix transport | terminal/entry/transfer | KI cleaner |
| H3 | immigration/baggage | exact | strong |
| Keyword language | `入国 / e-Arrival` | mostly meta/body | surface could improve | **P1** |
| Intent | combined arrival | tightly scoped arrival | KI advantage |
| Depth | broad first-hour | border-flow depth | strong |
| Practical info | documents/transport | terminal/transfer | strong |
| Decision support | medium | high | KEEP |
| Freshness | regulations | critical | **P1** |
| Entities | T1/T2 | T1/T2 | strong |
| Internal links | mixed | Airport + transport cluster | KI advantage |
| FAQ | common | 6 | strong |
| CTA | sometimes commercial | neutral | trust advantage |
| Trust/source | varies | official immigration/airport | major advantage |
| Commercial usefulness | lower | routes next to Airport/Transfer | correct |

## 6.9 KEEP

- current title/meta/H1
- terminal verification
- entering Korea vs transfer
- immigration → baggage → customs → public hall
- T1/T2 distinction
- codeshare caution
- FAQ
- official-source links
- handoff to Airport / Transfer after public hall

## 6.10 P0 / P1 / P2

### P0
None.

### P1 — Search-facing + Regulatory Freshness
Improve visibility of:
- `電子入国申告書（e-Arrival Card）`
- `K-ETA`
- who must check which rule
- official website only
- current exemption/requirement state

Do this without turning the page into an immigration-law article.

Maintain a visible “checked” layer because rules change.

### P2
No structural expansion required.

## 6.11 SEO Surface Recommendation

Current title can remain.

Potential H2 / Quick Answer module:
`入国前に確認：K-ETAと電子入国申告書（e-Arrival Card）`

Only if the English/source-level architecture approves this section.

Alternative without adding a new structural H2:
surface it in the intro/quick facts.

## 6.12 Content Gap

No body-flow gap.

The only meaningful gap is **search visibility + current regulatory status**.

## 6.13 Japan-Specific Market Gap

Japanese travelers currently search:
- K-ETA exemption
- e-Arrival Card
- electronic arrival form
- terminal
- immigration wait/flow

Korea Inside should answer the rule in a dated official-source manner.

## 6.14 Competitor-Only Content

- fixed immigration wait times
- universal “takes 60 minutes”
- outdated entry-form requirements
- transport fare tables

Not reliable as evergreen flow content.

## 6.15 New Page Candidate

Potential:
**`韓国 e-Arrival Card / K-ETA`**

Only if independent Japanese search demand is strong enough to justify a regulation-specific detail URL.

This would reduce regulatory burden on the Arrival page.

## 6.16 GEO / AI Search

The page should answer:
- T1 or T2?
- entering Korea or transferring?
- immigration before baggage?
- e-Arrival Card / K-ETA relationship
- baggage missing
- customs
- when the public arrival hall begins
- what page to read next

## 6.17 Internal Links

Outbound:
- Airport first 30 minutes
- Airport Transfer
- AREX
- Airport Bus
- Taxi
- eSIM
- Checklist

Inbound:
- Home
- Airport
- Checklist
- Airport Transfer
- eSIM where arrival activation is discussed

## 6.18 Final Primary Keyword

**`仁川空港 到着 流れ`**

## 6.19 Final Status

# **P1 — Search-facing + Regulatory Freshness**

---

# 7. Cross-Page Findings

## 7.1 Product Comparison and How-To Must Stay Separate

The three eSIM pages have correct different roles:

### eSIM Guide
How it works / which communication method

### Best eSIM
Which product/provider

### Phone-Number eSIM
What Korean 010 / calls / SMS / verification actually means

Do not merge them into one page.

## 7.2 Japanese Product SERP Is More Local-Market Specific Than General Localization

This is the largest Batch 8 lesson.

For commercial comparison pages:
- Japanese buyer availability
- Japanese support
- local marketplace
- Japanese pricing
- Korean carrier-backed products

can materially change the competitive set.

This is not just translating keywords.

## 7.3 Telecom Facts Need Capability Matrices

Avoid labels such as:
`電話番号付き`

without specifying:
- incoming voice
- outgoing voice
- incoming SMS
- outgoing SMS
- SMS code
- resident identity verification
- airport verification
- passport requirement

## 7.4 Airport Architecture Should Remain Split

Correct sequence:

> **Arrival Guide**  
> aircraft → immigration → baggage → customs → public hall

then

> **Airport Guide**  
> connection → destination → payment → transport

then

> **Transport Details**  
> AREX / bus / taxi / transfer

This prevents duplication and makes AI/Search extraction cleaner.

## 7.5 Regulatory Content Needs a Dated Fact Layer

K-ETA / e-Arrival rules should never become undated evergreen assertions.

Use:
- official source
- checked date
- current validity period
- recheck prompt

## 7.6 “Official” Is Especially Important for Telecom and Immigration

Competitor blogs can be useful for:
- keyword language
- page structure
- traveler questions

But carrier and immigration facts should come from:
- SKT
- KT
- LG U+
- Incheon Airport
- K-ETA
- e-Arrival Card official systems

---

# 8. Japan SEO Restructuring Rules — Batch 8 Additions

## Rule 41 — Commercial Comparison Pages May Need Market-Specific Product Sets

Localized SEO can fail even with natural Japanese if the compared products are not representative of what Japanese users can actually buy.

## Rule 42 — Capability Must Be More Precise Than Product Label

`電話番号付き` is not enough.

Explain exact functions.

## Rule 43 — Regulatory Facts Require Expiry-Aware Maintenance

For:
- K-ETA
- e-Arrival
- immigration forms
- terminal assignment

attach a current date/source.

## Rule 44 — Arrival Information Architecture Must Follow Physical Sequence

Do not put T-money before immigration if the page begins inside immigration.

Page boundaries should mirror traveler state.

## Rule 45 — Competitor Prices Are Not the Source of Truth

Use competitors to understand:
- what Japanese users compare
- how they phrase the problem

Use official providers for:
- plan conditions
- prices
- activation
- verification

---

# 9. P0 Action List

## Best eSIM for Korea
- Japanese market product-set review
- decide provider universe
- maintain the existing decision architecture
- then rebuild the provider matrix only after approval

No other Batch 8 page needs P0 restructuring.

---

# 10. P1 Action List

## Phone-Number eSIM
- current carrier capability matrix
- official carrier verification
- checked date

## Arrival
- e-Arrival Card / K-ETA search visibility
- regulatory checked date
- official-site warning

## Cross-page
- maintain current plan/entry rules
- keep how-to and product comparison separate

---

# 11. P2 Action List

- eSIM device screenshots if maintainable
- dedicated K-ETA/e-Arrival detail page research
- no Airport Guide structural expansion

---

# 12. New Page Backlog

| Candidate | Why it surfaced | Gate |
|---|---|---|
| 韓国旅行 eSIM vs ローミング | strong Japan carrier context | avoid overlap with eSIM guide |
| 韓国 e-Arrival Card / K-ETA | distinct regulatory intent | high freshness / official-source maintenance |

No candidate is approved for production.

---

# 13. Final Recommendation

## eSIM Guide
**PASS**

Protect the architecture/how-to role.

## Best eSIM for Korea
**P0 — Japan Market Intent / Product Set Review**

The provider set is the problem, not Title/H1.

## Phone-Number eSIM
**P1 — Freshness / Trust**

The current decision model is excellent. Keep exact carrier capabilities current.

## Airport
**PASS**

The first-30-minutes protocol is differentiated and practical.

## Arrival
**P1 — Search-facing + Regulatory Freshness**

Keep the physical arrival flow; surface e-Arrival/K-ETA more clearly and maintain official current rules.

---

# 14. Source Register

## Korea Inside current Japanese source
- `ja/esim.html`
- `ja/best-esim-for-korea.html`
- `ja/korea-esim-with-phone-number.html`
- `ja/airport.html`
- `ja/arrival.html`

## eSIM Guide benchmarks
- tabicle  
  https://tabicle.com/korea-esim/
- eSIM韓国ナビ communication-method comparison  
  https://esim-kankoku.com/korea-esim-vs-localsim/
- LG U+ official installation guide  
  https://www.lguplus.com/korea-sim/jpn/pc/support/blog/korea_esim_sim_installation_guide
- LG U+ official eSIM  
  https://www.lguplus.com/korea-sim/jpn/pc/product/esim?tab=data

## Best eSIM benchmarks
- mybest  
  https://my-best.com/23488
- 価格.com Korea eSIM comparison  
  https://kakaku.com/mobile_data/world-wifi/world-esim/search.html?we_area=korea
- Saily Japanese comparison  
  https://saily.com/ja/blog/korea-esim-recommendations/
- supporting KANKOKO  
  https://www.thekankoko.com/2026-korea-esim/

## Phone-number eSIM benchmarks
- 韓国eSIM比較ナビ  
  https://kankoku-esim.com/phone-number-included/
- ちょぴログ  
  https://www.rinablog.org/korea-esim-phone-number/
- KT official Japanese eSIM  
  https://m.roaming.kt.com/jpn/esim
- LG U+ official product/guide  
  https://www.lguplus.com/korea-sim/jpn/pc/
- SK Telecom official  
  https://www.skroaming.com/

## Airport / public-arrival-hall benchmarks
- LG U+ official arrival blog  
  https://www.lguplus.com/korea-sim/jpn/pc/support/blog/incheon_airport_arrival_guide
- AirportGuide.jp  
  https://www.airportguide.jp/icn-airport-guide/
- Incheon official tourism arrival/access  
  https://jap-itour.incheon.go.kr/

## Arrival / immigration benchmarks
- KOREA-LOGUE  
  https://korealogue.net/2026/08/11/incheon-airport-seoul-transport-guide/
- KoreaPlus  
  https://koreaplus-lifes.com/guide/ja/airport
- Incheon International Airport official Japanese arrival process  
  https://www.airport.kr/ap_ja/1823/subview.do
- official e-Arrival Card Japanese  
  https://e-arrivalcard.go.kr/portal/main/index.do?locale=JP
- official K-ETA  
  https://www.k-eta.go.kr/

---

# 15. Current Official Fact Notes Used for SEO/Trust Review

These are not instructions to rewrite current public copy automatically.

## K-ETA
Official current notice:
- temporary exemption period extended through **2026-12-31**
- currently exempt countries/regions remain subject to the extension
- voluntary K-ETA remains possible
- valid K-ETA can remove the separate arrival-card submission requirement

## e-Arrival Card
Official current Japanese site:
- no fee
- available from **3 days before arrival**
- eligibility depends on visa / K-ETA / residence status
- official site warns about fake paid sites

## Incheon Airport
Official current guidance:
- verify the actual T1/T2 assignment with e-ticket/real-time airport information
- codeshare/airline operational changes can affect terminal
- immigration precedes baggage claim
- baggage claim precedes public arrival hall

## Telecom
Carrier-specific conditions are not uniform.
Do not publish one blanket statement for:
- 010 number
- SMS
- calls
- Korean identity verification
- QR reuse
- passport limit
- airport verification

---

# 16. Batch Close

- Japanese pages analyzed: **5 / 5**
- Current Korea Inside Japanese source checked: **5 / 5**
- Japanese SERP intent review: **5 / 5**
- Direct competitor/editorial comparison: **5 / 5**
- Current official telecom/airport/immigration fact layer checked
- 1:1 SEO GAP tables: **5 / 5**
- Internal-link analysis: **5 / 5**
- GEO / AI Search review: **5 / 5**
- New Page Candidate backlog: **updated**
- HTML changes: **0**
- Git changes: **0**
- Production changes: **0**

## Final Batch 8 Classification

1. eSIM Guide — **PASS**
2. Best eSIM for Korea — **P0 Japan Market Intent / Product Set Review**
3. Korea eSIM with Phone Number — **P1 Freshness / Trust**
4. Airport / First 30 Minutes — **PASS**
5. Arrival Guide — **P1 Search-facing + Regulatory Freshness**

**Batch 8 research: COMPLETE**
