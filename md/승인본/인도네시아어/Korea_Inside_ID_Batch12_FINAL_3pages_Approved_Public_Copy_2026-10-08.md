# Korea Inside — Indonesian Batch 12 FINAL (3 pages) — Approved Public Copy

**Date:** 2026-10-08  
**Status:** **APPROVED PUBLIC COPY — CONTENT LOCKED**  
**User approval:** `다음` (2026-10-08), following the Batch 12 FINAL three-page Review  
**Single wording Source of Truth:** **THIS FILE ONLY** — 28 case-sensitive FROM → TO mappings at 28 exact source occurrences  
**Language / audience:** Indonesian (`id`) — first-time Korea travelers, food/culture, beauty shopping/appointments  
**Implementation target:** `id-localization-2026-10-06` (feature branch only; no main merge or Production)  
**Baseline HEAD:** `4ef0d6ed56f35dd4d23a0c0f54c09c59bc69da10`  
**Scope:** exactly the three existing Indonesian pages 56–58: `taste-korea.html`, `k-beauty.html`, `index.html` under `id/`.  
**Research:** Indonesian SEO research 58/58 COMPLETE per existing research handover. The separate Batch12 FINAL research MD was not retrievable in this working environment; cross-check that existing local research before implementation. Do **not** redo the research or invent keyword volumes, KD or rankings.  
**Output:** one final user-approved Public Copy MD. Codex must perform only the exact implementation of this document; no independent translation or editorial decisions.  

## 1. Verified source baseline and page roles

Current English Production HTML on `main` matched the identical English Git blobs on the Indonesian feature branch for all three source pages (3/3). Existing ID source files are first-pass working HTML (3/3).

| # | Indonesian target | English Git blob | Indonesian Git blob | H1/H2/H3 (EN=ID) | P (EN=ID) | Visible FAQ / FAQPage JSON-LD | Images (EN=ID) | Corrections |
|---|---|---|---|---:|---:|---|---:|---:|
| 56 | `id/taste-korea.html` | `371dc96a9879afd5efb6454d16ec927f8a4cfbe7` | `334dfbb1478602bc0aa6eebd11db443f8736850c` | 1/6/27 | 93 | 8 visible / 0 FAQPage JSON-LD | 10/10 | 12 |
| 57 | `id/k-beauty.html` | `7a9df7712ebffe7b28633f3dffdbad958b6a71e8` | `3804f73166ef04e0db03ed04b52c095b71c5f48b` | 1/5/12 | 58 | 8 visible / 0 FAQPage JSON-LD | 7/7 | 8 |
| 58 | `id/index.html` | `c5614145c6b8439378d77f1a2a6e228df6aece20` | `a78ef2e278bf2d63fb65b5abdb58f8205956b660` | 1/3/5 | 24 | 0 visible / 0 FAQPage JSON-LD | 7/7 | 8 |

**Structural comparison:** each English–Indonesian pair has identical HTML tag order, heading/paragraph counts, section/order, and image count. All `href` and `src`/`srcset` sequences match after normalizing the legitimate relative `../` path difference. Existing `lang="id"` is already correct on all three ID files. **Do not touch current canonical/hreflang/sitemap, language switcher, or common UI** during this Batch; final integration/Technical Closure is separate and user-authorized only.

**Page roles / SEO ownership (preserve, not extend):**
- `taste-korea.html`: Korea food-experience decision hub. Market vs barbecue vs cafés vs regional/seafood vs classes; Seoul, Busan, Jeonju and Jeju; Seoul-area food/hotel trade-offs. NOT a restaurant ranking, halal guarantee or standalone city-itinerary substitute.
- `k-beauty.html`: Shopping, personal color, salon/scalp care, clinical consultation, Seoul area, and reordering via Olive Young US versus Global. Preserve clinician-risk/recovery cautions and conditional shipping eligibility.
- `index.html`: Indonesia-language Korea Inside homepage / first-visit navigation hub. Maintain where-to-stay, connectivity, airport route, food and K-Beauty handoffs; do not turn it into a generic keyword list or duplicate its hub pages.

