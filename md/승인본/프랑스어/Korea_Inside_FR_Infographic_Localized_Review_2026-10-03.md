# Korea Inside — French Infographic Localized Review

**Date:** 2026-10-03  
**Status:** **LOCALIZATION REVIEW COMPLETE — AWAITING USER APPROVAL**  
**Language:** French / Français  
**Scope:** 30 LOCALIZE infographic assets / INF-001~033 excluding INF-028, INF-030, INF-032  
**Implementation:** NOT STARTED  
**Image production:** NOT STARTED  
**HTML changes:** 0  
**Git / Production:** 0  

---

## 1. Source basis

Stage 1 source manifest:

`Korea_Inside_FR_Infographic_Source_Manifest_2026-10-03.md`

Stage 1 manifest SHA-256: `8459e99a8d75644169d7cab351876bcb3a4adc9bbc977d7cfb0ea33df5562007`

English wording Source of Truth:

- `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` — INF-001~005
- `md/작업자료/Korea_Inside_TH_Infographic_Remaining_Source_Extraction_2026-10-02.md` — INF-006~033 excluding INF-028 / INF-030 / INF-032

Important:

- French wording was translated directly from the exact English source units.
- Thai / Spanish / Japanese wording was **not** used as a translation source.
- Existing ES/JA/TH assets are layout/production precedents only.
- Facts, numbers, route pairings, recommendation strength, brands and visual order remain protected.

---

## 2. Coverage and QA

| Measure | Result |
|---|---:|
| LOCALIZE assets | 30 |
| EXCLUDE assets | 3 |
| Exact source units | 1,456 |
| TRANSLATE units | **707 / 707** |
| RETAIN units | **749 / 749** |
| Missing French mappings | **0** |
| Empty French mappings | **0** |
| Duplicate unit IDs | **0** |
| Numeric token drift detected | **0** |
| Key brand/proper-noun token drift detected | **0** |
| Added facts | **0** |
| New recommendations | **0** |
| Image edits | **0** |
| HTML edits | **0** |

### French editorial rule

The French copy is written as concise, natural travel-guide French rather than literal English syntax. Compact labels are intentionally kept short where the source layout is dense. This does not authorize shortening or changing meaning during image production.

French typography uses normal French punctuation and accents. Brand/service names such as AREX, T-money, WOWPASS, NAVER Map, KTX, COEX and Korea Inside remain protected.

---

## 3. Production handling decisions for inherited REVIEW_REQUIRED items

These decisions are part of this Review Copy so Codex must not stop the whole batch for the already-known source ambiguities.

### INF-007
- Source typo `ARRINAL HALL` is **not reproduced** in French. Approved French: `HALL DES ARRIVÉES`.

### INF-008
- Ambiguous upper-left pickup micro-label: do not invent or normalize hidden letters; preserve the source visual where necessary.
- Source-visible `TA` remains source visual context; do not silently change it to T2.
- `9A` without a terminal code and blank cells remain as shown; do not invent missing terminal/platform values.
- Tiny hotel-sign lettering remains source visual; no fabricated French microtext.

### INF-012
- The clipped top-right Korea Inside wordmark must be rendered as the complete `Korea Inside` wordmark inside the canvas if the localized production rebuild exposes the editorial brand layer. Do not clip the final French asset.

### INF-013
- Ticket-machine screens and distant platform-sign microtext are source-photo/UI context. Preserve those pixels; do not reconstruct unreadable text.
- Main French editorial text must be fully readable with no clipping, especially the repeated lower T2 steps.

### INF-016
- Tiny background gate/kiosk lettering is preserved as source visual context; do not invent French microtext.

### INF-017
- Uncertain tiny badge candidates `23` and `17-` are not promoted to French public copy. Preserve source pixels/geometry unless a verified master proves the text.

### INF-020
- Blurred card/kiosk UI microtext is preserved; do not reconstruct it.

### INF-022
- Tiny mall façade wordmark is preserved as visual context; no invented letters.

### INF-025
- Clipped/occluded NAVER results and map-name suffixes remain clipped/occluded. Do not expand hidden place or branch names.

### INF-026
- Ellipsized/cropped NAVER UI stays truncated. Do not restore hidden trailing names.

### INF-027
- Tiny kiosk/tap-reader/sticker microtext remains source visual.
- Candidate `Airbway Counters` is not French public copy and must not be silently corrected or translated without verified source text.

### INF-029
- Keep the 900×560 SVG canvas. The approved French bottom tip may be wrapped/reflowed inside the canvas; no clipping or overflow.

### INF-031
- Preserve tiny machine-screen/product replicas and photographed brand UI. Translate only the approved readable editorial units below.

### INF-033
- Preserve miniature machine/card microtext. Do not infer hidden text from INF-031.

---

## 4. French exact mapping

Rule:

- `TRANSLATE` → use the French target below exactly.
- `RETAIN` → preserve the English/source token exactly as printed in the source extraction.
- Unit order and asset order must remain unchanged.

### INF-001

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-001-U001` | TRANSLATE | Where Should Couples Stay in Seoul? | Où loger à Séoul en couple ? |
| `INF-001-U002` | TRANSLATE | A quick area guide for different couple travel styles | Guide rapide des quartiers selon votre style de voyage à deux |
| `INF-001-U003` | TRANSLATE | Start with the atmosphere you want, then check transport, nighttime noise and the final hotel route. | Commencez par l’ambiance recherchée, puis vérifiez les transports, le bruit la nuit et le trajet final jusqu’à l’hôtel. |
| `INF-001-U004` | TRANSLATE | QUICK MATCH | CHOIX RAPIDE |
| `INF-001-U005` | TRANSLATE | Nightlife and cafés | Vie nocturne et cafés |
| `INF-001-U006` | RETAIN | Hongdae | Hongdae |
| `INF-001-U007` | TRANSLATE | Design shops and local cafés | Boutiques design et cafés locaux |
| `INF-001-U008` | RETAIN | Seongsu | Seongsu |
| `INF-001-U009` | TRANSLATE | Traditional streets and quiet evenings | Ruelles traditionnelles et soirées calmes |
| `INF-001-U010` | RETAIN | Insadong | Insadong |
| `INF-001-U011` | TRANSLATE | Central sightseeing and shopping | Visites centrales et shopping |
| `INF-001-U012` | RETAIN | Myeongdong | Myeongdong |
| `INF-001-U013` | TRANSLATE | Premium shopping and southern Seoul | Shopping haut de gamme et sud de Séoul |
| `INF-001-U014` | RETAIN | Gangnam | Gangnam |
| `INF-001-U015` | TRANSLATE | Lotte World and evening lake walks | Lotte World et promenades au bord du lac le soir |
| `INF-001-U016` | RETAIN | Jamsil | Jamsil |
| `INF-001-U017` | TRANSLATE | TOP PICKS | SÉLECTION |
| `INF-001-U018` | RETAIN | Hongdae | Hongdae |
| `INF-001-U019` | TRANSLATE | Best for nightlife and cafés | Idéal pour la vie nocturne et les cafés |
| `INF-001-U020` | TRANSLATE | Watch out: Busy streets and late-night noise | À savoir : rues animées et bruit tard le soir |
| `INF-001-U021` | RETAIN | Seongsu | Seongsu |
| `INF-001-U022` | TRANSLATE | Best for design cafés and daytime walks | Idéal pour les cafés design et les balades en journée |
| `INF-001-U023` | TRANSLATE | Watch out: Airport access is less direct | À savoir : accès à l’aéroport moins direct |
| `INF-001-U024` | RETAIN | Insadong | Insadong |
| `INF-001-U025` | TRANSLATE | Best for culture and quieter evenings | Idéal pour la culture et des soirées plus calmes |
| `INF-001-U026` | TRANSLATE | Watch out: Check elevator and taxi access | À savoir : vérifiez l’accès par ascenseur et en taxi |

### INF-002

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-002-U001` | TRANSLATE | Where Should Your Family Stay in Seoul? | Où loger à Séoul en famille ? |
| `INF-002-U002` | TRANSLATE | A quick area guide for families | Guide rapide des quartiers pour les familles |
| `INF-002-U003` | TRANSLATE | Start with your main family priority, then choose the Seoul area that makes the trip easier. | Commencez par votre priorité en famille, puis choisissez le quartier de Séoul qui simplifie le séjour. |
| `INF-002-U004` | TRANSLATE | QUICK MATCH | CHOIX RAPIDE |
| `INF-002-U005` | TRANSLATE | First family trip | Premier voyage en famille |
| `INF-002-U006` | RETAIN | Myeongdong | Myeongdong |
| `INF-002-U007` | TRANSLATE | Best all-round base for central sightseeing, meals and easy planning. | Base polyvalente pour les visites centrales, les repas et une organisation simple. |
| `INF-002-U008` | TRANSLATE | Lotte World and family attractions | Lotte World et attractions familiales |
| `INF-002-U009` | RETAIN | Jamsil | Jamsil |
| `INF-002-U010` | TRANSLATE | Best for family attractions, modern hotels and indoor activities. | Idéal pour les attractions familiales, les hôtels modernes et les activités en intérieur. |
| `INF-002-U011` | TRANSLATE | Airport access and large luggage | Accès à l’aéroport et gros bagages |
| `INF-002-U012` | RETAIN | Mapo / Gongdeok | Mapo / Gongdeok |
| `INF-002-U013` | TRANSLATE | Best for practical transfers, easier luggage handling and quieter nights. | Idéal pour des correspondances pratiques, des bagages plus faciles à gérer et des nuits plus calmes. |
| `INF-002-U014` | TRANSLATE | Traditional culture and calmer evenings | Culture traditionnelle et soirées plus calmes |
| `INF-002-U015` | RETAIN | Insadong | Insadong |
| `INF-002-U016` | TRANSLATE | Best for palaces, traditional streets and a quieter cultural base. | Idéal pour les palais, les rues traditionnelles et un séjour culturel plus paisible. |
| `INF-002-U017` | TRANSLATE | One-night transit or KTX connection | Transit d’une nuit ou correspondance KTX |
| `INF-002-U018` | RETAIN | Seoul Station | Seoul Station |
| `INF-002-U019` | TRANSLATE | Best for airport rail, train connections and short stopovers. | Idéal pour le train de l’aéroport, les correspondances ferroviaires et les courts séjours. |
| `INF-002-U020` | TRANSLATE | Teenagers, cafés and nightlife | Ados, cafés et vie nocturne |
| `INF-002-U021` | RETAIN | Hongdae | Hongdae |
| `INF-002-U022` | TRANSLATE | Best for older kids or teens who enjoy cafés, music and a lively atmosphere. | Idéal avec de grands enfants ou des ados qui aiment les cafés, la musique et une ambiance animée. |
| `INF-002-U023` | TRANSLATE | TOP PICKS | SÉLECTION |
| `INF-002-U024` | TRANSLATE | Top 3 family-friendly Seoul bases | Top 3 des quartiers pour les familles à Séoul |
| `INF-002-U025` | RETAIN | Myeongdong | Myeongdong |
| `INF-002-U026` | TRANSLATE | Best first family base | Meilleure base pour un premier voyage en famille |
| `INF-002-U027` | TRANSLATE | Watch out: Busy streets and some smaller rooms | À savoir : rues animées et certaines chambres plus petites |
| `INF-002-U028` | RETAIN | Jamsil | Jamsil |
| `INF-002-U029` | TRANSLATE | Best for attractions | Meilleur choix pour les attractions |
| `INF-002-U030` | TRANSLATE | Watch out: Longer rides to northwest Seoul | À savoir : trajets plus longs vers le nord-ouest de Séoul |
| `INF-002-U031` | RETAIN | Mapo / Gongdeok | Mapo / Gongdeok |
| `INF-002-U032` | TRANSLATE | Best for airport and luggage | Meilleur choix pour l’aéroport et les bagages |
| `INF-002-U033` | TRANSLATE | Watch out: Check the exact station exit | À savoir : vérifiez la sortie exacte de la station |

### INF-003

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-003-U001` | TRANSLATE | Where Should First-Time Visitors Stay in Seoul? | Où loger à Séoul pour un premier voyage ? |
| `INF-003-U002` | TRANSLATE | A quick area guide for your first trip | Guide rapide des quartiers pour une première visite |
| `INF-003-U003` | TRANSLATE | QUICK MATCH | CHOIX RAPIDE |
| `INF-003-U004` | TRANSLATE | Central sightseeing and easy planning | Visites centrales et organisation facile |
| `INF-003-U005` | RETAIN | Myeongdong | Myeongdong |
| `INF-003-U006` | TRANSLATE | Cafés, nightlife and direct AREX | Cafés, vie nocturne et AREX direct |
| `INF-003-U007` | RETAIN | Hongdae | Hongdae |
| `INF-003-U008` | TRANSLATE | Airport rail, KTX and large luggage | Train de l’aéroport, KTX et gros bagages |
| `INF-003-U009` | RETAIN | Seoul Station | Seoul Station |
| `INF-003-U010` | TRANSLATE | Airport access and quieter nights | Accès à l’aéroport et nuits plus calmes |
| `INF-003-U011` | RETAIN | Mapo / Gongdeok | Mapo / Gongdeok |
| `INF-003-U012` | TRANSLATE | Palaces and traditional streets | Palais et rues traditionnelles |
| `INF-003-U013` | RETAIN | Insadong | Insadong |
| `INF-003-U014` | TRANSLATE | Lotte World and southern Seoul | Lotte World et sud de Séoul |
| `INF-003-U015` | RETAIN | Jamsil | Jamsil |
| `INF-003-U016` | TRANSLATE | TOP PICKS | SÉLECTION |
| `INF-003-U017` | TRANSLATE | SHOP | SHOPPING |
| `INF-003-U018` | RETAIN | Myeongdong | Myeongdong |
| `INF-003-U019` | TRANSLATE | Best all-round first base | Meilleur choix polyvalent pour une première visite |
| `INF-003-U020` | TRANSLATE | Watch out: Busy streets and some smaller rooms | À savoir : rues animées et certaines chambres plus petites |
| `INF-003-U021` | RETAIN | Hongdae | Hongdae |
| `INF-003-U022` | TRANSLATE | Best for cafés and nightlife | Meilleur choix pour les cafés et la vie nocturne |
| `INF-003-U023` | TRANSLATE | COFFEE | CAFÉ |
| `INF-003-U024` | TRANSLATE | Watch out: Check distance from the busiest streets | À savoir : vérifiez la distance depuis les rues les plus animées |
| `INF-003-U025` | RETAIN | Seoul Station | Seoul Station |
| `INF-003-U026` | TRANSLATE | Best for airport rail and luggage | Meilleur choix pour le train de l’aéroport et les bagages |
| `INF-003-U027` | RETAIN | SEOUL STATION | SEOUL STATION |
| `INF-003-U028` | TRANSLATE | Watch out: Verify the exact exit and walking route | À savoir : vérifiez la sortie exacte et l’itinéraire à pied |

### INF-004

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-004-U001` | TRANSLATE | Where Should Luxury Travelers Stay in Seoul? | Où loger à Séoul pour un séjour haut de gamme ? |
| `INF-004-U002` | TRANSLATE | A quick area guide for premium stays | Guide rapide des quartiers pour un séjour premium |
| `INF-004-U003` | TRANSLATE | QUICK MATCH | CHOIX RAPIDE |
| `INF-004-U004` | TRANSLATE | Luxury shopping and fine dining | Shopping de luxe et gastronomie |
| `INF-004-U005` | RETAIN | Gangnam | Gangnam |
| `INF-004-U006` | TRANSLATE | Lotte World and modern comfort | Lotte World et confort moderne |
| `INF-004-U007` | RETAIN | Jamsil | Jamsil |
| `INF-004-U008` | TRANSLATE | First trip and central sightseeing | Premier voyage et visites centrales |
| `INF-004-U009` | RETAIN | Myeongdong | Myeongdong |
| `INF-004-U010` | TRANSLATE | Airport rail and large luggage | Train de l’aéroport et gros bagages |
| `INF-004-U011` | RETAIN | Seoul Station / Namdaemun | Seoul Station / Namdaemun |
| `INF-004-U012` | TRANSLATE | Palaces and quiet cultural stays | Palais et séjours culturels au calme |
| `INF-004-U013` | RETAIN | Insadong | Insadong |
| `INF-004-U014` | TRANSLATE | International dining and nightlife | Cuisine internationale et vie nocturne |
| `INF-004-U015` | RETAIN | Itaewon | Itaewon |
| `INF-004-U016` | TRANSLATE | TOP PICKS | SÉLECTION |
| `INF-004-U017` | RETAIN | Gangnam | Gangnam |
| `INF-004-U018` | TRANSLATE | DEPARTMENT STORE | GRAND MAGASIN |
| `INF-004-U019` | RETAIN | G | G |
| `INF-004-U020` | TRANSLATE | Best for shopping, dining and business | Idéal pour le shopping, les restaurants et les affaires |
| `INF-004-U021` | TRANSLATE | Watch out: Longer routes to palaces and airport rail | À savoir : trajets plus longs vers les palais et le train de l’aéroport |
| `INF-004-U022` | RETAIN | Jamsil | Jamsil |
| `INF-004-U023` | TRANSLATE | Best for modern comfort and attractions | Idéal pour le confort moderne et les attractions |
| `INF-004-U024` | TRANSLATE | Watch out: Farther from central historic sights | À savoir : plus loin des sites historiques du centre |
| `INF-004-U025` | RETAIN | Myeongdong | Myeongdong |
| `INF-004-U026` | RETAIN | Myeongdong | Myeongdong |
| `INF-004-U027` | TRANSLATE | Best for first-time central convenience | Idéal pour la praticité centrale lors d’un premier séjour |
| `INF-004-U028` | TRANSLATE | DUTY FREE | DUTY FREE |
| `INF-004-U029` | TRANSLATE | Watch out: Busy streets and less privacy | À savoir : rues animées et moins d’intimité |

### INF-005

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-005-U001` | TRANSLATE | Where Should Nightlife Travelers Stay in Seoul? | Où loger à Séoul pour profiter de la vie nocturne ? |
| `INF-005-U002` | TRANSLATE | A quick area guide for nights out and easier returns | Guide rapide des quartiers pour sortir le soir et rentrer plus facilement |
| `INF-005-U003` | TRANSLATE | 1. Clubs, live music and youthful energy | 1. Clubs, musique live et énergie jeune |
| `INF-005-U004` | RETAIN | Hongdae | Hongdae |
| `INF-005-U005` | TRANSLATE | 2. International bars and social nights | 2. Bars internationaux et soirées conviviales |
| `INF-005-U006` | RETAIN | Itaewon | Itaewon |
| `INF-005-U007` | TRANSLATE | 3. Upscale clubs and late dinners | 3. Clubs haut de gamme et dîners tardifs |
| `INF-005-U008` | RETAIN | Gangnam | Gangnam |
| `INF-005-U009` | TRANSLATE | 4. Central sightseeing with occasional nights out | 4. Visites centrales avec quelques sorties le soir |
| `INF-005-U010` | RETAIN | Myeongdong | Myeongdong |
| `INF-005-U011` | TRANSLATE | 5. Quieter sleep with easy Hongdae access | 5. Nuits plus calmes avec accès facile à Hongdae |
| `INF-005-U012` | RETAIN | Mapo / Gongdeok | Mapo / Gongdeok |
| `INF-005-U013` | TRANSLATE | 6. Airport rail and early departures | 6. Train de l’aéroport et départs tôt |
| `INF-005-U014` | RETAIN | Seoul Station | Seoul Station |

### INF-006

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-006-U001` | RETAIN | Hongdae | Hongdae |
| `INF-006-U002` | TRANSLATE | vs | vs |
| `INF-006-U003` | RETAIN | Myeongdong | Myeongdong |
| `INF-006-U004` | TRANSLATE | Cafés • Creative Culture • Youthful Energy | Cafés • Culture créative • Énergie jeune |
| `INF-006-U005` | TRANSLATE | Shopping • Central Location • Easy Access | Shopping • Emplacement central • Accès facile |

