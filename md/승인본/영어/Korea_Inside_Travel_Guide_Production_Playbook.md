# Korea Inside Travel Guide Production Playbook

**File:** `Korea_Inside_Travel_Guide_Production_Playbook.md`  
**Status:** ACTIVE / PRODUCTION PLAYBOOK  
**Version:** 1.0  
**Effective date:** 2026-09-15  
**Applies to:** Korea Inside Travel Guide 계열의 신규 제작, 대규모 개편, 시각 보강, Detail 확장  
**Governing standard:** `Korea_Inside_Public_Content_Master_Standard.md` Version 1.2 이상 최신 승인본  

> 이 문서는 Korea Inside의 Travel Guide를 **매 방, 매 지역에서 같은 품질로 제작하기 위한 실행 표준**이다.
>
> 이 문서는 Master Standard를 대체하지 않는다.
>
> **Master Standard = 왜 그렇게 하는가**  
> **Production Playbook = 정확히 어떻게 만드는가**  
> **Current Handover = 지금 어디까지 했는가**

---

# 0. 문서 목적

Korea Inside는 방이 바뀌거나 작업자가 달라져도 페이지 품질이 흔들리면 안 된다.

이 Playbook의 목적은 다음을 고정하는 것이다.

- 조사 순서
- 검색의도 판단 순서
- PUBLIC COPY 작성·승인 순서
- Humanization 감사 방법
- 체험·숙소·교통·제휴 연결 방식
- Hero / 지도 / 본문 사진의 역할과 적용 방식
- Codex의 역할과 재량 범위
- 로컬 QA / 화면 QA / Production QA
- Git 보호와 배포 절차
- DONE LOCKED 보호
- 방 이동 후 동일 품질 복원 절차

중요:

> **고정하는 것은 제작 절차와 품질 기준이다.**  
> **지역별 목차를 고정하는 것이 아니다.**

홍대와 인사동, 성수와 서울역은 같은 페이지가 아니다.
같은 목차를 복제하면 오히려 품질이 낮아진다.

---

# 1. 적용 우선순위

작업 판단 우선순위는 Master Standard를 그대로 따른다.

1. Humanization
2. 사용자 문제 해결과 사실 정확성
3. 검색 의도 / SEO
4. 콘텐츠 구조
5. 디자인
6. 제휴 / 수익화
7. 개발 편의성

하위 단계가 상위 단계와 충돌하면 상위 원칙을 따른다.

공통 다음 파일·영역은 사용자 명시 승인 없이 수정하지 않는다.

- header
- navigation
- footer
- `common.js`
- mobile hamburger

---

# 2. 세 문서의 역할 분리

## 2.1 Master Standard

영구적인 상위 원칙을 관리한다.

예:

- Humanization
- 검색 의도
- 사실 정확성
- 페이지 역할
- Accommodation Decision Layer
- 제휴 원칙
- 사진 권리
- 디자인 철학
- SEO 기본 원칙

Master Standard에는 시간이 지나면 바뀌는 페이지 완료 상태를 넣지 않는다.

## 2.2 Production Playbook — 이 문서

Travel Guide를 실제로 만드는 **공정과 품질 Gate**를 관리한다.

예:

- 조사 완료 전 원고 작성 금지
- PUBLIC COPY 승인 전 Codex 구현 금지
- Hero 구조
- 사진 최적화 방식
- Codex exact implementation
- Browser QA
- Production QA

이 문서는 특정 지역의 현재 완료 상태를 기록하지 않는다.

## 2.3 Current Handover

시간에 따라 바뀌는 현재 상태만 관리한다.

예:

- DONE LOCKED 페이지
- 현재 미배포 변경
- 마지막 commit
- 현재 후보 이미지
- 다음 작업
- 보호해야 할 사용자 git 변경
- 승인 대기 사항

새 방에서는 항상:

**Master Standard → Production Playbook → Current Handover**

순으로 읽는다.

---

# 3. 새 방 / 새 작업 시작 규칙

실제 작업 전에 반드시 아래를 확인한다.

1. 최신 Master Standard 전체
2. 이 Production Playbook 전체
3. 최신 Current Handover 전체
4. 사용자가 새로 제공한 완료보고 / 캡처 / 승인사항
5. 필요 시 `git status --short`
6. 필요 시 `git log -5 --oneline`
7. 필요 시 `origin/main` ahead / behind

사용자가 새로 제공한 완료보고는 오래된 Handover보다 우선한다.

