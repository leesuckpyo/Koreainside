# Korea Inside — Japanese Wording-Only Correction Review Copy

- File: `Korea_Inside_JA_Wording_Only_Correction_Review_Copy_2026-09-29.md`
- Date: 2026-09-29
- Status: **REVIEW COPY — AWAITING USER APPROVAL**
- Scope: 19 Japanese pages classified as wording-only FIX
- Basis:
  - JA Audit Batch 1–5
  - current English `main`
  - current Japanese `main`
- Structural rebuild: **0**
- Source-position repair: **0**
- HTML implementation: **NOT STARTED**
- Git / stage / commit / push / deploy: **0**

## Scope lock

This Review Copy does **not** reopen:
- 33 PASS pages
- the 6 source-position/head-repair pages already implemented

Only the 19 wording-only FIX pages below are in scope.

## Global protection

Do not change:
- facts
- numbers
- prices
- dates / times
- recommendation hierarchy
- area / hotel / attraction ordering
- affiliate URLs / tracking
- canonical / hreflang
- image / srcset
- class / id / data-*
- CSS / JS
- schema structure

Where a visible FAQ answer has a FAQPage JSON-LD mirror, both must receive the same corrected Japanese wording.

Natural uses that are **not** defects remain unchanged, including examples such as:
- `旅行者向けの雰囲気が強い`
- `国際色が強い`
- `存在感が強い`
- `強い雨`
- `強い寒さ`
- literal automatic behavior such as a balance or room condition that does not happen `自動的に`

---

# 1. `accommodation.html`

Apply only these corrections.

1. Current:
`大きな荷物と到着・出発の楽さが重要なら、ソウル駅や麻浦／孔徳が強くなります。`

Replace:
`大きな荷物と到着・出発の楽さが重要なら、ソウル駅や麻浦／孔徳が使いやすくなります。`

2. Current:
`最も安全な基本候補になります。`

Replace:
`最も無難な第一候補になります。`

3. Current:
`観光の雰囲気より、旅行の実務を簡単にするエリアです。空港移動、鉄道利用、到着・出発日に大きなスーツケースがある場合に特に強くなります。`

Replace:
`観光の雰囲気より、移動や到着・出発を簡単にしやすいエリアです。空港移動、鉄道利用、到着・出発日に大きなスーツケースがある場合に特に使いやすくなります。`

4. Current:
`利便性を優先するなら明洞は今も強いです。`

Replace:
`利便性を優先するなら明洞は今も有力な候補です。`

5. Current:
`旅程の中に「漢江南側へ行く理由」がすでにあるときに強い宿泊エリアです。`

Replace:
`旅程の中に「漢江南側へ行く理由」がすでにあるときに、宿泊先として使いやすいエリアです。`

6. Current:
`伝統文化中心の旅程と、静かな夜を求める旅行者には強い代替です。`

Replace:
`伝統文化中心の旅程と、静かな夜を求める旅行者には有力な候補です。`

7. Current:
`弘大の中心に泊まらず、空港アクセスを重視する旅行者にとって強い代替です。`

Replace:
`弘大の中心に泊まらず、空港アクセスを重視する旅行者にとって有力な候補です。`

8. FAQ / FAQPage mirror:

Current:
`はい。特に空港アクセス、KTX、大きな荷物が重要なら強い選択です。`

Replace:
`はい。特に空港アクセス、KTX、大きな荷物が重要なら有力な選択肢です。`

Do not change contextual `旅行者向けの便利さが強い`-type descriptive wording unless listed above.

---

# 2. `best-area-for-airport-access-seoul.html`

1. H2 current:
`ソウル駅：空港以外の乗り換えも重なるときに強い`

Replace:
`ソウル駅：空港以外の乗り換えも重なるときに便利`

2. Current:
`空港移動と別の交通上の課題が重なると、ソウル駅が最も強い選択になります。`

Replace:
`空港移動と別の交通上の課題が重なると、ソウル駅が最も有力な候補になります。`

3. Current:
`明洞がより強い選択になるのは次のような場合です。`

Replace:
`明洞のほうが使いやすくなるのは次のような場合です。`

4. FAQ / mirror current:
`AREX直通列車、KTX、重い荷物を重視するならソウル駅が強いです。`

Replace:
`AREX直通列車、KTX、重い荷物を重視するならソウル駅が特に便利です。`

5. Current:
`ソウル駅にはAREX直通列車とKTXがあるため、その後の鉄道移動に強いです。`

