# Korea Inside — Japanese Localization Batch 2 — AREX

**Date:** 2026-09-24  
**Status:** APPROVED PUBLIC COPY — CONTENT LOCKED  
**Target English:** `arex.html`  
**Target Japanese:** `ja/arex.html`  
**English source Git blob:** `66d291a7c4ed882db3fe983bcbe0dd3498059de0`  
**Page family:** Airport / Transport  

## Scope

- English Production source checked in full: `arex.html`
- Japanese common UI: reuse the existing CONTENT LOCKED Japanese Golden Sample exactly.
- Common UI reuse target: **83 / 83**.
- AREX page-specific user-facing localization scope extracted from `<title>`, meta description and `<main>`: **293 source nodes**.
- JSON-LD in English source: **0**. Do not invent or add FAQ schema in this Batch.
- Visible FAQ count: **12**.
- Preserve all facts, fares, times, station roles, terminal distinctions, service conditions, links, images, HTML structure, class/id/data-* and functional logic.

## Japanese editorial-localization rules

- This is editorial localization, not literal translation.
- `AREX Express` / `Express Train` → `AREX直通列車` / `直通列車`.
- `All-Stop Train` → `AREX各駅停車` / `各駅停車`.
- `Seoul Station` → `ソウル駅`.
- `Hongik University Station` → `弘大入口駅`.
- `Hongdae` → `弘大`.
- `Gongdeok` → `孔徳`.
- `Myeongdong` → `明洞`.
- `Jongno` → `鍾路`.
- `Dongdaemun` → `東大門`.
- `Gangnam` → `江南`.
- `Jamsil` → `蚕室`.
- `Gimpo Airport` → `金浦空港`.
- `Incheon Airport` → `仁川空港`.
- `T-money`, `AREX`, `KTX`, Visa, Mastercard, JCB, Diners Club, American Express, UnionPay and Korea Inside remain recognizable brand/product names.
- Fares, minutes, first/last train times and terminal numbers must remain exactly the same factual values.
- Existing internal/external URLs and tracking must not be changed by localization.
- Common UI is not retranslated; use the existing approved Japanese wording exactly.

---

# APPROVED JAPANESE PUBLIC COPY

## SEO

### Meta description

```text
仁川空港からソウルへ向かうAREX直通列車と各駅停車を、料金、所要時間、停車駅、乗車券、T-money、荷物、深夜到着時の選び方まで比較します。
```

### Title

```text
仁川空港からソウルへAREX：料金・所要時間・列車ガイド | Korea Inside
```

---

## Hero

### Breadcrumb visible text

```text
ホーム / 空港 / AREX
```

### H1

```text
仁川空港からソウルへAREXで移動：直通列車 vs 各駅停車
```

### Hero image alt

```text
仁川空港からソウルへ向かうAREXと、直通列車・各駅停車の選択肢
```

### Lead paragraph 1

```text
AREXには2種類の列車があります。直通列車は空港の各ターミナルからソウル駅まで指定席で運行し、各駅停車は弘大入口駅、孔徳、金浦空港など途中の駅にも停車します。
```

### Lead paragraph 2

```text
ソウル駅までなら直通列車のほうが速いものの、どちらが便利かは宿泊場所で変わります。ホテルによっては、所要時間の長い各駅停車でも乗り換えを1回減らせます。ソウル駅まで最速の列車が、ホテルまで最速とは限りません。
```

### Quick choice ARIA

```text
AREXのかんたん選択
```

### Express choice label

```text
直通列車が向いているのは…
```

### Express choice body

```text
ホテルがソウル駅の近くにある、次の移動がソウル駅から始まる、または安さより指定席を重視したい場合です。
```

### All-Stop choice label

```text
各駅停車が向いているのは…
```

### All-Stop choice body

```text
弘大や孔徳周辺に泊まる、金浦空港で乗り継ぐ、またはT-moneyを使って距離制の安い運賃で移動したい場合です。
```

### Quick links ARIA

```text
AREXガイドの目次
```

### Quick links

```text
直通列車 vs 各駅停車
主な駅
乗車券
深夜到着
```

---

## Section: Express or All-Stop?

### H2

```text
直通列車と各駅停車、どちらを選ぶ？
```

### Section answer

```text
どちらも仁川空港とソウルを結びますが、停車パターンと乗車券の仕組みが異なります。
```

### H3 — Express Train

```text
AREX直通列車
```

### Body

```text
直通列車は第2ターミナルから第1ターミナルを経由し、途中で乗客を乗せ降ろしせずソウル駅まで運行します。指定席で、各駅停車とは別の乗車券が必要です。ソウル駅へ直接向かう場合や、十分な乗り換え時間を確保してKTXへ接続する場合に使いやすい選択です。
```

