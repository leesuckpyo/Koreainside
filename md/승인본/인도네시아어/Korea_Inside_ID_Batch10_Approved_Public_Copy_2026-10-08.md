# Korea Inside — Indonesian Batch 10 — Approved Public Copy

**Date:** 2026-10-08  
**Status:** **APPROVED PUBLIC COPY — CONTENT LOCKED**  
**User wording approval:** `다음 자행` (2026-10-08); approved targeted wording, including 7 English factual corrections  
**Single wording Source of Truth:** THIS FILE ONLY for Batch 10; no supplement, no independent Codex rewriting  
**Implementation target branch:** `id-localization-2026-10-06` (feature branch only; current HEAD must be checked by Codex)  
**Baseline HEAD:** `286dce5f3f32675f986f247f5a4682d3fda57dde`  
**Research status:** Indonesian SEO Research 58/58 COMPLETE; Batch10 (46–50) P0 Rental Car, P1-URGENT Checklist. No SEO re-research.  
**Approved change model:** Existing first-pass Indonesian public copy + targeted English factual source corrections; no repository edits by ChatGPT. Implementation pending Codex exact replacement and QA.  

## 1. Exact scope and source fingerprints

| Page | English Git blob | Indonesian Git blob | EN=ID H1/H2/H3 | Approved ID mappings | Approved EN mappings |
|---|---|---|---:|---:|---:|
| `incheon-airport-private-transfer.html` | `d9c83b95d75b163008915a68fe724f30802e093f` | `7b147cafae4c1f822c5c046a7bd88e5008a8faa2` | 1/11/10 | 4 | 0 |
| `rental-car.html` | `474572bf7dac20d561e5be91d0a6af0acc6380e5` | `59267ef316189f620c343ae718a8f8bd4278ab14` | 1/16/28 | 4 | 4 |
| `apps.html` | `a747808784e7c2f80f155d9c30c3f5f28dfe0689` | `89ce7889546eb5e8da93baf3c45cf10c575c40c4` | 1/11/25 | 4 | 0 |
| `checklist.html` | `62ca9def0363429ec6bbd15a62c2d37edda7a936` | `2173560847c5d4d1a1bd90f9498a05ba69afa582` | 1/12/3 | 3 | 3 |
| `payments.html` | `51334b434e30a1b6b4a4572a7c9c2b53f57836b2` | `a4f45970ff70146eea4e156c740010cbaea6290b` | 1/6/8 | 5 | 0 |

**Totals:** 20 Indonesian mappings; 7 English factual corrections; **27 exact FROM → TO mappings** in 7 HTML files (5 ID + 2 EN).

**Locked:** English/ID page roles, section order, all numeric facts except any explicitly user-approved correction, hotel/area/service judgments, real routes, links/affiliate/tracking, titles/meta outside the mapped corrections, images/srcset, class/id/data-*, CSS/JS, FAQ counts, JSON-LD structure, common UI, current `lang`, canonical, hreflang and sitemap. Keep `lang="en"` on Indonesian working copies until Final Technical Closure. No other English public copy may change.

## 2. P0 and P1-URGENT factual basis

### P0 — Rental car eligibility (documents issued in Indonesia)
- Under the Republic of Korea Road Traffic Act, Article 96, permitted foreign driving requires a qualifying 1949 Geneva/1968 Vienna convention permit or a separately recognized reciprocal instrument; a company's willingness to rent does not replace legal authorization.
- The UN 1968 Vienna Convention treaty register records Indonesia as a signatory (8 Nov 1968) **without ratification**. Indonesia is not included among the 1949 Geneva Convention parties. Indonesia's national police does issue a `SIM Internasional` referencing the Vienna Convention, but issuance alone does not demonstrate acceptance for driving in Korea.
- Therefore avoid categorical blanket conclusions by **nationality**; the relevant issue is the **issuing jurisdiction and recognized authorization**. Travellers relying solely on Indonesian-issued national/international driving documents must confirm legal recognition with a Korean competent authority and rental-company document acceptance before paying; without recognized authorization, do not drive.
- User-facing English and Indonesian corrections below are necessary to resolve this established market-specific P0 without rewriting the guide for all travellers.