Replace:
`ソウル駅にはAREX直通列車とKTXがあるため、その後の鉄道移動にも便利です。`

---

# 3. `best-area-for-couples-seoul.html`

Table / body only.

1. Current:
`漢江より南側の移動に強い`

Replace:
`漢江より南側を移動しやすい`

2. Current:
`ソウル南東部の予定に強い`

Replace:
`ソウル南東部の予定が多い旅に向く`

3. Current:
`旅程の中に江南を拠点にする明確な理由がある場合に強い選択肢です。`

Replace:
`旅程の中に江南を拠点にする明確な理由がある場合に有力な候補です。`

Keep `旅行者向けの雰囲気が強い` unchanged.

---

# 4. `best-area-for-families-seoul.html`

Current:
`大きな荷物、空港アクセス、静かな夜を優先するなら麻浦／孔徳が強くなります。`

Replace:
`大きな荷物、空港アクセス、静かな夜を優先するなら麻浦／孔徳がより使いやすくなります。`

Keep `予定変更に強い拠点` unchanged.

---

# 5. `best-area-for-first-time-visitors-seoul.html`

1. Current:
`夜の雰囲気より、大きな荷物と到着・出発の楽さを重視するならソウル駅や麻浦／孔徳が強くなります。`

Replace:
`夜の雰囲気より、大きな荷物と到着・出発の楽さを重視するならソウル駅や麻浦／孔徳が使いやすくなります。`

2. Current:
`ソウル市内での分泊が合理的なのは、旅の性格自体が大きく変わる場合です。`

Replace:
`ソウル市内で分泊を検討するのは、旅の性格自体が大きく変わる場合です。`

3. Table current:
`交通接続が強い`

Replace:
`交通の選択肢が多い`

4. Table current:
`雰囲気より実用性`

Replace:
`街の雰囲気より移動のしやすさ`

5. Table current:
`南側では強い`

Replace:
`南側の予定に向く`

6. Current:
`宮殿、伝統街、昔のソウル中心部に興味がある旅行者に強いです。`

Replace:
`宮殿、伝統街、昔のソウル中心部に興味がある旅行者に向いています。`

7. Current:
`「高級エリアだから」ではなく、実際の旅程に理由がある場合に強い拠点です。`

Replace:
`「高級エリアだから」ではなく、実際の旅程に理由がある場合に有力な拠点です。`

8. FAQ / mirror current:
`多くの家族には明洞が使いやすく、ロッテワールドが旅程の大きな部分なら蚕室も強いです。`

Replace:
`多くの家族には明洞が使いやすく、ロッテワールドが旅程の大きな部分なら蚕室も使いやすいです。`

9. Final current:
`夜の雰囲気より、大きな荷物や到着・出発の実務が重要ならソウル駅または麻浦／孔徳。`

Replace:
`夜の雰囲気より、大きな荷物や到着・出発のしやすさが重要ならソウル駅または麻浦／孔徳。`

---

# 6. `best-area-for-luxury-hotels-seoul.html`

1. H2 current:
`高級ホテルに泊まるなら、どのエリアが意味を持つ？`

Replace:
`高級ホテルに泊まるなら、どのエリアが自分の旅に合う？`

2. Current:
`夕食や夜の予定まで近くで完結するほど、高い宿泊費を払う意味が出てきます。`

Replace:
`夕食や夜の予定まで近くで完結するほど、高い宿泊費にも納得しやすくなります。`

3. Table:
`ソウル南東部の予定に強い`
→ `ソウル南東部の予定に向く`

4. Table:
`乗り換えや移動の多い旅に強い`
→ `乗り換えや移動の多い旅で使いやすい`

5. Table:
`鍾路・王宮方面の動線に強い`
→ `鍾路・王宮方面へ動きやすい`

6. Table:
`食事中心の滞在に強い`
→ `食事中心の滞在に向く`

7. Current:
`旅行の予定がすでに漢江より南側にまとまっているなら、江南は高級ホテル拠点として最も理由を作りやすいエリアです。`

Replace:
`旅行の予定がすでに漢江より南側にまとまっているなら、江南は高級ホテルの立地に追加料金を払う理由が最もはっきりしやすいエリアです。`

8. Current:
`つまり江南は、高い立地に払う理由が旅程の中にはっきりある場合に最も強い選択です。`

Replace:
`つまり江南は、高い立地に払う理由が旅程の中にはっきりある場合に特に有力な候補です。`

