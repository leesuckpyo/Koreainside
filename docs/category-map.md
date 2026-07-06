# Korea Inside Master Architecture Map v1.1

Date: 2026-07-06

## Purpose

Category Map v1.1 is the internal architecture map for Korea Inside.

The v1.0 map showed the site as parallel categories. v1.1 changes the architecture principle to User Journey First.

The map should show the order in which a traveler actually prepares for Korea, not merely the internal content categories.

The HTML map lives at:

- `admin/category-map.html`

The page is admin-only and must remain:

- `noindex, nofollow`
- `translate="no"`

## Architecture Principle

Korea Inside should first answer:

What does the traveler need to decide next?

The top-level structure follows the travel preparation sequence:

1. Home
2. Travel Planning
3. Accommodation
4. Connectivity
5. Payment
6. Arrival
7. Airport Transfer
8. Maps
9. Transportation
10. How Korea Works

This does not remove categories. It changes their visual priority. Categories exist under a journey order.

## User Journey First Structure

```text
Korea Inside
  ↓
Home (Landing)
  ↓
Travel Planning
  ↓
① Accommodation
  ↓
② Connectivity (eSIM)
  ↓
③ Payment
  ↓
④ Arrival
  ↓
⑤ Airport Transfer
  ↓
⑥ Maps
  ↓
⑦ Transportation
  ↓
⑧ How Korea Works
```

## Accommodation Priority

Accommodation is the most important decision area in the v1.1 architecture.

Reason:

- Stay area affects airport transfer choice.
- Stay area affects subway convenience.
- Stay area affects luggage burden.
- Stay area affects nightlife, shopping, family suitability and noise.
- Hotel booking can become an affiliate revenue path only after area choice is clear.

Accommodation structure:

```text
Accommodation
  ↓
Where to stay in Seoul
  ↓
Area Selection
  ├─ Hongdae
  ├─ Myeongdong
  ├─ Gangnam
  ├─ Jongno
  └─ Seongsu
  ↓
Hotel
  ↓
Booking (Affiliate)
```

Current and planned pages:

| Node | URL | Status | Notes |
|---|---|---|---|
| Accommodation Main | `accommodation.html` | Done / Core | Main accommodation decision entry |
| Where to stay in Seoul | `where-to-stay-in-seoul.html` | Done / Core | Seoul area decision guide |
| Area Selection | Future structured area layer | Core | Must come before hotel recommendation |
| Hotel | Future property layer | Planned | Specific hotel evaluation |
| Booking | Future affiliate path | Planned / Affiliate | Revenue path after decision support |

## Arrival Structure

Arrival should be shown as a journey sequence, not as an isolated category.

```text
Arrival
  ↓
Arrival Guide
  ↓
Airport Transfer
  ↓
Hotel Check-in
```

Current and planned pages:

| Node | URL | Status | Notes |
|---|---|---|---|
| Arrival Guide | `arrival.html` | Done / Core | Arrival preparation and first-step guide |
| Airport Transfer | `airport-transfer.html` | Done / Core | Also appears as its own journey step |
| Hotel Check-in | Future | Planned | Connects arrival back to Accommodation |

## Payment Structure

Payment connects directly to Transportation because T-money belongs to both payment and movement decisions.

```text
Payment
  ↓
WOWPASS
  ↓
T-money
  ↓
Payments
  ↓
Transportation
```

Current pages:

| Node | URL | Status | Notes |
|---|---|---|---|
| WOWPASS | `wowpass.html` | Done / Affiliate | Prepaid card guide |
| T-money | `tmoney.html` | Done / Core | One actual page shared with Transportation |
| Payments | `payments.html` | Done / Core | General payment guide |
| Transportation | Multiple pages | Core | Connected because T-money is also transit infrastructure |

## Connectivity

Connectivity is the second practical preparation step after Accommodation.

Current page:

| Node | URL | Status | Notes |
|---|---|---|---|
| eSIM | `esim.html` | Done / Affiliate | Data setup before arrival |

## Maps, Apps and How Korea Works

Apps is not an independent top-level category in v1.1.

Apps becomes a Bridge Guide between Maps and How Korea Works.

```text
Maps
  ↓
Apps Bridge Guide
  ↓
How Korea Works
```

Current pages:

| Node | URL | Status | Notes |
|---|---|---|---|
| Maps | `maps.html` | Done / Core | Navigation guide |
| Apps Bridge Guide | `apps.html` | Done | Bridge between map behavior and Korea practical systems |
| How Korea Works | Future hub | Planned | Practical systems, norms and usage patterns |

## Transportation

Transportation remains a journey step after Maps because users often understand where they are going before choosing detailed transit behavior.

Current and planned pages:

| Node | URL | Status | Notes |
|---|---|---|---|
| T-money | `tmoney.html` | Done / Core | Shared with Payment |
| Airport Transfer | `airport-transfer.html` | Done / Core | Shared with Arrival |
| Subway | Future | Planned | City transit guide |
| Taxi | Future | Planned | Taxi decision guide |

## Admin Tools

Admin remains a separate management area.

Admin tools are not part of the public user journey, but they can manage a public journey area.

Current admin structure:

| Tool | URL | Status | Managed area |
|---|---|---|---|
| Accommodation Analyzer | `admin/accommodation-analyzer.html` | Admin / Done | Accommodation |
| Category Map | `admin/category-map.html` | Admin / Done | Overall architecture |

The map must show that Accommodation Analyzer manages Accommodation data.

## Status Rules

| Status | Meaning |
|---|---|
| Done | The page or admin tool exists and is usable in its current role |
| Planned | The page, data structure, or feature is expected but not treated as completed |
| Affiliate | The page can support affiliate revenue, but user suitability and trust remain first |
| Admin | Internal tool or internal operating document, not a public user-facing page |
| Core | A primary decision-support page for visitor problem solving |

## Shared Page Rules

Shared pages can appear in more than one journey area, but the actual URL remains one page.

Current shared-page rules:

- `tmoney.html` belongs to both Payment and Transportation.
- `airport-transfer.html` belongs to both Arrival and Airport Transfer.
- `apps.html` is a bridge between Maps and How Korea Works, not an independent top-level category.

## Future Expansion Rules

### Home Menu

Home should reflect the strongest user journey paths, not the full internal sitemap.

Accommodation should receive the highest priority in Home-level architecture because it is the strongest travel decision and future revenue path.

### Hamburger Menu

The hamburger menu may expose more categories than Home, but its order should still follow the user journey:

Accommodation before Connectivity, Payment, Arrival, Airport Transfer, Maps, Transportation and How Korea Works.

### Admin Page Expansion

New admin tools should live under `admin/` and remain `noindex, nofollow`.

Admin tools should clearly state which public journey area they manage.

### Affiliate Expansion

Affiliate revenue should appear after decision support, not before it.

For Accommodation, the correct order is:

```text
Area decision → Hotel decision → Booking affiliate
```

Affiliate status must not determine whether a page is Core. Core status depends on user value, practical importance and decision impact.