**Protected cross-market safeguards:**
- No unverified halal certification or Muslim-friendly guarantee. Vegetarian, allergy, and religion-related dietary needs require vendor-specific confirmation.
- Keep all provider/brand/restaurant/street/station/city names, hotel/transport trade-offs, prices, numbers, dates, opening times and geographic constraints unchanged unless an individual approved FROM→TO below explicitly covers the visible phrasing.
- OLIVE YOUNG Global shipping **to Indonesia is not assumed**; the approved-draft wording explicitly requires confirming whether the delivery address is supported. OLIVE YOUNG US remains US-only. Do not change any existing affiliate or product link.
- Do not present beauty clinic consultation as a guaranteed procedure outcome, and do not imply the writer personally visited any shop/clinic/restaurant.

## 2. Exact targeted corrections — 28 mappings / 28 source occurrences

**Application rule:** exact, case-sensitive `FROM → TO` within the named file only; each occurrence is expected exactly once. No other visible text, user-facing attributes, HTML structure, FAQ/question count, URLs or scripts may change. Line numbers refer to the baseline source Git blob above; source matching is authoritative.

### Page 56 — `id/taste-korea.html`

**Mappings:** 12, **source occurrences:** 12.

#### TK12-01 — H2: food-experience choice (source line 711)

**FROM (exact; case-sensitive):**
```text
Pengalaman seperti apa yang Anda inginkan dari perjalanan kuliner Korea?
```

**TO (approved final public copy):**
```text
Pengalaman kuliner seperti apa yang ingin Anda coba di Korea?
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Makes the choice natural in Indonesian while retaining the markets/BBQ/café/regional-food intent.

#### TK12-02 — Choice introduction: pace versus setting (source line 712)

**FROM (exact; case-sensitive):**
```text
Pasar, barbeku, kafe, dan kuliner daerah menghasilkan hari yang sangat berbeda. Sebagian singkat dan penuh energi; yang lain lebih masuk akal jika Anda memberi waktu agar kawasan — atau bahkan kota lain — menjadi bagian pengalaman.
```

**TO (approved final public copy):**
```text
Pasar, barbeku, kafe, dan kuliner daerah menawarkan pengalaman yang berbeda. Ada yang cocok untuk singgah sebentar, sementara yang lain lebih berkesan jika Anda meluangkan waktu menjelajahi kawasan atau kota di sekitarnya.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Removes a literal construction without assigning all markets a fixed pace.

#### TK12-03 — H2: dining setting and rhythm (source line 723)

**FROM (exact; case-sensitive):**
```text
Ritme makan berbeda di setiap tempat.
```

**TO (approved final public copy):**
```text
Suasana dan cara menikmati makanan berbeda menurut tempatnya.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Clarifies the same dining-setting distinction without ranking experiences.

#### TK12-04 — Image caption label: night dining (source line 704)

**FROM (exact; case-sensitive):**
```text
Barbeku · Makan malam
```

**TO (approved final public copy):**
```text
Barbeku · Makan larut malam
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Restores the English caption’s late-night distinction; keep its media and layout unchanged.

#### TK12-05 — H3: café neighborhood (source line 740)

**FROM (exact; case-sensitive):**
```text
Kawasan menjadi bagian hari menikmati kafe
```

**TO (approved final public copy):**
```text
Menikmati kafe sekaligus menjelajahi kawasannya
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Naturalizes an awkward heading while retaining the neighborhood-plus-café role.

#### TK12-06 — Café paragraph: local walking context (source line 716)

**FROM (exact; case-sensitive):**
```text
Hari menikmati kafe di Seoul sering berkaitan dengan jalan di sekitar kafe sama besarnya dengan kopi itu sendiri. Seongsu, Yeonnam, dan Ikseon-dong memadukan kafe dengan kawasan yang sangat berbeda.
```

**TO (approved final public copy):**
```text
Di Seoul, menikmati kafe sering berarti sekaligus menjelajahi jalan-jalan di sekitarnya, bukan hanya mencoba kopinya. Seongsu, Yeonnam, dan Ikseon-dong masing-masing menawarkan suasana kawasan yang berbeda.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Keeps the Seongsu/Yeonnam/Ikseon-dong comparison without travelogue claims.

