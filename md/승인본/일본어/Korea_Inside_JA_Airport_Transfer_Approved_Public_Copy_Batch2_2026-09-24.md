# Korea Inside — Japanese Localization Batch 2 — Airport Transfer

**Date:** 2026-09-24  
**Status:** APPROVED PUBLIC COPY — CONTENT LOCKED  
**Source MD:** `Korea_Inside_JA_Airport5_Localization_Source_Batch2_2026-09-24.md`  
**Target:** `ja/airport-transfer.html`  

## Scope

- `airport-transfer.html` Source ITEMs: **265 / 265**
- Japanese Golden Sample common UI reused exactly: **83 / 83**
- Airport Transfer page-specific Japanese editorial localization: **182 / 182**
- English source SHA-256: `43353916F65304D6FE92636E963B2F48AB2C011ACD33EC418B2D244D4431286F`
- `ja/airport-transfer.html` matched the English source before localization.

## Japanese editorial-localization rules

- This is Japanese editorial localization, not literal translation.
- Facts, fares, dates, times, terminal details, luggage rules, service conditions and recommendation logic remain unchanged in meaning.
- `AREX`, `T-money`, `Airport Limousine`, `K Airport Limousine`, `International Taxi`, `Korea Inside` and other established product/brand names remain recognizable.
- Korean place names use natural Japanese travel forms such as `Seoul Station → ソウル駅`, `Hongdae → 弘大`, `Gongdeok → 孔徳`, `Myeongdong → 明洞`, `Hongik University → 弘大入口駅`.
- Airport `Call Van` is localized as `コールバン`; broader pre-booked `private transfer` is localized as `貸切送迎` so the source distinction is preserved.
- Common UI is inherited exactly from the CONTENT LOCKED Japanese Golden Sample and is not retranslated.
- JSON-LD FAQ and visible FAQ use the same approved Japanese wording in this draft.

## ITEMs

### ITEM 0415

- Element/type: meta description
- Source target: `airport-transfer.html|html:nth-of-type(1) > head:nth-of-type(1) > meta:nth-of-type(3)@content`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Compare AREX, airport buses, taxis, Call Van and private transfers from Incheon Airport to Seoul, with current fares, luggage and late-night guidance.
```

Japanese:

```text
仁川空港からソウルまでのAREX、空港バス、タクシー、コールバン、貸切送迎を比較。現在の運賃、荷物、深夜到着時の選び方までまとめています。
```

### ITEM 0416

- Element/type: title
- Source target: `airport-transfer.html|html:nth-of-type(1) > head:nth-of-type(1) > title:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Incheon Airport to Seoul: AREX, Bus, Taxi & Transfer | Korea Inside
```

Japanese:

```text
仁川空港からソウル：AREX・バス・タクシー・送迎比較 | Korea Inside
```

### ITEM 0417

- Element/type: JSON-LD FAQ question
- Source target: `airport-transfer.html|json-ld::mainEntity[1].name`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Is AREX or the airport bus better?
```

Japanese:

```text
AREXと空港バス、どちらが便利？
```

### ITEM 0418

- Element/type: JSON-LD FAQ answer
- Source target: `airport-transfer.html|json-ld::mainEntity[1].acceptedAnswer.text`
- Reuse: NEW airport-transfer-specific localization

English:

```text
AREX is more predictable and works especially well for Seoul Station or Hongdae. The airport bus can be easier when its actual stop is close to the hotel and removes another station transfer. The final walk matters as much as the headline travel time.
```

Japanese:

```text
AREXは所要時間を読みやすく、特にソウル駅や弘大へ向かうときに使いやすい選択肢です。空港バスは、実際の停留所がホテルの近くにあり、駅での乗り換えを1回減らせるなら楽になることがあります。表示上の所要時間と同じくらい、最後にどれだけ歩くかが重要です。
```

### ITEM 0419

- Element/type: JSON-LD FAQ question
- Source target: `airport-transfer.html|json-ld::mainEntity[2].name`
- Reuse: NEW airport-transfer-specific localization

English:

```text
What is the cheapest way from Incheon Airport to Seoul?
```

Japanese:

```text
仁川空港からソウルまで一番安い方法は？
```

### ITEM 0420

- Element/type: JSON-LD FAQ answer
- Source target: `airport-transfer.html|json-ld::mainEntity[2].acceptedAnswer.text`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The AREX All-stop Train is usually the cheapest rail option. Current adult transit-card fares are ₩4,650 from T1 and ₩5,250 from T2 to Hongik University, and ₩4,750 from T1 and ₩5,350 from T2 to Seoul Station.
```

Japanese:

```text
鉄道では、AREX各駅停車が通常もっとも安い選択肢です。現在の大人交通カード運賃は、弘大入口駅まで第1ターミナルから₩4,650、第2ターミナルから₩5,250、ソウル駅まで第1ターミナルから₩4,750、第2ターミナルから₩5,350です。
```

### ITEM 0421

- Element/type: JSON-LD FAQ question
- Source target: `airport-transfer.html|json-ld::mainEntity[3].name`
- Reuse: NEW airport-transfer-specific localization

English:

```text
What is the fastest way to Seoul Station?
```

Japanese:

```text
ソウル駅まで一番速い方法は？
```

### ITEM 0422

- Element/type: JSON-LD FAQ answer
- Source target: `airport-transfer.html|json-ld::mainEntity[3].acceptedAnswer.text`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The AREX Express has published airport-to-Seoul Station journey times of 43 minutes from T1 and 51 minutes from T2. The full trip still includes the walk to the airport station and whatever comes after Seoul Station.
```

Japanese:

```text
AREX直通列車の公表所要時間は、空港からソウル駅まで第1ターミナルから43分、第2ターミナルから51分です。ただし実際の全行程には、到着ロビーから空港駅までの徒歩と、ソウル駅に着いた後の移動も含まれます。
```

### ITEM 0423

- Element/type: JSON-LD FAQ question
- Source target: `airport-transfer.html|json-ld::mainEntity[4].name`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Does the AREX Express stop at Hongdae?
```

Japanese:

```text
AREX直通列車は弘大に停まる？
```

### ITEM 0424

- Element/type: JSON-LD FAQ answer
- Source target: `airport-transfer.html|json-ld::mainEntity[4].acceptedAnswer.text`
- Reuse: NEW airport-transfer-specific localization

English:

```text
No. The AREX Express runs directly to Seoul Station and does not stop at Hongik University. The All-stop Train stops at Hongik University and is usually the more direct rail option for Hongdae.
```

Japanese:

```text
停まりません。AREX直通列車はソウル駅まで直通で、弘大入口駅には停車しません。各駅停車は弘大入口駅に停まるため、弘大へは通常こちらのほうが直接的です。
```

### ITEM 0425

- Element/type: JSON-LD FAQ question
- Source target: `airport-transfer.html|json-ld::mainEntity[5].name`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Can I use T-money on the AREX?
```

Japanese:

```text
AREXでT-moneyは使える？
```

### ITEM 0426

- Element/type: JSON-LD FAQ answer
- Source target: `airport-transfer.html|json-ld::mainEntity[5].acceptedAnswer.text`
- Reuse: NEW airport-transfer-specific localization

English:

```text
T-money can be used on the AREX All-stop Train. The AREX Express uses a separate ticket and reserved-seat system.
```

Japanese:

```text
AREX各駅停車ではT-moneyを利用できます。AREX直通列車は別の乗車券と指定席の仕組みです。
```

### ITEM 0427

- Element/type: JSON-LD FAQ question
- Source target: `airport-transfer.html|json-ld::mainEntity[6].name`
- Reuse: NEW airport-transfer-specific localization

English:

```text
What works better with several large suitcases?
```

Japanese:

```text
大きなスーツケースが複数ある場合はどれが便利？
```

### ITEM 0428

- Element/type: JSON-LD FAQ answer
- Source target: `airport-transfer.html|json-ld::mainEntity[6].acceptedAnswer.text`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The answer changes with the number and size of the bags. An airport bus can work well when its stop is close to the hotel, while a larger taxi, Call Van or pre-booked transfer becomes more useful when moving the luggage through a station would be difficult. Passenger capacity and luggage capacity are separate limits.
```

Japanese:

```text
荷物の数と大きさで変わります。空港バスは停留所がホテルの近くなら便利です。一方、駅構内で荷物を運ぶのが難しい量なら、大型タクシー、コールバン、事前予約の送迎が使いやすくなります。乗車できる人数と積める荷物量は別の制限です。
```

### ITEM 0429

- Element/type: JSON-LD FAQ question
- Source target: `airport-transfer.html|json-ld::mainEntity[7].name`
- Reuse: NEW airport-transfer-specific localization

English:

```text
What can I use after midnight?
```

Japanese:

```text
深夜0時を過ぎたら何が使える？
```

### ITEM 0430

- Element/type: JSON-LD FAQ answer
- Source target: `airport-transfer.html|json-ld::mainEntity[7].acceptedAnswer.text`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Incheon Airport currently lists overnight services including N6000, N6002, N6701 and N6703, with different departure times at T1 and T2. Official taxis and pre-booked vehicles are alternatives when the useful train or night-bus window has closed.
```

Japanese:

```text
仁川空港は現在、N6000、N6002、N6701、N6703などの深夜便を案内しており、第1ターミナルと第2ターミナルで出発時刻が異なります。利用できる列車や深夜バスの時間を過ぎた場合は、公式タクシーや事前予約車両が代替手段になります。
```