9. Current:
`ソウル南東部とロッテ複合施設がすでに旅程の中心なら、蚕室の高い宿泊費に意味が出ます。`

Replace:
`ソウル南東部とロッテ複合施設がすでに旅程の中心なら、蚕室に高い宿泊費を払う理由がはっきりします。`

10. FAQ / mirror current:
`プレミアムショッピングの総合候補としては江南が強く、`

Replace:
`プレミアムショッピングの総合候補としては江南が有力で、`

11. Final current:
`江南が最も理由を作りやすい高級ホテル拠点です。`

Replace:
`江南は、高級ホテルの立地に追加料金を払う理由を見つけやすい拠点です。`

12. Final current:
`ソウル駅・南大門のほうが賢い高級滞在になることもあります。`

Replace:
`ソウル駅・南大門のほうが納得しやすい高級滞在になることもあります。`

Keep literal automatic-condition warnings about lounge, check-in, pool/spa and restaurant reservations unchanged.

---

# 7. `best-area-for-nightlife-seoul.html`

1. Current:
`明洞、麻浦・孔徳、ソウル駅は、ナイトライフが旅行の一部にすぎない場合に意味が出ます。`

Replace:
`明洞、麻浦・孔徳、ソウル駅は、ナイトライフが旅行の一部にすぎない場合にも候補になります。`

2. Current:
`梨泰院で何夜も過ごす予定があるほど、ここに泊まる意味が出ます。`

Replace:
`梨泰院で何夜も過ごす予定があるほど、ここに泊まる利点が大きくなります。`

3. Current:
`明洞はソウルで最も強いナイトライフエリアではありませんが、`

Replace:
`明洞はソウルを代表するナイトライフエリアではありませんが、`

4. Current:
`終電前は地下鉄、さらに遅い時間はタクシーで戻ることを、昼の便利な立地と引き換える合理的なトレードオフにできます。`

Replace:
`昼の便利な立地を優先するなら、終電前は地下鉄、さらに遅い時間はタクシーで戻る形でも十分受け入れやすいです。`

5. FAQ / mirror current:
`江南は、上質なナイトライフと漢江より南側の昼の予定が重なる場合に最も意味があります。`

Replace:
`江南は、上質なナイトライフと漢江より南側の昼の予定が重なる場合に特に使いやすいです。`

6. FAQ / mirror current:
`ホテルのすぐ外にバーやクラブがあることより、中心部観光や買い物に強いエリアです。`

Replace:
`ホテルのすぐ外にバーやクラブがあることより、中心部観光や買い物に便利なエリアです。`

7. Related card current:
`夜の活気に強い西側の拠点`

Replace:
`夜の活気を楽しみやすい西側の拠点`

8. Final current:
`上質な夜とソウル南部の予定がすでに重なるなら江南がより強くなります。`

Replace:
`上質な夜とソウル南部の予定がすでに重なるなら江南のほうが合いやすくなります。`

Keep descriptive `国際色がより強い` unchanged.

---

# 8. `best-area-for-shopping-seoul.html`

1. Current:
`百貨店、COEX、高級ブランドを中心にするなら江南がより強い拠点です。`

Replace:
`百貨店、COEX、高級ブランドを中心にするなら江南がより使いやすい拠点です。`

2. Current:
`歴史地区の中心部に泊まることより、買い物の性格を優先するときに強くなる選択です。`

Replace:
`歴史地区の中心部に泊まることより、買い物の性格を優先するときに候補になるエリアです。`

3. Table current:
`ソウル全体を回る拠点としては弱め`

Replace:
`ソウル全体を回る拠点にはやや不向き`

4. Current:
`漢江より南側で何日も買い物する予定なら、江南に泊まる意味があります。`

Replace:
`漢江より南側で何日も買い物する予定なら、江南に泊まる利点があります。`

5. Current:
`ソウル南東部がすでに旅程の大きな部分を占める場合に宿泊拠点として強くなります。`

Replace:
`ソウル南東部がすでに旅程の大きな部分を占める場合に宿泊拠点として使いやすくなります。`

6. Final current:
`プレミアムショッピングを最優先するなら江南がより強くなります。`

Replace:
`プレミアムショッピングを最優先するなら江南のほうが合いやすくなります。`

7. Final current:
`東大門、聖水、蚕室は、それぞれの買い物スタイルが数日の旅程を形作るほど重要な場合に宿泊地として意味が出ます。`

