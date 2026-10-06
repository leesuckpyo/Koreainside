# Korea Inside — Thai Localization Standard

**File:** `Korea_Inside_Thai_Localization_Standard.md`  
**Status:** ACTIVE / SPECIALIZED STANDARD  
**Version:** 1.1  
**Effective date:** 2026-10-02  
**Applies to:** Korea Inside 태국어(Thai) 현지화 전체  
**Target audience:** 태국에서 한국 여행을 준비하는 태국어 사용자  
**Primary folder:** `/th/`  
**HTML language:** `th`  
**hreflang:** `th`  
**Language-switcher label:** `ไทย`  
**Compact language code:** `TH`

**Higher standards**
1. `Korea_Inside_Public_Content_Master_Standard.md`
2. `Korea_Inside_Navigation_Hub_Architecture_Standard.md`
3. `Korea_Inside_Language_Localization_Standard.md`

> 이 문서는 Korea Inside 태국어판의 **현지화 품질, 문체, 검색의도, 구현 및 QA 방식을 장기적으로 동일하게 유지하기 위한 태국어 전용 실행 표준**이다.
>
> 태국어 현지화는 영어 문장을 태국어 단어로 치환하는 작업이 아니다.
>
> **현재 English Production의 사실·수치·추천 판단·페이지 역할·HTML 구조를 보존하면서, 태국 여행자가 실제로 검색하고 읽고 선택하는 방식으로 자연스럽게 다시 쓰는 작업**이다.
>
> 이 문서는 상위 Standard를 대체하지 않는다. 충돌하면 상위 기준을 따른다.

---

# 0. 이 문서를 사용하는 방법

태국어 신규 현지화, 태국어 Humanization, 태국어 SEO 문구 작성, 태국어 Batch Review Copy, 태국어 Production QA에 이 문서를 적용한다.

작업 시작 순서:

1. `Korea_Inside_Public_Content_Master_Standard.md`
2. `Korea_Inside_Navigation_Hub_Architecture_Standard.md`
3. `Korea_Inside_Language_Localization_Standard.md`
4. `Korea_Inside_Thai_Localization_Standard.md`
5. 최신 `Korea_Inside_Room_Handover_*.md`
6. 해당 Page Family에 실제 ACTIVE Design Standard가 있으면 해당 문서
7. 해당 페이지 Research Master / Approved Public Copy / CONTENT LOCKED 자료
8. 현재 English Production HTML
9. `Korea_Inside_Thai_Localization_Master_Inventory_*.md`

새 방에서 태국어 실제 작업을 처음 시작할 때 위 기준을 확인한다.

같은 방에서 이미 전체 확인했고 파일이 바뀌지 않았다면 Batch마다 반복해서 전체 읽지 않는다.

다시 확인하는 조건:

- Standard Version 또는 내용 변경
- 더 최신 Handover 생성
- 새 Specialized Standard 적용
- 처음 작업하는 새 Page Family
- 사용자 재확인 지시
- 실제 작업과 Standard 사이의 충돌 발견

Standard 자체를 검토·수정하는 작업은 Production Batch 시작으로 보지 않는다.

---

# 0.1 Version 1.0 초기 Golden Sample

Version 1.0의 Thai 문체 기준점은 사용자 승인된 `accommodation.html` 1-page pilot이다.

현재 Review MD 기준 경로:

`md/승인본/태국어/Korea_Inside_TH_Accommodation_Pilot_Localized_Review_2026-10-02.md`

이 Pilot이 사용자 승인 상태를 유지하는 동안 다음 용도로 사용한다.

- Thai 문체
- Thai Stay 검색어 방향
- Common UI 태국어 표현
- 문장 길이
- CTA 톤
- FAQ 톤
- alt / ARIA 자연스러움

Golden Sample에서 확립된 대표 검색 표현:

- `ที่พักโซล`
- `พักโซลย่านไหนดี`

중요:

> Golden Sample은 **문체와 현지화 방식의 기준**이지, 다른 페이지의 사실·추천 판단·문장을 복제하는 템플릿이 아니다.

다른 페이지의 사실·SERP·호텔·지역 판단은 해당 English source와 해당 페이지 자료를 기준으로 새로 판단한다.

---

# 1. 적용 우선순위

충돌 시:

1. 현재 사용자의 명시적 지시
2. `Korea_Inside_Public_Content_Master_Standard.md`
3. `Korea_Inside_Navigation_Hub_Architecture_Standard.md`
4. 해당 작업에 적용되는 공통 Specialized Standard / Production Playbook
5. `Korea_Inside_Language_Localization_Standard.md`
6. `Korea_Inside_Thai_Localization_Standard.md`
7. Active Family Design Standard
8. 최신 Handover
9. Thai Master Inventory
10. Research Master
11. Approved Public Copy / CONTENT LOCKED
12. 현재 English Production HTML

Handover는 현재 상태 문서다. 상위 정책을 변경하지 않는다.

Inventory는 완료 여부를 관리한다. 콘텐츠 정책을 정하지 않는다.

---

# 1.1 현재 작업 범위 잠금 — SCOPE LOCK

이 Standard는 **작업 범위를 확대하는 명령이 아니다.**

실제 작업 범위는 다음 우선순위로 정한다.

1. 현재 사용자가 명시한 페이지·파일·자산·Batch 범위
2. 현재 진행 중인 Batch의 확정 범위
3. 최신 Handover가 기록한 active scope
4. 이 Standard의 기본 Batch 규칙

따라서 사용자가 `5페이지`, `인포그래픽 5개`, `현재 Batch만`, `이 파일 1개`, `다음 5개`라고 지시하면 **정확히 그 범위만** 처리한다.

금지:

- 전체 58페이지 규칙 때문에 현재 5페이지 작업을 58페이지로 확대
- Final Integration 규칙을 일반 Batch에 조기 적용
- 현재 5개 infographic 작업을 전체 infographic inventory 감사로 확대
- 현재 Batch에 필요하지 않은 다른 Family / 다른 언어 / 다른 자산을 예방적으로 재감사
- 이미 완료된 앞 Batch를 다시 처음부터 검사
- `next`를 임의로 `INF-001부터 다시 시작`처럼 해석

