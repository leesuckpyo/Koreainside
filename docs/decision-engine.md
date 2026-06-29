# Decision Engine

## 1. Decision Engine Purpose

The Korea Inside Decision Engine defines a shared recommendation structure for pages that help users choose the most suitable option.

It is used by decision-support pages such as:

- Airport Transfer
- Stay Guide

The shared structure is:

1 : User condition input
2 : Score calculation
3 : Recommendation result
4 : Recommendation reason

The goal is to make recommendations clear, repeatable, explainable, and useful for foreign visitors in Korea.

## 2. Common Principles

The Decision Engine follows these principles.

### Rule-based, not AI inference

Recommendations should be based on explicit rules and scoring, not unpredictable AI inference.

### Same input, same result

If the user enters the same conditions, the engine should return the same recommendation.

### Always explain the reason

Every recommendation must include a clear reason.

Users should understand why an option was recommended, not only what was recommended.

### User fit before monetization

The most suitable option for the user must come before revenue or affiliate potential.

### Affiliate links only when useful

Affiliate links should appear only when they are relevant to the user's situation.

Examples:

- Airport Pickup should not appear for every user.
- Booking or Agoda links should support the hotel recommendation, not replace the recommendation.

## 3. Airport Transfer Decision Factors

Airport Transfer uses the user's arrival and luggage situation to recommend the best airport transfer option.

Decision factors:

- Arrival Time
- Terminal
- Destination Area
- Travelers
- Suitcases
- Walking Distance
- Public Transport Availability
- Night Bus Availability
- Taxi Suitability
- Pickup Suitability

## 4. Airport Transfer Basic Rule Examples

These rules are examples for the first version of the Airport Transfer recommendation logic.

### Hongdae, daytime, low luggage

Condition:

- Destination Area: Hongdae
- Arrival Time: daytime
- Suitcases: none or low luggage

Recommended Option:

- AREX All Stop

Reason:

- Direct rail access to Hongik University Station
- Good balance of speed and cost
- Suitable when luggage burden is low

### Seoul Station, daytime

Condition:

- Destination Area: Seoul Station
- Arrival Time: daytime

Recommended Option:

- AREX Express

Reason:

- Direct connection to Seoul Station
- Fast airport-to-city access

### Late night with night bus route match

Condition:

- Arrival Time: late night
- Night Bus Availability: route match

Recommended Option:

- Night Bus

Reason:

- Public transport option may still be available
- Lower cost than taxi or pickup

### Four travelers with four large suitcases

Condition:

- Travelers: 4
- Suitcases: 4 large suitcases

Recommended Options:

- Large Taxi
- Airport Pickup
- 2 taxis

Reason:

- Regular taxi capacity may not be enough
- Luggage loading and group comfort are high-priority factors

### Long walking distance with large luggage

Condition:

- Walking Distance: over 700m
- Suitcases: large luggage

Recommended Option:

- Taxi from nearest station

Reason:

- Long walking distance with luggage creates high friction
- Taxi can reduce the final walking burden

## 5. Stay Guide Decision Factors

Stay Guide uses the user's travel style and priorities to recommend the most suitable stay area and hotels.

Decision factors:

- Travel Purpose
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
- Family Friendly

## 6. Stay Guide Basic Rule Examples

These rules are examples for the first version of the Stay Guide recommendation logic.

### First time, shopping, airport access

Condition:

- Travel Purpose: First Time
- Priority: Shopping
- Priority: Airport Access

Recommended Areas:

- Myeongdong
- Hongdae

Reason:

- Good first-trip convenience
- Strong food, shopping, and transit access
- Easier for visitors who do not know Seoul well

### Nightlife and cafes

Condition:

- Priority: Nightlife
- Priority: Cafes

Recommended Area:

- Hongdae

Reason:

- Strong cafe and nightlife environment
- Good for solo travelers, couples, and flexible plans

### Traditional culture and quiet stay

Condition:

- Priority: Traditional Culture
- Priority: Quiet Stay

Recommended Area:

- Insadong

Reason:

- Better access to traditional culture areas
- Calmer atmosphere than nightlife-heavy areas

### Luxury and shopping

Condition:

- Priority: Luxury
- Priority: Shopping

Recommended Areas:

- Gangnam
- Myeongdong

Reason:

- Strong shopping access
- Better match for premium hotel and retail options

### Family with large suitcases

Condition:

- Travel Purpose: Family
- Priority: Large Suitcases

Recommended Preference:

- Airport access and suitcase-friendly hotels preferred

Reason:

- Fewer transfers and shorter walking distance matter more than nightlife
- Hotel entrance, station exit, and bus stop access should be weighted heavily

## 7. Scoring Method

The first scoring model should be simple and explainable.

Example scoring:

- Strong Match: +3
- Match: +2
- Weak Match: +1
- Conflict: -2

Each option receives points based on how well it matches the user's conditions.

The highest score becomes the Recommended Option.

When scores are close, the engine should show Alternative Options.

Conflict scores should be used when an option creates clear user friction.

Examples:

- Large suitcases plus long walking distance: Conflict
- Quiet Stay plus nightlife-heavy area: Conflict
- Family trip plus high night noise: Conflict
- Late-night arrival plus unavailable public transport: Conflict

## 8. Result Output Format

Every recommendation result should use a consistent format.

### Recommended Option

The primary recommended transfer option, area, or hotel.

### Why this is recommended

Short reasons explaining the score.

Examples:

- Strong airport access
- Low walking burden
- Best fit for first-time visitors
- Better for family travelers
- Lower risk after midnight

### Alternative Options

Other possible options that may still work.

Alternatives should explain when they are better.

### Not ideal because

A short warning about fit problems.

Examples:

- Not ideal for large suitcases
- Not ideal after midnight
- Not ideal for quiet stays
- Not ideal for families with small children

### Next Action

The next user action.

Examples:

- Open in Naver Map
- Compare hotels
- Open Booking
- Open Agoda
- Check airport bus stop
- Check walking distance

## 9. Future Expansion

The Decision Engine can expand as Korea Inside adds structured data and integrations.

Planned expansion:

- Hotel database
- Naver Map integration
- Review analysis
- Affiliate rules
- User feedback loop

Future expansion should preserve the core principles:

- rule-based recommendations
- repeatable results
- visible recommendation reasons
- user fit before monetization
- affiliate links only when useful
