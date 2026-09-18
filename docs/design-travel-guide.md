# Korea Inside Travel Guide Family Design Standard

## Document Metadata

Layer : L3
Status : Active
Authority Type : Family Design Standard
Primary Responsibility : Travel Guide family visual system, spacing, responsive behavior, and family component usage
Source of Truth For : Travel Guide family canvas, content widths, typography, spacing, image presentation, internal-link blocks, editorial navigation CTAs, affiliate CTA design, numbering, and date/time presentation
Not Responsible For : Codex behavior, repository-change approval, Git workflow, page-specific facts, research conclusions, approved public copy, SEO decisions, affiliate enrollment or status, navigation, header, footer, or implementation code
Higher Priority Documents : Current User Instruction, AGENTS.md, PROJECT.md, docs/product-constitution.md, docs/design-system.md, docs/component-library.md, docs/content-writing-standard.md, docs/business-operating-system.md
Related Documents : hongdae-travel-guide.html
Change Policy : Owner approval required before change
Last Reviewed : 2026-09-18
Review Trigger : Travel Guide family direction, Golden Sample, typography, spacing, image, CTA, affiliate, or responsive behavior changes

## 1. Purpose and Scope

This document defines the visual and component standard for the Korea Inside Travel Guide family.

Reference Page: `hongdae-travel-guide.html`

Role: Korea Inside Travel Guide Family visual and component reference.

The Hongdae page is the Golden Sample for:

- typography
- content width
- spacing
- image rules
- CTA design
- affiliate design
- internal-link design
- date and time presentation
- responsive behavior

It is not a mandatory table of contents or section template. Each Travel Guide may use a different section count, section order, route structure, map, event layer, affiliate count, FAQ, or comparison structure according to its search intent and user problem.

This standard does not authorize implementation. Repository scope, approval, protected-file, QA, and Git rules remain governed by the current user instruction and root `AGENTS.md`.

## 2. Base Canvas

The default Travel Guide canvas is:

```css
background: #fff;
```

Use white for the main page canvas and general editorial content areas.

Separate backgrounds are allowed when they serve a functional role, including:

- affiliate CTAs
- guidance or notice components
- functional cards
- maps
- event status
- other decision-support components

Do not turn every section into a tinted card or box. The visual goal is the Korea Inside Travel Guide UI, not a generic travel-magazine layout.

## 3. Width Standard

| Role | Standard |
|---|---:|
| Global container max-width | `1120px` |
| Global horizontal padding | `20px` per side |
| Maximum usable container content | `1080px` |
| General prose and itinerary max-width | `900px` |
| Hero or lead image max-width | `1080px` |
| Editorial body image max-width | `740px` |

Keep general prose left-aligned within the page container. Center editorial body images within their prose or content area.

A map or infographic may use a separately approved width when information density requires it. Do not force every map or infographic into the editorial-image width.

## 4. Typography

### Desktop

| Role | Font size | Line height | Weight | Letter spacing |
|---|---:|---:|---:|---:|
| H1 | `56px` | `1.05` | `600` | `-0.025em` |
| H2 | `42px` | `1.10` | `600` | `-0.025em` |
| H3 | `28px` | `1.20` | `600` | `-0.015em` |
| Body | `18px` | `1.68` | `400` | inherited |

### Tablet

| Role | Font size | Line height |
|---|---:|---:|
| H1 | `44px` | `1.05` |
| H2 | `36px` | `1.10` |
| H3 | `25px` | `1.20` |
| Body | `18px` | `1.65` |

### Mobile

| Role | Font size | Line height |
|---|---:|---:|
| H1 | `36px` | `1.08` |
| H2 | `32px` | `1.10` |
| H3 | `22px` | `1.22` |
| Body | `17px` | `1.65` |

Keep `16px` between consecutive general paragraphs.

## 5. Top and Heading Spacing

| Relationship | Desktop | Tablet | Mobile |
|---|---:|---:|---:|
| Breadcrumb to H1 | `20px` | `18px` | `14px` |
| Hero image to first H2 | `88px` | `72px` | `56px` |
| H2 to first paragraph | `24px` | `22px` | `20px` |
| H3 top spacing | `44px` | `40px` | `36px` |
| H3 to first paragraph | `16px` | `16px` | `14px` |
| General section rhythm | `72px` | `64px` | `52px` |

Keep the H1-to-lead-text spacing in the `22-24px` range by default. Prefer the verified Golden Sample value when implementing the shared component.

