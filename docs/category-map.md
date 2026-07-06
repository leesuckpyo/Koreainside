# Korea Inside Project Explorer Tree v1.2

Date: 2026-07-06

## Purpose

Category Map v1.2 converts the admin architecture map into a Project Explorer Tree.

The v1.1 card and architecture-diagram layout was useful for journey thinking, but it was not the right structure-validation tool. v1.2 prioritizes a Windows Explorer-style folder tree so Korea Inside can review the whole project structure quickly and consistently.

The HTML map lives at:

- `admin/category-map.html`

The page is admin-only and must remain:

- `noindex, nofollow`
- `translate="no"`

## Phase 1 Rule

v1.2 is Phase 1: structure validation.

Rules:

- No page navigation links.
- No public-page edits.
- No admin analyzer edits.
- No external libraries.
- No localStorage.
- Default state is fully expanded.
- Collapse and expand may use vanilla JavaScript only.
- Print view must show the full tree.

The tree is an operating map, not a public navigation menu.

## Phase 2 Direction

Phase 2 can begin after page ownership and page readiness are clearer.

Possible Phase 2 additions:

- Add links to completed pages.
- Add links to admin tools.
- Add management dashboard states.
- Add page ownership and review dates.
- Add source-of-truth document links.

Phase 2 should not start until the structure itself is approved.

## Why Explorer Tree

Explorer Tree is preferred over card or family-tree layouts for this admin map because:

- It is easier to scan many pages vertically.
- Parent and child relationships are clearer.
- Planned pages can sit beside completed pages without visual noise.
- Admin can collapse categories while reviewing one branch.
- It resembles a project filesystem, which matches how the site will grow.
- It avoids implying that every node is a public navigation link.

Card layouts are better for presentation. Explorer Tree is better for structure governance.

## Tree Structure

```text
Korea Inside
├─ Accommodation
│  ├─ Accommodation Guide
│  ├─ Where to Stay in Seoul
│  ├─ Area Selection
│  │  ├─ Hongdae
│  │  ├─ Myeongdong
│  │  ├─ Gangnam
│  │  ├─ Jongno
│  │  └─ Seongsu
│  ├─ Hotel
│  └─ Booking Affiliate
│
├─ Connectivity
│  ├─ eSIM
│  ├─ Korean SIM
│  └─ Wi-Fi
│
├─ Payment
│  ├─ WOWPASS
│  ├─ T-money
│  └─ Payments
│
├─ Arrival
│  ├─ Arrival Guide
│  ├─ Airport Transfer
│  └─ Travel Checklist
│
├─ Maps & Navigation
│  ├─ Maps Guide
│  ├─ Apps Bridge Guide
│  ├─ Naver Map
│  ├─ Kakao Map
│  └─ Google Maps
│
├─ Transportation
│  ├─ Subway
│  ├─ Bus
│  ├─ Taxi
│  ├─ KTX
│  └─ Rental Car
│
├─ How Korea Works
│  ├─ Practical Systems
│  ├─ Etiquette
│  ├─ Shopping
│  ├─ Emergency
│  └─ Common Mistakes
│
└─ Admin Tools
   ├─ Accommodation Analyzer
   ├─ Category Map / Project Explorer
   └─ Future Data Tools
```

## Branch Notes

### Accommodation

Accommodation remains the highest-priority content branch.

Accommodation should contain:

- Accommodation Guide
- Where to Stay in Seoul
- Area Selection
- Area pages such as Hongdae, Myeongdong, Gangnam, Jongno and Seongsu
- Hotel evaluation layer
- Booking Affiliate revenue layer

Booking Affiliate must stay below decision support. It should not appear before area and hotel choice.

### Connectivity

Connectivity contains practical connection setup.

Current and planned nodes:

- eSIM
- Korean SIM
- Wi-Fi

### Payment

Payment contains payment decision support.

Current nodes:

- WOWPASS
- T-money
- Payments

T-money can also be related to Transportation, but in Phase 1 the tree keeps one visible node in Payment to avoid duplicate-link behavior.

### Arrival

Arrival contains first-entry preparation.

Current and planned nodes:

- Arrival Guide
- Airport Transfer
- Travel Checklist

Travel Checklist remains Planned in this structure.

### Maps & Navigation

Maps & Navigation contains both map guidance and app behavior.

Apps Bridge Guide is not an independent top-level category. It belongs inside Maps & Navigation as the bridge toward How Korea Works.

Current and planned nodes:

- Maps Guide
- Apps Bridge Guide
- Naver Map
- Kakao Map
- Google Maps

### Transportation

Transportation is a broader movement branch.

Planned nodes:

- Subway
- Bus
- Taxi
- KTX
- Rental Car

### How Korea Works

How Korea Works contains practical systems that do not fit neatly into single travel-task pages.

Planned nodes:

- Practical Systems
- Etiquette
- Shopping
- Emergency
- Common Mistakes

### Admin Tools

Admin Tools is separate from public user-facing content.

Current and planned nodes:

- Accommodation Analyzer
- Category Map / Project Explorer
- Future Data Tools

Admin pages must remain `noindex, nofollow`.

## Status Badge Rules

| Badge | Meaning |
|---|---|
| Done | Exists and is usable in its current role |
| Planned | Planned or structurally expected but not complete |
| Core | Primary decision-support content |
| Affiliate | Revenue-related path or affiliate opportunity |
| Admin | Internal operating tool or internal architecture page |

Multiple badges may appear on one node.

Examples:

- Accommodation Guide: Done / Core
- Booking Affiliate: Planned / Affiliate
- Accommodation Analyzer: Done / Admin
- Category Map / Project Explorer: Done / Admin

## UI Rules

The admin page should behave like a project explorer:

- Folder and page icons are visible.
- Korean labels are primary.
- English labels are smaller support labels.
- Nodes are not links in Phase 1.
- Default state is fully expanded.
- Collapse and expand should not persist after refresh.
- Print should show the full expanded tree.

The Project Explorer is a structure governance tool. It should help Korea Inside decide what exists, what is planned and where each piece belongs before public navigation is expanded.
