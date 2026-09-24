# Korea Inside — Japanese Production Humanization & Search Fix Batch 1

**Date:** 2026-09-24  
**Status:** APPROVED PUBLIC COPY — CONTENT LOCKED  
**Scope:** Existing Japanese Production 7 pages only  
**Purpose:** Fix confirmed Japanese copy breakage, search-facing terminology, FAQ/schema parity, and main-content links to Japanese siblings.  
**Not in scope:** Common header/navigation/footer, `common.js`, common `style.css`, new facts, recommendation changes, structural redesign, affiliate/tracking changes.

---

# Governing Standards

Apply in this order:

1. `Korea_Inside_Public_Content_Master_Standard.md`
2. `Korea_Inside_Navigation_Hub_Architecture_Standard.md`
3. `Korea_Inside_Language_Localization_Standard.md`
4. `Korea_Inside_Japanese_Localization_Standard.md`
5. Latest `Korea_Inside_Room_Handover_*.md`

This Batch is a **Fix Batch**, not a retranslating Batch.

Locked:
- facts
- numbers
- dates
- fares
- operating conditions
- recommendation judgments
- hotel/area ranking
- HTML section order
- class/id/data-*
- images/srcset
- affiliate URLs/tracking
- schema structure

---

# Source Fingerprints

| Page | English blob SHA | Japanese blob SHA |
|---|---|---|
| `index.html` | `9300a2c0c8644fb50c6926c380f7133c6c429bb6` | `6805f56010ee896deee00585adb965431fcc870f` |
| `dongdaemun-travel-guide.html` | `a55ddc4c1bb3ba2a641e89142ba46b995306fae0` | `e514bff8909c81460017308faa9e8fd3ab075b73` |
| `where-to-stay-in-dongdaemun.html` | `b97908469895fdd89cdec14921f234c0ddfd5c63` | `ccab0a77b77fb931bf2382665d6a583ed95763e2` |
| `airport.html` | `c16b5df76a32a3f0aa8dd871ce4b14f4724399c7` | `219b44723c89a1d7c9d1436691469702933320a3` |
| `arrival.html` | `89d4720e9a392dcfed1e42626c4499d48a536469` | `100d2a9baa4ac002791f8108ce73059e11a27a5a` |
| `airport-transfer.html` | `8deb329f136a3632151d875a3e96c51b32dbf8af` | `3ce097ba9d8300f86d485244a27dbaee9f040248` |
| `arex.html` | `9ce33e55e668329f53f2c7323f80bca3a40d9d01` | `57f246b5f977e763fab566595182cd12d44e61c2` |

If any source blob changes before implementation, STOP and recheck the affected item.

---

# Japan Search / Official Terminology Basis

Use only as wording guidance; do not add new facts.

- `東大門（トンデムン）` is established Japanese official/tourism usage.
- Japanese airport search wording strongly uses `仁川空港からソウル市内`, `行き方`, `アクセス`, `料金`, `所要時間`.
- Official Japanese Korea tourism wording uses AREX `直通列車` and `一般列車`.
- Official Japanese e-Arrival service uses `大韓民国電子入国申告書` / `電子入国申告書`.

Search wording must not change page role or recommendation judgment.

---

# PAGE 1 — `ja/index.html`

## Audit status

**PASS COPY / FIX MAIN-CONTENT LINKS ONLY**

No Japanese public-copy rewrite is approved.

## Main-content Japanese sibling link sync

Only within `<main>`:

- `../airport.html` → `/ja/airport.html` — 1 occurrence
- `../arrival.html` → `/ja/arrival.html` — 1 occurrence
- `../airport-transfer.html` → `/ja/airport-transfer.html` — 2 occurrences
- `../maps.html` → `/ja/maps.html` — 1 occurrence
- `../tmoney.html` → `/ja/tmoney.html` — 1 occurrence
- `../wowpass.html` → `/ja/wowpass.html` — 1 occurrence

**Total: 7**

Do not change common navigation or footer in this Batch.

---

# PAGE 2 — `ja/dongdaemun-travel-guide.html`

## Audit status

**FIX NEEDED**

## SEO / Search-facing copy

### Title

**Current**  
`東大門ガイド2026｜DDP・市場・ナイトショッピング`

