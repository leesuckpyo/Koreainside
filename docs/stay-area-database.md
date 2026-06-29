# Stay Area Database

## Document Purpose

This document is a draft area database for the Korea Inside Stay Guide recommendation engine.

It defines baseline scoring for major Seoul stay areas so the Stay Guide can match user travel purpose and priorities to suitable areas.

This is not official statistical data.

This is a Korea Inside editorial scoring draft for product planning and recommendation logic.

Actual hotel recommendations require additional verification before publication, including hotel location, current reviews, access conditions, price range, and booking availability.

## Scoring Criteria

Scores use a 1 to 5 scale.

| Score | Meaning |
|---|---|
| 1 | Weak |
| 2 | Below Average |
| 3 | Average |
| 4 | Good |
| 5 | Strong |

## Evaluation Items

Each area is evaluated using the following items:

- Airport Access
- Shopping
- Food
- Cafes
- Nightlife
- Traditional Culture
- Quiet Stay
- Family Friendly
- Luxury
- Budget
- Large Suitcases
- Subway Access

## 1. Hongdae

### Area Summary

Hongdae is a strong choice for travelers who want cafes, food, nightlife, youth culture, and airport rail access. It works well for solo travelers, couples, and first-time visitors who prefer an active area.

### Score Table

| Item | Score |
|---|---:|
| Airport Access | 5 |
| Shopping | 4 |
| Food | 5 |
| Cafes | 5 |
| Nightlife | 5 |
| Traditional Culture | 2 |
| Quiet Stay | 2 |
| Family Friendly | 3 |
| Luxury | 2 |
| Budget | 4 |
| Large Suitcases | 4 |
| Subway Access | 5 |

### Best For

- Solo travelers
- Couples
- First-time visitors who want an active base
- Cafe and nightlife travelers
- Travelers using AREX

### Not Ideal For

- Quiet stays
- Luxury-focused travelers
- Families who want calm streets at night

### Recommendation Notes

Hongdae should score strongly when users select Airport Access, Food, Cafes, Nightlife, Budget, or Subway Access.

It should lose points when Quiet Stay or Luxury is a high priority.

### Suggested Alternative Areas

- Mapo / Gongdeok
- Myeongdong
- Seongsu

## 2. Myeongdong

### Area Summary

Myeongdong is a central and practical stay area for first-time visitors. It is strong for shopping, food, sightseeing access, and hotel choice.

### Score Table

| Item | Score |
|---|---:|
| Airport Access | 4 |
| Shopping | 5 |
| Food | 5 |
| Cafes | 4 |
| Nightlife | 3 |
| Traditional Culture | 3 |
| Quiet Stay | 2 |
| Family Friendly | 4 |
| Luxury | 4 |
| Budget | 3 |
| Large Suitcases | 3 |
| Subway Access | 5 |

### Best For

- First-time visitors
- Shopping-focused travelers
- Travelers who want central access
- Families who want many hotel options

### Not Ideal For

- Travelers who want a quiet local atmosphere
- Travelers who strongly dislike crowded areas
- Visitors seeking a nightlife-focused base

### Recommendation Notes

Myeongdong should score strongly for First Time, Shopping, Food, Subway Access, and central sightseeing convenience.

It should be treated carefully for Quiet Stay and Large Suitcases because street crowding and hotel location details can vary.

### Suggested Alternative Areas

- Seoul Station
- Insadong
- Dongdaemun

## 3. Gangnam

### Area Summary

Gangnam is suitable for business, shopping, luxury hotels, clinics, dining, and nightlife. It is less ideal for users who want traditional culture or the simplest airport rail access.

### Score Table

| Item | Score |
|---|---:|
| Airport Access | 3 |
| Shopping | 5 |
| Food | 5 |
| Cafes | 4 |
| Nightlife | 4 |
| Traditional Culture | 1 |
| Quiet Stay | 3 |
| Family Friendly | 3 |
| Luxury | 5 |
| Budget | 2 |
| Large Suitcases | 3 |
| Subway Access | 5 |

### Best For

- Business travelers
- Luxury travelers
- Shopping travelers
- Nightlife and dining travelers

### Not Ideal For

- Traditional culture trips
- Budget-first travelers
- Travelers who want the easiest airport rail route

### Recommendation Notes

