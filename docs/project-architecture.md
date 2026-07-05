# Korea Inside Project Architecture Standard

## 1. 기본 원칙

- 기능보다 구조를 먼저 설계한다.
- 공통 요소는 재사용한다.
- 페이지마다 동일한 구조를 유지한다.
- 파일 위치는 일관성을 유지한다.

---

## 2. 폴더 구조

/docs

/assets

/assets/images

/assets/icons

/assets/animations

/assets/fonts

/css

/js

/pages

---

## 3. 이미지 구조

money/

arrival/

wowpass/

tmoney/

maps/

airport/

hotel/

transport/

food/

---

## 4. CSS

base.css

layout.css

components.css

utilities.css

page.css

페이지별 CSS는 필요한 경우에만 추가한다.

---

## 5. JavaScript

공통 기능

↓

컴포넌트

↓

페이지 기능

순서로 관리한다.

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

모든 주요 기능은

대응하는 md 문서를 유지한다.

예)

wowpass.html

↓

wowpass.md

---

## 8. Assets

이미지는

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

필요한 경우에만 신규 파일을 생성한다.

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
