# Korea Inside Airport 6 Spanish Localization Source - Batch 4

## Document metadata

- Date: 2026-09-23
- Purpose: Exact technical extraction of page-specific English strings for later approved Spanish localization.
- Scope: The six English Production/main source files listed below.
- No-language-work boundary: This document contains no translation, localization, grammar improvement, humanization, rewriting, summarization, expansion, or recommendation change.
- Exactness rule: Text is decoded as browser-visible text and HTML whitespace is normalized only; wording, spelling, punctuation, capitalization, numbers, and meaning remain unchanged.
- Common UI boundary: Global navigation, the language switcher, and the global footer are excluded because approved Spanish common UI strings already exist.
- Mobile-label rule: A `data-label` value is extracted as its own ITEM only where CSS exposes it through `content: attr(data-label)`; functional, tracking, analytics, affiliate, and event `data-*` values remain protected structure.
- Protection rule: Facts, numbers, recommendations, proper names, brands, products, addresses, stations/exits/routes/bus numbers, prices, dates, hours, distances, measurements, URLs/tracking, functional data attributes, class/id values, image/srcset, CSS/JS, and schema structure must remain unchanged.

## Page index and source integrity

| Page | English file | SHA-256 | ITEM count |
|---:|---|---|---:|
| 1 | `airport.html` | `0a5f143aefca411fa88739fda825dae24257f05f73016003effb00c57eef844d` | 110 |
| 2 | `arrival.html` | `89fb08d898bd93bb26c2069464129d5b9cbc8ee7b2a54b0dc68db37cb276df0f` | 111 |
| 3 | `airport-transfer.html` | `b8649faf6bf1b61b234e0331a83f2d053f3281fb3ea69b95b53a14aeaa559ae4` | 166 |
| 4 | `arex.html` | `17d3fb49924732eb4db5fab1871b224bd11a8ff52799ad1c13f485c19b57d598` | 221 |
| 5 | `airport-bus.html` | `f58295df09fa7122ee587696103b51f61e83d7f91127a10f550c362cdb872283` | 182 |
| 6 | `incheon-airport-private-transfer.html` | `532ff0942a5eabb17d0d928e5c46cb4855901b5e985b9c2109a55cf6986bcbc8` | 139 |
| **Total** | **6 files** |  | **929** |

## EXCLUDE / protected structural records

- EXCLUDE - Shared global UI: `<header data-common-header>`, global navigation, language switcher, and global footer strings. Reason: approved Spanish common UI already exists and must not be duplicated in this page-specific source.
- EXCLUDE - Non-user-facing structure: HTML tags, schema keys/types, CSS, JavaScript, `class`, `id`, functional `data-*`, `src`, `srcset`, internal control attributes, and code-only values. Reason: preserve exactly; these are not localization strings.
- EXCLUDE - Decorative or empty alternative text. Reason: it does not expose a page-specific English string.
- PROTECTED - Link destinations, affiliate/tracking values, image paths/srcsets, IDs/classes, functional attributes, and JSON-LD structure remain byte-for-byte unchanged even when associated user-facing copy is localized later.
- PROTECTED - Source facts and recommendation judgments remain unchanged; each ITEM's `Protected tokens` field identifies exact-value names, numbers, routes, transport products, brands, and related fixed tokens present in the English copy.

## Page extraction items


## PAGE - airport.html

- English source: `airport.html`
- Source SHA-256: `0a5f143aefca411fa88739fda825dae24257f05f73016003effb00c57eef844d`
- Extracted ITEM count: 110

### ITEM 001

- Page ITEM: 001 of 110
- File: `airport.html`
- Line/context: L6 - `html > head > meta[name="description"] @content`
- Element/type: Meta description
- Exact English:

  ```text
  After immigration and customs, use this Incheon Airport arrival hall guide to connect, save your address, check payment and choose transport.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 002

- Page ITEM: 002 of 110
- File: `airport.html`
- Line/context: L8 - `html > head > title`
- Element/type: Title
- Exact English:

  ```text
  Incheon Airport Arrival Hall Guide: First 30 Minutes | Korea Inside
  ```
- Protected tokens: `Incheon Airport`, `30 Minutes`, `Korea Inside`

### ITEM 003

- Page ITEM: 003 of 110
- File: `airport.html`
- Line/context: L80 - `p.page-hero__breadcrumb:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Home / Airport
  ```
- Protected tokens: None identified in this item.

### ITEM 004

- Page ITEM: 004 of 110
- File: `airport.html`
- Line/context: L81 - `h1.airport-page-hero__title`
- Element/type: H1
- Exact English:

  ```text
  Your First 30 Minutes in the Incheon Airport Arrival Hall
  ```
- Protected tokens: `30 Minutes`, `Incheon Airport`

### ITEM 005

- Page ITEM: 005 of 110
- File: `airport.html`
- Line/context: L82 - `p.airport-page-hero__desc:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  This guide begins once immigration, baggage claim and customs are behind you and you have entered the public arrival hall. Before heading into the city, take a few minutes to make sure your phone can connect, keep your accommodation details in a form people in Korea can use, set up a payment backup and look at the whole route to your stay.
  ```
- Protected tokens: `Korea`

### ITEM 006

- Page ITEM: 006 of 110
- File: `airport.html`
- Line/context: L86 - `a.airport-pill:nth-of-type(1)`
- Element/type: Visible link text
- Exact English:

  ```text
  First steps
  ```
- Protected tokens: None identified in this item.

### ITEM 007

- Page ITEM: 007 of 110
- File: `airport.html`
- Line/context: L87 - `a.airport-pill:nth-of-type(2)`
- Element/type: Visible link text
- Exact English:

  ```text
  Transport
  ```
- Protected tokens: None identified in this item.

### ITEM 008

- Page ITEM: 008 of 110
- File: `airport.html`
- Line/context: L88 - `a.airport-pill:nth-of-type(3)`
- Element/type: Visible link text
- Exact English:

  ```text
  Problems
  ```
- Protected tokens: None identified in this item.

### ITEM 009

- Page ITEM: 009 of 110
- File: `airport.html`
- Line/context: L89 - `a.airport-pill:nth-of-type(4)`
- Element/type: Visible link text
- Exact English:

  ```text
  FAQ
  ```
- Protected tokens: None identified in this item.

### ITEM 010

- Page ITEM: 010 of 110
- File: `airport.html`
- Line/context: L93 - `img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  Illustrated first 30 minutes in the Incheon Airport arrival hall: connect, save your address, check payment and choose transport
  ```
- Protected tokens: `30 minutes`, `Incheon Airport`

### ITEM 011

- Page ITEM: 011 of 110
- File: `airport.html`
- Line/context: L110 - `h2.section__title.section__title--lg`
- Element/type: H2
- Exact English:

  ```text
  The Four Things Worth Sorting Out Before You Leave the Terminal
  ```
- Protected tokens: None identified in this item.

### ITEM 012

- Page ITEM: 012 of 110
- File: `airport.html`
- Line/context: L111 - `p.section__subtitle`
- Element/type: Body text
- Exact English:

  ```text
  None of these takes long, but each is easier to sort out while airport Wi-Fi, information desks and a comfortable place to stop are still close by. T-money is useful for buses and subways, although it can wait when your first ride is a taxi, pre-booked transfer or rental car.
  ```
- Protected tokens: `T-money`

### ITEM 013

- Page ITEM: 013 of 110
- File: `airport.html`
- Line/context: L117 - `li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Make sure mobile data works away from Wi-Fi The arrival hall Wi-Fi can hide a problem with roaming or an eSIM. Turn Wi-Fi off briefly and open a map or web page over mobile data instead of trusting the LTE or 5G symbol alone. If nothing loads, it is much easier to follow the provider's instructions or contact support before leaving the terminal. Open the eSIM guide
  ```
- Protected tokens: `LTE`, `5G`

### ITEM 014

- Page ITEM: 014 of 110
- File: `airport.html`
- Line/context: L119 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Make sure mobile data works away from Wi-Fi
  ```
- Protected tokens: None identified in this item.

### ITEM 015

- Page ITEM: 015 of 110
- File: `airport.html`
- Line/context: L120 - `p`
- Element/type: Body text
- Exact English:

  ```text
  The arrival hall Wi-Fi can hide a problem with roaming or an eSIM. Turn Wi-Fi off briefly and open a map or web page over mobile data instead of trusting the LTE or 5G symbol alone. If nothing loads, it is much easier to follow the provider's instructions or contact support before leaving the terminal.
  ```
- Protected tokens: `LTE`, `5G`

### ITEM 016

- Page ITEM: 016 of 110
- File: `airport.html`
- Line/context: L124 - `li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Keep your destination in a form people can use An English hotel name may not be enough for a taxi driver or a search in a Korean map app. Save the Korean place name, Korean road address, phone number and booking confirmation together, then keep a screenshot available offline. Open the maps guide
  ```
- Protected tokens: None identified in this item.

### ITEM 017

- Page ITEM: 017 of 110
- File: `airport.html`
- Line/context: L126 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Keep your destination in a form people can use
  ```
- Protected tokens: None identified in this item.

### ITEM 018

- Page ITEM: 018 of 110
- File: `airport.html`
- Line/context: L127 - `p`
- Element/type: Body text
- Exact English:

  ```text
  An English hotel name may not be enough for a taxi driver or a search in a Korean map app. Save the Korean place name, Korean road address, phone number and booking confirmation together, then keep a screenshot available offline.
  ```
- Protected tokens: None identified in this item.

### ITEM 019

- Page ITEM: 019 of 110
- File: `airport.html`
- Line/context: L131 - `li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Give yourself a payment backup A long arrival becomes more stressful when one card is the only way to pay. Keep a second card or some cash separate from your main card so a decline at a machine or counter does not block the rest of the journey. Open the payments guide
  ```
- Protected tokens: None identified in this item.

### ITEM 020

- Page ITEM: 020 of 110
- File: `airport.html`
- Line/context: L133 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Give yourself a payment backup
  ```
- Protected tokens: None identified in this item.

### ITEM 021

- Page ITEM: 021 of 110
- File: `airport.html`
- Line/context: L134 - `p`
- Element/type: Body text
- Exact English:

  ```text
  A long arrival becomes more stressful when one card is the only way to pay. Keep a second card or some cash separate from your main card so a decline at a machine or counter does not block the rest of the journey.
  ```
- Protected tokens: None identified in this item.

### ITEM 022

- Page ITEM: 022 of 110
- File: `airport.html`
- Line/context: L138 - `li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Look at the whole route, not only the first train or bus The fastest or cheapest first leg can still end with another transfer, stairs or a long walk with luggage. Your destination, arrival time, group size and final route from the stop or station matter more than the headline journey alone. See how the airport transfer choices differ
  ```
- Protected tokens: None identified in this item.

### ITEM 023

- Page ITEM: 023 of 110
- File: `airport.html`
- Line/context: L140 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Look at the whole route, not only the first train or bus
  ```
- Protected tokens: None identified in this item.

### ITEM 024

- Page ITEM: 024 of 110
- File: `airport.html`
- Line/context: L141 - `p`
- Element/type: Body text
- Exact English:

  ```text
  The fastest or cheapest first leg can still end with another transfer, stairs or a long walk with luggage. Your destination, arrival time, group size and final route from the stop or station matter more than the headline journey alone.
  ```
- Protected tokens: None identified in this item.

### ITEM 025

- Page ITEM: 025 of 110
- File: `airport.html`
- Line/context: L153 - `h2.section__title`
- Element/type: H2
- Exact English:

  ```text
  Use the Map for the Terminal You Actually Arrived At
  ```
- Protected tokens: None identified in this item.

### ITEM 026

- Page ITEM: 026 of 110
- File: `airport.html`
- Line/context: L154 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Terminal 1 and Terminal 2 have different layouts, so first confirm the terminal shown for your flight. Then switch the official map to that terminal and the arrivals floor before looking for transport, an information desk or another facility.
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 027

- Page ITEM: 027 of 110
- File: `airport.html`
- Line/context: L157 - `a.airport-official-map-link__button`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Open the Official Airport Map
  ```
- Protected tokens: None identified in this item.

### ITEM 028

- Page ITEM: 028 of 110
- File: `airport.html`
- Line/context: L165 - `p.airport-official-map-link__tip`
- Element/type: Body text
- Exact English:

  ```text
  The map may open on a different terminal or floor. Match both settings to your current arrival before following it.
  ```
- Protected tokens: None identified in this item.

### ITEM 029

- Page ITEM: 029 of 110
- File: `airport.html`
- Line/context: L175 - `h2.section__title.section__title--lg`
- Element/type: H2
- Exact English:

  ```text
  Still Working Through Immigration or Baggage Claim?
  ```
- Protected tokens: None identified in this item.

### ITEM 030

- Page ITEM: 030 of 110
- File: `airport.html`
- Line/context: L178 - `p`
- Element/type: Body text
- Exact English:

  ```text
  If you are still looking for transfer, immigration, baggage claim or customs instructions, start with the Arrival Guide. This page begins only after you have entered the public arrival hall and are ready to sort out the practical first steps of the trip.
  ```
- Protected tokens: None identified in this item.

### ITEM 031

- Page ITEM: 031 of 110
- File: `airport.html`
- Line/context: L179 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Follow the route from the aircraft to the arrival hall
  ```
- Protected tokens: None identified in this item.

### ITEM 032

- Page ITEM: 032 of 110
- File: `airport.html`
- Line/context: L188 - `h2.section__title.section__title--lg`
- Element/type: H2
- Exact English:

  ```text
  Getting from the Airport to Your Stay
  ```
- Protected tokens: None identified in this item.

### ITEM 033

- Page ITEM: 033 of 110
- File: `airport.html`
- Line/context: L189 - `p.section__subtitle`
- Element/type: Body text
- Exact English:

  ```text
  There is no single best transfer for everyone. The useful comparison is the whole door-to-door journey: where you are staying, when you arrive, how much luggage you have, who is traveling with you and what happens after the first train or bus. Current routes and operating times should still be checked for the day of travel.
  ```
- Protected tokens: None identified in this item.

### ITEM 034

- Page ITEM: 034 of 110
- File: `airport.html`
- Line/context: L194 - `h3`
- Element/type: H3
- Exact English:

  ```text
  AREX
  ```
- Protected tokens: `AREX`

### ITEM 035

- Page ITEM: 035 of 110
- File: `airport.html`
- Line/context: L196 - `p`
- Element/type: Body text
- Exact English:

  ```text
  AREX is easy to understand when the route points toward Seoul Station, Hongdae or another rail-connected destination. The train is only one part of the journey, though. If your hotel still requires another subway transfer and a long walk with luggage, a bus or taxi may be easier door to door even when the rail journey itself is faster.
  ```
- Protected tokens: `AREX`, `Seoul Station`, `Hongdae`

### ITEM 036

- Page ITEM: 036 of 110
- File: `airport.html`
- Line/context: L197 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Read the AREX guide
  ```
- Protected tokens: `AREX`

### ITEM 037

- Page ITEM: 037 of 110
- File: `airport.html`
- Line/context: L202 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Airport Bus
  ```
- Protected tokens: None identified in this item.

### ITEM 038

- Page ITEM: 038 of 110
- File: `airport.html`
- Line/context: L204 - `p`
- Element/type: Body text
- Exact English:

  ```text
  An airport bus can be surprisingly comfortable when it stops close to your accommodation. It avoids moving suitcases through a large station, but traffic and the walk from the actual stop still matter. Look at the current route and timetable rather than assuming the nearest-sounding stop is the easiest one.
  ```
- Protected tokens: None identified in this item.

### ITEM 039

- Page ITEM: 039 of 110
- File: `airport.html`
- Line/context: L205 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Read the airport bus guide
  ```
- Protected tokens: None identified in this item.

### ITEM 040

- Page ITEM: 040 of 110
- File: `airport.html`
- Line/context: L210 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Taxi
  ```
- Protected tokens: None identified in this item.

### ITEM 041

- Page ITEM: 041 of 110
- File: `airport.html`
- Line/context: L212 - `p`
- Element/type: Body text
- Exact English:

  ```text
  A taxi costs more than public transport, but the calculation changes with several travelers, children, heavy bags or a late arrival. Door-to-door travel can remove multiple transfers at the moment they feel most tiring. Use an official taxi stand and keep the Korean destination name, address and phone number ready.
  ```
- Protected tokens: None identified in this item.

### ITEM 042

- Page ITEM: 042 of 110
- File: `airport.html`
- Line/context: L213 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Read the taxi guide
  ```
- Protected tokens: None identified in this item.

### ITEM 043

- Page ITEM: 043 of 110
- File: `airport.html`
- Line/context: L218 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Pre-booked Transfer
  ```
- Protected tokens: None identified in this item.

### ITEM 044

- Page ITEM: 044 of 110
- File: `airport.html`
- Line/context: L220 - `p`
- Element/type: Body text
- Exact English:

  ```text
  The main advantage is not simply that a car is waiting. It is arriving with a meeting plan already arranged, which can be valuable with children, older adults or a large amount of luggage. Save the booking contact, meeting point and terminal instructions offline before the flight.
  ```
- Protected tokens: None identified in this item.

### ITEM 045

- Page ITEM: 045 of 110
- File: `airport.html`
- Line/context: L221 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Compare airport transfer options
  ```
- Protected tokens: None identified in this item.

### ITEM 046

- Page ITEM: 046 of 110
- File: `airport.html`
- Line/context: L227 - `h3#airport-driving-note-title`
- Element/type: H3
- Exact English:

  ```text
  Driving Beyond Seoul?
  ```
- Protected tokens: `Seoul`

### ITEM 047

- Page ITEM: 047 of 110
- File: `airport.html`
- Line/context: L228 - `p`
- Element/type: Body text
- Exact English:

  ```text
  A rental car belongs in a different decision from the four common rides into Seoul. It becomes relevant when a regional road trip or an itinerary outside the city makes driving genuinely useful. Licence requirements, insurance, pickup instructions and the correct terminal should all be settled before arrival.
  ```
- Protected tokens: `Seoul`

### ITEM 048

- Page ITEM: 048 of 110
- File: `airport.html`
- Line/context: L229 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Read the rental car guide
  ```
- Protected tokens: None identified in this item.

### ITEM 049

- Page ITEM: 049 of 110
- File: `airport.html`
- Line/context: L231 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Where you stay changes which airport route is actually easiest. If you are still planning the trip, compare Hongdae, Gongdeok, Seoul Station and Myeongdong by the full airport-to-hotel journey.
  ```
- Protected tokens: `Hongdae`, `Gongdeok`, `Seoul Station`, `Myeongdong`

### ITEM 050

- Page ITEM: 050 of 110
- File: `airport.html`
- Line/context: L232 - `p:nth-of-type(2)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Compare Seoul areas for airport access →
  ```
- Protected tokens: `Seoul`

### ITEM 051

- Page ITEM: 051 of 110
- File: `airport.html`
- Line/context: L239 - `h2.section__title`
- Element/type: H2
- Exact English:

  ```text
  Save These Before Leaving the Airport
  ```
- Protected tokens: None identified in this item.

### ITEM 052

- Page ITEM: 052 of 110
- File: `airport.html`
- Line/context: L240 - `p.section__subtitle`
- Element/type: Body text
- Exact English:

  ```text
  Keep the information you may need even if your internet or payment method stops working.
  ```
- Protected tokens: None identified in this item.

### ITEM 053

- Page ITEM: 053 of 110
- File: `airport.html`
- Line/context: L244 - `li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Accommodation details in Korean Keep the Korean name, road address and phone number together so a driver, information desk or Korean map app can identify the place.
  ```
- Protected tokens: None identified in this item.

### ITEM 054

- Page ITEM: 054 of 110
- File: `airport.html`
- Line/context: L245 - `li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Booking confirmation Save the confirmation and any arrival or check-in instructions where they remain available without mobile data.
  ```
- Protected tokens: None identified in this item.

### ITEM 055

- Page ITEM: 055 of 110
- File: `airport.html`
- Line/context: L246 - `li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  The complete transport route Record the terminal, route, stop or station, useful exit and the final walk or transfer to the accommodation—not only the first train or bus.
  ```
- Protected tokens: None identified in this item.

### ITEM 056

- Page ITEM: 056 of 110
- File: `airport.html`
- Line/context: L247 - `li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Mobile data support details Keep the eSIM activation information, support contact and QR code if one may be needed again. Follow the provider's instructions before changing or deleting a profile.
  ```
- Protected tokens: None identified in this item.

### ITEM 057

- Page ITEM: 057 of 110
- File: `airport.html`
- Line/context: L248 - `li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  An offline map reference Save the destination in Naver Map or KakaoMap and keep a screenshot or map pin that can still be shown when the connection is unreliable.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`

### ITEM 058

- Page ITEM: 058 of 110
- File: `airport.html`
- Line/context: L249 - `li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  A second way to pay Keep another card or some cash separate from the main card so one declined transaction does not stop the onward journey.
  ```
- Protected tokens: None identified in this item.

### ITEM 059

- Page ITEM: 059 of 110
- File: `airport.html`
- Line/context: L258 - `h2.section__title.section__title--lg`
- Element/type: H2
- Exact English:

  ```text
  Solve Problems in the Public Arrival Hall
  ```
- Protected tokens: None identified in this item.

### ITEM 060

- Page ITEM: 060 of 110
- File: `airport.html`
- Line/context: L259 - `p.section__subtitle`
- Element/type: Body text
- Exact English:

  ```text
  A problem with data, payment or transport is easier to untangle before you leave the terminal. The arrival hall gives you Wi-Fi, official signs and information desks while you work out a backup.
  ```
- Protected tokens: None identified in this item.

### ITEM 061

- Page ITEM: 061 of 110
- File: `airport.html`
- Line/context: L264 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Your eSIM Still Has No Data
  ```
- Protected tokens: None identified in this item.

### ITEM 062

- Page ITEM: 062 of 110
- File: `airport.html`
- Line/context: L266 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Stay on airport Wi-Fi for a few more minutes. Make sure the travel line is switched on and selected for mobile data, then follow the provider's activation and roaming instructions. If it still does not connect, contact the provider before deleting the eSIM profile; your offline screenshots can cover the immediate journey while support is being arranged.
  ```
- Protected tokens: None identified in this item.

### ITEM 063

- Page ITEM: 063 of 110
- File: `airport.html`
- Line/context: L267 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Review the eSIM guide
  ```
- Protected tokens: None identified in this item.

### ITEM 064

- Page ITEM: 064 of 110
- File: `airport.html`
- Line/context: L272 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Your Foreign Card Was Declined
  ```
- Protected tokens: None identified in this item.

### ITEM 065

- Page ITEM: 065 of 110
- File: `airport.html`
- Line/context: L274 - `p`
- Element/type: Body text
- Exact English:

  ```text
  One decline does not have to stop the trip. Try a staffed counter or a different card, and look at the card issuer's settings while you still have Wi-Fi. A second card or some cash keeps the transport decision separate from the problem with the first payment method.
  ```
- Protected tokens: None identified in this item.

### ITEM 066

- Page ITEM: 066 of 110
- File: `airport.html`
- Line/context: L275 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Review the payments guide
  ```
- Protected tokens: None identified in this item.

### ITEM 067

- Page ITEM: 067 of 110
- File: `airport.html`
- Line/context: L280 - `h3`
- Element/type: H3
- Exact English:

  ```text
  You Cannot Find AREX, the Bus or a Taxi
  ```
- Protected tokens: `AREX`

### ITEM 068

- Page ITEM: 068 of 110
- File: `airport.html`
- Line/context: L282 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Start with the terminal and floor shown on the official airport map, then follow the current signs for the route you chose. An information desk can point out the correct entrance, stop or official taxi stand. That is safer and clearer than following an unsolicited ride offer.
  ```
- Protected tokens: None identified in this item.

### ITEM 069

- Page ITEM: 069 of 110
- File: `airport.html`
- Line/context: L283 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Compare airport transport
  ```
- Protected tokens: None identified in this item.

### ITEM 070

- Page ITEM: 070 of 110
- File: `airport.html`
- Line/context: L288 - `h3`
- Element/type: H3
- Exact English:

  ```text
  You Arrived Late at Night
  ```
- Protected tokens: None identified in this item.

### ITEM 071

- Page ITEM: 071 of 110
- File: `airport.html`
- Line/context: L290 - `p`
- Element/type: Body text
- Exact English:

  ```text
  A late arrival changes the order of the decision: check the current official timetable for your terminal before walking toward a daytime route from memory. A late-night bus may still be available. If it is not, use an official taxi stand or the saved meeting instructions for a pre-booked pickup.
  ```
- Protected tokens: None identified in this item.

### ITEM 072

- Page ITEM: 072 of 110
- File: `airport.html`
- Line/context: L291 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Check the airport bus guide
  ```
- Protected tokens: None identified in this item.

### ITEM 073

- Page ITEM: 073 of 110
- File: `airport.html`
- Line/context: L296 - `h3`
- Element/type: H3
- Exact English:

  ```text
  The Driver Cannot Identify Your Accommodation
  ```
- Protected tokens: None identified in this item.

### ITEM 074

- Page ITEM: 074 of 110
- File: `airport.html`
- Line/context: L298 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Show the Korean accommodation name, road address and phone number rather than relying on the English brand name alone. A saved location in a Korean map app or the booking screenshot can give the driver another reference, and airport Wi-Fi leaves you a way to contact the accommodation if the destination is still unclear.
  ```
- Protected tokens: None identified in this item.

### ITEM 075

- Page ITEM: 075 of 110
- File: `airport.html`
- Line/context: L299 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Review the maps guide
  ```
- Protected tokens: None identified in this item.

### ITEM 076

- Page ITEM: 076 of 110
- File: `airport.html`
- Line/context: L309 - `h2.section__title.section__title--lg`
- Element/type: H2
- Exact English:

  ```text
  Incheon Airport First 30 Minutes FAQ
  ```
- Protected tokens: `Incheon Airport`, `30 Minutes`

### ITEM 077

- Page ITEM: 077 of 110
- File: `airport.html`
- Line/context: L310 - `p.section__subtitle.section__subtitle--center`
- Element/type: Body text
- Exact English:

  ```text
  Short answers for decisions you need to make in the public arrival hall.
  ```
- Protected tokens: None identified in this item.

### ITEM 078

- Page ITEM: 078 of 110
- File: `airport.html`
- Line/context: L315 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What should I do first after entering the arrival hall?
  ```
- Protected tokens: None identified in this item.

### ITEM 079

- Page ITEM: 079 of 110
- File: `airport.html`
- Line/context: L316 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Use the airport Wi-Fi as a safety net while you test mobile data, save the Korean accommodation details, make sure you have a second way to pay and look at the complete route to your stay. These small jobs become harder once you are outside with luggage.
  ```
- Protected tokens: None identified in this item.

### ITEM 080

- Page ITEM: 080 of 110
- File: `airport.html`
- Line/context: L319 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Do I need to buy T-money immediately?
  ```
- Protected tokens: `T-money`

### ITEM 081

- Page ITEM: 081 of 110
- File: `airport.html`
- Line/context: L320 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  No. T-money is useful when a bus or subway is part of the first journey, but it does not need to be the first purchase for someone leaving by taxi, pre-booked transfer or rental car. Getting connected and understanding the route may matter more in those first few minutes.
  ```
- Protected tokens: `T-money`

### ITEM 082

- Page ITEM: 082 of 110
- File: `airport.html`
- Line/context: L323 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  How do I choose between AREX, airport bus and taxi?
  ```
- Protected tokens: `AREX`

### ITEM 083

- Page ITEM: 083 of 110
- File: `airport.html`
- Line/context: L324 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Look beyond the first leg. AREX is straightforward for rail-connected destinations, an airport bus can be easier when it stops near the accommodation, and a taxi removes transfers when luggage, children, group size or a late arrival make them especially tiring.
  ```
- Protected tokens: `AREX`

### ITEM 084

- Page ITEM: 084 of 110
- File: `airport.html`
- Line/context: L327 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What should I do after a late-night arrival?
  ```
- Protected tokens: None identified in this item.

### ITEM 085

- Page ITEM: 085 of 110
- File: `airport.html`
- Line/context: L328 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Check the current official timetable for your terminal before following a route planned for daytime. A late-night bus may still operate; otherwise, use an official taxi stand or the saved meeting instructions for a pre-booked pickup.
  ```
- Protected tokens: None identified in this item.

### ITEM 086

- Page ITEM: 086 of 110
- File: `airport.html`
- Line/context: L331 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Where can I check my terminal and airport facilities?
  ```
- Protected tokens: None identified in this item.

### ITEM 087

- Page ITEM: 087 of 110
- File: `airport.html`
- Line/context: L332 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Check the official arrival flight page for your terminal and use the official airport map for facilities. If you must change terminals in the public area, use the free terminal shuttle bus or the paid Airport Railroad and confirm current operating details. The airside shuttle train is for transfer routes, not ordinary public-area terminal travel.
  ```
- Protected tokens: `Airport Railroad`

### ITEM 088

- Page ITEM: 088 of 110
- File: `airport.html`
- Line/context: L335 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What information should I save before leaving the airport?
  ```
- Protected tokens: None identified in this item.

### ITEM 089

- Page ITEM: 089 of 110
- File: `airport.html`
- Line/context: L336 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Save the Korean accommodation name, road address and phone number, the booking confirmation, the complete transport route, useful map screenshots, mobile-data activation and support details, any important QR code and a backup payment plan.
  ```
- Protected tokens: None identified in this item.

### ITEM 090

- Page ITEM: 090 of 110
- File: `airport.html`
- Line/context: L341 - `p.related-links__title`
- Element/type: Related-guide text
- Exact English:

  ```text
  Continue with the part you need
  ```
- Protected tokens: None identified in this item.

### ITEM 091

- Page ITEM: 091 of 110
- File: `airport.html`
- Line/context: L342 - `a.chip:nth-of-type(1)`
- Element/type: Related-guide visible link
- Exact English:

  ```text
  Set up mobile data
  ```
- Protected tokens: None identified in this item.

### ITEM 092

- Page ITEM: 092 of 110
- File: `airport.html`
- Line/context: L343 - `a.chip:nth-of-type(2)`
- Element/type: Related-guide visible link
- Exact English:

  ```text
  Understand T-money
  ```
- Protected tokens: `T-money`

### ITEM 093

- Page ITEM: 093 of 110
- File: `airport.html`
- Line/context: L344 - `a.chip:nth-of-type(3)`
- Element/type: Related-guide visible link
- Exact English:

  ```text
  Plan payment backups
  ```
- Protected tokens: None identified in this item.

### ITEM 094

- Page ITEM: 094 of 110
- File: `airport.html`
- Line/context: L345 - `a.chip:nth-of-type(4)`
- Element/type: Related-guide visible link
- Exact English:

  ```text
  Set up Korean maps
  ```
- Protected tokens: None identified in this item.

### ITEM 095

- Page ITEM: 095 of 110
- File: `airport.html`
- Line/context: L346 - `a.chip:nth-of-type(5)`
- Element/type: Related-guide visible link
- Exact English:

  ```text
  Compare the full airport route
  ```
- Protected tokens: None identified in this item.

### ITEM 096

- Page ITEM: 096 of 110
- File: `airport.html`
- Line/context: L354 - `h2.section__title.section__title--lg`
- Element/type: H2
- Exact English:

  ```text
  Official Airport Information
  ```
- Protected tokens: None identified in this item.

### ITEM 097

- Page ITEM: 097 of 110
- File: `airport.html`
- Line/context: L355 - `p.section__subtitle`
- Element/type: Body text
- Exact English:

  ```text
  Terminal assignments, transport schedules and facility locations can change. These are the official Incheon Airport pages to use when the detail needs to be current.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 098

- Page ITEM: 098 of 110
- File: `airport.html`
- Line/context: L359 - `li.airport-copy-row:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Airport Map Select Terminal 1 or Terminal 2, then use the arrivals floor to find facilities and transport entrances. Open the official airport map
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 099

- Page ITEM: 099 of 110
- File: `airport.html`
- Line/context: L360 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Airport Map
  ```
- Protected tokens: None identified in this item.

### ITEM 100

- Page ITEM: 100 of 110
- File: `airport.html`
- Line/context: L362 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Select Terminal 1 or Terminal 2, then use the arrivals floor to find facilities and transport entrances.
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 101

- Page ITEM: 101 of 110
- File: `airport.html`
- Line/context: L366 - `li.airport-copy-row:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Arrival Flights Use the live flight information to confirm the actual arrival terminal instead of relying on a fixed airline list. Check official arrival flights
  ```
- Protected tokens: None identified in this item.

### ITEM 102

- Page ITEM: 102 of 110
- File: `airport.html`
- Line/context: L367 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Arrival Flights
  ```
- Protected tokens: None identified in this item.

### ITEM 103

- Page ITEM: 103 of 110
- File: `airport.html`
- Line/context: L369 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Use the live flight information to confirm the actual arrival terminal instead of relying on a fixed airline list.
  ```
- Protected tokens: None identified in this item.

### ITEM 104

- Page ITEM: 104 of 110
- File: `airport.html`
- Line/context: L373 - `li.airport-copy-row:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Inter-terminal Transportation Review the public-area terminal shuttle bus and Airport Railroad options, together with current operating and fare information. Check official terminal transport
  ```
- Protected tokens: `Airport Railroad`

### ITEM 105

- Page ITEM: 105 of 110
- File: `airport.html`
- Line/context: L374 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Inter-terminal Transportation
  ```
- Protected tokens: None identified in this item.

### ITEM 106

