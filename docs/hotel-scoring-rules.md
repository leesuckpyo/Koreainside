# Hotel Scoring Rules

## 1. Document Purpose

This document defines the standard scoring rules for the Korea Inside Stay Guide recommendation engine.

Every hotel should be evaluated using exactly the same scoring rules so recommendations are consistent, explainable, and repeatable.

The scoring system exists to help Korea Inside compare hotels by user fit, not by popularity, advertising value, or affiliate potential.

This document supports:

- `docs/hotel-database.md`
- `docs/stay-area-database.md`
- `docs/decision-engine.md`
- Stay Guide recommendation results

## 2. Editorial Principles

### User First

The hotel that best fits the user's situation should be recommended first.

### Recommendation Before Affiliate

Affiliate links must never decide the recommendation. Booking or Agoda links should appear only after the hotel is useful for the user.

### Same Input = Same Result

The same user conditions should produce the same scoring result.

### Explain Every Recommendation

Every recommended hotel must include short reasons that explain the score.

### No Review Copying

Do not copy review text from Booking, Agoda, Google Reviews, Reddit, blogs, or any other source.

### Editorial Scoring Only

Scores are Korea Inside editorial scores for decision support. They are not official hotel ratings or live booking scores.

## 3. Score Scale

| Score | Meaning |
| ----- | ------- |
| 5 | Excellent |
| 4 | Good |
| 3 | Average |
| 2 | Weak |
| 1 | Poor |

## 4. Scoring Categories

### Airport Access

Score 5:

- Direct or very simple airport access.
- Strong AREX or airport bus access.
- Low transfer burden with luggage.

Score 4:

- Good airport access with one manageable transfer or reliable airport bus access.
- Reasonable for most travelers.

Score 3:

- Usable airport access, but requires route planning or moderate transfer burden.

Score 2:

- Inconvenient airport access with multiple transfers, long walking, or limited direct service.

Score 1:

- Poor airport access for foreign visitors, especially with luggage or late-night arrival.

### Suitcase Friendly

Score 5:

- Very easy with large suitcases.
- Short walk, elevator-friendly route, easy entrance, and low station-exit friction.

Score 4:

- Generally suitcase-friendly with minor walking or route checks.

Score 3:

- Manageable with suitcases, but route details must be checked.

Score 2:

- Difficult with large suitcases due to distance, stairs, slopes, or crowded streets.

Score 1:

- Poor for large suitcases and should not be recommended when luggage is a priority.

### Family Friendly

Score 5:

- Strong fit for families.
- Good room practicality, quietness, access, luggage handling, and low walking burden.

Score 4:

- Good family fit with minor limitations.

Score 3:

- Average family fit. Works for some families, but room type and location need review.

Score 2:

- Weak family fit due to noise, small rooms, difficult access, or nightlife-heavy surroundings.

Score 1:

- Poor family fit and should usually be avoided for family travelers.

### Business Friendly

Score 5:

- Excellent for business travelers.
- Strong business district access, reliable transit, quiet work environment, and practical room setup.

Score 4:

- Good business fit with useful access and predictable stay conditions.

Score 3:

- Average business fit. Usable but not a clear business recommendation.

Score 2:

- Weak business fit due to location, noise, room practicality, or transit burden.

Score 1:

- Poor business fit.

### Quiet Stay

Score 5:

- Very strong quiet-stay profile.
- Low nightlife, low road noise risk, and calm surroundings.

Score 4:

- Generally quiet with only minor noise concerns.

Score 3:

- Average quietness. Noise may depend on room location or floor.

Score 2:

- Weak quiet-stay fit due to busy streets, nightlife, traffic, or crowd noise.

Score 1:

- Poor for quiet stays.

### Night Noise

Score 5:

- Very low night noise risk.

Score 4:

- Low night noise risk.

Score 3:

- Moderate night noise risk depending on room location.

Score 2:

- High night noise risk from nightlife, traffic, crowds, or nearby venues.

Score 1:

- Very high night noise risk and not suitable for quiet-sensitive users.

### Luxury

Score 5:

- Luxury hotel positioning with premium service, facilities, room quality, and brand expectations.

Score 4:

- Strong upscale experience, but not necessarily top luxury.

Score 3:

- Average comfort and service level.

Score 2:

- Basic or practical hotel experience.

Score 1:

- Minimal luxury value.

### Budget

Score 5:

- Strong budget fit for the area and useful for cost-sensitive travelers.

Score 4:

- Good value compared with similar hotels or areas.

Score 3:

- Average price-value fit.

Score 2:

- Weak budget fit or often expensive for the value offered.

Score 1:

- Poor budget fit.

### Shopping

Score 5:

- Excellent shopping access within the area or very easy transit to major shopping zones.

Score 4:

- Good shopping access.

Score 3:

- Average shopping access.

Score 2:

- Weak shopping access.

Score 1:

- Poor shopping access.

### Food

Score 5:

- Excellent restaurant access with many practical meal options nearby.

Score 4:

- Good food access.

Score 3:

- Average food access.

Score 2:

- Weak food access or limited convenient options.

Score 1:

- Poor food access.

### Cafes

Score 5:

- Excellent cafe access and strong cafe-focused travel fit.

Score 4:

- Good cafe access.

Score 3:

- Average cafe access.

Score 2:

- Weak cafe access.

Score 1:

- Poor cafe access.

### Subway Access

Score 5:

- Very close and easy access to useful subway lines with low walking difficulty.

Score 4:

- Good subway access with manageable walking.

