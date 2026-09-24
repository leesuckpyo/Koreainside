# Korea Inside — Japanese Localization Batch 2 — Arrival

**Date:** 2026-09-24  
**Status:** APPROVED PUBLIC COPY — CONTENT LOCKED  
**Source MD:** `Korea_Inside_JA_Airport5_Localization_Source_Batch2_2026-09-24.md`  
**Target:** `ja/arrival.html`  

## Scope

- `arrival.html` Source ITEMs: **204 / 204**
- Japanese Golden Sample common UI reused exactly: **83 / 83**
- Arrival page-specific Japanese editorial localization: **121 / 121**
- English source SHA-256: `CEF8C21F0C4D2F4F825FF3B8FFCD5E3CAAB7740D3318791D6D3AA2333F68B017`
- `ja/arrival.html` matched the English source before localization.

## Japanese editorial-localization rules

- This is Japanese editorial localization, not literal translation.
- Facts, dates, terminal distinctions, entry-process order, operating-condition wording, recommendation logic, URLs, HTML structure, class/id/data attributes, images/srcset, JSON-LD structure, CSS and JS logic remain unchanged in meaning.
- Natural Japanese travel-language forms are used for Korean places and airport terms, while official product/service names such as `K-ETA`, `e-Arrival Card`, `AREX`, `T-money`, `Korea Inside` remain recognizable.
- Where airport signage matters operationally, English sign terms such as `Arrivals`, `Immigration`, `Transfer`, and `Connecting Flights` are retained within the Japanese guidance so travelers can match the signs they see on site.
- Common UI is inherited exactly from the CONTENT LOCKED Japanese Golden Sample and is not retranslated.

## ITEMs

### ITEM 0211

- Element/type: meta description
- Source target: `arrival.html|html:nth-of-type(1) > head:nth-of-type(1) > meta:nth-of-type(3)@content`
- Reuse: NEW arrival-specific localization

English:

```text
Incheon Airport arrival guide covering terminal checks, immigration, K-ETA and e-Arrival Card, baggage claim, customs and the public arrival hall.
```

Japanese:

```text
仁川空港の到着ガイド。利用ターミナルの確認、入国審査、K-ETAとe-Arrival Card、手荷物受取、税関、一般到着ロビーまでの流れをまとめています。
```

### ITEM 0212

- Element/type: title
- Source target: `arrival.html|html:nth-of-type(1) > head:nth-of-type(1) > title:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Incheon Airport Arrival Guide: Immigration, Baggage & Customs | Korea Inside
```

Japanese:

```text
仁川空港 到着ガイド：入国審査・手荷物受取・税関 | Korea Inside
```

### ITEM 0213

- Element/type: JSON-LD FAQ question
- Source target: `arrival.html|json-ld::mainEntity[1].name`
- Reuse: NEW arrival-specific localization

English:

```text
How do I know whether my flight arrives at Terminal 1 or Terminal 2?
```

Japanese:

```text
自分の便が第1ターミナルと第2ターミナルのどちらに到着するか、どう確認すればいい？
```

### ITEM 0214

- Element/type: JSON-LD FAQ answer
- Source target: `arrival.html|json-ld::mainEntity[1].acceptedAnswer.text`
- Reuse: NEW arrival-specific localization

English:

```text
Check your e-ticket or search your flight number on the official Incheon Airport arrival page. That is safer than relying on a fixed airline list, particularly for codeshare flights.
```

Japanese:

```text
eチケットを確認するか、仁川空港公式の到着便ページで便名を検索してください。特にコードシェア便では、固定された航空会社一覧に頼るより確実です。
```

### ITEM 0215

- Element/type: JSON-LD FAQ question
- Source target: `arrival.html|json-ld::mainEntity[2].name`
- Reuse: NEW arrival-specific localization

English:

```text
Do I go through immigration before collecting my baggage?
```

Japanese:

```text
手荷物を受け取る前に入国審査を通る？
```

### ITEM 0216

- Element/type: JSON-LD FAQ answer
- Source target: `arrival.html|json-ld::mainEntity[2].acceptedAnswer.text`
- Reuse: NEW arrival-specific localization

English:

```text
Yes, if you are entering Korea. The usual order is immigration, baggage claim, customs and then the public arrival hall. Passengers connecting to another flight should follow Transfer or Connecting Flights signs and their airline’s instructions instead.
```

Japanese:

```text
韓国に入国する場合は、はい。通常は「入国審査 → 手荷物受取 → 税関 → 一般到着ロビー」の順です。別の便へ乗り継ぐ場合は、この流れではなく「Transfer」または「Connecting Flights」の案内表示と航空会社の指示に従ってください。
```

### ITEM 0217

- Element/type: JSON-LD FAQ question
- Source target: `arrival.html|json-ld::mainEntity[3].name`
- Reuse: NEW arrival-specific localization

English:

```text
What should I do if my baggage does not arrive?
```

Japanese:

```text
預けた荷物が出てこないときはどうすればいい？
```

### ITEM 0218

- Element/type: JSON-LD FAQ answer
- Source target: `arrival.html|json-ld::mainEntity[3].acceptedAnswer.text`
- Reuse: NEW arrival-specific localization

English:

```text
First check that the baggage belt still matches your flight. If the bag is missing or damaged, contact your airline or the baggage service desk before leaving the baggage area, and keep your baggage tag and flight details ready.
```

Japanese:

```text
まず、手荷物受取レーンがまだ自分の便に割り当てられているか確認してください。荷物が見つからない、または破損している場合は、手荷物受取エリアを出る前に航空会社または手荷物サービスカウンターへ連絡し、手荷物タグと便情報を用意しておきましょう。
```

### ITEM 0219

- Element/type: JSON-LD FAQ question
- Source target: `arrival.html|json-ld::mainEntity[4].name`
- Reuse: NEW arrival-specific localization

English:

```text
Can I walk between Terminal 1 and Terminal 2?
```

Japanese:

```text
第1ターミナルと第2ターミナルの間は歩いて移動できる？
```

### ITEM 0220

- Element/type: JSON-LD FAQ answer
- Source target: `arrival.html|json-ld::mainEntity[4].acceptedAnswer.text`
- Reuse: NEW arrival-specific localization

English:

```text
No. Terminal 1 and Terminal 2 are separate buildings without a public walking route between them. If you need to change terminals in the public area, use the airport’s official inter-terminal transportation guidance.
```

Japanese:

```text
できません。第1ターミナルと第2ターミナルは別の建物で、一般区域を徒歩で移動するルートはありません。一般区域でターミナルを移動する必要がある場合は、仁川空港公式のターミナル間交通案内を確認してください。
```

### ITEM 0221

- Element/type: JSON-LD FAQ question
- Source target: `arrival.html|json-ld::mainEntity[5].name`
- Reuse: NEW arrival-specific localization

English:

```text
Which transport should I choose from Incheon Airport to Seoul?
```

Japanese:

```text
仁川空港からソウルへは、どの移動手段を選べばいい？
```

### ITEM 0222

- Element/type: JSON-LD FAQ answer
- Source target: `arrival.html|json-ld::mainEntity[5].acceptedAnswer.text`
- Reuse: NEW arrival-specific localization

English:

```text
Make that decision after you reach the public arrival hall. The best option depends on the exact accommodation, arrival time, luggage and group size. The separate Airport Transfer Guide compares AREX, airport bus, taxi and pre-booked transfer by the full journey to your stay.
```

Japanese:

```text
一般到着ロビーに出てから決めれば大丈夫です。最適な方法は、実際の宿泊先、到着時刻、荷物の量、人数によって変わります。別ページの空港送迎ガイドでは、AREX、空港バス、タクシー、事前予約送迎を、宿泊先までの全行程で比較しています。
```

### ITEM 0223

- Element/type: JSON-LD FAQ question
- Source target: `arrival.html|json-ld::mainEntity[6].name`
- Reuse: NEW arrival-specific localization

English:

```text
What should transfer passengers do?
```

Japanese:

```text
乗り継ぎ客はどうすればいい？
```

### ITEM 0224

