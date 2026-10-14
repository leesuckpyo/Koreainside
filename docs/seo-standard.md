# Korea Inside SEO Standard

## Document Metadata

- **Canonical file:** `docs/seo-standard.md`
- **Status:** ACTIVE / SEO & SERP SPECIALIZED STANDARD (저장소 반영 시 적용)
- **Version:** 1.0 — 기존 무버전 SEO Standard의 최초 버전 관리판
- **Effective date:** 2026-10-13
- **Scope:** Korea Inside의 영어 공개 콘텐츠 대상 58페이지와 이후 승인되는 영문 페이지의 SEO·SERP 개선 작업
- **Authority:** 현재 사용자 지시 → 최신 ACTIVE Public Content Master → Navigation Hub Architecture → 직접 적용되는 Specialized Standard → 본 SEO Standard. 다른 언어는 해당 Language Standard와 승인 절차를 추가 적용한다.
- **Related:** `AGENTS.md`, `docs/standards-hub.md`, `docs/content-writing-standard.md`, `docs/multilingual-seo-strategy.md`, `docs/measurement-framework.md`
- **Responsibility:** ChatGPT = 검색의도·SERP 해석·콘텐츠 및 추천 판단, 사용자 = 최종 공개문구·변경 및 Production 승인, Codex = 승인본 구현·기술 QA·Git·배포
- **Change policy:** 사용자 명시 승인에 따라 버전 갱신. 이 문서는 페이지 수정이나 Git/Production 실행 권한을 자체적으로 부여하지 않는다.

## Document Role

이 문서는 검색 의도, 대표 URL, 색인, 메타데이터, 내부 링크, 구조화 데이터, **실제 SERP 경쟁 분석 및 페이지별 검색 성과 개선 절차**를 관리한다. 기존 1~13절의 기술 SEO 규칙과 새 14~24절의 경쟁·측정 규칙을 함께 적용한다.

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

---

## 14. 영문 58페이지 공통 SEO + SERP 적용 원칙

### 14.1 목표와 범위

- 핵심 검색어의 **Google 검색 1위 도전**을 목표로 하되, 노출·클릭·CTR·평균 게재순위를 실측해 개선한다. 1위 달성을 보장하지 않는다.
- SEO는 검색엔진만을 위한 수정이 아니다. **검색자가 실제 질문에 답을 얻고 다음 여행 결정을 내릴 수 있게 하는 Humanization**을 최우선으로 한다.
- 58개 영어 페이지에 **동일한 검토·판단·승인·QA·측정 절차**를 적용하되, 같은 목차·H2 수·호텔 수·FAQ 수·이미지 수를 강제하지 않는다.
- 신규 페이지·대표 검색의도 변경·대규모 SEO 개편에서 본 절차를 적용한다. 단순 이미지·색상·링크 수정에 매번 전체 경쟁사 SERP 조사를 강제하지 않는다.
- 한 번에 58개 페이지를 다시 쓰지 않는다. 사용자 지정 범위(기본 1개 대표 URL) 내에서 조사 → 판단 → 승인 → 구현 → 측정한다.
- 기존 `DONE LOCKED`, `CONTENT LOCKED`는 보호한다. 실질적 검색의도 충돌, 명확한 사실오류, 사용자 변경 지시 등 승인된 근거가 있을 때 **해당 위치만** 검토한다.

### 14.2 매 작업 시작 시 기준 확인

- **새 대화방에서 영문 SEO/SERP 첫 실제 작업 시 1회:** 최신 승인 Public Content Master, Navigation Standard, 이 `docs/seo-standard.md`, 최신 유효 Handover를 확인한다. 필요한 경우 해당 페이지의 Research/Approved Copy 및 실제 English Production HTML을 추가 확인한다.
- 이미 같은 방에서 읽었고 SHA·Version·내용이 같으면 반복해 읽지 않는다. 문서 변경, 최신 Handover 변경, 새 작업 범위 또는 기준 충돌 시 필요한 항목만 다시 확인한다.
- Codex는 저장소 `AGENTS.md`와 `docs/standards-hub.md`의 기존 전문 Standard 참조를 따른다. 이 파일을 별도 실행 승인으로 간주하지 않는다.
- 다음 페이지를 Handover에서 임의 선택하거나 미승인 Batch를 자동 개시하지 않는다.

