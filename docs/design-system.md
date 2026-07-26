# Korea Inside Design System Standard

## Document Metadata

Layer : L2
Status : Active
Authority Type : Design Standard
Primary Responsibility : Site-wide visual language, component consistency, responsive design, image and infographic direction, and design QA
Source of Truth For : Color roles, typography, content width, spacing, cards, buttons, tables, guidance boxes, images, infographics, responsive visual rules, and design anti-patterns
Not Responsible For : Page-specific content, current work queues, migration lists, business strategy, Codex approval, Git workflow, reference records, or history
Higher Priority Documents : Current User Instruction, AGENTS.md, PROJECT.md, docs/product-constitution.md
Related Documents : docs/component-library.md, docs/golden-page-template.md, docs/page-template-standard.md, docs/anti-pattern-standard.md, docs/content-writing-standard.md
Change Policy : Owner approval required before change
Last Reviewed : 2026-07-25
Review Trigger : Major visual direction change, recurring design inconsistency, new page family, image policy change, or component-system change

## 1. Design Position

Korea Inside uses a warm, editorial travel-magazine visual language to help international visitors understand Korea and make practical choices.

The design should feel:

- useful, calm, and human
- trustworthy without feeling institutional
- visual without becoming decorative
- editorial without becoming promotional
- information-rich without becoming crowded

Every core screen should make its purpose understandable within about three seconds.

## 2. Color Roles

The default direction is a warm ivory and cream foundation.

- Prefer warm ivory or cream for broad page backgrounds and long-reading areas.
- Prefer dark brown-black or charcoal for headings.
- Prefer warm gray-brown for body text.
- Use low-saturation borders and dividers.
- Use charcoal green as the main functional color for stability, guidance, diagrams, and infographic structure.
- Use reddish brown or ochre only for an already approved editorial emphasis.
- Minimize large areas of pure white, pure black, or severe black-white contrast.

Blue is not a primary theme color for:

- broad page backgrounds
- main content cards
- headings
- large guidance boxes
- the overall page theme

Blue may be used in a limited functional role for links, status, transport-system distinctions, or another established semantic meaning.

Reuse an approved value from the existing CSS or an approved standard when an exact color is required. Do not invent or standardize a new HEX value in documentation without approval.

Color must never be the only way to communicate state, warning, selection, or meaning.

## 3. Application and Transition Scope

### New Pages or Fully Redesigned Pages

- Use the warm ivory and cream foundation.
- Use charcoal or warm gray-brown for body copy.
- Use charcoal green for guidance and infographic structure.
- Do not use blue as the overall dominant page color.

### Minor Changes to Existing Production Pages

- Preserve the current page structure and approved component family first.
- Do not convert the entire page to the ivory system without separate user approval.
- Do not force a new design system onto an existing page because of a small correction.

### Full Redesign of an Existing Production Page

- Apply the new design engine only after explicit user approval.
- Proceed one page at a time and review the rendered page before expanding the change.
- Do not recolor the entire site as one batch.

"Reuse approved existing CSS" does not mean every new page must inherit a blue component or blue-led theme.

Reuse the nearest existing structure, but report the conflict when it does not fit the approved design direction for the page.

Do not change the common header, navigation, footer, or `common.js` without separate explicit approval.

## 4. Typography

- Reuse the approved site typefaces and existing typographic tokens.
- Use a clear heading hierarchy with strong but not harsh contrast.
- Keep body copy comfortable for long reading on warm backgrounds.
- Avoid excessive font-weight changes, all-caps blocks, decorative display type, and unexplained font-family additions.
- Keep labels, captions, and helper text legible on mobile.
- Let line length and spacing support browser translation and text expansion.

## Detailed Guide Typography Standard

### Scope

This standard governs long-form travel guides and problem-solving detail pages, including airport, transportation, accommodation, eSIM, map, payment, travel-tip, K-Beauty, and Taste Korea content. It applies to body copy, cards, FAQ answers, guidance boxes, comparison explanations, step-by-step instructions, and destination- or situation-specific details.

Representative pages include:

