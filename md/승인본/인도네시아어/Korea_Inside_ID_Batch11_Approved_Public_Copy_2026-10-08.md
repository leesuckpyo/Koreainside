# Korea Inside — Indonesian Batch 11 — Approved Public Copy

**Date:** 2026-10-08  
**Status:** **APPROVED PUBLIC COPY — CONTENT LOCKED**  
**User wording approval:** `다음 진행` (2026-10-08), after the 5-page Batch 11 Localization Review.  
**Single public wording Source of Truth:** THIS FILE ONLY for the approved 28 exact corrections at 32 source occurrences.  
**Scope:** Indonesian pages 51–55, exactly five existing `id/` HTML files.  
**Current branch:** `id-localization-2026-10-06` (read only).  
**Baseline HEAD:** `c3ef7af8a87b042d1d3de951c6ceb8f983349816`.  
**Research state:** Indonesian SEO research is 58/58 complete per latest project Handover; existing Batch11 research is not reproduced or redone here. The separate Batch11 SEO Research MD was not retrievable in this workspace and must be cross-checked on the company PC before final exact implementation.  
**Change model:** exact case-sensitive FROM → TO mapping in the existing Indonesian first-pass HTML; no English HTML changes proposed.  

## 1. Scope, source fingerprints, and content role

| # | Indonesian target | Current English Git blob | Current Indonesian Git blob | H1/H2/H3 (EN=ID) | P (EN=ID) | FAQ (visible=JSON-LD) | Mappings | Positions |
|---|---|---|---|---|---:|---:|---:|---:|
| 51 | `id/foreign-credit-cards-korea.html` | `dc4bd83c1d3d055055b6bb1b69c9b17db6f02d6c` | `dc8928400dc30a51165f8adfc25a96974f805ea5` | 1 / 9 / 6 | 49 | 8 | 6 | 8 |
| 52 | `id/card-declined-korea.html` | `581314f3638bf8f968bf2e34814e79d0f8739537` | `337d4a39ee345d8d827504d06fde93aa5dfd5baf` | 1 / 9 / 0 | 60 | 8 | 5 | 5 |
| 53 | `id/korean-online-payments-foreigners.html` | `86d74cc91ef401217b0b37a77ffdb5717119a466` | `9506ea41c4e56e06b3df65b406bccae47ac3ed81` | 1 / 9 / 0 | 55 | 8 | 5 | 5 |
| 54 | `id/korea-atm-foreign-cards.html` | `af3a6d17aadecd517ca5cb6e5f331d91c0f8fe1c` | `b6edf65323629e7025e38a6d39afb5ac03234d9e` | 1 / 9 / 0 | 57 | 8 | 5 | 5 |
| 55 | `id/apple-pay-korea.html` | `c251aad5f8c314a9bebe060266f352b37cc65805` | `5ce1ead50a488c73628d37739d475ad13621bdc4` | 1 / 11 / 0 | 76 | 8 | 7 | 9 |
| **TOTAL** | **5 HTML** | — | — | — | — | **40** | **28** | **32** |

Five English/Indonesian files were read on the `id-localization-2026-10-06` branch. HTML tag order, H1/H2/H3/P/FAQ counts, and `href`/`src` sequences matched in each English/Indonesian pair after only the legitimate relative directory-depth adjustment for links/assets. The Indonesian files already have `lang="id"`; current canonical/hreflang/sitemap states must be preserved for future Technical Closure (do not change them in this Batch). No claim of full page-by-page translation from scratch: first-pass wording remains protected outside the patches below.

### Scope and editorial rules

- Preserve: English factual and recommendation judgments, cross-page intent, store-versus-ATM-versus-online differences, merchant/network/issuer distinctions, real-world recovery steps, numerical facts, prices, brand names, affiliates, tracking, HTML structure, and existing links.
- Adjust only: explicitly enumerated Indonesian page language and Indonesian-market examples, including SEO meta description where listed.
- US-issued-card examples in the English global page do **not** mean that Indonesian readers must use US-issued cards; for the Indonesian version, adapt those examples to cards issued in Indonesia while preserving acceptance caveats.
- Apple Pay exception is more important: do **not** present an Indonesia-issued card as enrollable in Apple Wallet unless Apple adds Indonesian participating issuers. Apple currently does not list Indonesia in Apple Pay card-issuer regions. This is a country-of-issuance condition, **not** a claim that Korean merchants reject eligible overseas Apple Pay cards.
- Apple Pay merchant purchases, T-money transit payment, direct Apple Wallet T-money funding, and the separate MobileTmoney international-user funding route remain four distinct questions. No blanket promise of a supported card, app, network, or terminal.
- All other existing body, FAQ, title, CTA, source list, metadata, and editorial judgment remain exactly unchanged.