Replace:
`東大門、聖水、蚕室は、それぞれの買い物スタイルが数日の旅程を形作るほど重要な場合に、宿泊地として選ぶ理由がはっきりします。`

Keep descriptive `旅行者向けの色が強く` unchanged.

---

# 9. `best-area-for-solo-travelers-seoul.html`

1. Quick answer current:
`ソウル駅と麻浦／孔徳は、雰囲気より空港移動、荷物、到着日を簡単にすることに強みがあります。`

Replace:
`ソウル駅と麻浦／孔徳は、雰囲気より空港移動、荷物、到着日の負担を減らしたい人に向いています。`

2. Current:
`空港移動や荷物の楽さを優先するならソウル駅や麻浦／孔徳が強くなります。`

Replace:
`空港移動や荷物の楽さを優先するならソウル駅や麻浦／孔徳が使いやすくなります。`

3. Current:
`ホテルの外にカフェやナイトライフを求めるなら別エリア、到着・出発の簡単さを優先するなら強いです。`

Replace:
`ホテルの外にカフェやナイトライフを求めるなら別エリア、到着・出発の簡単さを優先するなら使いやすいです。`

4. Current:
`旅程に明確な南側の理由があるときに強い拠点です。`

Replace:
`旅程に明確な南側の理由があるときに有力な拠点です。`

5. FAQ / mirror current:
`はい。中心部にあり分かりやすく、食事と観光を組みやすいため、初めての一人旅の拠点として簡単です。`

Replace:
`はい。中心部にあり分かりやすく、食事と観光を組みやすいため、初めての一人旅の拠点として選びやすいです。`

Keep `旅行者向けの性格が強い` unchanged.

---

# 10. `card-declined-korea.html`

One sentence only.

Current:
`問題がカードについて回るのか、場所について回るのかを見る。`

Replace:
`同じカードが場所を変えても失敗するのか、特定の店や端末だけで失敗するのかを確認します。`

Keep the following sentence unchanged:
`1台のキオスクだけで失敗するのと、複数の有人店舗で同じカードが失敗するのは意味が違います。`

---

# 11. `dongdaemun-travel-guide.html`

Only 3 cleanup points.

1. H1 current:
`ソウル・東大門（トンデムン）観光ガイド 2026 ：DDP・市場・ナイトショッピング`

Replace:
`ソウル・東大門（トンデムン）観光ガイド 2026：DDP・市場・ナイトショッピング`

2. Current:
`まず「時間帯」を選びます。その時間に行く意味がある場所だけに絞り込めます。`

Replace:
`まず「時間帯」を選びます。その時間帯に向いている場所だけに絞れます。`

3. Current:
`<strong>一般向け：</strong> 自分で使うものを買う。<br><strong>卸売：</strong> 他の店へ商品を供給する売り手と買い手の商取引の場で買う。`

Replace:
`<strong>一般向け：</strong> 自分で使うものを買う。<br><strong>卸売：</strong> 他の店へ商品を供給する売り手と買い手が取引する、業者向けの商環境で買う。`

Do not alter the following retail/wholesale conditions list.

---

# 12. `gangnam-travel-guide.html`

1. Current:
`韓国ファッション、K-pop、デザイン、目的地型カフェが漢江の南へ来た理由なら、別ルートを使います。`

Replace:
`韓国ファッション、K-pop、デザイン、わざわざ訪れたいカフェが漢江の南へ来た理由なら、別ルートを使います。`

2. Current:
`ここにある何かが「実際にソウルでしたかったこと」と一致するときに、江南の価値は最も強くなります。`

Replace:
`ここにある何かが「実際にソウルでしたかったこと」と一致するときに、江南へ時間を使う理由が最もはっきりします。`

3. Current:
`韓国ブランド、カフェ、ギャラリー、K-pop、ビューティー、特定レストランにすでに関心があるときに意味があるルートです。`

Replace:
`韓国ブランド、カフェ、ギャラリー、K-pop、ビューティー、特定レストランにすでに関心がある人向けのルートです。`

4. Current phrase:
`昼食を取れる実用的な場所`

Replace:
`昼食を取りやすい場所`

5. Current:
`COEXより、現在の韓国ファッション、カフェ、デザイン、K-pop、目的地型ショッピングを重視する旅行者向けです。`

Replace:
`COEXより、現在の韓国ファッション、カフェ、デザイン、K-pop、買い物そのものを目的にしたい旅行者向けです。`