- Element/type: JSON-LD FAQ answer
- Source target: `arrival.html|json-ld::mainEntity[6].acceptedAnswer.text`
- Reuse: NEW arrival-specific localization

English:

```text
Follow Transfer or Connecting Flights signs and your airline’s instructions. Confirm the connection terminal, available connection time and whether checked baggage is transferred through to the next flight instead of assuming that you should follow the general immigration route.
```

Japanese:

```text
「Transfer」または「Connecting Flights」の案内表示と航空会社の指示に従ってください。一般の入国ルートへ進むと決めつけず、乗り継ぎターミナル、乗り継ぎ可能時間、預け荷物が次の便まで通しで運ばれるかを確認してください.
```

### ITEM 0225

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > a.logo:nth-of-type(1)@aria-label`
- Reuse: Japanese Golden Sample Approved ITEM 0003

English:

```text
Korea Inside home
```

Japanese:

```text
Korea Inside ホーム
```

### ITEM 0226

- Element/type: image alt
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > a.logo:nth-of-type(1) > img.site-brand__logo:nth-of-type(1)@alt`
- Reuse: Japanese Golden Sample Approved ITEM 0004

English:

```text
Korea Inside
```

Japanese:

```text
Korea Inside
```

### ITEM 0227

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > button#site-nav-toggle@aria-label`
- Reuse: Japanese Golden Sample Approved ITEM 0005

English:

```text
Open menu
```

Japanese:

```text
メニューを開く
```

### ITEM 0228

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation@aria-label`
- Reuse: Japanese Golden Sample Approved ITEM 0006

English:

```text
Primary navigation
```

Japanese:

```text
メインナビゲーション
```

### ITEM 0229

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(1) > button#site-nav-trigger-discover::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0007

English:

```text
DISCOVER
```

Japanese:

```text
楽しむ
```

### ITEM 0230

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(1) > div#site-nav-panel-discover > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0008

English:

```text
Taste Korea
```

Japanese:

```text
Taste Korea
```

### ITEM 0231

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(1) > div#site-nav-panel-discover > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0009

English:

```text
K-Beauty
```

Japanese:

```text
K-Beauty
```

### ITEM 0232

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > button#site-nav-trigger-travel::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0010

English:

```text
Travel
```

Japanese:

```text
旅行ガイド
```

### ITEM 0233

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0011

English:

```text
Hongdae
```

Japanese:

```text
弘大
```

### ITEM 0234

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0012

English:

```text
Myeongdong
```

Japanese:

```text
明洞
```

### ITEM 0235

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0013

English:

```text
Seongsu
```

Japanese:

```text
聖水
```

### ITEM 0236

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(4)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0014

English:

```text
Insadong
```

Japanese:

```text
仁寺洞
```

### ITEM 0237

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(5)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0015

English:

```text
Gangnam
```

Japanese:

```text
江南
```

### ITEM 0238

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(6)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0016

English:

```text
Jamsil
```

Japanese:

```text
蚕室
```

### ITEM 0239

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(7)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0017

English:

```text
Gongdeok & Mapo
```

Japanese:

```text
孔徳・麻浦
```

### ITEM 0240

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(8)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0018

English:

```text
Itaewon
```

Japanese:

```text
梨泰院
```

### ITEM 0241

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(9)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0019

English:

```text
Dongdaemun
```

Japanese:

```text
東大門
```

### ITEM 0242

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > p.site-nav__group-label:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0020

English:

```text
Seoul Areas
```

Japanese:

```text
ソウルのエリア
```

### ITEM 0243

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(2) > div.site-nav__travel-attraction-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0021

English:

```text
Lotte World
```

Japanese:

```text
ロッテワールド
```

### ITEM 0244

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(2) > div.site-nav__travel-attraction-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0022

English:

```text
Seoul Sky
```

Japanese:

```text
ソウルスカイ
```

### ITEM 0245

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(2) > p.site-nav__group-label:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0023

English:

```text
Attractions
```

Japanese:

```text
観光スポット
```

### ITEM 0246

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > p#site-nav-travel-guides-label::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0024

English:

```text
Travel Guides
```

Japanese:

```text
旅行ガイド
```

### ITEM 0247

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > button#site-nav-trigger-stay::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0025

English:

```text
Stay
```

Japanese:

```text
宿泊
```

### ITEM 0248

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0026

English:

```text
Stay Guide
```

Japanese:

```text
宿泊ガイド
```

### ITEM 0249

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(10)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0027

English:

```text
Luxury Hotels
```

Japanese:

```text
高級ホテル
```

### ITEM 0250

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0028

English:

```text
Hongdae vs Myeongdong
```

Japanese:

```text
弘大 vs 明洞
```

### ITEM 0251

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0029

English:

```text
First-Time Visitors
```

Japanese:

```text
初めてのソウル
```

### ITEM 0252

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(4)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0030

English:

```text
Families
```

Japanese:

```text
家族旅行
```

### ITEM 0253

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(5)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0031

English:

```text
Solo Travelers
```

Japanese:

```text
ひとり旅
```

### ITEM 0254

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(6)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0032

English:

```text
Couples
```

Japanese:

```text
カップル
```

### ITEM 0255

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(7)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0033

English:

```text
Budget Travelers
```

Japanese:

```text
節約派
```

### ITEM 0256

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(8)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0034

English:

```text
Shopping
```

Japanese:

```text
ショッピング
```

### ITEM 0257

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(9)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0035

English:

```text
Nightlife
```

Japanese:

```text
ナイトライフ
```

### ITEM 0258

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > p.site-nav__group-label:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0036

English:

```text
Stay Guide
```

Japanese:

```text
宿泊ガイド
```

### ITEM 0259

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(4) > button#site-nav-trigger-esim::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0037

English:

```text
eSIM
```

Japanese:

```text
eSIM
```

### ITEM 0260

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(4) > div#site-nav-panel-esim > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0038

English:

```text
eSIM Guide
```

Japanese:

```text
eSIMガイド
```

### ITEM 0261

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(4) > div#site-nav-panel-esim > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0039

English:

```text
Best eSIM for Korea
```

Japanese:

```text
韓国旅行におすすめのeSIM
```

### ITEM 0262

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(4) > div#site-nav-panel-esim > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0040

English:

```text
Korea eSIM with a Phone Number
```

Japanese:

```text
電話番号付き韓国eSIM
```

### ITEM 0263

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > button#site-nav-trigger-airport::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0041

English:

```text
Airport
```

Japanese:

```text
空港
```

### ITEM 0264

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > div#site-nav-panel-airport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0042

English:

```text
Airport Guide
```

Japanese:

```text
空港ガイド
```

### ITEM 0265

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > div#site-nav-panel-airport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0043

English:

```text
Arrival Guide
```

Japanese:

```text
到着ガイド
```

### ITEM 0266

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > div#site-nav-panel-airport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0044

English:

```text
Airport Transfer
```

Japanese:

```text
空港送迎
```

### ITEM 0267

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > div#site-nav-panel-airport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(4)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0045

English:

```text
AREX Guide
```

Japanese:

```text
AREXガイド
```

### ITEM 0268

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > div#site-nav-panel-airport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(5)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0046

English:

```text
Airport Bus Guide
```

Japanese:

```text
空港バスガイド
```

### ITEM 0269

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(6) > button#site-nav-trigger-maps::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0047

English:

```text
Maps
```

Japanese:

```text
地図
```

### ITEM 0270

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(6) > div#site-nav-panel-maps > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0048

English:

```text
Maps Guide
```

Japanese:

```text
地図アプリガイド
```

### ITEM 0271

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > button#site-nav-trigger-transport::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0049

English:

```text
Transport
```

Japanese:

```text
交通
```

### ITEM 0272

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0050

English:

```text
T-money Guide
```

Japanese:

```text
T-moneyガイド
```

### ITEM 0273

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0051

English:

```text
WOWPASS Guide
```

Japanese:

```text
WOWPASSガイド
```

### ITEM 0274

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0052

English:

```text
T-money vs WOWPASS
```

Japanese:

```text
T-money vs WOWPASS
```

### ITEM 0275

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(1) > p.site-nav__group-label:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0053

English:

```text
Travel Cards
```

Japanese:

```text
交通カード
```

### ITEM 0276

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(2) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0054

English:

```text
Taxi Guide
```

Japanese:

```text
タクシーガイド
```

### ITEM 0277

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(2) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0055

English:

```text
Call Van / Private Transfer
```

Japanese:

```text
コールバン／貸切送迎
```

### ITEM 0278

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(2) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0056

English:

```text
Rental Car
```

Japanese:

```text
レンタカー
```

### ITEM 0279

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(2) > p.site-nav__group-label:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0057

English:

```text
Other Transport
```

Japanese:

```text
その他の交通
```

### ITEM 0280

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(8) > button#site-nav-trigger-apps::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0058

English:

```text
Apps
```

Japanese:

```text
アプリ
```

### ITEM 0281

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(8) > div#site-nav-panel-apps > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0059

English:

```text
Essential Apps
```

Japanese:

```text
必須アプリ
```

### ITEM 0282

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(9) > button#site-nav-trigger-travel-tips::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0060

English:

```text
Travel Tips
```

Japanese:

```text
旅行準備
```

### ITEM 0283

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(9) > div#site-nav-panel-travel-tips > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0061

English:

```text
Korea Travel Checklist
```

Japanese:

```text
韓国旅行チェックリスト
```

### ITEM 0284

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(9) > div#site-nav-panel-travel-tips > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0062

English:

```text
Paying in Korea
```

Japanese:

```text
韓国での支払い
```

### ITEM 0285

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > div.language-switcher:nth-of-type(1) > button.language-switcher__button:nth-of-type(1) > span.language-switcher__current:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0063

English:

```text
EN
```

Japanese:

```text
JA
```

### ITEM 0286

- Element/type: common UI header/navigation direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > div.language-switcher:nth-of-type(1) > button.language-switcher__button:nth-of-type(1) > span.language-switcher__label:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0064

English:

```text
Language
```

Japanese:

```text
日本語
```

### ITEM 0287

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > div.language-switcher:nth-of-type(1)@aria-label`
- Reuse: Japanese Golden Sample Approved ITEM 0065

