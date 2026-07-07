# T-money Page Standard

## Purpose

This document defines the minimum page standard for `tmoney.html`.

It is a page-spec baseline for future T-money v2 work. It is not an instruction to edit the live page in this task.

## User Problem

Visitors need to understand how to prepare for public transport payment in Korea without relying on unverified claims, product hype, or image-only explanations.

The page should help users decide whether T-money is relevant to their trip and what information must be checked before use.

## Page Scope

`tmoney.html` should remain a practical transport-card guide for visitors to Korea.

Approved scope:

- What T-money is for at a high level.
- When a visitor may need a transport card.
- How the page should explain buying, charging, tapping, balance, mobile use, and common mistakes only when supported by official sources.
- How T-money relates to public transport, WOWPASS, and payment preparation.

## Content Boundaries

Do not publish specific fares, policies, NFC support details, refund rules, locations, or accepted-use claims unless each claim has an official source and a last-reviewed date.

Avoid presenting T-money as the only valid option for every traveler. Explain fit, limits, and verification needs.

Keep procedural guidance short. The page should reduce traveler uncertainty, not become a long technical manual.

## Relationship with WOWPASS and Payments

T-money should be treated as the transport-card topic.

WOWPASS should be treated as a visitor money-management and prepaid payment topic that may include a T-money transport function.

Payments should remain the broader payment decision page. It may point users to `tmoney.html` for transport-card details, but should not duplicate the full T-money guide.

Any WOWPASS and T-money comparison should be short, practical, and role-based. Do not turn `tmoney.html` into a long `WOWPASS vs T-money` comparison page.

## Required Content Principles

- Important information must remain visible HTML text.
- Do not rely on images, `alt`, `title`, or `figcaption` as the only place for essential instructions, labels, warnings, comparisons, or values.
- Use short sections and plain English suitable for browser translation.
- Explain advantages and limitations together.
- Keep claims objective and verifiable.
- Preserve the existing URL unless a separate approved URL strategy changes it.
- Use existing Korea Inside component patterns before adding new page-specific structure in future work.

## Official Source and Last-reviewed Rules

Official source required / last-reviewed required for:

- Purchase locations or channels.
- Recharge methods.
- Fare or discount claims.
- Mobile T-money or NFC support.
- Refund or balance rules.
- Accepted transport modes or non-transport uses.
- App, card, kiosk, machine, or service availability.

Use official sources first, including public transportation operators, official T-money sources, airport authorities, or other official service documentation where relevant.

If a claim cannot be verified, label it as requiring verification or omit it from the public page.

## Image Rules

Use real, official, or clearly referenced images for T-money cards, machines, app screens, gates, or payment interfaces.

Do not use AI-generated images to represent official cards, transport gates, app screens, kiosks, machines, maps, or payment interfaces.

Every meaningful image must have a clear role:

- Recognition: helps users identify the real card or object.
- Usage: helps users understand how the card is used.
- Decision: helps users choose between practical options.

Visible HTML near the image must explain the important meaning of the image.

## SEO / FAQ / Structured Data Rules

- Keep one clear `h1`.
- Keep title, meta description, canonical URL, and Open Graph data aligned with the T-money page identity.
- Keep FAQ visible in HTML if FAQ structured data is used.
- FAQ JSON-LD must match visible FAQ content.
- Use descriptive internal links to related guides where useful.
- Do not change SEO metadata without separate approval and a complete diff.

## Known Issues and Future Verification

- `tmoney.html` is not final-approved yet.
- Current page was restored from the clean `881767e` baseline with only launch navigation aligned.
- Quick Answer is not present.
- Source note is not present.
- `.table-scroll` is not present.
- Mobile 375px has a known 4-column comparison table overflow issue.
- T-money v2 improvement is planned but not part of this task.

## Out of Scope

- Editing `tmoney.html`.
- Adding Quick Answer.
- Adding Source note.
- Fixing mobile table overflow.
- Adding or changing `.table-scroll`.
- Updating official links.
- Changing WOWPASS, Payments, or navigation pages.
- Stage, commit, push.
