# Korea Inside — Japanese Source-Position Correction Review Copy

- File: `Korea_Inside_JA_Source_Position_Correction_APPROVED_2026-09-29.md`
- Date: 2026-09-29
- Status: **APPROVED / CONTENT LOCKED FOR IMPLEMENTATION**
- Scope: 6 Japanese pages with confirmed source-position / rendering / head-parity defects
- Basis: Current `main` English source + current `main` Japanese source + JA Audit 3/5 and 4/5 findings
- User approval: **APPROVED 2026-09-29**
- Implementation status: **NOT IMPLEMENTED**
- Git / stage / commit / push / deploy: **0**
- Rule: This document corrects only the 6 already-open FIX pages below. It does not reopen PASS pages.

## Protected

Do not change:
- facts
- numbers / prices / dates / times
- attraction / hotel / area ordering
- recommendation hierarchy
- affiliate URLs / tracking
- canonical / hreflang except where separately approved
- images / srcset
- CSS / JS
- common header / navigation / footer
- page-family structure

---

# 1. `gongdeok-mapo-seoul-guide.html`

## 1A. One Good Meal Is Enough — replace the complete Japanese block

### Current problem

Japanese inline emphasis and `<br>` positions split:
- `麻浦`
- `または`
- `食事`

and the English source's four dinner alternatives are no longer readable as one coherent choice block.

### Replace from

`<h3>良い食事は1回で十分</h3>`

through the final paragraph immediately before `</div>` of that section with:

```html
<h3>良い食事は1回で十分</h3>
<p>孔徳・麻浦の魅力は、何軒「制覇」するかではなく、選択肢が多いことです。</p>
<p>現実的な夜なら、次のどれか1つで十分です。</p>
<p><strong>孔徳市場でジョン＋酒</strong><br>または <strong>チョッパルを夕食の主役にする</strong><br>または <strong>麻浦駅近くで豚カルビ</strong><br>または <strong>ソルロンタンや冷麺など昔ながらのソウルの食事</strong></p>
<p>そのあとは別のことをしてください。</p>
<p>孔徳が単なるレストラン一覧より面白くなるのは、<strong>飲み物、手を動かすクラス、短い散歩、別の小さな体験を食事の前後へ自然に入れられるからです。4回連続で食べる夜にする必要はありません。</strong></p>
```

Facts / hierarchy preserved:
- market jeon + drinks
- jokbal
- Mapo pork galbi
- seolleongtang / naengmyeon
- one proper meal is enough

---

## 1B. Simple Evening Flow — rebuild only this route block

### Current problem

The three English route options are mixed across Japanese nodes and `<br>` positions split:
- `麻浦`
- `体験`
- `ホテル`

### Replace the content under `<h3>シンプルな夜の流れ</h3>` through the two closing explanatory paragraphs with:

```html
<p>初めてなら、立ち寄り先を増やしすぎる必要はありません。</p>
<div class="gongdeok-route-flow">
  <div class="gongdeok-route-flow__option">
    <p><strong>選択肢1 — 市場の夜</strong></p>
    <p>孔徳市場<br>→ ジョンまたはチョッパル<br>→ 飲みたければ一杯<br>→ 森の道周辺を短く散歩<br>→ カフェまたはホテルへ戻る</p>
  </div>
  <div class="gongdeok-route-flow__option">
    <p><strong>選択肢2 — 麻浦で夕食</strong></p>
    <p>麻浦または孔徳に到着<br>→ 豚カルビ、ソルロンタンなど昔ながらの食事<br>→ 孔徳方向へ歩く<br>→ デザート、カフェ、または小さな体験を1つ<br>→ 夜を終える</p>
  </div>
  <div class="gongdeok-route-flow__option">
    <p><strong>選択肢3 — 体験から始める</strong></p>
    <p>デザートクラスなどの予約体験<br>→ きちんと夕食を1回<br>→ 近所を短く歩く<br>→ スパ、カフェ、またはホテルへ</p>
  </div>
</div>
<p>正確な順番そのものが目的ではありません。</p>
<p>すべての横丁、レストラン、体験を入れるために街を何度も往復しないことが重要です。</p>
```