- `arex.html`
- `airport-bus.html`
- `airport-transfer.html`
- `arrival.html`
- `airport.html`
- `tmoney.html`
- `maps.html`
- `taxi.html`
- eSIM detail pages
- long-form K-Beauty and Taste Korea detail pages

### Official Type Scale

#### General Detail-Page Body

| Viewport | Font size | Line height |
| --- | ---: | ---: |
| Desktop | `18px` | `1.72` |
| Mobile | `17px` | `1.68` |

Apply this scale to:

- general body paragraphs
- body list items
- definition-style content in `dd`
- step-by-step explanations
- destination- and situation-specific detail copy

#### Card Body

| Viewport | Font size | Line height |
| --- | ---: | ---: |
| Desktop | `18px` | `1.68` |
| Mobile | `17px` | `1.65` |

Do not reduce copy merely because it appears inside a card. Apply the same scale to choice cards, comparison cards, destination cards, and mistake-prevention cards.

#### Guidance Box and Quick Answer Body

| Viewport | Font size | Line height |
| --- | ---: | ---: |
| Desktop | `18px` | `1.72` |
| Mobile | `17px` | `1.68` |

Apply this scale to Quick Answer, Tip, Warning, Important Note, Editorial Decision, and Source Note copy.

#### FAQ

| Element | Viewport | Font size | Font weight | Line height |
| --- | --- | ---: | ---: | ---: |
| Question | Desktop | `17px` | `700` | `1.45` |
| Question | Mobile | `16px` | `700` | `1.45` |
| Answer | Desktop | `18px` | Inherited | `1.72` |
| Answer | Mobile | `17px` | Inherited | `1.68` |

Keep question and answer horizontal padding consistent within the FAQ component. Give long answers enough usable width for natural wrapping.

#### Tables

| Element | Viewport | Font size | Font weight | Line height |
| --- | --- | ---: | ---: | ---: |
| Title and header | Desktop | `15px` | `700` | `1.5` |
| Title and header | Mobile | `14px` | `700` | `1.5` |
| Body | Desktop | `15px` | Inherited | `1.6` |
| Body | Mobile | `14px` | Inherited | `1.55` |

#### Image Captions and Source Metadata

Apply this scale to:

- image captions
- supporting information that displays only a URL or domain
- research and verification dates
- short source footnotes

| Viewport | Font size | Line height |
| --- | ---: | ---: |
| Desktop | `14px` | `1.5` |
| Mobile | `13px` | `1.5` |

#### Official Source Lists

Treat official source names and sentences explaining what was verified from each source as general detail-page body copy.

| Viewport | Font size | Line height |
| --- | ---: | ---: |
| Desktop | `18px` | `1.72` |
| Mobile | `17px` | `1.68` |

- Do not reduce official source explanations to caption size.
- Only URLs, research dates, verification dates, and short source metadata may use the `14px` desktop and `13px` mobile scale.
- Official source descriptions remain subject to the rule that desktop detail-body copy must not be `16px` or below.

#### Dates, Eyebrows, and Section Numbers

Dates, eyebrows, section numbers, and equivalent metadata labels are outside the detail-body enlargement target. Preserve their established metadata hierarchy instead of forcing them to `18px`.

### Heading Rules

- Do not apply the body-copy `72–78ch` measure to `h1`, `h2`, or `h3`.
- Let headings use enough of the available content container to wrap naturally.
- Do not insert arbitrary `<br>` elements into `h1` or `h2`.
- Do not reduce heading sizes excessively to force a preferred line count.
- Adjust `max-width` and `line-height` before considering a size change.
- Prefer two or three lines at most for desktop `h1` headings.
- `text-wrap: balance` may be used in supporting browsers.
- Let card titles and FAQ questions wrap naturally at word boundaries.

Exact `h1`, `h2`, and `h3` sizes follow the approved page type and design. Heading sizes must not be used to compensate for undersized body copy.

### Body Measure and Wrapping

Apply the following rules only to long general body copy:

- recommended reading measure: `72–78ch`
- `text-align: left`
- never use `text-align: justify`
- `letter-spacing: normal`
- `word-spacing: normal`
- `word-break: normal`
- `hyphens: none`
- `overflow-wrap: break-word`

Do not force the `72–78ch` measure onto:

- `h1`, `h2`, or `h3`
- section introductions
- card titles
- short card copy
- FAQ questions
- buttons
- tabs
- tables
- guidance boxes

Section introductions should use enough of the content width to avoid unnecessary extra lines from an overly narrow `max-width`.

### Text Spacing

- Keep `16px` between consecutive general paragraphs.
- Keep at least `8px` between list items.
- Keep `12px` between a card title and its body.
- Keep `16px` between consecutive paragraphs inside a card.
- Never reduce type size to equalize card heights.
- Keep `12–16px` between an `h2` and its introduction.
- Keep `24–32px` between a section introduction and its first content element.
- Do not use empty space to inflate page length.

### Common UI Exclusions

Do not automatically apply the `18px` detail-body standard to:

- the common header
- navigation
- the mobile hamburger control
- buttons
- menus
- language controls
- breadcrumbs
- dates
- eyebrows
- section numbers
- image captions
- short table labels

Do not change the root `html` or `body` font size to enlarge the whole interface. Scope the standard with explicit selectors for the detail-page content area.

### Page-Level Consistency and Exceptions

- Do not choose a new body size independently for each detail page.
- Do not mix `15px`, `16px`, `17px`, and `18px` body copy across equivalent detail pages.
- Apply the approved body values consistently to equivalent content roles.
- Do not reduce card body copy to `15px` or below merely because it is inside a card.
- Do not add an unapproved page-specific `font-size` override.
- If an exception is necessary, report the target, reason, and replacement value and obtain user approval before implementation.

### Rollout and QA

The numeric scale is a site-wide standard, but implementation and QA proceed page by page.

- Do not apply an unverified whole-site batch change.
- Keep the approved values consistent from page to page.
- Verify each page at `1440px`, `768px`, and `390px`.
- Check cards, tables, buttons, headings, and shared UI for regressions.
- Obtain approval for one rendered page before proceeding to the next page.

In short: the values are shared globally; implementation and QA are sequential by page.

### Codex Completion Criteria

Confirm the following with browser computed styles:

- general body: `18px` desktop and `17px` mobile
- card body: `18px` desktop and `17px` mobile
- FAQ answer: `18px` desktop and `17px` mobile
- table body: `15px` desktop and `14px` mobile
- image caption: `14px` desktop and `13px` mobile

Also verify:

- no desktop detail-body copy remains at `16px` or below
- card body copy is not smaller than equivalent general body copy
- no unapproved page-specific size override exists
- the body measure is not applied to `h1` or `h2`
- no `text-align: justify` exists in the detail content
- the common header and navigation have not been enlarged
- no page-level horizontal scrolling occurs at `1440px`, `768px`, or `390px`
- buttons, tables, and cards wrap without layout defects

If browser verification is unavailable, do not mark the implementation as passed. Report it as a Known QA Limitation.

## 5. Content Width and Spacing

- Reuse established content-width and section-spacing patterns.
- Keep long body text narrower than broad media or comparison sections.
- Use whitespace to separate decisions and topics, not to make cards unnecessarily large.
- Keep section rhythm consistent while allowing dense practical sections to use compact spacing.
- Mobile spacing is the baseline; desktop should expand the same information hierarchy naturally.
- Do not add a new spacing scale for one page when an approved token or nearby component already fits.

## 6. Page Rhythm and Hierarchy

Pages should progress through meaning rather than repeat equally weighted sections.

A common rhythm is:

1. Context or scene
2. Quick answer
3. Choice or comparison
4. Practical detail
5. Warning or tip
6. Next useful action

This is a flexible rhythm, not a mandatory template. Use `golden-page-template.md` for detailed page flow.

Headings, images, tables, cards, and guidance boxes should make the recommended reading order obvious.

## 7. Component Consistency

Before creating or styling a component:

1. Search for the nearest approved component in the same page family.
2. Reuse it when it serves the same semantic role.
3. Extend it only when the current task explicitly approves the change.
4. Create a new component only when no existing pattern can express the required behavior.

Do not invent page-specific versions of:

- card corners
- shadows
- borders
- button height or typography
- table structure
- captions
- warning and tip boxes
- commercial CTA blocks
- section spacing
- content width

