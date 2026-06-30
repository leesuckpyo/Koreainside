# Korea Inside Intelligence Tracking Plan

## Purpose

This document defines which Korea Inside Intelligence (KII) events should be tracked first when Korea Inside goes live.

This is documentation only. It does not implement JavaScript, cookies, localStorage, frontend tracking attributes, API calls, analytics integrations, or backend storage.

KII tracking should help Korea Inside understand anonymous decision behavior:

- which pages help users make decisions
- which sections create useful engagement
- which travel styles and stay areas create demand
- which partner categories show qualified interest

Tracking should never reduce user trust.

## 1. Tracking Priority

### Phase 1: Launch Essentials

Phase 1 should track only the minimum events needed to understand page performance and decision intent.

| Event | Purpose | Priority |
| --- | --- | --- |
| `page_view` | Understand which pages attract users. | Critical |
| `section_view` | Understand which sections users reach. | Critical |
| `decision_click` | Understand which decision-support links, cards and CTAs users choose. | Critical |
| `partner_click` | Understand qualified partner interest after decision support. | Critical |

### Phase 2: Decision Detail

Phase 2 should capture structured decision choices after the basic funnel is working.

| Event | Purpose | Priority |
| --- | --- | --- |
| `accommodation_type_select` | Track interest in hotel, apartment, serviced apartment, hanok stay, hostel and budget hotel decisions. | High |
| `area_select` | Track stay area selection and comparison demand. | High |
| `travel_style_select` | Track traveler segment choices such as family, solo, couple, budget and luxury. | High |
| `stay_duration_select` | Track short-stay vs long-stay planning behavior. | Medium |

### Phase 3: Intelligence Reports

Phase 3 should support KII dashboard reporting and partner analysis.

| Event | Purpose | Priority |
| --- | --- | --- |
| `report_export` | Track internal report export demand. | Medium |
| `partner_report_view` | Track which partner reports are reviewed internally. | Medium |
| `seasonal_compare` | Track seasonal market analysis usage. | Medium |

## 2. Stay Cluster Events

Stay Cluster tracking should focus on decision support, not passive reading alone.

| Page / Area | Tracking targets | Suggested events |
| --- | --- | --- |
| Where to Stay | Quick decision cards, area comparison table, related guide clicks, partner interest | `page_view`, `section_view`, `decision_click`, `area_select`, `partner_click` |
| Budget Travelers | Hidden cost section, budget area comparison, room type interest, related guide clicks | `page_view`, `section_view`, `decision_click`, `accommodation_type_select`, `area_select` |
| Luxury Hotels | luxury area comparison, family/luxury fit, shopping/dining interest, partner intent | `page_view`, `section_view`, `decision_click`, `area_select`, `partner_click` |
| Families | family-friendly area decisions, luggage convenience, quietness, related guide clicks | `page_view`, `section_view`, `decision_click`, `travel_style_select`, `area_select` |
| Solo Travelers | safety notes, late-night movement, budget fit, food access, related guide clicks | `page_view`, `section_view`, `decision_click`, `travel_style_select`, `area_select` |
| Nightlife | noise warnings, late-night transport notes, nightlife area comparison | `page_view`, `section_view`, `decision_click`, `area_select` |
| Shopping | shopping style comparison, suitcase convenience, luxury vs budget shopping intent | `page_view`, `section_view`, `decision_click`, `area_select` |
| Couples | atmosphere, quietness, restaurants, transport and experience-based choices | `page_view`, `section_view`, `decision_click`, `travel_style_select`, `area_select` |
| Accommodation Type future page | hotel vs apartment vs serviced apartment vs hostel vs hanok stay selection | `page_view`, `section_view`, `accommodation_type_select`, `decision_click`, `partner_click` |

### Stay Cluster Notes

- Track the decision path from broad guide to specific stay page.
- Track related guide clicks because they reveal the user's next decision.
- Do not track free-text personal preferences in Phase 1.
- Use controlled values for stay areas, travel styles and accommodation types.

## 3. eSIM Events

eSIM tracking should identify practical connectivity decisions before partner monetization.