## 15. 검색의도(Search Intent)와 대표 URL 확정

페이지별로 다음을 결정한다.

1. 현재 페이지 역할: Hub / Stay Detail / Comparison / Travel Detail / Transport / Service / 기타.
2. **대표 검색 의도 1개**와 해당 의도를 대표할 기존 canonical URL 1개.
3. 검색자가 첫 화면에서 원하는 결론, 결정을 좌우하는 조건, 다음에 확인할 실무정보.
4. 대표 쿼리와 의미상 연관된 보조 쿼리. 검색량만으로 페이지 역할을 바꾸지 않는다.
5. 다른 Korea Inside URL과의 **검색의도 중복(cannibalization) 위험**: 제목이 비슷하다는 이유만으로 확정하지 않고 실제 내용·GSC 쿼리/URL 분포 및 SERP를 함께 본다.

**서울 숙소 예:**

- `accommodation.html` = 서울에서 *어느 지역에 머무를 것인가*를 결정하는 대표 Stay Hub.
- `best-area-for-first-time-visitors-seoul.html` = 첫 방문자의 숙소 위치와 여행조건을 따로 판단하는 Decision Detail.
- `where-to-stay-in-myeongdong.html` = 명동 내 어느 블록·호텔·객실이 맞는가를 판단하는 Area Stay Detail.

이 구분은 현재 페이지 역할을 설명하는 예시이며, 실제 순위나 URL 간 충돌이 검증됐다는 주장이 아니다.

## 16. 실제 SERP 조사 절차 — 관찰과 추론을 분리

### 16.1 조사 조건

- 기본 대상 시장: **미국 Google 검색, 영어 검색자**. 필요 시 실제 유입국에 맞추되 국가·언어를 반드시 기록한다.
- 대표 검색어와 검색의도가 유사한 보조 검색어를 **필요한 개수만** 선정한다(보통 3~5개). 검색어 수를 채우기 위해 무관한 쿼리를 추가하지 않는다.
- 조사 일자, 검색 국가/언어, 기기(Desktop/Mobile), 사용된 접근 경로와 개인화·위치제한 여부를 기록한다.
- 실제 검색결과에서는 상위 자연검색 URL과 제목·스니펫, 광고/OTA/공식기관의 점유, 지도·이미지·동영상·PAA·AI 요약 등 **화면에 관찰된 SERP 요소만** 기록한다.
- 실시간 순위를 직접 확인하지 못한 경우 **`SERP RANK NOT VERIFIED`**로 명시한다. 경쟁사 제목, Ahrefs/Semrush 지표, 일반 웹검색 결과만으로 특정 미국 Google 순위를 만들어내지 않는다.

### 16.2 검증할 질문

- 검색자가 정보를 찾는가, 지역을 비교하는가, 호텔 자체를 고르는가, 즉시 예약하는가?
- 상위 페이지가 첫 화면에서 어떤 결론을 주는가? 실제로 검색자의 다음 문제까지 해결하는가?
- Google 검색결과 화면에서 사용자가 클릭할 이유가 무엇인가? 표시 Title과 스니펫이 우리 내용과 일치하는가? (검색엔진이 이를 자동 재작성할 수 있음을 인정)
- 상위 문서에 있는 사실·지도·위치·동선 중 우리에게 **필요하지만 없는 것**은 무엇인가?
- 경쟁사에 있어도 검색의도·사실검증·실용성이 낮아 **가져오지 말아야 할 요소**는 무엇인가?

### 16.3 결과의 증거 등급

- **관찰:** 확인한 SERP, 실제 경쟁사 공개 페이지, 실제 Korea Inside Production/GSC 데이터.
- **추론:** 순위에 유리할 가능성이 있는 콘텐츠·내부링크·외부링크·정보구조의 영향 가설.
- **제안:** Korea Inside에 실제 적용할 편집 및 SEO 개선안.

관찰·추론·제안을 하나의 확정 사실처럼 섞지 않는다. 검색결과 순위의 원인을 직접 증명할 수 없으면 그렇게 명시한다.

## 17. 경쟁사 벤치마크 — 고정 4축, 실제 SERP로 재선정

다음은 **2026-10-13 기준 편집·SEO 벤치마크 4축**이다. 고정된 구글 순위표가 아니다.

