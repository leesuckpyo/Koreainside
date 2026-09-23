# Korea Inside 작업 인계 — 2026-09-23

**Status:** ACTIVE HANDOVER  
**Project:** Korea Inside  
**Repository:** `C:\Projects\Koreainside`  
**Production:** `https://www.getkoreainside.com/`  
**Primary branch:** `main`  
**Created:** 2026-09-23

---

# 1. 새 방 시작 시 반드시 먼저 읽을 문서

아래 순서대로 전체 읽고 현재 상태를 그대로 승계한다.

1. `Korea_Inside_Public_Content_Master_Standard.md`
2. `Korea_Inside_Navigation_Hub_Architecture_Standard.md`
3. `Korea_Inside_Language_Localization_Standard.md`
4. 이 파일: `Korea_Inside_Room_Handover_2026-09-23.md`

다국어 작업을 계속할 경우 추가로:

5. `md/작업자료/Korea_Inside_Spanish_Localization_Master_Inventory_2026-09-22.md`
6. 현재 Batch의 Source MD
7. 현재 Batch의 Approved Public Copy MD
8. 대상 English Production HTML / Spanish 작업본

상위 기준 충돌 시:

`Public Content Master Standard`
→ `Navigation Hub Architecture Standard`
→ 현재 사용자의 최신 명시 지시
→ `Language Localization Standard`
→ Handover

단, 현재 사용자 명시 지시는 항상 해당 작업 범위에서 최우선이다.

---

# 2. 절대 고정 역할 분담

## ChatGPT + 사용자

담당:

- 공개 콘텐츠 문구
- 번역 / 현지화
- Humanization
- 추천 판단
- 사용자 노출 문구 결정
- 예외 승인
- 최종 승인

## Codex

기본 담당:

- 승인 문구 exact implementation
- HTML/CSS 기술 적용
- canonical / hreflang / sitemap
- 내부링크
- QA
- Git
- Production

Codex가 임의로 하지 않는 것:

- 번역
- 현지화
- 문법 개선
- 문구 추가/삭제
- 요약/확장
- Humanization
- 추천 판단 변경
- 호텔/지역 순위 변경

---

# 3. 중요 — 현지화 생산방식 최신 승인 변경

기존 `Korea_Inside_Language_Localization_Standard.md v2.0`에는
Codex의 문구 추출 / Source MD 생성 금지가 적혀 있다.

그러나 이후 사용자가 다음 새 workflow를 명시적으로 승인했다.

이 최신 사용자 승인 workflow가 해당 부분을 **대체한다.**

## 현재 승인된 Localization Workflow

```text
English Production HTML
→ Codex가 현지화 대상 사용자 노출 문자열 100% 기술 추출
→ Source MD 생성
→ ChatGPT가 Source MD 전체를 Spanish 현지화
→ Batch Localization MD
→ 사용자 Batch 승인
→ Approved Public Copy — CONTENT LOCKED
→ Codex exact implementation
→ 정적 QA
→ scoped stage
→ commit
→ push
→ Vercel Production READY
→ 공개 URL QA
→ Inventory 갱신
```

Codex의 문자열 추출은 **기술 추출만 허용**한다.

금지:

- 번역
- 현지화
- 문법 개선
- 추천 판단
- Humanization
- 임의 요약/확장

이 workflow를 다시 과거 방식으로 되돌리지 않는다.

---

# 4. 사용자 노출 data-* 예외 — 최신 확정

기본적으로 `data-*`는 구조 보호 대상이다.

하지만 다음처럼 CSS/JS가 실제 화면에 노출하는 값은
기술 데이터가 아니라 **사용자 노출 문구**로 본다.

예:

```css
content: attr(data-label)
```

따라서 모바일 화면에서 실제 표시되는:

```html
data-label="Area"
data-label="Works well for"
```

같은 값은 반드시 Source ITEM으로 추출하고 현지화한다.

반대로 다음은 계속 보호한다.

- tracking
- analytics
- affiliate
- event
- functional `data-*`