Do not stack multiple margins and paddings to recreate one target gap. Give each relationship one clear source.

Special sections may use their own component spacing when needed. Do not mechanically force the general section rhythm onto a Hero, map, route, CTA, stay bridge, or other specialized component.

## 6. HERO_IMAGE_V1

`HERO_IMAGE_V1` is the default large-image treatment for a Travel Guide.

- Maximum width: `1080px`
- Aspect ratio: `16:9`
- Position: below H1 and lead text
- Alignment: common container left edge
- Text overlay: not used by default
- Border radius: not used by default
- Desktop: may use the full container content width
- Mobile: use `width: 100%` inside the `20px` horizontal page padding

Change the photograph for each page. Image file, alt text, and any caption or credit must match that page's verified facts and approved direction.

Verified photo credit must remain visible when attribution is required. A visual redesign must not remove an approved source credit. License classifications such as `KOGL Type 1` remain in internal provenance by default unless the applicable license specifically requires that classification to be displayed publicly.

Use a larger hero image only when the page topic genuinely requires it and the exception is approved.

## 7. EDITORIAL_IMAGE_V1

Use `EDITORIAL_IMAGE_V1` for ordinary body photography.

- Maximum width: `740px`
- Width: `100%`
- Alignment: centered within the prose or content area
- Aspect ratio: preserve the source image ratio
- Forced crop: prohibited unless separately approved

| Viewport | Margin top | Margin bottom |
|---|---:|---:|
| Desktop | `32px` | `36px` |
| Tablet | `28px` | `32px` |
| Mobile | `24px` | `28px` |

Caption standard:

| Viewport | Font size | Line height |
|---|---:|---:|
| Desktop | `14px` | `1.5` |
| Mobile | `13px` | `1.5` |

Keep `10px` between the image and caption. Align captions and credits to the image's left edge.

Verified photo credit must remain visible when attribution is required. A visual redesign must not remove an approved source credit. License classifications such as `KOGL Type 1` remain in internal provenance by default unless the applicable license specifically requires that classification to be displayed publicly.

### Image Provenance and Public Attribution

For every external image, retain the following internal provenance when the information is available:

- original filename
- source
- creator or author
- original URL
- license
- usage or page role

Apply public attribution according to the applicable license:

- For `CC0` and Public Domain images, public credit is optional and internal provenance is mandatory.
- For `CC BY`, `CC BY-SA`, and other attribution-required licenses, retain the public attribution required by that license.
- For KOGL images, retain the license type internally and show the necessary public attribution. Display an additional license classification only when the applicable terms require it.

When storing or optimizing a new external image, preserve source or creator identity in the filename when practical. Do not rename existing assets solely to apply this convention unless the user explicitly approves the rename.

## 8. INTERNAL_LINK_BLOCK_V1

`INTERNAL_LINK_BLOCK_V1` connects the current editorial flow to the next relevant decision. The Hongdae stay bridge is the reference structure.

Structure:

- optional eyebrow
- short heading
- short explanatory paragraph
- clear text link

Use a white background and a restrained editorial treatment. Do not turn it into an oversized card.

The standard block is one of two preferred internal-link patterns. A natural inline text link remains allowed when it fits the sentence and does not create a weak cluster of links.

## 9. EDITORIAL_NAV_CTA_V1

`EDITORIAL_NAV_CTA_V1` is a stronger next-step navigation CTA. The Hongdae `AFTER DINNER` stay-guide bridge is the reference.

Structure:

- top divider
- blue eyebrow
- large heading
- one short paragraph
- one blue solid button
- white canvas
- no affiliate disclosure

Use it for transitions such as:

- Travel Guide to Stay Guide
- Attraction Guide to Area Guide
- Area Guide to Detail Guide

It must read as editorial navigation, not as an advertisement.

## 10. AFFILIATE_CTA_V1

The default Travel Guide affiliate CTA is a text-based editorial card without an image. The Hongdae K-pop dance class CTA is the reference.

Structure:

- short heading
- one- or two-sentence description
- blue solid button
- visible disclosure

Layout:

| Property | Value |
|---|---:|
| Max-width | `740px` |
| Margin top | `24px` |
| Padding | `20px` |
| Border radius | `12px` |

Button:

| Property | Value |
|---|---:|
| Padding | `10px 18px` |
| Font size | `16px` |
| Line height | `1.5` |
| Border radius | `8px` |