| 벤치마크 | 우선 분석할 경쟁력 | Korea Inside 적용 시 질문 |
|---|---|---|
| **KoreaToDo** | 지하철 출구, 공항버스, 호텔/정류장 위치, 마지막 도보 등 실행 가능한 현장정보 | 우리 페이지에서 실제 이동·위치 판단이 구체적인가? |
| **Travel Lemming** | 대표 쿼리 응답, 지역 비교, 한눈에 보는 지도, 검색결과 진입과 문서 구성 | 첫 화면에서 검색의도에 맞는 답과 지역 구조가 보이는가? |
| **The Invisible Tourist** | 실제 숙박·방문에 기반한 서술, 호텔의 불편과 제한 조건, 구체적인 시각 증거 | 우리도 **확인된 근거**로 신뢰를 형성하는가? 직접 경험을 꾸미지 않는가? |
| **The Soul of Seoul** | 자연스러운 현지 맥락, 지역 주제의 폭·깊이, 연관 콘텐츠 간 연결 | Travel → Stay → Transport/Activity의 문맥형 클러스터가 충분한가? |

- 위 4개를 모든 페이지의 필수 직접 경쟁사라고 가정하지 않는다. **해당 쿼리의 실제 상위 페이지**가 우선이며, 상황에 따라 In My Korea·The Broke Backpacker·공식 관광사이트 등의 비교군을 선택한다.
- Agoda/Expedia/Trip.com/Booking 등 OTA는 검색화면 경쟁 **관찰 대상**이지만, Korea Inside가 그대로 따라야 하는 독립 여행가이드의 편집 모델은 아니다.
- 경쟁사의 호텔·교통·가격·운영 정보는 그대로 복사하거나 공식 사실로 승격하지 않는다. 해당 페이지별 공식/1차 출처로 재검증한다.
- 도메인 권위·트래픽·백링크 등 도구 추정 지표는 **도구명·측정일·측정대상**과 함께 기록하고 Google 직접 순위 요인 또는 해당 URL 실적처럼 표현하지 않는다.
- 경쟁사 링크가 사라지거나 SERP가 달라지면 대조군을 바꾼다. 경쟁사 이름보다 **비교 방법**이 고정 기준이다.

## 18. Korea Inside와 비교할 콘텐츠·SEO 항목

**반드시 확인할 비교축** (필요하지 않은 자료를 채우기 위해 새 섹션을 만들지 않음):

1. 대표 검색질문의 답: Title·H1·Lead·Quick Answer의 의도 일치와 중복 여부.
2. 정보 깊이: 실제 장소·호텔·역·출구·환승·짐·마지막 도보·운영 및 예약 조건.
3. 시각 전달: 지역 Orientation Map, 사진, 호텔 위치 및 이동관계. 지도는 실제 공간 판단에 기여할 때만 고려.
4. 검증과 신뢰: 공식자료, 확인일, 직접 경험 여부의 정직한 구분, 사실과 편집 판단의 분리.
5. 사이트 연결: 대표 Hub ↔ 적합한 Detail ↔ Transport/Travel/Stay ↔ 자연스러운 제휴 행동.
6. 기술 SEO: canonical, robots, sitemap, crawlable links, hreflang 영향, Mobile, Core Web Vitals, 실제 보이는 내용과 schema 일치.
7. SERP 클릭 경쟁력: 결과 제목·스니펫·질문 부합도; 근거 없는 `best`/최저가/최신 연도 남용 금지.
8. 전환: 여행자의 선택이 충분히 끝난 지점에서 숙박·서비스 CTA가 자연스럽게 이어지는가.

**평가 결론은** `KEEP(현행 유지) / IMPROVE(근거 있는 부분 개선) / HOLD(사실·정책 재검증 필요) / NOT VERIFIED(측정 불가)`로 정리한다.

## 19. Page Family별 SEO 적용 방향

### 19.1 Stay Hub / Area Stay Detail

- Hub: *서울에서 어디에 묵을 것인지* 비교하고 하위 지역 Stay Detail로 연결한다. 지역별 호텔 목록·사진을 기계적으로 늘리지 않는다.
- Area Stay Detail: 블록·역 출구·공항 이동·호텔 입구·객실/침대/인원·짐·소음·체크인·취소 등 기존 **Accommodation Decision Layer**를 보호하고 필요한 근거만 정교화한다.
- 호텔 수·호텔 순위를 경쟁사에 맞춰 늘리거나 바꾸지 않는다. 호텔별 선택 이유와 제외 이유를 보존한다.
- 대표 지도는 실제 지역 선택에 도움이 될 때만 제안하며, 지도만 추가해 SEO가 올라간다고 주장하지 않는다.

