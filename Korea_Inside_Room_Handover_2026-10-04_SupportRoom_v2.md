# Korea Inside — Room Handover — Support Room Migration
**Date:** 2026-10-04
**Status:** ACTIVE HANDOVER
**Next room role:** Korea Inside 지원방
**Reason for migration:** 현재 방에서 간단한 작업에도 반복 정지/도구 오호출/응답 중단이 발생. 새 방에서 안정적으로 승계한다.

---

## 1. 최우선 운영 규칙

### Deep Research 금지
SEO 경쟁분석에서는 **Deep Research 기능을 사용하지 않는다.**
사용자가 "심층 분석 / 심층 조사 / 깊게 조사 / 심층 보고서"라고 해도 이는 **분석의 깊이**를 뜻한다.
사용자가 앞으로 명시적으로 "Deep Research 기능을 사용해라"라고 별도 지시하지 않는 한 호출 금지.

기존 Deep Research 테스트 결과는 58페이지 SEO 구조조정 근거로 사용하지 않는다.

### 지원방 역할
이 방/새 방은 조사, SEO/GEO 분석, 오류 해결, 상태 확인, Codex 결과 검토, 지시문 보조, Handover, 파일/경로 확인을 담당한다.
사용자 승인 없는 HTML/Production/Git 변경은 하지 않는다.

---

## 2. 10월 사용자 확정 방향

주력은 **기존 58페이지 보완**이다.

우선:
1. SEO
2. 현지 검색의도
3. GEO/AI Search
4. Humanization
5. 최신성
6. 내부링크/Topic Cluster
7. CTA/제휴 흐름

신규 페이지는 무작정 만들지 않는다.
조사 중 독립 검색의도가 확인되면 **NEW PAGE CANDIDATE**로 Backlog에 기록하고 제작은 별도 승인 후 진행한다.

---

## 3. Taste Korea / Food Guide 확장 아이디어

장기 구상:
```
Taste Korea (Landing/Hub)
→ Food Travel Guide / Category Branch
→ Area Food Guide
→ Restaurant entries
↔ Area Travel Guide
↔ Stay Guide
```

식당 수는 5~6개로 제한하지 않는다. 근거와 여행가치가 있으면 동네별 20/30/50곳도 가능.
아침/점심/저녁/야식과 실제 여행동선에 연결한다.
Travel Guide → 식사 시점 → Food Guide,
Food Guide → Area Travel Guide / Stay Guide로 역링크한다.
국가별 시장 선호에 따라 식당 selection은 다르게 할 수 있다.
현재는 신규 제작보다 58페이지 보완이 우선이므로 아이템 발굴/Research Backlog로 유지.

---

## 4. 현재 ACTIVE 작업 — Thailand SEO 5-page Pilot

목적:
태국어판 58페이지 전체 SEO 구조조정 전에 5페이지씩 경쟁분석한다.

운영:
- 58페이지 한방 조사 금지.
- 5페이지씩 Batch.
- Batch 완료 후 **MD 한 파일로만 제출**.
- 중간에 페이지별 결과를 따로 보고하지 않는다.
- 완료 Batch는 다시 처음부터 조사하지 않는다.
- 최종적으로 Batch MD를 통합해 Thailand 58P SEO Restructuring Master Report 작성.

현재 Pilot:
1. `th/hongdae-travel-guide.html`
2. `th/myeongdong-travel-guide.html`
3. `th/seongsu-travel-guide.html`
4. `th/lotte-world-seoul.html`
5. `th/k-beauty.html`

조사 방식:
- 일반 GPT 분석
- 일반 웹검색
- 실제 태국어 SERP
- 실제 경쟁페이지 직접 열람
- GitHub 현재 Korea Inside 소스
- Semrush는 데이터 사용 가능할 때만

Semrush 상태:
2026-10-04 API 호출에서 `API UNITS BALANCE IS ZERO` 확인.
새 Volume/KD/traffic 수치 추측 금지.

SERP:
시점/지역/개인화가 있으므로 확인되지 않은 "절대 1위/2위" 표현 금지.
필요하면 Direct Editorial Benchmark #1/#2로 기록.

---

## 5. 현재까지 확인한 Thai 소스

Repository: `leesuckpyo/Koreainside`
확인 branch: `th-localization-2026-10-03`

### Hongdae
Path: `th/hongdae-travel-guide.html`
SHA: `1d6dda56229936bab68bd51b50ed3d94677e0362`
Title: `เที่ยวฮงแด 2026: ที่เที่ยว ช้อป กิน คาเฟ่ + แผนเที่ยว 1 วัน | Korea Inside`
Meta: `วางแผนเที่ยวฮงแดแบบไม่ยัดทุกอย่างในวันเดียว เริ่มยอนนัม ต่อช้อป กิน คาเฟ่ Red Road และช่วงค่ำ พร้อมเส้นทาง 3–4 ชม. เต็มวัน และวันที่มีมังวอน`
H1: `เที่ยวฮงแด 2026 : ยอนนัม ช้อป กิน คาเฟ่ และแผนเที่ยว 1 วัน`