- Page ITEM: 106 of 110
- File: `airport.html`
- Line/context: L376 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Review the public-area terminal shuttle bus and Airport Railroad options, together with current operating and fare information.
  ```
- Protected tokens: `Airport Railroad`

### ITEM 107

- Page ITEM: 107 of 110
- File: `airport.html`
- Line/context: L380 - `li.airport-copy-row:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Late-night Buses Use the page for the terminal you arrived at to check current routes, stops and departure times. Terminal 1 Terminal 2
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 108

- Page ITEM: 108 of 110
- File: `airport.html`
- Line/context: L381 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Late-night Buses
  ```
- Protected tokens: None identified in this item.

### ITEM 109

- Page ITEM: 109 of 110
- File: `airport.html`
- Line/context: L383 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Use the page for the terminal you arrived at to check current routes, stops and departure times.
  ```
- Protected tokens: None identified in this item.

### ITEM 110

- Page ITEM: 110 of 110
- File: `airport.html`
- Line/context: L384 - `p.airport-source-links:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Terminal 1 Terminal 2
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`


## PAGE - arrival.html

- English source: `arrival.html`
- Source SHA-256: `89fb08d898bd93bb26c2069464129d5b9cbc8ee7b2a54b0dc68db37cb276df0f`
- Extracted ITEM count: 111

### ITEM 111

- Page ITEM: 001 of 111
- File: `arrival.html`
- Line/context: L6 - `html > head > meta[name="description"] @content`
- Element/type: Meta description
- Exact English:

  ```text
  Incheon Airport arrival guide covering terminal checks, immigration, K-ETA and e-Arrival Card, baggage claim, customs and the public arrival hall.
  ```
- Protected tokens: `Incheon Airport`, `K-ETA`, `e-Arrival Card`

### ITEM 112

- Page ITEM: 002 of 111
- File: `arrival.html`
- Line/context: L9 - `html > head > title`
- Element/type: Title
- Exact English:

  ```text
  Incheon Airport Arrival Guide: Immigration, Baggage & Customs | Korea Inside
  ```
- Protected tokens: `Incheon Airport`, `Korea Inside`

### ITEM 113

- Page ITEM: 003 of 111
- File: `arrival.html`
- Line/context: L22 - `script[type="application/ld+json"] > mainEntity.0.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  How do I know whether my flight arrives at Terminal 1 or Terminal 2?
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 114

- Page ITEM: 004 of 111
- File: `arrival.html`
- Line/context: L25 - `script[type="application/ld+json"] > mainEntity.0.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Check your e-ticket or search your flight number on the official Incheon Airport arrival page. That is safer than relying on a fixed airline list, particularly for codeshare flights.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 115

- Page ITEM: 005 of 111
- File: `arrival.html`
- Line/context: L30 - `script[type="application/ld+json"] > mainEntity.1.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Do I go through immigration before collecting my baggage?
  ```
- Protected tokens: None identified in this item.

### ITEM 116

- Page ITEM: 006 of 111
- File: `arrival.html`
- Line/context: L33 - `script[type="application/ld+json"] > mainEntity.1.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Yes, if you are entering Korea. The usual order is immigration, baggage claim, customs and then the public arrival hall. Passengers connecting to another flight should follow Transfer or Connecting Flights signs and their airline’s instructions instead.
  ```
- Protected tokens: `Korea`

### ITEM 117

- Page ITEM: 007 of 111
- File: `arrival.html`
- Line/context: L38 - `script[type="application/ld+json"] > mainEntity.2.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  What should I do if my baggage does not arrive?
  ```
- Protected tokens: None identified in this item.

### ITEM 118

- Page ITEM: 008 of 111
- File: `arrival.html`
- Line/context: L41 - `script[type="application/ld+json"] > mainEntity.2.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  First check that the baggage belt still matches your flight. If the bag is missing or damaged, contact your airline or the baggage service desk before leaving the baggage area, and keep your baggage tag and flight details ready.
  ```
- Protected tokens: None identified in this item.

### ITEM 119

- Page ITEM: 009 of 111
- File: `arrival.html`
- Line/context: L46 - `script[type="application/ld+json"] > mainEntity.3.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Can I walk between Terminal 1 and Terminal 2?
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 120

- Page ITEM: 010 of 111
- File: `arrival.html`
- Line/context: L49 - `script[type="application/ld+json"] > mainEntity.3.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  No. Terminal 1 and Terminal 2 are separate buildings without a public walking route between them. If you need to change terminals in the public area, use the airport’s official inter-terminal transportation guidance.
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 121

- Page ITEM: 011 of 111
- File: `arrival.html`
- Line/context: L54 - `script[type="application/ld+json"] > mainEntity.4.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Which transport should I choose from Incheon Airport to Seoul?
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 122

- Page ITEM: 012 of 111
- File: `arrival.html`
- Line/context: L57 - `script[type="application/ld+json"] > mainEntity.4.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Make that decision after you reach the public arrival hall. The best option depends on the exact accommodation, arrival time, luggage and group size. The separate Airport Transfer Guide compares AREX, airport bus, taxi and pre-booked transfer by the full journey to your stay.
  ```
- Protected tokens: `AREX`

### ITEM 123

- Page ITEM: 013 of 111
- File: `arrival.html`
- Line/context: L62 - `script[type="application/ld+json"] > mainEntity.5.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  What should transfer passengers do?
  ```
- Protected tokens: None identified in this item.

### ITEM 124

- Page ITEM: 014 of 111
- File: `arrival.html`
- Line/context: L65 - `script[type="application/ld+json"] > mainEntity.5.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Follow Transfer or Connecting Flights signs and your airline’s instructions. Confirm the connection terminal, available connection time and whether checked baggage is transferred through to the next flight instead of assuming that you should follow the general immigration route.
  ```
- Protected tokens: None identified in this item.

### ITEM 125

- Page ITEM: 015 of 111
- File: `arrival.html`
- Line/context: L138 - `p.page-hero__breadcrumb:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Home / Airport / Arrival Guide
  ```
- Protected tokens: None identified in this item.

### ITEM 126

- Page ITEM: 016 of 111
- File: `arrival.html`
- Line/context: L139 - `h1#arrival-title.arrival-hero__title`
- Element/type: H1
- Exact English:

  ```text
  What to Do After Landing at Incheon Airport
  ```
- Protected tokens: `Incheon Airport`

### ITEM 127

- Page ITEM: 017 of 111
- File: `arrival.html`
- Line/context: L140 - `p.arrival-hero__desc:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Landing at Incheon Airport is fairly straightforward once you know which signs matter. If you are entering Korea, the usual route is Arrivals, immigration, baggage claim, customs and then the public arrival hall.
  ```
- Protected tokens: `Incheon Airport`, `Korea`

### ITEM 128

- Page ITEM: 018 of 111
- File: `arrival.html`
- Line/context: L141 - `p.arrival-hero__desc:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  If you are connecting to another flight, do not follow that sequence automatically. Follow Transfer or Connecting Flights signs and the instructions for your itinerary instead.
  ```
- Protected tokens: None identified in this item.

### ITEM 129

- Page ITEM: 019 of 111
- File: `arrival.html`
- Line/context: L144 - `img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  Travelers with luggage walking beneath an Arrivals sign inside Incheon International Airport
  ```
- Protected tokens: `Incheon International Airport`

### ITEM 130

- Page ITEM: 020 of 111
- File: `arrival.html`
- Line/context: L145 - `figcaption`
- Element/type: Figcaption
- Exact English:

  ```text
  Arrivals · Incheon International Airport
  ```
- Protected tokens: `Incheon International Airport`

### ITEM 131

- Page ITEM: 021 of 111
- File: `arrival.html`
- Line/context: L154 - `h2#flight-check-title`
- Element/type: H2
- Exact English:

  ```text
  Check the Terminal Your Flight Actually Uses
  ```
- Protected tokens: None identified in this item.

### ITEM 132

- Page ITEM: 022 of 111
- File: `arrival.html`
- Line/context: L155 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Terminal assignments can change, especially with codeshare flights. Your e-ticket and the airport’s live arrival search are more reliable than a saved airline list.
  ```
- Protected tokens: None identified in this item.

### ITEM 133

- Page ITEM: 023 of 111
- File: `arrival.html`
- Line/context: L158 - `a.arrival-button.arrival-button--light @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Check your arrival flight on the official Incheon Airport website, external site, opens in a new tab
  ```
- Protected tokens: `Incheon Airport`

### ITEM 134

- Page ITEM: 024 of 111
- File: `arrival.html`
- Line/context: L158 - `a.arrival-button.arrival-button--light`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Check Your Arrival Flight ↗
  ```
- Protected tokens: None identified in this item.

### ITEM 135

- Page ITEM: 025 of 111
- File: `arrival.html`
- Line/context: L169 - `h2#journey-type-title`
- Element/type: H2
- Exact English:

  ```text
  Entering Korea or Transferring?
  ```
- Protected tokens: `Korea`

### ITEM 136

- Page ITEM: 026 of 111
- File: `arrival.html`
- Line/context: L173 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Entering Korea
  ```
- Protected tokens: `Korea`

### ITEM 137

- Page ITEM: 027 of 111
- File: `arrival.html`
- Line/context: L174 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  If this is your final flight and you are entering Korea, follow the Arrivals or Immigration signs. The usual sequence is immigration first, checked baggage next, customs after that and then the public arrival hall.
  ```
- Protected tokens: `Korea`

### ITEM 138

- Page ITEM: 028 of 111
- File: `arrival.html`
- Line/context: L175 - `p.arrival-route-line:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Immigration, then baggage claim, then customs, then arrival hall
  ```
- Protected tokens: None identified in this item.

### ITEM 139

- Page ITEM: 029 of 111
- File: `arrival.html`
- Line/context: L175 - `p.arrival-route-line:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Immigration → Baggage Claim → Customs → Arrival Hall
  ```
- Protected tokens: None identified in this item.

### ITEM 140

- Page ITEM: 030 of 111
- File: `arrival.html`
- Line/context: L180 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Connecting to another flight
  ```
- Protected tokens: None identified in this item.

### ITEM 141

- Page ITEM: 031 of 111
- File: `arrival.html`
- Line/context: L181 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  If you are only connecting at Incheon, follow Transfer or Connecting Flights signs instead of joining the general arrival flow automatically. Your connection can depend on the airline, terminal and whether your checked baggage is transferred through to the next flight.
  ```
- Protected tokens: `Incheon`

### ITEM 142

- Page ITEM: 032 of 111
- File: `arrival.html`
- Line/context: L182 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  If anything is unclear, use your airline’s transfer instructions rather than assuming that every connection requires Korean immigration.
  ```
- Protected tokens: None identified in this item.

### ITEM 143

- Page ITEM: 033 of 111
- File: `arrival.html`
- Line/context: L191 - `h2#arrival-steps-title`
- Element/type: H2
- Exact English:

  ```text
  From the Aircraft to the Arrival Hall
  ```
- Protected tokens: None identified in this item.

### ITEM 144

- Page ITEM: 034 of 111
- File: `arrival.html`
- Line/context: L192 - `p`
- Element/type: Body text
- Exact English:

  ```text
  For travelers entering Korea, the route is usually simple. The walking distance can vary by gate and terminal, but the order of the main steps is the same.
  ```
- Protected tokens: `Korea`

### ITEM 145

- Page ITEM: 035 of 111
- File: `arrival.html`
- Line/context: L196 - `img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  Travelers following Immigration and Baggage Claim signs inside Incheon International Airport
  ```
- Protected tokens: `Incheon International Airport`

### ITEM 146

- Page ITEM: 036 of 111
- File: `arrival.html`
- Line/context: L197 - `figcaption`
- Element/type: Figcaption
- Exact English:

  ```text
  Follow the airport signs in order; your exact walking route depends on the gate and terminal.
  ```
- Protected tokens: None identified in this item.

### ITEM 147

- Page ITEM: 037 of 111
- File: `arrival.html`
- Line/context: L200 - `li.arrival-step:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  01 Follow the Arrivals Signs After leaving the aircraft, follow signs for Arrivals or Immigration. Transfer and Connecting Flights signs lead to a different process, so use them only when you are continuing to another flight.
  ```
- Protected tokens: `01`

### ITEM 148

- Page ITEM: 038 of 111
- File: `arrival.html`
- Line/context: L203 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Follow the Arrivals Signs
  ```
- Protected tokens: None identified in this item.

### ITEM 149

- Page ITEM: 039 of 111
- File: `arrival.html`
- Line/context: L204 - `p`
- Element/type: Body text
- Exact English:

  ```text
  After leaving the aircraft, follow signs for Arrivals or Immigration. Transfer and Connecting Flights signs lead to a different process, so use them only when you are continuing to another flight.
  ```
- Protected tokens: None identified in this item.

### ITEM 150

- Page ITEM: 040 of 111
- File: `arrival.html`
- Line/context: L207 - `li.arrival-step:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  02 Immigration Have your passport and the entry documents that apply to your own travel status ready. The rules are not identical for every passport, visa or residence status, so check the official Korean guidance before the flight rather than copying another traveler’s checklist. K-ETA and the e-Arrival Card are not the same thing Korea’s temporary K-ETA exemption for countries already covered by the measure has been extended through December 31, 2026. Being exempt from K-ETA does not automatically mean that you are exempt from the arrival declaration. A traveler who holds a valid K-ETA is exempt from the arrival declaration. A traveler using the temporary K-ETA exemption may still need to submit an e-Arrival Card, depending on status. The official e-Arrival Card is free and can be submitted within three days before arrival in Korea. The official website also has a navigator that tells you whether you need to submit one. The official e-Arrival Card website does not charge a fee. Do not enter payment information on a website claiming to sell the Korean arrival card. K-ETA official ↗ e-Arrival Card official ↗
  ```
- Protected tokens: `02`, `K-ETA`, `e-Arrival Card`, `Korea`, `December 31, 2026`

### ITEM 151

- Page ITEM: 041 of 111
- File: `arrival.html`
- Line/context: L210 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Immigration
  ```
- Protected tokens: None identified in this item.

### ITEM 152

- Page ITEM: 042 of 111
- File: `arrival.html`
- Line/context: L211 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Have your passport and the entry documents that apply to your own travel status ready. The rules are not identical for every passport, visa or residence status, so check the official Korean guidance before the flight rather than copying another traveler’s checklist.
  ```
- Protected tokens: None identified in this item.

### ITEM 153

- Page ITEM: 043 of 111
- File: `arrival.html`
- Line/context: L213 - `h4`
- Element/type: H4
- Exact English:

  ```text
  K-ETA and the e-Arrival Card are not the same thing
  ```
- Protected tokens: `K-ETA`, `e-Arrival Card`

### ITEM 154

- Page ITEM: 044 of 111
- File: `arrival.html`
- Line/context: L214 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Korea’s temporary K-ETA exemption for countries already covered by the measure has been extended through December 31, 2026. Being exempt from K-ETA does not automatically mean that you are exempt from the arrival declaration.
  ```
- Protected tokens: `Korea`, `K-ETA`, `December 31, 2026`

### ITEM 155

- Page ITEM: 045 of 111
- File: `arrival.html`
- Line/context: L215 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  A traveler who holds a valid K-ETA is exempt from the arrival declaration. A traveler using the temporary K-ETA exemption may still need to submit an e-Arrival Card, depending on status.
  ```
- Protected tokens: `K-ETA`, `e-Arrival Card`

### ITEM 156

- Page ITEM: 046 of 111
- File: `arrival.html`
- Line/context: L216 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  The official e-Arrival Card is free and can be submitted within three days before arrival in Korea. The official website also has a navigator that tells you whether you need to submit one.
  ```
- Protected tokens: `e-Arrival Card`, `Korea`

### ITEM 157

- Page ITEM: 047 of 111
- File: `arrival.html`
- Line/context: L217 - `p.arrival-entry-note__warning:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  The official e-Arrival Card website does not charge a fee. Do not enter payment information on a website claiming to sell the Korean arrival card.
  ```
- Protected tokens: `e-Arrival Card`

### ITEM 158

- Page ITEM: 048 of 111
- File: `arrival.html`
- Line/context: L218 - `p.arrival-context-links:nth-of-type(5)`
- Element/type: Body text
- Exact English:

  ```text
  K-ETA official ↗ e-Arrival Card official ↗
  ```
- Protected tokens: `K-ETA`, `e-Arrival Card`

### ITEM 159

- Page ITEM: 049 of 111
- File: `arrival.html`
- Line/context: L219 - `a:nth-of-type(1) @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  K-ETA official website, external site, opens in a new tab
  ```
- Protected tokens: `K-ETA`

### ITEM 160

- Page ITEM: 050 of 111
- File: `arrival.html`
- Line/context: L220 - `a:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Official Korea e-Arrival Card website, external site, opens in a new tab
  ```
- Protected tokens: `Korea e-Arrival Card`

### ITEM 161

- Page ITEM: 051 of 111
- File: `arrival.html`
- Line/context: L225 - `li.arrival-step:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  03 Baggage Claim After immigration, check the airport display for the baggage belt assigned to your flight and collect your checked bags before continuing toward customs. If your suitcase does not appear, first make sure the belt still matches your flight. If it is genuinely missing or damaged, speak to your airline or the baggage service desk while you are still inside the baggage area. Keep the baggage tag from check-in and your flight details with you.
  ```
- Protected tokens: `03`

### ITEM 162

- Page ITEM: 052 of 111
- File: `arrival.html`
- Line/context: L228 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Baggage Claim
  ```
- Protected tokens: None identified in this item.

### ITEM 163

- Page ITEM: 053 of 111
- File: `arrival.html`
- Line/context: L229 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  After immigration, check the airport display for the baggage belt assigned to your flight and collect your checked bags before continuing toward customs.
  ```
- Protected tokens: None identified in this item.

### ITEM 164

- Page ITEM: 054 of 111
- File: `arrival.html`
- Line/context: L230 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  If your suitcase does not appear, first make sure the belt still matches your flight. If it is genuinely missing or damaged, speak to your airline or the baggage service desk while you are still inside the baggage area. Keep the baggage tag from check-in and your flight details with you.
  ```
- Protected tokens: None identified in this item.

### ITEM 165

- Page ITEM: 055 of 111
- File: `arrival.html`
- Line/context: L233 - `li.arrival-step:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  04 Customs After collecting your baggage, follow the customs signs toward the exit. If you are carrying goods that may need to be declared, check Korea Customs rather than relying on rules from another country. Items over the duty-free allowance and restricted goods can require a declaration. If you have nothing to declare, follow the appropriate customs channel shown at the airport. Korea Customs Service — Travelers ↗
  ```
- Protected tokens: `04`, `Korea`, `Korea Customs Service`

### ITEM 166

- Page ITEM: 056 of 111
- File: `arrival.html`
- Line/context: L236 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Customs
  ```
- Protected tokens: None identified in this item.

### ITEM 167

- Page ITEM: 057 of 111
- File: `arrival.html`
- Line/context: L237 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  After collecting your baggage, follow the customs signs toward the exit.
  ```
- Protected tokens: None identified in this item.

### ITEM 168

- Page ITEM: 058 of 111
- File: `arrival.html`
- Line/context: L238 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  If you are carrying goods that may need to be declared, check Korea Customs rather than relying on rules from another country. Items over the duty-free allowance and restricted goods can require a declaration.
  ```
- Protected tokens: `Korea`

### ITEM 169

- Page ITEM: 059 of 111
- File: `arrival.html`
- Line/context: L239 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  If you have nothing to declare, follow the appropriate customs channel shown at the airport.
  ```
- Protected tokens: None identified in this item.

### ITEM 170

- Page ITEM: 060 of 111
- File: `arrival.html`
- Line/context: L240 - `p.arrival-context-link:nth-of-type(4)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Korea Customs Service — Travelers ↗
  ```
- Protected tokens: `Korea Customs Service`

### ITEM 171

- Page ITEM: 061 of 111
- File: `arrival.html`
- Line/context: L240 - `a @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Korea Customs Service travelers information, external site, opens in a new tab
  ```
- Protected tokens: `Korea Customs Service`

### ITEM 172

- Page ITEM: 062 of 111
- File: `arrival.html`
- Line/context: L243 - `li.arrival-step:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  05 Arrival Hall Once you pass customs and enter the public arrival hall, the immigration part of the journey is finished. Before heading into the city, you may still need to sort out mobile data, save your Korean accommodation details, check your payment backup or decide how to reach your stay. Those first practical steps are covered separately in the Incheon Airport Arrival Hall Guide.
  ```
- Protected tokens: `05`, `Incheon Airport`

### ITEM 173

- Page ITEM: 063 of 111
- File: `arrival.html`
- Line/context: L246 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Arrival Hall
  ```
- Protected tokens: None identified in this item.

### ITEM 174

- Page ITEM: 064 of 111
- File: `arrival.html`
- Line/context: L247 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Once you pass customs and enter the public arrival hall, the immigration part of the journey is finished.
  ```
- Protected tokens: None identified in this item.

### ITEM 175

- Page ITEM: 065 of 111
- File: `arrival.html`
- Line/context: L248 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Before heading into the city, you may still need to sort out mobile data, save your Korean accommodation details, check your payment backup or decide how to reach your stay. Those first practical steps are covered separately in the Incheon Airport Arrival Hall Guide.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 176

- Page ITEM: 066 of 111
- File: `arrival.html`
- Line/context: L259 - `h2#terminal-title`
- Element/type: H2
- Exact English:

  ```text
  Terminal 1 and Terminal 2 Are Separate Buildings
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 177

- Page ITEM: 067 of 111
- File: `arrival.html`
- Line/context: L260 - `p`
- Element/type: Body text
- Exact English:

  ```text
  You do not need to memorize a fixed list of airlines for each terminal. Check the terminal shown for your actual flight number, then use the map for orientation after landing.
  ```
- Protected tokens: None identified in this item.

### ITEM 178

- Page ITEM: 068 of 111
- File: `arrival.html`
- Line/context: L265 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Terminal 1
  ```
- Protected tokens: `Terminal 1`

### ITEM 179

- Page ITEM: 069 of 111
- File: `arrival.html`
- Line/context: L266 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Terminal 1 is a separate passenger terminal with its own immigration, baggage claim and arrival hall. Follow the signs for the flight you actually arrived on rather than using a saved airline list.
  ```
- Protected tokens: `Terminal 1`

### ITEM 180

- Page ITEM: 070 of 111
- File: `arrival.html`
- Line/context: L269 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Terminal 2
  ```
- Protected tokens: `Terminal 2`

### ITEM 181

- Page ITEM: 071 of 111
- File: `arrival.html`
- Line/context: L270 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Terminal 2 has its own arrival facilities and is not connected to Terminal 1 by a public walking route. If you need to move between terminals after entering the public area, use the airport’s current inter-terminal transportation guidance.
  ```
- Protected tokens: `Terminal 2`, `Terminal 1`

### ITEM 182

- Page ITEM: 072 of 111
- File: `arrival.html`
- Line/context: L271 - `p.arrival-context-link:nth-of-type(2)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Incheon Airport — Inter-terminal Transportation ↗
  ```
- Protected tokens: `Incheon Airport`

### ITEM 183

- Page ITEM: 073 of 111
- File: `arrival.html`
- Line/context: L271 - `a @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Incheon Airport inter-terminal transportation guidance, external site, opens in a new tab
  ```
- Protected tokens: `Incheon Airport`

### ITEM 184

- Page ITEM: 074 of 111
- File: `arrival.html`
- Line/context: L273 - `p.arrival-terminal-orientation`
- Element/type: Body text
- Exact English:

  ```text
  The map is useful for understanding where immigration, baggage claim, customs and the public arrival hall sit in relation to one another. For live gate, belt or facility information, use the airport’s current signs and flight information.
  ```
- Protected tokens: None identified in this item.

### ITEM 185

- Page ITEM: 075 of 111
- File: `arrival.html`
- Line/context: L276 - `a.arrival-terminal-map__media @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Open the Incheon Airport Terminal 1 and Terminal 2 arrival map at full size in a new tab
  ```
- Protected tokens: `Incheon Airport Terminal 1`, `Terminal 2`

### ITEM 186

- Page ITEM: 076 of 111
- File: `arrival.html`
- Line/context: L277 - `img.arrival-terminal-map__image @alt`
- Element/type: Image alt
- Exact English:

  ```text
  Combined Incheon Airport Terminal 1 and Terminal 2 arrival maps showing immigration, baggage claim, customs, arrival halls, transport and service locations
  ```
- Protected tokens: `Incheon Airport Terminal 1`, `Terminal 2`

### ITEM 187

- Page ITEM: 077 of 111
- File: `arrival.html`
- Line/context: L279 - `figcaption.arrival-terminal-map__caption`
- Element/type: Figcaption
- Exact English:

  ```text
  Terminal 1 and Terminal 2 arrival maps. Use them for orientation; live airport signs and flight information take priority. Open the terminal map at full size ↗
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 188

- Page ITEM: 078 of 111
- File: `arrival.html`
- Line/context: L290 - `h2#arrival-handoff-title`
- Element/type: H2
- Exact English:

  ```text
  You’re Through — What Happens Next?
  ```
- Protected tokens: None identified in this item.

### ITEM 189

- Page ITEM: 079 of 111
- File: `arrival.html`
- Line/context: L291 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Once you are in the public arrival hall, the entry process is behind you. What matters next depends on the trip: getting your phone online, keeping the accommodation address in Korean, having a payment backup and choosing a route that still works with your luggage.
  ```
- Protected tokens: None identified in this item.

### ITEM 190

- Page ITEM: 080 of 111
- File: `arrival.html`
- Line/context: L292 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Continue with the Incheon Airport Arrival Hall Guide for those first practical steps.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 191

- Page ITEM: 081 of 111
- File: `arrival.html`
- Line/context: L293 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  If your only remaining question is how to reach your accommodation, the Airport Transfer Guide compares AREX, airport bus, taxi and pre-booked transfer by the whole door-to-door journey.
  ```
- Protected tokens: `AREX`

### ITEM 192

- Page ITEM: 082 of 111
- File: `arrival.html`
- Line/context: L300 - `h2#faq-title`
- Element/type: H2
- Exact English:

  ```text
  Incheon Airport Arrival FAQ
  ```
- Protected tokens: `Incheon Airport`

### ITEM 193

- Page ITEM: 083 of 111
- File: `arrival.html`
- Line/context: L304 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  How do I know whether my flight arrives at Terminal 1 or Terminal 2?
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 194

- Page ITEM: 084 of 111
- File: `arrival.html`
- Line/context: L305 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Check your e-ticket or search your flight number on the official Incheon Airport arrival page. That is safer than relying on a fixed airline list, particularly for codeshare flights.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 195

- Page ITEM: 085 of 111
- File: `arrival.html`
- Line/context: L308 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Do I go through immigration before collecting my baggage?
  ```
- Protected tokens: None identified in this item.

### ITEM 196

- Page ITEM: 086 of 111
- File: `arrival.html`
- Line/context: L309 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Yes, if you are entering Korea. The usual order is immigration, baggage claim, customs and then the public arrival hall. Passengers connecting to another flight should follow Transfer or Connecting Flights signs and their airline’s instructions instead.
  ```
- Protected tokens: `Korea`

### ITEM 197

- Page ITEM: 087 of 111
- File: `arrival.html`
- Line/context: L312 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What should I do if my baggage does not arrive?
  ```
- Protected tokens: None identified in this item.

### ITEM 198

- Page ITEM: 088 of 111
- File: `arrival.html`
- Line/context: L313 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  First check that the baggage belt still matches your flight. If the bag is missing or damaged, contact your airline or the baggage service desk before leaving the baggage area, and keep your baggage tag and flight details ready.
  ```
- Protected tokens: None identified in this item.

### ITEM 199

- Page ITEM: 089 of 111
- File: `arrival.html`
- Line/context: L316 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Can I walk between Terminal 1 and Terminal 2?
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 200

- Page ITEM: 090 of 111
- File: `arrival.html`
- Line/context: L317 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  No. Terminal 1 and Terminal 2 are separate buildings without a public walking route between them. If you need to change terminals in the public area, use the airport’s official inter-terminal transportation guidance.
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 201

- Page ITEM: 091 of 111
- File: `arrival.html`
- Line/context: L320 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Which transport should I choose from Incheon Airport to Seoul?
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 202

- Page ITEM: 092 of 111
- File: `arrival.html`
- Line/context: L321 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Make that decision after you reach the public arrival hall. The best option depends on the exact accommodation, arrival time, luggage and group size. The separate Airport Transfer Guide compares AREX, airport bus, taxi and pre-booked transfer by the full journey to your stay.
  ```
- Protected tokens: `AREX`

### ITEM 203

- Page ITEM: 093 of 111
- File: `arrival.html`
- Line/context: L324 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What should transfer passengers do?
  ```
- Protected tokens: None identified in this item.

### ITEM 204

- Page ITEM: 094 of 111
- File: `arrival.html`
- Line/context: L325 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Follow Transfer or Connecting Flights signs and your airline’s instructions. Confirm the connection terminal, available connection time and whether checked baggage is transferred through to the next flight instead of assuming that you should follow the general immigration route.
  ```
- Protected tokens: None identified in this item.

### ITEM 205

- Page ITEM: 095 of 111
- File: `arrival.html`
- Line/context: L334 - `h2#official-sources-title`
- Element/type: H2
- Exact English:

  ```text
  Official Sources
  ```
- Protected tokens: None identified in this item.

### ITEM 206

- Page ITEM: 096 of 111
- File: `arrival.html`
- Line/context: L335 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Entry rules and airport operations can change. Use these official sources when a detail affects your own trip.
  ```
- Protected tokens: None identified in this item.

### ITEM 207

- Page ITEM: 097 of 111
- File: `arrival.html`
- Line/context: L338 - `li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Incheon Airport — Arrival Procedures ↗
  ```
- Protected tokens: `Incheon Airport`

### ITEM 208

- Page ITEM: 098 of 111
- File: `arrival.html`
- Line/context: L338 - `a @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Incheon Airport arrival procedures, external site, opens in a new tab
  ```
- Protected tokens: `Incheon Airport`

### ITEM 209

- Page ITEM: 099 of 111
- File: `arrival.html`
- Line/context: L339 - `li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Incheon Airport — Arrival Flights ↗
  ```
- Protected tokens: `Incheon Airport`

### ITEM 210

- Page ITEM: 100 of 111
- File: `arrival.html`
- Line/context: L339 - `a @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Incheon Airport arrival flights, external site, opens in a new tab
  ```
- Protected tokens: `Incheon Airport`

### ITEM 211

- Page ITEM: 101 of 111
- File: `arrival.html`
- Line/context: L340 - `li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Incheon Airport — Inter-terminal Transportation ↗
  ```
- Protected tokens: `Incheon Airport`

### ITEM 212

- Page ITEM: 102 of 111
- File: `arrival.html`
- Line/context: L340 - `a @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Incheon Airport inter-terminal transportation, external site, opens in a new tab
  ```
- Protected tokens: `Incheon Airport`

### ITEM 213

- Page ITEM: 103 of 111
- File: `arrival.html`
- Line/context: L341 - `li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  K-ETA ↗
  ```
- Protected tokens: `K-ETA`

### ITEM 214

- Page ITEM: 104 of 111
- File: `arrival.html`
- Line/context: L341 - `a @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  K-ETA official website, external site, opens in a new tab
  ```
- Protected tokens: `K-ETA`

### ITEM 215

- Page ITEM: 105 of 111
- File: `arrival.html`
- Line/context: L342 - `li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  Official Korea e-Arrival Card ↗
  ```
- Protected tokens: `Korea e-Arrival Card`

### ITEM 216

- Page ITEM: 106 of 111
- File: `arrival.html`
- Line/context: L342 - `a @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Official Korea e-Arrival Card website, external site, opens in a new tab
  ```
- Protected tokens: `Korea e-Arrival Card`

### ITEM 217

- Page ITEM: 107 of 111
- File: `arrival.html`
- Line/context: L343 - `li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  Korea Customs Service ↗
  ```
- Protected tokens: `Korea Customs Service`

### ITEM 218

- Page ITEM: 108 of 111
- File: `arrival.html`
- Line/context: L343 - `a @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Korea Customs Service, external site, opens in a new tab
  ```
- Protected tokens: `Korea Customs Service`

### ITEM 219

- Page ITEM: 109 of 111
- File: `arrival.html`
- Line/context: L348 - `aside.arrival-service-notice @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Information review notice
  ```
- Protected tokens: None identified in this item.

### ITEM 220

- Page ITEM: 110 of 111
- File: `arrival.html`
- Line/context: L350 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Airport operations, entry requirements and transport services may change. Confirm time-sensitive details on the relevant official website or with your airline.
  ```
- Protected tokens: None identified in this item.

### ITEM 221

