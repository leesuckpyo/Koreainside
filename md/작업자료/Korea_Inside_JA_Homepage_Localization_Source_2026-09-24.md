# Korea Inside — Japanese Homepage Localization Source

## Document Metadata

- Date: 2026-09-24
- Purpose: Exact technical extraction of all user-facing English strings required to turn the current Japanese homepage working copy into a substantive Japanese-localized homepage after separate user approval.
- Scope: English source `index.html`, identical working copy `ja/index.html`, and shared `common.js` runtime strings actually exposed through the homepage common UI.
- No-language-work boundary: No Japanese translation, localization, rewriting, grammar improvement, humanization, summary, expansion, CTA change, or recommendation change is included.
- Exactness rule: `Exact English` reproduces each current source leaf text, direct text node, visible attribute, SEO field, JSON-LD leaf, or runtime string without translation. HTML whitespace is normalized only where browsers collapse it for display; entities are recorded as their rendered characters.
- Common UI reuse rule: Header, navigation, language selector, footer, and shared runtime strings that match Japanese Golden Sample approved targets are marked `REUSE`. They must reuse the corresponding approved Japanese wording rather than trigger a new translation decision.
- Parent/child rule: Composite container `innerText` is not an ITEM. Only leaf/direct text nodes, visible attributes, JSON-LD user-facing leaf values, and runtime leaf strings are targets.
- User-facing data rule: A `data-*` value is included only when CSS or JS exposes it to users, including `content: attr(data-label)`. Functional, tracking, analytics, affiliate, and event data remain protected and are not localization ITEMs.
- Protection rule: Preserve facts, numbers, dates, prices, units, brands, products, places, addresses, operating conditions, URLs, affiliate/tracking values, class/id, functional data, event IDs, images/srcset, schema structure, CSS, and JS logic exactly.

## Source Integrity

| English source | SHA-256 | Japanese working copy | SHA-256 | SHA identical | Extracted ITEMs |
|---|---|---|---|---|---:|
| `index.html` | `6FBD0E3A8CEBDDA994D24BE933450E79202CFE16ED4E1D3474B32F496D59B7E6` | `ja/index.html` | `6FBD0E3A8CEBDDA994D24BE933450E79202CFE16ED4E1D3474B32F496D59B7E6` | YES | 151 |
| **Total** | — | — | — | **1/1** | **151** |

## Extraction Notes

- Golden Sample reuse references point to ITEM numbers in `md/작업자료/Korea_Inside_JA_Golden_Sample_Source_Batch1_2026-09-24.md`; their approved Japanese wording remains governed only by the corresponding `Approved_Public_Copy` document.
- Common UI is inventoried here because it is visible on the homepage, but matching `REUSE` targets are not requests for new translation.
- The current homepage contains no user-facing `data-*` value. Existing `data-section`, `data-common-header`, `data-nav-section`, and `data-supported-languages` values are functional and remain protected.
- Decorative direct text inside `aria-hidden="true"` elements is not a language-dependent localization target.
- URLs, script verification values, canonical/hreflang values, image paths, and tracking attributes are protected implementation data and are not localization ITEMs.

# PAGE: index.html

ITEM 0001
Page ITEM: 0001
File: `index.html`
Line/context: L6 · html > head > meta @content
Element/type: meta description
Exact English:

```text
Practical Korea travel guides for first-time visitors, covering where to stay, airport arrival, eSIMs, transport, payments, maps and local travel decisions.
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `eSIM`.
Source target: `index.html|offset:146|@content`

ITEM 0002
Page ITEM: 0002
File: `index.html`
Line/context: L7 · html > head > title direct text node
Element/type: title
Exact English:

```text
Korea Travel Guide for First-Time Visitors | Korea Inside
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `Korea Inside`.
Source target: `index.html|offset:349|direct-text`

ITEM 0003
Page ITEM: 0003
File: `index.html`
Line/context: L16 · html > head > meta @content
Element/type: OG title
Exact English:

```text
Korea Travel Guide for First-Time Visitors | Korea Inside
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `Korea Inside`.
Source target: `index.html|offset:972|@content`

ITEM 0004
Page ITEM: 0004
File: `index.html`
Line/context: L17 · html > head > meta @content
Element/type: OG description
Exact English:

```text
Practical Korea travel guides for first-time visitors, covering where to stay, airport arrival, eSIMs, transport, payments, maps and local travel decisions.
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `eSIM`.
Source target: `index.html|offset:1070|@content`

ITEM 0005
Page ITEM: 0005
File: `index.html`
Line/context: L18 · html > head > meta @content
Element/type: OG site name
Exact English:

```text
Korea Inside
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `Korea Inside`.
Source target: `index.html|offset:1273|@content`

ITEM 0006
Page ITEM: 0006
File: `index.html`
Line/context: L19 · html > head > meta @content
Element/type: Twitter title
Exact English:

```text
Korea Travel Guide for First-Time Visitors | Korea Inside
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `Korea Inside`.
Source target: `index.html|offset:1330|@content`

ITEM 0007
Page ITEM: 0007
File: `index.html`
Line/context: L20 · html > head > meta @content
Element/type: Twitter description
Exact English:

```text
Practical Korea travel guides for first-time visitors, covering where to stay, airport arrival, eSIMs, transport, payments, maps and local travel decisions.
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `eSIM`.
Source target: `index.html|offset:1429|@content`

ITEM 0008
Page ITEM: 0008
File: `index.html`
Line/context: L33 · JSON-LD $.@graph[0].name
Element/type: JSON-LD user-facing name
Exact English:

```text
Korea Inside
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `Korea Inside`.
Source target: `index.html|jsonld:$.@graph[0].name`

ITEM 0009
Page ITEM: 0009
File: `index.html`
Line/context: L44 · JSON-LD $.@graph[1].name
Element/type: JSON-LD user-facing name
Exact English:

```text
Korea Inside
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `Korea Inside`.
Source target: `index.html|jsonld:$.@graph[1].name`

ITEM 0010
Page ITEM: 0010
File: `index.html`
Line/context: L45 · JSON-LD $.@graph[1].description
Element/type: JSON-LD user-facing description
Exact English:

```text
Practical Korea travel guides for first-time visitors, covering where to stay, airport arrival, eSIMs, transport, payments, maps and local travel decisions.
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `eSIM`.
Source target: `index.html|jsonld:$.@graph[1].description`

