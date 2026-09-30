# Korea Inside Room Handover — 2026-09-29

- File: `Korea_Inside_Room_Handover_2026-09-29.md`
- Date: 2026-09-29
- Status: **ACTIVE HANDOVER — supersedes earlier room handover for current state**
- Purpose: Move to a new chat without redoing completed audits or losing the Spanish/Japanese localization state.

---

# 1. Governing documents already established

Use the active/approved versions already confirmed in the previous room:

1. `Korea_Inside_Public_Content_Master_Standard.md` — ACTIVE MASTER v1.4, Effective 2026-09-26
2. `Korea_Inside_Navigation_Hub_Architecture_Standard.md` — ACTIVE v1.1, Effective 2026-09-26
3. `Korea_Inside_Language_Localization_Standard.md` — ACTIVE specialized v2.1, Effective 2026-09-26
4. `Korea_Inside_Japanese_Localization_Standard.md` — v1.1
5. `Korea_Inside_Travel_Guide_Production_Playbook.md` — v1.1
6. `Korea_Inside_ACTIVE_DOCUMENT_INDEX_2026-09-26.md`

Do not reread all Standards in the new room if these exact files/versions have not changed. The user wants continuity, not repeated initialization work.

Priority remains:
current user instruction → Public Content Master Standard → Navigation Hub Architecture Standard → applicable Specialized Standard / Production Playbook → language Standard → Active Family Design Standard → latest Handover → Inventory → Research / Approved Copy → current Production HTML

---

# 2. Core operating rules to preserve

- Final public wording/editorial judgment belongs to user + ChatGPT.
- Codex is implementation / technical QA / Git / Production, not content strategy.
- ChatGPT should directly audit localization wording.
- Do not reopen DONE LOCKED / CONTENT LOCKED pages without a concrete defect or explicit user instruction.
- Protect user working-tree changes.
- Never use `git add .`, `git add -A`, `git restore`, `git reset`, `git clean` without explicit user approval.
- Common header/nav/footer/common.js/shared style.css are protected unless specifically approved.
- Localization preserves facts, numbers, recommendation logic, HTML structure, IDs/classes/data attributes, affiliate/tracking, images, CSS/JS/schema structure.
- Localize visible copy naturally; do not translate literally.
- Humanization = practical traveler judgment, not sentimental writing.
- Do not repeatedly re-audit pages already audited and passed. Fix only concrete remaining defects.
- Long multilingual audits must be split into smaller batches to prevent tool/context errors.

---

# 3. French status

**French = DONE LOCKED**

French localization:
- 58 pages completed
- direct GPT editorial audit completed
- Production deployed
- final technical QA completed
- do not reopen unless there is a concrete defect or explicit user instruction

French completed deployment baseline before Spanish work:
`30f8991e915ebf72c383f8e9c87fb56cdf874d00`

---

# 4. Spanish status

## 4.1 Content

Spanish direct GPT audit completed.

Completed process:
- 58-page direct editorial audit
- 6-page structural/source sync
- remaining 37-page exact correction batch
- final 12-page micro-fix
- final content judgment: **Spanish 58/58 CONTENT PASS**

Do **not** redo the Spanish 58-page content audit.

## 4.2 Spanish Production deployment

Commit:
`4300b888eda34c445dd955f5419050ac2d190df3`

Commit message:
`Finalize Spanish localization`

Push:
- `main → origin/main` success
- HEAD = origin/main
- divergence `0/0`

Vercel Production:
- deployment ID: `E6WWotxh532zSqzGKARo5WArFxNA`
- GitHub deployment record: `6727313212`
- status: READY / success
- environment: Production
- Production Git SHA matches commit

Production QA:
- Spanish HTTP 200: `58/58`
- redirect / 404 / 5xx: `0 / 0 / 0`
- modified 43 Production source match: `43/43 exact PASS`
- JSON-LD: 29 blocks, parse errors `0`
- final 12-page micro-fix: `12/12 PASS`
- OLD residue `0`
- NEW missing `0`
- Dongdaemun construction-noise residue EN/FR/ES/JA/ZH-TW: `0/0/0/0/0`
- canonical errors: `0`
- affiliate/tracking unexpected changes: `0`
- image src/srcset unexpected changes: `0`
- Production images checked: `232/232`
- final Git working tree after deployment: clean

## 4.3 Spanish remaining technical defect

Spanish is **not yet DONE LOCKED** because Production QA found existing Japanese hreflang omissions on 5 Spanish pages.

Missing `hreflang="ja"`:

1. `es/accommodation.html`
2. `es/best-area-for-families-seoul.html`
3. `es/best-area-for-first-time-visitors-seoul.html`
4. `es/best-area-for-solo-travelers-seoul.html`
5. `es/hongdae-vs-myeongdong.html`

Effects:
- Japanese sibling pages exist and return HTTP 200.
- JA → ES reciprocal links exist.
- ES pages lack JA alternate.
- On those 5 Spanish pages, the `日本語` language-switcher option is not generated.
- Language switcher: `53/58 PASS`, `5 FIX NEEDED`.

This is a technical hreflang/switcher issue only. Spanish content itself is complete.

Do not modify `common.js` merely to solve this. Intended fix: restore page-level JA alternate on the 5 affected Spanish pages, then focused QA and deploy.

The user moved on to Japanese audit before this 5-page technical fix was completed. Preserve it as an unresolved task.

---

# 5. Dongdaemun construction-noise notice

The stale Sotetsu/Dongdaemun construction-noise notice was intentionally removed from public HTML.

Production residue:
- EN 0
- FR 0
- ES 0
- JA 0
- ZH-TW 0