작업 시작 전 먼저 아래 4가지만 보고한다.

1. Master Standard MD 경로
2. 전체 내용 확인 여부
3. 이번 작업에 적용할 핵심 기준
4. 작업 대상 파일

그 후에만 실제 작업을 시작한다.

---

# 4. Travel Guide Production Pipeline — 전체 고정 순서

기본 제작 순서:

**Context Restore**  
→ **Search Intent**  
→ **Research**  
→ **Editorial Structure**  
→ **PUBLIC COPY**  
→ **Humanization**  
→ **Action / Affiliate Mapping**  
→ **Visual Plan**  
→ **User Approval**  
→ **Codex Exact Implementation**  
→ **Static QA**  
→ **Browser QA**  
→ **User Final Approval**  
→ **Stage / Commit / Push**  
→ **Production QA**  
→ **DONE / DONE LOCKED / Handover**

각 단계를 건너뛰지 않는다.

단순 이미지 위치 이동, 색상 조정, 링크 수정처럼 저위험 작업은 필요한 Gate만 축약할 수 있다.

---

# 5. Quality Gate 0 — Context Restore

## PASS 조건

- Master Standard 최신판 확인
- Production Playbook 확인
- Current Handover 확인
- DONE LOCKED 상태 승계
- 현재 미배포 변경 확인
- 보호해야 할 사용자 파일 확인

## FAIL 조건

- 이전 방 상태를 추측함
- 사용자 승인 사항을 다시 처음부터 논의함
- DONE LOCKED를 근거 없이 재감사함
- 현재 git 변경을 모르고 구현을 시작함

---

# 6. Quality Gate 1 — Search Intent / Page Role

신규 공개 페이지 또는 검색 의도가 크게 바뀌는 경우만 미국 기준 검색 수요와 현재 SERP를 먼저 확인한다.

단순 Hero, 사진, CTA, 색상 수정에는 SERP 조사를 반복하지 않는다.

## 반드시 정할 것

### A. 대표 검색 의도 1개

예:

- 지역 전체 이해
- 특정 장소 Detail
- 비교
- 숙박 선택
- 이동
- 일정
- 음식
- 행사
- 체험 / 서비스

### B. 사용자가 해결하려는 문제 1문장

예:

> 처음 서울에 온 여행자가 인사동에서 무엇을 보고 어떤 순서로 움직여야 하는지 결정하게 한다.

### C. 페이지 역할

- Hub
- Detail
- Comparison
- Stay / Hotel
- Transport / Airport
- Food
- Events / Nightlife / Culture
- Itinerary
- Service / Affiliate

## 원칙

하나의 검색 의도에는 하나의 대표 URL을 사용한다.

페이지 수를 늘리기 위해 같은 의도를 여러 URL로 쪼개지 않는다.

---

# 7. Hub → Detail 분리 기준

Travel Guide 한 페이지에 많은 내용을 담았다고 해서 Detail을 만들 수 없는 것이 아니다.

Hub는:

> **갈지 말지 / 어떻게 이해할지 판단**

Detail은:

> **가기로 했다면 어떻게 이용할지 깊게 해결**

을 맡는다.

## 독립 Detail 생성 조건

다음 중 3개 이상을 만족하면 독립 페이지를 적극 검토한다.

1. 독립 검색의도가 분명하다.
2. Hub보다 더 깊게 설명할 실제 내용이 있다.
3. 위치·시간·예약·비용·실패 가능성 등 별도 판단이 필요하다.
4. Hub 내용을 복사하지 않고 새 정보 밀도를 만들 수 있다.
5. 별도 체험 / 예약 / 이동 행동으로 연결된다.
6. Search Console 또는 SERP에서 독립 수요가 확인된다.

예시:

- 지역 Travel Guide → 특정 시장 Detail
- 지역 Travel Guide → 특정 복합문화공간 Detail
- 지역 Travel Guide → 전통차 / 공예 / 체험 Detail
- 지역 Travel Guide → 지역 + 인접지역 One-Day Route

단, 예시는 제작 의무가 아니다.

---

# 8. Quality Gate 2 — Research

조사는 3층으로 분리한다.

## 8.1 사실층

우선 출처:

- 한국관광공사 / Visit Korea
- Visit Seoul
- 서울시 / 구청
- 코레일 / 공항철도 / 서울교통공사
- 공식 행사 주최기관
- 공식 박물관 / 시장 / 공연장 / 매장
- 호텔 / 브랜드 공식 사이트

