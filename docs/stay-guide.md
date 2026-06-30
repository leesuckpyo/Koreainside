# Stay Guide

## 1. Page Purpose

Stay Guide is a decision-support page for choosing where to stay in Korea.

It is not a hotel introduction page.

The page helps users choose the most suitable area and hotel based on their travel style, trip purpose, and priorities.

Korea Inside should help users:

- choose the best area
- understand why that area fits their trip
- compare hotel suitability
- continue booking through Booking or Agoda

## 2. User Goal

The main user question is:

> Where should I stay in Korea for my trip?

Stay Guide should answer this question by recommending:

- the best area
- the reason for the recommendation
- suitable hotels
- alternative areas

The page should support decision-making first. Hotel descriptions should support the decision, not dominate the page.

## 3. User Input

The recommendation flow uses the user's travel purpose and priority.

### Travel Purpose

Users select one primary travel purpose:

- First Time
- Family
- Couple
- Solo
- Business

### Priority

Users select one or more priorities:

- Airport Access
- Shopping
- Food
- Cafes
- Nightlife
- Traditional Culture
- Quiet Stay
- Luxury
- Budget
- Large Suitcases

## 4. Recommendation Engine

The recommendation engine uses the user's input to recommend an area and hotels.

Flow:

1 : Input values
2 : Recommended Area
3 : Recommended Hotels
4 : Booking / Agoda

Decision factors include:

- travel purpose
- preferred activities
- airport access
- suitcase convenience
- hotel density
- nearby subway access
- food and cafe access
- nightlife preference
- quietness
- budget level
- luxury preference
- family suitability
- business suitability

The recommendation should explain why an area is suitable, not just list hotels.

## 5. Output Result

The page should return a clear decision result.

### Recommended Area

The primary area recommended for the user's situation.

Example:

- Myeongdong
- Hongdae
- Gangnam
- Insadong
- Seoul Station
- Dongdaemun

### Recommendation Reason

Reasons should be short and scannable.

Examples:

- Easy airport access
- Good for first-time visitors
- Many hotels and restaurants
- Better for shopping
- Better for nightlife
- Quiet and traditional atmosphere
- Suitable with large suitcases

### Recommended Hotels

Recommended hotels should be selected based on area fit and user priority.

Each hotel card or listing should include:

- hotel name
- area
- best-for label
- key strengths
- booking action

Booking links should use Booking or Agoda after approval.

### Alternative Areas

Alternative areas should be shown when another area could also work.

Examples:

- If Myeongdong is recommended, alternatives may include Seoul Station or Insadong.
- If Hongdae is recommended, alternatives may include Mapo or Sinchon.
- If Gangnam is recommended, alternatives may include COEX or Yeoksam.

Alternative areas should include a short reason.

## 6. Hotel Evaluation

Each hotel or hotel group should be evaluated with practical travel criteria.

### Evaluation Criteria

- Airport Access
- Suitcase Friendly
- Family Friendly
- Shopping
- Food
- Nightlife
- Quiet
- Luxury
- Best For
- Not Ideal For

### Evaluation Notes

Airport Access should consider:

- airport bus access
- AREX or subway access
- transfer burden
- late-night arrival difficulty

Suitcase Friendly should consider:

- walking distance
- stairs
- station exit convenience
- hotel entrance access

Family Friendly should consider:

- room size
- quietness
- transportation convenience
- stroller or luggage burden

Best For should clearly describe who should choose the hotel or area.

Not Ideal For should clearly describe who may want to avoid it.

## 7. FAQ

### Which area is best for first-time visitors?

Myeongdong is often suitable for first-time visitors because it has many hotels, food options, shopping, and easy access to major areas.

### Which area is best for airport access?

Seoul Station and Hongdae can be convenient because of rail access. Some hotels in Myeongdong or Gangnam may be better if they are close to an airport bus stop.

### Which area is best for families?

Families should prioritize fewer transfers, short walking distance, quiet surroundings, and suitcase-friendly access. Airport bus access can be more important than nightlife or shopping.

### Which area is best for nightlife?

Hongdae and parts of Gangnam are usually better for nightlife. Users who want a quiet stay may prefer another area.

### Which area is best for shopping?

Myeongdong, Dongdaemun, and Gangnam can be good for shopping, depending on the user's shopping style and budget.

### Should I choose a hotel near a subway station?

Yes, but subway distance alone is not enough. Users with large suitcases should also check stairs, exits, walking distance, and airport access.

### Should I book through Booking or Agoda?

Booking and Agoda may both be useful. The final integration should compare availability, price, cancellation policy, and affiliate rules after approval.

## 8. Future Development Plan

Stay Guide should evolve into a recommendation engine supported by structured hotel and review data.

Planned data expansion:

- Hotel Database
- Review Analysis
- Reddit Analysis
- Google Review Summary
- Booking Integration
- Agoda Integration

Planned feature flow:

1 : User selects travel purpose and priority
2 : System recommends area
3 : System recommends hotels
4 : User compares hotel evaluation criteria
5 : User opens Booking or Agoda

Future implementation requires approval before adding:

- hotel affiliate links
- booking integrations
- Agoda integrations
- review scraping or automated review summaries
- new navigation links
- common CSS changes
- URL changes

## 9. Stay Cluster Page: Budget Travelers

The Budget Travelers stay cluster page should help users choose a budget-friendly Seoul area without treating the cheapest room as the best answer.

Primary page:

- `best-area-for-budget-travelers-seoul.html`

Purpose:

- Compare budget areas by total trip value.
- Explain accommodation price ranges as planning ranges, not fixed hotel prices.
- Highlight hidden costs such as distance from subway exits, no elevator, hills, shared bathrooms, luggage storage, late-night taxis, non-refundable rates, and weekend price jumps.
- Keep the page decision-focused rather than hotel-ranking-focused.

Recommended structure:

1 : Hero
2 : What budget means in Seoul
3 : Budget area comparison
4 : Traveler-type decision cards
5 : Hidden costs
6 : Average budget per night
7 : Money-saving tips
8 : Common mistakes
9 : FAQ
10 : Related guides

Decision rules:

- Do not claim one area is universally best.
- Use neutral price levels and price ranges.
- Mention that prices vary by season, room type, location, booking timing, cancellation policy, holidays, and special events.
- Prioritize subway access, airport access, luggage route, bathroom type, elevator availability, and late-night movement over headline price.
