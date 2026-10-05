# Korea Inside — Japanese SEO Competitive Restructuring Report
## Batch 12 — Final 3 Pages

**Date:** 2026-10-04  
**Status:** RESEARCH COMPLETE — SEO RESTRUCTURING REVIEW / NO IMPLEMENTATION  
**Language:** Japanese  
**Scope:** Existing Korea Inside Japanese 58-page program — Batch 12 / final 3 pages  
**Research basis:** Current Korea Inside Japanese source + current Japanese-language SERP sampling + direct competitor/editorial page review + official current NAVER / VISITKOREA / Apple / MobileTmoney sources where time-sensitive facts mattered  
**Implementation:** HTML 0 / Git 0 / Production 0  

### Batch 12 Pages
1. `ja/korean-online-payments-foreigners.html`
2. `ja/korea-atm-foreign-cards.html`
3. `ja/apple-pay-korea.html`

> **SERP caution:** Search-result order varies by time, location, device and personalization. “Direct Editorial Benchmark #1/#2” means pages selected for useful structural comparison, not absolute Google Japan rank positions.

---

# 0. Executive Summary

Batch 12 completes the Japanese 58-page SEO competitive restructuring research.

The last three pages form a clean troubleshooting/payment cluster:

- Korean online payment
- ATM cash withdrawal
- Apple Pay in Korea

The main conclusion is:

> **These pages are already structurally strong. The remaining work is mostly freshness and Japanese-market cleanup, not expansion.**

## Final Batch 12 Classification

| Page | Final Status | Main Finding |
|---|---|---|
| Korean Online Payments | **P1 — Platform / Search-facing Freshness** | KI already reflects the 2026 reality that NAVER and CATCHTABLE have foreign-traveler routes. Maintain current platform capability and make “global/traveler route first” highly visible. |
| Korea ATM for Foreign Cards | **PASS** | Current `Global ATM → another ATM if failure → fee layers → DCC → withdrawal steps` structure closely matches the best current Japanese guidance. No structural rewrite is needed. |
| Apple Pay Korea | **P1 Freshness + P0 micro-fix** | Current page is unusually current and strong, including direct Wallet vs MobileTmoney top-up. Remove `米国発行カード / 米国から来る旅行者` residue and continue to maintain current international-user MobileTmoney conditions. |

With this Batch, the research program reaches:

# **58 / 58 Japanese pages COMPLETE**
# **12 / 12 Batch MD files COMPLETE**

---

# 1. Current Korea Inside Japanese Source Baseline

## 1.1 Korean Online Payments

- File: `ja/korean-online-payments-foreigners.html`
- SHA: `66cb0aad7f1cf49336ee8ceedfd870703332c058`
- Title: `外国人旅行者の韓国オンライン決済：なぜ支払いに失敗する？ | Korea Inside`
- Meta: `韓国の店舗では使える海外カードがウェブサイトで使えない理由を解説。韓国の本人確認、電話番号認証、旅行者向け決済、カード認証の違いを整理します。`
- H1: `外国人旅行者の韓国オンライン決済`

Current structure:
- inspect the checkout before blaming the card
- Korean phone number can still be a barrier, but not always
- NAVER as an example of fast-changing access
- global/traveler version as the solution
- overseas-card flow can still require issuer authentication
- determine where checkout stopped
- not every payment problem starts in the same place
- 8 FAQ
- official information

Current page already includes:
- NAVER 2026 passport verification
- NAVER Map travel-related reservation/order/payment route
- CATCHTABLE Global
- overseas card deposit
- 3-D Secure
- global/traveler route logic

## 1.2 Korea ATM for Foreign Cards

- File: `ja/korea-atm-foreign-cards.html`
- SHA: `ac0d226a4081c5eb05e6dcbc33f3ee59275de447`
- Title: `海外発行カードで韓国ATMを使う方法 | Korea Inside`
- Meta: `韓国で現金を引き出す旅行者向けガイド。海外カード対応ATMの探し方、引き出しに失敗したときの対処、ATM手数料、DCC（動的通貨換算）を解説します。`
- H1: `海外発行カードで韓国ATMを使う方法`

Current structure:
- find ATM that accepts foreign cards
- a card working in a store does not guarantee ATM use
- try a different ATM after failure
- Incheon Airport: do not obsess over one ATM
- multiple fee layers
- choose KRW vs home currency
- withdrawal sequence
- 8 FAQ
- official information

## 1.3 Apple Pay Korea

- File: `ja/apple-pay-korea.html`
- SHA: `65896cb154e23e7b33a7b85a3424044ca8f8286a`
- Title: `韓国でApple Payは使える？旅行者向けガイド | Korea Inside`
- Meta: `韓国でApple Payは使える？店舗での海外発行カード利用、公共交通のT-money、Apple Walletへのチャージ、MobileTmoneyを使う旅行者向け方法を解説します。`
- H1: `韓国で旅行者はApple Payを使える？`

