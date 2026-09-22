# Korea Inside 작업 인계 — 2026-09-22

**Status:** ACTIVE HANDOVER  
**Project:** Korea Inside  
**Repository:** `C:\Projects\Koreainside`  
**Production:** `https://www.getkoreainside.com/`  
**Primary branch:** `main`

---

# 1. 새 방 시작 시 가장 먼저 읽을 파일

저장소 루트에서 아래 순서대로 전체 읽는다.

1. `Korea_Inside_Public_Content_Master_Standard.md`
2. `Korea_Inside_Navigation_Hub_Architecture_Standard.md`
3. 이 파일: `Korea_Inside_Room_Handover_2026-09-22.md`

공개 페이지 작업이면 이후 해당 지역 Research Master / Approved Public Copy를 읽는다.

새 방에서 과거 대화 내용을 추측하거나 다시 설계하지 않는다.
이 인계문에 적힌 확정 사항은 그대로 승계한다.

---

# 2. Master 파일 위치 — 고정

아래 2개는 저장소 루트에 그대로 둔다.

- `Korea_Inside_Public_Content_Master_Standard.md`
- `Korea_Inside_Navigation_Hub_Architecture_Standard.md`

Master 2개를 `md/` 아래로 이동하지 않는다.

관리용 MD만 `md/` 아래에서 관리한다.

권장 구조:

```text
C:\Projects\Koreainside\
├─ Korea_Inside_Public_Content_Master_Standard.md
├─ Korea_Inside_Navigation_Hub_Architecture_Standard.md
├─ md\
│  ├─ 승인본\
│  │  ├─ 영어\
│  │  └─ 스페인어\
│  ├─ 검토중\
│  ├─ 조사자료\
│  ├─ 작업자료\
│  └─ 인계문\
├─ es\
├─ images\
├─ *.html
├─ style.css
└─ common.js
```

사용자는 MD 이동/분류를 직접 관리할 필요가 없다.
ChatGPT가 저장 위치를 정하고 Codex가 파일 작업을 처리한다.

---

# 3. 절대 고정 운영 원칙

## 콘텐츠 권한

공개 문구의 번역/현지화/편집 판단:
- ChatGPT + 사용자

최종 승인:
- 사용자

Codex:
- 승인 문구 exact implementation
- HTML/CSS 구조 적용
- hreflang / sitemap / link / QA / Git 작업

Codex 금지:
- 자체 번역
- 문구 개선
- humanization
- 추천 판단
- 승인 문구 추가/삭제/요약
- 호텔/지역 순위 변경

## Git 보호

금지:
- `git add .`
- `git add -A`
- 범위 외 restore/reset/delete/clean

기존 사용자 변경을 보호한다.

## 공통 파일

사용자 명시 승인 없이 수정 금지:
- common header
- navigation
- footer
- `common.js`
- 모바일 hamburger
- `style.css` 공통 시스템

Spanish 페이지에서는 이미 승인된 Spanish Golden Sample UI를 페이지 안에서 재사용할 수 있다.

---

# 4. Spanish 다국어 구조 — 확정

단일 도메인:
`getkoreainside.com`

영어:
- root 유지
- `/en/` 없음

Spanish:
- `/es/`

추후:
- `/fr/`
- `/de/`
- `/it/`
- `/ja/`
- `/zh-tw/`

공통 원칙:
- self canonical
- reciprocal hreflang
- `en`
- `es`
- `x-default`
- sitemap 등록
- 존재하는 Spanish sibling만 Spanish 내부링크
- 아직 없는 Spanish URL은 English fallback
- 미래 404 Spanish URL 생성 금지
- IP/browser-language 자동 redirect 금지

Spanish Golden Sample:
- `es/dongdaemun-travel-guide.html`

Spanish Stay Golden Sample:
- `es/where-to-stay-in-dongdaemun.html`

---

# 5. Spanish Pilot 완료

## Dongdaemun Travel

Production:
`https://www.getkoreainside.com/es/dongdaemun-travel-guide.html`

Status:
- Production
- Humanization DONE LOCKED
- Approved Public Copy locked