### Evidence and verification scope

**New current check, 2026-10-08:** Apple official participating-issuer country/region list does not include Indonesia, including the Asia-Pacific subsection. Verify again at content update time; do not mistake Apple Pay acceptance in South Korea for Apple Pay card-issuance support in Indonesia.
- Official Apple Pay supported countries/regions: `https://support.apple.com/ko-kr/102775`
- Apple Pay eligible issuer details: `https://support.apple.com/en-us/102897`
- Apple Pay merchant acceptance and network conditions: `https://support.apple.com/id-id/120364`
- MobileTmoney App Store official listing: `https://apps.apple.com/kr/app/모바일티머니/id1470361790`
- Approved existing English/Indonesian `apple-pay-korea.html` already contains supported MobileTmoney foreign-user card networks: Mastercard / American Express / UnionPay / JCB; Visa is not listed for that route. These facts are preserved, not recast as new network claims.

**Research scope:** No new Indonesian SERP, traffic, ranking, keyword volume, or competitor count is asserted here. This document is a localization review against the existing English source and first-pass Indonesian copies, with one independently verified Indonesia-specific Apple issuer caveat. The local Batch11 research MD must be checked, not regenerated.

## 2. Exact wording corrections

**Execution order:** For each HTML file below, apply each numbered mapping in order, to only its specified file. Use case-sensitive exact text. Source occurrence count is part of the contract. FAQ questions/answers listed with count 2 must modify visible FAQ and JSON-LD in the same file without changing the schema structure. A count mismatch or source drift means STOP for that file; never guess the intended new copy.

### Page 51 — `id/foreign-credit-cards-korea.html`

**Search / decision role:** Indonesian-bank card vs generic/US-issued foreign card; acceptance is issuer + network + terminal, not a guarantee.  
**English Git blob:** `dc4bd83c1d3d055055b6bb1b69c9b17db6f02d6c`  
**Indonesian Git blob:** `dc8928400dc30a51165f8adfc25a96974f805ea5`  
**Targeted corrections:** 6 mapping(s), 8 exact location(s).

#### FC11-01 — Meta description; Indonesian search-intent localization

**FROM (exact; case-sensitive):**
```text
Bisakah kartu kredit asing atau kartu terbitan AS dipakai di Korea? Pelajari tempat yang menerima kartu, penyebab pembayaran gagal, dan kapan uang tunai cadangan membantu.
```
**TO (approved final copy):**
```text
Bisakah kartu kredit Indonesia dipakai di Korea? Pelajari kapan kartu asing diterima, mengapa pembayaran bisa gagal, dan kapan perlu uang tunai cadangan.
```
**Occurrences in this file:** 1.  
**Reason:** Replace the US-issued reference with the Indonesian market query, without implying universal card acceptance..

#### FC11-02 — H2: card-network logos

**FROM (exact; case-sensitive):**
```text
Apa arti logo pada kartu Anda
```
**TO (approved final copy):**
```text
Apa arti logo jaringan pembayaran pada kartu Anda?
```
**Occurrences in this file:** 1.  
**Reason:** Specifies why the logo matters, while preserving Visa/Mastercard/JCB/Amex/UnionPay facts..

#### FC11-03 — H3: foreign issuer example

**FROM (exact; case-sensitive):**
```text
Kartu terbitan AS tetap merupakan kartu asing
```
**TO (approved final copy):**
```text
Kartu terbitan Indonesia tetap dianggap kartu asing di Korea
```
**Occurrences in this file:** 1.  
**Reason:** The original US market example should be adapted to the actual Indonesian audience..

#### FC11-04 — Paragraph: overseas-issued card