6. H3 current:
`K-Star Roadは「ファンの層」として使う`

Replace:
`K-Star RoadはK-popファン向けの立ち寄り先として使う`

7. Current phrase:
`一部のK-pop関心には強いエリアです。`

Replace:
`K-popに関心がある人にも向いています。`

8. Current:
`なければ狎鴎亭で終えても合理的です。`

Replace:
`なければ狎鴎亭で終えても構いません。`

9. Current:
`その理由がないなら、このガイドでは島山／狎鴎亭側のほうが役割が明確です。`

Replace:
`その理由がないなら、このガイドでは島山／狎鴎亭側を優先する理由がはっきりします。`

10. Current:
`韓国ブランド、旗艦店、現在のリテール文化が旅の一部ならこちらが強いです。`

Replace:
`韓国ブランド、旗艦店、現在のリテール文化が旅の一部ならこちらが向いています。`

11. Any second occurrence of `目的地型カフェ` used in the same editorial sense:

Replace:
`わざわざ訪れたいカフェ`

12. Current:
`到着前に「どのCOEX」を使うか決めてください。`

Replace:
`到着前に、COEXで何をするか決めてください。`

13. Current:
`江南は、ソウル南部・南東部中心の旅行では非常に良い拠点になり得ます。ただし初回旅行の自動的な標準ではありません。`

Replace:
`江南は、ソウル南部・南東部中心の旅行では非常に良い拠点になり得ます。ただし、初回旅行で最初から選ぶ定番拠点ではありません。`

Do not alter the two approved route sequences or route stops.

---

# 13. `hongdae-travel-guide.html`

1. Current:
`その場合は、ここに泊まるより午後から夜に一度訪れるほうが合理的です。`

Replace:
`その場合は、ここに泊まるより午後から夜に一度訪れるほうが使いやすいです。`

2. Current:
`これは弘大に宿泊する強い理由のひとつです。`

Replace:
`これは弘大に宿泊する大きな理由のひとつです。`

3. Current:
`だから、過ごしやすい天気の日には強い追加先になります。`

Replace:
`だから、過ごしやすい天気の日には立ち寄りやすい追加先になります。`

4. Current:
`大雨、厳しい暑さ、強い寒さ、全員の足がすでに疲れている日には、弱い追加先です。`

Replace:
`大雨、厳しい暑さ、強い寒さ、全員の足がすでに疲れている日には、無理に追加する必要はありません。`

5. Current:
`買い物、焼肉、フォトブース、K-POP体験、カラオケ、路上ライブ、ライブミュージック、バー、クラブ。全員で考えると、どれも合理的に見えます。`

Replace:
`買い物、焼肉、フォトブース、K-POP体験、カラオケ、路上ライブ、ライブミュージック、バー、クラブ。全員の希望を並べると、どれも予定に入れたくなります。`

Keep literal `自動的` wording about language support, weather failure or family suitability unchanged unless separately listed.

---

# 14. `hotels-near-seoul-station.html`

1. Current:
`ソウルだけを回る初めての旅行で、ソウル駅を自動的な第一候補にはしません。`

Replace:
`ソウルだけを回る初めての旅行で、ソウル駅を最初から第一候補にはしません。`

2. Current:
`ソウル駅が旅程の中で実際に役割を持つとき、このエリアに泊まる意味が生まれます。`

Replace:
`ソウル駅が旅程の中で実際に必要になるとき、このエリアに泊まる利点が出てきます。`

3. Current:
`すべてのホテルでAREXが自動的に最適とは限りません。`

Replace:
`すべてのホテルでAREXがいつでも最適とは限りません。`

---

# 15. `insadong-travel-guide.html`

1. Current:
`初めてのソウルなら、仁寺洞は大きな歴史地区ルートの中に入れるとさらに強くなります。`

Replace:
`初めてのソウルなら、仁寺洞は大きな歴史地区ルートの中に入れるとさらに組みやすくなります。`

2. Current:
`仁寺洞は、一日の真ん中でペースを落としたいときに強い街です。`

Replace:
`仁寺洞は、一日の真ん中でペースを落としたいときに使いやすい街です。`

3. Current:
`景福宮 → 北村 → 仁寺洞 → 曹渓寺 → 夜の選択1つ のほうが強い一日です。`

Replace:
`景福宮 → 北村 → 仁寺洞 → 曹渓寺 → 夜の選択1つ のほうが充実した一日になります。`