### P1-URGENT — e-Arrival Card omitted from Checklist
- The official Korean e-Arrival Card is **free**. Eligible travellers may submit within **three days before arrival**, and the official website provides a Navigator to identify whether submission is required.
- Visa, K-ETA and e-Arrival Card are different questions. An Indonesian passport does not automatically imply a uniform e-Arrival/K-ETA status: the exact visa, K-ETA and residence documentation matters.
- Add the fact only to the existing EN/ID checklist document label/FAQ; do not create a new section, CTA, extra FAQ, schema object or hyperlink. The official domain is plain text in the existing `span`.

### Official verification references (research only; not new public links)
- Korean Road Traffic Act Art96: https://www.law.go.kr/LSW/lsLinkCommonInfo.do?chrClsCd=010202&lsJoLnkSeq=1030973589
- UN Vienna 1968 treaty record: https://treaties.un.org/Pages/ViewDetailsIII.aspx?chapter=11&clang=_en&mtdsg_no=XI-B-19&src=TREATY
- UN Geneva 1949 treaty record: https://treaties.un.org/pages/ViewDetailsV.aspx?Temp=mtdsg5&chapter=11&clang=_en&mtdsg_no=XI-B-1&src=TREATY
- Indonesian Police SIM Internasional: https://digitalkorlantas.polri.go.id/sim-internasional/
- Korean official public legal guide (Indonesian): https://m.easylaw.go.kr/MOM/SubCsmOvRetrieve.laf?ccfNo=1&cciNo=1&cnpClsNo=1&csmSeq=2448&langCd=700105
- KTO official rental requirements: https://english1.visitkorea.or.kr/enu/TRP/TP_ENG_8_3.jsp
- e-Arrival official website: https://www.e-arrivalcard.go.kr/

## 3. Source-based exact corrections

Apply only using THIS final Approved MD. Each `FROM` is an exact literal source substring; locate it in the named file only. For any repeated FAQ text, update both visible and JSON-LD user-facing string without changing schema structure. Preserve all markup surrounding the literal substring.

### File — `id/incheon-airport-private-transfer.html`

#### PT10-01 — Lead paragraph, original line 164
FROM:
```text
Transportasi pribadi mulai layak dipertimbangkan ketika satu kendaraan dapat membawa seluruh rombongan dan semua barang bawaan langsung dari bandara ke hotel.
```
TO:
```text
Transfer pribadi dari bandara patut dipertimbangkan jika satu kendaraan dapat membawa seluruh rombongan beserta barang bawaan langsung ke hotel.
```
Reason: Removes a literal opening while preserving the one-vehicle/luggage decision.

#### PT10-02 — Lead decision paragraph, original line 166
FROM:
```text
Pertanyaannya bukan sekadar apakah mobil pribadi lebih mahal. Yang perlu dipertimbangkan adalah apakah biaya tambahan itu sepadan dengan terbebasnya Anda dari kerepotan membawa barang, berpindah kendaraan, dan menempuh perjalanan terakhir ke hotel setelah penerbangan panjang.
```
TO:
```text
Jangan hanya membandingkan harga kendaraan pribadi dengan tiket kereta atau bus. Pertimbangkan apakah biaya tambahannya sepadan dengan berkurangnya kerepotan membawa bagasi, berganti kendaraan, dan berjalan dari pemberhentian terakhir ke hotel setelah penerbangan panjang.
```
Reason: Makes the door-to-door comparison actionable; no recommendation or price changes.

#### PT10-03 — Vehicle capacity paragraph, original line 279
FROM:
```text
Ketika jumlah barang bawaan mendekati batas yang tercantum, memesan kendaraan lebih besar mungkin lebih aman daripada berusaha mengisi semua kursi yang tersedia.
```
TO:
```text
Jika jumlah atau ukuran bagasi mendekati batas kendaraan yang tercantum, pertimbangkan kendaraan yang lebih besar. Jumlah kursi yang cukup belum tentu berarti semua koper juga muat.
```
Reason: Distinguishes seats from luggage without promising a larger model is always sufficient.

#### PT10-04 — Provider comparison paragraph, original line 314
FROM:
```text
Korea Inside tidak menempatkan salah satu perusahaan ini sebagai pilihan “terbaik” untuk semua orang. Bandingkan kendaraan dan ketentuan pemesanan yang benar-benar tersedia untuk tanggal perjalanan Anda.
```
TO:
```text
Tidak ada satu penyedia yang otomatis paling sesuai untuk semua rombongan. Bandingkan kapasitas kendaraan, aturan bagasi, waktu tunggu, dan pembatalan pada produk yang tersedia untuk tanggal kedatangan Anda.
```
Reason: Makes the original condition-first provider decision explicit; no ranking change.