**FROM (exact; case-sensitive):**
```text
Kartu terbitan AS tidak mendapat perlakuan khusus di Korea. Penggunaannya tetap bergantung pada jaringan kartu, terminal tempat usaha, dan persetujuan transaksi dari penerbit. Kartu yang bisa dipakai di negara asal mungkin berhasil di satu tempat usaha Korea tetapi gagal di tempat lain.
```
**TO (approved final copy):**
```text
Kartu yang diterbitkan bank di Indonesia tetap diproses sebagai kartu luar negeri saat digunakan di Korea. Keberhasilan pembayaran bergantung pada jaringan kartu, terminal tempat usaha, serta persetujuan bank penerbit. Kartu yang biasa Anda pakai di Indonesia bisa berhasil di satu toko Korea, tetapi gagal di toko lain.
```
**Occurrences in this file:** 1.  
**Reason:** Market localization preserves the issuer/network/merchant-terminal trade-off and avoids a card-success promise..

#### FC11-05 — FAQ question, visible + JSON-LD

**FROM (exact; case-sensitive):**
```text
Bisakah saya memakai kartu kredit terbitan AS di Korea?
```
**TO (approved final copy):**
```text
Bisakah kartu kredit terbitan Indonesia dipakai di Korea?
```
**Occurrences in this file:** 2.  
**Reason:** Keep FAQ question in visible details and structured data identical..

#### FC11-06 — FAQ answer, visible + JSON-LD

**FROM (exact; case-sensitive):**
```text
Ya. Kartu terbitan AS mengikuti ketentuan dasar yang sama dengan kartu asing lainnya: jaringan kartu, terminal tempat usaha, dan penerbit kartu harus sama-sama mendukung transaksi.
```
**TO (approved final copy):**
```text
Bisa, jika jaringan kartu, terminal tempat usaha, dan bank penerbit mendukung transaksi. Kartu terbitan Indonesia tetap mengikuti ketentuan kartu asing di Korea; keberhasilan tidak dijamin di semua toko atau mesin.
```
**Occurrences in this file:** 2.  
**Reason:** Avoid implying that all Indonesian cards work everywhere; parity in FAQ and JSON-LD is mandatory..


### Page 52 — `id/card-declined-korea.html`

**Search / decision role:** Failure-first diagnostic: contactless/issuer/kiosk/online/ATM and a workable payment fallback.  
**English Git blob:** `581314f3638bf8f968bf2e34814e79d0f8739537`  
**Indonesian Git blob:** `337d4a39ee345d8d827504d06fde93aa5dfd5baf`  
**Targeted corrections:** 5 mapping(s), 5 exact location(s).

#### CD11-01 — Hero decision paragraph

**FROM (exact; case-sensitive):**
```text
Mulailah dari tempat pembayaran gagal. Pembaca nirsentuh yang tidak bereaksi, kasir dengan petugas yang menampilkan penolakan, kios layanan mandiri, dan situs web Korea merupakan masalah yang berbeda. Dengan membedakan situasinya terlebih dahulu, Anda lebih mudah menentukan langkah berikutnya.
```
**TO (approved final copy):**
```text
Mulailah dengan melihat di mana pembayaran gagal. Pembaca nirsentuh yang tidak merespons, transaksi yang ditolak di kasir, kios layanan mandiri, dan situs web Korea membutuhkan penanganan berbeda. Menentukan titik masalah terlebih dahulu akan membantu Anda memilih langkah berikutnya.
```
**Occurrences in this file:** 1.  
**Reason:** Replace the awkward cashier-with-staff phrasing with actual diagnostic choices..

#### CD11-02 — H2: issuer/merchant split

**FROM (exact; case-sensitive):**
```text
Jika kartu gagal di beberapa tempat usaha dengan petugas, periksa sisi penerbit
```
**TO (approved final copy):**
```text
Jika kartu ditolak di beberapa toko dengan kasir, periksa bank penerbit
```
**Occurrences in this file:** 1.  
**Reason:** Keeps the staffed-merchant qualifier and makes the next action clear..

#### CD11-03 — H2: online-only failure

**FROM (exact; case-sensitive):**
```text
Jika kartu bisa dipakai di toko tetapi gagal di situs Korea, telusuri sebagai masalah berbeda
```
**TO (approved final copy):**
```text
Kartu berfungsi di toko tetapi gagal di situs Korea? Periksa alur pembayaran online
```
**Occurrences in this file:** 1.  
**Reason:** Distinct online payment failure deserves a checkout-specific answer..

#### CD11-04 — H2: recover immediate payment

**FROM (exact; case-sensitive):**
```text
Selesaikan pembayaran dulu, lalu cari tahu penyebab kegagalannya
```
**TO (approved final copy):**
```text
Selesaikan transaksi dengan cara cadangan, lalu telusuri penyebab penolakan
```
**Occurrences in this file:** 1.  
**Reason:** Aligns with the existing practical steps about a second card or cash..

