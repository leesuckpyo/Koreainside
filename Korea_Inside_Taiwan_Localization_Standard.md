# Korea Inside — Taiwan Traditional Chinese Localization Standard

**File:** `Korea_Inside_Taiwan_Localization_Standard.md`
**Status:** ACTIVE / SPECIALIZED STANDARD
**Version:** 1.1
**Effective date:** 2026-09-27
**Language:** Traditional Chinese for Taiwan (`zh-TW`)
**Production URL folder:** `/zh-tw/`

## Higher standards

1. `Korea_Inside_Public_Content_Master_Standard.md`
2. `Korea_Inside_Navigation_Hub_Architecture_Standard.md`
3. applicable common Specialized Standard
4. `Korea_Inside_Language_Localization_Standard.md`
5. this Standard
6. applicable Active Family Design Standard
7. latest Room Handover
8. Taiwan Master Inventory
9. page Research Master / Approved Public Copy / current English Production

If documents conflict, follow the project-wide document priority rules. This Standard does not replace higher standards.

---

# 0. Purpose

This Standard fixes the operating method for Korea Inside's Taiwan edition so the workflow does not change when the conversation room, computer, batch, or operator changes.

Taiwan localization is not a mechanical conversion from English or Simplified Chinese into Traditional Chinese.

The goal is:

> Preserve the facts, numbers, recommendation judgments, page role, and technical structure of the current English Production while writing natural Taiwan Traditional Chinese that a Taiwanese traveler would actually search, read, and use to make a decision.

The target locale is **Taiwan (`zh-TW`)**, not generic Traditional Chinese and not Mainland-Chinese wording converted into Traditional characters.

---

# 1. Canonical folder structure

The Taiwan edition uses the following fixed repository structure.

## 1.1 Working documents

`md/작업자료/`

Use for:

- Source Extraction
- Codex localization drafts
- ChatGPT review working copies
- QA reports
- terminology/glossary work
- routing audits
- temporary batch documents

No file in this folder is automatically Approved Public Copy.

## 1.2 Approved public copy

`md/승인본/대만어/`

Use only for:

- user-approved final Taiwan public copy
- `APPROVED PUBLIC COPY — CONTENT LOCKED` files
- approved correction supplements when an exception is genuinely required

Draft files must not be stored here.

## 1.3 Production HTML

`zh-tw/`

Example:

- English: `where-to-stay-in-myeongdong.html`
- Taiwan: `zh-tw/where-to-stay-in-myeongdong.html`

HTML language:

`lang="zh-TW"`

URL path remains lowercase:

`/zh-tw/`

Do not use mixed variants such as `/zh-TW/`, `/tw/`, `/zh/`, or `/tc/` for the same edition.

---

# 2. File naming convention

Use the following naming pattern unless a more specific Family Standard requires another form.

Source Extraction:

`Korea_Inside_ZH-TW_Batch##_Source_Extraction_YYYY-MM-DD.md`

Codex Draft:

`Korea_Inside_ZH-TW_Batch##_Codex_Draft_YYYY-MM-DD.md`

ChatGPT Review Copy:

`Korea_Inside_ZH-TW_Batch##_Review_Copy_YYYY-MM-DD.md`

Approved Public Copy:

`Korea_Inside_ZH-TW_Batch##_Approved_Public_Copy_YYYY-MM-DD.md`

Routing Audit:

`Korea_Inside_ZH-TW_Internal_Link_Routing_Audit_YYYY-MM-DD.md`

Taiwan Master Inventory:

`Korea_Inside_ZH-TW_Master_Inventory_YYYY-MM-DD.md`

Do not infer "latest" from filename suffixes alone. Check internal Status, Version, Effective date, change record, and when useful Git SHA.

---

# 3. Role division — fixed

## 3.1 User

The user has final authority over:

- direction
- exceptions
- final wording approval
- CONTENT LOCK
- Production approval
- batch-wide approval

## 3.2 ChatGPT

ChatGPT remains the final public-content editor.