---

## 1C. Stay judgment — replace only the audited literal `strong` wording

Current:
`孔徳は、一日中観光する場所としてより、寝る場所として強い旅行者もいます。`

Replace:
`孔徳は、一日中観光する場所というより、宿泊拠点として使いやすい旅行者がいます。`

Current:
`孔徳が強いのは、交通、食、落ち着いた拠点という長所を本当に使う旅行者です。`

Replace:
`孔徳が合うのは、交通・食・落ち着いた拠点という長所を実際に活かせる旅行者です。`

---

## 1D. Who should visit — naturalize the strongest-fit sentence

Current:
`孔徳・麻浦が最も強いのは、食事を観光の合間の休憩ではなく、一日の主な体験として使う人です。`

Replace:
`孔徳・麻浦が特に合うのは、食事を観光の合間の休憩ではなく、一日の主な体験として楽しみたい人です。`

### Simple decision line breaks

Replace the current broken two result blocks with:

```html
<p><strong>孔徳・麻浦を選ぶ：</strong></p>
<p>ローカルフード<br>＋ 体験1つ<br>＋ 落ち着いたソウルの夜</p>
<p><strong>孔徳・麻浦を外す：</strong></p>
<p>大きな観光地<br>＋ 大きなナイトライフ<br>＋ 詰まった初回旅行</p>
<p>この違いが判断のポイントです。</p>
```

Keep the following two existing paragraphs:
- `孔徳・麻浦は有名エリアと同じことをする必要がありません。`
- `違うことができるから価値があります。`

---

## 1E. One-way evening — replace the two route paragraphs

```html
<p><strong>孔徳駅</strong><br>→ 市場または予約体験<br>→ ローカルな夕食<br>→ 麻浦側または森の道側へ歩く<br>→ 夜を終える</p>
<p>または、</p>
<p><strong>麻浦駅</strong><br>→ 豚カルビまたは老舗食堂<br>→ 孔徳方向へ歩く<br>→ カフェ、一杯、または静かな締めくくり<br>→ 孔徳駅から帰る</p>
```

Keep:
- `最も重要な予定から方向を決めてください。`
- `この街に複雑な交通戦略は必要ありません。`

---

## 1F. Gongdeok / Mapo station distinction — replace the broken label block

Current broken:
`孔徳 = 市場＋周辺体験`
and
`<strong>麻</strong>浦 = 龍江<br><strong>洞</strong>の食・豚カルビ`

Replace the four paragraphs in this subsection with:

```html
<p>組み合わせやすい距離ですが、開始点は別です。</p>
<p>まず重要な側から始めます。</p>
<p><strong>孔徳</strong> = 市場＋周辺体験<br><strong>麻浦</strong> = 龍江洞の食・豚カルビ</p>
<p>そのあと一方向に動きます。</p>
```

---

# 2. `hongdae-vs-myeongdong.html`

## 2A. Hero short answer — replace both paragraphs

### Current defect
The Japanese second paragraph incorrectly contains the image-description text:
`ソウルの弘大と明洞の夜の雰囲気を比較した編集用イラスト`

That text belongs only in the image `alt` and `figcaption`.

### Replacement

```html
<article class="hm-practical-card">
  <h2>まず結論</h2>
  <p>宮殿、買い物、ソウル中心部の観光が中心の初回旅行なら、通常は明洞のほうが使いやすいです。カフェで過ごす時間、遅い夜、空港鉄道を何度も使う旅程なら、弘大のほうが合います。</p>
  <p>大きな荷物や静かな睡眠を重視する場合は、エリア名だけでは決まりません。AREXホームからホテルまでの実際の動線や、客室の向きまで確認すると選びやすくなります。</p>
</article>
```

Do not change the image `alt` or `figcaption`.

---

## 2B. At-a-glance table — Shopping / Myeongdong cell

Current:
`Kビューティーと旅行者向けの買い物に強い。`

Replace:
`Kビューティーと旅行者向けの買い物に便利。`

---

## 2C. First-trip paragraph

