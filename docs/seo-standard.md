# Korea Inside SEO Standard

## Document Role

이 문서는 검색 의도, 대표 URL, 색인, 메타데이터, 내부 링크, 구조화 데이터 기준만 관리한다.

문체와 콘텐츠 QA는 `content-writing-standard.md`, 시각 표현은 `design-system.md`, 다국어 SEO는 `multilingual-seo-strategy.md`를 따른다.

## 1. 검색 의도와 대표 URL

- 대표 검색 의도 하나에 대표 URL 하나를 사용한다.
- 유사 키워드만 바꾼 대량 페이지를 만들지 않는다.
- 기존 URL을 임의로 변경하지 않는다.
- 새 페이지를 제안하기 전에 기존 대표 URL과 검색 의도 중복을 확인한다.
- 대표 검색어는 자연스럽게 `title`과 `h1`에 반영한다.
- 첫 화면에서 대표 검색 질문에 즉답한다.
- 연관 검색어는 별도 유사 페이지보다 관련 소제목과 실제 FAQ에 통합한다.

## 2. URL

- 새 URL은 소문자, 하이픈, 짧고 명확한 표현을 사용한다.
- 공개 URL마다 self canonical을 사용한다.
- 리디렉션과 URL 변경은 별도 승인과 이전 계획 없이 실행하지 않는다.

## 3. Title, Description, Heading

- 각 대표 URL은 고유하고 의미 있는 `title`과 비어 있지 않은 meta description을 가진다.
- `title`은 대표 검색어와 페이지가 해결하는 문제를 자연스럽게 설명한다.
- meta description은 사용자가 얻는 답을 과장 없이 요약한다.
- 페이지마다 `h1`은 정확히 하나만 사용한다.
- `h2`와 `h3`는 실제 정보 계층에 따라 순서대로 사용한다.

## 4. 색인과 Sitemap

- 각 공개 페이지는 `index` 또는 `noindex` 의도를 명확히 결정한다.
- 검색 결과에 제공할 가치가 없는 중복·유틸리티·임시 페이지는 자동으로 색인 대상으로 간주하지 않는다.
- 색인을 허용한 새 대표 URL은 canonical과 sitemap 반영 여부를 함께 검토한다.
- sitemap에는 색인 가능한 canonical URL만 포함한다.

## 5. 본문 동등성과 HTML

- 핵심 내용은 HTML Text
- 이미지 안에 핵심 정보 금지
- 자동 번역 가능 구조 유지
- 모바일과 데스크톱에서 동일한 핵심 본문과 결론을 제공한다.
- 중요한 문구를 CSS 숨김이나 이미지 대체만으로 제공하지 않는다.

## 6. FAQ

- 실제 사용자 질문만 포함한다.
- 연관 검색 의도는 필요한 경우 FAQ와 소제목에 통합한다.
- 화면에 보이는 질문과 답변이 있을 때만 대응하는 FAQ 구조화 데이터를 사용한다.
- 중복 질문과 검색어 변형만을 위한 질문을 만들지 않는다.

## 7. 내부 링크

- 허브 → 세부 가이드 → 비교 페이지 → 관련 허브로 이어지는 탐색 경로를 설계한다.
- 링크 수를 기계적으로 채우지 않는다.
- 사용자의 다음 판단에 도움이 되는 관련 페이지에 설명적인 앵커 텍스트로 연결한다.
- 고립된 공개 페이지가 생기지 않도록 대표 진입 경로를 확인한다.

## 8. 이미지 검색 기준

- WebP or AVIF and other optimized modern image formats should be considered first.
- SVG may be used for icons, diagrams, and other vector content when appropriate.
- PNG may be used when transparency, diagram fidelity, or image-quality preservation requires it.
- Choose the format by file size, visual quality, browser compatibility, and purpose rather than by one mandatory extension.
- 정보 이미지에는 의미 있는 파일명과 맥락에 맞는 `alt`를 작성한다.
- 장식 이미지는 불필요한 키워드를 `alt`에 넣지 않는다.
- 이미지 텍스트를 핵심 본문 대체물로 사용하지 않는다.

## 9. 구조화 데이터

- 페이지의 실제 보이는 내용과 일치하는 유형만 사용한다.
- Breadcrumb, Article, Organization, FAQ 등은 페이지 역할과 Google 지원 조건에 맞을 때만 적용한다.
- 허위 Review 또는 AggregateRating을 생성하지 않는다.
- 구조화 데이터는 화면에 없는 주장, 평점, 후기, 가격을 추가하는 수단이 아니다.

## 10. 출처와 날짜

- 공식자료의 발표일, 조사·확인일, 페이지 수정일을 혼동하지 않는다.
- 변동 정보는 콘텐츠 기준에 따라 공식 출처와 확인일을 제공한다.
- 단순 수정일 갱신으로 콘텐츠 신선도를 가장하지 않는다.

## 11. 관리자·측정 연계

- 공개 페이지의 대표 검색 의도, canonical URL, 색인 정책, 제휴 추적 구조가 바뀌면 관련 관리자 대시보드 또는 측정 구조의 영향도 함께 점검한다.
- 이 점검은 관리자 코드 변경을 자동 승인하지 않는다.

## 12. 금지 사항

- 키워드 반복
- 과장 표현
- 클릭베이트
- 의미 없는 페이지 생성
- 유사 검색어별 대량 페이지 생성
- 내용과 불일치하는 구조화 데이터
- 승인 없는 URL 변경

## 13. 공개 전 기본 기술 QA

- 대표 검색 의도와 대표 URL이 중복되지 않는가
- 고유한 `title`, meta description, self canonical, 하나의 `h1`이 있는가
- 정상적인 HTTP 응답을 반환하는가
- 깨진 내부 링크가 없는가
- 모바일과 데스크톱의 핵심 본문이 동일한가
- `title`, `h1`, canonical, robots 지시, sitemap 반영이 서로 일치하는가
- `index` 또는 `noindex` 의도가 명확한가
- 이미지 `alt`, 표시 크기, 파일 크기와 로딩 성능이 적절한가
- Core Web Vitals 또는 동등한 성능 점검을 수행했는가
- 성능 수치는 페이지 유형과 실제 측정 환경을 고려하며, 근거 없는 절대 합격값으로 고정하지 않았는가
- 중요한 내부 링크가 크롤링 가능한 HTML 링크인가
- 내부 링크와 구조화 데이터가 실제 페이지 내용과 일치하는가
- 허위 리뷰·평점 마크업이 없는가
- 공개 전 기본 기술 QA의 실패 또는 미확인 항목을 보고했는가
