# Korea Inside — ES + JA Infographic English-Text Audit

**Status:** LOCAL IMPLEMENTATION COMPLETE — FINAL LOCAL QA PASS — 30/30 TARGETS

**Research date:** 2026-09-27

**Repository branch:** `infographic-localization`

**Original inventory baseline:** `8d84b5bd88eb3871d386521caf4012af2af57c4b`

**Final implementation / local QA checkpoint:** `2cf26a55861eb264e2103c29f9953b0632f3266c`

**Taiwan Production baseline included in the checkpoint:** `6d327d40c2758d6545b7f6bb0958dd4f8ee5a86a`

**Scope:** Production HTML directly under `es/` and `ja/`

**Next action:** Authorized final audit commit, feature push, guarded main merge/push, Production READY confirmation, and public QA. These release steps are pending at this document checkpoint and are not claimed as completed here.

## 1. Scope

This document preserves the original inventory-only audit and records the final localization decision and local QA in section 10. The original audit identified image assets used by the Spanish and Japanese production pages whose pixels or SVG markup contain English instructional, decision, comparison, map, UI-style, or callout text.

During the original inventory-only audit, no HTML, image, CSS, JavaScript, `src`, `srcset`, alt text, caption, navigation, SEO metadata, or production state was changed. Later approved implementation is recorded separately below; this historical statement does not describe the current implementation status.

### Original audit method

- Enumerated every `*.html` file directly under `es/` and `ja/`.
- Scanned every `<img src>`, `<img srcset>`, and `<picture><source srcset>` reference.
- Checked HTML context, filename, classes, alt text, captions, and nearby copy to produce a candidate set.
- Inspected candidate raster pixels directly.
- Inspected SVG text nodes directly.
- Deduplicated referenced assets by SHA-256, not filename alone.
- Counted a rendered `<img>` as one occurrence; `srcset` alternatives were scanned as references but were not counted as additional rendered occurrences.
- Found no content infographic implemented through CSS `background-image`; color and gradient backgrounds were excluded.

### Original audit scope totals

| Measure | Result |
|---|---:|
| ES HTML pages scanned | 58 |
| JA HTML pages scanned | 58 |
| Rendered image occurrences scanned (`<img>`) | 570 |
| Local `src` / `srcset` reference records scanned | 776 |
| Referenced image file paths | 239 |
| Unique referenced image assets after SHA-256 deduplication | 235 |
| Unique infographic candidates escalated to pixel/SVG inspection | 37 |
| Confirmed English-text infographic assets | 33 |
| Review-required assets | 0 |

## 2. Original Audit Aggregate and Final Scope Correction

| Measure | Result |
|---|---:|
| Unique English-text infographic assets | 33 |
| ES affected pages | 18 |
| JA affected pages | 18 |
| ES occurrences | 34 |
| JA occurrences | 34 |
| Total occurrences | 68 |
| Shared ES + JA assets | 33 |
| ES-only assets | 0 |
| JA-only assets | 0 |
| P1 assets | 14 |
| P2 assets | 16 |
| P3 assets | 3 |
| REVIEW_REQUIRED | 0 |

The table above preserves the initial 33-item inventory. At that baseline, no ES or JA localized replacements were referenced. The initial recommendation to localize all 33 items is superseded by the user's final decision: INF-028, INF-030, and INF-032 are EXCLUDE / BRAND-PRODUCT VISUAL. Their real card/machine appearance must remain original across languages.

| Final measure | Result |
|---|---:|
| Localization target | 30 |
| P1 | 14 COMPLETE |
| P2 | 16 COMPLETE |
| EXCLUDE / BRAND-PRODUCT VISUAL | 3 |
| ES localization | 30/30 COMPLETE |
| JA localization | 30/30 COMPLETE |
| Localized asset files | 60 |
| Localized image references | 60 |
| Excluded original product-visual references | 8 |

Final completion is **30/30**, not 33/33. See section 10 for verification and release status.

### Affected ES pages

`airport-bus.html`, `airport.html`, `arex.html`, `arrival.html`, `best-area-for-couples-seoul.html`, `best-area-for-families-seoul.html`, `best-area-for-first-time-visitors-seoul.html`, `best-area-for-luxury-hotels-seoul.html`, `best-area-for-nightlife-seoul.html`, `esim.html`, `hongdae-travel-guide.html`, `hongdae-vs-myeongdong.html`, `index.html`, `jamsil-travel-guide.html`, `maps.html`, `seongsu-travel-guide.html`, `tmoney.html`, `wowpass.html`.

### Affected JA pages

The same 18 filenames under `ja/`.

## 3. Unique Asset Inventory

The embedded-English field preserves the original inventory of readable headings, labels, decision lines, and key callouts. Repeated brand names and repeated app-screen labels are consolidated. Original paths, hashes, dimensions, and occurrence records remain historical source evidence. For P1/P2 entries, the original “localization needed: YES” fields describe the initial requirement; all 30 are now COMPLETE in both ES and JA. The three excluded entries explicitly record the superseding final decision.

### INF-001 — Couples stay area guide