#### CD11-05 — Paragraph: transaction not found by issuer

**FROM (exact; case-sensitive):**
```text
Jika penerbit sama sekali tidak melihat transaksi, itu juga informasi yang berguna. Artinya, pembayaran mungkin tidak pernah sampai ke penerbit seperti yang Anda perkirakan. Terus mengubah pengaturan bank mungkin tidak menyelesaikan masalah tempat usaha atau terminal.
```
**TO (approved final copy):**
```text
Jika bank penerbit tidak menemukan catatan transaksi sama sekali, kegagalan mungkin terjadi sebelum permintaan pembayaran sampai ke bank. Mengubah pengaturan kartu berulang kali tidak akan menyelesaikan masalah pada terminal atau sistem pembayaran toko.
```
**Occurrences in this file:** 1.  
**Reason:** Separates unprocessed terminal events from true issuer authorization declines..


### Page 53 — `id/korean-online-payments-foreigners.html`

**Search / decision role:** Account verification vs global checkout vs issuer 3-D Secure authentication.  
**English Git blob:** `86d74cc91ef401217b0b37a77ffdb5717119a466`  
**Indonesian Git blob:** `9506ea41c4e56e06b3df65b406bccae47ac3ed81`  
**Targeted corrections:** 5 mapping(s), 5 exact location(s).

#### ON11-01 — H2: check checkout first

**FROM (exact; case-sensitive):**
```text
Periksa proses pembayarannya terlebih dahulu
```
**TO (approved final copy):**
```text
Periksa alur pembayaran sebelum menganggap kartu bermasalah
```
**Occurrences in this file:** 1.  
**Reason:** Makes the real first action explicit..

#### ON11-02 — H2: Korea phone verification

**FROM (exact; case-sensitive):**
```text
Nomor telepon Korea terkadang menjadi hambatan, tetapi tidak selalu
```
**TO (approved final copy):**
```text
Apakah pembayaran online selalu memerlukan nomor telepon Korea?
```
**Occurrences in this file:** 1.  
**Reason:** Preserves the answer that some services offer foreign-user alternatives..

#### ON11-03 — H2: NAVER example

**FROM (exact; case-sensitive):**
```text
NAVER menunjukkan betapa cepatnya hal ini berubah
```
**TO (approved final copy):**
```text
Verifikasi paspor NAVER membuka pilihan bagi wisatawan
```
**Occurrences in this file:** 1.  
**Reason:** Name the relevant official visitor verification capability already described in the body..

#### ON11-04 — H2: global service path

**FROM (exact; case-sensitive):**
```text
Terkadang versi global merupakan solusi sebenarnya
```
**TO (approved final copy):**
```text
Jika versi domestik gagal, periksa versi global layanan
```
**Occurrences in this file:** 1.  
**Reason:** Actionable distinction between local/domestic and international checkout..

#### ON11-05 — H2: failed payment stage

**FROM (exact; case-sensitive):**
```text
Saat pembayaran terhenti, cari tahu tahap yang sudah Anda capai
```
**TO (approved final copy):**
```text
Saat pembayaran gagal, periksa tahap mana yang terhenti
```
**Occurrences in this file:** 1.  
**Reason:** Keeps the checkout-stage diagnostic hierarchy..


### Page 54 — `id/korea-atm-foreign-cards.html`

**Search / decision role:** Find supported network/Global ATM, distinguish issuer withdrawal limits from machine failure and DCC.  
**English Git blob:** `af3a6d17aadecd517ca5cb6e5f331d91c0f8fe1c`  
**Indonesian Git blob:** `b6edf65323629e7025e38a6d39afb5ac03234d9e`  
**Targeted corrections:** 5 mapping(s), 5 exact location(s).

#### ATM11-01 — Hero paragraph: network logo guidance