### ITEM 0431

- Element/type: JSON-LD FAQ question
- Source target: `airport-transfer.html|json-ld::mainEntity[8].name`
- Reuse: NEW airport-transfer-specific localization

English:

```text
How do taxis from Incheon Airport work?
```

Japanese:

```text
仁川空港からのタクシーはどう利用する？
```

### ITEM 0432

- Element/type: JSON-LD FAQ answer
- Source target: `airport-transfer.html|json-ld::mainEntity[8].acceptedAnswer.text`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Official airport taxi stands serve regular, Deluxe, Jumbo and International Taxi services. A regular Seoul taxi currently starts at ₩4,800 for 1.6 km, while Deluxe, SUV and Jumbo services start at ₩7,000 for 3 km. The final fare depends on the service, destination, traffic and time of travel.
```

Japanese:

```text
空港公式タクシー乗り場から、一般、Deluxe、Jumbo、International Taxiを利用できます。現在、ソウルの一般タクシーは1.6 kmまで₩4,800から、Deluxe、SUV、Jumboは3 kmまで₩7,000からです。最終料金はサービス、目的地、交通状況、利用時間によって変わります。
```

### ITEM 0433

- Element/type: JSON-LD FAQ question
- Source target: `airport-transfer.html|json-ld::mainEntity[9].name`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Does Terminal 1 or Terminal 2 change the route?
```

Japanese:

```text
第1ターミナルと第2ターミナルで移動方法は変わる？
```

### ITEM 0434

- Element/type: JSON-LD FAQ answer
- Source target: `airport-transfer.html|json-ld::mainEntity[9].acceptedAnswer.text`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The main transport choices are similar, but the ticketing areas, rail levels, taxi stands and Call Van locations are different. T1 and T2 also use different departure times for some late-night services.
```

Japanese:

```text
主な移動手段はほぼ同じですが、乗車券売り場、鉄道フロア、タクシー乗り場、コールバンの場所は異なります。一部の深夜便は、第1ターミナルと第2ターミナルで出発時刻も異なります。
```

### ITEM 0435

- Element/type: JSON-LD FAQ question
- Source target: `airport-transfer.html|json-ld::mainEntity[10].name`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Is Call Van the same as a private transfer?
```

Japanese:

```text
コールバンと貸切送迎は同じもの？
```

### ITEM 0436

- Element/type: JSON-LD FAQ answer
- Source target: `airport-transfer.html|json-ld::mainEntity[10].acceptedAnswer.text`
- Reuse: NEW airport-transfer-specific localization

English:

```text
No. Incheon Airport's Call Van is a Korean commercial van service with its own airport conditions. “Private transfer” is a broader booking term for a vehicle reserved for one party. Some private-transfer bookings may use vans, but the terms are not interchangeable.
```

Japanese:

```text
同じではありません。仁川空港のコールバンは、空港独自の利用条件がある韓国の商用バンサービスです。「貸切送迎」は、1組の利用者が車両を予約するサービス全般を指す、より広い予約用語です。貸切送迎でバン車両を使う場合もありますが、同じ意味ではありません。
```

### ITEM 0437

- Element/type: literal ARIA label
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > a.logo:nth-of-type(1)@aria-label`
- Reuse: Japanese Golden Sample Approved ITEM 0003

English:

```text
Korea Inside home
```

Japanese:

```text
Korea Inside ホーム
```

### ITEM 0438

- Element/type: image alt
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > a.logo:nth-of-type(1) > img.site-brand__logo:nth-of-type(1)@alt`
- Reuse: Japanese Golden Sample Approved ITEM 0004

English:

```text
Korea Inside
```

Japanese:

```text
Korea Inside
```

### ITEM 0439

- Element/type: literal ARIA label
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > button#site-nav-toggle@aria-label`
- Reuse: Japanese Golden Sample Approved ITEM 0005

English:

```text
Open menu
```

Japanese:

```text
メニューを開く
```

### ITEM 0440

- Element/type: literal ARIA label
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation@aria-label`
- Reuse: Japanese Golden Sample Approved ITEM 0006

English:

```text
Primary navigation
```

Japanese:

```text
メインナビゲーション
```

### ITEM 0441

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(1) > button#site-nav-trigger-discover::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0007

English:

```text
DISCOVER
```

Japanese:

```text
楽しむ
```

### ITEM 0442

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(1) > div#site-nav-panel-discover > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0008

English:

```text
Taste Korea
```

Japanese:

```text
Taste Korea
```

### ITEM 0443

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(1) > div#site-nav-panel-discover > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0009

English:

```text
K-Beauty
```

Japanese:

```text
K-Beauty
```

### ITEM 0444

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > button#site-nav-trigger-travel::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0010

English:

```text
Travel
```

Japanese:

```text
旅行ガイド
```

### ITEM 0445

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0011

English:

```text
Hongdae
```

Japanese:

```text
弘大
```

### ITEM 0446

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0012

English:

```text
Myeongdong
```

Japanese:

```text
明洞
```

### ITEM 0447

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0013

English:

```text
Seongsu
```

Japanese:

```text
聖水
```

### ITEM 0448

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(4)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0014

English:

```text
Insadong
```

Japanese:

```text
仁寺洞
```

### ITEM 0449

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(5)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0015

English:

```text
Gangnam
```

Japanese:

```text
江南
```

### ITEM 0450

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(6)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0016

English:

```text
Jamsil
```

Japanese:

```text
蚕室
```

### ITEM 0451

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(7)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0017

English:

```text
Gongdeok & Mapo
```

Japanese:

```text
孔徳・麻浦
```

### ITEM 0452

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(8)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0018

English:

```text
Itaewon
```

Japanese:

```text
梨泰院
```

### ITEM 0453

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > div.site-nav__travel-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(9)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0019

English:

```text
Dongdaemun
```

Japanese:

```text
東大門
```

### ITEM 0454

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(1) > p.site-nav__group-label:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0020

English:

```text
Seoul Areas
```

Japanese:

```text
ソウルのエリア
```

### ITEM 0455

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(2) > div.site-nav__travel-attraction-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0021

English:

```text
Lotte World
```

Japanese:

```text
ロッテワールド
```

### ITEM 0456

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(2) > div.site-nav__travel-attraction-grid:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0022

English:

```text
Seoul Sky
```

Japanese:

```text
ソウルスカイ
```

### ITEM 0457

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > div.site-nav__group:nth-of-type(2) > p.site-nav__group-label:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0023

English:

```text
Attractions
```

Japanese:

```text
観光スポット
```

### ITEM 0458

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(2) > div#site-nav-panel-travel > p#site-nav-travel-guides-label::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0024

English:

```text
Travel Guides
```

Japanese:

```text
旅行ガイド
```

### ITEM 0459

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > button#site-nav-trigger-stay::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0025

English:

```text
Stay
```

Japanese:

```text
宿泊
```

### ITEM 0460

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0026

English:

```text
Stay Guide
```

Japanese:

```text
宿泊ガイド
```

### ITEM 0461

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(10)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0027

English:

```text
Luxury Hotels
```

Japanese:

```text
高級ホテル
```

### ITEM 0462

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0028

English:

```text
Hongdae vs Myeongdong
```

Japanese:

```text
弘大 vs 明洞
```

### ITEM 0463

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0029

English:

```text
First-Time Visitors
```

Japanese:

```text
初めてのソウル
```

### ITEM 0464

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(4)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0030

English:

```text
Families
```

Japanese:

```text
家族旅行
```

### ITEM 0465

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(5)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0031

English:

```text
Solo Travelers
```

Japanese:

```text
ひとり旅
```

### ITEM 0466

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(6)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0032

English:

```text
Couples
```

Japanese:

```text
カップル
```

### ITEM 0467

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(7)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0033

English:

```text
Budget Travelers
```

Japanese:

```text
節約派
```

### ITEM 0468

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(8)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0034

English:

```text
Shopping
```

Japanese:

```text
ショッピング
```

### ITEM 0469

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(9)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0035

English:

```text
Nightlife
```

Japanese:

```text
ナイトライフ
```

### ITEM 0470

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(3) > div#site-nav-panel-stay > div.site-nav__group:nth-of-type(1) > p.site-nav__group-label:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0036

English:

```text
Stay Guide
```

Japanese:

```text
宿泊ガイド
```

### ITEM 0471

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(4) > button#site-nav-trigger-esim::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0037

English:

```text
eSIM
```

Japanese:

```text
eSIM
```

### ITEM 0472

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(4) > div#site-nav-panel-esim > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0038

English:

```text
eSIM Guide
```

Japanese:

```text
eSIMガイド
```

### ITEM 0473

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(4) > div#site-nav-panel-esim > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0039

English:

```text
Best eSIM for Korea
```

Japanese:

```text
韓国旅行におすすめのeSIM
```

### ITEM 0474

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(4) > div#site-nav-panel-esim > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0040

English:

```text
Korea eSIM with a Phone Number
```

Japanese:

```text
電話番号付き韓国eSIM
```

### ITEM 0475

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > button#site-nav-trigger-airport::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0041

English:

```text
Airport
```

Japanese:

```text
空港
```

### ITEM 0476

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > div#site-nav-panel-airport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0042

English:

```text
Airport Guide
```

Japanese:

```text
空港ガイド
```

### ITEM 0477

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > div#site-nav-panel-airport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0043

English:

```text
Arrival Guide
```

Japanese:

```text
到着ガイド
```

### ITEM 0478

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > div#site-nav-panel-airport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0044

English:

```text
Airport Transfer
```

Japanese:

```text
空港送迎
```

### ITEM 0479

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > div#site-nav-panel-airport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(4)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0045

English:

```text
AREX Guide
```

Japanese:

```text
AREXガイド
```

### ITEM 0480

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(5) > div#site-nav-panel-airport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(5)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0046

English:

```text
Airport Bus Guide
```

Japanese:

```text
空港バスガイド
```

### ITEM 0481

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(6) > button#site-nav-trigger-maps::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0047

English:

```text
Maps
```

Japanese:

```text
地図
```

### ITEM 0482

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(6) > div#site-nav-panel-maps > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0048

English:

```text
Maps Guide
```

Japanese:

```text
地図アプリガイド
```

### ITEM 0483

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > button#site-nav-trigger-transport::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0049

English:

```text
Transport
```

Japanese:

```text
交通
```

### ITEM 0484

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0050

English:

```text
T-money Guide
```

Japanese:

```text
T-moneyガイド
```

### ITEM 0485

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0051

English:

```text
WOWPASS Guide
```

Japanese:

```text
WOWPASSガイド
```

### ITEM 0486

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0052

English:

```text
T-money vs WOWPASS
```

Japanese:

```text
T-money vs WOWPASS
```

### ITEM 0487

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(1) > p.site-nav__group-label:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0053

English:

```text
Travel Cards
```

Japanese:

```text
交通カード
```

### ITEM 0488

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(2) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0054

English:

```text
Taxi Guide
```

Japanese:

```text
タクシーガイド
```

### ITEM 0489

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(2) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0055

English:

```text
Call Van / Private Transfer
```

Japanese:

```text
コールバン／貸切送迎
```

### ITEM 0490

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(2) > a.site-nav__link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0056

English:

```text
Rental Car
```

Japanese:

```text
レンタカー
```

### ITEM 0491

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(7) > div#site-nav-panel-transport > div.site-nav__group:nth-of-type(2) > p.site-nav__group-label:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0057

English:

```text
Other Transport
```

Japanese:

```text
その他の交通
```

### ITEM 0492

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(8) > button#site-nav-trigger-apps::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0058

English:

```text
Apps
```

Japanese:

```text
アプリ
```

### ITEM 0493

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(8) > div#site-nav-panel-apps > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0059

English:

```text
Essential Apps
```

Japanese:

```text
必須アプリ
```

### ITEM 0494

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(9) > button#site-nav-trigger-travel-tips::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0060

English:

```text
Travel Tips
```

Japanese:

```text
旅行準備
```

### ITEM 0495

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(9) > div#site-nav-panel-travel-tips > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0061

English:

```text
Korea Travel Checklist
```

Japanese:

```text
韓国旅行チェックリスト
```

### ITEM 0496

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > nav#site-primary-navigation > ul.site-nav__list:nth-of-type(1) > li.site-nav__item:nth-of-type(9) > div#site-nav-panel-travel-tips > div.site-nav__group:nth-of-type(1) > a.site-nav__link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0062

English:

```text
Paying in Korea
```

Japanese:

```text
韓国での支払い
```

### ITEM 0497

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > div.language-switcher:nth-of-type(1) > button.language-switcher__button:nth-of-type(1) > span.language-switcher__current:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0063

English:

```text
EN
```

Japanese:

```text
JA
```

### ITEM 0498

- Element/type: common UI header/navigation direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > div.language-switcher:nth-of-type(1) > button.language-switcher__button:nth-of-type(1) > span.language-switcher__label:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0064

English:

```text
Language
```

Japanese:

```text
日本語
```

### ITEM 0499

- Element/type: literal ARIA label
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > header.header:nth-of-type(1) > div.container:nth-of-type(1) > div.language-switcher:nth-of-type(1)@aria-label`
- Reuse: Japanese Golden Sample Approved ITEM 0065