### H3 — All-Stop Train

```text
AREX各駅停車
```

### Body

```text
各駅停車は金浦空港、弘大入口駅、孔徳を含むAREXの全駅に停車します。通勤電車に近い使い方でT-moneyも利用できるため、ソウル駅まで行かずにホテルの近くで降りられれば、市内を戻るような余計な移動を避けられます。
```

### Closing body with inline links

```text
「ソウル」は1つの目的地ではありません。空港からソウル駅までの区間で時間を節約しても、乗り換え、長い駅構内の移動、西側へ戻る移動でその差がなくなることがあります。荷物を持って鉄道ルートが複雑になるなら、仁川空港からのほかの移動手段も比較してください。空港バスや公式タクシーのほうが、荷物を持ち替える回数を減らせる場合があります。
```

Inline linked phrases, in source order:

```text
仁川空港からのほかの移動手段を比較
空港バス
公式タクシー
```

---

## Section: Express vs All-Stop Quick Comparison

### H2

```text
直通列車 vs 各駅停車 早見比較
```

### Section answer

```text
この表では、券売機の前で判断するときに必要な違いをまとめています。降車後に何が待っているかは、この後の目的地別セクションで説明します。
```

### Comparison ARIA

```text
AREX直通列車と各駅停車の比較
```

### Table caption

```text
仁川空港からソウル方面へ向かうAREX
```

### Table headings

```text
項目
直通列車
各駅停車
```

### Table rows — exact order

```text
停車駅
空港各ターミナルとソウル駅の間を途中停車なしで運行
金浦空港、弘大入口駅、孔徳を含む全駅

T1からソウル駅
43分
59分。一部列車はさらに2～6分長くかかります

T2からソウル駅
51分
66分。一部列車はさらに2～6分長くかかります

大人運賃
公式販売運賃 ₩13,000
交通カード利用でソウル駅まで：T1 ₩4,750、T2 ₩5,350

座席
指定席
通勤電車型の自由席。着席は保証されません

T-money
利用不可。直通列車専用の乗車券が必要
利用可。十分な残高のある交通カードでタッチ

乗車券の種類
QRコード付きの予約済み直通列車乗車券、または駅発券の乗車券
T-moneyなど利用可能な交通カード、または1回用交通カード

主な違い
速く指定席だが、料金が高くソウル駅までのみ
安く便利な途中駅が多いが、時間がかかり指定席ではない
```

---

## Section: How Much Does AREX Cost?

### H2

```text
AREXの料金はいくら？
```

### Section answer

```text
直通列車はソウル駅まで定額の販売運賃です。各駅停車は距離制なので、利用する空港ターミナルと降車駅の両方で料金が変わります。
```

### H3

```text
直通列車の運賃
```

### Body

```text
公式予約システムでは、非会員の大人が₩13,000、会員の大人が₩12,500、子どもが₩9,500と案内されています。直通列車は距離制の交通運賃ではなく、別途予約する乗車券を使用します。T1・T2のどちらから乗っても、ソウル駅までの販売運賃は同じです。
```

### H3

```text
各駅停車の運賃
```

### Body

```text
プリペイドまたは後払いの交通カードを使う場合、大人運賃はT1からソウル駅まで₩4,750、弘大入口駅まで₩4,650です。T2からはそれぞれ₩5,350、₩5,250です。追加の₩600は、第2ターミナルからの距離が長いことによる差です。
```

### H3

```text
T-moneyと1回用交通カードの違い
```

### Body

```text
T-moneyは各駅停車で入場・出場時にタッチして使えますが、直通列車の乗車券にはなりません。大人用の1回用交通カードは、交通カード運賃に₩100を加えた金額で、返金可能な保証金₩500が必要です。目的地で改札を出たあと、カードを保証金返還機に入れると保証金が返金されます。
```

---

## Section: How Much Time Does Express Really Save?

### H2

```text
直通列車で実際にどれくらい時間を節約できる？
```

### Section answer

```text
公表されている所要時間はソウル駅までです。実際の移動には、空港駅までの徒歩、待ち時間、その先の乗り継ぎ、ホテルまでの徒歩も含まれます。
```

### H3 / body — Terminal 1

```text
第1ターミナルから
直通列車はソウル駅まで43分です。各駅停車は59分の予定ですが、一部列車はさらに2～6分長くかかります。
```

### H3 / body — Terminal 2

