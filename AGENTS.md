# AGENTS.md

# Korea Inside AI Development Guide

## Project Goal

Korea Inside helps foreign visitors solve practical problems when using services in Korea.

This is not a simple tourism site or a destination guide.
It is a problem-solving platform for understanding and using Korea.

Always prioritize:

1. Accuracy
2. User experience
3. SEO
4. Mobile usability
5. Browser translation

---

## General Rules

Never redesign pages.

Keep the existing visual design.

Keep spacing, typography, colors and layout.

Do not change the design, colors, spacing, typography or layout unless the user explicitly requests that change.

Mobile First.

Responsive only.

---

## UI/UX Work Rules

Apply these rules to every new page and every improvement to an existing page.

Korea Inside is Mobile First by default.

Mobile users want fast solutions more than long explanations.

Every core screen must make its purpose understandable within 3 seconds.

Do not make card UI excessively large in width, height or spacing.

Prefer short Step formats over long sentences.

When explaining a procedure, use this format whenever possible:

1 : Open Settings
2 : Go to Cellular
3 : Add eSIM
4 : Scan QR Code
5 : Activate eSIM

Do not repeat the same infographic content below the infographic as long cards.

Keep explanations to one line or less by default.

Reduce text width on mobile so the user's eyes do not need to travel too far left and right.

Preserve existing design colors, fonts, icons and button styles.

Do not create a new design. Make the existing design smaller, clearer and easier to read.

All core information must be provided as visible HTML text, not only inside images.

Desktop layouts should naturally expand the mobile structure.

---

## HTML Rules

HTML is the source of truth.

Images are visual illustrations only.

Important information must never exist only inside images.

Every infographic must have equivalent semantic HTML.

Every meaningful phrase, instruction, label, comparison and warning in an infographic must also appear as visible semantic HTML near that infographic.

Before finishing an infographic change, verify a one-to-one mapping between its important image text and visible HTML text.

Use:

- h1~h4
- p
- ul
- li
- table
- section
- article

whenever appropriate.

---

## Translation Rules

Support browser automatic translation.

Do not place important information inside images.

Do not solve translation problems using only:

- alt
- title
- figcaption

Those are accessibility helpers only.

The same information must exist as visible HTML.

`alt`, `title` and `figcaption` alone do not satisfy automatic-translation support.

---

## SEO Rules

Use semantic HTML.

Keep heading hierarchy.

Every page must have:

- a unique, meaningful `title`
- one non-empty `meta name="description"`
- a canonical URL
- exactly one `h1`
- meaningful `alt` text for informative images

Write meaningful alt text.

Avoid duplicated content.

Preserve internal links.

Do not remove metadata.

---

## Content Freshness Rules

Transportation, fares, travel cards, telecom plans, airport procedures and other changeable information must be checked against an official source.

Maintain the official source URL and last-reviewed date for each changeable claim or content section.

Do not present time-sensitive details as permanent facts when their availability, policy, price or operating conditions can change.

---

## CSS Rules

Reuse existing CSS whenever possible.

Avoid unnecessary new classes.

Avoid inline styles.

Keep Mobile First.

---

## Images

Never delete images unless requested.

Never replace images unless requested.

Images are visual aids.

HTML provides searchable and translatable content.

---

## Safety

Never overwrite user work.

Before editing, run `git status --short` or inspect the changed-file state.

If the requested edit overlaps existing user changes, stop and ask for confirmation before modifying that area.

Ask before destructive changes.

Always explain modified files.

Show diff before finishing.

---

## Visual Verification

Verify visual changes at these viewport widths:

- Mobile: 375px
- Tablet: 768px
- Desktop: 1440px

Confirm that existing design, colors, spacing, typography, layout and responsive behavior remain unchanged unless the user explicitly approved a design change.

---

## Project Structure

Each page owns its own assets.

images/

home/

arrival/

esim/

maps/

wowpass/

tmoney/

apps/

common/

---

## Preferred Workflow

1. Analyze

2. Explain plan

3. Wait for approval

4. Modify files

5. Verify layout at 375px, 768px and 1440px

6. Explain changes

7. Show diff

---

Always preserve the Korea Inside design language.

---

## Codex Development Rules

### Approval Policy

Automatic Approval:

* Markdown
* New HTML
* Documentation
* Reports
* Database Documents

Require Approval:

* Existing HTML modification
* Navigation
* Common CSS
* URL changes
* Redirects
* File deletion

Commit Policy:

* Approved automatic tasks should be committed immediately to the local Git repository.
* Push remains manual.

### 1. 작업 범위

* 한 번의 작업은 하나의 기능 또는 하나의 페이지만 수행한다.
* 승인된 범위를 벗어난 파일은 수정하지 않는다.

### 2. 수정 가능한 파일 수

* HTML 최대 2개
* CSS 최대 1개
* JavaScript 최대 1개
* Markdown 제한 없음

초과 시 작업을 중단하고 승인 요청한다.

### 3. 기존 페이지 보호

기존 HTML, CSS, 디자인, 레이아웃, SEO는 승인 없이 수정하지 않는다.

### 4. 신규 페이지 우선

새 기능은 신규 HTML과 Markdown으로 먼저 구현한다.

기존 페이지 연결은 별도 승인 후 진행한다.

### 5. Navigation

전체 Navigation 수정 금지.
메뉴 변경은 승인 후 진행한다.

### 6. CSS

style.css 전체 수정 금지.
가능하면 별도 CSS를 사용한다.

### 7. SEO

기존 URL 삭제 금지.
Redirect Stub을 우선 사용한다.

### 8. Markdown

코드 수정 시 관련 Markdown을 반드시 함께 수정한다.

### 9. Commit

승인 전 Commit, Push, Merge 금지.

### 10. 보고

작업 완료 후

* 변경 파일
* 수정된 Markdown
* Diff 요약
* 영향받는 페이지
* 잠재적 문제
  를 반드시 보고한다.

### 11. 승인 필수 작업

* 프로젝트 구조 변경
* 메뉴 변경
* 공통 CSS 수정
* URL 변경
* 파일 삭제
* Redirect 추가
* 디자인 변경

### 12. 절대 금지

승인 없이

* 프로젝트 전체 수정
* 전체 리팩터링
* 전체 CSS 수정
* 전체 Navigation 수정
* 전체 치환
  을 수행하지 않는다.

### 13. 개발 철학

빠른 개발보다
안전한 개발
작은 단위 개발
검토 후 승인
원칙을 우선한다.