강점:
Exit 3 vs 9 / Yeonnam 체류시간 / 쇼핑 / 음식 timing / paid experience / nightlife / Mangwon-Hapjeong-Sangsu / route / traveler type / weather / October layer.
잠정: **P1 / 본문 KEEP**

### Myeongdong
Path: `th/myeongdong-travel-guide.html`
SHA: `f2119d9cf180f10c47725c04b26ffb39e7f0884f`
Title: `เที่ยวเมียงดง 2026: K-Beauty ช้อปปิ้ง ของกิน + แผนเที่ยว | Korea Inside`
Meta: `เที่ยวเมียงดงแบบไม่เสียเวลา วางแผน K-Beauty และ Olive Young เลือกเข้าจาก Myeongdong หรือ Euljiro 1-ga กินมื้อจริง แวะโบสถ์เมียงดง แล้วต่อ Namsan, NANTA หรือ Euljiro`
H1: `เที่ยวเมียงดง 2026 : K-Beauty ช้อปปิ้ง ของกิน และเส้นทางช่วงเย็น`

강점:
Myeongdong Station vs Euljiro 1-ga / 목적별 쇼핑 / K-Beauty / street food vs real meal / Cathedral / Personal Color-NANTA / Namdaemun-Namsan-Euljiro / weather / route / October layer.
잠정: **P1 / 본문 KEEP**

### Seongsu
Path: `th/seongsu-travel-guide.html`
SHA: `c5bcbd980f2214e5deaa0b28a51f574ed7dc50cc`
Title: `เที่ยวซองซู 2026: คาเฟ่ ป๊อปอัพ ช้อปปิ้ง K-Beauty + Seoul Forest | Korea Inside`
Meta: `เที่ยวซองซูแบบไม่ไล่ตามทุกกระแส เลือก Seongsu Station หรือ Seoul Forest วางเส้นทางคาเฟ่ ป๊อปอัพ แฟชั่น K-Beauty และร้านถาวรที่คุ้มเวลา`
H1: `เที่ยวซองซู 2026 : ป๊อปอัพ คาเฟ่ ช้อปปิ้ง K-Beauty และ Seoul Forest`

강점:
Seongsu Station vs Seoul Forest / Yeonmujang-gil / popup freshness / flagship / Olive Young N vs AMORE / cafe / industrial layer / Seoul Forest / real meal / routes / traveler fit.
잠정: **P1 / 본문 KEEP / 일부 Search-facing H2 검토**

---

## 6. Lotte World / K-Beauty 상태 주의

조사 당시 `th-localization-2026-10-03`에서:
- `th/lotte-world-seoul.html`
- `th/k-beauty.html`
을 GitHub API로 찾지 못했다.

새 방에서 최신 Thai Batch 상태/branch/실제 sibling 존재 여부를 먼저 확인한다.
존재하면 Thai source 기준으로 분석한다.
없으면 Thai Search-facing 문구는 Research Candidate로만 취급한다.

### Lotte World English main reference
SHA: `0cb3a3b214f33ce06cf3ae13872080fffbf6c8a4`
Title: `Lotte World Seoul Guide 2026: Tickets, Magic Pass, Rides & Tips`
강점: ticket / Adventure vs Magic Island / rides / traveler type / must-rides / queues / Magic Pass / kids / weather / food-rest / school uniform / getting there / combination / booking / FAQ.
잠정: **P0 Thai Search-facing / body role KEEP**

### K-Beauty English main reference
SHA: `1d61310b3ca562ee9ff5acb6316e1c832bd11dcd`
Title: `K-Beauty in Korea: Experiences, Seoul Areas & Reordering | Korea Inside`
강점: skincare / makeup-personal color / hair-scalp / clinic consultation / fragrance / Myeongdong-Seongsu-Hongdae-Gangnam-Apgujeong comparison / reorder / Stay connection.
잠정: **P0 Thai Market Intent**

---

## 7. 지금까지 잡은 경쟁 Benchmark

### Hongdae
- Mushroom Travel Hongdae guide
- Tourkrub Hongdae guide

경쟁사 강점: `ที่เที่ยวฮงแด`, shopping, food, cafe 같은 직접 검색어, 구체 Entity/시간/지도.
KI 강점: 동선/선택판단/체류시간/날씨/여행자유형.
잠정: Title/H1 KEEP, Quick Answer/Entity visibility P1.

### Myeongdong
- Wherebest Myeongdong guide
- MEETORY Myeongdong guide

경쟁사 강점: K-Beauty / street food / station / quick facts.
KI 강점: 역 선택, 쇼핑목적, 음식 선택, 다음 지역 연결.
잠정: Title/H1 KEEP, Quick Facts/즉답성 P1.

