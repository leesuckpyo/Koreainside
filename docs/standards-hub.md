# Korea Inside OS v1.0 Documentation Architecture

## 목적

Korea Inside OS v1.0은 지금까지 생성된 Standards, Handbooks, Playbooks, Patterns, Assets, Page Specs를 하나의 운영 체계로 재구성한다.

새로운 Standard를 계속 추가하는 방식이 아니라, 기존 문서를 10~15개의 핵심 Handbook 아래로 계층화하고 중복 역할을 줄이는 것을 목표로 한다.

## 0. 현재 권한 기준

- Root `AGENTS.md`는 Codex 작업 규칙의 최상위 기준 문서이다.
- `docs/codex-guidelines.md`는 `AGENTS.md`의 하위 보조 실행 가이드이다.
- `docs/standards-hub.md`는 문서 체계 인덱스와 정리 방향을 제안하는 문서이며, 파일 이동, 삭제, 병합, 이름 변경, 구조 변경을 승인하지 않는다.
- 이 문서의 트리, 병합, Migration Plan은 명시적 사용자 승인 전까지 제안/미승인 상태이다.

## 1. 모든 문서 목록

현재 `docs/`에는 70개의 Markdown 문서가 있다. (2026-07-07 기준)

### Core / Governance

- `project-charter.md`
- `product-vision.md`
- `founder-principles.md`
- `business-operating-system.md`
- `competitive-moat-strategy.md`
- `ai-development-constitution.md`
- `project-memory.md`
- `decision-log.md`
- `conflict-resolution-standard.md`
- `change-management.md`
- `release-strategy.md`

### AI / Codex

Root 기준:

- `../AGENTS.md` (Codex 작업 규칙의 최상위 기준, `docs/` 밖에 위치)

보조 문서:

- `codex-guidelines.md`
- `ai-decision-framework.md`
- `ai-collaboration-protocol.md`
- `ai-self-audit.md`
- `ai-evaluation-standard.md`

### Design / UX / Page Structure

- `design-system.md`
- `page-template-standard.md`
- `golden-page-template.md`
- `component-library.md`
- `anti-pattern-standard.md`

### SEO / Content / Localization

- `seo-standard.md`
- `multilingual-seo-strategy.md`
- `content-where-to-stay-in-seoul.md`

### Quality / Review / Risk

- `review-checklist.md`
- `code-review-standard.md`
- `definition-of-ready.md`
- `risk-management.md`
- `evolution-standard.md`
- `lifecycle-management.md`

### Knowledge / Assets / Intelligence

- `knowledge-management.md`
- `asset-library.md`
- `data-dictionary.md`
- `decision-engine.md`
- `korea-inside-intelligence.md`
- `korea-inside-intelligence/event-schema.md`
- `korea-inside-intelligence/tracking-plan.md`

### Playbooks / Successor

- `master-playbook.md`
- `successor-guide.md`

### Page Specs / Page Reviews

- `wowpass.md`
- `esim.md`
- `payments.md`
- `apps.md`
- `accommodation.md`
- `airport-transfer.md`
- `stay-guide.md`
- `stay-area-database.md`
- `checklist.md`
- `home-hero.md`
- `home-quality-review.md`
- `header.md`
- `footer.md`

### Domain Data

- `hotel-database.md`
- `hotel-scoring-rules.md`

### Asset Specs

- `assets/esim-decision-flow.md`
- `assets/payments-comparison.md`

### Project Logs

- `project/TODO.md`
- `project/ROADMAP.md`
- `project/IDEAS.md`
- `project/HISTORY.md`
- `project/DECISIONS.md`
- `project/CHANGELOG.md`
- `project/BUSINESS.md`
- `project/BUGS.md`

## 2. 중복 분석

### 상위 철학 중복

`project-charter.md`, `product-vision.md`, `founder-principles.md`, `business-operating-system.md`, `competitive-moat-strategy.md`는 모두 프로젝트 방향과 정체성을 다룬다.

정리 방향: `Project Charter`를 최상위로 두고, 나머지는 Product & Business Handbook의 하위 장으로 흡수한다.

### AI 작업 규칙 중복

