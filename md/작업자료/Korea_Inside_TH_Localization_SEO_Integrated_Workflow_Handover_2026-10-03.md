# Korea Inside — Thai Localization + SEO Integrated Workflow Handover

**Date:** 2026-10-03  
**Status:** ACTIVE WORKFLOW HANDOVER — FOR THAI LOCALIZATION ROOM  
**Purpose:** 현재 태국어 현지화 작업방에서 번역/현지화를 계속할 때, 2026-10-03 태국 SEO 58-page 구조감사 결과를 중복 작업 없이 함께 반영하기 위한 실행 순서  
**Scope:** 기존 English 58-page target set → `/th/` Thai localization  
**Important:** 이 문서는 운영 순서/Handover다. Public Content Master, Navigation Standard, Localization Standard, Thai Localization Standard를 대체하지 않는다.

---

# 1. 작업 전 기준 문서

실제 Thai 페이지 작업을 시작할 때 아래 기준을 사용한다.

1. `Korea_Inside_Public_Content_Master_Standard.md`
2. `Korea_Inside_Navigation_Hub_Architecture_Standard.md`
3. `Korea_Inside_Language_Localization_Standard.md`
4. `Korea_Inside_Thai_Localization_Standard.md` / 현재 ACTIVE v1.1
5. 최신 `Korea_Inside_Room_Handover_*.md`
6. 해당 페이지의 English Production HTML
7. 해당 페이지의 기존 Thai Approved/Review Copy가 있으면 그것
8. `Korea_Inside_TH_58_Page_SEO_Structure_Audit_2026-10-03.xlsx`

SEO 감사에서 우선 보는 시트:

- `17_TH_58_Page_Audit`
- `18_TH_Cluster_Compare`
- `19_TH_Executive_Plan`

유료 Semrush/API 추가조사는 이 workflow의 필수조건이 아니다.  
현재 저장된 SEO 감사 결과를 재사용한다.

---

# 2. 58-page SEO 감사에서 확정된 핵심

전체 조사:

- 대상: **58 / 58**
- P0: **14**
- P1: **42**
- P2: **2**
- Contextual internal links 0: **7 pages**
- Contextual internal links <=1: **10 pages**

구조 결론:

> **Korea Inside는 콘텐츠 깊이보다 Hub ↔ Detail / Area ↔ Stay / Utility ↔ Travel 연결이 상대적으로 약하다.**

Thai 적용 핵심:

1. 새 URL을 만드는 것이 우선이 아니다.
2. 기존 58개 URL의 역할을 보존한다.
3. Thai Title / Meta / H1 / Lead / Quick Answer는 실제 Thai 검색·판단 언어로 쓴다.
4. CONTENT LOCKED 본문을 SEO 이유만으로 대량 재작성하지 않는다.
5. 내부링크는 숫자를 늘리기 위해 넣지 않는다.
6. 현재 문단에서 생기는 실제 다음 질문에만 연결한다.
7. 존재하지 않는 미래 `/th/` sibling을 미리 링크하지 않는다.
8. Thai sibling이 실제 생성된 뒤 reciprocal link를 닫는다.
9. 공통 navigation / `common.js` / `style.css`는 Final Integration 전 보호한다.
10. 10월 구조조정은 **기존 URL만 사용**한다.

---

# 3. 현재 Thai 현지화 완료 Working Copy — 13 pages

아래 13개는 이미 Thai 현지화 working copy가 존재한다.

> **중요: 다시 처음부터 번역하지 않는다.**

1. `accommodation.html`
2. `best-area-for-budget-travelers-seoul.html`
3. `best-area-for-couples-seoul.html`
4. `best-area-for-families-seoul.html`
5. `best-area-for-first-time-visitors-seoul.html`
6. `best-area-for-shopping-seoul.html`
7. `best-area-for-solo-travelers-seoul.html`
8. `hongdae-vs-myeongdong.html`
9. `where-to-stay-in-gangnam.html`
10. `where-to-stay-in-hongdae.html`
11. `where-to-stay-in-insadong.html`
12. `where-to-stay-in-itaewon.html`
13. `where-to-stay-in-myeongdong.html`

현재 상태 해석:

- Thai localization working copy: 존재
- SEO 58-page 감사: 이후 완료
- 따라서 필요한 것은 **재번역이 아니라 SEO Delta 반영**
- Git / Production COMPLETE 여부는 별도 확인 전 추정하지 않는다.

### 현재 13개 감사 판정