### File — `rental-car.html`

#### REN10-EN01 — Quick-answer paragraph, original line 181
FROM:
```text
Confirm your licence, IDP, age, card and insurance requirements first.
```
TO:
```text
First verify that Korea legally recognizes the issuing country and type of your driving authorization; only then check the rental company's age, card, deposit and insurance rules.
```
Reason: P0: legal driver eligibility precedes commercial rental criteria.

#### REN10-EN02 — Document-recognition paragraph, original line 302
FROM:
```text
Obtain the IDP before traveling. Check its validity, the vehicle class shown and whether the issuing country and document format are recognized.
```
TO:
```text
Check the IDP's validity, vehicle class, issuing country and legal recognition before traveling. An Indonesia-issued SIM Internasional must not be assumed valid for driving in Korea: Indonesia signed but has not ratified the 1968 Vienna Convention, and it is not a contracting party to the 1949 Geneva Convention. If relying on Indonesian-issued documents, confirm legal eligibility with the relevant Korean authority before making a non-refundable rental booking.
```
Reason: P0: UN treaty status and Korea Road Traffic Act Article 96; avoids assuming a foreign permit is accepted merely because it exists.

#### REN10-EN03 — Law-versus-rental paragraph, original line 309
FROM:
```text
Operators can require additional documents, a minimum period of driving experience or a different licence class for larger vehicles. Ask for written confirmation if your situation is not standard.
```
TO:
```text
Rental companies can require extra documents, minimum driving experience or a different licence class for larger vehicles. Separately confirm that the permit is legally recognized in Korea and obtain written confirmation that the operator will accept your exact documents. If the driving authorization is not recognized, do not drive.
```
Reason: Keeps separate legal and commercial gates; no blanket nationality ban.

#### REN10-EN04 — FAQ answer; visible plus JSON-LD, original line 572
FROM:
```text
Most short-term visitors should plan to bring their original driving licence, passport and a valid official International Driving Permit that Korea recognizes. Korean law also permits certain mutually recognized foreign licences, but rental companies may apply narrower document rules, so confirm acceptance before paying.
```
TO:
```text
Most short-term visitors need their original licence, passport and driving authorization legally recognized in Korea. Not every foreign-issued IDP qualifies: Indonesia's Vienna Convention signature has not been ratified and Indonesia is not a Geneva Convention party. Someone relying on Indonesian-issued driving documents should verify Korean legal recognition and the rental company's written acceptance before paying. Without valid driving authorization, do not drive.
```
Reason: P0 FAQ visible/schema exact parity required; preserve existing FAQ count.

### File — `id/rental-car.html`

#### REN10-ID01 — Quick-answer paragraph, original line 181
FROM:
```text
Pastikan persyaratan SIM, IDP, usia, kartu, dan asuransi terlebih dahulu.
```
TO:
```text
Periksa dulu apakah dokumen izin mengemudi yang Anda miliki diakui secara hukum di Korea. Setelah itu, pastikan syarat usia, kartu pembayaran, deposit, dan asuransi dari perusahaan rental.
```
Reason: Moves legal eligibility ahead of commercial requirements.

#### REN10-ID02 — Document-recognition paragraph, original line 302
FROM:
```text
Urus IDP sebelum bepergian. Periksa masa berlaku, golongan kendaraan yang tercantum, serta apakah negara penerbit dan format dokumennya diakui.
```
TO:
```text
Sebelum bepergian, periksa masa berlaku IDP, golongan kendaraan, negara penerbit, dan pengakuannya di Korea. Jangan menganggap SIM Internasional terbitan Indonesia otomatis berlaku: Indonesia menandatangani Konvensi Wina 1968 tetapi belum meratifikasinya, serta bukan negara pihak Konvensi Jenewa 1949. Jika Anda mengandalkan dokumen mengemudi terbitan Indonesia, pastikan lebih dahulu pengakuan hukumnya kepada otoritas Korea sebelum memesan mobil dengan biaya yang tidak dapat dikembalikan.
```
Reason: P0: applies to documents issued in Indonesia, not every Indonesian national.