**Approved proposal**  
`東大門（トンデムン）観光ガイド2026｜DDP・市場・ナイトショッピング`

### H1

**Current**  
`ソウル・東大門ガイド 2026 ：DDP・市場・ナイトショッピング`

**Approved proposal**  
`ソウル・東大門（トンデムン）観光ガイド 2026：DDP・市場・ナイトショッピング`

No recommendation or factual change.

---

## Confirmed copy fixes

### Fix DD-01

**Current**
`まずは <strong>一般向けショッピング側</strong>.`

**Replace with**
`まずは<strong>一般向けショッピング側</strong>から始めましょう。`

**Preserve**
- retail side is the default first-time starting point

---

### Fix DD-02

Current Japanese was split unnaturally across two paragraphs.

**Current paragraph 1**
`東大門周辺には、実際に夜の露店市場があり、よく <strong>セビッ市場</strong> または <strong>黄色いテント市場</strong>.`

**Current paragraph 2**
`と呼ばれます。模倣品が多いことでも知られ、現地当局による取り締まりも続いています。`

**Replace paragraph 1 with**
`東大門周辺には、実際に夜の露店市場があります。`

**Replace paragraph 2 with**
`よく<strong>セビッ市場</strong>または<strong>黄色いテント市場</strong>と呼ばれます。模倣品が多いことでも知られ、現地当局による取り締まりも続いています。`

HTML paragraph count remains unchanged.

---

### Fix DD-03

**Current**
`まず <strong>東大門歴史文化公園駅</strong>.`

**Replace with**
`まずは<strong>東大門歴史文化公園駅</strong>から始めます。`

---

### Fix DD-04

**Current**
`実用的な流れは：`

**Replace with**
`回りやすい順番は、次のとおりです。`

Meaning preserved: practical sequence.

---

### Fix DD-05 — raw Markdown removal

**Current**
`**何がしたい？<br>それは東大門のどちら側にある？<br>その場所が実際に動くのは何時？<br>戻り歩きをせずに行けるルートは？**`

**Replace visible text with**
`何がしたい？<br>それは東大門のどちら側にある？<br>その場所が実際に動くのは何時？<br>戻り歩きをせずに行けるルートは？`

Do not output literal `**`.

---

## Main-content Japanese sibling link sync

Only within `<main>`:

- `../airport.html` → `/ja/airport.html` — 1 occurrence
- `../tmoney.html` → `/ja/tmoney.html` — 1 occurrence
- `../wowpass.html` → `/ja/wowpass.html` — 1 occurrence

**Total: 3**

Do not change common navigation or footer.

---

# PAGE 3 — `ja/where-to-stay-in-dongdaemun.html`

## Audit status

**FIX NEEDED — COPY INTEGRITY**

SEO title/search direction remains approved. No title/H1 rewrite.

---

## Confirmed copy fixes

### Fix STAY-DD-01 — Sotetsu / Exit 4

**Current**
`だからSotetsuのエレベーター情報は、単に『徒歩1分』という表示より実用的です。ホテルは荷物のある旅行者に <strong>エレベーターがある4番出口</strong>.`

**Replace with**
`だからSotetsuのエレベーター情報は、単に「徒歩1分」と書かれているより役立ちます。ホテルは荷物のある旅行者に、<strong>エレベーターのある4番出口</strong>を案内しています。`

**Preserve**
- Exit 4
- elevator
- luggage traveler judgment

---

### Fix STAY-DD-02 — Novotel / Airport Bus 6001

**Current**
`空港からの移動はSotetsuほど楽ではありません。東大門歴史文化公園駅12番出口からは徒歩約3分ですが、ホテルが案内する空港バス6001を利用すると、 <strong>降車後に徒歩約10分</strong>.`

**Replace with**
`空港からの移動はSotetsuほど楽ではありません。東大門歴史文化公園駅12番出口からは徒歩約3分ですが、ホテルが案内する空港バス6001を利用すると、降車後に<strong>徒歩約10分</strong>です。`

**Preserve**
- Exit 12
- about 3 minutes
- bus 6001
- about 10-minute walk after getting off

---

### Fix STAY-DD-03 — Skypark station exits

**Current**
`ホテルはHyundai City Outlet Dongdaemunの建物内にあります。公式アクセスでは <strong>東大門駅8番出口から徒歩約5分</strong> 、そして <strong>東大門歴史文化公園駅14番出口から徒歩約6分</strong>.`