Current:
`通常は明洞のほうが合理的です。`

Replace the full first paragraph under `3〜5泊の初回ソウル旅行` with:

`通常は明洞のほうが使いやすいです。初めての旅行では、到着前に想像する以上に、市庁、鍾路、宮殿エリア、南山などソウル中心部を使う時間が増えやすいからです。中心部に泊まっても地下鉄移動は必要ですが、「毎朝まずソウルの真ん中へ戻る」回数を減らせます。`

---

## 2D. FAQ — visible FAQ and FAQPage JSON-LD must use identical corrected answers

### Shopping

Replace with:
`Kビューティーと旅行者向けの買い物なら明洞が便利で、ホテルが近ければ買った物を置きやすいです。カジュアルファッション、小さな店、カフェを混ぜた買い物なら弘大が合います。`

### Nightlife

Replace with:
`バー、ライブ音楽、遅い食事が旅行の通常パターンなら弘大のほうが向いています。ナイトライフが時々だけで、昼の観光のほうが重要なら明洞が使いやすいです。`

### Quiet

Replace with:
`エリア全体では、ナイトライフ中心ではない明洞がやや選びやすいです。ただし、どちらもエリア名だけで静かとは言えません。通り、大通りへの向き、客室の向きがエリア名より重要です。`

### Large luggage

Replace with:
`一概にどちらが楽とは言えません。弘大はAREXホームからホテルまでのルートが簡単なら便利で、明洞はホテル横の空港バス停が長い鉄道徒歩より楽なことがあります。`

### Is Myeongdong too touristy?

Replace with:
`旅行者向けの店やサービスが多く、人混みもあります。一方で、買い物、複数言語サービス、ホテルの選択肢が多いという便利さもあります。それを便利と感じるか、個性が薄いと感じるかは旅の優先順位次第です。`

---

## 2E. Final Myeongdong judgment

Current:
`名前が有名というだけで、西ソウルや漢江南側が主役の旅行に最適になるわけではありません。`

Replace:
`名前が有名というだけで、西ソウルや漢江南側が主役の旅行にも明洞が合うとは限りません。`

---

# 3. `hotels-near-gongdeok-station.html`

## 3A. Why Gongdeok intro

Current:
`空港アクセスが最も注目されますが、出張、長めの滞在、ソウル各地を横断する日にもその交通の便利さが生きると、孔徳を選ぶ理由が強くなります。`

Replace:
`空港アクセスが最も注目されますが、出張、長めの滞在、ソウル各地を横断する日にも同じ交通の便利さを使えるなら、孔徳を選ぶ理由がはっきりします。`

---

## 3B. Gongdeok vs Mapo intro

Current:
`交通ハブとしては孔徳が強く、ホテル自体、ファミリールーム、漢江側の立地を重視するなら麻浦の魅力が上がります。`

Replace:
`交通の選択肢は孔徳のほうが多く、ホテル自体、ファミリールーム、漢江側の立地を重視するなら麻浦の魅力が上がります。`

---

## 3C. Roynet wording

Current:
`広いデスク、ビジネスセンター、フィットネスルーム、24時間セルフランドリーは数泊すると実用性がはっきりします。`

Replace:
`広いデスク、ビジネスセンター、フィットネスルーム、24時間セルフランドリーは、数泊すると便利さを実感しやすい設備です。`

---

## 3D. Research note

Current:
`ホテルの役割判断は可能な限り公式の客室・施設ページを使用し`

Replace:
`ホテルの位置づけは可能な限り公式の客室・施設ページを基に判断し`

Full sentence after correction:

`2026年9月。交通情報は空港鉄道とソウル市の情報で確認しました。ホテルの位置づけは可能な限り公式の客室・施設ページを基に判断し、Gongdeok Stay MasilはVisit Seoulの公式掲載情報を使用しています。条件は変更されることがあるため、予約前に正確な客室とルートを確認してください。`

---

## 3E. Travel scenarios — replace the entire `<ol>` only

This removes the visible stray fragments:
- `。`
- `場合。`

outside the scenario-title `<p>` nodes.

