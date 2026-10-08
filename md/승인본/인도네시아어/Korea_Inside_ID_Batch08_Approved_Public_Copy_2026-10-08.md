# Korea Inside — Indonesian Batch 08 — Approved Public Copy

**Date:** 2026-10-08  
**Status:** **APPROVED PUBLIC COPY — CONTENT LOCKED**  
**User approval:** `다음 진행` (2026-10-08), following the 5-page Batch 08 Localization Review  
**Single wording Source of Truth:** **THIS FILE ONLY** for the approved 23 exact corrections plus 44 terminology occurrences  
**Source branch checkpoint (pre-implementation):** `21e527af0e8f2dfec631b7444264b1b14c9fa6d2` (feature branch only; independently rechecked)  
**Branch examined:** `id-localization-2026-10-06` (read only)  
**Scope:** exactly five existing Indonesian working-copy HTML files, pages 36–40.  
**Approved change model:** exactly 23 page-specific FROM → TO corrections followed by 44 exact case-sensitive airport-arrival terminology occurrences, as enumerated below. All other first-pass wording remains locked; no full retranslation.  
**Purpose:** make the existing first-pass Indonesian useful and natural without changing facts, recommendations, page roles, HTML structure, or affiliate/tracking.

## 1. Source fingerprints and structural baseline

| Page | English source Git blob | Indonesian source Git blob | Source H1/H2/H3 (EN=ID) | Page-specific corrections | Terminology occurrences |
|---|---|---|---:|---:|---:|
| `airport.html` | `38abb5c83f94257e0bf57f7858b9f2dcb57b0d28` | `b93bb6f8a14ce82023af484d82e4ea6a54b021ad` | 1/8/18 | 4 | 11 |
| `arrival.html` | `4b4044fa17c691f57e604db86f2244dd92208e15` | `f6ce78f049c1426e86430b91f63b1f785bee3a5b` | 1/7/9 | 4 | 18 |
| `airport-transfer.html` | `00a343484802d573cc8785f47d2aca12e693f589` | `74526f4b7e9a5e4c8169be642acdbbc1a5d32f95` | 1/11/10 | 4 | 3 |
| `arex.html` | `d80977d569e855e0dfe76e4583bf7073fb2149ec` | `2f6ba8756adb24235a14dcda0345d5da558606cc` | 1/14/31 | 4 | 8 |
| `airport-bus.html` | `93021120a59a729770309a8d8e7015aa0e6d8fbd` | `23f4c3a1ad465f6e91466b84aee0054f55f2a037` | 1/10/26 | 7 | 4 |
| **Total** | — | — | — | **23** | **44** |

HTML tag order and element counts matched 5/5. The total `href` attributes matched by sequence after normalizing only the legitimate `../` folder-depth prefix (97, 85, 89, 99, 101 respectively). The analogous image/script source references also matched (4, 6, 3, 9, 10 respectively). This is a **structural baseline**, not an assertion that all full-body semantics were audited from scratch. All five Indonesian copies currently have `lang="en"`; preserve that actual current state until the authorized final Technical Closure, including their current canonical/hreflang/sitemap arrangements.

### Editorial / SEO scope

Existing Indonesia SEO Research Batch08 is marked COMPLETE in the valid Indonesia SEO Research Handover. The exact SEO Research MD was not present among the available source files for this review; the research must NOT be re-created, and this review does not claim to replace it. The Handover marks all five pages P1 and says:

- Entry information must distinguish nationality/passport and actual visa, K-ETA, residence and e-Arrival Card status. Do **not** claim Indonesians are uniformly visa-exempt or K-ETA-exempt.
- K-ETA and e-Arrival Card are different procedures; an official e-Arrival Card is free and can be submitted in the three days before arrival when applicable.
- Timetables, fares and airport-terminal procedures are time-sensitive. Do not silently update them during a language review.