| File | SEO audit | 적용 강도 |
|---|---|---|
| `accommodation.html` | P0 / KEEP-STRENGTHEN | 구조 유지, search-facing 점검 |
| `best-area-for-budget-travelers-seoul.html` | P1 | Title/Meta/H1/Lead/Quick Answer 점검 |
| `best-area-for-couples-seoul.html` | P1 | 동일 |
| `best-area-for-families-seoul.html` | P1 | 동일 |
| `best-area-for-first-time-visitors-seoul.html` | P1 | 동일 |
| `best-area-for-shopping-seoul.html` | P1 | 동일 |
| `best-area-for-solo-travelers-seoul.html` | P1 | 동일 |
| `hongdae-vs-myeongdong.html` | P1 | Short Answer + reciprocal-link 점검 |
| `where-to-stay-in-gangnam.html` | P0 / INTERNAL LINK FIX | 고립 해소 |
| `where-to-stay-in-hongdae.html` | P1 / INTERNAL LINK TUNE | 연결 보강 |
| `where-to-stay-in-insadong.html` | P0 / INTERNAL LINK FIX | 고립 해소 |
| `where-to-stay-in-itaewon.html` | P0 / INTERNAL LINK FIX | 고립 해소 |
| `where-to-stay-in-myeongdong.html` | P1 / INTERNAL LINK TUNE | 연결 보강 |

### 현재 13개 처리 원칙

현재 즉시 전체를 다시 뜯지 않는다.

먼저:

- Thai wording/facts/recommendation은 보존
- Title / Meta / H1 / Lead / Quick Answer만 SEO Delta 후보로 표시
- 현재 실제 존재하는 Thai sibling에만 link 가능
- 아직 생성되지 않은 `gangnam-travel-guide.html`, `insadong-travel-guide.html` 등의 `/th/` 링크는 선링크 금지

**가장 효율적인 방법은 아래 Batch 1~3에서 Area sibling을 만든 뒤, 한 번에 Stay/Area SEO Closure를 수행하는 것이다.**

---

# 4. 최적 현지화 순서 — 전체 58페이지 연결형 Workflow

현재 13개는 재번역하지 않는다.

남은 **45개 = 9 Batch × 5 pages** 순서로 진행한다.

이 순서는 단순 파일번호 순서가 아니라:

> **Stay 결정 → Area/Stay reciprocal → Airport/Transport → Payment/Service → Apps/eSIM → Topic Hub → Home Hub**

순으로 설계한다.

목적:

- 미래 Thai URL 선링크 최소화
- 같은 페이지를 여러 번 다시 여는 작업 최소화
- Hub가 생길 때 이미 연결할 Detail이 존재하도록 구성
- 최종 `/th/` Home Hub를 가장 늦게 만들어 실제 Thai child page로 연결

---

# 5. BATCH 1 — 남은 Stay Decision + 현재 Stay Detail과 연결할 Area Guide

1. `best-area-for-airport-access-seoul.html`
2. `best-area-for-luxury-hotels-seoul.html`
3. `best-area-for-nightlife-seoul.html`
4. `gangnam-travel-guide.html`
5. `hongdae-travel-guide.html`

### 목적

- Stay Decision Family 9개를 완성
- 이미 존재하는:
  - `where-to-stay-in-gangnam.html`
  - `where-to-stay-in-hongdae.html`

  와 Area Guide reciprocal 구조를 만들 준비

### SEO 적용

Stay Decision:

- Thai Title / Meta / H1 / Lead / Quick Answer
- English recommendation strength 보존
- `best area` 직역 반복 금지
- 자연스러운 Thai decision query 사용

Area Guide:

- body depth 유지
- Thai area entity/orientation wording
- Stay Detail / Accommodation 연결

---

# 6. BATCH 2 — 현재 Stay Detail의 Area Guide 완성 + Dongdaemun Pair

1. `insadong-travel-guide.html`
2. `itaewon-travel-guide.html`
3. `myeongdong-travel-guide.html`
4. `dongdaemun-travel-guide.html`
5. `where-to-stay-in-dongdaemun.html`

### 목적

이미 존재하는:

- `where-to-stay-in-insadong.html`
- `where-to-stay-in-itaewon.html`
- `where-to-stay-in-myeongdong.html`

과 Area Guide를 연결한다.

Dongdaemun은 Area + Stay Detail을 같은 Batch에서 함께 만든다.

### SEO 적용