이 판단은 Spanish Batch 2에서 45개 누락 blocker가 발생한 뒤 확정되었고,
Batch 3부터 Source Extraction 단계에서 처음부터 반영한다.

---

# 5. Spanish Golden Samples / 기본 구조

Spanish 폴더:

`/es/`

English:

root 유지, `/en/` 없음.

Golden Samples:

- Travel: `es/dongdaemun-travel-guide.html`
- Stay: `es/where-to-stay-in-dongdaemun.html`

기본 SEO:

- self canonical
- reciprocal `en / es / x-default`
- sitemap 등록
- 존재하는 Spanish sibling만 `/es/`
- 미완료 sibling은 English fallback
- 존재하지 않는 Spanish URL 생성 금지
- IP/browser 자동 redirect 금지

---

# 6. Spanish Batch 2 — Production 완료 / 확정

2026-09-23 현재 Production 완료.

대상 6페이지:

1. `es/accommodation.html`
2. `es/hongdae-vs-myeongdong.html`
3. `es/best-area-for-first-time-visitors-seoul.html`
4. `es/best-area-for-families-seoul.html`
5. `es/best-area-for-solo-travelers-seoul.html`
6. `es/hongdae-travel-guide.html`

최종 결과:

- ITEM: `1,974 / 1,974`
- 누락: `0`
- page-specific English: `0`
- 사용자 노출 `data-label`: 45개 Spanish 적용
- Hongdae runtime badge: 12/12 Spanish
- runtime English: 0
- affiliate/tracking mismatch: 0
- canonical/hreflang/sitemap: PASS
- broken internal link: 0
- 구조/class/id/기타 data-*/event ID/이미지/srcset mismatch: 0
- 공개 URL 6/6 HTTP 200
- Vercel Production success

Hongdae event-status Spanish:

```text
UPCOMING → PRÓXIMAMENTE
HAPPENING NOW → EN CURSO
ENDED → FINALIZADO
```

`event-status.js`는 `document.documentElement.lang === "es"`일 때만 Spanish를 사용하고
English/기타 언어 동작은 유지하는 최소 language branch로 처리함.

Batch 2 commits:

- 구현 commit:
  `43b6db5ce5b78bcf5bc1b59d6857664a3dbfcb34`
- Inventory commit / 최종 확인 HEAD:
  `7b69d16e21c837dcfa7cbe82db904c9c8f52ebca`

당시:

- ahead/behind `0/0`
- staged `0`

Batch 2 완료 후 Inventory:

```text
COMPLETE 26
MISSING 37
```

실제 브라우저 desktop/tablet/mobile 시각 QA는 연결 가능한 브라우저 부재로
Known QA Limitation이었고 사용자가 실제 기기에서 확인하는 방식으로 남김.

---

# 7. 현재 작업 — Spanish Stay Batch 3

## 대상 6페이지

1. `best-area-for-airport-access-seoul.html`
2. `best-area-for-budget-travelers-seoul.html`
3. `best-area-for-couples-seoul.html`
4. `best-area-for-luxury-hotels-seoul.html`
5. `best-area-for-nightlife-seoul.html`
6. `best-area-for-shopping-seoul.html`

## Source Extraction — 완료

Source MD:

`md/작업자료/Korea_Inside_Stay6_ES_Localization_Source_Batch3_2026-09-23.md`

페이지별 ITEM:

- Airport access: 148
- Budget travelers: 259
- Couples: 201
- Luxury hotels: 214
- Nightlife: 183
- Shopping: 248

총:

```text
1,253 ITEM
ITEM 001–1253
번호 누락 0
중복 0
```

추출 QA:

- title/meta/H1-H3 누락 0
- body/CTA/FAQ 누락 0
- FAQ 질문/답변 ITEM 113개
- alt 32개
- literal ARIA 3개
- related-card ITEM 54개
- JSON-LD 사용자 노출 102개
- 사용자 노출 `data-label` 72개
  - Budget travelers 36
  - Shopping 36
- 미수록 page-specific English 0
- 범위 외 변경 0
- Source SHA-256 6/6 일치
- protected token이 원문에 없는 사례 0
- UTF-8 replacement character 0

---