- Page ITEM: 111 of 111
- File: `arrival.html`
- Line/context: L351 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Last reviewed: August 2026
  ```
- Protected tokens: `August 2026`


## PAGE - airport-transfer.html

- English source: `airport-transfer.html`
- Source SHA-256: `b8649faf6bf1b61b234e0331a83f2d053f3281fb3ea69b95b53a14aeaa559ae4`
- Extracted ITEM count: 166

### ITEM 222

- Page ITEM: 001 of 166
- File: `airport-transfer.html`
- Line/context: L6 - `html > head > meta[name="description"] @content`
- Element/type: Meta description
- Exact English:

  ```text
  Compare AREX, airport buses, taxis, Call Van and private transfers from Incheon Airport to Seoul, with current fares, luggage and late-night guidance.
  ```
- Protected tokens: `AREX`, `Incheon Airport`, `Seoul`

### ITEM 223

- Page ITEM: 002 of 166
- File: `airport-transfer.html`
- Line/context: L9 - `html > head > title`
- Element/type: Title
- Exact English:

  ```text
  Incheon Airport to Seoul: AREX, Bus, Taxi & Transfer | Korea Inside
  ```
- Protected tokens: `Incheon Airport`, `Seoul`, `AREX`, `Korea Inside`

### ITEM 224

- Page ITEM: 003 of 166
- File: `airport-transfer.html`
- Line/context: L21 - `script[type="application/ld+json"] > mainEntity.0.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Is AREX or the airport bus better?
  ```
- Protected tokens: `AREX`

### ITEM 225

- Page ITEM: 004 of 166
- File: `airport-transfer.html`
- Line/context: L24 - `script[type="application/ld+json"] > mainEntity.0.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  AREX is more predictable and works especially well for Seoul Station or Hongdae. The airport bus can be easier when its actual stop is close to the hotel and removes another station transfer. The final walk matters as much as the headline travel time.
  ```
- Protected tokens: `AREX`, `Seoul Station`, `Hongdae`

### ITEM 226

- Page ITEM: 005 of 166
- File: `airport-transfer.html`
- Line/context: L29 - `script[type="application/ld+json"] > mainEntity.1.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  What is the cheapest way from Incheon Airport to Seoul?
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 227

- Page ITEM: 006 of 166
- File: `airport-transfer.html`
- Line/context: L32 - `script[type="application/ld+json"] > mainEntity.1.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  The AREX All-stop Train is usually the cheapest rail option. Current adult transit-card fares are ₩4,650 from T1 and ₩5,250 from T2 to Hongik University, and ₩4,750 from T1 and ₩5,350 from T2 to Seoul Station.
  ```
- Protected tokens: `AREX All-stop Train`, `₩4,650`, `T1`, `₩5,250`, `T2`, `Hongik University`, `₩4,750`, `₩5,350`, `Seoul Station`

### ITEM 228

- Page ITEM: 007 of 166
- File: `airport-transfer.html`
- Line/context: L37 - `script[type="application/ld+json"] > mainEntity.2.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  What is the fastest way to Seoul Station?
  ```
- Protected tokens: `Seoul Station`

### ITEM 229

- Page ITEM: 008 of 166
- File: `airport-transfer.html`
- Line/context: L40 - `script[type="application/ld+json"] > mainEntity.2.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  The AREX Express has published airport-to-Seoul Station journey times of 43 minutes from T1 and 51 minutes from T2. The full trip still includes the walk to the airport station and whatever comes after Seoul Station.
  ```
- Protected tokens: `AREX Express`, `Seoul Station`, `43 minutes`, `T1`, `51 minutes`, `T2`

### ITEM 230

- Page ITEM: 009 of 166
- File: `airport-transfer.html`
- Line/context: L45 - `script[type="application/ld+json"] > mainEntity.3.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Does the AREX Express stop at Hongdae?
  ```
- Protected tokens: `AREX Express`, `Hongdae`

### ITEM 231

- Page ITEM: 010 of 166
- File: `airport-transfer.html`
- Line/context: L48 - `script[type="application/ld+json"] > mainEntity.3.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  No. The AREX Express runs directly to Seoul Station and does not stop at Hongik University. The All-stop Train stops at Hongik University and is usually the more direct rail option for Hongdae.
  ```
- Protected tokens: `AREX Express`, `Seoul Station`, `Hongik University`, `All-stop Train`, `Hongdae`

### ITEM 232

- Page ITEM: 011 of 166
- File: `airport-transfer.html`
- Line/context: L53 - `script[type="application/ld+json"] > mainEntity.4.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Can I use T-money on the AREX?
  ```
- Protected tokens: `T-money`, `AREX`

### ITEM 233

- Page ITEM: 012 of 166
- File: `airport-transfer.html`
- Line/context: L56 - `script[type="application/ld+json"] > mainEntity.4.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  T-money can be used on the AREX All-stop Train. The AREX Express uses a separate ticket and reserved-seat system.
  ```
- Protected tokens: `T-money`, `AREX All-stop Train`, `AREX Express`

### ITEM 234

- Page ITEM: 013 of 166
- File: `airport-transfer.html`
- Line/context: L61 - `script[type="application/ld+json"] > mainEntity.5.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  What works better with several large suitcases?
  ```
- Protected tokens: None identified in this item.

### ITEM 235

- Page ITEM: 014 of 166
- File: `airport-transfer.html`
- Line/context: L64 - `script[type="application/ld+json"] > mainEntity.5.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  The answer changes with the number and size of the bags. An airport bus can work well when its stop is close to the hotel, while a larger taxi, Call Van or pre-booked transfer becomes more useful when moving the luggage through a station would be difficult. Passenger capacity and luggage capacity are separate limits.
  ```
- Protected tokens: None identified in this item.

### ITEM 236

- Page ITEM: 015 of 166
- File: `airport-transfer.html`
- Line/context: L69 - `script[type="application/ld+json"] > mainEntity.6.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  What can I use after midnight?
  ```
- Protected tokens: None identified in this item.

### ITEM 237

- Page ITEM: 016 of 166
- File: `airport-transfer.html`
- Line/context: L72 - `script[type="application/ld+json"] > mainEntity.6.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Incheon Airport currently lists overnight services including N6000, N6002, N6701 and N6703, with different departure times at T1 and T2. Official taxis and pre-booked vehicles are alternatives when the useful train or night-bus window has closed.
  ```
- Protected tokens: `Incheon Airport`, `N6000`, `N6002`, `N6701`, `N6703`, `T1`, `T2`

### ITEM 238

- Page ITEM: 017 of 166
- File: `airport-transfer.html`
- Line/context: L77 - `script[type="application/ld+json"] > mainEntity.7.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  How do taxis from Incheon Airport work?
  ```
- Protected tokens: `Incheon Airport`

### ITEM 239

- Page ITEM: 018 of 166
- File: `airport-transfer.html`
- Line/context: L80 - `script[type="application/ld+json"] > mainEntity.7.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Official airport taxi stands serve regular, Deluxe, Jumbo and International Taxi services. A regular Seoul taxi currently starts at ₩4,800 for 1.6 km, while Deluxe, SUV and Jumbo services start at ₩7,000 for 3 km. The final fare depends on the service, destination, traffic and time of travel.
  ```
- Protected tokens: `International Taxi`, `Seoul`, `₩4,800`, `1.6 km`, `SUV`, `₩7,000`, `3 km`

### ITEM 240

- Page ITEM: 019 of 166
- File: `airport-transfer.html`
- Line/context: L85 - `script[type="application/ld+json"] > mainEntity.8.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Does Terminal 1 or Terminal 2 change the route?
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 241

- Page ITEM: 020 of 166
- File: `airport-transfer.html`
- Line/context: L88 - `script[type="application/ld+json"] > mainEntity.8.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  The main transport choices are similar, but the ticketing areas, rail levels, taxi stands and Call Van locations are different. T1 and T2 also use different departure times for some late-night services.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 242

- Page ITEM: 021 of 166
- File: `airport-transfer.html`
- Line/context: L93 - `script[type="application/ld+json"] > mainEntity.9.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Is Call Van the same as a private transfer?
  ```
- Protected tokens: None identified in this item.

### ITEM 243

- Page ITEM: 022 of 166
- File: `airport-transfer.html`
- Line/context: L96 - `script[type="application/ld+json"] > mainEntity.9.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  No. Incheon Airport's Call Van is a Korean commercial van service with its own airport conditions. “Private transfer” is a broader booking term for a vehicle reserved for one party. Some private-transfer bookings may use vans, but the terms are not interchangeable.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 244

- Page ITEM: 023 of 166
- File: `airport-transfer.html`
- Line/context: L169 - `p.page-hero__breadcrumb:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Home / Airport Transfer
  ```
- Protected tokens: None identified in this item.

### ITEM 245

- Page ITEM: 024 of 166
- File: `airport-transfer.html`
- Line/context: L170 - `h1.airport-page-hero__title`
- Element/type: H1
- Exact English:

  ```text
  Incheon Airport to Seoul: Which Transfer Is Best?
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 246

- Page ITEM: 025 of 166
- File: `airport-transfer.html`
- Line/context: L171 - `p.transfer-review-date:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Official fares and operating details checked August 18, 2026.
  ```
- Protected tokens: `August 18, 2026`

### ITEM 247

- Page ITEM: 026 of 166
- File: `airport-transfer.html`
- Line/context: L173 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Where you are staying matters more than the headline travel time. The AREX Express is a neat fit for Seoul Station, while the All-stop Train reaches Hongik University for Hongdae. An airport limousine bus can be easier when its stop is close to the hotel.
  ```
- Protected tokens: `AREX Express`, `Seoul Station`, `All-stop Train`, `Hongik University`, `Hongdae`

### ITEM 248

- Page ITEM: 027 of 166
- File: `airport-transfer.html`
- Line/context: L174 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  With a family, several large suitcases or a late arrival, the balance shifts. A taxi, Call Van or pre-booked private transfer costs more, but it can remove the station transfer, stairs and final walk that make an airport journey feel much longer than it looks on a timetable.
  ```
- Protected tokens: None identified in this item.

### ITEM 249

- Page ITEM: 028 of 166
- File: `airport-transfer.html`
- Line/context: L183 - `h2`
- Element/type: H2
- Exact English:

  ```text
  The Whole Trip Matters
  ```
- Protected tokens: None identified in this item.

### ITEM 250

- Page ITEM: 029 of 166
- File: `airport-transfer.html`
- Line/context: L184 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  A 43-minute rail ride ends at Seoul Station, not at your hotel. If another subway transfer, stairs or a long walk follows, the fastest first leg may not be the easiest arrival.
  ```
- Protected tokens: `43`, `Seoul Station`

### ITEM 251

- Page ITEM: 030 of 166
- File: `airport-transfer.html`
- Line/context: L185 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  The same route can also feel very different with luggage. One suitcase is usually manageable. Several large bags, a stroller or luggage for an entire family can turn a simple station transfer into the hardest part of the trip.
  ```
- Protected tokens: None identified in this item.

### ITEM 252

- Page ITEM: 031 of 166
- File: `airport-transfer.html`
- Line/context: L186 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  That is why the useful comparison runs all the way to the hotel door: where the train or bus leaves you, how much luggage is moving with you, how many people are sharing the trip and what time you are actually ready to leave the airport.
  ```
- Protected tokens: None identified in this item.

### ITEM 253

- Page ITEM: 032 of 166
- File: `airport-transfer.html`
- Line/context: L194 - `h2#transfer-comparison-heading`
- Element/type: H2
- Exact English:

  ```text
  Incheon Airport to Seoul at a Glance
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 254

- Page ITEM: 033 of 166
- File: `airport-transfer.html`
- Line/context: L195 - `p.transfer-chapter__answer`
- Element/type: Body text
- Exact English:

  ```text
  These are the current published fares and the practical differences that matter most. Bus fares vary by route and operator, while taxi and private-transfer totals depend on the destination, vehicle and travel conditions.
  ```
- Protected tokens: None identified in this item.

### ITEM 255

- Page ITEM: 034 of 166
- File: `airport-transfer.html`
- Line/context: L202 - `th:nth-of-type(1)`
- Element/type: Table header
- Exact English:

  ```text
  Option
  ```
- Protected tokens: None identified in this item.

### ITEM 256

- Page ITEM: 035 of 166
- File: `airport-transfer.html`
- Line/context: L203 - `th:nth-of-type(2)`
- Element/type: Table header
- Exact English:

  ```text
  Current fare
  ```
- Protected tokens: None identified in this item.

### ITEM 257

- Page ITEM: 036 of 166
- File: `airport-transfer.html`
- Line/context: L204 - `th:nth-of-type(3)`
- Element/type: Table header
- Exact English:

  ```text
  What the trip is like
  ```
- Protected tokens: None identified in this item.

### ITEM 258

- Page ITEM: 037 of 166
- File: `airport-transfer.html`
- Line/context: L205 - `th:nth-of-type(4)`
- Element/type: Table header
- Exact English:

  ```text
  Main limitation
  ```
- Protected tokens: None identified in this item.

### ITEM 259

- Page ITEM: 038 of 166
- File: `airport-transfer.html`
- Line/context: L210 - `th`
- Element/type: Table header
- Exact English:

  ```text
  AREX Express
  ```
- Protected tokens: `AREX Express`

### ITEM 260

- Page ITEM: 039 of 166
- File: `airport-transfer.html`
- Line/context: L211 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Adult ₩13,000 Child ₩9,500
  ```
- Protected tokens: `₩13,000`, `₩9,500`

### ITEM 261

- Page ITEM: 040 of 166
- File: `airport-transfer.html`
- Line/context: L212 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Reserved-seat nonstop train to Seoul Station. Published journey time is 43 minutes from T1 and 51 minutes from T2.
  ```
- Protected tokens: `Seoul Station`, `43 minutes`, `T1`, `51 minutes`, `T2`

### ITEM 262

- Page ITEM: 041 of 166
- File: `airport-transfer.html`
- Line/context: L213 - `td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  It does not stop at Hongik University, and the hotel may still require another transfer or walk.
  ```
- Protected tokens: `Hongik University`

### ITEM 263

- Page ITEM: 042 of 166
- File: `airport-transfer.html`
- Line/context: L216 - `th`
- Element/type: Table header
- Exact English:

  ```text
  AREX All-stop
  ```
- Protected tokens: `AREX All-stop`

### ITEM 264

- Page ITEM: 043 of 166
- File: `airport-transfer.html`
- Line/context: L217 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Seoul Station: ₩4,750 from T1 / ₩5,350 from T2 Hongik University: ₩4,650 from T1 / ₩5,250 from T2
  ```
- Protected tokens: `Seoul Station`, `₩4,750`, `T1`, `₩5,350`, `T2`, `Hongik University`, `₩4,650`, `₩5,250`

### ITEM 265

- Page ITEM: 044 of 166
- File: `airport-transfer.html`
- Line/context: L218 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Direct rail service to Hongik University and Seoul Station, using a transit card or single-use ticket.
  ```
- Protected tokens: `Hongik University`, `Seoul Station`

### ITEM 266

- Page ITEM: 045 of 166
- File: `airport-transfer.html`
- Line/context: L219 - `td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  It is slower, has no reserved seats and you keep your luggage with you.
  ```
- Protected tokens: None identified in this item.

### ITEM 267

- Page ITEM: 046 of 166
- File: `airport-transfer.html`
- Line/context: L222 - `th`
- Element/type: Table header
- Exact English:

  ```text
  Airport limousine bus
  ```
- Protected tokens: None identified in this item.

### ITEM 268

- Page ITEM: 047 of 166
- File: `airport-transfer.html`
- Line/context: L223 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Major Seoul routes currently range from ₩16,000 to ₩18,000 for adults. Child fares on the examples checked are ₩12,000.
  ```
- Protected tokens: `Seoul`, `₩16,000 to ₩18,000`, `₩12,000`

### ITEM 269

- Page ITEM: 048 of 166
- File: `airport-transfer.html`
- Line/context: L224 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Stored luggage and fewer station transfers when the bus stop is close to the hotel.
  ```
- Protected tokens: None identified in this item.

### ITEM 270

- Page ITEM: 049 of 166
- File: `airport-transfer.html`
- Line/context: L225 - `td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  Traffic varies, and the final walk from the actual stop can change the whole journey.
  ```
- Protected tokens: None identified in this item.

### ITEM 271

- Page ITEM: 050 of 166
- File: `airport-transfer.html`
- Line/context: L228 - `th`
- Element/type: Table header
- Exact English:

  ```text
  Regular / Deluxe-Jumbo taxi
  ```
- Protected tokens: None identified in this item.

### ITEM 272

- Page ITEM: 051 of 166
- File: `airport-transfer.html`
- Line/context: L229 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Regular taxi starts at ₩4,800 for 1.6 km. Deluxe, SUV and Jumbo services start at ₩7,000 for 3 km.
  ```
- Protected tokens: `₩4,800`, `1.6 km`, `SUV`, `₩7,000`, `3 km`

### ITEM 273

- Page ITEM: 052 of 166
- File: `airport-transfer.html`
- Line/context: L230 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Door-to-door travel from the official airport taxi stand.
  ```
- Protected tokens: None identified in this item.

### ITEM 274

- Page ITEM: 053 of 166
- File: `airport-transfer.html`
- Line/context: L231 - `td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  The total depends on destination, traffic, time and vehicle type. Passenger seats do not guarantee enough luggage space.
  ```
- Protected tokens: None identified in this item.

### ITEM 275

- Page ITEM: 054 of 166
- File: `airport-transfer.html`
- Line/context: L234 - `th`
- Element/type: Table header
- Exact English:

  ```text
  Official Airport Call Van
  ```
- Protected tokens: `Official Airport Call Van`

### ITEM 276

- Page ITEM: 055 of 166
- File: `airport-transfer.html`
- Line/context: L235 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Distance-based fare; tolls are separate.
  ```
- Protected tokens: None identified in this item.

### ITEM 277

- Page ITEM: 056 of 166
- File: `airport-transfer.html`
- Line/context: L236 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  A commercial van service described by Incheon Airport for groups of five or fewer with 20 kg of luggage per person.
  ```
- Protected tokens: `Incheon Airport`, `20 kg`

### ITEM 278

- Page ITEM: 057 of 166
- File: `airport-transfer.html`
- Line/context: L237 - `td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  Published operating hours are 08:00–21:00 and the airport's eligibility conditions apply.
  ```
- Protected tokens: `08:00`, `21:00`

### ITEM 279

- Page ITEM: 058 of 166
- File: `airport-transfer.html`
- Line/context: L240 - `th`
- Element/type: Table header
- Exact English:

  ```text
  Pre-booked private transfer
  ```
- Protected tokens: None identified in this item.

### ITEM 280

- Page ITEM: 059 of 166
- File: `airport-transfer.html`
- Line/context: L241 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Provider-specific.
  ```
- Protected tokens: None identified in this item.

### ITEM 281

- Page ITEM: 060 of 166
- File: `airport-transfer.html`
- Line/context: L242 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  A vehicle reserved in advance for one party, with a meeting arrangement set before arrival.
  ```
- Protected tokens: None identified in this item.

### ITEM 282

- Page ITEM: 061 of 166
- File: `airport-transfer.html`
- Line/context: L243 - `td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  Vehicle size, luggage, waiting time and cancellation terms depend on the individual booking.
  ```
- Protected tokens: None identified in this item.

### ITEM 283

- Page ITEM: 062 of 166
- File: `airport-transfer.html`
- Line/context: L249 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Fares and operating details were checked against the relevant transport operators and Incheon Airport on August 18, 2026. Routes, schedules and service conditions can change.
  ```
- Protected tokens: `Incheon Airport`, `August 18, 2026`

### ITEM 284

- Page ITEM: 063 of 166
- File: `airport-transfer.html`
- Line/context: L256 - `h2`
- Element/type: H2
- Exact English:

  ```text
  AREX: Seoul Station and Hongdae Are Different Trips
  ```
- Protected tokens: `AREX`, `Seoul Station`, `Hongdae`

### ITEM 285

- Page ITEM: 064 of 166
- File: `airport-transfer.html`
- Line/context: L258 - `h3:nth-of-type(1)`
- Element/type: H3
- Exact English:

  ```text
  The Express is built around Seoul Station
  ```
- Protected tokens: `Seoul Station`

### ITEM 286

- Page ITEM: 065 of 166
- File: `airport-transfer.html`
- Line/context: L259 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  The AREX Express makes the most sense when Seoul Station is genuinely useful for the rest of the journey. It runs nonstop from the airport with reserved seats. The current selling fare is ₩13,000 for adults and ₩9,500 for children, with published journey times of 43 minutes from T1 and 51 minutes from T2.
  ```
- Protected tokens: `AREX Express`, `Seoul Station`, `₩13,000`, `₩9,500`, `43 minutes`, `T1`, `51 minutes`, `T2`

### ITEM 287

- Page ITEM: 066 of 166
- File: `airport-transfer.html`
- Line/context: L260 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Current first departures are 05:16 from T2 and 05:24 from T1. The last departures are 22:40 from T2 and 22:48 from T1. Those times still have to work with immigration, baggage claim and the walk from the arrival hall to the airport station.
  ```
- Protected tokens: `05:16`, `T2`, `05:24`, `T1`, `22:40`, `22:48`

### ITEM 288

- Page ITEM: 067 of 166
- File: `airport-transfer.html`
- Line/context: L262 - `h3:nth-of-type(2)`
- Element/type: H3
- Exact English:

  ```text
  Hongdae is different
  ```
- Protected tokens: `Hongdae`

### ITEM 289

- Page ITEM: 068 of 166
- File: `airport-transfer.html`
- Line/context: L263 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  For Hongdae, the All-stop Train is usually the more direct rail journey because it stops at Hongik University. Adult transit-card fares are currently ₩4,650 from T1 and ₩5,250 from T2 to Hongik University.
  ```
- Protected tokens: `Hongdae`, `All-stop Train`, `Hongik University`, `₩4,650`, `T1`, `₩5,250`, `T2`

### ITEM 290

- Page ITEM: 069 of 166
- File: `airport-transfer.html`
- Line/context: L264 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  To Seoul Station, the equivalent fares are ₩4,750 from T1 and ₩5,350 from T2. Published travel times are about 59 minutes from T1 and 66 minutes from T2, with some trains taking a few minutes longer.
  ```
- Protected tokens: `Seoul Station`, `₩4,750`, `T1`, `₩5,350`, `T2`, `59 minutes`, `66 minutes`

### ITEM 291

- Page ITEM: 070 of 166
- File: `airport-transfer.html`
- Line/context: L266 - `h3:nth-of-type(3)`
- Element/type: H3
- Exact English:

  ```text
  Luggage changes the experience
  ```
- Protected tokens: None identified in this item.

### ITEM 292

- Page ITEM: 071 of 166
- File: `airport-transfer.html`
- Line/context: L267 - `p:nth-of-type(5)`
- Element/type: Body text
- Exact English:

  ```text
  AREX carriage rules allow no more than two items per passenger, each under 32 kg and under 158 cm in total dimensions. Even within those limits, several large suitcases can make the station transfer and final walk more tiring than the rail journey itself.
  ```
- Protected tokens: `AREX`, `32 kg`, `158`

### ITEM 293

- Page ITEM: 072 of 166
- File: `airport-transfer.html`
- Line/context: L268 - `p:nth-of-type(6)`
- Element/type: Body text
- Exact English:

  ```text
  For ticket details, station access and the difference between the two trains, see the full AREX guide. The T-money guide explains the transit-card side of the All-stop Train.
  ```
- Protected tokens: `AREX`, `T-money`, `All-stop Train`

### ITEM 294

- Page ITEM: 073 of 166
- File: `airport-transfer.html`
- Line/context: L276 - `h2`
- Element/type: H2
- Exact English:

  ```text
  The Airport Bus Gets Easier When the Stop Is Close
  ```
- Protected tokens: None identified in this item.

### ITEM 295

- Page ITEM: 074 of 166
- File: `airport-transfer.html`
- Line/context: L277 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  An airport limousine bus can remove a station transfer and give large bags a dedicated luggage compartment. The important part is not the neighborhood name on the route map but where the bus actually leaves you.
  ```
- Protected tokens: None identified in this item.

### ITEM 296

- Page ITEM: 075 of 166
- File: `airport-transfer.html`
- Line/context: L278 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  A stop one block from the hotel can make the bus much easier than rail. A stop across a wide road, several uphill blocks away or on the wrong side of a complicated intersection can erase that advantage.
  ```
- Protected tokens: None identified in this item.

### ITEM 297

- Page ITEM: 076 of 166
- File: `airport-transfer.html`
- Line/context: L280 - `h3:nth-of-type(1)`
- Element/type: H3
- Exact English:

  ```text
  Current fares vary by route
  ```
- Protected tokens: None identified in this item.

### ITEM 298

- Page ITEM: 077 of 166
- File: `airport-transfer.html`
- Line/context: L281 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  Airport Limousine routes 6001, 6002 and 6015 are currently ₩17,000 for adults and ₩12,000 for children. Route 6003 is ₩16,000 for adults and ₩12,000 for children. K Airport Limousine's central-Seoul routes are currently ₩18,000 for adults and ₩12,000 for children.
  ```
- Protected tokens: `6001`, `6002`, `6015`, `₩17,000`, `₩12,000`, `6003`, `₩16,000`, `K Airport Limousine`, `Seoul`, `₩18,000`

### ITEM 299

- Page ITEM: 078 of 166
- File: `airport-transfer.html`
- Line/context: L282 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  These are current examples rather than a universal Seoul airport-bus fare.
  ```
- Protected tokens: `Seoul`

### ITEM 300

- Page ITEM: 079 of 166
- File: `airport-transfer.html`
- Line/context: L284 - `h3:nth-of-type(2)`
- Element/type: H3
- Exact English:

  ```text
  Luggage rules are not identical
  ```
- Protected tokens: None identified in this item.

### ITEM 301

- Page ITEM: 080 of 166
- File: `airport-transfer.html`
- Line/context: L285 - `p:nth-of-type(5)`
- Element/type: Body text
- Exact English:

  ```text
  Airport Limousine currently describes free baggage as two pieces up to 28 inches and 20 kg each, or one piece larger than 28 inches.
  ```
- Protected tokens: `28 inches`, `20 kg`

### ITEM 302

- Page ITEM: 081 of 166
- File: `airport-transfer.html`
- Line/context: L286 - `p:nth-of-type(6)`
- Element/type: Body text
- Exact English:

  ```text
  K Airport Limousine's English FAQ and transport terms are not completely consistent with each other, so unusually large or heavy luggage is better treated as an operator-specific condition rather than a single rule for every airport bus.
  ```
- Protected tokens: `K Airport Limousine`

### ITEM 303

- Page ITEM: 082 of 166
- File: `airport-transfer.html`
- Line/context: L288 - `h3:nth-of-type(3)`
- Element/type: H3
- Exact English:

  ```text
  Terminal 2 has a different ticketing routine
  ```
- Protected tokens: `Terminal 2`

### ITEM 304

- Page ITEM: 083 of 166
- File: `airport-transfer.html`
- Line/context: L289 - `p:nth-of-type(7)`
- Element/type: Body text
- Exact English:

  ```text
  At T1, airport-bus ticketing is on the arrivals level. At T2, ticketing and boarding are concentrated in Transportation Center B1.
  ```
- Protected tokens: `T1`, `T2`, `Transportation Center`, `B1`

### ITEM 305

- Page ITEM: 084 of 166
- File: `airport-transfer.html`
- Line/context: L290 - `p:nth-of-type(8)`
- Element/type: Body text
- Exact English:

  ```text
  Since March 5, 2026, Seoul-bound Airport Limousine services from T2 require a ticket from the staffed counter or ticket machine before boarding.
  ```
- Protected tokens: `March 5, 2026`, `Seoul`, `T2`

### ITEM 306

- Page ITEM: 085 of 166
- File: `airport-transfer.html`
- Line/context: L291 - `p:nth-of-type(9)`
- Element/type: Body text
- Exact English:

  ```text
  Traffic remains the main trade-off. A bus can be the easier door-to-door journey and still take longer when the roads are congested.
  ```
- Protected tokens: None identified in this item.

### ITEM 307

- Page ITEM: 086 of 166
- File: `airport-transfer.html`
- Line/context: L292 - `p:nth-of-type(10)`
- Element/type: Body text
- Exact English:

  ```text
  The airport bus guide covers routes and terminal stops in more detail.
  ```
- Protected tokens: None identified in this item.

### ITEM 308

- Page ITEM: 087 of 166
- File: `airport-transfer.html`
- Line/context: L300 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Taxi: Door to Door, Until Luggage Changes the Vehicle
  ```
- Protected tokens: None identified in this item.

### ITEM 309

- Page ITEM: 088 of 166
- File: `airport-transfer.html`
- Line/context: L301 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  A taxi removes the station transfer completely. At the official airport stands, a regular Seoul taxi currently starts at ₩4,800 for the first 1.6 km. Deluxe, SUV and Jumbo services start at ₩7,000 for the first 3 km.
  ```
- Protected tokens: `Seoul`, `₩4,800`, `1.6 km`, `SUV`, `₩7,000`, `3 km`

### ITEM 310

- Page ITEM: 089 of 166
- File: `airport-transfer.html`
- Line/context: L302 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Regular-taxi late-night surcharges are 20% from 22:00–23:00, 40% from 23:00–02:00 and 20% from 02:00–04:00. Deluxe, SUV and van services use a 20% late-night surcharge from 22:00–04:00.
  ```
- Protected tokens: `20`, `22:00`, `23:00`, `40`, `02:00`, `04:00`, `SUV`

### ITEM 311

- Page ITEM: 090 of 166
- File: `airport-transfer.html`
- Line/context: L303 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  Passenger count is only half of the space question. Four people may fit in a car while four large suitcases do not. The larger taxi categories become more useful when luggage, rather than seats, is the limiting factor.
  ```
- Protected tokens: None identified in this item.

### ITEM 312

- Page ITEM: 091 of 166
- File: `airport-transfer.html`
- Line/context: L305 - `h3:nth-of-type(1)`
- Element/type: H3
- Exact English:

  ```text
  Official taxi stands
  ```
- Protected tokens: None identified in this item.

### ITEM 313

- Page ITEM: 092 of 166
- File: `airport-transfer.html`
- Line/context: L306 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  At T1, regular Seoul taxis use 5C, 6C and 6D. Deluxe and Jumbo taxis use 7C and 8C, while International Taxi uses 4C.
  ```
- Protected tokens: `T1`, `Seoul`, `5C`, `6C`, `7C`, `8C`, `International Taxi`, `4C`

### ITEM 314

- Page ITEM: 093 of 166
- File: `airport-transfer.html`
- Line/context: L307 - `p:nth-of-type(5)`
- Element/type: Body text
- Exact English:

  ```text
  At T2, regular Seoul taxis use 7C, Deluxe and Jumbo taxis use 7D, and International Taxi uses 3C.
  ```
- Protected tokens: `T2`, `Seoul`, `7C`, `7D`, `International Taxi`, `3C`

### ITEM 315

- Page ITEM: 094 of 166
- File: `airport-transfer.html`
- Line/context: L309 - `h3:nth-of-type(2)`
- Element/type: H3
- Exact English:

  ```text
  International Taxi
  ```
- Protected tokens: `International Taxi`

### ITEM 316

- Page ITEM: 095 of 166
- File: `airport-transfer.html`
- Line/context: L310 - `p:nth-of-type(6)`
- Element/type: Body text
- Exact English:

  ```text
  International Taxi uses Seoul zone fares rather than the ordinary meter structure for its airport service. Current published ranges are ₩70,000–95,000 for sedans and ₩100,000–140,000 for larger vehicles, depending on the destination zone.
  ```
- Protected tokens: `International Taxi`, `Seoul`, `₩70,000–95,000`, `₩100,000–140,000`

### ITEM 317

- Page ITEM: 096 of 166
- File: `airport-transfer.html`
- Line/context: L311 - `p:nth-of-type(7)`
- Element/type: Body text
- Exact English:

  ```text
  Official information is not completely consistent about how tolls are described across every International Taxi service, so the terms attached to the actual reservation are the safer reference than applying one toll rule to every booking.
  ```
- Protected tokens: `International Taxi`

### ITEM 318

- Page ITEM: 097 of 166
- File: `airport-transfer.html`
- Line/context: L312 - `p:nth-of-type(8)`
- Element/type: Body text
- Exact English:

  ```text
  The taxi guide explains Seoul taxi types, payment and fare structure in more detail.
  ```
- Protected tokens: `Seoul`

### ITEM 319

- Page ITEM: 098 of 166
- File: `airport-transfer.html`
- Line/context: L320 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Call Van and Pre-booked Private Transfers
  ```
- Protected tokens: None identified in this item.

### ITEM 320

- Page ITEM: 099 of 166
- File: `airport-transfer.html`
- Line/context: L321 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Korea's airport Call Van is a commercial van service. It is not the universal English name for every private transfer sold online.
  ```
- Protected tokens: `Korea`

### ITEM 321

- Page ITEM: 100 of 166
- File: `airport-transfer.html`
- Line/context: L322 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Incheon Airport currently describes the service for groups of five or fewer with 20 kg of luggage per person. The information counter is between exits 12 and 13 at T1 and near exit 7 at T2. Pickup is at 10C in T1 and 8D in T2. Published operating hours are 08:00–21:00, fares are distance-based and tolls are separate.
  ```
- Protected tokens: `Incheon Airport`, `20 kg`, `12`, `13`, `T1`, `7`, `T2`, `10C`, `8D`, `08:00`, `21:00`

### ITEM 322

- Page ITEM: 101 of 166
- File: `airport-transfer.html`
- Line/context: L323 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  A private transfer is the broader booking term travelers see on international platforms. The vehicle, luggage allowance, meeting point, waiting time and cancellation terms belong to the individual reservation rather than to the words “private transfer” themselves.
  ```
- Protected tokens: None identified in this item.

### ITEM 323

- Page ITEM: 102 of 166
- File: `airport-transfer.html`
- Line/context: L324 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  For larger groups, luggage capacity and pre-booking details, see the Call Van / Private Transfer guide.
  ```
- Protected tokens: None identified in this item.

### ITEM 324

- Page ITEM: 103 of 166
- File: `airport-transfer.html`
- Line/context: L332 - `h2`
- Element/type: H2
- Exact English:

  ```text
  A Late Arrival Is About When You Leave the Terminal
  ```