**FROM (exact; case-sensitive):**
```text
Langkah awal paling mudah adalah melihat logo pada ATM. Visa meminta wisatawan mencari logo PLUS, sementara Mastercard mengarahkan pemegang kartunya ke mesin berlogo Mastercard, Maestro, atau Cirrus. Hana Bank juga mencantumkan jaringan kartu asing yang didukung Global ATM miliknya, termasuk Visa/PLUS, Mastercard/Maestro/Cirrus, JCB, dan UnionPay.
```
**TO (approved final copy):**
```text
Mulailah dengan mencocokkan logo jaringan pada kartu dan mesin ATM. Untuk kartu Visa yang mendukung PLUS, cari logo PLUS. Untuk Mastercard, periksa logo Mastercard, Maestro, atau Cirrus yang sesuai. Hana Bank juga mencantumkan Visa/PLUS, Mastercard/Maestro/Cirrus, JCB, dan UnionPay sebagai jaringan yang didukung pada Global ATM miliknya.
```
**Occurrences in this file:** 1.  
**Reason:** Removes personification of Visa while preserving every supported network listed by the source..

#### ATM11-02 — H2: Incheon airport machine alternatives

**FROM (exact; case-sensitive):**
```text
Di Bandara Incheon, ada lebih dari satu pilihan
```
**TO (approved final copy):**
```text
Jika ATM pertama di Bandara Incheon gagal, coba mesin lain
```
**Occurrences in this file:** 1.  
**Reason:** Action-oriented airport recovery..

#### ATM11-03 — H2: cash withdrawal steps

**FROM (exact; case-sensitive):**
```text
Menarik tunai tanpa membuatnya rumit
```
**TO (approved final copy):**
```text
Langkah menarik uang tunai di ATM Korea
```
**Occurrences in this file:** 1.  
**Reason:** Search-facing and practical title, same section role..

#### ATM11-04 — Paragraph: retry/issuer distinction

**FROM (exact; case-sensitive):**
```text
Tidak banyak manfaatnya memasukkan kartu yang sama ke mesin yang sama berulang kali jika tidak ada yang berubah. Coba ATM lain yang sesuai terlebih dahulu. Jika beberapa mesin yang kompatibel menolak kartu, barulah lebih berguna memeriksa aplikasi penerbit atau menghubungi bank penerbit kartu.
```
**TO (approved final copy):**
```text
Jangan terus mengulang penarikan pada mesin yang sama tanpa mengubah apa pun. Cari ATM lain yang menampilkan jaringan kartu Anda. Jika beberapa ATM yang kompatibel masih menolak transaksi, periksa aplikasi bank atau hubungi penerbit untuk memastikan izin tarik tunai luar negeri dan batas transaksi.
```
**Occurrences in this file:** 1.  
**Reason:** Explicitly preserves practical check order: network compatibility then issuer restrictions..

#### ATM11-05 — Paragraph: DCC currency example

**FROM (exact; case-sensitive):**
```text
Melihat jumlah dalam dolar, euro, atau mata uang negara asal lain yang familier bisa terasa meyakinkan, tetapi itu tidak menunjukkan apakah konversinya lebih murah. Alternatifnya adalah mempertahankan penarikan dalam won Korea dan membiarkan penerbit kartu menerapkan ketentuan valuta asingnya sendiri.
```
**TO (approved final copy):**
```text
Jika ATM menawarkan jumlah dalam rupiah atau mata uang lain yang lebih familier, tampilannya mungkin terasa nyaman, tetapi belum tentu kursnya lebih murah. Pilihan lainnya adalah tetap menarik dalam won Korea dan membiarkan penerbit kartu menerapkan ketentuan konversinya.
```
**Occurrences in this file:** 1.  
**Reason:** Indonesia-relevant example, explicitly conditional: does not claim ATMs universally offer IDR..


### Page 55 — `id/apple-pay-korea.html`

**Search / decision role:** Indonesian Apple Pay issuer-region limitation, distinct store/Apple Wallet transit payment and MobileTmoney foreign-user funding.  
**English Git blob:** `c251aad5f8c314a9bebe060266f352b37cc65805`  
**Indonesian Git blob:** `5ce1ead50a488c73628d37739d475ad13621bdc4`  
**Targeted corrections:** 7 mapping(s), 9 exact location(s).

#### AP11-01 — Hero paragraph: existing eligible card and Indonesia availability

