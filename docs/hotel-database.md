# Hotel Database

## 1. Document Purpose

This document defines the initial hotel database template for the Korea Inside Stay Guide recommendation engine.

This is not a hotel introduction page.

This is not a finalized hotel ranking.

The purpose of this document is to prepare a structured investigation and evaluation template for each hotel before any real recommendation score is confirmed.

This document is a Korea Inside Editorial Database Draft.

Actual hotel recommendations require additional verification before publication.

## 2. Hotel Database Use Purpose

The Hotel Database will support Stay Guide recommendations by connecting user needs to hotel suitability.

The database should help the recommendation engine evaluate:

- which area fits the user's travel purpose
- which hotel fits the user's priorities
- why a hotel is recommended
- when a hotel is not ideal
- whether Booking or Agoda links are useful for the user

The database must prioritize user fit over affiliate potential.

## 3. Initial Target Hotels

The first research batch includes 20 Seoul hotels.

### Hongdae

- RYSE Hotel
- L7 Hongdae
- Holiday Inn Express Seoul Hongdae

### Myeongdong

- L7 Myeongdong
- Nine Tree Premier Hotel Myeongdong 2
- Sotetsu Fresa Inn Seoul Myeong-dong

### Gangnam

- Grand InterContinental Seoul Parnas
- L7 Gangnam
- Hotel Peyto Samseong

### Insadong

- Orakai Insadong Suites
- NINE TREE BY PARNAS Seoul Insadong
- Amid Hotel Seoul

### Seoul Station

- Four Points by Sheraton Josun Seoul Station
- ENA Suite Hotel Namdaemun

### Jamsil

- Signiel Seoul
- Sofitel Ambassador Seoul

### Seongsu

- Hotel POCO Seongsu
- H Avenue Hotel Seongsu

### Itaewon

- Mondrian Seoul Itaewon
- Imperial Palace Boutique Hotel

## 4. Hotel Evaluation Template

Use this template for each hotel during research.

Do not confirm final scores until the hotel has been checked against location, price, transportation, reviews, and operating status.

```text
Hotel Name:
Area:
Hotel Class:
Luxury Level:

Airport Access:
Suitcase Friendly:
Family Friendly:
Business Friendly:
Night Noise:
Shopping:
Food:
Cafes:
Subway Access:
Walking Difficulty:
Budget Level:

Best For:
Not Ideal For:

Strengths:
Weaknesses:
Korea Inside Notes:
Verification Needed:

Review Summary:
- Review Strengths:
- Review Weaknesses:
- Common Complaints:
- Frequently Praised:
- Korea Inside Notes:

Recommended For:
- First Time:
- Family:
- Couple:
- Solo:
- Business:
- Luxury:
- Budget:
- Food Trip:
- Nightlife:
- Shopping:
- Long Stay:
```

## 5. Evaluation Items

Each hotel should be evaluated with the following fields.

### Area

The stay area where the hotel belongs.

Examples:

- Hongdae
- Myeongdong
- Gangnam
- Insadong
- Seoul Station
- Jamsil
- Seongsu
- Itaewon

### Hotel Class

The general hotel category.

Examples:

- Budget
- Mid-range
- Mid-upscale
- Lifestyle
- Luxury
- Serviced residence
- Business hotel

### Luxury Level

Draft luxury level based on hotel positioning, facilities, room quality, and service expectations.

Final value should be added only after verification.

### Airport Access

How convenient the hotel is from Incheon Airport.

Check:

- AREX access
- airport bus availability
- taxi or pickup practicality
- number of transfers
- late-night arrival difficulty

### Suitcase Friendly

How easy the hotel is for travelers with large luggage.

Check:

- walking distance
- stairs
- station exit convenience
- sidewalk condition
- hotel entrance access
- elevator availability

### Family Friendly

How suitable the hotel is for families.

Check:

- room size
- room types
- quietness
- stroller access
- surrounding streets
- transportation burden

### Business Friendly

How suitable the hotel is for business travelers.

Check:

- business district access
- desk or work suitability
- quietness
- transport reliability
- meeting-area convenience

### Night Noise

Expected night noise risk from nightlife, roads, crowds, or nearby venues.

This field should be verified with recent review patterns.

### Shopping

How useful the hotel location is for shopping-focused travelers.

### Food

How useful the hotel location is for restaurants and practical meal options.

### Cafes

How useful the hotel location is for cafes and relaxed daytime plans.

### Subway Access

How easy it is to reach useful subway lines from the hotel.

Subway distance alone is not enough. Exit convenience and walking difficulty must also be checked.

### Walking Difficulty

How difficult the walking route is from station or airport bus stop to the hotel.

Consider:

- distance
- slopes
- stairs
- crossings
- crowded streets
- luggage burden

### Budget Level

Draft price-position category.

Do not treat this as live pricing.

Examples:

- Budget
- Mid
- Upper-mid
- High
- Luxury

### Best For

The user types or travel styles that fit the hotel best.

### Not Ideal For

The user types or travel styles that may not fit the hotel.

### Strengths

The hotel's main recommendation strengths after verification.

### Weaknesses

The hotel's main recommendation risks after verification.

### Korea Inside Notes

Editorial notes for recommendation logic.

This field should explain how the hotel may be used by the recommendation engine.

### Verification Needed

Required checks before using the hotel in a live recommendation.

## 6. Review Summary Fields

Each hotel should include foreigner-review analysis fields.

Do not copy actual review text.

Use summarized patterns only after review analysis.

### Review Strengths

Positive patterns found in reviews.

### Review Weaknesses

Negative patterns found in reviews.

### Common Complaints

Repeated complaints that may affect recommendation fit.

### Frequently Praised

Repeated praise that may support recommendation fit.

### Korea Inside Notes

Editorial interpretation of review patterns for foreign travelers.

## 7. Recommended For Fields

Each hotel should define whether it fits the following recommendation tags.

Do not mark a hotel as recommended for a tag until verification is complete.

- First Time
- Family
- Couple
- Solo
- Business
- Luxury
- Budget
- Food Trip
- Nightlife
- Shopping
- Long Stay

Recommended For values may use:

- Yes
- Maybe
- No
- Needs Verification

## 8. Verification Needed Fields

Every hotel must include verification requirements before being used in production recommendations.

Minimum checks:

- exact hotel name
- exact address
- current operating status
- nearest subway station
- nearest station exit
- walking distance from station
- walking difficulty with suitcases
- airport bus stop availability
- AREX or subway route difficulty
- taxi and pickup suitability
- recent foreigner review patterns
- room size and room type suitability
- family suitability
- night noise risk
- current price range
- booking availability
- cancellation policy

## 9. Future Research Procedure

Research should happen in small, verifiable steps.

1 : Confirm exact hotel identity

Check the official hotel name, branch, address, and operating status.

2 : Confirm transportation access

Check airport access, subway access, airport bus availability, and taxi practicality.

3 : Check suitcase route

Review walking distance, station exits, stairs, slopes, and hotel entrance access.

4 : Analyze review patterns

Summarize foreigner review patterns without copying review text.

5 : Fill evaluation template

Complete the hotel template with draft values and verification notes.

6 : Connect to Stay Guide tags

Map the hotel to Recommended For tags only when the evidence supports it.

7 : Review before publication

Do not publish a hotel recommendation until location, price, transportation, reviews, and operating status are rechecked.

## 10. Production Use Rule

This document must not be treated as a final recommendation database.

It is a research template and initial target list.

Final hotel recommendations require verified data and editorial review.