- Insadong / Itaewon: P0 reciprocal-link fix
- Myeongdong: P1 tune
- Dongdaemun: Area ↔ Stay를 같은 Batch 안에서 닫는다.

---

# 7. BATCH 3 — Jamsil / Seongsu / Gongdeok-MaPo Cluster

1. `jamsil-travel-guide.html`
2. `where-to-stay-in-jamsil.html`
3. `seongsu-travel-guide.html`
4. `where-to-stay-in-seongsu.html`
5. `gongdeok-mapo-seoul-guide.html`

### 목적

Jamsil과 Seongsu는 Area + Stay를 같은 Batch에서 생성한다.

Gongdeok/Mapo는 이후:

- `hotels-near-gongdeok-station.html`
- Airport/AREX

와 연결할 기반을 만든다.

---

# 8. STAY / AREA SEO CLOSURE — Batch 1~3 직후 1회

이 단계는 **재번역 단계가 아니다.**

현재 13개 + Batch 1~3의 새 Thai 페이지를 함께 보고 한 번만 reciprocal closure를 수행한다.

### 해야 할 것

#### A. Current 13 SEO Delta

- `accommodation.html`
  - `ที่พักโซล`
  - `พักโซลย่านไหนดี`
  - Title / Meta / H1 / Quick Answer 자연스러움 확인
  - 기존 dense link architecture 보존

- 9 Stay Decision pages
  - Thai question-style search wording
  - Quick Answer가 첫 판단을 빠르게 제공
  - 추천 순서/조건 변경 금지

- `hongdae-vs-myeongdong.html`
  - Short Answer 점검
  - Hongdae / Myeongdong Stay + Area Guide 양방향 연결

#### B. Area ↔ Stay reciprocal closure

가능하면:

- Gangnam Guide ↔ Where to Stay in Gangnam
- Hongdae Guide ↔ Where to Stay in Hongdae
- Insadong Guide ↔ Where to Stay in Insadong
- Itaewon Guide ↔ Where to Stay in Itaewon
- Myeongdong Guide ↔ Where to Stay in Myeongdong
- Dongdaemun Guide ↔ Where to Stay in Dongdaemun
- Jamsil Guide ↔ Where to Stay in Jamsil
- Seongsu Guide ↔ Where to Stay in Seongsu

그리고 자연스러운 위치에서:

- → `accommodation.html`

로 돌아갈 수 있게 한다.

### 금지

- 내부링크 개수를 맞추기 위한 억지 링크
- 아직 없는 future `/th/` 링크
- 공통 navigation 수정
- locked body 전체 재작성

---

# 9. BATCH 4 — Core Airport / Transport

1. `airport.html`
2. `arrival.html`
3. `arex.html`
4. `airport-bus.html`
5. `airport-transfer.html`

### 목적

Korea Inside의 강한 Airport cluster를 Thai에서도 먼저 완성한다.

### SEO 적용

경쟁사 대비 부족한 것은 본문 깊이가 아니다.

따라서:

- Thai Quick Answer / choice를 위쪽에 명확히
- luggage / transfer / last-walk friction 보존
- action/booking은 판단 뒤에 배치
- Stay/Area pages로 실제 다음 질문을 연결

---

# 10. BATCH 5 — Transport Execution + Station Stay Detail

1. `incheon-airport-private-transfer.html`
2. `taxi.html`
3. `rental-car.html`
4. `hotels-near-seoul-station.html`
5. `hotels-near-gongdeok-station.html`

### 목적

Batch 4 Transport cluster와:

- Seoul Station Stay
- Gongdeok Stay

를 연결한다.

### 핵심

`hotels-near-seoul-station.html`은 English audit에서 contextual internal link **0**이므로 P0 fix 대상이다.

`hotels-near-gongdeok-station.html`은 P1 tune.

Thai page 제작 시:

- Accommodation
- AREX / Airport
- 해당 Travel/Area context

중 실제 다음 질문에 맞는 link를 함께 넣는다.

---

# 11. BATCH 6 — Core Payments

1. `payments.html`
2. `tmoney.html`
3. `wowpass.html`
4. `tmoney-vs-wowpass.html`
5. `apple-pay-korea.html`

### 목적

Payment Hub와 핵심 service pages를 먼저 만든다.

### SEO 구조

`payments.html`

→ T-money  
→ WOWPASS  
→ foreign cards / declined / ATM / online payment  
→ 실제 여행 상황

### WOWPASS 특별 규칙

WOWPASS는 SEO audit P0.