- Protected tokens: None identified in this item.

### ITEM 325

- Page ITEM: 104 of 166
- File: `airport-transfer.html`
- Line/context: L333 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  A flight landing late in the evening does not put you outside the terminal at that time. Immigration, baggage claim and customs come first, and that gap matters when the last convenient train or bus is approaching.
  ```
- Protected tokens: None identified in this item.

### ITEM 326

- Page ITEM: 105 of 166
- File: `airport-transfer.html`
- Line/context: L334 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Incheon Airport currently lists N6000, N6002, N6701 and N6703 among its overnight bus services. N6000 and N6002 are currently ₩17,000 for adults and ₩10,000 for children. N6701 and N6703 are ₩18,000 for adults and ₩12,000 for children.
  ```
- Protected tokens: `Incheon Airport`, `N6000`, `N6002`, `N6701`, `N6703`, `₩17,000`, `₩10,000`, `₩18,000`, `₩12,000`

### ITEM 327

- Page ITEM: 106 of 166
- File: `airport-transfer.html`
- Line/context: L335 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  T1 and T2 use different departure times, so the airport's current timetable is more useful than a static list copied into a travel plan.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 328

- Page ITEM: 107 of 166
- File: `airport-transfer.html`
- Line/context: L336 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  If the usable train or night-bus window has already closed, the official taxi stand or a pre-booked vehicle becomes the practical alternative.
  ```
- Protected tokens: None identified in this item.

### ITEM 329

- Page ITEM: 108 of 166
- File: `airport-transfer.html`
- Line/context: L337 - `p:nth-of-type(5)`
- Element/type: Body text
- Exact English:

  ```text
  The Arrival Guide explains the airport steps that come before you reach the public arrival hall.
  ```
- Protected tokens: None identified in this item.

### ITEM 330

- Page ITEM: 109 of 166
- File: `airport-transfer.html`
- Line/context: L345 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Terminal 1 and Terminal 2 Work Differently
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 331

- Page ITEM: 110 of 166
- File: `airport-transfer.html`
- Line/context: L346 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  The transport choices are largely the same at both terminals, but the ticketing areas, rail levels and vehicle stands are not.
  ```
- Protected tokens: None identified in this item.

### ITEM 332

- Page ITEM: 111 of 166
- File: `airport-transfer.html`
- Line/context: L348 - `h3:nth-of-type(1)`
- Element/type: H3
- Exact English:

  ```text
  Terminal 1
  ```
- Protected tokens: `Terminal 1`

### ITEM 333

- Page ITEM: 112 of 166
- File: `airport-transfer.html`
- Line/context: L349 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Airport buses leave from the arrivals level. Ticketing is available inside near exits 4 and 9, with additional facilities outside near exits 4, 6, 7, 8, 11 and 13.
  ```
- Protected tokens: `4`, `9`, `6`, `7`, `8`, `11`, `13`

### ITEM 334

- Page ITEM: 113 of 166
- File: `airport-transfer.html`
- Line/context: L350 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  AREX ticketing and guidance are in Transportation Center B1, while trains use B4.
  ```
- Protected tokens: `AREX`, `Transportation Center`, `B1`, `B4`

### ITEM 335

- Page ITEM: 114 of 166
- File: `airport-transfer.html`
- Line/context: L351 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  Regular Seoul taxis use 5C, 6C and 6D; Deluxe and Jumbo taxis use 7C and 8C; International Taxi uses 4C.
  ```
- Protected tokens: `Seoul`, `5C`, `6C`, `7C`, `8C`, `International Taxi`, `4C`

### ITEM 336

- Page ITEM: 115 of 166
- File: `airport-transfer.html`
- Line/context: L352 - `p:nth-of-type(5)`
- Element/type: Body text
- Exact English:

  ```text
  The Call Van information counter is between exits 12 and 13, with pickup at 10C.
  ```
- Protected tokens: `12`, `13`, `10C`

### ITEM 337

- Page ITEM: 116 of 166
- File: `airport-transfer.html`
- Line/context: L354 - `h3:nth-of-type(2)`
- Element/type: H3
- Exact English:

  ```text
  Terminal 2
  ```
- Protected tokens: `Terminal 2`

### ITEM 338

- Page ITEM: 117 of 166
- File: `airport-transfer.html`
- Line/context: L355 - `p:nth-of-type(6)`
- Element/type: Body text
- Exact English:

  ```text
  Airport-bus ticketing, guidance and boarding are in Transportation Center B1.
  ```
- Protected tokens: `Transportation Center`, `B1`

### ITEM 339

- Page ITEM: 118 of 166
- File: `airport-transfer.html`
- Line/context: L356 - `p:nth-of-type(7)`
- Element/type: Body text
- Exact English:

  ```text
  AREX ticketing and guidance are also on B1, while trains use B3.
  ```
- Protected tokens: `AREX`, `B1`, `B3`

### ITEM 340

- Page ITEM: 119 of 166
- File: `airport-transfer.html`
- Line/context: L357 - `p:nth-of-type(8)`
- Element/type: Body text
- Exact English:

  ```text
  Regular Seoul taxis use 7C; Deluxe and Jumbo taxis use 7D; International Taxi uses 3C.
  ```
- Protected tokens: `Seoul`, `7C`, `7D`, `International Taxi`, `3C`

### ITEM 341

- Page ITEM: 120 of 166
- File: `airport-transfer.html`
- Line/context: L358 - `p:nth-of-type(9)`
- Element/type: Body text
- Exact English:

  ```text
  The Call Van information counter is near exit 7, with pickup at 8D.
  ```
- Protected tokens: `7`, `8D`

### ITEM 342

- Page ITEM: 121 of 166
- File: `airport-transfer.html`
- Line/context: L359 - `p:nth-of-type(10)`
- Element/type: Body text
- Exact English:

  ```text
  The Incheon Airport guide has the wider arrival-hall context for both terminals.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 343

- Page ITEM: 122 of 166
- File: `airport-transfer.html`
- Line/context: L367 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Which Airport Transfer Fits Your Trip?
  ```
- Protected tokens: None identified in this item.

### ITEM 344

- Page ITEM: 123 of 166
- File: `airport-transfer.html`
- Line/context: L368 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Seoul Station is the clearest case for the AREX Express. Hongdae is usually simpler on the All-stop Train because it stops at Hongik University.
  ```
- Protected tokens: `Seoul Station`, `AREX Express`, `Hongdae`, `All-stop Train`, `Hongik University`

### ITEM 345

- Page ITEM: 124 of 166
- File: `airport-transfer.html`
- Line/context: L369 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  An airport bus becomes attractive when its actual stop is close to the hotel. A taxi becomes more useful as the final walk, luggage or group size gets harder to manage.
  ```
- Protected tokens: None identified in this item.

### ITEM 346

- Page ITEM: 125 of 166
- File: `airport-transfer.html`
- Line/context: L370 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  For a family, several large suitcases or a late arrival, Call Van or a pre-booked private transfer can justify the extra cost by removing a station transfer and the last kilometre with luggage.
  ```
- Protected tokens: None identified in this item.

### ITEM 347

- Page ITEM: 126 of 166
- File: `airport-transfer.html`
- Line/context: L371 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  The fastest first ride is not always the easiest arrival. The journey that matters is the one that ends at the hotel door.
  ```
- Protected tokens: None identified in this item.

### ITEM 348

- Page ITEM: 127 of 166
- File: `airport-transfer.html`
- Line/context: L372 - `p:nth-of-type(5)`
- Element/type: Body text
- Exact English:

  ```text
  Your transport choice and your hotel area are linked. If the accommodation is not fixed yet, compare Hongdae, Gongdeok, Seoul Station and Myeongdong by luggage handling, the final walk and the rest of your Seoul itinerary.
  ```
- Protected tokens: `Hongdae`, `Gongdeok`, `Seoul Station`, `Myeongdong`, `Seoul`

### ITEM 349

- Page ITEM: 128 of 166
- File: `airport-transfer.html`
- Line/context: L373 - `p:nth-of-type(6)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Compare where to stay for airport access →
  ```
- Protected tokens: None identified in this item.

### ITEM 350

- Page ITEM: 129 of 166
- File: `airport-transfer.html`
- Line/context: L381 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Frequently Asked Questions
  ```
- Protected tokens: None identified in this item.

### ITEM 351

- Page ITEM: 130 of 166
- File: `airport-transfer.html`
- Line/context: L385 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Is AREX or the airport bus better?
  ```
- Protected tokens: `AREX`

### ITEM 352

- Page ITEM: 131 of 166
- File: `airport-transfer.html`
- Line/context: L386 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  AREX is more predictable and works especially well for Seoul Station or Hongdae. The airport bus can be easier when its actual stop is close to the hotel and removes another station transfer. The final walk matters as much as the headline travel time.
  ```
- Protected tokens: `AREX`, `Seoul Station`, `Hongdae`

### ITEM 353

- Page ITEM: 132 of 166
- File: `airport-transfer.html`
- Line/context: L389 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What is the cheapest way from Incheon Airport to Seoul?
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 354

- Page ITEM: 133 of 166
- File: `airport-transfer.html`
- Line/context: L390 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  The AREX All-stop Train is usually the cheapest rail option. Current adult transit-card fares are ₩4,650 from T1 and ₩5,250 from T2 to Hongik University, and ₩4,750 from T1 and ₩5,350 from T2 to Seoul Station.
  ```
- Protected tokens: `AREX All-stop Train`, `₩4,650`, `T1`, `₩5,250`, `T2`, `Hongik University`, `₩4,750`, `₩5,350`, `Seoul Station`

### ITEM 355

- Page ITEM: 134 of 166
- File: `airport-transfer.html`
- Line/context: L393 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What is the fastest way to Seoul Station?
  ```
- Protected tokens: `Seoul Station`

### ITEM 356

- Page ITEM: 135 of 166
- File: `airport-transfer.html`
- Line/context: L394 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  The AREX Express has published airport-to-Seoul Station journey times of 43 minutes from T1 and 51 minutes from T2. The full trip still includes the walk to the airport station and whatever comes after Seoul Station.
  ```
- Protected tokens: `AREX Express`, `Seoul Station`, `43 minutes`, `T1`, `51 minutes`, `T2`

### ITEM 357

- Page ITEM: 136 of 166
- File: `airport-transfer.html`
- Line/context: L397 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Does the AREX Express stop at Hongdae?
  ```
- Protected tokens: `AREX Express`, `Hongdae`

### ITEM 358

- Page ITEM: 137 of 166
- File: `airport-transfer.html`
- Line/context: L398 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  No. The AREX Express runs directly to Seoul Station and does not stop at Hongik University. The All-stop Train stops at Hongik University and is usually the more direct rail option for Hongdae.
  ```
- Protected tokens: `AREX Express`, `Seoul Station`, `Hongik University`, `All-stop Train`, `Hongdae`

### ITEM 359

- Page ITEM: 138 of 166
- File: `airport-transfer.html`
- Line/context: L401 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Can I use T-money on the AREX?
  ```
- Protected tokens: `T-money`, `AREX`

### ITEM 360

- Page ITEM: 139 of 166
- File: `airport-transfer.html`
- Line/context: L402 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  T-money can be used on the AREX All-stop Train. The AREX Express uses a separate ticket and reserved-seat system.
  ```
- Protected tokens: `T-money`, `AREX All-stop Train`, `AREX Express`

### ITEM 361

- Page ITEM: 140 of 166
- File: `airport-transfer.html`
- Line/context: L405 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What works better with several large suitcases?
  ```
- Protected tokens: None identified in this item.

### ITEM 362

- Page ITEM: 141 of 166
- File: `airport-transfer.html`
- Line/context: L406 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  The answer changes with the number and size of the bags. An airport bus can work well when its stop is close to the hotel, while a larger taxi, Call Van or pre-booked transfer becomes more useful when moving the luggage through a station would be difficult. Passenger capacity and luggage capacity are separate limits.
  ```
- Protected tokens: None identified in this item.

### ITEM 363

- Page ITEM: 142 of 166
- File: `airport-transfer.html`
- Line/context: L409 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What can I use after midnight?
  ```
- Protected tokens: None identified in this item.

### ITEM 364

- Page ITEM: 143 of 166
- File: `airport-transfer.html`
- Line/context: L410 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Incheon Airport currently lists overnight services including N6000, N6002, N6701 and N6703, with different departure times at T1 and T2. Official taxis and pre-booked vehicles are alternatives when the useful train or night-bus window has closed.
  ```
- Protected tokens: `Incheon Airport`, `N6000`, `N6002`, `N6701`, `N6703`, `T1`, `T2`

### ITEM 365

- Page ITEM: 144 of 166
- File: `airport-transfer.html`
- Line/context: L413 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  How do taxis from Incheon Airport work?
  ```
- Protected tokens: `Incheon Airport`

### ITEM 366

- Page ITEM: 145 of 166
- File: `airport-transfer.html`
- Line/context: L414 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Official airport taxi stands serve regular, Deluxe, Jumbo and International Taxi services. A regular Seoul taxi currently starts at ₩4,800 for 1.6 km, while Deluxe, SUV and Jumbo services start at ₩7,000 for 3 km. The final fare depends on the service, destination, traffic and time of travel.
  ```
- Protected tokens: `International Taxi`, `Seoul`, `₩4,800`, `1.6 km`, `SUV`, `₩7,000`, `3 km`

### ITEM 367

- Page ITEM: 146 of 166
- File: `airport-transfer.html`
- Line/context: L417 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Does Terminal 1 or Terminal 2 change the route?
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 368

- Page ITEM: 147 of 166
- File: `airport-transfer.html`
- Line/context: L418 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  The main transport choices are similar, but the ticketing areas, rail levels, taxi stands and Call Van locations are different. T1 and T2 also use different departure times for some late-night services.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 369

- Page ITEM: 148 of 166
- File: `airport-transfer.html`
- Line/context: L421 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Is Call Van the same as a private transfer?
  ```
- Protected tokens: None identified in this item.

### ITEM 370

- Page ITEM: 149 of 166
- File: `airport-transfer.html`
- Line/context: L422 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  No. Incheon Airport's Call Van is a Korean commercial van service with its own airport conditions. “Private transfer” is a broader booking term for a vehicle reserved for one party. Some private-transfer bookings may use vans, but the terms are not interchangeable.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 371

- Page ITEM: 150 of 166
- File: `airport-transfer.html`
- Line/context: L431 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Official Sources
  ```
- Protected tokens: None identified in this item.

### ITEM 372

- Page ITEM: 151 of 166
- File: `airport-transfer.html`
- Line/context: L432 - `p.transfer-source-intro`
- Element/type: Body text
- Exact English:

  ```text
  Fares, schedules, baggage rules and operating locations can change. The links below are the primary sources used for the information on this page.
  ```
- Protected tokens: None identified in this item.

### ITEM 373

- Page ITEM: 152 of 166
- File: `airport-transfer.html`
- Line/context: L435 - `li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  AREX Express fares and journey times
  ```
- Protected tokens: `AREX Express`

### ITEM 374

- Page ITEM: 153 of 166
- File: `airport-transfer.html`
- Line/context: L436 - `li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  AREX Express timetable
  ```
- Protected tokens: `AREX Express`

### ITEM 375

- Page ITEM: 154 of 166
- File: `airport-transfer.html`
- Line/context: L437 - `li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  AREX All-stop fares
  ```
- Protected tokens: `AREX All-stop`

### ITEM 376

- Page ITEM: 155 of 166
- File: `airport-transfer.html`
- Line/context: L438 - `li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  AREX All-stop service
  ```
- Protected tokens: `AREX All-stop`

### ITEM 377

- Page ITEM: 156 of 166
- File: `airport-transfer.html`
- Line/context: L439 - `li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  AREX transport terms and baggage
  ```
- Protected tokens: `AREX`

### ITEM 378

- Page ITEM: 157 of 166
- File: `airport-transfer.html`
- Line/context: L440 - `li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  Incheon Airport bus information — Terminal 1
  ```
- Protected tokens: `Incheon Airport`, `Terminal 1`

### ITEM 379

- Page ITEM: 158 of 166
- File: `airport-transfer.html`
- Line/context: L441 - `li:nth-of-type(7)`
- Element/type: List text
- Exact English:

  ```text
  Incheon Airport bus information — Terminal 2
  ```
- Protected tokens: `Incheon Airport`, `Terminal 2`

### ITEM 380

- Page ITEM: 159 of 166
- File: `airport-transfer.html`
- Line/context: L442 - `li:nth-of-type(8)`
- Element/type: List text
- Exact English:

  ```text
  Incheon Airport late-night buses — Terminal 1
  ```
- Protected tokens: `Incheon Airport`, `Terminal 1`

### ITEM 381

- Page ITEM: 160 of 166
- File: `airport-transfer.html`
- Line/context: L443 - `li:nth-of-type(9)`
- Element/type: List text
- Exact English:

  ```text
  Incheon Airport late-night buses — Terminal 2
  ```
- Protected tokens: `Incheon Airport`, `Terminal 2`

### ITEM 382

- Page ITEM: 161 of 166
- File: `airport-transfer.html`
- Line/context: L444 - `li:nth-of-type(10)`
- Element/type: List text
- Exact English:

  ```text
  Incheon Airport taxi guide
  ```
- Protected tokens: `Incheon Airport`

### ITEM 383

- Page ITEM: 162 of 166
- File: `airport-transfer.html`
- Line/context: L445 - `li:nth-of-type(11)`
- Element/type: List text
- Exact English:

  ```text
  Incheon Airport Call Van guide
  ```
- Protected tokens: `Incheon Airport`

### ITEM 384

- Page ITEM: 163 of 166
- File: `airport-transfer.html`
- Line/context: L446 - `li:nth-of-type(12)`
- Element/type: List text
- Exact English:

  ```text
  Airport Limousine routes and fares
  ```
- Protected tokens: None identified in this item.

### ITEM 385

- Page ITEM: 164 of 166
- File: `airport-transfer.html`
- Line/context: L447 - `li:nth-of-type(13)`
- Element/type: List text
- Exact English:

  ```text
  K Airport Limousine fares
  ```
- Protected tokens: `K Airport Limousine`

### ITEM 386

- Page ITEM: 165 of 166
- File: `airport-transfer.html`
- Line/context: L448 - `li:nth-of-type(14)`
- Element/type: List text
- Exact English:

  ```text
  Seoul taxi information
  ```
- Protected tokens: `Seoul`

### ITEM 387

- Page ITEM: 166 of 166
- File: `airport-transfer.html`
- Line/context: L449 - `li:nth-of-type(15)`
- Element/type: List text
- Exact English:

  ```text
  International Taxi
  ```
- Protected tokens: `International Taxi`


## PAGE - arex.html

- English source: `arex.html`
- Source SHA-256: `17d3fb49924732eb4db5fab1871b224bd11a8ff52799ad1c13f485c19b57d598`
- Extracted ITEM count: 221

### ITEM 388

- Page ITEM: 001 of 221
- File: `arex.html`
- Line/context: L6 - `html > head > meta[name="description"] @content`
- Element/type: Meta description
- Exact English:

  ```text
  Compare AREX Express and All-Stop trains from Incheon Airport to Seoul by price, travel time, stops, tickets, T-money, luggage and late-night options.
  ```
- Protected tokens: `AREX Express`, `Incheon Airport`, `Seoul`, `T-money`

### ITEM 389

- Page ITEM: 002 of 221
- File: `arex.html`
- Line/context: L9 - `html > head > title`
- Element/type: Title
- Exact English:

  ```text
  Incheon Airport to Seoul AREX: Price, Time & Train Guide | Korea Inside
  ```
- Protected tokens: `Incheon Airport`, `Seoul`, `AREX`, `Korea Inside`

### ITEM 390

- Page ITEM: 003 of 221
- File: `arex.html`
- Line/context: L80 - `p.page-hero__breadcrumb`
- Element/type: Body text
- Exact English:

  ```text
  Home / Airport / AREX
  ```
- Protected tokens: `AREX`

### ITEM 391

- Page ITEM: 004 of 221
- File: `arex.html`
- Line/context: L81 - `h1.arex-hero__title`
- Element/type: H1
- Exact English:

  ```text
  Incheon Airport to Seoul by AREX: Express vs All-Stop
  ```
- Protected tokens: `Incheon Airport`, `Seoul`, `AREX`

### ITEM 392

- Page ITEM: 005 of 221
- File: `arex.html`
- Line/context: L83 - `img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  AREX train traveling from Incheon Airport toward Seoul with Express and All-Stop route choices
  ```
- Protected tokens: `AREX`, `Incheon Airport`, `Seoul`

### ITEM 393

- Page ITEM: 006 of 221
- File: `arex.html`
- Line/context: L92 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  AREX has two train types. Express runs from the airport terminals to Seoul Station with an assigned seat, while All-Stop serves intermediate stations including Hongik University, Gongdeok and Gimpo Airport.
  ```
- Protected tokens: `AREX`, `Seoul Station`, `Hongik University`, `Gongdeok`, `Gimpo Airport`

### ITEM 394

- Page ITEM: 007 of 221
- File: `arex.html`
- Line/context: L93 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Express is faster to Seoul Station, but the better train depends on where you are staying. For some hotels, the slower All-Stop Train removes a transfer. The fastest train to Seoul Station is not always the fastest way to your hotel.
  ```
- Protected tokens: `Seoul Station`, `All-Stop Train`

### ITEM 395

- Page ITEM: 008 of 221
- File: `arex.html`
- Line/context: L95 - `div.transfer-fast-choice:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Quick AREX choice
  ```
- Protected tokens: `AREX`

### ITEM 396

- Page ITEM: 009 of 221
- File: `arex.html`
- Line/context: L97 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Express makes sense when...
  ```
- Protected tokens: None identified in this item.

### ITEM 397

- Page ITEM: 010 of 221
- File: `arex.html`
- Line/context: L98 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Your hotel is near Seoul Station, your onward connection starts there, or an assigned seat matters more than the lower fare.
  ```
- Protected tokens: `Seoul Station`

### ITEM 398

- Page ITEM: 011 of 221
- File: `arex.html`
- Line/context: L101 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  All-Stop makes more sense when...
  ```
- Protected tokens: None identified in this item.

### ITEM 399

- Page ITEM: 012 of 221
- File: `arex.html`
- Line/context: L102 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  You are staying near Hongdae or Gongdeok, connecting at Gimpo Airport, or using T-money for the lower distance-based fare.
  ```
- Protected tokens: `Hongdae`, `Gongdeok`, `Gimpo Airport`, `T-money`

### ITEM 400

- Page ITEM: 013 of 221
- File: `arex.html`
- Line/context: L105 - `nav.arex-quick-links @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  AREX guide chapters
  ```
- Protected tokens: `AREX`

### ITEM 401

- Page ITEM: 014 of 221
- File: `arex.html`
- Line/context: L106 - `a.arex-pill:nth-of-type(1)`
- Element/type: Visible link text
- Exact English:

  ```text
  Express vs All-Stop
  ```
- Protected tokens: None identified in this item.

### ITEM 402

- Page ITEM: 015 of 221
- File: `arex.html`
- Line/context: L107 - `a.arex-pill:nth-of-type(2)`
- Element/type: Visible link text
- Exact English:

  ```text
  Stations
  ```
- Protected tokens: None identified in this item.

### ITEM 403

- Page ITEM: 016 of 221
- File: `arex.html`
- Line/context: L108 - `a.arex-pill:nth-of-type(3)`
- Element/type: Visible link text
- Exact English:

  ```text
  Tickets
  ```
- Protected tokens: None identified in this item.

### ITEM 404

- Page ITEM: 017 of 221
- File: `arex.html`
- Line/context: L109 - `a.arex-pill:nth-of-type(4)`
- Element/type: Visible link text
- Exact English:

  ```text
  Late arrivals
  ```
- Protected tokens: None identified in this item.

### ITEM 405

- Page ITEM: 018 of 221
- File: `arex.html`
- Line/context: L119 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Express or All-Stop?
  ```
- Protected tokens: None identified in this item.

### ITEM 406

- Page ITEM: 019 of 221
- File: `arex.html`
- Line/context: L120 - `p.transfer-chapter__answer`
- Element/type: Body text
- Exact English:

  ```text
  Both trains connect Incheon Airport with Seoul, but they follow different stopping patterns and use different tickets.
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 407

- Page ITEM: 020 of 221
- File: `arex.html`
- Line/context: L124 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Express Train
  ```
- Protected tokens: `Express Train`

### ITEM 408

- Page ITEM: 021 of 221
- File: `arex.html`
- Line/context: L125 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Express runs from Terminal 2 through Terminal 1 to Seoul Station without intermediate passenger stops. It has an assigned seat and requires a separate ticket, which suits a direct trip to Seoul Station or an onward KTX connection with enough transfer time.
  ```
- Protected tokens: `Terminal 2`, `Terminal 1`, `Seoul Station`, `KTX`

### ITEM 409

- Page ITEM: 022 of 221
- File: `arex.html`
- Line/context: L128 - `h3`
- Element/type: H3
- Exact English:

  ```text
  All-Stop Train
  ```
- Protected tokens: `All-Stop Train`

### ITEM 410

- Page ITEM: 023 of 221
- File: `arex.html`
- Line/context: L129 - `p`
- Element/type: Body text
- Exact English:

  ```text
  All-Stop serves every AREX station, including Gimpo Airport, Hongik University and Gongdeok. It works like commuter rail and accepts T-money, so it can take you closer to a hotel before Seoul Station and avoid an unnecessary trip back across the city.
  ```
- Protected tokens: `AREX`, `Gimpo Airport`, `Hongik University`, `Gongdeok`, `T-money`, `Seoul Station`

### ITEM 411

- Page ITEM: 024 of 221
- File: `arex.html`
- Line/context: L132 - `p`
- Element/type: Body text
- Exact English:

  ```text
  “Seoul” is not a single destination. A train that saves time on the airport-to-Seoul Station segment can lose that advantage during a transfer, a long station walk or a trip back toward the west. When the rail route becomes awkward with luggage, compare the other Incheon Airport transfer options; an airport bus or official taxi may involve less handling.
  ```
- Protected tokens: `Seoul`, `Seoul Station`, `Incheon Airport`

### ITEM 412

- Page ITEM: 025 of 221
- File: `arex.html`
- Line/context: L139 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Express vs All-Stop Quick Comparison
  ```
- Protected tokens: None identified in this item.

### ITEM 413

- Page ITEM: 026 of 221
- File: `arex.html`
- Line/context: L140 - `p.transfer-chapter__answer`
- Element/type: Body text
- Exact English:

  ```text
  This table covers the differences that are useful at the ticket machine. The destination sections below explain what happens after you leave the train.
  ```
- Protected tokens: None identified in this item.

### ITEM 414

- Page ITEM: 027 of 221
- File: `arex.html`
- Line/context: L142 - `div.transfer-comparison.table-scroll @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  AREX Express and All-Stop comparison
  ```
- Protected tokens: `AREX Express`

### ITEM 415

- Page ITEM: 028 of 221
- File: `arex.html`
- Line/context: L144 - `caption`
- Element/type: Table caption
- Exact English:

  ```text
  AREX from Incheon Airport toward Seoul
  ```
- Protected tokens: `AREX`, `Incheon Airport`, `Seoul`

### ITEM 416

- Page ITEM: 029 of 221
- File: `arex.html`
- Line/context: L147 - `th:nth-of-type(1)`
- Element/type: Table header
- Exact English:

  ```text
  Feature
  ```
- Protected tokens: None identified in this item.

### ITEM 417

- Page ITEM: 030 of 221
- File: `arex.html`
- Line/context: L148 - `th:nth-of-type(2)`
- Element/type: Table header
- Exact English:

  ```text
  Express Train
  ```
- Protected tokens: `Express Train`

### ITEM 418

- Page ITEM: 031 of 221
- File: `arex.html`
- Line/context: L149 - `th:nth-of-type(3)`
- Element/type: Table header
- Exact English:

  ```text
  All-Stop Train
  ```
- Protected tokens: `All-Stop Train`

### ITEM 419

- Page ITEM: 032 of 221
- File: `arex.html`
- Line/context: L153 - `th`
- Element/type: Table header
- Exact English:

  ```text
  Stops
  ```
- Protected tokens: None identified in this item.

### ITEM 420

- Page ITEM: 033 of 221
- File: `arex.html`
- Line/context: L153 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Nonstop between the airport terminals and Seoul Station
  ```
- Protected tokens: `Seoul Station`

### ITEM 421

- Page ITEM: 034 of 221
- File: `arex.html`
- Line/context: L153 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  All stations, including Gimpo Airport, Hongik University and Gongdeok
  ```
- Protected tokens: `Gimpo Airport`, `Hongik University`, `Gongdeok`

### ITEM 422

- Page ITEM: 035 of 221
- File: `arex.html`
- Line/context: L154 - `th`
- Element/type: Table header
- Exact English:

  ```text
  T1 to Seoul Station
  ```
- Protected tokens: `T1`, `Seoul Station`

### ITEM 423

- Page ITEM: 036 of 221
- File: `arex.html`
- Line/context: L154 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  43 minutes
  ```
- Protected tokens: `43 minutes`

### ITEM 424

- Page ITEM: 037 of 221
- File: `arex.html`
- Line/context: L154 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  59 minutes; some trains take 2–6 minutes longer
  ```
- Protected tokens: `59 minutes`, `2–6 minutes`

### ITEM 425

- Page ITEM: 038 of 221
- File: `arex.html`
- Line/context: L155 - `th`
- Element/type: Table header
- Exact English:

  ```text
  T2 to Seoul Station
  ```
- Protected tokens: `T2`, `Seoul Station`

### ITEM 426

- Page ITEM: 039 of 221
- File: `arex.html`
- Line/context: L155 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  51 minutes
  ```
- Protected tokens: `51 minutes`

### ITEM 427

- Page ITEM: 040 of 221
- File: `arex.html`
- Line/context: L155 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  66 minutes; some trains take 2–6 minutes longer
  ```
- Protected tokens: `66 minutes`, `2–6 minutes`

### ITEM 428

- Page ITEM: 041 of 221
- File: `arex.html`
- Line/context: L156 - `th`
- Element/type: Table header
- Exact English:

  ```text
  Adult fare
  ```
- Protected tokens: None identified in this item.

### ITEM 429

- Page ITEM: 042 of 221
- File: `arex.html`
- Line/context: L156 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  ₩13,000 official selling fare
  ```
- Protected tokens: `₩13,000`

### ITEM 430

- Page ITEM: 043 of 221
- File: `arex.html`
- Line/context: L156 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  To Seoul Station with a transit card: T1 ₩4,750; T2 ₩5,350
  ```
- Protected tokens: `Seoul Station`, `T1`, `₩4,750`, `T2`, `₩5,350`

### ITEM 431

- Page ITEM: 044 of 221
- File: `arex.html`
- Line/context: L157 - `th`
- Element/type: Table header
- Exact English:

  ```text
  Seat
  ```
- Protected tokens: None identified in this item.

### ITEM 432

- Page ITEM: 045 of 221
- File: `arex.html`
- Line/context: L157 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Assigned seat
  ```
- Protected tokens: None identified in this item.

### ITEM 433

- Page ITEM: 046 of 221
- File: `arex.html`
- Line/context: L157 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Open commuter seating; a seat is not guaranteed
  ```
- Protected tokens: None identified in this item.

### ITEM 434

- Page ITEM: 047 of 221
- File: `arex.html`
- Line/context: L158 - `th`
- Element/type: Table header
- Exact English:

  ```text
  T-money
  ```
- Protected tokens: `T-money`

### ITEM 435

- Page ITEM: 048 of 221
- File: `arex.html`
- Line/context: L158 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  No; use a separate Express ticket
  ```
- Protected tokens: None identified in this item.

### ITEM 436

- Page ITEM: 049 of 221
- File: `arex.html`
- Line/context: L158 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Yes; tap a sufficiently funded transit card
  ```
- Protected tokens: None identified in this item.

### ITEM 437

- Page ITEM: 050 of 221
- File: `arex.html`
- Line/context: L159 - `th`
- Element/type: Table header
- Exact English:

  ```text
  Ticket type
  ```
- Protected tokens: None identified in this item.

### ITEM 438

- Page ITEM: 051 of 221
- File: `arex.html`
- Line/context: L159 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Reserved Express ticket with QR code or station-issued ticket
  ```
- Protected tokens: None identified in this item.

### ITEM 439

- Page ITEM: 052 of 221
- File: `arex.html`
- Line/context: L159 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  T-money or another accepted transit card, or a single-use transportation card
  ```
- Protected tokens: `T-money`

### ITEM 440

- Page ITEM: 053 of 221
- File: `arex.html`
- Line/context: L160 - `th`
- Element/type: Table header
- Exact English:

  ```text
  Main trade-off
  ```
- Protected tokens: None identified in this item.

### ITEM 441

- Page ITEM: 054 of 221
- File: `arex.html`
- Line/context: L160 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Faster and seated, but more expensive and limited to Seoul Station
  ```
- Protected tokens: `Seoul Station`

### ITEM 442

- Page ITEM: 055 of 221
- File: `arex.html`
- Line/context: L160 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Cheaper with more useful stops, but slower and without an assigned seat
  ```
- Protected tokens: None identified in this item.

### ITEM 443

- Page ITEM: 056 of 221
- File: `arex.html`
- Line/context: L170 - `h2`
- Element/type: H2
- Exact English:

  ```text
  How Much Does AREX Cost?
  ```
- Protected tokens: `AREX`

### ITEM 444