Current structure:
- overseas-issued Apple Pay card at Korean merchant
- tourist does not need to create a Korean Hyundai Card
- failed tap ≠ issuer card decline
- T-money in Apple Wallet for subway/bus
- direct Wallet T-money funding limitation
- MobileTmoney traveler route
- direct Wallet vs MobileTmoney funding distinction
- one T-money card cannot be active on iPhone and Apple Watch simultaneously
- keep a physical-card fallback
- 8 FAQ
- official information

Current Japanese-market residue found:
- FAQ: `米国発行カードのApple Payを韓国で使える？`
- body: `米国から来る旅行者なら...`

These are not factually wrong, but they are not appropriate as the representative example in the Japanese-localized page.

---

# 2. PAGE 1 — Korean Online Payments

## 2.1 Search Intent

Primary Japanese intent:

> **韓国のサイトやアプリで海外カードが使えない。韓国の電話番号・本人認証・海外カードのどこで止まっている？**

This is not a generic “card declined” query.

The problem may occur before a card transaction is even attempted.

Typical failure surfaces:
1. account
2. Korean phone number
3. local identity verification
4. traveler/global route
5. card-network support
6. 3-D Secure / issuer authentication
7. final payment approval

The current KI page correctly separates them.

## 2.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `韓国 オンライン決済 海外カード`

Supporting:
- `韓国 オンライン決済 外国人`
- `韓国 ネット決済 海外カード`
- `韓国 予約 海外カード`
- `韓国 電話番号なし 予約`
- `NAVER 予約 外国人`
- `NAVER 海外カード`
- `NAVER パスポート認証`
- `CATCHTABLE Global`
- `韓国 3Dセキュア`
- `韓国 オンライン カード 使えない`

## 2.3 SERP Observation

Japanese search does not strongly reward one giant abstract “Korean online payment” article.

Users often arrive through a specific service:
- NAVER
- CATCHTABLE
- restaurant booking
- ticket booking
- cinema
- K-pop / sports
- ferry
- beauty reservation

This means the representative KI page should own the **diagnostic framework**, while entity-specific pages/services can provide the transaction route.

That is the right architecture.

## 2.4 Direct Editorial Benchmark #1 — DeliverySpot

URL:
`https://deliveryspot.kr/ja/guides/naver-vs-catchtable-korea-restaurant-reservations`

Observed framing:
`韓国レストラン予約｜CATCHTABLEとNAVER比較`

Current structure:
1. CATCHTABLE Global first
2. no Korean phone number required
3. overseas card deposit
4. NAVER foreigner booking has expanded
5. overseas cards can be supported for participating services
6. foreign passport verification
7. provider/store settings still matter
8. distinguish reservation vs waiting

### Strength
- highly current
- exact 2026 NAVER/CATCHTABLE state
- foreign-user pathways
- practical entity-level answer

### Weakness vs KI
- restaurant-specific
- does not provide a general online-payment diagnostic tree
- less issuer-authentication / 3-D Secure detail
- cannot own the broader “why did payment fail?” problem

## 2.5 Direct Editorial Benchmark #2 — HaniSeoul / CATCHTABLE Global

URL:
`https://www.haniseoul.com/ja/travels/korea/catchtable-global-reservation-guide-for-tourists`

Observed title:
`2026年キャッチテーブル・グローバル活用術：韓国の電話番号なしで人気店を予約する完璧ガイド`

Structure:
- local vs global version
- email/Apple/Google account
- no Korean number
- overseas-card deposit
- reservation
- waiting
- cancellation/no-show
- current traveler workflow

### Strength
- strong foreign-traveler exact query
- concrete solution
- Japanese language
- current 2026

### Weakness vs KI
- one platform
- not a general online-payment failure guide
- product-specific rules can change

## 2.6 Current Official NAVER Fact Layer

Current NAVER official information is important because older Japanese content is now stale.

### 2025 foreigner booking/order expansion
NAVER states:
- participating reservation/order merchants can accept foreign tourists
- traveler-facing languages include Japanese
- where prepayment is required, overseas-issued cards can be supported for foreign tourist flows
- service availability remains merchant/service dependent

### 2026 passport verification
NAVER announced on 2026-06-09:
- foreign passport authentication introduced from June 4
- foreigners without Korean mobile-phone authentication can use passport verification
- this can support NAVER Map-related reservations, orders and payments
- eligible passport must be valid and foreign-issued

This is a meaningful market change.

## 2.7 Current Official CATCHTABLE Global Fact Layer

VISITKOREA currently states:
- no Korean phone number required
- Google / Apple / email account registration
- overseas-issued credit card accepted for reservation deposits
- Japanese supported
- restaurant reservation and waiting
- not every restaurant necessarily supports every function

This supports KI’s “global version can be the solution” section.

## 2.8 Korea Inside Actual Structure

Current KI already has the correct diagnostic model:

### Stage 1
Is there a foreign/traveler checkout?

### Stage 2
Are you stopped by:
- local account
- phone number
- identity verification?

### Stage 3
Can you reach the foreign-card payment screen?

### Stage 4
Does the issuer request authentication?

### Stage 5
Did final payment actually approve?

This is stronger than most platform-specific competitor pages.

## 2.9 1:1 SEO GAP

