# Home Journey Preview

Last updated: 2026-07-03

## Purpose

The Home page now introduces a clear Korea travel journey immediately after the Hero section. The goal is to help visitors understand what to prepare next without changing the Hero image, overlay, navigation, or the existing guide sections below.

## Approved Flow

1. Accommodation
2. Internet
3. Arrival
4. Transportation
5. Payments
6. Maps

## Component Rules

- The section appears after the Home Hero and before the trust section.
- Each step uses a representative image, title, short introduction, and a Learn More CTA.
- Step titles are not links.
- Only the Learn More CTA links to the destination page.
- Desktop and tablet use a two-column grid.
- Mobile uses a one-column grid.
- All journey cards use the same full-width 16:9 media box.
- Card images must fill the media box with `object-fit: cover`.
- Journey card images must be full-bleed assets without baked-in outer canvas margins.
- The Internet card should not use `images/esim/hero-esim.webp` because that file includes internal white margin.
- The Payments card uses `/images/wowpass-card.webp` with a Payments-only image modifier so the WOWPASS card stays visible without changing other Journey cards.
- Existing colors, font family, radius tokens, and button structure are reused.

## Link Targets

| Step | Label | Target |
| --- | --- | --- |
| 1 | Accommodation | `accommodation.html` |
| 2 | Internet | `esim.html` |
| 3 | Arrival | `arrival.html` |
| 4 | Transportation | `tmoney.html` |
| 5 | Payments | `wowpass.html` |
| 6 | Maps | `maps.html` |

## Verification

Verification level: Level 2 - Feature Verify

Requested checks:

- CTA links exist and point to the approved destination pages.
- Mobile layout is one column.
- Tablet and desktop layouts are two columns.
- Hero CTA buttons and old four-step Hero cards are removed.
- Console errors should be checked in browser tooling when available.

## Known QA Limitation

Desktop responsive mode and browser preview checks are simulation only. Final mobile approval should be based on the Product Owner's real-device test on Galaxy S25+.
