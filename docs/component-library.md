# Korea Inside UI Component Library

## Purpose

This document defines reusable UI components for Korea Inside pages.

The goal is to keep future pages consistent, mobile-first, SEO-friendly, accessible and easier to maintain without redesigning common patterns for every page.

This is documentation only. It does not define implementation code, CSS, JavaScript or navigation changes.

## Component Principles

Every reusable component should:

- Help users make practical decisions.
- Keep important information as visible HTML text.
- Support browser translation.
- Be easy to scan on mobile.
- Preserve the existing Korea Inside visual language.
- Avoid decorative complexity.
- Explain trade-offs honestly.
- Support SEO through clear headings and structured content.
- Remain accessible without relying on images, icons or color alone.

---

## 1. Hero

### Purpose

The Hero introduces the page problem, audience and decision value.

It should quickly answer:

- What is this page about?
- Who is it for?
- What decision will it help with?

### Where It Should Be Used

Use on:

- All major SEO content pages.
- Decision guides.
- Comparison pages.
- Hub pages.
- Tool landing pages.

### Standard Layout

Recommended structure:

```text
Breadcrumb
Eyebrow
H1
Subtitle
CTA row
Optional support card or image
```

### Title

The title should:

- Be the only H1 on the page.
- Match the main search intent.
- Be direct and practical.
- Avoid vague marketing language.

### Subtitle

The subtitle should:

- Explain the user's problem.
- Mention the decision criteria.
- Stay short enough for mobile scanning.

### CTA

Hero CTAs should:

- Link to the most useful in-page sections.
- Use clear labels.
- Avoid more than three primary CTAs.
- Prioritize decision support.

Good examples:

- `Quick Answer`
- `Compare Options`
- `Common Mistakes`
- `FAQ`

### Image Placement

Images are optional.

Use images only when they help the user understand the topic.

Rules:

- Do not use decorative images as a substitute for content.
- Keep important information as visible text.
- Place images near the content they support.
- Use meaningful alt text for informative images.

### Design Principles

- Practical, not promotional.
- Clean opening structure.
- Strong hierarchy.
- Clear next action.

### UX Principles

- The user should understand the page purpose within a few seconds.
- Mobile users should see the core value before scrolling too far.

### Accessibility Considerations

- One H1 only.
- CTA links should have descriptive text.
- Images must include useful alt text when informative.

### SEO Considerations

- H1 should align with page title and search intent.
- Subtitle should reinforce topical relevance.
- Avoid hiding main content inside images.

---

## 2. Quick Answer Card

### Purpose

The Quick Answer Card gives the practical answer before the detailed explanation.

### Where It Should Be Used

Use on:

- Guide pages.
- Comparison pages.
- Decision pages.
- Troubleshooting pages.

### Content

It should include:

- Best default recommendation.
- 2-3 alternative choices.
- Main trade-off.
- Warning or limitation if needed.

### Layout

Recommended structure:

```text
Quick answer heading
Short answer
Choose this if...
Choose another option if...
Watch out...
```

### When To Use

Use when the page answers a user decision.

Do not use for purely informational pages unless there is a clear practical answer.

### Design Principles

- Strong visual priority.
- Short text.
- Clear labels.
- No dense paragraphs.

### UX Principles

- Reduce uncertainty immediately.
- Help users decide whether they need to read the full page.

### Accessibility Considerations

- Do not rely on color alone to mark the best answer.
- Use text labels such as `Best for`, `Avoid if`, or `Watch out`.

### SEO Considerations

- Use natural language that matches search intent.
- Keep the answer crawlable as visible text.

---

## 3. Decision Card

### Purpose

Decision Cards summarize who an option fits and what trade-off it carries.

### Where It Should Be Used

Use for:

- Transport options.
- Stay areas.
- Payment options.
- eSIM/SIM choices.
- Map app choices.
- Hotel suitability summaries.

### Structure

Recommended structure:

```text
Option name
Best for
Main advantage
Main limitation
Alternative
```

### Recommended Icons

Use icons only to support scanning.

Recommended categories:

- Airport / transport
- Subway / train
- Taxi
- Luggage
- Family
- Budget
- Time
- Phone / internet
- Card / payment
- Warning

### Visual Hierarchy

Suggested order:

1. Option name.
2. Suitability label.
3. Short explanation.
4. Limitation.
5. Alternative.

### Design Principles

- Compact.
- Consistent.
- Clear trade-offs.
- Avoid long paragraph blocks.

### UX Principles

- Users should be able to compare cards quickly.
- Every card should explain why the option fits.

### Accessibility Considerations

- Icons must not be the only source of meaning.
- Headings should be semantic and logical.

### SEO Considerations

- Keep option names and decision criteria as text.
- Use descriptive headings where possible.

---

## 4. Comparison Table

### Purpose

Comparison Tables make trade-offs visible across multiple options.

### Where It Should Be Used

Use for:

- Area comparisons.
- Transport comparisons.
- Payment method comparisons.
- eSIM plan comparisons.
- Hotel suitability comparisons.
- App comparisons.

### Standard Columns

Choose columns based on the decision.

Common columns:

| Column | Purpose |
|---|---|
| Option | Name of the choice. |
| Best For | User fit. |
| Main Advantage | Why to choose it. |
| Main Limitation | Why it may not fit. |
| Budget | Cost sensitivity. |
| Convenience | Ease of use. |
| Luggage / Family Fit | Practical constraints. |
| Late-night Notes | Time-sensitive limitations. |

### Mobile Behavior

Tables must remain usable on mobile.

Acceptable behavior:

- Horizontal scroll in a clear container.
- Short column names.
- Concise cell text.
- Mobile card alternative for very wide tables.

### Responsive Rules

- Avoid too many columns.
- Avoid long sentences inside cells.
- Prioritize decision columns over complete data.
- If a table becomes unreadable, split it into smaller tables.

### Design Principles

- Neutral by default.
- Highlight only when it clarifies the best default option.
- Do not overuse color.

### UX Principles

- Help users compare quickly.
- Show limitations as clearly as advantages.

### Accessibility Considerations

- Use meaningful row and column labels.
- Do not communicate ranking by color alone.
- Keep text readable without zoom.

### SEO Considerations

- Tables should contain useful crawlable text.
- Avoid creating tables only for decoration.

---

## 5. Info Card

### Purpose

Info Cards explain useful context that supports a decision.

### Where It Should Be Used

Use for:

- Local context.
- Definitions.
- Practical background.
- Setup notes.
- Process explanations.

### Design Principles

- Calm and neutral.
- Short heading.
- One short paragraph or small bullet list.

### UX Principles

- Add clarity without distracting from the main decision.
- Avoid repeating content already shown in the main section.

### Accessibility Considerations

- Use text labels, not only icons.
- Keep content concise.

### SEO Considerations

- Include useful explanatory terms naturally.
- Avoid generic filler.

---

## 6. Warning Card

### Purpose

Warning Cards prevent users from making costly or inconvenient mistakes.

### Where It Should Be Used

Use for:

- Late-night transport limitations.
- Payment limitations.
- Airport timing issues.
- Luggage and walking difficulty.
- Noise risk.
- Changeable information.

### Design Principles

- Use warning styling sparingly.
- Make the risk clear.
- Explain what to do instead.

### UX Principles

- A warning should be actionable.
- Avoid fear-based wording.

### Accessibility Considerations

- Do not rely on warning color alone.
- Include text such as `Watch out`, `Important`, or `Before you choose`.

### SEO Considerations

- Warning text often matches real user concerns.
- Keep it visible and specific.

---

## 7. Tip Card

### Purpose

Tip Cards provide practical shortcuts or small pieces of local advice.

### Where It Should Be Used