- Priority: P1
- Asset: `images/Accommodation/couples-stay-seoul-area-guide.png`
- SHA-256: `184d0526b2c08d4cdc1c5c4b8a09591e1486a38180fb711caae1b40464a35a9c`
- File / dimensions: PNG, 1536 × 1024
- Embedded English: “Where Should Couples Stay in Seoul?”; “A quick area guide for different couple travel styles”; “Start with the atmosphere you want, then check transport, nighttime noise and the final hotel route.”; “QUICK MATCH”; “Nightlife and cafés → Hongdae”; “Design shops and local cafés → Seongsu”; “Traditional streets and quiet evenings → Insadong”; “Central sightseeing and shopping → Myeongdong”; “Premium shopping and southern Seoul → Gangnam”; “Lotte World and evening lake walks → Jamsil”; “TOP PICKS”; Hongdae / Seongsu / Insadong best-for and “Watch out” callouts.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/best-area-for-couples-seoul.html` — 1 occurrence; `<figure class="couples-stay-infographic">`; localized alt and caption present; `src` only, no `srcset`.
- JA: `ja/best-area-for-couples-seoul.html` — 1 occurrence; same DOM context; localized alt and caption present; `src` only, no `srcset`.
- Total occurrences: 2

### INF-002 — Family stay area guide

- Priority: P1
- Asset: `images/Accommodation/family-stay-seoul-area-guide.png`
- SHA-256: `5b241eb23e123238e527694cf2d3e377dac5244b355d661ec180b4f2fd5442a1`
- File / dimensions: PNG, 1491 × 1055
- Embedded English: “Where Should Your Family Stay in Seoul?”; “A quick area guide for families”; “Start with your main family priority, then choose the Seoul area that makes the trip easier.”; six quick matches for Myeongdong, Jamsil, Mapo / Gongdeok, Insadong, Seoul Station, and Hongdae; “TOP PICKS”; “Top 3 family-friendly Seoul bases”; best-for and “Watch out” callouts.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/best-area-for-families-seoul.html` — 1 occurrence; `<figure class="family-stay-infographic">`; localized alt; no caption in this figure; `src` only.
- JA: `ja/best-area-for-families-seoul.html` — 1 occurrence; same context; localized alt; no caption in this figure; `src` only.
- Total occurrences: 2

### INF-003 — First-time visitor stay area guide

- Priority: P1
- Asset: `images/Accommodation/first-time-seoul-area-guide.png`
- SHA-256: `2e958e12213927ce0a4a15588c525875ec14323eaf04486ae4e468784d988421`
- File / dimensions: PNG, 1536 × 1024
- Embedded English: “Where Should First-Time Visitors Stay in Seoul?”; “A quick area guide for your first trip”; “QUICK MATCH”; “Central sightseeing and easy planning → Myeongdong”; “Cafés, nightlife and direct AREX → Hongdae”; “Airport rail, KTX and large luggage → Seoul Station”; “Airport access and quieter nights → Mapo / Gongdeok”; “Palaces and traditional streets → Insadong”; “Lotte World and southern Seoul → Jamsil”; “TOP PICKS”; best-for and “Watch out” callouts.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/best-area-for-first-time-visitors-seoul.html` — 1 occurrence; `<figure class="first-time-stay-infographic">`; localized alt; no caption; `src` only.
- JA: `ja/best-area-for-first-time-visitors-seoul.html` — 1 occurrence; same context; localized alt; no caption; `src` only.
- Total occurrences: 2

### INF-004 — Luxury stay area guide

- Priority: P1
- Asset: `images/Accommodation/luxury-stay-seoul-area-guide.png`
- SHA-256: `a88d4eeb4a1678a82136a1bdb8893859e99544c41152c884990735e5ad364935`
- File / dimensions: PNG, 1536 × 1024
- Embedded English: “Where Should Luxury Travelers Stay in Seoul?”; “A quick area guide for premium stays”; six quick matches for Gangnam, Jamsil, Myeongdong, Seoul Station / Namdaemun, Insadong, and Itaewon; “TOP PICKS”; Gangnam / Jamsil / Myeongdong best-for and “Watch out” callouts.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/best-area-for-luxury-hotels-seoul.html` — 1 occurrence; `<figure class="luxury-stay-infographic">`; localized alt and caption; `src` only.
- JA: `ja/best-area-for-luxury-hotels-seoul.html` — 1 occurrence; same context; localized alt and caption; `src` only.
- Total occurrences: 2

### INF-005 — Nightlife stay area guide

- Priority: P1
- Asset: `images/Accommodation/nightlife-stay-seoul-area-guide.png`
- SHA-256: `ae765cc088069bc9bd156b6e8f9b45e564d08ad6d4e460cfb39249401c2b5a2f`
- File / dimensions: PNG, 1536 × 1024
- Embedded English: “Where Should Nightlife Travelers Stay in Seoul?”; “A quick area guide for nights out and easier returns”; “Clubs, live music and youthful energy → Hongdae”; “International bars and social nights → Itaewon”; “Upscale clubs and late dinners → Gangnam”; “Central sightseeing with occasional nights out → Myeongdong”; “Quieter sleep with easy Hongdae access → Mapo / Gongdeok”; “Airport rail and early departures → Seoul Station”.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/best-area-for-nightlife-seoul.html` — 1 occurrence; `<figure class="nightlife-stay-infographic">`; localized alt; no caption; `src` only.
- JA: `ja/best-area-for-nightlife-seoul.html` — 1 occurrence; same context; localized alt; no caption; `src` only.
- Total occurrences: 2

### INF-006 — Hongdae vs Myeongdong hero comparison

- Priority: P1
- Asset: `images/Accommodation/hongdae-vs-myeongdong.webp`
- SHA-256: `a1bdbff1f785e12489fc30bd3009f8c46648d5f3aba48e5dc314c4b7b6707537`
- File / dimensions: WebP, 1672 × 941
- Embedded English: “Hongdae”; “vs”; “Myeongdong”; “Cafés • Creative Culture • Youthful Energy”; “Shopping • Central Location • Easy Access”. Natural storefront signage is not part of the localization inventory.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/hongdae-vs-myeongdong.html` — 1 occurrence; `<figure class="stay-hero-media">`; localized alt and caption; `src` only.
- JA: `ja/hongdae-vs-myeongdong.html` — 1 occurrence; same context; localized alt and caption; `src` only.
- Total occurrences: 2

### INF-007 — Airport bus boarding location guide

