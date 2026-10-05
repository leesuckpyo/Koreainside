# Korea Inside — Thai D Utility Gap Correction Supplement — 2026-10-05

**Status:** LOCALIZATION REVIEW — GAP CORRECTION ONLY  
**Scope:** D Utility 5-page reconciliation blockers only  
**Existing Approved Public Copy:** `Korea_Inside_TH_D_Utility_5Page_Approved_Public_Copy_2026-10-05.md`  
**Existing Approved Public Copy SHA-256:** `be1ec7c3ab48fa145842f1c886707a1fef797e40c141f8711cda80dcdcfdecb9`  
**Existing Manifest:** `Korea_Inside_TH_D_Utility_Approved_Manifest_2026-10-05.md`  
**Existing Manifest SHA-256:** `708fb0019bf84ec770b5dcb7b1d4590f6ccdd27e111c4c3336611e298e85ef7c`

This supplement resolves only the reported D Utility blockers:

- Unmapped: **26**
- Orphan: **0**
- Ambiguous: **19**

It does **not** reopen or rewrite any other Approved / CONTENT LOCKED wording.

---

# 1. `apps.html` — 11 missing official-source descriptions

The source link titles already approved remain unchanged. Add the following Thai values to the corresponding `<span>` description nodes.

## APP-GAP-01
English:
`Official interface and map-language information.`

Thai:
`ข้อมูลทางการเกี่ยวกับภาษาของอินเทอร์เฟซและแผนที่`

## APP-GAP-02
English:
`Official visitor-focused taxi features, supported languages and app downloads.`

Thai:
`ฟีเจอร์แท็กซี่สำหรับนักท่องเที่ยว ภาษาที่รองรับ และลิงก์ดาวน์โหลดแอปจากแหล่งทางการ`

## APP-GAP-03
English:
`Official restaurant discovery, reservation and waitlist service.`

Thai:
`บริการทางการสำหรับค้นหาร้านอาหาร จองโต๊ะ และเข้าคิว`

## APP-GAP-04
English:
`Official ordering service and current delivery availability for international users.`

Thai:
`บริการสั่งอาหารทางการและพื้นที่ให้บริการปัจจุบันสำหรับผู้ใช้ต่างชาติ`

## APP-GAP-05
English:
`Official Woowa Brothers engineering overview of the multilingual core order flow.`

Thai:
`ภาพรวมทางวิศวกรรมอย่างเป็นทางการจาก Woowa Brothers เกี่ยวกับขั้นตอนสั่งอาหารหลักแบบหลายภาษา`

## APP-GAP-06
English:
`Official payment, top-up and separate T-money balance instructions.`

Thai:
`คำแนะนำทางการเรื่องการชำระเงิน การเติมเงิน และยอด T-money ที่แยกต่างหาก`

## APP-GAP-07
English:
`Official mobile Tmoney availability and supported-device guidance.`

Thai:
`ข้อมูลทางการเกี่ยวกับการรองรับ Mobile Tmoney และอุปกรณ์ที่รองรับ`

## APP-GAP-08
English:
`Official booking channel for KTX and other KORAIL-operated trains.`

Thai:
`ช่องทางจองทางการสำหรับ KTX และรถไฟอื่นที่ KORAIL ให้บริการ`

## APP-GAP-09
English:
`Official travel information, planner and itinerary features from the Korea Tourism Organization.`

Thai:
`ข้อมูลท่องเที่ยว เครื่องมือวางแผน และฟีเจอร์แผนการเดินทางทางการจาก Korea Tourism Organization`

## APP-GAP-10
English:
`Official overview of alerts, shelters, emergency facilities, embassy information and safety guidance.`

Thai:
`ภาพรวมทางการของการแจ้งเตือน ที่พักพิง จุดฉุกเฉิน ข้อมูลสถานทูต และคำแนะนำด้านความปลอดภัย`

## APP-GAP-11
English:
`Official travel-information and interpretation support channel, distinct from police and fire or medical dispatch.`

Thai:
`ช่องทางทางการสำหรับข้อมูลการท่องเที่ยวและบริการล่าม ซึ่งแยกจากตำรวจ ดับเพลิง และการแพทย์ฉุกเฉิน`