```html
<ol>
  <li>
    <p class="hm-scenario-title">仁川空港到着後、ソウルに滞在する</p>
    <p>仁川空港から到着し、その後数日間ソウル各地を移動する予定なら、孔徳は最初の拠点として便利です。AREX一般列車（各駅停車）で直接エリアへ入り、チェックイン後は5号線と6号線も使えます。</p>
    <p>空港アクセスは重視するものの、毎晩ホテル前に弘大のナイトライフや人混みは必要ない場合に特に合います。</p>
  </li>
  <li>
    <p class="hm-scenario-title">汝矣島とソウル中心部に分かれる出張</p>
    <p>会議が一つの地区ではなく、汝矣島、ソウル中心部、ほかのビジネスエリアに分散しているなら、孔徳は現実的な中間地点になります。</p>
    <p>その場合、観光エリアに泊まって毎朝通勤するより、GLAD Mapo、Roynet、Shilla Stayのほうが使いやすいことがあります。ただし、会議がすべて江南なら孔徳の利点は大きく下がります。</p>
  </li>
  <li>
    <p class="hm-scenario-title">弘大に行くが弘大には泊まらない</p>
    <p>夕食、カフェ、買い物、夜の外出で弘大を楽しみたい一方、滞在全体を弘大に置く必要がない旅行者もいます。</p>
    <p>孔徳なら弘大を近くに保ちつつ、夜はより落ち着いたビジネス・住宅エリアへ戻れます。</p>
  </li>
  <li>
    <p class="hm-scenario-title">家族・グループで一つの拠点を使う</p>
    <p>3～4人ならSeoul GardenやLOTTE City Hotelの広めの客室タイプで一般的なホテル滞在を成立させられます。5～6人ならGongdeok Stay Masilが、1つのプライベート宿で寝室を分ける別の形を提供します。</p>
    <p>正しい選択は、どの施設が駅に最も近いかより、客室構成が実際にグループに合うかで決まります。</p>
  </li>
  <li>
    <p class="hm-scenario-title">空港前後の初日・最終泊</p>
    <p>空港アクセスを特に重視する韓国旅行の初日や最終泊では、孔徳を選ぶ理由があります。</p>
    <p>ただし、空港移動1回を楽にするためだけにホテルを移る必要はありません。ソウル滞在の残りに合うホテルにすでに泊まっているなら、孔徳へ1泊だけ移るのは、実際にスケジュールが明確に楽になる場合に限ります。</p>
  </li>
</ol>
```

---

# 4. `itaewon-travel-guide.html`

## 4A. Hannam / Hangangjin starting point

Current:
`夕食とナイトライフだけではない梨泰院を見たいなら、最も強いスタート地点です。`

Replace:
`夕食とナイトライフだけではない梨泰院を見たいなら、漢南・漢江鎮から始めるのが分かりやすいです。`

---

## 4B. Daytime ordering

Current:
`この順番が使いやすいのは、漢南は昼に強く、中央梨泰院は夕食・夜へ近づくほど使いやすくなるからです。`

Replace:
`この順番が使いやすいのは、漢南は昼に回りやすく、中央梨泰院は夕食・夜に近づくほど使いやすくなるからです。`

---

## 4C. Food reason

Current:
`梨泰院を別のソウルの街ではなくここで使う理由として、食はかなり強いです。`

Replace:
`梨泰院を選ぶ大きな理由の1つが食です。`

---

## 4D. Traveler-fit headings

Replace the five repeated headings:

- `強く合う — 食中心の旅行者`
- `強く合う — ムスリム旅行者`
- `強く合う — アート・デザイン・漢南ショッピング`
- `強く合う — 夜を大切にするカップル・グループ`
- `強く合う — リピーター`

with:

- `特に合う — 食中心の旅行者`
- `特に合う — ムスリム旅行者`
- `特に合う — アート・デザイン・漢南ショッピング`
- `特に合う — 夜を大切にするカップル・グループ`
- `特に合う — リピーター`

No body facts change.

---

## 4E. Taxi wording

Current:
`長い一日の最後に上り坂が旅を不快にするなら、短いタクシーが合理的です。`