### INF-007

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-007-U001` | RETAIN | Korea Inside | Korea Inside |
| `INF-007-U002` | TRANSLATE | Airport Bus Boarding Location Guide | Guide des zones d’embarquement des bus de l’aéroport |
| `INF-007-U003` | TRANSLATE | INCHEON AIRPORT BUS NUMBER & BOARDING CHECK | VÉRIFIEZ LE NUMÉRO DU BUS ET LE QUAI À L’AÉROPORT D’INCHEON |
| `INF-007-U004` | TRANSLATE | START HERE | COMMENCEZ ICI |
| `INF-007-U005` | TRANSLATE | Check your bus number first. Match the route number and destination on the airport sign before you go to the boarding area. | Vérifiez d’abord le numéro de votre bus. Faites correspondre le numéro de ligne et la destination indiqués sur le panneau de l’aéroport avant de rejoindre la zone d’embarquement. |
| `INF-007-U006` | RETAIN | TERMINAL 1 | TERMINAL 1 |
| `INF-007-U007` | TRANSLATE | ARRINAL HALL | HALL DES ARRIVÉES |
| `INF-007-U008` | TRANSLATE | Follow airport bus signs after arrival. Go to the official bus stop area and check the route number on the platform sign. | Après l’arrivée, suivez les panneaux des bus de l’aéroport. Rejoignez la zone officielle des arrêts et vérifiez le numéro de ligne sur le panneau du quai. |
| `INF-007-U009` | RETAIN | TERMINAL 2 | TERMINAL 2 |
| `INF-007-U010` | TRANSLATE | TRANSPORTATION CENTER | CENTRE DE TRANSPORT |
| `INF-007-U011` | TRANSLATE | Use the official airport bus signs to reach the boarding area. Confirm the bus number and destination again before boarding. | Suivez la signalétique officielle des bus de l’aéroport jusqu’à la zone d’embarquement. Vérifiez de nouveau le numéro du bus et la destination avant de monter. |
| `INF-007-U012` | TRANSLATE | STEP 1 — FIND THE NUMBER | ÉTAPE 1 — TROUVEZ LE NUMÉRO |
| `INF-007-U013` | TRANSLATE | Look for your route number, such as 6000-series, 6700-series, late-night buses, or regional routes. | Repérez votre numéro de ligne : séries 6000, 6700, bus de nuit ou lignes régionales, par exemple. |
| `INF-007-U014` | RETAIN | 6001 | 6001 |
| `INF-007-U015` | RETAIN | 6002 | 6002 |
| `INF-007-U016` | RETAIN | 6701 | 6701 |
| `INF-007-U017` | RETAIN | N6001 | N6001 |
| `INF-007-U018` | TRANSLATE | Examples only | Exemples uniquement |
| `INF-007-U019` | TRANSLATE | STEP 2 — MATCH THE DESTINATION | ÉTAPE 2 — VÉRIFIEZ LA DESTINATION |
| `INF-007-U020` | TRANSLATE | Some bus numbers are similar. Always match both the route number and the destination name shown on the airport sign. | Certains numéros de bus se ressemblent. Vérifiez toujours à la fois le numéro de ligne et le nom de la destination affichés sur le panneau de l’aéroport. |
| `INF-007-U021` | RETAIN | 6001 | 6001 |
| `INF-007-U022` | RETAIN | Seoul Station | Seoul Station |
| `INF-007-U023` | RETAIN | 6701 | 6701 |
| `INF-007-U024` | RETAIN | Gangnam | Gangnam |
| `INF-007-U025` | TRANSLATE | STEP 3 — CHECK THE PLATFORM | ÉTAPE 3 — VÉRIFIEZ LE QUAI |
| `INF-007-U026` | TRANSLATE | At the boarding area, read the platform sign again. The platform may serve several routes, so confirm your bus before you wait. | Dans la zone d’embarquement, relisez le panneau du quai. Plusieurs lignes peuvent utiliser le même quai : confirmez votre bus avant d’attendre. |
| `INF-007-U027` | TRANSLATE | STEP 4 — BOARD SAFELY | ÉTAPE 4 — MONTEZ EN TOUTE SÉCURITÉ |
| `INF-007-U028` | TRANSLATE | When the bus arrives, check the front display and ask the staff or driver if you are unsure. | À l’arrivée du bus, vérifiez l’affichage à l’avant et demandez au personnel ou au chauffeur en cas de doute. |
| `INF-007-U029` | TRANSLATE | COMMON ROUTE TYPES | TYPES DE LIGNES COURANTS |
| `INF-007-U030` | TRANSLATE | • 6000-series: many Seoul hotel and district routes | • Série 6000 : nombreuses lignes vers les hôtels et quartiers de Séoul |
| `INF-007-U031` | TRANSLATE | • 6700-series: premium limousine routes | • Série 6700 : lignes de limousine premium |
| `INF-007-U032` | TRANSLATE | • Late-night buses: limited hours | • Bus de nuit : horaires limités |
| `INF-007-U033` | TRANSLATE | • Regional buses: cities outside central Seoul | • Bus régionaux : villes hors du centre de Séoul |
| `INF-007-U034` | TRANSLATE | These are route families, not exact boarding assignments. Please check the airport sign. | Il s’agit de familles de lignes, pas de quais fixes. Vérifiez le panneau de l’aéroport. |
| `INF-007-U035` | TRANSLATE | IMPORTANT NOTE | IMPORTANT |
| `INF-007-U036` | TRANSLATE | Boarding locations can change. Always confirm the latest route number, destination, and platform on the official airport sign on the day of travel. | Les zones d’embarquement peuvent changer. Le jour du trajet, vérifiez toujours le numéro de ligne, la destination et le quai les plus récents sur la signalétique officielle de l’aéroport. |
| `INF-007-U037` | RETAIN | Korea Inside | Korea Inside |

### INF-008

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-008-U001` | RETAIN | Korea Inside | Korea Inside |
| `INF-008-U002` | TRANSLATE | Terminal 2 Ground Transportation Guide | Guide des transports terrestres du Terminal 2 |
| `INF-008-U003` | TRANSLATE | INCHEON AIRPORT TERMINAL 2 ARRIVAL HALL (1F) | HALL DES ARRIVÉES DU TERMINAL 2 DE L’AÉROPORT D’INCHEON (1F) |
| `INF-008-U004` | TRANSLATE | START HERE | COMMENCEZ ICI |
| `INF-008-U005` | TRANSLATE | • Exit customs on the Arrivals Hall (1F) | • Sortez de la douane vers le hall des arrivées (1F) |
| `INF-008-U006` | TRANSLATE | • Follow the “GROUND TRANSPORTATION” signs | • Suivez les panneaux « GROUND TRANSPORTATION » |
| `INF-008-U007` | TRANSLATE | • Choose your ride | • Choisissez votre moyen de transport |
| `INF-008-U008` | TRANSLATE | • Pay at the counter or kiosk | • Payez au comptoir ou à la borne |
| `INF-008-U009` | TRANSLATE | • Enjoy a safe trip! | • Bon trajet ! |
| `INF-008-U010` | TRANSLATE | TERMINAL 2 ARRIVAL HALL (1F) | HALL DES ARRIVÉES DU TERMINAL 2 (1F) |
| `INF-008-U011` | TRANSLATE | GATE | PORTE |
| `INF-008-U012` | TRANSLATE | GATE | PORTE |
| `INF-008-U013` | TRANSLATE | ARRIVAL HALL | HALL DES ARRIVÉES |
| `INF-008-U014` | RETAIN | 4 | 4 |
| `INF-008-U015` | TRANSLATE | Bus Stop | Arrêt de bus |
| `INF-008-U016` | TRANSLATE | TAXI | TAXI |
| `INF-008-U017` | TRANSLATE | Taxi Stand | Station de taxis |
| `INF-008-U018` | TRANSLATE | Pick-up Point | Point de prise en charge |
| `INF-008-U019` | TRANSLATE | Cross the zebra crossing to the outer curb (1F) | Traversez le passage piéton jusqu’au trottoir extérieur (1F) |
| `INF-008-U020` | RETAIN | 1 | 1 |
| `INF-008-U021` | TRANSLATE | AIRPORT BUS | BUS DE L’AÉROPORT |
| `INF-008-U022` | TRANSLATE | Take the airport limousine bus to major cities. | Prenez le bus limousine de l’aéroport vers les principales villes. |
| `INF-008-U023` | TRANSLATE | Ticket: ₩ 17,000 ~ 18,000 (varies by destination) | Billet : ₩ 17,000 ~ 18,000 (selon la destination) |
| `INF-008-U024` | TRANSLATE | Buy at the ticket counter or kiosk. | Achetez-le au guichet ou à la borne. |
| `INF-008-U025` | TRANSLATE | Bus Stop 6 ~ 11 | Arrêts de bus 6 ~ 11 |
| `INF-008-U026` | RETAIN | 2 | 2 |
| `INF-008-U027` | TRANSLATE | TAXI | TAXI |
| `INF-008-U028` | TRANSLATE | Take a taxi to your destination. | Prenez un taxi jusqu’à votre destination. |
| `INF-008-U029` | TRANSLATE | Basic Fare: ₩ 4,800 (metered) | Tarif de base : ₩ 4,800 (au compteur) |
| `INF-008-U030` | TRANSLATE | Taxi Stand 4D ~ 6D | Station de taxis 4D ~ 6D |
| `INF-008-U031` | RETAIN | 3 | 3 |
| `INF-008-U032` | TRANSLATE | AREX / TRAIN | AREX / TRAIN |
| `INF-008-U033` | TRANSLATE | Take the AREX Express or All Stop Train to Seoul Station. | Prenez l’AREX Express ou l’All-Stop Train jusqu’à Seoul Station. |
| `INF-008-U034` | TRANSLATE | Express: 43 min | Express : 43 min |
| `INF-008-U035` | TRANSLATE | All Stop: 56 min | All-Stop : 56 min |
| `INF-008-U036` | TRANSLATE | B1F (Follow the signs) | B1F (suivez les panneaux) |
| `INF-008-U037` | RETAIN | 4 | 4 |
| `INF-008-U038` | TRANSLATE | PICK-UP | PRISE EN CHARGE |
| `INF-008-U039` | TRANSLATE | Meet your driver in the designated pick-up area. | Retrouvez votre chauffeur dans la zone de prise en charge indiquée. |
| `INF-008-U040` | TRANSLATE | Pick-up Point 4C, 5C | Points de prise en charge 4C, 5C |
| `INF-008-U041` | TRANSLATE | Traffic may vary. Allow extra time for your journey. | La circulation peut varier. Prévoyez du temps supplémentaire pour le trajet. |
| `INF-008-U042` | RETAIN | Korea Inside | Korea Inside |
| `INF-008-U043` | RETAIN | Korea Inside | Korea Inside |
| `INF-008-U044` | RETAIN | T1·T2 | T1·T2 |
| `INF-008-U045` | RETAIN | 1 (1F) | 1 (1F) |
| `INF-008-U046` | RETAIN | 6 | 6 |
| `INF-008-U047` | RETAIN | 7 | 7 |
| `INF-008-U048` | RETAIN | 8 | 8 |
| `INF-008-U049` | RETAIN | 9 | 9 |
| `INF-008-U050` | RETAIN | 10 | 10 |
| `INF-008-U051` | RETAIN | 11 | 11 |
| `INF-008-U052` | RETAIN | 6001 | 6001 |
| `INF-008-U053` | RETAIN | 8A-1 | 8A-1 |
| `INF-008-U054` | RETAIN | 6002 | 6002 |
| `INF-008-U055` | RETAIN | 8A-2 | 8A-2 |
| `INF-008-U056` | RETAIN | 6003 | 6003 |
| `INF-008-U057` | RETAIN | 8B-1 | 8B-1 |
| `INF-008-U058` | RETAIN | 6004 | 6004 |
| `INF-008-U059` | RETAIN | 9A | 9A |
| `INF-008-U060` | RETAIN | 6701 | 6701 |
| `INF-008-U061` | RETAIN | 10B | 10B |
| `INF-008-U062` | RETAIN | 6702 | 6702 |
| `INF-008-U063` | RETAIN | 10A | 10A |
| `INF-008-U064` | RETAIN | 6703 | 6703 |
| `INF-008-U065` | RETAIN | 11B | 11B |
| `INF-008-U066` | RETAIN | 6705 | 6705 |
| `INF-008-U067` | RETAIN | 6A | 6A |
| `INF-008-U068` | RETAIN | 2 (1F) | 2 (1F) |
| `INF-008-U069` | RETAIN | 6 | 6 |
| `INF-008-U070` | RETAIN | 7 | 7 |
| `INF-008-U071` | RETAIN | 8 | 8 |
| `INF-008-U072` | RETAIN | 9 | 9 |
| `INF-008-U073` | RETAIN | 10 | 10 |
| `INF-008-U074` | RETAIN | 11 | 11 |
| `INF-008-U075` | RETAIN | 6001 | 6001 |
| `INF-008-U076` | RETAIN | 6A-1 | 6A-1 |
| `INF-008-U077` | RETAIN | 6002 | 6002 |
| `INF-008-U078` | RETAIN | 6A-2 | 6A-2 |
| `INF-008-U079` | RETAIN | 6003 | 6003 |
| `INF-008-U080` | RETAIN | 7A | 7A |
| `INF-008-U081` | RETAIN | 6004 | 6004 |
| `INF-008-U082` | RETAIN | 8A | 8A |
| `INF-008-U083` | RETAIN | 6701 | 6701 |
| `INF-008-U084` | RETAIN | 9B | 9B |
| `INF-008-U085` | RETAIN | 6702 | 6702 |
| `INF-008-U086` | RETAIN | 10A | 10A |
| `INF-008-U087` | RETAIN | 6703 | 6703 |
| `INF-008-U088` | RETAIN | 11A | 11A |
| `INF-008-U089` | RETAIN | 6705 | 6705 |
| `INF-008-U090` | RETAIN | 6B | 6B |
| `INF-008-U091` | RETAIN | Korea Inside | Korea Inside |
| `INF-008-U092` | RETAIN | Korea Inside | Korea Inside |
| `INF-008-U093` | RETAIN | 6004 | 6004 |
| `INF-008-U094` | RETAIN | T1 | T1 |
| `INF-008-U095` | RETAIN | 9A | 9A |
| `INF-008-U096` | RETAIN | 9A | 9A |
| `INF-008-U097` | RETAIN | 6001 | 6001 |
| `INF-008-U098` | RETAIN | T1 | T1 |
| `INF-008-U099` | RETAIN | 8A-1 | 8A-1 |
| `INF-008-U100` | RETAIN | T2 | T2 |
| `INF-008-U101` | RETAIN | 6002 | 6002 |
| `INF-008-U102` | RETAIN | T1 | T1 |
| `INF-008-U103` | RETAIN | 8A-2 | 8A-2 |
| `INF-008-U104` | RETAIN | T2 | T2 |
| `INF-008-U105` | RETAIN | 6003 | 6003 |
| `INF-008-U106` | RETAIN | T1 | T1 |
| `INF-008-U107` | RETAIN | 8B-1 | 8B-1 |
| `INF-008-U108` | RETAIN | TA | TA |
| `INF-008-U109` | RETAIN | 6702 | 6702 |
| `INF-008-U110` | RETAIN | T1 | T1 |
| `INF-008-U111` | RETAIN | 10A | 10A |
| `INF-008-U112` | RETAIN | T2 | T2 |
| `INF-008-U113` | RETAIN | 10A | 10A |
| `INF-008-U114` | RETAIN | 6701 | 6701 |
| `INF-008-U115` | RETAIN | T1 | T1 |
| `INF-008-U116` | RETAIN | 10B | 10B |
| `INF-008-U117` | RETAIN | T2 | T2 |
| `INF-008-U118` | RETAIN | 9B | 9B |
| `INF-008-U119` | RETAIN | 6702 | 6702 |
| `INF-008-U120` | RETAIN | T1 | T1 |
| `INF-008-U121` | RETAIN | 10A | 10A |
| `INF-008-U122` | RETAIN | T2 | T2 |
| `INF-008-U123` | RETAIN | 10A | 10A |
| `INF-008-U124` | RETAIN | 6703 | 6703 |
| `INF-008-U125` | RETAIN | T1 | T1 |
| `INF-008-U126` | RETAIN | 11B | 11B |
| `INF-008-U127` | RETAIN | T2 | T2 |
| `INF-008-U128` | RETAIN | 11A | 11A |
| `INF-008-U129` | RETAIN | 6705 | 6705 |
| `INF-008-U130` | RETAIN | T1 | T1 |
| `INF-008-U131` | RETAIN | 6A | 6A |
| `INF-008-U132` | RETAIN | T2 | T2 |
| `INF-008-U133` | RETAIN | 6B | 6B |
| `INF-008-U134` | RETAIN | Korea Inside | Korea Inside |
| `INF-008-U135` | RETAIN | 2024 | 2024 |
| `INF-008-U136` | RETAIN | 7 | 7 |

