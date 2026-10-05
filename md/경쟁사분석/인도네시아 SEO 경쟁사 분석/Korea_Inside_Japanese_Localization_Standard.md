# Korea Inside — Japanese Localization Standard

**File:** `Korea_Inside_Japanese_Localization_Standard.md`  
**Status:** ACTIVE / SPECIALIZED STANDARD  
**Version:** 1.1  
**Effective date:** 2026-09-26  
**Supersedes:** Version 1.0  

**Higher standards**
1. `Korea_Inside_Public_Content_Master_Standard.md`
2. `Korea_Inside_Navigation_Hub_Architecture_Standard.md`
3. `Korea_Inside_Language_Localization_Standard.md`

> 이 문서는 Korea Inside 일본어판의 **현지화 품질과 제작 방식을 장기적으로 동일하게 유지하기 위한 일본어 전용 실행 표준**이다.
>
> 목적은 “오늘은 이렇게 번역하고 내일은 다르게 번역하는 것”을 막는 것이다.
>
> 일본어 현지화는 영어를 일본어로 바꾸는 작업이 아니다.
>
> **현재 English Production의 사실·수치·추천 판단·페이지 역할·HTML 구조를 그대로 보존하면서, 일본인 여행자가 실제로 검색하고 읽고 판단하는 방식으로 자연스럽게 현지화한다.**
>
> 이 문서는 상위 3개 Standard를 대체하지 않는다.  
> 상위 기준과 충돌하면 항상 상위 기준을 따른다.

---

# 0. 이 문서를 사용하는 방법

Japanese 신규 현지화, 기존 일본어 페이지 Humanization, 일본어 SEO 문구 수정, 일본어 검색의도 감사에는 이 문서를 적용한다.

작업 시작 순서:

1. `Korea_Inside_Public_Content_Master_Standard.md`
2. `Korea_Inside_Navigation_Hub_Architecture_Standard.md`
3. `Korea_Inside_Language_Localization_Standard.md`
4. `Korea_Inside_Japanese_Localization_Standard.md`
5. 최신 `Korea_Inside_Room_Handover_*.md`
6. 해당 페이지 Research Master / Approved Public Copy / CONTENT LOCKED 자료
7. 현재 English Production HTML
8. `Korea_Inside_Japanese_Localization_Master_Inventory_*.md`

과거 작업을 기억이나 추측으로 복원하지 않는다.

새로운 대화방에서 일본어 실제 작업을 처음 시작할 때 위 기준을 전체 확인한다.

같은 대화방에서 이미 전체 확인한 Standard가 변경되지 않았다면 다음 Batch마다 다시 전체 읽지 않는다. 다음 경우에만 다시 확인한다.

- Standard Version 또는 내용 변경
- 사용자가 다시 읽으라고 명시
- 더 최신 Handover 생성
- 새 Specialized Standard 적용
- 현재 작업의 Family / Research / Approved 자료가 변경됨

일본어 페이지를 새로 만들 때마다 “전에 했으니 같은 방식일 것”이라고 가정하지 않는다. 다만 **이미 확인한 공통 Standard 자체를 불필요하게 반복해서 읽지는 않는다.**

Standard 자체의 검토·수정·최적화는 일본어 Batch 실행으로 간주하지 않는다.

> **기준 MD를 확인하고 → 현재 English 구조를 확인하고 → 그 구조 안에서 일본어를 작성한다.**

---

# 1. 적용 우선순위

충돌 시 우선순위:

1. 현재 사용자의 명시적 지시
2. `Korea_Inside_Public_Content_Master_Standard.md`
3. `Korea_Inside_Navigation_Hub_Architecture_Standard.md`
4. `Korea_Inside_Language_Localization_Standard.md`
5. `Korea_Inside_Japanese_Localization_Standard.md`
6. 최신 `Korea_Inside_Room_Handover_*.md`
7. 페이지별 Research Master / Approved Public Copy / CONTENT LOCKED
8. 현재 English Production HTML
9. Japanese Localization Master Inventory

이 문서가 허용하는 일본어 표현 변경도 상위 문서가 잠근 사실·추천 판단·구조를 변경할 수 없다.

---

# 2. 일본어판의 목표

일본어판의 목표:

> **日本人旅行者が実際に読む韓国旅行ガイド**

즉 다음 세 가지를 동시에 만족해야 한다.

1. **정확성**
   - English의 사실·수치·날짜·가격·조건·추천 판단 보존
2. **일본어 자연스러움**
   - 영어 어순과 번역투를 제거
   - 일본 여행 콘텐츠에서 자연스럽게 읽히는 문장
3. **일본 여행자의 검색·판단 방식**
   - 일본인이 실제로 쓰는 검색어·공식 용어·질문형 표현 반영
   - 단, 검색어를 넣기 위해 페이지 역할이나 추천 판단을 바꾸지 않음

---

# 3. 일본어판도 Korea Inside다

일본어판은 별도의 일본 여행 사이트를 새로 만드는 작업이 아니다.

English의 Korea Inside 정체성을 유지한다.

고정:

- Humanization 우선
- 여행자 문제 해결
- 이동 마찰
- 짐
- 환승
- 출구
- 마지막 도보
- 객실·침대·인원
- 운영·예약 조건
- 누구에게 맞고 안 맞는지
- 각 선택지가 존재하는 서로 다른 이유

일본어라는 이유로 다음을 하지 않는다.

- 모든 선택지를 부드럽게 긍정
- 단점을 완곡하게 지워버림
- 추천 강도를 높임
- 호텔을 “人気・おすすめ・高級・コスパ” 표현으로 단순화
- 원문에 없는 일본인 전용 혜택·시설·서비스를 추가
- “日本人に人気”를 근거 없이 추가
- 직접 방문·숙박한 것처럼 개인 경험을 추가

기본 작성 기반:

> **확인된 사실 + 현지 맥락 + 독립적 편집 판단**

---

# 4. 역할 분리 — 절대 고정

