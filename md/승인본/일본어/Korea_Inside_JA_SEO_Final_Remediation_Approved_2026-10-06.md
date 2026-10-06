# Korea Inside — Japanese SEO Final Remediation Source

**Date:** 2026-10-06  
**Status:** USER-APPROVED REMEDIATION / CHATGPT FINAL COPY  
**Scope:** 8 approved remediation axes / 10 Japanese HTML files  
**Implementation target:** Current local working tree only  
**Git / stage / commit / push / Production:** NOT AUTHORIZED IN THIS STEP

## 0. Protection rules

- Do not implement these changes against GitHub `main` as a substitute for the current local Japanese SEO working tree.
- The current local Batch01–12 Japanese SEO changes and all pre-existing user changes must be preserved.
- Do not modify common header / navigation / footer / `common.js` / shared `style.css`.
- Do not use `git add .`, `git add -A`, `git restore`, `git reset`, `git clean`, `git stash`, or force push.
- For the two synchronization items below, the current local Japanese value is the Source of Truth because Batch01 SEO changes are not yet committed remotely.
- No change to Seoul Sky / AREX / Maps.

---

# 1. Hongdae breadcrumb ↔ H1 synchronization

**File:** `ja/hongdae-travel-guide.html`

## Approved action

The visible hero breadcrumb still uses the pre-SEO H1 wording.

Update only the breadcrumb lexical wording after `ホーム /` so that it matches the **current local H1 lexical wording**.

Implementation rule:

- Read the current local H1 in the same file after Batch01 SEO changes.
- Copy its wording to the breadcrumb.
- Preserve the existing `data-guide-year="current"` marker if the breadcrumb/H1 uses it.
- Do not change the H1, Title, Meta, Hero copy, section copy, navigation, or year automation.

**PASS condition:** breadcrumb wording = current local H1 wording, excluding only HTML markup differences.

---

# 2. K-Beauty OG metadata synchronization

**File:** `ja/k-beauty.html`

## Approved action

Batch01 changed local Title / Meta / H1, but old OG metadata remained.

### `og:title`

Set the local `og:title` to the **current local SEO title wording**, removing only a trailing ` | Korea Inside` suffix if the local `<title>` contains it.

### `og:description`

Set the local `og:description` to the **current local meta description exactly**.

Do not change the already-approved local Title / Meta / H1.

Do not add new Twitter tags if the local page does not already contain them.

---

# 3. K-Beauty return-home repurchase block — Japan-market correction

**File:** `ja/k-beauty.html`

## Official fact basis checked 2026-10-06

OLIVE YOUNG Global currently provides a Japanese-language / JPY shopping surface and Japan-specific sales / shipping information. Japan is therefore not correctly served by a US-vs-rest decision block.

Official reference:
- https://global.oliveyoung.com/
- https://global.oliveyoung.com/foot-info/footer-contents?foterMenuSeq=67

## 3.1 Replace the existing US-vs-Global store-choice block

Replace the content of the current `<aside class="kb-store-choice" ...>` under:

`韓国で買う？ 帰国後に買い直す？`

with the following Japan-market block.