English:

```text
Language selector
```

Japanese:

```text
言語選択
```

### ITEM 0500

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-page-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.airport-page-hero__copy:nth-of-type(1) > p.page-hero__breadcrumb:nth-of-type(1) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Home
```

Japanese:

```text
ホーム
```

### ITEM 0501

- Element/type: breadcrumb direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-page-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.airport-page-hero__copy:nth-of-type(1) > p.page-hero__breadcrumb:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
/ Airport Transfer
```

Japanese:

```text
/ 空港送迎
```

### ITEM 0502

- Element/type: H1
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-page-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.airport-page-hero__copy:nth-of-type(1) > h1.airport-page-hero__title:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Incheon Airport to Seoul:
```

Japanese:

```text
仁川空港からソウルへ：
```

### ITEM 0503

- Element/type: H1
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-page-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.airport-page-hero__copy:nth-of-type(1) > h1.airport-page-hero__title:nth-of-type(1)::text[2]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Which Transfer Is Best?
```

Japanese:

```text
どの移動手段が合う？
```

### ITEM 0504

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-page-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.airport-page-hero__copy:nth-of-type(1) > p.transfer-review-date:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Official fares and operating details checked August 18, 2026.
```

Japanese:

```text
公式運賃・運行情報は2026年8月18日に確認。
```

### ITEM 0505

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-page-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.airport-page-hero__copy:nth-of-type(1) > div.transfer-hero-answer:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Where you are staying matters more than the headline travel time. The AREX Express is a neat fit for Seoul Station, while the All-stop Train reaches Hongik University for Hongdae. An airport limousine bus can be easier when its stop is close to the hotel.
```

Japanese:

```text
表示上の所要時間より、どこに泊まるかのほうが重要です。AREX直通列車はソウル駅へ向かう旅程に合いやすく、各駅停車は弘大の弘大入口駅に停まります。空港リムジンバスは、停留所がホテルの近くにあるなら、より楽なことがあります。
```

### ITEM 0506

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section.airport-page-hero:nth-of-type(1) > div.container:nth-of-type(1) > div.airport-page-hero__copy:nth-of-type(1) > div.transfer-hero-answer:nth-of-type(1) > p:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
With a family, several large suitcases or a late arrival, the balance shifts. A taxi, Call Van or pre-booked private transfer costs more, but it can remove the station transfer, stairs and final walk that make an airport journey feel much longer than it looks on a timetable.
```

Japanese:

```text
家族旅行、大きなスーツケースが複数ある場合、または到着が遅い場合は判断が変わります。タクシー、コールバン、事前予約の貸切送迎は料金が高くなりますが、駅での乗り換え、階段、最後の徒歩移動をなくせるため、時刻表で見るより長く感じる空港移動を楽にできることがあります。
```

### ITEM 0507

- Element/type: H2
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#whole-trip > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h2:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The Whole Trip Matters
```

Japanese:

```text
大切なのは宿泊先までの全行程
```

### ITEM 0508

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#whole-trip > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
A 43-minute rail ride ends at Seoul Station, not at your hotel. If another subway transfer, stairs or a long walk follows, the fastest first leg may not be the easiest arrival.
```

Japanese:

```text
43分の鉄道移動が終わるのはソウル駅であって、ホテルではありません。その後に地下鉄の乗り換え、階段、長い徒歩移動があるなら、最初の区間が最速でも、到着全体では一番楽とは限りません。
```

### ITEM 0509

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#whole-trip > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The same route can also feel very different with luggage. One suitcase is usually manageable. Several large bags, a stroller or luggage for an entire family can turn a simple station transfer into the hardest part of the trip.
```

Japanese:

```text
同じルートでも、荷物量によって負担は大きく変わります。スーツケース1個なら通常対応しやすくても、大型の荷物が複数、ベビーカー、家族全員分の荷物があると、簡単に見える駅での乗り換えが旅の中で一番大変になることがあります。
```

### ITEM 0510

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#whole-trip > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
That is why the useful comparison runs all the way to the hotel door: where the train or bus leaves you, how much luggage is moving with you, how many people are sharing the trip and what time you are actually ready to leave the airport.
```

Japanese:

```text
だから比較すべきなのはホテルの入口までです。電車やバスを降りる場所、運ぶ荷物の量、同行人数、実際に空港を出られる時刻まで含めて考えてください。
```

### ITEM 0511

- Element/type: H2
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > header.transfer-chapter__header:nth-of-type(1) > h2#transfer-comparison-heading::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Incheon Airport to Seoul at a Glance
```

Japanese:

```text
仁川空港からソウルまでを一覧比較
```

### ITEM 0512

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > header.transfer-chapter__header:nth-of-type(1) > p.transfer-chapter__answer:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
These are the current published fares and the practical differences that matter most. Bus fares vary by route and operator, while taxi and private-transfer totals depend on the destination, vehicle and travel conditions.
```

Japanese:

```text
以下は現在公表されている運賃と、実際の選択で重要になる違いです。バス運賃は路線と運行会社によって異なり、タクシーや貸切送迎の総額は目的地、車両、交通状況などで変わります。
```

### ITEM 0513

- Element/type: table header direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > thead:nth-of-type(1) > tr:nth-of-type(1) > th:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Option
```

Japanese:

```text
移動手段
```

### ITEM 0514

- Element/type: table header direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > thead:nth-of-type(1) > tr:nth-of-type(1) > th:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Current fare
```

Japanese:

```text
現在の運賃
```

### ITEM 0515

- Element/type: table header direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > thead:nth-of-type(1) > tr:nth-of-type(1) > th:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
What the trip is like
```