ITEM 0011
Page ITEM: 0011
File: `index.html`
Line/context: L54 · JSON-LD $.@graph[2].name
Element/type: JSON-LD user-facing name
Exact English:

```text
Korea Travel Guide for First-Time Visitors | Korea Inside
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `Korea Inside`.
Source target: `index.html|jsonld:$.@graph[2].name`

ITEM 0012
Page ITEM: 0012
File: `index.html`
Line/context: L55 · JSON-LD $.@graph[2].description
Element/type: JSON-LD user-facing description
Exact English:

```text
Practical Korea travel guides for first-time visitors, covering where to stay, airport arrival, eSIMs, transport, payments, maps and local travel decisions.
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `eSIM`.
Source target: `index.html|jsonld:$.@graph[2].description`

ITEM 0013
Page ITEM: 0013
File: `index.html`
Line/context: L56 · JSON-LD $.@graph[2].headline
Element/type: JSON-LD user-facing headline
Exact English:

```text
Practical Korea Travel Guides for First-Time Visitors
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|jsonld:$.@graph[2].headline`

ITEM 0014
Page ITEM: 0014
File: `index.html`
Line/context: L82 · html > body.page-home > header.header > div.container > a.logo @aria-label
Element/type: literal ARIA label
Exact English:

```text
Korea Inside home
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0003`
Protected tokens: Global protection rule applies; preserve exact tokens `Korea Inside`.
Source target: `index.html|offset:4392|@aria-label`

ITEM 0015
Page ITEM: 0015
File: `index.html`
Line/context: L83 · html > body.page-home > header.header > div.container > a.logo > img.site-brand__logo @alt
Element/type: image alt
Exact English:

```text
Korea Inside
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0004`
Protected tokens: Global protection rule applies; preserve exact tokens `Korea Inside`.
Source target: `index.html|offset:4476|@alt`

ITEM 0016
Page ITEM: 0016
File: `index.html`
Line/context: L85 · html > body.page-home > header.header > div.container > button#site-nav-toggle @aria-label
Element/type: literal ARIA label
Exact English:

```text
Open menu
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0005`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:4607|@aria-label`

ITEM 0017
Page ITEM: 0017
File: `index.html`
Line/context: L86 · html > body.page-home > header.header > div.container > nav#site-primary-navigation @aria-label
Element/type: literal ARIA label
Exact English:

```text
Primary navigation
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0006`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:4902|@aria-label`

ITEM 0018
Page ITEM: 0018
File: `index.html`
Line/context: L89 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > button#site-nav-trigger-discover direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
DISCOVER
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0007`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:5265|direct-text`

ITEM 0019
Page ITEM: 0019
File: `index.html`
Line/context: L90 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-discover > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Taste Korea
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0008`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:5484|direct-text`

ITEM 0020
Page ITEM: 0020
File: `index.html`
Line/context: L90 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-discover > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
K-Beauty
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0009`
Protected tokens: Global protection rule applies; preserve exact tokens `K-Beauty`.
Source target: `index.html|offset:5546|direct-text`

ITEM 0021
Page ITEM: 0021
File: `index.html`
Line/context: L93 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > button#site-nav-trigger-travel direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Travel
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0010`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:5824|direct-text`

ITEM 0022
Page ITEM: 0022
File: `index.html`
Line/context: L94 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-travel > p#site-nav-travel-guides-label direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Travel Guides
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0024`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:6092|direct-text`

ITEM 0023
Page ITEM: 0023
File: `index.html`
Line/context: L94 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-travel > div.site-nav__group > p.site-nav__group-label direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Seoul Areas
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0020`
Protected tokens: Global protection rule applies; preserve exact tokens `Seoul`.
Source target: `index.html|offset:6201|direct-text`

ITEM 0024
Page ITEM: 0024
File: `index.html`
Line/context: L94 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-travel > div.site-nav__group > div.site-nav__travel-grid > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Hongdae
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0011`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:6310|direct-text`

ITEM 0025
Page ITEM: 0025
File: `index.html`
Line/context: L94 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-travel > div.site-nav__group > div.site-nav__travel-grid > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Myeongdong
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0012`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:6383|direct-text`

ITEM 0026
Page ITEM: 0026
File: `index.html`
Line/context: L94 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-travel > div.site-nav__group > div.site-nav__travel-grid > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Seongsu
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0013`
Protected tokens: Global protection rule applies; preserve exact tokens `Seongsu`.
Source target: `index.html|offset:6456|direct-text`

ITEM 0027
Page ITEM: 0027
File: `index.html`
Line/context: L94 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-travel > div.site-nav__group > div.site-nav__travel-grid > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Insadong
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0014`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:6527|direct-text`

ITEM 0028
Page ITEM: 0028
File: `index.html`
Line/context: L94 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-travel > div.site-nav__group > div.site-nav__travel-grid > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Gangnam
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0015`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:6598|direct-text`

ITEM 0029
Page ITEM: 0029
File: `index.html`
Line/context: L94 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-travel > div.site-nav__group > div.site-nav__travel-grid > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Jamsil
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0016`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:6667|direct-text`

ITEM 0030
Page ITEM: 0030
File: `index.html`
Line/context: L94 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-travel > div.site-nav__group > div.site-nav__travel-grid > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Gongdeok & Mapo
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0017`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:6741|direct-text`

ITEM 0031
Page ITEM: 0031
File: `index.html`
Line/context: L94 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-travel > div.site-nav__group > div.site-nav__travel-grid > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Itaewon
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0018`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:6823|direct-text`

ITEM 0032
Page ITEM: 0032
File: `index.html`
Line/context: L94 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-travel > div.site-nav__group > div.site-nav__travel-grid > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Dongdaemun
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0019`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:6896|direct-text`

ITEM 0033
Page ITEM: 0033
File: `index.html`
Line/context: L94 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-travel > div.site-nav__group > p.site-nav__group-label direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Attractions
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0023`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:7020|direct-text`

ITEM 0034
Page ITEM: 0034
File: `index.html`
Line/context: L94 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-travel > div.site-nav__group > div.site-nav__travel-attraction-grid > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Lotte World
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0021`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:7138|direct-text`

ITEM 0035
Page ITEM: 0035
File: `index.html`
Line/context: L94 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-travel > div.site-nav__group > div.site-nav__travel-attraction-grid > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Seoul Sky
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0022`
Protected tokens: Global protection rule applies; preserve exact tokens `Seoul`.
Source target: `index.html|offset:7207|direct-text`