English:

```text
Language selector
```

Japanese:

```text
言語選択
```

### ITEM 0288

- Element/type: visible link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.arrival-hero__copy:nth-of-type(1) > p.page-hero__breadcrumb:nth-of-type(1) > a:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Home
```

Japanese:

```text
ホーム
```

### ITEM 0289

- Element/type: visible link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.arrival-hero__copy:nth-of-type(1) > p.page-hero__breadcrumb:nth-of-type(1) > a:nth-of-type(2)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Airport
```

Japanese:

```text
空港
```

### ITEM 0290

- Element/type: breadcrumb direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.arrival-hero__copy:nth-of-type(1) > p.page-hero__breadcrumb:nth-of-type(1)::text[2]`
- Reuse: NEW arrival-specific localization

English:

```text
/ Arrival Guide
```

Japanese:

```text
/ 到着ガイド
```

### ITEM 0291

- Element/type: H1
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.arrival-hero__copy:nth-of-type(1) > h1#arrival-title::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
What to Do After Landing at Incheon Airport
```

Japanese:

```text
仁川空港に到着したら何をする？
```

### ITEM 0292

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.arrival-hero__copy:nth-of-type(1) > p.arrival-hero__desc:nth-of-type(2)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Landing at Incheon Airport is fairly straightforward once you know which signs matter. If you are entering Korea, the usual route is Arrivals, immigration, baggage claim, customs and then the public arrival hall.
```

Japanese:

```text
仁川空港への到着は、見るべき案内表示が分かっていればそれほど難しくありません。韓国に入国する場合、通常は「Arrivals（到着）」→ 入国審査 → 手荷物受取 → 税関 → 一般到着ロビーの順に進みます。
```

### ITEM 0293

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.arrival-hero__copy:nth-of-type(1) > p.arrival-hero__desc:nth-of-type(3)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
If you are connecting to another flight, do not follow that sequence automatically. Follow Transfer or Connecting Flights signs and the instructions for your itinerary instead.
```

Japanese:

```text
別の便へ乗り継ぐ場合は、この順序にそのまま従わないでください。「Transfer」または「Connecting Flights」の案内表示と、自分の旅程に合った指示に従います。
```

### ITEM 0294

- Element/type: image alt
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-hero:nth-of-type(1) > div.container:nth-of-type(1) > figure.arrival-hero__media:nth-of-type(1) > img:nth-of-type(1)@alt`
- Reuse: NEW arrival-specific localization

English:

```text
Travelers with luggage walking beneath an Arrivals sign inside Incheon International Airport
```

Japanese:

```text
仁川国際空港内の「Arrivals」表示の下を、荷物を持って歩く旅行者
```

### ITEM 0295

- Element/type: figcaption
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-hero:nth-of-type(1) > div.container:nth-of-type(1) > figure.arrival-hero__media:nth-of-type(1) > figcaption:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Arrivals · Incheon International Airport
```

Japanese:

```text
到着 · 仁川国際空港
```

### ITEM 0296

- Element/type: H2
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(2) > div.container:nth-of-type(1) > div.arrival-official-cta:nth-of-type(1) > div.arrival-official-cta__copy:nth-of-type(1) > h2#flight-check-title::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Check the Terminal Your Flight Actually Uses
```

Japanese:

```text
自分の便が実際に使うターミナルを確認する
```

### ITEM 0297

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(2) > div.container:nth-of-type(1) > div.arrival-official-cta:nth-of-type(1) > div.arrival-official-cta__copy:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Terminal assignments can change, especially with codeshare flights. Your e-ticket and the airport’s live arrival search are more reliable than a saved airline list.
```

Japanese:

```text
利用ターミナルは変更されることがあり、特にコードシェア便では注意が必要です。保存した航空会社一覧より、eチケットと空港公式のリアルタイム到着便検索を確認するほうが確実です。
```

### ITEM 0298

- Element/type: CTA/link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(2) > div.container:nth-of-type(1) > div.arrival-official-cta:nth-of-type(1) > div.arrival-official-cta__action:nth-of-type(2) > a.arrival-button:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Check Your Arrival Flight
```

Japanese:

```text
到着便を確認する
```

### ITEM 0299

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(2) > div.container:nth-of-type(1) > div.arrival-official-cta:nth-of-type(1) > div.arrival-official-cta__action:nth-of-type(2) > a.arrival-button:nth-of-type(1)@aria-label`
- Reuse: NEW arrival-specific localization

English:

```text
Check your arrival flight on the official Incheon Airport website, external site, opens in a new tab
```

Japanese:

```text
仁川空港公式サイトで到着便を確認（外部サイト、新しいタブで開きます）
```

### ITEM 0300

- Element/type: H2
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(3) > div.container:nth-of-type(1) > div.arrival-section__header:nth-of-type(1) > h2#journey-type-title::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Entering Korea or Transferring?
```

Japanese:

```text
韓国に入国する？ それとも乗り継ぎ？
```

### ITEM 0301

- Element/type: H3
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(3) > div.container:nth-of-type(1) > div.arrival-editorial-list:nth-of-type(2) > article.arrival-editorial-block:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Entering Korea
```

Japanese:

```text
韓国に入国する場合
```

### ITEM 0302

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(3) > div.container:nth-of-type(1) > div.arrival-editorial-list:nth-of-type(2) > article.arrival-editorial-block:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
If this is your final flight and you are entering Korea, follow the Arrivals or Immigration signs. The usual sequence is immigration first, checked baggage next, customs after that and then the public arrival hall.
```

Japanese:

```text
この便が最終便で韓国に入国するなら、「Arrivals」または「Immigration」の案内表示に従ってください。通常は、最初に入国審査、次に預け荷物の受取、その後に税関を通り、一般到着ロビーへ出ます。
```

### ITEM 0303

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(3) > div.container:nth-of-type(1) > div.arrival-editorial-list:nth-of-type(2) > article.arrival-editorial-block:nth-of-type(1) > p.arrival-route-line:nth-of-type(2)@aria-label`
- Reuse: NEW arrival-specific localization