**Replace with**
`ホテルはHyundai City Outlet Dongdaemunの建物内にあります。公式アクセスでは、<strong>東大門駅8番出口から徒歩約5分</strong>、<strong>東大門歴史文化公園駅14番出口から徒歩約6分</strong>です。`

---

### Fix STAY-DD-04 — Mangrove facilities

**Current paragraph 1**
`短期宿泊にも対応するコリビング施設で、個室に加えて広い共用スペースがあります。東大門の施設には <strong>24時間使えるコワーキング、共用キッチン、洗濯機・乾燥機を備えた24時間無料ランドリールーム、ラウンジ、フィットネス・リラクゼーションルーム、屋上テラス</strong>.`

**Current paragraph 2**
`があり、長めの滞在、リモートワーク、ときどき自炊したい人、洗濯代を抑えたい人にはかなり合います。`

**Replace paragraph 1 with**
`短期宿泊にも対応するコリビング施設で、個室に加えて広い共用スペースがあります。東大門の施設には、<strong>24時間使えるコワーキング、共用キッチン、洗濯機・乾燥機を備えた24時間無料ランドリールーム、ラウンジ、フィットネス・リラクゼーションルーム、屋上テラス</strong>があります。`

**Replace paragraph 2 with**
`長めの滞在、リモートワーク、ときどき自炊したい人、洗濯代を抑えたい人にはかなり合います。`

HTML paragraph count remains unchanged.

---

### Fix STAY-DD-05 — repeated Novotel 6001 sentence

**Current**
`Novotelも6001を案内していますが、ホテル公式アクセスでは <strong>降車後に徒歩約10分</strong>.`

followed by:

`としています。誰にとっても問題になる距離ではありませんが、子ども連れ、ベビーカー、大型スーツケース2個となると話は変わります。`

**Replace first paragraph with**
`Novotelも6001を案内していますが、ホテル公式アクセスでは<strong>降車後に徒歩約10分</strong>としています。`

**Replace second paragraph with**
`誰にとっても問題になる距離ではありませんが、子ども連れ、ベビーカー、大型スーツケース2個となると話は変わります。`

HTML paragraph count remains unchanged.

---

## Internal links

No stale main-content English fallback to an already released Japanese sibling was found in this page.

---

# PAGE 4 — `ja/airport.html`

## Audit status

**BODY PASS / SEARCH-FACING FIX + LINKS**

---

## SEO

### Title

**Current**
`仁川空港 到着ロビーガイド：最初の30分 | Korea Inside`

**Approved proposal**
`仁川空港 到着後ガイド：到着ロビーで最初の30分にやること | Korea Inside`

### Meta description

**Current**
`入国審査と税関を終えたら、この仁川空港の到着ロビーガイドで、通信を確保し、宿泊先の住所を保存し、支払い手段を確認して、移動方法を選びましょう。`

**Approved proposal**
`仁川空港到着後、入国審査と税関を終えて到着ロビーに出たら、通信、宿泊先の住所、支払い手段、空港からの移動方法を確認します。`

### H1

Keep current:
`仁川空港の到着ロビーで最初の30分にやること`

No H1 rewrite.

---

## Main-content Japanese sibling link sync

Only within `<main>`:

- `../arrival.html` → `/ja/arrival.html` — 1 occurrence
- `../airport-transfer.html` → `/ja/airport-transfer.html` — 3 occurrences
- `../arex.html` → `/ja/arex.html` — 1 occurrence
- `../airport-bus.html` → `/ja/airport-bus.html` — 2 occurrences
- `../maps.html` → `/ja/maps.html` — 3 occurrences
- `../tmoney.html` → `/ja/tmoney.html` — 1 occurrence

**Total: 11**

Do not change common navigation/footer.

---

# PAGE 5 — `ja/arrival.html`

## Audit status

**FIX NEEDED — OFFICIAL TERM / PUNCTUATION / FAQ-SCHEMA / LINKS**

---

## SEO

### Meta description

**Current**
`仁川空港の到着ガイド。利用ターミナルの確認、入国審査、K-ETAとe-Arrival Card、手荷物受取、税関、一般到着ロビーまでの流れをまとめています。`

