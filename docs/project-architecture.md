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