English:

```text
Immigration, then baggage claim, then customs, then arrival hall
```

Japanese:

```text
入国審査、手荷物受取、税関、到着ロビーの順
```

### ITEM 0304

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(3) > div.container:nth-of-type(1) > div.arrival-editorial-list:nth-of-type(2) > article.arrival-editorial-block:nth-of-type(1) > p.arrival-route-line:nth-of-type(2) > span:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Immigration
```

Japanese:

```text
入国審査
```

### ITEM 0305

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(3) > div.container:nth-of-type(1) > div.arrival-editorial-list:nth-of-type(2) > article.arrival-editorial-block:nth-of-type(1) > p.arrival-route-line:nth-of-type(2) > span:nth-of-type(3)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Baggage Claim
```

Japanese:

```text
手荷物受取
```

### ITEM 0306

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(3) > div.container:nth-of-type(1) > div.arrival-editorial-list:nth-of-type(2) > article.arrival-editorial-block:nth-of-type(1) > p.arrival-route-line:nth-of-type(2) > span:nth-of-type(5)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Customs
```

Japanese:

```text
税関
```

### ITEM 0307

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(3) > div.container:nth-of-type(1) > div.arrival-editorial-list:nth-of-type(2) > article.arrival-editorial-block:nth-of-type(1) > p.arrival-route-line:nth-of-type(2) > span:nth-of-type(7)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Arrival Hall
```

Japanese:

```text
到着ロビー
```

### ITEM 0308

- Element/type: H3
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(3) > div.container:nth-of-type(1) > div.arrival-editorial-list:nth-of-type(2) > article.arrival-editorial-block:nth-of-type(2) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Connecting to another flight
```

Japanese:

```text
別の便へ乗り継ぐ場合
```

### ITEM 0309

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(3) > div.container:nth-of-type(1) > div.arrival-editorial-list:nth-of-type(2) > article.arrival-editorial-block:nth-of-type(2) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
If you are only connecting at Incheon, follow Transfer or Connecting Flights signs instead of joining the general arrival flow automatically. Your connection can depend on the airline, terminal and whether your checked baggage is transferred through to the next flight.
```

Japanese:

```text
仁川空港で乗り継ぐだけなら、一般の到着ルートへそのまま進まず、「Transfer」または「Connecting Flights」の案内表示に従ってください。乗り継ぎ方法は、航空会社、ターミナル、預け荷物が次の便まで通しで運ばれるかによって変わります。
```

### ITEM 0310

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(3) > div.container:nth-of-type(1) > div.arrival-editorial-list:nth-of-type(2) > article.arrival-editorial-block:nth-of-type(2) > p:nth-of-type(2)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
If anything is unclear, use your airline’s transfer instructions rather than assuming that every connection requires Korean immigration.
```

Japanese:

```text
分からないことがある場合は、すべての乗り継ぎで韓国の入国審査が必要だと決めつけず、航空会社の乗り継ぎ案内を確認してください。
```

### ITEM 0311

- Element/type: H2
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-section__header:nth-of-type(1) > h2#arrival-steps-title::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
From the Aircraft to the Arrival Hall
```

Japanese:

```text
機内を降りて到着ロビーに出るまで
```

### ITEM 0312

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-section__header:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
For travelers entering Korea, the route is usually simple. The walking distance can vary by gate and terminal, but the order of the main steps is the same.
```

Japanese:

```text
韓国に入国する旅行者の場合、流れ自体はシンプルです。ゲートやターミナルによって歩く距離は変わりますが、主な手続きの順番は同じです。
```

### ITEM 0313

- Element/type: image alt
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > figure.arrival-steps-media:nth-of-type(1) > img:nth-of-type(1)@alt`
- Reuse: NEW arrival-specific localization

English:

```text
Travelers following Immigration and Baggage Claim signs inside Incheon International Airport
```

Japanese:

```text
仁川国際空港内で「Immigration」と「Baggage Claim」の案内に従う旅行者
```

### ITEM 0314

- Element/type: figcaption
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > figure.arrival-steps-media:nth-of-type(1) > figcaption:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Follow the airport signs in order; your exact walking route depends on the gate and terminal.
```

Japanese:

```text
空港内の案内表示を順にたどってください。実際に歩くルートはゲートとターミナルによって変わります。
```

### ITEM 0315

- Element/type: H3
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(1) > div.arrival-step__content:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Follow the Arrivals Signs
```

Japanese:

```text
「Arrivals」の案内表示に従う
```

### ITEM 0316

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(1) > div.arrival-step__content:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
After leaving the aircraft, follow signs for Arrivals or Immigration. Transfer and Connecting Flights signs lead to a different process, so use them only when you are continuing to another flight.
```

Japanese:

```text
機内を降りたら、「Arrivals」または「Immigration」の案内表示に従います。「Transfer」や「Connecting Flights」は別の手続きへ進む表示なので、別の便へ乗り継ぐ場合にだけ従ってください。
```

### ITEM 0317

- Element/type: H3
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(2) > div.arrival-step__content:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Immigration
```

Japanese:

```text
入国審査
```

### ITEM 0318

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(2) > div.arrival-step__content:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Have your passport and the entry documents that apply to your own travel status ready. The rules are not identical for every passport, visa or residence status, so check the official Korean guidance before the flight rather than copying another traveler’s checklist.
```

Japanese:

```text
パスポートと、自分の渡航条件に必要な入国書類を用意しておきましょう。必要条件はパスポート、ビザ、在留資格などによって同じではありません。他の旅行者のチェックリストをそのまま真似せず、出発前に韓国の公式案内を確認してください。
```

### ITEM 0319

- Element/type: H4
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(2) > div.arrival-step__content:nth-of-type(1) > div.arrival-entry-note:nth-of-type(1) > h4:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
K-ETA and the e-Arrival Card are not the same thing
```

Japanese:

```text
K-ETAとe-Arrival Cardは別のもの
```

### ITEM 0320

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(2) > div.arrival-step__content:nth-of-type(1) > div.arrival-entry-note:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Korea’s temporary K-ETA exemption for countries already covered by the measure has been extended through December 31, 2026. Being exempt from K-ETA does not automatically mean that you are exempt from the arrival declaration.
```

Japanese:

```text
現在、一時的なK-ETA免除措置の対象となっている国については、免除期間が2026年12月31日まで延長されています。K-ETAが免除されているからといって、入国申告まで自動的に免除されるわけではありません。
```

### ITEM 0321

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(2) > div.arrival-step__content:nth-of-type(1) > div.arrival-entry-note:nth-of-type(1) > p:nth-of-type(2)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
A traveler who holds a valid K-ETA is exempt from the arrival declaration. A traveler using the temporary K-ETA exemption may still need to submit an e-Arrival Card, depending on status.
```

Japanese:

```text
有効なK-ETAを持っている旅行者は、入国申告が免除されます。一方、一時的なK-ETA免除を利用する旅行者は、渡航資格によってはe-Arrival Cardの提出が必要になる場合があります。
```

### ITEM 0322

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(2) > div.arrival-step__content:nth-of-type(1) > div.arrival-entry-note:nth-of-type(1) > p:nth-of-type(3)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
The official e-Arrival Card is free and can be submitted within three days before arrival in Korea. The official website also has a navigator that tells you whether you need to submit one.
```

Japanese:

```text
公式のe-Arrival Cardは無料で、韓国到着の3日前から提出できます。公式サイトには、自分が提出対象かどうかを確認できる案内機能もあります。
```

### ITEM 0323

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(2) > div.arrival-step__content:nth-of-type(1) > div.arrival-entry-note:nth-of-type(1) > p.arrival-entry-note__warning:nth-of-type(4)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
The official e-Arrival Card website does not charge a fee. Do not enter payment information on a website claiming to sell the Korean arrival card.
```

Japanese:

```text
公式e-Arrival Cardサイトでは料金はかかりません。韓国の入国カードを販売すると称するサイトに、支払い情報を入力しないでください。
```