ITEM 0036
Page ITEM: 0036
File: `index.html`
Line/context: L97 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > button#site-nav-trigger-stay direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Stay
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0025`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:7486|direct-text`

ITEM 0037
Page ITEM: 0037
File: `index.html`
Line/context: L98 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-stay > div.site-nav__group > p.site-nav__group-label direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Stay Guide
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0026`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:7676|direct-text`

ITEM 0038
Page ITEM: 0038
File: `index.html`
Line/context: L98 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-stay > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Stay Guide
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0036`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:7742|direct-text`

ITEM 0039
Page ITEM: 0039
File: `index.html`
Line/context: L98 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-stay > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Hongdae vs Myeongdong
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0028`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:7816|direct-text`

ITEM 0040
Page ITEM: 0040
File: `index.html`
Line/context: L98 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-stay > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
First-Time Visitors
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0029`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:7919|direct-text`

ITEM 0041
Page ITEM: 0041
File: `index.html`
Line/context: L98 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-stay > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Families
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0030`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:8009|direct-text`

ITEM 0042
Page ITEM: 0042
File: `index.html`
Line/context: L98 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-stay > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Solo Travelers
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0031`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:8094|direct-text`

ITEM 0043
Page ITEM: 0043
File: `index.html`
Line/context: L98 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-stay > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Couples
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0032`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:8178|direct-text`

ITEM 0044
Page ITEM: 0044
File: `index.html`
Line/context: L98 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-stay > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Budget Travelers
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0033`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:8264|direct-text`

ITEM 0045
Page ITEM: 0045
File: `index.html`
Line/context: L98 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-stay > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Shopping
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0034`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:8351|direct-text`

ITEM 0046
Page ITEM: 0046
File: `index.html`
Line/context: L98 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-stay > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Nightlife
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0035`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:8431|direct-text`

ITEM 0047
Page ITEM: 0047
File: `index.html`
Line/context: L98 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-stay > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Luxury Hotels
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0027`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:8516|direct-text`

ITEM 0048
Page ITEM: 0048
File: `index.html`
Line/context: L101 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > button#site-nav-trigger-esim direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
eSIM
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0037`
Protected tokens: Global protection rule applies; preserve exact tokens `eSIM`.
Source target: `index.html|offset:8793|direct-text`

ITEM 0049
Page ITEM: 0049
File: `index.html`
Line/context: L102 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-esim > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
eSIM Guide
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0038`
Protected tokens: Global protection rule applies; preserve exact tokens `eSIM`.
Source target: `index.html|offset:8993|direct-text`

ITEM 0050
Page ITEM: 0050
File: `index.html`
Line/context: L102 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-esim > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Best eSIM for Korea
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0039`
Protected tokens: Global protection rule applies; preserve exact tokens `eSIM`.
Source target: `index.html|offset:9065|direct-text`

ITEM 0051
Page ITEM: 0051
File: `index.html`
Line/context: L102 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-esim > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Korea eSIM with a Phone Number
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0040`
Protected tokens: Global protection rule applies; preserve exact tokens `eSIM`.
Source target: `index.html|offset:9155|direct-text`

ITEM 0052
Page ITEM: 0052
File: `index.html`
Line/context: L105 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > button#site-nav-trigger-airport direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Airport
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0041`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:9458|direct-text`

ITEM 0053
Page ITEM: 0053
File: `index.html`
Line/context: L106 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-airport > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Airport Guide
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0042`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:9670|direct-text`

ITEM 0054
Page ITEM: 0054
File: `index.html`
Line/context: L106 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-airport > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Arrival Guide
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0043`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:9733|direct-text`

ITEM 0055
Page ITEM: 0055
File: `index.html`
Line/context: L106 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-airport > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Airport Transfer
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0044`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:9805|direct-text`

ITEM 0056
Page ITEM: 0056
File: `index.html`
Line/context: L106 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-airport > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
AREX Guide
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0045`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:9868|direct-text`

ITEM 0057
Page ITEM: 0057
File: `index.html`
Line/context: L106 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-airport > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Airport Bus Guide
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0046`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:9932|direct-text`

ITEM 0058
Page ITEM: 0058
File: `index.html`
Line/context: L109 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > button#site-nav-trigger-maps direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Maps
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0047`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:10213|direct-text`

ITEM 0059
Page ITEM: 0059
File: `index.html`
Line/context: L110 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-maps > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Maps Guide
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0048`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:10413|direct-text`

ITEM 0060
Page ITEM: 0060
File: `index.html`
Line/context: L113 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > button#site-nav-trigger-transport direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Transport
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0049`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:10702|direct-text`

ITEM 0061
Page ITEM: 0061
File: `index.html`
Line/context: L114 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-transport > div.site-nav__group > p.site-nav__group-label direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Travel Cards
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0053`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:10928|direct-text`

ITEM 0062
Page ITEM: 0062
File: `index.html`
Line/context: L114 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-transport > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
T-money Guide
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0050`
Protected tokens: Global protection rule applies; preserve exact tokens `T-money`.
Source target: `index.html|offset:10989|direct-text`

ITEM 0063
Page ITEM: 0063
File: `index.html`
Line/context: L114 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-transport > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
WOWPASS Guide
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0051`
Protected tokens: Global protection rule applies; preserve exact tokens `WOWPASS`.
Source target: `index.html|offset:11052|direct-text`