`다음 5개`는 현재 Batch/Handover/Inventory에서 **실제 다음 미완료 대상 5개**를 판정한다.
대상을 확정할 근거가 부족하면 전체 범위를 넓히지 말고 현재 상태에서 확인 가능한 대상만 좁혀 보고한다.

> **Standard는 품질·보호·QA 기준을 제공한다. 현재 작업의 양을 늘리지 않는다.**

> **범위가 5개면 5개만 한다. 나머지는 다음 Batch다.**

---

# 2. Thai 언어 아키텍처 — 고정

Version 1.0의 Thai 기본값:

- folder: `/th/`
- `<html lang="th">`
- hreflang: `th`
- menu label: `ไทย`
- compact code: `TH`
- homepage: `https://www.getkoreainside.com/th/`
- detail: `https://www.getkoreainside.com/th/FILENAME`
- `x-default`: English canonical sibling

Thai 페이지가 하나뿐인 현재 구조에서는 `th`를 사용한다.

`th-TH`와 `th`를 페이지마다 섞지 않는다.

향후 별도 Thai regional variant가 실제로 필요해지면 Standard Version을 올리고 전체 구조를 일관되게 변경한다.

현재 언어 메뉴 순서에 Thai를 추가할 때 기본 순서:

`English → Français → Deutsch → Español → 日本語 → 繁中 → ไทย`

단, `common.js`는 보호파일이므로 최종 통합 단계에서 사용자 명시 승인 후 수정한다.

---

# 3. 태국어판의 목표

태국어판 목표:

> **คู่มือเที่ยวเกาหลีที่คนไทยอ่านแล้วตัดสินใจได้จริง**

동시에 만족해야 한다.

1. **정확성**
   - English의 사실·수치·날짜·가격·조건·추천 강도 보존
2. **자연스러운 태국어**
   - 영어 어순 제거
   - 번역투 제거
   - 태국 여행 콘텐츠에서 실제로 읽히는 문장
3. **태국 여행자의 검색·판단 방식**
   - 실제 Thai query wording을 검색-facing 영역에 자연스럽게 반영
   - 검색어 삽입 때문에 페이지 역할이나 추천 판단을 바꾸지 않음
4. **Korea Inside 정체성 유지**
   - Decision-first
   - Humanization
   - 실제 이동 마찰
   - 조건과 trade-off
   - OTA형 나열 금지

---

# 4. 역할 분리 — 절대 고정

## 4.1 ChatGPT

ChatGPT가 직접 판단한다.

- Thai localization
- Thai Humanization
- Thai 문법·어순
- title / meta description
- H1 / H2 / H3
- body
- CTA / button
- FAQ
- breadcrumb
- alt / caption
- ARIA
- 사용자 노출 JSON-LD
- Thai search-intent wording
- Thai 지명/용어 표기
- 공식 Thai 용어와 영어 원형을 어느 위치에서 병기할지

## 4.2 사용자

사용자가 최종 결정한다.

- Batch wording 승인
- CONTENT LOCK
- 예외 승인
- 검색의도 방향 변경
- 추천 판단 변경
- Production 승인
- 공통 UI 변경 승인

사용자가 `승인`이라고 하면 현재 제시된 Batch 전체 승인으로 처리한다.

## 4.3 Codex

Codex 역할:

- 승인된 Thai MD exact implementation
- 사용자 승인 범위 안의 Mechanical Source Extraction
- HTML/CSS 기술 적용
- canonical
- hreflang
- sitemap
- 내부링크 구현
- affiliate/tracking 보존 QA
- Git
- Production
- public QA

## 4.4 Codex 금지

Codex가 임의로 하지 않는다.

- 태국어 번역
- 태국어 문법 개선
- 자연스럽게 다시 쓰기
- 검색어 추가
- Humanization
- 문장 추가·삭제·병합
- 요약·확장
- 지명 표기 변경
- 추천 판단 변경
- 호텔/지역 순위 변경
- 숫자/가격/조건 변경
- 누락 문구 추정
- 승인되지 않은 public copy 작성

---

# 5. English Production이 기준 원문

Thai 현지화의 사실·구조 Source of Truth는 현재 English Production이다.

기본 흐름:

```text
Current English Production
→ source structure / user-visible node inventory
→ Thai human localization
→ user approval
→ exact implementation
```

과거 기억, 다른 언어판, 경쟁사 문구만으로 Thai public copy를 만들지 않는다.

다른 언어판은:

- 문체 비교
- 구현 구조
- QA 방식

참고용으로만 사용할 수 있다.

다른 언어판의 사실·추천 판단을 Thai에 섞지 않는다.

---

# 6. 현지화 전에 English 구조를 잠근다

태국어 문구 작성 전에 현재 English HTML의 사용자 노출 target을 전수 확인한다.

최소:

- title
- meta description
- OG/Twitter user-facing copy
- H1 / H2 / H3
- paragraph
- list
- table caption / th / td
- dt / dd
- summary / details
- CTA
- button
- visible link text
- breadcrumb
- status text
- review/check date
- source label
- figure caption
- alt
- ARIA
- user-facing JSON-LD
- data-label 등 사용자 노출 data attribute
- inline JS 안의 사용자 노출 string
- 기타 accessibility tree에 노출되는 language-dependent string

금지 방식:

```text
페이지를 대충 읽음
→ 섹션을 요약
→ 태국어로 새로 작성
```

고정 방식:

```text
English structure lock
→ source target inventory
→ Thai localization
→ coverage 100%
```

---

# 7. Source Coverage 100%

Thai Approved Copy는 English 사용자 노출 source의 coverage 100%가 기본이다.

완료 조건:

- unmapped source target: 0
- 빈 Thai target: 0
- 승인되지 않은 신규 target: 0
- source order mismatch: 0
- 원문에 없는 섹션/heading 추가: 0

Inline element로 문장이 나뉘어 있어도 화면상 하나의 문장이라면 자연스러운 Thai 문장으로 만들 수 있다.

단:

- HTML 구조는 보존
- 모든 source node가 매핑
- `<strong>` 때문에 Thai 어순을 망가뜨리지 않음

Approved MD와 현재 English 구조가 다르면 Codex가 빈칸을 임의 번역하지 않는다.

---

# 8. Source Fingerprint / Drift Control

가능하면 페이지별로 다음 중 하나를 기록한다.

- Git blob SHA
- SHA-256
- 현재 Production과 repository source 일치 확인