```html
<aside class="kb-store-choice" aria-labelledby="kb-store-choice-reorder-title">
  <div class="kb-store-choice__head">
    <h3 id="kb-store-choice-reorder-title">日本に帰国したあと、OLIVE YOUNGで買い直すなら</h3>
    <p>日本向けにはOLIVE YOUNG Globalを利用できます。配送先を日本に設定し、日本向けの取扱商品、価格、送料、返品条件を注文前に確認してください。Global Mallは日本語表示と円表示に対応しています。</p>
  </div>
  <div class="kb-store-choice__layout">
    <div class="kb-store-choice__media">
      <img src="../images/k-beauty/k-beauty-makeup-product-display.webp" alt="リップ、リップグロス、ファンデーション、フェイスパウダーが並ぶ商品・色比較用ディスプレイ" width="2448" height="2448" loading="lazy" decoding="async">
    </div>
    <div class="kb-store-choice__content">
      <div class="kb-store-grid">
        <article class="kb-store-card kb-store-card--global">
          <p class="kb-store-card__region">日本への配送</p>
          <h4 class="kb-store-card__title">OLIVE YOUNG Global</h4>
          <p class="kb-store-card__description">日本へ帰国後に買い直す場合は、配送先を日本に設定してから、同じ商品・色番・容量が販売されているか確認します。商品、価格、送料、返品条件は注文時点の日本向け表示を基準にしてください。</p>
          <a class="kb-store-card__link" href="https://global.oliveyoung.com/" target="_blank" rel="noopener noreferrer" aria-label="OLIVE YOUNG Globalの日本向けストアを見る" data-store="olive-young-global" data-affiliate-track="true" data-affiliate-brand="olive_young_global" data-page-category="discover" data-content-topic="k_beauty" data-placement="kbeauty_store" data-link-stage="direct">OLIVE YOUNG Globalを見る</a>
        </article>
        <article class="kb-store-card kb-store-card--global">
          <p class="kb-store-card__region">買い直す前に確認</p>
          <h4 class="kb-store-card__title">商品名・色番・容量を照合</h4>
          <p class="kb-store-card__description">韓国で撮った商品名、色番、容量、パッケージ写真と照らし合わせてください。同じブランドでも日本向けの取扱商品やセット構成が異なる場合があります。</p>
        </article>
      </div>
    </div>
  </div>
</aside>
```

### Protection

- Remove the old `OLIVE YOUNG US` affiliate card from the Japanese page.
- Remove the old Olive Young US affiliate disclosure that becomes inapplicable.
- Do not invent or substitute a Japan affiliate URL.
- Preserve the image and all unrelated K-Beauty copy.

## 3.2 FAQ replacement

Replace only the answer to:

`帰国後に同じ商品を再購入できますか？`

with:

```text
はい。多くのKビューティー商品は帰国後も再購入できます。日本へ帰国した場合は、OLIVE YOUNG Globalで配送先を日本に設定し、対象商品、価格、送料、返品条件を確認できます。韓国で買った商品の正確な商品名、色番、容量を残しておくと、同じ商品を探しやすくなります。
```

Do not change the other FAQ items.

---

# 4. Airalo deleted-profile recovery wording

**File:** `ja/best-esim-for-korea.html`

## Official fact basis checked 2026-10-06

Airalo still states that eSIMs generally cannot be reinstalled after deletion. However, its current Help Center also says that after accidental deletion the app can check whether the **specific plan** can be reinstalled and, if possible, guide recovery; otherwise it may guide the user to Replacement or Support.

Official reference:
- https://www.airalo.com/help/using-managing-esims/ZSEEHBT5HW6F/posso-reinstalar-um-esim/54MPGHA4TCY1

## Approved replacement

In the Airalo comparison row, replace the old absolute wording:

```text
インストール状況、利用状況、返金理由により条件が異なる。削除済みプロファイルは通常再インストール不可
```

with:

```text
通常は削除後に再インストールできません。ただし誤って削除した場合は、Airaloアプリでそのプランを再インストールできるか確認でき、対応していれば復旧手順が表示されます。対応しない場合は、Replacementまたはサポートの案内に従います。
```

Also add the current Airalo reinstall/recovery Help Center link to the Airalo official-source line if the local page does not already contain it.

Do not change the provider order, recommendation ranking, pricing, plan data, affiliate structure, or other eSIM copy.

---

# 5. Luxury page — fill the empty hotel-comparison section

**File:** `ja/best-area-for-luxury-hotels-seoul.html`

The current section:

`<section class="airport-section airport-section--gray luxury-hotel-search" ...>`

has the H2 `ソウルの高級ホテルを比較` but no actual comparison content.

Keep the existing H2 and subtitle. Immediately below the existing section header, add:

```html
<div class="luxury-editorial-list">
  <article class="luxury-editorial-row">
    <h3>Park Hyatt Seoul — 江南・COEXを旅の中心にするなら</h3>
    <p>江南で高級ホテルを選ぶ代表例です。Park Hyatt SeoulはCOEXの向かいにあり、標準的なキング客室は42m²から。ソウル南部の予定、COEX、三成周辺を旅程の中心にするなら、ホテル立地へ料金を払う理由が分かりやすくなります。反対に、毎日の主目的が王宮や鍾路なら、有名なホテルという理由だけで選ぶと移動が増えます。</p>
    <p><a href="/ja/where-to-stay-in-gangnam.html">江南の宿泊ガイドで立地を確認する →</a></p>
  </article>

  <article class="luxury-editorial-row">
    <h3>SIGNIEL SEOUL — 蚕室・ロッテ複合施設を一日単位で使うなら</h3>
    <p>SIGNIEL SEOULは蚕室のLotte World Tower 76〜101階にあるホテルです。ロッテワールド、ソウルスカイ、買い物、石村湖など蚕室周辺で一日の多くを過ごす旅なら、ホテル自体を目的地の一部にしやすい立地です。中心部の王宮・鍾路を毎日往復する旅では、その距離を先に受け入れられるか確認してください。</p>
    <p><a href="/ja/where-to-stay-in-jamsil.html">蚕室の宿泊ガイドで立地を確認する →</a></p>
  </article>

  <article class="luxury-editorial-row">
    <h3>LOTTE HOTEL SEOUL Executive Tower — 明洞・乙支路の中心部を使うなら</h3>
    <p>中心部観光と高級ホテルを両立したいときの代表例です。Executive Towerは乙支路30にあり、ホテル公式では地下鉄2号線・乙支路入口駅8番出口から徒歩約1分と案内されています。客室はカテゴリーによって広さや含まれるサービスが異なるため、ホテル名だけでなく実際に予約する客室と料金プランを確認してください。</p>
    <p><a href="/ja/where-to-stay-in-myeongdong.html">明洞の宿泊ガイドで立地を確認する →</a></p>
  </article>
</div>

<p>この3軒は「ソウルの高級ホテル順位」ではありません。江南・蚕室・中心部という異なる旅程で、ホテル立地へ高い料金を払う理由がどう変わるかを見るための代表例です。実際の予約では、客室カテゴリー、ベッド構成、眺望、朝食・ラウンジ、キャンセル条件、空港や駅からの到着ルートをもう一度確認してください。</p>
```

## Official verification basis

- Park Hyatt Seoul: Hyatt official — COEX opposite / 185 rooms / standard King 42m².
- SIGNIEL SEOUL: LOTTE official — Lotte World Tower, 300 Olympic-ro, hotel floors 76–101.
- LOTTE HOTEL SEOUL Executive Tower: LOTTE official — 30 Eulji-ro; Line 2 Euljiro entrance Exit 8 about 1 minute; room categories and sizes vary.

Do not add dynamic prices, star-rank claims, review scores, or OTA inventory.

---

# 6. Payment BreadcrumbList JSON-LD — `/ja/` correction

**Files:**
1. `ja/foreign-credit-cards-korea.html`
2. `ja/korean-online-payments-foreigners.html`
3. `ja/korea-atm-foreign-cards.html`
4. `ja/apple-pay-korea.html`

Visible breadcrumb text remains unchanged.

In each page's `BreadcrumbList` JSON-LD, replace English-root item URLs with the Japanese sibling URLs.

## Exact URL rules

### Home

```text
https://www.getkoreainside.com/
```

→

```text
https://www.getkoreainside.com/ja/
```

### Payments parent

```text
https://www.getkoreainside.com/payments.html
```

→

```text
https://www.getkoreainside.com/ja/payments.html
```

### Current page

Use the same filename under `/ja/`.

Examples:

```text
https://www.getkoreainside.com/foreign-credit-cards-korea.html
```

→

```text
https://www.getkoreainside.com/ja/foreign-credit-cards-korea.html
```

Apply the same rule to the other three current-page URLs.

Do not change Breadcrumb names, canonical, visible copy, FAQ, or other schema.

---

# 7. Insadong Stay — Hanok decision layer

**File:** `ja/where-to-stay-in-insadong.html`

## Official fact basis checked 2026-10-06

