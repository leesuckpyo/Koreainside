# Korea Inside Project Explorer v1.4

Date: 2026-07-06

## Purpose

Project Explorer v1.4 changes the admin architecture map into a VS Code Explorer-style interface.

The v1.3 tree showed file names, descriptions and badges in the same row. That made long file names wrap and made the tree hard to scan.

v1.4 separates structure from metadata:

- Left side: file and folder names only.
- Right side: Detail Panel with description, status, tags and notes.

The HTML map lives at:

- `admin/category-map.html`

The page is admin-only and must remain:

- `noindex, nofollow`
- `translate="no"`

## Phase 1 Rule

v1.4 remains Phase 1: structure validation.

Rules:

- The Explorer Tree is not navigation.
- No `<a>` tags are used.
- No page movement behavior is allowed.
- No external libraries.
- No localStorage.
- Default state is fully expanded.
- Folder collapse and expand use vanilla JavaScript only.
- File and folder names must not wrap.
- Descriptions, status, tags, notes and badges appear only in the Detail Panel.

## Phase 2 Direction

Phase 2 may add real links or admin dashboard behavior after the structure is approved.

Possible Phase 2 additions:

- Link completed nodes to public pages.
- Link admin nodes to internal tools.
- Add source-of-truth document links.
- Add ownership, review status and last-reviewed dates.
- Add dashboard filters for status or category.

Phase 2 must keep Phase 1's structure readable before adding behavior.

## VS Code Explorer Layout

```text
┌──────────────────────────────┬───────────────────────────────────┐
│ Project Explorer              │ Detail Panel                       │
│                               │                                    │
│ ▾ Korea Inside                │ Name: index.html                   │
│   📄 index.html               │ Type: Page                         │
│   📄 style.css                │ Description: 홈 / 랜딩페이지       │
│   📄 common.js                │ Status: Done                       │
│   📄 accommodation.html       │ Tags: Core                         │
│   📄 arrival.html             │ Notes: Korea Inside의 첫 진입 페이지│
│   📁 admin                    │                                    │
│   📁 docs                     │                                    │
│   📁 images                   │                                    │
└──────────────────────────────┴───────────────────────────────────┘
```

## Explorer Tree Rules

The left Explorer Tree must show only:

- Folder icon
- File icon
- Folder or file name
- Folder twisty for collapse and expand

The left Explorer Tree must not show:

- Korean descriptions
- Long sentences
- Status badges
- Tags
- Notes
- Table-like columns

File names must use:

- `white-space: nowrap`
- hidden overflow or horizontal scroll
- no mid-word wrapping

## Detail Panel Rules

The right Detail Panel is the only place for metadata.

Required fields:

| Field | Meaning |
|---|---|
| Name | File or folder name |
| Type | Folder, Page, CSS, JS, Markdown, Image Folder or Admin Tool |
| Korean description | Human-readable Korean explanation |
| Status | Done, Planned or Needs Check |
| Tags | Core, Shared, Affiliate or Admin |
| Notes | Operational notes |

Example:

```text
Name: index.html
Type: Page
Korean description: 홈 / 랜딩페이지
Status: Done
Tags: Core
Notes: Korea Inside의 첫 진입 페이지
```

Example:

```text
Name: where-to-stay-in-seoul.html
Type: Page
Korean description: 서울 숙소 지역 선택 페이지
Status: Needs Check
Tags: Core
Notes: 문서에는 언급되었으나 실제 파일 존재 여부 재확인 필요
```

## Current Tree Scope

v1.4 removes logical `pages/` grouping from the left tree.

The left tree should focus on actual root files and actual folders:

```text
Korea Inside
├─ index.html
├─ style.css
├─ common.js
├─ accommodation.html
├─ arrival.html
├─ airport-transfer.html
├─ apps.html
├─ maps.html
├─ payments.html
├─ tmoney.html
├─ wowpass.html
├─ taxi.html
├─ rental-car.html
├─ admin
├─ docs
└─ images
```

## Status And Tag Rules

Status values:

| Status | Meaning |
|---|---|
| Done | File exists or the feature is usable in its current role |
| Planned | File or feature is planned but does not currently exist |
| Needs Check | Existence, ownership or readiness is unclear |

Tags:

| Tag | Meaning |
|---|---|
| Core | Primary decision-support page or foundation file |
| Shared | A single file is used by multiple pages or categories |
| Affiliate | Revenue-related path or affiliate opportunity |
| Admin | Internal operating tool or internal architecture page |

Tags and status are displayed as badges only in the Detail Panel.

## Governance Notes

Actual public files must not be moved, renamed or reorganized by this map.

The map is a planning and governance surface only.

Phase 2 may connect nodes to files, pages, documents or dashboards, but Phase 1 must remain no-link and structure-first.
