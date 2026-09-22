# Korea Inside Navigation & Hub Architecture Standard

**File:** `Korea_Inside_Navigation_Hub_Architecture_Standard.md`  
**Status:** ACTIVE / APPROVED STRUCTURE  
**Version:** 1.0  
**Effective date:** 2026-09-21  
**Higher standard:** `Korea_Inside_Public_Content_Master_Standard.md` Version 1.3  

> 이 문서는 Korea Inside의 글로벌 메뉴와 지역별 Hub 구조를 장기적으로 확장하기 위한 정보구조 표준이다.
>
> 핵심 원칙: **글로벌 메뉴는 사이트맵이 아니다. 글로벌 메뉴는 지역 Hub로 들어가는 짧은 진입점이고, 세부 지역·명소·행사·숙소 페이지는 각 Hub 안에서 설명과 함께 연결한다.**

---

# 1. 목적

Korea Inside는 서울만 다루는 사이트에 머무르지 않고 향후 부산, 제주, 전주, 강원 등으로 확장한다.

따라서 홍대, 명동, 성수, 동대문, 북촌, 청계천, DMZ, 행사 페이지 등을 글로벌 메뉴에 계속 직접 추가하는 방식은 사용하지 않는다.

콘텐츠가 30개, 50개, 100개로 증가해도 상단 메뉴는 짧고 이해하기 쉬워야 한다.

승인된 구조는 다음과 같다.

> **Global Menu → Regional Hub → Detail Page**

숙소도 같은 원칙을 적용한다.

> **Stay Menu → Regional Stay Hub → Stay Detail → Hotel Decision → OTA**

---

# 2. 글로벌 메뉴의 역할

글로벌 메뉴의 목적은 사용자가 사이트의 큰 축을 빠르게 선택하게 하는 것이다.

글로벌 메뉴에 모든 하위 페이지를 나열하지 않는다.

다음은 기본적으로 글로벌 메뉴에 직접 넣지 않는다.

- 개별 서울 지역 페이지 전체
- 개별 관광지 페이지 전체
- 개별 시장 페이지 전체
- 개별 축제·행사 페이지 전체
- 개별 숙소 지역 페이지 전체
- 향후 부산·제주 등의 모든 세부 지역 페이지

이 콘텐츠는 해당 지역 Hub 안에서 분류하고 짧은 설명과 함께 연결한다.

예외는 사용자의 명시적 승인으로만 허용한다.

---

# 3. 글로벌 Travel 메뉴 — 승인 구조

Travel 메뉴는 **지역 Hub 중심**으로 운영한다.

기본 구조:

```text
Travel
├─ Seoul Guide
├─ Busan Guide
├─ Jeju Guide
├─ Jeonju Guide
└─ Gangwon Guide
```

현재 실제 공개 페이지가 존재하지 않는 지역은 메뉴에 미리 노출하지 않는다.

즉 구조는 위와 같이 고정하되, 실제 링크는 해당 Hub 페이지가 Production에 공개된 뒤 추가한다.

새로운 지역이 추가되면 같은 원칙을 따른다.

예:

- Gyeongju Guide
- Incheon Guide
- Suwon Guide

지역이 추가된다고 해서 글로벌 메뉴에서 그 지역의 세부 동네를 바로 펼치지 않는다.

---

# 4. Seoul Guide — 서울 Travel Hub

`Seoul Guide`는 서울 관련 공개 콘텐츠의 대표 Hub다.

홍대·명동·성수 등 개별 페이지를 글로벌 Travel 메뉴에 계속 나열하지 않고, Seoul Guide 안에서 목차형으로 분류한다.

각 항목은 단순 링크 목록으로 만들지 않는다.

기본 형태:

> **페이지명 / 장소명**  
> 여행자가 왜 이 페이지를 봐야 하는지 1~3문장 설명  
> → Detail Guide 링크

## 4.1 Seoul Areas

예:

- Hongdae
- Myeongdong
- Seongsu
- Insadong
- Gangnam
- Jamsil
- Gongdeok & Mapo
- Itaewon
- Dongdaemun
- 이후 실제 지역 가이드 추가

각 지역은 여행 성격과 선택 이유를 짧게 설명한 뒤 Detail로 연결한다.

## 4.2 Attractions & Places

예:

- Bukchon
- Cheonggyecheon
- Gyeongbokgung
- Lotte World
- Seoul Sky
- Namsan
- 기타 향후 승인된 관광지

관광지 수가 늘어나더라도 글로벌 메뉴에 직접 추가하지 않는다.

## 4.3 Markets & Shopping

예:

- Dongdaemun
- Gwangjang Market
- Namdaemun Market
- 기타 전문 쇼핑·시장 콘텐츠

시장 자체가 독립 Detail로 발전해도 Seoul Guide에서 짧은 설명과 함께 연결한다.

## 4.4 Events & Festivals

예:

- Seoul Events & Festivals
- Seoul Lantern Festivals
- Seoul International Fireworks Festival
- Concerts in Seoul
- K-pop Concerts
- Seoul World DJ Festival
- New Year's Eve in Seoul

개별 행사 Detail을 글로벌 메뉴에 계속 추가하지 않는다.

`Seoul Events & Festivals`는 서울 행사 Current Hub 역할을 한다.

## 4.5 Day Trips from Seoul

예:

- DMZ
- Suwon
- Nami Island 등

서울 밖에 있더라도 서울 여행자가 서울에서 출발하는 검색 의도로 찾는 콘텐츠는 이 구간에서 연결할 수 있다.

## 4.6 필요 시 추가 가능한 Seoul Hub 구간

검색 의도와 콘텐츠 축적에 따라 다음 구간을 추가할 수 있다.

- Food
- Nightlife
- Family
- Seasonal Seoul
- One-day itineraries
- Transportation

단, Hub의 목차를 고정 템플릿으로 만들지 않는다.
실제 콘텐츠가 충분히 존재할 때만 추가한다.

---

# 5. 다른 지역 Travel Hub 확장 규칙

부산, 제주, 전주, 강원 등도 Seoul Guide와 동일한 원칙을 적용한다.

초기에는 글로벌 메뉴에 대표 Hub 하나만 둔다.

예:

```text
Busan Guide
```

Busan 콘텐츠가 늘어나면 Busan Guide 내부에서:

- Haeundae
- Gwangalli
- Seomyeon
- Nampo
- Attractions
- Beaches
- Food
- Events
- Day Trips

처럼 확장한다.

하지만 글로벌 메뉴에서 Haeundae, Gwangalli, Seomyeon을 직접 계속 추가하지 않는다.

Jeju, Jeonju, Gangwon도 동일하다.

---

# 6. 글로벌 Stay 메뉴 — 승인 구조

Stay도 Travel과 같은 확장 원칙을 사용한다.

기본 구조:

```text
Stay
├─ Seoul Stay Guide
├─ Busan Stay Guide
├─ Jeju Stay Guide
├─ Jeonju Stay Guide
└─ Gangwon Stay Guide
```

실제 공개 Stay Hub가 없는 지역은 메뉴에 미리 링크하지 않는다.

기존 URL은 특별한 이유 없이 변경하지 않는다.

현재 `accommodation.html`을 Seoul Stay Guide 역할로 발전시킬 수 있는지는 별도 감사 후 결정한다.
URL 변경이나 신규 대표 URL 생성은 SEO·검색 의도 검토 후 승인한다.

---

# 7. Seoul Stay Guide — 서울 숙소 Hub

Seoul Stay Guide는 단순 호텔 목록이 아니라 서울 숙박 선택의 상위 Decision Hub다.

목적:

> **서울에서 어디에 묵을지 결정 → 해당 Stay Detail로 이동 → 실제 호텔 선택 → OTA 예약**

각 항목은 짧은 판단 설명과 함께 Detail로 연결한다.

## 7.1 Where to Stay by Area

예:

- Hongdae
- Myeongdong
- Gangnam
- Jamsil
- Dongdaemun
- Itaewon
- Insadong
- Seongsu
- Gongdeok
- Seoul Station

## 7.2 Choose by Traveler

예:

- First-Time Visitors
- Families
- Solo Travelers
- Couples
- Budget Travelers
- Shopping
- Nightlife
- Luxury

## 7.3 Choose by Transport / Arrival

예:

- Airport Access
- AREX
- Seoul Station
- Gongdeok
- Late Arrival
- Luggage-heavy itineraries

실제 검색 수요와 기존 Detail 페이지에 따라 구성한다.

---

# 8. Travel과 Stay의 관계

Travel Hub와 Stay Hub는 분리된 사일로가 아니다.

기본 흐름:

```text
Travel
→ Regional Travel Hub
→ Area / Attraction / Event Detail
→ Stay decision
→ Regional Stay Hub or Area Stay Detail
→ Hotel Decision
→ Expedia / Trip.com / Agoda
```

반대 방향도 연결한다.

```text
Stay
→ Regional Stay Hub
→ Area Stay Detail
→ 해당 Area Travel Guide
```

즉 Travel은 지역·경험 선택 이유를 만들고,
Stay는 그 선택을 실제 숙박과 예약으로 전환한다.

---

# 9. Hub 내부 링크 작성 규칙

Hub는 링크 디렉터리가 아니다.

각 링크 앞에는 최소한 다음 중 하나 이상이 있어야 한다.

- 이 지역/페이지가 어떤 여행자에게 맞는지
- 무엇을 하게 되는지
- 다른 선택지와 무엇이 다른지
- 언제 볼 가치가 있는지
- 어떤 문제를 해결하는지

금지:

```text
Hongdae
Myeongdong
Seongsu
Gangnam
```

처럼 이름과 링크만 나열하는 구조.