Approved 후 English source가 바뀐 경우:

- 기술 변경인지 확인
- 사용자 노출 copy/structure 변경이면 해당 변경부분만 재현지화
- 기존 approved Thai 전체를 이유 없이 다시 번역하지 않음

---

# 9. English에서 반드시 보존할 것

보존:

- 사실
- 수치
- 추천 판단
- 추천 강도
- page role
- section 순서
- HTML 구조
- class / id / data-*
- 이미지
- srcset / imagesrcset
- credit / provenance / license
- 호텔명
- 장소명
- 브랜드명
- 상품명
- 객실명
- 객실 면적
- 침대
- 정원
- 주소
- 역
- 출구
- 노선
- 버스 번호
- 거리
- 가격
- 날짜
- 운영시간
- affiliate URL
- CID / subid / campaign
- tracking
- CSS / JS
- schema 구조
- event/status/year logic

Thai라는 이유로 원문에 없는 사실을 추가하지 않는다.

---

# 10. Thai로 현지화할 것

대상:

- title
- meta description
- OG / Twitter user-facing title / description
- H1 / H2 / H3
- body
- CTA / button
- FAQ
- breadcrumb
- visible source label
- alt / caption
- ARIA
- user-facing JSON-LD
- table headings
- dt/dd labels
- UI helper text
- 상태 문구

이미 승인된 Thai Common UI는 페이지마다 새로 번역하지 않는다.

---

# 11. Thai 기본 Register

기본 톤:

- 명확함
- 실용적
- 자연스러움
- 과장하지 않음
- 지나치게 공식적이지 않음
- 지나치게 채팅체가 아님
- 독자가 바로 선택할 수 있음

본문에서 기본적으로 사용하지 않는다.

- `ครับ`
- `ค่ะ`
- `นะคะ`
- `นะครับ`

Korea Inside editorial voice는 성별 화자에 종속되지 않는 중립 문체를 사용한다.

필요하면:

- `ถ้า...`
- `หาก...`
- `สำหรับคนที่...`
- `ถ้าให้ความสำคัญกับ...`
- `ถ้ามีกระเป๋า...`
- `ถ้ากลับดึก...`

처럼 조건을 직접 드러낸다.

---

# 12. Thai Humanization

Humanization은 감성 문구 추가가 아니다.

Thai 독자가 다음을 실제로 판단할 수 있어야 한다.

- 누구에게 맞는가
- 누구에게 안 맞는가
- 숙박 위치 때문에 하루 동선이 어떻게 달라지는가
- 짐이 있으면 무엇이 달라지는가
- 환승과 마지막 도보가 어디서 피로해지는가
- 늦은 도착이면 무엇이 달라지는가
- 아이/가족/성인 그룹이면 무엇이 달라지는가
- 객실 크기와 침대가 실제로 충분한가
- 예약조건이 선택에 어떤 영향을 주는가
- 가격이 싸도 전체 여행비가 더 비싸질 수 있는가

English 판단을 보존하면서 Thai 문장으로 자연스럽게 설명한다.

직접 경험하지 않은 장소를:

- `เราไปมาแล้ว`
- `เราเคยพัก`
- `จากประสบการณ์ของเรา`

처럼 직접 체험한 것처럼 쓰지 않는다.

---

# 13. Thai에서 피할 AI / 번역투 패턴

다음 표현 자체가 항상 금지는 아니지만 반복되면 FAIL 후보다.

- `เหมาะสำหรับ...`
- `เป็นตัวเลือกที่ดี`
- `เป็นตัวเลือกที่เหมาะ`
- `สะดวก`
- `คุ้มค่า`
- `ดีที่สุด`
- `แนะนำ`
- `เป็นทางเลือกที่ใช้งานได้จริง`
- `สมเหตุสมผล`
- `ขึ้นอยู่กับ...` 반복
- `ข้อดีคือ... / ข้อเสียคือ...` 반복
- `ไม่ใช่ X แต่เป็น Y` 기계적 반복
- 모든 선택지를 긍정적으로 끝내는 패턴
- 동일한 `추천 → 교통 → 장점 → 주의점` 구조 반복

판단 근거를 문장 안에 넣는다.

직역식:
`ตัวเลือกที่ดีที่สุดขึ้นอยู่กับโรงแรมที่แน่นอน เวลามาถึง กระเป๋า และขนาดกลุ่ม`

자연스러운 방향:
`ถ้าพักใกล้สถานี มาถึงไม่ดึก และมีกระเป๋าไม่มาก รถไฟอาจจัดการง่ายกว่า แต่ถ้ามากันหลายคนหรือมีสัมภาระเยอะ รถแท็กซี่หรือรถรับส่งอาจลดความเหนื่อยได้มากกว่า`

핵심은 단어 치환이 아니라 조건과 판단 흐름이다.

---

# 14. Thai Typography / Spacing

태국어는 영어처럼 단어마다 공백을 넣지 않는다.

원칙:

- Thai 단어 사이에 영어식 공백을 기계적으로 삽입하지 않음
- 문장·의미 덩어리 사이 공백은 Thai 읽기 흐름에 맞게 사용
- Latin brand / 숫자 / code와 Thai 사이 공백은 문맥상 자연스럽게 사용
- 링크 태그 앞뒤에 불필요한 공백을 만들지 않음
- double space 금지
- stray non-breaking space / zero-width character 임의 삽입 금지
- line wrapping을 위해 Thai 단어 내부에 space 삽입 금지

영어 source의 문장 끝 `.`을 Thai 문장 끝에 기계적으로 유지하지 않는다.

다만 다음은 보존 가능:

- URL
- 공식 브랜드 표기
- 약어
- decimal
- code
- 영어 인용문

질문형 heading에는 실제 Thai 문맥에 맞으면 `?`를 사용할 수 있다.

영어식 colon/semicolon을 문장마다 기계적으로 복제하지 않는다.

---

# 15. 숫자 / 날짜 / 단위

숫자는 기본적으로 Arabic numerals를 유지한다.

예:

- 2명
- 15분
- 2026
- 3.5 km
- 10:30

Thai 숫자 `๑๒๓`로 일괄 변환하지 않는다.

중요:

- Gregorian year를 Buddhist Era로 임의 변환하지 않는다.
- `2026`을 `2569`로 바꾸지 않는다.
- KRW 가격을 THB로 임의 환산하지 않는다.
- source에 없는 환율을 추가하지 않는다.
- source 숫자와 조건을 반올림하거나 단순화하지 않는다.

단위 label은 Thai 문장에 자연스럽게 현지화할 수 있지만 값은 보존한다.

---

# 16. 지명 표기 원칙

Thai 지명은 다음 순서로 판단한다.

1. Thai 공식기관/운영기관이 쓰는 확립 표기
2. Thai 여행시장에 널리 정착된 표기
3. 현재 approved Thai Golden Sample
4. 필요한 경우 English/Korean 원형 병기

확립 표기의 예:

- Seoul → `โซล`
- Incheon → `อินชอน`
- Hongdae → `ฮงแด`
- Myeongdong → `เมียงดง`
- Gangnam → `กังนัม`
- Itaewon → `อิแทวอน`
- Dongdaemun → `ทงแดมุน`

위 목록을 모든 고유명사에 기계적으로 확대하지 않는다.

표기가 불확실한 지명은:

- 공식 Thai source
- KTO Thai
- 운영기관 Thai 페이지
- Thai 검색결과의 확립 usage

를 확인한다.

같은 페이지 안에서 한 지명의 철자를 여러 방식으로 섞지 않는다.

---

# 17. 브랜드 / 서비스명

브랜드·제품·서비스는 공식 원형을 기본으로 한다.

예:

- AREX
- T-money
- WOWPASS
- KTX
- eSIM
- Apple Pay
- Naver Map
- KakaoMap
- Google Maps
- Trip.com
- Agoda
- Expedia
- Klook
- Korea Inside

Thai 독자에게 설명이 필요하면 Thai 설명을 붙일 수 있다.

하지만 브랜드 자체를 임의 번역·변형하지 않는다.

현재 approved Golden Sample 또는 공식 Thai market naming이 이미 있으면 동일하게 유지한다.

---

# 18. Thai 검색의도 현지화

SEO는 English page role을 바꾸는 작업이 아니다.

보존:

- 대표 검색 문제
- page role
- recommendation judgment
- facts
- URL identity

현지화:

- Thai query wording
- 자연스러운 질문형 title/H1
- Thai 여행자가 쓰는 용어
- Thai search-facing 어순

대표 Stay 방향:

- `ที่พักโซล`
- `พักโซลย่านไหนดี`
- `พักย่านไหนดีในโซล`
- `ที่พักใกล้...`

대표 Airport/Transport 방향:

- `จากสนามบินอินชอนไปโซล`
- `วิธีเดินทางจากสนามบินอินชอนเข้าโซล`
- `AREX`
- `รถบัสสนามบิน`
- `บัตร T-money`

Travel Guide 방향:

- `เที่ยวฮงแด`
- `เที่ยวเมียงดง`
- `[지역] เที่ยวอะไร`
- `[지역] คู่มือเที่ยว`

위 표현은 검색어 예시다.

모든 페이지에 기계적으로 삽입하지 않는다.

Thai SERP 때문에 English page role 자체를 바꿀 필요가 있다면 현지화로 처리하지 않고 사용자에게 별도 보고한다.

---

# 19. Thai-specific 사실 추가 금지

다음처럼 태국인에게 중요해 보이는 정보라도 English/Research에 없으면 자동 추가하지 않는다.

예:

- `พนักงานพูดภาษาไทย`
- `เมนูภาษาไทย`
- `คนไทยนิยม`
- `เหมาะกับคนไทย`
- Thai card 전용 혜택
- Thai phone number 전용 조건
- Thai bank/payment 특례
- 태국인 리뷰가 많다는 주장
- Thai tour package availability

필요하면:

1. 공식 자료 조사
2. 사실 검증
3. 페이지 역할 판단
4. 사용자 승인

후 English/다국어 전체 fact layer 문제로 처리한다.

---

# 20. Travel Guide Thai 기준

Travel Guide는 장소 나열이 아니다.

Thai에서도 다음을 유지한다.

- 왜 가는가
- 어느 시간대가 맞는가
- 누구에게 맞는가
- 누구에게 안 맞는가
- 얼마나 머무는가
- 무엇을 먼저 해야 하는가
- 무엇을 굳이 넣지 않아도 되는가
- 다음 장소로 어떻게 이어지는가

제목에서 `คู่มือเที่ยว` 또는 자연스러운 `เที่ยว...`를 고려할 수 있다.

모든 Area Guide에 같은 title template을 강제하지 않는다.

---

# 21. Stay / Hotel Thai 기준

Korea Inside의 Accommodation Decision Layer를 그대로 보존한다.

Thai에서 핵심 질문:

> **ไม่ใช่แค่ว่าโรงแรมนี้ดีไหม แต่คือโรงแรมนี้ใช้ชีวิตจริงระหว่างทริปของคนกลุ่มนี้สะดวกไหม**

English에 있다면 축소하지 않는다.

- 객실 면적
- 침대
- 정원
- 가족/성인 그룹
- 짐
- 역 출구
- 엘리베이터
- 계단
- 마지막 도보
- 언덕/도로횡단
- 공항버스 정류장
- 체크인
- 짐보관
- 소음
- 객실 방향
- 예약/취소/결제 조건

`สะดวก`, `คุ้ม`, `ดี`, `แนะนำ` 한 단어로 판단을 끝내지 않는다.

왜 그런지 실제 근거를 붙인다.

---

# 22. Airport / Transport Thai 기준

독자가 바로 알고 싶은 실행 순서:

- 어디서 타는가
- 어떤 표를 사는가
- 얼마인가
- 얼마나 걸리는가
- 어디에 서는가
- 짐이 있으면 어떤가
- 막차 후 대안은 무엇인가
- 역/정류장에서 호텔까지 무엇이 남는가

`เร็วที่สุด`와 `สะดวกที่สุด`를 같은 뜻처럼 번역하지 않는다.

English가 “fastest but not easiest door-to-door”라면 그 trade-off를 Thai에서도 살린다.

---

# 23. Service / Payment / App Thai 기준

기능 나열보다 먼저:

- 누구에게 필요한가
- 누구에게 불필요한가
- 실제 사용 조건
- 충전/결제/환불
- 해외 발행 카드
- 실패 가능성
- 대안

제품명은 공식 원형을 유지한다.