Use `component-library.md` for component-level structure.

## 8. Cards, Buttons, Tables, and Guidance Boxes

### Cards

- A card should organize a decision, option, action, or distinct information unit.
- Avoid generic title-description-button grids that do not help a choice.
- Keep padding, radius, shadow, border, and density aligned with the nearest approved family.
- Include advantages, limitations, suitability, or next action when the card represents an option.

### Buttons and Links

- Buttons represent actions; links represent navigation.
- Keep primary, secondary, and text-action hierarchy clear.
- Use pills and chips only for their established navigation, tag, or filter role.
- Commercial CTA styling must remain secondary to the decision-support content.

### Tables

- Use semantic tables when side-by-side comparison materially improves understanding.
- Keep labels concise and preserve readable mobile behavior.
- Use a wrapper for overflow when necessary rather than shrinking text below legibility.
- Do not use a table for prose that is easier to read as a list.

### Guidance Boxes

- Warning boxes communicate risk, limitation, or mistake prevention.
- Tip boxes communicate useful non-critical advice.
- Neutral guidance boxes explain context without simulating urgency.
- Use visible labels and text; do not rely on background color alone.

## 9. Images

Images are information, not decoration.

Use images to clarify:

- a real object or interface
- how something is used
- scale, movement, location, or atmosphere
- a practical difference between options
- a place the visitor must recognize

Use actual, official, user-provided, or clearly referenced images when real-world accuracy matters. Do not use AI-generated representations of official cards, banknotes, kiosks, maps, app screens, payment interfaces, or other objects whose exact appearance affects user behavior.

AI-created imagery may support atmosphere or a non-official illustrative context only when production is explicitly requested under `AGENTS.md`.

Essential labels, instructions, warnings, comparisons, and values must also appear as visible semantic HTML near the image. Do not rely on image text, `alt`, `title`, or `figcaption` alone.

## 10. Infographics

- Prefer a warm ivory background.
- Prefer charcoal-green icons, arrows, numbers, lines, and structural elements.
- Maintain enough lightness and contrast to separate background and internal elements.
- Avoid unnecessary multicolor palettes within one infographic.
- Keep essential text identifiable on mobile.
- Reproduce every important statement as visible semantic HTML nearby.
- Do not copy or lightly redraw an official map.
- Do not simplify official guidance in a way that changes its meaning.

## 11. Responsive Design

- Design mobile first.
- Preserve the same core content and decision logic on mobile and desktop.
- Prevent excessive text width, oversized cards, clipped controls, and unreadable tables.
- Keep touch targets and spacing usable without inflating the interface.
- Allow layouts to reflow by page family instead of forcing every page into one grid.
- Use existing responsive breakpoints and patterns unless the approved task requires another approach.

Responsive verification follows the QA level and screenshot limits in `AGENTS.md`.

## 12. Page-family Flexibility

Maintain a common visual core while allowing different composition for:

- practical information guides
- comparison pages
- hub pages
- conversion pages
- culture- or photography-led pages

Consistency means shared roles, tokens, and behavior—not identical page layouts.

## 13. Design QA

Check:

- Is the page purpose clear within about three seconds?
- Does the visual hierarchy support a decision?
- Are broad backgrounds warm and readable?
- Are text and background contrast sufficient without becoming harsh?
- Is blue limited to an established functional role?
- Are typography, width, spacing, cards, buttons, tables, and boxes consistent with approved patterns?
- Are images and infographics informative and accurate?
- Is essential image information repeated as visible text?
- Does the layout remain readable and operable on mobile?
- Does it support browser translation and text expansion?
- Does the page family remain recognizable without making every page identical?

## 14. Prohibited Design Patterns

- broad pure-white or pure-black surfaces without an approved need
- blue-led page themes, content cards, headings, or large guidance panels
- arbitrary new HEX values
- decorative images with no information role
- AI-generated official objects or interfaces
- excessive shadows, gradients, saturation, or multicolor infographic palettes
- giant cards and empty spacing that slow scanning
- image-only essential information
- repeated commercial CTAs throughout body content
- a one-off component when an approved equivalent exists
- whole-site recoloring or migration without page-specific approval
