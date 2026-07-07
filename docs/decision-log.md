# Korea Inside Decision Log

## 목적

프로젝트에서 이미 결정한 사항을

다시 논의하지 않도록 한다.

모든 중요한 결정은

이유와 함께 기록한다.

----------------------------------------

Decision Template

----------------------------------------

Decision ID

Date

Status

Decision

Reason

----------------------------------------

Accepted ADR

ADR-001

Real Photos First

Decision

실물 정확성이 중요한 대상은 실제 사진, 공식 이미지, 사용자 제공 사진, 실제 캡처를 우선한다.

Reason

Korea Inside는 사용자가 한국의 실제 사물과 흐름을 알아보는 것을 돕는 프로젝트이므로, 임의 생성 이미지보다 실제 인식 정보가 더 중요하다.

Status

Accepted

Date

2026-07-02

----------------------------------------

ADR-002

HTML First

Decision

핵심 정보는 이미지가 아니라 HTML 텍스트로 제공한다.

Reason

SEO, 접근성, 브라우저 자동번역, 장기 유지보수를 위해 중요한 내용은 검색 가능하고 번역 가능한 텍스트로 유지되어야 한다.

Status

Accepted

Date

2026-07-02

----------------------------------------

ADR-003

Diff Before Modify

Decision

구조 변경, 보호 파일 변경, 중요한 문서 변경은 먼저 diff를 제시하고 승인 후 수정한다.

Reason

기존 페이지, SEO, FAQ, 디자인 시스템, 문서 구조를 보호하고 회귀를 방지하기 위해 변경 전 영향 범위를 확인해야 한다.

Status

Accepted

Date

2026-07-02

----------------------------------------

ADR-004

Less is More

Decision

새 문서, 새 컴포넌트, 새 구조를 늘리기보다 기존 문서를 통합하고 구조를 단순화한다.

Reason

프로젝트가 커질수록 많은 문서와 구조는 판단 비용을 높인다. Korea Inside는 장기 유지보수성을 위해 더 적은 문서와 더 명확한 기준을 우선한다.

Status

Accepted

Date

2026-07-02

----------------------------------------

ADR-005

Trust Before Revenue

Decision

수익보다 사용자 신뢰를 우선한다.

Reason

Korea Inside의 장기 경쟁력은 광고나 제휴가 아니라 정확성, 객관성, 신뢰에서 나온다.

Status

Accepted

Date

2026-07-02

----------------------------------------

ADR-006

"Ah, I get it." UX

Decision

모든 주요 페이지와 섹션은 사용자가 "아~ 이거구나."라고 이해할 수 있도록 구성한다.

Reason

Korea Inside는 설명을 많이 하는 사이트가 아니라 사용자가 한국의 실제 사물, 장소, 절차를 빠르게 인식하도록 돕는 플랫폼이다.

Status

Accepted

Date

2026-07-02

----------------------------------------

ADR-007

SEO Preservation

Decision

기존 SEO 구조, 메타데이터, canonical, FAQ, 내부 링크, HTML 텍스트는 변경 목적이 명확하지 않으면 유지한다.

Reason

Korea Inside는 검색 기반 플랫폼이므로 불필요한 SEO 변경은 트래픽과 신뢰도에 장기적인 손실을 만들 수 있다.

Status

Accepted

Date

2026-07-02

----------------------------------------

ADR-008

No VS-first Comparison Pages

Decision

페이지를 VS 중심 비교 구조로 만들지 않는다. 비교는 사용자의 선택에 도움이 되는 경우에만 짧고 객관적으로 사용한다.

Reason

Korea Inside의 목적은 승자를 정하는 것이 아니라 사용자가 자신의 상황에 맞는 선택을 이해하도록 돕는 것이다.

Status

Accepted

Date

2026-07-02

----------------------------------------

ADR-009

AI Supports, Real Objects Explain

Decision

AI 이미지는 분위기, 배경, 보조 설명에 사용하고, 실물 정확성이 중요한 대상은 실제 사진과 실제 화면으로 설명한다.

Reason

WOWPASS, T-money, 한국 지폐, 키오스크, 앱 화면처럼 사용자가 실제로 마주치는 대상은 정확한 인식이 핵심이다.

Status

Accepted

Date

2026-07-02

----------------------------------------

ADR-010

User Does Not Memorize Rules

Decision

사용자는 프로젝트 운영 규칙을 기억할 필요가 없다. Codex가 이전 대화, 승인 내역, Project Memory, Decision Log, Handbook을 바탕으로 문맥을 해석한다.

Reason

AI의 역할은 사용자가 규칙을 외우게 만드는 것이 아니라 프로젝트 규칙을 기억하고 안전하게 이어가는 것이다.

Status

Accepted

Date

2026-07-02

----------------------------------------

ADR-011

Problem-solving Content First

Decision

Korea Inside는 관광지 소개 중심 사이트가 아니라 외국인의 한국 여행 문제를 해결하는 decision-support / problem-solving platform이다. 콘텐츠는 사용자가 실수하지 않고 더 나은 결정을 하도록 돕는 것을 우선한다.

Reason

일반 관광 콘텐츠는 쉽게 복제될 수 있고 Korea Inside의 장기 경쟁력이 아니다. Korea Inside의 가치는 사용자가 실제 상황에서 불확실성을 줄이고 더 나은 결정을 하도록 돕는 데 있다.

Status

Accepted

Date

2026-07-07

----------------------------------------

ADR-012

No Mobile App First

Decision

개발 순서는 Landing Page -> Website -> PWA -> Mobile App 순서를 따른다. 모바일 앱부터 개발하지 않는다.

Reason

모바일 앱 개발보다 트래픽, 수요 검증, 콘텐츠 구조, 수익모델 검증이 먼저다. 앱은 충분한 사용자 수요와 운영 구조가 검증된 뒤 검토한다.

Status

Accepted

Date

2026-07-07

----------------------------------------

최종 원칙

----------------------------------------

결정은

기억이 아니라

기록으로 관리한다.