Japanese:

```text
移動の特徴
```

### ITEM 0516

- Element/type: table header direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > thead:nth-of-type(1) > tr:nth-of-type(1) > th:nth-of-type(4)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Main limitation
```

Japanese:

```text
主な注意点
```

### ITEM 0517

- Element/type: table header direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(1) > th:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
AREX Express
```

Japanese:

```text
AREX直通列車
```

### ITEM 0518

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(1) > td:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Adult ₩13,000
```

Japanese:

```text
大人 ₩13,000
```

### ITEM 0519

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(1) > td:nth-of-type(1)::text[2]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Child ₩9,500
```

Japanese:

```text
子ども ₩9,500
```

### ITEM 0520

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(1) > td:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Reserved-seat nonstop train to Seoul Station. Published journey time is 43 minutes from T1 and 51 minutes from T2.
```

Japanese:

```text
ソウル駅までノンストップの指定席列車。公表所要時間は第1ターミナルから43分、第2ターミナルから51分。
```

### ITEM 0521

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(1) > td:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
It does not stop at Hongik University, and the hotel may still require another transfer or walk.
```

Japanese:

```text
弘大入口駅には停まらず、ホテルまでさらに乗り換えや徒歩移動が必要な場合があります。
```

### ITEM 0522

- Element/type: table header direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(2) > th:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
AREX All-stop
```

Japanese:

```text
AREX各駅停車
```

### ITEM 0523

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(2) > td:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Seoul Station: ₩4,750 from T1 / ₩5,350 from T2
```

Japanese:

```text
ソウル駅：第1ターミナルから₩4,750／第2ターミナルから₩5,350
```

### ITEM 0524

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(2) > td:nth-of-type(1)::text[2]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Hongik University: ₩4,650 from T1 / ₩5,250 from T2
```

Japanese:

```text
弘大入口駅：第1ターミナルから₩4,650／第2ターミナルから₩5,250
```

### ITEM 0525

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(2) > td:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Direct rail service to Hongik University and Seoul Station, using a transit card or single-use ticket.
```

Japanese:

```text
交通カードまたは1回用乗車券で、弘大入口駅とソウル駅へ直接行ける鉄道。
```

### ITEM 0526

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(2) > td:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
It is slower, has no reserved seats and you keep your luggage with you.
```

Japanese:

```text
直通列車より時間がかかり、指定席ではなく、荷物は自分で管理します。
```

### ITEM 0527

- Element/type: table header direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(3) > th:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Airport limousine bus
```

Japanese:

```text
空港リムジンバス
```

### ITEM 0528

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(3) > td:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Major Seoul routes currently range from ₩16,000 to ₩18,000 for adults. Child fares on the examples checked are ₩12,000.
```

Japanese:

```text
ソウル主要路線の大人運賃は現在₩16,000～₩18,000。確認した例の子ども運賃は₩12,000。
```

### ITEM 0529

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(3) > td:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Stored luggage and fewer station transfers when the bus stop is close to the hotel.
```

Japanese:

```text
荷物をトランクに預けられ、停留所がホテルの近くなら駅での乗り換えを減らせます。
```

### ITEM 0530

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(3) > td:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Traffic varies, and the final walk from the actual stop can change the whole journey.
```

Japanese:

```text
道路状況で所要時間が変わり、実際の停留所から最後にどれだけ歩くかで移動全体の負担が変わります。
```

### ITEM 0531

- Element/type: table header direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(4) > th:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Regular / Deluxe-Jumbo taxi
```

Japanese:

```text
一般／Deluxe・Jumboタクシー
```

### ITEM 0532

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(4) > td:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Regular taxi starts at ₩4,800 for 1.6 km.
```

Japanese:

```text
一般タクシーは1.6 kmまで₩4,800から。
```

### ITEM 0533

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(4) > td:nth-of-type(1)::text[2]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Deluxe, SUV and Jumbo services start at ₩7,000 for 3 km.
```

Japanese:

```text
Deluxe、SUV、Jumboは3 kmまで₩7,000から。
```

### ITEM 0534

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(4) > td:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Door-to-door travel from the official airport taxi stand.
```

Japanese:

```text
空港公式タクシー乗り場から目的地までドア・ツー・ドアで移動。
```

### ITEM 0535

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(4) > td:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The total depends on destination, traffic, time and vehicle type. Passenger seats do not guarantee enough luggage space.
```

Japanese:

```text
総額は目的地、交通状況、利用時間、車種によって変わります。乗車定員内でも、荷物がすべて積めるとは限りません。
```

### ITEM 0536

- Element/type: table header direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(5) > th:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Official Airport Call Van
```

Japanese:

```text
仁川空港公式コールバン
```

### ITEM 0537

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(5) > td:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Distance-based fare; tolls are separate.
```

Japanese:

```text
距離制運賃。通行料は別途。
```

### ITEM 0538

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(5) > td:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
A commercial van service described by Incheon Airport for groups of five or fewer with 20 kg of luggage per person.
```

Japanese:

```text
仁川空港が、5人以下・1人あたり20 kgの荷物を条件として案内している商用バンサービス。
```

### ITEM 0539

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(5) > td:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Published operating hours are 08:00–21:00 and the airport's eligibility conditions apply.
```

Japanese:

```text
公表営業時間は08:00～21:00で、空港が定める利用条件が適用されます。
```

### ITEM 0540

- Element/type: table header direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(6) > th:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Pre-booked private transfer
```

Japanese:

```text
事前予約の貸切送迎
```

### ITEM 0541

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(6) > td:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Provider-specific.
```

Japanese:

```text
事業者・予約内容による。
```

### ITEM 0542

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(6) > td:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
A vehicle reserved in advance for one party, with a meeting arrangement set before arrival.
```

Japanese:

```text
1組の利用者向けに事前予約する車両で、到着前に待ち合わせ方法も決めておけます。
```

### ITEM 0543

- Element/type: table cell direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > div.table-scroll:nth-of-type(1) > table:nth-of-type(1) > tbody:nth-of-type(1) > tr:nth-of-type(6) > td:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Vehicle size, luggage, waiting time and cancellation terms depend on the individual booking.
```

Japanese:

```text
車両サイズ、荷物条件、待ち時間、キャンセル条件は予約ごとに異なります。
```

### ITEM 0544

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#transfer-comparison > div.container:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Fares and operating details were checked against the relevant transport operators and Incheon Airport on August 18, 2026. Routes, schedules and service conditions can change.
```

Japanese:

```text
運賃と運行情報は、2026年8月18日に各交通事業者と仁川空港の情報で確認しました。路線、時刻表、利用条件は変更されることがあります。
```

### ITEM 0545

- Element/type: H2
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#arex > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h2:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
AREX: Seoul Station and Hongdae Are Different Trips
```

Japanese:

```text
AREX：ソウル駅と弘大では選ぶ列車が違う
```

### ITEM 0546

- Element/type: H3
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#arex > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The Express is built around Seoul Station
```

Japanese:

```text
直通列車はソウル駅へ行く人向け
```

### ITEM 0547

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#arex > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The AREX Express makes the most sense when Seoul Station is genuinely useful for the rest of the journey. It runs nonstop from the airport with reserved seats. The current selling fare is ₩13,000 for adults and ₩9,500 for children, with published journey times of 43 minutes from T1 and 51 minutes from T2.
```

Japanese:

```text
AREX直通列車は、その後の移動にソウル駅が本当に便利な場合に最も使いやすい選択肢です。空港から指定席でノンストップ運行し、現在の販売運賃は大人₩13,000、子ども₩9,500。公表所要時間は第1ターミナルから43分、第2ターミナルから51分です。
```

### ITEM 0548

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#arex > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Current first departures are 05:16 from T2 and 05:24 from T1. The last departures are 22:40 from T2 and 22:48 from T1. Those times still have to work with immigration, baggage claim and the walk from the arrival hall to the airport station.
```

Japanese:

```text
現在の始発は第2ターミナル05:16、第1ターミナル05:24。最終は第2ターミナル22:40、第1ターミナル22:48です。ただし、入国審査、手荷物受取、到着ロビーから空港駅まで歩く時間も含めて間に合うかを考える必要があります。
```

### ITEM 0549

- Element/type: H3
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#arex > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h3:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Hongdae is different
```

Japanese:

```text
弘大へ行くなら別
```

### ITEM 0550

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#arex > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
For Hongdae, the All-stop Train is usually the more direct rail journey because it stops at Hongik University. Adult transit-card fares are currently ₩4,650 from T1 and ₩5,250 from T2 to Hongik University.
```

Japanese:

```text
弘大へは、弘大入口駅に停まる各駅停車のほうが通常直接的です。現在の大人交通カード運賃は、弘大入口駅まで第1ターミナルから₩4,650、第2ターミナルから₩5,250です。
```

### ITEM 0551

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#arex > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(4)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
To Seoul Station, the equivalent fares are ₩4,750 from T1 and ₩5,350 from T2. Published travel times are about 59 minutes from T1 and 66 minutes from T2, with some trains taking a few minutes longer.
```

Japanese:

```text
ソウル駅までの運賃は、第1ターミナルから₩4,750、第2ターミナルから₩5,350。公表所要時間は第1ターミナルから約59分、第2ターミナルから約66分で、列車によってはさらに数分かかります。
```