| Element | Japanese competitors | Korea Inside | Gap / Action | Priority |
|---|---|---|---|---|
| Title | NAVER/Catchtable/entity-specific | broad online-payment failure | good hub role | P1 |
| Meta | number/card/reservation | exact diagnostic factors | strong |
| H1 | entity-specific | broad foreign-traveler payment | strong |
| Intro / Quick Answer | “use Global version” | payment-stage diagnosis | KI stronger |
| H2 | platform flow | stage/failure flow | KI stronger hub |
| H3 | platform detail | intentionally minimal | correct |
| Keyword language | `予約 / 電話番号 / 海外カード` | strong but `オンライン決済` abstract | P1 |
| Intent | entity solution | diagnostic hub | complementary |
| Depth | platform-specific | cross-platform | KI stronger |
| Practical info | concrete platform | strong diagnosis | strong |
| Decision support | high | exceptional | KEEP |
| Freshness | extremely high | NAVER/Catchtable changes | **P1** |
| Entities | NAVER/Catchtable | both current | strong |
| Internal links | narrow | Card Declined / Cards / Payments / Apps | KI advantage |
| FAQ | entity FAQs | 8 diagnostic FAQs | strong |
| CTA | service-specific | neutral | trust advantage |
| Trust/source | mixed | official NAVER/VISITKOREA | strong |
| Commercial usefulness | reservation conversion | troubleshooting/trust | strong |

## 2.10 KEEP

- current Title/Meta/H1
- checkout-first diagnosis
- phone-number barrier is conditional
- NAVER 2026 passport verification
- CATCHTABLE Global
- 3-D Secure
- global/traveler route
- separate card-decline problem
- 8 FAQ

## 2.11 P0 / P1 / P2

### P0
None.

### P1 — Platform / Search-facing Freshness

Maintain current platform state:
- NAVER passport verification
- NAVER foreigner booking/payment availability
- merchant participation limitation
- CATCHTABLE Global
- supported card networks
- 3-D Secure
- global versions of other services when relevant

Search-facing improvement:
make the top answer explicitly say:

> **海外カードが使えないときは、カードを変える前に「旅行者向け・Globalの決済経路があるか」を確認する。**

### P2
No broad platform directory is needed.

## 2.12 SEO Surface Candidate

Potential Title research candidate:
`韓国のオンライン決済で海外カードが使えない？外国人旅行者向け対処法 | Korea Inside`

Current title is still acceptable.

Do not change until final 58-page prioritization.

## 2.13 Content Gap

No strategic content gap.

The problem is **rapidly improving platform access**, which can make old statements wrong.

## 2.14 Japan-Specific Market Gap

Japanese travelers particularly need:
- Japanese UI
- no Korean phone number
- Japanese-issued cards
- passport authentication
- CATCHTABLE / NAVER reservations

Current KI is already well positioned.

## 2.15 Competitor-Only Content

- detailed restaurant reservation walkthrough
- specific KBO/cinema/ferry booking guides
- platform-level cancellation rules

These belong in entity/use-case details, not this diagnostic hub.

## 2.16 New Page Candidates

Potential:
- `NAVER予約 外国人`
- `CATCHTABLE Global 使い方`

Only if Search Console shows substantial independent intent.

## 2.17 GEO / AI Search

Answer:
- why store card works but web card fails
- Korean phone number needed?
- passport verification
- NAVER
- CATCHTABLE
- Global version
- 3-D Secure
- another card or another route?
- online failure vs store decline

## 2.18 Internal Links

Outbound:
- Payments
- Foreign Credit Cards
- Card Declined
- Apps
- Phone-number eSIM if identity/contact context is relevant

Inbound:
- Payments
- Foreign Credit Cards
- Card Declined
- Apps
- reservation-related future content

## 2.19 Final Primary Keyword

**`韓国 オンライン決済 海外カード`**

## 2.20 Final Status

# **P1 — Platform / Search-facing Freshness**

---

# 3. PAGE 2 — Korea ATM for Foreign Cards

## 3.1 Search Intent

Primary Japanese intent:

> **日本など海外で発行されたカードを使って、韓国ATMでウォンを引き出すにはどうする？**

The real sub-problems:
- which ATM
- card-network compatibility
- PIN
- fee
- issuer fee
- DCC
- failed ATM
- airport ATM
- card retained / cash not dispensed

## 3.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `韓国 ATM 海外カード`

Supporting:
- `韓国 ATM 使い方`
- `韓国 ATM 日本のカード`
- `韓国 Global ATM`
- `韓国 ATM 手数料`
- `韓国 ATM DCC`
- `韓国 ATM ウォン 引き出し`
- `韓国 ATM Visa`
- `韓国 ATM Mastercard`
- `韓国 ATM JCB`
- `韓国 ATM PIN`
- `仁川空港 ATM`

## 3.3 SERP Observation

The strongest current Japanese/foreigner ATM content converges on four rules:

1. find a foreign-card / global-compatible ATM
2. one rejection does not prove the card is unusable
3. fee comes from more than one layer
4. choose KRW / decline ATM currency conversion when possible

The current KI page already uses this model.

## 3.4 Direct Editorial Benchmark #1 — Korea Untangled

URL:
`https://koreauntangled.com/ja/guides/withdraw-korean-won-atm-korea-ja/`