Gangnam should score strongly when users select Luxury, Shopping, Food, Nightlife, or Business.

It should lose points when Budget, Traditional Culture, or Airport Access is the strongest priority.

### Suggested Alternative Areas

- Jamsil
- Myeongdong
- Seongsu

## 4. Insadong

### Area Summary

Insadong is a strong fit for traditional culture, quieter central stays, palaces, galleries, tea houses, and walkable cultural routes.

### Score Table

| Item | Score |
|---|---:|
| Airport Access | 3 |
| Shopping | 3 |
| Food | 4 |
| Cafes | 4 |
| Nightlife | 2 |
| Traditional Culture | 5 |
| Quiet Stay | 4 |
| Family Friendly | 4 |
| Luxury | 3 |
| Budget | 3 |
| Large Suitcases | 3 |
| Subway Access | 4 |

### Best For

- Traditional culture travelers
- Couples
- Families who prefer calmer streets
- First-time visitors focused on palaces and old Seoul

### Not Ideal For

- Nightlife-focused travelers
- Heavy shopping trips
- Travelers who want direct airport rail access

### Recommendation Notes

Insadong should score strongly for Traditional Culture, Quiet Stay, Family Friendly, Food, and Cafes.

It should not be the top result for Nightlife or Shopping-heavy users.

### Suggested Alternative Areas

- Myeongdong
- Seoul Station
- Dongdaemun

## 5. Seoul Station

### Area Summary

Seoul Station is useful for airport access, rail transfers, and travelers who prioritize transportation efficiency. It is practical but less atmospheric than culture, nightlife, or cafe-focused areas.

### Score Table

| Item | Score |
|---|---:|
| Airport Access | 5 |
| Shopping | 3 |
| Food | 3 |
| Cafes | 3 |
| Nightlife | 2 |
| Traditional Culture | 2 |
| Quiet Stay | 3 |
| Family Friendly | 3 |
| Luxury | 3 |
| Budget | 3 |
| Large Suitcases | 4 |
| Subway Access | 5 |

### Best For

- Airport access priority
- Rail transfer trips
- Short stays
- Travelers with large suitcases

### Not Ideal For

- Nightlife travelers
- Cafe-focused travelers
- Visitors who want a strong neighborhood atmosphere

### Recommendation Notes

Seoul Station should score strongly when Airport Access, Large Suitcases, Subway Access, or intercity rail transfer is important.

It should not dominate recommendations for food, nightlife, cafes, or traditional culture.

### Suggested Alternative Areas

- Myeongdong
- Mapo / Gongdeok
- Hongdae

## 6. Dongdaemun

### Area Summary

Dongdaemun is suitable for shopping, late-night retail, budget stays, food, and access to central Seoul. It can be busy and may not fit travelers who want quiet or luxury.

### Score Table

| Item | Score |
|---|---:|
| Airport Access | 3 |
| Shopping | 5 |
| Food | 4 |
| Cafes | 3 |
| Nightlife | 3 |
| Traditional Culture | 3 |
| Quiet Stay | 2 |
| Family Friendly | 3 |
| Luxury | 2 |
| Budget | 4 |
| Large Suitcases | 3 |
| Subway Access | 5 |

### Best For

- Shopping travelers
- Budget travelers
- Late-night retail plans
- Repeat visitors who know Seoul transit

### Not Ideal For

- Quiet stays
- Luxury-focused travelers
- Users who want the simplest airport access

### Recommendation Notes

Dongdaemun should score strongly for Shopping, Budget, Subway Access, and late-night shopping behavior.

It should lose points for Quiet Stay and Luxury.

### Suggested Alternative Areas

- Myeongdong
- Seoul Station
- Insadong

## 7. Jamsil

### Area Summary

Jamsil is useful for families, theme park visits, shopping malls, lake walks, and comfortable modern stays. It is farther from Incheon Airport than several central or western Seoul areas.

### Score Table

| Item | Score |
|---|---:|
| Airport Access | 2 |
| Shopping | 5 |
| Food | 4 |
| Cafes | 4 |
| Nightlife | 3 |
| Traditional Culture | 1 |
| Quiet Stay | 4 |
| Family Friendly | 5 |
| Luxury | 4 |
| Budget | 2 |
| Large Suitcases | 4 |
| Subway Access | 4 |

