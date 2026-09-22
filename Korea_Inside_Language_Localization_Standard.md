# Korea Inside — Language Localization Standard

**File:** `Korea_Inside_Language_Localization_Standard.md`  
**Status:** ACTIVE / SPECIALIZED STANDARD  
**Version:** 2.0  
**Effective date:** 2026-09-22  

**Higher standards**
1. `Korea_Inside_Public_Content_Master_Standard.md`
2. `Korea_Inside_Navigation_Hub_Architecture_Standard.md`

> 이 문서는 Korea Inside의 다국어 현지화 작업 방식을 고정한다.
>
> 핵심 원칙:
>
> **ChatGPT가 현재 English Production 원문을 직접 확인하고 자연스럽게 현지화한다.**
>
> **현지화 결과는 Batch Localization MD로 사용자에게 전달한다.**
>
> **사용자 승인 후 Codex는 승인된 MD를 읽고 대상 언어 HTML에 exact implementation만 한다.**
>
> **Codex는 번역·현지화·문구 판단을 하지 않는다.**
>
> **Inventory의 MISSING = 0이 되어야 해당 언어 전체 현지화 완료로 본다.**

---

# 1. 적용 우선순위

다국어 현지화 작업에서는 아래 우선순위를 적용한다.

1. 현재 사용자의 명시적 지시
2. `Korea_Inside_Public_Content_Master_Standard.md`
3. `Korea_Inside_Navigation_Hub_Architecture_Standard.md`
4. `Korea_Inside_Language_Localization_Standard.md`
5. 최신 `Korea_Inside_Room_Handover_*.md`
6. 해당 페이지 Research Master / Approved Public Copy / CONTENT LOCKED 자료
7. 현재 English Production HTML
8. 언어별 Localization Master Inventory

상위 기준과 충돌하는 하위 규칙은 적용하지 않는다.

---

# 2. 역할 분담 — 절대 고정

## 2.1 ChatGPT

담당:

- 현재 English Production 원문 전체 확인
- 페이지별 검색 의도와 역할 확인
- English 원문의 사실·수치·추천 판단·톤 보존
- 대상 언어의 자연스러운 현지화
- Humanization
- title / meta / heading / body / CTA / FAQ / alt / ARIA / JSON-LD 사용자 노출 문구 현지화
- 여러 페이지를 하나의 Batch Localization MD로 정리
- 사용자 승인 전 최종 문구 판단

## 2.2 사용자

담당:

- Batch 전체 최종 승인
- 필요한 경우 예외 승인
- 최종 방향 변경 승인

사용자가 `승인`이라고 하면 현재 Batch 전체 승인으로 처리한다.

승인 후 재번역하지 않는다.

## 2.3 Codex

담당:

- 승인된 Batch Localization MD 전체 읽기
- English HTML을 기준 구조로 사용
- 대상 언어 폴더의 동일 filename HTML에 승인 문구 exact implementation
- canonical / hreflang / sitemap
- 내부링크
- affiliate / tracking 보존 QA
- 정적 QA
- Git
- Production 배포
- 공개 URL QA
- Inventory 상태 갱신

## 2.4 Codex 금지

Codex는 임의로 다음을 하지 않는다.

- 번역
- 현지화
- 번역 준비용 English 문구 추출
- Source MD 생성
- 문법 개선
- 자연스럽게 수정
- 문구 추가 / 삭제
- 요약 / 확장
- Humanization
- 추천 판단 변경
- 호텔 / 지역 순위 변경
- 수치 변경
- 사실 수정
- affiliate 상품 변경

---

# 3. 고정 생산 흐름

다국어 현지화의 기본 생산 흐름은 다음과 같다.

```text
English Production 원문
→ ChatGPT 전체 확인
→ ChatGPT 현지화
→ Batch Localization MD 작성
→ 사용자 Batch 승인
→ Codex가 승인 MD 전체 읽기
→ 대상 언어 HTML exact implementation
→ QA
→ commit
→ push
→ Vercel Production
→ 공개 URL QA
→ Inventory MISSING → COMPLETE
```

중간에 불필요한 왕복 단계를 만들지 않는다.

금지 흐름:

```text
Codex가 English 문구 추출
→ Source MD 생성
→ ChatGPT가 다시 읽음
→ 번역
```

위 과정은 기본 현지화 절차로 사용하지 않는다.

---

# 4. English Production이 기준 원문