### INF-009

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-009-U001` | RETAIN | Korea Inside | Korea Inside |
| `INF-009-U002` | TRANSLATE | Airport Limousine Bus – How to Use (Step by Step) | Bus limousine de l’aéroport – mode d’emploi étape par étape |
| `INF-009-U003` | RETAIN | 1 | 1 |
| `INF-009-U004` | TRANSLATE | Check Your Destination & Bus | Vérifiez votre destination et votre bus |
| `INF-009-U005` | TRANSLATE | • Find your final stop (hotel or nearby stop) | • Repérez votre arrêt final (hôtel ou arrêt à proximité) |
| `INF-009-U006` | TRANSLATE | • Check bus number, route, and boarding location (T1 or T2) | • Vérifiez le numéro du bus, l’itinéraire et le lieu d’embarquement (T1 ou T2) |
| `INF-009-U007` | TRANSLATE | • Use official website or airport signs | • Utilisez le site officiel ou les panneaux de l’aéroport |
| `INF-009-U008` | RETAIN | 2 | 2 |
| `INF-009-U009` | TRANSLATE | Go to the Bus Stop | Rejoignez l’arrêt de bus |
| `INF-009-U010` | TRANSLATE | BUS TICKET | BILLET DE BUS |
| `INF-009-U011` | TRANSLATE | • Follow “BUS / 리무진버스” signs | • Suivez les panneaux « BUS / 리무진버스 » |
| `INF-009-U012` | TRANSLATE | • Find your bus stop number | • Repérez le numéro de votre arrêt |
| `INF-009-U013` | TRANSLATE | • T1: 1F 4A~6A, 8A~11A | • T1 : 1F 4A~6A, 8A~11A |
| `INF-009-U014` | TRANSLATE | • T2: B1 6~11 | • T2 : B1 6~11 |
| `INF-009-U015` | RETAIN | 3 | 3 |
| `INF-009-U016` | TRANSLATE | Buy Your Ticket | Achetez votre billet |
| `INF-009-U017` | TRANSLATE | BUS TICKET | BILLET DE BUS |
| `INF-009-U018` | TRANSLATE | • Buy a ticket at the ticket booth or kiosk | • Achetez un billet au guichet ou à la borne |
| `INF-009-U019` | TRANSLATE | • You can also pay with T-money card on the bus | • Vous pouvez aussi payer avec une carte T-money dans le bus |
| `INF-009-U020` | TRANSLATE | • Keep your ticket until you get off | • Gardez votre billet jusqu’à votre descente |
| `INF-009-U021` | RETAIN | 4 | 4 |
| `INF-009-U022` | TRANSLATE | Board the Bus | Montez dans le bus |
| `INF-009-U023` | TRANSLATE | • Check the bus number on the front of the bus | • Vérifiez le numéro à l’avant du bus |
| `INF-009-U024` | TRANSLATE | • Queue in order and board | • Faites la queue dans l’ordre puis montez |
| `INF-009-U025` | TRANSLATE | • Put luggage in the storage compartment | • Placez les bagages dans la soute |
| `INF-009-U026` | RETAIN | 5 | 5 |
| `INF-009-U027` | TRANSLATE | Enjoy the Ride | Profitez du trajet |
| `INF-009-U028` | TRANSLATE | • Relax and enjoy the ride | • Détendez-vous et profitez du trajet |
| `INF-009-U029` | TRANSLATE | • Stops may vary depending on route | • Les arrêts peuvent varier selon la ligne |
| `INF-009-U030` | TRANSLATE | • Traffic conditions may affect arrival time | • La circulation peut modifier l’heure d’arrivée |
| `INF-009-U031` | RETAIN | 6 | 6 |
| `INF-009-U032` | TRANSLATE | Get Off at Your Stop | Descendez à votre arrêt |
| `INF-009-U033` | RETAIN | 6002 | 6002 |
| `INF-009-U034` | TRANSLATE | • Press the stop button before your stop | • Appuyez sur le bouton d’arrêt avant votre arrêt |
| `INF-009-U035` | TRANSLATE | • Check your belongings before getting off | • Vérifiez vos affaires avant de descendre |
| `INF-009-U036` | TRANSLATE | • Take your luggage | • Récupérez vos bagages |
| `INF-009-U037` | RETAIN | 7 | 7 |
| `INF-009-U038` | TRANSLATE | Walk to Your Accommodation | Marchez jusqu’à votre hébergement |
| `INF-009-U039` | TRANSLATE | HOTEL | HÔTEL |
| `INF-009-U040` | TRANSLATE | • Check the direction to your accommodation | • Vérifiez la direction de votre hébergement |
| `INF-009-U041` | TRANSLATE | • Most hotels are within 5–10 minutes walk from the bus stop | • La plupart des hôtels se trouvent à 5–10 minutes à pied de l’arrêt de bus |
| `INF-009-U042` | TRANSLATE | • Use maps app if needed | • Utilisez une appli de cartes si nécessaire |
| `INF-009-U043` | RETAIN | 8 | 8 |
| `INF-009-U044` | TRANSLATE | Arrive Safely! | Vous êtes arrivé ! |
| `INF-009-U045` | TRANSLATE | • You’ve made it! | • C’est fait ! |
| `INF-009-U046` | TRANSLATE | • Check in and enjoy your trip in Korea | • Installez-vous et profitez de votre voyage en Corée |
| `INF-009-U047` | TRANSLATE | • Thank you for using airport limousine bus! | • Merci d’avoir utilisé le bus limousine de l’aéroport ! |
| `INF-009-U048` | TRANSLATE | TIP & INFO | CONSEILS & INFOS |
| `INF-009-U049` | TRANSLATE | Operating Hours | Horaires de service |
| `INF-009-U050` | TRANSLATE | 04:20 ~ 23:30 (Varies by route) | 04:20 ~ 23:30 (selon la ligne) |
| `INF-009-U051` | TRANSLATE | Luggage | Bagages |
| `INF-009-U052` | TRANSLATE | 1 piece of luggage per person is free (Additional fee may apply) | 1 bagage par personne est gratuit (des frais supplémentaires peuvent s’appliquer) |
| `INF-009-U053` | TRANSLATE | Payment | Paiement |
| `INF-009-U054` | TRANSLATE | Cash, Credit Card, T-money, and other transportation cards | Espèces, carte bancaire, T-money et autres cartes de transport |
| `INF-009-U055` | TRANSLATE | Traffic Notice | Circulation |
| `INF-009-U056` | TRANSLATE | Allow extra time during rush hour and holidays | Prévoyez du temps supplémentaire aux heures de pointe et pendant les jours fériés |
| `INF-009-U057` | TRANSLATE | Children | Enfants |
| `INF-009-U058` | TRANSLATE | Children under 6 years old ride free (no separate seat) | Les enfants de moins de 6 ans voyagent gratuitement (sans siège séparé) |
| `INF-009-U059` | TRANSLATE | Where to Check More Information | Où vérifier plus d’informations |
| `INF-009-U060` | TRANSLATE | Airport Limousine Official Website | Site officiel Airport Limousine |
| `INF-009-U061` | RETAIN | https://airportlimousine.co.kr/en/ | https://airportlimousine.co.kr/en/ |
| `INF-009-U062` | TRANSLATE | Incheon Airport Official Website | Site officiel de l’aéroport d’Incheon |
| `INF-009-U063` | RETAIN | https://www.airport.kr/ap/en/ | https://www.airport.kr/ap/en/ |
| `INF-009-U064` | TRANSLATE | Information Desks | Comptoirs d’information |
| `INF-009-U065` | TRANSLATE | Located in Arrivals Hall (T1 1F, T2 1F) | Situés dans le hall des arrivées (T1 1F, T2 1F) |
| `INF-009-U066` | TRANSLATE | Please check the latest information before your trip. | Vérifiez les informations les plus récentes avant votre voyage. |
| `INF-009-U067` | TRANSLATE | Bus routes and stop locations may change. | Les lignes de bus et les emplacements des arrêts peuvent changer. |
| `INF-009-U068` | RETAIN | Korea Inside | Korea Inside |
| `INF-009-U069` | TRANSLATE | Updated: July 2024 | Mise à jour : juillet 2024 |

### INF-010

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-010-U001` | RETAIN | Korea Inside | Korea Inside |
| `INF-010-U002` | TRANSLATE | AREX Express vs All-Stop Route Map | Carte des lignes AREX Express et All-Stop |
| `INF-010-U003` | TRANSLATE | Incheon Airport to Seoul | De l’aéroport d’Incheon à Séoul |
| `INF-010-U004` | RETAIN | AREX Express | AREX Express |
| `INF-010-U005` | TRANSLATE | Faster, reserved-seat train | Train plus rapide avec siège réservé |
| `INF-010-U006` | RETAIN | Terminal 2 | Terminal 2 |
| `INF-010-U007` | RETAIN | Incheon Airport | Incheon Airport |
| `INF-010-U008` | RETAIN | Terminal 1 | Terminal 1 |
| `INF-010-U009` | RETAIN | Incheon Airport | Incheon Airport |
| `INF-010-U010` | RETAIN | Seoul Station | Seoul Station |
| `INF-010-U011` | TRANSLATE | FARE (ONE WAY) | TARIF (ALLER SIMPLE) |
| `INF-010-U012` | TRANSLATE | Adult | Adulte |
| `INF-010-U013` | RETAIN | ₩13,000 | ₩13,000 |
| `INF-010-U014` | TRANSLATE | Child | Enfant |
| `INF-010-U015` | RETAIN | ₩9,500 | ₩9,500 |
| `INF-010-U016` | TRANSLATE | TO SEOUL STATION | VERS SEOUL STATION |
| `INF-010-U017` | TRANSLATE | 51 min from Terminal 2 | 51 min depuis le Terminal 2 |
| `INF-010-U018` | TRANSLATE | 43 min from Terminal 1 | 43 min depuis le Terminal 1 |
| `INF-010-U019` | TRANSLATE | Separate Express ticket required | Billet Express séparé requis |
| `INF-010-U020` | RETAIN | AREX All-Stop | AREX All-Stop |
| `INF-010-U021` | TRANSLATE | Local train with more stops | Train local avec davantage d’arrêts |
| `INF-010-U022` | RETAIN | Terminal 2 | Terminal 2 |
| `INF-010-U023` | RETAIN | Incheon Airport | Incheon Airport |
| `INF-010-U024` | RETAIN | Terminal 1 | Terminal 1 |
| `INF-010-U025` | RETAIN | Incheon Airport | Incheon Airport |
| `INF-010-U026` | RETAIN | Gimpo Airport | Gimpo Airport |
| `INF-010-U027` | RETAIN | Hongik University | Hongik University |
| `INF-010-U028` | RETAIN | Gongdeok | Gongdeok |
| `INF-010-U029` | RETAIN | Seoul Station | Seoul Station |
| `INF-010-U030` | TRANSLATE | FARE (ONE WAY) | TARIF (ALLER SIMPLE) |
| `INF-010-U031` | RETAIN | T2 → Seoul Station | T2 → Seoul Station |
| `INF-010-U032` | RETAIN | ₩5,350 | ₩5,350 |
| `INF-010-U033` | RETAIN | T1 → Seoul Station | T1 → Seoul Station |
| `INF-010-U034` | RETAIN | ₩4,750 | ₩4,750 |
| `INF-010-U035` | TRANSLATE | TO SEOUL STATION | VERS SEOUL STATION |
| `INF-010-U036` | TRANSLATE | 66 min from Terminal 2 | 66 min depuis le Terminal 2 |
| `INF-010-U037` | TRANSLATE | 59 min from Terminal 1 | 59 min depuis le Terminal 1 |
| `INF-010-U038` | RETAIN | T | T |
| `INF-010-U039` | RETAIN | money | money |
| `INF-010-U040` | TRANSLATE | Use T-money or a single-use subway ticket | Utilisez T-money ou un ticket de métro à usage unique |
| `INF-010-U041` | TRANSLATE | Check the train type, platform and departure display before boarding. | Avant de monter, vérifiez le type de train, le quai et l’affichage des départs. |
| `INF-010-U042` | RETAIN | Korea Inside | Korea Inside |

### INF-011

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-011-U001` | TRANSLATE | AREX Express or All-Stop? | AREX Express ou All-Stop ? |
| `INF-011-U002` | TRANSLATE | Choose by destination, luggage and transfer. | Choisissez selon votre destination, vos bagages et les correspondances. |
| `INF-011-U003` | RETAIN | Express | Express |
| `INF-011-U004` | RETAIN | Terminal 2 | Terminal 2 |
| `INF-011-U005` | RETAIN | Terminal 1 | Terminal 1 |
| `INF-011-U006` | RETAIN | Seoul Station | Seoul Station |
| `INF-011-U007` | RETAIN | All-Stop | All-Stop |
| `INF-011-U008` | RETAIN | Terminal 2 | Terminal 2 |
| `INF-011-U009` | RETAIN | Terminal 1 | Terminal 1 |
| `INF-011-U010` | RETAIN | Gimpo Airport | Gimpo Airport |
| `INF-011-U011` | RETAIN | Hongik University | Hongik University |
| `INF-011-U012` | RETAIN | Gongdeok | Gongdeok |
| `INF-011-U013` | RETAIN | Seoul Station | Seoul Station |

### INF-012

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-012-U001` | TRANSLATE | Which AREX Station Should You Use? | Quelle station AREX utiliser ? |
| `INF-012-U002` | TRANSLATE | Find the best AREX train and station for your destination in Seoul | Trouvez le meilleur train AREX et la meilleure station pour votre destination à Séoul |
| `INF-012-U003` | RETAIN | Korea | Korea |
| `INF-012-U004` | RETAIN | Insid | Insid |
| `INF-012-U005` | RETAIN | Express | Express |
| `INF-012-U006` | TRANSLATE | (Direct to Seoul Station) | (Direct jusqu’à Seoul Station) |
| `INF-012-U007` | RETAIN | All-Stop | All-Stop |
| `INF-012-U008` | TRANSLATE | (Every Station) | (Tous les arrêts) |
| `INF-012-U009` | TRANSLATE | Transfer Needed | Correspondance nécessaire |
| `INF-012-U010` | TRANSLATE | Better with Large Luggage | Mieux avec de gros bagages |
| `INF-012-U011` | TRANSLATE | (Bus/Taxi Recommended) | (Bus/taxi recommandé) |
| `INF-012-U012` | TRANSLATE | Best Choice | Meilleur choix |
| `INF-012-U013` | TRANSLATE | DESTINATION | DESTINATION |
| `INF-012-U014` | TRANSLATE | BEST AREX TRAIN | MEILLEUR TRAIN AREX |
| `INF-012-U015` | TRANSLATE | GET OFF AT | DESCENDEZ À |
| `INF-012-U016` | TRANSLATE | TRANSFER & CONNECTIONS | CORRESPONDANCES |
| `INF-012-U017` | TRANSLATE | BEST FOR | IDÉAL POUR |
| `INF-012-U018` | TRANSLATE | OUR RECOMMENDATION | NOTRE RECOMMANDATION |
| `INF-012-U019` | RETAIN | Seoul Station | Seoul Station |
| `INF-012-U020` | TRANSLATE | KTX, City Center | KTX, centre-ville |
| `INF-012-U021` | RETAIN | EXPRESS | EXPRESS |
| `INF-012-U022` | TRANSLATE | (or All-Stop) | (ou All-Stop) |
| `INF-012-U023` | RETAIN | Seoul Station | Seoul Station |
| `INF-012-U024` | TRANSLATE | (Last Stop) | (Terminus) |
| `INF-012-U025` | TRANSLATE | KTX, Line 1, 4 | KTX, lignes 1 et 4 |
| `INF-012-U026` | TRANSLATE | Airport Bus, Taxi | Bus de l’aéroport, taxi |
| `INF-012-U027` | TRANSLATE | KTX travelers, business travelers, first-time visitors | Voyageurs en KTX, voyageurs d’affaires, premiers visiteurs |
| `INF-012-U028` | TRANSLATE | BEST CHOICE | MEILLEUR CHOIX |
| `INF-012-U029` | TRANSLATE | Fastest and most convenient | Le plus rapide et le plus pratique |
| `INF-012-U030` | RETAIN | Hongdae | Hongdae |
| `INF-012-U031` | RETAIN | (Hongik Univ.) | (Hongik Univ.) |
| `INF-012-U032` | TRANSLATE | Shopping, Youth | Shopping, jeunesse |
| `INF-012-U033` | RETAIN | ALL-STOP | ALL-STOP |
| `INF-012-U034` | RETAIN | Hongik University | Hongik University |
| `INF-012-U035` | TRANSLATE | (6th Stop) | (6e arrêt) |
| `INF-012-U036` | TRANSLATE | Line 2 | Ligne 2 |
| `INF-012-U037` | TRANSLATE | (2 stops to Gangnam) | (2 arrêts jusqu’à Gangnam) |
| `INF-012-U038` | TRANSLATE | Young travelers, students, nightlife | Jeunes voyageurs, étudiants, vie nocturne |
| `INF-012-U039` | TRANSLATE | BEST CHOICE | MEILLEUR CHOIX |
| `INF-012-U040` | TRANSLATE | Direct, easy, and affordable | Direct, simple et économique |
| `INF-012-U041` | RETAIN | Gongdeok | Gongdeok |
| `INF-012-U042` | TRANSLATE | Mapo, Yeouido, Gov’t Offices | Mapo, Yeouido, administrations |
| `INF-012-U043` | RETAIN | ALL-STOP | ALL-STOP |
| `INF-012-U044` | RETAIN | Gongdeok | Gongdeok |
| `INF-012-U045` | TRANSLATE | (8th Stop) | (8e arrêt) |
| `INF-012-U046` | TRANSLATE | Line 5, 6 | Lignes 5 et 6 |
| `INF-012-U047` | RETAIN | Gyeongui-Jungang Line | Gyeongui-Jungang Line |
| `INF-012-U048` | TRANSLATE | Airport Bus | Bus de l’aéroport |
| `INF-012-U049` | TRANSLATE | Business travelers, Mapo/Yeouido area, City Hall | Voyageurs d’affaires, secteur Mapo/Yeouido, City Hall |
| `INF-012-U050` | TRANSLATE | BEST CHOICE | MEILLEUR CHOIX |
| `INF-012-U051` | TRANSLATE | Very convenient location | Emplacement très pratique |
| `INF-012-U052` | RETAIN | Myeongdong | Myeongdong |
| `INF-012-U053` | TRANSLATE | Shopping, Sightseeing | Shopping, visites |
| `INF-012-U054` | RETAIN | EXPRESS | EXPRESS |
| `INF-012-U055` | TRANSLATE | (or All-Stop) | (ou All-Stop) |
| `INF-012-U056` | RETAIN | Seoul Station | Seoul Station |
| `INF-012-U057` | TRANSLATE | (Last Stop) | (Terminus) |
| `INF-012-U058` | TRANSLATE | → Walk or take Subway (Line 4) | → Marchez ou prenez le métro (ligne 4) |
| `INF-012-U059` | TRANSLATE | Line 4 | Ligne 4 |
| `INF-012-U060` | TRANSLATE | (1 stop) | (1 arrêt) |
| `INF-012-U061` | TRANSLATE | Sightseeing, shopping, first-timers | Visites, shopping, premiers visiteurs |
| `INF-012-U062` | TRANSLATE | BOTH OK | LES DEUX |
| `INF-012-U063` | TRANSLATE | Express is faster overall | L’Express est plus rapide au total |
| `INF-012-U064` | RETAIN | Gangnam | Gangnam |
| `INF-012-U065` | TRANSLATE | Business, Luxury Area | Affaires, quartier haut de gamme |
| `INF-012-U066` | RETAIN | ALL-STOP | ALL-STOP |
| `INF-012-U067` | RETAIN | Hongik University | Hongik University |
| `INF-012-U068` | TRANSLATE | (6th Stop) | (6e arrêt) |
| `INF-012-U069` | TRANSLATE | Line 2 | Ligne 2 |
| `INF-012-U070` | TRANSLATE | (Direct to Gangnam) | (Direct jusqu’à Gangnam) |
| `INF-012-U071` | TRANSLATE | Shopping, business, clinics, COEX | Shopping, affaires, cliniques, COEX |
| `INF-012-U072` | TRANSLATE | BOTH OK | LES DEUX |
| `INF-012-U073` | TRANSLATE | Short transfer to Line 2 | Courte correspondance vers la ligne 2 |
| `INF-012-U074` | RETAIN | Jamsil | Jamsil |
| `INF-012-U075` | TRANSLATE | Lotte World, Sports | Lotte World, sport |
| `INF-012-U076` | RETAIN | ALL-STOP | ALL-STOP |
| `INF-012-U077` | RETAIN | Hongik University | Hongik University |
| `INF-012-U078` | TRANSLATE | (6th Stop) | (6e arrêt) |
| `INF-012-U079` | TRANSLATE | Line 2 | Ligne 2 |
| `INF-012-U080` | TRANSLATE | (To Jamsil Station) | (Jusqu’à Jamsil Station) |
| `INF-012-U081` | TRANSLATE | Families, Lotte World, events | Familles, Lotte World, événements |
| `INF-012-U082` | TRANSLATE | BOTH OK | LES DEUX |
| `INF-012-U083` | TRANSLATE | Transfer to Line 2 at Hongdae | Correspondance vers la ligne 2 à Hongdae |
| `INF-012-U084` | RETAIN | Gimpo Airport | Gimpo Airport |
| `INF-012-U085` | TRANSLATE | Domestic Flights | Vols intérieurs |
| `INF-012-U086` | RETAIN | ALL-STOP | ALL-STOP |
| `INF-012-U087` | RETAIN | Gimpo Airport | Gimpo Airport |
| `INF-012-U088` | TRANSLATE | (Last Stop) | (Terminus) |
| `INF-012-U089` | TRANSLATE | Walk | À pied |
| `INF-012-U090` | TRANSLATE | Domestic travelers, connecting flights | Voyageurs intérieurs, vols en correspondance |
| `INF-012-U091` | TRANSLATE | BEST CHOICE | MEILLEUR CHOIX |
| `INF-012-U092` | TRANSLATE | Direct and convenient | Direct et pratique |
| `INF-012-U093` | TRANSLATE | EXPRESS vs ALL-STOP AT A GLANCE | EXPRESS vs ALL-STOP EN UN COUP D’ŒIL |
| `INF-012-U094` | RETAIN | EXPRESS | EXPRESS |
| `INF-012-U095` | TRANSLATE | (Direct) | (Direct) |
| `INF-012-U096` | TRANSLATE | Incheon Airport T1 → Seoul Station 43 min | Incheon Airport T1 → Seoul Station 43 min |
| `INF-012-U097` | TRANSLATE | Incheon Airport T2 → Seoul Station ≈ 51 min | Incheon Airport T2 → Seoul Station ≈ 51 min |
| `INF-012-U098` | TRANSLATE | • Fastest to Seoul Station | • Le plus rapide vers Seoul Station |
| `INF-012-U099` | TRANSLATE | • Reserved seats | • Sièges réservés |
| `INF-012-U100` | TRANSLATE | • Slightly higher fare | • Tarif légèrement plus élevé |
| `INF-012-U101` | RETAIN | ₩13,000 | ₩13,000 |
| `INF-012-U102` | RETAIN | ALL-STOP | ALL-STOP |
| `INF-012-U103` | TRANSLATE | (Every Station) | (Tous les arrêts) |
| `INF-012-U104` | TRANSLATE | T1 → Seoul Station 59 min | T1 → Seoul Station 59 min |
| `INF-012-U105` | TRANSLATE | T2 → Seoul Station ≈ 66 min | T2 → Seoul Station ≈ 66 min |
| `INF-012-U106` | TRANSLATE | • Stops at every station | • S’arrête à toutes les stations |
| `INF-012-U107` | TRANSLATE | • Lower fare | • Tarif plus bas |
| `INF-012-U108` | TRANSLATE | • single-journey ticket | • billet pour un trajet |
| `INF-012-U109` | RETAIN | ₩4,750 (T1) | ₩4,750 (T1) |
| `INF-012-U110` | RETAIN | ₩5,350 (T2) | ₩5,350 (T2) |
| `INF-012-U111` | TRANSLATE | TIPS | CONSEILS |
| `INF-012-U112` | TRANSLATE | Both trains use the same station at T1 and T2. Check the train type before boarding. | Les deux trains utilisent la même station aux T1 et T2. Vérifiez le type de train avant de monter. |
| `INF-012-U113` | TRANSLATE | If your hotel is near a station on Line 2, Hongdae is a good transfer point. | Si votre hôtel est près d’une station de la ligne 2, Hongdae est un bon point de correspondance. |
| `INF-012-U114` | TRANSLATE | Traveling with large luggage or a group? Compare airport bus or taxi. | Vous voyagez avec de gros bagages ou en groupe ? Comparez avec le bus de l’aéroport ou le taxi. |
| `INF-012-U115` | TRANSLATE | Travel times and fares are approximate and may change. Check the latest schedule on the AREX official website. | Les temps de trajet et tarifs sont approximatifs et peuvent changer. Vérifiez les horaires les plus récents sur le site officiel d’AREX. |
| `INF-012-U116` | RETAIN | Korea Inside | Korea Inside |