| Tracking target | Decision question | Suggested events |
| --- | --- | --- |
| Compatibility check | Can this user's phone use an eSIM in Korea? | `section_view`, `decision_click` |
| Data-only vs phone-number comparison | Does the user need data only or a Korean phone number? | `section_view`, `decision_click` |
| Provider interest | Which provider type does the user show interest in? | `partner_click`, `decision_click` |
| Korean SIM vs international eSIM choice | Is the user choosing convenience or local verification support? | `decision_click`, `partner_click` |

### eSIM Notes

- Do not collect phone numbers.
- Do not collect IMEI, device identifiers or payment information.
- Track provider interest as aggregated category-level behavior.

## 4. Arrival / Transport Events

Arrival and transport tracking should show which transport decisions create the most friction for foreign visitors.

| Tracking target | Decision question | Suggested events |
| --- | --- | --- |
| Terminal guide | Does the user need airport terminal orientation? | `page_view`, `section_view` |
| AREX interest | Is the user considering train access from the airport? | `decision_click`, `partner_click` |
| Airport bus interest | Is the user considering airport bus convenience? | `decision_click`, `partner_click` |
| Taxi interest | Does the user need late-night or luggage-friendly transport? | `decision_click`, `partner_click` |
| T-money interest | Is the user preparing for public transport payment? | `decision_click`, `section_view` |
| WOWPASS interest | Is the user comparing travel card and payment options? | `decision_click`, `partner_click` |

### Arrival / Transport Notes

- Track transport interest by category, not by individual route history.
- Do not store exact personal movement paths.
- Transport tracking should support better decision content and partner fit.

## 5. Partner Interest Events

Partner interest events should be triggered only after useful decision context is provided.

| Partner event | Category | Purpose |
| --- | --- | --- |
| `booking_interest` | Hotel / accommodation | Measure interest in Booking-related options. |
| `agoda_interest` | Hotel / accommodation | Measure interest in Agoda-related options. |
| `esim_provider_interest` | eSIM | Measure interest in eSIM providers. |
| `airport_pickup_interest` | Airport transfer | Measure interest in airport pickup services. |
| `transport_pass_interest` | Transportation | Measure interest in transport passes. |

### Partner Interest Rules

- Partner events should not be the first interaction on a decision page.
- Partner events should preserve context such as page type, page cluster, travel style, stay area or service category.
- Partner reporting should be aggregated.
- Partner reporting should not include raw IP addresses or personal data.

## 6. Privacy Rules

KII tracking must protect user trust.

Rules:

- Track anonymous events only.
- Do not collect name.
- Do not collect email.
- Do not collect phone number.
- Do not collect passport data.
- Do not collect payment data.
- Do not expose raw IP addresses in reports.
- Use aggregated reporting for partners.
- Avoid free-text tracking unless it has a clear privacy review.
- Do not export raw user-level logs to partners.

## 7. MVP Recommendation

Before public launch, implement only the first 10 events needed to understand basic decision behavior.

Recommended MVP event list:

| Rank | Event | Where | Why it matters |
| --- | --- | --- | --- |
| 1 | `page_view` | All public pages | Establish baseline demand. |
| 2 | `section_view` | Stay Cluster quick decision sections | Understand whether users reach decision content. |
| 3 | `decision_click` | Stay Cluster CTA buttons | Identify decision-support engagement. |
| 4 | `decision_click` | Related Guides cards | Understand next-step decision paths. |
| 5 | `area_select` | Where to Stay and comparison pages | Identify area demand. |
| 6 | `travel_style_select` | family, solo, couples, budget and luxury pages | Understand traveler segments. |
| 7 | `accommodation_type_select` | future accommodation type page | Identify hotel vs apartment vs hostel demand. |
| 8 | `partner_click` | accommodation partner links after approval | Measure qualified hotel partner interest. |
| 9 | `partner_click` | eSIM provider links after approval | Measure connectivity partner interest. |
| 10 | `decision_click` | transport decision cards | Understand airport and transport decision friction. |

### MVP Implementation Rule

Do not implement all possible events at once.

Start with the smallest privacy-safe set that answers:

- Which pages attract users?
- Which decision sections matter?
- Which stay areas and traveler types create demand?
- Which partner categories receive qualified interest?

## Not Included In This Task

This task does not include:

- frontend tracking attributes
- JavaScript
- cookies
- localStorage
- analytics provider setup
- backend database tables
- API calls
- partner report exports
- public page changes
