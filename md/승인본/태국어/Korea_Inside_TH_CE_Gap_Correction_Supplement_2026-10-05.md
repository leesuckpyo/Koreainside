# Korea Inside — Thai C+E Gap Correction Supplement — 2026-10-05

**Status:** LOCALIZATION REVIEW — GAP CORRECTION ONLY  
**Scope:** C + E 10-page preflight reconciliation blockers only  
**Purpose:** Resolve the 10 unmapped Thai values and 1 ambiguous mapping reported before implementation.  
**Rule:** This supplement does not reopen any other APPROVED / CONTENT LOCKED wording. No facts, numbers, recommendations, URLs, HTML structure, affiliate/tracking, schema structure, or English public copy are changed by this supplement.

---

# 1. Unmapped Thai values — 10 targets

## C — `foreign-credit-cards-korea.html`

### GAP-C-01
- Source line/context: line 195
- Element: caption
- English:
  `How foreign cards behave in common Korea travel situations.`
- Thai:
  `บัตรที่ออกต่างประเทศใช้งานได้อย่างไรในสถานการณ์ที่พบบ่อยระหว่างเที่ยวเกาหลี`

### GAP-C-02
- Source line/context: line 333
- Element: description after source link
- English:
  `explains the March 2026 rollout for Climate Cards and single-journey tickets.`
- Thai:
  `อธิบายการเริ่มใช้ในเดือนมีนาคม 2026 สำหรับ Climate Card และตั๋วโดยสารเที่ยวเดียว`

### GAP-C-03
- Source line/context: line 334
- Element: description after source link
- English:
  `explains the currency, rate and fee information that should appear when DCC is offered.`
- Thai:
  `อธิบายข้อมูลสกุลเงิน อัตราแลกเปลี่ยน และค่าธรรมเนียมที่ควรแสดงเมื่อมีการเสนอ DCC`

---

## E — `best-esim-for-korea.html`

### GAP-E-01
- Source line/context: line 307
- Element: caption
- English:
  `International travel eSIM plans for South Korea. Prices are omitted because plan selection, currency, and promotions can change.`
- Thai:
  `แพ็กเกจ eSIM สำหรับการเดินทางระหว่างประเทศในเกาหลีใต้ ไม่แสดงราคาเนื่องจากตัวเลือกแพ็กเกจ สกุลเงิน และโปรโมชันอาจเปลี่ยนแปลงได้`

---

## E — `korea-esim-with-phone-number.html`

### GAP-E-02
- Source line/context: line 222
- Element: H2
- English:
  `Do you need to receive calls and texts, or send them too?`
- Thai:
  `คุณต้องการแค่รับสายและข้อความ หรือจำเป็นต้องโทรออกและส่งข้อความด้วย?`

### GAP-E-03
- Source line/context: line 225
- Element: paragraph
- English:
  `Some travelers only need a Korean number so a hotel, restaurant or driver can reach them. Others need to make domestic calls or send SMS themselves. Those are different requirements, and they can lead to different products even within the same carrier.`
- Thai:
  `นักเดินทางบางคนต้องการเบอร์เกาหลีเพียงเพื่อให้โรงแรม ร้านอาหาร หรือคนขับรถติดต่อกลับได้ แต่อีกบางคนต้องโทรภายในประเทศหรือส่ง SMS เอง ความต้องการสองแบบนี้ต่างกัน และอาจทำให้ต้องเลือกคนละผลิตภัณฑ์แม้จะเป็นผู้ให้บริการรายเดียวกัน`

### GAP-E-04
- Source line/context: line 226
- Element: paragraph
- English:
  `Incoming service may be available on a product that does not offer outgoing calls or texts. Outgoing use can require a voice-capable plan, a separate balance or an additional passport or entry check.`
- Thai:
  `บางแพ็กเกจอาจรับสายหรือข้อความเข้าได้แม้จะโทรออกหรือส่งข้อความไม่ได้ ส่วนการโทรออกหรือส่งข้อความอาจต้องใช้แพ็กเกจที่รองรับเสียง ยอดคงเหลือแยกต่างหาก หรือการตรวจหนังสือเดินทางหรือข้อมูลการเข้าประเทศเพิ่มเติม`

### GAP-E-05
- Source line/context: line 227
- Element: paragraph
- English:
  `Before comparing SK Telecom, KT and LG U+, decide whether receiving is enough or whether you genuinely need to call and text from the Korean number. That single distinction removes a lot of unnecessary complexity.`
- Thai:
  `ก่อนเปรียบเทียบ SK Telecom, KT และ LG U+ ให้ตัดสินใจก่อนว่าแค่รับสายและข้อความก็เพียงพอหรือไม่ หรือคุณจำเป็นต้องโทรและส่งข้อความจากเบอร์เกาหลีจริง ๆ การแยกสองกรณีนี้ตั้งแต่แรกช่วยลดความซับซ้อนที่ไม่จำเป็นได้มาก`

### GAP-E-06
- Source line/context: line 279
- Element: caption
- English:
  `Korean carrier tourist eSIM conditions. Confirm the selected product again before payment.`
- Thai:
  `เงื่อนไข eSIM สำหรับนักท่องเที่ยวของผู้ให้บริการเครือข่ายเกาหลี ตรวจสอบผลิตภัณฑ์ที่เลือกอีกครั้งก่อนชำระเงิน`

---

## E — `taste-korea.html`

### GAP-E-07
- Source line/context: line 810
- Element: caption
- English:
  `How the Seoul food neighborhoods differ`
- Thai:
  `ย่านอาหารในโซลแตกต่างกันอย่างไร`

---

# 2. Ambiguous mapping correction — 1 source group

## C — `foreign-credit-cards-korea.html` line 336

The existing single approved wording:

`Apple — ประเทศ/ภูมิภาค Apple Pay และข้อกำหนด merchant/network/NFC`

must not be mapped as one collapsed string.

For this source group only, supersede that single ambiguous mapping with the following exact 4-node mapping while preserving the existing four-node HTML structure.

### AMB-C-01-A — link 1
- English:
  `Apple: countries and regions that support Apple Pay`
- Thai:
  `Apple: ประเทศและภูมิภาคที่รองรับ Apple Pay`

### AMB-C-01-B — connecting description
- English:
  `includes South Korea, while`
- Thai:
  `มีเกาหลีใต้อยู่ในรายการ ขณะที่`

### AMB-C-01-C — link 2
- English:
  `Apple's merchant guidance`
- Thai:
  `คำแนะนำของ Apple สำหรับผู้ค้า`

### AMB-C-01-D — trailing description
- English:
  ` explains the card, network and NFC-terminal requirements.`
- Thai:
  `อธิบายข้อกำหนดเกี่ยวกับบัตร เครือข่าย และเครื่องรับชำระเงินแบบ NFC`

Implementation note: preserve the source node boundaries and existing HTML whitespace behavior. Do not merge the two links and two description nodes into one node.

---

# 3. Expected reconciliation result after approval

This supplement is intended to resolve only the reported blockers:

- unmapped: 10 → 0
- orphan: 0 → 0
- ambiguous: 1 → 0

All other C + E Approved / CONTENT LOCKED wording remains unchanged.

No implementation, Git, feature-branch commit/push, main integration, Production, or Final Integration is authorized by this review supplement alone.
