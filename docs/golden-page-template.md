# Korea Inside Golden Page Template

## Document Metadata

Layer : L2
Status : Active
Authority Type : Page Structure Standard
Primary Responsibility : Golden page structure and content flow for high-quality Korea Inside pages
Source of Truth For : Ideal page sequence, decision-support page flow, content hierarchy, section rhythm, and reusable page structure patterns
Not Responsible For : Visual style system, component-level rules, quick minimum checklist, business strategy, Codex behavior rules, reference records, history records
Higher Priority Documents : Current User Instruction, AGENTS.md, PROJECT.md, docs/product-constitution.md, docs/design-system.md
Related Documents : docs/component-library.md, docs/page-template-standard.md, docs/anti-pattern-standard.md, docs/standards-hub.md
Change Policy : Owner approval required before change
Last Reviewed : 2026-07-09
Review Trigger : Page architecture change, new page family, recurring weak page flow, or major section hierarchy change

## Purpose

This document defines the standard page template for future Korea Inside content pages.

The goal is to keep every page consistent, practical, SEO-friendly, mobile-first, accessible and easy to maintain.

This is a visual and content architecture reference only. It does not define HTML or CSS implementation.

## Core Page Principles

Every Korea Inside page should:

- Help the visitor make a decision.
- Explain trade-offs clearly.
- Use real visible text for important information.
- Preserve the existing Korea Inside design language.
- Stay mobile-first.
- Use objective, practical, trustworthy guidance.
- Avoid exaggerated recommendations.
- Prefer concise summaries, comparison tables and decision boxes.

---

## 1. Hero Section

### Purpose

The hero section should tell the user what problem the page solves within a few seconds.

It should not feel like a marketing banner. Korea Inside pages should open with practical clarity.

### Required Elements

Every hero should include:

| Element | Requirement |
|---|---|
| Breadcrumb | Show page context and return path where useful. |
| Eyebrow | Short category label such as `Airport Guide`, `Seoul Stay Guide` or `Payment Guide`. |
| H1 | One clear, search-friendly page title. |
| Summary | One short paragraph explaining who the page helps and what decision it supports. |
| Primary CTAs | Anchor links to the most useful sections on the same page. |
| Trust cue | A short practical note, source cue, update cue or decision summary where appropriate. |

### Layout

Recommended layout:

```text
Hero copy
  Breadcrumb
  Eyebrow
  H1
  Summary
  CTA row

Hero support card
  Practical answer
  Short decision summary
  2-4 bullets
```

On mobile, stack the hero copy above the support card.

On desktop, use a two-column layout only when the support card adds real decision value. Do not add decorative cards just to fill space.

### CTA Placement

Hero CTAs should:

- Link to important in-page sections.
- Use short labels.
- Prioritize decision support over generic browsing.
- Avoid more than three primary CTAs.

Recommended CTA examples:

- `Quick Answer`
- `Compare Options`
- `Who Should Choose This`
- `Common Mistakes`
- `FAQ`

### Trust Elements

Use trust elements when the topic involves changeable or high-stakes information.

Examples:

- `Last reviewed: YYYY-MM-DD`
- `Based on official source where available`
- `Check exact times before travel`
- `Prices and operating hours may change`

Trust elements should be calm and factual, not alarming.

---

## 2. Quick Answer Box

### Purpose

The quick answer box gives the user the answer before the detailed explanation.

It should reduce uncertainty immediately.

### How It Should Look

The quick answer should appear near the top of the page after the hero.

Recommended visual style:

- Strong card or highlighted panel.
- Clear heading.
- Short paragraphs.
- 3-5 bullets or mini-cards.
- No dense text blocks.

### Required Information

The quick answer should include:

| Item | Requirement |
|---|---|
| Best default answer | The safest or most common recommendation. |
| Best alternatives | 2-3 alternatives for different user types. |
| Main trade-off | Explain what the user gives up by choosing the default. |
| Warning if needed | Mention important limitations such as noise, late-night transport or verification. |

### Example Pattern

```text
Quick answer:
For most first-time visitors, choose X because it is simple and convenient.

Choose Y if you care more about budget.
Choose Z if you have large luggage.
Avoid X if you need quiet sleep.
```

---

## 3. Decision Summary

### Purpose

The decision summary turns information into action.

It should help the user understand which option fits their situation.

### Standard Layout

Use a three-to-five item summary whenever possible:

| Pattern | Use Case |
|---|---|
| Best overall | Most users can start here. |
| Best for budget | Cost-sensitive users. |
| Best for convenience | Users who want the easiest option. |
| Best for families / luggage | Users with practical constraints. |
| Avoid if | Prevents bad-fit decisions. |

### Icons

Icons may be used only when they improve scanning.

Recommended icon categories:

- Transport
- Money
- Luggage
- Family
- Time
- Warning
- Map
- Phone

Icon rules:

- Icons should support the label, not replace important text.
- Do not rely on icons alone for meaning.
- Keep icon style consistent with existing Korea Inside pages.