### 19.2 Travel / Attractions / Events

- 단순 관광지 숫자 경쟁보다 그 지역에서 **무엇을, 어느 시간대에, 어떤 순서로** 경험하는지 설명하는 검색의도와 실제 동선을 우선한다.
- Travel Guide에서 다음 질문이 생길 때 Stay·Airport·Transport·Tour/Experience로 관련 링크를 연결한다.
- 날짜성 행사와 운영정보는 현재 상태를 공식자료로 확인한다. 자동 연도 표시는 사실 확인의 대체가 아니다.

### 19.3 Transport / Airport / eSIM / Payment 등 Service

- 검색자의 실행 순서·선택 조건·실패 가능성을 우선한다(예: 탑승 위치 → 환승 → 출구 → 마지막 이동).
- 상품 비교·구매 검색의도가 강하면 필요한 CTA가 앞에 올 수 있으나, 선택근거와 제약조건 없이 CTA를 반복하지 않는다.

## 20. 변경 제안 우선순위와 보호 경계

- **우선 순위:** (a) 사용자 판단에 실제로 영향을 주는 오류·누락, (b) 대표 검색의도와 문서 역할 불일치, (c) 문맥형 내부링크/구조상 마찰, (d) 기술 SEO 결함, (e) 그 외 표현·시각 개선. 실제 상황에 따라 영향도와 증거로 순서를 조정한다.
- Title·Meta·H1은 경쟁사와 달라 보인다는 이유만으로 변경하지 않는다. 검색의도 불일치나 GSC CTR 문제 등 근거가 있을 때 제안한다.
- FAQ/Schema는 실제 질문과 Google 지원 조건에 따라 적용한다. 검색결과 강조 표시를 얻겠다는 이유만으로 부적합한 FAQPage·평점 Schema를 추가하지 않는다.
- 내부링크는 **현재 문단의 다음 여행 질문**을 해결할 때만 추가한다. 전역 메뉴 개편은 Navigation Standard와 사용자 명시 승인이 필요하다.
- 사진·지도·인포그래픽은 정확성·사용권·본문 역할을 먼저 확인한다. 이미지 속 텍스트로 핵심 정보를 대체하지 않는다.
- 기존 URL, 브랜드/호텔명, 사실·수치·추천 판단, 공개문구 잠금, 이미지·alt, 제휴 href/subid/tracking, `common.js` 및 공통 레이아웃을 보호한다.
- 영어 개선이 다른 언어의 사실·구조 또는 hreflang에 영향을 주는 경우 **영어 승인범위와 다국어 파급범위를 분리 보고**하고 해당 언어의 별도 승인 없이 일괄 변경하지 않는다.

## 21. 사용자 승인 및 Codex 실행 권한

1. ChatGPT가 SERP/실제 페이지/GSC를 해석하고 `관찰 → 격차 → 개선안 → 보존항목`을 제시한다.
2. 개선할 **공개 문구·SEO 판단·추천**은 ChatGPT가 작성하고 사용자가 최종 승인한다.
3. Codex는 승인된 `exact copy`/기술변경만 구현하고 범위 밖 최적화를 하지 않는다. Codex는 검색의도·추천순위·경쟁전략의 편집 판단자가 아니다.
4. 사용자가 요청한 범위에서 정적 QA/브라우저 QA를 수행하고 사용자 승인 후 Git/Production을 진행한다.
5. `DONE LOCKED / CONTENT LOCKED`는 단순 표현 개선이나 새 방 진입을 이유로 해제하지 않는다.

**이 문서는 정책 기준이며 실행 명령이 아니다.** 사용자가 명시적으로 요청하지 않았다면 Codex 지시문을 자동 작성하지 않고, 별도 MD·스크립트·PR·Commit·Push·Production을 자동으로 만들지 않는다.

## 22. 성과 측정 — 검색 순위와 전환의 분리

