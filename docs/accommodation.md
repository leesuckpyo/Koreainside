# Accommodation Page Standard

Last updated: 2026-07-07

## Purpose

This document defines the Accommodation area-selection standard for `accommodation.html`.

The Accommodation page helps foreign visitors choose where to stay before choosing a hotel. It should be an area-selection decision-support page, not a hotel recommendation, booking, or affiliate page.

## Page Identity

- Lead with area fit before hotel choice.
- Help users compare trade-offs instead of naming one universal best area.
- Use "best" wording only when it is tied to a clear traveler need, such as first-time visitors, families, budget travelers, shopping, nightlife, airport access, or large luggage.
- Keep hotel, booking, and room details as practical checks that support area choice.
- Do not add Booking API, Agoda, Booking.com, production booking, or affiliate integration guidance to this page standard.

## Current Page Structure

The current `accommodation.html` structure is:

1. Hero and area-first introduction.
2. Quick Decision guide-card hub.
3. Quick Answer.
4. Area Comparison cards.
5. Common Mistakes.
6. FAQ.
7. Compact Essential Korea Guides.

Do not treat removed or absent sections as required current structure. Booking checklist, source display, and related-guide layouts may be considered only as future internal page improvements after approval.

## Decision Criteria

Accommodation guidance should help users judge areas by practical travel constraints:

- Advantages and limitations.
- Typical price level or budget fit.
- Airport access.
- Subway and station access.
- Elevator availability and station exits.
- Hills, stairs, sidewalks, and suitcase friendliness.
- Noise and nightlife risk.
- Family suitability.
- Shopping, food, convenience stores, and daily errands.
- Walking distance that feels realistic with luggage, rain, children, or late arrivals.
- Room size, check-in, luggage storage, cancellation policy, and hotel support as booking checks.

Avoid absolute labels such as "good" or "bad" without context. Explain who an area fits, who it may not fit, and what users should verify before booking.

## Area-Focused Guide Flow

Detailed explanations should live in focused area guides or area-focused supporting guides when they would make the main Accommodation page too long.

The main page should stay fast to scan. Area cards should remain concise and use representative images, area names, quick ratings, and clear explore links.

## Hero and Image Policy

The hero introduces area selection, not hotel booking.

Core message: choose the area first, then choose the hotel.

Images are visual support only. Important Accommodation guidance must remain visible HTML text and must not be available only inside images.

## SEO and Content Notes

Preserve the existing URL: `accommodation.html`.

Accommodation SEO may use common search language such as "best area", but the visible content should keep the recommendation conditional and practical.

The page should preserve one clear H1, useful metadata, visible FAQ text, FAQ JSON-LD alignment, internal links, and crawlable guide-card links.

## Booking and Affiliate Scope

Booking integration is outside the current Accommodation page scope and should only be considered after the area-selection flow is stable.

Do not introduce Booking API, Agoda, Booking.com, production booking, hotel inventory, price feed, availability feed, or affiliate-ranking logic in this page standard.

Revenue opportunities must not outrank user fit, accuracy, transparent limits, or objective decision-support.

## Sources and Maintenance

Accommodation quality, hotel prices, reviews, transfer availability, check-in rules, and airport transport schedules change over time.

Before making stronger route, booking, or availability claims, verify current details against official operator pages or current provider information and record the reviewed date in the approved content task.

Useful source references:

- VISITKOREA official travel information: https://english.visitkorea.or.kr/svc/main/index.do
- AREX Airport Railroad official website: https://www.airportrailroad.com/main
- Existing Korea Inside Seoul stay decision guide: `where-to-stay-in-seoul.html`