UI label은 짧고 자연스러운 Thai로 현지화한다.

작은 `<dt>`, table label, helper text도 누락하지 않는다.

---

# 24. Thai Common UI Golden Sample

Thai Common UI가 Pilot에서 승인되면 동일 문구를 재사용한다.

매 페이지마다 새로 번역하지 않는다.

보호 대상:

- common header
- navigation
- footer
- `common.js`
- mobile hamburger
- shared `style.css`

사용자 명시 승인 없이 공통 파일을 변경하지 않는다.

Common UI wording correction이 필요하면:

- 페이지 body 수정과 분리
- 정확한 공통 영향 범위 보고
- 사용자 승인 후 처리

---

# 25. Internal Link Thai 규칙

Batch 작성 단계의 원칙:

```text
Thai sibling implementation-complete
→ /th/ sibling 사용 가능

아직 Thai sibling 미완료
→ English fallback 유지
```

금지:

- 미래 `/th/` URL 추정
- 영어 working copy를 Thai COMPLETE로 간주
- 존재하지 않는 Thai sibling으로 링크
- 404 생성

58/58 구현 후 Final Integration에서:

- Thai sibling closure audit
- English fallback residue audit
- 모든 실제 Thai sibling으로 정리

Legal 페이지처럼 별도 Thai sibling이 없는 보호된 root URL은 예외가 될 수 있다.

---

# 26. Canonical / hreflang

Thai page 기본:

- folder `/th/`
- `lang="th"`
- self canonical
- `x-default = English canonical`

hreflang은 현재 실제 Production sibling set을 사용한다.

Version 1.0 작성 시 이미 Production COMPLETE인 sibling은 기본적으로:

- `en`
- `es`
- `ja`
- `zh-TW`
- `fr`
- `de`

Thai final integration 후:

- `th`

를 포함한다.

모든 실제 Production sibling끼리 reciprocal이어야 한다.

미공개 Thai working copy를 Production HTML hreflang에 미리 추가하지 않는다.

Final Integration에서는 기존 Production 6개 언어 58페이지 전체에 `th` reciprocal을 추가하는 작업이 필요하다.

---

# 27. FAQ / JSON-LD

English source에 FAQPage가 있으면 구조를 보존한다.

English source에 schema가 없으면 Thai에 새 schema를 만들지 않는다.

Visible FAQ와 schema가 같은 질문/답변을 관리하면 Thai 승인 문구는 exact-copy를 기본으로 한다.

필수:

- 질문 수 일치
- 순서 일치
- 답변 수 일치
- 의미 일치
- page-specific English residue 0
- 승인 문구 parity

FAIL:

- visible Thai / schema English
- visible 10 / schema 9
- punctuation/space 때문에 실제 string이 다름
- English source에 없는 FAQPage 추가
- 전체 whitespace를 삭제해서 mismatch를 숨김

Thai string 비교 시 HTML entity decode는 가능하지만 의미 있는 Thai spacing은 보존한다.

---

# 28. Thai Unicode / Whitespace 보호

Thai는 결합 문자와 spacing 때문에 기계적 “정리”가 위험하다.

금지:

- 전체 Unicode normalization을 이유 없이 재작성
- zero-width character 일괄 삽입/삭제
- Thai 단어 사이 자동 space 삽입
- 전체 whitespace collapse로 QA PASS 처리
- formatter로 Thai 문장 전체 재직렬화

Approved Thai string은 가능하면 exact byte/character sequence를 구현한다.

Markdown hard-break 예외는 별도다.

---

# 29. Markdown hard-break 예외

공통 Localization Standard의 Markdown hard-break 예외는 오직:

- `.md`
- 줄 끝 정확히 두 개 ASCII space
- 의도된 Markdown line break

에만 적용한다.

적용 금지:

- HTML visible text
- JSON-LD
- FAQ parity
- href
- attribute
- meta content

HTML/JSON-LD의 불필요한 공백은 예외로 끌고 가지 않는다.

---

# 30. 이미지 / alt / caption / ARIA

이미지 파일과 구조는 보존한다.

현지화:

- alt
- caption
- ARIA label/description
- 사용자 노출 title

alt는 English를 단어별 번역하지 않는다.

이미지에 실제 보이는 내용을 Thai로 자연스럽게 설명한다.

원문에 없는 사실을 추가하지 않는다.

이미지 filename과 alt 내용이 충돌해 보이면 이미지 교체나 사실수정을 임의로 하지 않고 별도 보고한다.

`src`, `srcset`, `imagesrcset`, sizes는 구조 보호 대상이다.

---

# 31. Thai Batch 크기

초기 언어 품질 확립:

> **1-page Pilot**

Pilot 승인 후 기본 Batch:

> **5페이지**

Version 1.0에서는 `accommodation.html` Pilot이 Golden Sample 역할을 한다.

그 이후 사용자가 별도 지시하지 않는 한 5페이지를 기본으로 한다.

예외:

- 초장문/고복잡도 페이지가 많으면 더 작게
- 짧고 구조가 동일한 Family면 사용자 승인 후 확대 가능
- 마지막 Batch는 잔여 페이지 수에 맞춤

---

# 32. Batch 구성

가능하면 같은 Family를 묶는다.

예:

Stay Decision:
- comparison / first-time / family / solo / couples 등

Airport / Transport:
- airport
- arrival
- AREX
- airport bus
- transfer

Travel Guide:
- area guide / attraction

단, Family 통일보다 구조 복잡도와 QA 안정성을 우선한다.

---

# 32.1 현재 Batch 연속성 / 예방적 전체감사 금지

Thai 작업은 방·컴퓨터·세션이 바뀌어도 현재 Batch 상태를 이어간다.

기본 원칙:

- 이미 승인된 Batch 범위를 다시 설계하지 않는다.
- 이미 완료된 Source Extraction을 다시 만들지 않는다.
- 기존 Audit / Inventory / Approved Copy가 현재 source와 일치하면 재사용한다.
- 파일명이나 번호를 기억으로 추정하지 않는다.
- 현재 5개를 처리 중이면 그 5개가 끝난 뒤에만 다음 5개로 넘어간다.

예방적으로 하지 않는다:

- 전체 58페이지 재감사
- 전체 infographic 재감사
- 모든 언어 reciprocal hreflang 재감사
- 전체 sitemap 재작성
- 모든 Family Standard 재독
- 이미 DONE / CONTENT LOCKED인 Batch 재번역

