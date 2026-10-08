# Korea Inside — Indonesian Batch 09 — Approved Public Copy

**Date:** 2026-10-08  
**Status:** **APPROVED PUBLIC COPY — CONTENT LOCKED**  
**User approval:** `다음 진행` (2026-10-08), approving the complete 5-page Indonesian Batch 09 and the explicitly proposed narrow English `tmoney.html` factual correction.  
**Single implementation wording Source of Truth:** **THIS FINAL FILE ONLY**; 35 exact mappings (26 ID + 9 English) with 37 replacement occurrences (including FAQ visible/JSON-LD paired positions).  
**Current branch:** `id-localization-2026-10-06`  
**Pre-implementation HEAD:** `d7f64f000bc895d013d55fd9fa70627f74d0b109` (last confirmed feature checkpoint; Codex must recheck before editing)  
**Basis:** current approved English facts and structure + existing first-pass Indonesian HTML. The approved `tmoney.html` English factual exception is expressly authorized in this Batch; all other English wording stays locked.  
**Scope:** exactly five Indonesian HTML working copies (`id/maps.html`, `id/tmoney.html`, `id/wowpass.html`, `id/tmoney-vs-wowpass.html`, `id/taxi.html`) and one approved English factual correction in `tmoney.html`. No other English file or common UI, no Technical Closure, no Production.

## 1. Research / operating baseline

- Indonesian SEO Research is already **58/58 COMPLETE**; do not repeat SEO or SERP research. Batch 09 research is **COMPLETE** per Indonesia SEO handover; its full underlying standalone Batch09 Research MD was not present in the materials readable in this session, so this Approved MD does **not** claim to have reread that standalone document.
- Page identifiers 41–45: `maps.html`, `tmoney.html` (**P1-URGENT**), `wowpass.html`, `tmoney-vs-wowpass.html`, `taxi.html` (other four **P1**).
- Master Standard attached to the Project is ACTIVE v1.4, Navigation ACTIVE v1.1; repository currently has older v1.3/v1.0. Localization Standard in the repository is v2.0; latest Handover refers to v2.1, not accessible in current repository. This version discrepancy does **not** grant permission to modify Standards or rewrite locked Batches.
- This approved mapping set performs targeted human editing of first-pass copy. All unlisted text remains unchanged. Published facts, numbers, prices, brand names, place names, recommendations, schema structure, images, hrefs, affiliate/tracking, common UI and CSS/JS remain protected.
- Current Indonesian HTML uses `lang="en"` as a historical working-copy state; retain until full-language Technical Closure. Do not change canonical/hreflang/sitemap or common UI in this Batch.

## 2. Official current-fact basis — T-money P1-URGENT

**Confirmed current distinction (as of 2026-10-08):**
- Apple documentation describes a **South Korea-issued payment-card requirement for direct purchase/top-up of T-money inside Apple Wallet**.
- The **separate MobileTmoney iPhone app** lists an international-user route requiring no ordinary sign-up, with Apple Pay top-up supported for **Mastercard, American Express, UnionPay and JCB**. The developer states that **Visa is currently not supported for this app top-up route**.
- App top-up is **not** proof that ordinary physical T-money can be topped up with foreign cards at a standard subway kiosk, nor that ordinary overseas debit/credit cards can be tapped directly at every Korean subway gate.
- Preserve the separate **KOREA TOUR CARD Tmoney Android** route and its distinct published card-brand and NFC requirements; never merge its rules with the iPhone app.
- Existing `apple-pay-korea.html` in English and Indonesian already describes these facts. Batch 09 repairs the `tmoney.html` inconsistency without reopening that already-correct comparison page.

**Primary first-party verification:** `https://apps.apple.com/us/app/mobiletmoney/id1470361790` (developer Tmoney Co., Ltd; current version history and developer responses); `https://support.apple.com/en-ie/105079` (direct Wallet top-up policy).