Replace:
`長い一日の最後に上り坂を歩くのが負担なら、短い区間だけタクシーを使うのも有効です。`

---

## 4F. Evening choice block — replace the one broken `<p>`

```html
<p><strong>静かに：</strong>経理団<br>
<strong>街を歩く：</strong>解放村<br>
<strong>ナイトライフ：</strong>中央梨泰院のバー／パブクロール／クラブ<br>
<strong>もう十分：</strong>ホテルへ戻る</p>
```

This restores:
- 解放村
- ナイトライフ
- クラブ

as intact Japanese words.

---

## 4G. Remaining audited wording

Current:
`いいえ。漢南とLeeumには昼の強い理由があり、`

Replace:
`いいえ。漢南とLeeumには昼に訪れる理由があり、`

Current:
`坂、天気、疲労のトレードオフが悪ければ外してください。`

Replace:
`坂や天気、疲労との兼ね合いが悪ければ外してください。`

Current:
`強いルートは東から西へ移り、時間が進むほど予定を少し緩くしていきます。`

Replace:
`使いやすいルートは、東から西へ移り、時間が進むほど予定を少し緩くしていく形です。`

---

# 5. `jamsil-travel-guide.html`

## 5A. Weather block — replace complete content inside the prose block

```html
<p>蚕室は、大型アクティビティ同士が近い一方、天気の影響がそれぞれ違うため、組み直しやすいエリアです。</p>
<p><strong>雨の日：</strong><br>Aquarium → Mall → 屋内で食事。雨が止んだら石村湖を追加。</p>
<p><strong>晴れの日：</strong><br>AquariumまたはMall → 石村湖 → ソンリダンギル → あとでSeoul Sky。</p>
<p><strong>暑い夏の午後：</strong><br>最も暑い時間帯は屋内で過ごし、夕方に近づいてから湖へ。</p>
<p><strong>霞が強い日：</strong><br>展望台を一日の中心にしない。Seoul Skyは任意にし、景色に期待できないなら地上で時間を使う。</p>
<p>Seoul Skyの公式運営情報には、閉館前の入場締切など日付依存の内容があります。ガイドに印刷された固定時刻へ頼らず、行く前に現在のスケジュールを確認してください。</p>
```

---

## 5B. Aquarium judgment

Current:
`Aquarium自体がやりたいことの1つである場合に最も意味があります。`

Replace:
`Aquarium自体がやりたいことの1つなら、旅程に入れる価値があります。`

---

## 5C. Dinner judgment

Current:
`観光を終え、夕食そのものを最後の予定にしたいグループには強いです。`

Replace:
`観光を終え、夕食そのものを最後の予定にしたいグループに向いています。`

---

## 5D. Stay or visit

Current:
`旅行の理由がすでに蚕室にあるなら、強い宿泊エリアです。`

Replace:
`旅行の主な目的がすでに蚕室にあるなら、宿泊拠点として使いやすいエリアです。`

Current:
`一方、初回ソウルの主役が宮殿、北村、仁寺洞、明洞、弘大なら、自動的な拠点としては弱くなります。`

Replace:
`一方、初回ソウルの主役が宮殿、北村、仁寺洞、明洞、弘大なら、最初から蚕室を拠点にする必要性は下がります。`

---

## 5E. Who Jamsil suits

Current:
`ソウルのこの部分に、すでに重要な理由があるときに蚕室は最も強くなります。`

Replace:
`蚕室が特に合うのは、ソウル東南部にすでに重要な予定がある旅行者です。`

Keep the following examples unchanged:
- Lotte World
- concerts
- baseball
- Aquarium
- families benefiting from several nearby activities

---

## 5F. First-time-stay FAQ — visible FAQ and FAQPage JSON-LD must match

Replace answer with:

`初めての旅行では、最初から蚕室を標準の宿泊エリアにはしません。ロッテワールド、子ども向けアクティビティ、コンサート、スポーツ、ソウル東部が旅行の大きな部分なら拠点として使いやすいです。ソウル中心部の観光が主役の初回旅行なら、蚕室は一日訪問にするほうが簡単なことが多いです。`