- Priority: P2
- Asset: `images/airport/airport-bus-boarding-location-guide.png`
- SHA-256: `45f281d1e6a80f4fac1f36f12399324edf8526ec9d63771f63fa1a3529c7664d`
- File / dimensions: PNG, 1448 × 1086
- Embedded English: “Airport Bus Boarding Location Guide”; “INCHEON AIRPORT BUS NUMBER & BOARDING CHECK”; “START HERE”; Terminal 1 / Terminal 2 routing; “STEP 1 — FIND THE NUMBER”; “STEP 2 — MATCH THE DESTINATION”; “STEP 3 — CHECK THE PLATFORM”; “STEP 4 — BOARD SAFELY”; “COMMON ROUTE TYPES”; “IMPORTANT NOTE”; route-family and boarding-change warnings.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/airport-bus.html` — 1 occurrence; `<figure class="airport-bus-media__figure airport-bus-media__figure--wide">`; localized alt and caption; `src` only.
- JA: `ja/airport-bus.html` — 1 occurrence; same context; localized alt and caption; `src` only.
- Total occurrences: 2

### INF-008 — Airport bus by hotel area

- Priority: P2
- Asset: `images/airport/airport-bus-by-hotel-area.png`
- SHA-256: `c85d8dac5f4cc3c865944fb03b0f30ee1efc8f6402b5b6d47ef33ee3e93ee680`
- File / dimensions: PNG, 1536 × 1024
- Embedded English: “Terminal 2 Ground Transportation Guide”; “INCHEON AIRPORT TERMINAL 2 ARRIVAL HALL (1F)”; “START HERE”; “TERMINAL 2 ARRIVAL HALL (1F)”; “Bus Stop”; “TAXI”; “Pick-up Point”; “AIRPORT BUS”; “TAXI”; “AREX / TRAIN”; “PICK-UP”; fare, time, level, and traffic labels. The right and lower panels contain Korean labels plus route numbers.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/airport-bus.html` — 1 occurrence; wide airport-bus figure; localized alt and caption; `src` only.
- JA: `ja/airport-bus.html` — 1 occurrence; same context; localized alt and caption; `src` only.
- Total occurrences: 2

### INF-009 — Airport limousine bus step guide

- Priority: P2
- Asset: `images/airport/airport-bus-how-to-use.png`
- SHA-256: `dbd7c6e1cebad34b32fa29aba695eca172762e522f7848a32d9ee4389221c17b`
- File / dimensions: PNG, 1536 × 1024
- Embedded English: “Airport Limousine Bus – How to Use (Step by Step)”; “Check Your Destination & Bus”; “Go to the Bus Stop”; “Buy Your Ticket”; “Board the Bus”; “Enjoy the Ride”; “Get Off at Your Stop”; “Walk to Your Accommodation”; “Arrive Safely!”; “TIP & INFO”; operating hours, luggage, payment, traffic, children, official-source, and update labels.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/airport-bus.html` — 1 occurrence; wide airport-bus figure; localized alt and caption; `src` only.
- JA: `ja/airport-bus.html` — 1 occurrence; same context; localized alt and caption; `src` only.
- Total occurrences: 2

### INF-010 — AREX Express vs All-Stop route map

- Priority: P1
- Asset: `images/airport/arex/arex-express-vs-all-stop-route-map.png`
- SHA-256: `101e302778f7540d2eb7a484a6aaa63fad68138ced4bb7e88d87223b46de177c`
- File / dimensions: PNG, 1448 × 1086
- Embedded English: “AREX Express vs All-Stop Route Map”; “Incheon Airport to Seoul”; “AREX Express”; “Faster, reserved-seat train”; “AREX All-Stop”; “Local train with more stops”; terminal/station names; fare and travel-time labels; “Separate Express ticket required”; “Use T-money or a single-use subway ticket”; boarding check warning.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/arex.html` — 1 occurrence; `<figure class="arex-infographic">`; localized alt and caption; `src` only.
- JA: `ja/arex.html` — 1 occurrence; same context; localized alt and caption; `src` only.
- Total occurrences: 2

### INF-011 — AREX hero comparison

- Priority: P1
- Asset: `images/airport/arex/arex-hero-express-vs-all-stop.png`
- SHA-256: `9e2aa0864a69d90ab4c72d20d62251a1fa4e20b89513acfc63b37e7ed4ff13a8`
- File / dimensions: PNG, 1672 × 941
- Embedded English: “AREX Express or All-Stop?”; “Choose by destination, luggage and transfer.”; “Express”; “All-Stop”; Terminal 2, Terminal 1, Gimpo Airport, Hongik University, Gongdeok, Seoul Station.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/arex.html` — 1 occurrence; `<figure class="arex-hero-visual">`; localized alt; no caption; `src` only.
- JA: `ja/arex.html` — 1 occurrence; same context; localized alt; no caption; `src` only.
- Total occurrences: 2

### INF-012 — AREX station by destination

- Priority: P1
- Asset: `images/airport/arex/arex-station-by-destination.png`
- SHA-256: `79df52d85319a62198e7e3dfbbb4d9c9bd829484754c5733abc6fa3bdde82606`
- File / dimensions: PNG, 768 × 1024
- Embedded English: “Which AREX Station Should You Use?”; legend for Express, All-Stop, transfer, luggage, and best choice; table headings “DESTINATION”, “BEST AREX TRAIN”, “GET OFF AT”, “TRANSFER & CONNECTIONS”, “BEST FOR”, “OUR RECOMMENDATION”; destination rows for Seoul Station, Hongdae, Gongdeok, Myeongdong, Gangnam, Jamsil, and Gimpo Airport; “EXPRESS vs ALL-STOP AT A GLANCE”; “TIPS”; fare/time warning.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/arex.html` — 1 occurrence; AREX infographic figure; localized alt and caption; `src` only.
- JA: `ja/arex.html` — 1 occurrence; same context; localized alt and caption; `src` only.
- Total occurrences: 2

### INF-013 — AREX terminal directions