### INF-013

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-013-U001` | TRANSLATE | How to Find AREX at Incheon Airport (T1 & T2) | Comment trouver l’AREX à l’aéroport d’Incheon (T1 et T2) |
| `INF-013-U002` | TRANSLATE | Step-by-Step Directions from Arrival to Platform | Itinéraire étape par étape des arrivées jusqu’au quai |
| `INF-013-U003` | RETAIN | Korea Inside | Korea Inside |
| `INF-013-U004` | TRANSLATE | Express Train | Train Express |
| `INF-013-U005` | TRANSLATE | (Direct to Seoul Station) | (Direct jusqu’à Seoul Station) |
| `INF-013-U006` | TRANSLATE | All-Stop Train | Train All-Stop |
| `INF-013-U007` | TRANSLATE | (Every Station) | (Tous les arrêts) |
| `INF-013-U008` | TRANSLATE | Follow the signs: | Suivez les panneaux : |
| `INF-013-U009` | RETAIN | “Airport Railroad” 공항철도 | “Airport Railroad” 공항철도 |
| `INF-013-U010` | RETAIN | TERMINAL 1 | TERMINAL 1 |
| `INF-013-U011` | TRANSLATE | From Arrival Hall to Platform | Du hall des arrivées au quai |
| `INF-013-U012` | TRANSLATE | Allow extra time with luggage. | Prévoyez plus de temps avec des bagages. |
| `INF-013-U013` | RETAIN | 1 | 1 |
| `INF-013-U014` | TRANSLATE | Arrival Hall (1F) | Hall des arrivées (1F) |
| `INF-013-U015` | TRANSLATE | Complete immigration, baggage claim, and customs. | Passez l’immigration, récupérez vos bagages et franchissez la douane. |
| `INF-013-U016` | RETAIN | 2 | 2 |
| `INF-013-U017` | TRANSLATE | Follow the Signs | Suivez les panneaux |
| `INF-013-U018` | TRANSLATE | Look for the blue “Airport Railroad” signs. | Repérez les panneaux bleus « Airport Railroad ». |
| `INF-013-U019` | RETAIN | Airport Railroad | Airport Railroad |
| `INF-013-U020` | RETAIN | 3 | 3 |
| `INF-013-U021` | TRANSLATE | Go to Transportation Center (B1F) | Rejoignez le Transportation Center (B1F) |
| `INF-013-U022` | TRANSLATE | Take the escalator or elevator down to B1 (Transportation Center). | Descendez au B1 (Transportation Center) par l’escalator ou l’ascenseur. |
| `INF-013-U023` | RETAIN | Transportation Center | Transportation Center |
| `INF-013-U024` | RETAIN | B1 | B1 |
| `INF-013-U025` | RETAIN | 4 | 4 |
| `INF-013-U026` | TRANSLATE | Buy Your Ticket | Achetez votre billet |
| `INF-013-U027` | TRANSLATE | Purchase tickets at the machines or ticket counters. | Achetez vos billets aux automates ou aux guichets. |
| `INF-013-U028` | TRANSLATE | Express Train | Train Express |
| `INF-013-U029` | TRANSLATE | One-way ticket | Billet aller simple |
| `INF-013-U030` | TRANSLATE | (seat reservation included) | (réservation du siège incluse) |
| `INF-013-U031` | TRANSLATE | All-Stop Train | Train All-Stop |
| `INF-013-U032` | TRANSLATE | T-money card or single-journey ticket | Carte T-money ou billet pour un trajet |
| `INF-013-U033` | RETAIN | 5 | 5 |
| `INF-013-U034` | TRANSLATE | Go to the Platforms | Rejoignez les quais |
| `INF-013-U035` | TRANSLATE | Follow the signs and take the escalator or elevator down to the platform. | Suivez les panneaux puis descendez au quai par l’escalator ou l’ascenseur. |
| `INF-013-U036` | RETAIN | 6 | 6 |
| `INF-013-U037` | TRANSLATE | Board the Train | Montez dans le train |
| `INF-013-U038` | TRANSLATE | Check the train type (Express or All-Stop) and platform on the electronic board. | Vérifiez le type de train (Express ou All-Stop) et le quai sur le panneau électronique. |
| `INF-013-U039` | RETAIN | TERMINAL 2 | TERMINAL 2 |
| `INF-013-U040` | TRANSLATE | From Arrival Hall to Platform | Du hall des arrivées au quai |
| `INF-013-U041` | TRANSLATE | Allow extra time with luggage. | Prévoyez plus de temps avec des bagages. |
| `INF-013-U042` | RETAIN | 1 | 1 |
| `INF-013-U043` | TRANSLATE | Arrival Hall (1F) | Hall des arrivées (1F) |
| `INF-013-U044` | TRANSLATE | Complete immigration, baggage claim, and customs. | Passez l’immigration, récupérez vos bagages et franchissez la douane. |
| `INF-013-U045` | RETAIN | 2 | 2 |
| `INF-013-U046` | TRANSLATE | Follow the Signs | Suivez les panneaux |
| `INF-013-U047` | TRANSLATE | Look for the blue “Airport Railroad” signs. | Repérez les panneaux bleus « Airport Railroad ». |
| `INF-013-U048` | RETAIN | Airport Railroad | Airport Railroad |
| `INF-013-U049` | RETAIN | 3 | 3 |
| `INF-013-U050` | TRANSLATE | Go to Transportation Center (B1F) | Rejoignez le Transportation Center (B1F) |
| `INF-013-U051` | TRANSLATE | Take the escalator or elevator down to B1 (Transportation Center). | Descendez au B1 (Transportation Center) par l’escalator ou l’ascenseur. |
| `INF-013-U052` | RETAIN | Transportation Center | Transportation Center |
| `INF-013-U053` | RETAIN | B1 | B1 |
| `INF-013-U054` | RETAIN | 4 | 4 |
| `INF-013-U055` | TRANSLATE | Buy Your Ticket | Achetez votre billet |
| `INF-013-U056` | TRANSLATE | Purchase tickets at the machines or ticket counters. | Achetez vos billets aux automates ou aux guichets. |
| `INF-013-U057` | TRANSLATE | Express Train | Train Express |
| `INF-013-U058` | TRANSLATE | One-way ticket | Billet aller simple |
| `INF-013-U059` | TRANSLATE | (seat reservation included) | (réservation du siège incluse) |
| `INF-013-U060` | TRANSLATE | All-Stop Train | Train All-Stop |
| `INF-013-U061` | TRANSLATE | T-money card or single-journey ticket | Carte T-money ou billet pour un trajet |
| `INF-013-U062` | RETAIN | 5 | 5 |
| `INF-013-U063` | TRANSLATE | Go to the Platforms | Rejoignez les quais |
| `INF-013-U064` | TRANSLATE | Follow the signs and take the escalator or elevator down to the platform. | Suivez les panneaux puis descendez au quai par l’escalator ou l’ascenseur. |
| `INF-013-U065` | RETAIN | 6 | 6 |
| `INF-013-U066` | TRANSLATE | Board the Train | Montez dans le train |
| `INF-013-U067` | TRANSLATE | Check the train type (Express or All-Stop) and platform on the electronic board. | Vérifiez le type de train (Express ou All-Stop) et le quai sur le panneau électronique. |
| `INF-013-U068` | TRANSLATE | IMPORTANT NOTES | À SAVOIR |
| `INF-013-U069` | TRANSLATE | Each terminal has one AREX station. Express and All-Stop trains have separate ticket gates and boarding areas. Follow the signs and check the electronic board. | Chaque terminal possède une station AREX. Les trains Express et All-Stop ont des portiques et zones d’embarquement séparés. Suivez les panneaux et vérifiez l’affichage électronique. |
| `INF-013-U070` | TRANSLATE | Platform information can change. Always confirm the correct platform and train type before boarding. | Les informations de quai peuvent changer. Confirmez toujours le bon quai et le bon type de train avant de monter. |
| `INF-013-U071` | TRANSLATE | Ticket types and fares are shown on the left. | Les types de billets et tarifs sont indiqués à gauche. |
| `INF-013-U072` | TRANSLATE | Source: AREX Official Website (www.arex.or.kr) \| Incheon Airport Official Website (www.airport.kr) | Sources : site officiel AREX (www.arex.or.kr) \| site officiel de l’aéroport d’Incheon (www.airport.kr) |
| `INF-013-U073` | TRANSLATE | Last Updated: July 26, 2026 | Dernière mise à jour : 26 juillet 2026 |

### INF-014

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-014-U001` | TRANSLATE | AREX Ticket Decision Guide | Guide pour choisir son billet AREX |
| `INF-014-U002` | TRANSLATE | Answer a few simple questions to choose the best way to travel. | Répondez à quelques questions simples pour choisir la meilleure option. |
| `INF-014-U003` | TRANSLATE | Two Trains. Different Tickets. | Deux trains. Des billets différents. |
| `INF-014-U004` | TRANSLATE | • Express Train → reserved seats, not available with T-money | • Train Express → sièges réservés, T-money non acceptée |
| `INF-014-U005` | TRANSLATE | • All-Stop Train → can use T-money or single-journey ticket | • Train All-Stop → T-money ou billet pour un trajet |
| `INF-014-U006` | TRANSLATE | LEGEND | LÉGENDE |
| `INF-014-U007` | TRANSLATE | Express Train (Direct to Seoul Station) | Train Express (direct jusqu’à Seoul Station) |
| `INF-014-U008` | TRANSLATE | All-Stop Train (Every Station) | Train All-Stop (tous les arrêts) |
| `INF-014-U009` | TRANSLATE | START HERE | COMMENCEZ ICI |
| `INF-014-U010` | TRANSLATE | Where are you going? | Où allez-vous ? |
| `INF-014-U011` | RETAIN | 1 | 1 |
| `INF-014-U012` | TRANSLATE | Is your final destination Seoul Station? | Votre destination finale est-elle Seoul Station ? |
| `INF-014-U013` | TRANSLATE | YES | OUI |
| `INF-014-U014` | TRANSLATE | NO | NON |
| `INF-014-U015` | RETAIN | 2 | 2 |
| `INF-014-U016` | TRANSLATE | Do you want a reserved seat? | Voulez-vous un siège réservé ? |
| `INF-014-U017` | TRANSLATE | YES | OUI |
| `INF-014-U018` | TRANSLATE | NO | NON |
| `INF-014-U019` | RETAIN | 3 | 3 |
| `INF-014-U020` | TRANSLATE | Do you have T-money card? | Avez-vous une carte T-money ? |
| `INF-014-U021` | RETAIN | T | T |
| `INF-014-U022` | RETAIN | money | money |
| `INF-014-U023` | TRANSLATE | YES | OUI |
| `INF-014-U024` | TRANSLATE | NO | NON |
| `INF-014-U025` | RETAIN | 4 | 4 |
| `INF-014-U026` | TRANSLATE | Traveling with large luggage or with family/elderly? | Vous voyagez avec de gros bagages, en famille ou avec une personne âgée ? |
| `INF-014-U027` | TRANSLATE | YES | OUI |
| `INF-014-U028` | TRANSLATE | NO | NON |
| `INF-014-U029` | RETAIN | 5 | 5 |
| `INF-014-U030` | TRANSLATE | Arriving late at night (may miss the last train)? | Vous arrivez tard dans la nuit (risque de manquer le dernier train) ? |
| `INF-014-U031` | TRANSLATE | YES | OUI |
| `INF-014-U032` | TRANSLATE | NO | NON |
| `INF-014-U033` | RETAIN | EXPRESS TRAIN | EXPRESS TRAIN |
| `INF-014-U034` | TRANSLATE | (Online Reservation) | (Réservation en ligne) |
| `INF-014-U035` | TRANSLATE | Best for travelers going to Seoul Station who want fast, comfortable travel with a reserved seat. | Idéal pour aller à Seoul Station rapidement et confortablement avec un siège réservé. |
| `INF-014-U036` | TRANSLATE | How to Buy | Comment acheter |
| `INF-014-U037` | TRANSLATE | Buy online in advance | Achetez en ligne à l’avance |
| `INF-014-U038` | TRANSLATE | Pick your seat | Choisissez votre siège |
| `INF-014-U039` | TRANSLATE | QR ticket (mobile) | Billet QR (mobile) |
| `INF-014-U040` | TRANSLATE | Show QR at gate | Présentez le QR au portique |
| `INF-014-U041` | TRANSLATE | Pay | PAYER |
| `INF-014-U042` | TRANSLATE | Credit/Debit Card (International cards OK) | Carte de crédit/débit (cartes internationales acceptées) |
| `INF-014-U043` | RETAIN | EXPRESS TRAIN | EXPRESS TRAIN |
| `INF-014-U044` | TRANSLATE | (At Station) | (À la station) |
| `INF-014-U045` | TRANSLATE | Buy at the station on the day of travel. Reserved seat available if seats remain. | Achetez à la station le jour du trajet. Sièges réservés disponibles s’il reste des places. |
| `INF-014-U046` | TRANSLATE | How to Buy | Comment acheter |
| `INF-014-U047` | TRANSLATE | Use ticket machines or ticket counters | Utilisez les automates ou les guichets |
| `INF-014-U048` | TRANSLATE | Choose time & seat | Choisissez l’heure et le siège |
| `INF-014-U049` | TRANSLATE | Pay and get ticket | Payez et récupérez le billet |
| `INF-014-U050` | TRANSLATE | Pay | PAYER |
| `INF-014-U051` | TRANSLATE | Credit/Debit Card or Cash (KRW) | Carte de crédit/débit ou espèces (KRW) |
| `INF-014-U052` | RETAIN | ALL-STOP TRAIN | ALL-STOP TRAIN |
| `INF-014-U053` | TRANSLATE | (T-money) | (T-money) |
| `INF-014-U054` | TRANSLATE | Use your T-money card. Tap in and tap out. Cheapest and most convenient for multiple stops. | Utilisez votre carte T-money. Validez à l’entrée et à la sortie. C’est l’option la moins chère et la plus pratique avec plusieurs arrêts. |
| `INF-014-U055` | TRANSLATE | How to Use | Comment l’utiliser |
| `INF-014-U056` | TRANSLATE | Tap in at gate | Validez au portique d’entrée |
| `INF-014-U057` | TRANSLATE | Tap out at your destination | Validez à la sortie à destination |
| `INF-014-U058` | TRANSLATE | Fare is calculated automatically by distance | Le tarif est calculé automatiquement selon la distance |
| `INF-014-U059` | TRANSLATE | Pay | PAYER |
| `INF-014-U060` | RETAIN | T | T |
| `INF-014-U061` | RETAIN | money | money |
| `INF-014-U062` | TRANSLATE | T-money Card (Balance required) | Carte T-money (solde requis) |
| `INF-014-U063` | RETAIN | ALL-STOP TRAIN | ALL-STOP TRAIN |
| `INF-014-U064` | TRANSLATE | (Single-Journey Ticket) | (Billet pour un trajet) |
| `INF-014-U065` | TRANSLATE | Buy a single-journey ticket at the station. | Achetez un billet pour un trajet à la station. |
| `INF-014-U066` | TRANSLATE | How to Buy | Comment acheter |
| `INF-014-U067` | TRANSLATE | Use ticket machines or ticket counters | Utilisez les automates ou les guichets |
| `INF-014-U068` | TRANSLATE | Select your destination | Sélectionnez votre destination |
| `INF-014-U069` | TRANSLATE | Get ticket and go | Récupérez le billet et partez |
| `INF-014-U070` | TRANSLATE | Pay | PAYER |
| `INF-014-U071` | TRANSLATE | Credit/Debit Card or Cash (KRW) | Carte de crédit/débit ou espèces (KRW) |
| `INF-014-U072` | RETAIN | EXPRESS TRAIN | EXPRESS TRAIN |
| `INF-014-U073` | TRANSLATE | (Recommended) | (Recommandé) |
| `INF-014-U074` | TRANSLATE | More space, wider seats, and luggage racks. Better for families, seniors, or large luggage. | Plus d’espace, sièges plus larges et porte-bagages. Plus confortable pour les familles, les seniors ou les gros bagages. |
| `INF-014-U075` | TRANSLATE | How to Buy | Comment acheter |
| `INF-014-U076` | TRANSLATE | Reserve online or buy at the station | Réservez en ligne ou achetez à la station |
| `INF-014-U077` | TRANSLATE | Choose seat | Choisissez votre siège |
| `INF-014-U078` | TRANSLATE | Board and enjoy | Montez et profitez du trajet |
| `INF-014-U079` | TRANSLATE | Pay | PAYER |
| `INF-014-U080` | TRANSLATE | Credit/Debit Card (International cards OK) | Carte de crédit/débit (cartes internationales acceptées) |
| `INF-014-U081` | TRANSLATE | CHECK SCHEDULE FIRST | VÉRIFIEZ D’ABORD LES HORAIRES |
| `INF-014-U082` | TRANSLATE | If you might miss the last AREX train, consider Airport Bus or Taxi. | Si vous risquez de manquer le dernier AREX, envisagez le bus de l’aéroport ou le taxi. |
| `INF-014-U083` | TRANSLATE | What to Do | Que faire |
| `INF-014-U084` | TRANSLATE | Check last train time (on official website) | Vérifiez l’heure du dernier train (sur le site officiel) |
| `INF-014-U085` | TRANSLATE | If too late, use Airport Bus or Taxi instead | S’il est trop tard, prenez plutôt le bus de l’aéroport ou un taxi |
| `INF-014-U086` | TRANSLATE | Useful Links | Liens utiles |
| `INF-014-U087` | RETAIN | airport-bus.html | airport-bus.html |
| `INF-014-U088` | RETAIN | taxi.html | taxi.html |
| `INF-014-U089` | TRANSLATE | TICKET TYPES OVERVIEW | APERÇU DES TYPES DE BILLETS |
| `INF-014-U090` | TRANSLATE | Express Train | Train Express |
| `INF-014-U091` | TRANSLATE | • Reserved seat | • Siège réservé |
| `INF-014-U092` | TRANSLATE | • Direct to Seoul Station | • Direct jusqu’à Seoul Station |
| `INF-014-U093` | TRANSLATE | • Not available with T-money | • T-money non acceptée |
| `INF-014-U094` | TRANSLATE | • Online reservation available | • Réservation en ligne possible |
| `INF-014-U095` | TRANSLATE | All-Stop Train | Train All-Stop |
| `INF-014-U096` | TRANSLATE | • Stops at every station | • S’arrête à toutes les stations |
| `INF-014-U097` | TRANSLATE | • Use T-money or single ticket | • T-money ou billet simple |
| `INF-014-U098` | TRANSLATE | • Lower fare | • Tarif plus bas |
| `INF-014-U099` | TRANSLATE | • Great for multiple destinations | • Pratique pour plusieurs destinations |
| `INF-014-U100` | TRANSLATE | PAYMENT METHODS | MOYENS DE PAIEMENT |
| `INF-014-U101` | TRANSLATE | Credit/Debit Card | Carte de crédit/débit |
| `INF-014-U102` | RETAIN | (Visa, Mastercard, JCB, American Express) | (Visa, Mastercard, JCB, American Express) |
| `INF-014-U103` | TRANSLATE | Cash (KRW) | Espèces (KRW) |
| `INF-014-U104` | TRANSLATE | (At ticket counter only) | (Au guichet uniquement) |
| `INF-014-U105` | RETAIN | T | T |
| `INF-014-U106` | RETAIN | money | money |
| `INF-014-U107` | TRANSLATE | T-money Card | Carte T-money |
| `INF-014-U108` | TRANSLATE | (For All-Stop Train only) | (Train All-Stop uniquement) |
| `INF-014-U109` | TRANSLATE | GOOD TO KNOW | BON À SAVOIR |
| `INF-014-U110` | TRANSLATE | Children 6–12 get discounted fares. Under 6 ride free. | Les enfants de 6 à 12 ans bénéficient d’un tarif réduit. Moins de 6 ans : gratuit. |
| `INF-014-U111` | TRANSLATE | Bring your passport if you reserved online. | Apportez votre passeport si vous avez réservé en ligne. |
| `INF-014-U112` | TRANSLATE | Keep your single-journey ticket until you exit. | Gardez votre billet pour un trajet jusqu’à la sortie. |
| `INF-014-U113` | TRANSLATE | Schedules and fares may change. Check the official website. | Les horaires et tarifs peuvent changer. Vérifiez le site officiel. |
| `INF-014-U114` | TRANSLATE | OFFICIAL WEBSITE | SITE OFFICIEL |
| `INF-014-U115` | RETAIN | www.airportrailroad.com | www.airportrailroad.com |
| `INF-014-U116` | TRANSLATE | Check schedules, fares, and ticket information. | Vérifiez les horaires, tarifs et informations sur les billets. |
| `INF-014-U117` | TRANSLATE | Source: AREX Official Website (www.airportrailroad.com) \| Incheon Airport Official Website (www.airport.kr) | Sources : site officiel AREX (www.airportrailroad.com) \| site officiel de l’aéroport d’Incheon (www.airport.kr) |
| `INF-014-U118` | TRANSLATE | Last Updated: July 26, 2026 | Dernière mise à jour : 26 juillet 2026 |

