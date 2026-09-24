# Korea Inside — Japanese Localization Batch 2 — Airport

**Date:** 2026-09-24
**Status:** APPROVED PUBLIC COPY — CONTENT LOCKED
**Source MD:** `Korea_Inside_JA_Airport5_Localization_Source_Batch2_2026-09-24.md`
**Target:** `ja/airport.html`

## Scope

- `airport.html` Source ITEMs: **210 / 210**
- Japanese Golden Sample common UI reused exactly: **93 / 93**
- Airport page-specific Japanese editorial localization: **117 / 117**
- English source SHA-256: `E11D5A24D91796B47E02F4E72F60CE5FF3BF703F72B1DC4E7C3BABC28940960F`
- `ja/airport.html` matched the English source before localization.

## Japanese editorial-localization rules

- This is Japanese editorial localization, not literal translation.
- Facts, numbers, routes, terminal distinctions, operating-condition wording, recommendation logic, URLs, affiliate/tracking values, HTML structure, class/id/data attributes, images/srcset, schema structure, CSS and JS logic remain unchanged in meaning.
- Natural Japanese travel-language forms are used where appropriate: `Incheon Airport → 仁川空港`, `Seoul Station → ソウル駅`, `Hongdae → 弘大`, `Gongdeok → 孔徳`, `Myeongdong → 明洞`, `Terminal 1/2 → 第1/第2ターミナル`, `Airport Railroad → 空港鉄道`.
- Product/brand names such as `AREX`, `T-money`, `eSIM`, `Naver Map`, `KakaoMap`, and `Korea Inside` remain in their established brand form.
- Common UI is inherited exactly from the already CONTENT LOCKED Japanese Golden Sample and is not retranslated.

## ITEMs

### ITEM 0001

- Element/type: meta description
- Source target: `airport.html|html:nth-of-type(1) > head:nth-of-type(1) > meta:nth-of-type(3)@content`
- Reuse: NEW airport-specific localization

English:

```text
After immigration and customs, use this Incheon Airport arrival hall guide to connect, save your address, check payment and choose transport.
```

Japanese:

```text
入国審査と税関を終えたら、この仁川空港の到着ロビーガイドで、通信を確保し、宿泊先の住所を保存し、支払い手段を確認して、移動方法を選びましょう。
```

### ITEM 0002

- Element/type: title
- Source target: `airport.html|html:nth-of-type(1) > head:nth-of-type(1) > title:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Incheon Airport Arrival Hall Guide: First 30 Minutes | Korea Inside
```

Japanese:

```text
仁川空港 到着ロビーガイド：最初の30分 | Korea Inside
```

### ITEM 0003

- Element/type: literal ARIA label
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > a.logo:nth-of-type(1)@aria-label`
- Reuse: Japanese Golden Sample Approved ITEM 0003

English:

```text
Korea Inside home
```

Japanese:

```text
Korea Inside ホーム
```

### ITEM 0004

- Element/type: image alt
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > a.logo:nth-of-type(1) > img.site-brand__logo:nth-of-type(1)@alt`
- Reuse: Japanese Golden Sample Approved ITEM 0004

English:

```text
Korea Inside
```

Japanese:

```text
Korea Inside
```

### ITEM 0005

- Element/type: literal ARIA label
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > button#site-nav-toggle@aria-label`
- Reuse: Japanese Golden Sample Approved ITEM 0005

English:

```text
Open menu
```

Japanese:

```text
メニューを開く
```

### ITEM 0006

