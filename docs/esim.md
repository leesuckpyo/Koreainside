# eSIM Guide Page Standard

Last reviewed: 2026-07-03

## Scope

`esim.html` should remain a decision-support guide for visitors choosing between data-only eSIMs, Korean number eSIMs, physical SIM cards, and international roaming.

## Page Objective

The page should help travelers decide what mobile option fits their trip before arriving in Korea. It should not read like a sales page. Selection criteria, verification limits, compatibility, and common failure cases are presented before provider placeholders.

## Required Structure

- Use the shared Go Inside / KR Inside header and footer structure used on Home.
- Keep a single `h1`: `How to Use eSIM in Korea`.
- Include breadcrumb navigation and JSON-LD breadcrumb schema.
- Include FAQ content and FAQPage JSON-LD schema.
- Structure the main content around practical sections:
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

Provider buttons should remain placeholders only unless a separate approved task changes them. No affiliate or outbound provider links should be active before the provider page, disclosure language, plan criteria, and tracking policy are reviewed.

## SEO Requirements

- Title: `How to Use eSIM in Korea | KR Inside`
- Meta description focuses on comparing eSIM, Korean number eSIM, physical SIM, and roaming choices.
- Canonical URL: `https://getkoreainside.com/esim.html`
- Important text is visible HTML text, not image-only content.
- FAQ schema mirrors the visible FAQ content.

## Accessibility Requirements

- Header navigation uses `nav` with `aria-label`.
- Active navigation item uses `aria-current="page"`.
- Mobile menu behavior is handled by `common.js` with `aria-expanded`.
- The comparison table includes a visible `caption`.
- Provider placeholders are disabled buttons, not misleading purchase links.
- Focus-visible styles are retained for eSIM-specific links, summaries, and controls.
- The hero background image is decorative and supports recognition only; important eSIM decision content remains visible HTML text.

## Implementation Verification Requirements

- H1 count should be one.
- Header and footer should match Home structure.
- Hero H1 text should remain `How to Use eSIM in Korea`.
- FAQ count should be 8.
- FAQPage and BreadcrumbList JSON-LD should be present.
- CTA links point to in-page anchors:
  - `#phone-compatibility`
  - `#compare-options`
- Provider buttons should be intentionally disabled placeholders with no outbound provider URLs.
- The hero background uses `images/esim/hero-esim.webp`; important content is still visible HTML text and does not depend on the image.
- The comparison table uses horizontal overflow for small mobile widths instead of compressing columns.
- Responsive targets should be verified visually when browser verification is available:
  - 360px mobile
  - 768px tablet
  - 1440px desktop

## Source Verification Requirements

Mobile plan prices, passport requirements, airport pickup rules, supported verification features, and provider plan details can change frequently. Future provider-specific content should cite official provider pages and record last-reviewed dates.
