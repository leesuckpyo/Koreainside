# Korea Inside Intelligence Event Schema

## 1. Purpose

This document defines the shared event tracking structure for Korea Inside Intelligence (KII).

The schema is intended for both:

- Codex frontend implementation
- Manus data engine processing

KII events should record anonymous decision behavior, not personal data. The goal is to understand how foreign travelers make practical decisions in Korea, such as choosing a stay area, accommodation type, travel service, or partner option.

This document is documentation only. It does not implement tracking JavaScript, tracking attributes, cookies, localStorage, API calls, analytics integrations, or backend storage.

## 2. Core Event Fields

| Field | Type | Required | Description | Example |
| --- | --- | --- | --- | --- |
| `event_id` | string | Yes | Unique ID for the event. Should be generated per event. | `evt_20260630_001` |
| `event_name` | string | Yes | Standard event name from the recommended event list. | `page_view` |
| `timestamp` | string | Yes | ISO 8601 timestamp when the event occurred. | `2026-06-30T14:30:00+09:00` |
| `page_url` | string | Yes | Page URL where the event occurred. | `/where-to-stay-in-seoul.html` |
| `page_type` | string | Yes | Functional page type. | `stay_guide` |
| `page_cluster` | string | Yes | Content cluster or product area. | `stay_cluster` |
| `country` | string | No | Country inferred at an aggregated level when allowed. | `USA` |
| `language` | string | No | Browser or page language. | `en` |
| `device_type` | string | No | Device category. | `mobile` |
| `browser` | string | No | Browser family. | `Chrome` |
| `referrer` | string | No | Referring source or domain. Avoid storing sensitive full URLs when unnecessary. | `google` |
| `session_id` | string | Yes | Anonymous session identifier. Must not contain personal data. | `ses_a83f12` |
| `visitor_id` | string | No | Anonymous visitor identifier if used. Must not identify a real person. | `vis_91bc20` |

### Core Field Rules

- `event_id` must be unique enough for deduplication.
- `session_id` and `visitor_id` must be anonymous.
- `page_url` should not include sensitive query parameters.
- `country`, `language`, `device_type`, `browser`, and `referrer` should be used for aggregated analysis, not individual profiling.

## 3. Decision Fields

Decision fields describe what the traveler is trying to choose.

| Field | Type | Required | Description | Example |
| --- | --- | --- | --- | --- |
| `travel_style` | string | No | Traveler segment or trip style. | `first_time`, `family`, `solo`, `couple`, `budget`, `luxury` |
| `accommodation_type` | string | No | Accommodation category selected or shown. | `hotel`, `budget_hotel`, `apartment`, `serviced_apartment`, `hanok_stay`, `hostel` |
| `stay_area` | string | No | Seoul area or neighborhood involved in the decision. | `hongdae`, `myeongdong`, `gangnam`, `jamsil` |
| `stay_duration_range` | string | No | Approximate stay length category. | `1_3_nights`, `4_7_nights`, `8_14_nights`, `15_plus_nights` |
| `decision_stage` | string | No | Funnel stage or decision depth. | `awareness`, `comparison`, `selection`, `partner_interest` |

### Decision Field Rules

- Use controlled values whenever possible.
- Do not store free-text user input unless it has been reviewed for privacy and safety.
- Decision fields should explain why a user may choose something, not only what they clicked.

## 4. Partner Fields

Partner fields describe commercial intent after decision support.

| Field | Type | Required | Description | Example |
| --- | --- | --- | --- | --- |
| `partner_category` | string | No | Partner service category. | `hotel`, `esim`, `airport_pickup`, `transport_pass` |
| `partner_name` | string | No | Partner or provider name if shown. | `Booking`, `Agoda`, `Airalo` |
| `partner_target` | string | No | Destination, product, area, or page target. | `hongdae_apartment_options` |
| `click_type` | string | No | Type of partner action. | `outbound_click`, `compare_click`, `report_export` |

### Partner Field Rules

- Partner fields must never override user suitability as the main recommendation logic.
- Partner reports should use aggregated counts and trends.
- Do not include raw IP addresses, payment information, personal contact details, or booking confirmation details in partner reports.

## 5. Recommended Event Names

| Event name | Purpose |
| --- | --- |
| `page_view` | A page was viewed. |
| `section_view` | A meaningful section was viewed or reached. |
| `decision_click` | A decision-support link, card, filter, or CTA was clicked. |
| `accommodation_type_select` | A traveler selected or compared an accommodation type. |
| `area_select` | A traveler selected or compared a stay area. |
| `travel_style_select` | A traveler selected or compared a travel style. |
| `partner_click` | A traveler clicked a partner-related option. |
| `report_export` | A report export was requested in KII. |

## 6. Example JSON Events

### Stay Page View