Published:
- 2026-09-09
- official info check date shown

Observed structure:
- look for Global/card-network mark
- not every Global ATM guarantees every card
- use bank/airport/reputable convenience-store ATM
- one failure → another machine
- keep records if cash/card problem occurs
- fee is multi-layer
- local-currency choice
- operational troubleshooting

### Strength
- extremely current
- source/checked-date transparency
- exact traveler problem
- strong failure handling

### Weakness vs KI
- less integrated with broader KI payment cluster
- narrower article

## 3.5 Direct Editorial Benchmark #2 — Wise Japan

URL:
`https://wise.com/jp/blog/withdraw-money-with-wise-in-south-korea`

Observed structure:
- use ATM matching card network
- Korean ATM withdrawal
- compare fees
- ATM owner fee vs issuer/card fee
- DCC
- local-currency selection
- Wise product
- ATM procedure

### Strength
- strong Japanese money vocabulary
- clear DCC explanation
- very practical fee logic

### Weakness vs KI
- commercial Wise framing
- card/product-specific
- not fully neutral for general Visa/Master/JCB traveler

## 3.6 Supporting Current Global-ATM Signal

Recent 2026 foreigner guidance commonly says:
- look for `Global ATM / Global Services`
- Visa/Mastercard/Plus/Cirrus support can vary by machine
- machine owner may charge a fee
- home issuer may charge another fee
- DCC can add a poor exchange rate
- bank branch ATM is useful if a machine retains a card

These are the correct evergreen decision points.

## 3.7 Korea Inside Actual Structure

Current KI already:
- starts with foreign-card-compatible ATM
- says store acceptance and ATM acceptance differ
- says try another ATM
- says do not obsess over one airport ATM
- separates fee layers
- explains KRW vs home currency
- gives withdrawal sequence
- 8 FAQ
- official information

This is very strong.

## 3.8 1:1 SEO GAP

| Element | Competitors | Korea Inside | Judgment | Priority |
|---|---|---|---|---|
| Title | ATM / foreign card / won | exact | PASS |
| Meta | Global ATM/fees/DCC | exact | PASS |
| H1 | exact | exact | PASS |
| Intro / Quick Answer | find Global ATM | same principle | PASS |
| H2 | ATM/network/fees/DCC | exactly aligned | PASS |
| H3 | minimal | minimal | correct |
| Keyword language | direct | direct | PASS |
| Intent | exact | exact | PASS |
| Depth | good | strong | PASS |
| Practical info | excellent | excellent | PASS |
| Decision support | high | high | KEEP |
| Freshness | fees/limits | maintenance only | maintenance |
| Entities | ATM networks | neutral | appropriate |
| Internal links | payment guides | KI full payment cluster | advantage |
| FAQ | yes | 8 | strong |
| CTA | financial product often | neutral | trust advantage |
| Trust/source | current sources important | official/source layer | strong |
| Commercial usefulness | cash access | high utility | PASS |

## 3.9 KEEP

- current title/meta/H1
- foreign-card-compatible ATM
- store acceptance ≠ ATM acceptance
- another ATM after failure
- airport flexibility
- fee layers
- DCC / KRW choice
- PIN
- 8 FAQ

## 3.10 P0 / P1 / P2

### P0
None.

### P1
No structural work.

Maintenance:
- individual ATM fees
- card-network support
- withdrawal limits
- issuer rules
- airport bank/ATM availability

Do not publish one universal Korean ATM fee.

### P2
Potential short troubleshooting box:
- cash not dispensed
- card retained
- duplicate/held authorization

Only if not already sufficiently covered in body.

## 3.11 SEO Surface Gap

None.

Current title is exact and clear.

## 3.12 Content Gap

No major gap.

## 3.13 Japan-Specific Market Gap

Japanese users may hold:
- Japanese Visa/Mastercard/JCB
- debit card
- cash card with overseas withdrawal
- Wise-type travel debit

The current generic foreign-card framing is correct.

## 3.14 Competitor-Only Content

- product-specific “free ATM withdrawal”
- exact daily limits
- “this ATM always works” claims

Do not hard-code as general truth.

## 3.15 New Page Candidate

None.

## 3.16 GEO / AI Search

Answer:
- what is Global ATM?
- Japanese card works?
- one ATM rejects card
- PIN
- fee
- DCC
- choose KRW
- airport ATM
- store works but ATM fails

## 3.17 Internal Links

Outbound:
- Payments
- Foreign Credit Cards
- Card Declined
- WOWPASS
- Airport

Inbound:
- Payments
- Foreign Credit Cards
- Card Declined
- Checklist
- Airport

## 3.18 Final Primary Keyword

**`韓国 ATM 海外カード`**

## 3.19 Final Status

# **PASS**

---

# 4. PAGE 3 — Apple Pay Korea

## 4.1 Search Intent

Primary Japanese intent:

> **日本で使っているApple Payは韓国の店で使える？地下鉄では？T-moneyはどう入れて、どうチャージする？**

This query now contains **two separate Apple Pay systems**:

### Store payment
- normal credit/debit card in Apple Wallet
- merchant must support compatible contactless/card network

