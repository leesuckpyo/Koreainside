# Korea Inside Multilingual SEO Architecture Strategy

## Purpose

Korea Inside is currently an English-only website. This document defines a production-ready multilingual SEO architecture before translations are implemented.

The goal is to support long-term SEO, scalability, maintainability, browser translation, accessibility, and future localization without rebuilding the website later.

This is a documentation-only strategy. It does not implement translations, change URLs, modify HTML, update navigation, or change site behavior.

---

## 1. Current State Analysis

### Current Architecture

The current site is built as English root-level static HTML pages.

Examples:

```text
/arrival.html
/apps.html
/airport-transfer.html
/esim.html
/maps.html
/tmoney.html
/wowpass.html
```

The site already has a strong practical-information focus. It is structured around visitor problems such as arrival, airport transfer, eSIM setup, maps, payments, and transportation.

### Strengths

| Strength | SEO / Product Value |
|---|---|
| Static HTML pages | Fast, crawlable, simple to host, and easy for search engines to index. |
| English-first content | Clear master language for future translation workflow. |
| Practical topic structure | Aligns with high-intent visitor searches, not generic tourism browsing. |
| Semantic-content direction | Supports SEO, accessibility, and browser translation when important content remains HTML. |
| Existing stable URLs | Current pages can retain search value during future migration planning. |

### Weaknesses

| Weakness | Risk |
|---|---|
| Root-level English URLs only | Future language expansion may require URL migration and canonical decisions. |
| Hard-coded English page text | Translation updates can become manual, inconsistent, and difficult to synchronize. |
| No language folders yet | Harder to introduce clean multilingual sitemaps and hreflang sets later. |
| No translation status model | Hard to know which pages are translated, reviewed, outdated, or blocked. |
| Shared content may be duplicated | Changeable details can drift across pages and languages. |

### Risks of Continuing With Hard-Coded English Pages

Hard-coded English pages are acceptable for the current English-only phase, but they are risky as the final multilingual architecture.

| Risk | Explanation | Mitigation |
|---|---|---|
| Translation drift | English updates may not reach every translated page. | Track source updates and translation review status. |
| Metadata inconsistency | Titles, descriptions, canonicals, and hreflang may differ by language. | Use a page-language inventory and metadata checklist. |
| Duplicate-content confusion | Similar pages without correct canonicals and hreflang can confuse search engines. | Self-canonicalize each language page and use reciprocal hreflang. |
| Browser translation limitations | Important text inside images or scripts may not translate. | Keep all essential information as visible HTML. |
| Maintenance overhead | Every page change may require manual edits in many files. | Standardize page structure and content units before translation. |

---

## 2. URL Architecture

### Options Compared

| Option | Example | SEO | Maintainability | Scalability | Hosting Complexity | User Experience |
|---|---|---|---|---|---|---|
| A. Language subdirectories | `/ja/arrival/` | Strong. Consolidates domain authority and supports clear hreflang. | Strong. One domain, predictable folders, simpler analytics. | Strong. Easy to add languages and sitemaps. | Low. Same host and deployment model. | Strong. Clear, shareable, readable URLs. |
| B. Subdomains | `ja.domain.com/arrival/` | Mixed. Can work, but may split authority signals and Search Console properties. | Medium. More properties, configs, and reporting surfaces. | Medium. Scales, but operations become heavier. | Higher. DNS, hosting, certificates, and analytics per subdomain. | Medium. Clear language separation but more fragmented brand experience. |
| C. Query parameters | `/arrival.html?lang=ja` | Weak. Poor canonical and hreflang clarity, weaker indexability. | Weak. Language state can be inconsistent. | Weak. Harder to manage large multilingual sites. | Low initially, high later. | Weak. Less shareable and less trustworthy-looking URLs. |

### Recommendation

Use language subdirectories.

Recommended long-term structure:

```text
/
|-- en/
|   |-- arrival/
|   |-- esim/
|   |-- tmoney/
|
|-- ja/
|   |-- arrival/
|   |-- esim/
|   |-- tmoney/
|
|-- fr/
|-- de/
|-- zh-tw/
```

Recommended page pattern:

```text
/{language-code}/{page-slug}/
```

Examples:

```text
/en/arrival/
/ja/arrival/
/fr/arrival/
/de/arrival/
/zh-tw/arrival/
```

### Why This Is Best for Korea Inside

Language subdirectories provide the best balance of:

- SEO authority consolidation
- Simple hosting
- Clear analytics
- Predictable internal linking
- Easier sitemap management
- Future language expansion
- User-friendly URLs

Subdomains should be avoided unless Korea Inside later becomes a multi-region operation with separate teams, infrastructure, or market-specific products.

Query-string language switching should not be used for public SEO pages.

---

## 3. Language Structure

### Recommended Initial Languages