### ITEM 0552

- Element/type: H3
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#arex > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h3:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Luggage changes the experience
```

Japanese:

```text
荷物で快適さが変わる
```

### ITEM 0553

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#arex > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(5)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
AREX carriage rules allow no more than two items per passenger, each under 32 kg and under 158 cm in total dimensions. Even within those limits, several large suitcases can make the station transfer and final walk more tiring than the rail journey itself.
```

Japanese:

```text
AREXの手荷物規定では、1人2個まで、1個32 kg未満、3辺合計158 cm未満です。規定内でも、大型スーツケースが複数あると、鉄道そのものより駅での乗り換えや最後の徒歩移動のほうが負担になることがあります。
```

### ITEM 0554

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#arex > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(6) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
full AREX guide
```

Japanese:

```text
AREX完全ガイド
```

### ITEM 0555

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#arex > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(6) > a:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
T-money guide
```

Japanese:

```text
T-moneyガイド
```

### ITEM 0556

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#arex > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(6)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
For ticket details, station access and the difference between the two trains, see the
```

Japanese:

```text
乗車券の詳細、駅へのアクセス、2種類の列車の違いは
```

### ITEM 0557

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#arex > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(6)::text[2]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
. The
```

Japanese:

```text
で確認できます。また、
```

### ITEM 0558

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#arex > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(6)::text[3]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
explains the transit-card side of the All-stop Train.
```

Japanese:

```text
では、各駅停車での交通カード利用について説明しています。
```

### ITEM 0559

- Element/type: H2
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#airport-bus > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h2:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The Airport Bus Gets Easier When the Stop Is Close
```

Japanese:

```text
空港バスは、停留所が近いほど楽になる
```

### ITEM 0560

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#airport-bus > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
An airport limousine bus can remove a station transfer and give large bags a dedicated luggage compartment. The important part is not the neighborhood name on the route map but where the bus actually leaves you.
```

Japanese:

```text
空港リムジンバスは、駅での乗り換えをなくし、大型荷物を専用トランクに預けられます。重要なのは路線図に書かれたエリア名ではなく、実際にどこで降ろされるかです。
```

### ITEM 0561

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#airport-bus > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
A stop one block from the hotel can make the bus much easier than rail. A stop across a wide road, several uphill blocks away or on the wrong side of a complicated intersection can erase that advantage.
```

Japanese:

```text
ホテルから1ブロックの停留所なら、鉄道よりかなり楽になることがあります。一方、広い道路の反対側、上り坂を数ブロック歩く場所、複雑な交差点の反対側なら、その利点は小さくなります。
```

### ITEM 0562

- Element/type: H3
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#airport-bus > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Current fares vary by route
```

Japanese:

```text
現在の運賃は路線ごとに異なる
```

### ITEM 0563

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#airport-bus > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Airport Limousine routes 6001, 6002 and 6015 are currently ₩17,000 for adults and ₩12,000 for children. Route 6003 is ₩16,000 for adults and ₩12,000 for children. K Airport Limousine's central-Seoul routes are currently ₩18,000 for adults and ₩12,000 for children.
```

Japanese:

```text
Airport Limousineの6001、6002、6015は現在、大人₩17,000・子ども₩12,000。6003は大人₩16,000・子ども₩12,000。K Airport Limousineのソウル中心部路線は現在、大人₩18,000・子ども₩12,000です。
```

### ITEM 0564

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#airport-bus > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(4)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
These are current examples rather than a universal Seoul airport-bus fare.
```

Japanese:

```text
これは現在確認できる例であり、ソウル行き空港バスすべてに共通する運賃ではありません。
```

### ITEM 0565

- Element/type: H3
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#airport-bus > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h3:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Luggage rules are not identical
```

Japanese:

```text
荷物ルールは運行会社で同じではない
```

### ITEM 0566

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#airport-bus > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(5)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Airport Limousine currently describes free baggage as two pieces up to 28 inches and 20 kg each, or one piece larger than 28 inches.
```

Japanese:

```text
Airport Limousineは現在、無料手荷物を「28インチ以下・1個20 kg以下なら2個まで、または28インチ超なら1個」と案内しています。
```

### ITEM 0567

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#airport-bus > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(6)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
K Airport Limousine's English FAQ and transport terms are not completely consistent with each other, so unusually large or heavy luggage is better treated as an operator-specific condition rather than a single rule for every airport bus.
```

Japanese:

```text
K Airport Limousineは英語FAQと運送約款の記載が完全には一致していないため、特に大きい・重い荷物は、すべての空港バスに共通するルールではなく、運行会社ごとの条件として確認するほうが安全です。
```

### ITEM 0568

- Element/type: H3
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#airport-bus > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h3:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Terminal 2 has a different ticketing routine
```

Japanese:

```text
第2ターミナルは乗車券の買い方が異なる
```

### ITEM 0569

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#airport-bus > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(7)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
At T1, airport-bus ticketing is on the arrivals level. At T2, ticketing and boarding are concentrated in Transportation Center B1.
```

Japanese:

```text
第1ターミナルでは、空港バスの乗車券売り場は到着階にあります。第2ターミナルでは、乗車券購入と乗車場所が交通センターB1に集まっています。
```

### ITEM 0570

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#airport-bus > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(8)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Since March 5, 2026, Seoul-bound Airport Limousine services from T2 require a ticket from the staffed counter or ticket machine before boarding.
```

Japanese:

```text
2026年3月5日以降、第2ターミナル発のソウル行きAirport Limousineは、乗車前に有人カウンターまたは券売機で乗車券を購入する必要があります。
```

### ITEM 0571

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#airport-bus > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(9)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Traffic remains the main trade-off. A bus can be the easier door-to-door journey and still take longer when the roads are congested.
```

Japanese:

```text
一番のデメリットは渋滞です。ドア・ツー・ドアではバスのほうが楽でも、道路が混んでいれば所要時間は長くなることがあります。
```

### ITEM 0572

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#airport-bus > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(10) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
airport bus guide
```

Japanese:

```text
空港バスガイド
```

### ITEM 0573

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#airport-bus > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(10)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The
```

Japanese:

```text

詳しくは```

### ITEM 0574

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#airport-bus > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(10)::text[2]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
covers routes and terminal stops in more detail.
```

Japanese:

```text
では、路線と各ターミナルの停留所をさらに詳しく説明しています。
```

### ITEM 0575

- Element/type: H2
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#taxi > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h2:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Taxi: Door to Door, Until Luggage Changes the Vehicle
```

Japanese:

```text
タクシー：ドア・ツー・ドア。ただし荷物量で車種が変わる
```

### ITEM 0576

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#taxi > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
A taxi removes the station transfer completely. At the official airport stands, a regular Seoul taxi currently starts at ₩4,800 for the first 1.6 km. Deluxe, SUV and Jumbo services start at ₩7,000 for the first 3 km.
```

Japanese:

```text
タクシーなら駅での乗り換えはありません。空港公式乗り場では、ソウルの一般タクシーが現在、最初の1.6 kmまで₩4,800から。Deluxe、SUV、Jumboは最初の3 kmまで₩7,000からです。
```

### ITEM 0577

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#taxi > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Regular-taxi late-night surcharges are 20% from 22:00–23:00, 40% from 23:00–02:00 and 20% from 02:00–04:00. Deluxe, SUV and van services use a 20% late-night surcharge from 22:00–04:00.
```

Japanese:

```text
一般タクシーの深夜割増は22:00～23:00が20%、23:00～02:00が40%、02:00～04:00が20%。Deluxe、SUV、バンサービスは22:00～04:00に20%の深夜割増が適用されます。
```

### ITEM 0578

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#taxi > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Passenger count is only half of the space question. Four people may fit in a car while four large suitcases do not. The larger taxi categories become more useful when luggage, rather than seats, is the limiting factor.
```

Japanese:

```text
座席数だけでは車両サイズを決められません。4人が乗れても、大型スーツケース4個が積めるとは限りません。座席数より荷物容量が問題になる場合は、大型タクシーのカテゴリーが使いやすくなります。
```

### ITEM 0579

- Element/type: H3
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#taxi > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Official taxi stands
```

Japanese:

```text
公式タクシー乗り場
```

### ITEM 0580

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#taxi > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(4)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
At T1, regular Seoul taxis use 5C, 6C and 6D. Deluxe and Jumbo taxis use 7C and 8C, while International Taxi uses 4C.
```

Japanese:

```text
第1ターミナルでは、ソウル一般タクシーが5C・6C・6D、DeluxeとJumboが7C・8C、International Taxiが4Cを利用します。
```

### ITEM 0581

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#taxi > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(5)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
At T2, regular Seoul taxis use 7C, Deluxe and Jumbo taxis use 7D, and International Taxi uses 3C.
```

Japanese:

```text
第2ターミナルでは、ソウル一般タクシーが7C、DeluxeとJumboが7D、International Taxiが3Cを利用します。
```

### ITEM 0582

- Element/type: H3
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#taxi > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h3:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
International Taxi
```

Japanese:

```text
International Taxi
```

### ITEM 0583

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#taxi > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(6)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
International Taxi uses Seoul zone fares rather than the ordinary meter structure for its airport service. Current published ranges are ₩70,000–95,000 for sedans and ₩100,000–140,000 for larger vehicles, depending on the destination zone.
```

Japanese:

