# Korea Inside — Indonesian Batch 07 — Approved Public Copy

**Date:** 2026-10-08  
**Status:** **APPROVED PUBLIC COPY — CONTENT LOCKED**  
**User approval:** `진행` (2026-10-08), following Batch 07 second-pass Review presentation  
**Branch target:** `id-localization-2026-10-06` — **feature branch only**  
**Implementation scope:** exactly 5 existing `id/*.html` working copies, 18 approved inner-HTML replacements  
**Wording Source of Truth:** **THIS FILE ONLY** for the Batch 07 second-pass changes  
**Approach:** existing Indonesian first-pass text stays unchanged outside these 18 targets; no full-page retranslation.

## 1. Approved scope and immutable reference fingerprints

| Indonesian page (relative to repository root) | English source Git blob SHA | Pre-review Indonesian Git blob SHA | Approved changes |
|---|---|---|---:|
| `id/where-to-stay-in-seongsu.html` | `86728dfe5a701b335c27f68d39c5f1aa4618802f` | `61f48dfab61635abd00ce9961f2abafbc328118c` | 4 |
| `id/where-to-stay-in-itaewon.html` | `67f0d35104d324b08f68409616a8c30cb9e6f51d` | `31b68a3394f53b759af54536fa63b89b2c02f948` | 3 |
| `id/esim.html` | `e1db58b07322832c111653e549589f8bfa52e455` | `6f410260543079eb6b403de49b4a038358717cc2` | 3 |
| `id/best-esim-for-korea.html` | `f38dbde83532f7e51097822ef8acd7e4a11f57ae` | `a514aa8e82ed89127fd0b53e5109edb1be77fd05` | 4 |
| `id/korea-esim-with-phone-number.html` | `931a637fb1bf6935f7e6ed9acce922b259db83aa` | `8f7fa0b3406590076b1cdab92a0552606085df8b` | 4 |

These blob IDs identify the copies reviewed, **not permission to overwrite newer working-tree changes**. In the company-PC repository, verify branch/current git status and compare each candidate target with its exact FROM value before any write. Check whether source drift or user edits affect any target; do not reset/restore or silently replace such edits. If a target is missing or non-unique, defer that target and report exact evidence; do not infer new Indonesian wording.

**Editorial lock:** keep all unaffected Indonesian copy, titles, descriptions, H1s, other headings, facts, counts, hotel order/room configuration, station/exit routes, eSIM comparisons, affiliate CTA names/hrefs/tracking and recommendations exactly as currently implemented. For every approved mapping, the `FROM` and `TO` below are the exact inner HTML of the *same existing* source element; retain its surrounding tag and attributes. Replacements are **not** new sections or links.

## 1A. Scope-specific factual note — Saily

The 2026-10-08 Reviewed GitHub branch source already states **180-day Saily activation window** in both `best-esim-for-korea.html` and `id/best-esim-for-korea.html`. The existing 7/30-day plan **usage period** is distinct from the 180-day **activation deadline**. Preserve this existing 180-day statement and do not reinsert the old 30-day activation claim. The official-provider basis was checked during the Review (see source links in §5); verify again at the final Production QA if needed. **No Saily value change is authorized in these 18 replacements.**

---

## 2. PAGE 31 — `id/where-to-stay-in-seongsu.html`

**SEO role:** Decide whether to stay in Seongsu or make a day visit; practical hotel and room trade-offs.  
**SEO-facing Title, meta and H1:** KEEP CURRENT; no unsupported keyword rewrite.  
**Keep intact:** Seongsu vs other bases, Seongsu/Ttukseom/Seoul Forest station distinctions, POCO luggage charges, ONJAE no-lift and bed count, Stay BUT exact unit ambiguity, Seoul Forest Stay bathroom/check-in, AREX/bus door-to-door friction.

### S07-01 — Body paragraph, original line 193
FROM:
```html
Tidak ada jumlah minimum hari di Seongsu yang perlu Anda penuhi. Anda tetap bisa mengunjungi istana atau Myeongdong sambil menginap di Seongsu. Yang penting adalah apakah Anda nyaman dengan perjalanan pada hari-hari lain dan apakah penginapannya memenuhi kebutuhan Anda.
```
TO:
```html
Tidak ada keharusan menghabiskan jumlah hari tertentu di Seongsu. Anda tetap bisa pergi ke istana atau Myeongdong dari sini. Yang perlu dipertimbangkan adalah apakah perjalanan pada hari-hari lain masih nyaman dan penginapannya benar-benar sesuai kebutuhan Anda.
```
Reason: remove literal 'minimum number of days you need to qualify' phrasing, preserving conditional base decision.