## 4.1 ChatGPT

ChatGPT만 판단한다.

- 일본어 번역
- 일본어 현지화
- 일본어 Humanization
- 일본어 검색의도 표현
- 일본어 title / meta / H1 / H2 / H3
- body
- CTA
- FAQ
- breadcrumb
- alt
- caption
- ARIA
- JSON-LD 사용자 노출 문구
- 일본어 문법·어순·문장부호
- 일본어 여행자 관점의 자연스러운 표현
- 공식 일본어 용어를 어느 위치에서 어떻게 병기할지

## 4.2 사용자

사용자가 최종 결정한다.

- Batch 승인
- 예외 승인
- 검색의도 방향 변경
- 일본어 용어 변경
- 추천 판단 변경
- 기존 CONTENT LOCKED 재개방 여부

사용자가 `승인`이라고 하면 현재 Batch 전체 승인으로 처리한다.

## 4.3 Codex

Codex 역할:

- Approved Japanese MD exact implementation
- 사용자 승인 범위 안의 Technical Source Extraction
- HTML/CSS 기술 적용
- canonical
- hreflang
- sitemap
- 승인된 내부링크 적용
- QA
- Git
- Production

Codex는 일본어를 판단하지 않는다.

## 4.4 Codex 금지

Codex는 임의로 다음을 하지 않는다.

- 일본어 번역
- 일본어 현지화
- 문법 개선
- 자연스럽게 다시 쓰기
- 일본어 검색어 추가
- 문장 추가/삭제
- 문장 분리/결합
- 요약/확장
- Humanization
- 추천 판단 변경
- 호텔/지역 순위 변경
- 공식 용어 대체
- 사실·수치 수정
- 사용자 승인 또는 승인된 Codex 지시문 없이 Source Extraction 수행
- 추출 결과를 일본어 문구 판단의 근거로 독자 해석

## 4.5 Technical Source Extraction — 조건부 허용

대량·장문·복잡한 Japanese Batch에서 누락 방지를 위해, 사용자가 승인했거나 사용자 승인 범위 안의 Codex 지시문에 명시된 경우 Codex는 기계적 Source Extraction을 수행할 수 있다.

허용:

- 사용자 노출 문자열 추출
- source target / selector / node identity 기록
- ITEM numbering
- 구조 count
- source fingerprint / SHA 기록
- Common UI reuse position 식별

금지:

- 일본어 번역·현지화
- 문장 자연스러움 판단
- Humanization
- 누락 문구 추정
- 문장 병합·분리 판단
- 검색어 추가
- 사실·추천 판단 변경

추출 결과는 기술적 작업재료일 뿐이다. ChatGPT가 실제 English source와 대조하고 일본어 문구를 직접 판단한다.

---

# 5. English Production이 기준 원문

일본어 현지화의 기준은 **현재 English Production 전체 페이지**다.

기본:

```text
Current English Production HTML
→ 구조/노드 전수 확인
→ 일본어 현지화
```

기억에 남아 있는 English 문장이나 과거 Approved MD만으로 새 일본어 페이지를 만들지 않는다.

English Production과 저장소 HTML이 다를 가능성이 있으면 현재 승인 상태를 확인한다.

이미 CONTENT LOCKED된 English 사실·판단은 일본어 현지화 과정에서 다시 설계하지 않는다.

---

# 6. 현지화 전에 반드시 English 구조를 먼저 잠근다

이 절은 일본어 작업의 핵심 Quality Gate다.

일본어 문장을 쓰기 전에 현재 English HTML에서 **사용자 노출 구조 전체**를 먼저 확인한다.

최소 확인 대상:

- `<title>`
- meta description
- OG title / description
- Twitter title / description
- H1
- H2
- H3
- 일반 body paragraph
- list item
- table caption
- `<th>`
- `<td>`
- `<dt>`
- `<dd>`
- `<summary>`
- FAQ answer
- CTA
- button
- visible link text
- breadcrumb
- label
- review date
- official source link text
- official source description
- figure caption
- image alt
- ARIA label / description
- 사용자 노출 JSON-LD 문자열
- 상태 문구
- 앱스토어/외부서비스 링크명
- 기타 화면에 실제로 보이거나 접근성 트리에 노출되는 언어 의존 문자열

### 6.1 금지

다음 방식으로 먼저 번역하지 않는다.

```text
영어 페이지를 대충 읽음
→ 섹션별로 요약
→ 일본어로 다시 작성
```

이 방식은 금지한다.

### 6.2 고정 방식

```text
English 구조 확정
→ 사용자 노출 node inventory 확정
→ 누락 0 확인
→ 같은 source order를 유지해 일본어 작성
```

---

# 7. Source Coverage 100% 원칙

일본어 Approved Copy는 English 사용자 노출 문구와 **coverage 100%**여야 한다.

현지화 완료 조건:

- source target 누락: 0
- source target 중 빈 일본어: 0
- 승인되지 않은 신규 일본어 target: 0
- 원문에 없는 HTML 요소 추가: 0

### 7.1 Inline / composite 문장

하나의 문장이 여러 inline element로 분리되어 있어도 화면에서 하나의 문장으로 읽히는 경우:

- 자연스러운 일본어 문장으로 현지화 가능
- 단, HTML 구조를 바꾸지 않음
- source node 전체가 매핑되어야 함

### 7.2 구조에 없는 문구 금지

English HTML에 eyebrow가 없으면 일본어 eyebrow를 새로 만들지 않는다.

English HTML에 H3가 없으면 번역 편의를 위해 H3를 새로 만들지 않는다.

English HTML에 설명문이 없으면 검색어 삽입을 위해 설명문을 추가하지 않는다.

### 7.3 Source count 불일치 시

Approved MD와 현재 English 구조 count가 맞지 않으면 Codex는 STOP한다.

ChatGPT가 현재 English를 다시 확인하고 승인 문구를 보완한다.

Codex가 임의로 빈칸을 번역하지 않는다.