VISITKOREA confirms that Bukchon / Anguk is a real Seoul Hanok-stay area and that traditional Hanok accommodation may use floor bedding while other properties provide beds / modernized facilities.

Official references:
- https://japanese.visitkorea.or.kr/svc/contents/infoBscView.do?vcontsId=140843
- https://japanese.visitkorea.or.kr/svc/contents/contentsView.do?menuSn=219&page=1&sContsTtl=&sort=regDt&vcontsId=251285

## Approved insertion

Inside `#quick-decision`, add one decision card after the existing quick-decision cards:

```html
<div class="hm-decision-card">
  <p><strong>韓屋に泊まること自体を旅の一部にしたいなら：</strong>安国・北村側の韓屋ステイも、一般的なホテルとは別枠で比較してください。伝統様式の宿には布団で寝るタイプがあり、ベッドや現代的な設備を備えた施設もあります。雰囲気だけで決めず、ベッド／布団、専用バスルーム、荷物保管、階段、駅から最後の徒歩を予約前に確認してください。</p>
</div>
```

Do not add a ranked Hanok-hotel list in this remediation.
Do not replace the existing Nine Tree / Sunbee / AMID / Dormy Inn / Moxy / apartment recommendations.

---

# 8. Card Declined — duplicate-payment prevention step

**File:** `ja/card-declined-korea.html`

## Fact basis

A payment screen timeout or unclear response does not prove that the original transaction failed. Re-submitting before checking the original transaction status can create duplicate processing.

## Approved insertion

Inside the list under:

`まず支払いを終わらせてから、原因を切り分ける`

insert the following item **before** the existing item that begins `支払いを終える必要があるなら、別の方法を使う。`

```html
<li><strong>結果が曖昧なら、同じ支払いを繰り返す前に注文履歴とカードアプリを確認する。</strong> 画面が止まったりタイムアウトしたりしても、最初の取引が未処理とは限りません。注文完了、利用通知、保留中（pending）の取引が見えているなら、同じ支払いをもう一度送る前に加盟店または発行会社で状態を確認してください。</li>
```

Do not alter the existing issuer / terminal / kiosk / online-payment distinctions.

---

# 9. Explicitly unchanged after review

Do not modify in this remediation:

- `ja/seoul-sky-guide.html`
- `ja/arex.html`
- `ja/maps.html`

Their reviewed current direction remains accepted.

---

# 10. Required local QA after implementation

After applying only the approved remediation above:

1. `git diff --check` = PASS.
2. Confirm exact changed-file manifest.
3. Confirm no common protected file changed.
4. Hongdae breadcrumb = current local H1 wording.
5. K-Beauty local Title / Meta / H1 unchanged; OG synchronized.
6. K-Beauty US-market repurchase framing = 0.
7. K-Beauty obsolete Olive Young US affiliate/disclosure in Japanese page = 0.
8. Airalo old absolute deleted-profile wording = 0.
9. Luxury hotel-comparison section contains 3 representative examples and no dynamic prices.
10. Four Payment BreadcrumbList pages point to `/ja/` Home / Payments / self URLs.
11. Insadong Hanok decision card exists once.
12. Card Declined duplicate-payment prevention item exists once.
13. Visible FAQ ↔ FAQPage parity remains unchanged on pages where FAQPage exists.
14. Affiliate / tracking changes are limited only to the intentionally removed Japan-irrelevant Olive Young US link.
15. Existing Batch01–12 Japanese SEO local changes remain intact.
16. Existing unrelated working-tree changes remain untouched.
17. Stage / commit / push / Production remain **0** until the final 58-page QA is complete and release is explicitly approved.

---

# 11. Next gate

After this remediation is locally applied:

**Final 58-page Japanese QA**
→ market-context residue scan  
→ Title / Meta / H1 / OG audit  
→ canonical / hreflang / sitemap / reciprocal cluster  
→ visible FAQ / FAQPage parity  
→ internal links  
→ affiliate / tracking  
→ desktop ~1440px rendering  
→ mobile ~390px rendering  
→ exact release manifest  

Only after that QA report passes does the workflow move to release approval.