확인 항목:

- 위치
- 운영시간
- 휴무
- 교통
- 출구
- 행사 날짜
- 비용
- 예약조건
- 입장 조건
- 공식 프로그램

## 8.2 경험층

반복 패턴을 확인한다.

- Reddit
- Google Reviews
- TripAdvisor
- YouTube
- 독립 블로그
- 공개 여행 후기
- 사용자가 제공한 Naver 자료

한 사람의 경험을 전체 사실로 일반화하지 않는다.

## 8.3 편집 판단층

최종 결론은 Korea Inside가 한다.

예:

- 어디서 시작하는 것이 편한가
- 이 지역에서 무엇을 빼야 하는가
- 누구에게 맞지 않는가
- 어떤 일정에 체험을 넣으면 좋은가
- 숙박할 가치가 있는가

## Research PASS 조건

- 핵심 사실이 공식자료로 확인됨
- 경험층에서 반복되는 마찰이 확인됨
- 사실 / 경험 / 편집 판단이 섞이지 않음
- 직접 경험하지 않은 것을 직접 경험한 것처럼 쓰지 않음

---

# 9. Quality Gate 3 — Editorial Structure

목차를 먼저 채우지 않는다.

먼저 다음을 정한다.

1. 이 지역의 실제 성격
2. 여행자가 처음 막히는 지점
3. 어디서 시작해야 하는지
4. 시간대가 바뀌면 무엇이 달라지는지
5. 실제 이동 순서
6. 어디에서 쉬고 먹고 쇼핑하는지
7. 누구에게 맞고 안 맞는지
8. 다음 행동은 무엇인지

## 기본 구조 후보

필요한 것만 선택한다.

- Hero answer
- At a Glance / Orientation
- Where to Start
- Main Route
- Alternative Route
- Food
- Shopping
- Culture
- Cafes
- Night / Evening
- Experiences
- Mistakes
- Traveler Types
- Stay or Visit
- Where to Stay bridge
- Events / Current
- Final recommendation
- FAQ

모든 지역에 모든 섹션을 넣지 않는다.

---

# 10. Quality Gate 4 — PUBLIC COPY

공개 문구의 작성과 최종 판단 권한은:

- 사용자
- ChatGPT

에게만 있다.

Codex는 PUBLIC COPY를 자체적으로 작성하거나 개선하지 않는다.

## 작성 목표

페이지를 읽은 여행자가:

> **“그래서 나는 이렇게 하면 되겠다.”**

라고 판단할 수 있어야 한다.

## 반드시 드러낼 것

- 기본 추천
- 대안
- 예외 조건
- 누구에게 맞는지
- 누구에게 안 맞는지
- 실제 이동 마찰
- 시간대
- 체력 / 짐 / 아이 / 밤 일정 같은 현실 조건

## 금지

- 관광명소 백과사전식 나열
- 모든 선택지 칭찬
- `Top 20`만으로 끝나는 구성
- 직접 경험한 것처럼 쓰기
- OTA형 추천문 반복

---

# 11. Quality Gate 5 — Humanization

Humanization은 감성적인 문장을 만드는 작업이 아니다.

목표:

> **AI의 생산력을 쓰되 인간의 판단 구조로 결과물을 교정한다.**

작성 기반:

**확인된 사실 + 현지 맥락 + 편집 판단**

## 감사 항목

- made the list because
- stayed on our list
- is here because
- our value choice
- perfect for
- ideal for
- one of the best
- useful / works best / works well / makes sense 과반복
- trade-off / compromise 기계적 반복
- not X but Y 반복
- 추천 이유 → 교통 → 단점의 기계적 반복
- 비슷한 첫 문장 반복
- 이름만 바꾼 동일 결론
- 모든 선택지를 긍정하는 OTA 문체

## Humanization PASS

- 선택지마다 존재 이유가 다름
- 실제 마찰이 보임
- 부정적 판단도 필요하면 말함
- 누구에게 안 맞는지도 알 수 있음
- 여행자의 실제 하루가 떠오름

## DONE LOCKED

최종 감사와 사용자 승인을 통과한 문구는 DONE LOCKED로 보호한다.

다음 경우에만 다시 연다.

1. 사실 오류
2. 사용자 방향 변경
3. DONE LOCKED 이후 새 문구에서 AI-template 재발

---

# 12. Quality Gate 6 — Schedule / Experience / Affiliate Mapping