- Priority: P2
- Asset: `images/airport/arex/arex-terminal-1-2-directions.png`
- SHA-256: `66775d9edefd211995cfa6b0a9e8d6d3c10058b39270e78f503417048896daa0`
- File / dimensions: PNG, 736 × 1024
- Embedded English: “How to Find AREX at Incheon Airport (T1 & T2)”; “Step-by-Step Directions from Arrival to Platform”; Express / All-Stop legend; Terminal 1 and Terminal 2 step labels “Arrival Hall (1F)”, “Follow the Signs”, “Go to Transportation Center (B1F)”, “Buy Your Ticket”, “Go to the Platforms”, “Board the Train”; “IMPORTANT NOTES”; official-source and update labels.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/arex.html` — 1 occurrence; AREX infographic figure; localized alt and caption; `src` only.
- JA: `ja/arex.html` — 1 occurrence; same context; localized alt and caption; `src` only.
- Total occurrences: 2

### INF-014 — AREX ticket decision guide

- Priority: P1
- Asset: `images/airport/arex/arex-ticket-decision-guide.png`
- SHA-256: `bad56057f050c4b3f293535acf5a4a8a9bebf13ef8533651aeafb0fdc8c8c7bb`
- File / dimensions: PNG, 1536 × 1024
- Embedded English: “AREX Ticket Decision Guide”; “Answer a few simple questions to choose the best way to travel.”; “START HERE”; five decision questions; “EXPRESS TRAIN (Online Reservation)”; “EXPRESS TRAIN (At Station)”; “ALL-STOP TRAIN (T-money)”; “ALL-STOP TRAIN (Single-Journey Ticket)”; “EXPRESS TRAIN (Recommended)”; “CHECK SCHEDULE FIRST”; how-to-buy/use/pay labels; ticket overview; payment methods; “GOOD TO KNOW”; official website and source/update labels.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/arex.html` — 1 occurrence; AREX infographic figure; localized alt and caption; `src` only.
- JA: `ja/arex.html` — 1 occurrence; same context; localized alt and caption; `src` only.
- Total occurrences: 2

### INF-015 — When not to use AREX

- Priority: P2
- Asset: `images/airport/arex/when-not-to-use-arex.png`
- SHA-256: `7fcdaaa24938685cfc5b7f7201d4453a17e515bfa807b44a241f1ae6eb96959d`
- File / dimensions: PNG, 1448 × 1086
- Embedded English: “When Should You NOT Use AREX?”; “Incheon Airport to Seoul · When another option may be better”; “Heavy luggage or a stroller”; “Your hotel is in Gangnam or Jamsil”; “You arrive very late”; “You are traveling with family or older adults”; “A bus stop is near your hotel”; “BETTER ALTERNATIVES”; Airport Bus / Taxi / Private Pickup best-for labels; concluding AREX note and latest-route warning.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/arex.html` — 1 occurrence; AREX infographic figure; localized alt and caption; `src` only.
- JA: `ja/arex.html` — 1 occurrence; same context; localized alt and caption; `src` only.
- Total occurrences: 2

### INF-016 — Arrival hall first 30 minutes

- Priority: P1
- Asset: `images/airport-arrival-hall-first-30-minutes-infographic.webp`
- SHA-256: `b8fb577669046fd7945d74b46fa3969a0924d258fb39bf7abf8dc12f20318ab8`
- File / dimensions: WebP, 1402 × 1122
- Embedded English: “YOUR FIRST 30 MINUTES”; “IN THE ARRIVAL HALL”; explanatory arrival-hall sentence; “CONNECT”; “Check your eSIM or Wi-Fi.”; “SAVE YOUR ADDRESS”; “Keep your hotel address in Korean ready.”; “CHECK PAYMENT”; “Confirm one working payment method.”; “CHOOSE TRANSPORT”; “AREX · Bus · Taxi · Pre-booked Transfer · Rental Car”; added sign labels including Arrivals, Baggage Claim, Customs, AREX, Bus, Taxi, Parking, Information, and Free Wi-Fi.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/airport.html` — 1 occurrence; `<figure class="airport-page-hero__photo">`; localized alt; no caption; `src` only.
- JA: `ja/airport.html` — 1 occurrence; same context; localized alt; no caption; `src` only.
- Total occurrences: 2

### INF-017 — Terminal arrival maps

- Priority: P2
- Asset: `images/arrival/terminal-arrival-maps.png`
- SHA-256: `6ef80718a20a726ab5f73ddae0821284d43a887b7f2dbfd6f23f963e058735dd`
- File / dimensions: PNG, 1536 × 1024
- Embedded English: “Terminal 1 Arrival Map”; “Terminal 2 Arrival Map”; Gates ranges; “Quarantine Information”; bilingual location labels including Information, Medical Center, Restrooms, Currency Exchange, Arrivals Hall A, Bus Ticket Counters, Bus Stops, Taxi Stands, and Short-term Parking. Most remaining labels are Korean.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/arrival.html` — 1 occurrence; `<figure class="arrival-terminal-map">`; localized alt and caption; `src` only.
- JA: `ja/arrival.html` — 1 occurrence; same context; localized alt and caption; `src` only.
- Total occurrences: 2

### INF-018 — Homepage eSIM hero visual

- Priority: P1
- Asset: `images/esim/hero-esim.png`
- SHA-256: `ff4a3af40f390adfd66951f0e9127d158b6c8570df323958a14bb57c89d6b7e8`
- File / dimensions: PNG, 1663 × 946
- Embedded English: “eSIM for Korea”; “Stay Connected in Korea”; “Easy eSIM setup. Fast internet. Stay connected wherever you go.”; “High Speed Data”; “Voice Calls Available”; “SMS Supported”; “Check Compatibility”; “Compare Options”; phone UI text “KOREA eSIM”, “Scan QR code to install eSIM”, and “Connected!”.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/index.html` — 1 occurrence; home journey section; localized alt; no caption; `src` only.
- JA: `ja/index.html` — 1 occurrence; same context; localized alt; no caption; `src` only.
- Total occurrences: 2

### INF-019 — eSIM quick decision flow