---

# 6. `lotte-world-seoul.html`

## 6A. Head parity — remove Japanese-only meta description

Current English source has:
`<!-- Meta description awaits approved public copy. -->`

and **no** `<meta name="description">`.

Japanese currently adds a description target not present in English.

### Remove only:

```html
<meta name="description" content="ロッテワールドソウルの1Day・After4、マジックパス、人気アトラクション、待ち時間対策、子連れ、雨の日、チケット購入まで実用的に解説します。">
```

Keep:
`<!-- Meta description awaits approved public copy. -->`

Do not create a new English meta description in this correction batch.

---

## 6B. Quick decision — replace all four `<p>` blocks

```html
<p><strong>ロッテワールドを丸一日使うなら：</strong><br>
乗りたいものが複数ある、初訪問、または1本の長い行列で別の優先アトラクションを諦めたくない場合。</p>
<p><strong>After4を選ぶなら：</strong><br>
優先アトラクションを少なくしてよい、夜の雰囲気が主目的、またはロッテワールドが蚕室の一日の一部だけの場合。</p>
<p><strong>小さい子どもと一緒なら：</strong><br>
乗りたいものとMagic Passの価値を決める前に身長制限を確認します。目標乗車数より、体力と休憩を中心に計画してください。</p>
<p><strong>購入前に1つだけ覚えること：</strong><br>
先に乗りたいライドを決める。Magic Passはそのあと決める。</p>
```

---

## 6C. Must-rides — three-category block

Replace the content from `<h3>3つに分類する</h3>` through `最初のグループだけがルートを決めます。` with:

```html
<h3>3つに分類する</h3>
<p><strong>必須</strong><br>
長く待ってでも乗りたい、または他の予定を変えてでも守りたい。</p>
<p><strong>できれば乗りたい</strong><br>
乗りたいが、待ち時間がその日の予定に収まる場合だけ。</p>
<p><strong>ボーナス</strong><br>
タイミングが良ければ乗る。</p>
<p>このグループのために一日のルートを組み替える必要はありません。</p>
<p>ルートを決めるのは最初の「必須」グループだけです。</p>
```

---

## 6D. Must-rides — one audited evaluative sentence

Current:
`多くがMagic Islandなら、特に天気が不安定な日は屋外エリアへ早めの時間を取る強い理由になります。`

Replace:
`多くがMagic Islandなら、特に天気が不安定な日は屋外エリアへ早めに時間を取る理由になります。`

---

## 6E. Ride roles — rebuild the broken role list

Replace from the role-list `<p>` through the paragraph currently merged with `Pharaoh's Fury` with:

```html
<p><strong>屋外の代表スリル</strong> → Atlantis<br>
<strong>屋外コースターブロック</strong> → Comet Express<br>
<strong>回転・落下が強いスリル系</strong> → Gyro Swing / Gyro Drop<br>
<strong>屋内スリルの予備</strong> → French Revolution<br>
<strong>ファミリー／中程度の屋内</strong> → Flume Ride<br>
<strong>屋内テーマ系の予備</strong> → Pharaoh's Fury</p>
<p>激しいライドが苦手なら、Atlantisが有名でも優先する必要はありません。家族なら身長制限を確認したあと、この候補の多くを外すこともあります。</p>
<p>各ライドに、自分の計画の中で何のために乗るのかを決めておきます。</p>
```

Current:
`Magic Islandに複数あり、天気が安定しているなら、早めの屋外ブロックが合理的です。`

Replace:
`優先ライドがMagic Islandに複数あり、天気が安定しているなら、早めに屋外へ時間を取るほうが使いやすいです。`

---

## 6F. Magic Pass value wording

Current:
`次の条件がいくつか当てはまるほど、Magic Passを選ぶ理由は強くなります。`

Replace:
`次の条件がいくつか重なるほど、Magic Passを検討する価値が上がります。`

---

## 6G. Magic Pass purchase block — replace the broken lines