Reference-point spot checks on 2026-10-08: the official e-Arrival Card service states **no fee** and **within 3 days before arrival**, with a status navigator; the official K-ETA notice states that the *existing eligible temporary-exemption group* is extended to **2026-12-31**, not that all nationalities qualify; official Incheon Airport pages confirm T1 bus tickets at arrivals level and T2 bus tickets at B1; Airport Limousine Co. confirms the T2 pre-ticket rule effective **2026-03-05**. Existing English/Indonesian factual wording is preserved in this approved copy.

Reference URLs (verification only; do not add new public source blocks):
- `https://www.e-arrivalcard.go.kr/`
- `https://www.k-eta.go.kr/portal/board/viewboarddetail.do?bbsSn=299707&locale=EN`
- `https://www.airport.kr/ap_en/1502/subview.do`
- `https://www.airport.kr/ap_en/1503/subview.do`
- `https://airportlimousine.co.kr/en/`

### Lock and version control

This is the final Approved MD for Batch 08; no additional supplement or separate approval file is required. It does **not** reapprove or reopen Batches 01–07. Before editing, Codex must confirm the branch and all ten EN/ID source Git blob identifiers in §1, compare them with the actual company-PC files and preserve all independent user working-tree changes. If a requested phrase is missing, occurs more than the specified count, overlaps a user edit, or touches protected code, **STOP for the affected target and report the exact conflict**; do not improvise a new phrase or replace the entire file.

The current observed repository-level Localization Standard is **ACTIVE v2.0**, while the latest Handover lists **v2.1** without its full body. For this batch, use the actual ACTIVE local Standard and the higher-level Master/Navigation/explicit user controls; if a real incompatible newer Standard exists locally, report the specific conflict before applying affected items. Do not modify either Standard as part of this batch.

## 2. Cross-page exact terminology normalization — T08-01

The existing first-pass Indonesian repeatedly uses the English-calque `aula kedatangan` for **arrival hall**. Adopt the natural Indonesian `area kedatangan` consistently across this Batch, including visible text, meta descriptions, headings, image alt, accessible labels, and already-present user-facing FAQ JSON-LD strings.

**Mechanism:** exact **case-sensitive** substring replacements in these five specific `id/` files only. Do **not** perform language-wide or repository-wide replacement. Do not modify tag names, HTML attributes other than the language-bearing attribute values explicitly containing these phrases, classes, IDs, data-* logic, URLs, scripts' executable logic, or schema structure. Use the exact source-count manifest below.

| Filename | `Aula Kedatangan` → `Area Kedatangan` | `aula kedatangan` → `area kedatangan` | `aula umum` → `area umum` | Total |
|---|---:|---:|---:|---:|
| `id/airport.html` | 3 | 8 | 0 | 11 |
| `id/arrival.html` | 5 | 13 | 0 | 18 |
| `id/airport-transfer.html` | 0 | 3 | 0 | 3 |
| `id/arex.html` | 0 | 7 | 1 | 8 |
| `id/airport-bus.html` | 1 | 2 | 1 | 4 |
| **TOTAL** | **9** | **33** | **2** | **44** |

Any count mismatch or a match inside non-user-facing technical code means **STOP and report the exact location**. The existing semantic distinctions remain: `arrival.html` = flight-to-customs procedure; `airport.html` = public arrival-area first 30 minutes. Do not merge pages or change URLs.

## 3. PAGE 36 — `id/airport.html`

**Page role:** After immigration/baggage/customs, what to prepare during the first 30 minutes in the public arrival area. Avoid duplicating the immigration process on `arrival.html`.

### A08-01 — H2, original line 161
FROM:
```text
Gunakan Peta Terminal Tempat Anda Benar-Benar Tiba
```
TO:
```text
Periksa peta terminal kedatangan Anda
```
Reason: removes redundant literal wording and tells the reader what to do.

### A08-02 — Map-setting paragraph, original line 173
FROM:
```text
Peta mungkin terbuka pada terminal atau lantai yang berbeda. Sesuaikan kedua pengaturan dengan lokasi kedatangan Anda sebelum mengikutinya.
```
TO:
```text
Peta resmi bisa terbuka pada terminal atau lantai yang berbeda dari lokasi Anda. Pilih terminal dan lantai kedatangan yang sesuai sebelum mengikuti petunjuknya.
```
Reason: keep both location controls explicit.