Score 3:

- Average subway access. Distance or line usefulness may require checking.

Score 2:

- Weak subway access due to distance, transfers, or walking burden.

Score 1:

- Poor subway access.

### Walking Difficulty

Score 5:

- Very easy walking route.
- Short, flat, simple, and suitable for luggage.

Score 4:

- Mostly easy walking route with minor friction.

Score 3:

- Average walking difficulty.
- Manageable for most users, but luggage or weather may matter.

Score 2:

- Difficult walking route due to distance, slope, stairs, crossings, or crowding.

Score 1:

- Very difficult walking route and unsuitable for users with luggage, children, or mobility concerns.

## 5. Best For Rules

### First Time

Use when the hotel has strong area convenience, subway access, food access, and low decision burden for visitors who do not know Seoul well.

### Couple

Use when the hotel fits cafes, food, atmosphere, shopping, quiet comfort, or nightlife depending on the couple's travel style.

### Family

Use when the hotel has strong family-friendly score, suitcase-friendly access, practical room options, and low night-noise risk.

### Solo

Use when the hotel has strong subway access, food access, safe and practical surroundings, and flexible area fit.

### Business

Use when the hotel has strong business-friendly score, reliable transit, quietness, and access to business districts.

### Luxury

Use when luxury score is 4 or 5 and the hotel supports premium service, comfort, and facilities.

### Budget

Use when budget score is 4 or 5 and the hotel still has acceptable access, cleanliness, and safety fit.

### Shopping

Use when shopping score is 4 or 5 and shopping access is a meaningful reason to choose the hotel.

### Food Trip

Use when food score is 4 or 5 and the surrounding area supports convenient meals and food exploration.

### Nightlife

Use when nightlife fit is strong and night noise is acceptable for the target user.

Do not use for quiet-sensitive travelers.

### Long Stay

Use when the hotel has practical room types, laundry or residence-like convenience, good food access, and manageable transport.

## 6. Not Ideal For Rules

Use `Not Ideal For` when a hotel has a clear mismatch with a user type or priority.

Standard rules:

- If Airport Access is 1 or 2, mark not ideal for airport-access-first users.
- If Suitcase Friendly is 1 or 2, mark not ideal for large-suitcase travelers.
- If Family Friendly is 1 or 2, mark not ideal for families.
- If Quiet Stay is 1 or 2, mark not ideal for quiet-sensitive users.
- If Night Noise is 1 or 2, mark not ideal for light sleepers.
- If Luxury is 1 or 2, mark not ideal for luxury-focused users.
- If Budget is 1 or 2, mark not ideal for budget-first users.
- If Subway Access is 1 or 2, mark not ideal for subway-first itineraries.
- If Walking Difficulty is 1 or 2, mark not ideal for travelers with luggage, children, or mobility concerns.

`Not Ideal For` should be specific and practical, not generic.

## 7. Review Summary Rules

Review summaries must describe recurring patterns only.

Do not copy reviews.

Do not quote reviews.

Do not overstate a pattern from a small number of comments.

### Strengths

Summarize repeated positive patterns that support recommendation fit.

Examples:

- convenient location
- helpful staff
- clean rooms
- easy subway access
- good room view

### Weaknesses

Summarize repeated negative patterns that affect recommendation fit.

Examples:

- small rooms
- noise
- difficult walking route
- slow elevators
- inconsistent cleaning

### Frequently Praised

Use for positive comments that appear repeatedly across sources.

Do not treat one review as a trend.

### Common Complaints

Use for repeated complaints that may change the recommendation.

Complaints should be connected to user fit when possible.

### Korea Inside Notes

Write the editorial interpretation for foreign travelers.

Examples:

- Good hotel, but not ideal with two large suitcases.
- Strong location for shopping, but light sleepers should avoid lower floors.
- Works better for couples than families.

## 8. Verification Sources

Use multiple sources before final scoring.

Required or useful sources:

- Official Website
- Google Maps
- Booking
- Agoda
- Google Reviews
- Reddit
- Public Travel Blogs

Source use rules:

- Official Website: confirm hotel identity, address, facilities, room types, and operating status.
- Google Maps: confirm location, walking route, nearby transit, and recent review patterns.
- Booking: check room types, traveler reviews, cancellation patterns, and availability signals.
- Agoda: compare review patterns, room options, and booking conditions.
- Google Reviews: identify recurring location, service, noise, and cleanliness patterns.
- Reddit: identify practical foreign-traveler concerns and repeated warnings.
- Public Travel Blogs: use for practical route and neighborhood context, not final scoring alone.

## 9. Scoring Workflow

Hotel Research

↓

Verification

↓

Scoring

↓

Review Summary

↓

Decision Engine

↓

Stay Guide

Workflow rules:

1 : Research the hotel identity and location.
2 : Verify transportation, walking route, and suitcase access.
3 : Review multiple sources for recurring patterns.
4 : Assign category scores using this document.
5 : Write review summaries without copying reviews.
6 : Connect the hotel to `Best For` and `Not Ideal For` rules.
7 : Send the result to the Decision Engine.
8 : Show the result in Stay Guide with clear recommendation reasons.

## 10. Future Expansion

Future expansion may include:

- Hotel Database
- Review Tag Database
- User Feedback Database
- AI Review Summary
- Affiliate Integration

Expansion rules:

- Review tags should remain traceable to summarized patterns.
- User feedback should be used to improve scoring, not override safety or fit.
- AI review summaries must not copy review text.
- Affiliate integration must not override recommendation logic.
