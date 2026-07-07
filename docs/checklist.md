# Korea Travel Checklist Page Standard

Last reviewed: 2026-07-01

## Page

- File: `checklist.html`
- URL: `https://getkoreainside.com/checklist.html`
- Purpose: Travel preparation checklist hub for foreign visitors before and after arriving in Korea.

## Objective

Define the page standard for `checklist.html` without requiring changes to other HTML pages, `style.css`, or `common.js`.

The page should help users decide what to prepare, not simply list items. Each checklist item should connect to a related guide so users can continue into the deeper decision page when needed.

## Reference Pages

| Reference | Reason |
|---|---|
| `accommodation.html` | Current high-quality decision-guide structure with hero, quick answer, cards, checklist, warning box, FAQ and schema. |
| `where-to-stay-in-seoul.html` | Strong decision-support content flow and FAQ pattern. |
| `docs/design-system.md` | Project component reuse audit and standard component guidance. |

## Component Requirements

| Component | Existing class or pattern |
|---|---|
| Hero | `.airport-page-hero`, `.airport-page-hero__grid`, `.airport-transport-card` |
| Quick links | `.airport-quick-links`, `.airport-pill` |
| Decision cards | `.airport-action-grid`, `.airport-action-card` |
| Checklist | `.checklist`, `.checklist__item`, `.checklist__checkbox`, `.checklist__label` |
| Table | `.table-scroll`, `.info-table` |
| Tip box | `.tip-box` |
| Warning box | `.warning-box` |
| FAQ | `.airport-faq`, `<details>`, `<summary>` |
| Related links | `.related-links`, `.chip` |

Use existing CSS components unless a separate approved task changes the page structure or design system.

## Required Sections

| Section | Requirement |
|---|---|
| Hero | Required |
| Quick Answer | Required |
| Before You Fly | Required checklist section |
| After Landing | Required, with airport and first-day subsections |
| Transportation | Required |
| Payments | Required |
| Accommodation | Required |
| Internet | Required |
| Safety | Required, including emergency, illness, lost item and embassy content |
| Common Mistakes | Required |
| Final Checklist | Required |
| FAQ | Required visible FAQ items with matching FAQ JSON-LD |

## Content Preservation Requirements

The page should preserve core travel-preparation topics unless a separate approved task changes the page scope:

- Before you fly
- At the airport
- First day in Korea
- Emergency numbers
- If you get sick
- If you lose something
- Embassy contacts
- Related guides

These topics should remain accurate, readable and organized within the checklist hub structure.

## SEO Requirements

The page should maintain:

- Unique title
- Meta description
- Canonical URL
- Robots meta
- Open Graph metadata
- Twitter card metadata
- Breadcrumb JSON-LD
- FAQ JSON-LD
- Exactly one H1
- Descriptive internal links

## UX Requirements

- Keep a quick answer near the top.
- Keep preparation-status cards directly below the hero.
- Group tasks by travel situation.
- Connect checklist items to related guide pages where useful.
- Keep related guide CTAs at the end of major sections.
- Use action-oriented card titles.
- Keep FAQ answers in short visible paragraphs.
- Preserve checkbox/localStorage behavior through existing checklist classes.
- Include common mistakes and final checklist.
- Include an emergency number table with a caution to verify official details before travel.

## Accessibility Requirements

- Keep semantic sections and heading hierarchy.
- Use `aria-label` on primary navigation.
- Checklist inputs use explicit labels with `for` and `id`.
- FAQ uses native `<details>` and `<summary>`.
- Important information remains visible HTML text.

## Implementation Verification Requirements

- H1 count should be 1.
- Visible FAQ content should match FAQ JSON-LD.
- JSON-LD should parse successfully.
- Internal file links and anchor links should be checked.
- Corrupted character scan should be checked.
- `style.css` and `common.js` should remain unchanged unless separately approved.
- Browser-based console and 360px / 768px / 1440px visual checks should be performed when a browser is available.

## Sources

| Source | Use |
|---|---|
| Korea Tourism Organization, Visit Korea: 1330 Travel Helpline & Complaint Center (`https://english.visitkorea.or.kr/svc/contents/infoHtmlView.do?vcontsId=140632`) | Official source for 1330 Travel Helpline purpose and overseas number reference. |

## Source Verification Requirements

- Emergency numbers such as 112, 119 and 1339 should be rechecked against official Korean government or public-service sources during future safety content updates.
- Entry requirements, K-ETA and visa rules should be handled on a dedicated entry/arrival page and verified close to publication.
- If the project later centralizes header/footer markup, checklist should be migrated only after page-by-page approval.