## Dongdaemun Stay

Production:
`https://www.getkoreainside.com/es/where-to-stay-in-dongdaemun.html`

Status:
- Production
- Humanization DONE LOCKED
- Approved Public Copy locked

이 2개가 Spanish 공통 UI/Navigation/Footer/ARIA의 Golden Sample이다.

---

# 6. Spanish Travel Batch 1 — 현재 구현된 4개

대상:

1. `es/insadong-travel-guide.html`
2. `es/myeongdong-travel-guide.html`
3. `es/seongsu-travel-guide.html`
4. `es/gangnam-travel-guide.html`

현재 상태:
- Spanish HTML 4개 구현 완료 상태
- 영어 원본 hreflang 반영 상태
- sitemap 변경 포함
- 기존 staged 13개가 이 Travel Batch 1 관련 파일일 가능성이 매우 높음
- 정확한 staged 파일 목록을 Codex가 다시 확인해야 함
- 아직 commit/push/deploy 하지 않은 상태로 유지 중

Travel Batch 1 Approved MD는 스페인어 승인본 폴더에서 관리한다.

---

# 7. Spanish Stay Batch 1 — 방금 구현 완료 / QA PASS

대상 English:

1. `where-to-stay-in-myeongdong.html`
2. `where-to-stay-in-seongsu.html`
3. `where-to-stay-in-insadong.html`
4. `where-to-stay-in-gangnam.html`

대상 Spanish:

1. `es/where-to-stay-in-myeongdong.html`
2. `es/where-to-stay-in-seongsu.html`
3. `es/where-to-stay-in-insadong.html`
4. `es/where-to-stay-in-gangnam.html`

Codex 최신 보고:

### 구현 결과
- Spanish HTML 4개 생성 완료
- title/meta/H1 4페이지 승인 MD와 일치
- H1 각 1개
- 공개 문구 mismatch 0

### 호텔/숙소 수
- Myeongdong 11
- Seongsu 4
- Insadong 7
- Gangnam 10
- 순서 mismatch 0

### OTA
- Myeongdong 33
- Seongsu 11
- Insadong 21
- Gangnam 30
- 총 95
- URL mismatch 0
- tracking attribute mismatch 0
- Stay BUT Trip.com 신규 생성 0

### 사실/수치
- 객실
- 침대
- 인원
- 출구
- 공항버스
- 수치
mismatch 0

### FAQ
- Myeongdong 5
- Seongsu 5
- Insadong 4
- Gangnam 8
- Myeongdong FAQ JSON-LD 5와 visible 일치
- 나머지는 영어 원본대로 신규 schema 추가하지 않음

### SEO
- Spanish self canonical 적용
- reciprocal `en / es / x-default` 적용
- 영어 원본 body 변경 0
- 영어 원본은 head hreflang만 추가

### Sitemap
4개 신규 Stay URL 각각 1회
`lastmod 2026-09-22`
중복 0

### Internal link
- 존재하는 Spanish Travel Guide 5개는 Spanish 경로 사용
- 존재하지 않는 Spanish URL 0
- 누락 로컬 링크 0

### 공통 자산
변경 0:
- common.js
- style.css
- guide-year.js
- affiliate-tracking.js

### QA
- `git diff --check`: PASS
- trailing whitespace 0
- conflict marker 0
- Desktop/Tablet/Mobile 구조 보존 PASS
- ARIA 보존 PASS

### Git 상태
- 이번 Stay 작업 신규 stage 0
- 기존 staged 13개 유지
- `sitemap.xml`은 기존 staged + 이번 unstaged가 함께 있어 `MM`
- commit 0
- push 0
- deploy 0

---

# 8. Stay MD 관리상 현재 정리 필요 사항

Codex 보고상 현재 경로:

- Myeongdong:
  `md/승인본/스페인어/Korea_Inside_Myeongdong_Stay_ES_Review_Copy_2026-09-22.md`

- Seongsu:
  `md/승인본/영어/Korea_Inside_Seongsu_Stay_ES_Review_Copy_2026-09-22.md`