### S07-02 — Quick-choice paragraph, original line 206
FROM:
```html
<strong>Hotel dekat Stasiun Seongsu:</strong> Hotel POCO menjadi titik awal. Perhatikan ukuran kamar sama cermatnya dengan lokasi.
```
TO:
```html
<strong>Hotel dekat Stasiun Seongsu:</strong> Bandingkan Hotel POCO terlebih dahulu, tetapi periksa ukuran kamarnya sama telitinya dengan lokasi.
```
Reason: practical instruction instead of abstract 'starting point'; preserve `<strong>`.

### S07-03 — Arrival paragraph, original line 351
FROM:
```html
Mencapai bangunan hanyalah sebagian dari proses kedatangan.
```
TO:
```html
Tiba di depan penginapan belum berarti seluruh proses kedatangan selesai.
```
Reason: Indonesian clarity before the following check-in, stairs and access-code checks.

### S07-04 — First-time FAQ answer, original line 380
FROM:
```html
Bisa saja. Status pengunjung pertama kali tidak otomatis membuatnya tidak sesuai.
```
TO:
```html
Bisa. Seongsu tidak otomatis menjadi pilihan yang kurang cocok hanya karena ini perjalanan pertama Anda ke Seoul.
```
Reason: eliminate the awkward literal 'first-time visitor status', without changing conditional judgment.

## 3. PAGE 32 — `id/where-to-stay-in-itaewon.html`

**SEO role:** Decide where to stay in Itaewon by actual nighttime plans, noise, slope, bed configuration, hotel-vs-hostel requirements.  
**SEO-facing Title, meta and H1:** KEEP CURRENT.  
**Keep intact:** Hamilton nightlife noise, Imperial Palace under-19 policy ambiguity, H HOSTEL and G Guesthouse access limits, Mondrian room-size/pool conditions, Grand Hyatt hill/airport-bus trade-offs. Do not add unverified halal-room or halal-breakfast claims.

### I07-01 — Noise advisory paragraph, original line 213
FROM:
```html
Permintaan kamar tetaplah permintaan. Jangan menyusun jadwal wisata pagi berdasarkan asumsi bahwa lantai yang lebih tinggi akan menghilangkan suara musik dari luar.
```
TO:
```html
Meminta kamar di lantai yang lebih tinggi tidak menjamin permintaan itu dipenuhi. Jangan merencanakan wisata pagi dengan anggapan bahwa suara musik dari luar pasti tidak terdengar.
```
Reason: explicit, traveler-facing risk without falsely promising quiet rooms.

### I07-02 — H HOSTEL opening paragraph, original line 300
FROM:
```html
H HOSTEL layak dibandingkan jika Anda ingin kamar sendiri dan fasilitas praktis daripada kolam hotel. Listing mencakup <strong>lift, sarapan, fasilitas laundry, dapur bersama, dan penitipan bagasi</strong>.
```
TO:
```html
Bagi yang lebih mementingkan kamar pribadi dan fasilitas sehari-hari daripada kolam renang hotel, H HOSTEL layak dibandingkan. Informasi penginapannya mencantumkan <strong>lift, sarapan, fasilitas laundry, dapur bersama, dan penitipan bagasi</strong>.
```
Reason: remove marketplace-English 'Listing', preserve amenities, trade-off and inline markup.

### I07-03 — Section H2, original line 343
FROM:
```html
Hotel dengan pengalaman menginap sebagai daya tarik
```
TO:
```html
Hotel yang dipilih untuk menikmati pengalaman menginapnya
```
Reason: more natural section title while retaining hotels chosen for the stay experience itself.

## 4. PAGE 33 — `id/esim.html`

**SEO role:** setup and practical use of eSIM/physical SIM/roaming; not the same purchase-comparison intent as Best eSIM.  
**SEO-facing Title, meta and H1:** KEEP CURRENT.  
**Keep intact:** compatibility/carrier unlock, original number and roaming costs, setup-before-arrival caveats, data usage, activation vs installation and Korean-number limits.

### E07-01 — Section H2, original line 215
FROM:
```html
Pengaturan eSIM Korea yang Sederhana untuk Sebagian Besar Perjalanan
```
TO:
```html
Pengaturan eSIM paling sederhana untuk kebanyakan perjalanan ke Korea
```
Reason: natural sentence-case Indonesian without changing the page role or claim.

