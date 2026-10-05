# Korea Inside — Active Document Index

**Date:** 2026-09-26  
**Status:** ACTIVE DOCUMENT SET  
**Purpose:** 저장소/프로젝트에 현재 활성 기준 문서만 다시 저장할 때 사용하는 기준 목록.

## 1. 저장할 활성 MD

1. `Korea_Inside_Public_Content_Master_Standard.md`
   - Status: ACTIVE / MASTER STANDARD
   - Version: 1.4
   - Role: 공개 콘텐츠 최상위 Source of Truth

2. `Korea_Inside_Navigation_Hub_Architecture_Standard.md`
   - Status: ACTIVE / APPROVED STRUCTURE
   - Version: 1.1
   - Role: Navigation / Regional Hub 정보구조 기준

3. `Korea_Inside_Travel_Guide_Production_Playbook.md`
   - Status: ACTIVE / PRODUCTION PLAYBOOK
   - Version: 1.1
   - Role: Travel Guide 제작 공정 / Quality Gate / Codex 구현 절차

4. `Korea_Inside_Language_Localization_Standard.md`
   - Status: ACTIVE / SPECIALIZED STANDARD
   - Version: 2.1
   - Role: 다국어 현지화 공통 실행 기준

5. `Korea_Inside_Japanese_Localization_Standard.md`
   - Status: ACTIVE / SPECIALIZED STANDARD
   - Version: 1.1
   - Role: 일본어 전용 현지화 / Humanization / QA 기준

6. `Korea_Inside_Room_Handover_2026-09-26.md`
   - Status: ACTIVE PROJECT HANDOVER
   - Role: 현재 작업상태, Batch, Git/Production, 보호 변경, 다음 작업 승계

## 1A. 조건부 활성 MD — 작업 Family별 보존 / 필요 시 확인

아래 문서는 모든 작업에서 예방적으로 읽는 공통 세트가 아니라, **해당 Page Family 작업에 실제로 적용될 때만 확인하는 조건부 활성 문서**다.

- 현재 `ACTIVE / APPROVED` 상태인 Family Design Standard
- Travel Guide Family Design Standard
- Stay / Hotel Family Design Standard
- 향후 새 Family에 대해 사용자 승인 후 ACTIVE가 된 Design Standard

원칙:

- Active Family Design Standard는 **활성 소스에서 삭제하지 않는다.**
- 다만 현재 작업과 관계없는 Family Standard를 매번 전체 읽지 않는다.
- 동일 Family 문서가 여러 개면 파일명 suffix가 아니라 문서 내부 `Status / Version / Effective date / 변경기록`으로 최신 유효본을 판정한다.
- `ARCHIVED / SUPERSEDED / OLD` Family 문서는 현재 기준으로 사용하지 않는다.
- 해당 Family에 Active Design Standard가 없으면 기존 페이지나 과거 Handover를 근거로 임의의 새 디자인 규격을 확정하지 않는다.

## 2. 문서 역할 구분

- Standard = 정책과 판단 기준
- Production Playbook = 제작/구현 공정과 Quality Gate
- Handover = 현재 작업 상태
- Inventory = 언어별 COMPLETE / MISSING / EXCLUDE 상태
- Research Master = 페이지별 사실·조사 근거
- Approved Public Copy / CONTENT LOCKED = 승인된 공개 문구
- Active Family Design Standard = 해당 Page Family의 디자인/컴포넌트 기준

## 3. 기본 우선순위

현재 사용자 명시 지시
→ Public Content Master Standard
→ Navigation Hub Architecture Standard
→ 해당 작업에 적용되는 공통 Specialized Standard / Production Playbook
→ 해당 언어 전용 Standard
→ Active Family Design Standard
→ 최신 Handover
→ 언어별 Master Inventory
→ Research Master
→ Approved Public Copy / CONTENT LOCKED
→ 현재 Production HTML

Codex의 저장소 실행·Git·QA 범위는 별도의 `AGENTS.md` 및 `docs/standards-hub.md` 체계가 적용될 수 있으며, ChatGPT의 콘텐츠 판단 명령체계와 혼동하지 않는다.

## 4. 저장하지 않을 과거 Handover

활성 세트에는 아래 과거 Handover를 포함하지 않는다.

- `Korea_Inside_Room_Handover_2026-09-17*.md`
- `Korea_Inside_Room_Handover_2026-09-18*.md`
- `Korea_Inside_Room_Handover_2026-09-22*.md`

현재 상태 승계용 Handover는 `Korea_Inside_Room_Handover_2026-09-26.md` 하나만 사용한다.

## 5. 저장 원칙

- 파일명 뒤 `(1)`, `(2)`, `(7)`, `copy`, `backup` 등을 붙이지 않는다.
- 위 canonical filename 그대로 저장한다.
- 같은 문서의 구버전을 활성 소스 목록에 함께 두지 않는다.
- 새 버전을 만들면 Version / Effective date / 변경기록을 갱신한다.
- 조건부 Active Family Design Standard도 활성 소스 자산으로 보존하되, 실제 작업에 적용될 때만 읽는다.