| Priority | Language | URL Code | Reason |
|---|---|---|---|
| 1 | English | `/en/` | Master language and current source content. |
| 2 | Japanese | `/ja/` | Strong Korea travel demand, close market, high practical-search intent. |
| 3 | Traditional Chinese | `/zh-tw/` | Important market for Korea travel and practical visitor planning. |
| 4 | French | `/fr/` | Global expansion language with long-term SEO value. |
| 5 | German | `/de/` | Strong outbound travel market and useful for European expansion. |

### Suggested Rollout Order

```text
English master
  -> Japanese
  -> Traditional Chinese
  -> French
  -> German
```

### Language Code Notes

| Audience | Recommended URL | hreflang Option |
|---|---|---|
| English global | `/en/` | `en` |
| Japanese | `/ja/` | `ja` |
| French global | `/fr/` | `fr` |
| German global | `/de/` | `de` |
| Traditional Chinese | `/zh-tw/` | `zh-TW` if Taiwan-focused, `zh-Hant` if script-focused |

Recommendation: use `/zh-tw/` as the URL folder for clarity and market targeting. Use `zh-TW` in hreflang if content is localized for Taiwan users. Use `zh-Hant` only if Korea Inside intentionally targets Traditional Chinese readers broadly beyond Taiwan.

---

## 4. hreflang Strategy

`hreflang` helps search engines understand equivalent pages for different languages or regions.

### Concepts

| Concept | Meaning | Korea Inside Usage |
|---|---|---|
| Language targeting | Targets a language regardless of region. | `en`, `ja`, `fr`, `de` |
| Regional targeting | Targets a language in a specific region. | `zh-TW` for Traditional Chinese users in Taiwan |
| `x-default` | Fallback page when no language or region match is clear. | Usually the English page or language selector page |

### Page-Level Example

For an arrival page:

```html
<link rel="alternate" hreflang="en" href="https://koreainside.com/en/arrival/" />
<link rel="alternate" hreflang="ja" href="https://koreainside.com/ja/arrival/" />
<link rel="alternate" hreflang="fr" href="https://koreainside.com/fr/arrival/" />
<link rel="alternate" hreflang="de" href="https://koreainside.com/de/arrival/" />
<link rel="alternate" hreflang="zh-TW" href="https://koreainside.com/zh-tw/arrival/" />
<link rel="alternate" hreflang="x-default" href="https://koreainside.com/en/arrival/" />
```

### hreflang Rules

- Add hreflang only for published, indexable, canonical pages.
- Every language page must reference all equivalent language pages.
- Hreflang links must be reciprocal.
- Hreflang URLs should be final canonical URLs, not redirects.
- Do not include draft, missing, noindex, or blocked translations.
- Keep page-level hreflang and sitemap hreflang consistent.

### Relationship Diagram

```text
/en/arrival/
  <-> /ja/arrival/
  <-> /fr/arrival/
  <-> /de/arrival/
  <-> /zh-tw/arrival/
  <-> x-default: /en/arrival/
```

---

## 5. Canonical Strategy

Each language page should self-canonicalize.

Do not canonicalize translated pages back to English. Translated pages are not duplicates when they serve different language audiences.

### Canonical Examples

| Page | Canonical |
|---|---|
| `/en/arrival/` | `https://koreainside.com/en/arrival/` |
| `/ja/arrival/` | `https://koreainside.com/ja/arrival/` |
| `/fr/arrival/` | `https://koreainside.com/fr/arrival/` |
| `/de/arrival/` | `https://koreainside.com/de/arrival/` |
| `/zh-tw/arrival/` | `https://koreainside.com/zh-tw/arrival/` |

Example:

```html
<link rel="canonical" href="https://koreainside.com/ja/arrival/" />
```

### Legacy English Root Pages

Current root pages such as `/arrival.html` should be handled in a separate approved migration plan.

Possible options:

| Option | Description | Notes |
|---|---|---|
| Keep root pages | Current English URLs remain live. | Lowest immediate risk, less clean long-term. |
| Redirect to `/en/` | Root pages redirect to new English URLs. | Cleanest final architecture, requires careful SEO migration. |
| Canonical alias | Root pages stay live but canonical to `/en/`. | Transitional option, but can create maintenance duplication. |

Recommended final state: `/en/` should become the canonical English structure after a controlled migration.

---

## 6. Sitemap Strategy

Korea Inside should use a sitemap index plus language-specific sitemaps.

### Recommended Sitemap Structure

```text
/sitemap.xml
/sitemap-index.xml
/sitemaps/sitemap-en.xml
/sitemaps/sitemap-ja.xml
/sitemaps/sitemap-fr.xml
/sitemaps/sitemap-de.xml
/sitemaps/sitemap-zh-tw.xml
```

### Relationship Diagram