---

# 8. Source Fingerprint / Drift Control

가능하면 각 Batch MD에 페이지별 English source fingerprint를 기록한다.

우선:

- Git blob SHA
- 또는 SHA-256
- 또는 현재 Production과 저장소 source가 동일함을 확인한 기준

목적:

- 현지화 승인 후 English가 변경됐는지 확인
- 다른 버전의 원문에 승인 문구를 적용하는 사고 방지

### 8.1 Source가 바뀐 경우

Approved 이후 English source가 변경됐다면:

- 변경 부분이 언어 비의존 기술 변경인지 확인
- 사용자 노출 문구/구조가 바뀌었으면 STOP
- 바뀐 부분만 ChatGPT가 재현지화
- 기존 승인 부분은 임의 재번역하지 않음

---

# 9. 반드시 보존할 것

English 원문에서 다음은 잠근다.

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
- 역
- 출구
- 버스 번호
- 노선
- 거리
- 가격
- 날짜
- 운영시간
- affiliate URL
- CID
- subid
- campaign
- tracking
- CSS
- JS
- schema 구조
- event ID
- 자동 연도 로직
- 상태 로직
- 기능 코드

원문에 없는 사실을 일본어판에 추가하지 않는다.

---

# 10. 일본어로 현지화할 것

현지화 대상:

- title
- meta description
- OG / Twitter의 사용자 노출 title / description
- H1 / H2 / H3
- body copy
- CTA
- button
- FAQ
- breadcrumb
- visible source names/descriptions
- alt
- caption
- ARIA 설명
- 사용자 노출 JSON-LD
- table headings
- dt/dd labels
- UI helper text
- 사용자가 읽는 상태 문구

이미 승인된 Common UI는 페이지별로 새로 번역하지 않는다.

---

# 11. 일본어 문체 — 기본 Register

기본 톤:

- 명확함
- 실용적
- 과장하지 않음
- 지나치게 딱딱하지 않음
- 지나치게 친근하지 않음
- 여행자가 바로 행동할 수 있음

기본 종결:

- `〜です`
- `〜ます`
- 필요 시 짧은 명사형 heading

같은 단락 안에서 `です・ます`와 과도한 반말/광고체를 섞지 않는다.

### 11.1 목표

좋은 일본어는 영어 원문을 일본어 단어로 바꾼 문장이 아니다.

예:

English:
`The best option depends on the exact hotel, arrival time, luggage and group size.`

권장 방향:
`どの移動手段が合うかは、ホテルの場所、到着時刻、荷物の量、人数で変わります。`

직역식:
`最適な選択肢は、正確なホテル、到着時刻、荷物、グループサイズに依存します。`

후자는 사용하지 않는다.

---

# 12. 일본어 Humanization 기준

Humanization은 감성화가 아니다.

다음이 읽혀야 한다.

- 누구에게 맞는가
- 누구에게 맞지 않는가
- 실제 이동이 어디서 힘들어지는가
- 짐이 있으면 무엇이 달라지는가
- 아이/가족이면 무엇이 달라지는가
- 야간이면 무엇이 달라지는가
- 출구/환승/마지막 도보가 어떻게 달라지는가
- 예약·운영 조건이 선택에 어떤 영향을 주는가

단, English에 없는 새 판단을 일본어판에서 만들지 않는다.

### 12.1 일본어에서 특히 피할 AI/직역 패턴

과도한 반복 금지:

- `〜に最適です`
- `〜にぴったりです`
- `おすすめです`
- `良い選択肢です`
- `便利です`
- `実用的です`
- `合理的です`
- `〜に向いています`
- `〜の場合に意味があります`
- `判断してください`
- `メリットは〜です`
- `デメリットは〜です`
- `〜ではなく、〜です` 반복
- `条件付き`
- `かなり向いている`
- 동일한 `おすすめ → 交通 → 注意点` 구조 반복

위 표현을 한 번 쓰는 것 자체가 오류는 아니다.

문제는 **같은 페이지에서 기계적으로 반복돼 번역 템플릿처럼 읽히는 경우**다.

### 12.2 자연스러운 대체 방향

영어식:
`合理的な空港鉄道の選択になりやすいです。`

일본어 방향:
`ソウル駅でKTXに乗り継ぐなら、直通列車を選ぶと動きやすくなります。`

영어식:
`実用的な流れは：`

일본어 방향:
`回りやすい順番は、次のとおりです。`

영어식:
`券売機の前で判断するときに必要な違い`

일본어 방향:
`券売機の前で迷わないよう、2種類の列車の違いをまとめます。`

---

# 13. 일본어 문장부호 / Typography

기본:

- 일본어 문장 종결은 `。`
- 일본어 열거는 `、`
- 괄호는 기본적으로 `（ ）`
- 필요 시 `「 」`
- 제품/브랜드 원형은 그대로 유지 가능

### 13.1 금지

Production Japanese에 다음이 남으면 QA FAIL:

- 일본어 문장 끝의 ASCII `.`
- `.</p>`
- `<strong>日本語</strong>.`
- raw Markdown `**日本語**`
- 영어식 `: `가 일본어 문장을 부자연스럽게 분절
- 링크 앞뒤 불필요한 공백
- 조사와 링크가 분리되어 어색한 문장
- English source의 문장 분리가 그대로 들어가 일본어 문장이 중간에서 끊김

### 13.2 링크가 문장 안에 들어갈 때

권장:

`詳しくは、<a>空港バスガイド</a>で確認できます。`

금지:

`詳しくは <a>空港バスガイド</a> では、確認できます。`

권장:

`<a>仁川空港 到着ロビーガイド</a>でまとめています。`

금지:

`<a>仁川空港 到着ロビーガイド</a>.`

---

# 14. `<strong>` / inline 요소 처리

Japanese 조사·종결어가 `<strong>` 때문에 끊기지 않아야 한다.

권장:

`<strong>エレベーターのある4番出口</strong>を案内しています。`