### Comparison Style

Use compact decision cards for quick scanning.

Each card should include:

- Option name.
- Best for.
- Main advantage.
- Main limitation.

Avoid long paragraphs inside decision cards.

---

## 4. Main Content Sections

### Recommended Structure

Most content pages should follow this flow:

```text
Hero
Quick Answer
Decision Summary
Main Options / Areas / Steps
Comparison Table
Who Should Choose What
Safety / Practical Notes
Common Mistakes
FAQ
Related Guides
Affiliate Section, if appropriate
```

### Recommended Spacing

Spacing should feel calm and scannable.

Rules:

- Keep sections visually separated.
- Avoid stacking too many large cards.
- Use whitespace to clarify groups.
- Keep mobile paragraphs short.
- Avoid dense walls of text.

### Heading Hierarchy

Every page should use:

- One H1 only.
- H2 for major sections.
- H3 for cards, options and subsections.
- H4 only when a section genuinely needs another level.

Do not skip heading levels.

### Image Placement

Images should support understanding, not replace text.

Rules:

- Place images near the content they explain.
- Keep all important information as visible HTML text.
- Avoid image-only infographics.
- Use meaningful alt text for informative images.
- Avoid decorative images unless they improve user understanding.

### Comparison Tables

Use tables when comparing:

- Options.
- Areas.
- Transport methods.
- Payment methods.
- Pros and cons.
- User fit.

Tables should help users decide quickly.

### Tips

Tips should be short and practical.

Use tips for:

- Helpful shortcuts.
- Common traveler behavior.
- Local context.
- Simple checks before travel.

### Warnings

Warnings should be used sparingly.

Use warnings for:

- Time-sensitive information.
- Safety or late-night transport limitations.
- Payment limitations.
- Booking mistakes.
- Situations where users may choose the wrong option.

Warnings should explain what to do, not only what to avoid.

---

## 5. Comparison Table Standards

### Purpose

Comparison tables should make trade-offs visible.

They should not become decorative data blocks.

### Required Table Qualities

Every comparison table should:

- Compare meaningful options.
- Use concise column labels.
- Avoid very long sentences.
- Include both strengths and limitations.
- Be readable on mobile.

### Recommended Columns

Choose only the columns that help the decision.

Common columns:

| Column | Use |
|---|---|
| Option / Area | Name of the choice. |
| Best For | User fit. |
| Main Advantage | Why to choose it. |
| Main Limitation | Why it may not fit. |
| Cost / Budget | When relevant. |
| Convenience | When relevant. |
| Luggage / Family Fit | When relevant. |
| Late-night / Safety Notes | When relevant. |

### Colors

Use color to clarify, not decorate.

Recommended use:

- Soft highlight for the best default option.
- Neutral background for standard rows.
- Warning color only for real caution.
- Avoid strong color overload.

### Mobile Behavior

Tables must remain usable on mobile.

Acceptable patterns:

- Horizontal scroll table inside a clear container.
- Simplified mobile cards if the table is too wide.
- Short labels and compact text.

Do not force tiny unreadable columns on mobile.

---

## 6. Decision Box

### Purpose

The decision box explains exactly who should choose an option and who should avoid it.

It should appear after major option sections or before the final recommendation.

### Required Content

Each decision box should answer:

| Question | Requirement |
|---|---|
| Who is this for? | List suitable users. |
| Who should avoid it? | List poor-fit users. |
| What are the alternatives? | Suggest better options for different cases. |
| What is the main trade-off? | Explain the downside clearly. |

### Recommended Layout

```text
Decision Box

Choose this if:
- ...
- ...

Avoid this if:
- ...
- ...

Consider instead:
- Option A for ...
- Option B for ...
```

### Tone

Decision boxes should be direct but not absolute.

Avoid phrases like:

- `This is always the best`
- `Everyone should choose this`
- `Never choose this`

Prefer:

- `This usually works best if...`
- `This may not fit if...`
- `Consider another option if...`

---

## 7. FAQ Design

### Purpose

FAQ sections should answer practical questions that remain after the main content.

They should support SEO, but should not exist only for SEO.

### Accordion Behavior

FAQ items may use accordion behavior when the existing page pattern supports it.

Accordion rules:

- Questions should be visible.
- Answers should be accessible.
- Keyboard interaction should work.
- Content should remain crawlable.

### Question Style

Questions should match real user searches.

Good patterns:

- `Is Hongdae better than Myeongdong?`
- `Can I use a foreign card in Korea?`
- `Should I take AREX or airport bus?`
- `Where should I stay with large suitcases?`

Avoid vague questions:

- `What about Korea?`
- `Is it good?`
- `More information?`

### Answer Length

FAQ answers should usually be:

- 1-3 short paragraphs.
- Direct first sentence.
- Extra detail only when necessary.

Recommended answer pattern:

```text
Yes, if...
No, if...
The main trade-off is...
```

---

## 8. Related Guides

### Purpose

Related guides should help users continue the same decision journey.

They should not be random internal links.