- Page ITEM: 057 of 221
- File: `arex.html`
- Line/context: L171 - `p.transfer-chapter__answer`
- Element/type: Body text
- Exact English:

  ```text
  Express has a fixed selling fare to Seoul Station. All-Stop charges by distance, so both your airport terminal and the station where you leave the train affect the price.
  ```
- Protected tokens: `Seoul Station`

### ITEM 445

- Page ITEM: 058 of 221
- File: `arex.html`
- Line/context: L173 - `h3:nth-of-type(1)`
- Element/type: H3
- Exact English:

  ```text
  Express Train fare
  ```
- Protected tokens: `Express Train`

### ITEM 446

- Page ITEM: 059 of 221
- File: `arex.html`
- Line/context: L174 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  The official booking system lists ₩13,000 for a nonmember adult, ₩12,500 for a member adult and ₩9,500 for a child. Express uses a separate reserved ticket rather than a distance-based transit fare, and the selling fare is the same from T1 or T2 to Seoul Station.
  ```
- Protected tokens: `₩13,000`, `₩12,500`, `₩9,500`, `T1`, `T2`, `Seoul Station`

### ITEM 447

- Page ITEM: 060 of 221
- File: `arex.html`
- Line/context: L175 - `h3:nth-of-type(2)`
- Element/type: H3
- Exact English:

  ```text
  All-Stop fare
  ```
- Protected tokens: None identified in this item.

### ITEM 448

- Page ITEM: 061 of 221
- File: `arex.html`
- Line/context: L176 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  With a prepaid or postpaid transit card, an adult trip from T1 costs ₩4,750 to Seoul Station or ₩4,650 to Hongik University. From T2, those fares are ₩5,350 and ₩5,250. The extra ₩600 reflects the additional distance from Terminal 2.
  ```
- Protected tokens: `T1`, `₩4,750`, `Seoul Station`, `₩4,650`, `Hongik University`, `T2`, `₩5,350`, `₩5,250`, `₩600`, `Terminal 2`

### ITEM 449

- Page ITEM: 062 of 221
- File: `arex.html`
- Line/context: L177 - `h3:nth-of-type(3)`
- Element/type: H3
- Exact English:

  ```text
  T-money versus a single-use card
  ```
- Protected tokens: `T-money`

### ITEM 450

- Page ITEM: 063 of 221
- File: `arex.html`
- Line/context: L178 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  T-money works on All-Stop by tapping in and out; it is not an Express ticket. An adult single-use transportation card costs the transit-card fare plus ₩100, with a ₩500 refundable deposit. The deposit is returned after the card is inserted into a refund machine at the destination.
  ```
- Protected tokens: `T-money`, `₩100,`, `₩500`

### ITEM 451

- Page ITEM: 064 of 221
- File: `arex.html`
- Line/context: L185 - `h2`
- Element/type: H2
- Exact English:

  ```text
  How Much Time Does Express Really Save?
  ```
- Protected tokens: None identified in this item.

### ITEM 452

- Page ITEM: 065 of 221
- File: `arex.html`
- Line/context: L186 - `p.transfer-chapter__answer`
- Element/type: Body text
- Exact English:

  ```text
  The published journey ends at Seoul Station. Your actual trip also includes the airport station walk, waiting, the next connection and the walk to the hotel.
  ```
- Protected tokens: `Seoul Station`

### ITEM 453

- Page ITEM: 066 of 221
- File: `arex.html`
- Line/context: L189 - `h3`
- Element/type: H3
- Exact English:

  ```text
  From Terminal 1
  ```
- Protected tokens: `Terminal 1`

### ITEM 454

- Page ITEM: 067 of 221
- File: `arex.html`
- Line/context: L189 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Express reaches Seoul Station in 43 minutes. All-Stop is scheduled for 59 minutes, although some services take another 2–6 minutes.
  ```
- Protected tokens: `Seoul Station`, `43 minutes`, `59 minutes`, `2–6 minutes`

### ITEM 455

- Page ITEM: 068 of 221
- File: `arex.html`
- Line/context: L190 - `h3`
- Element/type: H3
- Exact English:

  ```text
  From Terminal 2
  ```
- Protected tokens: `Terminal 2`

### ITEM 456

- Page ITEM: 069 of 221
- File: `arex.html`
- Line/context: L190 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Express takes 51 minutes to Seoul Station. All-Stop takes 66 minutes, with the same possible 2–6-minute extension on some trains.
  ```
- Protected tokens: `51 minutes`, `Seoul Station`, `66 minutes`, `2–6`

### ITEM 457

- Page ITEM: 070 of 221
- File: `arex.html`
- Line/context: L192 - `p`
- Element/type: Body text
- Exact English:

  ```text
  That makes Express about 15–16 minutes faster on the train itself. The gap can shrink when Express leaves you with a subway transfer at Seoul Station, while All-Stop reaches Hongik University or Gongdeok directly. Luggage, elevators and the final walk can matter more than the timetable difference.
  ```
- Protected tokens: `15–16 minutes`, `Seoul Station`, `Hongik University`, `Gongdeok`

### ITEM 458

- Page ITEM: 071 of 221
- File: `arex.html`
- Line/context: L199 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Main AREX Stations and Stops
  ```
- Protected tokens: `AREX`

### ITEM 459

- Page ITEM: 072 of 221
- File: `arex.html`
- Line/context: L200 - `p.transfer-chapter__answer`
- Element/type: Body text
- Exact English:

  ```text
  Express serves the two airport terminals and Seoul Station. All-Stop adds the intermediate stations that often decide whether the rest of the journey is straightforward.
  ```
- Protected tokens: `Seoul Station`

### ITEM 460

- Page ITEM: 073 of 221
- File: `arex.html`
- Line/context: L203 - `img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  AREX Express and All-Stop route map showing Incheon Airport terminals, major stations and Seoul Station
  ```
- Protected tokens: `AREX Express`, `Incheon Airport`, `Seoul Station`

### ITEM 461

- Page ITEM: 074 of 221
- File: `arex.html`
- Line/context: L204 - `figcaption`
- Element/type: Figcaption
- Exact English:

  ```text
  AREX Express and All-Stop routes from Incheon Airport to Seoul
  ```
- Protected tokens: `AREX Express`, `Incheon Airport`, `Seoul`

### ITEM 462

- Page ITEM: 075 of 221
- File: `arex.html`
- Line/context: L208 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Incheon Airport Terminal 2
  ```
- Protected tokens: `Incheon Airport Terminal 2`

### ITEM 463

- Page ITEM: 076 of 221
- File: `arex.html`
- Line/context: L209 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Express and All-Stop both begin here before calling at T1. The station is linked to the Terminal 2 transportation center, but it is separate from the T1 station, so the terminal shown on the flight booking matters.
  ```
- Protected tokens: `T1`, `Terminal 2`

### ITEM 464

- Page ITEM: 077 of 221
- File: `arex.html`
- Line/context: L212 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Incheon Airport Terminal 1
  ```
- Protected tokens: `Incheon Airport Terminal 1`

### ITEM 465

- Page ITEM: 078 of 221
- File: `arex.html`
- Line/context: L213 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Both trains also stop at the station beneath the Terminal 1 transportation center. Airport circulation can change, so the current Airport Railroad / AREX signs are more reliable than a gate number saved from an older guide.
  ```
- Protected tokens: `Terminal 1`, `Airport Railroad`, `AREX`

### ITEM 466

- Page ITEM: 079 of 221
- File: `arex.html`
- Line/context: L216 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Gimpo International Airport
  ```
- Protected tokens: `Gimpo International Airport`

### ITEM 467

- Page ITEM: 080 of 221
- File: `arex.html`
- Line/context: L217 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Only All-Stop serves Gimpo Airport, where passengers can connect with Subway Lines 5 and 9, the Gimpo Goldline and other rail services. A flight connection still includes a walk to the correct terminal, so the train arrival minute is not the boarding time.
  ```
- Protected tokens: `Gimpo Airport`, `Subway Lines 5 and 9`, `Gimpo Goldline`

### ITEM 468

- Page ITEM: 081 of 221
- File: `arex.html`
- Line/context: L220 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Hongik University
  ```
- Protected tokens: `Hongik University`

### ITEM 469

- Page ITEM: 082 of 221
- File: `arex.html`
- Line/context: L221 - `p`
- Element/type: Body text
- Exact English:

  ```text
  All-Stop reaches Hongik University directly, with connections to Subway Line 2 and the Gyeongui-Jungang Line. For a Hongdae hotel, riding Express to Seoul Station and then traveling back west is usually unnecessary. The station is large, though, and the correct exit can make a noticeable difference with luggage.
  ```
- Protected tokens: `Hongik University`, `Subway Line 2`, `Gyeongui-Jungang Line`, `Hongdae`, `Seoul Station`

### ITEM 470

- Page ITEM: 083 of 221
- File: `arex.html`
- Line/context: L224 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Gongdeok
  ```
- Protected tokens: `Gongdeok`

### ITEM 471

- Page ITEM: 084 of 221
- File: `arex.html`
- Line/context: L225 - `p`
- Element/type: Body text
- Exact English:

  ```text
  All-Stop calls at Gongdeok before Seoul Station. Hotels nearby need no second train, while Subway Lines 5 and 6 and the Gyeongui-Jungang Line provide onward connections. The exact hotel pin still determines which exit and transfer are practical.
  ```
- Protected tokens: `Gongdeok`, `Seoul Station`, `Subway Lines 5 and 6`, `Gyeongui-Jungang Line`

### ITEM 472

- Page ITEM: 085 of 221
- File: `arex.html`
- Line/context: L228 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Seoul Station
  ```
- Protected tokens: `Seoul Station`

### ITEM 473

- Page ITEM: 086 of 221
- File: `arex.html`
- Line/context: L229 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Both services terminate at Seoul Station, where Subway Lines 1 and 4, the Gyeongui-Jungang Line and national rail services connect. AREX platforms are deep in the complex, and reaching a subway or KTX platform involves corridors, elevators or escalators and additional walking.
  ```
- Protected tokens: `Seoul Station`, `Subway Lines 1 and 4`, `Gyeongui-Jungang Line`, `AREX`, `KTX`

### ITEM 474

- Page ITEM: 087 of 221
- File: `arex.html`
- Line/context: L238 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Where Are You Going After AREX?
  ```
- Protected tokens: `AREX`

### ITEM 475

- Page ITEM: 088 of 221
- File: `arex.html`
- Line/context: L239 - `p.transfer-chapter__answer`
- Element/type: Body text
- Exact English:

  ```text
  The useful question is not only which train is faster, but where you can leave AREX and how much travel remains after that.
  ```
- Protected tokens: `AREX`

### ITEM 476

- Page ITEM: 089 of 221
- File: `arex.html`
- Line/context: L242 - `img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  Guide to choosing an AREX train and station for Seoul Station, Hongdae, Gongdeok, Myeongdong, Gangnam and other destinations
  ```
- Protected tokens: `AREX`, `Seoul Station`, `Hongdae`, `Gongdeok`, `Myeongdong`, `Gangnam`

### ITEM 477

- Page ITEM: 090 of 221
- File: `arex.html`
- Line/context: L243 - `figcaption`
- Element/type: Figcaption
- Exact English:

  ```text
  AREX train and station options for major Seoul destinations
  ```
- Protected tokens: `AREX`, `Seoul`

### ITEM 478

- Page ITEM: 091 of 221
- File: `arex.html`
- Line/context: L246 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Seoul Station
  ```
- Protected tokens: `Seoul Station`

### ITEM 479

- Page ITEM: 092 of 221
- File: `arex.html`
- Line/context: L246 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Express gets there faster and provides an assigned seat; All-Stop reaches the same station for less. A nearby hotel may be walkable, but a hotel on the other side of the complex can still mean a long exit route. When the final walk looks awkward with bags, compare a bus stop or taxi drop-off near the address.
  ```
- Protected tokens: None identified in this item.

### ITEM 480

- Page ITEM: 093 of 221
- File: `arex.html`
- Line/context: L247 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Hongdae
  ```
- Protected tokens: `Hongdae`

### ITEM 481

- Page ITEM: 094 of 221
- File: `arex.html`
- Line/context: L247 - `p`
- Element/type: Body text
- Exact English:

  ```text
  All-Stop stops directly at Hongik University Station, so there is little reason to ride Express all the way to Seoul Station and then travel back west for a Hongdae hotel. The station is large, though, and the correct exit can make a noticeable difference when you are pulling luggage.
  ```
- Protected tokens: `Hongik University Station`, `Seoul Station`, `Hongdae`

### ITEM 482

- Page ITEM: 095 of 221
- File: `arex.html`
- Line/context: L248 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Gongdeok
  ```
- Protected tokens: `Gongdeok`

### ITEM 483

- Page ITEM: 096 of 221
- File: `arex.html`
- Line/context: L248 - `p`
- Element/type: Body text
- Exact English:

  ```text
  All-Stop reaches Gongdeok without a transfer. Hotels close to the station are a straightforward walk; other addresses can continue on Lines 5 or 6, with the exact hotel pin deciding which route works.
  ```
- Protected tokens: `Gongdeok`, `5`, `6`

### ITEM 484

- Page ITEM: 097 of 221
- File: `arex.html`
- Line/context: L249 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Myeongdong
  ```
- Protected tokens: `Myeongdong`

### ITEM 485

- Page ITEM: 098 of 221
- File: `arex.html`
- Line/context: L249 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Express to Seoul Station followed by Line 4 can be a fast rail route, while All-Stop covers the airport segment for less. Neither removes the transfer. With several bags, an airport bus that stops close to the hotel may be easier than moving through Seoul Station.
  ```
- Protected tokens: `Seoul Station`, `Line 4`

### ITEM 486

- Page ITEM: 099 of 221
- File: `arex.html`
- Line/context: L250 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Jongno
  ```
- Protected tokens: `Jongno`

### ITEM 487

- Page ITEM: 100 of 221
- File: `arex.html`
- Line/context: L250 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Jongno covers too wide an area for one AREX answer. Some hotels connect naturally from Gongdeok on Line 5; others work better from Seoul Station or by airport bus. Use the exact address and station exit rather than the district name alone.
  ```
- Protected tokens: `Jongno`, `AREX`, `Gongdeok`, `Line 5`, `Seoul Station`

### ITEM 488

- Page ITEM: 101 of 221
- File: `arex.html`
- Line/context: L251 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Dongdaemun
  ```
- Protected tokens: `Dongdaemun`

### ITEM 489

- Page ITEM: 102 of 221
- File: `arex.html`
- Line/context: L251 - `p`
- Element/type: Body text
- Exact English:

  ```text
  A common rail route is Express to Seoul Station and then Line 4 toward the station nearest the hotel. It is still a transfer with luggage, so a direct airport-bus stop near the accommodation can be worth comparing.
  ```
- Protected tokens: `Seoul Station`, `Line 4`

### ITEM 490

- Page ITEM: 103 of 221
- File: `arex.html`
- Line/context: L252 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Gangnam
  ```
- Protected tokens: `Gangnam`

### ITEM 491

- Page ITEM: 104 of 221
- File: `arex.html`
- Line/context: L252 - `p`
- Element/type: Body text
- Exact English:

  ```text
  There is no single AREX transfer that works for all of Gangnam. All-Stop can connect with Line 9 at Magongnaru or Gimpo Airport, but the useful station depends on the exact address and any further connection. Many Gangnam hotels are simpler to reach by airport bus than by carrying luggage through several rail legs.
  ```
- Protected tokens: `AREX`, `Gangnam`, `Line 9`, `Magongnaru`, `Gimpo Airport`

### ITEM 492

- Page ITEM: 105 of 221
- File: `arex.html`
- Line/context: L253 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Jamsil
  ```
- Protected tokens: `Jamsil`

### ITEM 493

- Page ITEM: 106 of 221
- File: `arex.html`
- Line/context: L253 - `p`
- Element/type: Body text
- Exact English:

  ```text
  AREX usually leaves at least one substantial onward journey to Jamsil, and often more than one rail leg. A suitable airport bus may be simpler, especially with luggage, children or limited mobility.
  ```
- Protected tokens: `AREX`, `Jamsil`

### ITEM 494

- Page ITEM: 107 of 221
- File: `arex.html`
- Line/context: L254 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Gimpo Airport
  ```
- Protected tokens: `Gimpo Airport`

### ITEM 495

- Page ITEM: 108 of 221
- File: `arex.html`
- Line/context: L254 - `p`
- Element/type: Body text
- Exact English:

  ```text
  All-Stop runs directly to Gimpo International Airport Station; Express passes without stopping. Leave time for signs, elevators and the walk to the correct flight terminal or onward subway line.
  ```
- Protected tokens: `Gimpo International Airport Station`

### ITEM 496

- Page ITEM: 109 of 221
- File: `arex.html`
- Line/context: L255 - `h3`
- Element/type: H3
- Exact English:

  ```text
  KTX connection
  ```
- Protected tokens: `KTX`

### ITEM 497

- Page ITEM: 110 of 221
- File: `arex.html`
- Line/context: L255 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Express is often the sensible airport train when the next journey starts on KTX at Seoul Station. The AREX arrival time is not the time you can board the KTX: you still need to leave the deep AREX platform, cross the station and reach the correct national-rail platform.
  ```
- Protected tokens: `KTX`, `Seoul Station`, `AREX`

### ITEM 498

- Page ITEM: 111 of 221
- File: `arex.html`
- Line/context: L257 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Use Korean map apps to verify the hotel pin, station exit and final walk. An area name alone is not enough to judge the transfer.
  ```
- Protected tokens: None identified in this item.

### ITEM 499

- Page ITEM: 112 of 221
- File: `arex.html`
- Line/context: L258 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  AREX access alone should not decide where you stay. Hongdae and Gongdeok have direct All-Stop service, Seoul Station adds Express and national-rail connections, while some Myeongdong hotels can be easier to reach by airport bus.
  ```
- Protected tokens: `AREX`, `Hongdae`, `Gongdeok`, `Seoul Station`, `Myeongdong`

### ITEM 500

- Page ITEM: 113 of 221
- File: `arex.html`
- Line/context: L259 - `p:nth-of-type(3)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Compare airport-friendly Seoul bases →
  ```
- Protected tokens: `Seoul`

### ITEM 501

- Page ITEM: 114 of 221
- File: `arex.html`
- Line/context: L266 - `h2`
- Element/type: H2
- Exact English:

  ```text
  How to Find AREX at Terminal 1 and Terminal 2
  ```
- Protected tokens: `AREX`, `Terminal 1`, `Terminal 2`

### ITEM 502

- Page ITEM: 115 of 221
- File: `arex.html`
- Line/context: L267 - `p.transfer-chapter__answer`
- Element/type: Body text
- Exact English:

  ```text
  T1 and T2 have separate stations, but the route from the arrival hall follows the same basic sequence.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 503

- Page ITEM: 116 of 221
- File: `arex.html`
- Line/context: L270 - `figcaption`
- Element/type: Figcaption
- Exact English:

  ```text
  Arrival hall to the correct AREX platform
  ```
- Protected tokens: `AREX`

### ITEM 504

- Page ITEM: 117 of 221
- File: `arex.html`
- Line/context: L273 - `li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Complete immigration, baggage claim and customs, then enter the public arrival hall.
  ```
- Protected tokens: None identified in this item.

### ITEM 505

- Page ITEM: 118 of 221
- File: `arex.html`
- Line/context: L274 - `li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Follow Airport Railroad / AREX signs toward the terminal transportation center.
  ```
- Protected tokens: `Airport Railroad`, `AREX`

### ITEM 506

- Page ITEM: 119 of 221
- File: `arex.html`
- Line/context: L275 - `li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Continue to the rail area and separate the Express entrance from the All-Stop entrance.
  ```
- Protected tokens: None identified in this item.

### ITEM 507

- Page ITEM: 120 of 221
- File: `arex.html`
- Line/context: L276 - `li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Buy the correct ticket, or prepare a sufficiently funded transit card for All-Stop.
  ```
- Protected tokens: None identified in this item.

### ITEM 508

- Page ITEM: 121 of 221
- File: `arex.html`
- Line/context: L277 - `li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  Read the live departure display and confirm the Seoul-bound platform before entering.
  ```
- Protected tokens: `Seoul`

### ITEM 509

- Page ITEM: 122 of 221
- File: `arex.html`
- Line/context: L280 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  At Terminal 1: the rail area is in the T1 transportation center. Follow signs for that building rather than directions written for T2.
  ```
- Protected tokens: `Terminal 1`, `T1`, `T2`

### ITEM 510

- Page ITEM: 123 of 221
- File: `arex.html`
- Line/context: L281 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  At Terminal 2: continue toward the T2 transportation center and its own station. There is no need to travel to T1 first.
  ```
- Protected tokens: `Terminal 2`, `T2`, `T1`

### ITEM 511

- Page ITEM: 124 of 221
- File: `arex.html`
- Line/context: L286 - `img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  Step-by-step directions from Incheon Airport Terminal 1 and Terminal 2 arrival halls to the AREX platforms
  ```
- Protected tokens: `Incheon Airport Terminal 1`, `Terminal 2`, `AREX`

### ITEM 512

- Page ITEM: 125 of 221
- File: `arex.html`
- Line/context: L287 - `figcaption`
- Element/type: Figcaption
- Exact English:

  ```text
  How to reach the AREX station from Terminal 1 and Terminal 2
  ```
- Protected tokens: `AREX`, `Terminal 1`, `Terminal 2`

### ITEM 513

- Page ITEM: 126 of 221
- File: `arex.html`
- Line/context: L289 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Airport circulation and temporary routes can change, so current signs are more dependable than a saved gate or exit number. The broader Incheon Airport arrival process explains what happens before you reach the public hall.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 514

- Page ITEM: 127 of 221
- File: `arex.html`
- Line/context: L296 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Tickets for Express and All-Stop
  ```
- Protected tokens: None identified in this item.

### ITEM 515

- Page ITEM: 128 of 221
- File: `arex.html`
- Line/context: L297 - `p.transfer-chapter__answer`
- Element/type: Body text
- Exact English:

  ```text
  The two trains use different gates and ticket systems. T-money works on All-Stop, but it does not replace an Express reservation.
  ```
- Protected tokens: `T-money`

### ITEM 516

- Page ITEM: 129 of 221
- File: `arex.html`
- Line/context: L300 - `img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  AREX ticket decision guide for Express reservations, station tickets, T-money and single-use cards
  ```
- Protected tokens: `AREX`, `T-money`

### ITEM 517

- Page ITEM: 130 of 221
- File: `arex.html`
- Line/context: L301 - `figcaption`
- Element/type: Figcaption
- Exact English:

  ```text
  AREX ticket types for Express and All-Stop trains
  ```
- Protected tokens: `AREX`

### ITEM 518

- Page ITEM: 131 of 221
- File: `arex.html`
- Line/context: L305 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Express tickets
  ```
- Protected tokens: None identified in this item.

### ITEM 519

- Page ITEM: 132 of 221
- File: `arex.html`
- Line/context: L306 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  The official website and app can search travel dates from today through a maximum of two months ahead. Once a selected train is placed in the cart, payment must be completed within 20 minutes; that limit is not a promise that reservations remain available until 20 minutes before departure. The paid ticket carries a QR code and identifies the train, car and assigned seat.
  ```
- Protected tokens: `20 minutes`

### ITEM 520

- Page ITEM: 133 of 221
- File: `arex.html`
- Line/context: L307 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Tickets are also sold at Express machines and customer information centers at T1, T2 and Seoul Station. AREX lists Visa, Mastercard, JCB, Diners Club, American Express and UnionPay among the supported foreign-issued card networks for Express ticket purchases.
  ```
- Protected tokens: `T1`, `T2`, `Seoul Station`, `AREX`, `Visa`, `Mastercard`, `JCB`, `Diners Club`, `American Express`, `UnionPay`

### ITEM 521

- Page ITEM: 134 of 221
- File: `arex.html`
- Line/context: L310 - `h3`
- Element/type: H3
- Exact English:

  ```text
  All-Stop tickets
  ```
- Protected tokens: None identified in this item.

### ITEM 522

- Page ITEM: 135 of 221
- File: `arex.html`
- Line/context: L311 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  A prepaid transit card such as T-money can be used after it has enough balance for the trip. Tap at the All-Stop gate and tap out at the destination.
  ```
- Protected tokens: `T-money`

### ITEM 523

- Page ITEM: 136 of 221
- File: `arex.html`
- Line/context: L312 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Travelers without a transit card can buy a single-use transportation card from a station ticket machine, keep it for the journey and recover the deposit after exiting. The official All-Stop guidance confirms machine sales, but it does not clearly state that foreign-issued cards are accepted, so do not rely on a specific payment method without checking at the station.
  ```
- Protected tokens: None identified in this item.

### ITEM 524

- Page ITEM: 137 of 221
- File: `arex.html`
- Line/context: L315 - `p`
- Element/type: Body text
- Exact English:

  ```text
  The official Express booking site also provides change and return functions and shows any applicable conditions before confirmation. Reseller rules can differ from the operator's own terms.
  ```
- Protected tokens: None identified in this item.

### ITEM 525

- Page ITEM: 138 of 221
- File: `arex.html`
- Line/context: L322 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Seoul Station Is Often Only the Middle of the Journey
  ```
- Protected tokens: `Seoul Station`

### ITEM 526

- Page ITEM: 139 of 221
- File: `arex.html`
- Line/context: L323 - `p.transfer-chapter__answer`
- Element/type: Body text
- Exact English:

  ```text
  Arriving at Seoul Station is not the same as arriving at your hotel.
  ```
- Protected tokens: `Seoul Station`

### ITEM 527

- Page ITEM: 140 of 221
- File: `arex.html`
- Line/context: L325 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  AREX platforms sit deep inside a large station complex. Reaching the subway, a KTX platform or street level can involve long corridors, an elevator or escalator, and time spent following signs with luggage.
  ```
- Protected tokens: `AREX`, `KTX`

### ITEM 528

- Page ITEM: 141 of 221
- File: `arex.html`
- Line/context: L326 - `h3:nth-of-type(1)`
- Element/type: H3
- Exact English:

  ```text
  Subway and hotel connections
  ```
- Protected tokens: None identified in this item.

### ITEM 529

- Page ITEM: 142 of 221
- File: `arex.html`
- Line/context: L327 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Line 4 is the usual continuation toward Myeongdong and parts of Dongdaemun, while Line 1 serves other central and eastern connections. The trip is not finished at the subway gate: the correct direction, hotel-side exit and final walk all affect how manageable the route feels.
  ```
- Protected tokens: `Line 4`, `Myeongdong`, `Dongdaemun`, `Line 1`

### ITEM 530

- Page ITEM: 143 of 221
- File: `arex.html`
- Line/context: L328 - `h3:nth-of-type(2)`
- Element/type: H3
- Exact English:

  ```text
  KTX and national rail
  ```
- Protected tokens: `KTX`

### ITEM 531

- Page ITEM: 144 of 221
- File: `arex.html`
- Line/context: L329 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  A published AREX arrival minute does not leave you ready to board a KTX. Include the walk from the AREX platform, elevators or escalators, any ticket checks and the route across the station to the correct national-rail platform.
  ```
- Protected tokens: `AREX`, `KTX`

### ITEM 532

- Page ITEM: 145 of 221
- File: `arex.html`
- Line/context: L330 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  Use Korean map apps to verify the hotel pin, station exit and final walk. An area name alone is not enough to judge the transfer.
  ```
- Protected tokens: None identified in this item.

### ITEM 533

- Page ITEM: 146 of 221
- File: `arex.html`
- Line/context: L331 - `h3:nth-of-type(3)`
- Element/type: H3
- Exact English:

  ```text
  City Airport Terminal
  ```
- Protected tokens: None identified in this item.

### ITEM 534

- Page ITEM: 147 of 221
- File: `arex.html`
- Line/context: L332 - `p:nth-of-type(5)`
- Element/type: Body text
- Exact English:

  ```text
  The Seoul Station City Airport Terminal is a departure service, not help for passengers arriving from Incheon Airport. Eligible same-day international passengers using Express may be able to complete departure procedures there, subject to the current airline and service conditions.
  ```
- Protected tokens: `Seoul Station City Airport Terminal`, `Incheon Airport`

### ITEM 535

- Page ITEM: 148 of 221
- File: `arex.html`
- Line/context: L339 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Luggage, Families and Accessibility
  ```
- Protected tokens: None identified in this item.

### ITEM 536

- Page ITEM: 149 of 221
- File: `arex.html`
- Line/context: L340 - `p.transfer-chapter__answer`
- Element/type: Body text
- Exact English:

  ```text
  A comfortable train ride does not remove the work of moving bags through the airport station, an interchange and the final hotel route.
  ```
- Protected tokens: None identified in this item.

### ITEM 537

- Page ITEM: 150 of 221
- File: `arex.html`
- Line/context: L342 - `h3:nth-of-type(1)`
- Element/type: H3
- Exact English:

  ```text
  On the train
  ```
- Protected tokens: None identified in this item.

### ITEM 538

- Page ITEM: 151 of 221
- File: `arex.html`
- Line/context: L343 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Express provides an assigned seat and dedicated luggage space, which makes its main ride more predictable. All-Stop has commuter-style seating, so a seat is not guaranteed and crowded periods can be difficult with large suitcases, a stroller or children.
  ```
- Protected tokens: None identified in this item.

### ITEM 539

- Page ITEM: 152 of 221
- File: `arex.html`
- Line/context: L344 - `h3:nth-of-type(2)`
- Element/type: H3
- Exact English:

  ```text
  Between the train and the hotel
  ```
- Protected tokens: None identified in this item.

### ITEM 540

- Page ITEM: 153 of 221
- File: `arex.html`
- Line/context: L345 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Every bag still has to move from the arrival hall to the airport station, onto AREX, through any Seoul transfer and along the final walk. For a wheelchair user, a traveler with limited mobility or a family managing several bags, elevator routes and transfer distance can matter more than a 15-minute difference on the train.
  ```
- Protected tokens: `AREX`, `Seoul`, `15`

### ITEM 541

- Page ITEM: 154 of 221
- File: `arex.html`
- Line/context: L346 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  When that chain includes repeated lifting or long corridors, an airport bus stopping near the hotel or an appropriately sized official taxi may reduce the total burden.
  ```
- Protected tokens: None identified in this item.

### ITEM 542

- Page ITEM: 155 of 221
- File: `arex.html`
- Line/context: L348 - `img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  Situations when travelers should consider an airport bus, taxi or private pickup instead of AREX
  ```
- Protected tokens: `AREX`

### ITEM 543

- Page ITEM: 156 of 221
- File: `arex.html`
- Line/context: L349 - `figcaption`
- Element/type: Figcaption
- Exact English:

  ```text
  When an airport bus, taxi or private pickup may be easier than AREX
  ```
- Protected tokens: `AREX`

### ITEM 544

- Page ITEM: 157 of 221
- File: `arex.html`
- Line/context: L357 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Late Arrival? Work Backward from the Train, Not the Landing Time
  ```
- Protected tokens: None identified in this item.

### ITEM 545

- Page ITEM: 158 of 221
- File: `arex.html`
- Line/context: L358 - `p.transfer-chapter__answer`
- Element/type: Body text
- Exact English:

  ```text
  AREX does not run 24 hours, and landing time is not platform arrival time.
  ```
- Protected tokens: `AREX`, `24 hours`

### ITEM 546

- Page ITEM: 159 of 221
- File: `arex.html`
- Line/context: L360 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  The current official Express timetable lists the first airport departures at 05:16 from T2 and 05:24 from T1, and the last at 22:40 from T2 and 22:48 from T1. All-Stop follows a different schedule, so use the official dated AREX timetable rather than an old screenshot.
  ```
- Protected tokens: `05:16`, `T2`, `05:24`, `T1`, `22:40`, `22:48`, `AREX`

### ITEM 547

- Page ITEM: 160 of 221
- File: `arex.html`
- Line/context: L362 - `li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Start with the last departure from the terminal where your flight lands.
  ```
- Protected tokens: None identified in this item.

### ITEM 548

- Page ITEM: 161 of 221
- File: `arex.html`
- Line/context: L363 - `li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Work backward through the platform walk and ticket purchase.
  ```
- Protected tokens: None identified in this item.

### ITEM 549

- Page ITEM: 162 of 221
- File: `arex.html`
- Line/context: L364 - `li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Then allow for immigration, baggage claim and customs.
  ```
- Protected tokens: None identified in this item.

### ITEM 550

- Page ITEM: 163 of 221
- File: `arex.html`
- Line/context: L365 - `li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Compare the result with the scheduled landing time and a realistic delay margin.
  ```
- Protected tokens: None identified in this item.

### ITEM 551

- Page ITEM: 164 of 221
- File: `arex.html`
- Line/context: L366 - `li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  Have a backup that still operates after the time you can actually reach the platform.
  ```
- Protected tokens: None identified in this item.

### ITEM 552

- Page ITEM: 165 of 221
- File: `arex.html`
- Line/context: L368 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  If the last train no longer works, look for an official night bus or another airport bus option. If no suitable service remains, use an official airport taxi stand or a verified pickup with clear meeting and delay terms.
  ```
- Protected tokens: None identified in this item.

### ITEM 553

- Page ITEM: 166 of 221
- File: `arex.html`
- Line/context: L375 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Common AREX Mistakes
  ```
- Protected tokens: `AREX`

### ITEM 554

- Page ITEM: 167 of 221
- File: `arex.html`
- Line/context: L376 - `p.transfer-chapter__answer`
- Element/type: Body text
- Exact English:

  ```text
  The train itself is straightforward. Trouble usually starts when the route is planned only as far as an AREX station.
  ```