### Transit
- prepaid T-money card inside Apple Wallet
- direct Wallet funding rule differs from MobileTmoney app funding

The current KI page correctly separates these.

## 4.2 Primary / Supporting Japanese Keywords

**Final Primary Keyword:** `韓国 Apple Pay 使える`

Supporting:
- `韓国 Apple Pay 日本のカード`
- `韓国 Apple Pay 海外発行カード`
- `韓国 Apple Pay T-money`
- `韓国 Apple Pay 地下鉄`
- `韓国 T-money iPhone`
- `韓国 Apple Wallet T-money`
- `MobileTmoney 外国人`
- `MobileTmoney 海外カード`
- `韓国 Apple Pay Hyundai Card`
- `韓国 Apple Pay タッチ決済`

## 4.3 SERP Observation

Japanese current Apple Pay results contain significant confusion.

Three questions are repeatedly mixed:

1. Which **Korean-issued cards** can be added to Apple Pay?
2. Can a **foreign card already added to Apple Wallet** be used at a Korean contactless merchant?
3. How can a traveler use **T-money on iPhone**?

The current KI page is unusually strong because it separates all three.

The remaining issues are:
- current MobileTmoney funding conditions must stay fresh
- U.S.-traveler wording must be localized out of the Japanese page

## 4.4 Direct Editorial Benchmark #1 — UtilKorea

URL:
`https://utilkorea.com/ja/korea/travel/apple-pay-korea`

Observed title:
`Apple Payが使える場所 2026 — Hyundai Cardなしの使い方、コンビニ・カフェ・マート対応店`

Observed structure:
- Korean-issued Hyundai Card issue
- overseas-issued wallet card
- store terminal / contactless requirement
- card-network acceptance
- tourist distinction
- T-money
- MobileTmoney

### Strength
- current 2026
- exact Japanese confusion
- separates Korean-issued-card question from visitor question
- transit distinction

### Weakness / Risk vs KI
- exact merchant-acceptance claims can become stale
- entity/shop percentages are difficult to maintain
- less clean troubleshooting/internal cluster

## 4.5 Direct Editorial Benchmark #2 — Korea Events Now

URL:
`https://koreaeventsnow.com/ja/guides/mobile-payments-tourist/`

Observed framing:
- Korean Apple Pay issuer story and tourist foreign-card story are different questions
- foreign card already in Wallet can work when merchant/card network conditions fit
- terminal support is the practical problem
- use physical backup
- transport is a separate problem

### Strength
- very concise
- correct question separation
- current September 2026

### Weakness vs KI
- less detailed T-money funding logic
- less MobileTmoney traveler detail
- less FAQ depth

## 4.6 Current Official Apple Fact Layer — Store Payment

Apple currently states:
- Apple Pay is usable where Apple Pay/contactless marks are accepted
- actual card must be from a participating issuer
- foreign use may be subject to issuer terms/fees
- Korea is an Apple Pay supported region
- Korea’s local participating issuer list is about cards **issued in Korea**

That local issuer list should not be confused with whether a Japanese traveler’s already-supported Apple Pay card can be used abroad.

Korea Inside already explains this distinction.

## 4.7 Current Official T-money Direct-Wallet Rule

Apple’s current support continues to state:

> **A South Korea-issued credit or debit card is required to purchase or top up T-money directly from Apple Wallet.**

So:
- adding/using Apple Wallet T-money
- funding it directly from Wallet

must not be presented as identical questions.

## 4.8 Current MobileTmoney International-User Change

This is the most important current fact.

The Japanese App Store current MobileTmoney listing now says:

- made for travelers
- no sign-up required
- start immediately
- top up in the app using Apple Pay

Current version history for 2026 lists:
- available for international users
- no sign-up required
- Apple Pay top-up:
  - Mastercard
  - American Express
  - UnionPay
  - JCB

The current list does **not** include Visa.

This is a major 2026 change compared with older reviews that said the app was not usable by foreigners.

Korea Inside’s current page already reflects this newer state.

## 4.9 Korea Inside Actual Structure

Current KI:
- explains supported foreign Wallet card at compatible store
- says tourist does not need Hyundai Card
- separates failed contactless tap from issuer decline
- separates normal card payment from transit
- explains Apple Wallet T-money
- explains direct Wallet funding limitation
- explains MobileTmoney traveler route
- distinguishes direct Wallet vs MobileTmoney
- explains iPhone/Watch one-card limitation
- recommends physical backup
- FAQ includes current Visa limitation for MobileTmoney

This is excellent and unusually current.

## 4.10 1:1 SEO GAP

| Element | Competitors | Korea Inside | Gap / Action | Priority |
|---|---|---|---|---|
| Title | `韓国 Apple Pay 使える` | exact | PASS |
| Meta | store + Tmoney | same + MobileTmoney | KI stronger |
| H1 | traveler question | exact | PASS |
| Intro / Quick Answer | conditions | very clear store/transit split | KI stronger |
| H2 | issuer/store/Tmoney | excellent full sequence | KI stronger |
| H3 | often none | none | correct |
| Keyword language | exact | exact | PASS |
| Intent | exact | exact | PASS |
| Depth | medium/high | exceptional | KI stronger |
| Practical info | store/Tmoney | store + funding paths | KI stronger |
| Decision support | high | exceptional | KEEP |
| Freshness | extremely high | **P1** |
| Entities | Apple/Tmoney/Hyundai | exact | strong |
| Internal links | payment only | Tmoney/Payments/Card Declined | KI advantage |
| FAQ | variable | 8 | strong |
| CTA | none | none | appropriate |
| Trust/source | current Apple/App Store | current official layer | strong |
| Commercial usefulness | utility | high trust value | PASS/P1 |