- 변경 전 가급적 **Google Search Console**의 대상 URL·검색어·국가·기기별 최근 28일 및 90일 데이터를 기준선으로 확인한다: `impressions`, `clicks`, `CTR`, `average position`.
- 실시간 미국 Google SERP의 화면상 위치와 GSC 평균 게재순위는 서로 다른 값이다. Ahrefs/Semrush 트래픽·키워드·Authority 추정과도 혼동하지 않는다.
- 공개 후 28일·56일 등 비교 가능한 기간을 정해 **동일 검색어·동일 URL·국가·기기** 기준으로 변화를 검토한다. 색인 지연·표본 부족·시즌 변동·SERP 구성 변화를 고려한다.
- 검색 노출이 늘어도 사용자의 페이지 이동·숙박/제휴 CTA 이용이 감소할 수 있다. 필요한 경우 내부링크 클릭·Affiliate CTR 등을 별도로 확인한다.
- 검색 데이터에 접근할 수 없으면 `NOT AVAILABLE` 또는 `NOT VERIFIED`로 보고한다. 수치·순위·성공률을 생성하지 않는다.
- 개선했다고 즉시 순위가 올라가는 것은 아니며, 성과가 작으면 무조건 전체 페이지를 재작성하지 않는다. 관찰된 원인에 따라 필요한 범위만 재점검한다.

## 23. 반복 작업의 최소 산출물 — 중복 MD 금지

**페이지별 검토 보고는 기본적으로 다음 6개만 간결하게 제시한다.**

| 항목 | 최소 기록 |
|---|---|
| 대상 | URL, Page Family, 대표 검색어/검색의도 |
| 현재 상태 | Title/H1, 콘텐츠 역할, GSC 기준선(있으면) |
| SERP 근거 | 조사일·국가·기기·상위 실제 URL·검색화면 특징·미검증 범위 |
| 경쟁 차이 | 우리 장점, 경쟁사 장점, 적용하면 안 되는 요소 |
| 조치 판단 | KEEP / IMPROVE / HOLD / NOT VERIFIED 및 이유 |
| 사용자 선택 | 수정할 정확한 문구/위치, 영향 범위, 승인 필요 여부 |

- 이미 승인된 Research/Approved Copy/경쟁사 감사 내용은 재사용한다. 필요하지 않은 중간 MD, 반복 완결성 검사, 무의미한 승인루프를 만들지 않는다.
- 완료한 페이지를 자동 재개방하지 않는다. 실제 검색성과나 오류가 신규 근거일 때만 좁게 제안한다.
- 모든 페이지의 우선순위를 사전에 획일화하지 않는다. 사용자 지정 페이지·실제 GSC 성과·검색의도 중요도를 우선한다.

## 24. 버전 및 문서 관리

- **SEO + SERP 운영기준의 단일 Source of Truth:** `docs/seo-standard.md`.
- `AGENTS.md`와 `docs/standards-hub.md`는 이미 이 SEO Standard를 참조하므로 동일 내용을 중복해 복사하지 않는다. 추후 참조 변경이 필요할 때만 사용자 승인 후 처리한다.
- 본문 1~13절은 기존 SEO Standard에서 승계한다. 14~23절은 이번 SEO + SERP 체계 보강이다.
- 이번 Version 1.0은 기존 무버전 파일에 첫 정식 Version을 부여한 판이다. 이후 정책 변경은 변경 이유·유효일·Version을 명시한다.
- 경쟁사 4곳은 **비교 관점**으로 유지하고, 실제 순위·유입·DR·페이지 가격·날짜 같은 변동 수치를 Standard에 고정하지 않는다.
- Standard는 정책, Handover는 상태, GSC/SERP는 시점별 관측값, Approved Copy는 잠금된 공개문구, Codex는 기술 구현을 담당한다.

### Version 1.0 변경기록 — 2026-10-13

- 기존 SEO Standard 1~13절을 보존하면서 문서 역할에 SERP 경쟁 분석을 포함.
- 영문 58페이지의 공통 검토 절차(동일 방법·페이지별 다른 판단), 새 방 1회 확인 규칙 추가.
- KoreaToDo / Travel Lemming / The Invisible Tourist / The Soul of Seoul 비교축 고정(실제 순위표 아님).
- 실시간 SERP 관찰과 추론 분리, GSC 기준선·성과 비교 및 Cannibalization 검토 추가.
- 기존 CONTENT LOCK, 제휴·다국어·공통 컴포넌트 보호 및 Codex 역할 분리 재확인.