- 현재 승인된 factual correction이 있으면 반영
- Myeongdong / Hongdae / Accommodation과 실제 사용 맥락에서 연결
- service page 안에 갇히지 않게 한다.

---

# 12. BATCH 7 — Payment Failure Cluster + Apps

1. `foreign-credit-cards-korea.html`
2. `card-declined-korea.html`
3. `korea-atm-foreign-cards.html`
4. `korean-online-payments-foreigners.html`
5. `apps.html`

### 목적

Payments cluster를 완전히 닫고 Apps utility로 넘어간다.

### SEO 적용

Payment detail:

- failure / backup / action first
- service-to-service link만 반복하지 않음
- Arrival / Stay / Area로 자연스러운 cross-cluster bridge

Apps:

- 하나의 canonical app utility URL 유지
- Maps / Taxi / eSIM / Checklist로 연결 준비

---

# 13. BATCH 8 — Maps + eSIM + Planning

1. `maps.html`
2. `esim.html`
3. `best-esim-for-korea.html`
4. `korea-esim-with-phone-number.html`
5. `checklist.html`

### 목적

Thai 여행 전 준비 Utility cluster를 완성한다.

### SEO 적용

Maps/eSIM:

- 새 synonym URL 생성 금지
- 기존 canonical URL에서 Thai semantic coverage 강화
- Quick Answer / use case / failure handling
- Apps / Airport / Stay flow와 연결

Checklist:

- P2
- 대규모 구조변경 금지
- 이미 완성된 Utility/Transport/Stay page로 자연스럽게 분배

---

# 14. BATCH 9 — Attraction + Topic Hubs + Thai Home Hub

1. `lotte-world-seoul.html`
2. `seoul-sky-guide.html`
3. `k-beauty.html`
4. `taste-korea.html`
5. `index.html`

### 왜 마지막인가

이 Batch는 하위 페이지가 먼저 존재해야 가치가 커진다.

### Lotte World / Seoul Sky

이 시점에는:

- Jamsil Travel
- Jamsil Stay

가 이미 있으므로:

> Jamsil ↔ Lotte World ↔ Seoul Sky ↔ Jamsil Stay

구조를 실제 Thai sibling끼리 닫을 수 있다.

### K-Beauty / Taste Korea

SEO audit에서 Hub 기능이 약함.

이 시점에는 Area / Stay / Transport / Service page가 대부분 존재하므로:

- 관련 Area
- Stay
- Transport
- Service

로 실제 contextual exits를 만들 수 있다.

새 URL은 만들지 않는다.

### `index.html`

**58-page 현지화 순서상 가장 마지막에 처리한다.**

이유:

- `/th/` Home은 Country/Korea Hub
- child page가 실제 존재한 뒤 링크해야 future Thai URL 선링크를 피할 수 있음
- Traveloka/TrueID식 broad hub의 장점만 구조적으로 참고
- Korea Inside의 decision-first 성격 유지

목표:

> `/th/` → Stay / Area / Airport / Transport / Payment / Apps / eSIM / Attraction / Topic

실제 존재하는 Thai child page로 분배.

---

# 15. 각 페이지의 실제 현지화 작업 순서

각 페이지마다 아래 순서를 고정한다.

```text
1. 현재 English Production HTML 확인
2. 해당 17_TH_58_Page_Audit 행 확인
3. 기존 Research / Approved Copy / Family Standard가 있으면 재사용
4. user-facing source extraction 확인
5. ChatGPT Thai localization
6. facts / numbers / recommendation / structure 보존 QA
7. Thai naturalness QA
8. Search-facing QA
   - Title
   - Meta
   - H1
   - Lead
   - Quick Answer
9. 현재 존재하는 Thai sibling만 대상으로 contextual internal-link 판단
10. Localized Review MD 작성
11. ChatGPT self-QA
12. 사용자 wording 승인 1회
13. CONTENT LOCK
14. Codex exact implementation
15. static / staged QA
16. feature-branch/local 상태 유지
```

---

# 16. SEO 반영 기준 — 번역과 별도 재작업으로 만들지 않는다

앞으로 새 Thai 페이지를 번역할 때는 처음부터 아래를 함께 처리한다.

## A. Search-facing

- `<title>`
- meta description
- H1
- lead
- Short Answer / Quick Answer
- 주요 H2가 실제 Thai 질문/판단 언어인지

## B. Internal links

현재 문장에서 생기는 다음 질문만 연결.

예:

```text
Area Guide
→ Where to Stay
→ Accommodation
```

```text
Airport / Transport
→ 해당 이동이 편한 Stay 선택
→ Area Guide
```

```text
Payment / Apps
→ Arrival / Transport / Stay에서 실제로 사용하는 상황
```

## C. 금지

- competitor copy 복제
- keyword stuffing
- `best area` 기계적 직역
- SEO 때문에 recommendation 변경
- SEO 때문에 사실 추가
- 현재 source에 없는 호텔/장소 순위 추가
- 미래 `/th/` sibling 링크
- 새 URL 생성

---

# 17. Existing 13 Working Copy 재작업 방지 원칙

현재 13개는 이미 현지화되어 있으므로:

> **Full translation 재시작 금지**

다음만 허용:

1. 58-page audit에 따른 search-facing delta
2. 실제 Thai sibling이 새로 생성됐을 때 필요한 reciprocal contextual link
3. 명백한 Thai 품질 문제
4. 사용자 명시 재검토

“더 좋은 표현이 떠올랐다”만으로 본문 전체를 다시 쓰지 않는다.

---

# 18. Link Closure 운영 방식

재작업을 줄이기 위해 link closure는 세 번만 크게 한다.

## Closure A — Stay / Area
Batch 1~3 이후

## Closure B — Transport / Stay / Service
Batch 4~8 과정에서 각 신규 페이지가 기존 Thai pages로 link하도록 처리  
기존 Stay pages를 매 Batch마다 다시 열지 않는다.

## Closure C — Whole Thai
58/58 이후 Final Integration에서 전체 reciprocal / fallback residue audit

---

# 19. Thai 58/58 완료 후에만 Final Integration

일반 Batch에서는 하지 않는다.

58/58 + 사용자 명시 승인 후:

1. Thai internal-link closure
2. `lang="th"` 58/58
3. self canonical 58/58
4. hreflang final sibling set
5. 기존 Production 언어 reciprocal `th`
6. sitemap Thai 58 URLs
7. `/th/` exactly 1
8. `/th/index.html` 0
9. language switcher `ไทย`
10. `common.js` Thai support
11. affiliate/tracking parity
12. assets
13. FAQ/schema parity
14. public URL QA
15. Inventory COMPLETE

Partial Thai HTML Production은 기본적으로 하지 않는다.

---

# 20. Codex 역할

Codex:

- Source Extraction
- 승인된 Thai wording exact implementation
- approved internal-link implementation
- technical QA
- Git
- final integration

Codex 금지:

- Thai 문구 작성
- 번역
- Humanization
- SEO 문구 임의 개선
- 추천 판단 변경
- 누락 문구 추정
- unrelated file 수정

개별 anomaly는:

```text
SKIP / DEFER
→ 나머지 계속
→ 마지막 일괄보고
```

저장소 안전 문제만 전체 STOP.

---

# 21. Git / 보호 규칙

항상 보호:

- 기존 사용자 working-tree
- 현재 13 Thai working copies
- 이미 승인된 MD
- common header/navigation/footer
- `common.js`
- `style.css`
- 범위 밖 이미지/HTML

금지:

- `git add .`
- `git add -A`
- `git restore`
- `git reset`
- `git clean`
- `git stash`
- force push

---

# 22. 최종 진행 순서 한 줄 요약

```text
현재 13개 재번역 금지
→ Batch 1 Stay Decision + Gangnam/Hongdae
→ Batch 2 Insadong/Itaewon/Myeongdong/Dongdaemun
→ Batch 3 Jamsil/Seongsu/Gongdeok
→ Stay/Area SEO Closure 1회
→ Batch 4 Airport Core
→ Batch 5 Transport + Station Stay
→ Batch 6 Payment Core
→ Batch 7 Payment Failure + Apps
→ Batch 8 Maps/eSIM/Checklist
→ Batch 9 Attractions + Topic Hubs + index.html
→ 58/58 Whole-Language Final Integration
```

---

# 23. 완료 기준

이 workflow가 끝나면:

- Thai target: **58 / 58**
- current 13: 재번역 없이 SEO Delta 반영
- Remaining: **45 / 45**
- 새 URL: **0**
- future `/th/` broken links: **0**
- facts/numbers/recommendation drift: **0**
- keyword stuffing: **0**
- internal-link closure: PASS
- Thai search-facing naturalness: PASS
- Final Integration: 사용자 명시 승인 후 1회

**END — Use this file as the operational sequence in the Thai localization room.**