**Approved English factual exception:** the user approved the exact narrowly scoped correction in `tmoney.html` together with `id/tmoney.html`. The exact nine paired mappings below preserve FAQ visible/JSON-LD equality. This is a narrow factual-error re-opening, not general rewriting of English CONTENT LOCKED material.

## 3. Source fingerprints / structure snapshot

| English page | EN Git blob | ID Git blob | EN=ID H1/H2/H3 | ID editorial mappings | EN fact mappings |
|---|---|---|---|---:|---:|
| `maps.html` | `edd3ca4aea1e35c483eaed39eadeadb0a92d51ab` | `8954c974341ac1f27728997717f27316135fb308` | 1/11/27 | 5 | 0 |
| `tmoney.html` | `c1e1b005ffcb762b78ce33f1d031d3128135f8a1` | `51f787d059a86142f4b611ddb345f7c57e7048cc` | 1/16/7 | 9 | 9 |
| `wowpass.html` | `bfbf8ed3fd746f56c570106a303f128e27005a5f` | `e741c325fa1239eda0f1265e7e253e4dc3e44f10` | 1/16/7 | 4 | 0 |
| `tmoney-vs-wowpass.html` | `3f8a58a69b7e4b0ad78a72a311ea602ddac31723` | `3edd33a8a1c5a5aafa1a6bf742ee317b12f085a4` | 1/11/0 | 4 | 0 |
| `taxi.html` | `1c50ed373c17736a7c2281c969fdc28b347b8dc9` | `28def056d9a9e891246b8584b34bd1967ba6dc8a` | 1/13/6 | 4 | 0 |

All five matched the EN source for H1/H2/H3, paragraph/list counts when checked. Source fingerprints MUST be reverified against the local repository before Codex implementation; GitHub read-only blob SHA alone is not evidence of no local working-tree edits.

## 4. FINAL APPROVED EXACT MAPPINGS — CONTENT LOCKED

For every mapping: match `FROM` exactly, require specified occurrence count within the **single named file** before replacement; use `TO` verbatim as approved. Source locators are descriptive and paired with exact strings. NO fuzzy matching, no paraphrase by Codex.

### File — `id/maps.html`

#### M09-01 — Intro paragraph around original line 226
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Gunakan Naver Map sebagai aplikasi navigasi utama di Korea. Siapkan KakaoMap sebagai cadangan, dan gunakan Google Maps untuk menyimpan tempat, membaca ulasan, serta merencanakan perjalanan di berbagai negara.
```
TO:
```text
Untuk navigasi sehari-hari di Korea, mulailah dengan Naver Map. Siapkan KakaoMap sebagai cadangan untuk memeriksa hasil pencarian atau rute lain. Google Maps tetap berguna untuk tempat yang disimpan, ulasan, dan perencanaan perjalanan lintas negara.
```
- Rationale: Keep the Naver-first / Kakao-backup / Google-planning judgment, with natural Indonesian flow.

#### M09-02 — Comparison lead, original line 253
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Pilih aplikasi sesuai kebutuhan perjalanan yang ingin Anda atasi, bukan hanya karena sudah mengenal mereknya.
```
TO:
```text
Pilih aplikasi berdasarkan masalah perjalanan yang perlu Anda selesaikan, bukan sekadar karena sudah akrab dengan mereknya.
```
- Rationale: Remove literal translation of solve a travel problem.

#### M09-03 — English-search failure lead, original line 471
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Temukan hasil yang tepat dengan mengganti nama terjemahan menggunakan informasi tempat yang persis dalam bahasa Korea.
```
TO:
```text
Jika nama terjemahan tidak ditemukan, coba nama atau alamat resmi tempat tersebut dalam bahasa Korea agar hasil pencariannya lebih tepat.
```
- Rationale: Give the exact recovery action without implying a new app feature.

#### M09-04 — Google offline maps H3, original line 544
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Peta Google tanpa internet
```
TO:
```text
Menggunakan Google Maps tanpa internet
```
- Rationale: Make the heading a natural Indonesian task label; do not assert offline navigation availability.