### INF-015

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-015-U001` | RETAIN | Korea Inside | Korea Inside |
| `INF-015-U002` | TRANSLATE | When Should You NOT Use AREX? | Quand NE PAS prendre l’AREX ? |
| `INF-015-U003` | TRANSLATE | Incheon Airport to Seoul · When another option may be better | Aéroport d’Incheon → Séoul · Quand une autre option peut être préférable |
| `INF-015-U004` | RETAIN | 1 | 1 |
| `INF-015-U005` | TRANSLATE | Heavy luggage or a stroller | Gros bagages ou poussette |
| `INF-015-U006` | TRANSLATE | AREX is still possible, but multiple escalators, long station walks and a later subway transfer can be tiring. A direct airport bus or private pickup is often easier. | L’AREX reste possible, mais les escalators, les longues marches en station et une correspondance en métro peuvent fatiguer. Un bus direct de l’aéroport ou une prise en charge privée est souvent plus simple. |
| `INF-015-U007` | RETAIN | 2 | 2 |
| `INF-015-U008` | TRANSLATE | Your hotel is in Gangnam or Jamsil | Votre hôtel est à Gangnam ou Jamsil |
| `INF-015-U009` | TRANSLATE | AREX usually means Seoul Station or Gongdeok first, then another subway ride. If you want fewer transfers, compare an airport bus or taxi instead. | Avec l’AREX, il faut généralement passer d’abord par Seoul Station ou Gongdeok, puis reprendre le métro. Pour limiter les correspondances, comparez plutôt le bus de l’aéroport ou le taxi. |
| `INF-015-U010` | RETAIN | 3 | 3 |
| `INF-015-U011` | TRANSLATE | You arrive very late | Vous arrivez très tard |
| `INF-015-U012` | TRANSLATE | AREX does not run all night. If your flight lands close to the last train, immigration and baggage claim may make you miss it. Check the latest train time before choosing AREX. | L’AREX ne circule pas toute la nuit. Si votre vol atterrit près de l’heure du dernier train, l’immigration et les bagages peuvent vous le faire manquer. Vérifiez l’horaire le plus récent avant de choisir l’AREX. |
| `INF-015-U013` | RETAIN | 4 | 4 |
| `INF-015-U014` | TRANSLATE | You are traveling with family or older adults | Vous voyagez en famille ou avec des personnes âgées |
| `INF-015-U015` | TRANSLATE | The train can be efficient, but long walks, stairs and crowded subway transfers may be stressful for children, parents or seniors. A direct bus or private ride may be more comfortable. | Le train peut être efficace, mais les longues marches, les escaliers et les correspondances dans un métro chargé peuvent être pénibles pour les enfants, les parents ou les seniors. Un bus direct ou un trajet privé peut être plus confortable. |
| `INF-015-U016` | RETAIN | 5 | 5 |
| `INF-015-U017` | TRANSLATE | HOTEL | HÔTEL |
| `INF-015-U018` | TRANSLATE | A bus stop is near your hotel | Un arrêt de bus est proche de votre hôtel |
| `INF-015-U019` | TRANSLATE | If an airport limousine bus drops you close to the hotel entrance, it may save time and effort even if the train is faster on paper. | Si un bus limousine de l’aéroport vous dépose près de l’entrée de l’hôtel, il peut économiser du temps et des efforts même si le train est plus rapide sur le papier. |
| `INF-015-U020` | TRANSLATE | BETTER ALTERNATIVES | MEILLEURES ALTERNATIVES |
| `INF-015-U021` | TRANSLATE | Airport Bus | Bus de l’aéroport |
| `INF-015-U022` | TRANSLATE | Best for direct hotel-area access | Idéal pour rejoindre directement le secteur de l’hôtel |
| `INF-015-U023` | TRANSLATE | TAXI | TAXI |
| `INF-015-U024` | TRANSLATE | Taxi | Taxi |
| `INF-015-U025` | TRANSLATE | Best for late arrival or door-to-door convenience | Idéal pour une arrivée tardive ou un trajet porte à porte |
| `INF-015-U026` | TRANSLATE | Private Pickup | Prise en charge privée |
| `INF-015-U027` | TRANSLATE | Best for families, groups and lots of luggage | Idéal pour les familles, les groupes et beaucoup de bagages |
| `INF-015-U028` | TRANSLATE | AREX is still one of the best options for Seoul Station, Hongdae, Gongdeok and Gimpo Airport — but it is not always the easiest choice. | L’AREX reste l’une des meilleures options pour Seoul Station, Hongdae, Gongdeok et Gimpo Airport — mais ce n’est pas toujours la plus simple. |
| `INF-015-U029` | TRANSLATE | Always check the latest train times and route details before travel. | Vérifiez toujours les horaires de train et les détails de l’itinéraire les plus récents avant de partir. |

### INF-016

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-016-U001` | TRANSLATE | YOUR FIRST 30 MINUTES | VOS 30 PREMIÈRES MINUTES |
| `INF-016-U002` | TRANSLATE | IN THE ARRIVAL HALL | DANS LE HALL DES ARRIVÉES |
| `INF-016-U003` | TRANSLATE | After immigration and customs, use this Incheon Airport arrival hall guide to connect, save your address, check payment and choose transport. | Après l’immigration et la douane, utilisez ce guide du hall des arrivées de l’aéroport d’Incheon pour vous connecter, enregistrer votre adresse, vérifier votre moyen de paiement et choisir votre transport. |
| `INF-016-U004` | TRANSLATE | Arrivals | Arrivées |
| `INF-016-U005` | TRANSLATE | Baggage Claim | Récupération des bagages |
| `INF-016-U006` | TRANSLATE | Customs | Douane |
| `INF-016-U007` | RETAIN | AREX | AREX |
| `INF-016-U008` | TRANSLATE | Bus | Bus |
| `INF-016-U009` | TRANSLATE | Taxi | Taxi |
| `INF-016-U010` | RETAIN | P | P |
| `INF-016-U011` | TRANSLATE | Parking | Parking |
| `INF-016-U012` | TRANSLATE | INFORMATION | INFORMATIONS |
| `INF-016-U013` | TRANSLATE | FREE Wi-Fi | Wi-Fi GRATUIT |
| `INF-016-U014` | RETAIN | 01 | 01 |
| `INF-016-U015` | TRANSLATE | CONNECT | SE CONNECTER |
| `INF-016-U016` | TRANSLATE | Check your eSIM or Wi-Fi. | Vérifiez votre eSIM ou votre Wi-Fi. |
| `INF-016-U017` | RETAIN | 02 | 02 |
| `INF-016-U018` | TRANSLATE | SAVE YOUR ADDRESS | ENREGISTREZ VOTRE ADRESSE |
| `INF-016-U019` | TRANSLATE | Keep your hotel address in Korean ready. | Gardez l’adresse de votre hôtel en coréen à portée de main. |
| `INF-016-U020` | RETAIN | 03 | 03 |
| `INF-016-U021` | RETAIN | ₩ | ₩ |
| `INF-016-U022` | TRANSLATE | CHECK PAYMENT | VÉRIFIEZ LE PAIEMENT |
| `INF-016-U023` | TRANSLATE | Confirm one working payment method. | Confirmez qu’au moins un moyen de paiement fonctionne. |
| `INF-016-U024` | RETAIN | 04 | 04 |
| `INF-016-U025` | TRANSLATE | CHOOSE TRANSPORT | CHOISISSEZ VOTRE TRANSPORT |
| `INF-016-U026` | TRANSLATE | AREX · Bus · Taxi · Pre-booked Transfer · Rental Car | AREX · Bus · Taxi · Transfert réservé · Voiture de location |

### INF-017

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-017-U001` | TRANSLATE | Terminal 1 Arrival Map | Plan des arrivées du Terminal 1 |
| `INF-017-U002` | RETAIN | eSIM/SIM | eSIM/SIM |
| `INF-017-U003` | RETAIN | 1–50 | 1–50 |
| `INF-017-U004` | TRANSLATE | Gates 1–50 | Portes 1–50 |
| `INF-017-U005` | RETAIN | 101–132 | 101–132 |
| `INF-017-U006` | TRANSLATE | Gates 101–132 | Portes 101–132 |
| `INF-017-U007` | RETAIN | 3F | 3F |
| `INF-017-U008` | RETAIN | 12 | 12 |
| `INF-017-U009` | RETAIN | 11 | 11 |
| `INF-017-U010` | RETAIN | 9 | 9 |
| `INF-017-U011` | RETAIN | 7 | 7 |
| `INF-017-U012` | RETAIN | 5 | 5 |
| `INF-017-U013` | RETAIN | 14 | 14 |
| `INF-017-U014` | RETAIN | 17 | 17 |
| `INF-017-U015` | RETAIN | 3 | 3 |
| `INF-017-U016` | RETAIN | 20 | 20 |
| `INF-017-U017` | RETAIN | E | E |
| `INF-017-U018` | RETAIN | 1F | 1F |
| `INF-017-U019` | RETAIN | D | D |
| `INF-017-U020` | RETAIN | 10 | 10 |
| `INF-017-U021` | RETAIN | 10 | 10 |
| `INF-017-U022` | RETAIN | 1 | 1 |
| `INF-017-U023` | RETAIN | 12 | 12 |
| `INF-017-U024` | RETAIN | 1 | 1 |
| `INF-017-U025` | RETAIN | 6 | 6 |
| `INF-017-U026` | RETAIN | 1 | 1 |
| `INF-017-U027` | RETAIN | 1 | 1 |
| `INF-017-U028` | RETAIN | eSIM/SIM | eSIM/SIM |
| `INF-017-U029` | RETAIN | CU | CU |
| `INF-017-U030` | RETAIN | ATM | ATM |
| `INF-017-U031` | RETAIN | P | P |
| `INF-017-U032` | RETAIN | 1F H | 1F H |
| `INF-017-U033` | RETAIN | 1 | 1 |
| `INF-017-U034` | RETAIN | 1 | 1 |
| `INF-017-U035` | RETAIN | 6, 7, 8 | 6, 7, 8 |
| `INF-017-U036` | RETAIN | 12, 13 | 12, 13 |
| `INF-017-U037` | RETAIN | P | P |
| `INF-017-U038` | RETAIN | A06 | A06 |
| `INF-017-U039` | RETAIN | eSIM/SIM | eSIM/SIM |
| `INF-017-U040` | RETAIN | CU | CU |
| `INF-017-U041` | RETAIN | CU, GS25 | CU, GS25 |
| `INF-017-U042` | RETAIN | ATM | ATM |
| `INF-017-U043` | TRANSLATE | Terminal 2 Arrival Map | Plan des arrivées du Terminal 2 |
| `INF-017-U044` | RETAIN | eSIM/SIM | eSIM/SIM |
| `INF-017-U045` | RETAIN | 201–230 | 201–230 |
| `INF-017-U046` | TRANSLATE | Gates 201–230 | Portes 201–230 |
| `INF-017-U047` | RETAIN | 231–270 | 231–270 |
| `INF-017-U048` | TRANSLATE | Gates 231–270 | Portes 231–270 |
| `INF-017-U049` | RETAIN | 20 | 20 |
| `INF-017-U050` | RETAIN | 17 | 17 |
| `INF-017-U051` | RETAIN | 15 | 15 |
| `INF-017-U052` | RETAIN | 10 | 10 |
| `INF-017-U053` | RETAIN | 8 | 8 |
| `INF-017-U054` | RETAIN | 6 | 6 |
| `INF-017-U055` | RETAIN | 2 | 2 |
| `INF-017-U056` | TRANSLATE | Quarantine Information | Informations de quarantaine |
| `INF-017-U057` | RETAIN | A | A |
| `INF-017-U058` | TRANSLATE | (Information) | (Informations) |
| `INF-017-U059` | TRANSLATE | (Medical Center) | (Centre médical) |
| `INF-017-U060` | TRANSLATE | (Restrooms) | (Toilettes) |
| `INF-017-U061` | TRANSLATE | (Currency Exchange) | (Bureau de change) |
| `INF-017-U062` | TRANSLATE | (Restrooms) | (Toilettes) |
| `INF-017-U063` | RETAIN | A | A |
| `INF-017-U064` | TRANSLATE | (Arrivals Hall A) | (Hall des arrivées A) |
| `INF-017-U065` | RETAIN | P | P |
| `INF-017-U066` | TRANSLATE | (Bus Ticket Counters) | (Guichets de billets de bus) |
| `INF-017-U067` | TRANSLATE | (Bus Stops) | (Arrêts de bus) |
| `INF-017-U068` | TRANSLATE | (Taxi Stands) | (Stations de taxis) |
| `INF-017-U069` | TRANSLATE | (Short-term Parking) | (Parking courte durée) |
| `INF-017-U070` | RETAIN | P | P |
| `INF-017-U071` | TRANSLATE | (Information) | (Informations) |
| `INF-017-U072` | TRANSLATE | (Medical Center) | (Centre médical) |
| `INF-017-U073` | TRANSLATE | (Restrooms) | (Toilettes) |
| `INF-017-U074` | TRANSLATE | (Currency Exchange) | (Bureau de change) |
| `INF-017-U075` | TRANSLATE | (Bus Stops) | (Arrêts de bus) |
| `INF-017-U076` | TRANSLATE | (Taxi Stands) | (Stations de taxis) |
| `INF-017-U077` | TRANSLATE | (Short-term Parking) | (Parking courte durée) |

### INF-018

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-018-U001` | TRANSLATE | eSIM for Korea | eSIM pour la Corée |
| `INF-018-U002` | TRANSLATE | Stay Connected in Korea | Restez connecté en Corée |
| `INF-018-U003` | TRANSLATE | Easy eSIM setup. Fast internet. | Installation facile de l’eSIM. Internet rapide. |
| `INF-018-U004` | TRANSLATE | Stay connected wherever you go. | Restez connecté où que vous alliez. |
| `INF-018-U005` | TRANSLATE | High Speed Data | Données haut débit |
| `INF-018-U006` | TRANSLATE | Voice Calls Available | Appels vocaux disponibles |
| `INF-018-U007` | TRANSLATE | SMS Supported | SMS pris en charge |
| `INF-018-U008` | TRANSLATE | Check Compatibility | Vérifiez la compatibilité |
| `INF-018-U009` | RETAIN | › | › |
| `INF-018-U010` | TRANSLATE | Compare Options | Comparez les options |
| `INF-018-U011` | RETAIN | › | › |
| `INF-018-U012` | RETAIN | 9:41 | 9:41 |
| `INF-018-U013` | RETAIN | KOREA | KOREA |
| `INF-018-U014` | RETAIN | eSIM | eSIM |
| `INF-018-U015` | TRANSLATE | Scan QR code to install eSIM | Scannez le QR code pour installer l’eSIM |
| `INF-018-U016` | RETAIN | eSIM | eSIM |
| `INF-018-U017` | TRANSLATE | Connected! | Connecté ! |