체험 제휴는 광고를 끼우는 작업이 아니다.

체험은 여행자가:

> **“이 일정에서 실제로 이걸 할 수 있구나.”**

라고 판단하고 자기 스케줄을 조정하게 만드는 실행 도구다.

Korea Inside는 정답 일정 하나를 강요하지 않는다.

- 기본 동선
- 표준 일정
- 시간대
- 선택 기준

을 제시하고, 여행자가 체험을 넣거나 빼면서 일정을 완성하게 한다.

## 고정 원칙

> **Relevant section → matching action → matching affiliate**

## 제휴 개수

기계적 상한 없음.

본문과 직접 맞는 체험·서비스라면 적극적으로 검토한다.

잘못된 방향:

- 제휴가 많아 보일까 봐 필요한 CTA를 억지로 제거
- 수익을 위해 무관한 CTA 삽입

정확한 기준:

> **본문과 맞으면 적극 적용 / 엉뚱하면 제외**

## 체험 CTA 전 확인

가능하면 다음을 확인한다.

- 위치
- 소요시간
- 언어
- 가격
- 최소인원
- 예약 필요 여부
- 즉시확정 여부
- 취소조건
- 어느 시간대에 넣기 좋은지
- 실제 동선과 충돌하는지
- 제휴 deep link 정상 여부

## 체험 배치

본문이 먼저:

- 왜
- 누구에게
- 언제
- 어떤 조건에서

를 설명한다.

그 다음 CTA가 실행을 담당한다.

독립 광고 섹션을 만들 필요는 없다.

## Stay 연결

Travel Guide가 지역에서의 하루를 설명한 뒤:

- 이 지역에 묵을 가치가 있는지
- 어느 동선에 숙박이 유리한지

판단이 생기면 Stay Guide 또는 대표 숙박 선택으로 연결한다.

---

# 13. Quality Gate 7 — Visual Plan

사진을 먼저 많이 넣지 않는다.

각 이미지에는 역할이 있어야 한다.

## 시각 역할 예

- Hero = 지역의 얼굴
- Orientation map = 공간관계 이해
- Main street = 첫인상 / 보행 흐름
- Shopping = 쇼핑 행동
- Culture = 문화적 전환
- Food = 식사 선택
- Craft = 디테일
- Evening = 하루 후반 전환

사진 숫자를 채우기 위해 중복 사진을 넣지 않는다.

---

# 14. Travel Guide Hero Family — 고정 시각 규칙

Travel Guide Hero는 Stay Hero family와 같은 시각 언어를 사용한다.

## 기본 구조

**wide representative photo**  
+ **dark overlay**  
+ **real HTML breadcrumb**  
+ **real HTML H1**  
+ **existing short intro / answer**

## 고정 원칙

- 이미지 파일에 H1 삽입 금지
- Hero H1은 실제 HTML
- H1은 페이지당 정확히 1개
- `data-guide-year="current"` marker 보존
- Hero에 affiliate CTA 없음
- 가격 / rating / 할인 / review count 없음
- 광고 banner처럼 만들지 않음
- 지역 첫인상 전달이 목적

## 이미지 성격

선호:

- 실제 지역
- 보행자
- 거리 구조
- 건축 / 상점 / 공간 분위기
- 현재 여행자가 실제로 보는 장면

피함:

- 특정 브랜드만 화면을 지배
- 장소를 잘못 대표하는 인접지역 사진
- 랜드마크 한 개만 보여 지역 전체를 왜곡
- 세로사진을 무리하게 wide crop

## 이미지 성능

Hero는 LCP 후보이므로:

- lazy loading 금지
- 필요 시 `fetchpriority="high"`
- width / height 또는 aspect-ratio 명시
- CLS 최소화

웹용 Hero가 원본 4K~5K 그대로라면 품질을 유지하면서 최적화한다.

기본 참고 범위:

- 긴 변 약 2000~2200px 수준
- 원본 aspect ratio 우선
- 필요 시 mobile crop 별도

단, 이미 최적화돼 있으면 재인코딩하지 않는다.

## Mobile

- Hero가 지나치게 얇아지지 않음
- H1 overflow 0
- year marker 어색한 단독 줄바꿈 방지
- 주요 피사체 유지
- overlay 가독성 유지

---

# 15. 본문 Editorial Image — 고정 규칙

본문 사진은 Hero와 역할이 다르다.

## 기본 표시 체계