```text
第2ターミナルから
直通列車はソウル駅まで51分です。各駅停車は66分で、一部列車は同様にさらに2～6分長くかかります。
```

### Closing body

```text
列車に乗っている時間だけを見ると、直通列車は約15～16分短縮できます。ただし、ソウル駅で地下鉄への乗り換えが必要になる一方、各駅停車なら弘大入口駅や孔徳へ直接行ける場合は、その差が小さくなります。荷物、エレベーター、最後の徒歩は、時刻表上の差より重要になることがあります。
```

---

## Section: Main AREX Stations and Stops

### H2

```text
AREXの主な駅と停車駅
```

### Section answer

```text
直通列車が停まるのは空港の2つのターミナルとソウル駅です。各駅停車はその途中駅にも停まり、その違いが到着後の移動を楽にできるかどうかを左右します。
```

### Route map alt

```text
仁川空港の各ターミナル、主要駅、ソウル駅を示したAREX直通列車と各駅停車の路線図
```

### Figcaption

```text
仁川空港からソウルまでのAREX直通列車・各駅停車ルート
```

### Incheon Airport Terminal 2

```text
仁川空港 第2ターミナル
直通列車と各駅停車はいずれもここを出発し、その後T1に停車します。駅は第2ターミナル交通センターにつながっていますが、T1の駅とは別です。航空券に表示されている利用ターミナルを必ず確認してください。
```

### Incheon Airport Terminal 1

```text
仁川空港 第1ターミナル
両列車とも第1ターミナル交通センター下の駅に停車します。空港内の動線は変わることがあるため、古いガイドに保存されたゲート番号より、現地の最新のAirport Railroad / AREX表示を優先してください。
```

### Gimpo International Airport

```text
金浦国際空港
金浦空港に停車するのは各駅停車だけです。ここで地下鉄5号線・9号線、金浦ゴールドラインなどへ乗り換えられます。航空便へ乗り継ぐ場合も正しいターミナルまで歩く必要があるため、列車の到着時刻がそのまま搭乗時刻になるわけではありません。
```

### Hongik University

```text
弘大入口駅
各駅停車なら弘大入口駅へ直接行け、地下鉄2号線と京義・中央線に乗り換えられます。弘大のホテルへ行くなら、直通列車でソウル駅まで行ってから西側へ戻る必要は通常ありません。ただし駅は広く、荷物があると正しい出口を選ぶかどうかで移動の負担がかなり変わります。
```

### Gongdeok

```text
孔徳駅
各駅停車はソウル駅の手前で孔徳駅に停車します。駅周辺のホテルなら2本目の列車は不要で、地下鉄5号線・6号線、京義・中央線へも乗り継げます。実際に便利な出口や乗り換えは、ホテルの正確な位置で決まります。
```

### Seoul Station

```text
ソウル駅
両列車ともソウル駅が終点です。地下鉄1号線・4号線、京義・中央線、全国鉄道に接続しています。AREXホームは大きな駅構内の深い位置にあり、地下鉄やKTXのホームへ移動するには、通路、エレベーターまたはエスカレーターと追加の徒歩が必要です。
```

---

## Section: Where Are You Going After AREX?

### H2

```text
AREXを降りたあと、どこへ行く？
```

### Section answer

```text
見るべきなのは、どちらの列車が速いかだけではありません。AREXをどこで降りられるか、そしてそこから目的地までどれだけ移動が残るかが重要です。
```

### Destination graphic alt

```text
ソウル駅、弘大、孔徳、明洞、江南などへ向かう際のAREX列車と降車駅の選び方
```

### Figcaption

```text
ソウル主要エリア別のAREX列車・降車駅の選択肢
```

### Seoul Station

```text
ソウル駅
直通列車は早く、指定席があります。各駅停車なら同じソウル駅までより安く移動できます。ホテルが近ければ徒歩圏でも、駅の反対側にあるホテルでは長い出口動線が残ることがあります。荷物を持った最後の徒歩が大変そうなら、ホテル住所に近いバス停やタクシー降車地点も比較してください。
```

### Hongdae

```text
弘大
各駅停車は弘大入口駅に直接停車します。弘大のホテルへ向かうのに、直通列車でソウル駅まで行ってから西へ戻る理由はほとんどありません。ただし駅は広く、スーツケースを引いていると正しい出口を選ぶかどうかで負担がかなり変わります。
```

### Gongdeok

```text
孔徳
各駅停車なら乗り換えなしで孔徳へ行けます。駅に近いホテルならそのまま歩けますが、別の住所なら地下鉄5号線・6号線へ乗り継ぐこともできます。どのルートが便利かはホテルの正確な位置で決まります。
```