```text
sitemap.xml
  -> sitemap-index.xml
      -> sitemap-en.xml
      -> sitemap-ja.xml
      -> sitemap-fr.xml
      -> sitemap-de.xml
      -> sitemap-zh-tw.xml
```

### Sitemap Responsibilities

| Sitemap | Purpose |
|---|---|
| `sitemap.xml` | Public default entry point for search engines. Can point to the sitemap index. |
| `sitemap-index.xml` | Lists all language-specific sitemaps. |
| `sitemap-en.xml` | Lists canonical English URLs. |
| `sitemap-ja.xml` | Lists canonical Japanese URLs. |
| `sitemap-fr.xml` | Lists canonical French URLs. |
| `sitemap-de.xml` | Lists canonical German URLs. |
| `sitemap-zh-tw.xml` | Lists canonical Traditional Chinese URLs. |

### Multilingual URL Entry Example

```xml
<url>
  <loc>https://koreainside.com/en/arrival/</loc>
  <xhtml:link rel="alternate" hreflang="en" href="https://koreainside.com/en/arrival/" />
  <xhtml:link rel="alternate" hreflang="ja" href="https://koreainside.com/ja/arrival/" />
  <xhtml:link rel="alternate" hreflang="fr" href="https://koreainside.com/fr/arrival/" />
  <xhtml:link rel="alternate" hreflang="de" href="https://koreainside.com/de/arrival/" />
  <xhtml:link rel="alternate" hreflang="zh-TW" href="https://koreainside.com/zh-tw/arrival/" />
  <xhtml:link rel="alternate" hreflang="x-default" href="https://koreainside.com/en/arrival/" />
</url>
```

### Sitemap Rules

- Include only published, canonical, indexable URLs.
- Exclude draft translations.
- Exclude pages blocked by robots rules.
- Use `lastmod` only when meaningful page content changes.
- Keep sitemap hreflang consistent with page-level hreflang.

---

## 7. Translation Workflow

English should remain the master language.

```text
English Master
  -> Translation
  -> Review
  -> Publish
  -> Maintenance
```

### Workflow Stages

| Stage | Output | Key Rule |
|---|---|---|
| English Master | Final English source content, metadata, sources, and last-reviewed dates. | English is the source of truth. |
| Translation | Localized content preserving meaning and structure. | Never translate from another translated language. |
| Review | Native or expert review for clarity, terminology, and accuracy. | Practical meaning matters more than literal wording. |
| Publish | Canonical page, hreflang, metadata, sitemap inclusion. | Publish only reviewed pages. |
| Maintenance | Updates, re-review, source verification. | English updates trigger translation review. |

### Update Propagation

```text
English source change
  -> mark translated pages as "review needed"
  -> verify official sources if claim is changeable
  -> update translations
  -> review localized wording
  -> publish
  -> update last-reviewed date
```

### Translation Status Model

| Status | Meaning |
|---|---|
| Planned | Page selected for future translation. |
| In translation | Translation is being prepared. |
| In review | Translation exists but needs review. |
| Published | Page is live and indexable. |
| Needs update | English source changed or official information changed. |
| Paused | Translation is not ready for publication. |

---

## 8. Content Architecture

Future multilingual content should preserve identical hierarchy across languages.

Example content hierarchy:

```text
Where to Stay
  -> Hongdae vs Myeongdong
      -> Family
      -> Shopping
      -> Nightlife
```

### Why Hierarchy Consistency Matters

| Requirement | Reason |
|---|---|
| Same section order | Easier translation review and future synchronization. |
| Same heading hierarchy | Better SEO consistency and accessibility. |
| Same internal-link structure | Helps search engines understand equivalent page relationships. |
| Same decision criteria | Keeps recommendations consistent across languages. |
| Same visible HTML content | Supports browser translation and accessibility. |

### Page Family Example

```text
/en/where-to-stay-in-seoul/
/ja/where-to-stay-in-seoul/
/fr/where-to-stay-in-seoul/
/de/where-to-stay-in-seoul/
/zh-tw/where-to-stay-in-seoul/
```

### Content Inventory Example

| Page Family | English | Japanese | Traditional Chinese | French | German |
|---|---|---|---|---|---|
| Arrival | Published | Planned | Planned | Planned | Planned |
| eSIM | Published | Planned | Planned | Planned | Planned |
| T-money | Published | Planned | Planned | Planned | Planned |
| Where to Stay | Published | Planned | Planned | Planned | Planned |

---

## 9. HTML Strategy

Important content must remain visible HTML text.

### Why HTML Matters

| Area | Reason |
|---|---|
| SEO | Search engines need crawlable text, headings, lists, and tables. |
| Browser translation | Browser translation works best with visible text in the DOM. |
| Accessibility | Screen readers and assistive technologies need semantic HTML. |
| Maintenance | Text in HTML or content data is easier to update than image-only text. |
| Localization | Translators need editable text, not flattened image content. |