- Element/type: literal ARIA label
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation@aria-label`
- Reuse: Japanese Golden Sample Approved ITEM 0006

English:

```text
Primary navigation
```

Japanese:

```text
メインナビゲーション
```

### ITEM 0007

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(1) > button#site-nav-trigger-discover::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0007

English:

```text
DISCOVER
```

Japanese:

```text
楽しむ
```

### ITEM 0008

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(1) > div#site-nav-panel-discover > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0008

English:

```text
Taste Korea
```

Japanese:

```text
Taste Korea
```

### ITEM 0009

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(1) > div#site-nav-panel-discover > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0009

English:

```text
K-Beauty
```

Japanese:

```text
K-Beauty
```

### ITEM 0010

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > button#site-nav-trigger-travel::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0010

English:

```text
Travel
```

Japanese:

```text
旅行ガイド
```

### ITEM 0011

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0011

English:

```text
Hongdae
```

Japanese:

```text
弘大
```

### ITEM 0012

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0012

English:

```text
Myeongdong
```

Japanese:

```text
明洞
```

### ITEM 0013

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0013

English:

```text
Seongsu
```

Japanese:

```text
聖水
```

### ITEM 0014

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(4)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0014

English:

```text
Insadong
```

Japanese:

```text
仁寺洞
```

### ITEM 0015

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(5)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0015

English:

```text
Gangnam
```

Japanese:

```text
江南
```

### ITEM 0016

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(6)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0016

English:

```text
Jamsil
```

Japanese:

```text
蚕室
```

### ITEM 0017

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(7)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0017

English:

```text
Gongdeok & Mapo
```

Japanese:

```text
孔徳・麻浦
```

### ITEM 0018

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(8)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0018

English:

```text
Itaewon
```

Japanese:

```text
梨泰院
```

### ITEM 0019

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(9)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0019

English:

```text
Dongdaemun
```

Japanese:

```text
東大門
```

### ITEM 0020

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > p.site-nav__group-label:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0020

English:

```text
Seoul Areas
```

Japanese:

```text
ソウルのエリア
```

### ITEM 0021

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(2) > div.site-nav__travel-attraction-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0021

English:

```text
Lotte World
```

Japanese:

```text
ロッテワールド
```

### ITEM 0022

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(2) > div.site-nav__travel-attraction-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0022

English:

```text
Seoul Sky
```

Japanese:

```text
ソウルスカイ
```

### ITEM 0023

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(2) > p.site-nav__group-label:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0023

English:

```text
Attractions
```

Japanese:

```text
観光スポット
```

### ITEM 0024

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > p#site-nav-travel-guides-label::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0024

English:

```text
Travel Guides
```

Japanese:

```text
旅行ガイド
```

### ITEM 0025

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > button#site-nav-trigger-stay::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0025

English:

```text
Stay
```

Japanese:

```text
宿泊
```

### ITEM 0026

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0026

English:

```text
Stay Guide
```

Japanese:

```text
宿泊ガイド
```

### ITEM 0027

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(10)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0027

English:

```text
Luxury Hotels
```

Japanese:

```text
高級ホテル
```

### ITEM 0028

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0028

English:

```text
Hongdae vs Myeongdong
```

Japanese:

```text
弘大 vs 明洞
```

### ITEM 0029

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0029

English:

```text
First-Time Visitors
```

Japanese:

```text
初めてのソウル
```

### ITEM 0030

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(4)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0030

English:

```text
Families
```

Japanese:

```text
家族旅行
```

### ITEM 0031

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(5)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0031

English:

```text
Solo Travelers
```

Japanese:

```text
ひとり旅
```

### ITEM 0032

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(6)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0032

English:

```text
Couples
```

Japanese:

```text
カップル
```

### ITEM 0033

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(7)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0033

English:

```text
Budget Travelers
```

Japanese:

```text
節約派
```

### ITEM 0034

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(8)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0034

English:

```text
Shopping
```

Japanese:

```text
ショッピング
```

### ITEM 0035

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(9)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0035

English:

```text
Nightlife
```

Japanese:

```text
ナイトライフ
```

### ITEM 0036

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > p.site-nav__group-label:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0036

English:

```text
Stay Guide
```

Japanese:

```text
宿泊ガイド
```

### ITEM 0037

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(4) > button#site-nav-trigger-esim::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0037

English:

```text
eSIM
```

Japanese:

```text
eSIM
```

### ITEM 0038

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(4) > div#site-nav-panel-esim > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0038

English:

```text
eSIM Guide
```

Japanese:

```text
eSIMガイド
```

### ITEM 0039

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(4) > div#site-nav-panel-esim > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0039

English:

```text
Best eSIM for Korea
```

Japanese:

```text
韓国旅行におすすめのeSIM
```

### ITEM 0040

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(4) > div#site-nav-panel-esim > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0040

English:

```text
Korea eSIM with a Phone Number
```

Japanese:

```text
電話番号付き韓国eSIM
```

### ITEM 0041

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > button#site-nav-trigger-airport::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0041

English:

```text
Airport
```

Japanese:

```text
空港
```

### ITEM 0042

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > div#site-nav-panel-airport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0042

English:

```text
Airport Guide
```

Japanese:

```text
空港ガイド
```

### ITEM 0043

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > div#site-nav-panel-airport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0043

English:

```text
Arrival Guide
```

Japanese:

```text
到着ガイド
```

### ITEM 0044

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > div#site-nav-panel-airport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0044

English:

```text
Airport Transfer
```

Japanese:

```text
空港送迎
```

### ITEM 0045

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > div#site-nav-panel-airport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(4)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0045

English:

```text
AREX Guide
```

Japanese:

```text
AREXガイド
```

### ITEM 0046

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > div#site-nav-panel-airport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(5)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0046

English:

```text
Airport Bus Guide
```

Japanese:

```text
空港バスガイド
```

### ITEM 0047

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(6) > button#site-nav-trigger-maps::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0047

English:

```text
Maps
```

Japanese:

```text
地図
```

### ITEM 0048

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(6) > div#site-nav-panel-maps > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0048

English:

```text
Maps Guide
```

Japanese:

```text
地図アプリガイド
```

### ITEM 0049

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > button#site-nav-trigger-transport::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0049

English:

```text
Transport
```

Japanese:

```text
交通
```

### ITEM 0050

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0050

English:

```text
T-money Guide
```

Japanese:

```text
T-moneyガイド
```

### ITEM 0051

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0051

English:

```text
WOWPASS Guide
```

Japanese:

```text
WOWPASSガイド
```

### ITEM 0052

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0052

English:

```text
T-money vs WOWPASS
```

Japanese:

```text
T-money vs WOWPASS
```

### ITEM 0053

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(1) > p.site-nav__group-label:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0053

English:

```text
Travel Cards
```

Japanese:

```text
交通カード
```

### ITEM 0054

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(2) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0054

English:

```text
Taxi Guide
```

Japanese:

```text
タクシーガイド
```

### ITEM 0055

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(2) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0055

English:

```text
Call Van / Private Transfer
```

Japanese:

```text
コールバン／貸切送迎
```

### ITEM 0056

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(2) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0056

English:

```text
Rental Car
```

Japanese:

```text
レンタカー
```

### ITEM 0057

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(2) > p.site-nav__group-label:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0057

English:

```text
Other Transport
```

Japanese:

```text
その他の交通
```

### ITEM 0058

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(8) > button#site-nav-trigger-apps::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0058

English:

```text
Apps
```

Japanese:

```text
アプリ
```

### ITEM 0059

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(8) > div#site-nav-panel-apps > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0059

English:

```text
Essential Apps
```

Japanese:

```text
必須アプリ
```

### ITEM 0060

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(9) > button#site-nav-trigger-travel-tips::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0060

English:

```text
Travel Tips
```

Japanese:

```text
旅行準備
```

### ITEM 0061

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(9) > div#site-nav-panel-travel-tips > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0061

English:

```text
Korea Travel Checklist
```

Japanese:

```text
韓国旅行チェックリスト
```

### ITEM 0062

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(9) > div#site-nav-panel-travel-tips > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0062

English:

```text
Paying in Korea
```

Japanese:

```text
韓国での支払い
```

### ITEM 0063

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > div.language-switcher:nth-of-type(1) > button.language-switcher__button:nth-of-type(1) > span.language-switcher__current:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0063

English:

```text
EN
```

Japanese:

```text
JA
```

### ITEM 0064

- Element/type: common UI header/navigation direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > div.language-switcher:nth-of-type(1) > button.language-switcher__button:nth-of-type(1) > span.language-switcher__label:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0064

English:

```text
Language
```

Japanese:

```text
日本語
```

### ITEM 0065

- Element/type: literal ARIA label
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > div.language-switcher:nth-of-type(1)@aria-label`
- Reuse: Japanese Golden Sample Approved ITEM 0065

English:

```text
Language selector
```

Japanese:

```text
言語選択
```

### ITEM 0066

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-page-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.airport-page-hero__copy:nth-of-type(1) > p.page-hero__breadcrumb:nth-of-type(1) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Home
```

Japanese:

```text
ホーム
```

### ITEM 0067

- Element/type: breadcrumb direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-page-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.airport-page-hero__copy:nth-of-type(1) > p.page-hero__breadcrumb:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
/ Airport
```

Japanese:

```text
/ 空港
```

### ITEM 0068

- Element/type: H1
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-page-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.airport-page-hero__copy:nth-of-type(1) > h1.airport-page-hero__title:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Your First 30 Minutes in the Incheon Airport Arrival Hall
```

Japanese:

```text
仁川空港の到着ロビーで最初の30分にやること
```

### ITEM 0069

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-page-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.airport-page-hero__copy:nth-of-type(1) > p.airport-page-hero__desc:nth-of-type(2)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
This guide begins once immigration, baggage claim and customs are behind you and you have entered the public arrival hall. Before heading into the city, take a few minutes to make sure your phone can connect, keep your accommodation details in a form people in Korea can use, set up a payment backup and look at the whole route to your stay.
```

Japanese:

```text
このガイドは、入国審査、手荷物受取、税関を終えて一般の到着ロビーに出たところから始まります。市内へ向かう前に数分だけ時間を取り、スマホが通信できるか確認し、韓国で相手に伝わる形で宿泊先情報を保存し、予備の支払い手段を用意して、宿泊先までの移動ルート全体を確認しておきましょう。
```

### ITEM 0070

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-page-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.airport-page-hero__copy:nth-of-type(1) > div.airport-quick-links:nth-of-type(1) > a.airport-pill:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
First steps
```

Japanese:

```text
最初にやること
```

### ITEM 0071

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-page-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.airport-page-hero__copy:nth-of-type(1) > div.airport-quick-links:nth-of-type(1) > a.airport-pill:nth-of-type(2)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Transport
```

Japanese:

```text
移動手段
```

### ITEM 0072

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-page-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.airport-page-hero__copy:nth-of-type(1) > div.airport-quick-links:nth-of-type(1) > a.airport-pill:nth-of-type(3)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Problems
```

Japanese:

```text
困ったとき
```

### ITEM 0073

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-page-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.airport-page-hero__copy:nth-of-type(1) > div.airport-quick-links:nth-of-type(1) > a.airport-pill:nth-of-type(4)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
FAQ
```

Japanese:

```text
FAQ
```

### ITEM 0074

- Element/type: image alt
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-page-hero:nth-of-type(1) > div.container:nth-of-type(1) > figure.airport-page-hero__photo:nth-of-type(1) > img:nth-of-type(1)@alt`
- Reuse: NEW airport-specific localization

English:

```text
Illustrated first 30 minutes in the Incheon Airport arrival hall: connect, save your address, check payment and choose transport
```

Japanese:

```text
仁川空港の到着ロビーで過ごす最初の30分を示すイラスト：通信確認、住所保存、支払い確認、移動手段選び
```

### ITEM 0075

- Element/type: H2
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#first-steps > div.container:nth-of-type(1) > div.section__header:nth-of-type(1) > h2.section__title:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
The Four Things Worth Sorting Out Before You Leave the Terminal
```