# 8. Spanish Stay Batch 3 — ChatGPT 현지화 완료 + 사용자 승인 완료

ChatGPT가 Source 1,253 ITEM 전체를 Spanish로 현지화함.

Localization MD 생성:

`Korea_Inside_Stay6_ES_Localized_Batch3_2026-09-23.md`

사용자가 `승인`이라고 명시함.

따라서 Batch 전체:

**APPROVED PUBLIC COPY — CONTENT LOCKED**

승인본 파일명:

`Korea_Inside_Stay6_ES_Approved_Public_Copy_Batch3_2026-09-23.md`

저장소 목표 경로:

`md/승인본/스페인어/Korea_Inside_Stay6_ES_Approved_Public_Copy_Batch3_2026-09-23.md`

중요:

- 승인 후 재번역 금지
- 재작성 금지
- Codex는 exact implementation만
- 모바일 사용자 노출 `data-label` 72개도 승인 범위
- `Korea`가 `Korean`의 부분문자열로 보호 토큰에 잡힌 기계적 false-positive는 자연스러운 Spanish 형용사 `coreano/coreana/coreanos/coreanas` 허용
- 실제 고유명사 `Korea Inside`, `Korea Tourism Organization` 등은 그대로 보존

---

# 9. Batch 3 현재 정확한 진행 상태

현재 **사용자 승인까지 완료**.

Codex에게 exact implementation + QA + Production 지시문까지 전달함.

하지만 이 Handover 생성 시점에는 아직 Codex의 최종 Production 완료 보고가 없음.

따라서 현재 확정 상태:

```text
Batch 3 Source Extraction: COMPLETE
Batch 3 Spanish Localization: COMPLETE
Batch 3 User Approval: COMPLETE / CONTENT LOCKED
Batch 3 Codex implementation: PENDING / 진행 여부 다음 방에서 확인
Batch 3 Production: NOT YET CONFIRMED
Inventory: 아직 확정적으로 26 / 37
```

Codex Production 성공 후 목표:

```text
COMPLETE 32
MISSING 25
```

**Production 성공 보고 전에 32/25로 확정해서 말하지 않는다.**

---

# 10. Batch 3 Codex 구현 시 고정 QA

반드시 확인:

- `1,253 / 1,253` 승인 ITEM 적용
- 누락 0
- 모바일 `data-label 72 / 72`
- page-specific user-facing English 0
- title/meta/H1-H3
- body
- CTA/button
- FAQ
- JSON-LD
- related cards
- alt/ARIA
- caption
- table text
- 사실/수치 mismatch 0
- 추천 판단 mismatch 0
- affiliate/tracking mismatch 0
- 구조/class/id mismatch 0
- image/srcset mismatch 0
- broken internal link 0
- H1 각 1개
- self canonical
- reciprocal en/es/x-default
- sitemap Spanish URL 각 1회

Production 기본 순서:

```text
정적 QA
→ 범위 파일만 stage
→ staged 범위 검증
→ commit
→ push
→ ahead/behind
→ Vercel Production READY
→ 공개 URL 6/6 HTTP 200
→ 실제 Spanish 본문/SEO/link/affiliate QA
→ Inventory 6개 MISSING → COMPLETE
```

브라우저 연결이 없으면:

- 실제 화면 QA = Known QA Limitation
- 정적 responsive QA + Production HTTP/HTML QA가 PASS하면 배포 진행 가능
- 사용자가 이후 실제 desktop/mobile 화면 확인

---

# 11. 보호해야 할 기존 사용자 변경

다음은 계속 절대 보호:

- Accommodation 이미지 관련 사용자 삭제/교체/신규 파일
- `_CleanTemp/`
- 범위 밖 `/es/` 작업본
- 기타 기존 working-tree 변경
- English CONTENT LOCKED / DONE LOCKED
- 범위 밖 MD

금지:

```text
git add .
git add -A
git restore ...
git reset ...
git clean ...
```

사용자 명시 승인 없이:

- common header
- navigation
- footer
- common.js
- mobile hamburger
- 공통 style.css

수정 금지.

