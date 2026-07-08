# Korea Inside Master Playbook

## 목적

Korea Inside의 실제 작업을 시나리오별로 표준화한다.

모든 작업은 프로젝트 철학, 승인 절차, 품질 기준, 문서 업데이트 기준을 따른다.

이 문서는 작업 판단 보조 기준이며, 파일 변경 권한을 자동 승인하지 않는다.

실제 작업 권한은 현재 사용자 명시 지시, 승인된 작업 범위, root `AGENTS.md`, Conflict Resolution Standard를 우선한다.

관련 MD, Change Log, Project Memory, Decision Log 업데이트는 사용자가 명시 요청하거나 승인한 범위 안에서만 수행한다.

변경이 필요해 보이면 먼저 문제점, 영향 범위, 대상 파일, 대안, 권장안을 보고하고 승인 대기한다.

## 공통 원칙

- Project Charter를 우선 확인한다.
- Product Vision과 Project Memory를 확인한다.
- 구조 변경 전에는 diff를 제시하고 승인받는다.
- 기존 기능, SEO, FAQ, HTML Text를 보존한다.
- 코드 변경 시 관련 문서 보정이 필요해 보이면 대상 문서와 이유를 보고하고 승인 대기한다.
- 작업 완료 후 Review Checklist와 AI Evaluation Standard로 점검한다.

---

## 1. 신규 페이지 개발

### 목적

새로운 사용자 문제를 해결하는 페이지를 만든다.

### 참조 Handbook

- Project Charter
- Product Vision
- Definition of Ready
- Page Template Standard
- SEO Standard
- Design System
- Review Checklist

### 작업 순서

1. 사용자 문제와 검색 의도를 정의한다.
2. 기존 페이지로 해결 가능한지 확인한다.
3. Definition of Ready를 확인한다.
4. 페이지 구조와 SEO 방향을 설계한다.
5. 필요한 이미지와 실물 사진 여부를 정리한다.
6. diff를 제시하고 승인 후 구현한다.
7. 관련 page spec 작성 또는 업데이트가 필요하면 대상 문서와 이유를 보고하고 승인 후 수행한다.

### Checklist

- 사용자 문제가 명확한가?
- 새 페이지가 정말 필요한가?
- URL, Title, Meta 방향이 정리되었는가?
- "아~ 이거구나." 이미지가 있는가?
- FAQ와 Related Pages가 계획되었는가?

### 완료 기준

- Page Template Standard를 따른다.
- SEO Standard를 통과한다.
- Review Checklist를 통과한다.
- 승인된 범위 안에서 필요한 관련 문서가 정리된다.

---

## 2. 기존 페이지 개선

### 목적

기존 페이지의 정보, UX, SEO, 신뢰도를 높인다.

### 참조 Handbook

- Project Memory
- Change Management
- Page Template Standard
- Design System
- Review Checklist
- Code Review Standard

### 작업 순서

1. 개선 목적과 영향 범위를 정의한다.
2. 기존 유용한 콘텐츠와 FAQ를 확인한다.
3. 삭제 없이 보완 가능한 방향을 찾는다.
4. SEO, UX, 접근성 영향을 분석한다.
5. diff를 제시하고 승인 후 수정한다.
6. 관련 md 문서 업데이트가 필요하면 대상 문서와 이유를 보고하고 승인 후 수행한다.

### Checklist

- 기존 섹션을 삭제하지 않았는가?
- 기존 SEO 메타데이터를 보존했는가?
- FAQ를 유지했는가?
- 정보가 더 명확해졌는가?
- 모바일 가독성이 유지되는가?

### 완료 기준

- 기존 기능과 콘텐츠가 보존된다.
- 개선 목적이 충족된다.
- 승인된 범위 안에서 관련 문서와 Change Management 반영 여부가 정리된다.

---

## 3. SEO 개선

### 목적

검색 품질을 높이되 사용자 신뢰와 콘텐츠 품질을 해치지 않는다.

### 참조 Handbook

- SEO Standard
- Content Standard
- Review Checklist
- Anti-Pattern Standard
- Change Management

### 작업 순서

1. 개선 대상 SEO 요소를 정의한다.
2. 검색 의도와 사용자 문제를 확인한다.
3. Title, Meta, Heading, Internal Link, FAQ 영향을 분석한다.
4. 키워드 남용 여부를 점검한다.
5. diff를 제시하고 승인 후 수정한다.
6. SEO 변경 기록이 필요하면 대상 문서와 이유를 보고하고 승인 후 수행한다.

### Checklist

- 키워드 남용이 없는가?
- 핵심 정보가 HTML Text인가?
- Canonical과 Open Graph가 유지되는가?
- Heading 구조가 올바른가?
- 내부 링크가 자연스러운가?

### 완료 기준

- SEO Standard를 통과한다.
- 사용자 가치가 유지된다.
- 변경 이유가 보고된다.

---

## 4. 이미지 추가