### ITEM 0324

- Element/type: visible link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(2) > div.arrival-step__content:nth-of-type(1) > div.arrival-entry-note:nth-of-type(1) > p.arrival-context-links:nth-of-type(5) > a:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
K-ETA official
```

Japanese:

```text
K-ETA公式サイト
```

### ITEM 0325

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(2) > div.arrival-step__content:nth-of-type(1) > div.arrival-entry-note:nth-of-type(1) > p.arrival-context-links:nth-of-type(5) > a:nth-of-type(1)@aria-label`
- Reuse: NEW arrival-specific localization

English:

```text
K-ETA official website, external site, opens in a new tab
```

Japanese:

```text
K-ETA公式サイト（外部サイト、新しいタブで開きます）
```

### ITEM 0326

- Element/type: visible link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(2) > div.arrival-step__content:nth-of-type(1) > div.arrival-entry-note:nth-of-type(1) > p.arrival-context-links:nth-of-type(5) > a:nth-of-type(2)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
e-Arrival Card official
```

Japanese:

```text
e-Arrival Card公式サイト
```

### ITEM 0327

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(2) > div.arrival-step__content:nth-of-type(1) > div.arrival-entry-note:nth-of-type(1) > p.arrival-context-links:nth-of-type(5) > a:nth-of-type(2)@aria-label`
- Reuse: NEW arrival-specific localization

English:

```text
Official Korea e-Arrival Card website, external site, opens in a new tab
```

Japanese:

```text
韓国公式e-Arrival Cardサイト（外部サイト、新しいタブで開きます）
```

### ITEM 0328

- Element/type: H3
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(3) > div.arrival-step__content:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Baggage Claim
```

Japanese:

```text
手荷物受取
```

### ITEM 0329

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(3) > div.arrival-step__content:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
After immigration, check the airport display for the baggage belt assigned to your flight and collect your checked bags before continuing toward customs.
```

Japanese:

```text
入国審査を終えたら、空港の案内表示で自分の便に割り当てられた手荷物受取レーンを確認し、預け荷物を受け取ってから税関へ進みます。
```

### ITEM 0330

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(3) > div.arrival-step__content:nth-of-type(1) > p:nth-of-type(2)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
If your suitcase does not appear, first make sure the belt still matches your flight. If it is genuinely missing or damaged, speak to your airline or the baggage service desk while you are still inside the baggage area. Keep the baggage tag from check-in and your flight details with you.
```

Japanese:

```text
スーツケースが出てこない場合は、まず手荷物受取レーンがまだ自分の便に割り当てられているか確認してください。本当に見つからない、または破損している場合は、手荷物受取エリアにいるうちに航空会社または手荷物サービスカウンターへ相談してください。チェックイン時の手荷物タグと便情報も手元に用意しておきましょう。
```

### ITEM 0331

- Element/type: H3
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(4) > div.arrival-step__content:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Customs
```

Japanese:

```text
税関
```

### ITEM 0332

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(4) > div.arrival-step__content:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
After collecting your baggage, follow the customs signs toward the exit.
```

Japanese:

```text
荷物を受け取ったら、税関の案内表示に従って出口へ進みます。
```

### ITEM 0333

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(4) > div.arrival-step__content:nth-of-type(1) > p:nth-of-type(2)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
If you are carrying goods that may need to be declared, check Korea Customs rather than relying on rules from another country. Items over the duty-free allowance and restricted goods can require a declaration.
```

Japanese:

```text
申告が必要になる可能性のある品物を持っている場合は、他国のルールではなくKorea Customs Serviceの案内を確認してください。免税範囲を超える品物や規制対象品は、申告が必要になることがあります。
```

### ITEM 0334

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(4) > div.arrival-step__content:nth-of-type(1) > p:nth-of-type(3)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
If you have nothing to declare, follow the appropriate customs channel shown at the airport.
```

Japanese:

```text
申告するものがない場合は、空港に表示されている適切な税関レーンへ進んでください。
```

### ITEM 0335

- Element/type: visible link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(4) > div.arrival-step__content:nth-of-type(1) > p.arrival-context-link:nth-of-type(4) > a:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Korea Customs Service — Travelers
```

Japanese:

```text
Korea Customs Service — 旅行者向け情報
```

### ITEM 0336

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(4) > div.arrival-step__content:nth-of-type(1) > p.arrival-context-link:nth-of-type(4) > a:nth-of-type(1)@aria-label`
- Reuse: NEW arrival-specific localization

English:

```text
Korea Customs Service travelers information, external site, opens in a new tab
```

Japanese:

```text
Korea Customs Serviceの旅行者向け情報（外部サイト、新しいタブで開きます）
```

### ITEM 0337

- Element/type: H3
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(5) > div.arrival-step__content:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Arrival Hall
```

Japanese:

```text
到着ロビー
```

### ITEM 0338

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(5) > div.arrival-step__content:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Once you pass customs and enter the public arrival hall, the immigration part of the journey is finished.
```

Japanese:

```text
税関を通って一般到着ロビーに出れば、入国手続きの部分は完了です。
```

### ITEM 0339

- Element/type: visible link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(5) > div.arrival-step__content:nth-of-type(1) > p:nth-of-type(2) > a:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Incheon Airport Arrival Hall Guide
```

Japanese:

```text
仁川空港 到着ロビーガイド
```

### ITEM 0340

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(4) > div.container:nth-of-type(1) > div.arrival-steps-layout:nth-of-type(2) > ol.arrival-timeline:nth-of-type(1) > li.arrival-step:nth-of-type(5) > div.arrival-step__content:nth-of-type(1) > p:nth-of-type(2)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Before heading into the city, you may still need to sort out mobile data, save your Korean accommodation details, check your payment backup or decide how to reach your stay. Those first practical steps are covered separately in the
```

Japanese:

```text
市内へ向かう前に、モバイルデータの確認、宿泊先の韓国語情報の保存、予備の支払い手段、宿泊先までの移動方法を整える必要があるかもしれません。旅行開始直後のこうした実用的な準備は、別ページでまとめています：
```

### ITEM 0341

- Element/type: H2
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(5) > div.container:nth-of-type(1) > div.arrival-section__header:nth-of-type(1) > h2#terminal-title::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Terminal 1 and Terminal 2 Are Separate Buildings
```

Japanese:

```text
第1ターミナルと第2ターミナルは別の建物
```

### ITEM 0342

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(5) > div.container:nth-of-type(1) > div.arrival-section__header:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
You do not need to memorize a fixed list of airlines for each terminal. Check the terminal shown for your actual flight number, then use the map for orientation after landing.
```

Japanese:

```text
ターミナルごとの固定された航空会社一覧を覚える必要はありません。実際の便名に表示されているターミナルを確認し、到着後はマップを使って位置関係を把握してください。
```

### ITEM 0343

- Element/type: H3
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(5) > div.container:nth-of-type(1) > div.arrival-terminal-layout:nth-of-type(2) > div.arrival-terminal-summary:nth-of-type(1) > article.arrival-terminal-row:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Terminal 1
```

Japanese:

```text
第1ターミナル
```

### ITEM 0344

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(5) > div.container:nth-of-type(1) > div.arrival-terminal-layout:nth-of-type(2) > div.arrival-terminal-summary:nth-of-type(1) > article.arrival-terminal-row:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Terminal 1 is a separate passenger terminal with its own immigration, baggage claim and arrival hall. Follow the signs for the flight you actually arrived on rather than using a saved airline list.
```

Japanese:

```text
第1ターミナルは独立した旅客ターミナルで、入国審査、手荷物受取、到着ロビーもそれぞれ設けられています。保存した航空会社一覧ではなく、実際に到着した便の案内表示に従ってください。
```

### ITEM 0345

- Element/type: H3
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(5) > div.container:nth-of-type(1) > div.arrival-terminal-layout:nth-of-type(2) > div.arrival-terminal-summary:nth-of-type(1) > article.arrival-terminal-row:nth-of-type(2) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Terminal 2
```