**Approved proposal**
`仁川空港の到着ガイド。利用ターミナルの確認、入国審査、K-ETAと電子入国申告書（e-Arrival Card）、手荷物受取、税関、一般到着ロビーまでの流れをまとめています。`

Title and H1 remain unchanged.

---

## Official terminology

### Heading

**Current**
`K-ETAとe-Arrival Cardは別のもの`

**Replace with**
`K-ETAと電子入国申告書（e-Arrival Card）は別のもの`

### Body term normalization in the same section

Where the current Japanese copy says:
- `e-Arrival Cardの提出`
- `公式のe-Arrival Card`
- `公式e-Arrival Cardサイト`
- `e-Arrival Card公式サイト`

use:
- `電子入国申告書の提出`
- `公式の電子入国申告書`
- `公式電子入国申告書サイト`
- `電子入国申告書 公式サイト`

Do not change eligibility facts, dates, K-ETA exemption facts, fee statement, or submission timing.

---

## Copy integrity fixes

### Fix ARR-01

**Current**
`市内へ向かう前に、モバイルデータの確認、宿泊先の韓国語情報の保存、予備の支払い手段、宿泊先までの移動方法を整える必要があるかもしれません。旅行開始直後のこうした実用的な準備は、別ページでまとめています： <a href="airport.html">仁川空港 到着ロビーガイド</a>.`

**Replace with**
`市内へ向かう前に、モバイルデータの確認、宿泊先の韓国語情報の保存、予備の支払い手段、宿泊先までの移動方法を整える必要があるかもしれません。旅行開始直後のこうした実用的な準備は、別ページの<a href="airport.html">仁川空港 到着ロビーガイド</a>でまとめています。`

---

### Fix ARR-02

**Current**
`旅行開始直後の実用的な準備は、 <a href="airport.html">仁川空港 到着ロビーガイド</a> で確認できます。`

**Replace with**
`旅行開始直後の実用的な準備は、<a href="airport.html">仁川空港 到着ロビーガイド</a>で確認できます。`

---

## FAQ / JSON-LD exact parity

Visible FAQ: **6**  
JSON-LD FAQ: **6**

Target after implementation: **6 / 6 exact Japanese wording parity**

### FAQ 5 exact approved answer

Use this exact text in both visible FAQ and JSON-LD:

`一般到着ロビーに出てから決めれば大丈夫です。最適な方法は、実際の宿泊先、到着時刻、荷物の量、人数によって変わります。空港送迎ガイドでは、AREX、空港バス、タクシー、事前予約送迎を、宿泊先までの全行程で比較しています。`

Visible link markup may remain around `空港送迎ガイド`, but the rendered wording must match this sentence.

### FAQ 6 exact approved answer

Use this exact text in both visible FAQ and JSON-LD:

`「Transfer」または「Connecting Flights」の案内表示と航空会社の指示に従ってください。一般の入国ルートへ進むと決めつけず、乗り継ぎターミナル、乗り継ぎ可能時間、預け荷物が次の便まで通しで運ばれるかを確認してください。`

Final Japanese punctuation: `。`

---

## Main-content Japanese sibling link sync

Only within `<main>`:

- `../airport-transfer.html` → `/ja/airport-transfer.html` — 2 occurrences

**Total: 2**

Do not change common navigation/footer.

---

# PAGE 6 — `ja/airport-transfer.html`

## Audit status

**FIX NEEDED — SEARCH-FACING COPY / GRAMMAR / LINKS**

---

## SEO

### Title

**Current**
`仁川空港からソウル：AREX・バス・タクシー・送迎比較 | Korea Inside`

**Approved proposal**
`仁川空港からソウル市内への行き方：AREX・バス・タクシー・送迎比較 | Korea Inside`

### Meta description

**Current**
`仁川空港からソウルまでのAREX、空港バス、タクシー、コールバン、貸切送迎を比較。現在の運賃、荷物、深夜到着時の選び方までまとめています。`

**Approved proposal**
`仁川空港からソウル市内への行き方を、AREX、空港バス、タクシー、コールバン、貸切送迎で比較。現在の運賃、荷物、深夜到着時の選び方までまとめています。`

### H1

**Current**
`仁川空港からソウルへ： どの移動手段が合う？`