## 4.11 KEEP

- current title/meta/H1
- overseas Wallet card vs Korean-issued-card distinction
- Hyundai Card explanation
- merchant terminal/card-network condition
- failed tap vs card decline
- store payment vs transit
- T-money in Wallet
- direct Wallet funding limitation
- MobileTmoney international route
- Visa currently absent from listed foreign top-up networks
- iPhone/Watch one-card rule
- physical-card backup
- FAQ

## 4.12 P0 / P1 / P2

### P0 micro-fix — Japanese Market Residue

Remove or replace:
- `米国発行カードのApple Payを韓国で使える？`
- `米国から来る旅行者なら...`

Suggested Japanese-market framing:
- `日本発行カードをApple Payに登録して韓国で使える？`
or
- broader `海外発行カード`

This is a localization/search-intent fix, not a content-strategy rewrite.

### P1 — Freshness

Maintain:
- Korean local participating issuer list
- foreign Wallet card conditions
- MobileTmoney App Store current supported networks
- direct Wallet funding rule
- MobileTmoney international-user availability
- transport coverage
- device/iOS requirements

### P2
No structural expansion.

## 4.13 SEO Surface Recommendation

Current title/H1: **KEEP**

FAQ candidate:
`日本発行カードをApple Payに登録して韓国で使える？`

Body example:
replace:
`米国から来る旅行者なら`

with:
`日本から来る旅行者を含め、海外で発行された対応カードをすでにApple Walletへ追加している場合は`

Exact final wording should be decided only in implementation-review phase.

## 4.14 Content Gap

No major content gap.

This page is already more current than much of the Japanese SERP.

## 4.15 Japan-Specific Market Gap

Japan-specific questions:
- Japanese Apple Pay card
- iPhone T-money
- Suica-like usage expectations
- Japanese-issued card for top-up
- Visa vs Mastercard/JCB support

The current page already handles almost all of these after the micro-fix.

## 4.16 Competitor-Only Content

- merchant-chain percentage estimates
- “X% of cafes accept Apple Pay”
- store lists that become stale quickly

Avoid hard-coding broad penetration claims.

## 4.17 New Page Candidate

No new page needed.

T-money page and Apple Pay page together cover the mobile transit/payment cluster well.

## 4.18 GEO / AI Search

Answer:
- Apple Pay works in Korea?
- Japanese card?
- Hyundai Card required?
- store terminal requirement?
- failed tap?
- subway/bus?
- T-money in Wallet?
- direct Wallet top-up?
- MobileTmoney?
- Visa?
- iPhone + Apple Watch?

## 4.19 Internal Links

Outbound:
- T-money
- Payments
- Foreign Credit Cards
- Card Declined
- Apps

Inbound:
- Payments
- T-money
- Foreign Credit Cards
- Card Declined
- Checklist/Apps

## 4.20 Final Primary Keyword

**`韓国 Apple Pay 使える`**

## 4.21 Final Status

# **P1 Freshness + P0 micro-fix**

---

# 5. Cross-Page Findings

## 5.1 Korea’s Foreign-Traveler Digital Access Is Improving Fast

Old advice:
- Korean number always required
- NAVER not usable
- MobileTmoney not usable by foreigners

is increasingly wrong.

Current changes include:
- NAVER foreign passport authentication
- NAVER traveler reservation/payment paths
- CATCHTABLE Global
- MobileTmoney international-user mode

This means Korean travel SEO must distinguish:
- historical friction
- current actual friction

## 5.2 Global/Traveler Route Is Now a Core Search Concept

For Korean digital services, the first question should be:

> **Is there a Global / foreign-traveler version?**

This can solve:
- phone number
- identity
- foreign card
- language

before the traveler tries to force the Korean domestic checkout.

## 5.3 ATM Content Should Remain Simple

ATM does not need a huge bank ranking.

The durable sequence is:
1. compatible/global ATM
2. card network
3. PIN
4. fee display
5. choose KRW
6. withdraw
7. if failure, change machine/bank
8. preserve evidence if cash/card problem

## 5.4 Apple Pay in Korea Requires Three Separate Questions

Never collapse these:

### A. Korean-issued card support
Who can add a Korean-issued card to Apple Pay?

### B. Foreign traveler store payment
Can a card already in Apple Wallet be used at a Korean contactless merchant?

### C. Transit
How is T-money created/funded/used?

The current KI page handles these well.

## 5.5 Current MobileTmoney Facts Supersede Old Reviews

Older App Store reviews and blog articles can now be materially stale.

Current 2026 MobileTmoney app listing explicitly targets international users.