### Myeongdong

```text
明洞
直通列車でソウル駅へ行き、4号線へ乗り換えるルートは速い鉄道移動になり得ます。各駅停車なら空港区間をより安く移動できます。ただし、どちらも乗り換え自体はなくなりません。荷物が多いなら、ソウル駅構内を移動するよりホテル近くに停まる空港バスのほうが楽な場合があります。
```

### Jongno

```text
鍾路
鍾路は広いため、AREXの降車駅を1つに決められません。ホテルによっては孔徳から5号線が自然な場合もあれば、ソウル駅や空港バスのほうが便利な場合もあります。地区名だけでなく、正確な住所と駅出口で判断してください。
```

### Dongdaemun

```text
東大門
一般的な鉄道ルートの1つは、直通列車でソウル駅へ行き、そこから4号線でホテル最寄り駅へ向かう方法です。ただし荷物を持った乗り換えは残るため、宿泊先の近くに直通する空港バス停があるなら比較する価値があります。
```

### Gangnam

```text
江南
江南全域に共通するAREX乗り換え駅はありません。各駅停車から麻谷ナルや金浦空港で9号線へ乗り換えることはできますが、便利な駅は正確な住所とその後の接続で変わります。江南のホテルによっては、荷物を持って複数の鉄道を乗り継ぐより空港バスのほうが簡単です。
```

### Jamsil

```text
蚕室
AREXを使うと、蚕室まで少なくとも1回はまとまった距離の移動が残り、複数の鉄道区間になることもあります。荷物、子ども、移動制約がある場合は、適切な空港バスのほうが簡単なことがあります。
```

### Gimpo Airport

```text
金浦空港
各駅停車は金浦国際空港駅へ直接行きます。直通列車は停車しません。航空便の正しいターミナルや、その先の地下鉄路線まで、案内表示、エレベーター、徒歩に必要な時間を確保してください。
```

### KTX connection

```text
KTX乗り継ぎ
次の移動がソウル駅発のKTXなら、直通列車は合理的な空港鉄道の選択になりやすいです。ただし、AREXの到着時刻にそのままKTXへ乗れるわけではありません。深い位置にあるAREXホームから出て、駅構内を横断し、正しい全国鉄道ホームまで移動する時間が必要です。
```

### Map-app paragraph

```text
韓国の地図アプリで、ホテルの正確な位置、駅出口、最後の徒歩を確認してください。地区名だけでは乗り換えのしやすさは判断できません。
```

Linked phrase:

```text
韓国の地図アプリでホテル位置・駅出口・最後の徒歩を確認
```

### Stay decision paragraph

```text
AREXで行きやすいという理由だけで宿泊エリアを決める必要はありません。弘大と孔徳は各駅停車で直通、ソウル駅は直通列車と全国鉄道にもつながります。一方、明洞の一部ホテルは空港バスのほうが行きやすいことがあります。
```

### Internal CTA

```text
空港アクセスに便利なソウルの宿泊エリアを比較 →
```

---

## Section: How to Find AREX at Terminal 1 and Terminal 2

### H2

```text
第1・第2ターミナルでAREX駅を見つける方法
```

### Section answer

```text
T1とT2にはそれぞれ別の駅がありますが、到着ロビーからの基本的な流れは同じです。
```

### Flow figcaption

```text
到着ロビーから正しいAREXホームまで
```

### Ordered steps

```text
1. 入国審査、手荷物受取、税関を終え、一般到着ロビーへ出ます。
2. Airport Railroad / AREX の表示に従い、ターミナルの交通センター方面へ進みます。
3. 鉄道エリアまで進み、直通列車の入口と各駅停車の入口を確認します。
4. 正しい乗車券を購入するか、各駅停車を利用する場合は十分な残高のある交通カードを用意します。
5. 最新の発車案内を確認し、ソウル方面のホームであることを確かめてから入場します。
```

### Terminal notes

```text
第1ターミナル：鉄道エリアはT1交通センター内にあります。T2向けの案内ではなく、その建物への表示に従ってください。

第2ターミナル：T2交通センターとその駅へ進みます。先にT1へ移動する必要はありません。
```

### Terminal directions image alt

```text
仁川空港第1・第2ターミナルの到着ロビーからAREXホームまでの手順
```

### Figcaption

```text
第1・第2ターミナルからAREX駅までの行き方
```

### Closing body

```text
空港内の動線や一時的な通路は変わることがあるため、保存しておいた古いゲート番号や出口番号より、現地の最新表示のほうが確実です。一般到着ロビーへ出るまでの流れは、仁川空港到着ガイドで確認できます。
```