**Approved proposal**
`仁川空港からソウル市内へ：どの移動手段が合う？`

---

## Grammar fixes

### Fix TR-01 — Airport Bus link sentence

**Current**
`詳しくは <a href="../airport-bus.html">空港バスガイド</a> では、路線と各ターミナルの停留所をさらに詳しく説明しています。`

**Replace with**
`詳しくは、<a href="/ja/airport-bus.html">空港バスガイド</a>で、路線と各ターミナルの停留所を説明しています。`

---

### Fix TR-02 — Taxi link sentence

**Current**
`詳しくは <a href="../taxi.html">タクシーガイド</a> では、ソウルのタクシー種類、支払い、運賃体系をさらに詳しく説明しています。`

**Replace with**
`詳しくは、<a href="../taxi.html">タクシーガイド</a>で、ソウルのタクシーの種類、支払い方法、運賃体系を説明しています。`

`taxi.html` Japanese sibling is still MISSING in this scope, so English fallback remains.

---

### Fix TR-03 — Call Van / Private Transfer link sentence

**Current**
`大人数、荷物容量、事前予約の詳細は <a href="../incheon-airport-private-transfer.html">コールバン／貸切送迎ガイド</a>.`

**Replace with**
`大人数、荷物容量、事前予約の詳細は、<a href="../incheon-airport-private-transfer.html">コールバン／貸切送迎ガイド</a>で確認できます。`

Japanese sibling is MISSING, so English fallback remains.

Protected terminology:
- `コールバン`
- `貸切送迎`

---

## Main-content Japanese sibling link sync

Only within `<main>`:

- `../arex.html` → `/ja/arex.html` — 1 occurrence
- `../airport-bus.html` → `/ja/airport-bus.html` — 1 occurrence
- `../tmoney.html` → `/ja/tmoney.html` — 1 occurrence

**Total: 3**

`airport-bus.html` is already covered in Fix TR-01; count it once in implementation QA.

Do not change common navigation/footer.

---

# PAGE 7 — `ja/arex.html`

## Audit status

**FIX NEEDED — OFFICIAL/SEARCH TERM + MINOR HUMANIZATION + LINKS**

---

## SEO

### Title

**Current**
`仁川空港からソウルへAREX：料金・所要時間・列車ガイド | Korea Inside`

**Approved proposal**
`仁川空港AREXガイド：直通列車・一般列車（各駅停車）の料金・所要時間 | Korea Inside`

### Meta description

**Current**
`仁川空港からソウルへ向かうAREX直通列車と各駅停車を、料金、所要時間、停車駅、乗車券、T-money、荷物、深夜到着時の選び方まで比較します。`

**Approved proposal**
`仁川空港からソウルへ向かうAREX直通列車と一般列車（各駅停車）を、料金、所要時間、停車駅、乗車券、T-money、荷物、深夜到着時の選び方まで比較します。`

### H1

**Current**
`仁川空港からソウルへAREXで移動：直通列車 vs 各駅停車`

**Approved proposal**
`仁川空港からソウルへAREXで移動：直通列車 vs 一般列車（各駅停車）`

---

## Official term establishment

Use `一般列車（各駅停車）` in search-facing / first-introduction positions.

### Hero paragraph 1

**Current**
`AREXには2種類の列車があります。直通列車は空港の各ターミナルからソウル駅まで指定席で運行し、各駅停車は弘大入口駅、孔徳、金浦空港など途中の駅にも停車します。`

**Replace with**
`AREXには2種類の列車があります。直通列車は空港の各ターミナルからソウル駅まで指定席で運行し、一般列車（各駅停車）は弘大入口駅、孔徳、金浦空港など途中の駅にも停車します。`

### Fast-choice label

**Current**
`各駅停車が向いているのは…`

**Replace with**
`一般列車（各駅停車）が向いているのは…`

### Quick-link label

**Current**
`直通列車 vs 各駅停車`

**Replace with**
`直通列車 vs 一般列車`

### H2

**Current**
`直通列車と各駅停車、どちらを選ぶ？`

**Replace with**
`直通列車と一般列車（各駅停車）、どちらを選ぶ？`

### H3

**Current**
`AREX各駅停車`

**Replace with**
`AREX一般列車（各駅停車）`

### Comparison H2