금지:

`<strong>エレベーターがある4番出口</strong>.`

권장:

`降車後に<strong>徒歩約10分</strong>です。`

금지:

`<strong>降車後に徒歩約10分</strong>.`

Inline markup는 구조를 보존하면서 일본어 조사와 어미가 자연스럽게 연결되도록 한다.

---

# 15. 지명 표기 기준

확립된 기본 표기:

- Dongdaemun → `東大門`
- Hongdae → `弘大`
- Myeongdong → `明洞`
- Seongsu → `聖水`
- Insadong → `仁寺洞`
- Gangnam → `江南`
- Jamsil → `蚕室`
- Gongdeok → `孔徳`
- Mapo → `麻浦`
- Itaewon → `梨泰院`
- Seoul Station → `ソウル駅`
- Incheon Airport → `仁川空港`

### 15.1 읽는 법 병기

일본 검색자에게 읽는 법이 실제 도움이 되는 경우 첫 등장 또는 검색-facing title/H1에서 병기할 수 있다.

예:

`東大門（トンデムン）`

하지만 매번 반복하지 않는다.

### 15.2 한자/가타카나 병기 규칙

다음 조건에서만 병기한다.

- 일본 검색자가 한자와 가타카나를 함께 쓰는 경향이 명확함
- 첫 방문자가 읽기 어려운 지명
- title/H1에서 검색 이해도가 개선됨

병기 때문에 title이 과도하게 길어지면 본문 첫 등장으로 이동한다.

---

# 16. 브랜드 / 서비스명

브랜드·제품·서비스명은 공식 원형을 기본으로 한다.

예:

- AREX
- T-money
- WOWPASS
- Airport Limousine
- K Airport Limousine
- International Taxi
- Korea Inside
- Naver Map
- KakaoMap
- Google Maps

일본어로 의미가 필요한 경우 설명을 붙일 수 있지만 브랜드 자체를 임의 번역하지 않는다.

---

# 17. 공식 일본어 용어 우선

공식기관이 일본어 명칭을 제공하는 경우 검색·설명에서 그 명칭을 우선 검토한다.

확립된 예:

- AREX `Express` → `直通列車`
- AREX `All-Stop` → 최초 검색-facing 표현 `一般列車（各駅停車）`
- e-Arrival Card → 최초 설명 `電子入国申告書（e-Arrival Card）`
- 이후 문맥에서는 `電子入国申告書` 등 자연스러운 축약 가능

### 17.1 기계적 전체 치환 금지

공식 용어를 확인했다고 페이지 전체 표현을 기계적으로 바꾸지 않는다.

예:

`一般列車（各駅停車）`를 title/H1/첫 설명에서 확립한 뒤,
본문에서는 문맥상 `各駅停車`가 더 자연스러우면 사용할 수 있다.

---

# 18. 검색의도 현지화 원칙

Japanese SEO는 English page role을 바꾸는 작업이 아니다.

보존:

- 대표 검색 문제
- 페이지 역할
- 추천 결론
- URL
- 사실

현지화:

- 일본인이 실제로 쓰는 query wording
- 일본어 질문형 표현
- 공식 일본어 용어
- 일본 여행 콘텐츠에서 자연스러운 명사

### 18.1 검색-facing 영역

특히 확인:

- title
- meta description
- H1
- 첫 lead
- FAQ question
- 주요 H2

### 18.2 일본어 검색 표현 예

페이지 역할에 맞을 때 고려:

Travel:
- `観光`
- `ガイド`
- `見どころ`
- `買い物`
- `夜`
- `何時`
- `何時間`

Stay:
- `おすすめホテル`
- `宿泊エリア`
- `どこに泊まる`
- `アクセス`
- `駅`
- `空港バス`

Airport / Transport:
- `行き方`
- `アクセス`
- `料金`
- `所要時間`
- `始発`
- `終電`
- `乗り方`

First trip:
- `初めての韓国旅行`
- `韓国旅行ガイド`
- `準備`

### 18.3 키워드 삽입 금지

검색어를 넣기 위해:

- 원문에 없는 섹션 추가
- 추천 판단 변경
- 사실 추가
- 같은 단어 반복
- 어색한 title
- 일본어 문장 품질 저하

를 하지 않는다.

### 18.4 검색의도 변경은 별도 승인

Japanese SERP를 보고 English page role 자체를 바꿀 필요가 있다고 판단되면,
현지화로 처리하지 않는다.

사용자에게:

- 현재 English role
- Japanese search intent
- 충돌 지점
- 제안

을 보고하고 별도 승인을 받는다.

---

# 19. Japan-specific 사실 추가 금지

일본 경쟁 페이지에서 자주 보인다는 이유만으로 다음을 자동 추가하지 않는다.

예:

- 日本語対応スタッフ
- 日本語OK
- バスタブ
- ウォシュレット
- 変圧器
- 電圧
- 変換プラグ
- 日本人に人気
- 日本人向けサービス
- 日本語メニュー

이런 정보가 일본 여행자에게 중요해 보여도:

1. 현재 English fact layer에 존재하는지
2. Research Master 또는 공식 출처에서 확인되는지
3. 페이지 역할에 필요한지

를 확인한다.

필요하면 **별도 사실 조사 + 사용자 승인** 후 English/다국어 전체에 반영할 문제이지,
일본어판에만 임의 추가하지 않는다.

---

# 20. Travel Guide 일본어 기준

Travel Guide는 관광지 나열로 만들지 않는다.

일본어에서도 다음 판단을 유지한다.

- 왜 가는가
- 어느 시간대가 맞는가
- 누구에게 맞는가
- 누구에게 안 맞는가
- 얼마나 머무는가
- 어느 역/지역부터 시작하는가
- 무엇을 굳이 하지 않아도 되는가

### 20.1 일본어 제목

`[지역명] ガイド`만으로 검색의도가 약하면
페이지 역할에 맞는 `観光` 등을 자연스럽게 고려한다.