#### M09-05 — Common mistakes lead, original line 624
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Pemeriksaan kecil membantu menghindari kesalahan tempat dan arah yang paling sering terjadi.
```
TO:
```text
Memeriksa beberapa detail sebelum berangkat membantu Anda menghindari salah tempat atau salah arah.
```
- Rationale: Keep the small-checks rationale while removing translationese.

### File — `tmoney.html`

#### TE09-01 — English meta description attribute, original line near head
- Kind: EN factual source correction; expected occurrences: **1**.
FROM:
```text
How to use T-money in Korea, including where to buy and recharge a physical card, Apple Wallet top-up limits, and how it compares with WOWPASS and the Climate Card.
```
TO:
```text
How to use T-money in Korea: physical card top-ups, Apple Wallet versus MobileTmoney foreign-card funding, and when WOWPASS or the Climate Card fits.
```
- Rationale: English fact-source repair; distinct funding paths are current.

#### TE09-02 — English hero second paragraph, original line 194
- Kind: EN factual source correction; expected occurrences: **1**.
FROM:
```text
Since July 2025, T-money can also live in Apple Wallet on supported iPhone and Apple Watch models. That sounds like the obvious choice until you reach the top-up rule: Apple currently requires a South Korea-issued credit or debit card to add money in Wallet. For many overseas visitors, the physical card is still the least complicated option.
```
TO:
```text
Since July 2025, T-money has also worked in Apple Wallet on supported iPhone and Apple Watch models. Apple says direct top-up inside Wallet requires a South Korea-issued payment card. The separate MobileTmoney app now offers an international-user route for top-ups with supported foreign Apple Pay cards. A physical card remains a straightforward choice if you prefer cash top-ups or do not have a supported card.
```
- Rationale: Clarify separate app route without removing Apple direct-Wallet rule.

#### TE09-03 — English comparison table, Apple Wallet best-fit cell, original line 222
- Kind: EN factual source correction; expected occurrences: **1**.
FROM:
```text
iPhone or Apple Watch users who already have a supported South Korea-issued payment card
```
TO:
```text
iPhone or Apple Watch users with a South Korea-issued card for direct Wallet top-up, or an eligible overseas card for MobileTmoney app top-up
```
- Rationale: Older best-fit statement excludes the supported foreign-app route.

#### TE09-04 — English comparison table, Apple Wallet top-up method, original line 234
- Kind: EN factual source correction; expected occurrences: **1**.
FROM:
```text
Apple Wallet or Mobile T-money; Wallet funding currently requires a South Korea-issued credit or debit card
```
TO:
```text
Directly in Apple Wallet with a South Korea-issued card, or inside MobileTmoney using supported overseas Mastercard, American Express, UnionPay or JCB cards
```
- Rationale: State both actual routes and current listed foreign networks.

#### TE09-05 — English comparison table, Apple Wallet main catch, original line 240
- Kind: EN factual source correction; expected occurrences: **1**.
FROM:
```text
An overseas card alone does not solve the Wallet top-up problem
```
TO:
```text
Direct Wallet top-up and MobileTmoney use different payment rules; foreign Visa cards are not currently listed for in-app top-up
```
- Rationale: Prevent false impression every foreign card is unsupported while retaining Visa limitation.

#### TE09-06 — English Apple Wallet H2, original line 300
- Kind: EN factual source correction; expected occurrences: **1**.
FROM:
```text
Apple Wallet T-money: Convenient, With One Important Catch
```
TO:
```text
Apple Wallet T-money: Two Different Ways to Top Up
```
- Rationale: Title now signals direct Wallet vs app funding distinction.

#### TE09-07 — English body follow-up after official Apple link, original line 305
- Kind: EN factual source correction; expected occurrences: **1**.
FROM:
```text
So an iPhone is not, by itself, a reason to skip the physical card. If the payment cards in your Wallet were issued outside Korea, confirm your funding route before deciding to rely only on digital T-money.
```
TO:
```text
Foreign-issued cards may still work through the separate MobileTmoney app. Its international-user route offers top-ups with Apple Pay using Mastercard, American Express, UnionPay or JCB without ordinary sign-up; Visa is not currently listed. Confirm your phone and payment card work with that app before relying on digital T-money alone. Otherwise, a physical card topped up with cash remains a practical fallback.
```
- Rationale: Repair missing visitor action: choose app route, check network, fallback.

#### TE09-08 — English troubleshooting paragraph, original line 361
- Kind: EN factual source correction; expected occurrences: **1**.
FROM:
```text
Apple Wallet now supports T-money, but the funding rule is the catch. Apple currently requires a South Korea-issued credit or debit card to purchase or top up T-money in Wallet.
```
TO:
```text
Apple Wallet supports T-money, but direct Wallet top-up still has a South Korea-issued card rule. The separate MobileTmoney app now lists Mastercard, American Express, UnionPay and JCB for top-ups by international users; Visa is not listed. If the app route does not work with your card or phone, use a physical T-money card and cash top-ups.
```
- Rationale: No longer implies all foreign cards are blocked from mobile use.

#### TE09-09 — English FAQ answer, appears both in visible FAQ and FAQPage JSON-LD, original line 391 and head
- Kind: EN factual source correction; expected occurrences: **2**.
FROM:
```text
Yes. Apple Wallet has supported prepaid T-money since July 21, 2025 on supported iPhone and Apple Watch models. However, Apple currently requires a South Korea-issued credit or debit card to purchase or top up T-money in Wallet.
```
TO:
```text
Yes. Apple Wallet supports prepaid T-money on compatible iPhone and Apple Watch models. Apple says direct Wallet top-up requires a South Korea-issued card, but the separate MobileTmoney app lists Mastercard, American Express, UnionPay and JCB for international-user top-ups. Visa is not currently listed for that app route.
```
- Rationale: Ensure visible FAQ and structured FAQ agree.

### File — `id/tmoney.html`

#### T09-01 — Indonesian meta description attribute, original line near head
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Cara pakai T-money di Korea, termasuk tempat membeli dan mengisi saldo kartu fisik, batas pengisian saldo Apple Wallet, serta perbandingannya dengan WOWPASS dan Climate Card.
```
TO:
```text
Cara pakai T-money di Korea: isi saldo kartu fisik, perbedaan Apple Wallet dan MobileTmoney untuk kartu asing, serta kapan memakai WOWPASS atau Climate Card.
```
- Rationale: Align Indonesian search-facing fact statement with repaired English source.

