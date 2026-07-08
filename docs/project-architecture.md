# Korea Inside Project Architecture Standard

This document is the current project architecture reference.

It documents the repository structure that exists now. It is not an architecture proposal and does not approve new folders, file moves, renames, redirects, or restructuring.

## 1. 기본 원칙

- 현재 저장소 구조를 기준으로 작업한다.
- 공통 요소는 재사용한다.
- 기존 페이지 배치와 파일 위치를 유지한다.
- 파일 위치는 일관성을 유지한다.
- 새 파일, 새 폴더, 파일 이동, 이름 변경, 구조 변경은 사용자 명시 승인 전까지 금지한다.

---

## 2. 현재 저장소 구조

현재 상위 구조:

- `admin/`
- `backup/` reference only
- `docs/`
- `images/`
- `prompts/`
- `reference/`

현재 루트 주요 파일:

- root-level HTML pages such as `index.html`, `arrival.html`, `airport-transfer.html`, `esim.html`, `maps.html`, `payments.html`, `tmoney.html`, `wowpass.html`, and stay-cluster pages.
- `style.css`
- `common.js`
- `PROJECT.md`
- `AGENTS.md`
- `README.md`

현재 존재하지 않는 기준 구조:

- root `assets/`
- root `css/`
- root `js/`
- root `pages/`

These folders must not be created unless the user explicitly approves that structure change.

---

## 3. 이미지 구조

Image assets currently live under root `images/`.

Current image folders include:

- `images/Accommodation/`
- `images/airport/`
- `images/arrival/`
- `images/esim/`
- `images/home/`
- `images/infographics/`
- `images/maps/`
- `images/tmoney/`
- `images/wowpass/`

Do not create a separate root `assets/images/` structure without explicit approval.

---

## 4. CSS

Current shared CSS file:

- `style.css`

There is no current root `css/` folder and no split CSS architecture such as `base.css`, `layout.css`, `components.css`, `utilities.css`, or `page.css`.

Do not split or reorganize CSS files unless a separate approved task authorizes the change and shows the full impact.

페이지별 CSS 파일은 현재 기본 구조가 아니다. 필요한 경우에도 사용자 명시 승인 전까지 새 CSS 파일을 만들지 않는다.

---

## 5. JavaScript

Current shared JavaScript file:

- `common.js`

There is no current root `js/` folder.

Do not create a new JavaScript folder or split `common.js` unless a separate approved task authorizes the change and shows the full impact.

Page-specific JavaScript should remain minimal and must follow the approved scope for the current task.

---

## 6. 페이지 구조

Hero

↓

Quick Answer

↓

Real Recognition

↓

How It Works

↓

Key Information

↓

Tips

↓

FAQ

↓

Related Pages

↓

Footer CTA

---

## 7. 문서 구조

Project documentation lives under `docs/`.

Page standards and page reviews currently live as Markdown files directly under `docs/`.

Supporting documentation subfolders currently include:

- `docs/assets/`
- `docs/korea-inside-intelligence/`
- `docs/project/`

When code changes behavior, update related Markdown documentation only when the user explicitly requests or approves that documentation action.

예)

wowpass.html

↓

wowpass.md

---

## 8. Assets

현재 이미지 자산은 root `images/` 아래에서 관리한다.

이미지는 가능한 경우:

WebP 우선

SVG 우선

필요 시 PNG

---

## 9. 파일명 규칙

소문자

하이픈 사용

예)

korean-money.webp

wowpass-machine.webp

arrival-terminal.webp

---

## 10. 금지 사항

폴더 중복

중복 이미지

중복 CSS

중복 JS

중복 컴포넌트

---

## 11. 유지보수

새 기능 추가 시

기존 구조를 우선 활용한다.

공통 컴포넌트를 먼저 검토한다.

신규 파일이나 신규 폴더는 사용자 명시 승인 후에만 생성한다.

현재 구조와 다른 폴더 체계가 필요해 보이면 먼저 이유, 영향 범위, 대안을 보고하고 승인 전까지 생성하지 않는다.

---

## 12. Internal Navigation Principle

### Core Principle

Every page should help users solve the next travel problem.

Korea Inside is not a collection of independent pages.

It is one connected travel platform.

Each page should naturally guide users to the next relevant step in their journey.

Every core Korea Inside page must connect users to other relevant Korea Inside guides.

No major page should become a dead end. Users should always have a clear next step.

### Required Pattern

Every major page should include a near-bottom internal navigation section such as:

- Complete Your Korea Trip
- Related Korea Guides
- Continue Your Journey

This section should contain navigation cards linking to relevant Korea Inside pages.

### Purpose

- Prevent dead-end pages.
- Improve the user journey.
- Increase internal linking.
- Improve SEO.
- Increase page views.
- Help users solve the next travel problem naturally.
- Build Korea Inside as one connected platform instead of isolated pages.

### Guide Selection Rules

Each page should display only guides that are relevant to that page.

Recommended guide categories include:

- Airport Arrival
- eSIM
- T-money
- WOWPASS / Payments
- Transportation
- Maps
- Accommodation

Example relationships:

- Accommodation pages may link to Airport Arrival, eSIM, T-money, Transportation and Maps.
- Airport Arrival pages may link to eSIM, Transportation, T-money, Maps and Accommodation.
- eSIM pages may link to Airport Arrival, T-money, Payments and Accommodation.
- Maps pages may link to Transportation, Accommodation, T-money and Airport Arrival.

### Design Rules

- Use consistent card design across the site.
- Keep spacing and layout identical.
- Use a maximum of 6 cards.
- Keep the layout mobile-first and responsive.
- Use short titles.
- Use one-line descriptions.
- Include an icon and clear CTA for each card.

### Implementation Standard

Before adding or editing a major page, confirm that it has a relevant internal navigation section near the bottom.

Do not add random link blocks. Links must support the user's next practical travel decision.