#### TK12-07 — H3: traditional and regional food context (source line 746)

**FROM (exact; case-sensitive):**
```text
Kuliner daerah dimulai dari tempat di sekitarnya
```

**TO (approved final public copy):**
```text
Kuliner khas daerah lebih berkesan saat Anda menjelajahi kotanya
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Explains the role of city setting, not a dish-only ranking.

#### TK12-08 — H3: guided cooking and food experiences (source line 758)

**FROM (exact; case-sensitive):**
```text
Kelas dan jalan-jalan kuliner menambah konteks pada makanan
```

**TO (approved final public copy):**
```text
Kelas memasak dan tur kuliner membantu Anda memahami makanan lokal
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Makes the tour/class distinction readable in Indonesian.

#### TK12-09 — Food-tour trade-off sentence within paragraph (source line 759)

**FROM (exact; case-sensitive):**
```text
Tur kuliner kelompok kecil mengurangi sebagian kebebasan sebagai gantinya menawarkan beberapa pencicipan dan lebih banyak penjelasan sepanjang perjalanan.
```

**TO (approved final public copy):**
```text
Dalam tur kuliner kelompok kecil, Anda bisa mencicipi hidangan di beberapa tempat sambil mendapat penjelasan, tetapi rute dan waktunya lebih terikat daripada saat berjalan sendiri.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Preserves the guidance-versus-independence trade-off; no additional venue claims.

#### TK12-10 — Busan–Jeju geographic trade-off within paragraph (source line 755)

**FROM (exact; case-sensitive):**
```text
Busan mendekatkan pelabuhan, pasar, dan kota ke meja makan; Jeju menghubungkan makanan dengan pesisir dan jalan antartempat.
```

**TO (approved final public copy):**
```text
Di Busan, pasar, pelabuhan, dan tempat makan bisa menjadi bagian dari satu perjalanan yang berdekatan; di Jeju, pengalaman kuliner lebih berkaitan dengan pesisir dan perjalanan antartempat.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Explains compact Busan versus spread-out Jeju without modifying prices or routes.

#### TK12-11 — H2: Seoul neighborhood and food route (source line 807)

**FROM (exact; case-sensitive):**
```text
Di Seoul, pilihan kawasan mengubah perjalanan kuliner.
```

**TO (approved final public copy):**
```text
Di Seoul, pilihan kawasan ikut membentuk pengalaman kuliner Anda.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Natural phrasing; the underlying area and hotel recommendations remain intact.

#### TK12-12 — Visible dietary FAQ answer (source line 923)

**FROM (exact; case-sensitive):**
```text
Ya. Resep, saus, dan cara persiapan berbeda-beda, sehingga alergi, kebutuhan vegetarian, dan aturan makanan agama perlu dibahas dengan jelas sebelum memesan kelas atau makanan. Jangan menganggap hidangan sesuai hanya dari nama atau tampilannya.
```

**TO (approved final public copy):**
```text
Ya. Resep, saus, dan cara mengolah makanan bisa berbeda-beda. Jika Anda memiliki alergi, kebutuhan vegetarian, atau pantangan makanan berdasarkan agama, sampaikan dengan jelas sebelum memesan hidangan atau kelas memasak. Nama dan tampilan hidangan saja tidak menjamin bahan maupun cara pengolahannya sesuai.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Clarifies advance disclosure; does not label any restaurant/dish halal or certified.


### Page 57 — `id/k-beauty.html`

**Mappings:** 8, **source occurrences:** 8.

#### KB12-01 — Hero: walk-in shopping versus scheduled services (source line 93)

**FROM (exact; case-sensitive):**
```text
Kegiatan K-Beauty biasanya terbagi menjadi dua: produk yang bisa dilihat-lihat kapan pun ada waktu, dan layanan yang sebaiknya dijadwalkan sebelum perjalanan. Jika menemukan produk favorit, simpan nama produk, warna, dan ukuran yang tepat selagi kemasannya masih di depan Anda.
```