ITEM 0064
Page ITEM: 0064
File: `index.html`
Line/context: L114 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-transport > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
T-money vs WOWPASS
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0052`
Protected tokens: Global protection rule applies; preserve exact tokens `T-money`, `WOWPASS`.
Source target: `index.html|offset:11125|direct-text`

ITEM 0065
Page ITEM: 0065
File: `index.html`
Line/context: L114 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-transport > div.site-nav__group > p.site-nav__group-label direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Other Transport
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0057`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:11215|direct-text`

ITEM 0066
Page ITEM: 0066
File: `index.html`
Line/context: L114 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-transport > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Taxi Guide
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0054`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:11277|direct-text`

ITEM 0067
Page ITEM: 0067
File: `index.html`
Line/context: L114 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-transport > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Call Van / Private Transfer
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0055`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:11362|direct-text`

ITEM 0068
Page ITEM: 0068
File: `index.html`
Line/context: L114 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-transport > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Rental Car
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0056`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:11442|direct-text`

ITEM 0069
Page ITEM: 0069
File: `index.html`
Line/context: L117 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > button#site-nav-trigger-apps direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Apps
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0058`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:11716|direct-text`

ITEM 0070
Page ITEM: 0070
File: `index.html`
Line/context: L118 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-apps > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Essential Apps
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0059`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:11937|direct-text`

ITEM 0071
Page ITEM: 0071
File: `index.html`
Line/context: L121 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > button#site-nav-trigger-travel-tips direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Travel Tips
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0060`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:12236|direct-text`

ITEM 0072
Page ITEM: 0072
File: `index.html`
Line/context: L122 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-travel-tips > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Korea Travel Checklist
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0061`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:12483|direct-text`

ITEM 0073
Page ITEM: 0073
File: `index.html`
Line/context: L122 · html > body.page-home > header.header > div.container > nav#site-primary-navigation > ul.site-nav__list > li.site-nav__item > div#site-nav-panel-travel-tips > div.site-nav__group > a.site-nav__link direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Paying in Korea
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0062`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:12556|direct-text`

ITEM 0074
Page ITEM: 0074
File: `index.html`
Line/context: L126 · html > body.page-home > header.header > div.container > div.language-switcher @aria-label
Element/type: literal ARIA label
Exact English:

```text
Language selector
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0065`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:12641|@aria-label`

ITEM 0075
Page ITEM: 0075
File: `index.html`
Line/context: L126 · html > body.page-home > header.header > div.container > div.language-switcher > button.language-switcher__button > span.language-switcher__current direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
EN
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0063`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:12872|direct-text`

ITEM 0076
Page ITEM: 0076
File: `index.html`
Line/context: L126 · html > body.page-home > header.header > div.container > div.language-switcher > button.language-switcher__button > span.language-switcher__label direct text node
Element/type: common UI header/navigation direct text
Exact English:

```text
Language
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0064`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:12920|direct-text`

ITEM 0077
Page ITEM: 0077
File: `index.html`
Line/context: L135 · html > body.page-home > main > section.hero-banner > picture.hero-banner__media > img.hero-banner__image @alt
Element/type: image alt
Exact English:

```text
N Seoul Tower above the illuminated Hanyangdoseong city wall at dusk in Seoul
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `N Seoul Tower`, `Hanyangdoseong`, `Seoul`.
Source target: `index.html|offset:13246|@alt`

ITEM 0078
Page ITEM: 0078
File: `index.html`
Line/context: L146 · html > body.page-home > main > section.hero-banner > div.container > h1.hero-banner__title direct text node
Element/type: H1 direct text
Exact English:

```text
Korea Travel Guides for Your First Trip
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:13741|direct-text`

ITEM 0079
Page ITEM: 0079
File: `index.html`
Line/context: L147 · html > body.page-home > main > section.hero-banner > div.container > p.hero-banner__subtitle direct text node
Element/type: body direct text
Exact English:

```text
A first trip to Korea feels much easier when a few practical things are sorted out before you land. Knowing where you'll stay, how you'll get from the airport, how you'll get online, and what you'll use for transport and payments takes a lot of stress out of the first day.
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:13828|direct-text`

ITEM 0080
Page ITEM: 0080
File: `index.html`
Line/context: L151 · html > body.page-home > main > section.hero-banner > p.hero-banner__credit direct text node
Element/type: body direct text
Exact English:

```text
Photo:
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:14180|direct-text`

ITEM 0081
Page ITEM: 0081
File: `index.html`
Line/context: L151 · html > body.page-home > main > section.hero-banner > p.hero-banner__credit > a direct text node
Element/type: visible link/CTA text
Exact English:

```text
Seoul Tourism Organization
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `Seoul Tourism Organization`, `Seoul`.
Source target: `index.html|offset:14292|direct-text`

ITEM 0082
Page ITEM: 0082
File: `index.html`
Line/context: L157 · html > body.page-home > main > section.home-discover > div.container > div.home-discover-intro > h2#home-discover-title direct text node
Element/type: H2 direct text
Exact English:

```text
Start with what you came for.
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:14534|direct-text`

ITEM 0083
Page ITEM: 0083
File: `index.html`
Line/context: L158 · html > body.page-home > main > section.home-discover > div.container > div.home-discover-intro > p direct text node
Element/type: body direct text
Exact English:

```text
Some trips begin with food. Others are about shopping, beauty, nightlife, or simply spending time in a neighborhood you've wanted to see. Once you know what you're most excited about, the rest of the itinerary becomes easier to shape.
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:14583|direct-text`

ITEM 0084
Page ITEM: 0084
File: `index.html`
Line/context: L163 · html > body.page-home > main > section.home-discover > div.container > article.home-taste-feature > div.home-taste-feature-media > img @alt
Element/type: image alt
Exact English:

```text
A Korean bibimbap meal representing food travel experiences in Korea
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `Korean`.
Source target: `index.html|offset:14949|@alt`

ITEM 0085
Page ITEM: 0085
File: `index.html`
Line/context: L173 · html > body.page-home > main > section.home-discover > div.container > article.home-taste-feature > div.home-taste-feature-copy > h3 direct text node
Element/type: H3 direct text
Exact English:

```text
Taste Korea
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:15330|direct-text`

ITEM 0086
Page ITEM: 0086
File: `index.html`
Line/context: L174 · html > body.page-home > main > section.home-discover > div.container > article.home-taste-feature > div.home-taste-feature-copy > p direct text node
Element/type: body direct text
Exact English:

```text
Korean food changes from neighborhood to neighborhood and city to city. A barbecue night in Seoul, a market morning in Busan, or a café day in Seongsu can lead to very different places—and sometimes a different place to stay.
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `Seongsu`, `Busan`, `Seoul`, `Korean`.
Source target: `index.html|offset:15363|direct-text`

ITEM 0087
Page ITEM: 0087
File: `index.html`
Line/context: L175 · html > body.page-home > main > section.home-discover > div.container > article.home-taste-feature > div.home-taste-feature-copy > a.home-taste-feature__link direct text node
Element/type: visible link/CTA text
Exact English:

```text
Explore Korean food
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `Korean`.
Source target: `index.html|offset:15666|direct-text`