Japanese:

```text
ターミナルを出る前に済ませたい4つのこと
```

### ITEM 0076

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#first-steps > div.container:nth-of-type(1) > div.section__header:nth-of-type(1) > p.section__subtitle:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
None of these takes long, but each is easier to sort out while airport Wi-Fi, information desks and a comfortable place to stop are still close by. T-money is useful for buses and subways, although it can wait when your first ride is a taxi, pre-booked transfer or rental car.
```

Japanese:

```text
どれも時間はかかりませんが、空港Wi-Fiや案内カウンター、落ち着いて立ち止まれる場所が近くにあるうちに済ませるほうが簡単です。T-moneyはバスや地下鉄に便利ですが、最初の移動がタクシー、事前予約の送迎、レンタカーなら、すぐに用意しなくても構いません。
```

### ITEM 0077

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#first-steps > div.container:nth-of-type(1) > ol.airport-first-step-list:nth-of-type(1) > li:nth-of-type(1) > div:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Make sure mobile data works away from Wi-Fi
```

Japanese:

```text
Wi-Fiを切ってもモバイルデータが使えるか確認する
```

### ITEM 0078

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#first-steps > div.container:nth-of-type(1) > ol.airport-first-step-list:nth-of-type(1) > li:nth-of-type(1) > div:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
The arrival hall Wi-Fi can hide a problem with roaming or an eSIM. Turn Wi-Fi off briefly and open a map or web page over mobile data instead of trusting the LTE or 5G symbol alone. If nothing loads, it is much easier to follow the provider's instructions or contact support before leaving the terminal.
```

Japanese:

```text
到着ロビーのWi-Fiにつながっていると、ローミングやeSIMの不具合に気づかないことがあります。Wi-Fiを一度切り、LTEや5Gの表示だけを信じず、モバイルデータで地図やウェブページが実際に開くか確認してください。読み込めない場合は、ターミナルを出る前のほうが事業者の案内を確認したりサポートへ連絡したりしやすくなります。
```

### ITEM 0079

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#first-steps > div.container:nth-of-type(1) > ol.airport-first-step-list:nth-of-type(1) > li:nth-of-type(1) > div:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Open the eSIM guide
```

Japanese:

```text
eSIMガイドを見る
```

### ITEM 0080

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#first-steps > div.container:nth-of-type(1) > ol.airport-first-step-list:nth-of-type(1) > li:nth-of-type(2) > div:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Keep your destination in a form people can use
```

Japanese:

```text
目的地を、現地で伝わる形で保存する
```

### ITEM 0081

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#first-steps > div.container:nth-of-type(1) > ol.airport-first-step-list:nth-of-type(1) > li:nth-of-type(2) > div:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
An English hotel name may not be enough for a taxi driver or a search in a Korean map app. Save the Korean place name, Korean road address, phone number and booking confirmation together, then keep a screenshot available offline.
```

Japanese:

```text
英語のホテル名だけでは、タクシー運転手に伝わらなかったり、韓国の地図アプリで検索しにくかったりすることがあります。韓国語の施設名、韓国語の道路名住所、電話番号、予約確認書をまとめて保存し、オフラインでも見られるスクリーンショットを用意しておきましょう。
```

### ITEM 0082

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#first-steps > div.container:nth-of-type(1) > ol.airport-first-step-list:nth-of-type(1) > li:nth-of-type(2) > div:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Open the maps guide
```

Japanese:

```text
地図ガイドを見る
```

### ITEM 0083

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#first-steps > div.container:nth-of-type(1) > ol.airport-first-step-list:nth-of-type(1) > li:nth-of-type(3) > div:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Give yourself a payment backup
```

Japanese:

```text
予備の支払い手段を用意する
```

### ITEM 0084

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#first-steps > div.container:nth-of-type(1) > ol.airport-first-step-list:nth-of-type(1) > li:nth-of-type(3) > div:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
A long arrival becomes more stressful when one card is the only way to pay. Keep a second card or some cash separate from your main card so a decline at a machine or counter does not block the rest of the journey.
```

Japanese:

```text
長時間の移動後に、支払い方法が1枚のカードしかないとトラブル時の負担が大きくなります。メインカードとは別にもう1枚のカードか現金を分けて持っておけば、券売機や窓口で決済できなくても、その後の移動まで止めずに済みます。
```

### ITEM 0085

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#first-steps > div.container:nth-of-type(1) > ol.airport-first-step-list:nth-of-type(1) > li:nth-of-type(3) > div:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Open the payments guide
```

Japanese:

```text
支払いガイドを見る
```

### ITEM 0086

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#first-steps > div.container:nth-of-type(1) > ol.airport-first-step-list:nth-of-type(1) > li:nth-of-type(4) > div:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Look at the whole route, not only the first train or bus
```

Japanese:

```text
最初の電車やバスだけでなく、宿泊先までの全行程を見る
```

### ITEM 0087

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#first-steps > div.container:nth-of-type(1) > ol.airport-first-step-list:nth-of-type(1) > li:nth-of-type(4) > div:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
The fastest or cheapest first leg can still end with another transfer, stairs or a long walk with luggage. Your destination, arrival time, group size and final route from the stop or station matter more than the headline journey alone.
```

Japanese:

```text
最初の区間が最速・最安でも、その後に乗り換え、階段、荷物を持った長い徒歩移動が続くことがあります。最初の電車やバスの所要時間だけでなく、目的地、到着時刻、人数、停留所や駅から宿泊先までの最後の移動まで含めて考えるほうが大切です。
```

### ITEM 0088

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#first-steps > div.container:nth-of-type(1) > ol.airport-first-step-list:nth-of-type(1) > li:nth-of-type(4) > div:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
See how the airport transfer choices differ
```

Japanese:

```text
空港からの移動手段の違いを見る
```

### ITEM 0089

- Element/type: H2
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-official-map-link-wrap:nth-of-type(3) > div.container:nth-of-type(1) > div.airport-official-map-link:nth-of-type(1) > div:nth-of-type(1) > h2.section__title:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Use the Map for the Terminal You Actually Arrived At
```

Japanese:

```text
実際に到着したターミナルの地図を使う
```

### ITEM 0090

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-official-map-link-wrap:nth-of-type(3) > div.container:nth-of-type(1) > div.airport-official-map-link:nth-of-type(1) > div:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Terminal 1 and Terminal 2 have different layouts, so first confirm the terminal shown for your flight. Then switch the official map to that terminal and the arrivals floor before looking for transport, an information desk or another facility.
```

Japanese:

```text
第1ターミナルと第2ターミナルは構造が異なります。まず自分の便の到着ターミナルを確認し、公式マップをそのターミナルと到着階に切り替えてから、交通機関、案内カウンター、その他の施設を探してください。
```

### ITEM 0091

- Element/type: CTA/link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-official-map-link-wrap:nth-of-type(3) > div.container:nth-of-type(1) > div.airport-official-map-link:nth-of-type(1) > div.airport-official-map-link__action:nth-of-type(2) > a.airport-official-map-link__button:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Open the Official Airport Map
```

Japanese:

```text
空港公式マップを開く
```