`ai-development-constitution.md`, `codex-guidelines.md`, `ai-decision-framework.md`, `ai-collaboration-protocol.md`, `ai-self-audit.md`, `ai-evaluation-standard.md`는 모두 AI 작업 절차와 평가 기준을 다룬다.

정리 방향: Codex 작업 범위, 승인, 수정, commit, push, QA, Markdown cleanup 기준은 root `AGENTS.md`를 최상위로 둔다. `codex-guidelines.md`는 AGENTS.md 하위의 보조 실행 가이드로 유지하고, 나머지 AI 문서는 판단 프레임, 협업 방식, 감사, 평가 기준으로 역할을 분리한다.

### 품질 검사 중복

`review-checklist.md`, `code-review-standard.md`, `definition-of-ready.md`, `risk-management.md`, `lifecycle-management.md`는 품질, 준비, 위험, 유지관리 기준이 겹친다.

정리 방향: Quality & Risk Handbook으로 통합하고, DoR은 개발 시작 기준, Review Checklist는 공개 전 기준, Code Review는 코드 변경 기준으로 역할을 분리한다.

### 페이지 템플릿 중복

`page-template-standard.md`와 `golden-page-template.md`는 페이지 구조와 콘텐츠 흐름을 함께 다룬다.

정리 방향: Golden Page Template을 상세 기준으로 유지하고, Page Template Standard는 간단한 기본 구조로 축소한다.

### 지식/자산 관리 중복

`knowledge-management.md`, `asset-library.md`, `lifecycle-management.md`, `data-dictionary.md`는 자산과 지식의 등록, 상태, 출처, 검토 주기를 다룬다.

정리 방향: Knowledge & Asset Handbook으로 통합하고, Lifecycle은 모든 자산의 상태 모델로 사용한다.

### 로그/결정 기록 중복

`decision-log.md`, `project-memory.md`, `project/DECISIONS.md`, `project/HISTORY.md`, `project/CHANGELOG.md`는 결정과 이력을 다룬다.

정리 방향: Decision Log를 공식 ADR로 사용하고, `project/` 폴더는 운영 로그로 유지한다.

## 3. 충돌 분석

### Standards Hub vs Project Charter

기존 Hub는 Product Vision을 최상위로 두었지만, OS v1.0에서는 Project Charter가 모든 문서보다 상위에 위치한다.

해결: Reference Order의 1순위를 Project Charter로 변경한다.

### Page Template Standard vs Golden Page Template

두 문서가 모두 페이지 구조를 정의한다.

해결: Page Template Standard는 최소 골격, Golden Page Template은 상세 실행 기준으로 역할을 나눈다.

### AGENTS.md vs Codex Guidelines vs AI Decision Framework

`AGENTS.md`는 Codex 작업 규칙의 최상위 기준이다.

`docs/codex-guidelines.md`는 AGENTS.md를 반복하지 않는 보조 실행 가이드이다.

`docs/ai-decision-framework.md`는 판단 순서와 의사결정 프레임을 보조한다.

해결: 범위, 승인, 수정, 삭제, 생성, commit, push, QA, Markdown cleanup 기준은 `AGENTS.md`를 우선한다. 실행 감각은 `codex-guidelines.md`, 판단 순서는 `ai-decision-framework.md`를 보조로 사용한다.

### Asset Library vs Knowledge Management

둘 다 출처, 상태, 재사용을 다룬다.

해결: Knowledge Management는 지식 체계, Asset Library는 등록 양식으로 구분한다.

### Release Strategy vs Evolution Standard

Release Strategy는 공개 기준이고 Evolution Standard는 운영 후 개선 기준이다.

해결: Release 이전은 Release Strategy, Release 이후는 Evolution Standard를 따른다.

## 4. 병합 제안

OS v1.0에서는 아래 13개 Handbook 체계를 제안 구조로 검토한다. 이 목록은 실제 폴더 이동, 문서 병합, 파일 삭제, 이름 변경을 승인하지 않는다.

1. Project Charter Handbook
2. Product & Business Handbook
3. Governance & Decision Handbook
4. AI Collaboration Handbook
5. Documentation & Knowledge Handbook
6. UX & Design Handbook
7. SEO & Content Handbook
8. Architecture & Development Handbook
9. Quality & Risk Handbook
10. Release & Evolution Handbook
11. Asset & Lifecycle Handbook
12. Intelligence & Data Handbook
13. Page Specs Handbook