**TO (approved final public copy):**
```text
Rencana K-Beauty biasanya terbagi dua: produk yang bisa Anda lihat kapan saja, dan layanan yang lebih baik dipesan sebelum perjalanan. Jika menemukan produk favorit, catat nama, nomor warna, dan ukurannya selagi kemasannya masih bisa Anda lihat.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Keeps walk-in shopping versus booking and product-reordering details.

#### KB12-02 — Skincare: department-store wording (source line 108)

**FROM (exact; case-sensitive):**
```text
Cabang Olive Young tersebar di Seoul, sedangkan pusat perbelanjaan serba ada, toko utama merek, dan toko khusus yang lebih kecil menawarkan pilihan produk dan kisaran harga lebih luas.
```

**TO (approved final public copy):**
```text
Toko Olive Young tersebar di Seoul. Department store, gerai utama merek, dan toko khusus yang lebih kecil menawarkan pilihan produk serta kisaran harga yang lebih beragam.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Replaces a misleading literal Indonesian rendering of department stores.

#### KB12-03 — Choosing Seoul areas by the beauty plan (source line 133)

**FROM (exact; case-sensitive):**
```text
Bagian Seoul yang paling berguna tergantung kegiatan Anda yang sebenarnya. Myeongdong menyediakan beragam belanja dalam area ringkas, Seongsu lebih menarik jika menemukan merek baru menjadi bagian hari Anda, dan Seoul selatan lebih praktis ketika beberapa janji layanan sudah dipesan.
```

**TO (approved final public copy):**
```text
Pilih kawasan Seoul sesuai rencana K-Beauty Anda. Myeongdong memudahkan belanja banyak merek dalam satu area, Seongsu lebih menarik untuk menemukan merek baru, sedangkan Seoul bagian selatan lebih praktis jika beberapa layanan sudah dipesan sebelumnya.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Retains the Myeongdong/Seongsu/southern Seoul distinction and appointment friction.

#### KB12-04 — Seongsu: beauty, cafés, fashion and design (source line 151)

**FROM (exact; case-sensitive):**
```text
Kunjungan toko kecantikan mudah dipadukan dengan hari menikmati kafe, mode, dan toko desain.
```

**TO (approved final public copy):**
```text
Anda bisa menggabungkan kunjungan ke toko kecantikan dengan kafe, belanja mode, dan gerai desain dalam satu hari.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Natural Indonesian phrasing; does not add shops or pop-ups.

#### KB12-05 — H3: destination-dependent Olive Young storefront (source line 168)

**FROM (exact; case-sensitive):**
```text
Olive Young mengoperasikan toko daring berbeda menurut tujuan pengiriman.
```

**TO (approved final public copy):**
```text
Pilih toko online Olive Young berdasarkan negara tujuan pengiriman
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Puts the shipping decision first without altering the two store brands.

#### KB12-06 — Olive Young US/Global shipping for Indonesian readers (source line 169)

**FROM (exact; case-sensitive):**
```text
Wisatawan dengan pengiriman ke Amerika Serikat sebaiknya menggunakan OLIVE YOUNG US, sedangkan tujuan lain yang didukung umumnya dilayani melalui OLIVE YOUNG Global. Ketersediaan, biaya pengiriman, dan promosi bisa berbeda antara kedua toko.
```

**TO (approved final public copy):**
```text
Jika ingin menerima pesanan di Indonesia, periksa dahulu apakah alamat Anda termasuk tujuan pengiriman yang didukung OLIVE YOUNG Global. OLIVE YOUNG US melayani pengiriman di Amerika Serikat, sedangkan OLIVE YOUNG Global melayani tujuan lain yang didukung. Ketersediaan produk, biaya kirim, dan promosi dapat berbeda.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Keeps US-store facts and conditional Global support; does NOT assert Indonesia is currently shippable. Affiliate URLs stay unchanged.

#### KB12-07 — Visible FAQ: repeat purchase / Indonesia destination (source line 208)