예:

`東大門（トンデムン）観光ガイド`

단, 모든 Area Guide에 `観光`을 기계적으로 붙이지 않는다.

---

# 21. Stay / Hotel 일본어 기준

Accommodation Decision Layer를 그대로 보존한다.

일본어에서도 핵심은:

> **“人気ホテルか”가 아니라 “この旅行者に実際に使いやすいか”**

다음 정보가 English에 있다면 축소하지 않는다.

- 객실 면적
- 침대
- 정원
- 가족/성인 그룹
- 짐
- 역 출구
- 엘리베이터
- 계단
- 마지막 도보
- 공항버스 정류장
- 체크인
- 짐보관
- 소음
- 객실 방향
- 운영 조건

### 21.1 금지

다음 문구로 단순화하지 않는다.

- `立地が良い`
- `コスパが良い`
- `おすすめ`
- `人気`
- `便利`
- `家族に最適`

실제 판단 근거가 문장 안에 있어야 한다.

---

# 22. Airport / Transport 일본어 기준

검색자가 바로 필요한 것은:

- 어디서 타는가
- 어떤 표를 사는가
- 얼마인가
- 얼마나 걸리는가
- 어느 역에 서는가
- 짐이 있으면 어떤가
- 막차를 놓치면 어떻게 하는가
- 도착 후 호텔까지 무엇이 남는가

일본어 문구도 이 실행 순서를 명확히 한다.

### 22.1 “가장 빠름”과 “가장 편함”을 구분

English 판단을 보존한다.

예:

`直通列車が最速でも、ホテルまで最も楽とは限りません。`

시간 단축과 전체 이동 편의가 같은 개념처럼 번역되지 않게 한다.

### 22.2 공항 이동 검색어

페이지 역할에 맞으면:

- `仁川空港からソウル市内`
- `行き方`
- `アクセス`
- `料金`
- `所要時間`

을 search-facing 영역에서 자연스럽게 검토한다.

---

# 23. Service / Payment / App 일본어 기준

제품 기능 설명보다 먼저:

- 누구에게 필요한가
- 누구에게 불필요한가
- 충전/결제/환불 조건
- 해외 발행 카드 사용 여부
- 실제 실패 가능성
- 대안

을 분명히 한다.

제품명은 원형 유지.

표·비교라벨은 일본인 여행자가 바로 이해할 수 있는 짧은 일본어로 작성한다.

예:

- `Main navigation` → `メインナビ`
- `Planning, saved places and international reviews` → 문맥에 맞는 짧은 일본어

단, `<dt>` 같은 작은 label도 사용자 노출 source target이므로 누락하면 안 된다.

---

# 24. Common UI Golden Sample

일본어 Common UI는 이미 승인된 Golden Sample을 재사용한다.

페이지마다 새로 번역하지 않는다.

보호:

- common header
- navigation
- footer
- `common.js`
- mobile hamburger
- 공통 `style.css`

사용자 명시 승인 없이 수정하지 않는다.

### 24.1 Common UI가 오래된 fallback을 갖고 있는 경우

새 Japanese sibling이 Production되면서 Common UI 링크가 English fallback에 남을 수 있다.

이 경우:

- Audit에서 문제로 기록 가능
- 페이지 본문 수정과 섞지 않음
- common navigation/footer 변경은 별도 사용자 승인 필요

---

# 25. Internal Link 일본어 규칙

실제 Production Japanese sibling만 `/ja/`로 연결한다.

```text
Japanese sibling Production COMPLETE
→ Japanese URL

Japanese sibling MISSING
→ English fallback
```

금지:

- 미래 `/ja/` URL 추측
- 작업본으로 링크
- 404 생성
- 아직 미배포 sibling으로 미리 전환

### 25.1 sibling이 나중에 공개된 경우

기존 페이지의 English fallback은 별도 링크 동기화 QA에서 일본어 sibling으로 갱신할 수 있다.

본문 링크:
- 범위 승인 후 갱신 가능

common navigation / footer:
- 보호 영역
- 사용자 명시 승인 필요

---

# 26. Canonical / hreflang

Japanese 기본:

- folder: `/ja/`
- `lang="ja"`
- self canonical
- `x-default = English`

hreflang은 고정 언어 목록을 하드코딩하지 않는다.

원칙:

- English Production sibling은 `en`
- Japanese Production sibling은 `ja`
- Spanish 등 다른 언어 sibling이 실제 Production COMPLETE 상태이면 해당 언어 코드 포함
- 향후 독일어·중국어·기타 언어도 실제 Production sibling이 존재할 때만 포함
- 모든 실제 Production sibling끼리 reciprocal
- 미공개·작업본·미래 언어 URL은 hreflang에 미리 넣지 않음

즉, **현재 실제 Production sibling set 전체 + x-default**를 기준으로 관리한다.

---

# 27. FAQ / JSON-LD

English source에 schema가 있으면 구조를 보존한다.

English source에 schema가 없으면 일본어판에 새 schema를 만들지 않는다.

### 27.1 사용자 노출 FAQ와 schema

일본어 FAQ가 schema에도 존재하면:

- 질문 수 일치
- 순서 일치
- 의미 일치
- 가능하면 승인 일본어 wording exact parity

특히 같은 FAQ를 visible과 schema에 중복 관리하는 구조라면
승인 일본어는 **동일 문구**를 기본으로 한다.

### 27.2 QA FAIL

다음은 FAIL:

- visible FAQ 10 / schema 9
- visible 일본어 / schema English
- 동일 질문의 일본어 의미가 달라짐
- schema에 ASCII `.`가 남고 visible은 `。`
- English source에 없는 FAQPage를 일본어판에 임의 추가

---

# 28. 이미지 / alt / caption / ARIA

이미지 파일과 구조는 보존한다.

현지화:

- alt
- figcaption
- ARIA description / label
- user-facing title

### 28.1 이미지 filename과 alt가 이상해 보여도