```html
<h3>2026年のMagic Pass購入方法</h3>
<p>Magic Passは<strong>通常のLotte World入場券とは別</strong>で、Lotte World Adventure公式アプリから販売されます。</p>
<p>現在の購入ルール：</p>
<p><strong>Premium事前購入</strong><br>
訪問2日前の0:00から、<strong>訪問前日11:59 PMまで予約可能</strong>です。</p>
<p><strong>当日購入</strong><br>
すべてのMagic Pass tierは、訪問当日<strong>8:30 AMからアプリで販売されることがあります</strong>。</p>
<p>数量限定で、早期売り切れの可能性があります。</p>
<p>計画上Magic Passが重要なら、午後に行って必要になったとき買えばいいと考えないでください。</p>
```

Keep the factual times unchanged.

---

## 6H. Getting there — replace the subway + address block

```html
<p>Lotte World Adventureは蚕室エリアと直結しており、地下鉄が使いやすいです。</p>
<p><strong>地下鉄2号線または8号線で蚕室駅へ。</strong>Lotte World Adventureへ行くなら、単に「Lotte」と書かれた表示ではなく、<strong>Exit 3／Lotte World側</strong>の案内を確認してください。</p>
<h3>テーマパークとLotte World Towerを混同しない</h3>
<p>名前が似ています。</p>
<p><strong>Lotte World Adventure</strong> = テーマパーク、<strong>240 Olympic-ro</strong>。<br>
<strong>Lotte World Tower / Mall</strong> = 別複合施設、<strong>300 Olympic-ro</strong>。</p>
<p>チケットがLotte World Adventureなら、Seoul SkyやLotte World Mallではなく遊園地の表示に従ってください。</p>
<h3>荷物がある場合</h3>
<p>空港から直接、またはホテル移動中なら、大きな荷物をパークへ持ち込まないほうが楽です。</p>
<p>複合施設内にロッカーはありますが、サイズと空き状況は変わります。</p>
<p>大型スーツケースなら、全部のロッカーに入ると想定せず先に保管方法を確認してください。基本は、その日に必要な荷物だけで行くことです。</p>
```

---

## 6I. Stay bridge — newly confirmed source-position mismatch inside this already-open FIX page

English source:
- kicker: `STAY GUIDE`
- body: hotel location affects airport access, luggage handling, subway time

Japanese currently moved the English body concept into the kicker and prepended `宿泊ガイド` to the body.

### Replace the bridge copy with:

```html
<p class="jamsil-stay-bridge__kicker">宿泊ガイド</p>
<h2 class="jamsil-stay-bridge__title" id="lotte-world-stay-bridge-transport-title">まだソウルの宿泊エリアを決めていない？</h2>
<p class="jamsil-stay-bridge__body">ホテルの場所は、空港アクセス、荷物の扱いやすさ、市内を移動する地下鉄時間に影響します。</p>
<a class="jamsil-stay-bridge__link" href="/ja/accommodation.html">自分に合う宿泊エリアを見る</a>
```

This is a concrete source-position repair found while preparing the correction copy. It does not expand the page scope.

---

# 7. Review checklist before approval

The user should approve this Review Copy as one correction set.

After approval, implementation must verify:

## Structure
- Gongdeok route-flow: 3 option cards preserved
- Gongdeok one-way route: no Japanese word split
- Hongdae vs Myeongdong hero: image-description removed only from body paragraph
- Gongdeok hotel scenarios: no text outside scenario-title/body `<p>`
- Itaewon evening choice: 4 intact lines
- Jamsil weather: 4 weather labels in correct paragraphs
- Lotte World: all corrected `<br>` boundaries intact
- Lotte World Japanese-only meta description removed
- Lotte World Stay bridge source mapping restored

## Content
- facts unchanged
- numbers unchanged
- times unchanged
- prices unchanged
- recommendation hierarchy unchanged
- attraction/hotel order unchanged
- affiliate/tracking unchanged
- visible FAQ and JSON-LD mirrors remain identical where applicable

## Approval state
This correction copy is approved and content-locked for exact implementation.
Do not change wording, facts, numbers, recommendation logic, or scope during implementation without new user approval.