### E07-02 — Usage-estimate paragraph, original line 372
FROM:
```html
Pertanyaan yang berguna bukan apakah paket terdengar besar atau kecil. Pikirkan apa yang akan dilakukan ponsel Anda pada hari biasa di Korea.
```
TO:
```html
Jangan terpaku pada kesan bahwa kuota paket itu besar atau kecil. Coba perkirakan penggunaan ponsel Anda selama satu hari biasa di Korea.
```
Reason: remove literal translation and maintain decision-first data-usage advice.

### E07-03 — Final choice H2, original line 577
FROM:
```html
Pengaturan Layanan Seluler Korea Mana yang Sebaiknya Dipilih?
```
TO:
```html
Layanan seluler apa yang sebaiknya Anda pilih di Korea?
```
Reason: clear, natural conclusion question; preserve alternatives.

## 5. PAGE 34 — `id/best-esim-for-korea.html`

**SEO role:** compare international data eSIM providers and data limits; no invented speed rankings or prices.  
**SEO-facing Title, meta and H1:** KEEP CURRENT.  
**Keep intact:** Ubigi/Saily/Airalo order, measured-vs-declared distinction, 25 GB/2 Mbps and other applicable published provider terms, hotspot/return/reissue caveats, phone-number eSIM as a separate decision.

### B07-01 — Price-value H2, original line 369
FROM:
```html
Hal yang lebih penting daripada harga utama
```
TO:
```html
Jangan hanya membandingkan harga yang tertera
```
Reason: remove literal 'headline price' phrasing; preserve what matters beyond price.

### B07-02 — Provider-comparison H2, original line 408
FROM:
```html
Perbedaan pilihan penyedia saat ini
```
TO:
```html
Perbedaan utama antarpenyedia saat ini
```
Reason: natural Indonesian heading with temporal qualifier retained.

### B07-03 — Final decision H2, original line 543
FROM:
```html
Pilih Berdasarkan Penggunaan Data, Lalu Periksa Ketentuan Paket
```
TO:
```html
Pilih berdasarkan pemakaian data, lalu periksa ketentuan paket
```
Reason: concise idiomatic wording; no change to comparison logic.

### B07-04 — Provider CTA section H2, original line 555
FROM:
```html
Lanjutkan ke paket penyedia
```
TO:
```html
Lihat paket dari masing-masing penyedia
```
Reason: clearer next action without changing any affiliate button, destination or ordering.

### Saily activation — fact check / KEEP (not a replacement)

The current English Git blob **and** current Indonesian Git blob already contain the Saily standard data-plan activation window as **180 days** in the provider comparison table. Do **not** revert to 30 days. This resolves the stale 30→180 correction instruction from the Handover for the currently inspected source copies. Official basis: https://saily.com/legal/terms-of-service/ and https://saily.com/download-esim-app/. Standard non-subscription plans activate upon arrival or are automatically activated after the 180-calendar-day deadline; subscription products activate immediately on purchase. Do not conflate the 7- or 30-day *usage duration* of a listed plan with the 180-day *activation window*. A web-indexed version of the public page still showed 30 days, so a **fresh live Production check at final release** remains necessary.

## 6. PAGE 35 — `id/korea-esim-with-phone-number.html`

**SEO role:** help travelers distinguish an 010 contact number, incoming calls/SMS, outbound service and resident identity verification; compare SK Telecom, KT and LG U+ without changing product details.  
**SEO-facing Title, meta and H1:** KEEP CURRENT.  
**Keep intact:** carrier activation methods, passport/airport step, family pass-through limits, product-specific voice/SMS availability, no resident-ID guarantees.

### N07-01 — Function-choice H2, original line 223
FROM:
```html
Perlukah menerima panggilan dan pesan saja, atau juga mengirimnya?
```
TO:
```html
Anda hanya perlu menerima telepon dan SMS, atau juga menelepon dan mengirim SMS?
```
Reason: unambiguous inbound vs outbound requirement in natural Indonesian.

### N07-02 — Korean identity H3, original line 241
FROM:
```html
Nomor 010 bukan identitas Korea
```
TO:
```html
Nomor 010 bukan bukti identitas penduduk Korea
```
Reason: prevent confusing a functioning mobile number with resident identity verification.

### N07-03 — Additional setup H2, original line 496
FROM:
```html
Kapan eSIM dengan Nomor Korea Sepadan dengan Pengaturan Tambahan
```
TO:
```html
Kapan eSIM dengan nomor Korea layak dipilih meski pengaturannya lebih rumit?
```
Reason: readable conditional trade-off, with no stronger recommendation.