Linked phrase:

```text
仁川空港到着ガイド
```

---

## Section: Tickets for Express and All-Stop

### H2

```text
直通列車・各駅停車の乗車券
```

### Section answer

```text
2種類の列車は改札と乗車券の仕組みが異なります。T-moneyは各駅停車で使えますが、直通列車の予約の代わりにはなりません。
```

### Ticket guide image alt

```text
AREX直通列車の予約、駅発券、T-money、1回用交通カードの選び方
```

### Figcaption

```text
AREX直通列車・各駅停車の乗車券の種類
```

### H3 — Express tickets

```text
直通列車の乗車券
```

### Body 1

```text
公式サイトとアプリでは、当日から最大2か月先までの乗車日を検索できます。選んだ列車をカートに入れたあとは20分以内に支払いを完了する必要がありますが、これは出発20分前まで予約できることを保証する意味ではありません。支払い済みの乗車券にはQRコードがあり、列車、号車、指定席が表示されます。
```

### Body 2

```text
T1、T2、ソウル駅の直通列車券売機とカスタマー案内センターでも購入できます。AREXは、直通列車の乗車券購入に対応する海外発行カードとしてVisa、Mastercard、JCB、Diners Club、American Express、UnionPayを案内しています。
```

### H3 — All-Stop tickets

```text
各駅停車の乗車券
```

### Body 1

```text
T-moneyなどのプリペイド交通カードは、乗車に十分な残高があれば利用できます。各駅停車の改札でタッチして入り、目的地でタッチして出ます。
```

### Body 2

```text
交通カードを持っていない旅行者は、駅の券売機で1回用交通カードを購入し、移動中はそのカードを保管し、降車後に保証金を返金できます。公式の各駅停車案内では券売機での販売は確認できますが、海外発行カードが利用できるとは明確に書かれていません。現地で確認せず特定の支払い方法だけを前提にしないでください。
```

### Closing body

```text
公式の直通列車予約サイトには変更・払い戻し機能もあり、確定前に適用条件が表示されます。販売代理店の条件は運行会社の公式条件と異なる場合があります。
```

---

## Section: Seoul Station Is Often Only the Middle of the Journey

### H2

```text
ソウル駅は移動の途中にすぎないことが多い
```

### Section answer

```text
ソウル駅に着くことと、ホテルに着くことは同じではありません。
```

### Body

```text
AREXホームは大きな駅構内の深い位置にあります。地下鉄、KTXホーム、地上へ移動するには、長い通路、エレベーターやエスカレーター、そして荷物を持って案内表示を追う時間が必要になることがあります。
```

### H3

```text
地下鉄とホテルへの乗り継ぎ
```

### Body

```text
明洞や東大門の一部へは4号線が一般的な続き方で、1号線はほかの都心・東側方面へつながります。地下鉄の改札を通れば終わりではありません。進行方向、ホテル側の出口、最後の徒歩まで含めて、実際に楽なルートかが決まります。
```

### H3

```text
KTXと全国鉄道
```

### Body

```text
AREXの公表到着時刻に、すぐKTXへ乗れる状態になるわけではありません。AREXホームからの徒歩、エレベーターやエスカレーター、必要な乗車券確認、駅構内を横断して正しい全国鉄道ホームへ向かう時間を含めてください。
```

### Map app paragraph

```text
韓国の地図アプリでホテルの正確な位置、駅出口、最後の徒歩を確認してください。地区名だけでは乗り換えのしやすさは判断できません。
```

Linked phrase:

```text
韓国の地図アプリでホテル位置・駅出口・最後の徒歩を確認
```

### H3

```text
都心空港ターミナル
```

### Body

```text
ソウル駅都心空港ターミナルは出国時のサービスであり、仁川空港から到着した旅行者を助ける施設ではありません。直通列車を利用する当日出国の対象国際線旅客は、現在の航空会社・サービス条件を満たせば、ここで出国関連手続きを行える場合があります。
```

---

## Section: Luggage, Families and Accessibility

### H2

```text
荷物・家族旅行・バリアフリー
```

### Section answer

```text
列車内が快適でも、空港駅、乗り換え、ホテルまでの最後の区間で荷物を運ぶ作業はなくなりません。
```

### H3

```text
列車内
```

### Body

```text
直通列車は指定席と専用の荷物置き場があり、乗車中の状況を予測しやすいのが特徴です。各駅停車は通勤電車型の座席で、着席は保証されません。混雑時は、大きなスーツケース、ベビーカー、子ども連れでは負担が大きくなることがあります。
```