**FROM (exact; case-sensitive):**
```text
Ya. Jika Anda sudah memakai Apple Pay di negara asal, Anda bisa menggunakannya di toko yang menerimanya di Korea, selama tempat usaha mendukung pembayaran nirsentuh dan jaringan kartu dalam Wallet Anda.
```
**TO (approved final copy):**
```text
Ya, jika Anda sudah mempunyai kartu yang terdaftar di Apple Wallet melalui penerbit yang mendukung Apple Pay. Di Korea, kartu tersebut bisa digunakan di toko yang menerima jaringan kartunya dan mengaktifkan pembayaran nirsentuh. Namun, pada daftar resmi Apple per Oktober 2026, Indonesia belum termasuk wilayah penerbit yang mendukung penambahan kartu ke Apple Pay. Jadi, jangan menganggap kartu bank Indonesia bisa langsung ditambahkan ke Wallet hanya untuk perjalanan ke Korea.
```
**Occurrences in this file:** 1.  
**Reason:** Apple's published participating-issuer countries/regions omit Indonesia. This is a dated source-backed Indonesian-market eligibility caveat; do not confuse issuer-country availability with merchant acceptance in Korea..

#### AP11-02 — H2: stores, qualifying issuer

**FROM (exact; case-sensitive):**
```text
Kartu Apple Pay asing Anda bisa dipakai di toko Korea
```
**TO (approved final copy):**
```text
Kartu Apple Pay dari penerbit yang didukung bisa dipakai di toko Korea
```
**Occurrences in this file:** 1.  
**Reason:** Eligibility is conditional on an already supported card in Apple Wallet..

#### AP11-03 — Paragraph: remove American-market referent

**FROM (exact; case-sensitive):**
```text
Karena itu, bagi pengunjung dari AS, pertanyaan yang berguna bukan “Apakah saya punya Hyundai Card?”, melainkan “Apakah terminal Korea ini menerima Apple Pay dan jaringan kartu yang sudah ada dalam Wallet saya?”
```
**TO (approved final copy):**
```text
Jadi, bagi pengunjung yang sudah memiliki kartu di Apple Wallet dari penerbit yang didukung, pertanyaan utamanya bukan “Apakah saya harus punya Hyundai Card Korea?”, tetapi “Apakah terminal toko ini menerima Apple Pay dan jaringan kartu dalam Wallet saya?”
```
**Occurrences in this file:** 1.  
**Reason:** Applies to any traveler with a supported card without pretending an Indonesian bank-issued card can be provisioned..

#### AP11-04 — H2: direct Wallet T-money top-up caveat

**FROM (exact; case-sensitive):**
```text
Dulu, bagian yang sulit adalah mengisi saldo T-money
```
**TO (approved final copy):**
```text
Mengapa pengisian saldo T-money di Apple Wallet bisa menjadi kendala
```
**Occurrences in this file:** 1.  
**Reason:** Evergreen distinction between direct Wallet and the separate MobileTmoney route..

#### AP11-05 — Paragraph: Visa network not listed for MobileTmoney foreign-user funding

**FROM (exact; case-sensitive):**
```text
Ada batasan penting bagi wisatawan dari AS: Visa saat ini tidak tercantum dalam jaringan kartu asing yang didukung untuk pengisian saldo MobileTmoney. Tanggapan pengembang Tmoney di App Store juga telah menyatakan bahwa Visa tidak didukung untuk jalur pengisian saldo pengguna asing ini.
```
**TO (approved final copy):**
```text
Ada batasan jika kartu utama Anda adalah Visa: jaringan ini saat ini belum tercantum dalam kartu asing yang didukung untuk isi saldo MobileTmoney pengguna internasional. Tmoney juga menyatakan melalui tanggapan pengembang di App Store bahwa Visa tidak didukung pada jalur tersebut.
```
**Occurrences in this file:** 1.  
**Reason:** Removes irrelevant US traveler scope while preserving exact foreign-card network limitation. Do not conflate with Visa support in other Android apps..

#### AP11-06 — FAQ question, visible + JSON-LD

**FROM (exact; case-sensitive):**
```text
Bisakah saya memakai kartu Apple Pay terbitan AS di Korea?
```
**TO (approved final copy):**
```text
Bisakah kartu Apple Pay dari negara lain dipakai di Korea?
```
**Occurrences in this file:** 2.  
**Reason:** Keep visible FAQ and JSON-LD name byte-identical..

#### AP11-07 — FAQ answer, visible + JSON-LD