Japanese:

```text
第2ターミナル
```

### ITEM 0346

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(5) > div.container:nth-of-type(1) > div.arrival-terminal-layout:nth-of-type(2) > div.arrival-terminal-summary:nth-of-type(1) > article.arrival-terminal-row:nth-of-type(2) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Terminal 2 has its own arrival facilities and is not connected to Terminal 1 by a public walking route. If you need to move between terminals after entering the public area, use the airport’s current inter-terminal transportation guidance.
```

Japanese:

```text
第2ターミナルにも専用の到着施設があり、第1ターミナルとの間に一般利用できる徒歩ルートはありません。一般区域に出た後でターミナル間を移動する必要がある場合は、仁川空港の最新のターミナル間交通案内を確認してください。
```

### ITEM 0347

- Element/type: visible link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(5) > div.container:nth-of-type(1) > div.arrival-terminal-layout:nth-of-type(2) > div.arrival-terminal-summary:nth-of-type(1) > article.arrival-terminal-row:nth-of-type(2) > p.arrival-context-link:nth-of-type(2) > a:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Incheon Airport — Inter-terminal Transportation
```

Japanese:

```text
仁川空港 — ターミナル間交通
```

### ITEM 0348

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(5) > div.container:nth-of-type(1) > div.arrival-terminal-layout:nth-of-type(2) > div.arrival-terminal-summary:nth-of-type(1) > article.arrival-terminal-row:nth-of-type(2) > p.arrival-context-link:nth-of-type(2) > a:nth-of-type(1)@aria-label`
- Reuse: NEW arrival-specific localization

English:

```text
Incheon Airport inter-terminal transportation guidance, external site, opens in a new tab
```

Japanese:

```text
仁川空港のターミナル間交通案内（外部サイト、新しいタブで開きます）
```

### ITEM 0349

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(5) > div.container:nth-of-type(1) > div.arrival-terminal-layout:nth-of-type(2) > div.arrival-terminal-summary:nth-of-type(1) > p.arrival-terminal-orientation:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
The map is useful for understanding where immigration, baggage claim, customs and the public arrival hall sit in relation to one another. For live gate, belt or facility information, use the airport’s current signs and flight information.
```

Japanese:

```text
このマップを見ると、入国審査、手荷物受取、税関、一般到着ロビーの位置関係を把握しやすくなります。ゲート、手荷物受取レーン、施設のリアルタイム情報は、空港内の最新案内表示と運航情報を確認してください。
```

### ITEM 0350

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(5) > div.container:nth-of-type(1) > div.arrival-terminal-layout:nth-of-type(2) > figure.arrival-terminal-map:nth-of-type(1) > a.arrival-terminal-map__media:nth-of-type(1)@aria-label`
- Reuse: NEW arrival-specific localization

English:

```text
Open the Incheon Airport Terminal 1 and Terminal 2 arrival map at full size in a new tab
```

Japanese:

```text
仁川空港の第1ターミナル・第2ターミナル到着マップをフルサイズで開く（新しいタブ）
```

### ITEM 0351

- Element/type: image alt
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(5) > div.container:nth-of-type(1) > div.arrival-terminal-layout:nth-of-type(2) > figure.arrival-terminal-map:nth-of-type(1) > a.arrival-terminal-map__media:nth-of-type(1) > img.arrival-terminal-map__image:nth-of-type(1)@alt`
- Reuse: NEW arrival-specific localization

English:

```text
Combined Incheon Airport Terminal 1 and Terminal 2 arrival maps showing immigration, baggage claim, customs, arrival halls, transport and service locations
```

Japanese:

```text
入国審査、手荷物受取、税関、到着ロビー、交通機関、サービス施設の位置を示した仁川空港 第1ターミナル・第2ターミナルの到着マップ
```

### ITEM 0352

- Element/type: figcaption
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(5) > div.container:nth-of-type(1) > div.arrival-terminal-layout:nth-of-type(2) > figure.arrival-terminal-map:nth-of-type(1) > figcaption.arrival-terminal-map__caption:nth-of-type(1) > span:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Terminal 1 and Terminal 2 arrival maps. Use them for orientation; live airport signs and flight information take priority.
```

Japanese:

```text
第1ターミナル・第2ターミナルの到着マップ。位置確認に使い、最新の空港案内表示と運航情報を優先してください。
```

### ITEM 0353

- Element/type: figcaption visible link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(5) > div.container:nth-of-type(1) > div.arrival-terminal-layout:nth-of-type(2) > figure.arrival-terminal-map:nth-of-type(1) > figcaption.arrival-terminal-map__caption:nth-of-type(1) > a.arrival-terminal-map__link:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Open the terminal map at full size
```

Japanese:

```text
ターミナルマップをフルサイズで開く
```

### ITEM 0354

- Element/type: H2
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(6) > div.container:nth-of-type(1) > h2#arrival-handoff-title::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
You’re Through — What Happens Next?
```

Japanese:

```text
入国手続き完了。その次は？
```

### ITEM 0355

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(6) > div.container:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Once you are in the public arrival hall, the entry process is behind you. What matters next depends on the trip: getting your phone online, keeping the accommodation address in Korean, having a payment backup and choosing a route that still works with your luggage.
```

Japanese:

```text
一般到着ロビーに出れば、入国手続きは終わりです。その後に必要なことは旅によって変わります。スマホをネットにつなぐ、宿泊先の住所を韓国語で保存する、予備の支払い手段を用意する、荷物を持って無理なく移動できるルートを選ぶ、といった準備です。
```

### ITEM 0356

- Element/type: visible link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(6) > div.container:nth-of-type(1) > p:nth-of-type(2) > a:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Incheon Airport Arrival Hall Guide
```

Japanese:

```text
仁川空港 到着ロビーガイド
```

### ITEM 0357

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(6) > div.container:nth-of-type(1) > p:nth-of-type(2)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Continue with the
```

Japanese:

```text
旅行開始直後の実用的な準備は、
```

### ITEM 0358

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(6) > div.container:nth-of-type(1) > p:nth-of-type(2)::text[2]`
- Reuse: NEW arrival-specific localization

English:

```text
for those first practical steps.
```

Japanese:

```text
で確認できます。
```

### ITEM 0359

- Element/type: visible link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(6) > div.container:nth-of-type(1) > p:nth-of-type(3) > a:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Airport Transfer Guide
```

Japanese:

```text
空港送迎ガイド
```

### ITEM 0360

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(6) > div.container:nth-of-type(1) > p:nth-of-type(3)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
If your only remaining question is how to reach your accommodation, the
```

Japanese:

```text
宿泊先までの行き方だけを決めたいなら、
```

### ITEM 0361

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(6) > div.container:nth-of-type(1) > p:nth-of-type(3)::text[2]`
- Reuse: NEW arrival-specific localization

English:

```text
compares AREX, airport bus, taxi and pre-booked transfer by the whole door-to-door journey.
```

Japanese:

```text
で、AREX、空港バス、タクシー、事前予約送迎をドア・ツー・ドアの全行程で比較できます。
```

### ITEM 0362

- Element/type: H2
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(7) > div.container:nth-of-type(1) > div.arrival-section__header:nth-of-type(1) > h2#faq-title::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Incheon Airport Arrival FAQ
```

Japanese:

```text
仁川空港 到着FAQ
```

### ITEM 0363

- Element/type: visible FAQ question
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(7) > div.container:nth-of-type(1) > div.arrival-faq:nth-of-type(2) > details:nth-of-type(1) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
How do I know whether my flight arrives at Terminal 1 or Terminal 2?
```

Japanese:

```text
自分の便が第1ターミナルと第2ターミナルのどちらに到着するか、どう確認すればいい？
```

### ITEM 0364