ITEM 0088
Page ITEM: 0088
File: `index.html`
Line/context: L181 · html > body.page-home > main > section.home-discover > div.container > article.home-kbeauty-row > div.home-kbeauty-row__media > img @alt
Element/type: image alt
Exact English:

```text
K-Beauty makeup products displayed for product and shade comparison
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `K-Beauty`.
Source target: `index.html|offset:15836|@alt`

ITEM 0089
Page ITEM: 0089
File: `index.html`
Line/context: L191 · html > body.page-home > main > section.home-discover > div.container > article.home-kbeauty-row > div.home-kbeauty-row__copy > h3 direct text node
Element/type: H3 direct text
Exact English:

```text
K-Beauty
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `K-Beauty`.
Source target: `index.html|offset:16221|direct-text`

ITEM 0090
Page ITEM: 0090
File: `index.html`
Line/context: L192 · html > body.page-home > main > section.home-discover > div.container > article.home-kbeauty-row > div.home-kbeauty-row__copy > p direct text node
Element/type: body direct text
Exact English:

```text
Korean beauty shopping is easy to overcomplicate because the biggest stores carry far more than most travelers need. The useful part is knowing where to browse, what changes between neighborhoods, and when a flagship store is actually worth the detour.
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `Korean`.
Source target: `index.html|offset:16251|direct-text`

ITEM 0091
Page ITEM: 0091
File: `index.html`
Line/context: L193 · html > body.page-home > main > section.home-discover > div.container > article.home-kbeauty-row > div.home-kbeauty-row__copy > a.home-kbeauty-row__link direct text node
Element/type: visible link/CTA text
Exact English:

```text
Explore K-Beauty in Korea
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `K-Beauty`.
Source target: `index.html|offset:16576|direct-text`

ITEM 0092
Page ITEM: 0092
File: `index.html`
Line/context: L202 · html > body.page-home > main > section.home-journey > div.container > div.home-journey__header > h2#home-journey-title direct text node
Element/type: H2 direct text
Exact English:

```text
A few decisions make arrival day much easier
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:16863|direct-text`

ITEM 0093
Page ITEM: 0093
File: `index.html`
Line/context: L203 · html > body.page-home > main > section.home-journey > div.container > div.home-journey__header > p direct text node
Element/type: body direct text
Exact English:

```text
You do not need a finished itinerary before flying to Korea. But three things are worth settling early: where you will stay, how your phone will get online, and how you will get from the airport to your first accommodation.
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:16927|direct-text`

ITEM 0094
Page ITEM: 0094
File: `index.html`
Line/context: L209 · html > body.page-home > main > section.home-journey > div.container > div.home-journey__grid > article.home-journey__row > div.home-journey__media > img.home-journey__image @alt
Element/type: image alt
Exact English:

```text
Accommodation planning card for choosing where to stay in Korea
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:17324|@alt`

ITEM 0095
Page ITEM: 0095
File: `index.html`
Line/context: L217 · html > body.page-home > main > section.home-journey > div.container > div.home-journey__grid > article.home-journey__row > div.home-journey__body > h3 direct text node
Element/type: H3 direct text
Exact English:

```text
Where you stay changes more than the hotel
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:17666|direct-text`

ITEM 0096
Page ITEM: 0096
File: `index.html`
Line/context: L218 · html > body.page-home > main > section.home-journey > div.container > div.home-journey__grid > article.home-journey__row > div.home-journey__body > p direct text node
Element/type: body direct text
Exact English:

```text
A convenient base can save more time than choosing a hotel for price alone. Subway access matters, but so do airport arrival, late-night returns, shopping bags, and how often you expect to cross the city.
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:17732|direct-text`

ITEM 0097
Page ITEM: 0097
File: `index.html`
Line/context: L219 · html > body.page-home > main > section.home-journey > div.container > div.home-journey__grid > article.home-journey__row > div.home-journey__body > a.home-journey__link direct text node
Element/type: visible link/CTA text
Exact English:

```text
Find the right area to stay
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:18012|direct-text`

ITEM 0098
Page ITEM: 0098
File: `index.html`
Line/context: L225 · html > body.page-home > main > section.home-journey > div.container > div.home-journey__grid > article.home-journey__row > div.home-journey__media > img.home-journey__image @alt
Element/type: image alt
Exact English:

```text
Mobile data setup for staying connected in Korea
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:18224|@alt`

ITEM 0099
Page ITEM: 0099
File: `index.html`
Line/context: L233 · html > body.page-home > main > section.home-journey > div.container > div.home-journey__grid > article.home-journey__row > div.home-journey__body > h3 direct text node
Element/type: H3 direct text
Exact English:

```text
Have your phone ready before you need it
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:18529|direct-text`

ITEM 0100
Page ITEM: 0100
File: `index.html`
Line/context: L234 · html > body.page-home > main > section.home-journey > div.container > div.home-journey__grid > article.home-journey__row > div.home-journey__body > p direct text node
Element/type: body direct text
Exact English:

```text
Mobile data becomes useful almost immediately after landing—for maps, hotel directions, messages, reservations, and translation. An eSIM is convenient for many travelers, but phone compatibility and activation timing are worth checking before departure.
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `eSIM`.
Source target: `index.html|offset:18593|direct-text`

ITEM 0101
Page ITEM: 0101
File: `index.html`
Line/context: L235 · html > body.page-home > main > section.home-journey > div.container > div.home-journey__grid > article.home-journey__row > div.home-journey__body > a.home-journey__link direct text node
Element/type: visible link/CTA text
Exact English:

```text
Read the Korea eSIM guide
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `eSIM`.
Source target: `index.html|offset:18913|direct-text`

ITEM 0102
Page ITEM: 0102
File: `index.html`
Line/context: L241 · html > body.page-home > main > section.home-journey > div.container > div.home-journey__grid > article.home-journey__row > div.home-journey__media > img.home-journey__image @alt
Element/type: image alt
Exact English:

```text
Airport arrival guide for first steps after landing in Korea
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:19096|@alt`

ITEM 0103
Page ITEM: 0103
File: `index.html`
Line/context: L249 · html > body.page-home > main > section.home-journey > div.container > div.home-journey__grid > article.home-journey__row > div.home-journey__body > h3 direct text node
Element/type: H3 direct text
Exact English:

```text
Plan the trip from the airport to your actual hotel
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:19417|direct-text`

ITEM 0104
Page ITEM: 0104
File: `index.html`
Line/context: L250 · html > body.page-home > main > section.home-journey > div.container > div.home-journey__grid > article.home-journey__row > div.home-journey__body > p direct text node
Element/type: body direct text
Exact English:

```text
The fastest airport option is not always the easiest one. Arrival time, luggage, transfers, walking distance from the station, and your hotel location can matter more than a few minutes of travel time.
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:19492|direct-text`

ITEM 0105
Page ITEM: 0105
File: `index.html`
Line/context: L252 · html > body.page-home > main > section.home-journey > div.container > div.home-journey__grid > article.home-journey__row > div.home-journey__body > div.home-journey__links > a.home-journey__link direct text node
Element/type: visible link/CTA text
Exact English:

```text
Compare airport transfer options
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:19823|direct-text`

ITEM 0106
Page ITEM: 0106
File: `index.html`
Line/context: L253 · html > body.page-home > main > section.home-journey > div.container > div.home-journey__grid > article.home-journey__row > div.home-journey__body > div.home-journey__links > a.home-journey__link direct text node
Element/type: visible link/CTA text
Exact English:

```text
See the full arrival guide
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:19957|direct-text`

ITEM 0107
Page ITEM: 0107
File: `index.html`
Line/context: L265 · html > body.page-home > main > section.section > div.container > div.home-prep-grid > div.first-hour-panel > h2.section__title direct text node
Element/type: H2 direct text
Exact English:

```text
Your first hour in Korea
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:20314|direct-text`

ITEM 0108
Page ITEM: 0108
File: `index.html`
Line/context: L266 · html > body.page-home > main > section.section > div.container > div.home-prep-grid > div.first-hour-panel > p.first-hour-panel__intro > a direct text node
Element/type: visible link/CTA text
Exact English:

```text
After baggage claim, most travelers need the same few things
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:20417|direct-text`

ITEM 0109
Page ITEM: 0109
File: `index.html`
Line/context: L266 · html > body.page-home > main > section.section > div.container > div.home-prep-grid > div.first-hour-panel > p.first-hour-panel__intro direct text node
Element/type: body direct text
Exact English:

```text
: a working phone, a way to pay for local transport, a map that works well in Korea, and a clear route to the city. Once those are sorted, there is usually no reason to spend much longer at the airport.
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:20481|direct-text`

ITEM 0110
Page ITEM: 0110
File: `index.html`
Line/context: L268 · html > body.page-home > main > section.section > div.container > div.home-prep-grid > div.first-hour-panel > ol.first-hour-list > li > div > span direct text node
Element/type: list direct text
Exact English:

```text
Get your phone online
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:20760|direct-text`

ITEM 0111
Page ITEM: 0111
File: `index.html`
Line/context: L268 · html > body.page-home > main > section.section > div.container > div.home-prep-grid > div.first-hour-panel > ol.first-hour-list > li > div > a.first-hour-list__link direct text node
Element/type: visible link/CTA text
Exact English:

```text
Korea eSIM guide
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `eSIM`.
Source target: `index.html|offset:20838|direct-text`

ITEM 0112
Page ITEM: 0112
File: `index.html`
Line/context: L269 · html > body.page-home > main > section.section > div.container > div.home-prep-grid > div.first-hour-panel > ol.first-hour-list > li > div > span direct text node
Element/type: list direct text
Exact English:

```text
Get or load T-money
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `T-money`.
Source target: `index.html|offset:20900|direct-text`

ITEM 0113
Page ITEM: 0113
File: `index.html`
Line/context: L269 · html > body.page-home > main > section.section > div.container > div.home-prep-grid > div.first-hour-panel > ol.first-hour-list > li > div > a.first-hour-list__link direct text node
Element/type: visible link/CTA text
Exact English:

```text
T-money guide
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `T-money`.
Source target: `index.html|offset:20978|direct-text`

ITEM 0114
Page ITEM: 0114
File: `index.html`
Line/context: L270 · html > body.page-home > main > section.section > div.container > div.home-prep-grid > div.first-hour-panel > ol.first-hour-list > li > div > span direct text node
Element/type: list direct text
Exact English:

```text
Keep a small amount of cash as backup
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:21037|direct-text`

ITEM 0115
Page ITEM: 0115
File: `index.html`
Line/context: L270 · html > body.page-home > main > section.section > div.container > div.home-prep-grid > div.first-hour-panel > ol.first-hour-list > li > div > a.first-hour-list__link direct text node
Element/type: visible link/CTA text
Exact English:

```text
Payment guide
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:21135|direct-text`

ITEM 0116
Page ITEM: 0116
File: `index.html`
Line/context: L270 · html > body.page-home > main > section.section > div.container > div.home-prep-grid > div.first-hour-panel > ol.first-hour-list > li > div > a.first-hour-list__link direct text node
Element/type: visible link/CTA text
Exact English:

```text
WOWPASS guide
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `WOWPASS`.
Source target: `index.html|offset:21205|direct-text`

ITEM 0117
Page ITEM: 0117
File: `index.html`
Line/context: L271 · html > body.page-home > main > section.section > div.container > div.home-prep-grid > div.first-hour-panel > ol.first-hour-list > li > div > span direct text node
Element/type: list direct text
Exact English:

```text
Open Naver Map or KakaoMap
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; preserve exact tokens `Naver Map`, `KakaoMap`.
Source target: `index.html|offset:21264|direct-text`

ITEM 0118
Page ITEM: 0118
File: `index.html`
Line/context: L271 · html > body.page-home > main > section.section > div.container > div.home-prep-grid > div.first-hour-panel > ol.first-hour-list > li > div > a.first-hour-list__link direct text node
Element/type: visible link/CTA text
Exact English:

```text
Map apps in Korea
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:21347|direct-text`

ITEM 0119
Page ITEM: 0119
File: `index.html`
Line/context: L272 · html > body.page-home > main > section.section > div.container > div.home-prep-grid > div.first-hour-panel > ol.first-hour-list > li > div > span direct text node
Element/type: list direct text
Exact English:

```text
Take the airport route that makes sense for your hotel
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:21410|direct-text`

ITEM 0120
Page ITEM: 0120
File: `index.html`
Line/context: L272 · html > body.page-home > main > section.section > div.container > div.home-prep-grid > div.first-hour-panel > ol.first-hour-list > li > div > a.first-hour-list__link direct text node
Element/type: visible link/CTA text
Exact English:

```text
Airport transfer guide
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:21533|direct-text`

ITEM 0121
Page ITEM: 0121
File: `index.html`
Line/context: L274 · html > body.page-home > main > section.section > div.container > div.home-prep-grid > div.first-hour-panel > a.btn direct text node
Element/type: visible link/CTA text
Exact English:

```text
See the full airport guide →
```

Reuse status: NEW — homepage-specific localization required
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:21648|direct-text`