4. Current:
`初回旅行者には安国駅を基本にしやすいですが、その日の他の予定で決めるほうが合理的です。`

Replace:
`初回旅行者には安国駅を基本にしやすいですが、その日の他の予定に合わせて決めるほうが自然です。`

5. Current:
`茶屋1軒 → しっかり座る → 次へ が仁寺洞の時間を強くします。`

Replace:
`茶屋1軒 → しっかり座る → 次へ のほうが、仁寺洞で過ごす時間にゆとりが生まれます。`

6. H3 current:
`ベジタリアン旅行者には意外に強い選択肢がある`

Replace:
`ベジタリアン旅行者にも意外と選択肢がある`

7. Current:
`ギャラリー、陶器店、文房具、工芸スペースが本当に気になったとき横道へ入る使い方が最も強いです。`

Replace:
`ギャラリー、陶器店、文房具、工芸スペースが本当に気になったときに横道へ入るのが最も自然です。`

8. Current:
`大きな歴史地区の一日に入れると強いです。`

Replace:
`大きな歴史地区の一日に組み込むと使いやすいです。`

9. Current:
`仁寺洞は昼〜早い夕方が強いです。`

Replace:
`仁寺洞は昼〜早い夕方に向いています。`

Keep ordinary contextual `役割` wording not listed above.

---

# 16. `myeongdong-travel-guide.html`

1. H3 current:
`買う目的があると明洞は強い`

Replace:
`買いたいものがあるなら明洞は使いやすい`

2. Current:
`明洞を含む「ソウル中心部の1日」として組むなら、1日使う意味があります。`

Replace:
`明洞を含む「ソウル中心部の1日」として組むなら、1日かけても無理はありません。`

3. H3 current:
`ブランド直営店にも役割がある`

Replace:
`ブランド直営店を選ぶ理由もある`

4. Current:
`特定の韓国ブランドが買い物の大きな目的なら、直営店でより広いラインナップ、異なるセット、ブランド独自のサービスを確認する意味があります。`

Replace:
`特定の韓国ブランドが買い物の大きな目的なら、直営店でより広いラインナップ、異なるセット、ブランド独自のサービスを確認する価値があります。`

5. H3 current:
`麺なら明洞餃子には明確な役割がある`

Replace:
`麺を食べるなら明洞餃子は目的がはっきりしている`

6. Current:
`役割が分かりやすい店です。`

Replace:
`どんなときに使いやすいかが分かりやすい店です。`

7. H3 current:
`高いコースを自動的に選ばない`

Replace:
`高いコースを最初から選ばない`

8. Current phrase:
`明洞との役割分担`

Replace:
`明洞とどう使い分けるか`

9. Current:
`だからといって、自動的に行くべき場所ではありません。`

Replace:
`だからといって、必ず行くべき場所ではありません。`

10. Current:
`晴れた午後遅くや夜なら、「ソウル定番だから」より強い理由になります。`

Replace:
`晴れた午後遅くや夜なら、「ソウル定番だから」ではなく、景色を楽しむために行く理由がはっきりします。`

11. Current:
`買い物 → ホテル → 荷物を置く／少し休む → 南山 のほうが、元の予定に休憩がないからという理由で全部持って坂を上るより合理的です。`

Replace:
`買い物 → ホテル → 荷物を置く／少し休む → 南山 のほうが、元の予定に休憩がないからという理由で全部持って坂を上るより楽です。`

12. Current:
`その便利さには意味があります。`

Replace:
`初めての旅行では、その便利さ自体が助けになります。`

13. Current:
`明洞そのものが主目的でなくても、拠点としては強いです。`

Replace:
`明洞そのものが主目的でなくても、拠点として使いやすいです。`

14. Current:
`買い物、食事、公演、中心部ホテルなど、明洞が効率よく解決することだけに使い、その役割が終わったら移動してください。`

Replace:
`買い物、食事、公演、中心部ホテルなど、明洞が効率よく解決できることに使い、必要なことを済ませたら移動してください。`

15. Final current:
`Kビューティー、まとまった買い物、中心部での休憩、公演、便利なホテル拠点として役立つなら、その役割に使ってください。役割が終わったら、次へ移動します。`

Replace:
`Kビューティー、まとまった買い物、中心部での休憩、公演、便利なホテル拠点として役立つなら、必要な場面で使ってください。目的を果たしたら、次へ移動します。`