English Production의 이미지 filename과 visible meaning이 서로 어색해 보여도
현지화 과정에서 임의로 이미지 교체하거나 사실을 수정하지 않는다.

명확한 사실 오류로 보이면 별도 보고한다.

### 28.2 alt는 번역문이 아니다

alt는 일본어로 자연스럽게 이미지 내용을 설명한다.

하지만 원문에 없는 사실을 추가하지 않는다.

---

# 29. 일본어 Batch 기본 크기

기본 Batch:

> **5페이지**

사용자가 별도 지시하지 않는 한 5페이지를 기본으로 한다.

이유:

- 너무 큰 Batch에서 현지화 누락과 모델 불안정 방지
- 사용자 승인 횟수는 과도하게 늘리지 않음
- 5페이지 단위로 구조/QA를 충분히 검증 가능

예외:

- 초장문 페이지가 여러 개 포함된 경우 더 작게
- 매우 짧고 동일 Family인 경우 사용자 승인 후 확대 가능

---

# 30. Batch 구성 원칙

가능하면 같은 Family를 묶는다.

예:

Airport / Transport:
- Airport Bus
- Maps
- T-money
- WOWPASS
- T-money vs WOWPASS

Stay:
- 같은 지역 또는 같은 Decision Layer Family

Travel Area:
- 지역 가이드 Family

하지만 Family를 맞추기 위해 구조가 너무 복잡한 페이지를 억지로 한 Batch에 넣지 않는다.

---

# 31. Batch Localization MD 필수 구성

Batch MD는 Codex가 추측 없이 구현할 수 있어야 한다.

페이지별 최소:

```md
# PAGE N — filename.html

## SOURCE
- English file
- source fingerprint
- H1 count
- H2 count
- H3 count
- FAQ count
- JSON-LD count
- image count
- figcaption count
- 기타 중요 구조 count

## SEO
- title
- meta
- OG/Twitter if source has user-facing copy

## H1

## SOURCE ORDER
- H2/H3/body/table/list/dt/dd...
- source order 유지

## FAQ
- visible FAQ
- schema user-facing FAQ

## ALT / CAPTION / ARIA

## PROTECTED
- facts
- numbers
- recommendations
- affiliate/tracking
```

### 31.1 전체 HTML 복제는 불필요

하지만 **누락 없이 exact location을 찾을 수 있어야 한다.**

긴 페이지에서 source order와 item numbering이 안정적이면 번호를 유지할 수 있다.

---

# 32. 승인 상태

기본 상태:

### 1. LOCALIZATION REVIEW
ChatGPT 작성 완료, 사용자 미승인

### 2. APPROVED PUBLIC COPY — CONTENT LOCKED
사용자 승인 완료

### 3. IMPLEMENTED
Codex exact implementation 완료, Production 전/후 QA 중

### 4. COMPLETE
Production 공개 및 Inventory QA 완료

`LOCALIZATION REVIEW`를 Production 기준으로 사용하지 않는다.

---

# 33. 사용자 승인 후 변경 금지

사용자 승인 후:

- 재번역 0
- 문법 개선 0
- 자연스럽게 수정 0
- 요약 0
- 확장 0
- 추천 변경 0

승인 후 문제를 발견하면:

- 임의 수정하지 않음
- STOP
- 정확한 문제 위치 보고
- ChatGPT가 Correction Supplement 또는 새 FINAL 작성
- 사용자 승인
- Codex 재개

---

# 34. FINAL 파일 관리

한 Batch에서 여러 correction이 생기더라도 구현 Source of Truth는 최종적으로 **1개**로 정리한다.

예:

```text
Localized
Approved
Corrected
FINAL v2
FINAL v3
```

파일이 누적되더라도 Codex에는 최종 Source of Truth 하나만 지정한다.

### 34.1 작은 correction

승인본이 이미 저장소에 있고 1~3개 문구만 빠진 경우:

- 전체 파일 재다운로드를 강요하지 않아도 됨
- 사용자 승인 exact patch를 기존 승인 MD에 추가 가능
- 최종 파일명/version만 명확히 갱신
- 다른 문구 변경 0 확인

### 34.2 최종 Source 명시

Codex 지시에는 반드시:

`이 파일 1개만 Source of Truth`

를 명시한다.

---

# 35. Codex STOP 조건

Codex는 다음 경우 구현하지 않고 STOP해야 한다.

- English source fingerprint 불일치
- Approved target count와 source count 불일치
- source에 문구가 있는데 일본어 승인값 없음
- Approved에 문구가 있는데 source에 대응 요소 없음
- FAQ/schema count 불일치
- 구조를 바꿔야만 Approved 문구를 넣을 수 있음
- affiliate/tracking 충돌
- 사용자 working-tree 변경과 범위 충돌
- common UI 수정이 필요하지만 승인 없음
- 일본어 sibling 존재 여부가 불명확
- 승인 파일 지정 경로에 없음
- 여러 FINAL 중 무엇을 써야 하는지 불명확

STOP은 실패가 아니다.

> **승인되지 않은 판단을 하지 않는 것이 정상 동작이다.**

---

# 36. 현지화 QA — ChatGPT 단계

사용자 승인 전 ChatGPT가 확인한다.

## Structure

- English H1 count
- H2 count
- H3 count
- table
- dt/dd
- FAQ
- JSON-LD
- image
- caption
- alt
- ARIA
- source description
- CTA

## Coverage

- source target = approved target
- missing 0
- duplicate 0
- empty Japanese 0
- source에 없는 추가 target 0

## Content

- 사실 mismatch 0
- 수치 mismatch 0
- 날짜 mismatch 0
- 추천 판단 mismatch 0
- 운영조건 mismatch 0
- 장소/호텔 순서 mismatch 0

## Japanese quality

- 주요 직역투 0
- 일본어 문장 끝 ASCII `.` 0
- raw Markdown 0
- link spacing 오류 0
- `<strong>` 뒤 영어식 종결 0
- 일본어 문장 중간 분절 0
- 한국어 혼입 0
- 승인되지 않은 English residual 0