- Protected tokens: `AREX`

### ITEM 555

- Page ITEM: 168 of 221
- File: `arex.html`
- Line/context: L379 - `li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Express does not stop at Hongik University. For most Hongdae hotels, All-Stop avoids going to Seoul Station and then traveling back west.
  ```
- Protected tokens: `Hongik University`, `Hongdae`, `Seoul Station`

### ITEM 556

- Page ITEM: 169 of 221
- File: `arex.html`
- Line/context: L380 - `li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Seoul Station is not the end of the journey unless the hotel is nearby. Subway corridors, exits and the final walk need time of their own.
  ```
- Protected tokens: `Seoul Station`

### ITEM 557

- Page ITEM: 170 of 221
- File: `arex.html`
- Line/context: L381 - `li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Express and All-Stop have separate tickets and gates. Entering the wrong area creates an avoidable return to the concourse.
  ```
- Protected tokens: None identified in this item.

### ITEM 558

- Page ITEM: 171 of 221
- File: `arex.html`
- Line/context: L382 - `li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  T-money works on All-Stop, not as an Express ticket. Express requires its own reservation or station-issued ticket.
  ```
- Protected tokens: `T-money`

### ITEM 559

- Page ITEM: 172 of 221
- File: `arex.html`
- Line/context: L383 - `li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  T1 and T2 are different stations. Directions and departure times must match the terminal where the flight arrives.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 560

- Page ITEM: 173 of 221
- File: `arex.html`
- Line/context: L384 - `li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  Old fare tables, timetable screenshots and reseller conditions can be out of date. The operator's current pages should be the final reference.
  ```
- Protected tokens: None identified in this item.

### ITEM 561

- Page ITEM: 174 of 221
- File: `arex.html`
- Line/context: L385 - `li:nth-of-type(7)`
- Element/type: List text
- Exact English:

  ```text
  A late-night plan based on landing time leaves out immigration, baggage, customs, the station walk and ticket purchase.
  ```
- Protected tokens: None identified in this item.

### ITEM 562

- Page ITEM: 175 of 221
- File: `arex.html`
- Line/context: L386 - `li:nth-of-type(8)`
- Element/type: List text
- Exact English:

  ```text
  Rail is not automatically easier with several large bags. A bus stop near the hotel or an official taxi may remove repeated lifting and transfers.
  ```
- Protected tokens: None identified in this item.

### ITEM 563

- Page ITEM: 176 of 221
- File: `arex.html`
- Line/context: L387 - `li:nth-of-type(9)`
- Element/type: List text
- Exact English:

  ```text
  Gangnam and Jongno are too broad for one default transfer station. The hotel address, connecting line and final walk can change the sensible route.
  ```
- Protected tokens: `Gangnam`, `Jongno`

### ITEM 564

- Page ITEM: 177 of 221
- File: `arex.html`
- Line/context: L395 - `h2`
- Element/type: H2
- Exact English:

  ```text
  AREX Frequently Asked Questions
  ```
- Protected tokens: `AREX`

### ITEM 565

- Page ITEM: 178 of 221
- File: `arex.html`
- Line/context: L396 - `p.transfer-chapter__answer`
- Element/type: Body text
- Exact English:

  ```text
  These are the practical details that tend to matter once a flight, hotel and arrival time are fixed.
  ```
- Protected tokens: None identified in this item.

### ITEM 566

- Page ITEM: 179 of 221
- File: `arex.html`
- Line/context: L399 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  How much is AREX from Incheon Airport to Seoul?
  ```
- Protected tokens: `AREX`, `Incheon Airport`, `Seoul`

### ITEM 567

- Page ITEM: 180 of 221
- File: `arex.html`
- Line/context: L399 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  The Express adult selling fare is ₩13,000. With a transit card, All-Stop to Seoul Station costs ₩4,750 from T1 or ₩5,350 from T2. Check the current official fare before travel.
  ```
- Protected tokens: `₩13,000`, `Seoul Station`, `₩4,750`, `T1`, `₩5,350`, `T2`

### ITEM 568

- Page ITEM: 181 of 221
- File: `arex.html`
- Line/context: L400 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  How long does AREX take to Seoul Station?
  ```
- Protected tokens: `AREX`, `Seoul Station`

### ITEM 569

- Page ITEM: 182 of 221
- File: `arex.html`
- Line/context: L400 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Express takes 43 minutes from T1 or 51 minutes from T2. All-Stop takes 59 minutes from T1 or 66 minutes from T2, with some trains taking 2–6 minutes longer.
  ```
- Protected tokens: `43 minutes`, `T1`, `51 minutes`, `T2`, `59 minutes`, `66 minutes`, `2–6 minutes`

### ITEM 570

- Page ITEM: 183 of 221
- File: `arex.html`
- Line/context: L401 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Is AREX Express faster than All-Stop?
  ```
- Protected tokens: `AREX Express`

### ITEM 571

- Page ITEM: 184 of 221
- File: `arex.html`
- Line/context: L401 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  It is faster to Seoul Station. A hotel near Hongik University, Gongdeok or another intermediate stop may still be quicker to reach on All-Stop because no backtracking is needed.
  ```
- Protected tokens: `Seoul Station`, `Hongik University`, `Gongdeok`

### ITEM 572

- Page ITEM: 185 of 221
- File: `arex.html`
- Line/context: L402 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Can I use T-money on AREX?
  ```
- Protected tokens: `T-money`, `AREX`

### ITEM 573

- Page ITEM: 186 of 221
- File: `arex.html`
- Line/context: L402 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Use T-money on the All-Stop Train. Express requires a separate reserved ticket.
  ```
- Protected tokens: `T-money`, `All-Stop Train`

### ITEM 574

- Page ITEM: 187 of 221
- File: `arex.html`
- Line/context: L403 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Which AREX train should I take to Hongdae?
  ```
- Protected tokens: `AREX`, `Hongdae`

### ITEM 575

- Page ITEM: 188 of 221
- File: `arex.html`
- Line/context: L403 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  All-Stop runs directly to Hongik University Station. Express passes Hongdae and ends at Seoul Station, so it usually adds unnecessary travel back toward the west.
  ```
- Protected tokens: `Hongik University Station`, `Hongdae`, `Seoul Station`

### ITEM 576

- Page ITEM: 189 of 221
- File: `arex.html`
- Line/context: L404 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Does AREX run 24 hours?
  ```
- Protected tokens: `AREX`, `24 hours`

### ITEM 577

- Page ITEM: 190 of 221
- File: `arex.html`
- Line/context: L404 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  No. Check the official timetable for your terminal and travel date, especially after a late flight.
  ```
- Protected tokens: None identified in this item.

### ITEM 578

- Page ITEM: 191 of 221
- File: `arex.html`
- Line/context: L405 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Where is AREX at Terminal 1?
  ```
- Protected tokens: `AREX`, `Terminal 1`

### ITEM 579

- Page ITEM: 192 of 221
- File: `arex.html`
- Line/context: L405 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  After customs, follow Airport Railroad / AREX signs from the public arrival hall toward the Terminal 1 Transportation Center and rail area.
  ```
- Protected tokens: `Airport Railroad`, `AREX`, `Terminal 1 Transportation Center`

### ITEM 580

- Page ITEM: 193 of 221
- File: `arex.html`
- Line/context: L406 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Where is AREX at Terminal 2?
  ```
- Protected tokens: `AREX`, `Terminal 2`

### ITEM 581

- Page ITEM: 194 of 221
- File: `arex.html`
- Line/context: L406 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  After customs, follow Airport Railroad / AREX signs from the public arrival hall toward the Terminal 2 Transportation Center and rail area.
  ```
- Protected tokens: `Airport Railroad`, `AREX`, `Terminal 2 Transportation Center`

### ITEM 582

- Page ITEM: 195 of 221
- File: `arex.html`
- Line/context: L407 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Is AREX good with large luggage?
  ```
- Protected tokens: `AREX`

### ITEM 583

- Page ITEM: 196 of 221
- File: `arex.html`
- Line/context: L407 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Express has assigned seating and dedicated luggage space, but every bag still has to move through the airport station and any Seoul transfer. A bus or taxi may involve less handling when there are several large suitcases.
  ```
- Protected tokens: `Seoul`

### ITEM 584

- Page ITEM: 197 of 221
- File: `arex.html`
- Line/context: L408 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Is AREX better than the airport bus?
  ```
- Protected tokens: `AREX`

### ITEM 585

- Page ITEM: 198 of 221
- File: `arex.html`
- Line/context: L408 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  AREX is better for predictable rail time and hotels with a simple station connection. A bus can be better when an official stop is close to the hotel and reduces luggage transfers.
  ```
- Protected tokens: `AREX`

### ITEM 586

- Page ITEM: 199 of 221
- File: `arex.html`
- Line/context: L409 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Can I buy an AREX ticket with a foreign credit card?
  ```
- Protected tokens: `AREX`

### ITEM 587

- Page ITEM: 200 of 221
- File: `arex.html`
- Line/context: L409 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  For Express tickets, AREX lists Visa, Mastercard, JCB, Diners Club, American Express and UnionPay as supported foreign-issued card networks. For All-Stop, the official guide confirms that single-use transportation cards are sold at station machines, but it does not clearly confirm foreign-card acceptance at those machines.
  ```
- Protected tokens: `AREX`, `Visa`, `Mastercard`, `JCB`, `Diners Club`, `American Express`, `UnionPay`

### ITEM 588

- Page ITEM: 201 of 221
- File: `arex.html`
- Line/context: L410 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What happens if I miss the last train?
  ```
- Protected tokens: None identified in this item.

### ITEM 589

- Page ITEM: 202 of 221
- File: `arex.html`
- Line/context: L410 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Check official night-bus and airport-bus services first. If none fits your destination and time, use an official airport taxi stand or a verified pickup.
  ```
- Protected tokens: None identified in this item.

### ITEM 590

- Page ITEM: 203 of 221
- File: `arex.html`
- Line/context: L418 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Official Sources and Related Guides
  ```
- Protected tokens: None identified in this item.

### ITEM 591

- Page ITEM: 204 of 221
- File: `arex.html`
- Line/context: L419 - `p.transfer-chapter__answer`
- Element/type: Body text
- Exact English:

  ```text
  Fares, schedules, payment support and operating conditions can change. These official pages are the references for a final check before travel.
  ```
- Protected tokens: None identified in this item.

### ITEM 592

- Page ITEM: 205 of 221
- File: `arex.html`
- Line/context: L422 - `li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  AREX Express introduction — stops, travel time, official selling fare, assigned seating and station locations.
  ```
- Protected tokens: `AREX Express`

### ITEM 593

- Page ITEM: 206 of 221
- File: `arex.html`
- Line/context: L423 - `li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  AREX Express timetable — first, last and dated departure times.
  ```
- Protected tokens: `AREX Express`

### ITEM 594

- Page ITEM: 207 of 221
- File: `arex.html`
- Line/context: L424 - `li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  AREX Express ticket guide — online and station purchase, foreign cards, QR tickets, changes and returns.
  ```
- Protected tokens: `AREX Express`

### ITEM 595

- Page ITEM: 208 of 221
- File: `arex.html`
- Line/context: L425 - `li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  AREX All-Stop introduction — stations, transfer lines, travel time and ticket types.
  ```
- Protected tokens: `AREX All-Stop`

### ITEM 596

- Page ITEM: 209 of 221
- File: `arex.html`
- Line/context: L426 - `li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  AREX All-Stop fare table — distance fares, single-use surcharge and refundable deposit.
  ```
- Protected tokens: `AREX All-Stop`

### ITEM 597

- Page ITEM: 210 of 221
- File: `arex.html`
- Line/context: L427 - `li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  Incheon Airport rail transportation guide — airport transport context and terminal guidance.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 598

- Page ITEM: 211 of 221
- File: `arex.html`
- Line/context: L428 - `li:nth-of-type(7)`
- Element/type: List text
- Exact English:

  ```text
  Incheon Airport city terminal guide — Express eligibility and departure-service conditions.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 599

- Page ITEM: 212 of 221
- File: `arex.html`
- Line/context: L429 - `li:nth-of-type(8)`
- Element/type: List text
- Exact English:

  ```text
  VISITKOREA airport transportation guide — independent official overview of AREX train types and times.
  ```
- Protected tokens: `VISITKOREA`, `AREX`

### ITEM 600

- Page ITEM: 213 of 221
- File: `arex.html`
- Line/context: L431 - `nav.related-links @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Related Korea travel guides
  ```
- Protected tokens: `Korea`

### ITEM 601

- Page ITEM: 214 of 221
- File: `arex.html`
- Line/context: L432 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Continue planning
  ```
- Protected tokens: None identified in this item.

### ITEM 602

- Page ITEM: 215 of 221
- File: `arex.html`
- Line/context: L434 - `li:nth-of-type(1)`
- Element/type: Related-guide visible link
- Exact English:

  ```text
  Airport transfer comparison
  ```
- Protected tokens: None identified in this item.

### ITEM 603

- Page ITEM: 216 of 221
- File: `arex.html`
- Line/context: L435 - `li:nth-of-type(2)`
- Element/type: Related-guide visible link
- Exact English:

  ```text
  Airport bus guide
  ```
- Protected tokens: None identified in this item.

### ITEM 604

- Page ITEM: 217 of 221
- File: `arex.html`
- Line/context: L436 - `li:nth-of-type(3)`
- Element/type: Related-guide visible link
- Exact English:

  ```text
  Incheon Airport arrival process
  ```
- Protected tokens: `Incheon Airport`

### ITEM 605

- Page ITEM: 218 of 221
- File: `arex.html`
- Line/context: L437 - `li:nth-of-type(4)`
- Element/type: Related-guide visible link
- Exact English:

  ```text
  Incheon Airport guide
  ```
- Protected tokens: `Incheon Airport`

### ITEM 606

- Page ITEM: 219 of 221
- File: `arex.html`
- Line/context: L438 - `li:nth-of-type(5)`
- Element/type: Related-guide visible link
- Exact English:

  ```text
  T-money guide
  ```
- Protected tokens: `T-money`

### ITEM 607

- Page ITEM: 220 of 221
- File: `arex.html`
- Line/context: L439 - `li:nth-of-type(6)`
- Element/type: Related-guide visible link
- Exact English:

  ```text
  Korean map apps guide
  ```
- Protected tokens: None identified in this item.

### ITEM 608

- Page ITEM: 221 of 221
- File: `arex.html`
- Line/context: L440 - `li:nth-of-type(7)`
- Element/type: Related-guide visible link
- Exact English:

  ```text
  Official taxi guide
  ```
- Protected tokens: None identified in this item.


## PAGE - airport-bus.html

- English source: `airport-bus.html`
- Source SHA-256: `f58295df09fa7122ee587696103b51f61e83d7f91127a10f550c362cdb872283`
- Extracted ITEM count: 182

### ITEM 609

- Page ITEM: 001 of 182
- File: `airport-bus.html`
- Line/context: L6 - `html > head > meta[name="description"] @content`
- Element/type: Meta description
- Exact English:

  ```text
  Choose the right Incheon Airport bus for Seoul, Gyeonggi or other cities. Learn routes, T1/T2 tickets, boarding, luggage, late-night buses and return trips.
  ```
- Protected tokens: `Incheon Airport`, `Seoul`, `Gyeonggi`, `T1`, `T2`

### ITEM 610

- Page ITEM: 002 of 182
- File: `airport-bus.html`
- Line/context: L9 - `html > head > title`
- Element/type: Title
- Exact English:

  ```text
  Incheon Airport Bus Guide: Routes, Tickets and Boarding | Korea Inside
  ```
- Protected tokens: `Incheon Airport`, `Korea Inside`

### ITEM 611

- Page ITEM: 003 of 182
- File: `airport-bus.html`
- Line/context: L79 - `p.page-hero__breadcrumb:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Home / Airport / Airport Bus
  ```
- Protected tokens: None identified in this item.

### ITEM 612

- Page ITEM: 004 of 182
- File: `airport-bus.html`
- Line/context: L80 - `h1.page-hero__title`
- Element/type: H1
- Exact English:

  ```text
  Incheon Airport Bus Guide
  ```
- Protected tokens: `Incheon Airport`

### ITEM 613

- Page ITEM: 005 of 182
- File: `airport-bus.html`
- Line/context: L81 - `p.page-hero__desc:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  An airport bus can be one of the easiest ways to leave Incheon Airport when it stops close to your hotel. You avoid a subway transfer, most large bags go underneath the coach, and the trip can end within walking distance of your accommodation.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 614

- Page ITEM: 006 of 182
- File: `airport-bus.html`
- Line/context: L82 - `p.page-hero__desc:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  The difficult part is that “airport bus” does not mean one network. Seoul, Incheon, Gyeonggi, intercity and late-night services use different routes, operators and ticket rules. The bus number matters less than where the bus actually leaves you.
  ```
- Protected tokens: `Seoul`, `Incheon`, `Gyeonggi`

### ITEM 615

- Page ITEM: 007 of 182
- File: `airport-bus.html`
- Line/context: L88 - `p`
- Element/type: Body text
- Exact English:

  ```text
  A bus is especially useful when there is a stop within an easy walk of your hotel. For Hongdae, Seoul Station or another address with a simple rail connection, AREX may still be easier. Outside Seoul, search by the actual city or terminal rather than starting with a Seoul limousine route.
  ```
- Protected tokens: `Hongdae`, `Seoul Station`, `AREX`, `Seoul`

### ITEM 616

- Page ITEM: 008 of 182
- File: `airport-bus.html`
- Line/context: L94 - `img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  Terminal 2 signs for local-city, Seoul and Gyeonggi airport bus boarding areas at Incheon Airport
  ```
- Protected tokens: `Terminal 2`, `Seoul`, `Gyeonggi`, `Incheon Airport`

### ITEM 617

- Page ITEM: 009 of 182
- File: `airport-bus.html`
- Line/context: L95 - `figcaption`
- Element/type: Figcaption
- Exact English:

  ```text
  Terminal 2 separates bus boarding areas by destination. Bay ranges and route assignments can change, so confirm the current signs before boarding.
  ```
- Protected tokens: `Terminal 2`

### ITEM 618

- Page ITEM: 010 of 182
- File: `airport-bus.html`
- Line/context: L98 - `h2#choose-bus`
- Element/type: H2
- Exact English:

  ```text
  Start with the Stop, Not the Bus Number
  ```
- Protected tokens: None identified in this item.

### ITEM 619

- Page ITEM: 011 of 182
- File: `airport-bus.html`
- Line/context: L99 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  The easiest airport-bus trip starts at the hotel, not at the airport. Find your accommodation on a Korean map, then look for bus stops that leave you with a reasonable final walk.
  ```
- Protected tokens: None identified in this item.

### ITEM 620

- Page ITEM: 012 of 182
- File: `airport-bus.html`
- Line/context: L100 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  A stop 300 metres away on a flat street can be excellent. A slightly closer stop on the other side of a wide intersection, up a hill or several levels below the hotel can be much less pleasant with two suitcases.
  ```
- Protected tokens: `300 metres`

### ITEM 621

- Page ITEM: 013 of 182
- File: `airport-bus.html`
- Line/context: L101 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  That matters especially in large districts such as Myeongdong, Jongno, Dongdaemun, Gangnam and Hongdae. Two hotels described as being in the same neighborhood can have very different airport-bus access.
  ```
- Protected tokens: `Myeongdong`, `Jongno`, `Dongdaemun`, `Gangnam`, `Hongdae`

### ITEM 622

- Page ITEM: 014 of 182
- File: `airport-bus.html`
- Line/context: L107 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Seoul
  ```
- Protected tokens: `Seoul`

### ITEM 623

- Page ITEM: 015 of 182
- File: `airport-bus.html`
- Line/context: L108 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  For a Seoul hotel, an airport limousine is most attractive when the stop is genuinely close to the accommodation.
  ```
- Protected tokens: `Seoul`

### ITEM 624

- Page ITEM: 016 of 182
- File: `airport-bus.html`
- Line/context: L109 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  The main benefit is not necessarily speed. It is avoiding the sequence of airport station, train, transfer station, subway exit and final walk while carrying luggage.
  ```
- Protected tokens: None identified in this item.

### ITEM 625

- Page ITEM: 017 of 182
- File: `airport-bus.html`
- Line/context: L110 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  Traffic is the trade-off. A bus can take longer than rail when roads are busy, so an address with a simple AREX connection may still work better.
  ```
- Protected tokens: `AREX`

### ITEM 626

- Page ITEM: 018 of 182
- File: `airport-bus.html`
- Line/context: L111 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  If you are still choosing a Seoul base, a direct AREX station is not automatically easier than an airport-bus stop close to the hotel. Compare the full arrival for Hongdae, Gongdeok, Seoul Station and Myeongdong before choosing the area.
  ```
- Protected tokens: `Seoul`, `AREX`, `Hongdae`, `Gongdeok`, `Seoul Station`, `Myeongdong`

### ITEM 627

- Page ITEM: 019 of 182
- File: `airport-bus.html`
- Line/context: L112 - `p.airport-bus-context-link:nth-of-type(5)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Compare Seoul areas for airport access →
  ```
- Protected tokens: `Seoul`

### ITEM 628

- Page ITEM: 020 of 182
- File: `airport-bus.html`
- Line/context: L115 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Incheon and Gyeonggi
  ```
- Protected tokens: `Incheon`, `Gyeonggi`

### ITEM 629

- Page ITEM: 021 of 182
- File: `airport-bus.html`
- Line/context: L116 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Destinations outside central Seoul use different bus networks.
  ```
- Protected tokens: `Seoul`

### ITEM 630

- Page ITEM: 022 of 182
- File: `airport-bus.html`
- Line/context: L117 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  A traveler heading to Suwon, Seongnam, Goyang, Bucheon or another Gyeonggi destination should search that destination directly rather than trying to adapt a Seoul airport-bus route.
  ```
- Protected tokens: `Suwon`, `Seongnam`, `Goyang`, `Bucheon`, `Gyeonggi`, `Seoul`

### ITEM 631

- Page ITEM: 023 of 182
- File: `airport-bus.html`
- Line/context: L118 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  Route numbers, operators, ticket arrangements and stops vary, and Gyeonggi late-night services are also published separately from Seoul services.
  ```
- Protected tokens: `Gyeonggi`, `Seoul`

### ITEM 632

- Page ITEM: 024 of 182
- File: `airport-bus.html`
- Line/context: L119 - `p.airport-bus-context-link:nth-of-type(4)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  Gyeonggi Bus Information
  ```
- Protected tokens: `Gyeonggi Bus Information`

### ITEM 633

- Page ITEM: 025 of 182
- File: `airport-bus.html`
- Line/context: L122 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Other Korean cities
  ```
- Protected tokens: None identified in this item.

### ITEM 634

- Page ITEM: 026 of 182
- File: `airport-bus.html`
- Line/context: L123 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  If the airport is only the beginning of a longer trip, an intercity bus can remove the need to travel into Seoul first.
  ```
- Protected tokens: `Seoul`

### ITEM 635

- Page ITEM: 027 of 182
- File: `airport-bus.html`
- Line/context: L124 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Search for the actual city and the terminal name. Large cities can have more than one bus terminal, and arriving at the wrong side of the city can erase the convenience of taking a direct bus from the airport.
  ```
- Protected tokens: None identified in this item.

### ITEM 636

- Page ITEM: 028 of 182
- File: `airport-bus.html`
- Line/context: L125 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  BusTago provides online reservation information for participating intercity terminals, but not every route supports internet booking.
  ```
- Protected tokens: `BusTago`

### ITEM 637

- Page ITEM: 029 of 182
- File: `airport-bus.html`
- Line/context: L128 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Late at night
  ```
- Protected tokens: None identified in this item.

### ITEM 638

- Page ITEM: 030 of 182
- File: `airport-bus.html`
- Line/context: L129 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  A late-night bus is not simply the daytime route running later.
  ```
- Protected tokens: None identified in this item.

### ITEM 639

- Page ITEM: 031 of 182
- File: `airport-bus.html`
- Line/context: L130 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Incheon Airport publishes separate night-bus information for Terminal 1 and Terminal 2, and separates Seoul from Gyeonggi services. Start with the terminal where your flight actually lands and then look at where the night route finishes.
  ```
- Protected tokens: `Incheon Airport`, `Terminal 1`, `Terminal 2`, `Seoul`, `Gyeonggi`

### ITEM 640

- Page ITEM: 032 of 182
- File: `airport-bus.html`
- Line/context: L131 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  A bus that technically runs at 1 a.m. is not very useful if its last stop leaves you far from the hotel with luggage.
  ```
- Protected tokens: `1`

### ITEM 641

- Page ITEM: 033 of 182
- File: `airport-bus.html`
- Line/context: L134 - `h3`
- Element/type: H3
- Exact English:

  ```text
  The free airport shuttle
  ```
- Protected tokens: None identified in this item.

### ITEM 642

- Page ITEM: 034 of 182
- File: `airport-bus.html`
- Line/context: L135 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  The airport shuttle is different again.
  ```
- Protected tokens: None identified in this item.

### ITEM 643

- Page ITEM: 035 of 182
- File: `airport-bus.html`
- Line/context: L136 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  It moves passengers between airport terminals and airport-area facilities. It is not free transportation into Seoul or Gyeonggi.
  ```
- Protected tokens: `Seoul`, `Gyeonggi`

### ITEM 644

- Page ITEM: 036 of 182
- File: `airport-bus.html`
- Line/context: L139 - `h3`
- Element/type: H3
- Exact English:

  ```text
  When the Bus Is Not the Obvious Choice
  ```
- Protected tokens: None identified in this item.

### ITEM 645

- Page ITEM: 037 of 182
- File: `airport-bus.html`
- Line/context: L140 - `p`
- Element/type: Body text
- Exact English:

  ```text
  A bus is not automatically better because it is direct. If the nearest stop still leaves a difficult walk, AREX may be easier for a rail-connected hotel. A taxi makes more sense when door-to-door travel matters more than price, while a pre-booked vehicle can be useful for a larger group or unusual luggage. The full trade-offs are covered in the Incheon Airport transfer guide.
  ```
- Protected tokens: `AREX`, `Incheon Airport`

### ITEM 646

- Page ITEM: 038 of 182
- File: `airport-bus.html`
- Line/context: L146 - `h2#find-route`
- Element/type: H2
- Exact English:

  ```text
  Find a Route That Still Works After You Get Off
  ```
- Protected tokens: None identified in this item.

### ITEM 647

- Page ITEM: 039 of 182
- File: `airport-bus.html`
- Line/context: L147 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Once you have the hotel pin, the route search becomes much easier. The official airport search tells you which buses serve the area; the operator page tells you the details that can change from one company to another.
  ```
- Protected tokens: None identified in this item.

### ITEM 648

- Page ITEM: 040 of 182
- File: `airport-bus.html`
- Line/context: L150 - `li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Save the hotel in Korean. Keep the Korean place name, road address and phone number on your phone. A map pin is even more useful than the neighborhood name.
  ```
- Protected tokens: None identified in this item.

### ITEM 649

- Page ITEM: 041 of 182
- File: `airport-bus.html`
- Line/context: L151 - `li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Look at the last part of the journey. Compare the candidate stops with the hotel entrance, not simply the center of the district. Hills, underpasses and wide intersections matter when you are carrying luggage.
  ```
- Protected tokens: None identified in this item.

### ITEM 650

- Page ITEM: 042 of 182
- File: `airport-bus.html`
- Line/context: L152 - `li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Use the Incheon Airport bus search. Search the destination and identify the route, stop and operator. The airport separates Seoul, Gyeonggi and intercity services rather than treating them as one system.
  ```
- Protected tokens: `Incheon Airport`, `Seoul`, `Gyeonggi`

### ITEM 651

- Page ITEM: 043 of 182
- File: `airport-bus.html`
- Line/context: L153 - `li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Then open the operator’s own page. Timetables, fares, ticketing and baggage rules can differ even between buses serving similar parts of Seoul.
  ```
- Protected tokens: `Seoul`

### ITEM 652

- Page ITEM: 044 of 182
- File: `airport-bus.html`
- Line/context: L154 - `li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  Match the route to T1 or T2. Your flight terminal affects where you buy the ticket, where you board and sometimes the departure time.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 653

- Page ITEM: 045 of 182
- File: `airport-bus.html`
- Line/context: L155 - `li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  Leave room for the airport itself. Incheon Airport advises passengers booking public transportation to consider roughly 1–2 hours for immigration after flight arrival. Baggage and customs add their own uncertainty, so a very tight last-bus connection deserves a backup.
  ```
- Protected tokens: `Incheon Airport`, `1–2 hours`

### ITEM 654

- Page ITEM: 046 of 182
- File: `airport-bus.html`
- Line/context: L159 - `img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  Airport bus selection guide by hotel area in Seoul and nearby regions
  ```
- Protected tokens: `Seoul`

### ITEM 655

- Page ITEM: 047 of 182
- File: `airport-bus.html`
- Line/context: L160 - `figcaption`
- Element/type: Figcaption
- Exact English:

  ```text
  Bus routes and boarding locations may change. Confirm the latest information on the official airport or bus operator website before travel.
  ```
- Protected tokens: None identified in this item.

### ITEM 656

- Page ITEM: 048 of 182
- File: `airport-bus.html`
- Line/context: L166 - `h2#operators`
- Element/type: H2
- Exact English:

  ```text
  Official Seoul Airport Bus Operators
  ```
- Protected tokens: `Seoul`

### ITEM 657

- Page ITEM: 049 of 182
- File: `airport-bus.html`
- Line/context: L167 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Seoul airport buses are not operated by one company. Once you have a route, the operator name tells you where to check the rules that actually apply to it.
  ```
- Protected tokens: `Seoul`

### ITEM 658

- Page ITEM: 050 of 182
- File: `airport-bus.html`
- Line/context: L170 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Airport Limousine Co.
  ```
- Protected tokens: `Airport Limousine Co.`

### ITEM 659

- Page ITEM: 051 of 182
- File: `airport-bus.html`
- Line/context: L171 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Many Seoul airport routes are operated by Airport Limousine Co. Its site publishes routes, timetables, fares, baggage guidance and ticket information.
  ```
- Protected tokens: `Seoul`, `Airport Limousine Co.`

### ITEM 660

- Page ITEM: 052 of 182
- File: `airport-bus.html`
- Line/context: L174 - `h3`
- Element/type: H3
- Exact English:

  ```text
  K Airport Limousine
  ```
- Protected tokens: `K Airport Limousine`

### ITEM 661

- Page ITEM: 053 of 182
- File: `airport-bus.html`
- Line/context: L175 - `p`
- Element/type: Body text
- Exact English:

  ```text
  K Airport Limousine operates its own route network and publishes stop search, timetables, fares, ticket information and bus tracking.
  ```
- Protected tokens: `K Airport Limousine`

### ITEM 662

- Page ITEM: 054 of 182
- File: `airport-bus.html`
- Line/context: L178 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Seoul Airport Limousine
  ```
- Protected tokens: `Seoul Airport Limousine`

### ITEM 663

- Page ITEM: 055 of 182
- File: `airport-bus.html`
- Line/context: L179 - `p`
- Element/type: Body text
- Exact English:

  ```text
  This is another separate Seoul operator, including routes serving parts of Gangnam and southeastern Seoul. It has its own timetable and operating information.
  ```
- Protected tokens: `Seoul`, `Gangnam`

### ITEM 664

- Page ITEM: 056 of 182
- File: `airport-bus.html`
- Line/context: L182 - `h3`
- Element/type: H3
- Exact English:

  ```text
  CALT
  ```
- Protected tokens: `CALT`

### ITEM 665

- Page ITEM: 057 of 182
- File: `airport-bus.html`
- Line/context: L183 - `p`
- Element/type: Body text
- Exact English:

  ```text
  CALT operates its own airport-limousine routes, including the COEX-area connection, with separate route and timetable information.
  ```
- Protected tokens: `CALT`, `COEX`

### ITEM 666

- Page ITEM: 058 of 182
- File: `airport-bus.html`
- Line/context: L186 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  The company name is not administrative trivia. It tells you whose fare, baggage and ticket rules actually apply to your bus.
  ```
- Protected tokens: None identified in this item.

### ITEM 667

- Page ITEM: 059 of 182
- File: `airport-bus.html`
- Line/context: L190 - `h2#tickets-boarding`
- Element/type: H2
- Exact English:

  ```text
  Buying a Ticket and Boarding at T1 or T2
  ```
- Protected tokens: `T1`, `T2`

### ITEM 668

- Page ITEM: 060 of 182
- File: `airport-bus.html`
- Line/context: L191 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Once you reach the public arrival hall, most of the planning is already done. Now you only need to match the route you saved with the correct ticket point and boarding area.
  ```
- Protected tokens: None identified in this item.

### ITEM 669

- Page ITEM: 061 of 182
- File: `airport-bus.html`
- Line/context: L195 - `img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  Incheon Airport overhead sign pointing separately to Arrivals and Transfer
  ```
- Protected tokens: `Incheon Airport`

### ITEM 670

- Page ITEM: 062 of 182
- File: `airport-bus.html`
- Line/context: L196 - `figcaption`
- Element/type: Figcaption
- Exact English:

  ```text
  Passengers entering Korea should follow Arrivals. Transfer signs are for connecting passengers following a separate route.
  ```
- Protected tokens: `Korea`

### ITEM 671

- Page ITEM: 063 of 182
- File: `airport-bus.html`
- Line/context: L199 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Terminal 1
  ```
- Protected tokens: `Terminal 1`

### ITEM 672