#### T09-02 — Indonesian hero second paragraph, original line 194
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Sejak Juli 2025, T-money juga bisa disimpan di Apple Wallet pada model iPhone dan Apple Watch yang didukung. Pilihan ini terdengar paling masuk akal sampai Anda melihat aturan pengisian saldonya: saat ini Apple mewajibkan kartu kredit atau debit yang diterbitkan di Korea Selatan untuk menambahkan saldo di Wallet. Bagi banyak pengunjung dari luar negeri, kartu fisik tetap menjadi pilihan paling sederhana.
```
TO:
```text
Sejak Juli 2025, T-money juga bisa digunakan melalui Apple Wallet pada model iPhone dan Apple Watch yang didukung. Menurut Apple, pengisian saldo langsung di Wallet memerlukan kartu pembayaran terbitan Korea Selatan. Namun, aplikasi MobileTmoney menyediakan cara lain bagi pengguna internasional untuk mengisi saldo melalui Apple Pay dengan kartu asing yang didukung. Kartu fisik tetap praktis jika Anda lebih memilih isi saldo tunai atau tidak memiliki kartu yang didukung.
```
- Rationale: Preserve physical fallback and distinguish funding routes.

#### T09-03 — Indonesian comparison table, Apple Wallet best-fit cell, original line 222
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Pengguna iPhone atau Apple Watch yang sudah memiliki kartu pembayaran terbitan Korea Selatan yang didukung
```
TO:
```text
Pengguna iPhone atau Apple Watch dengan kartu terbitan Korea Selatan untuk isi saldo langsung di Wallet, atau kartu asing yang didukung untuk isi saldo lewat MobileTmoney
```
- Rationale: Correct table eligibility without claiming all overseas cards are accepted.

