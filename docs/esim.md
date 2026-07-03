# eSIM Guide Page Review

Date: 2026-07-03

## Scope

Updated `esim.html` as a decision-support guide for visitors choosing between data-only eSIMs, Korean number eSIMs, physical SIM cards, and international roaming.

## Page Objective

The page should help travelers decide what mobile option fits their trip before arriving in Korea. It should not read like a sales page. Selection criteria, verification limits, compatibility, and common failure cases are presented before provider placeholders.

## Structural Changes

- Replaced the old page-specific header and footer with the shared Go Inside / KR Inside header and footer structure used on Home.
- Kept a single `h1`: `How to Use eSIM in Korea`.
- Added breadcrumb navigation and JSON-LD breadcrumb schema.
- Added FAQ content and FAQPage JSON-LD schema.
- Rebuilt the main content around practical sections:
  - Quick answer
  - Phone compatibility
  - Decision flow
  - Korea mobile options comparison table
  - Korean phone number needs
  - Passport verification
  - Common eSIM problems
  - Provider selection placeholders
  - FAQ
  - Recommended next guides

## Provider Policy

Provider buttons are placeholders only. No affiliate or outbound provider links are active on this version. Future provider links should be added only after the provider page, disclosure language, plan criteria, and tracking policy are reviewed.

## SEO Notes

- Title: `How to Use eSIM in Korea | KR Inside`
- Meta description focuses on comparing eSIM, Korean number eSIM, physical SIM, and roaming choices.
- Canonical URL: `https://getkoreainside.com/esim.html`
- Important text is visible HTML text, not image-only content.
- FAQ schema mirrors the visible FAQ content.

## Accessibility Notes

- Header navigation uses `nav` with `aria-label`.
- Active navigation item uses `aria-current="page"`.
- Mobile menu behavior is handled by `common.js` with `aria-expanded`.
- The comparison table includes a visible `caption`.
- Provider placeholders are disabled buttons, not misleading purchase links.
- Focus-visible styles are retained for eSIM-specific links, summaries, and controls.
- The hero background image is decorative and supports recognition only; important eSIM decision content remains visible HTML text.

## QA Checklist

- H1 count: one expected.
- Header and footer should match Home structure.
- CTA links point to in-page anchors:
  - `#phone-compatibility`
  - `#compare-options`
- Provider buttons are intentionally disabled placeholders.
- The hero background uses `images/esim/hero-esim.webp`; important content is still visible HTML text and does not depend on the image.
- The comparison table uses horizontal overflow for small mobile widths instead of compressing columns.
- Responsive targets to verify visually:
  - 360px mobile
  - 768px tablet
  - 1440px desktop

## Post-Implementation Review

Reviewed after implementation on 2026-07-01. One spacing issue was corrected: the eSIM page no longer adds extra top padding below the shared sticky header. Header and footer markup were not changed during this review.

Static checks completed:

- Single H1 confirmed.
- Hero H1 text confirmed as `How to Use eSIM in Korea`.
- FAQ count confirmed as 8.
- Provider links confirmed as disabled placeholder buttons with no outbound provider URLs.
- No content image dependency found in `esim.html`; the hero image is applied through CSS as a background.
- FAQPage and BreadcrumbList JSON-LD are present.

## Hero Image Update

Updated on 2026-07-03.

- Applied approved hero image: `images/esim/hero-esim.webp`.
- Moved source asset from `images/hero-esim.webp`.
- Hero layout, copy, buttons, and overlay gradient were preserved.
- Verification level: Level 1 - Quick Verify.
- Known QA limitation: Desktop responsive mode is only a simulation. Final mobile approval must be based on Product Owner testing on Galaxy S25+.

## Future Verification

Mobile plan prices, passport requirements, airport pickup rules, supported verification features, and provider plan details can change frequently. Future provider-specific content should cite official provider pages and record last-reviewed dates.