- Page ITEM: 064 of 182
- File: `airport-bus.html`
- Line/context: L200 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Terminal 1 bus ticketing is on the arrivals level.
  ```
- Protected tokens: `Terminal 1`

### ITEM 673

- Page ITEM: 065 of 182
- File: `airport-bus.html`
- Line/context: L201 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  The airport currently lists ticket booths inside the terminal near exits 4 and 9, with additional outside booths around exits 4, 6, 7, 8, 11 and 13. Use those locations for orientation, but follow the live airport signs if the layout has changed.
  ```
- Protected tokens: `4`, `9`, `6`, `7`, `8`, `11`, `13`

### ITEM 674

- Page ITEM: 066 of 182
- File: `airport-bus.html`
- Line/context: L202 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  There is no reason to memorize a bus bay weeks before the trip. Buy or verify the ticket first, then follow the current display to the boarding position.
  ```
- Protected tokens: None identified in this item.

### ITEM 675

- Page ITEM: 067 of 182
- File: `airport-bus.html`
- Line/context: L207 - `img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  Terminal 1 transportation information board showing airport bus boarding areas at Incheon Airport
  ```
- Protected tokens: `Terminal 1`, `Incheon Airport`

### ITEM 676

- Page ITEM: 068 of 182
- File: `airport-bus.html`
- Line/context: L208 - `figcaption`
- Element/type: Figcaption
- Exact English:

  ```text
  This Terminal 1 transportation information board shows the bus areas by destination. Use it for orientation, then confirm your route and boarding bay on the live airport display.
  ```
- Protected tokens: `Terminal 1`

### ITEM 677

- Page ITEM: 069 of 182
- File: `airport-bus.html`
- Line/context: L211 - `h3:nth-of-type(1)`
- Element/type: H3
- Exact English:

  ```text
  Terminal 2
  ```
- Protected tokens: `Terminal 2`

### ITEM 678

- Page ITEM: 070 of 182
- File: `airport-bus.html`
- Line/context: L212 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  At Terminal 2, buses are handled from the B1 Bus Terminal in the Transportation Center.
  ```
- Protected tokens: `Terminal 2`, `B1 Bus Terminal`, `Transportation Center`

### ITEM 679

- Page ITEM: 071 of 182
- File: `airport-bus.html`
- Line/context: L213 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  That is where the airport currently directs passengers for bus information and ticket purchases.
  ```
- Protected tokens: None identified in this item.

### ITEM 680

- Page ITEM: 072 of 182
- File: `airport-bus.html`
- Line/context: L216 - `img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  Incheon Airport bus boarding location guide for Terminal 1 and Terminal 2
  ```
- Protected tokens: `Incheon Airport`, `Terminal 1`, `Terminal 2`

### ITEM 681

- Page ITEM: 073 of 182
- File: `airport-bus.html`
- Line/context: L217 - `figcaption`
- Element/type: Figcaption
- Exact English:

  ```text
  Visual guide to Terminal 1 and Terminal 2 bus boarding locations
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 682

- Page ITEM: 074 of 182
- File: `airport-bus.html`
- Line/context: L222 - `img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  Terminal 2 sign for Seoul-bound airport bus boarding bays 29 to 35 at Incheon Airport
  ```
- Protected tokens: `Terminal 2`, `Seoul`, `29 to 35`, `Incheon Airport`

### ITEM 683

- Page ITEM: 075 of 182
- File: `airport-bus.html`
- Line/context: L223 - `figcaption`
- Element/type: Figcaption
- Exact English:

  ```text
  This photo shows the Seoul-bound airport bus boarding area for bays 29 to 35 at Terminal 2. Bay and route assignments can change, so check your ticket and the live airport display before boarding.
  ```
- Protected tokens: `Seoul`, `29 to 35`, `Terminal 2`

### ITEM 684

- Page ITEM: 076 of 182
- File: `airport-bus.html`
- Line/context: L226 - `h3`
- Element/type: H3
- Exact English:

  ```text
  One T2 rule worth knowing in 2026
  ```
- Protected tokens: `T2`, `2026`

### ITEM 685

- Page ITEM: 077 of 182
- File: `airport-bus.html`
- Line/context: L227 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Do not assume every Seoul airport bus lets you walk up and tap a transit card.
  ```
- Protected tokens: `Seoul`

### ITEM 686

- Page ITEM: 078 of 182
- File: `airport-bus.html`
- Line/context: L228 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Airport Limousine Co. changed its T2 procedure in March 2026: passengers taking its buses into Seoul must buy a ticket before boarding, using a staffed counter or ticket machine on B1. K Airport Limousine also tells passengers leaving Incheon Airport to purchase a reservation ticket at a booth or machine.
  ```
- Protected tokens: `Airport Limousine Co.`, `T2`, `March 2026`, `Seoul`, `B1`, `K Airport Limousine`, `Incheon Airport`

### ITEM 687

- Page ITEM: 079 of 182
- File: `airport-bus.html`
- Line/context: L229 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  That is why payment advice from one airport-bus company should not be copied to another.
  ```
- Protected tokens: None identified in this item.

### ITEM 688

- Page ITEM: 080 of 182
- File: `airport-bus.html`
- Line/context: L233 - `h3:nth-of-type(2)`
- Element/type: H3
- Exact English:

  ```text
  From the Arrival Hall to Your Seat
  ```
- Protected tokens: None identified in this item.

### ITEM 689

- Page ITEM: 081 of 182
- File: `airport-bus.html`
- Line/context: L235 - `li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Enter the public arrival hall after immigration, baggage claim and customs.
  ```
- Protected tokens: None identified in this item.

### ITEM 690

- Page ITEM: 082 of 182
- File: `airport-bus.html`
- Line/context: L236 - `li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Go to the bus ticket area for your terminal and show the saved destination if the route is unclear.
  ```
- Protected tokens: None identified in this item.

### ITEM 691

- Page ITEM: 083 of 182
- File: `airport-bus.html`
- Line/context: L237 - `li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Read the ticket before leaving the counter. The destination stop and terminal matter more than memorizing the route number.
  ```
- Protected tokens: None identified in this item.

### ITEM 692

- Page ITEM: 084 of 182
- File: `airport-bus.html`
- Line/context: L238 - `li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Follow the current display to the boarding bay. If the bay shown on an old screenshot disagrees with the airport display, use the airport display.
  ```
- Protected tokens: None identified in this item.

### ITEM 693

- Page ITEM: 085 of 182
- File: `airport-bus.html`
- Line/context: L239 - `li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  Show the destination before your suitcase goes underneath the bus. Keep any baggage claim tag until the bag is back in your hands.
  ```
- Protected tokens: None identified in this item.

### ITEM 694

- Page ITEM: 086 of 182
- File: `airport-bus.html`
- Line/context: L240 - `li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  Keep the hotel pin open during the ride. Onboard displays and announcements help, but seeing your position on the map makes an unfamiliar stop easier to recognize.
  ```
- Protected tokens: None identified in this item.

### ITEM 695

- Page ITEM: 087 of 182
- File: `airport-bus.html`
- Line/context: L244 - `img @alt`
- Element/type: Image alt
- Exact English:

  ```text
  Step-by-step guide to using the Incheon Airport limousine bus
  ```
- Protected tokens: `Incheon Airport`

### ITEM 696

- Page ITEM: 088 of 182
- File: `airport-bus.html`
- Line/context: L245 - `figcaption`
- Element/type: Figcaption
- Exact English:

  ```text
  Airport limousine bus process from route check to hotel arrival
  ```
- Protected tokens: None identified in this item.

### ITEM 697

- Page ITEM: 089 of 182
- File: `airport-bus.html`
- Line/context: L248 - `h3:nth-of-type(3)`
- Element/type: H3
- Exact English:

  ```text
  Payment Is an Operator Rule, Not an Airport-Wide Rule
  ```
- Protected tokens: None identified in this item.

### ITEM 698

- Page ITEM: 090 of 182
- File: `airport-bus.html`
- Line/context: L249 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  There is no single payment rule for every bus leaving Incheon Airport.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 699

- Page ITEM: 091 of 182
- File: `airport-bus.html`
- Line/context: L250 - `p:nth-of-type(5)`
- Element/type: Body text
- Exact English:

  ```text
  K Airport Limousine, for example, publishes credit-card, transportation-card and T-money payment support, while also requiring reservation tickets for buses departing from Incheon Airport. Airport Limousine Co. has its own airport ticket procedure.
  ```
- Protected tokens: `K Airport Limousine`, `T-money`, `Incheon Airport`, `Airport Limousine Co.`

### ITEM 700

- Page ITEM: 092 of 182
- File: `airport-bus.html`
- Line/context: L251 - `p:nth-of-type(6)`
- Element/type: Body text
- Exact English:

  ```text
  So a traveler who successfully tapped T-money on one route should not assume the same process applies at the next counter.
  ```
- Protected tokens: `T-money`

### ITEM 701

- Page ITEM: 093 of 182
- File: `airport-bus.html`
- Line/context: L252 - `p:nth-of-type(7)`
- Element/type: Body text
- Exact English:

  ```text
  A second payment card and some Korean won are still sensible arrival-day backups. The ticket machine or staffed counter for the actual route is the final answer.
  ```
- Protected tokens: None identified in this item.

### ITEM 702

- Page ITEM: 094 of 182
- File: `airport-bus.html`
- Line/context: L256 - `h2#luggage-problems`
- Element/type: H2
- Exact English:

  ```text
  Luggage, Families and Late Arrivals
  ```
- Protected tokens: None identified in this item.

### ITEM 703

- Page ITEM: 095 of 182
- File: `airport-bus.html`
- Line/context: L258 - `h3:nth-of-type(1)`
- Element/type: H3
- Exact English:

  ```text
  Luggage
  ```
- Protected tokens: None identified in this item.

### ITEM 704

- Page ITEM: 096 of 182
- File: `airport-bus.html`
- Line/context: L259 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  The luggage compartment is one of the best reasons to take an airport bus. It can also be the part where travelers make the wrong assumption.
  ```
- Protected tokens: None identified in this item.

### ITEM 705

- Page ITEM: 097 of 182
- File: `airport-bus.html`
- Line/context: L260 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  There is no universal Incheon Airport baggage allowance.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 706

- Page ITEM: 098 of 182
- File: `airport-bus.html`
- Line/context: L261 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  Airport Limousine Co. currently publishes an allowance of up to two pieces when each item is under 28 inches and 20 kg, while K Airport Limousine publishes a separate allowance of two pieces with a combined weight of up to 40 kg for a paying passenger.
  ```
- Protected tokens: `Airport Limousine Co.`, `28 inches`, `20 kg`, `K Airport Limousine`, `40 kg`

### ITEM 707

- Page ITEM: 099 of 182
- File: `airport-bus.html`
- Line/context: L262 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  Those are examples of operator rules, not a standard for every airport bus.
  ```
- Protected tokens: None identified in this item.

### ITEM 708

- Page ITEM: 100 of 182
- File: `airport-bus.html`
- Line/context: L263 - `p:nth-of-type(5)`
- Element/type: Body text
- Exact English:

  ```text
  Keep passports, medication, electronics and other valuables with you. If a suitcase goes under the bus, keep the matching claim tag until the bag is returned.
  ```
- Protected tokens: None identified in this item.

### ITEM 709

- Page ITEM: 101 of 182
- File: `airport-bus.html`
- Line/context: L265 - `h3:nth-of-type(2)`
- Element/type: H3
- Exact English:

  ```text
  Children, strollers and groups
  ```
- Protected tokens: None identified in this item.

### ITEM 710

- Page ITEM: 102 of 182
- File: `airport-bus.html`
- Line/context: L266 - `p:nth-of-type(6)`
- Element/type: Body text
- Exact English:

  ```text
  A family can find the bus much easier than rail when the stop is beside the hotel.
  ```
- Protected tokens: None identified in this item.

### ITEM 711

- Page ITEM: 103 of 182
- File: `airport-bus.html`
- Line/context: L267 - `p:nth-of-type(7)`
- Element/type: Body text
- Exact English:

  ```text
  The calculation changes if a stroller, several suitcases or a tired child still have to travel another 800 metres from the bus stop.
  ```
- Protected tokens: `800 metres`

### ITEM 712

- Page ITEM: 104 of 182
- File: `airport-bus.html`
- Line/context: L268 - `p:nth-of-type(8)`
- Element/type: Body text
- Exact English:

  ```text
  Child fares, separate-seat rules and stroller handling also belong to the operator, not to “Incheon Airport buses” as a whole.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 713

- Page ITEM: 105 of 182
- File: `airport-bus.html`
- Line/context: L269 - `p:nth-of-type(9)`
- Element/type: Body text
- Exact English:

  ```text
  For a family or group, compare the whole trip after the bus stops with the cost and vehicle capacity of a taxi or pre-booked transfer.
  ```
- Protected tokens: None identified in this item.

### ITEM 714

- Page ITEM: 106 of 182
- File: `airport-bus.html`
- Line/context: L271 - `h3:nth-of-type(3)`
- Element/type: H3
- Exact English:

  ```text
  Late arrivals
  ```
- Protected tokens: None identified in this item.

### ITEM 715

- Page ITEM: 107 of 182
- File: `airport-bus.html`
- Line/context: L272 - `p:nth-of-type(10)`
- Element/type: Body text
- Exact English:

  ```text
  Use the time you can reach the public hall, not the aircraft landing time.
  ```
- Protected tokens: None identified in this item.

### ITEM 716

- Page ITEM: 108 of 182
- File: `airport-bus.html`
- Line/context: L273 - `p:nth-of-type(11)`
- Element/type: Body text
- Exact English:

  ```text
  Immigration, baggage claim and customs sit between those two moments. Incheon Airport itself tells travelers to allow for immigration time when booking public transportation.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 717

- Page ITEM: 109 of 182
- File: `airport-bus.html`
- Line/context: L274 - `p:nth-of-type(12)`
- Element/type: Body text
- Exact English:

  ```text
  Then check the current night-bus page for your terminal and your destination area. Seoul and Gyeonggi services are listed separately, and T1 and T2 do not share an identical timetable.
  ```
- Protected tokens: `Seoul`, `Gyeonggi`, `T1`, `T2`

### ITEM 718

- Page ITEM: 110 of 182
- File: `airport-bus.html`
- Line/context: L275 - `p:nth-of-type(13)`
- Element/type: Body text
- Exact English:

  ```text
  If the remaining bus leaves you far from the hotel, an official airport taxi can be the more practical late-night choice.
  ```
- Protected tokens: None identified in this item.

### ITEM 719

- Page ITEM: 111 of 182
- File: `airport-bus.html`
- Line/context: L279 - `h2#problems`
- Element/type: H2
- Exact English:

  ```text
  If Something Goes Wrong
  ```
- Protected tokens: None identified in this item.

### ITEM 720

- Page ITEM: 112 of 182
- File: `airport-bus.html`
- Line/context: L282 - `h3`
- Element/type: H3
- Exact English:

  ```text
  The card does not work
  ```
- Protected tokens: None identified in this item.

### ITEM 721

- Page ITEM: 113 of 182
- File: `airport-bus.html`
- Line/context: L283 - `p`
- Element/type: Body text
- Exact English:

  ```text
  A declined card does not necessarily mean the bus cannot be used. Ask the staffed counter which payment methods are accepted for that route before trying the same card repeatedly. If the bus cannot be ticketed with the payment you have, compare another verified route or the official taxi stand.
  ```
- Protected tokens: None identified in this item.

### ITEM 722

- Page ITEM: 114 of 182
- File: `airport-bus.html`
- Line/context: L286 - `h3`
- Element/type: H3
- Exact English:

  ```text
  The ticket is wrong
  ```
- Protected tokens: None identified in this item.

### ITEM 723

- Page ITEM: 115 of 182
- File: `airport-bus.html`
- Line/context: L287 - `p`
- Element/type: Body text
- Exact English:

  ```text
  If the destination, terminal or departure is wrong, return to the seller before boarding. Show the accommodation address and ask whether the ticket can be changed or refunded under that operator’s rules.
  ```
- Protected tokens: None identified in this item.

### ITEM 724

- Page ITEM: 116 of 182
- File: `airport-bus.html`
- Line/context: L290 - `h3`
- Element/type: H3
- Exact English:

  ```text
  The bus has already left
  ```
- Protected tokens: None identified in this item.

### ITEM 725

- Page ITEM: 117 of 182
- File: `airport-bus.html`
- Line/context: L291 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Look at the next departure before abandoning the route. A 20-minute wait may still be easier than moving luggage to the railway; a much longer wait may make AREX or a taxi more sensible.
  ```
- Protected tokens: `20`, `AREX`

### ITEM 726

- Page ITEM: 118 of 182
- File: `airport-bus.html`
- Line/context: L294 - `h3`
- Element/type: H3
- Exact English:

  ```text
  The boarding bay is not where the old guide said
  ```
- Protected tokens: None identified in this item.

### ITEM 727

- Page ITEM: 119 of 182
- File: `airport-bus.html`
- Line/context: L295 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Use the live airport display. Bus bays and route assignments can move, which is why a screenshot saved months earlier should never override the terminal signage.
  ```
- Protected tokens: None identified in this item.

### ITEM 728

- Page ITEM: 120 of 182
- File: `airport-bus.html`
- Line/context: L298 - `h3`
- Element/type: H3
- Exact English:

  ```text
  You searched the wrong terminal
  ```
- Protected tokens: None identified in this item.

### ITEM 729

- Page ITEM: 121 of 182
- File: `airport-bus.html`
- Line/context: L299 - `p`
- Element/type: Body text
- Exact English:

  ```text
  T1 and T2 have different bus areas. Check the terminal on the flight information before following another terminal’s ticket or boarding instructions.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 730

- Page ITEM: 122 of 182
- File: `airport-bus.html`
- Line/context: L302 - `h3`
- Element/type: H3
- Exact English:

  ```text
  The bus stop is farther from the hotel than expected
  ```
- Protected tokens: None identified in this item.

### ITEM 731

- Page ITEM: 123 of 182
- File: `airport-bus.html`
- Line/context: L303 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Open the walking route before getting off. A short taxi from the bus stop can sometimes save a difficult final walk, but if the problem is obvious before leaving the airport, a different transport option may be easier from the beginning.
  ```
- Protected tokens: None identified in this item.

### ITEM 732

- Page ITEM: 124 of 182
- File: `airport-bus.html`
- Line/context: L306 - `h3`
- Element/type: H3
- Exact English:

  ```text
  The last useful bus is gone
  ```
- Protected tokens: None identified in this item.

### ITEM 733

- Page ITEM: 125 of 182
- File: `airport-bus.html`
- Line/context: L307 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Check the terminal-specific night-bus information once more. If nothing reaches a practical stop, use the official airport taxi stand or a confirmed pickup rather than accepting an unsolicited ride inside the terminal.
  ```
- Protected tokens: None identified in this item.

### ITEM 734

- Page ITEM: 126 of 182
- File: `airport-bus.html`
- Line/context: L310 - `h3`
- Element/type: H3
- Exact English:

  ```text
  The baggage tag is missing
  ```
- Protected tokens: None identified in this item.

### ITEM 735

- Page ITEM: 127 of 182
- File: `airport-bus.html`
- Line/context: L311 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Tell the driver or bus staff before leaving the stop. Keep the ticket, route number, travel time and a description of the suitcase available while ownership is checked.
  ```
- Protected tokens: None identified in this item.

### ITEM 736

- Page ITEM: 128 of 182
- File: `airport-bus.html`
- Line/context: L317 - `h2#return-airport`
- Element/type: H2
- Exact English:

  ```text
  Returning to Incheon Airport
  ```
- Protected tokens: `Incheon Airport`

### ITEM 737

- Page ITEM: 129 of 182
- File: `airport-bus.html`
- Line/context: L318 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  The stop that brought you into Seoul is not automatically the stop that takes you back to the airport.
  ```
- Protected tokens: `Seoul`

### ITEM 738

- Page ITEM: 130 of 182
- File: `airport-bus.html`
- Line/context: L319 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Airport-bound stops can sit on another side of a large road or at a completely different location. Find the return stop before departure day, save the Korean name and map pin, and look at the street-level approach with luggage.
  ```
- Protected tokens: None identified in this item.

### ITEM 739

- Page ITEM: 131 of 182
- File: `airport-bus.html`
- Line/context: L321 - `li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Find the airport-bound stop separately. Do not reverse the arrival route by assumption.
  ```
- Protected tokens: None identified in this item.

### ITEM 740

- Page ITEM: 132 of 182
- File: `airport-bus.html`
- Line/context: L322 - `li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Check your airline terminal. Know whether you need T1 or T2 and whether the bus serves both.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 741

- Page ITEM: 133 of 182
- File: `airport-bus.html`
- Line/context: L323 - `li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Read the current timetable the day before departure. Early flights deserve particular caution.
  ```
- Protected tokens: None identified in this item.

### ITEM 742

- Page ITEM: 134 of 182
- File: `airport-bus.html`
- Line/context: L324 - `li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Leave more margin than the scheduled road time. Airport buses share the road with Seoul traffic.
  ```
- Protected tokens: `Seoul`

### ITEM 743

- Page ITEM: 135 of 182
- File: `airport-bus.html`
- Line/context: L325 - `li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  Know the alternative before leaving the hotel. If the first bus, stop or reservation no longer works, an AREX or taxi plan should already be saved.
  ```
- Protected tokens: `AREX`

### ITEM 744

- Page ITEM: 136 of 182
- File: `airport-bus.html`
- Line/context: L327 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  For an early flight, the first airport-bound bus matters more than the route that looked convenient during the daytime. If the schedule leaves little margin for airline check-in, an earlier rail or taxi departure is the safer decision.
  ```
- Protected tokens: None identified in this item.

### ITEM 745

- Page ITEM: 137 of 182
- File: `airport-bus.html`
- Line/context: L331 - `h2#departure-checklist`
- Element/type: H2
- Exact English:

  ```text
  Before Leaving the Hotel
  ```
- Protected tokens: None identified in this item.

### ITEM 746

- Page ITEM: 138 of 182
- File: `airport-bus.html`
- Line/context: L333 - `li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Airport-bound stop saved in Korean
  ```
- Protected tokens: None identified in this item.

### ITEM 747

- Page ITEM: 139 of 182
- File: `airport-bus.html`
- Line/context: L334 - `li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  T1 or T2 confirmed
  ```
- Protected tokens: `T1`, `T2`

### ITEM 748

- Page ITEM: 140 of 182
- File: `airport-bus.html`
- Line/context: L335 - `li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Current departure time checked
  ```
- Protected tokens: None identified in this item.

### ITEM 749

- Page ITEM: 141 of 182
- File: `airport-bus.html`
- Line/context: L336 - `li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Ticket or payment method understood
  ```
- Protected tokens: None identified in this item.

### ITEM 750

- Page ITEM: 142 of 182
- File: `airport-bus.html`
- Line/context: L337 - `li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  Luggage route to the stop checked
  ```
- Protected tokens: None identified in this item.

### ITEM 751

- Page ITEM: 143 of 182
- File: `airport-bus.html`
- Line/context: L338 - `li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  AREX or taxi backup saved
  ```
- Protected tokens: `AREX`

### ITEM 752

- Page ITEM: 144 of 182
- File: `airport-bus.html`
- Line/context: L343 - `h2#faq`
- Element/type: H2
- Exact English:

  ```text
  Airport Bus FAQ
  ```
- Protected tokens: None identified in this item.

### ITEM 753

- Page ITEM: 145 of 182
- File: `airport-bus.html`
- Line/context: L346 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What is the difference between an airport bus and an airport limousine bus?
  ```
- Protected tokens: None identified in this item.

### ITEM 754

- Page ITEM: 146 of 182
- File: `airport-bus.html`
- Line/context: L347 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  At Incheon Airport, “airport bus” covers several different services. “Airport limousine” is commonly used for coach-style routes, especially into Seoul, but there is no single limousine operator or universal ticket rule. The route and company matter more than the label.
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 755

- Page ITEM: 147 of 182
- File: `airport-bus.html`
- Line/context: L350 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  How do I find the right bus for my hotel?
  ```
- Protected tokens: None identified in this item.

### ITEM 756

- Page ITEM: 148 of 182
- File: `airport-bus.html`
- Line/context: L351 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Start with the exact hotel pin. Find bus stops that leave a reasonable final walk, then search those destinations in the official Incheon Airport bus tool. Once you have a route, use the operator page for the current timetable, fare and baggage rules.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 757

- Page ITEM: 149 of 182
- File: `airport-bus.html`
- Line/context: L354 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Are all Seoul airport buses operated by Airport Limousine Co.?
  ```
- Protected tokens: `Seoul`, `Airport Limousine Co.`

### ITEM 758

- Page ITEM: 150 of 182
- File: `airport-bus.html`
- Line/context: L355 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  No. Several companies operate airport buses in Seoul, including Airport Limousine Co., K Airport Limousine, Seoul Airport Limousine and CALT. Their route and ticket rules are not interchangeable.
  ```
- Protected tokens: `Seoul`, `Airport Limousine Co.`, `K Airport Limousine`, `Seoul Airport Limousine`, `CALT`

### ITEM 759

- Page ITEM: 151 of 182
- File: `airport-bus.html`
- Line/context: L358 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Where can I buy a bus ticket at Terminal 1?
  ```
- Protected tokens: `Terminal 1`

### ITEM 760

- Page ITEM: 152 of 182
- File: `airport-bus.html`
- Line/context: L359 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Incheon Airport currently lists T1 ticket booths inside the terminal near exits 4 and 9 and additional outside locations near exits 4, 6, 7, 8, 11 and 13. Follow the current signs if the airport layout has changed.
  ```
- Protected tokens: `Incheon Airport`, `T1`, `4`, `9`, `6`, `7`, `8`, `11`, `13`

### ITEM 761

- Page ITEM: 153 of 182
- File: `airport-bus.html`
- Line/context: L362 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Where can I buy a bus ticket at Terminal 2?
  ```
- Protected tokens: `Terminal 2`

### ITEM 762

- Page ITEM: 154 of 182
- File: `airport-bus.html`
- Line/context: L363 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Bus information and ticket purchases are handled at the B1 Bus Terminal in the Terminal 2 Transportation Center. Some operators require a ticket before you reach the bus, so identify the company before going to the boarding bay.
  ```
- Protected tokens: `B1 Bus Terminal`, `Terminal 2 Transportation Center`

### ITEM 763

- Page ITEM: 155 of 182
- File: `airport-bus.html`
- Line/context: L366 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Can I use T-money or a foreign credit card?
  ```
- Protected tokens: `T-money`

### ITEM 764

- Page ITEM: 156 of 182
- File: `airport-bus.html`
- Line/context: L367 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  There is no airport-wide answer. Some operators support transportation cards or credit cards, while airport departures on certain services require a ticket from a counter or machine. Use the rule for the actual operator rather than assuming that payment on one bus applies to another.
  ```
- Protected tokens: None identified in this item.

### ITEM 765

- Page ITEM: 157 of 182
- File: `airport-bus.html`
- Line/context: L370 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  How much luggage can I take?
  ```
- Protected tokens: None identified in this item.

### ITEM 766

- Page ITEM: 158 of 182
- File: `airport-bus.html`
- Line/context: L371 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Baggage rules are set by the bus company. Even major Seoul operators publish different free allowances, so check the company running your route if you have several large bags, sports equipment or another oversized item.
  ```
- Protected tokens: `Seoul`

### ITEM 767

- Page ITEM: 159 of 182
- File: `airport-bus.html`
- Line/context: L374 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What should I do after a late-night arrival?
  ```
- Protected tokens: None identified in this item.

### ITEM 768

- Page ITEM: 160 of 182
- File: `airport-bus.html`
- Line/context: L375 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Use the official night-bus page for the terminal where your flight lands and then separate Seoul from Gyeonggi routes. If no bus ends near a practical destination, use the official taxi stand or a confirmed pre-booked pickup.
  ```
- Protected tokens: `Seoul`, `Gyeonggi`

### ITEM 769

- Page ITEM: 161 of 182
- File: `airport-bus.html`
- Line/context: L378 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Are there buses from Incheon Airport to cities outside Seoul?
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 770

- Page ITEM: 162 of 182
- File: `airport-bus.html`
- Line/context: L379 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Yes. Incheon, Gyeonggi and intercity services connect the airport with many other destinations. Search by the exact city or bus terminal and then confirm the operator and reservation method.
  ```
- Protected tokens: `Incheon`, `Gyeonggi`

### ITEM 771

- Page ITEM: 163 of 182
- File: `airport-bus.html`
- Line/context: L382 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  How do I take the bus back to Incheon Airport?
  ```
- Protected tokens: `Incheon Airport`

### ITEM 772

- Page ITEM: 164 of 182
- File: `airport-bus.html`
- Line/context: L383 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Find the airport-bound stop separately, check whether the bus serves T1 or T2, and recheck the timetable before departure. Allow extra margin for road traffic and keep another airport route available if the bus timing is too tight.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 773

- Page ITEM: 165 of 182
- File: `airport-bus.html`
- Line/context: L389 - `h2#official-sources`
- Element/type: H2
- Exact English:

  ```text
  Official Sources
  ```
- Protected tokens: None identified in this item.

### ITEM 774

- Page ITEM: 166 of 182
- File: `airport-bus.html`
- Line/context: L391 - `li:nth-of-type(1)`
- Element/type: List text
- Exact English:

  ```text
  Incheon Airport Terminal 1 public bus information
  ```
- Protected tokens: `Incheon Airport Terminal 1`

### ITEM 775

- Page ITEM: 167 of 182
- File: `airport-bus.html`
- Line/context: L392 - `li:nth-of-type(2)`
- Element/type: List text
- Exact English:

  ```text
  Incheon Airport Terminal 2 bus terminal information
  ```
- Protected tokens: `Incheon Airport Terminal 2`

### ITEM 776

- Page ITEM: 168 of 182
- File: `airport-bus.html`
- Line/context: L393 - `li:nth-of-type(3)`
- Element/type: List text
- Exact English:

  ```text
  Incheon Airport official bus search
  ```
- Protected tokens: `Incheon Airport`

### ITEM 777

- Page ITEM: 169 of 182
- File: `airport-bus.html`
- Line/context: L394 - `li:nth-of-type(4)`
- Element/type: List text
- Exact English:

  ```text
  Terminal 1 Seoul late-night buses
  ```
- Protected tokens: `Terminal 1`, `Seoul`

### ITEM 778

- Page ITEM: 170 of 182
- File: `airport-bus.html`
- Line/context: L395 - `li:nth-of-type(5)`
- Element/type: List text
- Exact English:

  ```text
  Terminal 2 Seoul late-night buses
  ```
- Protected tokens: `Terminal 2`, `Seoul`

### ITEM 779

- Page ITEM: 171 of 182
- File: `airport-bus.html`
- Line/context: L396 - `li:nth-of-type(6)`
- Element/type: List text
- Exact English:

  ```text
  Terminal 1 Gyeonggi late-night buses
  ```
- Protected tokens: `Terminal 1`, `Gyeonggi`

### ITEM 780

- Page ITEM: 172 of 182
- File: `airport-bus.html`
- Line/context: L397 - `li:nth-of-type(7)`
- Element/type: List text
- Exact English:

  ```text
  Terminal 2 Gyeonggi late-night buses
  ```
- Protected tokens: `Terminal 2`, `Gyeonggi`

### ITEM 781

- Page ITEM: 173 of 182
- File: `airport-bus.html`
- Line/context: L398 - `li:nth-of-type(8)`
- Element/type: List text
- Exact English:

  ```text
  Airport Limousine Co.
  ```
- Protected tokens: `Airport Limousine Co.`

### ITEM 782

- Page ITEM: 174 of 182
- File: `airport-bus.html`
- Line/context: L399 - `li:nth-of-type(9)`
- Element/type: List text
- Exact English:

  ```text
  K Airport Limousine
  ```
- Protected tokens: `K Airport Limousine`

### ITEM 783

- Page ITEM: 175 of 182
- File: `airport-bus.html`
- Line/context: L400 - `li:nth-of-type(10)`
- Element/type: List text
- Exact English:

  ```text
  Seoul Airport Limousine
  ```
- Protected tokens: `Seoul Airport Limousine`

### ITEM 784

- Page ITEM: 176 of 182
- File: `airport-bus.html`
- Line/context: L401 - `li:nth-of-type(11)`
- Element/type: List text
- Exact English:

  ```text
  CALT City Airport Limousine
  ```
- Protected tokens: `CALT City Airport Limousine`

### ITEM 785

- Page ITEM: 177 of 182
- File: `airport-bus.html`
- Line/context: L402 - `li:nth-of-type(12)`
- Element/type: List text
- Exact English:

  ```text
  Gyeonggi Bus Information
  ```
- Protected tokens: `Gyeonggi Bus Information`

### ITEM 786

- Page ITEM: 178 of 182
- File: `airport-bus.html`
- Line/context: L403 - `li:nth-of-type(13)`
- Element/type: List text
- Exact English:

  ```text
  BusTago
  ```
- Protected tokens: `BusTago`

### ITEM 787

- Page ITEM: 179 of 182
- File: `airport-bus.html`
- Line/context: L407 - `nav.related-links @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Related guides
  ```
- Protected tokens: None identified in this item.

### ITEM 788

- Page ITEM: 180 of 182
- File: `airport-bus.html`
- Line/context: L408 - `p.related-links__title:nth-of-type(1)`
- Element/type: Related-guide text
- Exact English:

  ```text
  Related guides
  ```
- Protected tokens: None identified in this item.

### ITEM 789