- Priority: P1
- Asset: `images/esim/esim-korea-quick-decision.webp`
- SHA-256: `2fda5c641e8febb1599054e7fd9ad7044224e8acf430a245139ea29304b037e7`
- File / dimensions: WebP, 1536 × 600
- Embedded English: “PHONE SUPPORTS ESIM?”; “No → Physical SIM”; “DATA ONLY?”; “Yes → Data-only travel eSIM”; “NEED A KOREAN NUMBER?”; “Yes → Korean carrier tourist eSIM or SIM”; “ROAMING INSTEAD?”; “Choose → Roaming”.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/esim.html` — 1 occurrence; `<figure class="esim-infographic">`; localized alt and caption; `src` only.
- JA: `ja/esim.html` — 1 occurrence; same context; localized alt and caption; `src` only.
- Total occurrences: 2

### INF-020 — Homepage arrival/navigation/payment guide

- Priority: P1
- Asset: `images/home/arrival-guide.png`
- SHA-256: `3ac066f417301e78c6ada98552f22a4e085e916b55702370a836a333aab2d57c`
- File / dimensions: PNG, 1457 × 1080
- Embedded English: “Airport arrival”; “Step by step guide from landing to the city”; “Local maps”; “Apps, navigation tips and must-know info”; “T-money”; “How to buy, top up and use in Korea”; “WOWPASS”; “Payment, balance, benefits and usage guide”; additional phone-map and product labels.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/index.html` — 1 occurrence; home journey section; localized alt; no caption; `src` only.
- JA: `ja/index.html` — 1 occurrence; same context; localized alt; no caption; `src` only.
- Total occurrences: 2

### INF-021 — Hongdae at-a-glance map

- Priority: P2
- Asset: `images/hongdae/hongdae-at-a-glance-map.webp`
- SHA-256: `0c3ef2afcf52b2bef155443b5684a1a8400337a4ae01ee8b3c06f17a66544b88`
- File / dimensions: WebP, 1672 × 941
- Embedded English: “Hongdae at a Glance”; subtitle; compass labels; Gyeongui Line Forest Park; Mangwon Station; Mangwon Market & Local Eats; Hapjeong Station; Hapjeong Food & Stay; Yeonnam Cafés; Hongik Univ. Station; Central Hongdae Shopping & Cafés; Red Road Busking & Nightlife; Sangsu Station; Sangsu Indie Cafés & Bars; Seogang Bridge; Han River; “Editorial map — not to scale”.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/hongdae-travel-guide.html` — 1 occurrence; `<figure id="hongdae-at-a-glance-map">`; localized alt; no figure caption; `src` only.
- JA: `ja/hongdae-travel-guide.html` — 1 occurrence; same context; localized alt; no figure caption; `src` only.
- Total occurrences: 2

### INF-022 — Jamsil orientation map

- Priority: P2
- Asset: `images/jamsil/jamsil-at-a-glance-map.webp`
- SHA-256: `8ab5f95de4abf41422b9b33f456f06b7a193d64fd2f6d1d40122a2d663555eb2`
- File / dimensions: WebP, 1536 × 1024
- Embedded English: “WEST”; “CENTER — MAIN CLUSTER”; “EAST”; “Jamsil Sports Complex”; “Sports Complex Station Lines 2 / 9”; “Jamsil Station Lines 2 / 8”; “Lotte World Adventure”; “Lotte World Tower / Mall”; “Lotte World Aquarium”; “Seokchon Lake”; “Songridan-gil”; “Seokchon Lake east side”; “Olympic Park”; “KSPO Dome”; “Olympic Park Station Lines 5 / 9”; “Orientation map — not to scale”.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/jamsil-travel-guide.html` — 1 occurrence; `<figure id="jamsil-at-a-glance-map">`; English alt and English caption remain; `src` only.
- JA: `ja/jamsil-travel-guide.html` — 1 occurrence; same context; localized alt, but caption retains “Editorial map — not to scale”; `src` only.
- Total occurrences: 2

### INF-023 — Seongsu at-a-glance map

- Priority: P2
- Asset: `images/seongsu/seongsu-at-a-glance-map.webp`
- SHA-256: `02810e427bb7f812b50338c70fb733e169fca444f713bb1b2c51bd60f79a8f04`
- File / dimensions: WebP, 1376 × 768
- Embedded English: “Seongsu at a Glance”; subtitle; WEST / NORTH / EAST; Seoul Forest Station; Ttukseom Station; Seongsu Station; Seoul Forest; Yeonmujang-gil; Cafés & Industrial Alleys; Pop-Ups & Flagships; Hands-On Beauty; Han River; “Editorial map — not to scale”.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/seongsu-travel-guide.html` — 1 occurrence; `<figure id="seongsu-at-a-glance-map">`; English alt; no caption; `src` only.
- JA: `ja/seongsu-travel-guide.html` — 1 occurrence; same context; localized alt; no caption; `src` only.
- Total occurrences: 2

### INF-024 — NAVER Map language guide

- Priority: P2
- Asset: `images/naver-map-language-guide.webp`
- SHA-256: `cfb535c19d1d67f8f8d512df3f24d001a32fcf1b436727ed5fb9a5e5fec280d1`
- File / dimensions: WebP, 1448 × 1086
- Embedded English: “How to Change NAVER Map to English”; subtitle; “Open the profile panel”; “Go to Language/언어”; “Select English and tap OK”; “Menu labels may vary slightly by app version and device.”; English app UI labels visible in all three screenshots.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/maps.html` — 1 occurrence; `<figure class="maps-guide-figure">`; localized alt; no caption; `src` only.
- JA: `ja/maps.html` — 1 occurrence; same context; localized alt; no caption; `src` only.
- Total occurrences: 2

### INF-025 — NAVER Map place-search guide