### H3

```text
列車を降りてからホテルまで
```

### Body

```text
すべての荷物を、到着ロビーから空港駅へ運び、AREXに載せ、ソウルで必要な乗り換えをし、最後はホテルまで運ぶ必要があります。車いす利用者、移動に制約がある旅行者、複数の荷物を扱う家族では、列車上の15分差よりエレベーター経路や乗り換え距離のほうが重要になることがあります。
```

### Closing paragraph

```text
その動線で何度も荷物を持ち上げたり長い通路を歩いたりするなら、ホテル近くに停まる空港バスや、荷物量に合った公式タクシーのほうが全体の負担を減らせる場合があります。
```

Linked phrases:

```text
空港バス
公式タクシー
```

### Image alt

```text
AREXではなく空港バス、タクシー、事前予約送迎を検討したほうがよい状況
```

### Figcaption

```text
AREXより空港バス、タクシー、事前予約送迎のほうが楽な場合
```

---

## Section: Late Arrival? Work Backward from the Train, Not the Landing Time

### H2

```text
深夜到着なら、着陸時刻ではなく最終列車から逆算する
```

### Section answer

```text
AREXは24時間運行ではなく、着陸時刻はホーム到着時刻でもありません。
```

### Main body

```text
現在の公式直通列車時刻表では、空港発の始発はT2が05:16、T1が05:24、最終はT2が22:40、T1が22:48です。各駅停車は別の時刻表で運行するため、古いスクリーンショットではなく、日付入りのAREX公式時刻表を確認してください。
```

Linked phrase:

```text
日付入りのAREX公式時刻表
```

### Ordered steps

```text
1. 到着ターミナルから出る最終列車の時刻を確認します。
2. そこからホームまでの徒歩と乗車券購入時間を逆算します。
3. さらに入国審査、手荷物受取、税関に必要な時間を見込みます。
4. その結果を予定着陸時刻と現実的な遅延余裕と比較します。
5. 実際にホームへ着ける時刻を過ぎても利用できる代替手段を用意します。
```

### Closing body

```text
最終列車に間に合わない場合は、まず公式の深夜バスやほかの空港バスを確認してください。目的地と時間に合う便がなければ、空港の公式タクシー乗り場か、待ち合わせ場所と遅延時の条件が明確な確認済み送迎を利用してください。
```

Linked phrases:

```text
空港バスの選択肢
空港の公式タクシー乗り場
```

---

## Section: Common AREX Mistakes

### H2

```text
AREXでよくある失敗
```

### Section answer

```text
列車そのものは難しくありません。問題は、AREXの駅に着くところまでしかルートを考えていないときに起こりがちです。
```

### Mistake list — exact order

```text
直通列車は弘大入口駅に停まりません。多くの弘大ホテルなら、各駅停車を使えばソウル駅まで行ってから西へ戻る必要がありません。

ホテルが近くない限り、ソウル駅は移動の終点ではありません。地下鉄の通路、出口、最後の徒歩にもそれぞれ時間がかかります。

直通列車と各駅停車は乗車券も改札も別です。間違ったエリアに入ると、コンコースまで戻る余計な移動が発生します。

T-moneyは各駅停車で使えますが、直通列車の乗車券にはなりません。直通列車には専用の予約済み乗車券または駅発券の乗車券が必要です。

T1とT2は別の駅です。案内と発車時刻は、実際に到着するターミナルに合わせて確認してください。

古い運賃表、時刻表のスクリーンショット、販売代理店の条件は最新でないことがあります。最終確認は運行会社の現在の公式ページを優先してください。

着陸時刻だけで深夜移動を計画すると、入国審査、手荷物受取、税関、駅までの徒歩、乗車券購入の時間が抜けます。

大きな荷物が複数あるからといって、鉄道が自動的に楽とは限りません。ホテル近くのバス停や公式タクシーなら、持ち上げや乗り換えの回数を減らせることがあります。

江南と鍾路は、1つの標準乗り換え駅でまとめるには広すぎます。ホテルの住所、接続路線、最後の徒歩で合理的なルートが変わります。
```

---

## Section: AREX Frequently Asked Questions

### H2

```text
AREX よくある質問
```

### Section answer

```text
フライト、ホテル、到着時刻が決まったあとに重要になりやすい実用的なポイントをまとめます。
```

### FAQ 1

**Q**

```text
仁川空港からソウルまでAREXはいくら？
```

**A**

```text
直通列車の大人販売運賃は₩13,000です。交通カードを使う各駅停車は、ソウル駅までT1から₩4,750、T2から₩5,350です。旅行前に最新の公式運賃を確認してください。
```