- 약 max-width 740px 계열
- 가운데 정렬
- 원본 비율 유지
- mobile에서 container 폭 사용
- lazy loading 가능
- width / height 명시

## 웹용 파일

실제 표시폭 740px 기준 Retina 대응으로:

- 가로 약 1480px 수준을 기본 참고

단:

- crop을 위한 숫자가 아니다.
- 원본 비율 유지가 우선이다.
- 이미 적절하면 재인코딩하지 않는다.

## 배치 원칙

사진은 해당 문단의 의미와 직접 연결한다.

금지:

- 섹션 수에 맞춰 기계적으로 한 장씩 배치
- Hero와 동일 사진을 바로 아래에 반복
- 관련 없는 사진을 단순 장식으로 배치

## 중복 사진 처리

Hero와 같은 사진이 본문에 이미 있다면:

1. 삭제가 필요한지
2. 더 적절한 본문 위치로 이동할지
3. 다른 역할의 사진으로 교체할지

판단한다.

사용자 승인 전 임의 삭제하지 않는다.

---

# 16. 사진 권리 / Credit

사진 우선순위는 Master Standard를 따른다.

- 한국관광공사 / 공공누리
- Wikimedia Commons
- Visit Seoul 등 권리 명확 자료
- 직접 촬영
- 사용조건 확인된 공식 이미지

사용자는 Wikimedia Commons의 실제 지역 사진을 선호한다.
사진이 부족하거나 더 나은 실제 지역 장면이 필요하면 Wikimedia Commons를 적극 검토한다.

## 원본과 웹용 분리

원본 보관 파일은:

- 이동 금지
- 삭제 금지
- 덮어쓰기 금지

웹용 최적화본만 repository `images/...`에 복사한다.

## Credit

기존 Korea Inside 형식을 우선 재사용한다.

예:

- `Photo: Korea Tourism Organization / [photographer] · KOGL Type 1`
- `Photo: Wikimedia Commons / [author] · [license]`

출처 표기만으로 사용권이 생긴다고 가정하지 않는다.

---

# 17. Infographic Map — 사용 조건

모든 Travel Guide에 지도를 기계적으로 만들지 않는다.

## 지도가 필요한 경우

- 지역이 여러 하위구역으로 나뉨
- 어느 역에서 시작할지 중요함
- 지역 간 이동 방향이 본문 이해에 중요함
- 사용자에게 공간관계 이해가 먼저 필요함

## 역할

지도는 관광지 핀 목록이 아니라:

> **orientation map**

이어야 한다.

보여줄 것:

- 주요 역
- 핵심 하위구역
- 대표 이동 방향
- 인접지역 관계
- 본문 route와 연결되는 최소 정보

피할 것:

- 변동성 큰 개별 매장
- 행사 일정
- 너무 많은 관광지
- 실제 네비게이션처럼 보이는 과도한 정확도 주장

필요 시:

`Editorial map — not to scale`

표시 가능.

## 표시폭

기존 승인 family를 우선 재사용한다.

기본 참고:

- 약 max-width 880px 계열
- 가운데 정렬
- mobile에서 전체 지도 축소
- map crop 금지

---

# 18. H1 Year Automation — 보호 규칙

Travel Guide H1의 현재 연도는 기존 자동화 체계를 재사용한다.

Marker:

```html
<span data-guide-year="current">2026</span>
```

고정 원칙:

- H1 문구 전체 자동치환 금지
- 본문 연도 자동치환 금지
- 행사 연도 자동치환 금지
- title/meta/JSON-LD를 이 marker와 혼동하지 않음
- 기존 `scripts/update-guide-year.py`와 workflow를 임의 수정하지 않음

새 Travel Guide를 H1 자동화 대상에 추가할 경우 별도 승인과 QA를 거친다.

---

# 19. Event / Current Layer — 보호 및 확장 규칙

날짜성 행사는 Travel Guide evergreen 본문과 분리해서 관리한다.

기본 중앙 구조:

- `data/events.json`
- `event-status.js`
- `event-status.css`

상태:

- UPCOMING
- HAPPENING NOW
- ENDED

한국시간 `Asia/Seoul` 기준.

종료된 행사는 삭제하지 않고 자산으로 남긴다.

## 자동화 대상

- 특정 날짜 행사
- 기간형 팝업
- 전시
- 공연
- 축제
- 기간 한정 캠페인

## 자동화 비대상

- H1 year
- Checked 날짜
- 일반 영업시간
- itinerary 시간
- 예약시간
- 역사적 연도