### INF-019

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-019-U001` | RETAIN | 1 | 1 |
| `INF-019-U002` | TRANSLATE | PHONE SUPPORTS ESIM? | VOTRE TÉLÉPHONE ACCEPTE L’ESIM ? |
| `INF-019-U003` | TRANSLATE | No → Physical SIM | Non → SIM physique |
| `INF-019-U004` | RETAIN | → | → |
| `INF-019-U005` | RETAIN | 2 | 2 |
| `INF-019-U006` | TRANSLATE | DATA ONLY? | DONNÉES UNIQUEMENT ? |
| `INF-019-U007` | TRANSLATE | Yes → Data-only travel eSIM | Oui → eSIM de voyage avec données uniquement |
| `INF-019-U008` | RETAIN | → | → |
| `INF-019-U009` | RETAIN | 3 | 3 |
| `INF-019-U010` | TRANSLATE | NEED A KOREAN NUMBER? | BESOIN D’UN NUMÉRO CORÉEN ? |
| `INF-019-U011` | TRANSLATE | Yes → Korean carrier tourist eSIM or SIM | Oui → eSIM touristique d’un opérateur coréen ou SIM |
| `INF-019-U012` | RETAIN | → | → |
| `INF-019-U013` | RETAIN | 4 | 4 |
| `INF-019-U014` | TRANSLATE | ROAMING INSTEAD? | PRÉFÉREZ LE ROAMING ? |
| `INF-019-U015` | TRANSLATE | Choose → Roaming | Choisissez → Roaming |

### INF-020

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-020-U001` | TRANSLATE | Airport arrival | Arrivée à l’aéroport |
| `INF-020-U002` | TRANSLATE | Step by step guide from landing to the city | Guide étape par étape de l’atterrissage jusqu’à la ville |
| `INF-020-U003` | RETAIN | 10:29 | 10:29 |
| `INF-020-U004` | RETAIN | Search here | Search here |
| `INF-020-U005` | RETAIN | Gyeongbokgung Palace | Gyeongbokgung Palace |
| `INF-020-U006` | RETAIN | Myeong-dong | Myeong-dong |
| `INF-020-U007` | RETAIN | N Seoul Tower | N Seoul Tower |
| `INF-020-U008` | TRANSLATE | Local maps | Cartes locales |
| `INF-020-U009` | TRANSLATE | Apps, navigation tips and must-know info | Applications, conseils de navigation et infos essentielles |
| `INF-020-U010` | RETAIN | CARD | CARD |
| `INF-020-U011` | RETAIN | Tmoney | Tmoney |
| `INF-020-U012` | RETAIN | T-money | T-money |
| `INF-020-U013` | TRANSLATE | How to buy, top up and use in Korea | Comment acheter, recharger et utiliser en Corée |
| `INF-020-U014` | RETAIN | WOWPASS | WOWPASS |
| `INF-020-U015` | RETAIN | W | W |
| `INF-020-U016` | RETAIN | WOWPASS | WOWPASS |
| `INF-020-U017` | RETAIN | T | T |
| `INF-020-U018` | RETAIN | Tmoney | Tmoney |
| `INF-020-U019` | RETAIN | W | W |
| `INF-020-U020` | RETAIN | WOWPASS | WOWPASS |
| `INF-020-U021` | TRANSLATE | Payment, balance, benefits and usage guide | Paiement, solde, avantages et guide d’utilisation |

### INF-021

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-021-U001` | TRANSLATE | Hongdae at a Glance | Hongdae en un coup d’œil |
| `INF-021-U002` | TRANSLATE | A simple guide to Yeonnam, central Hongdae, shopping, nightlife, and nearby neighborhoods | Guide simple de Yeonnam, du centre de Hongdae, du shopping, de la vie nocturne et des quartiers voisins |
| `INF-021-U003` | TRANSLATE | WEST | OUEST |
| `INF-021-U004` | TRANSLATE | NORTH | NORD |
| `INF-021-U005` | TRANSLATE | EAST | EST |
| `INF-021-U006` | RETAIN | Gyeongui Line Forest Park | Gyeongui Line Forest Park |
| `INF-021-U007` | RETAIN | Mangwon Station | Mangwon Station |
| `INF-021-U008` | TRANSLATE | (Line 6) | (Ligne 6) |
| `INF-021-U009` | TRANSLATE | Yeonnam Cafés | Cafés de Yeonnam |
| `INF-021-U010` | TRANSLATE | Mangwon Market & Local Eats | Marché de Mangwon & cuisine locale |
| `INF-021-U011` | RETAIN | Hongik Univ. Station | Hongik Univ. Station |
| `INF-021-U012` | TRANSLATE | (Line 2 · AREX · Gyeongui-Jungang) | (Ligne 2 · AREX · Gyeongui-Jungang) |
| `INF-021-U013` | TRANSLATE | Central Hongdae Shopping & Cafés | Shopping & cafés au centre de Hongdae |
| `INF-021-U014` | RETAIN | Seogyo-dong | Seogyo-dong |
| `INF-021-U015` | RETAIN | Hapjeong Station | Hapjeong Station |
| `INF-021-U016` | TRANSLATE | (Lines 2 & 6) | (Lignes 2 & 6) |
| `INF-021-U017` | TRANSLATE | Hapjeong Food & Stay | Restaurants & séjour à Hapjeong |
| `INF-021-U018` | TRANSLATE | Red Road Busking & Nightlife | Spectacles de rue & vie nocturne sur Red Road |
| `INF-021-U019` | RETAIN | Mapo-gu | Mapo-gu |
| `INF-021-U020` | RETAIN | Sangsu Station | Sangsu Station |
| `INF-021-U021` | TRANSLATE | (Line 6) | (Ligne 6) |
| `INF-021-U022` | RETAIN | Seogang Bridge | Seogang Bridge |
| `INF-021-U023` | TRANSLATE | Sangsu Indie Cafés & Bars | Cafés indépendants & bars à Sangsu |
| `INF-021-U024` | RETAIN | Han River | Han River |
| `INF-021-U025` | TRANSLATE | Editorial map — not to scale | Carte éditoriale — non à l’échelle |

### INF-022

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-022-U001` | TRANSLATE | WEST | OUEST |
| `INF-022-U002` | TRANSLATE | CENTER — MAIN CLUSTER | CENTRE — ZONE PRINCIPALE |
| `INF-022-U003` | TRANSLATE | EAST | EST |
| `INF-022-U004` | RETAIN | Jamsil Station | Jamsil Station |
| `INF-022-U005` | TRANSLATE | Lines 2 / 8 | Lignes 2 / 8 |
| `INF-022-U006` | RETAIN | Jamsil Sports Complex | Jamsil Sports Complex |
| `INF-022-U007` | RETAIN | Sports Complex Station | Sports Complex Station |
| `INF-022-U008` | TRANSLATE | Lines 2 / 9 | Lignes 2 / 9 |
| `INF-022-U009` | RETAIN | Lotte World Adventure | Lotte World Adventure |
| `INF-022-U010` | RETAIN | Lotte World Tower / Mall | Lotte World Tower / Mall |
| `INF-022-U011` | RETAIN | Lotte World Aquarium | Lotte World Aquarium |
| `INF-022-U012` | RETAIN | Olympic Park | Olympic Park |
| `INF-022-U013` | RETAIN | KSPO Dome | KSPO Dome |
| `INF-022-U014` | RETAIN | Olympic Park Station | Olympic Park Station |
| `INF-022-U015` | TRANSLATE | Lines 5 / 9 | Lignes 5 / 9 |
| `INF-022-U016` | RETAIN | Seokchon Lake | Seokchon Lake |
| `INF-022-U017` | RETAIN | Songridan-gil | Songridan-gil |
| `INF-022-U018` | TRANSLATE | Seokchon Lake east side | Côté est du lac Seokchon |
| `INF-022-U019` | TRANSLATE | Orientation map — not to scale | Carte d’orientation — non à l’échelle |

### INF-023

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-023-U001` | TRANSLATE | Seongsu at a Glance | Seongsu en un coup d’œil |
| `INF-023-U002` | TRANSLATE | A simple guide to pop-ups, cafés, hands-on beauty, and Seoul Forest | Guide simple des pop-ups, cafés, expériences beauté et de Seoul Forest |
| `INF-023-U003` | TRANSLATE | WEST | OUEST |
| `INF-023-U004` | TRANSLATE | NORTH | NORD |
| `INF-023-U005` | TRANSLATE | EAST | EST |
| `INF-023-U006` | RETAIN | Seoul Forest Station | Seoul Forest Station |
| `INF-023-U007` | RETAIN | (Suin-Bundang Line) | (Suin-Bundang Line) |
| `INF-023-U008` | RETAIN | Ttukseom Station | Ttukseom Station |
| `INF-023-U009` | TRANSLATE | (Line 2) | (Ligne 2) |
| `INF-023-U010` | RETAIN | Seongsu Station | Seongsu Station |
| `INF-023-U011` | TRANSLATE | (Line 2) | (Ligne 2) |
| `INF-023-U012` | TRANSLATE | Pop-Ups & Flagships | Pop-ups & boutiques phares |
| `INF-023-U013` | RETAIN | Yeonmujang-gil | Yeonmujang-gil |
| `INF-023-U014` | RETAIN | Seoul Forest | Seoul Forest |
| `INF-023-U015` | TRANSLATE | Cafés & Industrial Alleys | Cafés & ruelles industrielles |
| `INF-023-U016` | TRANSLATE | Hands-On Beauty | Expériences beauté |
| `INF-023-U017` | TRANSLATE | Editorial map — not to scale | Carte éditoriale — non à l’échelle |
| `INF-023-U018` | RETAIN | Han River | Han River |

### INF-024

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-024-U001` | TRANSLATE | How to Change NAVER Map to English | Comment passer NAVER Map en anglais |
| `INF-024-U002` | TRANSLATE | Use the app settings before your trip so menus are easier to read. | Réglez la langue de l’application avant le voyage pour lire les menus plus facilement. |
| `INF-024-U003` | TRANSLATE | Open the profile panel | Ouvrez le panneau de profil |
| `INF-024-U004` | RETAIN | SKT | SKT |
| `INF-024-U005` | RETAIN | 1:37 | 1:37 |
| `INF-024-U006` | RETAIN | 33 | 33 |
| `INF-024-U007` | RETAIN | Please log in. | Please log in. |
| `INF-024-U008` | RETAIN | Commute | Commute |
| `INF-024-U009` | RETAIN | Favorites | Favorites |
| `INF-024-U010` | RETAIN | Bus | Bus |
| `INF-024-U011` | RETAIN | Subway | Subway |
| `INF-024-U012` | RETAIN | N | N |
| `INF-024-U013` | RETAIN | Booking | Booking |
| `INF-024-U014` | RETAIN | Order | Order |
| `INF-024-U015` | RETAIN | Reviews | Reviews |
| `INF-024-U016` | RETAIN | Coupons | Coupons |
| `INF-024-U017` | RETAIN | Directions | Directions |
| `INF-024-U018` | RETAIN | Navigation | Navigation |
| `INF-024-U019` | RETAIN | Subway | Subway |
| `INF-024-U020` | RETAIN | Book train tickets | Book train tickets |
| `INF-024-U021` | RETAIN | My timeline | My timeline |
| `INF-024-U022` | RETAIN | 1 | 1 |
| `INF-024-U023` | TRANSLATE | Open the profile panel | Ouvrez le panneau de profil |
| `INF-024-U024` | RETAIN | → | → |
| `INF-024-U025` | TRANSLATE | Go to Language/언어 | Allez dans Language/언어 |
| `INF-024-U026` | RETAIN | SKT | SKT |
| `INF-024-U027` | RETAIN | 1:35 | 1:35 |
| `INF-024-U028` | RETAIN | 34 | 34 |
| `INF-024-U029` | RETAIN | Settings | Settings |
| `INF-024-U030` | RETAIN | Maps & Directions | Maps & Directions |
| `INF-024-U031` | RETAIN | Map settings | Map settings |
| `INF-024-U032` | RETAIN | Driving navigation | Driving navigation |
| `INF-024-U033` | RETAIN | Transit directions | Transit directions |
| `INF-024-U034` | RETAIN | Walking Directions | Walking Directions |
| `INF-024-U035` | RETAIN | Use my location as start | Use my location as start |
| `INF-024-U036` | RETAIN | Manage mobility data | Manage mobility data |
| `INF-024-U037` | RETAIN | App & Display | App & Display |
| `INF-024-U038` | RETAIN | Language/언어 | Language/언어 |
| `INF-024-U039` | RETAIN | English | English |
| `INF-024-U040` | RETAIN | Display theme | Display theme |
| `INF-024-U041` | RETAIN | Dark | Dark |
| `INF-024-U042` | RETAIN | Open with | Open with |
| `INF-024-U043` | RETAIN | Default | Default |
| `INF-024-U044` | RETAIN | Keep screen on | Keep screen on |
| `INF-024-U045` | RETAIN | i | i |
| `INF-024-U046` | RETAIN | Auto-rotate screen | Auto-rotate screen |
| `INF-024-U047` | RETAIN | i | i |
| `INF-024-U048` | TRANSLATE | Go to Language/언어 | Allez dans Language/언어 |
| `INF-024-U049` | RETAIN | → | → |
| `INF-024-U050` | TRANSLATE | Select English and tap OK | Sélectionnez English puis appuyez sur OK |
| `INF-024-U051` | RETAIN | SKT | SKT |
| `INF-024-U052` | RETAIN | 1:35 | 1:35 |
| `INF-024-U053` | RETAIN | 34 | 34 |
| `INF-024-U054` | RETAIN | Settings | Settings |
| `INF-024-U055` | RETAIN | Maps & Directions | Maps & Directions |
| `INF-024-U056` | RETAIN | Map settings | Map settings |
| `INF-024-U057` | RETAIN | Language | Language |
| `INF-024-U058` | RETAIN | Use the system language | Use the system language |
| `INF-024-U059` | RETAIN | English | English |
| `INF-024-U060` | RETAIN | Cancel | Cancel |
| `INF-024-U061` | RETAIN | OK | OK |
| `INF-024-U062` | RETAIN | Display theme | Display theme |
| `INF-024-U063` | RETAIN | Dark | Dark |
| `INF-024-U064` | RETAIN | Open with | Open with |
| `INF-024-U065` | RETAIN | Default | Default |
| `INF-024-U066` | RETAIN | Keep screen on | Keep screen on |
| `INF-024-U067` | RETAIN | i | i |
| `INF-024-U068` | RETAIN | Auto-rotate screen | Auto-rotate screen |
| `INF-024-U069` | RETAIN | i | i |
| `INF-024-U070` | TRANSLATE | Select English and tap OK | Sélectionnez English puis appuyez sur OK |
| `INF-024-U071` | TRANSLATE | Menu labels may vary slightly by app version and device. | Les libellés de menu peuvent varier légèrement selon la version de l’application et l’appareil. |