### FAQ 2

**Q**

```text
AREXでソウル駅まで何分かかる？
```

**A**

```text
直通列車はT1から43分、T2から51分です。各駅停車はT1から59分、T2から66分で、一部列車はさらに2～6分長くかかります。
```

### FAQ 3

**Q**

```text
AREX直通列車は各駅停車より速い？
```

**A**

```text
ソウル駅までなら速いです。ただし弘大入口駅、孔徳、その他の途中駅近くのホテルでは、戻る移動が不要な各駅停車のほうがホテルまで早く着くことがあります。
```

### FAQ 4

**Q**

```text
AREXでT-moneyは使える？
```

**A**

```text
T-moneyは各駅停車で使えます。直通列車には別の予約済み乗車券が必要です。
```

### FAQ 5

**Q**

```text
弘大へ行くならどのAREXに乗る？
```

**A**

```text
各駅停車は弘大入口駅へ直接行きます。直通列車は弘大を通過してソウル駅が終点なので、通常は西側へ戻る余計な移動が発生します。
```

### FAQ 6

**Q**

```text
AREXは24時間運行している？
```

**A**

```text
いいえ。特に遅い時間の便で到着する場合は、利用ターミナルと旅行日の公式時刻表を確認してください。
```

### FAQ 7

**Q**

```text
第1ターミナルのAREX駅はどこ？
```

**A**

```text
税関を出たら、一般到着ロビーからAirport Railroad / AREXの表示に従い、第1ターミナル交通センターと鉄道エリア方面へ進んでください。
```

### FAQ 8

**Q**

```text
第2ターミナルのAREX駅はどこ？
```

**A**

```text
税関を出たら、一般到着ロビーからAirport Railroad / AREXの表示に従い、第2ターミナル交通センターと鉄道エリア方面へ進んでください。
```

### FAQ 9

**Q**

```text
大きな荷物があってもAREXは使いやすい？
```

**A**

```text
直通列車には指定席と専用の荷物置き場がありますが、すべての荷物を空港駅やソウルでの乗り換え区間を通して運ぶ必要はあります。大きなスーツケースが複数あるなら、バスやタクシーのほうが荷物の持ち替えを減らせる場合があります。
```

### FAQ 10

**Q**

```text
AREXと空港バス、どちらが便利？
```

**A**

```text
AREXは鉄道の所要時間を読みやすく、駅からホテルまでの接続が簡単な場合に便利です。公式のバス停がホテル近くにあり、荷物を持った乗り換えを減らせるなら、空港バスのほうが楽なことがあります。
```

### FAQ 11

**Q**

```text
海外発行のクレジットカードでAREX乗車券を買える？
```

**A**

```text
直通列車について、AREXは対応する海外発行カードとしてVisa、Mastercard、JCB、Diners Club、American Express、UnionPayを案内しています。各駅停車については、公式案内で1回用交通カードを駅の券売機で販売していることは確認できますが、その券売機で海外発行カードが使えるとは明確に確認できません。
```

### FAQ 12

**Q**

```text
最終列車に乗り遅れたらどうする？
```

**A**

```text
まず公式の深夜バス・空港バスを確認してください。目的地と時間に合う便がなければ、空港の公式タクシー乗り場か、確認済みの事前予約送迎を利用してください。
```

**Schema note:** English source has no JSON-LD FAQ block. Keep visible FAQ only; do not create new schema in this Batch.

---

## Section: Official Sources and Related Guides

### H2

```text
公式情報・関連ガイド
```

### Section answer

```text
運賃、時刻表、対応する支払い方法、運行条件は変更されることがあります。旅行前の最終確認には、以下の公式ページを利用してください。
```

### Official source list — exact order

```text
AREX直通列車の案内 — 停車駅、所要時間、公式販売運賃、指定席、駅の場所。
AREX直通列車の時刻表 — 始発、最終、日付別の発車時刻。
AREX直通列車の乗車券案内 — オンライン・駅での購入、海外発行カード、QR乗車券、変更、払い戻し。
AREX各駅停車の案内 — 駅、乗換路線、所要時間、乗車券の種類。
AREX各駅停車の運賃表 — 距離制運賃、1回用交通カードの追加料金、返金可能な保証金。
仁川空港 鉄道交通案内 — 空港交通の全体像とターミナル案内。
仁川空港 都心空港ターミナル案内 — 直通列車の利用条件と出国時サービスの条件。
VISITKOREA 空港交通ガイド — AREXの列車種別と所要時間をまとめた公的機関による別の案内。
```