ChatGPT is responsible for:

- interpreting the English source
- checking factual and recommendation preservation
- Taiwan-language quality review
- Taiwan localization judgment
- Humanization
- SEO wording judgment
- correcting or rewriting Codex draft wording when needed
- producing the final Review Copy
- deciding whether the draft is good enough to become Approved Public Copy

## 3.3 Codex — Taiwan draft exception

For the Taiwan edition only, when explicitly authorized by the user, Codex may create a **DRAFT Taiwan localization**.

This is a limited exception to the normal rule that Codex does not make localization judgments.

The exception permits Codex to:

- translate English user-facing copy into Taiwan Traditional Chinese
- use Taiwan travel vocabulary
- restructure sentences naturally
- perform basic Humanization while preserving the English judgment
- create a complete `zh-TW` draft MD for ChatGPT review

This exception does **not** give Codex final editorial authority.

Codex draft output is never automatically:

- Approved Public Copy
- CONTENT LOCKED
- Production-ready
- a replacement for ChatGPT review

Codex must not independently:

- add new facts
- change recommendations
- change hotel/area ranking
- create new traveler claims
- invent experience
- alter page intent
- change technical structure during the language-draft step

The fixed model is:

> **Codex = first-pass Taiwan draft
> ChatGPT = full editorial audit + final relocalization
> User = approval / CONTENT LOCK
> Codex = exact technical implementation**

---

# 4. Standard Taiwan workflow

Default workflow:

**Current English Production**
→ **Technical Source Extraction**
→ **Codex `zh-TW` Draft**
→ **ChatGPT full page audit**
→ **ChatGPT rewrites only what needs correction**
→ **Final Review Copy**
→ **user wording approval once**
→ **APPROVED PUBLIC COPY — CONTENT LOCKED**
→ **Codex exact implementation**
→ **technical QA**
→ **Production approval once, unless the user already gave batch-wide approval**
→ **stage only approved files**
→ **commit / push / Vercel Production**
→ **public QA**
→ **Taiwan Inventory update**

Do not repeat research, extraction, wording approval, or implementation gates without a real defect or user-requested re-review.

---

# 5. Draft branch workflow

When ChatGPT needs to inspect a large Codex-generated Taiwan draft directly from the repository, use a non-Production working branch.

Default draft branch name:

`zh-tw-draft`

Allowed flow:

English Production on `main`
→ create/update Taiwan draft on `zh-tw-draft`
→ commit
→ push `zh-tw-draft`
→ ChatGPT reviews the draft
→ corrections and Approved Public Copy are finalized
→ only after approval may implementation be prepared for `main`

While on `zh-tw-draft`:

- do not merge to `main`
- do not deploy Production
- do not mark Inventory COMPLETE
- do not treat draft HTML or draft MD as CONTENT LOCKED

A local commit alone is not sufficient when ChatGPT needs remote repository access. The draft branch must be pushed if the review workflow depends on remote access.

---

# 6. Language identity

The public language is:

**繁體中文（台灣） / Traditional Chinese (Taiwan) / `zh-TW`**

Use Taiwan usage, not generic Chinese.

Avoid:

- Simplified Chinese residues
- Mainland-specific vocabulary when a normal Taiwan equivalent exists
- Mainland punctuation or UI terminology when it sounds foreign in Taiwan
- literal English sentence order
- mechanical word-for-word translation

Do not convert Simplified Chinese text into Traditional characters and call it Taiwan localization.

---

# 7. Taiwan terminology baseline

Use the following as the default editorial direction. Page context can override when a more specific expression is clearly better.