현재 Batch를 정확히 처리하기 위한 최소 범위 검증만 허용한다.

---

# 32.2 Infographic / Visual Localization Batch 규칙

Infographic, map, comparison graphic, instructional visual 등 이미지 내부에 user-facing text가 있는 2차 자산 현지화도 Batch 단위로 처리한다.

기본 Batch:

> **5 assets**

사용자가 다른 수를 명시하면 그 수가 우선한다.

중요:

- `5개`라고 하면 정확히 5개만 조사·현지화·QA한다.
- 전체 infographic inventory를 동시에 재감사하지 않는다.
- 기존 infographic audit가 현재 source와 일치하면 재사용한다.
- 현재 Batch의 실제 affected page / asset mapping을 먼저 확인한다.
- 번호를 임의로 `INF-001~005`라고 가정하지 않는다.
- `다음 5개`는 기존 완료상태 다음의 실제 미완료 5개를 사용한다.

기본 흐름:

```text
현재 Batch asset 5개 확정
→ English embedded-text source 확인
→ ChatGPT Thai localization
→ 사용자 wording 승인
→ localized asset production
→ pixel/SVG QA
→ 해당 Thai HTML src replacement
→ Browser/visual QA where available
→ staged QA
→ feature branch commit/push
```

이미 ES/JA 등에서 source path, SHA-256, dimensions, embedded English inventory, P1/P2/EXCLUDE classification, 제작 방식이 검증돼 있다면 재사용한다.

Thai 문구는 다른 언어를 번역하지 않고 English source를 기준으로 ChatGPT가 작성한다.

실제 카드·기계·제품 외형처럼 기존 final classification이 `EXCLUDE / BRAND-PRODUCT VISUAL`이면 Thai에서도 임의로 가짜 Thai 제품 이미지를 만들지 않는다.

현재 infographic Batch에 속하지 않는 asset은 수정·생성·HTML src 변경·재QA를 하지 않는다.

---

# 33. Thai Batch Localization MD 필수 구성

각 페이지 최소:

```md
# PAGE N — filename.html

## SOURCE
- English file
- source fingerprint
- H1/H2/H3 counts
- FAQ visible/schema counts
- alt/caption/ARIA counts
- important user-facing data label counts

## SEO
- title
- meta
- OG/Twitter if user-facing source exists

## H1

## SOURCE ORDER
- 모든 user-facing target을 English source order대로

## FAQ
- visible
- schema paired target

## ALT / CAPTION / ARIA

## PROTECTED
- facts
- numbers
- recommendation
- affiliate/tracking
- URLs
- structure
```

Batch MD는 Codex가 추측 없이 exact implementation할 수 있어야 한다.

---

# 34. Review / Approval 상태

Thai 작업 상태는 구분한다.

1. `LOCALIZATION REVIEW`
   - ChatGPT 작성·자체검토 완료
   - 사용자 승인 전

2. `APPROVED PUBLIC COPY — CONTENT LOCKED`
   - 사용자 wording 승인 완료
   - Codex 재작성 금지

3. `IMPLEMENTED`
   - feature branch/local HTML에 exact implementation
   - QA 필요

4. `PRODUCTION COMPLETE`
   - Production 공개
   - public QA PASS
   - Inventory COMPLETE

Working copy 존재만으로 COMPLETE가 아니다.

---

# 35. Approved 후 변경 금지

사용자가 승인한 Thai public copy는 임의로 다시 열지 않는다.

재개방:

- 명백한 사실 오류
- 구조 drift로 인한 누락
- 사용자 명시적 재검토
- 승인 이후 새로 생긴 명백한 문법/표기 결함
- Production regression

단순히 다른 표현이 더 좋아 보인다는 이유로 전체 재번역하지 않는다.

---

# 36. Thai Golden Sample의 역할

Golden Sample이 정하는 것:

- 문체
- Thai 질문형 search wording의 방향
- CTA 강도
- FAQ 톤
- Common UI style
- proper-name 처리 방향

Golden Sample이 정하지 않는 것:

- 모든 페이지 title template
- 모든 지역의 추천 판단
- 다른 호텔의 사실
- 다른 페이지의 section 구성
- 모든 고유명사 철자

“Golden Sample과 똑같이”는 복제가 아니라 **동일 품질 원칙**을 의미한다.

---

# 37. Thai Search QA

검색-facing 영역:

- title
- meta
- H1
- lead
- 주요 H2
- FAQ question

확인:

- 실제 Thai 질문처럼 읽히는가
- `best area`를 직역한 어색한 표현이 없는가
- keyword stuffing이 없는가
- page role이 English와 같은가
- recommendation strength가 유지되는가

Stay에서 특히:

`พื้นที่ที่ดีที่สุดสำหรับ...`

같은 영어 구조 직역을 기계적으로 반복하지 않는다.

Thai에서는 페이지 역할에 맞으면:

`พักโซลย่านไหนดี`

같은 자연스러운 질문형을 우선 검토한다.

---

# 38. ChatGPT 자체 QA — 사용자 승인 전

사용자에게 Review Copy를 보여주기 전에 ChatGPT가 먼저 확인한다.

## Structure
- source order
- H1/H2/H3
- FAQ
- alt/ARIA
- visible source labels
- inline JS user-facing strings

## Coverage
- 100%
- UNMAPPED 0
- 빈 target 0

## Content
- fact mismatch 0
- number mismatch 0
- recommendation drift 0
- invented fact 0
- fabricated firsthand 0

## Thai quality
- literal English syntax 제거
- unnatural spacing 제거
- gendered polite endings 반복 0
- AI phrase repetition 최소화
- place/brand spelling consistency
- Thai search phrasing naturalness

## Search
- title/meta/H1 page role 일치
- keyword stuffing 0

사용자에게 “미완성 Review”를 던져 반복 승인 루프를 만들지 않는다.

---

# 39. Codex 구현 QA

Codex가 확인:

- approved mapping 100%
- missing mapping 0
- position mismatch 0
- English residue 0
- 다른 언어 residue 0
- structure mismatch 0
- tag order mismatch 0
- class/id/data-* mismatch 0
- href mismatch 0
- affiliate/tracking mismatch 0
- image/srcset mismatch 0
- JSON-LD parse error 0
- FAQ parity
- `lang="th"`
- self canonical
- internal links
- `git diff --check`