### A08-03 — H3, original line 296
FROM:
```text
Anda Tiba Larut Malam
```
TO:
```text
Jika Anda tiba larut malam
```
Reason: the subsection is a conditional recovery branch, not a statement about every reader.

### A08-04 — Late arrival decision paragraph, original line 298
FROM:
```text
Kedatangan larut malam mengubah urutan pertimbangan: periksa jadwal resmi terkini untuk terminal Anda sebelum berjalan menuju rute siang hari hanya berdasarkan ingatan. Bus malam mungkin masih tersedia. Jika tidak, gunakan pangkalan taksi resmi atau petunjuk pertemuan yang telah disimpan untuk penjemputan yang sudah dipesan.
```
TO:
```text
Jika tiba larut malam, cek dulu jadwal resmi terbaru untuk terminal Anda. Jangan langsung menuju kereta atau bus hanya berdasarkan jadwal siang hari yang Anda ingat. Bus malam mungkin masih beroperasi. Jika tidak, gunakan pangkalan taksi resmi atau petunjuk titik temu yang sudah disimpan untuk penjemputan yang dipesan sebelumnya.
```
Reason: simplify a literal construction while preserving the fallback decision and not inventing a night-bus departure.

**Protected:** Wi-Fi vs cellular-data test, Korean-address backup, T-money conditional use, formal taxi rank, T1/T2 layout, paid/unpaid access distinctions, luggage and late arrival.

## 4. PAGE 37 — `id/arrival.html`

**Page role:** From disembarkation through immigration, baggage collection, customs, and the public arrival area; transfers follow another route.

### R08-01 — H1, original line 147
FROM:
```text
Apa yang Perlu Dilakukan Setelah Mendarat di Incheon Airport
```
TO:
```text
Apa yang perlu dilakukan setelah mendarat di Bandara Incheon?
```
Reason: a natural Indonesian question without changing the purpose.

### R08-02 — H2, original line 162
FROM:
```text
Pastikan Terminal yang Benar-Benar Digunakan Penerbangan Anda
```
TO:
```text
Pastikan terminal kedatangan penerbangan Anda
```
Reason: remove repeated emphasis and sharpen the necessary check.

### R08-03 — Intro sentence, original line 148; replace this exact sentence within the existing paragraph only
FROM:
```text
Kedatangan di Incheon Airport cukup mudah jika Anda tahu petunjuk mana yang perlu diikuti.
```
TO:
```text
Setelah mendarat di Incheon Airport, proses kedatangan lebih mudah diikuti jika Anda tahu papan petunjuk mana yang harus dicari.
```
Reason: state the practical decision in natural Indonesian, without adding an entry-right assertion.

### R08-04 — H2, original line 298
FROM:
```text
Sudah Selesai — Apa Langkah Berikutnya?
```
TO:
```text
Sudah melewati bea cukai? Ini langkah berikutnya
```
Reason: the decision stage is after the arrivals process, not a generic 'finished'.

**Protected:** nationality- and status-dependent entry requirements, distinction between visa/K-ETA/e-Arrival Card, official free e-Arrival Card within three days, no blanket Indonesian exemption claim, missing/damaged baggage procedure, transfers vs entry, T1/T2 separated, existing FAQ visible/JSON-LD parity.

## 5. PAGE 38 — `id/airport-transfer.html`

**Page role:** Door-to-door comparison: AREX, airport bus, official taxi, Call Van, and pre-booked private transfer. Never rank the fastest train as automatically the easiest hotel arrival.

### TR08-01 — H2, original line 308
FROM:
```text
Taksi: Langsung ke Pintu Tujuan, Sampai Bagasi Memerlukan Kendaraan Berbeda
```
TO:
```text
Taksi: langsung ke tujuan, tetapi bagasi bisa menentukan jenis kendaraan
```
Reason: removes an unnatural calque while preserving taxi capacity and suitcase constraints.