Use for:

- Traveler shortcuts.
- Small setup steps.
- Local usage tips.
- App usage notes.
- Payment or transit hints.

### Design Principles

- Friendly and concise.
- One useful idea per card.
- No long explanations.

### UX Principles

- Help users avoid friction.
- Support the main decision instead of adding unrelated advice.

### Accessibility Considerations

- Label the card clearly as a tip.
- Keep wording simple.

### SEO Considerations

- Tips can include long-tail practical phrases.
- Avoid keyword stuffing.

---

## 8. FAQ Accordion

### Purpose

FAQ Accordions answer practical follow-up questions.

### Where It Should Be Used

Use near the bottom of:

- SEO guide pages.
- Comparison pages.
- Decision pages.
- Troubleshooting pages.

### Design Principles

- Questions visible.
- Answers concise.
- Accordion styling consistent.

### UX Principles

- Use real user questions.
- Answer directly in the first sentence.
- Keep most answers to 1-3 short paragraphs.

### Accessibility Considerations

- Accordions must be keyboard-accessible.
- Questions should remain readable and crawlable.
- Avoid hiding essential page content only inside FAQ.

### SEO Considerations

- FAQ should support the main content.
- Do not create FAQ only for keywords.
- Use natural search-style questions.

---

## 9. Related Guide Cards

### Purpose

Related Guide Cards help users continue the same decision journey.

### Where It Should Be Used

Use near the end of content pages.

### Structure

Each card should include:

- Guide title.
- One-sentence reason to read it.
- Descriptive link text.

### Design Principles

- Keep cards compact.
- Limit to 3-6 related guides.
- Link only to genuinely relevant pages.

### UX Principles

- Help the user choose the next practical step.
- Avoid random link blocks.

### Accessibility Considerations

- Link text should be descriptive.
- Do not use vague labels like `Click here`.

### SEO Considerations

- Supports internal linking.
- Anchor text should describe the destination.
- Avoid excessive repeated links.

---

## 10. Area Card

### Purpose

Area Cards summarize whether a Seoul area fits a travel scenario.

### Where It Should Be Used

Use on:

- Where-to-stay pages.
- Area comparison pages.
- Traveler-type stay guides.
- Hotel decision pages.

### Structure

Recommended fields:

```text
Area name
Best for
Why stay here
Watch out
Airport access
Suitcase friendliness
Noise / family / budget notes
Not ideal for
Alternative area
```

### Design Principles

- Use the same field order on every Area Card.
- Keep area descriptions practical.
- Do not exaggerate an area.

### UX Principles

- Help users understand fit quickly.
- Explain why an area may not work.

### Accessibility Considerations

- Use headings for area names.
- Avoid icon-only ratings.

### SEO Considerations

- Area names and travel criteria should be text.
- Cards should support long-tail search intent.

---

## 11. Hotel Card

### Purpose

Hotel Cards summarize hotel suitability without turning Korea Inside into a generic hotel listing site.

### Where It Should Be Used

Use only when hotel recommendations are verified and useful.

### Structure

Recommended fields:

```text
Hotel name
Area
Best for
Strengths
Limitations
Airport access
Suitcase friendliness
Family / business / budget fit
Verification status
Affiliate disclosure if relevant
```

### Design Principles

- Editorial first.
- Suitability over popularity.
- Clear limitations.
- Avoid ranking by commission.

### UX Principles

- Help users understand whether a hotel fits their situation.
- Avoid overwhelming users with too many hotel cards.

### Accessibility Considerations

- Hotel names should be readable text.
- Ratings or fit labels should not rely on color alone.

### SEO Considerations

- Avoid copied review text.
- Use original evaluation criteria.
- Include verification status where needed.

---

## 12. Transportation Card

### Purpose

Transportation Cards compare ways to move around Korea.

### Where It Should Be Used

Use for:

- Airport transfer pages.
- AREX, airport bus, taxi and pickup comparisons.
- Subway and bus guides.
- Late-night transport pages.

### Structure

Recommended fields:

```text
Transport option
Best for
Travel time / reliability note
Cost level
Luggage fit
Late-night availability
Main limitation
Alternative
```

### Design Principles

- Practical and direct.
- Compare fit, not only speed.
- Mention limitations clearly.

### UX Principles

- Help users choose based on constraints.
- Avoid assuming the cheapest or fastest option is always best.

### Accessibility Considerations

- Use text labels for timing, cost and suitability.
- Avoid icon-only transport indicators.

### SEO Considerations

- Use natural phrases users search for, such as airport bus, AREX, taxi and airport pickup.
- Keep route and timing notes easy to update.

---

## 13. Affiliate Card

### Purpose

Affiliate Cards present paid options only after the user understands the decision.

### Where It Should Be Used

Use for:

- eSIM providers.
- Airport pickup.
- Hotel booking links.
- Transport or travel passes.
- Carefully selected partner services.

### Structure

Recommended fields:

```text
Product or service
Useful if
Not ideal if
Why Korea Inside mentions it
Disclosure
Next action
```

### Design Principles

- Clearly labeled.
- Visually secondary to editorial advice.
- Trust-first.
- No hidden limitations.

### UX Principles

- Affiliate links should feel like a helpful next step, not pressure.
- Provide alternatives where useful.

### Accessibility Considerations

- Disclosure must be visible text.
- Link text must clearly describe the action.

### SEO Considerations

- Avoid thin affiliate-only content.
- Keep editorial content stronger than commercial blocks.
- Use appropriate disclosure language.

---

## 14. Call-to-Action Block

### Purpose

Call-to-Action Blocks tell users what to do next.

### Where It Should Be Used

Use:

- After major decisions.
- At the end of important pages.
- Near related guide paths.
- Before affiliate handoff, when appropriate.

### Structure

Recommended fields:

```text
Action heading
Short reason
Primary action
Secondary action
```

### Design Principles

- Clear and focused.
- No more than two primary actions.
- Avoid generic marketing copy.

### UX Principles

- The action should follow naturally from the page.
- Do not interrupt before the user understands the options.

### Accessibility Considerations

- Button or link labels should describe the destination.
- Avoid multiple links with the same vague label.

### SEO Considerations

- Internal CTAs strengthen crawl paths.
- Use descriptive anchor text.

---

## 15. Trust Box

### Purpose

Trust Boxes explain why users can rely on the guidance.

### Where It Should Be Used

Use on pages with:

- Changeable information.
- Transportation rules.
- Prices or schedules.
- Payment limitations.
- Affiliate recommendations.
- Hotel or area suitability claims.

### Structure

Recommended fields:

```text
What this guidance is based on
Last reviewed date
Official source note
What may change
How to verify before travel
```

### Design Principles

- Calm and factual.
- Short.
- No alarmist language.

### UX Principles

- Build confidence.
- Explain uncertainty where needed.

### Accessibility Considerations

- Keep trust notes as visible text.
- Do not hide them only in tooltips.

### SEO Considerations

- Supports freshness and credibility.
- Helps distinguish verified facts from editorial judgment.

---

## 16. Common Icons

### Purpose

Icons support quick scanning.

They should never replace text.

### Recommended Icon Set

Common categories:

| Icon Category | Use |
|---|---|
| Airport | Airport arrival, airport transfer. |
| Train | AREX, subway, rail access. |
| Bus | Airport bus, city bus. |
| Taxi | Taxi and pickup. |
| Luggage | Suitcase friendliness. |
| Family | Family travel. |
| Card | Payments, WOWPASS, T-money. |
| Phone | eSIM, apps, maps. |
| Map | Navigation and area choice. |
| Clock | late-night, timing, schedules. |
| Warning | Important limitations. |
| Check | Recommended or suitable. |

### Design Principles