- Priority: P2
- Asset: `images/naver-map-place-search-guide.webp`
- SHA-256: `c46af1a31417440f3964113188c305c7ae166267a36f8af481eb230a04180abc`
- File / dimensions: WebP, 1448 × 1086
- Embedded English: “How to Search and Confirm the Right Place”; subtitle; “1. Type the English name”; “2. Compare similar results”; “Check before you choose”; “Match the line or place type”; “Compare the station name carefully”; “Use the Korean name or address if needed”; “Do not rely on one result only”; fallback search flow; “Always confirm the exact station or branch before you go.”; English app UI and example search results.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/maps.html` — 1 occurrence; maps guide figure; localized alt; no caption; `src` only.
- JA: `ja/maps.html` — 1 occurrence; same context; localized alt; no caption; `src` only.
- Total occurrences: 2

### INF-026 — NAVER Map route, exit, and bus guide

- Priority: P2
- Asset: `images/naver-map-route-exit-bus-guide.webp`
- SHA-256: `417d8f50875ce1a768d107901f6fd63318d34bdd13a445d91d69ae6a0026c3ab`
- File / dimensions: WebP, 1448 × 1086
- Embedded English: “How to Check Routes, Subway Exits and Bus Stops”; subtitle; “Search the exact place”; “Compare the route options”; “Open the route details”; exact-place comparison labels; “Best route”; “Fastest route”; “Another bus option”; “Fastest bus option”; “Best AREX option”; “Boarding point”; “Get off stop”; “Final walk”; “Correct place”; “Route type”; “Get-off stop”; “Final walk”; “Screens can vary by app version and device.”; English app UI examples.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/maps.html` — 1 occurrence; maps guide figure; localized alt; no caption; `src` only.
- JA: `ja/maps.html` — 1 occurrence; same context; localized alt; no caption; `src` only.
- Total occurrences: 2

### INF-027 — T-money buy, recharge, and use guide

- Priority: P2
- Asset: `images/tmoney/tmoney-buy-recharge-use.png`
- SHA-256: `c3f70b0c29d169512cb085eec1cc29fbcde78ee81055ce837eeb882b588b86d3`
- File / dimensions: PNG, 1536 × 1024
- Embedded English: “How to Buy & Recharge T-money in Korea”; intro; “TIP”; “1. How to Buy”; Convenience Stores / Subway Station Kiosk / Airway Counters and prices; “2. How to Recharge”; three steps; “3. How to Use”; Subway / Bus / Transfer Discount / Taxi; “Check Your Balance”; “Refund”; supporting instructions and warnings.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/tmoney.html` — 1 occurrence; `<figure class="tmoney-editorial-figure">`; localized alt; no caption; `src` only.
- JA: `ja/tmoney.html` — 1 occurrence; same context; localized alt; no caption; `src` only.
- Total occurrences: 2

### INF-028 — T-money labeled card visual

- Original audit priority: P3
- Final classification: EXCLUDE / BRAND-PRODUCT VISUAL — physical T-money card; retain the original in every language.
- Primary asset: `images/tmoney/tmoney-card.png`
- Byte-identical alias: `images/wowpass/tmoney-card.png`
- SHA-256: `331fa6938582d5428bfe50cff11cb806ed5bf70c5bf4a71efe486fb4cc8c744c`
- File / dimensions: PNG, 1536 × 1024
- Embedded English: “Tmoney”; “One Card All Pass”; “Subway”; “Bus”; “Taxi”. The duplicated file is one unique asset by SHA-256.
- ES localization needed: NO — original product visual retained; supersedes the initial YES recommendation.
- JA localization needed: NO — original product visual retained; supersedes the initial YES recommendation.
- ES: `es/tmoney.html` hero figure via `images/tmoney/tmoney-card.png`; `es/wowpass.html` balance visual via `images/wowpass/tmoney-card.png` — 2 occurrences; localized alt; WOWPASS occurrence has localized caption; `src` only.
- JA: corresponding `ja/tmoney.html` and `ja/wowpass.html` occurrences — 2; localized alt; WOWPASS occurrence has localized caption; `src` only.
- Total occurrences: 4

### INF-029 — T-money recharge machine SVG

- Priority: P2
- Asset: `images/tmoney/tmoney-recharge-machine.svg`
- SHA-256: `22d48c8f650ae4a66eaab2d5632036e0849e9404b54b66d6c14e940ffb335ab9`
- File / dimensions: SVG, 900 × 560
- Embedded English from SVG text nodes: “T-money Recharge Machine”; “Choose English • Place card • Add cash • Check balance”; “CARD AREA”; “CASH SLOT”; “Tip: If a machine does not accept foreign cards, use cash or recharge at a convenience store.”
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/tmoney.html` — 1 occurrence; T-money editorial figure; localized alt; no caption; `src` only.
- JA: `ja/tmoney.html` — 1 occurrence; same context; localized alt; no caption; `src` only.
- Total occurrences: 2

### INF-030 — WOWPASS labeled card visual

- Original audit priority: P3
- Final classification: EXCLUDE / BRAND-PRODUCT VISUAL — physical WOWPASS card; retain the original in every language.
- Asset: `images/wowpass/wowpass-card.png`
- SHA-256: `6db2ea31da8f41c63ec86a36a48f952c63a7f07685aa5d9c0407fab0295f105a`
- File / dimensions: PNG, 681 × 526
- Embedded English: “WOWPASS”; “PREPAID CARD”; “WOWPASS All-in-One Prepaid Card for Travelers”; “Transportation”; “Use on subway, bus, AREX and more”; “Payments”; “Pay at stores, cafés, and convenience stores”; “Currency Exchange”; “Exchange foreign currency and use in Korea”; embedded Tmoney label.
- ES localization needed: NO — original product visual retained; supersedes the initial YES recommendation.
- JA localization needed: NO — original product visual retained; supersedes the initial YES recommendation.
- ES: `es/wowpass.html` — 1 occurrence; `<figure class="wowpass-balance-visual">`; localized alt and caption; `src` only.
- JA: `ja/wowpass.html` — 1 occurrence; same context; localized alt and caption; `src` only.
- Total occurrences: 2

### INF-031 — WOWPASS composite guide