권장:

```text
Hongdae
Choose Hongdae if late evenings, cafés, music and a younger street scene are part of the trip.
→ Hongdae Guide
```

Hub 자체가 검색 가치와 여행 판단 가치를 가져야 한다.

---

# 10. SEO 구조 원칙

지역 Hub는 해당 지역 클러스터의 대표 내부링크 중심점이다.

예:

```text
Seoul Guide
├─ Areas
├─ Attractions
├─ Markets
├─ Events
└─ Day Trips
```

Detail 페이지는 필요할 때 Seoul Guide로 되돌아가는 문맥 링크를 제공한다.

목적:

- 검색 의도별 대표 URL 명확화
- 얇은 중복 Hub 방지
- 내부 PageRank 분배
- 신규 페이지 발견성 향상
- 지역 클러스터 구조 명확화

연도별 Hub를 반복 생성하지 않는다.
예를 들어 `seoul-events.html` 같은 evergreen 대표 URL을 유지하고 연도·상태를 갱신한다.

---

# 11. 확장성 고정 원칙

콘텐츠가 늘어날수록 글로벌 메뉴를 늘리는 것이 아니라 **Hub를 깊게 만든다.**

고정 원칙:

> **Global navigation breadth를 제한하고, Regional Hub depth를 확장한다.**

따라서 서울 콘텐츠가 50개가 되어도 글로벌 Travel 메뉴에는 `Seoul Guide` 하나가 대표 진입점이 될 수 있다.

부산 콘텐츠가 30개가 되어도 글로벌 Travel 메뉴에는 `Busan Guide` 하나가 대표 진입점이 된다.

숙소도 동일하다.

---

# 12. 메뉴 마이그레이션 원칙

현재 기존 메뉴를 이 구조로 바꿀 때 한 번에 404 링크를 만들지 않는다.

기본 순서:

1. Regional Hub 구조와 URL 확정
2. Hub PUBLIC COPY 제작 및 승인
3. Hub Production 배포
4. 공개 URL HTTP 200 확인
5. 글로벌 메뉴를 Hub 중심 구조로 변경
6. 전체 HTML Navigation QA
7. 모바일/데스크톱 메뉴 QA
8. sitemap / Search Console 확인

현재 존재하지 않는 Busan / Jeju / Jeonju / Gangwon Hub를 메뉴에 먼저 넣지 않는다.

---

# 13. 공통 Navigation 보호

이 문서는 정보구조의 승인 기준이다.

실제 구현 시에도 다음 기존 원칙을 유지한다.

- 사용자의 명시적 승인 없이 공통 header / navigation / footer / common.js / mobile hamburger 수정 금지
- 메뉴 구조 변경은 별도 구현 지시 및 QA 후 적용
- 기존 current / active 상태 보존
- 모바일과 데스크톱에서 동일한 핵심 메뉴 제공
- 존재하지 않는 URL 링크 금지
- 기존 사용자 파일 변경 보호

---

# 14. 현재 승인된 핵심 구조 요약

## Travel

```text
Travel
→ Seoul Guide
→ Busan Guide
→ Jeju Guide
→ Jeonju Guide
→ Gangwon Guide
```

각 지역의 상세 페이지는 해당 Regional Travel Hub 안에서 목차형으로 설명하고 연결한다.

## Stay

```text
Stay
→ Seoul Stay Guide
→ Busan Stay Guide
→ Jeju Stay Guide
→ Jeonju Stay Guide
→ Gangwon Stay Guide
```

각 지역의 세부 숙소 페이지는 해당 Regional Stay Hub 안에서 판단 설명과 함께 연결한다.

---

# 15. 프로젝트 핵심 문장

> **메뉴에 페이지를 계속 추가하지 않는다. 지역 Hub를 추가하고, 세부 콘텐츠는 Hub 안에서 확장한다.**

> **Travel은 Regional Travel Hub 중심, Stay는 Regional Stay Hub 중심으로 확장한다.**

> **Global menu는 짧게 유지하고, 콘텐츠 깊이는 Hub에서 확장한다.**

> **Travel Guide가 여행 선택을 만들고, Stay Guide가 숙소 선택을 좁히며, Area Stay Detail이 호텔과 OTA 예약으로 전환한다.**

---

# 16. 문서 관리 원칙

- 이 문서는 글로벌 Navigation / Regional Hub 정보구조의 Source of Truth다.
- 상위 기준은 `Korea_Inside_Public_Content_Master_Standard.md` Version 1.3이다.
- 두 문서가 충돌하면 Master Standard를 우선한다.
- 실제 지역별 페이지 완료 상태는 이 문서에 하드코딩하지 않는다.
- 신규 지역이 추가되어도 기본 원칙은 바꾸지 않고 Regional Hub 항목만 확장한다.
- 메뉴 구조를 변경할 때는 사용자 승인 후 이 문서의 Version을 갱신한다.