다음 방에서는 정확한 working tree 숫자를 추측하지 말고 Codex `git status`로 실제 상태 확인.

---

# 12. Language selector — 미완료 이슈

기존 direct toggle을 dropdown으로 변경한 상태.

원칙:

- 현재 언어 active/check
- 같은 페이지의 available sibling으로 이동
- `hreflang`을 source of truth로 사용
- 없는 sibling은 비활성/404 생성 금지
- browser/IP auto redirect 금지

Desktop은 정상 확인된 적 있음.

하지만 실제 Android에서:

**언어 변경이 동작하지 않는 mobile selector bug가 보고됨.**

이 문제는 아직 해결 완료 보고가 없음.

또한 EN/ES 메뉴 라벨 길이에 따른 header width shift도 남아 있음.

이 두 문제는 Spanish Batch와 섞지 말고 별도 기술 작업으로 처리.

---

# 13. 디자인 TODO — Hongdae Stay Hero

사용자가 실제 화면을 보고 확정한 디자인 수정 희망:

현재 `where-to-stay-in-hongdae.html` Hero에서
사진이 H1/본문 오른쪽에 배치된 구조가 마음에 들지 않음.

사용자 의도:

**Travel Guide처럼 Hero 사진을 H1 아래에 넓게 배치**

예상 구조:

```text
H1
→ Hero image
→ lead/body
```

이건 지금 Spanish Batch 작업과 섞지 않고
나중에 Hongdae Stay 디자인 수정 작업으로 별도 처리.

사용자가 처음 “지도 위치”라고 했으나 즉시 “사진이구나”라고 정정함.
따라서 대상은 지도 아니라 **Hero 사진 위치**.

---

# 14. Facebook / Meta 현재 맥락

Facebook Page:

`Get Korea Inside`

Intro:

`Practical Korea travel guides for first-time visitors. Plan smarter, travel easier.`

최근 게시물용 copy 작업:

- Jamsil Travel Guide
- Seoul Accommodation / Where to Stay
- Hongdae Stay Guide

숙소가이드 메시지 핵심:

> 호텔을 고르기 전에 어느 동네가 내 여행에 맞는지 먼저 고른다.

Hongdae Stay 메시지 핵심:

> 지도상 가까움 ≠ 실제로 짐 들고 이동하기 편함.

Footer Facebook link 추가는 사용자가 “나중에 하자”고 해서 보류.

Meta 광고 계정은 false-positive restriction 후 review를 거쳐
`Access reinstated` 상태까지 확인된 적 있음.
광고 실제 live/spend 상태는 이후 다시 확인하기 전에는 추측하지 않는다.

---

# 15. 공항 짐보관 콘텐츠 후보 — 보류

사용자가 향후 Travel Tips 콘텐츠로 추가 예정이라고 함.

주제:

- Incheon Airport baggage/luggage storage
- early arrival
- late departure
- shopping
- transit
- luggage-heavy travelers

현재 스크린샷/블로그에서 수집한 값은
공식 최신 사실로 확정된 것이 아니므로
공개 콘텐츠 작성 전 공식 출처 재검증 필요.

지금은 작업하지 않는다.

---

# 16. 다음 Spanish 작업 방향

Batch 3 Production 완료 후 Inventory를 실제 확인한다.

Batch 3 성공 시 예상:

```text
COMPLETE 32
MISSING 25
```

그 다음 Batch는 Inventory에서 실제 MISSING 페이지를 다시 읽고
같은 workflow로 진행한다.

절대 기억으로 다음 25개를 추측하지 않는다.

순서:

```text
Inventory 확인
→ 같은 Family 묶음 선정
→ Codex 100% Source Extraction
→ ChatGPT Spanish localization
→ 사용자 승인
→ Codex exact implementation
→ QA / Production
```

---

# 17. Standard 업데이트 필요 — 아직 미완료

`Korea_Inside_Language_Localization_Standard.md v2.0`에는 아직:

- Codex extraction 금지
- Source MD 금지

가 남아 있음.

하지만 현재 실제 승인 workflow는 반대다.

따라서 나중에 별도 관리 작업으로 Standard를 개정해서 다음을 공식 반영해야 한다.