### 목적

이미지를 장식이 아니라 정보 전달 요소로 추가한다.

### 참조 Handbook

- Design System
- Knowledge Management
- Review Checklist
- Anti-Pattern Standard
- Page Template Standard

### 작업 순서

1. 이미지의 역할을 정의한다.
2. 실물 사진, 공식 이미지, AI 이미지 중 적합한 방식을 결정한다.
3. 핵심 정보가 이미지 안에만 들어가지 않도록 HTML Text를 준비한다.
4. alt, 파일명, WebP 여부를 점검한다.
5. diff를 제시하고 승인 후 추가한다.
6. 이미지 출처와 사용 페이지 기록이 필요하면 대상 문서와 이유를 보고하고 승인 후 수행한다.

### Checklist

- 이미지가 인식, 사용, 가치, 결정, 위치 중 하나의 역할을 하는가?
- 실물 정확성이 필요한 대상에 AI 이미지를 쓰지 않았는가?
- alt가 작성되었는가?
- HTML Text가 함께 제공되는가?
- 이미지 출처 기록이 필요한지 확인했는가?

### 완료 기준

- 이미지가 정보를 전달한다.
- 접근성과 SEO가 유지된다.
- 승인된 범위 안에서 이미지 출처 문서화 여부가 정리된다.

---

## 5. FAQ 추가

### 목적

실제 사용자 질문에 답하고 SEO 구조를 강화한다.

### 참조 Handbook

- SEO Standard
- Review Checklist
- Content Standard
- Knowledge Management

### 작업 순서

1. 실제 질문인지 확인한다.
2. 기존 FAQ와 중복 여부를 점검한다.
3. 짧고 객관적인 답변을 작성한다.
4. FAQ Schema 적용 가능성을 확인한다.
5. diff를 제시하고 승인 후 추가한다.
6. 관련 문서의 FAQ 방향 업데이트가 필요하면 대상 문서와 이유를 보고하고 승인 후 수행한다.

### Checklist

- 실제 검색 질문인가?
- 중복 질문이 아닌가?
- 답변이 과장 없이 객관적인가?
- HTML Text로 제공되는가?
- 기존 FAQ를 삭제하지 않았는가?

### 완료 기준

- 사용자 질문에 직접 답한다.
- 기존 FAQ와 충돌하지 않는다.
- SEO 구조를 해치지 않는다.

---

## 6. 기능 추가

### 목적

사용자 의사결정이나 문제 해결에 도움이 되는 기능을 추가한다.

### 참조 Handbook

- Product Vision
- AI Decision Framework
- Definition of Ready
- Project Architecture
- Code Review Standard
- Risk Management

### 작업 순서

1. 기능이 해결하는 문제를 정의한다.
2. 기존 기능으로 해결 가능한지 확인한다.
3. 신규 컴포넌트 필요 여부를 판단한다.
4. SEO, UX, 성능, 접근성 영향을 분석한다.
5. diff를 제시하고 승인 후 구현한다.
6. 테스트를 수행하고, 문서 업데이트가 필요하면 대상 문서와 이유를 보고하고 승인 후 수행한다.

### Checklist

- 기능이 실제 사용자 가치가 있는가?
- 기존 구조를 재사용했는가?
- 불필요한 JS를 추가하지 않았는가?
- 모바일에서 사용 가능한가?
- 관련 문서 업데이트가 필요한지 확인했는가?

### 완료 기준

- 기능이 요청 목적을 해결한다.
- 기존 기능이 회귀되지 않는다.
- Code Review Standard를 통과한다.

---

## 7. 버그 수정

### 목적

기존 동작의 오류를 최소 범위로 수정한다.

### 참조 Handbook

- Change Management
- Code Review Standard
- Review Checklist
- Risk Management

### 작업 순서

1. 버그 증상과 재현 조건을 정리한다.
2. 영향 범위를 확인한다.
3. 최소 수정안을 작성한다.
4. 회귀 가능성을 점검한다.
5. diff를 제시하고 승인 후 수정한다.
6. 수정 내용과 남은 리스크를 보고하고, 기록이 필요하면 대상 문서와 이유를 보고한 뒤 승인 후 수행한다.

### Checklist

- 원인이 확인되었는가?
- 수정 범위가 최소인가?
- 관련 기능이 정상 동작하는가?
- 기존 SEO와 콘텐츠가 유지되는가?
- 문서 업데이트가 필요한가?

### 완료 기준

- 버그가 재현되지 않는다.
- 기존 기능이 유지된다.
- 변경 이유가 보고된다.

---

## 8. 디자인 개선

### 목적

기존 디자인 언어를 유지하면서 명확성, 가독성, 인식성을 높인다.

### 참조 Handbook

- Design System
- Page Template Standard
- Review Checklist
- Accessibility Standard
- Anti-Pattern Standard

### 작업 순서