ITEM 0122
Page ITEM: 0122
File: `index.html`
Line/context: L286 · html > body.page-home > footer.footer > div.container > div.footer__grid > div.footer__brand-block > p.footer__brand direct text node
Element/type: common UI footer direct text
Exact English:

```text
Korea Inside
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0535`
Protected tokens: Global protection rule applies; preserve exact tokens `Korea Inside`.
Source target: `index.html|offset:21956|direct-text`

ITEM 0123
Page ITEM: 0123
File: `index.html`
Line/context: L287 · html > body.page-home > footer.footer > div.container > div.footer__grid > div.footer__brand-block > p.footer__eyebrow direct text node
Element/type: common UI footer direct text
Exact English:

```text
CREATED IN KOREA
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0536`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:22011|direct-text`

ITEM 0124
Page ITEM: 0124
File: `index.html`
Line/context: L288 · html > body.page-home > footer.footer > div.container > div.footer__grid > div.footer__brand-block > p.footer__statement direct text node
Element/type: common UI footer direct text
Exact English:

```text
Practical Korea travel guidance, written and reviewed locally by a Korean editor.
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0537`
Protected tokens: Global protection rule applies; preserve exact tokens `Korean`.
Source target: `index.html|offset:22072|direct-text`

ITEM 0125
Page ITEM: 0125
File: `index.html`
Line/context: L289 · html > body.page-home > footer.footer > div.container > div.footer__grid > div.footer__brand-block > p.footer__description direct text node
Element/type: common UI footer direct text
Exact English:

```text
Based on official sources, local context, and independent editorial judgment.
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0538`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:22200|direct-text`

ITEM 0126
Page ITEM: 0126
File: `index.html`
Line/context: L291 · html > body.page-home > footer.footer > div.container > div.footer__grid > nav.footer__nav @aria-label
Element/type: literal ARIA label
Exact English:

```text
Footer navigation
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0539`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:22307|@aria-label`

ITEM 0127
Page ITEM: 0127
File: `index.html`
Line/context: L293 · html > body.page-home > footer.footer > div.container > div.footer__grid > nav.footer__nav > div.footer__group > p.footer__heading direct text node
Element/type: common UI footer direct text
Exact English:

```text
PLAN YOUR TRIP
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0540`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:22443|direct-text`

ITEM 0128
Page ITEM: 0128
File: `index.html`
Line/context: L295 · html > body.page-home > footer.footer > div.container > div.footer__grid > nav.footer__nav > div.footer__group > ul.footer__links > li > a direct text node
Element/type: common UI footer direct text
Exact English:

```text
Airport
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0541`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:22544|direct-text`

ITEM 0129
Page ITEM: 0129
File: `index.html`
Line/context: L296 · html > body.page-home > footer.footer > div.container > div.footer__grid > nav.footer__nav > div.footer__group > ul.footer__links > li > a direct text node
Element/type: common UI footer direct text
Exact English:

```text
eSIM
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0542`
Protected tokens: Global protection rule applies; preserve exact tokens `eSIM`.
Source target: `index.html|offset:22600|direct-text`

ITEM 0130
Page ITEM: 0130
File: `index.html`
Line/context: L297 · html > body.page-home > footer.footer > div.container > div.footer__grid > nav.footer__nav > div.footer__group > ul.footer__links > li > a direct text node
Element/type: common UI footer direct text
Exact English:

```text
Checklist
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0543`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:22658|direct-text`

ITEM 0131
Page ITEM: 0131
File: `index.html`
Line/context: L301 · html > body.page-home > footer.footer > div.container > div.footer__grid > nav.footer__nav > div.footer__group > p.footer__heading direct text node
Element/type: common UI footer direct text
Exact English:

```text
USE KOREA
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0544`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:22793|direct-text`

ITEM 0132
Page ITEM: 0132
File: `index.html`
Line/context: L303 · html > body.page-home > footer.footer > div.container > div.footer__grid > nav.footer__nav > div.footer__group > ul.footer__links > li > a direct text node
Element/type: common UI footer direct text
Exact English:

```text
T-money
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0545`
Protected tokens: Global protection rule applies; preserve exact tokens `T-money`.
Source target: `index.html|offset:22888|direct-text`

ITEM 0133
Page ITEM: 0133
File: `index.html`
Line/context: L304 · html > body.page-home > footer.footer > div.container > div.footer__grid > nav.footer__nav > div.footer__group > ul.footer__links > li > a direct text node
Element/type: common UI footer direct text
Exact English:

```text
Payments
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0546`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:22948|direct-text`

ITEM 0134
Page ITEM: 0134
File: `index.html`
Line/context: L305 · html > body.page-home > footer.footer > div.container > div.footer__grid > nav.footer__nav > div.footer__group > ul.footer__links > li > a direct text node
Element/type: common UI footer direct text
Exact English:

```text
Maps
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0547`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:23005|direct-text`

ITEM 0135
Page ITEM: 0135
File: `index.html`
Line/context: L306 · html > body.page-home > footer.footer > div.container > div.footer__grid > nav.footer__nav > div.footer__group > ul.footer__links > li > a direct text node
Element/type: common UI footer direct text
Exact English:

```text
Apps
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0548`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:23058|direct-text`

ITEM 0136
Page ITEM: 0136
File: `index.html`
Line/context: L312 · html > body.page-home > footer.footer > div.container > div.footer__bottom > p direct text node
Element/type: common UI footer direct text
Exact English:

```text
© 2026 Korea Inside · Republic of Korea
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0549`
Protected tokens: Global protection rule applies; preserve exact tokens `Korea Inside`, `2026`.
Source target: `index.html|offset:23187|direct-text`