- Insadong:
  `md/승인본/스페인어/Korea_Inside_Insadong_Stay_ES_Review_Copy_2026-09-22.md`

- Gangnam:
  `md/승인본/스페인어/Korea_Inside_Gangnam_Stay_ES_Review_Copy_2026-09-22.md`

문제:
1. Seongsu가 잘못 `승인본/영어/`에 들어가 있음
2. 4개 모두 사용자 승인 상태인데 파일명이 아직 `_Review_Copy_`

다음 Codex 작업에서 PUBLIC COPY 본문 변경 없이 정리:

최종 경로:
`md/승인본/스페인어/`

최종 파일명:

- `Korea_Inside_Myeongdong_Stay_ES_Approved_Public_Copy_2026-09-22.md`
- `Korea_Inside_Seongsu_Stay_ES_Approved_Public_Copy_2026-09-22.md`
- `Korea_Inside_Insadong_Stay_ES_Approved_Public_Copy_2026-09-22.md`
- `Korea_Inside_Gangnam_Stay_ES_Approved_Public_Copy_2026-09-22.md`

문서 상태:
- `APPROVED PUBLIC COPY — CONTENT LOCKED`
- `Humanization status: DONE LOCKED`

PUBLIC COPY 내용 변경 0.

---

# 9. 현재 바로 다음 작업 — 가장 중요

Travel Batch 1 4개 + Stay Batch 1 4개를 합쳐
**Spanish 8페이지 통합 QA → commit 대상 확인** 단계다.

대상 8개:

Travel:
- `es/insadong-travel-guide.html`
- `es/myeongdong-travel-guide.html`
- `es/seongsu-travel-guide.html`
- `es/gangnam-travel-guide.html`

Stay:
- `es/where-to-stay-in-insadong.html`
- `es/where-to-stay-in-myeongdong.html`
- `es/where-to-stay-in-seongsu.html`
- `es/where-to-stay-in-gangnam.html`

다음 Codex에서 반드시:
1. Stay MD 4개 경로/이름 정리
2. 기존 staged 13개 실제 파일명 출력
3. staged 13개가 이전 Travel Batch 1 정확한 범위인지 검증
4. Travel 4 + Stay 4 통합 QA
5. commit 대상 파일 전체 목록만 보고
6. 아직 commit/push/deploy 하지 않음

사용자 최종 확인 후:
**8페이지를 한 Batch로 commit → push → Vercel Production → 공개 URL QA**

---

# 10. Spanish Travel Batch 2 — 미리 현지화 완료

다음 5개는 ChatGPT가 이미 Spanish Review Copy를 만들어 둠.

1. Jamsil
2. Gongdeok & Mapo
3. Itaewon
4. Lotte World
5. Seoul Sky

생성된 Review MD 파일명:

- `Korea_Inside_Jamsil_ES_Review_Copy_2026-09-22.md`
- `Korea_Inside_Gongdeok_Mapo_ES_Review_Copy_2026-09-22.md`
- `Korea_Inside_Itaewon_ES_Review_Copy_2026-09-22.md`
- `Korea_Inside_Lotte_World_ES_Review_Copy_2026-09-22.md`
- `Korea_Inside_Seoul_Sky_ES_Review_Copy_2026-09-22.md`

중요:
이 5개는 ChatGPT 아티팩트로 생성됨.
**저장소 안에 실제 저장되었는지는 아직 사용자 확인 없음.**
새 방에서 저장소 검색 후 존재 여부부터 확인해야 함.

이 5개는 아직:
- 사용자 최종 승인 전
- CONTENT LOCKED 전
- Codex 구현 전

현재 상태:
`LOCALIZATION REVIEW — NOT YET CONTENT LOCKED`

---

# 11. Batch 2 핵심 보호사항

## Jamsil
보존:
- 2.5 km lake loop
- Tower side Exit 2
- Adventure side Exit 4
- KSPO Dome: Olympic Park Station Lines 5/9 Exit 3
- Sports Complex Lines 2/9
- Lotte / Tower / Aquarium / Sports / Olympic Park 분리
- FAQ 8