### N07-04 — Korean-number functionality H2, original line 236
FROM:
```html
Fungsi yang sebenarnya diberikan nomor Korea 010
```
TO:
```html
Apa saja fungsi nomor Korea 010 bagi wisatawan?
```
Reason: clean Indonesian heading without implying any new function.

---

## 7. Mandatory local preflight — before implementation

1. Operate on the **existing company-PC primary repository**, `C:\Projects\Koreainside`, current branch `id-localization-2026-10-06`. **No temporary worktree** and no direct GitHub edits by ChatGPT.
2. Verify the authoritative local `Korea_Inside_Language_Localization_Standard.md` status/version and open the precompleted `Korea_Inside_ID_SEO_Competitive_Research_Batch07_5pages_2026-10-05.md` wherever actually stored. **Do not re-run SERP research.** The 2026-10-08 Handover states common v2.1, while the remote main/feature copies inspected during Review were ACTIVE v2.0; these differ. This was not resolved in the ChatGPT artifact session. Do not claim that v2.1 or the Research file was checked here. Report the exact local version/path and any substantive conflicts.
3. Record actual `git branch --show-current`, `git rev-parse HEAD`, `git status --short`, staged manifest and tracked/untracked baseline before writing. Handover baseline was **M0 / D7 / U29 / staged0**; do not assume the local state has remained unchanged. Protect all pre-existing changes.
4. For each of the 18 mapped elements, verify exact `FROM` inner HTML exists **once** in the intended page and matches the indicated same-element context; preserve its enclosing tag and attributes. If values or structural nodes have drifted, **do not improvise**, defer the affected item and report.
5. Check Saily 180-day statement remains intact in the English and Indonesian sources. If the actual local source has changed, report it as a specific fact issue; do not silently adjust facts in this approved wording batch.
6. Read local repository `AGENTS.md` / `docs/standards-hub.md` if present and honor nonconflicting scoped protection requirements.

## 8. Exact implementation and technical QA

- Apply **only 18 exact `FROM` → `TO` changes** to the 5 `id/` HTML files shown. No translation, stylistic improvement, reordering, new claims, edits to other paragraphs, modifications to English or other language pages, or affiliate links.
- Preserve every `class`, `id`, `data-*`, image, `srcset`, `href`, script, `style.css`, `common.js`, header, navigation, footer, mobile menu, CSS/JS functionality, JSON-LD/FAQ structure, and Saily value. Update no title/meta/H1 or other search-facing wording; those have no approved replacements in this Batch.
- **Technical Closure is NOT part of Batch 07**: preserve each page's existing `lang`, canonical, hreflang, sitemap, language-switcher and Indonesian internal-link status. Whole-language closing changes will be performed only after all 58/58 are done and authorized.
- Static QA: replacements **18/18 exact**, unmapped/ambiguous **0**, unintended copy changes **0**, valid HTML structure, source H1/H2/H3 count parity, FAQ/JSON-LD parity, images/assets and affiliate/tracking parity, `git diff --check` PASS. Inspect actual diff and ensure no out-of-scope changes caused by this operation.
- Protect the Handover's 7 deleted files, 29 untracked items, 12 Vietnamese SEO MDs, earlier Batch 01–06 `CONTENT LOCKED` material, and all other working-tree changes. Do not stage or modify anything outside this scope.
- If all checks pass, use explicit `git add` paths for **only these five HTML files plus this new approved MD** under `md/승인본/인도네시아어/`. Run `git diff --cached --check` and verify the exact staged manifest, then commit and push **only** to `origin/id-localization-2026-10-06`. If the approved MD was already tracked or has conflicting contents, do not overwrite it; report for reconciliation.
- **Forbidden:** `git add .`, `git add -A`, `git restore`, `git reset`, `git clean`, `git stash`, force push, changes to `main`, Vercel, Production, and partial deployment. Never clear pre-existing user changes merely to obtain a clean status.

## 9. Completion report format

Report: authoritative local Standard/version and Research filename/path; branch; old/new HEAD; preflight status; exact mapping **18/18** or IDs deferred; page-by-page counts **4/3/3/4/4**; structure/FAQ/schema and affiliate QA; Saily window preserved; touched/staged files; `git diff --check` and `git diff --cached --check`; checkpoint commit SHA and feature-branch ahead/behind; preservation of deleted/untracked/Vietnamese/earlier Approved material; `main/push/Production` status; any known QA limitation.

**Final state after user-approved implementation and checkpoint:** Batch 07 implemented, static QA PASS and feature-branch checkpoint complete; **not** Production complete. Do not mark all 58 Indonesian pages as Production COMPLETE.