Codex가 자연스러운 Thai인지 새로 판단하여 문장을 바꾸면 안 된다.

---

# 40. Anomaly 처리 — 전체 작업 중단 최소화

개별 파일/문자열/URL/asset에서 이상을 발견했다고 전체 Batch를 즉시 중단하지 않는다.

기본:

```text
개별 anomaly
→ 해당 항목 SKIPPED / DEFERRED
→ 나머지 승인범위 계속
→ QA 가능한 항목 끝까지
→ 마지막에 일괄 보고
```

임의 추정·수정은 하지 않는다.

전체 STOP은 저장소 전체 안전 문제에만 적용한다.

예:

- wrong repository
- wrong branch
- 예상 HEAD와 심각한 drift
- Git conflict
- 사용자 변경 덮어쓰기 위험
- stage manifest를 신뢰할 수 없음
- origin/main이 push 직전에 변경
- reset/restore 없이는 안전하게 진행 불가

미해결 Release blocker가 있으면 Production은 하지 않는다.

즉:

> **작업은 끝까지, 불완전한 Release는 배포하지 않는다.**

---

# 41. Git 작업 보호

항상 보호:

- 기존 사용자 working-tree 변경
- 범위 밖 file
- common protected files

금지:

- `git add .`
- `git add -A`
- `git restore`
- `git reset`
- `git clean`
- `git stash`
- force push

stage는 승인 manifest만 명시적으로 한다.

Staged QA를 commit 직전 필수 Gate로 둔다.

---

# 42. Thai Production 전략 — 기본

Version 1.0 기본 전략:

> **Partial Production 금지. 58/58 완료 후 Whole-Language Final Integration.**

작업 branch 기본:

`th-localization`

Batch:

- implementation
- static QA
- explicit staged QA
- commit/push to `origin/th-localization`
- main/Production에는 아직 반영하지 않음

현재 작업이 이미 다른 사용자 승인 branch/방식으로 시작됐다면,
Standard에 맞추겠다는 이유만으로 reset/move/rewrite하지 않는다.

그 예외는 Handover에 기록한다.

---

# 43. Thai 58/58 Final Integration — FINAL INTEGRATION ONLY / 일반 Batch에서는 비활성

이 Section은 **일반 5페이지 Batch에는 적용하지 않는다.**

활성화 조건:

1. 사용자가 명시적으로 `Final Integration`, `최종 통합`, `58/58 마감`을 지시함
2. Thai 58/58 implementation이 완료됐고 사용자가 최종 통합 진행을 승인함

위 조건 전에는 기존 6개 언어 348 HTML, 전체 sitemap Thai 등록, `common.js`, language switcher, main/Production 전체 통합을 시작하지 않는다.

이 Section은 미래 최종 마감 절차를 정의하는 참고 규칙이며 현재 Batch 범위를 확대하는 실행 명령이 아니다.


현재 English localization target population이 58페이지라면 Final Integration에서:

1. Thai 58/58 implementation complete
2. Thai internal-link closure
3. `lang="th"` 58/58
4. Thai self canonical 58/58
5. Thai page hreflang = 실제 Production sibling set 전체 + `th` + x-default
6. 기존 Production EN/ES/JA/zh-TW/FR/DE 58페이지 각각에 reciprocal `th` 추가
7. sitemap Thai 58개 추가
8. `/th/` exactly 1
9. `/th/index.html` 0
10. language switcher `ไทย` 추가
11. `common.js` Thai support
12. affiliate/tracking parity
13. assets
14. FAQ/schema
15. public URL QA

현재 6개 기존 언어가 모두 58페이지라면 reciprocal `th` 추가 대상은:

`6 × 58 = 348 HTML`

이다.

실제 Final Integration 시에는 현재 Production 언어 세트를 다시 계산한다.

---

# 44. common.js / Language Switcher Final Integration — FINAL INTEGRATION ONLY / 일반 Batch에서는 비활성

Section 43의 활성화 조건을 충족하기 전에는 이 Section을 실행하지 않는다.


`common.js`는 보호파일이다.

Thai 58/58 Final Integration에서 사용자 명시 승인 후에만:

- `th` support
- label `ไทย`
- compact/current code `TH`
- menu order
- alternate mapping

을 수정한다.

현재 localhost 보호 로직을 깨지 않는다.

Local development에서 언어전환:

- localhost
- 127.0.0.1
- [::1]

origin/port 유지.

Production에서는 Production sibling URL 유지.

Thai 추가 때문에 기존 6개 언어 menu behavior를 변경하지 않는다.

---

# 45. Final Release Staged QA — FINAL INTEGRATION ONLY

일반 Batch의 staged QA는 Section 41을 따른다.
이 Section은 Thai whole-language Final Integration manifest에만 적용한다.


Final Integration에서 commit 전에 반드시 staged QA.

확인:

- staged manifest exact
- unexpected staged 0
- common protected file은 승인된 경우만 포함
- `git diff --cached --check` PASS
- Thai 58 files
- reciprocal hreflang count
- sitemap count
- common.js exact diff
- body copy drift 0
- affiliate/tracking drift 0

Staged QA PASS 없이 commit하지 않는다.

---

# 46. Production 완료 순서

이 Section은 Production이 실제 승인된 단계에서만 사용한다.
일반 Batch 작업 중이라는 이유만으로 main/Production까지 자동 진행하지 않는다.
현재 Batch의 승인 범위와 Section 42의 partial-Production 금지 원칙을 우선한다.

기본:

```text
Static QA
→ Browser QA where available
→ 사용자 Production 승인
→ approved files only stage
→ Staged QA
→ commit
→ push feature branch
→ main integration according to approved strategy
→ origin/main push
→ Vercel READY
→ deployed SHA 확인
→ public HTTP QA
→ canonical/hreflang/sitemap
→ links/assets/affiliate
→ Inventory update
→ DONE LOCKED
```

Production 이후 결함 발견 시 자동 rollback/hotfix하지 않는다.

정확한 defect를 보고한다.

---

# 47. Thai Master Inventory

기본 파일명:

`md/작업자료/Korea_Inside_Thai_Localization_Master_Inventory_YYYY-MM-DD.md`