| Concept | Taiwan default |
|---|---|
| hotel | 飯店 |
| accommodation / stay | 住宿 |
| room type | 房型 |
| double room | 雙人房 |
| twin room | 雙床房 |
| family room | 家庭房 / 家庭房型, by context |
| suite | 套房 |
| luggage | 行李 |
| luggage storage | 行李寄放 |
| subway | 地鐵 |
| airport bus | 機場巴士 |
| airport transfer | 機場接送 |
| taxi | 計程車 |
| elevator | 電梯 |
| escalator | 手扶梯 |
| booking / reservation | 訂房 / 預訂, by context |
| cancellation policy | 取消政策 |
| room rate | 房價 |
| check-in | 入住 |
| check-out | 退房 |
| front desk | 櫃台 |
| walking distance | 步行距離 |
| route / travel flow | 動線 / 路線, by context |
| public transportation | 大眾運輸 |
| app | App |
| privacy policy | 隱私權政策 |
| affiliate links | 聯盟行銷連結 |

Avoid using `酒店` as the normal translation of hotel on Korea Inside Taiwan pages unless a proper name or special context requires it.

Use `計程車`, not Mainland-style taxi terminology.

Use `手扶梯`, not a Mainland-first equivalent, unless quoting an official proper term.

---

# 8. Place-name baseline

Use established Traditional Chinese names when they are clear and natural for Taiwan readers.

Examples:

- Seoul → 首爾
- Myeongdong → 明洞
- Hongdae → 弘大
- Seongsu → 聖水
- Insadong → 仁寺洞
- Gangnam → 江南
- Jamsil → 蠶室
- Itaewon → 梨泰院
- Dongdaemun → 東大門
- Incheon Airport → 仁川機場
- Gwanghwamun → 光化門
- Jongno → 鍾路
- Euljiro → 乙支路

Do not invent Chinese translations for hotel, brand, room-category, transport-brand, or product names.

When the English source intentionally preserves a Korean NAVER Map search term, preserve that Korean search string.

---

# 9. Proper nouns and brands

Normally preserve official names exactly for:

- hotels
- brands
- OTA names
- room-category names
- airport-limousine route brands
- eSIM brands
- payment products
- transport cards
- app names
- Korean map-search strings

Examples:

- Expedia
- Trip.com
- Agoda
- T-money
- WOWPASS
- AREX
- L7 MYEONGDONG by LOTTE HOTELS

Do not create unofficial Chinese hotel names merely to make the sentence look more localized.

---

# 10. Facts and judgment preservation

The following must remain unchanged in meaning:

- facts
- numbers
- dates
- prices
- times
- walking distances
- room sizes
- occupancy
- bed configuration
- airport-bus numbers
- subway line numbers
- station/exit distinctions
- hotel facility conditions
- cancellation or booking conditions
- recommendation strength
- trade-offs
- page role
- section order

If the English says:

- `can work`
- `worth considering`
- `we would usually choose`
- `not our first pick`
- `depends on`

the Taiwan version must preserve that conditional strength.

Do not turn conditional editorial judgment into:

- `最推薦`
- `一定要`
- `最佳`
- `必住`
- `絕對`

unless the English source actually makes that level of claim.

---

# 11. Humanization

Humanization is not emotional copywriting.

The Taiwan page should help the traveler decide:

- who this works for
- who it does not work for
- what becomes inconvenient with luggage
- whether stairs, elevators, or the final walk matter
- whether a station-side hotel is actually useful for the intended route
- how beds and occupancy work for the full party
- whether the hotel itself is worth paying for
- how airport arrival differs by property
- which trade-off matters for the trip

Use the model:

> **verified fact + local travel context + independent editorial judgment**

Do not fabricate personal travel or hotel experience.

Forbidden examples:

- `我們住過`
- `我們實際入住`
- `親自住過之後`
- `我們親自體驗`

unless such experience is genuinely documented and approved as a source.

---

# 12. Taiwan writing style

Write as if the page was originally written for a Taiwan traveler.

Prefer:

- concise, conversational explanatory prose
- clear conditional judgment
- practical route and booking language
- natural sentence rhythm
- direct comparison

Avoid repetitive AI patterns such as:

- `非常適合...`
- `非常推薦...`
- `是最佳選擇...`
- `很方便...`
- `如果你想要...`
- `對於...來說...`
- `無論...都...`
- repeated `不僅...而且...`