### Rules

- Do not place essential information only inside images.
- Use semantic HTML for headings, steps, comparisons, warnings, and decision criteria.
- Keep infographic text mirrored as visible HTML near the infographic.
- Use meaningful alt text for informative images, but do not treat alt text as a replacement for visible content.
- Keep procedure steps short and scannable.
- Use tables for comparisons when appropriate.

### Example Semantic Pattern

```text
section
  h2: Airport Transfer Options
  table: AREX vs airport bus vs taxi
  ul: Best option by traveler type
  p: Source and last-reviewed date
```

---

## 10. Migration Roadmap

### Phase Overview

```text
Phase 1: Current English site
  -> Phase 2: Language architecture
  -> Phase 3: Top pages translated
  -> Phase 4: Full multilingual expansion
```

### Migration Plan

| Phase | Goal | Actions | Approval Needed |
|---|---|---|---|
| Phase 1 | Preserve current English site. | Keep existing root English pages stable. Create page inventory and language plan. | No URL changes. |
| Phase 2 | Prepare language architecture. | Define `/en/`, `/ja/`, `/fr/`, `/de/`, `/zh-tw/` patterns, metadata rules, hreflang rules, and sitemap plan. | Yes before implementation. |
| Phase 3 | Translate top pages. | Start with arrival, airport transfer, eSIM, maps, T-money, WOWPASS, and apps. Add reviewed language pages only. | Yes before publishing. |
| Phase 4 | Expand multilingual coverage. | Translate content clusters, maintain update workflow, monitor Search Console and indexing. | Yes for URL, navigation, or template changes. |

### Priority Pages for Translation

| Priority | Page Type | Reason |
|---|---|---|
| High | Arrival, airport transfer, eSIM, maps, T-money, WOWPASS, apps | Strong practical intent and immediate visitor problems. |
| Medium | Where to stay, area comparisons, family guides | Useful decision-support content with broader search demand. |
| Later | Broad inspiration or general travel content | Less central to Korea Inside's practical problem-solving position. |

---

## 11. Risks

### Risk Matrix

| Risk Type | Risk | Mitigation |
|---|---|---|
| SEO risk | Search engines may not understand language equivalents. | Use reciprocal hreflang, self-canonicals, and multilingual sitemaps. |
| SEO risk | Existing English URL equity may be disrupted. | Use a separate approved migration plan before redirecting or canonicalizing root URLs. |
| Duplicate content risk | English root and `/en/` pages may overlap. | Decide whether root pages remain, redirect, or canonicalize during migration. |
| Duplicate content risk | Similar translated pages may be treated as duplicates. | Self-canonicalize each language and use hreflang. |
| Translation quality risk | Literal translations may be unclear or culturally awkward. | Use native or expert review before publishing. |
| Translation quality risk | Local terminology for transport, payments, or apps may be wrong. | Maintain terminology notes and official source references. |
| Maintenance risk | English updates may not propagate to translations. | Use translation status, "needs update" flags, and last-reviewed dates. |
| Maintenance risk | Changeable data may be duplicated across pages. | Centralize research notes and avoid repeating time-sensitive claims unnecessarily. |
| UX risk | Users may land on the wrong language. | Use clear language URLs and x-default fallback. |
| Performance risk | Language switching may add unnecessary scripts. | Prefer static localized pages and avoid loading unused language bundles. |

---

## 12. Korea Inside Recommendations

### SEO

- Use language subdirectories as the long-term public architecture.
- Use self-canonical URLs for every language page.
- Use reciprocal hreflang only for published equivalents.
- Use multilingual sitemaps with language alternates.
- Keep root English URL migration separate and carefully approved.

### Performance

- Prefer static localized pages.
- Avoid client-side machine translation as the primary multilingual solution.
- Do not load every language on every page.
- Share language-neutral images where possible.
- Keep essential text as HTML instead of image text.

### Scalability

- Maintain a page-language inventory.
- Use consistent slugs across languages where practical.
- Preserve identical content hierarchy across languages.
- Add languages only when review and maintenance can be supported.

### Future Maintenance

- Keep English as the master language.
- Track translation status per page.
- Track official sources and last-reviewed dates.
- Mark translated pages for review when English source content changes.
- Avoid duplicating changeable claims across many pages.

### Global Expansion

- Start with English, Japanese, and Traditional Chinese for practical regional demand.
- Expand to French and German once workflow quality is stable.
- Treat localization as more than translation: review terminology, examples, warnings, and decision criteria for each audience.

### Final Recommendation

Korea Inside should adopt this long-term multilingual architecture:

```text
/en/
/ja/
/fr/
/de/
/zh-tw/
```

This structure best supports SEO, scalability, maintainability, performance, and future localization while protecting the existing English site during migration.