Do not restore it.

---

# 6. Japanese audit strategy — current active task

The user explicitly changed the Japanese direct audit from a single large run to **5 smaller batches** because large tool runs were causing errors.

Total Japanese HTML: **58**

Audit structure:
- Batch 1: 12 pages
- Batch 2: 12 pages
- Batch 3: 12 pages
- Batch 4: 11 pages
- Batch 5: 11 pages

Rules:
- English Production ↔ Japanese Production direct GPT comparison
- once a batch is complete, do not redo it from the beginning
- keep PASS pages closed
- record only concrete FIX pages
- no file editing during audit unless user explicitly changes workflow
- no stage / commit / push / deploy during audit

---

# 7. JA Audit 1/5 — COMPLETE

Detailed record:
`Korea_Inside_JA_Audit_Batch1_2026-09-29.md`

Scope — first 12 Japanese pages:

1. `accommodation.html`
2. `airport-bus.html`
3. `airport-transfer.html`
4. `airport.html`
5. `apple-pay-korea.html`
6. `apps.html`
7. `arex.html`
8. `arrival.html`
9. `best-area-for-airport-access-seoul.html`
10. `best-area-for-budget-travelers-seoul.html`
11. `best-area-for-couples-seoul.html`
12. `best-area-for-families-seoul.html`

Result: **8 PASS / 4 FIX**

PASS:
- `airport-bus.html`
- `airport-transfer.html`
- `airport.html`
- `apple-pay-korea.html`
- `apps.html`
- `arex.html`
- `arrival.html`
- `best-area-for-budget-travelers-seoul.html`

FIX:
- `accommodation.html`
- `best-area-for-airport-access-seoul.html`
- `best-area-for-couples-seoul.html`
- `best-area-for-families-seoul.html`

Approximate wording fixes: **18 locations**

Common verification:
- 12/12 English↔Japanese main-body structure aligned
- section sequence aligned
- large omission/addition 0
- JSON-LD parse errors 0
- material recommendation distortion 0
- material factual/number error 0

The FIX items are mainly Japanese naturalness/calque issues around English “strong / stronger / strongest / safest default / practical” language, especially literal `強い / 強くなる` phrasing.

Do not reopen the 8 PASS pages during later audit batches.

---

# 8. Japanese 5-batch page allocation

## JA Audit 1 — COMPLETE — 12 pages
1–12:
- accommodation
- airport-bus
- airport-transfer
- airport
- apple-pay-korea
- apps
- arex
- arrival
- best-area-for-airport-access-seoul
- best-area-for-budget-travelers-seoul
- best-area-for-couples-seoul
- best-area-for-families-seoul

## JA Audit 2 — NEXT — 12 pages
13–24:
- `best-area-for-first-time-visitors-seoul.html`
- `best-area-for-luxury-hotels-seoul.html`
- `best-area-for-nightlife-seoul.html`
- `best-area-for-shopping-seoul.html`
- `best-area-for-solo-travelers-seoul.html`
- `best-esim-for-korea.html`
- `card-declined-korea.html`
- `checklist.html`
- `dongdaemun-travel-guide.html`
- `esim.html`
- `foreign-credit-cards-korea.html`
- `gangnam-travel-guide.html`

## JA Audit 3 — 12 pages
25–36:
- `gongdeok-mapo-seoul-guide.html`
- `hongdae-travel-guide.html`
- `hongdae-vs-myeongdong.html`
- `hotels-near-gongdeok-station.html`
- `hotels-near-seoul-station.html`
- `incheon-airport-private-transfer.html`
- `index.html`
- `insadong-travel-guide.html`
- `itaewon-travel-guide.html`
- `jamsil-travel-guide.html`
- `k-beauty.html`
- `korea-atm-foreign-cards.html`

## JA Audit 4 — 11 pages
37–47:
- `korea-esim-with-phone-number.html`
- `korean-online-payments-foreigners.html`
- `lotte-world-seoul.html`
- `maps.html`
- `myeongdong-travel-guide.html`
- `payments.html`
- `rental-car.html`
- `seongsu-travel-guide.html`
- `seoul-sky-guide.html`
- `taste-korea.html`
- `taxi.html`

## JA Audit 5 — 11 pages
48–58:
- `tmoney-vs-wowpass.html`
- `tmoney.html`
- `where-to-stay-in-dongdaemun.html`
- `where-to-stay-in-gangnam.html`
- `where-to-stay-in-hongdae.html`
- `where-to-stay-in-insadong.html`
- `where-to-stay-in-itaewon.html`
- `where-to-stay-in-jamsil.html`
- `where-to-stay-in-myeongdong.html`
- `where-to-stay-in-seongsu.html`
- `wowpass.html`

---

# 9. Exact next action in the new room

**Do not repeat JA Audit 1.**

Start directly with:

## JA Audit 2/5 — 12 pages only

Compare English Production ↔ Japanese Production directly for pages 13–24.

Audit:
- title/meta
- H1/H2/H3
- visible body
- CTA / FAQ
- visible table/list copy
- JSON-LD user-facing strings where applicable
- structural/source parity
- missing/extra sections
- facts/numbers/recommendation strength
- Japanese native naturalness
- English residue/calques

At the end report:
- PASS pages
- FIX pages
- exact problem locations / reason
- structural mismatch pages separately

Do not edit files during the audit.
Do not stage/commit/push/deploy.
Do not audit pages 1–12 again.

---

# 10. New-room start instruction

After loading this Handover and the active Standards, continue with:

**“JA Audit 1/5 is already complete. Continue directly with JA Audit 2/5, pages 13–24 only. Do not re-audit Batch 1.”**