Do not force every hotel section into the same:

recommendation → advantage → disadvantage

template.

Sentence openings, paragraph rhythm, and judgment structure should vary naturally.

---

# 13. Numbers, units, and typography

Preserve Arabic numerals unless the source clearly requires another form.

Preferred examples:

- `4 號線`
- `15 分鐘`
- `29.7 m²`
- `3–4 人`
- `約 100–150 公尺`
- `2026 年 8 月`

Do not convert room sizes into Taiwanese ping unless the English source or approved editorial direction explicitly requires it.

Use Traditional Chinese punctuation naturally.

Preserve official Latin-script room names and brands.

---

# 14. SEO localization

Localize:

- title
- meta description
- H1
- H2
- H3
- visible navigation text
- CTA
- FAQ
- breadcrumb visible text
- ALT
- ARIA
- user-facing JSON-LD wording

Do not mechanically translate SEO copy.

Preserve the original search intent and page role while making it natural for Taiwan search behavior.

Do not insert speculative keywords, popularity claims, or search-volume assumptions without research.

Example direction:

`Where to Stay in Myeongdong`

can naturally become:

`明洞住哪裡？`

when that matches the page role and Taiwan search language.

---

# 15. COMMON UI Golden Sample

Do not independently retranslate the common header, navigation, language selector, and footer on every page.

The first approved Taiwan batch must establish a **Taiwan COMMON UI Golden Sample**.

After approval:

- reuse the exact approved COMMON wording
- do not recreate variants page by page
- do not let Codex improvise new COMMON translations
- audit COMMON count and wording consistency in every batch

Typical COMMON areas:

- header
- global navigation
- submenu headings
- language selector
- footer headings
- footer utility links
- shared accessibility labels

If the common UI changes in English Production later, update the Taiwan Golden Sample through a controlled review rather than silently diverging.

---

# 16. Language selector

Taiwan display label should be fixed deliberately during the first implementation.

Recommended language identity:

`繁中`

or another user-approved concise Taiwan label.

Do not automatically use `ZH-TW` as the public-facing language label merely because it is the locale code.

Locale code:

`zh-TW`

is for technical language identification.

Public UI wording requires editorial approval.

---

# 17. Parent / child dependency

If Source Extraction contains:

- parent textContent ITEM
- nested child link or semantic ITEM

keep both and record dependency before implementation.

Never independently translate parent and child in a way that produces conflicting final output.

Required QA:

- parent groups counted
- unique child ITEMs counted
- dependency missing = 0
- parent/child implementation conflict = 0

---

# 18. Internal-link routing — must be correct from the start

The Japanese edition exposed a major failure mode: localized pages existed, but many internal links still routed to English.

The Taiwan edition must prevent this during initial implementation.

Rule:

If a Taiwan counterpart exists and is Production-ready:

English target:

`/page.html`

Taiwan target:

`/zh-tw/page.html`

A Taiwan page must route normal internal navigation to the Taiwan counterpart.

English fallback is allowed only when:

- no Taiwan counterpart exists
- the destination is intentionally excluded from Taiwan localization
- a higher rule explicitly requires the English page

Language-switcher links are separate and should intentionally point to the selected language.

After every Production batch:

- audit navigation
- body contextual links
- CTA
- related links
- breadcrumb
- footer

Final Taiwan completion requires a full-site routing audit.

Target:

- Taiwan counterpart available → correct Taiwan routing 100%
- routing defect = 0
- broken Taiwan target = 0
- invalid Taiwan target = 0
- legitimate English fallback preserved

---

# 19. hreflang / canonical / sitemap

For every published Taiwan page:

- `lang="zh-TW"`
- self canonical → Taiwan URL
- hreflang set must include the applicable published language siblings

Normally:

- `en`
- `es`
- `ja`
- `zh-TW`
- `x-default`

Do not add a hreflang target that does not actually exist.

Reciprocal hreflang must be verified.

Sitemap:

- every published Taiwan URL exactly once
- duplicate = 0
- unpublished draft URL = 0

---

# 20. Affiliate and tracking protection

Localization must not alter:

- affiliate destination
- tracking parameters
- subid
- hotel identifiers
- placement/context identifiers
- analytics attributes

Visible CTA text may be localized.

Brand names remain unchanged.

Examples:

- Expedia
- Trip.com
- Agoda

Technical QA must compare the Taiwan page against the English source or approved implementation baseline.

---

# 21. Codex draft QA

Before a Codex Taiwan draft is handed to ChatGPT, Codex must self-check:

- localized ITEM coverage = 100%
- missing Taiwan value = 0
- number mismatch = 0
- proper-name mismatch = 0
- recommendation drift = 0
- invented facts = 0
- fabricated firsthand experience = 0
- Simplified-only residue = 0
- Mainland-specific wording issues reviewed
- FAQ visible wording and schema wording consistent
- parent/child dependencies accounted for
- HTML modifications = 0 during language-only draft work
- stage/commit/push/deploy only within the explicitly approved draft-branch scope

Codex self-QA is evidence, not final approval.

---

# 22. ChatGPT final audit

ChatGPT must not rubber-stamp the Codex draft.

For every page, review at least:

1. facts
2. numbers
3. proper names
4. recommendation strength
5. Taiwan vocabulary
6. Mainland wording residue
7. literal English syntax
8. Humanization quality
9. repetitive AI phrasing
10. SEO title/meta/H1
11. headings
12. CTA
13. FAQ
14. ALT/ARIA
15. user-facing schema wording
16. group/room/bed interpretation
17. transport and luggage friction
18. direct-experience fabrication

ChatGPT may preserve good Codex copy and rewrite only the weak parts.

The final public wording is ChatGPT's editorial responsibility.

---

# 23. Approval policy

Do not recreate the repeated-approval loop that occurred during earlier localization work.

Default:

- research approval: only when genuinely needed
- Source Extraction: technical PASS, no wording approval
- Codex Draft: not an approval gate
- ChatGPT Review Copy: self-check before showing user
- user wording approval: once per batch
- Approved Public Copy → CONTENT LOCKED
- Production approval: once after technical QA

If the user explicitly gives batch-wide or one-shot approval, do not ask again for intermediate approval unless:

- source drift
- factual defect
- recommendation drift
- approved-copy defect
- out-of-scope technical requirement
- QA failure

---

# 24. CONTENT LOCK

After user approval:

`APPROVED PUBLIC COPY — CONTENT LOCKED`

Do not reopen merely because a better phrase occurs later.

Reopen only for:

- clear factual error
- explicit user direction change
- obvious post-lock quality defect
- explicit user re-review request

Technical routing, canonical, hreflang, or asset-path corrections do not automatically reopen public copy.

---

# 25. Implementation role

After CONTENT LOCK, Codex implementation must be exact.

Codex may:

- apply approved Taiwan wording
- preserve technical structure
- correct language-path asset references when required
- implement canonical/hreflang/sitemap
- implement approved Taiwan internal-link routing
- perform technical QA
- perform Git / Production work when authorized

Codex must not rewrite approved Taiwan wording during implementation.

---

# 26. Production QA

Before and after Production, verify:

- approved ITEM coverage = 100%
- COMMON UI = exact Golden Sample
- page-specific English residue = 0
- Simplified Chinese residue = 0 where it represents untranslated/localization residue
- facts/numbers mismatch = 0
- recommendation mismatch = 0
- parent/child conflict = 0
- internal links = PASS
- Taiwan routing defect = 0
- broken Taiwan target = 0
- affiliate/tracking mismatch = 0
- asset errors = 0
- canonical = PASS
- hreflang = PASS
- sitemap = PASS
- public HTTP = PASS
- public HTML matches committed output
- out-of-scope change = 0

Only after Production QA may the Taiwan Inventory status become COMPLETE.

---

# 27. Git safety