1. 개선하려는 UX 문제를 정의한다.
2. 기존 디자인 시스템과 컴포넌트를 확인한다.
3. 모바일, 태블릿, 데스크톱 영향을 분석한다.
4. 과도한 애니메이션이나 장식 여부를 점검한다.
5. diff를 제시하고 승인 후 수정한다.
6. 디자인 변경 문서화가 필요하면 대상 문서와 이유를 보고하고 승인 후 수행한다.

### Checklist

- 기존 KR Inside 디자인 언어를 유지하는가?
- 사용자가 즉시 이해하기 쉬워졌는가?
- 텍스트와 UI가 겹치지 않는가?
- 모바일에서 읽기 쉬운가?
- 접근성이 유지되는가?

### 완료 기준

- 디자인 시스템과 일치한다.
- 정보 인식성이 개선된다.
- 기존 페이지 구조가 불필요하게 바뀌지 않는다.

---

## 9. Release

### 목적

완성도 기준을 충족한 변경만 공개한다.

### 참조 Handbook

- Release Strategy
- Review Checklist
- Definition of Done
- SEO Standard
- Risk Management

### 작업 순서

1. 릴리스 대상과 범위를 확정한다.
2. SEO, UX, 접근성, 성능, 링크를 점검한다.
3. 모바일과 데스크톱 확인을 수행한다.
4. 남은 리스크를 정리한다.
5. 최종 diff와 변경 요약을 보고한다.
6. 사용자 승인 후 릴리스 준비 상태로 표시한다.

### Checklist

- Review Checklist를 통과했는가?
- Broken Link가 없는가?
- FAQ와 내부 링크가 정상인가?
- 이미지와 alt가 준비되었는가?
- 남은 리스크가 공개 가능한 수준인가?

### 완료 기준

- 출시 기준을 충족한다.
- 남은 리스크가 명확하다.
- 사용자가 최종 승인한다.

---

## 10. 긴급 수정

### 목적

오탈자, 깨진 링크, 명백한 오류 등 긴급 문제를 빠르게 수정한다.

### 참조 Handbook

- Change Management
- Risk Management
- Code Review Standard
- Conflict Resolution Standard

### 작업 순서

1. 긴급성 여부를 판단한다.
2. 수정 범위를 최소화한다.
3. 가능한 경우 즉시 diff를 제시한다.
4. 승인 후 수정한다.
5. 사후 변경 기록이 필요하면 대상 문서와 이유를 보고하고 승인 후 수행한다.

### Checklist

- 정말 긴급 수정인가?
- 구조 변경이 없는가?
- 수정 범위가 최소인가?
- 사후 기록이 필요한지 확인했는가?
- 추가 리스크가 없는가?

### 완료 기준

- 긴급 문제가 해결된다.
- 구조 변경 없이 처리된다.
- 승인된 범위 안에서 변경 기록 여부가 정리된다.

---

## 11. AI Review

### 목적

AI가 수행한 작업이 프로젝트 원칙과 품질 기준을 충족하는지 평가한다.

### 참조 Handbook

- AI Self Audit Standard
- AI Evaluation Standard
- Code Review Standard
- Review Checklist
- Conflict Resolution Standard

### 작업 순서

1. 요청 사항과 실제 변경 내용을 비교한다.
2. Project Charter와 Product Vision 일치 여부를 확인한다.
3. 기존 결정과 충돌하는지 점검한다.
4. SEO, UX, 접근성, 성능, 유지보수성을 평가한다.
5. Overall Score와 남은 리스크를 보고한다.

### Checklist

- 요청 범위를 벗어나지 않았는가?
- 승인 없는 구조 변경이 없는가?
- 기존 기능을 삭제하지 않았는가?
- 관련 문서 업데이트가 필요한지 확인했는가?
- 남은 리스크가 명확한가?

### 완료 기준

- 평가 항목이 모두 점검된다.
- 점수와 리스크가 보고된다.
- 필요한 후속 조치가 명확하다.

---

## 12. Documentation Update

### 목적

코드, 제품 방향, 운영 기준과 문서가 일치하도록 유지한다.

### 참조 Handbook

- Knowledge Management
- Change Management
- Standards Hub
- Decision Log
- Project Memory

### 작업 순서

1. 사용자가 명시 요청하거나 승인한 업데이트 대상 문서를 확인한다.
2. 기존 문서를 삭제하지 않고 병합한다.
3. 관련 Standards와 충돌 여부를 점검한다.
4. diff를 제시하고 승인 후 수정한다.
5. 변경 파일과 남은 리스크를 보고한다.

### Checklist

- 기존 문서를 보존했는가?
- 중복 또는 충돌이 없는가?
- 관련 상위 문서와 일치하는가?
- 변경 이유가 명확한가?
- 코드 파일을 수정하지 않았는가?

### 완료 기준

- 문서가 최신 상태가 된다.
- 관련 Standards와 충돌하지 않는다.
- 승인 가능한 diff가 제공된다.