Keep:
- `観光客向けの要素が強い`
- `強い雨`
unchanged.

---

# 17. `seongsu-travel-guide.html`

1. H3:
`ファッションとKビューティーが特に強い`
→ `ファッションとKビューティーを特に楽しみやすい`

2. Current:
`韓国ビューティーブランドが今どのように見せられているかまで興味がある人には、商品を買うだけ以上の意味があります。`

Replace:
`韓国ビューティーブランドが今どのように見せられているかまで興味がある人なら、商品を買うだけでなく、ブランドの見せ方まで楽しめます。`

3. Current:
`初めてなら、半日でも十分強い体験になる旅行者が多いです。`

Replace:
`初めてなら、半日でも十分楽しめる旅行者が多いです。`

4. Current:
`ファッション、Kビューティー、デザイン、ポップアップが旅行の主要目的なら、丸一日も合理的です。`

Replace:
`ファッション、Kビューティー、デザイン、ポップアップが旅行の主要目的なら、丸一日使っても無理はありません。`

5. Current:
`聖水のポップアップには、買い物目的でなくても時間を使う意味があるものがあります。`

Replace:
`聖水のポップアップには、買い物目的でなくても時間を使う価値があるものがあります。`

6. Current:
`インタラクティブだから自動的に価値があるわけではありません。`

Replace:
`インタラクティブだからといって、必ず時間を使う価値があるわけではありません。`

7. Current:
`本当に興味のあるポップアップ1つ＋常設店＋カフェまたは食事＋ソウルの森 のほうが、「開催しているから」という理由だけで5件回るより強い一日です。`

Replace:
`本当に興味のあるポップアップ1つ＋常設店＋カフェまたは食事＋ソウルの森 のほうが、「開催しているから」という理由だけで5件回るより満足しやすい一日です。`

8. Current:
`HAUS NOWHERE SEOULは、アイウェアを買う人だけに意味がある場所ではありません。`

Replace:
`HAUS NOWHERE SEOULは、アイウェアを買う人だけの場所ではありません。`

9. Current:
`こうした空間デザインに興味があるなら行く意味があります。`

Replace:
`こうした空間デザインに興味があるなら行く価値があります。`

10. Current phrase:
`...人の両方に意味があります。`

Replace:
`...人のどちらにも見どころがあります。`

11. Current:
`次のような人には、最初の1軒として強いです。`

Replace:
`次のような人には、最初の1軒として使いやすいです。`

12. Current:
`自動的に両方へ行く必要はありません。`

Replace:
`両方へ行く必要はありません。`

13. Current:
`判断するのは、カフェが一日の中で何の役割を持つかです。`

Replace:
`判断するのは、カフェを一日のどこで使うかです。`

14. Current:
`オーダー靴を買わなくても、このエリアを見る意味があります。`

Replace:
`オーダー靴を買わなくても、このエリアを見る価値があります。`

15. Current:
`イベント終了後も、ソウルの森の基本的な役割は変わりません。`

Replace:
`イベント終了後も、ソウルの森の基本的な魅力は変わりません。`

16. H3:
`カフェの食べ物と本当の食事は役割が違う`

Replace:
`カフェの軽食としっかりした食事は別に考える`

17. Current:
`同じ役割を競わせないでください。`

Replace:
`同じものとして比べないでください。`

18. Current:
`自動的に両方入れないでください。`

Replace:
`最初から両方入れないでください。`

19. Route current:
`1:30–3:10 PM — 練武場通り＋意味があれば現在のポップアップ1つ`

Replace:
`1:30–3:10 PM — 練武場通り＋本当に興味があれば現在のポップアップ1つ`

20. Current phrase:
`「聖水へ行く意味がない」`

Replace:
`「聖水へ行く理由がない」`

21. Current:
`すでに5時間使ったなら、聖水は十分役割を果たしています。`

Replace:
`すでに5時間使ったなら、聖水は十分楽しめています。`

22. Stay bridge H2 current:
`泊まる意味があるのは、聖水の立地がソウル旅行全体を良くするときです。`

Replace:
`聖水に泊まるのは、その立地がソウル旅行全体を楽にするときです。`

23. Current:
`ファッション、Kビューティー、カフェ、デザイン、ポップアップ、ソウルの森のどれかが、もともと自分の旅行スタイルに合っているときに強い街です。`

Replace:
`ファッション、Kビューティー、カフェ、デザイン、ポップアップ、ソウルの森のどれかが、もともと自分の旅行スタイルに合っているときに相性のいい街です。`

