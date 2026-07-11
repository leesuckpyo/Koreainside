# Apps Guide Page Standard

Last reviewed: 2026-07-01

## Scope

`apps.html` should remain a practical decision-support guide for visitors choosing Korea apps for navigation, translation, taxi use, messaging, food delivery, shopping, and common app problems.

## Content Preservation Requirements

`apps.html` should retain these content areas unless a separate approved task changes the page scope:

- Must-have apps
- Food & ordering
- Transport apps
- Communication
- Shopping & convenience
- App verification warning
- Apps by situation
- Related guides

App names and Korean terms should remain readable in English or romanized text. Use `Baemin (Baedal Minjok)` to keep the important app identity visible without broken Korean text.

## Required Structure

- Keep the H1 as `Essential Apps for Korea`.
- Include a quick answer section for faster mobile comprehension.
- Include a must-have apps table with visitor limitations.
- Include a setup-before-arrival checklist.
- Keep food, transport, communication, and shopping guidance without removing the original topics.
- Include situation-based backup advice.
- Include app choices by traveler type.
- Include common app problems.
- Keep visible FAQ content with 6 questions.
- Keep a visible sources and last-reviewed section.
- Keep WebPage, BreadcrumbList, and FAQPage JSON-LD.

## SEO Requirements

- Title: `Essential Apps for Korea | Maps, Translation, Taxi and Food Delivery`
- Meta description focuses on comparing app categories and visitor setup limits.
- Canonical URL: `https://getkoreainside.com/apps.html`
- Keep Open Graph and Twitter metadata.
- H1 count should remain one.
- Important app guidance remains visible HTML text, not image-only content.
- FAQ schema mirrors the visible FAQ content.

## UX Requirements

- The page remains text-first and uses existing KR Inside classes.
- Reuse existing design system components: `page-hero`, `guide-content`, `info-table`, `table-scroll`, `tip-box`, `warning-box`, `related-links`, and `chip`.
- CSS and shared JavaScript should remain unchanged unless separately approved.
- Tables use captions and horizontal scroll wrappers for mobile.
- The content should help users choose by trip type, situation, and app limitation instead of only listing apps.

## Official Source Requirements

Last reviewed: 2026-07-01.

Use official websites and app store listings to verify app identity, availability, language support, visitor limitations, and service details.

- NAVER Maps, Navigation on Google Play: https://play.google.com/store/apps/details?id=com.nhn.android.nmap&hl=en_US
- Papago official website: https://papago.naver.com/
- Naver Papago on Google Play: https://play.google.com/store/apps/details?id=com.naver.labs.translator&hl=en_US
- Kakao official services — KakaoTalk: https://www.kakaocorp.com/page/service/service?lang=en
- KakaoMap official service page: https://www.kakaocorp.com/page/service/service/KakaoMap?lang=en
- Kakao T on Google Play: https://play.google.com/store/apps/details?id=com.kakao.taxi&hl=en_US
- Baemin official website: https://www.baemin.com/
- Baemin on Google Play: https://play.google.com/store/apps/details?id=com.sampleapp&hl=en_US
- Coupang Eats official website: https://www.coupangeats.com/
- Coupang Eats on Google Play: https://play.google.com/store/apps/details?id=com.coupang.mobile.eats&hl=en_US

## Implementation Verification Requirements

- H1 count should be one.
- FAQ count should be 6.
- Internal links should include:
  - `maps.html`
  - `payments.html`
  - `esim.html`
  - `tmoney.html`
- External source links should open in a new tab with `rel="noopener noreferrer"`.
- Images should remain absent unless separately approved.
- CSS and shared JavaScript files should remain unchanged unless separately approved.
- 360px, 768px, and 1440px layouts should be checked when a browser is available.

## Remaining Risk

Korean app features, app language support, verification requirements, payment support, delivery fees, membership benefits, and service areas can change frequently. Future updates should re-check official app pages and store listings before making stronger claims about availability, pricing, or foreign-card support.