#### T09-04 — Indonesian comparison table, Apple Wallet top-up method, original line 234
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Apple Wallet atau Mobile T-money; pengisian saldo Wallet saat ini memerlukan kartu kredit atau debit terbitan Korea Selatan
```
TO:
```text
Langsung di Apple Wallet dengan kartu terbitan Korea Selatan, atau melalui MobileTmoney dengan kartu asing Mastercard, American Express, UnionPay, atau JCB yang didukung
```
- Rationale: Include supported foreign-card names with correct app boundary.

#### T09-05 — Indonesian comparison table, Apple Wallet main catch, original line 240
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Memiliki kartu terbitan luar negeri saja belum menyelesaikan kendala pengisian saldo Wallet
```
TO:
```text
Aturan isi saldo langsung di Wallet berbeda dari MobileTmoney; Visa terbitan luar negeri saat ini belum tercantum untuk pengisian saldo melalui aplikasi
```
- Rationale: Foreign-card Visa limitation is app-specific, not general Apple Pay card acceptance.

#### T09-06 — Indonesian Apple Wallet H2, original line 300
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
T-money di Apple Wallet: Mudah Digunakan, tetapi Ada Syarat Penting
```
TO:
```text
T-money di Apple Wallet: Dua Cara Mengisi Saldo
```
- Rationale: Natural parallel to the updated English heading.

#### T09-07 — Indonesian body follow-up after official Apple link, original line 305
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Jadi, memiliki iPhone saja bukan alasan untuk melewatkan kartu fisik. Jika kartu pembayaran di Wallet Anda diterbitkan di luar Korea, pastikan cara pengisian saldo sebelum memutuskan hanya mengandalkan T-money digital.
```
TO:
```text
Kartu terbitan luar negeri masih bisa digunakan melalui aplikasi MobileTmoney yang terpisah. Jalur untuk pengguna internasional memungkinkan isi saldo melalui Apple Pay dengan Mastercard, American Express, UnionPay, atau JCB tanpa pendaftaran akun biasa; Visa belum tercantum. Pastikan ponsel dan kartu Anda kompatibel dengan aplikasi tersebut sebelum hanya mengandalkan T-money digital. Jika tidak, kartu fisik yang diisi tunai tetap menjadi cadangan praktis.
```
- Rationale: Match English decision path; avoid suggesting that direct Wallet and app funding are identical.