### ITEM 0092

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-official-map-link-wrap:nth-of-type(3) > div.container:nth-of-type(1) > div.airport-official-map-link:nth-of-type(1) > div.airport-official-map-link__action:nth-of-type(2) > p.airport-official-map-link__tip:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
The map may open on a different terminal or floor. Match both settings to your current arrival before following it.
```

Japanese:

```text
マップを開いたときに、別のターミナルや階が表示されることがあります。案内に従う前に、ターミナルと階の両方を自分が今いる到着場所に合わせてください。
```

### ITEM 0093

- Element/type: H2
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#arrival-guide > div.container:nth-of-type(1) > div.airport-arrival-bridge:nth-of-type(1) > div:nth-of-type(1) > h2.section__title:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Still Working Through Immigration or Baggage Claim?
```

Japanese:

```text
まだ入国審査や手荷物受取の途中ですか？
```

### ITEM 0094

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#arrival-guide > div.container:nth-of-type(1) > div.airport-arrival-bridge:nth-of-type(1) > div.airport-arrival-bridge__body:nth-of-type(2) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
If you are still looking for transfer, immigration, baggage claim or customs instructions, start with the Arrival Guide. This page begins only after you have entered the public arrival hall and are ready to sort out the practical first steps of the trip.
```

Japanese:

```text
乗り継ぎ、入国審査、手荷物受取、税関の流れを確認したい場合は、まず到着ガイドを見てください。このページは、一般の到着ロビーに出て、旅行開始直後の実用的な準備をする段階から始まります。
```

### ITEM 0095

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#arrival-guide > div.container:nth-of-type(1) > div.airport-arrival-bridge:nth-of-type(1) > div.airport-arrival-bridge__body:nth-of-type(2) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Follow the route from the aircraft to the arrival hall
```

Japanese:

```text
機内を降りて到着ロビーに出るまでの流れを見る
```

### ITEM 0096

- Element/type: H2
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > div.section__header:nth-of-type(1) > h2.section__title:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Getting from the Airport to Your Stay
```

Japanese:

```text
空港から宿泊先までの移動
```

### ITEM 0097

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > div.section__header:nth-of-type(1) > p.section__subtitle:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
There is no single best transfer for everyone. The useful comparison is the whole door-to-door journey: where you are staying, when you arrive, how much luggage you have, who is traveling with you and what happens after the first train or bus. Current routes and operating times should still be checked for the day of travel.
```

Japanese:

```text
すべての旅行者に共通する「一番いい移動手段」はありません。比べるべきなのはドア・ツー・ドアの全行程です。どこに泊まるか、何時に到着するか、荷物の量、同行者、そして最初の電車やバスを降りた後に何があるかまで確認してください。路線や運行時間は、実際に利用する日の最新情報も確認する必要があります。
```

### ITEM 0098

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
AREX
```

Japanese:

```text
AREX
```

### ITEM 0099

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(1) > div.airport-copy-row__body:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
AREX is easy to understand when the route points toward Seoul Station, Hongdae or another rail-connected destination. The train is only one part of the journey, though. If your hotel still requires another subway transfer and a long walk with luggage, a bus or taxi may be easier door to door even when the rail journey itself is faster.
```

Japanese:

```text
AREXは、ソウル駅、弘大など鉄道でつながりやすい場所へ向かうときは分かりやすい選択肢です。ただし電車は移動全体の一部にすぎません。ホテルまでさらに地下鉄を乗り換え、荷物を持って長く歩く必要があるなら、鉄道そのものが速くても、ドア・ツー・ドアではバスやタクシーのほうが楽なことがあります。
```

### ITEM 0100

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(1) > div.airport-copy-row__body:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Read the AREX guide
```

Japanese:

```text
AREXガイドを見る
```

### ITEM 0101

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(2) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Airport Bus
```

Japanese:

```text
空港バス
```

### ITEM 0102

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(2) > div.airport-copy-row__body:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
An airport bus can be surprisingly comfortable when it stops close to your accommodation. It avoids moving suitcases through a large station, but traffic and the walk from the actual stop still matter. Look at the current route and timetable rather than assuming the nearest-sounding stop is the easiest one.
```

Japanese:

```text
空港バスは、宿泊先の近くに停まる路線なら想像以上に楽です。大きな駅でスーツケースを持って移動する必要がありません。ただし道路渋滞と、実際の停留所から宿泊先までの徒歩距離は確認が必要です。名前が近そうな停留所を選ぶのではなく、現在の路線と時刻表を見て判断してください。
```

### ITEM 0103

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(2) > div.airport-copy-row__body:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Read the airport bus guide
```

Japanese:

```text
空港バスガイドを見る
```

### ITEM 0104

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(3) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Taxi
```

Japanese:

```text
タクシー
```

### ITEM 0105

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(3) > div.airport-copy-row__body:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
A taxi costs more than public transport, but the calculation changes with several travelers, children, heavy bags or a late arrival. Door-to-door travel can remove multiple transfers at the moment they feel most tiring. Use an official taxi stand and keep the Korean destination name, address and phone number ready.
```

Japanese:

```text
タクシーは公共交通より料金が高くなりますが、複数人、子ども連れ、重い荷物、深夜到着では判断が変わります。疲れが出やすい到着直後に、複数回の乗り換えをなくしてドア・ツー・ドアで移動できるのが利点です。公式のタクシー乗り場を利用し、韓国語の目的地名、住所、電話番号をすぐ見せられるようにしておきましょう。
```

### ITEM 0106

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(3) > div.airport-copy-row__body:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Read the taxi guide
```

Japanese:

```text
タクシーガイドを見る
```

### ITEM 0107

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(4) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Pre-booked Transfer
```

Japanese:

```text
事前予約の送迎
```

### ITEM 0108

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(4) > div.airport-copy-row__body:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
The main advantage is not simply that a car is waiting. It is arriving with a meeting plan already arranged, which can be valuable with children, older adults or a large amount of luggage. Save the booking contact, meeting point and terminal instructions offline before the flight.
```

Japanese:

```text
利点は、単に車が待っていることではありません。到着時の待ち合わせ方法まで事前に決まっている点です。子ども連れ、高齢者との旅行、荷物が多い場合には特に助けになります。予約先の連絡先、待ち合わせ場所、ターミナル案内は、出発前にオフラインでも見られる状態で保存しておきましょう。
```

### ITEM 0109

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(4) > div.airport-copy-row__body:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Compare airport transfer options
```

Japanese:

```text
空港送迎の選択肢を比較する
```

### ITEM 0110

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > aside.airport-driving-note:nth-of-type(1) > h3#airport-driving-note-title::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Driving Beyond Seoul?
```

Japanese:

```text
ソウルの外まで運転するなら？
```

### ITEM 0111

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > aside.airport-driving-note:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
A rental car belongs in a different decision from the four common rides into Seoul. It becomes relevant when a regional road trip or an itinerary outside the city makes driving genuinely useful. Licence requirements, insurance, pickup instructions and the correct terminal should all be settled before arrival.
```

Japanese:

```text
レンタカーは、ソウル市内へ向かう一般的な4つの移動手段とは別に考えるべき選択肢です。地方を車で巡る旅行や、市外中心の旅程で運転する意味がある場合に候補になります。必要な免許、保険、受取方法、利用するターミナルは到着前に確認しておきましょう。
```

### ITEM 0112

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > aside.airport-driving-note:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Read the rental car guide
```

Japanese:

```text
レンタカーガイドを見る
```

### ITEM 0113

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Where you stay changes which airport route is actually easiest. If you are still planning the trip, compare Hongdae, Gongdeok, Seoul Station and Myeongdong by the full airport-to-hotel journey.
```

Japanese:

```text
泊まる場所によって、実際に楽な空港アクセスは変わります。まだ旅程を決めている段階なら、弘大、孔徳、ソウル駅、明洞を、空港からホテルまでの全行程で比較してください。
```

### ITEM 0114

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#transport > div.container:nth-of-type(1) > p:nth-of-type(2) > a.airport-text-link:nth-of-type(1) > strong:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Compare Seoul areas for airport access →
```

Japanese:

```text
空港アクセスでソウルの滞在エリアを比較 →
```

### ITEM 0115

- Element/type: H2
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-section:nth-of-type(6) > div.container:nth-of-type(1) > div.section__header:nth-of-type(1) > h2.section__title:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Save These Before Leaving the Airport
```

Japanese:

```text
空港を出る前に保存しておきたい情報
```

### ITEM 0116

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-section:nth-of-type(6) > div.container:nth-of-type(1) > div.section__header:nth-of-type(1) > p.section__subtitle:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Keep the information you may need even if your internet or payment method stops working.
```

Japanese:

```text
インターネットや支払い手段が使えなくなっても必要になる情報は、あらかじめ保存しておきましょう。
```

### ITEM 0117

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-section:nth-of-type(6) > div.container:nth-of-type(1) > ul.airport-save-list:nth-of-type(1) > li:nth-of-type(1) > span:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Keep the Korean name, road address and phone number together so a driver, information desk or Korean map app can identify the place.
```

Japanese:

```text
運転手、案内カウンター、韓国の地図アプリのどれでも宿泊先を確認できるよう、韓国語の名称、道路名住所、電話番号をまとめて保存しておきます。
```

### ITEM 0118

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-section:nth-of-type(6) > div.container:nth-of-type(1) > ul.airport-save-list:nth-of-type(1) > li:nth-of-type(1) > strong:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Accommodation details in Korean
```

Japanese:

```text
宿泊先の韓国語情報
```

### ITEM 0119

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-section:nth-of-type(6) > div.container:nth-of-type(1) > ul.airport-save-list:nth-of-type(1) > li:nth-of-type(2) > span:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Save the confirmation and any arrival or check-in instructions where they remain available without mobile data.
```

Japanese:

```text
予約確認書と、到着方法やチェックインに関する案内は、モバイルデータがなくても見られる場所に保存してください。
```

### ITEM 0120

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-section:nth-of-type(6) > div.container:nth-of-type(1) > ul.airport-save-list:nth-of-type(1) > li:nth-of-type(2) > strong:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Booking confirmation
```

Japanese:

```text
予約確認書
```

### ITEM 0121

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-section:nth-of-type(6) > div.container:nth-of-type(1) > ul.airport-save-list:nth-of-type(1) > li:nth-of-type(3) > span:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Record the terminal, route, stop or station, useful exit and the final walk or transfer to the accommodation—not only the first train or bus.
```

Japanese:

```text
最初の電車やバスだけでなく、ターミナル、路線、停留所または駅、使いやすい出口、宿泊先までの最後の徒歩や乗り換えまで記録しておきます。
```

### ITEM 0122

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-section:nth-of-type(6) > div.container:nth-of-type(1) > ul.airport-save-list:nth-of-type(1) > li:nth-of-type(3) > strong:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
The complete transport route
```

Japanese:

```text
宿泊先までの全移動ルート
```

### ITEM 0123

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-section:nth-of-type(6) > div.container:nth-of-type(1) > ul.airport-save-list:nth-of-type(1) > li:nth-of-type(4) > span:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Keep the eSIM activation information, support contact and QR code if one may be needed again. Follow the provider's instructions before changing or deleting a profile.
```

Japanese:

```text
eSIMの開通情報、サポート連絡先、再度必要になる可能性があるQRコードを保存しておきましょう。プロファイルを変更・削除する前に、事業者の案内を確認してください。
```

### ITEM 0124

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-section:nth-of-type(6) > div.container:nth-of-type(1) > ul.airport-save-list:nth-of-type(1) > li:nth-of-type(4) > strong:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Mobile data support details
```

Japanese:

```text
モバイルデータのサポート情報
```

### ITEM 0125

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-section:nth-of-type(6) > div.container:nth-of-type(1) > ul.airport-save-list:nth-of-type(1) > li:nth-of-type(5) > span:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Save the destination in Naver Map or KakaoMap and keep a screenshot or map pin that can still be shown when the connection is unreliable.
```

Japanese:

```text
目的地をNaver MapまたはKakaoMapに保存し、通信が不安定でも見せられるスクリーンショットや地図ピンも残しておきましょう。
```

### ITEM 0126

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-section:nth-of-type(6) > div.container:nth-of-type(1) > ul.airport-save-list:nth-of-type(1) > li:nth-of-type(5) > strong:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
An offline map reference
```

Japanese:

```text
オフラインで見せられる地図情報
```

### ITEM 0127

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-section:nth-of-type(6) > div.container:nth-of-type(1) > ul.airport-save-list:nth-of-type(1) > li:nth-of-type(6) > span:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Keep another card or some cash separate from the main card so one declined transaction does not stop the onward journey.
```

Japanese:

```text
1回の決済エラーでその後の移動まで止まらないよう、別のカードか現金をメインカードとは別に持っておきましょう。
```

### ITEM 0128

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-section:nth-of-type(6) > div.container:nth-of-type(1) > ul.airport-save-list:nth-of-type(1) > li:nth-of-type(6) > strong:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
A second way to pay
```

Japanese:

```text
予備の支払い手段
```

### ITEM 0129

- Element/type: H2
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.section__header:nth-of-type(1) > h2.section__title:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Solve Problems in the Public Arrival Hall
```

Japanese:

```text
一般到着ロビーでトラブルを解決する
```

### ITEM 0130

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.section__header:nth-of-type(1) > p.section__subtitle:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
A problem with data, payment or transport is easier to untangle before you leave the terminal. The arrival hall gives you Wi-Fi, official signs and information desks while you work out a backup.
```

Japanese:

```text
通信、支払い、交通のトラブルは、ターミナルを出る前のほうが解決しやすいものです。到着ロビーならWi-Fi、公式案内表示、案内カウンターを利用しながら代替手段を考えられます。
```

### ITEM 0131

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Your eSIM Still Has No Data
```

Japanese:

```text
eSIMでまだ通信できない
```

### ITEM 0132

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(1) > div.airport-copy-row__body:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Stay on airport Wi-Fi for a few more minutes. Make sure the travel line is switched on and selected for mobile data, then follow the provider's activation and roaming instructions. If it still does not connect, contact the provider before deleting the eSIM profile; your offline screenshots can cover the immediate journey while support is being arranged.
```

Japanese:

```text
空港Wi-Fiにもう少しつないだまま確認しましょう。旅行用の回線がオンになっていて、モバイルデータ回線として選択されているか確認し、その後に事業者の開通・ローミング手順に従います。それでもつながらない場合は、eSIMプロファイルを削除する前に事業者へ連絡してください。サポートを受けている間も、オフライン保存したスクリーンショットがあれば当面の移動には対応できます。
```

### ITEM 0133

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(1) > div.airport-copy-row__body:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Review the eSIM guide
```

Japanese:

```text
eSIMガイドを確認する
```

### ITEM 0134

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(2) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Your Foreign Card Was Declined
```

Japanese:

```text
海外発行カードが使えない
```

### ITEM 0135

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(2) > div.airport-copy-row__body:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
One decline does not have to stop the trip. Try a staffed counter or a different card, and look at the card issuer's settings while you still have Wi-Fi. A second card or some cash keeps the transport decision separate from the problem with the first payment method.
```

Japanese:

```text
1回決済できなかっただけで、旅行全体を止める必要はありません。有人窓口や別のカードを試し、Wi-Fiが使えるうちにカード発行会社の設定も確認してください。2枚目のカードや現金があれば、最初の支払い手段の問題と、その後の交通手段を切り分けて考えられます。
```

### ITEM 0136

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(2) > div.airport-copy-row__body:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Review the payments guide
```

Japanese:

```text
支払いガイドを確認する
```

### ITEM 0137

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(3) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
You Cannot Find AREX, the Bus or a Taxi
```

Japanese:

```text
AREX・バス・タクシー乗り場が見つからない
```

### ITEM 0138

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(3) > div.airport-copy-row__body:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Start with the terminal and floor shown on the official airport map, then follow the current signs for the route you chose. An information desk can point out the correct entrance, stop or official taxi stand. That is safer and clearer than following an unsolicited ride offer.
```

Japanese:

```text
まず公式空港マップでターミナルと階を確認し、選んだ交通手段の現在の案内表示に従ってください。案内カウンターでは、正しい入口、停留所、公式タクシー乗り場を教えてもらえます。声をかけてくる非公式な送迎案内について行くより、安全で分かりやすい方法です。
```

### ITEM 0139

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(3) > div.airport-copy-row__body:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Compare airport transport
```

Japanese:

```text
空港からの移動手段を比較する
```

### ITEM 0140

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(4) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
You Arrived Late at Night
```

Japanese:

```text
深夜に到着した
```

### ITEM 0141

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(4) > div.airport-copy-row__body:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
A late arrival changes the order of the decision: check the current official timetable for your terminal before walking toward a daytime route from memory. A late-night bus may still be available. If it is not, use an official taxi stand or the saved meeting instructions for a pre-booked pickup.
```

Japanese:

```text
深夜到着では確認する順番が変わります。昼間に予定していたルートへ記憶だけで向かう前に、自分のターミナルの最新公式時刻表を確認してください。深夜バスがまだ運行している場合があります。利用できなければ、公式タクシー乗り場か、事前予約送迎で保存しておいた待ち合わせ案内を使いましょう。
```

### ITEM 0142

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(4) > div.airport-copy-row__body:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Check the airport bus guide
```

Japanese:

```text
空港バスガイドを確認する
```

### ITEM 0143

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(5) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
The Driver Cannot Identify Your Accommodation
```

Japanese:

```text
運転手に宿泊先が伝わらない
```

### ITEM 0144

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(5) > div.airport-copy-row__body:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Show the Korean accommodation name, road address and phone number rather than relying on the English brand name alone. A saved location in a Korean map app or the booking screenshot can give the driver another reference, and airport Wi-Fi leaves you a way to contact the accommodation if the destination is still unclear.
```

Japanese:

```text
英語のブランド名だけに頼らず、韓国語の宿泊先名、道路名住所、電話番号を見せてください。韓国の地図アプリに保存した場所や予約画面のスクリーンショットも別の手掛かりになります。それでも目的地が伝わらない場合は、空港Wi-Fiを使って宿泊先へ連絡できます。
```

### ITEM 0145

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#problems > div.container:nth-of-type(1) > div.airport-copy-list:nth-of-type(2) > article.airport-copy-row:nth-of-type(5) > div.airport-copy-row__body:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Review the maps guide
```

Japanese:

```text
地図ガイドを確認する
```

### ITEM 0146

- Element/type: H2
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.section__header:nth-of-type(1) > h2.section__title:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Incheon Airport First 30 Minutes FAQ
```

Japanese:

```text
仁川空港 到着後30分 FAQ
```

### ITEM 0147

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.section__header:nth-of-type(1) > p.section__subtitle:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Short answers for decisions you need to make in the public arrival hall.
```

Japanese:

```text
一般到着ロビーで判断が必要になることを、短く確認できます。
```

### ITEM 0148

- Element/type: visible FAQ question
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(2) > details:nth-of-type(1) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
What should I do first after entering the arrival hall?
```

Japanese:

```text
到着ロビーに出たら、まず何をすればいい？
```

### ITEM 0149

- Element/type: visible FAQ answer/direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(2) > details:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Use the airport Wi-Fi as a safety net while you test mobile data, save the Korean accommodation details, make sure you have a second way to pay and look at the complete route to your stay. These small jobs become harder once you are outside with luggage.
```

Japanese:

```text
空港Wi-Fiを予備として使いながらモバイルデータを確認し、宿泊先の韓国語情報を保存し、2つ目の支払い手段を確保して、宿泊先までの全移動ルートを確認してください。荷物を持って空港の外へ出てからだと、こうした小さな準備も面倒になります。
```

### ITEM 0150

- Element/type: visible FAQ question
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(2) > details:nth-of-type(2) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Do I need to buy T-money immediately?
```

Japanese:

```text
T-moneyはすぐ買う必要がある？
```

### ITEM 0151

- Element/type: visible FAQ answer/direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(2) > details:nth-of-type(2) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
No. T-money is useful when a bus or subway is part of the first journey, but it does not need to be the first purchase for someone leaving by taxi, pre-booked transfer or rental car. Getting connected and understanding the route may matter more in those first few minutes.
```

Japanese:

```text
いいえ。最初の移動にバスや地下鉄を使うならT-moneyは便利ですが、タクシー、事前予約送迎、レンタカーで空港を出る人にとって最初の買い物である必要はありません。到着直後の数分は、通信を確保して移動ルートを把握するほうが重要な場合があります。
```

### ITEM 0152

- Element/type: visible FAQ question
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(2) > details:nth-of-type(3) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
How do I choose between AREX, airport bus and taxi?
```

Japanese:

```text
AREX・空港バス・タクシーはどう選ぶ？
```

### ITEM 0153

- Element/type: visible FAQ answer/direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(2) > details:nth-of-type(3) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Look beyond the first leg. AREX is straightforward for rail-connected destinations, an airport bus can be easier when it stops near the accommodation, and a taxi removes transfers when luggage, children, group size or a late arrival make them especially tiring.
```

Japanese:

```text
最初の区間だけで判断しないでください。鉄道で行きやすい目的地ならAREXが分かりやすく、宿泊先の近くに停まるなら空港バスが楽なことがあります。荷物、子ども、人数、深夜到着によって乗り換えが特に負担になるなら、タクシーはその乗り換えをなくせます。
```

### ITEM 0154

- Element/type: visible FAQ question
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(2) > details:nth-of-type(4) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
What should I do after a late-night arrival?
```

Japanese:

```text
深夜に到着したらどうすればいい？
```

### ITEM 0155

- Element/type: visible FAQ answer/direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(2) > details:nth-of-type(4) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Check the current official timetable for your terminal before following a route planned for daytime. A late-night bus may still operate; otherwise, use an official taxi stand or the saved meeting instructions for a pre-booked pickup.
```

Japanese:

```text
昼間向けに考えていたルートへ向かう前に、自分のターミナルの最新公式時刻表を確認してください。深夜バスが運行している場合があります。利用できなければ、公式タクシー乗り場か、事前予約送迎で保存しておいた待ち合わせ案内を使いましょう。
```

### ITEM 0156

- Element/type: visible FAQ question
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(2) > details:nth-of-type(5) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Where can I check my terminal and airport facilities?
```

Japanese:

```text
利用ターミナルや空港施設はどこで確認できる？
```

### ITEM 0157

- Element/type: visible FAQ answer/direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(2) > details:nth-of-type(5) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Check the official arrival flight page for your terminal and use the official airport map for facilities. If you must change terminals in the public area, use the free terminal shuttle bus or the paid Airport Railroad and confirm current operating details. The airside shuttle train is for transfer routes, not ordinary public-area terminal travel.
```