### INF-025

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-025-U001` | TRANSLATE | How to Search and Confirm the Right Place | Comment rechercher et confirmer le bon lieu |
| `INF-025-U002` | TRANSLATE | Compare similar results before you choose a station, branch or entrance. | Comparez les résultats similaires avant de choisir une station, une agence ou une entrée. |
| `INF-025-U003` | TRANSLATE | 1. Type the English name | 1. Saisissez le nom en anglais |
| `INF-025-U004` | RETAIN | SKT | SKT |
| `INF-025-U005` | RETAIN | 1:45 | 1:45 |
| `INF-025-U006` | RETAIN | 31 | 31 |
| `INF-025-U007` | RETAIN | seoul station | seoul station |
| `INF-025-U008` | RETAIN | seoulstationline1 | seoulstationline1 |
| `INF-025-U009` | RETAIN | Seoul station (High-Speed Train) | Seoul station (High-Speed Train) |
| `INF-025-U010` | RETAIN | 43-205 Dongja-dong Yongsan-gu Seoul | 43-205 Dongja-dong Yongsan-gu Seoul |
| `INF-025-U011` | RETAIN | KTX,SRT stations | KTX,SRT stations |
| `INF-025-U012` | RETAIN | Seoul Station station Line1 | Seoul Station station Line1 |
| `INF-025-U013` | RETAIN | 73-6 Namdaemunno 5(o)-ga Jung-gu Seoul | 73-6 Namdaemunno 5(o)-ga Jung-gu Seoul |
| `INF-025-U014` | RETAIN | Metropolitan Line 1 | Metropolitan Line 1 |
| `INF-025-U015` | RETAIN | Seoul Station station Airport Railroad | Seoul Station station Airport Railroad |
| `INF-025-U016` | RETAIN | 43-205 Dongja-dong Yongsan-gu Seoul | 43-205 Dongja-dong Yongsan-gu Seoul |
| `INF-025-U017` | RETAIN | Airport train | Airport train |
| `INF-025-U018` | RETAIN | Seoul-forest station SuinBundang Line | Seoul-forest station SuinBundang Line |
| `INF-025-U019` | RETAIN | 656-436 Seongsu-dong 1(il)-ga | 656-436 Seongsu-dong 1(il)-ga |
| `INF-025-U020` | RETAIN | Seongdong-gu Seoul | Seongdong-gu Seoul |
| `INF-025-U021` | RETAIN | Suin-bundang line | Suin-bundang line |
| `INF-025-U022` | TRANSLATE | Type the English name | Saisissez le nom en anglais |
| `INF-025-U023` | RETAIN | → | → |
| `INF-025-U024` | TRANSLATE | 2. Compare similar results | 2. Comparez les résultats similaires |
| `INF-025-U025` | RETAIN | SKT | SKT |
| `INF-025-U026` | RETAIN | 1:46 | 1:46 |
| `INF-025-U027` | RETAIN | 31 | 31 |
| `INF-025-U028` | RETAIN | seoul station | seoul station |
| `INF-025-U029` | RETAIN | Places | Places |
| `INF-025-U030` | RETAIN | Buses | Buses |
| `INF-025-U031` | RETAIN | Stops | Stops |
| `INF-025-U032` | RETAIN | Bongrae BBQ | Bongrae BBQ |
| `INF-025-U033` | RETAIN | Ongsimi Seoul Station Branch | Ongsimi Seoul Station Branch |
| `INF-025-U034` | RETAIN | FOCALPOINT | FOCALPOINT |
| `INF-025-U035` | RETAIN | Syugaseukeol Seoul Station Branch | Syugaseukeol Seoul Station Branch |
| `INF-025-U036` | RETAIN | Seoul station (Hi | Seoul station (Hi |
| `INF-025-U037` | RETAIN | Speed Train) | Speed Train) |
| `INF-025-U038` | RETAIN | Seoul Station station Airport Railroad | Seoul Station station Airport Railroad |
| `INF-025-U039` | RETAIN | Seoul Station station Line4 | Seoul Station station Line4 |
| `INF-025-U040` | RETAIN | UPPERLINE | UPPERLINE |
| `INF-025-U041` | RETAIN | GS Caltex | GS Caltex |
| `INF-025-U042` | RETAIN | GS칼텍스 | GS칼텍스 |
| `INF-025-U043` | RETAIN | matsudo seoul | matsudo seoul |
| `INF-025-U044` | RETAIN | Map centered | Map centered |
| `INF-025-U045` | RETAIN | Relevance | Relevance |
| `INF-025-U046` | RETAIN | Seoul Station station Airport Railroad | Seoul Station station Airport Railroad |
| `INF-025-U047` | RETAIN | Subway | Subway |
| `INF-025-U048` | RETAIN | Yongsan-gu Seoul | Yongsan-gu Seoul |
| `INF-025-U049` | RETAIN | Call | Call |
| `INF-025-U050` | RETAIN | Get Directions | Get Directions |
| `INF-025-U051` | RETAIN | Seoul station (High-Speed Train) | Seoul station (High-Speed Train) |
| `INF-025-U052` | RETAIN | KTX,SRT stations | KTX,SRT stations |
| `INF-025-U053` | RETAIN | Open · Closes at 24:00 | Open · Closes at 24:00 |
| `INF-025-U054` | RETAIN | Yongsan-gu Seoul | Yongsan-gu Seoul |
| `INF-025-U055` | RETAIN | Call | Call |
| `INF-025-U056` | RETAIN | Get Directions | Get Directions |
| `INF-025-U057` | RETAIN | Seoul Station station Line1 | Seoul Station station Line1 |
| `INF-025-U058` | RETAIN | Subway | Subway |
| `INF-025-U059` | RETAIN | Jung-gu Seoul | Jung-gu Seoul |
| `INF-025-U060` | TRANSLATE | Compare similar results | Comparez les résultats similaires |
| `INF-025-U061` | RETAIN | → | → |
| `INF-025-U062` | TRANSLATE | Check before you choose | Vérifiez avant de choisir |
| `INF-025-U063` | TRANSLATE | Match the line or place type | Vérifiez la ligne ou le type de lieu |
| `INF-025-U064` | TRANSLATE | Check whether it’s Subway, KTX/SRT, Airport Railroad, etc. | Vérifiez s’il s’agit du métro, du KTX/SRT, de l’Airport Railroad, etc. |
| `INF-025-U065` | TRANSLATE | Compare the station name carefully | Comparez attentivement le nom de la station |
| `INF-025-U066` | TRANSLATE | Names can be very similar. Check each result. | Les noms peuvent être très proches. Vérifiez chaque résultat. |
| `INF-025-U067` | TRANSLATE | Use the Korean name or address if needed | Utilisez le nom ou l’adresse en coréen si nécessaire |
| `INF-025-U068` | TRANSLATE | It helps you find the exact place or branch. | Cela aide à trouver le lieu ou l’agence exacts. |
| `INF-025-U069` | TRANSLATE | Do not rely on one result only | Ne vous fiez pas à un seul résultat |
| `INF-025-U070` | TRANSLATE | Always compare a few options before you decide. | Comparez toujours plusieurs options avant de décider. |
| `INF-025-U071` | TRANSLATE | If English search fails | Si la recherche en anglais échoue |
| `INF-025-U072` | TRANSLATE | Find the Korean name | Trouvez le nom en coréen |
| `INF-025-U073` | TRANSLATE | Search in Korean or ask locally. | Recherchez en coréen ou demandez sur place. |
| `INF-025-U074` | RETAIN | → | → |
| `INF-025-U075` | TRANSLATE | Paste it into NAVER Map | Collez-le dans NAVER Map |
| `INF-025-U076` | TRANSLATE | Open NAVER Map and paste the name. | Ouvrez NAVER Map et collez le nom. |
| `INF-025-U077` | RETAIN | → | → |
| `INF-025-U078` | TRANSLATE | Compare the results again | Comparez de nouveau les résultats |
| `INF-025-U079` | TRANSLATE | Review the options and choose carefully. | Examinez les options et choisissez avec attention. |
| `INF-025-U080` | TRANSLATE | Always confirm the exact station or branch before you go. | Confirmez toujours la station ou l’agence exacte avant de partir. |

### INF-026

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-026-U001` | TRANSLATE | How to Check Routes, Subway Exits and Bus Stops | Comment vérifier les itinéraires, sorties de métro et arrêts de bus |
| `INF-026-U002` | TRANSLATE | Use the exact place result, compare route types, and confirm where to get on and get off. | Utilisez le résultat exact, comparez les types d’itinéraires et confirmez où monter et où descendre. |
| `INF-026-U003` | TRANSLATE | Search the exact place | Recherchez le lieu exact |
| `INF-026-U004` | TRANSLATE | Terminal 1 vs bus stop vs nearby stores | Terminal 1 vs arrêt de bus vs commerces voisins |
| `INF-026-U005` | RETAIN | incheon airport t1 | incheon airport t1 |
| `INF-026-U006` | RETAIN | incheonairportt1 | incheonairportt1 |
| `INF-026-U007` | RETAIN | Incheon Airport (Terminal 1) | Incheon Airport (Terminal 1) |
| `INF-026-U008` | RETAIN | 2851 Unseo-dong Yeongjong-gu Incheon | 2851 Unseo-dong Yeongjong-gu Incheon |
| `INF-026-U009` | RETAIN | 146km · Rent a car · Reviews 14 | 146km · Rent a car · Reviews 14 |
| `INF-026-U010` | RETAIN | Incheon Int’l Airport T1 Bus Stop | Incheon Int’l Airport T1 Bus Stop |
| `INF-026-U011` | RETAIN | 2851 Unseo-dong Yeongjong-gu Incheon | 2851 Unseo-dong Yeongjong-gu Incheon |
| `INF-026-U012` | RETAIN | 146km · Bus, Stop | 146km · Bus, Stop |
| `INF-026-U013` | RETAIN | Starbucks Incheon International Airport T1 Air 4F Branch | Starbucks Incheon International Airport T1 Air 4F Branch |
| `INF-026-U014` | RETAIN | 2840 Unseo-dong Yeongjong-gu Incheon | 2840 Unseo-dong Yeongjong-gu Incheon |
| `INF-026-U015` | RETAIN | 146km · Cafe · Reviews 637 | 146km · Cafe · Reviews 637 |
| `INF-026-U016` | RETAIN | SHAKE SHACK INCHEON AIRPORT T1 | SHAKE SHACK INCHEON AIRPORT T1 |
| `INF-026-U017` | RETAIN | 2840 Unseo-dong Yeongjong-gu Incheon | 2840 Unseo-dong Yeongjong-gu Incheon |
| `INF-026-U018` | RETAIN | 146km · hamburger · Reviews 999+ | 146km · hamburger · Reviews 999+ |
| `INF-026-U019` | TRANSLATE | Airport Railroad vs Line 2 vs other places | Airport Railroad vs ligne 2 vs autres lieux |
| `INF-026-U020` | RETAIN | hongikuniv.station | hongikuniv.station |
| `INF-026-U021` | RETAIN | Places | Places |
| `INF-026-U022` | RETAIN | Stops | Stops |
| `INF-026-U023` | RETAIN | On the map | On the map |
| `INF-026-U024` | RETAIN | Relevance | Relevance |
| `INF-026-U025` | RETAIN | Hongik Univ. station Airport Railroad | Hongik Univ. station Airport Railroad |
| `INF-026-U026` | RETAIN | 137km · Subway | 137km · Subway |
| `INF-026-U027` | RETAIN | 172-9 Donggyo-dong Mapo-gu Seoul | 172-9 Donggyo-dong Mapo-gu Seoul |
| `INF-026-U028` | RETAIN | 1599-7788 | 1599-7788 |
| `INF-026-U029` | RETAIN | 11 entrance(s) | 11 entrance(s) |
| `INF-026-U030` | RETAIN | Hongik Univ. station Line2 | Hongik Univ. station Line2 |
| `INF-026-U031` | RETAIN | 137km · Subway | 137km · Subway |
| `INF-026-U032` | RETAIN | 165 Donggyo-dong Mapo-gu Seoul | 165 Donggyo-dong Mapo-gu Seoul |
| `INF-026-U033` | RETAIN | 02-6110-2391 | 02-6110-2391 |
| `INF-026-U034` | RETAIN | 10 entrance(s) | 10 entrance(s) |
| `INF-026-U035` | RETAIN | Hongik Univ. station Gyeongui-Jungang Line | Hongik Univ. station Gyeongui-Jungang Line |
| `INF-026-U036` | RETAIN | 137km · Subway | 137km · Subway |
| `INF-026-U037` | RETAIN | 190-66 Donggyo-dong Mapo-gu Seoul | 190-66 Donggyo-dong Mapo-gu Seoul |
| `INF-026-U038` | RETAIN | 1588-7788 | 1588-7788 |
| `INF-026-U039` | TRANSLATE | Confirm the line, place type, and Korean address before you continue. | Confirmez la ligne, le type de lieu et l’adresse coréenne avant de continuer. |
| `INF-026-U040` | TRANSLATE | Compare the route options | Comparez les options d’itinéraire |
| `INF-026-U041` | RETAIN | Incheon Airport (Terminal 1) | Incheon Airport (Terminal 1) |
| `INF-026-U042` | RETAIN | Hongik Univ. station Airport Railroad | Hongik Univ. station Airport Railroad |
| `INF-026-U043` | RETAIN | Entrances | Entrances |
| `INF-026-U044` | RETAIN | Entrance | Entrance |
| `INF-026-U045` | RETAIN | 59min | 59min |
| `INF-026-U046` | RETAIN | All | All |
| `INF-026-U047` | RETAIN | Bus 2 | Bus 2 |
| `INF-026-U048` | RETAIN | Subway 1 | Subway 1 |
| `INF-026-U049` | RETAIN | Bus+Subway 2 | Bus+Subway 2 |
| `INF-026-U050` | RETAIN | Dep. Today 13:52 | Dep. Today 13:52 |
| `INF-026-U051` | RETAIN | Best route, Include stairs | Best route, Include stairs |
| `INF-026-U052` | RETAIN | Best | Best |
| `INF-026-U053` | RETAIN | 59min | 59min |
| `INF-026-U054` | RETAIN | 1:58 PM - 2:57 PM | 1:58 PM - 2:57 PM |
| `INF-026-U055` | RETAIN | ₩4,650 | ₩4,650 |
| `INF-026-U056` | RETAIN | 4m | 4m |
| `INF-026-U057` | RETAIN | 53m | 53m |
| `INF-026-U058` | RETAIN | Airport | Airport |
| `INF-026-U059` | RETAIN | Incheon Int’l Airport Terminal... | Incheon Int’l Airport Terminal... |
| `INF-026-U060` | RETAIN | Real | Real |
| `INF-026-U061` | RETAIN | Time | Time |
| `INF-026-U062` | RETAIN | 1min | 1min |
| `INF-026-U063` | RETAIN | Seoul Station bound \| Incheon Int’l Airport C... | Seoul Station bound \| Incheon Int’l Airport C... |
| `INF-026-U064` | RETAIN | Get off | Get off |
| `INF-026-U065` | RETAIN | Hongik Univ. Station | Hongik Univ. Station |
| `INF-026-U066` | RETAIN | GO | GO |
| `INF-026-U067` | RETAIN | Fastest · Short transfer · Less walk | Fastest · Short transfer · Less walk |
| `INF-026-U068` | RETAIN | 57min | 57min |
| `INF-026-U069` | RETAIN | 1:54 PM - 2:51 PM | 1:54 PM - 2:51 PM |
| `INF-026-U070` | RETAIN | ₩17,000 | ₩17,000 |
| `INF-026-U071` | RETAIN | 56m | 56m |
| `INF-026-U072` | RETAIN | 11 | 11 |
| `INF-026-U073` | RETAIN | Airport | Airport |
| `INF-026-U074` | RETAIN | Incheon Airport Arrival Lobby (1st Fl... | Incheon Airport Arrival Lobby (1st Fl... |
| `INF-026-U075` | RETAIN | ETA | ETA |
| `INF-026-U076` | RETAIN | Bus 6011 | Bus 6011 |
| `INF-026-U077` | RETAIN | 1h 9min | 1h 9min |
| `INF-026-U078` | RETAIN | 2:02 PM - 3:11 PM | 2:02 PM - 3:11 PM |
| `INF-026-U079` | RETAIN | ₩6,600 | ₩6,600 |
| `INF-026-U080` | RETAIN | 6011 | 6011 |
| `INF-026-U081` | RETAIN | 1h 9m | 1h 9m |
| `INF-026-U082` | TRANSLATE | Best route | Meilleur itinéraire |
| `INF-026-U083` | TRANSLATE | Fastest route | Itinéraire le plus rapide |
| `INF-026-U084` | TRANSLATE | Another bus option | Autre option en bus |
| `INF-026-U085` | TRANSLATE | Check total time, fare, and transfer type — not just the first result. | Vérifiez le temps total, le tarif et le type de correspondance — pas seulement le premier résultat. |
| `INF-026-U086` | TRANSLATE | Open the route details | Ouvrez les détails de l’itinéraire |
| `INF-026-U087` | TRANSLATE | Fastest bus option | Option de bus la plus rapide |
| `INF-026-U088` | RETAIN | 6002 | 6002 |
| `INF-026-U089` | RETAIN | Airport | Airport |
| `INF-026-U090` | RETAIN | No ETA | No ETA |
| `INF-026-U091` | RETAIN | Past timetable | Past timetable |
| `INF-026-U092` | RETAIN | View more | View more |
| `INF-026-U093` | RETAIN | Ride 3 stop(s) | Ride 3 stop(s) |
| `INF-026-U094` | RETAIN | 56min | 56min |
| `INF-026-U095` | RETAIN | Get off at Hongdae Entrance | Get off at Hongdae Entrance |
| `INF-026-U096` | RETAIN | 14801 | 14801 |
| `INF-026-U097` | RETAIN | Walk 54m · 1min | Walk 54m · 1min |
| `INF-026-U098` | RETAIN | Hongik Univ. Station Seoul Metropolitan Area Airport Railroad | Hongik Univ. Station Seoul Metropolitan Area Airport Railroad |
| `INF-026-U099` | RETAIN | 9 | 9 |
| `INF-026-U100` | RETAIN | 57min | 57min |
| `INF-026-U101` | RETAIN | Arr. at 2:51 PM | Arr. at 2:51 PM |
| `INF-026-U102` | RETAIN | Preview | Preview |
| `INF-026-U103` | RETAIN | GO | GO |
| `INF-026-U104` | TRANSLATE | Boarding point | Point d’embarquement |
| `INF-026-U105` | TRANSLATE | Get off stop | Arrêt de descente |
| `INF-026-U106` | TRANSLATE | Final walk | Dernière marche |
| `INF-026-U107` | TRANSLATE | Best AREX option | Meilleure option AREX |
| `INF-026-U108` | RETAIN | Airport Railroad Get on at Incheon Int’l Airport Terminal 1 Station | Airport Railroad Get on at Incheon Int’l Airport Terminal 1 Station |
| `INF-026-U109` | RETAIN | 14:03 | 14:03 |
| `INF-026-U110` | RETAIN | Incheon Int’l Airport Cargo Termi... | Incheon Int’l Airport Cargo Termi... |
| `INF-026-U111` | RETAIN | Fast arrival: 1-2, 5-4 | Fast arrival: 1-2, 5-4 |
| `INF-026-U112` | RETAIN | Real | Real |
| `INF-026-U113` | RETAIN | Time | Time |
| `INF-026-U114` | RETAIN | 14:03 | 14:03 |
| `INF-026-U115` | RETAIN | Seoul Station bound | Seoul Station bound |
| `INF-026-U116` | RETAIN | Ride 10 stop(s) | Ride 10 stop(s) |
| `INF-026-U117` | RETAIN | 53min | 53min |
| `INF-026-U118` | RETAIN | Get off at Hongik Univ. Station | Get off at Hongik Univ. Station |
| `INF-026-U119` | RETAIN | 11 | 11 |
| `INF-026-U120` | RETAIN | Door on Left | Door on Left |
| `INF-026-U121` | RETAIN | 59min | 59min |
| `INF-026-U122` | RETAIN | Arr. at 2:57 PM | Arr. at 2:57 PM |
| `INF-026-U123` | RETAIN | Preview | Preview |
| `INF-026-U124` | RETAIN | GO | GO |
| `INF-026-U125` | TRANSLATE | Boarding point | Point d’embarquement |
| `INF-026-U126` | TRANSLATE | Get off stop | Arrêt de descente |
| `INF-026-U127` | TRANSLATE | Correct place | Bon lieu |
| `INF-026-U128` | TRANSLATE | Choose the exact place result. | Choisissez le résultat correspondant exactement au lieu. |
| `INF-026-U129` | TRANSLATE | Route type | Type d’itinéraire |
| `INF-026-U130` | TRANSLATE | Pick the route type that fits you. | Choisissez le type d’itinéraire qui vous convient. |
| `INF-026-U131` | TRANSLATE | Get-off stop | Arrêt de descente |
| `INF-026-U132` | TRANSLATE | Confirm where to get off. | Confirmez où descendre. |
| `INF-026-U133` | TRANSLATE | Final walk | Dernière marche |
| `INF-026-U134` | TRANSLATE | Check the last walk to your destination. | Vérifiez la dernière marche jusqu’à votre destination. |
| `INF-026-U135` | TRANSLATE | Screens can vary by app version and device. | Les écrans peuvent varier selon la version de l’application et l’appareil. |

### INF-027

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-027-U001` | TRANSLATE | How to Buy & Recharge T-money in Korea | Comment acheter et recharger T-money en Corée |
| `INF-027-U002` | TRANSLATE | It’s easy! You can buy T-money at convenience stores or subway stations, and recharge it anytime. | C’est simple ! Vous pouvez acheter T-money dans les supérettes ou les stations de métro et la recharger à tout moment. |
| `INF-027-U003` | TRANSLATE | TIP | ASTUCE |
| `INF-027-U004` | TRANSLATE | If you plan to use public transportation, getting a T-money card is cheaper and more convenient than single-journey tickets. | Si vous comptez utiliser les transports publics, une carte T-money revient moins cher et est plus pratique que les billets à l’unité. |
| `INF-027-U005` | TRANSLATE | 1. How to Buy | 1. Comment acheter |
| `INF-027-U006` | RETAIN | GS25 | GS25 |
| `INF-027-U007` | RETAIN | T | T |
| `INF-027-U008` | RETAIN | money | money |
| `INF-027-U009` | RETAIN | T | T |
| `INF-027-U010` | RETAIN | money | money |
| `INF-027-U011` | TRANSLATE | Convenience Stores | Supérettes |
| `INF-027-U012` | TRANSLATE | Buy at CU, GS25, 7-Eleven and other stores. | Achetez-la chez CU, GS25, 7-Eleven et dans d’autres commerces. |
| `INF-027-U013` | TRANSLATE | Price: ₩2,500~₩4,000 | Prix : ₩2,500~₩4,000 |
| `INF-027-U014` | RETAIN | Transportation Card | Transportation Card |
| `INF-027-U015` | RETAIN | T | T |
| `INF-027-U016` | RETAIN | money | money |
| `INF-027-U017` | TRANSLATE | Subway Station Kiosk | Borne en station de métro |
| `INF-027-U018` | TRANSLATE | Vending machines are available in most subway stations. | Des distributeurs sont disponibles dans la plupart des stations de métro. |
| `INF-027-U019` | TRANSLATE | Price: ₩2,500~₩4,000 | Prix : ₩2,500~₩4,000 |
| `INF-027-U020` | RETAIN | Tmoney | Tmoney |
| `INF-027-U021` | RETAIN | Transportation Card | Transportation Card |
| `INF-027-U022` | RETAIN | AREX | AREX |
| `INF-027-U023` | TRANSLATE | You can also buy T-money at Incheon and Gimpo Airport. | Vous pouvez aussi acheter T-money aux aéroports d’Incheon et de Gimpo. |
| `INF-027-U024` | TRANSLATE | Price: ₩3,000~₩4,000 | Prix : ₩3,000~₩4,000 |
| `INF-027-U025` | TRANSLATE | 2. How to Recharge | 2. Comment recharger |
| `INF-027-U026` | RETAIN | 1 | 1 |
| `INF-027-U027` | TRANSLATE | Place your card on the reader. | Placez votre carte sur le lecteur. |
| `INF-027-U028` | RETAIN | Card Reload Device | Card Reload Device |
| `INF-027-U029` | RETAIN | T | T |
| `INF-027-U030` | RETAIN | money | money |
| `INF-027-U031` | RETAIN | Card Reload Device | Card Reload Device |
| `INF-027-U032` | RETAIN | 2 | 2 |
| `INF-027-U033` | TRANSLATE | Select the amount to recharge. | Sélectionnez le montant à recharger. |
| `INF-027-U034` | RETAIN | Please select the amount. | Please select the amount. |
| `INF-027-U035` | RETAIN | 1,000 | 1,000 |
| `INF-027-U036` | RETAIN | 2,000 | 2,000 |
| `INF-027-U037` | RETAIN | 3,000 | 3,000 |
| `INF-027-U038` | RETAIN | 5,000 | 5,000 |
| `INF-027-U039` | RETAIN | 10,000 | 10,000 |
| `INF-027-U040` | RETAIN | 20,000 | 20,000 |
| `INF-027-U041` | RETAIN | 30,000 | 30,000 |
| `INF-027-U042` | RETAIN | 30,000 | 30,000 |
| `INF-027-U043` | RETAIN | 50,000 | 50,000 |
| `INF-027-U044` | RETAIN | Previous | Previous |
| `INF-027-U045` | RETAIN | To the beginning | To the beginning |
| `INF-027-U046` | RETAIN | 3 | 3 |
| `INF-027-U047` | TRANSLATE | Pay in cash or by card. | Payez en espèces ou par carte. |
| `INF-027-U048` | RETAIN | Cash | Cash |
| `INF-027-U049` | RETAIN | T | T |
| `INF-027-U050` | RETAIN | money | money |
| `INF-027-U051` | TRANSLATE | 3. How to Use | 3. Comment utiliser |
| `INF-027-U052` | TRANSLATE | Subway (Enter & Exit) | Métro (entrée & sortie) |
| `INF-027-U053` | TRANSLATE | Tap your card when you enter and exit the station. (Missing the exit tap will charge the maximum fare.) | Validez votre carte à l’entrée et à la sortie de la station. (Sans validation à la sortie, le tarif maximal sera facturé.) |
| `INF-027-U054` | TRANSLATE | Bus (Board & Get Off) | Bus (montée & descente) |
| `INF-027-U055` | RETAIN | 1,450 | 1,450 |
| `INF-027-U056` | RETAIN | Tmoney | Tmoney |
| `INF-027-U057` | TRANSLATE | Tap when boarding and tap again when getting off. | Validez en montant puis de nouveau en descendant. |
| `INF-027-U058` | TRANSLATE | Transfer Discount | Réduction de correspondance |
| `INF-027-U059` | RETAIN | ↔ | ↔ |
| `INF-027-U060` | RETAIN | 30min | 30min |
| `INF-027-U061` | TRANSLATE | Free transfers within 30 minutes between bus and subway. | Correspondances gratuites dans les 30 minutes entre bus et métro. |
| `INF-027-U062` | TRANSLATE | Taxi | Taxi |
| `INF-027-U063` | RETAIN | 4,900 | 4,900 |
| `INF-027-U064` | RETAIN | Tmoney | Tmoney |
| `INF-027-U065` | TRANSLATE | Many taxis accept T-money. Ask the driver first: “T-money gayo?” (Can I use T-money?) | De nombreux taxis acceptent T-money. Demandez d’abord au chauffeur : « T-money gayo? » (Puis-je utiliser T-money ?) |
| `INF-027-U066` | TRANSLATE | Check Your Balance | Vérifiez votre solde |
| `INF-027-U067` | TRANSLATE | Tap your card on any subway gate or recharge machine to check your balance. | Posez votre carte sur un portique de métro ou une borne de recharge pour vérifier le solde. |
| `INF-027-U068` | RETAIN | Balance | Balance |
| `INF-027-U069` | RETAIN | ₩ 12,350 | ₩ 12,350 |
| `INF-027-U070` | TRANSLATE | Refund | Remboursement |
| `INF-027-U071` | TRANSLATE | Get a refund for the remaining balance (minus a small fee of around ₩500) at subway customer centers or convenience stores. | Faites rembourser le solde restant (moins de petits frais d’environ ₩500) dans un centre de service du métro ou une supérette. |
| `INF-027-U072` | RETAIN | Tmoney | Tmoney |
| `INF-027-U073` | RETAIN | Information | Information |

### INF-029

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-029-U001` | TRANSLATE | T-money Recharge Machine | Borne de recharge T-money |
| `INF-029-U002` | TRANSLATE | Choose English • Place card • Add cash • Check balance | Choisir English • Poser la carte • Ajouter des espèces • Vérifier le solde |
| `INF-029-U003` | TRANSLATE | CARD AREA | ZONE CARTE |
| `INF-029-U004` | TRANSLATE | CASH SLOT | FENTE À BILLETS |
| `INF-029-U005` | TRANSLATE | Tip: If a machine does not accept foreign cards, use cash or recharge at a convenience store. | Conseil : si une borne n’accepte pas les cartes étrangères, utilisez des espèces ou rechargez dans une supérette. |

### INF-031

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-031-U001` | RETAIN | wowpass-card.png | wowpass-card.png |
| `INF-031-U002` | RETAIN | W | W |
| `INF-031-U003` | RETAIN | WOWPASS | WOWPASS |
| `INF-031-U004` | RETAIN | T | T |
| `INF-031-U005` | RETAIN | Tmoney | Tmoney |
| `INF-031-U006` | RETAIN | PREPAID CARD | PREPAID CARD |
| `INF-031-U007` | TRANSLATE | WOWPASS All-in-One Prepaid Card for Travelers | WOWPASS, carte prépayée tout-en-un pour les voyageurs |
| `INF-031-U008` | TRANSLATE | Transportation | Transports |
| `INF-031-U009` | TRANSLATE | Use on subway, bus, AREX and more | Utilisez-la dans le métro, le bus, l’AREX et plus encore |
| `INF-031-U010` | TRANSLATE | Payments | Paiements |
| `INF-031-U011` | TRANSLATE | Pay at stores, cafés, and convenience stores | Payez dans les magasins, cafés et supérettes |
| `INF-031-U012` | RETAIN | $ ↔ ₩ | $ ↔ ₩ |
| `INF-031-U013` | TRANSLATE | Currency Exchange | Change de devises |
| `INF-031-U014` | TRANSLATE | Exchange foreign currency and use in Korea | Changez des devises étrangères et utilisez les fonds en Corée |
| `INF-031-U015` | RETAIN | wowpass-machine.png | wowpass-machine.png |
| `INF-031-U016` | RETAIN | WOWPASS | WOWPASS |
| `INF-031-U017` | RETAIN | WOWPASS | WOWPASS |
| `INF-031-U018` | RETAIN | ALL-IN-ONE PREPAID CARD | ALL-IN-ONE PREPAID CARD |
| `INF-031-U019` | RETAIN | TOUCH TO START | TOUCH TO START |
| `INF-031-U020` | RETAIN | ENGLISH | ENGLISH |
| `INF-031-U021` | RETAIN | WOWPASS CARD SALES & TOP-UP | WOWPASS CARD SALES & TOP-UP |
| `INF-031-U022` | RETAIN | RECEIPT | RECEIPT |
| `INF-031-U023` | RETAIN | CARD | CARD |
| `INF-031-U024` | RETAIN | CASH (KRW) | CASH (KRW) |
| `INF-031-U025` | RETAIN | W | W |
| `INF-031-U026` | RETAIN | WOWPASS | WOWPASS |
| `INF-031-U027` | RETAIN | All-in-One Prepaid Card for Travelers in Korea | All-in-One Prepaid Card for Travelers in Korea |
| `INF-031-U028` | RETAIN | $ ↔ ₩ | $ ↔ ₩ |
| `INF-031-U029` | RETAIN | wowpass-use-flow.png | wowpass-use-flow.png |
| `INF-031-U030` | RETAIN | 1 | 1 |
| `INF-031-U031` | TRANSLATE | Get Card | Obtenir la carte |
| `INF-031-U032` | RETAIN | W | W |
| `INF-031-U033` | RETAIN | WOWPASS | WOWPASS |
| `INF-031-U034` | RETAIN | T | T |
| `INF-031-U035` | RETAIN | Tmoney | Tmoney |
| `INF-031-U036` | RETAIN | PREPAID CARD | PREPAID CARD |
| `INF-031-U037` | RETAIN | W | W |
| `INF-031-U038` | TRANSLATE | Get your WOWPASS card at airport machines or partner locations. | Récupérez votre carte WOWPASS aux bornes de l’aéroport ou chez les partenaires. |
| `INF-031-U039` | RETAIN | → | → |
| `INF-031-U040` | RETAIN | 2 | 2 |
| `INF-031-U041` | TRANSLATE | Load Money / Exchange Currency | Charger de l’argent / changer des devises |
| `INF-031-U042` | RETAIN | $ | $ |
| `INF-031-U043` | RETAIN | € | € |
| `INF-031-U044` | RETAIN | ¥ | ¥ |
| `INF-031-U045` | RETAIN | → | → |
| `INF-031-U046` | RETAIN | ₩ | ₩ |
| `INF-031-U047` | TRANSLATE | Load Korean won (KRW) or exchange foreign currency onto your card. | Chargez des wons coréens (KRW) ou changez des devises étrangères sur votre carte. |
| `INF-031-U048` | RETAIN | → | → |
| `INF-031-U049` | RETAIN | 3 | 3 |
| `INF-031-U050` | TRANSLATE | Pay | Payer |
| `INF-031-U051` | RETAIN | W | W |
| `INF-031-U052` | RETAIN | WOWPASS | WOWPASS |
| `INF-031-U053` | RETAIN | T | T |
| `INF-031-U054` | RETAIN | Tmoney | Tmoney |
| `INF-031-U055` | TRANSLATE | Use your card to pay at stores, cafés and convenience stores. | Utilisez votre carte pour payer dans les magasins, cafés et supérettes. |
| `INF-031-U056` | RETAIN | → | → |
| `INF-031-U057` | RETAIN | 4 | 4 |
| `INF-031-U058` | TRANSLATE | Use Transportation | Utiliser les transports |
| `INF-031-U059` | TRANSLATE | Tap your card on subway, bus, AREX and other transportation. | Validez votre carte dans le métro, le bus, l’AREX et les autres transports. |
| `INF-031-U060` | RETAIN | → | → |
| `INF-031-U061` | RETAIN | 5 | 5 |
| `INF-031-U062` | TRANSLATE | Check Balance / Refund | Vérifier le solde / remboursement |
| `INF-031-U063` | RETAIN | BALANCE | BALANCE |
| `INF-031-U064` | RETAIN | W 30,000 | W 30,000 |
| `INF-031-U065` | RETAIN | W | W |
| `INF-031-U066` | TRANSLATE | Check balance in the app or at machines and get a refund if needed. | Vérifiez le solde dans l’application ou aux bornes et demandez un remboursement si nécessaire. |
| `INF-031-U067` | TRANSLATE | TIP | ASTUCE |
| `INF-031-U068` | TRANSLATE | You can top up, check balance and get a refund at WOWPASS machines. | Vous pouvez recharger, vérifier le solde et obtenir un remboursement aux bornes WOWPASS. |

### INF-033

| Unit | Mode | Exact English source | Approved French target |
|---|---|---|---|
| `INF-033-U001` | RETAIN | 1 | 1 |
| `INF-033-U002` | TRANSLATE | Get Card | Obtenir la carte |
| `INF-033-U003` | RETAIN | W | W |
| `INF-033-U004` | RETAIN | WOWPASS | WOWPASS |
| `INF-033-U005` | RETAIN | T | T |
| `INF-033-U006` | RETAIN | Tmoney | Tmoney |
| `INF-033-U007` | RETAIN | W | W |
| `INF-033-U008` | TRANSLATE | Get your WOWPASS card at airport machines or partner locations. | Récupérez votre carte WOWPASS aux bornes de l’aéroport ou chez les partenaires. |
| `INF-033-U009` | RETAIN | → | → |
| `INF-033-U010` | RETAIN | 2 | 2 |
| `INF-033-U011` | TRANSLATE | Load Money / Exchange Currency | Charger de l’argent / changer des devises |
| `INF-033-U012` | RETAIN | $ | $ |
| `INF-033-U013` | RETAIN | € | € |
| `INF-033-U014` | RETAIN | ¥ | ¥ |
| `INF-033-U015` | RETAIN | → | → |
| `INF-033-U016` | RETAIN | ₩ | ₩ |
| `INF-033-U017` | TRANSLATE | Load Korean won (KRW) or exchange foreign currency onto your card. | Chargez des wons coréens (KRW) ou changez des devises étrangères sur votre carte. |
| `INF-033-U018` | RETAIN | → | → |
| `INF-033-U019` | RETAIN | 3 | 3 |
| `INF-033-U020` | TRANSLATE | Pay | Payer |
| `INF-033-U021` | RETAIN | W | W |
| `INF-033-U022` | RETAIN | WOWPASS | WOWPASS |
| `INF-033-U023` | RETAIN | T | T |
| `INF-033-U024` | RETAIN | Tmoney | Tmoney |
| `INF-033-U025` | TRANSLATE | Use your card to pay at stores, cafés and convenience stores. | Utilisez votre carte pour payer dans les magasins, cafés et supérettes. |
| `INF-033-U026` | RETAIN | → | → |
| `INF-033-U027` | RETAIN | 4 | 4 |
| `INF-033-U028` | TRANSLATE | Use Transportation | Utiliser les transports |
| `INF-033-U029` | TRANSLATE | Tap your card on subway, bus, AREX and other transportation. | Validez votre carte dans le métro, le bus, l’AREX et les autres transports. |
| `INF-033-U030` | RETAIN | → | → |
| `INF-033-U031` | RETAIN | 5 | 5 |
| `INF-033-U032` | TRANSLATE | Check Balance / Refund | Vérifier le solde / remboursement |
| `INF-033-U033` | RETAIN | BALANCE | BALANCE |
| `INF-033-U034` | RETAIN | W 30,000 | W 30,000 |
| `INF-033-U035` | RETAIN | W | W |
| `INF-033-U036` | TRANSLATE | Check balance in the app or at machines and get a refund if needed. | Vérifiez le solde dans l’application ou aux bornes et demandez un remboursement si nécessaire. |
| `INF-033-U037` | TRANSLATE | TIP | ASTUCE |
| `INF-033-U038` | TRANSLATE | You can top up, check balance and get a refund at WOWPASS machines. | Vous pouvez recharger, vérifier le solde et obtenir un remboursement aux bornes WOWPASS. |

---

## 5. French production typography gate

French is often longer than English. Production may adjust typography only in this order:

1. line breaks
2. text-box width
3. inner padding
4. line-height
5. minimum necessary font-size
6. small text-box position adjustment

Not allowed:

- deleting French words to make them fit
- changing recommendation strength
- replacing a sentence with a shorter new meaning
- changing numbers, route names, terminals, fares or brands
- enlarging the source canvas
- redesigning the visual hierarchy

High-density assets that deserve particular fit QA: **INF-008, INF-012, INF-013, INF-014, INF-017, INF-025, INF-026, INF-027, INF-029, INF-031, INF-033**.

---

## 6. Approval boundary

This file is a completed French localization Review Copy. It is **not yet CONTENT LOCKED** until the user approves it.

On user approval:

> **Status becomes APPROVED FRENCH INFOGRAPHIC COPY — CONTENT LOCKED.**

Then Stage 3 may begin:

> **Codex produces all 30 French localized assets, runs visual QA, fixes only failing assets, updates the matching French HTML image references after asset PASS, and preserves Git without Production deployment unless separately instructed.**

**FRENCH INFOGRAPHIC STAGE 2 LOCALIZATION COMPLETE — AWAITING USER APPROVAL**