---

# 2. `maps.html` — 13 missing targets

## 2.1 Repeated `Action` labels — 7 nodes

For each of the seven `<span class="maps-small-label">Action</span>` nodes:

English:
`Action`

Thai:
`สิ่งที่ต้องทำ:`

Apply exactly to all **7** Action label nodes in source order.

## 2.2 `Important` label — 1 node

English:
`Important`

Thai:
`สำคัญ:`

This applies only to the `maps-caution` label in the Google Maps section.

## 2.3 Official-source descriptions — 5 nodes

### MAP-GAP-SRC-01
English:
`Official Android and iPhone paths for changing the app language.`

Thai:
`เส้นทางการตั้งค่าทางการสำหรับเปลี่ยนภาษาแอปบน Android และ iPhone`

### MAP-GAP-SRC-02
English:
`Official guidance on route modes and feature availability.`

Thai:
`คำแนะนำทางการเกี่ยวกับโหมดเส้นทางและความพร้อมใช้งานของฟีเจอร์`

### MAP-GAP-SRC-03
English:
`Official download limits and unavailable offline route modes.`

Thai:
`ข้อจำกัดการดาวน์โหลดและโหมดเส้นทางที่ใช้แบบ offline ไม่ได้ตามข้อมูลทางการ`

### MAP-GAP-SRC-04
English:
`Government decision and the security conditions that apply before data export.`

Thai:
`มติของรัฐบาลและเงื่อนไขด้านความปลอดภัยที่ใช้ก่อนการส่งออกข้อมูล`

### MAP-GAP-SRC-05
English:
`Official route, navigation and public-transport feature overview.`

Thai:
`ภาพรวมทางการของฟีเจอร์เส้นทาง การนำทาง และขนส่งสาธารณะ`

---

# 3. `maps.html` — 7 ambiguous app-store nodes resolved

The existing single approved summary:

`Official app store pages — Naver Map / KakaoMap / Google Maps`

must not be collapsed into one source node. Preserve the existing seven-node HTML structure and use the following exact mapping.

## MAP-AMB-01 — group title
English:
`Official app store pages`

Thai:
`หน้าร้านแอปทางการ`

## MAP-AMB-02 — link
English:
`Naver Map for iPhone`

Thai:
`Naver Map สำหรับ iPhone`

## MAP-AMB-03 — link
English:
`Naver Map for Android`

Thai:
`Naver Map สำหรับ Android`

## MAP-AMB-04 — link
English:
`KakaoMap for iPhone`

Thai:
`KakaoMap สำหรับ iPhone`

## MAP-AMB-05 — link
English:
`KakaoMap for Android`

Thai:
`KakaoMap สำหรับ Android`

## MAP-AMB-06 — link
English:
`Google Maps for iPhone`

Thai:
`Google Maps สำหรับ iPhone`

## MAP-AMB-07 — link
English:
`Google Maps for Android`

Thai:
`Google Maps สำหรับ Android`

---

# 4. `esim.html` — 2 missing targets

## ESIM-GAP-01 — comparison-table caption

English:
`Korea mobile options compared by traveler type, benefits, and limits.`

Thai:
`เปรียบเทียบตัวเลือกมือถือในเกาหลีตามประเภทนักเดินทาง ประโยชน์ และข้อจำกัด`

This is the `<caption>` inside the comparison table. It is separate from the infographic `<figcaption>` that is already approved.

## ESIM-GAP-02 — warning label

English:
`Important:`

Thai:
`สำคัญ:`

This applies only to the `<strong>` inside:

`Having a Korean phone number does not always mean you can complete identity verification on Korean apps.`

The existing approved Thai body remains unchanged:

`มีเบอร์เกาหลีไม่ได้หมายความว่าจะทำ identity verification ของแอปเกาหลีได้เสมอ`

---

# 5. `esim.html` — 12 ambiguous nodes resolved

The existing six approved Thai list items under **“5 นาทีแรกหลังลงเครื่อง”** combine each source `<strong>` and `<p>` into one line.