### TR08-02 — H2, original line 340
FROM:
```text
Kedatangan Malam Bergantung pada Waktu Anda Keluar dari Terminal
```
TO:
```text
Kedatangan larut malam: perhitungkan waktu keluar dari terminal
```
Reason: the relevant clock is terminal exit time, not wheels-down time.

### TR08-03 — Exact phrase within paragraph, original line 378
FROM:
```text
kilometer terakhir sambil membawa bagasi
```
TO:
```text
bagian terakhir perjalanan menuju hotel sambil membawa bagasi
```
Reason: avoids treating an idiomatic 'last kilometre' as a measured actual distance. Keep Call Van/private-transfer trade-off intact.

### TR08-04 — Closing decision paragraph, original line 379
FROM:
```text
Perjalanan awal paling cepat tidak selalu membuat kedatangan paling mudah. Perjalanan yang penting adalah yang berakhir di pintu hotel.
```
TO:
```text
Pilihan yang paling cepat pada tahap awal belum tentu paling mudah sampai ke hotel. Yang perlu dibandingkan adalah seluruh perjalanan hingga pintu hotel.
```
Reason: concrete door-to-door comparison; no new price or timing claim.

**Protected:** exact fares and units as published in current EN, late-night taxi surcharge bands, airport bus ticket/payment distinctions, actual terminal/stand identifiers, Call Van vs private transfer, travel-time statements, luggage and vehicle capacities, affiliate links.

## 6. PAGE 39 — `id/arex.html`

**Page role:** Correct choice between Express and All-Stop by final destination, station interchange, tickets, bags and onward journey.

### X08-01 — H2, original line 193
FROM:
```text
Berapa Banyak Waktu yang Benar-Benar Dihemat Express?
```
TO:
```text
Berapa menit lebih cepat AREX Express?
```
Reason: directly frames the scheduled-time comparison.

### X08-02 — H2, original line 330
FROM:
```text
Seoul Station Sering Kali Baru Pertengahan Perjalanan
```
TO:
```text
Seoul Station sering kali belum menjadi akhir perjalanan
```
Reason: makes the subway/KTX/hotel transfer consequence explicit and natural.

### X08-03 — H2, original line 365
FROM:
```text
Tiba Larut Malam? Hitung Mundur dari Keberangkatan Kereta, Bukan Waktu Mendarat
```
TO:
```text
Tiba larut malam? Perhitungkan jadwal kereta, bukan hanya waktu mendarat
```
Reason: avoids literal 'count backward' wording while preserving the late-night warning.

### X08-04 — Destination-choice paragraph, original line 247
FROM:
```text
Pertanyaan yang berguna bukan hanya kereta mana yang lebih cepat, tetapi di mana Anda bisa turun dari AREX dan seberapa jauh perjalanan yang masih tersisa.
```
TO:
```text
Selain melihat kereta mana yang lebih cepat, periksa stasiun tempat Anda turun dari AREX dan jarak yang masih harus ditempuh ke tujuan.
```
Reason: more concrete destination decision without changing the recommendation.

**Protected:** Express vs All-Stop stop patterns, AREX train times and fares, T-money only for All-Stop, QR/seat/ticket terms, luggage limits, bank-card acceptance qualifications, Seoul Station physical transfer friction, exact destination-to-hotel judgments.

## 7. PAGE 40 — `id/airport-bus.html`

**Page role:** Determine a workable airport-bus route by destination/stop, operator, ticket counter, terminal, baggage and return trip; Seoul/Gyeonggi/intercity/night buses differ.

### B08-01 — H1, original line 88
FROM:
```text
Panduan Bus Incheon Airport
```
TO:
```text
Panduan bus Bandara Incheon
```
Reason: consistent natural Indonesian location phrasing; unchanged bus-guide intent.

### B08-02 — H2, original line 154
FROM:
```text
Cari Rute yang Tetap Mudah Setelah Turun
```
TO:
```text
Cari rute yang tetap praktis setelah Anda turun dari bus
```
Reason: align route choice with the final stop, not just the ride.