#### REN10-ID03 — Law-versus-rental paragraph, original line 309
FROM:
```text
Operator dapat meminta dokumen tambahan, masa pengalaman mengemudi minimum, atau golongan SIM berbeda untuk kendaraan yang lebih besar. Mintalah konfirmasi tertulis jika situasi Anda tidak umum.
```
TO:
```text
Perusahaan rental juga dapat meminta dokumen tambahan, pengalaman mengemudi minimum, atau golongan SIM tertentu untuk kendaraan lebih besar. Pastikan secara terpisah bahwa izin mengemudi Anda diakui menurut hukum Korea, lalu minta konfirmasi tertulis bahwa perusahaan rental menerima dokumen yang sama. Jangan mengemudi jika izin tersebut tidak diakui.
```
Reason: Legal authorization and the operator's acceptance are independent checks.

#### REN10-ID04 — FAQ answer; visible plus JSON-LD, original line 572
FROM:
```text
Sebagian besar pengunjung jangka pendek sebaiknya membawa SIM asli, paspor, dan International Driving Permit resmi yang masih berlaku serta diakui Korea. Hukum Korea juga mengizinkan SIM asing tertentu yang diakui berdasarkan pengakuan timbal balik, tetapi perusahaan rental dapat menerapkan aturan dokumen yang lebih terbatas, jadi pastikan penerimaannya sebelum membayar.
```
TO:
```text
Sebagian besar pengunjung jangka pendek memerlukan SIM asli, paspor, dan izin mengemudi yang diakui secara hukum di Korea. Tidak semua IDP terbitan luar negeri memenuhi syarat: Indonesia belum meratifikasi Konvensi Wina dan bukan negara pihak Konvensi Jenewa. Jika hanya mengandalkan dokumen mengemudi terbitan Indonesia, pastikan pengakuan hukumnya serta persetujuan tertulis perusahaan rental sebelum membayar. Tanpa izin yang diakui, jangan mengemudi.
```
Reason: P0 FAQ visible/schema parity; does not assert an automatic ban for someone with independently recognized Korean or third-country authorization.

### File — `id/apps.html`

#### APP10-01 — Lead paragraph, original line 226
FROM:
```text
Untuk perjalanan singkat, Anda tidak perlu memenuhi ponsel dengan aplikasi Korea. Naver Map menangani sebagian besar navigasi lokal, Papago membantu ketika teks atau percakapan bahasa Korea menjadi kendala, dan k.ride berguna ketika taksi lebih mudah daripada transportasi umum.
```
TO:
```text
Untuk liburan singkat, Anda tidak perlu memasang semua aplikasi Korea. Mulailah dengan Naver Map untuk mencari tempat dan rute, Papago saat perlu memahami teks atau percakapan berbahasa Korea, serta k.ride jika perjalanan dengan taksi lebih praktis daripada transportasi umum.
```
Reason: Decision-first app shortlist, no extra apps or unsupported claims.

#### APP10-02 — Setup paragraph, original line 240
FROM:
```text
“Pasang sebelum tiba” berarti menyelesaikan pengaturan dasar dan menguji fitur yang ingin digunakan, bukan sekadar mengunduh aplikasinya.
```
TO:
```text
Memasang aplikasi sebelum tiba tidak cukup jika Anda belum mencobanya. Jika diperlukan, masuk ke akun atau atur bahasanya, lalu uji fitur yang akan digunakan selagi koneksi internet masih stabil.
```
Reason: Turns abstract guidance into actions while preserving original intent.

#### APP10-03 — Map search recovery paragraph, original line 339
FROM:
```text
Nama tempat dalam bahasa Inggris → nama tempat dalam bahasa Korea → alamat jalan Korea → nomor telepon. Untuk pilihan moda rute dan cara menggunakan peta lebih lanjut, baca
```
TO:
```text
nama tempat dalam bahasa Inggris → nama resmi dalam bahasa Korea → alamat jalan Korea → nomor telepon. Untuk memilih rute dan membaca peta lebih lanjut, lihat
```
Reason: More natural user action sequence; no location or app function changes.

#### APP10-04 — H3: mobile transit, original line 471
FROM:
```text
Mobile Tmoney bergantung pada perangkat dan cara isi ulang
```
TO:
```text
T-money di ponsel: periksa perangkat dan cara isi saldonya
```
Reason: Natural Indonesian while keeping compatibility and funding as the same decision.

### File — `checklist.html`