## Search

- title 일본 검색의도 자연스러움
- H1 일본 검색의도 자연스러움
- 공식 일본어 용어 확인
- 키워드 반복 없음
- English page role 보존

---

# 37. Codex 구현 QA

Codex는 구현 후 다음을 확인한다.

- 승인 일본어 exact implementation
- page-specific visible English 0
- common UI는 Golden Sample
- structure mismatch 0
- class/id/data mismatch 0
- image/srcset mismatch 0
- fact/number/date mismatch 0
- affiliate/tracking mismatch 0
- visible FAQ/schema parity
- lang `ja`
- self canonical
- reciprocal hreflang
- sitemap
- internal links
- public asset HTTP
- public page HTTP
- current Git blob / Production 반영 일치

---

# 38. Production 완료 순서

사용자가 Production 실행을 명시적으로 승인한 경우:

```text
정적 QA
→ 범위 파일만 stage
→ staged 범위 검증
→ commit
→ push
→ ahead/behind
→ Vercel Production READY
→ 공개 URL HTTP 200
→ 실제 Japanese HTML QA
→ link / asset / canonical / hreflang / affiliate QA
→ Inventory COMPLETE 갱신
→ Inventory만 별도 scoped stage
→ commit
→ push
→ final Vercel READY
→ HEAD / origin/main 확인
```

금지:

- `git add .`
- `git add -A`
- 범위 외 stage

---

# 39. Japanese Master Inventory

일본어 전체 완료 여부는 기억으로 판단하지 않는다.

`Korea_Inside_Japanese_Localization_Master_Inventory_*.md` 사용.

상태:

- COMPLETE
- MISSING
- EXCLUDE

전체 완료:

> **MISSING = 0**

작업본 존재는 COMPLETE가 아니다.

English exact copy가 `/ja/`에 있어도 Production 미완료면 MISSING이다.

---

# 40. COMPLETE / CONTENT LOCKED 감사

이미 COMPLETE된 일본어 페이지를 “더 자연스럽게 쓸 수 있다”는 이유만으로 전면 재작성하지 않는다.

감사 순서:

```text
Production Japanese Audit
→ PASS / FIX NEEDED / REVIEW
→ 문제 문장만 제안
→ 사용자 승인
→ Japanese Humanization/Search Fix Approved Copy
→ Codex exact implementation
→ QA
→ Production
```

### 40.1 재개방 사유

- 명확한 일본어 문장 파손
- 사실 오류
- 사용자 지시
- 승인 후 새로 생긴 AI-template
- 공식 일본어 용어/검색의도에서 중대한 문제
- visible FAQ/schema 불일치
- Production Japanese sibling link가 오래된 English fallback에 남아 사용자 흐름이 깨짐

### 40.2 재개방하지 않을 사유

- 다른 표현도 가능함
- 문장이 조금 더 예쁘게 바뀔 수 있음
- 특정 단어 1회 사용
- 새 방에서 과거 작업을 직접 기억하지 못함

---

# 41. 일본어 Production Regression Guard

다음 패턴은 발견 즉시 FIX NEEDED다.

## 41.1 ASCII period

금지:

`まずは<strong>一般向けショッピング側</strong>.`

권장:

`まずは<strong>一般向けショッピング側</strong>から始めましょう。`

## 41.2 문장 분절

금지:

`<strong>黄色いテント市場</strong>.`
다음 문단:
`と呼ばれます。`

권장:

`<strong>黄色いテント市場</strong>と呼ばれます。`

## 41.3 raw Markdown

금지:

`**何がしたい？**`

HTML에서는 승인된 markup로 구현한다.

## 41.4 `<strong>` + 조사 누락

금지:

`<strong>エレベーターがある4番出口</strong>.`

권장:

`<strong>エレベーターのある4番出口</strong>を案内しています。`

## 41.5 링크 문법

금지:

`詳しくは <a>空港バスガイド</a> では...`

권장:

`詳しくは、<a>空港バスガイド</a>で...`

## 41.6 공식 용어

초기 검색-facing 용어:

- `一般列車（各駅停車）`
- `電子入国申告書（e-Arrival Card）`

이후 본문은 자연스럽게 축약 가능.

---

# 42. 일본어 검색의도 감사의 한계

Japanese SERP 조사는 중요한 참고다.

하지만 다음과 동일하지 않다.

- Search Console 실제 query 데이터
- 검색량 보장
- 순위 보장
- 사실 검증

검색의도 조사 결과는:

- title/H1 표현
- 질문 wording
- 공식 용어
- 검색자가 문제를 부르는 방식

에 사용한다.

사실·운영조건은 Research Master / 공식 출처를 따른다.

---

# 43. 검색의도와 사실조사를 분리한다

예:

일본 검색에서 `おすすめホテル`이 강하게 보여도
호텔 자체의 추천 판단을 바꾸지 않는다.

일본 검색에서 `日本語対応`이 자주 보여도
공식 확인 없이 호텔 설명에 추가하지 않는다.

일본 검색에서 `最安`이 많이 보여도
Korea Inside가 확인하지 않은 최저가를 쓰지 않는다.

즉:

> **검색어는 표현을 현지화하고, 사실은 사실층에서 검증한다.**

---

# 44. Golden Sample의 역할

Golden Sample은:

- 공통 UI
- 일본어 톤
- CTA register
- breadcrumb
- footer
- navigation
- Japanese brand treatment

의 참고 기준이다.

하지만 Golden Sample의 문장 구조를 모든 페이지에 복제하지 않는다.

페이지 역할과 검색의도가 다르면 문장도 달라야 한다.

> **Golden Sample = 품질 기준**
>
> **Golden Sample ≠ 문장 템플릿**

---

# 45. 일본어 현지화 완료 전 최종 질문

ChatGPT는 Batch 승인 요청 전에 다음을 묻는다.