- Priority: P2
- Asset: `images/wowpass/wowpass-guide.png`
- SHA-256: `7f14f670bf1b23316f7108a710de40796f92cb1094cccfddda9ab72d3957fe60`
- File / dimensions: PNG, 1536 × 1024
- Embedded English: filename labels “wowpass-card.png”, “wowpass-machine.png”, and “wowpass-use-flow.png”; card, machine, and flow text; “WOWPASS All-in-One Prepaid Card for Travelers”; Transportation / Payments / Currency Exchange explanations; steps Get Card / Load Money or Exchange Currency / Pay / Use Transportation / Check Balance or Refund; tip about topping up, checking balance, and refunds.
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/wowpass.html` — 1 occurrence; wide WOWPASS editorial figure; localized alt and caption; `src` only.
- JA: `ja/wowpass.html` — 1 occurrence; same context; localized alt and caption; `src` only.
- Total occurrences: 2

### INF-032 — WOWPASS machine labeled visual

- Original audit priority: P3
- Final classification: EXCLUDE / BRAND-PRODUCT VISUAL — physical WOWPASS machine/product; retain the original in every language.
- Asset: `images/wowpass/wowpass-machine.png`
- SHA-256: `50fa17afb232dba7c14dc2dd9d49292677aec88e078c985251d5f24a5c49b873`
- File / dimensions: PNG, 792 × 526
- Embedded English: “WOWPASS”; “ALL-IN-ONE PREPAID CARD”; “TRANSPORTATION”; “PAYMENT”; “CURRENCY EXCHANGE”; “TOUCH TO START”; “WOWPASS CARD SALES & TOP-UP”; “RECEIPT”; “CARD”; “CASH (KRW)”; “WOWPASS All-in-One Prepaid Card for Travelers in Korea”.
- ES localization needed: NO — original product visual retained; supersedes the initial YES recommendation.
- JA localization needed: NO — original product visual retained; supersedes the initial YES recommendation.
- ES: `es/wowpass.html` — 1 occurrence; WOWPASS editorial figure; localized alt and caption; `src` only.
- JA: `ja/wowpass.html` — 1 occurrence; same context; localized alt and caption; `src` only.
- Total occurrences: 2

### INF-033 — WOWPASS use flow

- Priority: P2
- Asset: `images/wowpass/wowpass-use-flow.png`
- SHA-256: `8699eac1b04ea13b43d9b319a3e877629b4af85fa09eb7c70ca1e4ff952414a7`
- File / dimensions: PNG, 1505 × 395
- Embedded English: “Get Card”; “Get your WOWPASS card at airport machines or partner locations.”; “Load Money / Exchange Currency”; “Load Korean won (KRW) or exchange foreign currency onto your card.”; “Pay”; “Use your card to pay at stores, cafés and convenience stores.”; “Use Transportation”; “Tap your card on subway, bus, AREX and other transportation.”; “Check Balance / Refund”; “Check balance in the app or at machines and get a refund if needed.”; “TIP You can top up, check balance and get a refund at WOWPASS machines.”
- ES localization needed: YES
- JA localization needed: YES
- ES: `es/wowpass.html` — 1 occurrence; flow figure; localized alt and caption; `src` only.
- JA: `ja/wowpass.html` — 1 occurrence; same context; localized alt and caption; `src` only.
- Total occurrences: 2

## 4. Reused Asset Mapping

The original 33 unique source assets were reused across languages as mapped below. This historical source mapping includes the three subsequently excluded product visuals. The final implementation uses 60 localized references for 30 targets and keeps all eight excluded product-visual references on original files.

| Item | Primary asset | ES pages / occurrences | JA pages / occurrences | Total |
|---|---|---:|---:|---:|
| INF-001 | `couples-stay-seoul-area-guide.png` | 1 / 1 | 1 / 1 | 2 |
| INF-002 | `family-stay-seoul-area-guide.png` | 1 / 1 | 1 / 1 | 2 |
| INF-003 | `first-time-seoul-area-guide.png` | 1 / 1 | 1 / 1 | 2 |
| INF-004 | `luxury-stay-seoul-area-guide.png` | 1 / 1 | 1 / 1 | 2 |
| INF-005 | `nightlife-stay-seoul-area-guide.png` | 1 / 1 | 1 / 1 | 2 |
| INF-006 | `hongdae-vs-myeongdong.webp` | 1 / 1 | 1 / 1 | 2 |
| INF-007–009 | Three airport-bus guides | 1 page / 3 | 1 page / 3 | 6 |
| INF-010–015 | Six AREX visuals | 1 page / 6 | 1 page / 6 | 12 |
| INF-016 | `airport-arrival-hall-first-30-minutes-infographic.webp` | 1 / 1 | 1 / 1 | 2 |
| INF-017 | `terminal-arrival-maps.png` | 1 / 1 | 1 / 1 | 2 |
| INF-018 and INF-020 | Homepage eSIM and arrival visuals | 1 page / 2 | 1 page / 2 | 4 |
| INF-019 | `esim-korea-quick-decision.webp` | 1 / 1 | 1 / 1 | 2 |
| INF-021–023 | Three area maps | 3 pages / 3 | 3 pages / 3 | 6 |
| INF-024–026 | Three NAVER Map guides | 1 page / 3 | 1 page / 3 | 6 |
| INF-027 and INF-029 | T-money instructional visuals | 1 page / 2 | 1 page / 2 | 4 |
| INF-028 | Byte-identical T-money visual under two paths | 2 pages / 2 | 2 pages / 2 | 4 |
| INF-030–033 | Four WOWPASS visuals | 1 page / 4 | 1 page / 4 | 8 |

### Hash-based duplicate finding

`images/tmoney/tmoney-card.png` and `images/wowpass/tmoney-card.png` have the same SHA-256 (`331fa6938582d5428bfe50cff11cb806ed5bf70c5bf4a71efe486fb4cc8c744c`). They are one unique asset, four occurrences, and two repository paths. No duplicate primary inventory row was created.

## 5. Original Audit REVIEW_REQUIRED

None.

Every original candidate had enough directly visible or SVG-literal English for the inventory finding. OCR was not used as the sole basis for any finding. Small app-screen microcopy was treated as supporting evidence. Visible English alone does not override the final exclusion of physical brand/product visuals.

## 6. Original Audit Excluded Examples

These examples document the original exclusion boundary and were not included in the initial 33-item inventory. The final decision additionally excludes INF-028, INF-030, and INF-032 from localization; their inventory evidence is retained above.

| Asset | Reason excluded |
|---|---|
| `images/arrival/incheon-airport-arrival-route.webp` | Ordinary airport scene; English “Immigration” and “Baggage Claim” are natural environmental signage rather than Korea Inside infographic text. |
| `images/taste-korea/jeonju-bibimbap-guide.webp` | Ordinary food photograph with no embedded text. |
| `images/Accommodation/accommodation-hero-v1.webp` | Text-free accommodation hero photograph/illustration. |
| `images/esim/esim-hero-connected-seoul.webp` | Text-free editorial illustration using icons only. |

Logos, storefront brand names, signs naturally present in travel photographs, and source credits existing only in HTML were excluded throughout the audit.

## 7. Original Inventory-Only Quality and Protection Record

The following zero-change record applies only to the initial inventory audit, not to the later approved implementation. Final QA is recorded in section 10.

- ES pages scanned: ALL (58 / 58)
- JA pages scanned: ALL (58 / 58)
- Image references scanned: ALL (776 / 776 local reference records)
- Unique referenced assets deduplicated: PASS (239 paths → 235 SHA-256 assets)
- Confirmed English-text assets mapped: 33 / 33
- Duplicate primary rows: 0
- Unmapped confirmed assets: 0
- REVIEW_REQUIRED: 0
- Modified HTML: 0
- Modified images: 0
- Modified CSS / JavaScript: 0
- Stage: 0
- Commit: 0
- Push: 0
- Production deployment: 0
- Existing user files modified, deleted, restored, or staged: 0

## 8. Original Assumptions and Final Interpretation Notes

- In the original audit, “localization needed” meant the then-referenced asset exposed English pixels to users on that language edition; it was not production approval. Current completion/exclusion status is recorded above and in section 10.
- Proper nouns such as AREX, T-money, WOWPASS, NAVER Map, place names, station names, and route numbers are recorded as seen; this audit does not decide whether each proper noun should change in a localized asset.
- The original inventory included useful English labels beyond a brand logo, such as T-money card modes or WOWPASS machine controls. The user superseded that localization recommendation for INF-028 / INF-030 / INF-032: these are physical brand/product visuals, not translation targets.
- Responsive alternatives were scanned. None of the original 33 candidate assets had a candidate-specific `srcset`; their 68 rendered occurrences used `src` only. Final implementation changed only the approved 60 P1/P2 `src` values.
- The four excluded examples are representative boundary records, not an assertion that only four ordinary photos exist in the audited pages.

## 9. Original Inventory Completion Statement (Historical)

The statements below record the initial audit milestone only. They are retained as history; section 10 is the current implementation / QA status.

ES + JA Infographic English-Text Audit = COMPLETE

HTML Modified = 0

Images Modified = 0

Implementation = NOT STARTED

Commit / Push = NOT EXECUTED

Next = Localization Asset Production Review

## 10. Final Decision and Implementation / QA Record — 2026-09-27

### Approved scope and implementation

- Localization target = 30: P1 = 14 COMPLETE; P2 = 16 COMPLETE.
- ES localization = 30/30 COMPLETE; JA localization = 30/30 COMPLETE.
- EXCLUDE / BRAND-PRODUCT VISUAL = 3: INF-028, INF-030, INF-032.
- T-money / WOWPASS physical card and machine/product visuals remain original across languages. All eight ES/JA references use original files; no localized fake card/machine is referenced.
- Approved implementation: 60 localized assets and only 60 image `src` substitutions across 18 ES + 18 JA HTML pages. English source assets are preserved.
- Implementation checkpoints: Pilot 1 + Batch 2 `6f65a2ba389f6115b047ee93599e0b29da4dd092`; Batch 3 `ec758f714d04009fabd5323aca24be0aeb01edd7`; Batch 4 `12c5efa6d6132e3ee7acc39968193ebd06080650`.
- Integration checkpoint `2cf26a55861eb264e2103c29f9953b0632f3266c` includes Taiwan Production baseline `6d327d40c2758d6545b7f6bb0958dd4f8ee5a86a` without changing its protected files.

### Final local verification

| Check | Result |
|---|---|
| Localized existence / decode / original format / dimensions / aspect ratio | PASS — 60/60 files |
| Direct pixel inspection and SVG text inspection | PASS — 58 raster assets + 2 SVG assets |
| Target-language editorial text / no blank or wrong-language asset | PASS — ES 30/30; JA 30/30 |
| Original SHA-256 preservation | PASS — all 34 original paths, including the byte-identical T-money alias |
| HTML mapping | PASS — 60 correct localized references + 8 original product references |
| Local Chrome desktop + mobile | PASS — all 36 affected pages at 1280 × 900 and 390 × 844, 72 page/viewport checks |
| Image load / decode explicitly awaited; page and image HTTP | PASS — HTTP 200 |
| Missing / wrong-language / broken image references | 0 |
| Important text clipping / localized image horizontal overflow | 0 / 0 |
| Homepage cover crop | PASS — important text remains inside the visible safe area in both viewports and languages |
| WOWPASS aspect ratio | PASS — actual image content area excludes existing borders and padding |
| EN HTML / original images / Taiwan files / CSS / JS changes | 0 |
| Body / alt / caption / affiliate / tracking / canonical / hreflang / schema changes | 0 — HTML comparison permits only approved `src` substitutions |
| `git diff --check` and feature diff whitespace check | PASS |

Brand/product names, natural photographic signage, original app-screen UI, and pre-existing Korean labels are retained within the approved translation-only scope; they are not wrong-language editorial replacements.

### Known QA limitation

At the tested 1280 px desktop viewport, 14 affected ES pages have existing header/navigation/language-selector overflow (page width 1389 px, or 1425 px on the eSIM page). Overflowing elements are in the header; localized image boxes do not overflow. This is unrelated to the infographic assets and was not modified. All tested mobile pages and JA desktop pages fit the viewport.

### Current completion and release boundary

**30/30 LOCAL IMPLEMENTATION COMPLETE / FINAL LOCAL QA PASS / 3 BRAND-PRODUCT VISUALS EXCLUDED.**

This final audit changes only this document. Feature push, main merge/push, Vercel Production READY/SHA verification, and public QA follow under the user's separate explicit release authorization. They are pending at the document commit checkpoint; their actual outcome belongs in the final execution report. Existing unrelated user files, including the untracked Taiwan audit document, are excluded from this commit.