현재 Production에는 Trip.com affiliate CTA들이 있음.
영어 production URL/CTA/tracking exact 보존.

## Gongdeok & Mapo
핵심 역할:
- Local food
- one worthwhile experience
- relaxed local evening
- transport base

Affiliate 5:
1. Creatrip Wolhwa Sikdang
2. Creatrip Chowol
3. Creatrip JUNO Mapo
4. Klook SPA THE ZEN
5. Klook HANOI Gongdeok

Gongdeok Station ≠ Mapo Station.
AREX / Lines 5,6 / Gyeongui-Jungang 역할 보존.

## Itaewon
Zone roles:
- Hannam = art / design / shopping / beauty
- Central Itaewon = international / halal food + main nightlife
- Gyeongnidan = calmer bars / cafés
- Haebangchon = hillside neighborhood evening

보존:
- Gyeongnidan approx 788 m from Noksapyeong Exit 2
- Haebangchon approx 1.2 km
- Leeum Monday closed
- Leeum current operating facts
- hills / walking friction
- halal verification nuance

Affiliate:
- Creatrip Commenanabien Hannam
- Creatrip Kyochon Pilbang
- Klook Itaewon Pub Crawl

## Lotte World
보존:
Ticket prices checked 2026-09-15:
- 1Day Adult ₩67,000 / Youth ₩58,000 / Child ₩50,000
- After4 Adult ₩55,000 / Youth ₩47,000 / Child ₩39,000

Magic Pass:
- Light ₩54,000
- Standard A ₩65,000
- Standard B ₩65,000
- Premium ₩80,000
- 각 5 uses

Purchase rules:
- Premium: midnight 2 days before → 11:59 p.m. day before
- same-day: from 8:30 a.m.

Height examples:
- Comet Express 120 cm+
- Gyro Swing 130–190 cm
- Flume Ride 110 cm+
- Kids Bumper Car 110–125 cm

주소:
- Lotte World Adventure 240 Olympic-ro
- Tower/Mall 300 Olympic-ro

Trip.com affiliate:
`https://www.trip.com/t/A0oe7sS8MW2`

FAQ 8.

## Seoul Sky
보존:
- suggested 90–120 minutes
- entrance B1
- 300 Olympic-ro
- Korean name 서울스카이
- Sky Bridge 541 m
- carrier rental ₩3,000
- under 36 months
- Infant Lounge B1 in front of VIP room
- QR ≠ Fast Pass
- direct entry ≠ priority
- image slots 5
- attribution/license unchanged

Trip.com affiliate:
`https://www.trip.com/t/Am4PeLVvNW2`

FAQ 8.

---

# 12. Hongdae — Spanish에서 별도 처리

Hongdae Travel Guide는 Batch 1에서 제외함.

이유:
- event/status complexity
- external ad loader
- 약 12 event IDs
- body data-section inconsistency
- 기술 요소가 많음

따라서 Hongdae Spanish는 별도 Batch로 처리.
다른 페이지 번역과 섞지 않는다.

---

# 13. 번역/현지화 생산 방식 — 새 방에서도 절대 되돌리지 말 것

사용자가 가장 중요하게 요청한 운영 변경.

잘못된 방식:
- 번역 전체를 채팅창에 수만 자 출력
- 페이지 1개씩 승인 반복
- 승인 후 다시 번역
- Review/Draft를 여러 번 재생성
- 방 용량 소모

현재 확정 방식:

1. 여러 페이지 English source 읽기
2. ChatGPT가 Batch로 Spanish 현지화
3. 대화에는 전체 본문 출력하지 않음
4. MD 파일로 직접 생성
5. 채팅에는 페이지별 상태/특이사항만 짧게 보고
6. 사용자 Batch 승인 1회
7. 승인 후 재번역 0
8. Approved MD 확정
9. Codex exact implementation
10. 통합 QA
11. 한 번에 Production

사용자가 `승인`이라고 하면:
- 현재 Batch 전체 승인으로 처리
- 다시 번역하지 않음
- 바로 Approved 상태 확정 + 다음 Codex 지시문 생성