## stale-copy

자동 badge가 ENDED가 되는데 본문이 `currently running`이라고 남으면 실패다.

중립화 문구는 사용자 + ChatGPT가 승인한 exact replacement만 사용한다.

Codex가 자체적으로 새 문구를 작성하지 않는다.

---

# 20. Quality Gate 8 — Codex Implementation Protocol

Codex는 강력한 구현 도구이지만 완성단계에서 주변을 함께 개선하려는 성향이 있을 수 있다.

Korea Inside의 원칙:

> **Codex의 성질을 바꾸려 하지 않는다. 역할과 재량을 조절한다.**

## 초기 큰 뼈대

허용 재량이 비교적 넓다.

예:

- 신규 HTML 기본 골격
- 반복 컴포넌트
- 자동화 script
- 데이터 구조

## 완성단계

재량을 좁힌다.

> **Codex에게 판단권을 주지 않고 실행권만 준다.**

허용:

- exact replacement
- 승인 이미지 배치
- 승인된 구조 재사용
- 파일명 변경
- width/height 갱신
- 승인된 기술 수정

금지:

- PUBLIC COPY 작성
- 자연스럽게 문장 개선
- 문법 개선
- 추천순위 변경
- section 재구성
- 새 디자인 시스템 생성
- 범위 밖 CSS 정리
- 승인된 문구 삭제
- 주변 파일 개선

## Codex 지시문의 두 층

### 1. 실행 명령

- 무엇을 변경할지
- 어느 파일인지
- 어느 위치인지
- 승인 문구 / 이미지가 무엇인지

### 2. 보호 경계

- 무엇을 절대 건드리지 않는지
- 어떤 diff만 허용되는지
- 어떤 경우 중단할지

## 강제 Stop Rule

> **허용된 diff 이외의 변경이 하나라도 발견되면 수정·stage·commit하지 말고 중단 보고한다.**

---

# 21. Git Safety — 절대 규칙

사용자 명시 승인 없이는 금지:

- `git add .`
- `git add -A`
- `git restore`
- `git reset --hard`
- `git clean`

기존 사용자 변경은:

- 수정 금지
- 삭제 금지
- restore 금지
- stage 금지

Stage는 승인 파일을 하나씩 명시한다.

예:

```bash
git add page.html
git add images/area/file.webp
```

---

# 22. Quality Gate 9 — Static QA

Codex 완료보고만 믿지 않는다.

최소 확인:

- `git diff --check`
- 수정 파일 목록
- 범위 외 변경
- H1 개수
- year marker
- PUBLIC COPY 변경 여부
- Affiliate href / rel / tracking
- Event marker
- 이미지 src
- 이미지 dimensions
- alt / credit
- common.js / style.css 변경 여부

작업 종류에 따라 QA 수준을 조절한다.

CSS 한 줄 수정에 30개 항목 QA를 강제하지 않는다.

페이지 전면개편이나 자동화는 상세 QA를 한다.

---

# 23. Quality Gate 10 — Browser QA

Visual 작업은 정적 QA로 완료하지 않는다.

사용자가 실제 화면을 확인한다.

## Desktop

- Hero 높이
- crop
- H1 가독성
- 본문 사진 흐름
- 지도 크기
- 불필요한 공백
- 콘텐츠 폭
- CTA 밀도

## Mobile

- horizontal overflow 0
- H1 overflow 0
- year marker 줄바꿈
- Hero crop
- caption / credit
- 본문 이미지 폭
- 지도 전체 표시
- hamburger 영향 0

## 완료 기준

사용자 화면 승인 전:

- stage 금지
- commit 금지
- push 금지
- Production 금지

단, 사용자가 명시적으로 배포까지 승인한 경우만 진행한다.

---

# 24. Quality Gate 11 — Production Deploy

사용자 승인 후 다음 순서로 진행한다.

1. `git status --short`
2. 실제 diff 확인
3. 승인 파일만 명시적 stage
4. `git diff --cached --check`
5. `git diff --cached --stat`
6. staged 목록 확인
7. commit
8. push `main → origin/main`
9. Vercel Git 자동배포 대기
10. 별도 `vercel --prod` 중복 실행 금지
11. origin/main ahead/behind = 0/0 확인
12. Vercel READY 확인
13. Production SHA = commit SHA 확인
14. 공개 URL HTTP 200
15. 실제 HTML / 이미지 / asset 반영 확인