**FROM (exact; case-sensitive):**
```text
Ya. Banyak produk K-beauty bisa dipesan kembali setelah Anda pulang. Pembeli di Amerika Serikat bisa menggunakan OLIVE YOUNG US, sedangkan wisatawan di negara lain yang didukung bisa memesan melalui OLIVE YOUNG Global. Ketersediaan produk dan kelayakan pengiriman bisa berbeda menurut tujuan.
```

**TO (approved final public copy):**
```text
Bisa, untuk produk yang tersedia melalui penjual atau tujuan pengiriman yang didukung. Jika Anda memesan ke Indonesia, periksa pilihan produk, kelayakan alamat, biaya kirim, pajak, dan ketentuan impor di OLIVE YOUNG Global. OLIVE YOUNG US melayani alamat di Amerika Serikat; ketersediaan dan ketentuan pengiriman tetap berbeda menurut tujuan.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** User-relevant conditional shipping check; preserves import fees / delivery restrictions, no availability promise.

#### KB12-08 — Stay-location decision closing paragraph (source line 221)

**FROM (exact; case-sensitive):**
```text
Pandang hotel sebagai bagian rute harian, bukan cara untuk tinggal di dalam satu “kawasan K-Beauty.”
```

**TO (approved final public copy):**
```text
Pilih lokasi hotel berdasarkan rute harian Anda, bukan sekadar agar berada di dalam satu “kawasan K-Beauty”.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Humanizes accommodation choice without reopening stay page recommendations.


### Page 58 — `id/index.html`

**Mappings:** 8, **source occurrences:** 8.

#### H12-01 — Homepage hero: first-day decisions (source line 153)

**FROM (exact; case-sensitive):**
```text
Perjalanan pertama ke Korea terasa jauh lebih mudah jika beberapa hal praktis sudah disiapkan sebelum mendarat. Mengetahui tempat menginap, cara menuju hotel dari bandara, cara terhubung ke internet, serta pilihan transportasi dan pembayaran bisa mengurangi banyak tekanan pada hari pertama.
```

**TO (approved final public copy):**
```text
Perjalanan pertama ke Korea lebih mudah dijalani jika beberapa hal praktis sudah siap sebelum mendarat. Tentukan tempat menginap, rute dari bandara, koneksi internet, serta cara menggunakan transportasi dan membayar kebutuhan sehari-hari agar hari pertama tidak merepotkan.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Concrete traveler decisions replace literal anxiety phrasing; no airport schedule or product claims.

#### H12-02 — Homepage H2: purpose-led trip planning (source line 162)

**FROM (exact; case-sensitive):**
```text
Mulai dari alasan Anda ingin datang.
```

**TO (approved final public copy):**
```text
Mulailah dari hal yang paling ingin Anda lakukan.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Natural answer-first direction without changing the home hub role.

#### H12-03 — Homepage K-Beauty teaser: flagship detour (source line 197)

**FROM (exact; case-sensitive):**
```text
Belanja produk kecantikan Korea mudah terasa rumit karena toko terbesar menyediakan jauh lebih banyak daripada yang dibutuhkan sebagian besar wisatawan. Yang berguna adalah mengetahui tempat untuk melihat-lihat, perbedaan antarkawasan, dan kapan toko utama sebuah merek memang layak dikunjungi meski perlu memutar.
```

**TO (approved final public copy):**
```text
Belanja K-Beauty bisa membingungkan karena toko besar menawarkan jauh lebih banyak produk daripada yang perlu Anda beli. Yang lebih berguna adalah mengetahui area untuk membandingkan produk, perbedaan tiap kawasan, dan kapan toko utama sebuah merek layak didatangi meskipun tidak searah dengan rute Anda.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Preserves the compare-versus-detour choice and the link to the K-Beauty hub.

#### H12-04 — Homepage H2: three arrival-day choices (source line 207)

**FROM (exact; case-sensitive):**
```text
Beberapa pilihan membuat hari kedatangan jauh lebih mudah
```

**TO (approved final public copy):**
```text
Tiga keputusan yang membuat hari kedatangan lebih mudah
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** The source introduces exactly three decisions: accommodation base, connectivity, airport-to-hotel route.

#### H12-05 — Homepage H3: accommodation beyond room rate (source line 222)