Therefore:
- always use current App Store/product documentation for capability
- older user reviews may be useful only as historical context

## 5.6 Japanese-Market Residue Should Be Audited Across All 58 Pages

Batch 11 and 12 found:
- `米国発行クレジットカード`
- `米国発行カードのApple Pay`
- `米国から来る旅行者`

These are not translation errors.

They are **market-reference errors**.

The final 58-page implementation plan should include a dedicated:
> **JP market-context residue scan**

---

# 6. Japan SEO Restructuring Rules — Batch 12 Additions

## Rule 61 — Digital-Service Advice Must Have a “Current Access Route” Check

Before writing:
`外国人には使えない`

check:
- Global version
- passport verification
- overseas-card checkout
- foreign account path

## Rule 62 — Platform Capability Should Be Sourced From Current Platform Documentation

Especially:
- NAVER
- CATCHTABLE
- Apple
- MobileTmoney

Do not rely on old travel-blog assumptions.

## Rule 63 — Online Payment Failure Should Be Diagnosed by Stage

Account → identity → payment route → issuer auth → approval.

## Rule 64 — Apple Pay Requires Issuer / Merchant / Transit Separation

These are separate systems.

## Rule 65 — Market-Reference Residue Is a Final QA Category

Search for:
- U.S.
- UK
- English-market examples
- non-Japanese home-country assumptions

inside Japanese public copy.

---

# 7. P0 Action List

## Apple Pay — Micro Fix
- remove `米国発行カード` FAQ framing
- remove `米国から来る旅行者` example
- use Japan-relevant or generic foreign-issued-card wording

No full-page P0 restructuring in Batch 12.

---

# 8. P1 Action List

## Korean Online Payments
- maintain current NAVER/CATCHTABLE access
- make traveler/global route first answer highly visible
- keep 3-D Secure / issuer-auth logic current

## Apple Pay
- current MobileTmoney network support
- direct Apple Wallet top-up rule
- current device/app requirements
- store-contactless condition

## ATM
- only normal current maintenance of fees/network support

---

# 9. P2 Action List

- no major new utility page required
- possible NAVER/CATCHTABLE detail only if Search Console later proves independent demand

---

# 10. New Page Backlog

| Candidate | Why it surfaced | Gate |
|---|---|---|
| NAVER予約 外国人 | platform now supports foreign-passport travel flows | demand / maintenance burden |
| CATCHTABLE Global 使い方 | strong restaurant-booking intent | overlap with Apps/Food content |

No candidate is approved for production.

---

# 11. Final Batch 12 Recommendation

## Korean Online Payments
**P1 — Platform / Search-facing Freshness**

The architecture is correct and already reflects 2026 changes.

## Korea ATM for Foreign Cards
**PASS**

Protect current diagnostic/fee/DCC structure.

## Apple Pay Korea
**P1 Freshness + P0 micro-fix**

This is one of the strongest and most current Japanese utility pages, but the U.S.-market examples should be removed from the Japanese public copy.

---

# 12. Source Register

## Korea Inside current Japanese source
- `ja/korean-online-payments-foreigners.html`
- `ja/korea-atm-foreign-cards.html`
- `ja/apple-pay-korea.html`

## Korean Online Payments — benchmarks / official
- DeliverySpot — NAVER vs CATCHTABLE  
  https://deliveryspot.kr/ja/guides/naver-vs-catchtable-korea-restaurant-reservations
- HaniSeoul — CATCHTABLE Global 2026  
  https://www.haniseoul.com/ja/travels/korea/catchtable-global-reservation-guide-for-tourists
- NAVER official foreign-tourist booking/order  
  https://help.naver.com/service/11712/contents/24301
- NAVER official passport authentication help  
  https://help.naver.com/service/5640/contents/25234
- NAVER official 2026 passport-authentication press release  
  https://www.navercorp.com/media/pressReleasesDetail?seq=10034359
- VISITKOREA Japanese CATCHTABLE  
  https://japanese.visitkorea.or.kr/svc/contents/contentsView.do?menuSn=607&vcontsId=1589779
- VISITKOREA Catchtable Global  
  https://japanese.visitkorea.or.kr/svc/contents/infoHtmlView.do?vcontsId=205616

## ATM — benchmarks
- Korea Untangled  
  https://koreauntangled.com/ja/guides/withdraw-korean-won-atm-korea-ja/
- Wise Japan — Korea ATM  
  https://wise.com/jp/blog/withdraw-money-with-wise-in-south-korea
- Wise current ATM general guidance  
  https://wise.com/jp/blog/atm-withdraw-wise
- supporting current Global ATM guidance  
  https://nicekoreanfriend.com/money-banking/korea-atm-guide

## Apple Pay — benchmarks / official
- UtilKorea  
  https://utilkorea.com/ja/korea/travel/apple-pay-korea
- Korea Events Now  
  https://koreaeventsnow.com/ja/guides/mobile-payments-tourist/
- Apple Japan — Apple Pay  
  https://www.apple.com/jp/apple-pay/
- Apple — countries/regions supporting Apple Pay  
  https://support.apple.com/ja-jp/102775
- Apple — supported Asia-Pacific issuers  
  https://support.apple.com/ja-jp/102897