---

# 25. Production QA

Production에서는 최소 다음을 확인한다.

- 페이지 HTTP 200
- Hero 반영
- H1 1개
- year marker 유지
- 본문 이미지 HTTP 200
- 지도 HTTP 200
- Affiliate CTA 유지
- Event asset 정상
- PUBLIC COPY 승인본 일치
- 보호 파일 변경 없음

브라우저 연결이 불가능하면 Known QA Limitation에 정확히 기록한다.

---

# 26. 완료 상태 판정

## NOT STARTED

아직 Humanization 또는 페이지 제작 전.

## IN PROGRESS

조사 / 원고 / 구현 / QA 중.

## DONE

내용과 구현이 완료됐지만 최종 감사 또는 잠금 전.

## DONE LOCKED

최종 인간화 감사 + 사용자 승인 + Production 정상까지 완료된 보호 상태.

Visual 개선처럼 body PUBLIC COPY와 독립적인 작업은 DONE LOCKED body를 다시 열지 않는다.

---

# 27. 다음 지역으로 넘어가는 조건

현재 지역을 닫지 않은 채 여러 지역을 동시에 대규모로 진행하지 않는다.

기본:

**한 지역 → 한 문제 → QA → 승인 → Production → 다음 지역**

예외:

같은 배포 묶음으로 처리해도 위험이 낮고 사용자가 명시 승인한 경우만 함께 배포한다.

---

# 28. 경쟁사 활용 규칙

경쟁사 분석은 모방이 목적이 아니다.

확인:

- 어떤 검색의도를 잡는지
- 무엇을 빠르게 전달하는지
- 어떤 정보가 얕은지
- 실제 이동 마찰을 다루는지
- 최신성 검증을 어떻게 하는지
- 수익화를 어디에 연결하는지

페이지 수 자체는 품질 지표가 아니다.

Korea Inside는:

> **깊이 → 정확성 → 전달력 → 실행 → 확장**

순서를 우선한다.

---

# 29. Detail 확장 전략

Travel Guide가 깊다고 해서 한 URL로 끝낼 필요는 없다.

대표 Hub를 만든 후 독립 수요가 있는 부분만 Detail로 확장한다.

## Hub에 남길 것

- 왜 갈지
- 누구에게 맞는지
- 어떻게 일정에 넣을지
- Detail로 갈지 판단할 정도의 요약

## Detail에서 깊게 다룰 것

- 실제 위치
- 이동
- 시간
- 예약
- 비용
- 체험
- 실패 가능성
- 다른 일정과 결합 방법

Hub 내용을 복사해 Detail을 만들지 않는다.

---

# 30. Travel Guide 제작 완료 체크리스트

## Search / Role

- 대표 검색의도 1개인가
- 페이지 역할이 분명한가
- 다른 URL과 중복되지 않는가

## Facts

- 핵심 사실이 공식출처로 확인됐는가
- 날짜 / 비용 / 운영조건이 최신인가
- 추측을 사실처럼 쓰지 않았는가

## Humanization

- 누구에게 맞는가
- 누구에게 안 맞는가
- 실제 마찰이 보이는가
- 선택 이유가 서로 다른가
- AI 공식이 반복되지 않는가

## Schedule

- 실제 하루가 보이는가
- 시작 위치와 이동순서가 있는가
- 체험을 넣고 뺄 수 있는가
- 일정이 과도하지 않은가

## Affiliate

- 본문과 직접 맞는가
- 필요한 CTA를 광고처럼 보일까 봐 억지로 뺀 것은 아닌가
- 무관한 CTA가 섞이지 않았는가
- 실제 예약조건을 확인했는가

## Visual

- Hero가 지역의 얼굴인가
- 사진이 각자 역할이 있는가
- 중복 장면이 과도하지 않은가
- 권리가 명확한가
- 웹용으로 최적화됐는가
- map이 필요할 때만 있는가

## Technical

- H1 = 1
- year marker 정상
- mobile overflow 0
- images 200
- common files 보호
- git diff check PASS

## Production

- 승인 파일만 stage
- Vercel READY
- SHA 일치
- 공개 URL 200
- 승인본 일치

---

# 31. Codex 지시문 기본 골격

각 Codex 작업은 다음 틀을 사용한다.