- Element/type: visible FAQ answer/direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(7) > div.container:nth-of-type(1) > div.arrival-faq:nth-of-type(2) > details:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Check your e-ticket or search your flight number on the official Incheon Airport arrival page. That is safer than relying on a fixed airline list, particularly for codeshare flights.
```

Japanese:

```text
eチケットを確認するか、仁川空港公式の到着便ページで便名を検索してください。特にコードシェア便では、固定された航空会社一覧に頼るより確実です。
```

### ITEM 0365

- Element/type: visible FAQ question
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(7) > div.container:nth-of-type(1) > div.arrival-faq:nth-of-type(2) > details:nth-of-type(2) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Do I go through immigration before collecting my baggage?
```

Japanese:

```text
手荷物を受け取る前に入国審査を通る？
```

### ITEM 0366

- Element/type: visible FAQ answer/direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(7) > div.container:nth-of-type(1) > div.arrival-faq:nth-of-type(2) > details:nth-of-type(2) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Yes, if you are entering Korea. The usual order is immigration, baggage claim, customs and then the public arrival hall. Passengers connecting to another flight should follow Transfer or Connecting Flights signs and their airline’s instructions instead.
```

Japanese:

```text
韓国に入国する場合は、はい。通常は「入国審査 → 手荷物受取 → 税関 → 一般到着ロビー」の順です。別の便へ乗り継ぐ場合は、この流れではなく「Transfer」または「Connecting Flights」の案内表示と航空会社の指示に従ってください。
```

### ITEM 0367

- Element/type: visible FAQ question
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(7) > div.container:nth-of-type(1) > div.arrival-faq:nth-of-type(2) > details:nth-of-type(3) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
What should I do if my baggage does not arrive?
```

Japanese:

```text
預けた荷物が出てこないときはどうすればいい？
```

### ITEM 0368

- Element/type: visible FAQ answer/direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(7) > div.container:nth-of-type(1) > div.arrival-faq:nth-of-type(2) > details:nth-of-type(3) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
First check that the baggage belt still matches your flight. If the bag is missing or damaged, contact your airline or the baggage service desk before leaving the baggage area, and keep your baggage tag and flight details ready.
```

Japanese:

```text
まず、手荷物受取レーンがまだ自分の便に割り当てられているか確認してください。荷物が見つからない、または破損している場合は、手荷物受取エリアを出る前に航空会社または手荷物サービスカウンターへ連絡し、手荷物タグと便情報を用意しておきましょう。
```

### ITEM 0369

- Element/type: visible FAQ question
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(7) > div.container:nth-of-type(1) > div.arrival-faq:nth-of-type(2) > details:nth-of-type(4) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Can I walk between Terminal 1 and Terminal 2?
```

Japanese:

```text
第1ターミナルと第2ターミナルの間は歩いて移動できる？
```

### ITEM 0370

- Element/type: visible FAQ answer/direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(7) > div.container:nth-of-type(1) > div.arrival-faq:nth-of-type(2) > details:nth-of-type(4) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
No. Terminal 1 and Terminal 2 are separate buildings without a public walking route between them. If you need to change terminals in the public area, use the airport’s official inter-terminal transportation guidance.
```

Japanese:

```text
できません。第1ターミナルと第2ターミナルは別の建物で、一般区域を徒歩で移動するルートはありません。一般区域でターミナルを移動する必要がある場合は、仁川空港公式のターミナル間交通案内を確認してください。
```

### ITEM 0371

- Element/type: visible FAQ question
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(7) > div.container:nth-of-type(1) > div.arrival-faq:nth-of-type(2) > details:nth-of-type(5) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Which transport should I choose from Incheon Airport to Seoul?
```

Japanese:

```text
仁川空港からソウルへは、どの移動手段を選べばいい？
```

### ITEM 0372

- Element/type: visible FAQ answer/direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(7) > div.container:nth-of-type(1) > div.arrival-faq:nth-of-type(2) > details:nth-of-type(5) > p:nth-of-type(1) > a:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Airport Transfer Guide
```

Japanese:

```text
空港送迎ガイド
```

### ITEM 0373

- Element/type: visible FAQ answer/direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(7) > div.container:nth-of-type(1) > div.arrival-faq:nth-of-type(2) > details:nth-of-type(5) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Make that decision after you reach the public arrival hall. The best option depends on the exact accommodation, arrival time, luggage and group size. The separate
```

Japanese:

```text
一般到着ロビーに出てから決めれば大丈夫です。最適な方法は、実際の宿泊先、到着時刻、荷物の量、人数によって変わります。詳しくは
```

### ITEM 0374

- Element/type: visible FAQ answer/direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(7) > div.container:nth-of-type(1) > div.arrival-faq:nth-of-type(2) > details:nth-of-type(5) > p:nth-of-type(1)::text[2]`
- Reuse: NEW arrival-specific localization

English:

```text
compares AREX, airport bus, taxi and pre-booked transfer by the full journey to your stay.
```

Japanese:

```text
で、AREX、空港バス、タクシー、事前予約送迎を、宿泊先までの全行程で比較しています。
```

### ITEM 0375

- Element/type: visible FAQ question
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(7) > div.container:nth-of-type(1) > div.arrival-faq:nth-of-type(2) > details:nth-of-type(6) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
What should transfer passengers do?
```

Japanese:

```text
乗り継ぎ客はどうすればいい？
```

### ITEM 0376

- Element/type: visible FAQ answer/direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(7) > div.container:nth-of-type(1) > div.arrival-faq:nth-of-type(2) > details:nth-of-type(6) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Follow Transfer or Connecting Flights signs and your airline’s instructions. Confirm the connection terminal, available connection time and whether checked baggage is transferred through to the next flight instead of assuming that you should follow the general immigration route.
```

Japanese:

```text
「Transfer」または「Connecting Flights」の案内表示と航空会社の指示に従ってください。一般の入国ルートへ進むと決めつけず、乗り継ぎターミナル、乗り継ぎ可能時間、預け荷物が次の便まで通しで運ばれるかを確認してください。
```

### ITEM 0377

- Element/type: H2
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(8) > div.container:nth-of-type(1) > div.arrival-section__header:nth-of-type(1) > h2#official-sources-title::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Official Sources
```

Japanese:

```text
公式情報
```

### ITEM 0378

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(8) > div.container:nth-of-type(1) > div.arrival-section__header:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Entry rules and airport operations can change. Use these official sources when a detail affects your own trip.
```

Japanese:

```text
入国条件や空港の運用は変更されることがあります。自分の旅行に影響する内容は、以下の公式情報で確認してください。
```

### ITEM 0379

- Element/type: visible link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(8) > div.container:nth-of-type(1) > ul.arrival-official-sources:nth-of-type(1) > li:nth-of-type(1) > a:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Incheon Airport — Arrival Procedures
```

Japanese:

```text
仁川空港 — 到着手続き
```

### ITEM 0380

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(8) > div.container:nth-of-type(1) > ul.arrival-official-sources:nth-of-type(1) > li:nth-of-type(1) > a:nth-of-type(1)@aria-label`
- Reuse: NEW arrival-specific localization

English:

```text
Incheon Airport arrival procedures, external site, opens in a new tab
```

Japanese:

```text
仁川空港の到着手続き（外部サイト、新しいタブで開きます）
```

### ITEM 0381

- Element/type: visible link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(8) > div.container:nth-of-type(1) > ul.arrival-official-sources:nth-of-type(1) > li:nth-of-type(2) > a:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Incheon Airport — Arrival Flights
```

Japanese:

```text
仁川空港 — 到着便
```

### ITEM 0382

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(8) > div.container:nth-of-type(1) > ul.arrival-official-sources:nth-of-type(1) > li:nth-of-type(2) > a:nth-of-type(1)@aria-label`
- Reuse: NEW arrival-specific localization

English:

```text
Incheon Airport arrival flights, external site, opens in a new tab
```

Japanese:

```text
仁川空港の到着便情報（外部サイト、新しいタブで開きます）
```

### ITEM 0383

- Element/type: visible link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(8) > div.container:nth-of-type(1) > ul.arrival-official-sources:nth-of-type(1) > li:nth-of-type(3) > a:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Incheon Airport — Inter-terminal Transportation
```

Japanese:

```text
仁川空港 — ターミナル間交通
```