#### T09-08 — Indonesian troubleshooting paragraph, original line 361
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Apple Wallet kini mendukung T-money, tetapi syarat pengisian saldo menjadi kendalanya. Apple saat ini mewajibkan kartu kredit atau debit terbitan Korea Selatan untuk membeli atau mengisi saldo T-money di Wallet.
```
TO:
```text
Apple Wallet mendukung T-money, tetapi isi saldo langsung di Wallet masih mengikuti ketentuan kartu terbitan Korea Selatan. Aplikasi MobileTmoney yang terpisah kini mencantumkan Mastercard, American Express, UnionPay, dan JCB untuk pengisian saldo oleh pengguna internasional; Visa belum tercantum. Jika cara tersebut tidak cocok dengan kartu atau ponsel Anda, gunakan T-money fisik dan isi saldo tunai.
```
- Rationale: Troubleshooting now offers a verified fallback.

#### T09-09 — Indonesian FAQ answer, appears both in visible FAQ and FAQPage JSON-LD, original line 391 and head
- Kind: ID editorial; expected occurrences: **2**.
FROM:
```text
Ya. Apple Wallet mendukung T-money prabayar sejak 21 Juli 2025 pada model iPhone dan Apple Watch yang didukung. Namun, Apple saat ini mewajibkan kartu kredit atau debit terbitan Korea Selatan untuk membeli atau mengisi saldo T-money di Wallet.
```
TO:
```text
Ya. Apple Wallet mendukung T-money prabayar di iPhone dan Apple Watch yang kompatibel. Menurut Apple, isi saldo langsung di Wallet memerlukan kartu terbitan Korea Selatan, tetapi aplikasi MobileTmoney yang terpisah mencantumkan Mastercard, American Express, UnionPay, dan JCB untuk isi saldo pengguna internasional. Visa saat ini belum tercantum untuk jalur aplikasi tersebut.
```
- Rationale: Visible FAQ and FAQPage JSON-LD must be identical after replacement.

### File — `id/wowpass.html`

#### W09-01 — H2 around original line 136
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Fungsi WOWPASS Sebenarnya
```
TO:
```text
Apa saja fungsi WOWPASS?
```
- Rationale: A natural Indonesian question, no change in scope.

#### W09-02 — Lead sentence around original line 139
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Gabungan fungsi itulah daya tariknya, tetapi juga sumber sebagian besar kebingungan.
```
TO:
```text
Beberapa fungsi dalam satu kartu memang menarik, tetapi juga sering membuat wisatawan salah memahami cara kerjanya.
```
- Rationale: Remove literal compression while retaining the dual-function confusion.

#### W09-03 — Practical consequence paragraph around original line 171
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Dampaknya sederhana: pembayaran yang berhasil di toko tidak menunjukkan berapa sisa saldo kereta bawah tanah, dan saldo T-money yang cukup tidak menunjukkan kondisi saldo pembayaran.
```
TO:
```text
Pembayaran yang berhasil di toko tidak menunjukkan berapa sisa saldo T-money Anda. Sebaliknya, saldo T-money yang cukup untuk naik kereta tidak berarti saldo belanja WOWPASS juga cukup.
```
- Rationale: Keep each balance independent and directly actionable.

#### W09-04 — H2 around original line 262
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Masalah yang Membuat WOWPASS Terlihat Lebih Rumit
```
TO:
```text
Masalah umum yang membuat WOWPASS terasa rumit
```
- Rationale: Less literal heading without adding issues.

### File — `id/tmoney-vs-wowpass.html`

#### C09-01 — H2 around original line 201
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
T-money membuat perjalanan tetap sederhana
```
TO:
```text
Kapan T-money biasa lebih praktis
```
- Rationale: More decision-forward and consistent with the paragraph below.

#### C09-02 — Opening paragraph under the above H2, around line 202
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Jika terutama membutuhkan kartu untuk ditempelkan di kereta bawah tanah dan bus, T-money biasa sulit membuat urusan menjadi rumit.
```
TO:
```text
Jika Anda hanya perlu membayar bus dan kereta bawah tanah, kartu T-money biasa membuat pengaturan perjalanan tetap sederhana.
```
- Rationale: Remove literal phrasing; keep T-money-first for transit-only use.

#### C09-03 — H2 around original line 209
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
WOWPASS menambahkan fungsi selain transportasi
```
TO:
```text
WOWPASS: lebih dari sekadar transportasi
```
- Rationale: Keep differing product role in a concise heading.

#### C09-04 — Comparison explanation around original line 196
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Jadi, keduanya bukan sekadar dua versi yang bersaing dari produk yang sama.
```
TO:
```text
Keduanya memenuhi kebutuhan berbeda, bukan sekadar dua kartu transportasi yang saling bersaing.
```
- Rationale: Preserve decision distinction rather than treating them as interchangeable.

### File — `id/taxi.html`

#### X09-01 — H2 around original line 199
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Pilihan Taksi Mana yang Sebaiknya Digunakan?
```
TO:
```text
Cara memilih taksi sesuai kebutuhan perjalanan
```
- Rationale: Natural task-led heading.