```json
{
  "event_id": "evt_20260630_0001",
  "event_name": "page_view",
  "timestamp": "2026-06-30T14:30:00+09:00",
  "page_url": "/where-to-stay-in-seoul.html",
  "page_type": "stay_guide",
  "page_cluster": "stay_cluster",
  "country": "USA",
  "language": "en",
  "device_type": "mobile",
  "browser": "Chrome",
  "referrer": "google",
  "session_id": "ses_8f21a0",
  "visitor_id": "vis_4c9021",
  "decision_stage": "awareness"
}
```

### Accommodation Type Selection

```json
{
  "event_id": "evt_20260630_0002",
  "event_name": "accommodation_type_select",
  "timestamp": "2026-06-30T14:32:10+09:00",
  "page_url": "/where-to-stay-in-seoul.html",
  "page_type": "stay_guide",
  "page_cluster": "stay_cluster",
  "country": "Taiwan",
  "language": "en",
  "device_type": "mobile",
  "browser": "Safari",
  "referrer": "google",
  "session_id": "ses_3a7b51",
  "visitor_id": "vis_7d8220",
  "travel_style": "family",
  "accommodation_type": "serviced_apartment",
  "stay_duration_range": "4_7_nights",
  "decision_stage": "comparison"
}
```

### Partner Click

```json
{
  "event_id": "evt_20260630_0003",
  "event_name": "partner_click",
  "timestamp": "2026-06-30T14:35:42+09:00",
  "page_url": "/best-area-for-budget-travelers-seoul.html",
  "page_type": "stay_cluster_page",
  "page_cluster": "stay_cluster",
  "country": "Germany",
  "language": "en",
  "device_type": "desktop",
  "browser": "Firefox",
  "referrer": "google",
  "session_id": "ses_9c3441",
  "visitor_id": "vis_2a1987",
  "travel_style": "budget",
  "accommodation_type": "budget_hotel",
  "stay_area": "hongdae",
  "stay_duration_range": "4_7_nights",
  "decision_stage": "partner_interest",
  "partner_category": "hotel",
  "partner_name": "Booking",
  "partner_target": "hongdae_budget_hotel_options",
  "click_type": "outbound_click"
}
```

### eSIM Provider Interest

```json
{
  "event_id": "evt_20260630_0004",
  "event_name": "partner_click",
  "timestamp": "2026-06-30T14:40:25+09:00",
  "page_url": "/esim.html",
  "page_type": "service_guide",
  "page_cluster": "connectivity",
  "country": "France",
  "language": "en",
  "device_type": "mobile",
  "browser": "Chrome",
  "referrer": "google",
  "session_id": "ses_104bb8",
  "visitor_id": "vis_31e450",
  "decision_stage": "partner_interest",
  "partner_category": "esim",
  "partner_name": "Mock eSIM Provider",
  "partner_target": "korea_esim_short_trip",
  "click_type": "compare_click"
}
```

### Seasonal Report Export

```json
{
  "event_id": "evt_20260630_0005",
  "event_name": "report_export",
  "timestamp": "2026-06-30T14:45:00+09:00",
  "page_url": "/kii-dashboard.html",
  "page_type": "intelligence_dashboard",
  "page_cluster": "korea_inside_intelligence",
  "country": "internal",
  "language": "en",
  "device_type": "desktop",
  "browser": "Chrome",
  "referrer": "direct",
  "session_id": "ses_internal_001",
  "visitor_id": "vis_internal_001",
  "decision_stage": "reporting",
  "partner_category": "all",
  "partner_name": "all",
  "partner_target": "seasonal_report",
  "click_type": "report_export"
}
```

## 7. Privacy Rules

KII must protect traveler trust.

Privacy rules:

- No raw IP addresses in partner reports.
- No names.
- No email addresses.
- No phone numbers.
- No passport data.
- No payment data.
- No booking confirmation data.
- Use anonymous aggregated reporting.
- Raw logs, if used, are internal only.
- Raw logs should not be exported to partners.
- Raw logs should not be used in public reports.

### Data Minimization

Collect only the fields required to understand decision patterns. If a field is not needed for decision intelligence, do not collect it.

### Partner Reporting

Partner reports should show aggregated trends such as:

- country-level demand
- page cluster performance
- partner category interest
- seasonal patterns
- decision stage conversion

Partner reports should not show identifiable user records.

## 8. Future Integration Notes

Future KII integrations may include:

- Google Analytics
- Google Search Console
- server logs
- backend database
- PDF reports
- Excel reports
- Partner Insights

Integration principles:

- Analytics integrations must follow the same event naming structure where possible.
- Search Console should be used for aggregated search demand and query/page performance.
- Server logs, if used, should be processed internally and aggregated before reporting.
- Backend database tables should store structured event fields with controlled values.
- PDF and Excel reports should export aggregated summaries, not raw personal-level logs.
- Partner Insights should explain decision quality and user suitability, not only revenue potential.

## Implementation Status

This schema is a planning document only.

Not included in this task:

- frontend tracking attributes
- JavaScript tracking
- API calls
- cookies
- localStorage
- backend database tables
- analytics provider setup
- partner report exports