### B08-03 — H3, original line 256
FROM:
```text
Pembayaran Mengikuti Aturan Operator, Bukan Satu Aturan Bandara
```
TO:
```text
Cara membayar tiket bergantung pada operator bus
```
Reason: simpler, actionable distinction; preserves carrier-specific rules.

### B08-04 — H3, original line 234
FROM:
```text
Satu aturan T2 yang perlu diketahui pada 2026
```
TO:
```text
Aturan tiket bus Terminal 2 yang perlu diketahui pada 2026
```
Reason: makes the 2026 advance-ticket rule identifiable without inventing new conditions.

### B08-05 — H3, original line 294
FROM:
```text
Tiket salah
```
TO:
```text
Tiket yang dibeli tidak sesuai
```
Reason: more natural troubleshooting label.

### B08-06 — H3, original line 306
FROM:
```text
Anda mencari terminal yang salah
```
TO:
```text
Anda berada di terminal yang salah
```
Reason: corrects the practical situation, not an online search action.

### B08-07 — H3, original line 314
FROM:
```text
Bus terakhir yang sesuai sudah tidak ada
```
TO:
```text
Tidak ada lagi bus yang sesuai
```
Reason: natural late-service fallback without inventing a departure time.

**Protected:** which bus systems/operators serve Seoul vs Gyeonggi/intercity, T1/T2 purchase locations, T2 advance-purchase rules, the operator-specific luggage allowances and payment methods, free terminal shuttles vs paid city buses, separate return stop, late-night fallback, all fare and route particulars.

## 8. Approved exact-implementation and QA contract

1. This file is the **APPROVED PUBLIC COPY — CONTENT LOCKED**; it is the **only** Batch 08 wording authority for the expressly listed targets. Do not also apply a separate Review MD, and do not modify unlisted user-visible wording.
2. The consolidation is **complete in this file**. The 23 FROM/TO pairs and 44 case-sensitive terminology substitutions are final. Codex must not translate, paraphrase, shorten, extend, merge, or otherwise editorially change them.
3. Codex must check branch, all 10 source blob fingerprints and working-tree status immediately before any implementation. If source differs, compare the exact relevant targets and STOP on user-visible drift. Do not overwrite user edits.
4. Apply the **23 unique exact targets** first (each FROM occurs precisely once in the given source file), then the **44 case-sensitive terminology occurrences**. No target overlaps the terminology source strings in this manifest.
5. Preserve document tag order, text-node boundaries where practicable, section sequence, class/id/data-*, image paths/srcset, external/affiliate URL and tracking, all numbers/units/dates/times/recommendation judgments and functional scripts.
6. Current `lang="en"` state is intentional for this interim implementation phase. No `lang`/canonical/hreflang/sitemap/language-switcher/internal-link closure, main merge or Vercel until whole-language Technical Closure and explicit Production approval.
7. Technical QA must prove 23/23 unique changes + 44/44 counted occurrences, no old target residue, no unapproved text changes, 5/5 tag structure parity, exact FAQ/schema integrity, `git diff --check` PASS. Any pre-existing planned source differences must be reported, not automatically corrected.
8. Protect the latest user local working-tree changes, untracked/deleted files, earlier Indonesian approved material, Vietnamese SEO MDs and all common global header/footer/navigation/`common.js`/shared `style.css`.
9. The user’s `다음 진행` follows this review and authorizes the ordinary Batch 08 feature-branch exact-implementation, static/staged QA, and scoped Git checkpoint procedure. **After QA PASS**, stage only the five in-scope HTML files and this final approved MD, verify the staged manifest, and commit/push only `id-localization-2026-10-06`. No main merge, Vercel, Production, Technical Closure or partial release.
10. A verified v2.1 common Localization Standard text was not available; the current GitHub repository displayed ACTIVE **v2.0** while the 2026-10-08 Handover reports **v2.1**. Preserve this known discrepancy for local source-of-truth confirmation, without rewriting Standards or reopening closed Batches.

**END — BATCH 08 APPROVED PUBLIC COPY / CONTENT LOCKED**