1. Codex의 100% 기술 문자열 추출 허용
2. Source MD 생성 허용
3. 번역/문구 판단은 계속 ChatGPT + 사용자만
4. user-facing `data-label` 현지화 규칙
5. extracted ITEM count = localized ITEM count = applied ITEM count QA

사용자 승인 없이 지금 당장 Standard 파일을 수정하지 않는다.

---

# 18. 현재 Git / Production 기준점

마지막 확정 Production HEAD:

`7b69d16e21c837dcfa7cbe82db904c9c8f52ebca`

이는 Batch 2 완료 시점 기준.

Batch 3 Source Extraction 이후 Codex 보고:

- Source MD 1파일 생성
- stage 0
- commit 0
- push 0
- deploy 0

이후 ChatGPT Approved MD는 채팅에서 생성되었음.

다음 방에서:

- Approved MD가 저장소에 실제 저장되었는지
- Batch 3 Codex가 구현을 시작/완료했는지
- 현재 HEAD
- staged
- ahead/behind

를 실제 Codex 상태로 확인해야 함.

---

# 19. 새 방에서 가장 먼저 물어볼 현재 상태

새 방 첫 실제 작업 전에 Codex/사용자에게 필요한 핵심 확인:

```text
Spanish Stay Batch 3 Codex 구현/배포가 완료됐는가?
```

완료 보고가 있다면:

- commit SHA
- push
- Vercel READY
- 공개 URL 6/6
- Inventory 32/25
- staged 0
- protected user changes

를 검증/승계.

완료 전이라면 그대로 Batch 3 implementation부터 이어간다.

---

# 20. 새 방 첫 메시지용 지시문

아래를 새 방 첫 메시지로 그대로 사용해도 됨.

```text
Korea Inside 작업을 이어간다.

작업 시작 전에 저장소에서 아래를 순서대로 전체 읽어.

1. Korea_Inside_Public_Content_Master_Standard.md
2. Korea_Inside_Navigation_Hub_Architecture_Standard.md
3. Korea_Inside_Language_Localization_Standard.md
4. 가장 최신 Korea_Inside_Room_Handover_*.md

다국어 작업이므로 Spanish Master Inventory도 읽어.

최신 Handover의 확정 상태를 그대로 승계하고
과거 작업을 기억이나 추측으로 다시 설계하지 마.

특히 현재 승인된 최신 localization workflow는:

Codex 100% 기술 문자열 추출
→ Source MD
→ ChatGPT 현지화
→ 사용자 Batch 승인
→ Codex exact implementation
→ QA / Git / Production

이다.

기존 Localization Standard에 Codex extraction 금지 문구가 있어도
이 최신 사용자 승인 workflow가 해당 부분을 대체한다.

먼저 아래 6가지만 보고해.

1. 읽은 기준 MD 전체 경로
2. 각 문서 전체 내용 확인 여부
3. 최신 Handover 확인 여부
4. 현재 Spanish Batch 상태
5. 지금 바로 해야 할 다음 작업
6. 보호해야 할 기존 사용자 변경

현재 핵심:
Spanish Stay Batch 3의 6페이지 / 1,253 ITEM은
ChatGPT 현지화와 사용자 승인까지 완료되어 CONTENT LOCKED다.

Approved MD:
md/승인본/스페인어/Korea_Inside_Stay6_ES_Approved_Public_Copy_Batch3_2026-09-23.md

Source MD:
md/작업자료/Korea_Inside_Stay6_ES_Localization_Source_Batch3_2026-09-23.md

Batch 3 Production 완료 여부는 추측하지 말고
현재 실제 Git/Codex 상태를 확인해서 이어가.
```

---

# 21. 현재 상태 한 줄 요약

**Spanish Batch 2 Production 완료로 Inventory 26/37 → Stay Batch 3 6페이지 Source 1,253 ITEM 추출 완료 → ChatGPT Spanish 현지화 완료 → 사용자 승인 / CONTENT LOCKED 완료 → 현재 Codex exact implementation 및 Production 완료 보고 대기 상태.**