**Current**
`直通列車 vs 各駅停車 早見比較`

**Replace with**
`直通列車 vs 一般列車 早見比較`

### Comparison ARIA

**Current**
`AREX直通列車と各駅停車の比較`

**Replace with**
`AREX直通列車と一般列車の比較`

Do **not** mechanically replace every body occurrence of `各駅停車`.

---

## Humanization fixes

### Fix AREX-01

**Current**
`この表では、券売機の前で判断するときに必要な違いをまとめています。降車後に何が待っているかは、この後の目的地別セクションで説明します。`

**Replace with**
`券売機の前で迷わないよう、2種類の列車の違いをこの表にまとめています。降車後に何が待っているかは、この後の目的地別セクションで説明します。`

---

### Fix AREX-02

**Current**
`直通列車は早く、指定席があります。`

**Replace with**
`直通列車は速く、指定席があります。`

Only this sentence fragment changes. Remaining Seoul Station paragraph stays unchanged.

---

### Fix AREX-03

**Current**
`次の移動がソウル駅発のKTXなら、直通列車は合理的な空港鉄道の選択になりやすいです。`

**Replace with**
`次の移動がソウル駅発のKTXなら、直通列車を選ぶと動きやすくなります。`

Remaining KTX paragraph stays unchanged.

---

## Main-content Japanese sibling link sync

Only within `<main>`:

- `../airport-bus.html` → `/ja/airport-bus.html` — 4 occurrences
- `../maps.html` → `/ja/maps.html` — 3 occurrences
- `../tmoney.html` → `/ja/tmoney.html` — 2 occurrences

**Total: 9**

Do not change common navigation/footer.

---

# Main-Content Link Sync Summary

Released Japanese sibling fallback fixes in this Batch:

| Page | Count |
|---|---:|
| `ja/index.html` | 7 |
| `ja/dongdaemun-travel-guide.html` | 3 |
| `ja/where-to-stay-in-dongdaemun.html` | 0 |
| `ja/airport.html` | 11 |
| `ja/arrival.html` | 2 |
| `ja/airport-transfer.html` | 3 |
| `ja/arex.html` | 9 |
| **Total** | **35** |

Only `<main>` links are in scope.

Explicitly excluded:
- common navigation
- common footer

Those remain protected until separately approved.

---

# Batch QA Targets After Approval / Implementation

## Copy integrity

- Japanese sentence-ending ASCII `.` in approved fix locations: 0
- raw Markdown `**` in approved fix location: 0
- broken `<strong>` + Japanese particle/ending: 0
- broken link-spacing/particle constructions in approved fix locations: 0
- facts mismatch: 0
- numbers mismatch: 0
- recommendations mismatch: 0

## Search / terminology

- `東大門（トンデムン）観光ガイド` search-facing usage: PASS
- `仁川空港 到着後` title intent: PASS
- `電子入国申告書（e-Arrival Card）`: PASS
- `仁川空港からソウル市内への行き方`: PASS
- AREX `一般列車（各駅停車）` established in search-facing / first-use positions: PASS

## FAQ / Schema

- `arrival.html`: visible FAQ 6 / schema 6
- exact Japanese wording parity: 6 / 6
- `airport-transfer.html`: existing 10 / 10 parity preserved

## Links

- approved `<main>` English fallbacks to already released Japanese siblings: 0
- future/MISSING Japanese sibling links: 0
- common navigation/footer: unchanged

## Protected

- HTML section order: unchanged
- common header/nav/footer: unchanged
- `common.js`: unchanged
- common `style.css`: unchanged
- affiliate/tracking: unchanged
- images/srcset: unchanged

---

# Approval State

**APPROVED PUBLIC COPY — CONTENT LOCKED**

If the user approves this Batch:

1. Create `Korea_Inside_JA_Production_Humanization_Search_Fix_Batch1_Approved_Public_Copy_2026-09-24.md`
2. Status becomes `APPROVED PUBLIC COPY — CONTENT LOCKED`
3. Codex exact-implements only the approved fixes above
4. No retranslation or additional grammar improvement
5. Static QA
6. Scope-only stage
7. commit / push
8. Vercel Production READY
9. Public HTML QA
10. Existing Japanese Inventory COMPLETE count remains unchanged; these are fixes to already COMPLETE pages