현지화 대상 문구는 현재 공개 중인 English Production 페이지를 기준으로 한다.

가능하면 ChatGPT가 공개 English 페이지를 직접 전체 확인한다.

현재 저장소 English HTML과 Production이 다를 가능성이 있으면 최신 승인 상태를 확인한다.

직접 경험하지 않은 내용을 새로 추가하지 않는다.

다른 지역의 문구·사실·추천 판단을 현재 페이지에 섞지 않는다.

---

# 5. 언어별 Master Inventory — 누락 방지 Source of Truth

언어별 현지화 완료 여부는 기억이나 대화 추측으로 판단하지 않는다.

언어별 Master Inventory를 사용한다.

각 English 공개페이지는 아래 세 상태 중 하나로 관리한다.

- `COMPLETE`
- `MISSING`
- `EXCLUDE`

## COMPLETE

다음을 만족해야 한다.

- 해당 언어 sibling 존재
- Production 공개
- HTTP 200
- self canonical 정상
- reciprocal hreflang 정상
- 핵심 내부링크 정상
- affiliate / tracking 정상
- QA PASS

## MISSING

- English 공개페이지는 존재
- 해당 언어 sibling이 아직 완료되지 않음

## EXCLUDE

- 관리 / 기술 / 임시 / 법적 페이지 등 현지화 대상에서 제외
- 제외 사유 기록 필수

해당 언어 전체 현지화 완료 조건:

> **Inventory의 MISSING = 0**

메뉴 현지화 완료는 사이트 전체 현지화 완료가 아니다.

Travel 완료, Stay 완료도 사이트 전체 현지화 완료가 아니다.

---

# 6. 대상 언어 폴더 작업본

Inventory의 `MISSING` English HTML은 필요하면 대상 언어 폴더에 동일 filename으로 한 번에 exact copy할 수 있다.

예:

```text
where-to-stay-in-hongdae.html
→ es/where-to-stay-in-hongdae.html
```

목적:

- 누락 방지
- 반복 복사 작업 제거
- 구조 보존
- Batch 작업 준비

복제 직후 파일은 **현지화 작업본**이다.

현지화 완료 전에는:

- stage 금지
- commit 금지
- push 금지
- deploy 금지
- sitemap 공개 등록 금지
- 미완료 언어 URL로 내부링크 전환 금지

영어 문구가 남아 있는 언어판 작업본을 Production에 올리지 않는다.

---

# 7. English 원본에서 반드시 보존할 것

English HTML 전체를 기준으로 가져오되 언어 의존 문구만 현지화한다.

반드시 보존:

- 사실
- 수치
- 추천 판단
- 페이지 역할
- section 순서
- HTML 구조
- class
- id
- `data-*`
- 이미지
- `srcset`
- 이미지 credit
- provenance
- 라이선스 정보
- 호텔명
- 장소명
- 브랜드명
- 상품명
- 객실명
- 객실 면적
- 침대 수
- 투숙 인원
- 주소
- 역명
- 출구 번호
- 버스 번호
- 노선 번호
- 거리
- 가격의 사실값
- 날짜의 사실값
- 운영시간의 사실값
- affiliate URL
- CID / subid / campaign parameter
- tracking attribute
- CSS
- JS
- schema 구조
- event ID
- 자동 연도 로직
- 상태 로직
- 기타 기능 코드

원본에 없는 사실을 현지화 과정에서 추가하지 않는다.

---

# 8. 현지화 대상

다음 사용자 노출 문구는 대상 언어로 자연스럽게 현지화한다.

- `<title>`
- meta description
- H1 / H2 / H3
- lead / intro
- body copy
- 호텔 설명
- 비교 / 판단 문구
- CTA visible text
- button text
- FAQ 질문 / 답변
- breadcrumb visible text
- caption의 설명 문구
- alt text
- ARIA / accessible language text
- affiliate disclosure visible text
- JSON-LD의 사용자 노출 질문 / 답변 / 설명
- 사용자에게 보이는 상태 문구

이미 승인된 공통 메뉴명은 페이지마다 다시 번역하지 않는다.

---

# 9. 번역이 아니라 현지화

현지화는 영어 문장 구조를 그대로 옮기는 작업이 아니다.

목표:

> **영어 원문의 사실·판단·톤을 보존하면서 대상 언어 원어민이 자연스럽게 읽는 문장으로 다시 작성한다.**

허용:

- 어순 변경
- 문장 분리 / 결합
- 자연스러운 연결어
- 현지 언어의 관용적 표현
- 현지 여행 콘텐츠에 자연스러운 문체

금지:

- 의미 축소
- 의미 확장
- 추천 결론 변경
- 누가 적합한지 판단 변경
- 누가 부적합한지 판단 삭제
- 호텔 순위 변경
- 사실 변경
- 수치 변경
- 상품 조건 변경

직역투가 남는 경우 현지화 완료로 보지 않는다.

---

# 10. Batch Localization MD — 기본 Handoff 문서

대량 현지화에서는 **Batch Localization MD를 기본 handoff 방식**으로 사용한다.

ChatGPT가 작성한다.

권장 Batch 크기:

- 5~10페이지
- 같은 Family / 역할 우선
- 지나치게 긴 페이지는 더 작은 Batch 허용

파일 예:

```text
Korea_Inside_Stay_Area_Hotel_ES_Localized_Batch1_2026-09-22.md
```

Batch MD에는 Codex가 exact implementation할 수 있을 정도의 위치 식별정보와 현지화 문구를 포함한다.

권장 구조:

```md
# PAGE: where-to-stay-in-hongdae.html

## SEO
Title:
...

Meta:
...

## H1
...

## SECTION: ...
### H2
...

### BODY
...

### CTA
...

### FAQ
...

### ALT / ARIA
...

### JSON-LD USER-FACING COPY
...
```

필요하면 보호값을 함께 명시한다.

```text
Protected:
- Hotel name
- Exit 4
- 28 m²
- affiliate URL
```

HTML 전체를 MD에 복제할 필요는 없다.

Codex가 승인 문구의 적용 위치를 정확히 찾을 수 있을 만큼의 구조만 남긴다.

---

# 11. 사용자 승인

ChatGPT가 Batch Localization MD를 완성하면 사용자에게 다음만 간단히 보고한다.

- 대상 페이지
- 현지화 완료 여부
- 특이사항
- 보호값 확인
- 파일 링크

대화창에 전체 현지화 본문을 길게 다시 출력하지 않는다.

사용자가 `승인`하면:

- 해당 Batch 전체 승인
- 재번역 0
- 재작성 0
- Codex exact implementation 단계로 이동

---

# 12. Codex 구현 방식

Codex는 승인된 Localization MD를 전체 읽은 후:

1. English 원본 구조 확인
2. 대상 언어 작업본 확인
3. 승인된 언어 의존 문구만 exact replace
4. 구조 / 사실 / 수치 / affiliate / tracking 보존
5. canonical / hreflang 적용
6. 존재하는 언어 sibling 내부링크 적용
7. sitemap 적용
8. QA
9. commit / push / Production
10. 공개 URL QA
11. Inventory COMPLETE 갱신

Codex는 승인 문구의 자연스러움을 다시 판단하지 않는다.

---

# 13. 내부링크 규칙

내부링크는 실제 Production에 존재하는 언어 sibling만 대상 언어 URL로 연결한다.

```text
해당 언어 sibling 존재
→ 해당 언어 URL

해당 언어 sibling 없음
→ English fallback
```

금지:

- 미래 URL 추측
- 404 언어 URL 생성
- 미완료 작업본 URL로 공개 연결

새 sibling이 Production되면 기존 English fallback을 해당 언어 URL로 갱신할 수 있다.

---

# 14. SEO / hreflang 규칙

각 언어 sibling은 기본적으로:

- 해당 언어 `lang`
- self canonical
- reciprocal hreflang
- `en`
- 대상 언어 코드
- `x-default`
- sitemap 등록
- index 허용
- H1 1개
- 언어에 맞는 title
- 언어에 맞는 meta description

English root 구조는 유지한다.

예:

```text
English:
https://www.getkoreainside.com/hongdae-travel-guide.html

Spanish:
https://www.getkoreainside.com/es/hongdae-travel-guide.html
```

IP / browser-language 자동 redirect는 사용하지 않는다.

---

# 15. Affiliate 보호 규칙

현지화 과정에서 affiliate가 깨지면 안 된다.

반드시 보존:

- href
- CID
- subid
- campaign parameter
- tracking attributes
- placement / context
- 상품의 실제 역할

현지화 가능한 것은 CTA의 사용자 노출 문구뿐이다.

금지:

- 일반 URL로 교체
- tracking parameter 삭제
- 다른 partner로 임의 변경
- 현지화 때문에 상품 추천 변경

---

# 16. 공통 UI 보호

사용자의 명시적 승인 없이 다음을 수정하지 않는다.

- common header
- navigation
- footer
- `common.js`
- mobile hamburger
- 공통 `style.css` 시스템

이미 승인된 대상 언어 Golden Sample UI가 있으면 같은 역할의 페이지에 재사용한다.

공통 메뉴명은 이미 현지화된 값을 사용한다.

---

# 17. QA 기준

각 Batch 완료 전 아래를 확인한다.

## 콘텐츠

- English 의미 누락 0
- 사실 mismatch 0
- 수치 mismatch 0
- 추천 판단 mismatch 0
- 호텔 / 장소 순서 mismatch 0
- 직역투 주요 오류 0

## 구조

- section 누락 0
- class / id 변경 0
- 이미지 누락 0
- srcset mismatch 0
- CSS / JS 변경 0

## SEO

- lang 정상
- self canonical
- reciprocal hreflang
- sitemap 각 URL 1회
- H1 1개
- title / meta 현지화
- 존재하지 않는 언어 URL 0

## 링크

- 존재하는 sibling → 대상 언어
- 없는 sibling → English fallback
- broken internal link 0

## Affiliate

- href mismatch 0
- tracking mismatch 0

## FAQ / Schema

- visible FAQ와 schema 의미 일치
- schema 구조 보존
- 사용자 노출 문구 현지화
- English 사용자 노출 문구 잔존 여부 확인

## Git

- 범위 외 staged 0
- 기존 사용자 변경 보호
- `git add .` 금지
- `git add -A` 금지
- conflict marker 0
- 의도하지 않은 whitespace 오류 0

---

# 18. Markdown hard-break 예외

Approved / Localization MD에서 Markdown hard-break를 위해 줄 끝에 정확히 두 개의 ASCII space를 사용하는 경우:

- 의도된 Markdown 문법으로 인정 가능
- 사용자 승인 없이 정규화하지 않는다
- 본문 바이트 보존을 우선한다

QA에서는:

- 예외 건수 명시
- Approved / Localization MD 외 whitespace error 0 확인

---

# 19. Git / Production 기본 완료 순서

사용자가 commit / push / deploy 금지를 명시하지 않은 Production 작업에서는:

```text
정적 QA
→ 범위 파일만 stage
→ staged 범위 검증
→ commit
→ push
→ ahead / behind 확인
→ Vercel Production READY
→ Production commit 확인
→ 공개 URL HTTP 200
→ canonical / hreflang / internal link / affiliate QA
→ Inventory COMPLETE 갱신
```

범위 외 staged 파일이 있으면 commit하지 않는다.

---

# 20. 절대 보호

언어 현지화 작업에서 임의로 수정하지 않는다.

- English Approved Public Copy
- DONE LOCKED
- CONTENT LOCKED
- 기존 사용자 working-tree 변경
- Accommodation 관련 사용자 변경
- `_CleanTemp/`
- 공통 header / navigation / footer
- common.js
- style.css 공통 시스템
- mobile hamburger
- affiliate tracking
- 이미지 라이선스 / provenance
- 범위 외 파일

---

# 21. 현지화 완료 판정

언어판 완료는 일부 메뉴나 일부 카테고리 완료가 아니다.

완료 조건:

> **해당 언어 Master Inventory에서 MISSING = 0**

따라서:

- 메뉴 번역 완료 ≠ 전체 완료
- Travel 완료 ≠ 전체 완료
- Stay 완료 ≠ 전체 완료
- 주요 페이지 완료 ≠ 전체 완료

Inventory가 최종 완료 판정 기준이다.

---

# 22. 핵심 작업 원칙

> **ChatGPT가 English Production 원문을 직접 읽고 현지화한다.**

> **현지화 결과는 Batch Localization MD로 만든다.**

> **사용자가 Batch 전체를 승인한다.**

> **Codex는 승인 MD를 읽고 exact implementation만 한다.**

> **Codex는 번역·현지화·문구 추출 작업을 하지 않는다.**

> **사실·수치·추천 판단·HTML 구조·affiliate·tracking은 잠근다.**

> **문장은 대상 언어 원어민이 자연스럽게 읽도록 현지화한다.**

> **Inventory의 MISSING = 0이 되어야 해당 언어 전체 현지화 완료다.**