### Seongsu
- IVisitKorea Thai Seongsu guide
- Aumjumma Seongsu guide

경쟁사 강점: `วิธีเดินทาง`, cafe, restaurant, attractions 같은 직접 검색표현과 구체 매장/가격/시간/지도.
KI 강점: 출발점 선택, Yeonmujang-gil, popup freshness, K-Beauty 선택, route, traveler fit.
잠정: Title/H1 KEEP, 일부 H2 P1.

### Lotte World
- Klook Thailand editorial guide
- KoreaTiew Lotte World guide

태국 검색축: `ล็อตเต้เวิลด์` / `ตั๋ว` / `Magic Pass` / `เครื่องเล่น`.
KI English content depth는 강함.
경쟁사의 가격/티켓 정보는 공식 최신정보 검증 없이 복사 금지.
잠정: **P0 Search-facing**

### K-Beauty
- Creatrip Thai K-Beauty
- IVisitKorea Thai K-Beauty related content

태국 K-Beauty intent는 product shopping / Olive Young / Personal Color / Hair-Makeup / clinic-skincare / beauty experience booking으로 분화.
Creatrip 강점: category architecture / booking / price / trust.
KI 강점: 여행자 선택판단 / Seoul area comparison / shopping vs appointment / Stay 연결.
잠정: **P0 Market Intent**

---

## 8. 최종 5-page MD에 반드시 들어갈 것

각 페이지:
1. Search Intent
2. Primary/Supporting Thai Keywords
3. SERP observation
4. Direct Competitor #1/#2
5. 경쟁페이지 실제 구조
6. Korea Inside 실제 구조
7. 1:1 SEO GAP table
8. KEEP
9. P0/P1/P2
10. SEO Surface Gap
11. Content Gap
12. Market-specific Gap
13. Competitor-only content
14. New Page Candidate
15. 필요한 경우만 Thai Title/Meta/H1/H2/FAQ 후보
16. 기존 58페이지 기반 inbound/outbound internal links
17. 최종 Primary Keyword
18. 최종 PASS/P0/P1/P2

1:1 GAP table rows:
Title / Meta / H1 / Intro-Quick Answer / H2 / H3 / keyword language / intent / depth / practical info / decision support / freshness / entities / internal links / FAQ / CTA / media-map / trust-source / commercial usefulness.

5페이지 통합:
- Cross-Page Findings
- Thailand SEO Restructuring Rules
- P0 Action List
- P1 Action List
- P2 Action List
- New Page Backlog
- Final Recommendation

중요:
**중간보고 금지. 5페이지 전체를 끝내고 MD 한 파일로 제출.**

---

## 9. 구조조정의 현재 핵심 가설

현재 Pilot에서 보이는 방향:

경쟁사 강점:
**검색어를 그대로 쓴 heading → 구체 장소/상품 → 가격/시간 → 지도 → 예약/CTA**

Korea Inside 강점:
**어디서 시작 → 얼마나 머묾 → 무엇을 뺌 → 누구에게 맞음 → 다음 동선 → 숙소/다른 지역 선택**

따라서:
> 경쟁사처럼 본문을 단순 리스트로 바꾸지 않는다.
> Korea Inside의 판단 구조를 보호하고 태국 경쟁사의 Search-facing 명료성을 앞단에 결합한다.

잠정:
- Hongdae: P1
- Myeongdong: P1
- Seongsu: P1
- Lotte World: P0 Search-facing
- K-Beauty: P0 Market Intent

최종 판정은 5페이지 전체 경쟁분석 완료 후 확정한다.

---

## 10. 새 방의 정확한 다음 행동

1. Public Content Master / Navigation Standard / Thai Localization Standard v1.1 / 최신 Handover 확인.
2. 이 Handover의 ACTIVE SEO 상태 승계.
3. **Deep Research 호출 금지.**
4. Thai Lotte World / K-Beauty 실제 최신 sibling/branch 확인.
5. 기존 Hongdae/Myeongdong/Seongsu 조사 지점은 버리지 말고 이어서 경쟁페이지 검증.
6. 5페이지 모두 1:1 GAP/수정후보/internal-link까지 완성.
7. 중간보고하지 말고 최종 MD 하나로 제출.
8. HTML/Git/Production 구현은 하지 않는다.
9. 사용자 승인 후 다음 5페이지 Batch로 이동.

---

## 11. 다시 논의하지 않을 확정사항

- 58페이지 한방 SEO 조사 금지.
- 5페이지씩 Batch.
- Deep Research 기능 금지.
- "심층" = 분석 깊이.
- 10월 중심 = 기존 58페이지 보완.
- 신규 페이지 = 후보 발굴, 제작은 별도 승인.
- 경쟁사 문구 복사 금지.
- KI의 여행판단/Humanization 강점 보호.
- Batch 결과는 페이지별 채팅 중간보고가 아니라 **MD 한 파일**로 제출.