ITEM 0137
Page ITEM: 0137
File: `index.html`
Line/context: L313 · html > body.page-home > footer.footer > div.container > div.footer__bottom > p.footer__business direct text node
Element/type: common UI footer direct text
Exact English:

```text
Business Registration No. 462-39-01721
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0553`
Protected tokens: Global protection rule applies; preserve exact tokens `462-39-01721`.
Source target: `index.html|offset:23273|direct-text`

ITEM 0138
Page ITEM: 0138
File: `index.html`
Line/context: L313 · html > body.page-home > footer.footer > div.container > div.footer__bottom > p.footer__business direct text node
Element/type: common UI footer direct text
Exact English:

```text
Contact:
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0554`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:23345|direct-text`

ITEM 0139
Page ITEM: 0139
File: `index.html`
Line/context: L313 · html > body.page-home > footer.footer > div.container > div.footer__bottom > p.footer__business > a direct text node
Element/type: common UI footer direct text
Exact English:

```text
getkoreainside@gmail.com
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0552`
Protected tokens: Global protection rule applies; preserve exact tokens `getkoreainside@gmail.com`.
Source target: `index.html|offset:23397|direct-text`

ITEM 0140
Page ITEM: 0140
File: `index.html`
Line/context: L313 · html > body.page-home > footer.footer > div.container > div.footer__bottom > p.footer__business > a.footer__legal-link direct text node
Element/type: common UI footer direct text
Exact English:

```text
Affiliate Disclosure
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0550`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:23524|direct-text`

ITEM 0141
Page ITEM: 0141
File: `index.html`
Line/context: L313 · html > body.page-home > footer.footer > div.container > div.footer__bottom > p.footer__business > a.footer__legal-link direct text node
Element/type: common UI footer direct text
Exact English:

```text
Privacy Policy
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0551`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `index.html|offset:23634|direct-text`

ITEM 0142
Page ITEM: 0142
File: `common.js (shared runtime used by homepage)`
Line/context: L348 · common.js install dialog close ARIA
Element/type: shared common UI runtime string
Exact English:

```text
Close
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0684`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `common.js|runtime:install dialog close ARIA`

ITEM 0143
Page ITEM: 0143
File: `common.js (shared runtime used by homepage)`
Line/context: L9 · common.js navigation closed-state ARIA
Element/type: shared common UI runtime string
Exact English:

```text
Open menu
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0685`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `common.js|runtime:navigation closed-state ARIA`

ITEM 0144
Page ITEM: 0144
File: `common.js (shared runtime used by homepage)`
Line/context: L9 · common.js navigation open-state ARIA
Element/type: shared common UI runtime string
Exact English:

```text
Close menu
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0686`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `common.js|runtime:navigation open-state ARIA`

ITEM 0145
Page ITEM: 0145
File: `common.js (shared runtime used by homepage)`
Line/context: L164 · common.js language option en
Element/type: shared common UI runtime string
Exact English:

```text
English
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0687`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `common.js|runtime:language option en`

ITEM 0146
Page ITEM: 0146
File: `common.js (shared runtime used by homepage)`
Line/context: L165 · common.js language option es
Element/type: shared common UI runtime string
Exact English:

```text
Español
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0688`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `common.js|runtime:language option es`

ITEM 0147
Page ITEM: 0147
File: `common.js (shared runtime used by homepage)`
Line/context: L347 · common.js install button
Element/type: shared common UI runtime string
Exact English:

```text
Install Korea Inside
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0689`
Protected tokens: Global protection rule applies; preserve exact tokens `Korea Inside`.
Source target: `common.js|runtime:install button`

ITEM 0148
Page ITEM: 0148
File: `common.js (shared runtime used by homepage)`
Line/context: L349 · common.js iOS install title
Element/type: shared common UI runtime string
Exact English:

```text
Add Korea Inside to your Home Screen
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0690`
Protected tokens: Global protection rule applies; preserve exact tokens `Korea Inside`.
Source target: `common.js|runtime:iOS install title`

ITEM 0149
Page ITEM: 0149
File: `common.js (shared runtime used by homepage)`
Line/context: L350 · common.js iOS install step 1
Element/type: shared common UI runtime string
Exact English:

```text
Tap Share
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0691`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `common.js|runtime:iOS install step 1`

ITEM 0150
Page ITEM: 0150
File: `common.js (shared runtime used by homepage)`
Line/context: L350 · common.js iOS install step 2
Element/type: shared common UI runtime string
Exact English:

```text
Tap Add to Home Screen
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0692`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `common.js|runtime:iOS install step 2`

ITEM 0151
Page ITEM: 0151
File: `common.js (shared runtime used by homepage)`
Line/context: L351 · common.js browser install fallback
Element/type: shared common UI runtime string
Exact English:

```text
Open your browser menu and choose “Install app” or “Add to Home screen.”
```

Reuse status: REUSE — Japanese Golden Sample approved common UI
Golden Sample reference: `ITEM 0693`
Protected tokens: Global protection rule applies; no additional exact token isolated.
Source target: `common.js|runtime:browser install fallback`

## Independent Extraction QA Ledger

- Homepage ITEM count: 151
- Common UI reuse ITEM count: 93
- New homepage-specific ITEM count: 58
- ITEM range: ITEM 0001–ITEM 0151
- Number gaps: 0
- Duplicate source targets: 0
- Golden Sample common UI reference mismatches: 0
- Title/meta/OG/Twitter targets: 7; coverage gaps: 0
- H1–H4 targets: 9; coverage gaps: 0
- Body/card/list/CTA/button/link direct-text coverage gaps: 0
- Alt targets: 7; coverage gaps: 0
- Literal ARIA targets: 5; coverage gaps: 0
- Footer/common UI coverage gaps: 0
- JSON-LD user-facing targets: 6; coverage gaps: 0
- User-facing data-* targets: 0; coverage gaps: 0
- Shared runtime-visible targets: 10; coverage gaps: 0
- Direct/leaf text targets audited: 119; coverage gaps: 0
- Page-specific visible English unlisted after independent second pass: 0
- Protected-token extraction errors: 0
- Source SHA-256 mismatch: 0
- English HTML modifications during extraction: 0
- Japanese HTML modifications during extraction: 0