```text
[페이지 / 작업명]

0. Master Standard 확인
1. 작업 목적
2. 작업 대상
3. 현재 승인 상태
4. 정확한 실행 명령
5. 보호 대상
6. PUBLIC COPY 보호
7. Affiliate / Stay / Event 보호
8. 공통 파일 수정 금지
9. Git 보호
10. 이번 단계 범위
11. QA
12. 완료보고
13. Stop Rule
```

모든 작업에 동일한 장문의 항목을 기계적으로 붙이지 않는다.
위험도에 따라 필요한 보호항목을 선택한다.

하지만 다음은 빠뜨리지 않는다.

- 수정 파일
- 허용 diff
- 금지 diff
- stage/commit/push 허용 여부
- 범위 밖 변경 시 중단

---

# 32. Codex 완료보고 기본 형식

작업 종류에 따라 필요한 항목만 사용한다.

```text
1. Master Standard 경로
2. 작업 대상 파일
3. 실제 변경 파일
4. 허용 diff 외 변경 여부
5. PUBLIC COPY 변경 여부
6. H1 / year marker 상태
7. Affiliate 변경 여부
8. Event 변경 여부
9. style.css/common.js 변경 여부
10. 이미지 dimensions / file size
11. Desktop QA
12. Mobile QA
13. git diff --check
14. git status --short
15. stage/commit/push/Production 여부
16. Known QA Limitation
```

배포 단계에는 추가:

- staged files
- commit SHA
- push 결과
- origin/main ahead/behind
- Vercel READY
- Production SHA
- 공개 URL HTTP

---

# 33. 방 이동 시 품질 유지 프로토콜

새 방에서 첫 작업은 제작이 아니라 **상태 복원**이다.

반드시:

1. Master Standard 전체 읽기
2. Production Playbook 전체 읽기
3. 최신 Handover 전체 읽기
4. 사용자 최신 완료보고 반영
5. DONE LOCKED 승계
6. 미배포 diff 확인
7. 보호 파일 확인

새 방에서 다음을 하지 않는다.

- 기존 승인 방향 재논의
- DONE LOCKED를 이유 없이 재감사
- Hero / 사진 family를 새로 디자인
- 제휴 개수 제한을 새로 설정
- Codex에게 문구 작성권 부여

---

# 34. Playbook 변경 조건

이 문서는 실제 제작공정이 개선됐을 때만 수정한다.

수정 조건:

1. 반복되는 실패가 발견됨
2. 더 안전한 구현 방식이 검증됨
3. 사용자가 제작 원칙을 변경함
4. 새로운 자동화가 Production에서 안정적으로 검증됨

단순히 한 페이지에서만 발생한 예외는 이 문서에 즉시 일반화하지 않는다.

변경은 사용자 명시 승인 후 Version을 올린다.

---

# 35. 이 Playbook의 핵심 문장

> **같은 목차를 복제하지 않는다. 같은 품질 공정을 복제한다.**

> **AI의 생산력을 쓰되, 인간의 판단 기준으로 결과물을 통제한다.**

> **Codex는 강력한 구현 엔진이다. 큰 뼈대에서는 넓게 쓰고, 완성단계에서는 판단권을 주지 않고 실행권만 준다.**

> **본문이 여행자의 판단을 만들고, 체험·숙소·교통 제휴는 그 판단을 실제 일정과 예약으로 연결한다.**

> **페이지 수보다 한 페이지의 정확성·정보력·전달력·실행력을 우선한다.**

> **Hub는 판단을 만들고, 독립 수요가 생기면 Detail이 더 깊게 해결한다.**

---

# 36. 최종 품질 질문

페이지를 닫기 전 반드시 묻는다.

> **1. 이 페이지를 읽은 여행자가 실제 다음 행동을 더 잘 선택할 수 있는가?**

> **2. 사실과 편집 판단이 구분되어 있고, 틀린 확신을 주지 않는가?**

> **3. 사진과 지도는 장식이 아니라 여행 이해를 돕는가?**

> **4. 제휴 CTA는 방금 설명한 여행 행동과 직접 연결되는가?**

> **5. 예약 버튼을 모두 가려도 Korea Inside만의 판단 가치가 남는가?**

> **6. 버튼을 다시 보였을 때 그 판단이 실제 여행 실행으로 자연스럽게 이어지는가?**

> **7. Codex가 승인 범위 밖을 건드리지 않았는가?**

> **8. 새 방으로 이동해도 Master + Playbook + Handover만 읽으면 같은 품질로 이어갈 수 있는가?**

이 8개가 모두 YES일 때 다음 지역으로 넘어간다.