Protect existing user working-tree changes.

Without explicit approval, never use:

- `git add .`
- `git add -A`
- `git restore`
- `git reset`
- `git clean`

Stage only the approved batch files.

Draft-branch commits and Production commits are separate concepts.

Do not merge a Taiwan draft branch into `main` merely because the draft has complete language coverage.

---

# 28. Pilot precedent — 2026-09-27

The first Taiwan localization pilot used:

`where-to-stay-in-myeongdong.html`

Pilot output:

`Korea_Inside_ZH-TW_Myeongdong_Localization_Pilot_2026-09-27.md`

Pilot result:

- localized ITEMs: 415 / 415
- missing Taiwan values: 0
- numeric mismatch: 0
- protected proper-name mismatch: 0
- recommendation drift detected by Codex self-QA: 0
- invented facts detected by Codex self-QA: 0
- fabricated firsthand experience: 0
- HTML implementation: 0
- stage / commit / push / deploy: not performed

Editorial review concluded that the draft quality was sufficient to adopt the workflow:

> **Codex Taiwan draft → ChatGPT full audit/relocalization → user approval → exact implementation**

The pilot did **not** establish the Codex draft as final public copy.

---

# 29. Initial implementation priorities

Before scaling to the full Taiwan edition, complete these setup items:

1. use the existing `md/작업자료/` directly; do not create a language subfolder under it
2. create `md/승인본/대만어/`
3. adopt `/zh-tw/` as the Taiwan Production folder
4. create the Taiwan Master Inventory
5. establish the COMMON UI Golden Sample
6. decide the public language-selector label
7. confirm draft-branch workflow
8. define the first production Batch
9. ensure routing QA is built into every Batch from the start

Do not create all Production pages before these shared rules are fixed.

---

# 30. Room migration rule

When moving to a new conversation room during Taiwan localization, the new room must read:

1. `Korea_Inside_Public_Content_Master_Standard.md`
2. `Korea_Inside_Navigation_Hub_Architecture_Standard.md`
3. `Korea_Inside_Language_Localization_Standard.md`
4. `Korea_Inside_Taiwan_Localization_Standard.md`
5. latest `Korea_Inside_Room_Handover_*.md`

Then, only when relevant:

- Taiwan Master Inventory
- current Batch Source Extraction
- current Codex Draft
- current Approved Public Copy
- applicable Family Design Standard

The Room Handover carries current state.

This Standard carries the working method.

Do not renegotiate the workflow merely because the room changed.

---

# 31. Final operating principle

The Taiwan workflow is:

> **English Production supplies the source.
> Codex reduces first-pass localization labor.
> ChatGPT owns final Taiwan editorial quality.
> The user approves and locks the public copy.
> Codex implements only the approved result.**

The goal is to make the Taiwan edition faster than the Japanese edition without lowering factual accuracy, Humanization quality, routing quality, or user control.
---

# 32. Storage location — fixed

This Standard itself is stored at the Korea Inside repository root, alongside the other active Standards.

Repository-relative path:

`Korea_Inside_Taiwan_Localization_Standard.md`

Current local repository path used in this project:

`C:\\Projects\\Koreainside\\Koreainside\\Korea_Inside_Taiwan_Localization_Standard.md`

Do not store this Standard under:

- `md/작업자료/`
- `md/승인본/`
- `zh-tw/`

Taiwan working documents remain directly under:

`md/작업자료/`

Taiwan Approved Public Copy is stored under:

`md/승인본/대만어/`

Taiwan Production HTML is stored under:

`zh-tw/`

---

# 33. Version 1.1 change record

Version 1.1 corrects the repository storage structure established during initial Taiwan setup.

- Taiwan Standard location fixed to repository root.
- Working documents fixed to existing `md/작업자료/` with no Taiwan-language subfolder.
- Approved Public Copy remains `md/승인본/대만어/`.
- Taiwan Production HTML remains `zh-tw/`.
- No localization quality rule or Pilot judgment was changed.