- Apple / Tmoney launch and direct Wallet top-up condition  
  https://www.apple.com/kr/newsroom/2025/07/apple-and-tmoney-introduce-tmoney-for-apple-pay-on-iphone-and-apple-watch/
- Apple Korea transit  
  https://www.apple.com/kr/apple-pay/transit/
- Apple — travel card support  
  https://support.apple.com/ja-jp/105079
- MobileTmoney Japanese App Store  
  https://apps.apple.com/jp/app/mobiletmoney/id1470361790

---

# 13. Current Fact Notes Used for Freshness Review

These notes do not authorize public-copy changes by themselves.

## NAVER
Current official 2026 state:
- foreign passport authentication is available for foreign users
- traveler use can include NAVER Map reservation/order/payment functions
- merchant/service participation still matters
- not every domestic Korean service is automatically open

## CATCHTABLE Global
Current official VISITKOREA guidance:
- no Korean phone number required
- Google / Apple / email account
- overseas-issued credit card supported for reservation deposit
- Japanese supported
- not every restaurant offers every function

## ATM
Do not publish one universal ATM fee or withdrawal limit.

Separate:
- Korean ATM-owner fee
- home issuer fee
- card-network/exchange cost
- DCC

Prefer KRW/local-currency settlement rather than ATM-provided home-currency conversion when the goal is to avoid DCC.

## Apple Pay — Store
Current official Apple guidance:
- Apple Pay is used at compatible contactless merchants
- card issuer/network support still matters
- Korean-issued-card participating issuer list is a different question from using a supported foreign-issued Wallet card overseas

## Apple Wallet T-money
Current Apple support:
- direct purchase/top-up from Apple Wallet requires a South Korea-issued credit/debit card
- prepaid T-money can be used in Apple Wallet on supported devices
- one T-money card is active on one device at a time

## MobileTmoney
Current Japanese App Store version history:
- international users supported
- no sign-up required
- Apple Pay top-up currently lists:
  - Mastercard
  - American Express
  - UnionPay
  - JCB
- current listing does not list Visa in that international top-up set

This is time-sensitive and must be rechecked before public implementation.

---

# 14. FINAL 58-PAGE PROGRAM STATUS

## Completed Batch Files

1. Batch 01 — 5 pages
2. Batch 02 — 5 pages
3. Batch 03 — 5 pages
4. Batch 04 — 5 pages
5. Batch 05 — 5 pages
6. Batch 06 — 5 pages
7. Batch 07 — 5 pages
8. Batch 08 — 5 pages
9. Batch 09 — 5 pages
10. Batch 10 — 5 pages
11. Batch 11 — 5 pages
12. Batch 12 — 3 pages

Total:
**55 + 3 = 58 pages**

## Research Status

- Japanese target pages analyzed: **58 / 58**
- Batch reports: **12 / 12**
- Current KI source reviewed for all batch targets
- Japanese SERP intent reviewed for all batch targets
- competitor/editorial benchmarks reviewed
- high-risk official fact layers checked where needed
- internal-link / GEO / AI Search considerations included
- New Page Candidate backlog maintained

## Implementation Status

- HTML changes: **0**
- Git stage: **0**
- Commit: **0**
- Push: **0**
- Production: **0**

No current Production page was modified by this research program.

---

# 15. Program-Level Priority Pattern Before Master Synthesis

This section is only a preview of the 12-Batch pattern.  
The final Master synthesis should consolidate and deduplicate all Batch findings before implementation.

## Repeated P0 themes
- Japanese search-facing mismatch
- Japan-market product/payment ecosystem mismatch
- wrong market-reference residue
- page-role conflict where Japanese search intent differs materially

## Repeated P1 themes
- official fact freshness
- app/platform changes
- fare/price/route changes
- hotel micro-friction
- exact station/exit/final walk
- first-screen decision clarity
- Japanese-market entity visibility

## Repeated PASS themes
The strongest Korea Inside pages already outperform many competitors through:
- decision-first structure
- real travel friction
- luggage
- room/bed
- return route
- airport-to-hotel full journey
- conditional recommendations
- strong FAQ
- dedicated internal-link cluster

---

# 16. Batch Close

- Japanese pages analyzed: **3 / 3**
- Current Korea Inside Japanese source checked: **3 / 3**
- Japanese SERP intent review: **3 / 3**
- Direct competitor/editorial comparison: **3 / 3**
- Current official platform/payment fact layer checked
- 1:1 SEO GAP tables: **3 / 3**
- Internal-link analysis: **3 / 3**
- GEO / AI Search review: **3 / 3**
- New Page Candidate backlog: **updated**
- HTML changes: **0**
- Git changes: **0**
- Production changes: **0**

## Final Batch 12 Classification

1. Korean Online Payments — **P1 Platform / Search-facing Freshness**
2. Korea ATM for Foreign Cards — **PASS**
3. Apple Pay Korea — **P1 Freshness + P0 micro-fix**

# **Batch 12 research: COMPLETE**

# **Japanese SEO Competitive Research: 58 / 58 COMPLETE**
# **Batch MD: 12 / 12 COMPLETE**