---

# 14. 사용자 선호 / 작업 방식

- 사용자 본업은 토목/건설, Korea Inside는 병행 작업
- 반복 판단과 수동 파일 관리를 줄이고 싶어 함
- 사용자는 최종 판단/승인에 집중
- ChatGPT가 미리 조사/번역/MD/지시문 준비
- Codex가 구현/QA/Git
- Codex가 20–30분 작업하는 동안 ChatGPT는 다음 Batch 미리 준비
- 답변은 짧고 실행 중심
- 사용자가 이미 확정한 내용을 반복 설명하지 않음
- 방 용량을 낭비하는 대량 본문 출력 금지
- 대화방 용량이 임박하면 늦기 전에 인계문 생성

---

# 15. 보호해야 할 사용자 파일/상태

기존 사용자 변경:
- Accommodation 관련 삭제/변경
- 신규 파일
- `_CleanTemp/`
등이 존재함.

절대:
- restore
- reset
- delete
- clean
하지 않는다.

Codex 보고상 이 상태는 현재까지 보호됨.

---

# 16. 현재 Git 핵심

Codex 최신 보고:

- 기존 staged 13개 존재
- Stay Batch 신규 stage 0
- `sitemap.xml` = `MM`
- `git diff --check` PASS
- commit 0
- push 0
- deploy 0

다음 방에서 Git 상태를 추측하지 않는다.
Codex에게 실제 `git status` 및 staged 파일 목록을 다시 확인시킨다.

---

# 17. 바로 사용할 새 방 첫 지시문

새 방 첫 메시지로 아래를 사용:

```text
작업 시작 전 저장소에서 아래 파일을 순서대로 전체 읽어.

1. Korea_Inside_Public_Content_Master_Standard.md
2. Korea_Inside_Navigation_Hub_Architecture_Standard.md
3. Korea_Inside_Room_Handover_2026-09-22.md

먼저 아래만 보고해.
1. 세 파일 전체 경로
2. 전체 내용 확인 여부
3. 현재 Spanish 작업 상태
4. 지금 바로 해야 할 다음 작업
5. 보호해야 할 기존 사용자 변경

과거 내용을 다시 추측하거나 재설계하지 말고
Room Handover의 확정 상태를 그대로 승계해.

현재 최우선은:
Spanish Travel Batch 1 + Stay Batch 1 총 8페이지 통합 QA 및 commit 대상 검증이다.

아직 commit/push/deploy 하지 마.
```

---

# 18. 현재 최우선 다음 행동

1. 이 인계문을 저장소에 보관
2. 새 방으로 이동
3. 새 방에서 Master 2개 + 이 인계문 전체 확인
4. Spanish 8페이지 통합 QA / staged 13개 검증부터 이어감
5. 사용자 승인 후 8페이지 Production
6. 그 동안 준비된 Spanish Travel Batch 2 5개 검토/승인으로 진행

---

# 19. 다음 방에서 다시 논의하지 말아야 할 확정 사항

- Master 2개는 저장소 루트 고정
- MD 관리 폴더는 `md/` 아래
- Codex가 번역하지 않음
- ChatGPT가 번역/현지화
- 사용자 승인 후 재번역하지 않음
- 대량 번역문을 채팅창에 다시 출력하지 않음
- Batch 방식으로 진행
- Spanish Golden Sample은 Dongdaemun
- 현재 Travel 4 + Stay 4 구현 상태를 다시 처음부터 만들지 않음
- Hongdae는 별도 처리
- Git 기존 사용자 변경 보호
- common.js/style.css/global common UI 임의 수정 금지

---

# 20. 현재 상태 한 줄 요약

**Spanish Pilot 2개 Production 완료 → Travel Batch 1 4개 구현 상태 → Stay Batch 1 4개 구현/QA PASS → 현재 8페이지 통합 QA 및 commit 직전 정리 단계 → 다음 Travel Batch 2 (Jamsil/Gongdeok/Mapo/Itaewon/Lotte World/Seoul Sky) Spanish Review Copy 준비 완료.**