Japanese:

```text
利用ターミナルは公式の到着便ページで確認し、施設は公式空港マップで確認してください。一般区域でターミナルを移動する必要がある場合は、無料のターミナル間シャトルバスまたは有料の空港鉄道を利用し、最新の運行情報を確認します。制限区域内のシャトルトレインは乗り継ぎ用で、一般区域での通常のターミナル移動には使いません。
```

### ITEM 0158

- Element/type: visible FAQ question
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(2) > details:nth-of-type(6) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
What information should I save before leaving the airport?
```

Japanese:

```text
空港を出る前に、どんな情報を保存しておけばいい？
```

### ITEM 0159

- Element/type: visible FAQ answer/direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(2) > details:nth-of-type(6) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Save the Korean accommodation name, road address and phone number, the booking confirmation, the complete transport route, useful map screenshots, mobile-data activation and support details, any important QR code and a backup payment plan.
```

Japanese:

```text
宿泊先の韓国語名、道路名住所、電話番号、予約確認書、宿泊先までの全移動ルート、役立つ地図のスクリーンショット、モバイルデータの開通・サポート情報、必要なQRコード、予備の支払い手段を保存しておきましょう。
```

### ITEM 0160

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.related-links:nth-of-type(3) > p.related-links__title:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Continue with the part you need
```

Japanese:

```text
必要なところから確認する
```

### ITEM 0161

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.related-links:nth-of-type(3) > a.chip:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Set up mobile data
```

Japanese:

```text
モバイルデータを設定する
```

### ITEM 0162

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.related-links:nth-of-type(3) > a.chip:nth-of-type(2)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Understand T-money
```

Japanese:

```text
T-moneyの使い方を確認する
```

### ITEM 0163

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.related-links:nth-of-type(3) > a.chip:nth-of-type(3)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Plan payment backups
```

Japanese:

```text
支払いの予備手段を準備する
```

### ITEM 0164

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.related-links:nth-of-type(3) > a.chip:nth-of-type(4)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Set up Korean maps
```

Japanese:

```text
韓国で使う地図アプリを設定する
```

### ITEM 0165

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.related-links:nth-of-type(3) > a.chip:nth-of-type(5)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Compare the full airport route
```

Japanese:

```text
空港から宿泊先までの全ルートを比較する
```

### ITEM 0166

- Element/type: H2
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#sources > div.container:nth-of-type(1) > div.section__header:nth-of-type(1) > h2.section__title:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Official Airport Information
```

Japanese:

```text
空港の公式情報
```

### ITEM 0167

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#sources > div.container:nth-of-type(1) > div.section__header:nth-of-type(1) > p.section__subtitle:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Terminal assignments, transport schedules and facility locations can change. These are the official Incheon Airport pages to use when the detail needs to be current.
```

Japanese:

```text
利用ターミナル、交通機関の時刻表、施設の場所は変更されることがあります。最新情報が必要なときは、以下の仁川空港公式ページで確認してください。
```

### ITEM 0168

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#sources > div.container:nth-of-type(1) > ul.airport-copy-list:nth-of-type(1) > li.airport-copy-row:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Airport Map
```

Japanese:

```text
空港マップ
```

### ITEM 0169

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#sources > div.container:nth-of-type(1) > ul.airport-copy-list:nth-of-type(1) > li.airport-copy-row:nth-of-type(1) > div.airport-copy-row__body:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Select Terminal 1 or Terminal 2, then use the arrivals floor to find facilities and transport entrances.
```

Japanese:

```text
第1ターミナルまたは第2ターミナルを選び、到着階の表示に切り替えて、施設や交通機関の入口を確認します。
```

### ITEM 0170

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#sources > div.container:nth-of-type(1) > ul.airport-copy-list:nth-of-type(1) > li.airport-copy-row:nth-of-type(1) > div.airport-copy-row__body:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Open the official airport map
```

Japanese:

```text
空港公式マップを開く
```

### ITEM 0171

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#sources > div.container:nth-of-type(1) > ul.airport-copy-list:nth-of-type(1) > li.airport-copy-row:nth-of-type(2) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Arrival Flights
```

Japanese:

```text
到着便
```

### ITEM 0172

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#sources > div.container:nth-of-type(1) > ul.airport-copy-list:nth-of-type(1) > li.airport-copy-row:nth-of-type(2) > div.airport-copy-row__body:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Use the live flight information to confirm the actual arrival terminal instead of relying on a fixed airline list.
```

Japanese:

```text
固定された航空会社一覧ではなく、リアルタイムの運航情報で実際の到着ターミナルを確認してください。
```

### ITEM 0173

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#sources > div.container:nth-of-type(1) > ul.airport-copy-list:nth-of-type(1) > li.airport-copy-row:nth-of-type(2) > div.airport-copy-row__body:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Check official arrival flights
```

Japanese:

```text
公式の到着便情報を確認する
```

### ITEM 0174

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#sources > div.container:nth-of-type(1) > ul.airport-copy-list:nth-of-type(1) > li.airport-copy-row:nth-of-type(3) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Inter-terminal Transportation
```

Japanese:

```text
ターミナル間の移動
```

### ITEM 0175

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#sources > div.container:nth-of-type(1) > ul.airport-copy-list:nth-of-type(1) > li.airport-copy-row:nth-of-type(3) > div.airport-copy-row__body:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Review the public-area terminal shuttle bus and Airport Railroad options, together with current operating and fare information.
```

Japanese:

```text
一般区域で利用できるターミナル間シャトルバスと空港鉄道について、現在の運行情報と運賃をあわせて確認してください。
```

### ITEM 0176

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#sources > div.container:nth-of-type(1) > ul.airport-copy-list:nth-of-type(1) > li.airport-copy-row:nth-of-type(3) > div.airport-copy-row__body:nth-of-type(1) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Check official terminal transport
```

Japanese:

```text
公式のターミナル間交通を確認する
```

### ITEM 0177

- Element/type: H3
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#sources > div.container:nth-of-type(1) > ul.airport-copy-list:nth-of-type(1) > li.airport-copy-row:nth-of-type(4) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Late-night Buses
```

Japanese:

```text
深夜バス
```

### ITEM 0178

- Element/type: body direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#sources > div.container:nth-of-type(1) > ul.airport-copy-list:nth-of-type(1) > li.airport-copy-row:nth-of-type(4) > div.airport-copy-row__body:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Use the page for the terminal you arrived at to check current routes, stops and departure times.
```

Japanese:

```text
到着したターミナルのページで、現在の路線、停留所、出発時刻を確認してください。
```

### ITEM 0179

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#sources > div.container:nth-of-type(1) > ul.airport-copy-list:nth-of-type(1) > li.airport-copy-row:nth-of-type(4) > div.airport-copy-row__body:nth-of-type(1) > p.airport-source-links:nth-of-type(2) > a.airport-text-link:nth-of-type(1)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Terminal 1
```

Japanese:

```text
第1ターミナル
```

### ITEM 0180

- Element/type: visible link direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > main:nth-of-type(1) > section#sources > div.container:nth-of-type(1) > ul.airport-copy-list:nth-of-type(1) > li.airport-copy-row:nth-of-type(4) > div.airport-copy-row__body:nth-of-type(1) > p.airport-source-links:nth-of-type(2) > a.airport-text-link:nth-of-type(2)::text[1]`
- Reuse: NEW airport-specific localization

English:

```text
Terminal 2
```

Japanese:

```text
第2ターミナル
```

### ITEM 0181

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > div.footer__brand-block:nth-of-type(1) > p.footer__brand:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0535