- Consistent size.
- Consistent style.
- Used sparingly.

### UX Principles

- Icons should help scanning, not decorate every section.

### Accessibility Considerations

- Provide text labels.
- Do not communicate meaning with icons alone.

### SEO Considerations

- Icons do not replace crawlable text.

---

## 17. Color Usage Rules

### Purpose

Color should clarify hierarchy and meaning.

### Where It Should Be Used

Use color for:

- Primary CTAs.
- Best default highlights.
- Warning states.
- Section rhythm.
- Subtle card emphasis.

### Design Principles

- Preserve the existing Korea Inside palette.
- Avoid introducing new color systems without approval.
- Use color consistently across pages.
- Keep warning colors for real warnings.

### UX Principles

- Color should help users scan.
- Do not create visual noise.

### Accessibility Considerations

- Ensure sufficient contrast.
- Do not rely on color alone.

### SEO Considerations

- None directly, but better readability supports user engagement.

---

## 18. Typography Rules

### Purpose

Typography should make practical information easy to scan.

### Where It Should Be Used

Applies to all components.

### Design Principles

- Clear heading hierarchy.
- Short paragraphs.
- Consistent label style.
- Avoid oversized text inside compact cards.

### UX Principles

- Users should identify the answer quickly.
- Mobile reading should feel comfortable.

### Accessibility Considerations

- Text must remain readable without zoom.
- Avoid tiny table text.
- Maintain logical heading order.

### SEO Considerations

- Proper headings help search engines understand structure.
- Do not use visual text in images as a substitute for real headings.

---

## 19. Spacing Rules

### Purpose

Spacing creates readable structure.

### Where It Should Be Used

Applies to:

- Sections.
- Cards.
- Tables.
- CTA rows.
- FAQ groups.
- Related guide blocks.

### Design Principles

- Use consistent section rhythm.
- Avoid stacking too many large cards.
- Keep related items grouped.
- Avoid excessive whitespace that pushes the answer too far down.

### UX Principles

- Mobile users should not need to scroll through empty space.
- Dense sections should be broken into smaller groups.

### Accessibility Considerations

- Adequate spacing improves tap and reading comfort.

### SEO Considerations

- None directly, but readability supports engagement.

---

## 20. Responsive Rules

### Purpose

Every component must work on desktop, tablet and mobile.

### Where It Should Be Used

Applies to all components.

### Design Principles

- Mobile-first.
- Cards stack cleanly.
- CTA rows wrap naturally.
- Tables remain readable.
- Images do not push core text too far down.

### UX Principles

- The most important answer should appear early on mobile.
- Touch targets should be comfortable.
- Long components should remain easy to scan.

### Accessibility Considerations

- Avoid horizontal overflow except intentional table scroll containers.
- Maintain readable text size.
- Keep controls keyboard-accessible.

### SEO Considerations

- Responsive pages should preserve the same important text across viewports.
- Do not hide important content on mobile.

---

## Component Selection Rules

Use the smallest component that solves the user need.

| User Need | Recommended Component |
|---|---|
| Understand page purpose | Hero |
| Get fast answer | Quick Answer Card |
| Choose between options | Decision Card or Comparison Table |
| Learn context | Info Card |
| Avoid mistake | Warning Card |
| Get small practical advice | Tip Card |
| Answer remaining questions | FAQ Accordion |
| Continue journey | Related Guide Cards |
| Compare Seoul areas | Area Card |
| Evaluate hotels | Hotel Card |
| Compare transport | Transportation Card |
| Offer paid next step | Affiliate Card |
| Move to next action | Call-to-Action Block |
| Explain reliability | Trust Box |

## Final Component Standard

A Korea Inside component is reusable only if it:

1. Helps the user decide or understand.
2. Works on mobile.
3. Preserves important text as HTML.
4. Explains trade-offs honestly.
5. Can be reused without redesigning the page.
6. Supports SEO and accessibility.