## 5. 최종 트리

아래 트리는 제안/미승인 상태의 문서 구조안이다. 실제 폴더 이동, 파일 이름 변경, 문서 병합, 삭제, 보관 처리는 별도 지시와 diff 승인 전까지 수행하지 않는다.

```text
docs/
  00-project-charter/
    project-charter.md
    product-vision.md
    founder-principles.md

  01-product-business/
    business-operating-system.md
    competitive-moat-strategy.md
    release-strategy.md

  02-governance-decision/
    project-memory.md
    decision-log.md
    conflict-resolution-standard.md
    change-management.md

  03-ai-collaboration/
    ai-development-constitution.md
    codex-guidelines.md
    ai-decision-framework.md
    ai-collaboration-protocol.md
    ai-self-audit.md
    ai-evaluation-standard.md

  04-documentation-knowledge/
    standards-hub.md
    knowledge-management.md
    successor-guide.md
    master-playbook.md

  05-ux-design/
    design-system.md
    page-template-standard.md
    golden-page-template.md
    component-library.md
    anti-pattern-standard.md

  06-seo-content/
    seo-standard.md
    multilingual-seo-strategy.md
    content-where-to-stay-in-seoul.md

  07-architecture-development/
    project-architecture.md
    definition-of-ready.md
    code-review-standard.md

  08-quality-risk/
    review-checklist.md
    risk-management.md
    lifecycle-management.md

  09-assets-lifecycle/
    asset-library.md
    assets/

  10-intelligence-data/
    korea-inside-intelligence.md
    korea-inside-intelligence/
    data-dictionary.md
    decision-engine.md

  11-page-specs/
    wowpass.md
    esim.md
    payments.md
    apps.md
    accommodation.md
    airport-transfer.md
    stay-guide.md
    stay-area-database.md
    checklist.md
    header.md
    footer.md
    home-hero.md
    home-quality-review.md

  12-domain-data/
    hotel-database.md
    hotel-scoring-rules.md

  project/
    TODO.md
    ROADMAP.md
    IDEAS.md
    HISTORY.md
    DECISIONS.md
    CHANGELOG.md
    BUSINESS.md
    BUGS.md
```

## 6. Migration Plan

아래 Migration Plan은 제안/미승인 상태이다. 이 문서만으로 문서 이동, 삭제, 병합, 이름 변경, Archived 처리, 구조 변경을 실행하지 않는다.

### Phase 1: Architecture Approval

현재 문서 이동, 삭제, 병합 없이 OS v1.0 Documentation Architecture를 승인 대상으로 제안한다.

### Phase 2: Hub 정리

`standards-hub.md`를 문서 체계 진입점으로 유지하는 방향을 제안한다. Codex 작업 규칙은 root `AGENTS.md`를 우선한다.

### Phase 3: 역할 중복 정리

중복 문서는 삭제하지 않고 아래 상태로 표시한다.

- Active
- Absorbed
- Deprecated
- Archived

### Phase 4: 폴더 이동 제안

실제 폴더 이동은 SEO나 코드 영향은 없지만, 문서 참조 링크가 깨질 수 있으므로 별도 diff와 승인 후 진행한다.

### Phase 5: 문서 병합

각 Handbook 안에서 중복되는 문장을 제거하고 원본 문서를 보관 또는 Archived 처리하는 방안은 별도 승인 후 검토한다.

### Phase 6: 운영 적용

새로운 Codex 작업은 아래 순서로 문서를 참조한다.

1. AGENTS.md
2. Project Charter
3. Product Vision
4. Project Memory
5. Decision Log
6. Codex Guidelines
7. AI Decision Framework
8. Master Playbook
9. 관련 Handbook
10. Page Specs
11. Review Checklist

## 7. 승인 전 금지 사항

- 기존 문서 삭제 금지
- 폴더 이동 금지
- 파일명 변경 금지
- 기존 페이지 또는 코드 수정 금지
- Standards 통합 실행 금지

## 8. 최종 원칙

Korea Inside OS v1.0은 문서를 더 많이 만드는 체계가 아니다.

이미 만들어진 문서를 역할별로 정리하고, 프로젝트가 오래 성장해도 같은 기준으로 판단하게 만드는 운영 체계이다.
