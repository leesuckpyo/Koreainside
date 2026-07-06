# Korea Inside Category Map v1.0

Date: 2026-07-06

## Purpose

Category Map is an internal admin document and visual architecture page for Korea Inside.

Its purpose is to show the full category structure at a glance, clarify which public pages belong to which decision area, and prevent navigation, content, affiliate, and admin tools from growing in unrelated directions.

The HTML map lives at:

- `admin/category-map.html`

The page is admin-only and must remain `noindex, nofollow`.

## Primary Category Structure

Korea Inside v1.0 uses eight primary categories:

| Korean category | English support label | Role |
|---|---|---|
| 입국 / 공항 | Arrival & Airport | First problems after trip planning and arrival |
| 숙소 | Accommodation | Area and stay decision support |
| 통신 | Connectivity | Mobile data and connection setup |
| 결제 | Payment | Cards, cash, transit payment and payment friction |
| 교통 | Transportation | Movement between airport, city and local transit |
| 지도 / 길찾기 | Maps & Navigation | Map apps, navigation behavior and station guidance |
| 한국 사용법 | How Korea Works | Practical rules and everyday systems in Korea |
| 관리자 | Admin Tools | Internal tools and operating maps |

## Category Pages

### 입국 / 공항

| Page | URL | Status | Notes |
|---|---|---|---|
| 입국 준비 / Arrival guide | `arrival.html` | Done / Core | Main arrival guide |
| 공항 이동 / Airport transfer | `airport-transfer.html` | Done / Core | Shared with Transportation; one actual public page |
| 여행 체크리스트 / Travel checklist | `checklist.html` | Planned | Not confirmed as a core completed page in this map |

### 숙소

| Page | URL | Status | Notes |
|---|---|---|---|
| 숙소 메인 / Accommodation main | `accommodation.html` | Done / Core | Main accommodation decision page |
| 서울 숙소 지역 선택 / Where to stay in Seoul | `where-to-stay-in-seoul.html` | Done / Core | Area decision guide |
| 숙소 지역 데이터베이스 / Stay area database | Future | Planned | Structured stay-area data |
| 호텔 평가 규칙 / Hotel scoring rules | Future | Planned | Future decision engine rules |

### 통신

| Page | URL | Status | Notes |
|---|---|---|---|
| eSIM | `esim.html` | Done / Affiliate | Mobile data setup and affiliate opportunity |
| SIM / Wi-Fi 비교 | Future | Planned / Affiliate | Future comparison page |

Apps is not treated as a connectivity-only page. It is a bridge between Maps & Navigation and How Korea Works.

### 결제

| Page | URL | Status | Notes |
|---|---|---|---|
| 결제 기본 / Payment basics | `payments.html` | Done / Core | Main payment guide |
| WOWPASS | `wowpass.html` | Done / Affiliate | Affiliate-related payment guide |
| T-money | `tmoney.html` | Done / Core | Shared with Transportation; one actual public page |

### 교통

| Page | URL | Status | Notes |
|---|---|---|---|
| T-money | `tmoney.html` | Done / Core | Shared with Payment; one actual public page |
| 공항 이동 / Airport transfer | `airport-transfer.html` | Done | Shared with Arrival & Airport; one actual public page |
| 지하철 / Subway guide | Future | Planned | Future guide |
| 택시 / Taxi guide | Future | Planned | Future guide |

### 지도 / 길찾기

| Page | URL | Status | Notes |
|---|---|---|---|
| 지도 앱 / Maps and navigation | `maps.html` | Done / Core | Main navigation guide |
| 한국 여행 필수 앱 / Essential Korea travel apps | `apps.html` | Done | Bridge to How Korea Works |
| 역 출구 / Station exits | Future | Planned | Future practical guide |

### 한국 사용법

| Page | URL | Status | Notes |
|---|---|---|---|
| 한국 여행 필수 앱 / Essential Korea travel apps | `apps.html` | Done | Bridge from Maps & Navigation |
| 한국 생활 시스템 / Practical systems | Future | Planned | Future hub |
| 예절 / 이용 방식 / Etiquette and usage norms | Future | Planned | Future guide |
| 쇼핑 / 응급 상황 / Shopping and emergency basics | Future | Planned | Future guides |

### 관리자

| Page | URL | Status | Notes |
|---|---|---|---|
| 숙소 분석기 / Accommodation Analyzer | `admin/accommodation-analyzer.html` | Admin / Done | Internal accommodation database tool |
| 카테고리 설계도 / Category Map | `admin/category-map.html` | Admin / Done | This architecture map |
| 데이터 운영 도구 / Future data tools | Future | Admin / Planned | Future admin tools |

## Status Rules

| Status | Meaning |
|---|---|
| Done | The page or admin tool exists and is usable in its current role |
| Planned | The page, data structure, or feature is expected but not treated as completed |
| Affiliate | The page can support affiliate revenue, but user suitability and trust remain first |
| Admin | Internal tool or internal operating document, not a public user-facing page |
| Core | A primary decision-support page for visitor problem solving |

Shared pages can appear under more than one category, but the actual URL remains one page.

Current shared-page rules:

- `tmoney.html` belongs to both Payment and Transportation.
- `airport-transfer.html` belongs to both Arrival & Airport and Transportation.
- `apps.html` is a bridge between Maps & Navigation and How Korea Works, not a Connectivity-only page.

## Future Expansion Rules

### Home Menu

Add a category or page to the Home menu only when it helps visitors make a better decision from the first screen or connects to a high-priority travel problem.

Home should not become a complete sitemap. It should surface the strongest decision paths.

### Hamburger Menu

The hamburger menu can include more complete category coverage than Home, but it should still group pages by user problem, not by internal file structure.

Shared pages should appear only where they help the user choose. Repeated links are allowed when the user intent is genuinely different.

### Admin Page Expansion

New admin tools should live under `admin/` and remain `noindex, nofollow`.

Admin pages should use isolated CSS and should not depend on public Header, Footer, common JavaScript, or public navigation unless explicitly approved.

### Public Page Expansion

Before creating a public page from a Planned node, confirm:

- The user problem is clear.
- The page helps users choose, not merely browse.
- Changeable facts have reliable sources.
- The page has a maintainable update path.
- Existing pages do not already solve the same user need.

### Affiliate Expansion

Affiliate pages must remain decision-support pages first.

Affiliate status must not determine whether a page is Core. Core status depends on user value, practical importance, and decision impact.