24. Current:
`相性は良いですが、自動的に丸一日ではありません。`

Replace:
`相性は良いですが、最初から丸一日を使う必要はありません。`

25. Current:
`常設旗艦店、韓国ファッション、工業建築、短期コラボが相互に意味を作ります。`

Replace:
`常設旗艦店、韓国ファッション、工業建築、短期コラボが組み合わさることで、聖水らしい雰囲気が生まれます。`

Keep:
- `強い寒さ`
- `存在感が強い`
unchanged.

---

# 18. `where-to-stay-in-dongdaemun.html`

Only the editorial-calque locations below.

1. Current:
`初めてのソウル旅行で、東大門に行くのが午後の1回だけなら、ホテルまで移す理由はそれほど強くありません。`

Replace:
`初めてのソウル旅行で、東大門に行くのが午後の1回だけなら、ホテルまで移す必要性はそれほど高くありません。`

2. Current:
`夜の買い物に強いという利点を残しつつ、`

Replace:
`夜遅い買い物に便利という利点を残しつつ、`

3. Current:
`この組み合わせが、夜遅くまで買い物する旅行に強い理由です。`

Replace:
`この組み合わせなら、夜遅くまで買い物する旅行でも使いやすいです。`

4. Current:
`このページのほかのホテルにはない役割を持っています。`

Replace:
`このページのほかのホテルとは違う選択肢になります。`

5. Current:
`Summitも空港からの到着に強く、6702がホテル正面に停車します。`

Replace:
`Summitも空港から到着しやすく、6702がホテル正面に停車します。`

Keep:
- `存在感が強くなり`
- `別の強み`
- `自動的に安いわけではありません`
unchanged.

---

# 19. `where-to-stay-in-gangnam.html`

1. Current:
`新論峴周辺で遅くまで過ごすなら、夕食や飲みのあと三成まで戻るよりOcloudのほうが合理的です。`

Replace:
`新論峴周辺で遅くまで過ごすなら、夕食や飲みのあと三成まで戻るよりOcloudのほうが戻りやすいです。`

2. Current:
`新論峴は夜の動きも強いエリアです。`

Replace:
`新論峴は夜遅くまで動きやすいエリアです。`

3. Current:
`ラグジュアリーショッピング、美容予約、ギャラリー、ファインダイニング、狎鴎亭ロデオや清潭周辺で過ごすなら、こちらのほうが拠点として強いです。`

Replace:
`ラグジュアリーショッピング、美容予約、ギャラリー、ファインダイニング、狎鴎亭ロデオや清潭周辺で過ごすなら、こちらのほうが拠点として使いやすいです。`

4. Current:
`夜を新論峴周辺で過ごし、日中もソウル南側が中心なら立地の意味が出ます。`

Replace:
`夜を新論峴周辺で過ごし、日中もソウル南側が中心なら、この立地を活かしやすくなります。`

5. Current:
`AC Hotelは、駅三がすでに自分の目的地になっているときに最も強いホテルです。`

Replace:
`AC Hotelは、駅三がすでに自分の目的地になっているときに特に使いやすいホテルです。`

6. Current:
`新沙と島山がすでに旅程に入っているなら、vocoが最も意味を持ちます。`

Replace:
`新沙と島山がすでに旅程に入っているなら、vocoの立地を活かしやすくなります。`

7. Current:
`高級ブティックなら狎鴎亭・清潭のほうが強く、その旅程にはAndazの立地が合います。`

Replace:
`高級ブティックが目的なら狎鴎亭・清潭のほうが合い、その旅程にはAndazの立地が向いています。`

Keep literal:
- `高級だから自動的に『江南で一番』というわけではありません`
- `ファミリー客室だから自動的に眺めが良い...`
unchanged.

---

# Approval effect

If the user approves this Review Copy:

- status becomes `APPROVED / CONTENT LOCKED FOR IMPLEMENTATION`
- these exact 19 pages become the only wording-only implementation scope
- Codex may implement only the listed corrections
- no retranslation / no new wording judgment by Codex
- the 6 source-position pages remain separate and already implemented
- the 33 PASS pages remain closed

After implementation:
1. static QA
2. visible FAQ ↔ JSON-LD mirror check
3. numbers/facts regression check
4. only then combine the 25 corrected pages for final JA QA
5. commit / push / Production only after explicit user approval