각 English target:

- COMPLETE
- MISSING
- EXCLUDE

중 하나.

COMPLETE 조건:

- approved Thai copy
- exact implementation
- Production public
- HTTP 200
- self canonical
- reciprocal hreflang
- 핵심 internal links
- affiliate/tracking
- required QA PASS

전체 Thai 완료:

> **MISSING = 0**

Working copy나 branch commit만으로 COMPLETE 처리하지 않는다.

---

# 48. 저장 위치

Repository root:

`C:\Projects\Koreainside\`

Thai Standard:

`C:\Projects\Koreainside\Korea_Inside_Thai_Localization_Standard.md`

Repository-relative:

`Korea_Inside_Thai_Localization_Standard.md`

Approved Thai Review / Public Copy:

`C:\Projects\Koreainside\md\승인본\태국어\`

Working / Inventory / Extraction:

`C:\Projects\Koreainside\md\작업자료\`

Production folder:

`C:\Projects\Koreainside\th\`

---

# 49. 새 방 최초 Thai 작업 시작 보고

새 방에서 Thai 실제 작업을 처음 시작할 때만 짧게 보고한다.

1. Public Content Master Version
2. Navigation Standard Version
3. Language Localization Standard Version
4. Thai Localization Standard Version
5. 최신 Handover 날짜
6. Thai Inventory COMPLETE / MISSING / EXCLUDE
7. current Batch
8. source fingerprint 상태
9. Golden Sample
10. 보호해야 할 working-tree 변경
11. Production / feature branch 상태

같은 방에서 반복하지 않는다.

---

# 50. Thai 완료 전 최종 질문

페이지 또는 Batch를 COMPLETE로 보기 전에 확인한다.

- English facts preserved?
- numbers preserved?
- recommendation strength preserved?
- source coverage 100%?
- Thai natural?
- English syntax removed?
- gendered polite particles avoided?
- Thai spacing natural?
- place/brand spelling consistent?
- search intent natural?
- no keyword stuffing?
- FAQ visible/schema parity?
- internal Thai links only to valid siblings?
- affiliate/tracking preserved?
- canonical/hreflang correct?
- staged QA PASS?
- public QA PASS?
- current user scope를 초과하지 않았는가?
- 현재 Batch 밖 페이지/asset을 예방적으로 건드리지 않았는가?
- Final Integration이 명시적으로 활성화되기 전 전체언어 통합을 시작하지 않았는가?

하나라도 핵심 blocker가 남으면 COMPLETE가 아니다.

---

# 51. Version 1.0 재발 방지 항목

Version 1.0은 다음 문제를 막기 위해 만든다.

- 기준 없는 Thai 번역
- Batch마다 다른 문체
- 영어식 `best area` 직역
- `เหมาะสำหรับ / สะดวก / แนะนำ` 반복
- `ครับ/ค่ะ`가 섞인 성별 화자 문체
- Thai 단어 사이 영어식 공백
- Buddhist Era 임의 변환
- Thai numerals 임의 변환
- 브랜드명 임의 번역
- 지명 철자 흔들림
- 검색어 stuffing
- English source 누락
- FAQ visible/schema 불일치
- alt/ARIA 누락
- inline JS user-facing wording 누락
- 미래 `/th/` URL 선링크
- partial Production으로 broken Thai routing 생성
- common.js 조기 수정
- 승인되지 않은 Codex 번역
- 개별 anomaly 때문에 전체 작업을 반복
- staged QA 없이 commit
- Inventory를 Production보다 먼저 COMPLETE 처리

---

# 52. 이 Standard의 핵심 문장

> **Thai localization = English fact/judgment/structure preservation + natural Thai search/read/decision language.**

> **ความเป็นธรรมชาติของภาษาไทยต้องไม่แลกกับความถูกต้องของข้อเท็จจริง และความถูกต้องของข้อเท็จจริงก็ไม่ใช่เหตุผลให้เขียนภาษาไทยแบบแปลตรงตัว**

> **Pilot은 문체를 정하고, Inventory는 누락을 막고, Approved Copy는 문구를 잠그고, Codex는 승인 결과만 구현한다.**

---

# 53. Version 1.1 변경기록

- `SCOPE LOCK` 추가: Standard가 현재 사용자 작업범위를 확대하지 못하도록 고정
- 사용자 지정 페이지/asset 수를 절대 범위로 명시
- `다음 5개`를 실제 진행상태 기준 다음 미완료 5개로 해석하도록 규정
- 예방적 전체 58페이지 / 전체 infographic 재감사 금지
- 방·컴퓨터·세션 이동 후 현재 Batch 연속성 보호 규칙 추가
- Infographic / Visual Localization 기본 Batch를 5 assets로 명시
- 기존 infographic audit/source metadata 재사용 규칙 추가
- infographic 번호를 기억으로 임의 지정하는 행위 금지
- Final Integration Section 43~45를 일반 Batch에서 비활성화
- Final Integration 활성화 조건을 `58/58 + 사용자 명시 승인`으로 고정
- Production 절차가 일반 Batch scope를 자동 확대하지 않도록 보완
- 최종 QA checklist에 scope-overrun / premature-final-integration 검사를 추가

---

# 54. Version 1.0 변경기록

- Thai 전용 Specialized Standard 최초 제정
- `/th/`, `lang="th"`, hreflang `th`, label `ไทย`, code `TH` 고정
- `accommodation.html` 1-page Pilot을 초기 Golden Sample로 지정
- Pilot 이후 기본 5-page Batch 규칙 설정
- Thai 중립 editorial register 정의
- `ครับ/ค่ะ` 기본 배제
- Thai spacing / punctuation / Unicode 보호 규칙 추가
- Gregorian year / Arabic numeral 유지 규칙 추가
- Thai proper-name / brand 처리 기준 추가
- Thai search-intent 예시 추가
- visible FAQ / JSON-LD exact parity 기준 추가
- Markdown hard-break 예외의 HTML 확대 적용 금지
- 개별 anomaly SKIP/DEFER 후 끝까지 QA하는 운영 원칙 추가
- Staged QA를 commit 전 필수 Gate로 고정
- Thai 58/58 Whole-Language Final Integration을 기본 Production 전략으로 설정
- 기존 6개 언어 reciprocal `th` 및 최종 common.js integration 규칙 추가