Exact linked titles, in source order:

```text
AREX直通列車の案内
AREX直通列車の時刻表
AREX直通列車の乗車券案内
AREX各駅停車の案内
AREX各駅停車の運賃表
仁川空港 鉄道交通案内
仁川空港 都心空港ターミナル案内
VISITKOREA 空港交通ガイド
```

### Related guides ARIA

```text
関連する韓国旅行ガイド
```

### H3

```text
次の計画へ
```

### Related guide links — exact order

```text
空港アクセス比較
空港バスガイド
仁川空港到着ガイド
仁川空港ガイド
T-moneyガイド
韓国の地図アプリガイド
公式タクシーガイド
```

---

## Common UI exact reuse

Header / navigation / language switcher / footer visible strings and ARIA must reuse the current CONTENT LOCKED Japanese Golden Sample exactly. Do not retranslate them in AREX.

Key reused examples:

```text
Korea Inside ホーム
メニューを開く
メインナビゲーション
楽しむ
旅行ガイド
空港
空港ガイド
到着ガイド
空港アクセス
AREXガイド
空港バスガイド
言語
韓国発
韓国人エディターが現地で執筆・確認する、実用重視の韓国旅行ガイドです。
公式情報、現地事情、独立した編集判断をもとに作成しています。
旅行を計画する
韓国で使う
アフィリエイト開示
プライバシーポリシー
事業者登録番号 462-39-01721
お問い合わせ：
```

The complete common UI source of truth remains the Japanese Golden Sample Approved Public Copy; the examples above are not a substitute for exact 83/83 reuse.

---

## Protected factual values

The following are examples of locked values and must not change during implementation:

- Express selling fare: ₩13,000 nonmember adult / ₩12,500 member adult / ₩9,500 child
- All-Stop transit-card fares:
  - T1 → Seoul Station ₩4,750
  - T1 → Hongik University ₩4,650
  - T2 → Seoul Station ₩5,350
  - T2 → Hongik University ₩5,250
- Single-use card surcharge: ₩100
- Refundable deposit: ₩500
- Express journey time: T1 43 min / T2 51 min
- All-Stop journey time: T1 59 min / T2 66 min, some services +2–6 min
- Express first airport departures: T2 05:16 / T1 05:24
- Express last airport departures: T2 22:40 / T1 22:48
- Express booking search window: today through maximum 2 months ahead
- Cart payment limit: 20 minutes
- Supported Express foreign-card networks listed in source: Visa / Mastercard / JCB / Diners Club / American Express / UnionPay
- Express does not stop at Hongik University / Gimpo Airport.
- All-Stop serves Hongik University, Gongdeok and Gimpo Airport.
- T-money works on All-Stop, not as an Express ticket.
- T1 and T2 have separate AREX stations.
- Seoul Station City Airport Terminal is a departure service, not an arrival service.

---

## Implementation lock

Codex must:

- use the English Production HTML as the structural source;
- reuse Japanese common UI exactly;
- implement only the approved Japanese user-facing language;
- preserve all HTML structure, section order, class, id, functional `data-*`, images, `srcset`, CSS, JS and scripts;
- preserve all internal/external href values except approved multilingual sibling routing required by the Localization Standard;
- preserve affiliate/tracking values exactly;
- add no new FAQ JSON-LD because the English source has none;
- apply JA self canonical and reciprocal existing-sibling hreflang according to current Production state;
- use Japanese sibling links only where the sibling is already Production COMPLETE; otherwise keep English fallback;
- add the JA URL to sitemap only as part of the approved Production implementation workflow;
- perform static QA before staging;
- stage only exact scope files;
- never use `git add .` or `git add -A`.

Codex must not:

- retranslate;
- rewrite;
- improve grammar;
- summarize or expand;
- change facts, fares, times, station roles or route judgment;
- change recommendation logic;
- add schema not present in the English source;
- modify common header/navigation/footer/common.js/common style.css system without separate user approval.

---

## QA Ledger — localization handoff

- English source reviewed: **FULL**
- English JSON-LD FAQ: **0**
- Visible FAQ: **12**
- Page-specific extracted user-facing nodes: **293**
- Common UI exact reuse target: **83 / 83**
- Missing planned section: **0**
- Fact/fare/time changes introduced by localization: **0 intended**
- HTML modifications in this localization step: **0**
- stage / commit / push / deploy in this localization step: **0**

## Approval status

**APPROVED PUBLIC COPY — CONTENT LOCKED**

**User approval recorded on 2026-09-24.**

Implementation is now **Codex exact implementation only**.