**FROM (exact; case-sensitive):**
```text
Tempat menginap memengaruhi lebih dari sekadar hotel
```

**TO (approved final public copy):**
```text
Lokasi menginap memengaruhi lebih dari pilihan hotel
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Keeps airport arrival, late-night return, shopping bags and cross-city journeys as trade-offs.

#### H12-06 — Homepage H3: door-to-hotel airport route (source line 254)

**FROM (exact; case-sensitive):**
```text
Rencanakan perjalanan dari bandara ke hotel Anda yang sebenarnya
```

**TO (approved final public copy):**
```text
Rencanakan rute dari bandara sampai ke hotel Anda
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Natural phrasing of actual-hotel endpoint; does not add a route or ETA.

#### H12-07a — First-hour paragraph: existing Checklist anchor visible text (source line 271)

**FROM (exact; case-sensitive):**
```text
Setelah mengambil bagasi, sebagian besar wisatawan membutuhkan beberapa hal yang sama
```

**TO (approved final public copy):**
```text
Setelah mengambil bagasi, periksa kembali daftar persiapan Anda
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** Change **text inside existing checklist.html link only**, not its href/tag or location.

#### H12-07b — First-hour paragraph: plain text immediately after existing anchor (source line 271)

**FROM (exact; case-sensitive):**
```text
: ponsel yang bisa digunakan, cara membayar transportasi lokal, peta yang berfungsi baik di Korea, dan rute yang jelas ke kota. Setelah semuanya siap, biasanya tidak perlu tinggal lebih lama di bandara.
```

**TO (approved final public copy):**
```text
: internet ponsel aktif, cara membayar transportasi sudah tersedia, aplikasi peta Korea bisa digunakan, dan rute ke kota sudah jelas. Jika semuanya siap, Anda biasanya tidak perlu berlama-lama di bandara.
```

**Expected occurrences in this file:** 1.  
**Editorial rationale / protected judgment:** The colon and following text are outside the existing checklist link. Preserve the <a> and its href.

## 3. Approved implementation QA contract and pre-implementation safety

**Pre-implementation independent read-only simulated QA — PASS (2026-10-08):** all 28 case-sensitive FROM values were applied to their intended source copies in memory; no Git or local HTML writes were performed. The exact reverse application recovered source bytes (3/3). The checks below were executed and passed; Codex must rerun against actual local files and staged content:
- **28/28** unique FROM strings match precisely **28/28** original locations; no preexisting TO overlap.
- Reversing the proposed changes exactly reproduces source HTML **byte for byte: 3/3 PASS**.
- H1/H2/H3/H4, HTML tag order, IDs, classes, data-* and inline scripts are preserved.
- `href`, affiliate URL/tracking, canonical/hreflang, `src`, `srcset`, image and figure/caption structure remain unchanged.
- **Visible FAQ counts PASS:** Taste Korea 8; K-Beauty 8; homepage 0. **FAQPage JSON-LD counts PASS:** 0/0/0, consistent with English source. No JSON-LD parse errors in the simulated output. Do not invent FAQPage schema or treat its absence as a QA defect.
- Existing homepage checklist anchor remains the same `<a href="checklist.html">…</a>`; edit anchor text only for H12-07a and the adjacent plain text only for H12-07b.
- **Technical Closure note (not authorized now):** `id/index.html` is a physical file for the Indonesian home. Its public canonical should ultimately be `/id/` (and avoid a second `/id/index.html` sitemap/home entry) in the separately authorized final integration, not in Batch 12 wording edits.
- **This document is the single final Approved Public Copy — CONTENT LOCKED.** No supplement, reinterpretation, independent Codex rewrite or second wording approval is needed unless an actual source drift, clear factual error or user direction requires reopening.

**Existing working tree protections (reported, not directly observed):** preserve the user’s deleted 7 paths, untracked 34 paths (as of Batch11, plus any new additions), Vietnamese SEO MD 12, all Batches 01–11 materials and all unrelated edits. Verify `git status --short` afresh before any future exact implementation. Never use `git add .`, `git add -A`, `git restore`, `git reset`, `git clean`, `git stash` or force-push.

**Standards version state:** latest uploaded Public Master v1.4 and Navigation v1.1 versus repository-local ACTIVE v1.3 and v1.0; Localization v2.0 physically present while research/Handover references v2.1. This discrepancy is recorded and not resolved by rewriting any Standard. Apply the actual highest-priority ACTIVE user-held standards and report any material conflict, without reopening previously locked Batches.

**Authorized scope after user wording approval:** Codex may apply the exact 28 mappings to the three specified Indonesian HTML files, QA them, and commit/push only those three HTML files plus this Approved MD on the existing Indonesian feature branch. **Not authorized:** main merge, Vercel/Production, common UI edits, or Technical Closure.

## 4. Approval and next checkpoint

**Current status:** **APPROVED PUBLIC COPY — CONTENT LOCKED**. The user approved the exact 28 mappings with `다음` on 2026-10-08. This final file is `Korea_Inside_ID_Batch12_FINAL_3pages_Approved_Public_Copy_2026-10-08.md`. Implement only the three existing Indonesian HTML files listed in §1 and this single Approved MD. Keep the approved FROM/TO strings, their application order and their source-specific occurrence counts unchanged.

**Do not count 58/58 implemented yet:** Batches 01–11 = 55/58 Git checkpoint complete. Batch 12 = 3 pages **CONTENT LOCKED / implementation pending**. After Codex exact implementation, Static QA, staged QA, feature-branch commit/push and independent Git verification, count 58/58. **Technical Closure is separately authorized only after that**; main merge/Vercel Production and live browser QA require explicit subsequent user approval.


## 5. Approved Git checkpoint manifest — Batch 12 only

**Company PC repository:** `C:\Projects\Koreainside`  
**Branch:** `id-localization-2026-10-06`  
**Expected pre-implementation HEAD:** `4ef0d6ed56f35dd4d23a0c0f54c09c59bc69da10`. Read-only GitHub recheck at approval found the feature branch identical to this SHA; Codex must recheck its actual local HEAD and user edits before implementation.

**Allowed changed/staged files — exactly four:**

1. `id/taste-korea.html` — TK12-01 through TK12-12 (12 exact occurrences).
2. `id/k-beauty.html` — KB12-01 through KB12-08 (8 exact occurrences).
3. `id/index.html` — H12-01 through H12-06, H12-07a and H12-07b (8 exact occurrences).
4. `md/승인본/인도네시아어/Korea_Inside_ID_Batch12_FINAL_3pages_Approved_Public_Copy_2026-10-08.md` — preserve original attached bytes and verify SHA-256 before staging.

**Sequence:** verify project/source/working tree and active Standards → save this MD without edits → source SHA/28/28 count check → exact replacements → static QA → explicit four-file staged manifest → staged QA → commit → push only existing feature branch → HEAD/origin/ahead-behind confirmation.

**QA:** 28/28 exact mappings, 28/28 positions, exact reverse source reconstruction 3/3, visible FAQ counts `8/8/0`, FAQPage JSON-LD counts `0/0/0` (absence intentional), correct `lang="id"` on three pages, H1/H2/H3/H4, structural/tag order, existing scripts, links, asset URLs and affiliate/tracking parity, `git diff --check`, no unapproved modifications. Check all case-sensitive replacements against baseline source; preserve the existing homepage Checklist anchor structure, HTML name and all link targets.

**Protect:** all pre-existing user changes, reported deleted 7/untracked 34 (plus newly added), Vietnamese SEO MD 12 and all previous Batches 01–11; actual `git status --short` is authoritative. Never use `git add .`, `git add -A`, `git restore`, `git reset`, `git clean`, `git stash`, or force push. No changes outside four explicitly approved files; no canonical/hreflang/sitemap, global headers, navigation, `common.js` or shared `style.css`. No main merge, Vercel/Production, or Technical Closure.

**Completion report:** four-file manifest; mappings/positions 28/28; reverse-byte QA; structure, FAQ, JSON-LD and links; staged QA; exact commit SHA; actual HEAD/origin/ahead-behind; protected working-tree changes; no Production.