### Best For

- Families
- Lotte World and mall-focused trips
- Comfortable modern stays
- Travelers who do not mind longer airport travel

### Not Ideal For

- Airport access priority
- Budget-first travelers
- Traditional culture trips

### Recommendation Notes

Jamsil should score strongly for Family Friendly, Shopping, Quiet Stay, and Luxury.

It should lose points when Airport Access or Budget is the strongest priority.

### Suggested Alternative Areas

- Gangnam
- Myeongdong
- Seongsu

## 8. Seongsu

### Area Summary

Seongsu is strong for cafes, design shops, restaurants, and a trend-focused local atmosphere. It is better for repeat visitors than first-time travelers who need the easiest sightseeing base.

### Score Table

| Item | Score |
|---|---:|
| Airport Access | 2 |
| Shopping | 4 |
| Food | 4 |
| Cafes | 5 |
| Nightlife | 3 |
| Traditional Culture | 2 |
| Quiet Stay | 3 |
| Family Friendly | 3 |
| Luxury | 3 |
| Budget | 3 |
| Large Suitcases | 2 |
| Subway Access | 4 |

### Best For

- Cafe-focused travelers
- Repeat visitors
- Couples
- Design and trend-focused trips

### Not Ideal For

- First-time visitors who want the easiest base
- Travelers with large suitcases
- Airport access priority

### Recommendation Notes

Seongsu should score strongly for Cafes, Food, Shopping, and local atmosphere.

It should lose points for Airport Access and Large Suitcases.

### Suggested Alternative Areas

- Hongdae
- Gangnam
- Jamsil

## 9. Itaewon

### Area Summary

Itaewon is useful for international food, nightlife, bars, and a foreigner-friendly atmosphere. It is less ideal for traditional culture, quiet stays, or users with large suitcases.

### Score Table

| Item | Score |
|---|---:|
| Airport Access | 2 |
| Shopping | 3 |
| Food | 5 |
| Cafes | 4 |
| Nightlife | 5 |
| Traditional Culture | 2 |
| Quiet Stay | 2 |
| Family Friendly | 2 |
| Luxury | 3 |
| Budget | 3 |
| Large Suitcases | 2 |
| Subway Access | 3 |

### Best For

- Nightlife travelers
- International food travelers
- Solo travelers
- Repeat visitors

### Not Ideal For

- Families who want calm streets
- Large suitcase travelers
- Traditional culture trips
- Airport access priority

### Recommendation Notes

Itaewon should score strongly for Food and Nightlife.

It should lose points for Quiet Stay, Family Friendly, Large Suitcases, and Airport Access.

### Suggested Alternative Areas

- Hongdae
- Gangnam
- Myeongdong

## 10. Mapo / Gongdeok

### Area Summary

Mapo and Gongdeok are practical stay areas for airport access, food, business convenience, and quieter access to western and central Seoul. They are less iconic than Hongdae or Myeongdong but can be efficient.

### Score Table

| Item | Score |
|---|---:|
| Airport Access | 5 |
| Shopping | 3 |
| Food | 4 |
| Cafes | 4 |
| Nightlife | 3 |
| Traditional Culture | 2 |
| Quiet Stay | 4 |
| Family Friendly | 4 |
| Luxury | 3 |
| Budget | 3 |
| Large Suitcases | 4 |
| Subway Access | 5 |

### Best For

- Airport access priority
- Business travelers
- Families who want a calmer base
- Travelers with suitcases

### Not Ideal For

- Heavy shopping trips
- Traditional culture as the main goal
- Travelers who want an iconic tourist base

### Recommendation Notes

Mapo / Gongdeok should score strongly for Airport Access, Subway Access, Quiet Stay, Family Friendly, and Large Suitcases.

It can be a strong alternative when Hongdae is too busy or Seoul Station feels too transit-focused.

### Suggested Alternative Areas

- Hongdae
- Seoul Station
- Myeongdong

## Draft Use Notes

This database is intended for early scoring logic only.

Before using this data for live hotel recommendations, Korea Inside should verify:

- hotel-level location details
- nearest station and exit
- airport bus stop availability
- walking distance with suitcases
- current review patterns
- night noise
- room size and family suitability
- current price and availability

The scoring should be reviewed after user feedback and real recommendation test cases.