#### CHK10-EN01 — Entry-document checklist span, original line 244
FROM:
```text
Confirm passport validity and the visa or K-ETA requirements for your nationality.
```
TO:
```text
Check passport validity and the visa or K-ETA rules that apply to your passport. Separately check whether an e-Arrival Card is required; the official e-arrivalcard.go.kr service is free and accepts submissions within three days before arrival.
```
Reason: P1-URGENT: names e-Arrival Card separately without equating visa, K-ETA and arrival declaration.

#### CHK10-EN02 — Final-document checklist span, original line 617
FROM:
```text
Entry documents, accommodation booking and insurance details are saved offline.
```
TO:
```text
Your entry documents, any required e-Arrival Card submission, accommodation booking and insurance details are saved offline.
```
Reason: Keep required arrival declaration in practical departure checklist.

#### CHK10-EN03 — FAQ first answer; visible plus JSON-LD, original line 677
FROM:
```text
Confirm entry requirements, mobile data, airport transport, payment backups and your accommodation route. Save booking, insurance and emergency information offline before departure.
```
TO:
```text
Check the visa or K-ETA rules for your passport and whether you need an e-Arrival Card. If required, submit it free on the official site within three days before arrival. Prepare mobile data, airport transport, payment backups and your accommodation route; save bookings, insurance and emergency contacts offline.
```
Reason: P1-URGENT, FAQ visible/schema parity, nationality/status conditional.

### File — `id/checklist.html`

#### CHK10-ID01 — Entry-document checklist span, original line 244
FROM:
```text
Pastikan masa berlaku paspor serta persyaratan visa atau K-ETA untuk kewarganegaraan Anda.
```
TO:
```text
Periksa masa berlaku paspor dan aturan visa atau K-ETA sesuai paspor Anda. Secara terpisah, periksa apakah Anda wajib mengisi e-Arrival Card; layanan resmi e-arrivalcard.go.kr gratis dan bisa diajukan dalam tiga hari sebelum tiba.
```
Reason: P1-URGENT: explicit e-Arrival Card, not a universal requirement for all Indonesians.

#### CHK10-ID02 — Final-document checklist span, original line 617
FROM:
```text
Dokumen masuk, pemesanan akomodasi, dan rincian asuransi sudah disimpan agar tersedia tanpa internet.
```
TO:
```text
Dokumen masuk, bukti pengajuan e-Arrival Card jika diwajibkan, pemesanan penginapan, dan rincian asuransi sudah disimpan agar bisa diakses tanpa internet.
```
Reason: Travel-day practical record, no invented obligation.

#### CHK10-ID03 — FAQ first answer; visible plus JSON-LD, original line 677
FROM:
```text
Pastikan persyaratan masuk, data seluler, transportasi bandara, pembayaran cadangan, dan rute ke akomodasi. Simpan informasi pemesanan, asuransi, dan keadaan darurat agar tersedia tanpa internet sebelum keberangkatan.
```
TO:
```text
Periksa ketentuan visa atau K-ETA sesuai paspor Anda dan apakah Anda perlu e-Arrival Card. Jika wajib, ajukan gratis melalui situs resmi dalam tiga hari sebelum tiba. Siapkan data seluler, transportasi bandara, pembayaran cadangan, dan rute ke penginapan; simpan pemesanan, asuransi, serta kontak darurat agar bisa diakses tanpa internet.
```
Reason: P1-URGENT; Indonesian travel checklist query, same facts as EN correction.

### File — `id/payments.html`

#### PAY10-01 — Hero lead, original line 179
FROM:
```text
Kartu yang diterbitkan di luar negeri dapat digunakan untuk banyak kebutuhan selama perjalanan ke Korea, tetapi kartu yang berhasil digunakan di hotel atau jaringan toko tetap bisa gagal di toko kecil, pembayaran daring, atau isi ulang transportasi. Sedikit uang tunai dan saldo transportasi terpisah membantu menutup kekurangan ini tanpa membuat rencana pembayaran menjadi rumit.
```
TO:
```text
Kartu terbitan luar negeri bisa dipakai untuk banyak pembayaran di Korea, tetapi berhasil membayar di hotel tidak menjamin kartu yang sama diterima di kios kecil, situs lokal, atau mesin isi saldo transportasi. Siapkan sedikit uang tunai, metode pembayaran cadangan, dan saldo transportasi yang cukup agar masalah satu transaksi tidak mengganggu perjalanan.
```
Reason: Shorter, more direct payment-friction judgment; no universal card claim.