English:

```text
Korea Inside
```

Japanese:

```text
Korea Inside
```

### ITEM 0182

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > div.footer__brand-block:nth-of-type(1) > p.footer__eyebrow:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0536

English:

```text
CREATED IN KOREA
```

Japanese:

```text
韓国発
```

### ITEM 0183

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > div.footer__brand-block:nth-of-type(1) > p.footer__statement:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0537

English:

```text
Practical Korea travel guidance, written and reviewed locally by a Korean editor.
```

Japanese:

```text
韓国人エディターが現地で執筆・確認する、実用重視の韓国旅行ガイドです。
```

### ITEM 0184

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > div.footer__brand-block:nth-of-type(1) > p.footer__description:nth-of-type(4)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0538

English:

```text
Based on official sources, local context, and independent editorial judgment.
```

Japanese:

```text
公式情報、現地事情、独立した編集判断をもとに作成しています。
```

### ITEM 0185

- Element/type: literal ARIA label
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1)@aria-label`
- Reuse: Japanese Golden Sample Approved ITEM 0539

English:

```text
Footer navigation
```

Japanese:

```text
フッターナビゲーション
```

### ITEM 0186

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(1) > p.footer__heading:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0540

English:

```text
PLAN YOUR TRIP
```

Japanese:

```text
旅行を計画する
```

### ITEM 0187

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(1) > ul.footer__links:nth-of-type(1) > li:nth-of-type(1) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0541

English:

```text
Airport
```

Japanese:

```text
空港
```

### ITEM 0188

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(1) > ul.footer__links:nth-of-type(1) > li:nth-of-type(2) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0542

English:

```text
eSIM
```

Japanese:

```text
eSIM
```

### ITEM 0189

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(1) > ul.footer__links:nth-of-type(1) > li:nth-of-type(3) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0543

English:

```text
Checklist
```

Japanese:

```text
チェックリスト
```

### ITEM 0190

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(2) > p.footer__heading:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0544

English:

```text
USE KOREA
```

Japanese:

```text
韓国で使う
```

### ITEM 0191

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(2) > ul.footer__links:nth-of-type(1) > li:nth-of-type(1) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0545

English:

```text
T-money
```

Japanese:

```text
T-money
```

### ITEM 0192

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(2) > ul.footer__links:nth-of-type(1) > li:nth-of-type(2) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0546

English:

```text
Payments
```

Japanese:

```text
支払い
```

### ITEM 0193

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(2) > ul.footer__links:nth-of-type(1) > li:nth-of-type(3) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0547

English:

```text
Maps
```

Japanese:

```text
地図
```

### ITEM 0194

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(2) > ul.footer__links:nth-of-type(1) > li:nth-of-type(4) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0548

English:

```text
Apps
```

Japanese:

```text
アプリ
```

### ITEM 0195

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0549

English:

```text
© 2026 Korea Inside · Republic of Korea
```

Japanese:

```text
© 2026 Korea Inside · 大韓民国
```

### ITEM 0196

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p.footer__business:nth-of-type(2) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0552

English:

```text
getkoreainside@gmail.com
```

Japanese:

```text
getkoreainside@gmail.com
```

### ITEM 0197

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p.footer__business:nth-of-type(2) > a.footer__legal-link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0550

English:

```text
Affiliate Disclosure
```

Japanese:

```text
アフィリエイト開示
```

### ITEM 0198

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p.footer__business:nth-of-type(2) > a.footer__legal-link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0551

English:

```text
Privacy Policy
```

Japanese:

```text
プライバシーポリシー
```

### ITEM 0199

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p.footer__business:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0553

English:

```text
Business Registration No. 462-39-01721
```

Japanese:

```text
事業者登録番号 462-39-01721
```

### ITEM 0200

- Element/type: common UI footer direct text
- Source target: `airport.html|html:nth-of-type(1) > body.airport-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p.footer__business:nth-of-type(2)::text[2]`
- Reuse: Japanese Golden Sample Approved ITEM 0554

English:

```text
Contact:
```

Japanese:

```text
お問い合わせ：
```

### ITEM 0201

- Element/type: shared common UI runtime string
- Source target: `common.js|common.js::install dialog close ARIA`
- Reuse: Japanese Golden Sample Approved ITEM 0684

English:

```text
Close
```

Japanese:

```text
閉じる
```

### ITEM 0202

- Element/type: shared common UI runtime string
- Source target: `common.js|common.js::navigation closed-state ARIA`
- Reuse: Japanese Golden Sample Approved ITEM 0685

English:

```text
Open menu
```

Japanese:

```text
メニューを開く
```

### ITEM 0203

- Element/type: shared common UI runtime string
- Source target: `common.js|common.js::navigation open-state ARIA`
- Reuse: Japanese Golden Sample Approved ITEM 0686

English:

```text
Close menu
```

Japanese:

```text
メニューを閉じる
```

### ITEM 0204

- Element/type: shared common UI runtime string
- Source target: `common.js|common.js::language option en`
- Reuse: Japanese Golden Sample Approved ITEM 0687

English:

```text
English
```

Japanese:

```text
English
```

### ITEM 0205

- Element/type: shared common UI runtime string
- Source target: `common.js|common.js::language option es`
- Reuse: Japanese Golden Sample Approved ITEM 0688

English:

```text
Español
```

Japanese:

```text
Español
```

### ITEM 0206

- Element/type: shared common UI runtime string
- Source target: `common.js|common.js::install button`
- Reuse: Japanese Golden Sample Approved ITEM 0689

English:

```text
Install Korea Inside
```

Japanese:

```text
Korea Insideをインストール
```

### ITEM 0207

- Element/type: shared common UI runtime string
- Source target: `common.js|common.js::iOS install title`
- Reuse: Japanese Golden Sample Approved ITEM 0690

English:

```text
Add Korea Inside to your Home Screen
```

Japanese:

```text
Korea Insideをホーム画面に追加
```

### ITEM 0208

- Element/type: shared common UI runtime string
- Source target: `common.js|common.js::iOS install step 1`
- Reuse: Japanese Golden Sample Approved ITEM 0691

English:

```text
Tap Share
```

Japanese:

```text
「共有」をタップ
```

### ITEM 0209

- Element/type: shared common UI runtime string
- Source target: `common.js|common.js::iOS install step 2`
- Reuse: Japanese Golden Sample Approved ITEM 0692

English:

```text
Tap Add to Home Screen
```

Japanese:

```text
「ホーム画面に追加」をタップ
```

### ITEM 0210

- Element/type: shared common UI runtime string
- Source target: `common.js|common.js::browser install fallback`
- Reuse: Japanese Golden Sample Approved ITEM 0693

English:

```text
Open your browser menu and choose “Install app” or “Add to Home screen.”
```

Japanese:

```text
ブラウザのメニューを開き、「アプリをインストール」または「ホーム画面に追加」を選んでください。
```

## QA Ledger

- Source ITEM count: **210**
- Localized ITEM count: **210**
- ITEM range: **0001–0210**
- Common UI reuse: **93 / 93**
- Airport-specific localization: **117 / 117**
- Missing localization ITEM: **0**
- Empty Japanese ITEM: **0**
- Golden Sample approved reference missing: **0**
- HTML modifications in this localization step: **0**
- stage / commit / push / deploy in this localization step: **0**

## Approval status

**APPROVED PUBLIC COPY — CONTENT LOCKED**

**User approved on 2026-09-24.**

Implementation rules:
- no retranslation
- no rewriting
- no grammar improvement
- no summarization
- no expansion
- no CTA/recommendation change
- Codex exact implementation only