Default disclosure candidate:

> Affiliate link - Korea Inside may earn a commission at no extra cost to you.

Use the `no extra cost` statement only when the program cost structure has been officially verified. Otherwise, disclose the commission without that claim. Affiliate facts and wording remain governed by `docs/business-operating-system.md` and `docs/content-writing-standard.md`.

Do not impose a mechanical affiliate-count limit. Use this relevance chain:

Relevant section -> matching action -> matching affiliate

Only use an affiliate CTA when the context and user decision match.

## 11. IMAGE_AFFILIATE_CTA Optional Variant

An image affiliate CTA is an optional variant, not the default.

Use it only when the user explicitly requests the visual treatment or when the image materially explains the experience.

Possible structure:

- image
- title
- description
- button
- disclosure

When an AI-generated image is used, retain a visible disclosure such as:

> AI-generated image for reference

Do not select this variant automatically in place of `AFFILIATE_CTA_V1`.

## 12. Numbering Rule

Use `1`, `2`, and `3` only when the sequence itself is necessary information. Do not use numbering as decoration.

Appropriate uses:

- an actual step sequence
- a procedure that must be followed in order
- identifiers such as `Route 1` and `Route 2` when the distinction itself is necessary

Do not use numbering for:

- timetables, where the time already communicates sequence
- cards
- advantages or simple attraction lists
- parallel choices
- section decoration

Number = information, not decoration.

## 13. Time Standard

Use Korea local time and the 12-hour clock with `AM` or `PM` for travel itineraries.

Examples:

- `2:00 PM`
- `2:10-2:40 PM`
- `5:30-7:00 PM`

Prefer `2:00 PM` over `2 PM`. Do not repeat `KST` on every line. State the time zone only when ambiguity or travel planning makes it useful.

## 14. Date and Event Standard

Use these presentation patterns:

- Full date: `September 18, 2026`
- Date range: `October 16-18, 2026`
- Event heading: `October 16-18: Seoul Wow Book Festival comes to Red Road`
- Monthly program: `KT&G Sangsangmadang - September 2026 programs`
- Checked date: `Checked: September 10, 2026`

Include the year in the first explanation when readers may need it to judge relevance.

Event status values:

- `UPCOMING`
- `HAPPENING NOW`
- `ENDED`

Prefer automatic status evaluation through the existing `event-status.js` behavior when the event can be represented accurately. Use only a date on which the information was actually checked.

## 15. Link Quality Rule

Do not end a Travel Guide with several weak standalone text links such as:

- `Compare Hongdae and Myeongdong`
- `Compare hotels`

When the next decision is clear, use one of the two preferred internal-link treatments:

1. `INTERNAL_LINK_BLOCK_V1`
2. `EDITORIAL_NAV_CTA_V1`

A normal inline text link remains appropriate when it fits naturally inside the body sentence.

## 16. Responsive Principle

Desktop, Tablet, and Mobile must preserve:

- the same core content
- the same decision information
- no unintended horizontal overflow
- readable line length
- responsive images
- usable CTA buttons
- readable captions and credits

Do not merely shrink a desktop component proportionally on mobile. Stack or reflow it when necessary while preserving meaning and action priority.

## 17. Page Structure Freedom

This standard does not force every Travel Guide to use the same table of contents.

The following may vary by page:

- section count
- section order
- route count
- map
- event section
- FAQ
- comparison
- affiliate count
- Stay Bridge
- Current Layer

Choose them from the page's search intent, verified research, and actual user problem.

## 18. Golden Sample Rule

Current Golden Sample: Hongdae Travel Guide V1 (`hongdae-travel-guide.html`)

The Hongdae page is the Travel Guide Family visual and component reference. Reuse its approved family patterns, not its destination-specific substance.

Do not copy:

- Hongdae facts
- Hongdae section order
- Hongdae recommendations
- Hongdae itinerary

Use the relevant destination Research Master for another destination's facts, recommendations, and route logic.

## 19. Future Implementation Rule

For a new Travel Guide or an approved Travel Guide design revision:

1. Read `docs/design-system.md`.
2. Read this Family Design Standard.
3. Reuse the approved Golden Sample component when it serves the same role.
4. Do not create a new component when an equivalent approved component already exists.
5. If the page needs a different design, stop, report the reason, and obtain explicit user approval before implementing it.

This document does not expand task scope or authorize file changes.