#### PAY10-02 — Foreign card section paragraph, original line 275
FROM:
```text
Pembayaran yang berhasil di satu usaha tidak menjamin kartu yang sama dapat digunakan di tempat berikutnya. Terminal penjual, jaringan kartu, pengaturan penerbit, dan cara autentikasi pembayaran semuanya dapat memengaruhi hasil.
```
TO:
```text
Kartu yang diterima di satu toko bisa ditolak di tempat lain. Hasilnya bergantung pada terminal, jaringan kartu, pengaturan bank penerbit, dan cara verifikasi yang diminta saat membayar.
```
Reason: Removes literal repetition, retains all four determinants.

#### PAY10-03 — H3: transport balance, original line 290
FROM:
```text
Dana transportasi mungkin berada dalam saldo terpisah
```
TO:
```text
Saldo transportasi bisa terpisah dari saldo belanja
```
Reason: Makes prepaid/transit distinction understandable.

#### PAY10-04 — H2: tipping and bill splitting, original line 327
FROM:
```text
Tip &amp; membagi tagihan
```
TO:
```text
Memberi tip dan membagi tagihan
```
Reason: Fixes misleading reading of 'tip' as advice rather than gratuity.

#### PAY10-05 — Gratuities paragraph, original line 328
FROM:
```text
Tip tidak diharapkan dan bukan kebiasaan di Korea. Biaya layanan jarang ditambahkan. Membagi tagihan merupakan hal umum, dan banyak restoran menerima pembayaran kartu secara terpisah. Ucapkan "ttaro gyesanhae juseyo" untuk meminta pembayaran terpisah.
```
TO:
```text
Memberi tip bukan kebiasaan di Korea, dan biaya layanan jarang ditambahkan. Membagi tagihan cukup umum; banyak restoran juga mengizinkan pembayaran dengan beberapa kartu. Untuk meminta pembayaran terpisah, katakan "ttaro gyesanhae juseyo".
```
Reason: Natural distinction between tipping and splitting the bill; preserved phrase and charge conditions.

## 4. Mandatory implementation and staged QA

- Confirm EN/ID exact source blobs and expected strings for every patch at the named branch HEAD. Every `FROM` must match only the authorized locations; any unexpected duplicate, version drift or markup conflict is a blocker for that mapping.
- After simulated replacement, reverse all approved replacements and require byte-identical original source for each of 7 HTML files.
- Validate H1/H2/H3 counts, HTML tag order, all `href`, asset paths, affiliate/tracking, structural `class/id/data-*`, numeric facts and original recommendation judgments.
- FAQ visible/JSON-LD exact wording parity after correction on the 2 English factual-correction pages and both Indonesian sibling pages; JSON parse errors 0.
- No changes to the common header/nav/footer, `common.js`, shared `style.css`, lang/canonical/hreflang/sitemap or existing user working tree.
- Indonesian market distinction: do not turn passport nationality into driving-document issuing country, and do not assert all Indonesian travellers qualify or cannot qualify for e-Arrival or rentals.
- English factual corrections are **only** `rental-car.html` and `checklist.html` as enumerated above. Every other English public page remains locked.
- Existing Batch01–09 DONE/CONTENT LOCK unchanged. No Production until 58/58, Technical Closure and user approval.
- Project-supplied standards ACTIVE Public Master v1.4/Navigation v1.1 versus repository older v1.3/v1.0 and Localization v2.0 (Handover lists v2.1): note drift, do not modify standards in this batch.

## 5. Approval and implementation gate

**APPROVED PUBLIC COPY — CONTENT LOCKED.** User approved all 27 exact FROM → TO mappings (31 literal replacement occurrences) across precisely 7 HTML files (5 Indonesian + `rental-car.html` and `checklist.html` English factual corrections). No repeat wording approval, no extra supplement. Codex must verify actual source fingerprints and preexisting user changes, then perform exact replacements, Static QA and staged QA. If and only if QA passes, stage **the seven explicitly approved HTML files plus this one final Approved MD**, commit and push only the current Indonesian feature branch. Never use `git add .` or `git add -A`, `git restore`, `git reset`, `git clean`, stash or force push. Preserve deleted/untracked/protected files, all previous Batch01–09 locks, and common UI. **No main merge, Vercel, Production, Technical Closure, lang/canonical/hreflang/sitemap changes or browser QA claims** under this approval.