### ITEM 0384

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(8) > div.container:nth-of-type(1) > ul.arrival-official-sources:nth-of-type(1) > li:nth-of-type(3) > a:nth-of-type(1)@aria-label`
- Reuse: NEW arrival-specific localization

English:

```text
Incheon Airport inter-terminal transportation, external site, opens in a new tab
```

Japanese:

```text
仁川空港のターミナル間交通（外部サイト、新しいタブで開きます）
```

### ITEM 0385

- Element/type: visible link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(8) > div.container:nth-of-type(1) > ul.arrival-official-sources:nth-of-type(1) > li:nth-of-type(4) > a:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
K-ETA
```

Japanese:

```text
K-ETA
```

### ITEM 0386

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(8) > div.container:nth-of-type(1) > ul.arrival-official-sources:nth-of-type(1) > li:nth-of-type(4) > a:nth-of-type(1)@aria-label`
- Reuse: NEW arrival-specific localization

English:

```text
K-ETA official website, external site, opens in a new tab
```

Japanese:

```text
K-ETA公式サイト（外部サイト、新しいタブで開きます）
```

### ITEM 0387

- Element/type: visible link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(8) > div.container:nth-of-type(1) > ul.arrival-official-sources:nth-of-type(1) > li:nth-of-type(5) > a:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Official Korea e-Arrival Card
```

Japanese:

```text
韓国公式e-Arrival Card
```

### ITEM 0388

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(8) > div.container:nth-of-type(1) > ul.arrival-official-sources:nth-of-type(1) > li:nth-of-type(5) > a:nth-of-type(1)@aria-label`
- Reuse: NEW arrival-specific localization

English:

```text
Official Korea e-Arrival Card website, external site, opens in a new tab
```

Japanese:

```text
韓国公式e-Arrival Cardサイト（外部サイト、新しいタブで開きます）
```

### ITEM 0389

- Element/type: visible link direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(8) > div.container:nth-of-type(1) > ul.arrival-official-sources:nth-of-type(1) > li:nth-of-type(6) > a:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Korea Customs Service
```

Japanese:

```text
Korea Customs Service
```

### ITEM 0390

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > section.arrival-section:nth-of-type(8) > div.container:nth-of-type(1) > ul.arrival-official-sources:nth-of-type(1) > li:nth-of-type(6) > a:nth-of-type(1)@aria-label`
- Reuse: NEW arrival-specific localization

English:

```text
Korea Customs Service, external site, opens in a new tab
```

Japanese:

```text
Korea Customs Service（外部サイト、新しいタブで開きます）
```

### ITEM 0391

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > aside.arrival-service-notice:nth-of-type(1)@aria-label`
- Reuse: NEW arrival-specific localization

English:

```text
Information review notice
```

Japanese:

```text
情報確認に関する注意
```

### ITEM 0392

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > aside.arrival-service-notice:nth-of-type(1) > div.container:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Airport operations, entry requirements and transport services may change. Confirm time-sensitive details on the relevant official website or with your airline.
```

Japanese:

```text
空港の運用、入国条件、交通サービスは変更されることがあります。時期によって変わる情報は、該当する公式サイトまたは航空会社で確認してください。
```

### ITEM 0393

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > aside.arrival-service-notice:nth-of-type(1) > div.container:nth-of-type(1) > p:nth-of-type(2) > strong:nth-of-type(1)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
Last reviewed:
```

Japanese:

```text
最終確認：
```

### ITEM 0394

- Element/type: body direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > main:nth-of-type(1) > aside.arrival-service-notice:nth-of-type(1) > div.container:nth-of-type(1) > p:nth-of-type(2)::text[1]`
- Reuse: NEW arrival-specific localization

English:

```text
August 2026
```

Japanese:

```text
2026年8月
```

### ITEM 0395

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > div.footer__brand-block:nth-of-type(1) > p.footer__brand:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0535

English:

```text
Korea Inside
```

Japanese:

```text
Korea Inside
```

### ITEM 0396

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > div.footer__brand-block:nth-of-type(1) > p.footer__eyebrow:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0536

English:

```text
CREATED IN KOREA
```

Japanese:

```text
韓国発
```

### ITEM 0397

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > div.footer__brand-block:nth-of-type(1) > p.footer__statement:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0537

English:

```text
Practical Korea travel guidance, written and reviewed locally by a Korean editor.
```

Japanese:

```text
韓国人エディターが現地で執筆・確認する、実用重視の韓国旅行ガイドです。
```

### ITEM 0398

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > div.footer__brand-block:nth-of-type(1) > p.footer__description:nth-of-type(4)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0538

English:

```text
Based on official sources, local context, and independent editorial judgment.
```

Japanese:

```text
公式情報、現地事情、独立した編集判断をもとに作成しています。
```

### ITEM 0399

- Element/type: literal ARIA label
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1)@aria-label`
- Reuse: Japanese Golden Sample Approved ITEM 0539

English:

```text
Footer navigation
```

Japanese:

```text
フッターナビゲーション
```

### ITEM 0400

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(1) > p.footer__heading:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0540

English:

```text
PLAN YOUR TRIP
```

Japanese:

```text
旅行を計画する
```

### ITEM 0401

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(1) > ul.footer__links:nth-of-type(1) > li:nth-of-type(1) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0541

English:

```text
Airport
```

Japanese:

```text
空港
```

### ITEM 0402

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(1) > ul.footer__links:nth-of-type(1) > li:nth-of-type(2) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0542

English:

```text
eSIM
```

Japanese:

```text
eSIM
```

### ITEM 0403

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(1) > ul.footer__links:nth-of-type(1) > li:nth-of-type(3) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0543

English:

```text
Checklist
```

Japanese:

```text
チェックリスト
```

### ITEM 0404

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(2) > p.footer__heading:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0544

English:

```text
USE KOREA
```

Japanese:

```text
韓国で使う
```

### ITEM 0405

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(2) > ul.footer__links:nth-of-type(1) > li:nth-of-type(1) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0545

English:

```text
T-money
```

Japanese:

```text
T-money
```

### ITEM 0406

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(2) > ul.footer__links:nth-of-type(1) > li:nth-of-type(2) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0546

English:

```text
Payments
```

Japanese:

```text
支払い
```

### ITEM 0407

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(2) > ul.footer__links:nth-of-type(1) > li:nth-of-type(3) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0547

English:

```text
Maps
```

Japanese:

```text
地図
```

### ITEM 0408

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(2) > ul.footer__links:nth-of-type(1) > li:nth-of-type(4) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0548

English:

```text
Apps
```

Japanese:

```text
アプリ
```

### ITEM 0409

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0549

English:

```text
© 2026 Korea Inside · Republic of Korea
```

Japanese:

```text
© 2026 Korea Inside · 大韓民国
```

### ITEM 0410

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p.footer__business:nth-of-type(2) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0552

English:

```text
getkoreainside@gmail.com
```

Japanese:

```text
getkoreainside@gmail.com
```

### ITEM 0411

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p.footer__business:nth-of-type(2) > a.footer__legal-link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0550

English:

```text
Affiliate Disclosure
```

Japanese:

```text
アフィリエイト開示
```

### ITEM 0412

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p.footer__business:nth-of-type(2) > a.footer__legal-link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0551

English:

```text
Privacy Policy
```

Japanese:

```text
プライバシーポリシー
```

### ITEM 0413

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p.footer__business:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0553

English:

```text
Business Registration No. 462-39-01721
```

Japanese:

```text
事業者登録番号 462-39-01721
```

### ITEM 0414

- Element/type: common UI footer direct text
- Source target: `arrival.html|html:nth-of-type(1) > body.arrival-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p.footer__business:nth-of-type(2)::text[2]`
- Reuse: Japanese Golden Sample Approved ITEM 0554

English:

```text
Contact:
```

Japanese:

```text
お問い合わせ：
```

## QA Ledger

- Source ITEM count: **204**
- Localized ITEM count: **204**
- ITEM range: **0211–0414**
- Common UI reuse: **83 / 83**
- Arrival-specific localization: **121 / 121**
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
- no factual or operational-condition change
- Codex exact implementation only