**FROM (exact; case-sensitive):**
```text
Ya, jika kartu tersebut sudah didukung Apple Pay melalui penerbitnya dan diaktifkan untuk penggunaan di luar negeri. Menurut Apple, Apple Pay dapat digunakan di luar negeri, di negara dan wilayah yang mendukung pembayaran nirsentuh. Kompatibilitas tempat usaha dan jaringan tetap diperlukan.
```
**TO (approved final copy):**
```text
Bisa, jika kartunya sudah ditambahkan ke Apple Wallet melalui penerbit yang mendukung Apple Pay dan diizinkan untuk transaksi luar negeri. Apple belum mencantumkan Indonesia sebagai wilayah penerbit yang mendukung penambahan kartu, jadi jangan menganggap kartu bank Indonesia bisa langsung ditambahkan. Di Korea, toko dan jaringan pembayaran kartu tetap harus mendukung transaksi.
```
**Occurrences in this file:** 2.  
**Reason:** Explains how supported overseas cards can work without implying domestic Indonesia Apple Pay issuance; keep visible/schema exact parity..

## 3. Approved exact-implementation and QA contract

1. This is the sole user-approved Final Approved Public Copy for Indonesian Batch 11. **CONTENT LOCKED.** The preceding Review MD is reference material only; do not use it in parallel as another wording authority.
2. On the company PC, confirm `C:\Projects\Koreainside`, branch `id-localization-2026-10-06`, baseline HEAD `c3ef7af8a87b042d1d3de951c6ceb8f983349816`, local active Standards, latest Handover, the existing Batch11 SEO Research, and real `git status --short` before touching files.
3. Recheck all ten EN+ID Git blob fingerprints in §1, and protect existing working-tree changes and all independent untracked/deleted paths. In recent Codex report, expected protected status: deleted 7, untracked 33, Vietnamese SEO MD 12; always treat actual later status as authoritative.
4. Use **THIS Approved MD ONLY** as the source of public wording; exact replacements in **only five `id/` HTML files**. **No English HTML modifications**, no full retranslation, no independent Codex editorial decisions.
5. Verify 28 mapping IDs, 32 occurrences, correct file and exact count (FC11 FAQ 2+2; AP11 FAQ 2+2), source text absent after, replacement text present, and perfect backward reversion to each baseline HTML file. All unrelated displayed words unchanged.
6. For each affected FAQ entry, visible question/answer must be identical to its FAQPage JSON-LD counterpart. All five FAQ scripts must remain parseable with 8 visible FAQ = 8 structured answers per file (40 total). No new FAQ questions or JSON-LD nodes.
7. Preserve H1/H2/H3/P/item counts and HTML tag order; classes, IDs, `data-*`, existing canonical/hreflang, internal and outgoing `href`, alt/ARIA unless explicitly mapped, `src`, `srcset`, affiliate URLs, tracking, CSS, JavaScript, and numeric travel facts. No change to common navigation, header/footer, `common.js`, or `style.css`.
8. For the Apple issuer fact, retain **"per Oktober 2026"** (currently sourced as of 2026-10-08). This is not a promise of permanent unavailability. Do not write "Apple Pay cannot be used in Korea"; do not call direct Wallet recharge and MobileTmoney recharge the same service.
9. After static QA, inspect the staged manifest explicitly: **only five ID HTML files and this one Approved MD**, then staged QA and `git diff --check`. When QA passes, commit/push to `id-localization-2026-10-06` feature branch **only**, as authorized by the Batch 11 approval and implementation workflow. No `git add .`, `git add -A`, `git restore`, `git reset`, `git clean`, `git stash`, or force-push.
10. No main merge, Vercel/Production, browser QA success claim, lang/canonical/hreflang/sitemap or Technical Closure changes during Batch 11. Final Technical Closure only after 58/58 and user authorization.

## 4. Approval baseline and pending implementation

- Pre-review branch matches `c3ef7af8a87b042d1d3de951c6ceb8f983349816` (ahead/behind 0/0), GitHub read-only.
- Exact-source baseline: 28/28 mappings; 32/32 replaceable positions. Synthetic-swap QA against all five branch files confirms reversibility, HTML tag/attribute/reference protection, and matching visible FAQ and JSON-LD content (40/40). Codex must independently repeat these checks on the company PC before a Git checkpoint.
- **Current disposition:** 50/58 pages completed through Batch 10; Batch 11 is now **APPROVED / CONTENT LOCKED / IMPLEMENTATION PENDING**, taking the count to 55/58 only after successful Codex QA and feature-branch checkpoint. No Git files were edited to create this Approved MD.
- This document is that single Final Approved Public Copy MD. Save it under `md/승인본/인도네시아어/` without modifications and follow the approved Codex Remote implementation instructions. No supplemental review/approval loop unless a genuine source conflict is found.
