# Korea Inside — Dongdaemun Pre-Implementation Brief

**Status:** PRE-IMPLEMENTATION — COPY LOCKED / MAP & IMAGE FINALIZATION PENDING  
**Date:** 2026-09-20  
**Target file:** `dongdaemun-travel-guide.html`  
**Approved Public Copy:** `Korea_Inside_Dongdaemun_Approved_Public_Copy_2026-09-20.md`  
**Research Master:** `Korea_Inside_Dongdaemun_Research_Master_2026-09-20.md`  
**Codex implementation:** NOT YET — wait until map/image final approval

---

## 1. SEO identity — LOCKED

- URL: `dongdaemun-travel-guide.html`
- H1: `Dongdaemun Seoul Guide 2026: DDP, Markets & Night Shopping`
- SEO title: `Dongdaemun Seoul Guide 2026 | DDP, Markets & Night Shopping`
- Meta description: `Plan Dongdaemun by purpose and time: DDP, retail vs wholesale markets, fabric and accessories, night shopping, food alleys and practical routes.`
- Canonical: `https://www.getkoreainside.com/dongdaemun-travel-guide.html`
- Page family: Travel Guide
- Page role: Area Travel Guide / Hub
- Editorial type: Map-first Travel Guide / on-trip execution guide

Primary attack:
- `dongdaemun`

Strong supporting attack:
- `dongdaemun market`
- `dongdaemun night market`
- `dongdaemun shopping`
- `things to do in dongdaemun`

Separate future Detail candidate:
- `dongdaemun design plaza`
- do not create automatically in this implementation

---

## 2. Humanization — LOCKED

Status: `DONE`

Core rule:
- Dongdaemun is not treated as one market.
- The reader must choose by purpose + place + time.
- First-time normal shoppers start with retail, not wholesale.
- Specialty markets are purpose-led.
- Night wholesale is not automatically recommended.
- Food is a proper-meal layer, not a forced food crawl.
- Users are explicitly told when Dongdaemun can be skipped.

Humanization audit:
- no repeated `perfect for / ideal for / one of the best` formula
- imperative language is retained where it prevents a real navigation or timing mistake
- each zone has a distinct reason to exist

---

## 3. Internal-link map — LOCKED BY CONTEXT

Do not limit Travel Guide internal links to accommodation.

### Subway / transport context
Action:
- link to existing Korea Inside T-money guide

Placement:
- `Getting Around Without Losing the Plot`
- on the first meaningful mention of using the subway card

### Payment / cash context
Action:
- link to WOWPASS guide
- link to foreign-card / card-declined guide if that page exists at implementation time

Placement:
- `Payment can vary by shopping type`

### Airport / arrival context
Action:
- link to the existing airport/airport-transport guide where it solves luggage/arrival movement

Placement:
- accommodation / airport decision paragraph

### Stay context
Action:
- link to current Seoul accommodation hub
- future Dongdaemun Stay URL only after that page exists

Placement:
- `Should You Stay in Dongdaemun or Just Visit?`

### Myeongdong context
Action:
- link to Myeongdong Travel Guide only where comparing or combining shopping districts is natural

### DDP context
Action:
- future DDP Detail link when that URL exists
- until then, keep DDP information inside this Hub without dead/future links

Implementation rule:
- Codex must resolve the actual existing href from the repository.
- Do not invent a filename if the target page does not exist.
- Do not create new internal-link targets without separate approval.

---

## 4. Affiliate placement — LOCKED BY USER DECISION POINT

Principle:
`Relevant section → matching action → matching affiliate`

### A. Guided Dongdaemun
Placement:
- `Do You Need a Guided Dongdaemun Tour?`

Priority:
1. Klook active Dongdaemun walking/market product
2. Creatrip wholesale-market guide only if a current direct bookable affiliate product is verified
3. KKday only when an active Dongdaemun-specific product adds a distinct route/value

Copy rule:
- normal retail shoppers are told they do not need a guide
- guided product is for wholesale/specialist/communication/structured-route value

### B. Late-night rest
Placement:
- `If You Are Still Shopping Late: Do You Need a Break?`

Priority:
- Klook Dongdaemun SPAREX / jjimjilbang product if active on the publication date

### C. Stay
Placement:
- only after the stay/visit decision
Partners:
- Expedia
- Agoda
- Trip.com

### D. KKday
Partner status:
- ACTIVE Korea Inside affiliate
- user-confirmed CID: `26866`
- generate and verify the final product tracking URL before implementation

### E. Trazy
Partner status:
- NOT AFFILIATED
- research-only unless status changes

Do not use:
- sold-out products
- duplicate products that solve the same action without comparative value
- unrelated high-commission activities

---

## 5. Current-fact refresh — REQUIRED BEFORE CODEX

Recheck immediately before implementation:

- DDP current hours and date-specific night program
- Doota business hours
- Hyundai City Outlet business hours
- NYUNYU business hours / payment / tax refund
- Dongdaemun Shopping Complex section hours / Sunday closures
- DDP Fashion Mall hours / weekly closures
- apM / apM PLACE / apM Luxe hours and closure days
- Hello apM and Migliore only if retained; published sources currently conflict
- Sebit / Yellow Tent current location, operation pattern and enforcement context
- Dak Hanmari / Grilled Fish Alley current operating facts if exact hours are retained
- Klook / KKday / Creatrip inventory and cancellation conditions
- any exact affiliate link

If a fact cannot be verified:
- remove the brittle exact figure/time from public copy or soften it
- do not guess

---

## 6. Map — FINAL PRE-IMPLEMENTATION STEP

Map is the primary visual/content asset.
Photos are secondary.

Preferred implementation:
- official NAVER Maps Web Dynamic Map/API
- no screenshot-first implementation if official dynamic map remains technically viable

Required map layers:
1. DDP / visit
2. easy retail
3. late-night retail
4. specialty market
5. night wholesale
6. food

Required time filters:
- DAY
- EVENING
- AFTER 10 PM
- AFTER MIDNIGHT

Required route overlays:
- Route A — First Visit / DDP + Retail + Dinner
- Route B — Specialty Market + Food + DDP
- Route C — Night Fashion / Wholesale

Popup decision data:
- What
- When
- Buy one item?
- Good for first visit?

Map-first principle:
- first-time reader should understand the district before reading long shopping lists
- location + time must be shown together

---

## 7. Image — FINAL PRE-IMPLEMENTATION STEP

Images are secondary.

Initial target:
- Hero: 1
- DDP / district context: 1
- market / night / food context: 1 if it materially helps

Do not:
- fill a quota
- repeat generic shopping images
- let photography displace the map

Provenance required for every external image:
- original filename
- source
- creator
- source URL
- license / usage basis
- page role

---

## 8. Codex handoff condition

Do not hand off to Codex until all are true:

- Research Master complete
- Approved Public Copy = CONTENT LOCKED
- SEO identity locked
- internal-link contexts locked
- affiliate placements locked
- current facts refreshed
- Dynamic Map final pins/zones/routes approved
- images selected and rights/provenance recorded

Then Codex role:
- exact implementation only
- no copywriting
- no humanization
- no recommendation changes
- no new links/products without approval
- preserve protected common files unless separately approved
- section-by-section implementation and QA