### Internal Linking Strategy

Link to pages that help with:

- The next step.
- A related decision.
- A comparison.
- A practical problem.
- A safer alternative.

Example:

```text
Nightlife stay page
→ Where to Stay in Seoul
→ Hongdae vs Myeongdong
→ First-Time Visitors
→ Families
```

### Card Layout

Related guide cards should include:

| Element | Requirement |
|---|---|
| Page title | Clear and descriptive. |
| Short description | One sentence explaining why it is related. |
| Link text | Descriptive, not generic. |

Avoid:

- Too many related links.
- Irrelevant links.
- Repeating the same links in too many sections.

Recommended limit:

- 3-6 related guides per page.

---

## 9. Affiliate Section

### Purpose

Affiliate sections should appear only when they genuinely help the user take the next step.

They should never replace objective guidance.

### Placement

Affiliate sections should usually appear after:

1. The user understands the options.
2. The page has explained trade-offs.
3. The page has identified who the option is suitable for.

Recommended placements:

- After a decision box.
- After a comparison table.
- Near the final recommendation.
- Inside a relevant option section, only if the commercial link directly matches that option.

Avoid placing affiliate links before the user understands the decision.

### Design

Affiliate blocks should be:

- Clearly labeled.
- Visually secondary to the recommendation.
- Easy to distinguish from editorial content.
- Concise.

### Trust-First Policy

Affiliate sections must follow these rules:

- Disclose affiliate relationships clearly.
- Do not rank options by commission.
- Explain disadvantages.
- Offer non-affiliate alternatives where useful.
- Recommend only when the product fits the user.
- Never hide limitations.

### Recommended Affiliate Block Pattern

```text
Useful if:
- ...

Not ideal if:
- ...

Why Korea Inside mentions this:
- ...

Disclosure:
This page may contain affiliate links. Korea Inside prioritizes user fit over commission.
```

---

## 10. Mobile Design Rules

### Mobile Priority

Korea Inside is mobile-first.

Many visitors will use the site while traveling, standing in an airport, riding transit or comparing options quickly.

### Spacing

Mobile spacing should:

- Keep sections clear.
- Avoid oversized cards.
- Avoid excessive vertical scrolling caused by decorative elements.
- Keep key answers near the top.

### Touch Targets

Touch targets should be easy to tap.

Rules:

- Buttons and links should have comfortable tap areas.
- CTA rows should wrap cleanly.
- Tables should not require precise tapping.
- Accordions should have readable summaries.

### Typography

Mobile typography should:

- Be readable without zoom.
- Use short paragraphs.
- Avoid very wide text lines.
- Avoid tiny table text.
- Preserve hierarchy between headings, body text and labels.

### Mobile Content Behavior

On mobile:

- Put the quick answer high on the page.
- Stack cards vertically.
- Keep comparison cards concise.
- Use tables only when they remain readable.
- Avoid repeating the same information in multiple large blocks.

---

## 11. Accessibility Rules

Every page should follow accessibility basics.

### Required Rules

- Use semantic structure.
- Keep heading hierarchy logical.
- Use descriptive link text.
- Use meaningful alt text for informative images.
- Do not rely on color alone to communicate meaning.
- Keep important content as visible text.
- Ensure buttons and accordions are keyboard-accessible.
- Use labels for form controls.
- Keep text readable with sufficient contrast.

### Accessibility For Decision Content

Decision content must be understandable without visual decoration.

For example:

- Do not use only icons to indicate best or worst options.
- Do not use only color to show warnings.
- Do not place important comparison text only in an image.

---

## 12. Visual Design Principles

### Consistency

Future pages should preserve the Korea Inside design language.

Rules:

- Reuse existing visual patterns.
- Keep page rhythm consistent.
- Avoid one-off decorative layouts.
- Do not redesign pages unless explicitly requested.

### Whitespace

Whitespace should help users scan.

Use whitespace to:

- Separate sections.
- Group related cards.
- Make comparison tables easier to read.
- Reduce cognitive load.

Avoid:

- Overly dense sections.
- Excessive blank space that pushes answers too far down.

### Icons

Icons should:

- Improve scanning.
- Match the topic.
- Be consistent in size and style.
- Never replace essential text.

Use icons sparingly.

### Cards

Cards should be used for:

- Decision summaries.
- Option summaries.
- Warnings.
- Tips.
- Related guides.

Avoid using cards for every paragraph.

Card content should be concise:

- Short heading.
- One short paragraph.
- Optional bullet list.

### Trust Signals

Trust signals should be visible where they matter.

Examples:

- Official source references.
- Last-reviewed date.
- Clear limitations.
- Practical warnings.
- Affiliate disclosure.
- Explanation of why a recommendation fits.

### Final Page Standard

A strong Korea Inside page should answer:

1. What should the visitor do?
2. Who is this recommendation for?
3. Who should avoid it?
4. What are the trade-offs?
5. What should the visitor do next?

If a page does not answer these questions, it is not yet a Golden Page.