```text
International Taxiの空港サービスは通常のメーター制ではなく、ソウルのゾーン別運賃を使います。現在の公表範囲は、目的地ゾーンによりセダンが₩70,000～95,000、大型車両が₩100,000～140,000です。
```

### ITEM 0584

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#taxi > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(7)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Official information is not completely consistent about how tolls are described across every International Taxi service, so the terms attached to the actual reservation are the safer reference than applying one toll rule to every booking.
```

Japanese:

```text
公式情報では、International Taxiのすべてのサービスについて通行料の扱いが完全に統一して説明されているわけではありません。すべての予約に同じ通行料ルールを当てはめるより、実際の予約条件を確認するほうが安全です。
```

### ITEM 0585

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#taxi > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(8) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
taxi guide
```

Japanese:

```text
タクシーガイド
```

### ITEM 0586

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#taxi > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(8)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The
```

Japanese:

```text

詳しくは```

### ITEM 0587

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#taxi > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(8)::text[2]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
explains Seoul taxi types, payment and fare structure in more detail.
```

Japanese:

```text
では、ソウルのタクシー種類、支払い、運賃体系をさらに詳しく説明しています。
```

### ITEM 0588

- Element/type: H2
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#call-van-private-transfer > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h2:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Call Van and Pre-booked Private Transfers
```

Japanese:

```text
コールバンと事前予約の貸切送迎
```

### ITEM 0589

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#call-van-private-transfer > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Korea's airport Call Van is a commercial van service. It is not the universal English name for every private transfer sold online.
```

Japanese:

```text
韓国の空港コールバンは商用バンサービスです。オンラインで販売されるすべての貸切送迎を指す共通の英語名称ではありません。
```

### ITEM 0590

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#call-van-private-transfer > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Incheon Airport currently describes the service for groups of five or fewer with 20 kg of luggage per person. The information counter is between exits 12 and 13 at T1 and near exit 7 at T2. Pickup is at 10C in T1 and 8D in T2. Published operating hours are 08:00–21:00, fares are distance-based and tolls are separate.
```

Japanese:

```text
仁川空港は現在、5人以下・1人あたり20 kgの荷物を条件として案内しています。案内カウンターは第1ターミナルの12番出口と13番出口の間、第2ターミナルの7番出口付近。乗車場所は第1ターミナル10C、第2ターミナル8Dです。公表営業時間は08:00～21:00、運賃は距離制で、通行料は別途です。
```

### ITEM 0591

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#call-van-private-transfer > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
A private transfer is the broader booking term travelers see on international platforms. The vehicle, luggage allowance, meeting point, waiting time and cancellation terms belong to the individual reservation rather than to the words “private transfer” themselves.
```

Japanese:

```text
貸切送迎は、旅行者が国際予約サイトで目にする、より広い予約用語です。車種、荷物条件、待ち合わせ場所、待ち時間、キャンセル条件は「貸切送迎」という言葉だけで決まるものではなく、各予約の条件によります。
```

### ITEM 0592

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#call-van-private-transfer > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(4) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Call Van / Private Transfer guide
```

Japanese:

```text
コールバン／貸切送迎ガイド
```

### ITEM 0593

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#call-van-private-transfer > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(4)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
For larger groups, luggage capacity and pre-booking details, see the
```

Japanese:

```text
大人数、荷物容量、事前予約の詳細は
```

### ITEM 0594

- Element/type: H2
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#late-arrival > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h2:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
A Late Arrival Is About When You Leave the Terminal
```

Japanese:

```text
深夜到着は「飛行機の到着時刻」より「ターミナルを出る時刻」で考える
```

### ITEM 0595

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#late-arrival > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
A flight landing late in the evening does not put you outside the terminal at that time. Immigration, baggage claim and customs come first, and that gap matters when the last convenient train or bus is approaching.
```

Japanese:

```text
夜遅くに着陸しても、その時刻にすぐターミナルの外へ出られるわけではありません。まず入国審査、手荷物受取、税関があり、この時間差は最終の便利な列車やバスが迫っているときに重要です。
```

### ITEM 0596

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#late-arrival > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Incheon Airport currently lists N6000, N6002, N6701 and N6703 among its overnight bus services. N6000 and N6002 are currently ₩17,000 for adults and ₩10,000 for children. N6701 and N6703 are ₩18,000 for adults and ₩12,000 for children.
```

Japanese:

```text
仁川空港は現在、深夜バスとしてN6000、N6002、N6701、N6703などを案内しています。N6000とN6002は現在、大人₩17,000・子ども₩10,000。N6701とN6703は大人₩18,000・子ども₩12,000です。
```

### ITEM 0597

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#late-arrival > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
T1 and T2 use different departure times, so the airport's current timetable is more useful than a static list copied into a travel plan.
```

Japanese:

```text
第1ターミナルと第2ターミナルでは出発時刻が異なるため、旅行計画に固定時刻を書き写しておくより、仁川空港の最新時刻表を確認するほうが役立ちます。
```

### ITEM 0598

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#late-arrival > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(4)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
If the usable train or night-bus window has already closed, the official taxi stand or a pre-booked vehicle becomes the practical alternative.
```

Japanese:

```text
利用できる列車や深夜バスの時間をすでに過ぎているなら、公式タクシー乗り場または事前予約車両が現実的な代替手段です。
```

### ITEM 0599

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#late-arrival > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(5) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The Arrival Guide
```

Japanese:

```text
到着ガイド
```

### ITEM 0600

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#late-arrival > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(5)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
explains the airport steps that come before you reach the public arrival hall.
```

Japanese:

```text
では、一般到着ロビーに出るまでに必要な空港内の手続きを説明しています。
```

### ITEM 0601

- Element/type: H2
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#terminals > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h2:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Terminal 1 and Terminal 2 Work Differently
```

Japanese:

```text
第1ターミナルと第2ターミナルでは場所が違う
```

### ITEM 0602

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#terminals > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The transport choices are largely the same at both terminals, but the ticketing areas, rail levels and vehicle stands are not.
```

Japanese:

```text
利用できる交通手段は両ターミナルでほぼ同じですが、乗車券売り場、鉄道フロア、車両乗り場は異なります。
```

### ITEM 0603

- Element/type: H3
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#terminals > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h3:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Terminal 1
```

Japanese:

```text
第1ターミナル
```

### ITEM 0604

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#terminals > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Airport buses leave from the arrivals level. Ticketing is available inside near exits 4 and 9, with additional facilities outside near exits 4, 6, 7, 8, 11 and 13.
```

Japanese:

```text
空港バスは到着階から出発します。乗車券は館内の4番・9番出口付近で購入でき、屋外にも4、6、7、8、11、13番出口付近に関連施設があります。
```

### ITEM 0605

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#terminals > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
AREX ticketing and guidance are in Transportation Center B1, while trains use B4.
```

Japanese:

```text
AREXの乗車券・案内は交通センターB1、列車ホームはB4です。
```

### ITEM 0606

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#terminals > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(4)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Regular Seoul taxis use 5C, 6C and 6D; Deluxe and Jumbo taxis use 7C and 8C; International Taxi uses 4C.
```

Japanese:

```text
ソウル一般タクシーは5C・6C・6D、DeluxeとJumboは7C・8C、International Taxiは4Cを利用します。
```

### ITEM 0607

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#terminals > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(5)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The Call Van information counter is between exits 12 and 13, with pickup at 10C.
```

Japanese:

```text
コールバン案内カウンターは12番出口と13番出口の間、乗車場所は10Cです。
```

### ITEM 0608

- Element/type: H3
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#terminals > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h3:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Terminal 2
```

Japanese:

```text
第2ターミナル
```

### ITEM 0609

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#terminals > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(6)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Airport-bus ticketing, guidance and boarding are in Transportation Center B1.
```

Japanese:

```text
空港バスの乗車券、案内、乗車場所は交通センターB1にあります。
```

### ITEM 0610

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#terminals > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(7)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
AREX ticketing and guidance are also on B1, while trains use B3.
```

Japanese:

```text
AREXの乗車券・案内もB1、列車ホームはB3です。
```

### ITEM 0611

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#terminals > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(8)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Regular Seoul taxis use 7C; Deluxe and Jumbo taxis use 7D; International Taxi uses 3C.
```

Japanese:

```text
ソウル一般タクシーは7C、DeluxeとJumboは7D、International Taxiは3Cを利用します。
```

### ITEM 0612

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#terminals > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(9)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The Call Van information counter is near exit 7, with pickup at 8D.
```

Japanese:

```text
コールバン案内カウンターは7番出口付近、乗車場所は8Dです。
```

### ITEM 0613

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#terminals > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(10) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The Incheon Airport guide
```

Japanese:

```text
仁川空港ガイド
```

### ITEM 0614

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#terminals > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(10)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
has the wider arrival-hall context for both terminals.
```

Japanese:

```text
では、両ターミナルの到着ロビー全体についてさらに詳しく説明しています。
```

### ITEM 0615

- Element/type: H2
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#practical-answer > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > h2:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Which Airport Transfer Fits Your Trip?
```

Japanese:

```text
あなたの旅行に合う空港移動はどれ？
```

### ITEM 0616

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#practical-answer > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Seoul Station is the clearest case for the AREX Express. Hongdae is usually simpler on the All-stop Train because it stops at Hongik University.
```

Japanese:

```text
ソウル駅へ行くなら、AREX直通列車が最も分かりやすいケースです。弘大へは、弘大入口駅に停まる各駅停車のほうが通常シンプルです。
```

### ITEM 0617

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#practical-answer > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(2)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
An airport bus becomes attractive when its actual stop is close to the hotel. A taxi becomes more useful as the final walk, luggage or group size gets harder to manage.
```