1. 일본인이 번역문이 아니라 여행 가이드로 읽을 수 있는가?
2. English의 사실·수치·추천 판단이 모두 살아 있는가?
3. English 사용자 노출 target이 하나도 빠지지 않았는가?
4. 원문에 없는 요소를 추가하지 않았는가?
5. 일본 검색자가 이 문제를 부르는 방식이 title/H1에 자연스럽게 반영됐는가?
6. 공식 일본어 용어가 필요한 곳에 반영됐는가?
7. 짐·출구·환승·마지막 도보 같은 Korea Inside의 판단 가치가 약해지지 않았는가?
8. OTA형 “おすすめ” 문장으로 퇴행하지 않았는가?
9. visible FAQ와 schema가 맞는가?
10. 실제 Production sibling만 일본어 링크로 연결되는가?

하나라도 명확히 FAIL이면 승인 요청 전에 수정한다.

---

# 46. 이 Standard의 핵심 문장

> **일본어 현지화는 영어 페이지를 일본어로 요약하는 작업이 아니다.**

> **Current English Production의 모든 사용자 노출 구조를 먼저 잠그고, 누락 없이 그 구조 안에서 자연스러운 일본어를 작성한다.**

> **사실·수치·추천 판단·HTML 구조는 보존하고, 일본어 문장·검색 표현·공식 용어만 현지화한다.**

> **일본어가 자연스럽다는 이유로 English의 판단을 약화하거나 확장하지 않는다.**

> **일본 검색어를 넣기 위해 원문에 없는 요소를 만들지 않는다.**

> **ChatGPT와 사용자만 일본어를 판단하고, Codex는 승인본을 exact implementation한다.**

> **기본 Batch는 5페이지다.**

> **Source Coverage는 100%, missing은 0이어야 한다.**

> **STOP은 실패가 아니라 승인되지 않은 판단을 막는 정상 동작이다.**

> **오늘과 내일, 다른 방에서도 같은 Standard를 읽고 같은 품질 기준으로 제작한다.**

---

# 47. 문서 관리 원칙

- 이 문서는 일본어 현지화의 전용 Specialized Standard다.
- 상위 3개 Standard보다 우선하지 않는다.
- 현재 페이지 완료 수, commit SHA, 특정 Batch 진행상황은 이 문서에 고정하지 않는다.
- 현재 상태는 Japanese Master Inventory와 최신 Room Handover에서 관리한다.
- 일본어 공식 용어와 검색 표현이 장기적으로 바뀌면 사용자 승인 후 Version을 갱신한다.
- 예시 하나가 바뀌었다고 Standard 전체를 임의 수정하지 않는다.
- 이 문서를 변경하려면 사용자의 명시적 승인을 받는다.

---

# 48. 저장 위치

Korea Inside 저장소 루트:

`C:\Projects\Koreainside\Korea_Inside_Japanese_Localization_Standard.md`

Repository-relative path:

`Korea_Inside_Japanese_Localization_Standard.md`

페이지별 일본어 Localized / Approved Public Copy는 이 파일과 분리한다.

예:

`md/승인본/일본어/`

Japanese Inventory는 작업자료에서 관리한다.

예:

`md/작업자료/Korea_Inside_Japanese_Localization_Master_Inventory_YYYY-MM-DD.md`

---

# 49. 새 방 최초 일본어 작업 시작 보고

새로운 대화방에서 일본어 실제 작업을 처음 시작할 때만 먼저 짧게 보고한다.

1. 읽은 상위 Standard 전체 경로와 Version
2. Japanese Standard 전체 확인 여부
3. 최신 Handover 확인 여부와 날짜
4. 현재 Japanese Inventory
5. 이번 Batch 대상 페이지
6. source structure / fingerprint 확인 상태
7. 보호해야 할 사용자 변경

같은 대화방의 후속 Batch에서는 변경되지 않은 공통 Standard에 대한 동일 보고를 반복하지 않는다. 이번 Batch에 새로 필요한 자료와 변경된 상태만 확인한다.

그 후 사용자 지시 범위만 작업한다.

---

# 50. Version 1.0에 포함된 재발 방지 항목

Version 1.0은 다음 실제 문제의 재발을 막기 위해 만들어졌다.

- English 구조보다 짧은 요약형 일본어 Approved Copy
- H2 / FAQ / image / caption 누락
- `<dt>` 같은 작은 user-facing label 누락
- 원문에 없는 eyebrow 승인
- 공식 source link / source description 누락
- visible FAQ / JSON-LD wording 불일치
- 일본어 문장 끝 ASCII `.`
- raw Markdown의 Production 잔존
- `<strong>`와 조사 결합 오류
- 링크 앞뒤 영어식 spacing
- 영어식 직역 표현 반복
- 일본 공식 용어와 일본 검색 표현 미반영
- 이미 공개된 Japanese sibling이 있는데 English fallback이 남는 문제
- correction이 여러 파일로 분산되어 구현 Source가 불명확해지는 문제

Version 1.0의 목표는 하나다.

> **같은 Source, 같은 기준, 같은 QA로 일본어 58페이지 전체의 품질을 일관되게 유지한다.**


---

# 51. Version 1.1 변경기록

- 문서 상태를 `ACTIVE / SPECIALIZED STANDARD`로 전환
- 같은 방에서 변경되지 않은 Standard를 반복해서 전체 읽지 않는 재사용 원칙 추가
- Standard 검토·수정 자체는 Japanese Batch 실행으로 간주하지 않는 예외 추가
- Codex의 일본어 판단 금지는 유지하면서 승인된 **Technical Source Extraction**만 조건부 허용
- hreflang을 `en/es/ja` 고정 목록에서 **실제 Production sibling set 기반**으로 일반화
- Production은 사용자의 명시적 승인 후에만 실행하도록 Gate 강화
- 새 방 최초 일본어 작업 시작 보고는 1회만 수행하도록 정리