#### X09-02 — Street-hail paragraph around original line 261
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Pangkalan taksi atau taksi kosong di jalan bisa digunakan dengan mudah jika titik penjemputan jelas dan tujuan sudah Anda siapkan dalam bahasa Korea.
```
TO:
```text
Anda juga bisa naik taksi dari pangkalan resmi atau memberhentikan taksi kosong, asalkan titik jemputnya jelas dan alamat tujuan sudah disiapkan dalam bahasa Korea.
```
- Rationale: Focus on actual arrival and pickup friction.

#### X09-03 — On-board address check around original line 274
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Periksa apakah tujuan yang ditampilkan pengemudi sesuai dengan tempat yang Anda simpan, lalu tetap buka peta selama perjalanan. Ini membantu mengetahui cabang yang salah atau alamat yang disalahpahami sejak awal.
```
TO:
```text
Cocokkan tujuan pada perangkat pengemudi dengan lokasi yang sudah Anda simpan, lalu pantau perjalanan di peta. Jika alamat atau cabangnya salah, Anda bisa mengetahuinya sejak awal.
```
- Rationale: Natural route-monitoring guidance; no claim of driver device functionality beyond visible destination.

#### X09-04 — H2 around original line 421
- Kind: ID editorial; expected occurrences: **1**.
FROM:
```text
Kapan Taksi Memang Menjadi Pilihan yang Tepat
```
TO:
```text
Kapan naik taksi lebih masuk akal?
```
- Rationale: Straightforward conditional decision title.

## 5. Codex exact-implementation contract, QA and protection

- Counts: **26 Indonesian mappings + 9 paired English factual mappings**; 9 paired mappings change the English T-money fact layer. Other four English originals remain untouched.
- FAQ answer `TE09-09` and `T09-09`: each matches exactly **2 occurrences** (visible FAQ + FAQPage JSON-LD). Both strings must remain exactly equal in their respective language after replacement.
- User approved this Batch with `다음 진행` on 2026-10-08, including the narrow English `tmoney.html` current-fact correction. **This file is the single FINAL Approved Public Copy MD**. No correction supplement or repeated wording approval required; Codex must not rewrite approved values.
- During Codex implementation, require **pre-change source blob/HTML fingerprint and working-tree-scope checks**. Apply only the above exact mappings. If a source changed, identify actual textual drift; do not reset/restore or overwrite user changes.
- Static QA: each occurrence count exactly, target-not-present-before, reverse replacement reproduces original HTML bytes, no unexpected visible copy; structural tag/order/H1/H2/H3/FAQ/schema/ARIA/alt/links/images/affiliate/tracking parity. Numeric values and recommendations must not drift. `git diff --check` and staged manifest QA.
- After exact implementation and successful Static QA, stage **only** `id/maps.html`, `id/tmoney.html`, `id/wowpass.html`, `id/tmoney-vs-wowpass.html`, `id/taxi.html`, `tmoney.html` and the final approved MD. Feature branch commit/push only; never main merge/Vercel/Production/Technical Closure.
- Preserve deleted 7 / untracked 31 from previous local report, Vietnamese SEO MD 12, Batch 01–08 locks, and any new local edits as verified at runtime. Prohibit `git add .`, `git add -A`, `git restore`, `git reset`, `git clean`, `git stash`, force push, unrelated stage.
- This Approved MD was prepared read-only. ChatGPT has not modified any repository file, Git ref, or Production. Codex alone performs the approved local implementation and feature-branch checkpoint.

**BATCH 09 APPROVED PUBLIC COPY — CONTENT LOCKED / CODEX IMPLEMENTATION PENDING**