Japanese:

```text
空港バスは、実際の停留所がホテルの近くなら魅力的です。最後に歩く距離、荷物、人数の負担が大きくなるほど、タクシーの利点が増します。
```

### ITEM 0618

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#practical-answer > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(3)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
For a family, several large suitcases or a late arrival, Call Van or a pre-booked private transfer can justify the extra cost by removing a station transfer and the last kilometre with luggage.
```

Japanese:

```text
家族旅行、大型スーツケースが複数、深夜到着なら、コールバンや事前予約の貸切送迎は、駅での乗り換えと荷物を持った最後の1 kmをなくすことで、追加料金に見合う場合があります。
```

### ITEM 0619

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#practical-answer > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(4)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The fastest first ride is not always the easiest arrival. The journey that matters is the one that ends at the hotel door.
```

Japanese:

```text
最初の乗り物が一番速くても、到着全体が一番楽とは限りません。大切なのは、ホテルの入口まで続く移動です。
```

### ITEM 0620

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#practical-answer > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(5)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Your transport choice and your hotel area are linked. If the accommodation is not fixed yet, compare Hongdae, Gongdeok, Seoul Station and Myeongdong by luggage handling, the final walk and the rest of your Seoul itinerary.
```

Japanese:

```text
交通手段と宿泊エリアはつながっています。まだ宿泊先が決まっていないなら、弘大、孔徳、ソウル駅、明洞を、荷物の運びやすさ、最後の徒歩距離、その後のソウルでの旅程まで含めて比較してください。
```

### ITEM 0621

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#practical-answer > div.container:nth-of-type(1) > div.transfer-chapter__header:nth-of-type(1) > p:nth-of-type(6) > a:nth-of-type(1) > strong:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Compare where to stay for airport access →
```

Japanese:

```text
空港アクセスで泊まるエリアを比較 →
```

### ITEM 0622

- Element/type: H2
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > header.transfer-chapter__header:nth-of-type(1) > h2:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Frequently Asked Questions
```

Japanese:

```text
よくある質問
```

### ITEM 0623

- Element/type: visible FAQ question
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(1) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Is AREX or the airport bus better?
```

Japanese:

```text
AREXと空港バス、どちらが便利？
```

### ITEM 0624

- Element/type: visible FAQ answer/direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(1) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
AREX is more predictable and works especially well for Seoul Station or Hongdae. The airport bus can be easier when its actual stop is close to the hotel and removes another station transfer. The final walk matters as much as the headline travel time.
```

Japanese:

```text
AREXは所要時間を読みやすく、特にソウル駅や弘大へ向かうときに使いやすい選択肢です。空港バスは、実際の停留所がホテルの近くにあり、駅での乗り換えを1回減らせるなら楽になることがあります。表示上の所要時間と同じくらい、最後にどれだけ歩くかが重要です。
```

### ITEM 0625

- Element/type: visible FAQ question
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(2) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
What is the cheapest way from Incheon Airport to Seoul?
```

Japanese:

```text
仁川空港からソウルまで一番安い方法は？
```

### ITEM 0626

- Element/type: visible FAQ answer/direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(2) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The AREX All-stop Train is usually the cheapest rail option. Current adult transit-card fares are ₩4,650 from T1 and ₩5,250 from T2 to Hongik University, and ₩4,750 from T1 and ₩5,350 from T2 to Seoul Station.
```

Japanese:

```text
鉄道では、AREX各駅停車が通常もっとも安い選択肢です。現在の大人交通カード運賃は、弘大入口駅まで第1ターミナルから₩4,650、第2ターミナルから₩5,250、ソウル駅まで第1ターミナルから₩4,750、第2ターミナルから₩5,350です。
```

### ITEM 0627

- Element/type: visible FAQ question
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(3) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
What is the fastest way to Seoul Station?
```

Japanese:

```text
ソウル駅まで一番速い方法は？
```

### ITEM 0628

- Element/type: visible FAQ answer/direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(3) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The AREX Express has published airport-to-Seoul Station journey times of 43 minutes from T1 and 51 minutes from T2. The full trip still includes the walk to the airport station and whatever comes after Seoul Station.
```

Japanese:

```text
AREX直通列車の公表所要時間は、空港からソウル駅まで第1ターミナルから43分、第2ターミナルから51分です。ただし実際の全行程には、到着ロビーから空港駅までの徒歩と、ソウル駅に着いた後の移動も含まれます。
```

### ITEM 0629

- Element/type: visible FAQ question
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(4) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Does the AREX Express stop at Hongdae?
```

Japanese:

```text
AREX直通列車は弘大に停まる？
```

### ITEM 0630

- Element/type: visible FAQ answer/direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(4) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
No. The AREX Express runs directly to Seoul Station and does not stop at Hongik University. The All-stop Train stops at Hongik University and is usually the more direct rail option for Hongdae.
```

Japanese:

```text
停まりません。AREX直通列車はソウル駅まで直通で、弘大入口駅には停車しません。各駅停車は弘大入口駅に停まるため、弘大へは通常こちらのほうが直接的です。
```

### ITEM 0631

- Element/type: visible FAQ question
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(5) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Can I use T-money on the AREX?
```

Japanese:

```text
AREXでT-moneyは使える？
```

### ITEM 0632

- Element/type: visible FAQ answer/direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(5) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
T-money can be used on the AREX All-stop Train. The AREX Express uses a separate ticket and reserved-seat system.
```

Japanese:

```text
AREX各駅停車ではT-moneyを利用できます。AREX直通列車は別の乗車券と指定席の仕組みです。
```

### ITEM 0633

- Element/type: visible FAQ question
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(6) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
What works better with several large suitcases?
```

Japanese:

```text
大きなスーツケースが複数ある場合はどれが便利？
```

### ITEM 0634

- Element/type: visible FAQ answer/direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(6) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The answer changes with the number and size of the bags. An airport bus can work well when its stop is close to the hotel, while a larger taxi, Call Van or pre-booked transfer becomes more useful when moving the luggage through a station would be difficult. Passenger capacity and luggage capacity are separate limits.
```

Japanese:

```text
荷物の数と大きさで変わります。空港バスは停留所がホテルの近くなら便利です。一方、駅構内で荷物を運ぶのが難しい量なら、大型タクシー、コールバン、事前予約の送迎が使いやすくなります。乗車できる人数と積める荷物量は別の制限です。
```

### ITEM 0635

- Element/type: visible FAQ question
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(7) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
What can I use after midnight?
```

Japanese:

```text
深夜0時を過ぎたら何が使える？
```

### ITEM 0636

- Element/type: visible FAQ answer/direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(7) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Incheon Airport currently lists overnight services including N6000, N6002, N6701 and N6703, with different departure times at T1 and T2. Official taxis and pre-booked vehicles are alternatives when the useful train or night-bus window has closed.
```

Japanese:

```text
仁川空港は現在、N6000、N6002、N6701、N6703などの深夜便を案内しており、第1ターミナルと第2ターミナルで出発時刻が異なります。利用できる列車や深夜バスの時間を過ぎた場合は、公式タクシーや事前予約車両が代替手段になります。
```

### ITEM 0637

- Element/type: visible FAQ question
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(8) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
How do taxis from Incheon Airport work?
```

Japanese:

```text
仁川空港からのタクシーはどう利用する？
```

### ITEM 0638

- Element/type: visible FAQ answer/direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(8) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Official airport taxi stands serve regular, Deluxe, Jumbo and International Taxi services. A regular Seoul taxi currently starts at ₩4,800 for 1.6 km, while Deluxe, SUV and Jumbo services start at ₩7,000 for 3 km. The final fare depends on the service, destination, traffic and time of travel.
```

Japanese:

```text
空港公式タクシー乗り場から、一般、Deluxe、Jumbo、International Taxiを利用できます。現在、ソウルの一般タクシーは1.6 kmまで₩4,800から、Deluxe、SUV、Jumboは3 kmまで₩7,000からです。最終料金はサービス、目的地、交通状況、利用時間によって変わります。
```

### ITEM 0639

- Element/type: visible FAQ question
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(9) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Does Terminal 1 or Terminal 2 change the route?
```

Japanese:

```text
第1ターミナルと第2ターミナルで移動方法は変わる？
```

### ITEM 0640

- Element/type: visible FAQ answer/direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(9) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
The main transport choices are similar, but the ticketing areas, rail levels, taxi stands and Call Van locations are different. T1 and T2 also use different departure times for some late-night services.
```

Japanese:

```text
主な移動手段はほぼ同じですが、乗車券売り場、鉄道フロア、タクシー乗り場、コールバンの場所は異なります。一部の深夜便は、第1ターミナルと第2ターミナルで出発時刻も異なります。
```

### ITEM 0641

- Element/type: visible FAQ question
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(10) > summary:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Is Call Van the same as a private transfer?
```

Japanese:

```text
コールバンと貸切送迎は同じもの？
```

### ITEM 0642

