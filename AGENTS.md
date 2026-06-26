# AGENTS.md

# Korea Inside AI Development Guide

## Project Goal

Korea Inside helps foreign visitors solve practical problems when using services in Korea.

This is not a simple tourism site or a destination guide.
It is a problem-solving platform for understanding and using Korea.

Always prioritize:

1. Accuracy
2. User experience
3. SEO
4. Mobile usability
5. Browser translation

---

## General Rules

Never redesign pages.

Keep the existing visual design.

Keep spacing, typography, colors and layout.

Do not change the design, colors, spacing, typography or layout unless the user explicitly requests that change.

Mobile First.

Responsive only.

---

## HTML Rules

HTML is the source of truth.

Images are visual illustrations only.

Important information must never exist only inside images.

Every infographic must have equivalent semantic HTML.

Every meaningful phrase, instruction, label, comparison and warning in an infographic must also appear as visible semantic HTML near that infographic.

Before finishing an infographic change, verify a one-to-one mapping between its important image text and visible HTML text.

Use:

- h1~h4
- p
- ul
- li
- table
- section
- article

whenever appropriate.

---

## Translation Rules

Support browser automatic translation.

Do not place important information inside images.

Do not solve translation problems using only:

- alt
- title
- figcaption

Those are accessibility helpers only.

The same information must exist as visible HTML.

`alt`, `title` and `figcaption` alone do not satisfy automatic-translation support.

---

## SEO Rules

Use semantic HTML.

Keep heading hierarchy.

Every page must have:

- a unique, meaningful `title`
- one non-empty `meta name="description"`
- a canonical URL
- exactly one `h1`
- meaningful `alt` text for informative images

Write meaningful alt text.

Avoid duplicated content.

Preserve internal links.

Do not remove metadata.

---

## Content Freshness Rules

Transportation, fares, travel cards, telecom plans, airport procedures and other changeable information must be checked against an official source.

Maintain the official source URL and last-reviewed date for each changeable claim or content section.

Do not present time-sensitive details as permanent facts when their availability, policy, price or operating conditions can change.

---

## CSS Rules

Reuse existing CSS whenever possible.

Avoid unnecessary new classes.

Avoid inline styles.

Keep Mobile First.

---

## Images

Never delete images unless requested.

Never replace images unless requested.

Images are visual aids.

HTML provides searchable and translatable content.

---

## Safety

Never overwrite user work.

Before editing, run `git status --short` or inspect the changed-file state.

If the requested edit overlaps existing user changes, stop and ask for confirmation before modifying that area.

Ask before destructive changes.

Always explain modified files.

Show diff before finishing.

---

## Visual Verification

Verify visual changes at these viewport widths:

- Mobile: 375px
- Tablet: 768px
- Desktop: 1440px

Confirm that existing design, colors, spacing, typography, layout and responsive behavior remain unchanged unless the user explicitly approved a design change.

---

## Project Structure

Each page owns its own assets.

images/

home/

arrival/

esim/

maps/

wowpass/

tmoney/

apps/

common/

---

## Preferred Workflow

1. Analyze

2. Explain plan

3. Wait for approval

4. Modify files

5. Verify layout at 375px, 768px and 1440px

6. Explain changes

7. Show diff

---

Always preserve the Korea Inside design language.