- Page ITEM: 181 of 182
- File: `airport-bus.html`
- Line/context: L409 - `p:nth-of-type(2)`
- Element/type: Related-guide text
- Exact English:

  ```text
  Incheon Airport guide · Arrival procedures · AREX guide · Taxi backup
  ```
- Protected tokens: `Incheon Airport`, `AREX`

### ITEM 790

- Page ITEM: 182 of 182
- File: `airport-bus.html`
- Line/context: L410 - `p:nth-of-type(3)`
- Element/type: Related-guide text
- Exact English:

  ```text
  Compare airport transfers · Korean maps · Payments
  ```
- Protected tokens: None identified in this item.


## PAGE - incheon-airport-private-transfer.html

- English source: `incheon-airport-private-transfer.html`
- Source SHA-256: `532ff0942a5eabb17d0d928e5c46cb4855901b5e985b9c2109a55cf6986bcbc8`
- Extracted ITEM count: 139

### ITEM 791

- Page ITEM: 001 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L6 - `html > head > meta[name="description"] @content`
- Element/type: Meta description
- Exact English:

  ```text
  When an Incheon Airport private transfer makes sense, how luggage affects vehicle size, how airport pickup works, and where to book for Seoul.
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 792

- Page ITEM: 002 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L9 - `html > head > title`
- Element/type: Title
- Exact English:

  ```text
  Incheon Airport Private Transfer to Seoul | Korea Inside
  ```
- Protected tokens: `Incheon Airport`, `Seoul`, `Korea Inside`

### ITEM 793

- Page ITEM: 003 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L21 - `script[type="application/ld+json"] > mainEntity.0.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Is a private transfer worth it for four people?
  ```
- Protected tokens: None identified in this item.

### ITEM 794

- Page ITEM: 004 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L24 - `script[type="application/ld+json"] > mainEntity.0.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Sometimes. Four separate public-transport fares can narrow the price gap, especially when luggage or another taxi ride is needed at the Seoul end. The useful comparison is the total door-to-door cost, not only the first fare you see.
  ```
- Protected tokens: `Seoul`

### ITEM 795

- Page ITEM: 005 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L29 - `script[type="application/ld+json"] > mainEntity.1.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Can a regular taxi fit four people and four large suitcases?
  ```
- Protected tokens: None identified in this item.

### ITEM 796

- Page ITEM: 006 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L32 - `script[type="application/ld+json"] > mainEntity.1.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Not reliably. Passenger capacity and trunk space are separate limits. When every passenger has a large suitcase, a jumbo taxi or a pre-booked larger vehicle may be more practical.
  ```
- Protected tokens: None identified in this item.

### ITEM 797

- Page ITEM: 007 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L37 - `script[type="application/ld+json"] > mainEntity.2.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  What happens if my flight is delayed?
  ```
- Protected tokens: None identified in this item.

### ITEM 798

- Page ITEM: 008 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L40 - `script[type="application/ld+json"] > mainEntity.2.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Many pre-booked airport transfers ask for the flight number and track delays, but free waiting time varies by provider and product. The current waiting rule on the selected reservation is the one that matters.
  ```
- Protected tokens: None identified in this item.

### ITEM 799

- Page ITEM: 009 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L45 - `script[type="application/ld+json"] > mainEntity.3.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Where will I meet the driver at Incheon Airport?
  ```
- Protected tokens: `Incheon Airport`

### ITEM 800

- Page ITEM: 010 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L48 - `script[type="application/ld+json"] > mainEntity.3.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  The exact arrangement depends on the booking. Common methods include a driver waiting in the arrival hall with a name sign or a message directing you to a specific exit or meeting point.
  ```
- Protected tokens: None identified in this item.

### ITEM 801

- Page ITEM: 011 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L53 - `script[type="application/ld+json"] > mainEntity.4.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Does it matter whether I arrive at Terminal 1 or Terminal 2?
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 802

- Page ITEM: 012 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L56 - `script[type="application/ld+json"] > mainEntity.4.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Yes. The terminals have separate arrival areas and pickup points. Use the terminal shown for your arriving flight when making the reservation.
  ```
- Protected tokens: None identified in this item.

### ITEM 803

- Page ITEM: 013 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L61 - `script[type="application/ld+json"] > mainEntity.5.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Is a private transfer the same as a call van in Korea?
  ```
- Protected tokens: `Korea`

### ITEM 804

- Page ITEM: 014 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L64 - `script[type="application/ld+json"] > mainEntity.5.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  No. “Private transfer” is an international booking term for a vehicle reserved for one party. “Call van” is a Korean commercial van-service term. Some airport transfer vehicles may fall into that category, but the terms are not interchangeable.
  ```
- Protected tokens: None identified in this item.

### ITEM 805

- Page ITEM: 015 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L69 - `script[type="application/ld+json"] > mainEntity.6.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Do I need to pre-book transportation from Incheon Airport?
  ```
- Protected tokens: `Incheon Airport`

### ITEM 806

- Page ITEM: 016 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L72 - `script[type="application/ld+json"] > mainEntity.6.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Not for every trip. Solo travelers and couples with manageable luggage often have straightforward train, bus or taxi options. Pre-booking becomes more useful when vehicle size, luggage space, a late arrival or a large group makes finding the right vehicle on arrival more important.
  ```
- Protected tokens: None identified in this item.

### ITEM 807

- Page ITEM: 017 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L77 - `script[type="application/ld+json"] > mainEntity.7.name`
- Element/type: JSON-LD FAQ question
- Exact English:

  ```text
  Can I book a private transfer for a late-night arrival?
  ```
- Protected tokens: None identified in this item.

### ITEM 808

- Page ITEM: 018 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L80 - `script[type="application/ld+json"] > mainEntity.7.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Exact English:

  ```text
  Many Incheon Airport transfer products operate late at night or around the clock, but availability and possible night surcharges depend on the provider and vehicle. The booking page for the selected date should show the current terms.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 809

- Page ITEM: 019 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L153 - `p.page-hero__breadcrumb`
- Element/type: Body text
- Exact English:

  ```text
  Home / Transport / Private Transfer
  ```
- Protected tokens: None identified in this item.

### ITEM 810

- Page ITEM: 020 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L154 - `h1.airport-page-hero__title`
- Element/type: H1
- Exact English:

  ```text
  Incheon Airport Private Transfer to Seoul
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 811

- Page ITEM: 021 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L156 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  A private transfer starts to make sense when one vehicle can take your whole group and all of your luggage straight from the airport to the hotel.
  ```
- Protected tokens: None identified in this item.

### ITEM 812

- Page ITEM: 022 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L157 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  For one or two people traveling light, AREX or the airport limousine bus will usually cost less. The calculation changes with a family or group, several large suitcases, a stroller, an older traveler, a late arrival, or a hotel that leaves an awkward walk from the nearest station.
  ```
- Protected tokens: `AREX`

### ITEM 813

- Page ITEM: 023 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L158 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  The real question is not simply whether a private car costs more. It is whether the extra cost is worth removing the luggage, transfer and last-mile problems after a long flight.
  ```
- Protected tokens: None identified in this item.

### ITEM 814

- Page ITEM: 024 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L167 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Who Benefits Most from an Airport Private Transfer?
  ```
- Protected tokens: None identified in this item.

### ITEM 815

- Page ITEM: 025 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L170 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Private transfer is usually worth considering when your group and luggage fit comfortably into one vehicle and door-to-door travel removes a difficult final leg.
  ```
- Protected tokens: None identified in this item.

### ITEM 816

- Page ITEM: 026 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L171 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Traveling alone or as a couple with manageable luggage? Public transport will usually be the cheaper answer.
  ```
- Protected tokens: None identified in this item.

### ITEM 817

- Page ITEM: 027 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L172 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  Traveling as a family or group with several large bags, arriving late, or staying away from a convenient airport-bus or rail stop? The price gap can become much smaller than it first appears.
  ```
- Protected tokens: None identified in this item.

### ITEM 818

- Page ITEM: 028 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L180 - `h2`
- Element/type: H2
- Exact English:

  ```text
  What “private transfer” means in Korea
  ```
- Protected tokens: `Korea`

### ITEM 819

- Page ITEM: 029 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L182 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  On international booking sites, a private transfer means a pre-booked vehicle reserved for your party rather than a shared ride.
  ```
- Protected tokens: None identified in this item.

### ITEM 820

- Page ITEM: 030 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L183 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  The word “private” describes the ride. It does not mean that an ordinary private car can legally operate as a paid airport service.
  ```
- Protected tokens: None identified in this item.

### ITEM 821

- Page ITEM: 031 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L184 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  In Korea, paid airport transfers are provided through commercial transport or other legally permitted business structures. You may also see the local term “call van.” A Korean call van is a commercial van service and should not be treated as the universal English name for every private transfer sold online.
  ```
- Protected tokens: `Korea`

### ITEM 822

- Page ITEM: 032 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L185 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  For travelers, the practical point is simpler: book through an established service and look at the actual vehicle, passenger limit and luggage limit shown for that reservation.
  ```
- Protected tokens: None identified in this item.

### ITEM 823

- Page ITEM: 033 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L192 - `h2`
- Element/type: H2
- Exact English:

  ```text
  When the extra cost starts to make sense
  ```
- Protected tokens: None identified in this item.

### ITEM 824

- Page ITEM: 034 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L194 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  The economics change once several people are traveling together.
  ```
- Protected tokens: None identified in this item.

### ITEM 825

- Page ITEM: 035 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L195 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Airport rail and buses are priced per passenger. A private transfer is normally priced for the vehicle. Four separate public-transport fares, followed by a taxi or a long walk at the Seoul end, can narrow the difference surprisingly quickly.
  ```
- Protected tokens: `Seoul`

### ITEM 826

- Page ITEM: 036 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L196 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  Luggage can matter even more than the number of people. A group that could easily use the train without bags may feel very different with four full-size suitcases, cabin bags and a stroller.
  ```
- Protected tokens: None identified in this item.

### ITEM 827

- Page ITEM: 037 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L197 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  Late arrivals create another problem. The scheduled landing time is not the time you leave the terminal. Immigration, baggage claim and customs can push the usable arrival time much later, which matters when the last convenient train or bus is approaching.
  ```
- Protected tokens: None identified in this item.

### ITEM 828

- Page ITEM: 038 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L198 - `p:nth-of-type(5)`
- Element/type: Body text
- Exact English:

  ```text
  Door-to-door travel is also more valuable when the hotel is uphill, several blocks from the station, or difficult to reach with luggage.
  ```
- Protected tokens: None identified in this item.

### ITEM 829

- Page ITEM: 039 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L205 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Private transfer compared with other airport options
  ```
- Protected tokens: None identified in this item.

### ITEM 830

- Page ITEM: 040 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L207 - `div.table-scroll.transfer-comparison @aria-label`
- Element/type: ARIA label
- Exact English:

  ```text
  Scrollable comparison of Incheon Airport transfer options
  ```
- Protected tokens: `Incheon Airport`

### ITEM 831

- Page ITEM: 041 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L209 - `caption`
- Element/type: Table caption
- Exact English:

  ```text
  Incheon Airport to Seoul transportation comparison
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 832

- Page ITEM: 042 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L212 - `th:nth-of-type(1)`
- Element/type: Table header
- Exact English:

  ```text
  Option
  ```
- Protected tokens: None identified in this item.

### ITEM 833

- Page ITEM: 043 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L213 - `th:nth-of-type(2)`
- Element/type: Table header
- Exact English:

  ```text
  Door to door
  ```
- Protected tokens: None identified in this item.

### ITEM 834

- Page ITEM: 044 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L214 - `th:nth-of-type(3)`
- Element/type: Table header
- Exact English:

  ```text
  Luggage
  ```
- Protected tokens: None identified in this item.

### ITEM 835

- Page ITEM: 045 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L215 - `th:nth-of-type(4)`
- Element/type: Table header
- Exact English:

  ```text
  Late arrival
  ```
- Protected tokens: None identified in this item.

### ITEM 836

- Page ITEM: 046 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L216 - `th:nth-of-type(5)`
- Element/type: Table header
- Exact English:

  ```text
  Booking
  ```
- Protected tokens: None identified in this item.

### ITEM 837

- Page ITEM: 047 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L221 - `th`
- Element/type: Table header
- Exact English:

  ```text
  Private transfer
  ```
- Protected tokens: None identified in this item.

### ITEM 838

- Page ITEM: 048 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L222 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Yes
  ```
- Protected tokens: None identified in this item.

### ITEM 839

- Page ITEM: 049 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L223 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Depends on booked vehicle
  ```
- Protected tokens: None identified in this item.

### ITEM 840

- Page ITEM: 050 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L224 - `td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  Good when pre-booked
  ```
- Protected tokens: None identified in this item.

### ITEM 841

- Page ITEM: 051 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L225 - `td:nth-of-type(4)`
- Element/type: Table cell
- Exact English:

  ```text
  Usually
  ```
- Protected tokens: None identified in this item.

### ITEM 842

- Page ITEM: 052 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L228 - `th`
- Element/type: Table header
- Exact English:

  ```text
  Regular taxi
  ```
- Protected tokens: None identified in this item.

### ITEM 843

- Page ITEM: 053 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L229 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Yes
  ```
- Protected tokens: None identified in this item.

### ITEM 844

- Page ITEM: 054 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L230 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Trunk space can be limiting
  ```
- Protected tokens: None identified in this item.

### ITEM 845

- Page ITEM: 055 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L231 - `td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  Generally available
  ```
- Protected tokens: None identified in this item.

### ITEM 846

- Page ITEM: 056 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L232 - `td:nth-of-type(4)`
- Element/type: Table cell
- Exact English:

  ```text
  Usually not required
  ```
- Protected tokens: None identified in this item.

### ITEM 847

- Page ITEM: 057 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L235 - `th`
- Element/type: Table header
- Exact English:

  ```text
  Jumbo taxi
  ```
- Protected tokens: None identified in this item.

### ITEM 848

- Page ITEM: 058 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L236 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Yes
  ```
- Protected tokens: None identified in this item.

### ITEM 849

- Page ITEM: 059 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L237 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  More room than a regular taxi
  ```
- Protected tokens: None identified in this item.

### ITEM 850

- Page ITEM: 060 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L238 - `td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  Availability can vary
  ```
- Protected tokens: None identified in this item.

### ITEM 851

- Page ITEM: 061 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L239 - `td:nth-of-type(4)`
- Element/type: Table cell
- Exact English:

  ```text
  Pre-booking can help
  ```
- Protected tokens: None identified in this item.

### ITEM 852

- Page ITEM: 062 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L242 - `th`
- Element/type: Table header
- Exact English:

  ```text
  Airport limousine bus
  ```
- Protected tokens: None identified in this item.

### ITEM 853

- Page ITEM: 063 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L243 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Stop to stop
  ```
- Protected tokens: None identified in this item.

### ITEM 854

- Page ITEM: 064 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L244 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  Good luggage storage, but you handle the last leg
  ```
- Protected tokens: None identified in this item.

### ITEM 855

- Page ITEM: 065 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L245 - `td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  Timetable matters
  ```
- Protected tokens: None identified in this item.

### ITEM 856

- Page ITEM: 066 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L246 - `td:nth-of-type(4)`
- Element/type: Table cell
- Exact English:

  ```text
  Usually optional
  ```
- Protected tokens: None identified in this item.

### ITEM 857

- Page ITEM: 067 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L249 - `th`
- Element/type: Table header
- Exact English:

  ```text
  AREX
  ```
- Protected tokens: `AREX`

### ITEM 858

- Page ITEM: 068 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L250 - `td:nth-of-type(1)`
- Element/type: Table cell
- Exact English:

  ```text
  Rail to Seoul Station / connected stations
  ```
- Protected tokens: `Seoul Station`

### ITEM 859

- Page ITEM: 069 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L251 - `td:nth-of-type(2)`
- Element/type: Table cell
- Exact English:

  ```text
  You carry your own bags
  ```
- Protected tokens: None identified in this item.

### ITEM 860

- Page ITEM: 070 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L252 - `td:nth-of-type(3)`
- Element/type: Table cell
- Exact English:

  ```text
  Last-train time matters
  ```
- Protected tokens: None identified in this item.

### ITEM 861

- Page ITEM: 071 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L253 - `td:nth-of-type(4)`
- Element/type: Table cell
- Exact English:

  ```text
  Optional
  ```
- Protected tokens: None identified in this item.

### ITEM 862

- Page ITEM: 072 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L258 - `p`
- Element/type: Body text
- Exact English:

  ```text
  For a full comparison of AREX, airport buses and taxis, see the main Incheon Airport transfer guide.
  ```
- Protected tokens: `AREX`, `Incheon Airport`

### ITEM 863

- Page ITEM: 073 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L265 - `h2`
- Element/type: H2
- Exact English:

  ```text
  A seven-seat vehicle is not always a seven-person airport vehicle
  ```
- Protected tokens: None identified in this item.

### ITEM 864

- Page ITEM: 074 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L267 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Seat count and airport luggage capacity are two different things.
  ```
- Protected tokens: None identified in this item.

### ITEM 865

- Page ITEM: 075 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L268 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  A vehicle may technically have enough seats for your group and still run out of usable space once every passenger brings a large suitcase.
  ```
- Protected tokens: None identified in this item.

### ITEM 866

- Page ITEM: 076 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L269 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  Count the luggage before looking at the passenger number. Full-size suitcases, cabin bags, strollers, golf bags, bicycle boxes and other oversized items can change the vehicle you need.
  ```
- Protected tokens: None identified in this item.

### ITEM 867

- Page ITEM: 077 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L270 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  Booking sites often show passenger capacity and luggage capacity separately. Those two numbers matter more than the model name of the van.
  ```
- Protected tokens: None identified in this item.

### ITEM 868

- Page ITEM: 078 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L271 - `p:nth-of-type(5)`
- Element/type: Body text
- Exact English:

  ```text
  When the luggage sits close to the published limit, a larger vehicle may be the safer reservation than trying to use every available seat.
  ```
- Protected tokens: None identified in this item.

### ITEM 869

- Page ITEM: 079 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L278 - `h2`
- Element/type: H2
- Exact English:

  ```text
  What happens after you book
  ```
- Protected tokens: None identified in this item.

### ITEM 870

- Page ITEM: 080 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L280 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  The reservation normally asks for your flight number, pickup terminal and destination in Seoul.
  ```
- Protected tokens: `Seoul`

### ITEM 871

- Page ITEM: 081 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L281 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  The flight number matters because the pickup service can follow the actual flight rather than relying only on the scheduled landing time.
  ```
- Protected tokens: None identified in this item.

### ITEM 872

- Page ITEM: 082 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L282 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  Driver details may arrive the day before the trip or closer to arrival, depending on the provider. Some services use the arrival hall and a name sign. Others send a specific exit or meeting point through the booking app, email or a messaging service.
  ```
- Protected tokens: None identified in this item.

### ITEM 873

- Page ITEM: 083 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L283 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  Keep the booking confirmation and the provider's contact channel available after landing. Airport Wi-Fi is usually enough to receive a message if your mobile data is not active yet.
  ```
- Protected tokens: None identified in this item.

### ITEM 874

- Page ITEM: 084 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L284 - `p:nth-of-type(5)`
- Element/type: Body text
- Exact English:

  ```text
  Terminal 1 and Terminal 2 are not interchangeable. The terminal in the reservation should match the arriving flight.
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 875

- Page ITEM: 085 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L291 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Flight delays and waiting time
  ```
- Protected tokens: None identified in this item.

### ITEM 876

- Page ITEM: 086 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L293 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Flight tracking is common on pre-booked airport transfers, but the waiting policy is not identical across every provider or every listing.
  ```
- Protected tokens: None identified in this item.

### ITEM 877

- Page ITEM: 087 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L294 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  Some bookings automatically move the pickup time when the flight is delayed. Others include a fixed free waiting period after landing or after the agreed pickup time. Extra waiting can become chargeable once that window ends.
  ```
- Protected tokens: None identified in this item.

### ITEM 878

- Page ITEM: 088 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L295 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  This is one of the few conditions worth reading on the actual product page before payment, especially for a late-night arrival or a trip involving checked baggage.
  ```
- Protected tokens: None identified in this item.

### ITEM 879

- Page ITEM: 089 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L296 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  Do not hard-code one waiting policy and apply it to all four booking companies.
  ```
- Protected tokens: None identified in this item.

### ITEM 880

- Page ITEM: 090 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L303 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Where to book an Incheon Airport private transfer
  ```
- Protected tokens: `Incheon Airport`

### ITEM 881

- Page ITEM: 091 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L304 - `p.transfer-chapter__answer`
- Element/type: Body text
- Exact English:

  ```text
  The same Incheon Airport–Seoul route appears on several booking platforms, sometimes with different vehicle sizes, luggage limits, waiting rules and cancellation terms.
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 882

- Page ITEM: 092 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L306 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Korea Inside does not rank one of these companies as the universal “best” choice. The useful comparison is the actual vehicle and booking conditions available for your date.
  ```
- Protected tokens: `Korea Inside`

### ITEM 883

- Page ITEM: 093 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L309 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Klook
  ```
- Protected tokens: `Klook`

### ITEM 884

- Page ITEM: 094 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L310 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Klook lists multiple Incheon Airport transfer options and vehicle sizes. Passenger limits, luggage capacity and waiting conditions can differ between individual listings, so the selected vehicle matters more than the marketplace name alone.
  ```
- Protected tokens: `Klook`, `Incheon Airport`

### ITEM 885

- Page ITEM: 095 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L311 - `p:nth-of-type(2)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  View Klook transfer options
  ```
- Protected tokens: `Klook`

### ITEM 886

- Page ITEM: 096 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L314 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Trip.com
  ```
- Protected tokens: `Trip.com`

### ITEM 887

- Page ITEM: 097 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L315 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Trip.com has a dedicated airport-transfer booking flow for Incheon Airport, with vehicle options tied to the flight and pickup details entered during booking. The exact passenger, luggage and cancellation conditions depend on the selected reservation.
  ```
- Protected tokens: `Trip.com`, `Incheon Airport`

### ITEM 888

- Page ITEM: 098 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L316 - `p:nth-of-type(2)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  View Trip.com transfer options
  ```
- Protected tokens: `Trip.com`

### ITEM 889

- Page ITEM: 099 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L319 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Trazy
  ```
- Protected tokens: `Trazy`

### ITEM 890

- Page ITEM: 100 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L320 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Trazy focuses on Korea travel and its airport-transfer listings often show passenger and luggage limits in detail. Late-night charges, waiting time and cancellation rules can vary by product and should remain attached to the specific reservation rather than being generalized across the whole platform.
  ```
- Protected tokens: `Trazy`, `Korea`

### ITEM 891

- Page ITEM: 101 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L321 - `p:nth-of-type(2)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  View Trazy transfer options
  ```
- Protected tokens: `Trazy`

### ITEM 892

- Page ITEM: 102 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L324 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Booking.com
  ```
- Protected tokens: `Booking.com`

### ITEM 893

- Page ITEM: 103 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L325 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  Booking.com offers airport taxi and private-transfer reservations alongside its wider travel booking services. Vehicle size, luggage capacity, meeting arrangements and cancellation terms vary by listing.
  ```
- Protected tokens: `Booking.com`

### ITEM 894

- Page ITEM: 104 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L326 - `p:nth-of-type(2)`
- Element/type: CTA / visible link text
- Exact English:

  ```text
  View Booking.com transfer options
  ```
- Protected tokens: `Booking.com`

### ITEM 895

- Page ITEM: 105 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L335 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Common booking mistakes
  ```
- Protected tokens: None identified in this item.

### ITEM 896

- Page ITEM: 106 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L339 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Booking by seats alone
  ```
- Protected tokens: None identified in this item.

### ITEM 897

- Page ITEM: 107 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L340 - `p`
- Element/type: Body text
- Exact English:

  ```text
  A vehicle with enough seats can still be too small once several large suitcases are added. Passenger capacity and luggage capacity need to work at the same time.
  ```
- Protected tokens: None identified in this item.

### ITEM 898

- Page ITEM: 108 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L343 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Leaving out the flight number
  ```
- Protected tokens: None identified in this item.

### ITEM 899

- Page ITEM: 109 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L344 - `p`
- Element/type: Body text
- Exact English:

  ```text
  The flight number is what allows the pickup service to follow delays and identify the correct terminal.
  ```
- Protected tokens: None identified in this item.

### ITEM 900

- Page ITEM: 110 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L347 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Entering the wrong terminal
  ```
- Protected tokens: None identified in this item.

### ITEM 901

- Page ITEM: 111 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L348 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Incheon Terminal 1 and Terminal 2 use different arrival areas. The terminal on the reservation should match the arriving flight.
  ```
- Protected tokens: `Incheon`, `Terminal 1`, `Terminal 2`

### ITEM 902

- Page ITEM: 112 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L351 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Treating midnight as the previous day
  ```
- Protected tokens: None identified in this item.

### ITEM 903

- Page ITEM: 113 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L352 - `p`
- Element/type: Body text
- Exact English:

  ```text
  A pickup just after midnight belongs to the new calendar date. This is an easy mistake when the flight departs one day and lands after midnight on another.
  ```
- Protected tokens: None identified in this item.

### ITEM 904

- Page ITEM: 114 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L355 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Using only the hotel name
  ```
- Protected tokens: None identified in this item.

### ITEM 905

- Page ITEM: 115 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L356 - `p`
- Element/type: Body text
- Exact English:

  ```text
  A full hotel or accommodation address is safer than relying on a translated property name, especially for smaller hotels and apartments.
  ```
- Protected tokens: None identified in this item.

### ITEM 906

- Page ITEM: 116 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L359 - `h3`
- Element/type: H3
- Exact English:

  ```text
  Assuming every booking has unlimited waiting
  ```
- Protected tokens: None identified in this item.

### ITEM 907

- Page ITEM: 117 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L360 - `p`
- Element/type: Body text
- Exact English:

  ```text
  Flight tracking and free waiting are not the same thing. The included waiting period depends on the actual product booked.
  ```
- Protected tokens: None identified in this item.

### ITEM 908

- Page ITEM: 118 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L369 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Frequently asked questions
  ```
- Protected tokens: None identified in this item.

### ITEM 909

- Page ITEM: 119 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L373 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Is a private transfer worth it for four people?
  ```
- Protected tokens: None identified in this item.

### ITEM 910

- Page ITEM: 120 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L374 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Sometimes. Four separate public-transport fares can narrow the price gap, especially when luggage or another taxi ride is needed at the Seoul end. The useful comparison is the total door-to-door cost, not only the first fare you see.
  ```
- Protected tokens: `Seoul`

### ITEM 911

- Page ITEM: 121 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L377 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Can a regular taxi fit four people and four large suitcases?
  ```
- Protected tokens: None identified in this item.

### ITEM 912

- Page ITEM: 122 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L378 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Not reliably. Passenger capacity and trunk space are separate limits. When every passenger has a large suitcase, a jumbo taxi or a pre-booked larger vehicle may be more practical.
  ```
- Protected tokens: None identified in this item.

### ITEM 913

- Page ITEM: 123 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L381 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  What happens if my flight is delayed?
  ```
- Protected tokens: None identified in this item.

### ITEM 914

- Page ITEM: 124 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L382 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Many pre-booked airport transfers ask for the flight number and track delays, but free waiting time varies by provider and product. The current waiting rule on the selected reservation is the one that matters.
  ```
- Protected tokens: None identified in this item.

### ITEM 915

- Page ITEM: 125 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L385 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Where will I meet the driver at Incheon Airport?
  ```
- Protected tokens: `Incheon Airport`

### ITEM 916

- Page ITEM: 126 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L386 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  The exact arrangement depends on the booking. Common methods include a driver waiting in the arrival hall with a name sign or a message directing you to a specific exit or meeting point.
  ```
- Protected tokens: None identified in this item.

### ITEM 917

- Page ITEM: 127 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L389 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Does it matter whether I arrive at Terminal 1 or Terminal 2?
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 918

- Page ITEM: 128 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L390 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Yes. The terminals have separate arrival areas and pickup points. Use the terminal shown for your arriving flight when making the reservation.
  ```
- Protected tokens: None identified in this item.

### ITEM 919

- Page ITEM: 129 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L393 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Is a private transfer the same as a call van in Korea?
  ```
- Protected tokens: `Korea`

### ITEM 920

- Page ITEM: 130 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L394 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  No. “Private transfer” is an international booking term for a vehicle reserved for one party. “Call van” is a Korean commercial van-service term. Some airport transfer vehicles may fall into that category, but the terms are not interchangeable.
  ```
- Protected tokens: None identified in this item.

### ITEM 921

- Page ITEM: 131 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L397 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Do I need to pre-book transportation from Incheon Airport?
  ```
- Protected tokens: `Incheon Airport`

### ITEM 922

- Page ITEM: 132 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L398 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Not for every trip. Solo travelers and couples with manageable luggage often have straightforward train, bus or taxi options. Pre-booking becomes more useful when vehicle size, luggage space, a late arrival or a large group makes finding the right vehicle on arrival more important.
  ```
- Protected tokens: None identified in this item.

### ITEM 923

- Page ITEM: 133 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L401 - `summary`
- Element/type: FAQ question
- Exact English:

  ```text
  Can I book a private transfer for a late-night arrival?
  ```
- Protected tokens: None identified in this item.

### ITEM 924

- Page ITEM: 134 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L402 - `p`
- Element/type: FAQ answer
- Exact English:

  ```text
  Many Incheon Airport transfer products operate late at night or around the clock, but availability and possible night surcharges depend on the provider and vehicle. The booking page for the selected date should show the current terms.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 925

- Page ITEM: 135 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L411 - `h2`
- Element/type: H2
- Exact English:

  ```text
  Should You Book an Airport Private Transfer?
  ```
- Protected tokens: None identified in this item.

### ITEM 926

- Page ITEM: 136 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L413 - `p:nth-of-type(1)`
- Element/type: Body text
- Exact English:

  ```text
  If you are traveling alone or as a couple with manageable luggage and your arrival still fits the train or airport-bus timetable, paying for a private vehicle is usually hard to justify.
  ```
- Protected tokens: None identified in this item.

### ITEM 927

- Page ITEM: 137 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L414 - `p:nth-of-type(2)`
- Element/type: Body text
- Exact English:

  ```text
  The case becomes stronger with a family or group, several large bags, a stroller, an older traveler, a late usable arrival time, or a hotel that leaves an awkward final walk from the station.
  ```
- Protected tokens: None identified in this item.

### ITEM 928

- Page ITEM: 138 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L415 - `p:nth-of-type(3)`
- Element/type: Body text
- Exact English:

  ```text
  Once a private transfer makes sense for the trip, look at the vehicle and luggage limit first, then the waiting and cancellation terms.
  ```
- Protected tokens: None identified in this item.

### ITEM 929

- Page ITEM: 139 of 139
- File: `incheon-airport-private-transfer.html`
- Line/context: L416 - `p:nth-of-type(4)`
- Element/type: Body text
- Exact English:

  ```text
  The OTA logo comes after those details.
  ```
- Protected tokens: None identified in this item.

## Independent omission audit

The six English files were re-read from the beginning with an independent coverage pass after the primary extraction. The audit compared raw user-visible text nodes and user-facing attributes against the ITEM corpus, then separately rechecked headings, body/CTA/FAQ, alternative text, ARIA labels, JSON-LD user-facing values, related-guide copy, responsive `data-label` values, and CSS/JS-rendered strings.

| English file | ITEMs | title/meta/H1-H3 missing | body/CTA/FAQ missing | alt/ARIA missing | JSON-LD missing | visible `data-label` missing | final unlisted English |
|---|---:|---:|---:|---:|---:|---:|---:|
| `airport.html` | 110 | 0 | 0 | 0 | 0 | 0 | 0 |
| `arrival.html` | 111 | 0 | 0 | 0 | 0 | 0 | 0 |
| `airport-transfer.html` | 166 | 0 | 0 | 0 | 0 | 0 | 0 |
| `arex.html` | 221 | 0 | 0 | 0 | 0 | 0 | 0 |
| `airport-bus.html` | 182 | 0 | 0 | 0 | 0 | 0 | 0 |
| `incheon-airport-private-transfer.html` | 139 | 0 | 0 | 0 | 0 | 0 | 0 |
| **Total** | **929** | **0** | **0** | **0** | **0** | **0** | **0** |

### Audit evidence totals

- Continuous Batch ITEM numbering: `ITEM 001` through `ITEM 929`; gaps `0`; duplicate source targets `0`.
- Page ITEM numbering: each page begins at `001` and ends at its recorded page count; gaps `0`; duplicates `0`.
- Source-integrity SHA-256 mismatches: `0`.
- H1 count: one per page; pages outside the required count: `0`.
- JSON-LD user-facing values audited: `48`; missing `0`.
- Page-specific non-empty alt values audited: `17`; missing `0`.
- Page-specific literal ARIA labels audited: `20`; missing `0`. Structural `aria-labelledby`/`aria-describedby` ID references remain protected, and their referenced visible copy is represented by the corresponding visible-text ITEM.
- Related-guide user-facing ITEMs audited: `19`; missing `0`.
- Visible FAQ question/answer ITEMs audited: `104`; missing `0`.
- CSS exposure check: `style.css` contains `content: attr(data-label)` rules.
- Mobile user-facing `data-label` ITEMs in the six target files: `0`; missing `0`.
- Page-specific CSS/JS-generated user-facing literals in the six target files: `0`; missing `0`.
- Protected-token record errors: `0`.
- Final page-specific user-visible English strings not represented in this Source MD: `0`.