- Element/type: visible FAQ answer/direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#faq > div.container:nth-of-type(1) > div.airport-faq:nth-of-type(1) > details:nth-of-type(10) > p:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
No. Incheon Airport's Call Van is a Korean commercial van service with its own airport conditions. “Private transfer” is a broader booking term for a vehicle reserved for one party. Some private-transfer bookings may use vans, but the terms are not interchangeable.
```

Japanese:

```text
同じではありません。仁川空港のコールバンは、空港独自の利用条件がある韓国の商用バンサービスです。「貸切送迎」は、1組の利用者が車両を予約するサービス全般を指す、より広い予約用語です。貸切送迎でバン車両を使う場合もありますが、同じ意味ではありません。
```

### ITEM 0643

- Element/type: H2
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > header.transfer-chapter__header:nth-of-type(1) > h2:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Official Sources
```

Japanese:

```text
公式情報
```

### ITEM 0644

- Element/type: body direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > header.transfer-chapter__header:nth-of-type(1) > p.transfer-source-intro:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Fares, schedules, baggage rules and operating locations can change. The links below are the primary sources used for the information on this page.
```

Japanese:

```text
運賃、時刻表、手荷物規定、乗り場は変更されることがあります。以下は、このページの情報確認に使用した主な一次情報です。
```

### ITEM 0645

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > ul.transfer-official-sources:nth-of-type(1) > li:nth-of-type(1) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
AREX Express fares and journey times
```

Japanese:

```text
AREX直通列車の運賃・所要時間
```

### ITEM 0646

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > ul.transfer-official-sources:nth-of-type(1) > li:nth-of-type(2) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
AREX Express timetable
```

Japanese:

```text
AREX直通列車の時刻表
```

### ITEM 0647

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > ul.transfer-official-sources:nth-of-type(1) > li:nth-of-type(3) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
AREX All-stop fares
```

Japanese:

```text
AREX各駅停車の運賃
```

### ITEM 0648

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > ul.transfer-official-sources:nth-of-type(1) > li:nth-of-type(4) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
AREX All-stop service
```

Japanese:

```text
AREX各駅停車の運行情報
```

### ITEM 0649

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > ul.transfer-official-sources:nth-of-type(1) > li:nth-of-type(5) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
AREX transport terms and baggage
```

Japanese:

```text
AREX運送約款・手荷物規定
```

### ITEM 0650

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > ul.transfer-official-sources:nth-of-type(1) > li:nth-of-type(6) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Incheon Airport bus information — Terminal 1
```

Japanese:

```text
仁川空港 バス案内 — 第1ターミナル
```

### ITEM 0651

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > ul.transfer-official-sources:nth-of-type(1) > li:nth-of-type(7) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Incheon Airport bus information — Terminal 2
```

Japanese:

```text
仁川空港 バス案内 — 第2ターミナル
```

### ITEM 0652

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > ul.transfer-official-sources:nth-of-type(1) > li:nth-of-type(8) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Incheon Airport late-night buses — Terminal 1
```

Japanese:

```text
仁川空港 深夜バス — 第1ターミナル
```

### ITEM 0653

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > ul.transfer-official-sources:nth-of-type(1) > li:nth-of-type(9) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Incheon Airport late-night buses — Terminal 2
```

Japanese:

```text
仁川空港 深夜バス — 第2ターミナル
```

### ITEM 0654

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > ul.transfer-official-sources:nth-of-type(1) > li:nth-of-type(10) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Incheon Airport taxi guide
```

Japanese:

```text
仁川空港 タクシー案内
```

### ITEM 0655

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > ul.transfer-official-sources:nth-of-type(1) > li:nth-of-type(11) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Incheon Airport Call Van guide
```

Japanese:

```text
仁川空港 コールバン案内
```

### ITEM 0656

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > ul.transfer-official-sources:nth-of-type(1) > li:nth-of-type(12) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Airport Limousine routes and fares
```

Japanese:

```text
Airport Limousineの路線・運賃
```

### ITEM 0657

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > ul.transfer-official-sources:nth-of-type(1) > li:nth-of-type(13) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
K Airport Limousine fares
```

Japanese:

```text
K Airport Limousineの運賃
```

### ITEM 0658

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > ul.transfer-official-sources:nth-of-type(1) > li:nth-of-type(14) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
Seoul taxi information
```

Japanese:

```text
ソウルのタクシー情報
```

### ITEM 0659

- Element/type: visible link direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > main:nth-of-type(1) > section#official-sources > div.container:nth-of-type(1) > ul.transfer-official-sources:nth-of-type(1) > li:nth-of-type(15) > a:nth-of-type(1)::text[1]`
- Reuse: NEW airport-transfer-specific localization

English:

```text
International Taxi
```

Japanese:

```text
International Taxi
```

### ITEM 0660

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > div.footer__brand-block:nth-of-type(1) > p.footer__brand:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0535

English:

```text
Korea Inside
```

Japanese:

```text
Korea Inside
```

### ITEM 0661

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > div.footer__brand-block:nth-of-type(1) > p.footer__eyebrow:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0536

English:

```text
CREATED IN KOREA
```

Japanese:

```text
韓国発
```

### ITEM 0662

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > div.footer__brand-block:nth-of-type(1) > p.footer__statement:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0537

English:

```text
Practical Korea travel guidance, written and reviewed locally by a Korean editor.
```

Japanese:

```text
韓国人エディターが現地で執筆・確認する、実用重視の韓国旅行ガイドです。
```

### ITEM 0663

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > div.footer__brand-block:nth-of-type(1) > p.footer__description:nth-of-type(4)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0538

English:

```text
Based on official sources, local context, and independent editorial judgment.
```

Japanese:

```text
公式情報、現地事情、独立した編集判断をもとに作成しています。
```

### ITEM 0664

- Element/type: literal ARIA label
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1)@aria-label`
- Reuse: Japanese Golden Sample Approved ITEM 0539

English:

```text
Footer navigation
```

Japanese:

```text
フッターナビゲーション
```

### ITEM 0665

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(1) > p.footer__heading:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0540

English:

```text
PLAN YOUR TRIP
```

Japanese:

```text
旅行を計画する
```

### ITEM 0666

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(1) > ul.footer__links:nth-of-type(1) > li:nth-of-type(1) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0541

English:

```text
Airport
```

Japanese:

```text
空港
```

### ITEM 0667

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(1) > ul.footer__links:nth-of-type(1) > li:nth-of-type(2) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0542

English:

```text
eSIM
```

Japanese:

```text
eSIM
```

### ITEM 0668

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(1) > ul.footer__links:nth-of-type(1) > li:nth-of-type(3) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0543

English:

```text
Checklist
```

Japanese:

```text
チェックリスト
```

### ITEM 0669

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(2) > p.footer__heading:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0544

English:

```text
USE KOREA
```

Japanese:

```text
韓国で使う
```

### ITEM 0670

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(2) > ul.footer__links:nth-of-type(1) > li:nth-of-type(1) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0545

English:

```text
T-money
```

Japanese:

```text
T-money
```

### ITEM 0671

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(2) > ul.footer__links:nth-of-type(1) > li:nth-of-type(2) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0546

English:

```text
Payments
```

Japanese:

```text
支払い
```

### ITEM 0672

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(2) > ul.footer__links:nth-of-type(1) > li:nth-of-type(3) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0547

English:

```text
Maps
```

Japanese:

```text
地図
```

### ITEM 0673

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__grid:nth-of-type(1) > nav.footer__nav:nth-of-type(1) > div.footer__group:nth-of-type(2) > ul.footer__links:nth-of-type(1) > li:nth-of-type(4) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0548

English:

```text
Apps
```

Japanese:

```text
アプリ
```

### ITEM 0674

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0549

English:

```text
© 2026 Korea Inside · Republic of Korea
```

Japanese:

```text
© 2026 Korea Inside · 大韓民国
```

### ITEM 0675

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p.footer__business:nth-of-type(2) > a:nth-of-type(1)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0552

English:

```text
getkoreainside@gmail.com
```

Japanese:

```text
getkoreainside@gmail.com
```

### ITEM 0676

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p.footer__business:nth-of-type(2) > a.footer__legal-link:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0550

English:

```text
Affiliate Disclosure
```

Japanese:

```text
アフィリエイト開示
```

### ITEM 0677

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p.footer__business:nth-of-type(2) > a.footer__legal-link:nth-of-type(3)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0551

English:

```text
Privacy Policy
```

Japanese:

```text
プライバシーポリシー
```

### ITEM 0678

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p.footer__business:nth-of-type(2)::text[1]`
- Reuse: Japanese Golden Sample Approved ITEM 0553

English:

```text
Business Registration No. 462-39-01721
```

Japanese:

```text
事業者登録番号 462-39-01721
```

### ITEM 0679

- Element/type: common UI footer direct text
- Source target: `airport-transfer.html|html:nth-of-type(1) > body.airport-transfer-page:nth-of-type(1) > footer.footer:nth-of-type(1) > div.container:nth-of-type(1) > div.footer__bottom:nth-of-type(2) > p.footer__business:nth-of-type(2)::text[2]`
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

- Source ITEM count: **265**
- Localized ITEM count: **265**
- ITEM range: **0415–0679**
- Common UI reuse: **83 / 83**
- Airport Transfer-specific localization: **182 / 182**
- Missing localization ITEM: **0**
- Empty Japanese ITEM: **0**
- Golden Sample approved reference missing: **0**
- JSON-LD FAQ ↔ visible FAQ Japanese wording parity: **10 / 10**
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
- no factual, fare, time, luggage-rule, terminal, or operating-condition change
- Codex exact implementation only