Do not rewrite those six approved Thai sentences. Split each existing approved sentence across the original two source nodes as follows.

## ESIM-AMB-01A — Step 1 `<strong>`
English:
`Make sure the eSIM line is turned on.`

Thai:
`เช็กว่า eSIM line เปิดอยู่`

## ESIM-AMB-01B — Step 1 `<p>`
English:
`The profile may already be installed without being active as a usable line.`

Thai:
`Profile อาจติดตั้งแล้วแต่ยังไม่ได้เปิดเป็น line ใช้งาน`

## ESIM-AMB-02A — Step 2 `<strong>`
English:
`Set the Korea eSIM as the mobile data line.`

Thai:
`ตั้ง Korea eSIM เป็น mobile-data line`

## ESIM-AMB-02B — Step 2 `<p>`
English:
`Installing an eSIM does not automatically guarantee that the phone is using it for data.`

Thai:
`การติดตั้งไม่ได้รับประกันว่าโทรศัพท์กำลังใช้มันสำหรับ data`

## ESIM-AMB-03A — Step 3 `<strong>`
English:
`Turn on data roaming for the travel eSIM if the provider requires it.`

Thai:
`เปิด data roaming สำหรับ travel eSIM ถ้า provider กำหนด`

## ESIM-AMB-03B — Step 3 `<p>`
English:
`This setting is common with international travel eSIMs, but follow the selected provider's instructions.`

Thai:
`เป็น setting ที่พบบ่อยกับ international eSIM แต่ให้ทำตาม provider`

## ESIM-AMB-04A — Step 4 `<strong>`
English:
`Make sure the plan's activation period has actually started.`

Thai:
`เช็กว่า activation period เริ่มแล้ว`

## ESIM-AMB-04B — Step 4 `<p>`
English:
`A correctly installed profile may still have no service if the plan is not yet active.`

Thai:
`Profile ถูกต้องอาจยังไม่มี service หากแพ็กยังไม่ active`

## ESIM-AMB-05A — Step 5 `<strong>`
English:
`Open a real webpage or map.`

Thai:
`เปิดหน้าเว็บหรือ map จริง`

## ESIM-AMB-05B — Step 5 `<p>`
English:
`Do not rely only on seeing LTE or 5G in the status bar.`

Thai:
`อย่าเชื่อแค่ LTE หรือ 5G ใน status bar`

## ESIM-AMB-06A — Step 6 `<strong>`
English:
`Keep the airport Wi-Fi until the connection is proven.`

Thai:
`เก็บ airport Wi-Fi ไว้จนพิสูจน์ว่า connection ใช้ได้`

## ESIM-AMB-06B — Step 6 `<p>`
English:
`If something is wrong, you still have internet access to read the setup instructions or contact support.`

Thai:
`ถ้ามีปัญหายังอ่าน instruction หรือติดต่อ support ได้`

---

# 6. Protection / implementation boundary

This supplement changes only the reported gap and node-mapping targets.

Do not change:

- ATM approved wording
- Online Payments approved wording
- any other Apps / Maps / eSIM approved wording
- facts / numbers / dates / recommendation strength
- href / affiliate / tracking
- images / srcset
- class / id / data-*
- schema structure
- common header / navigation / footer / `common.js` / shared `style.css`
- eSIM embedded-English visual assets

No new contextual Thai links are authorized.

---

# 7. Expected reconciliation after approval

Expected D Utility result:

- Source targets: **937**
- Mapped: **937**
- Unmapped: **0**
- Orphan: **0**
- Ambiguous: **0**

Per page expected blocker state:

- ATM: 0 / 0 / 0
- Online Payments: 0 / 0 / 0
- Apps: 0 / 0 / 0
- Maps: 0 / 0 / 0
- eSIM: 0 / 0 / 0

After user approval, this supplement becomes:

**APPROVED PUBLIC COPY — CONTENT LOCKED — GAP CORRECTION SUPPLEMENT**

Only then may Codex rerun the D 5-page reconciliation and, if the Gate is zero, continue exact implementation → QA → feature branch commit/push.

main / Production / Final Integration remain unauthorized.
