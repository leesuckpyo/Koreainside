# Data Dictionary

## Document Purpose

This document defines every shared field used by the Korea Inside recommendation engine and related databases.

Every field must have one clear meaning so the Stay Guide, Hotel Database, Area Database, Review Summary, and Decision Engine use the same language.

This dictionary supports consistent scoring, repeatable recommendations, and clear recommendation reasons.

## Data Types

| Type | Meaning |
|---|---|
| Text | A plain text value. |
| Integer | A whole number. |
| Boolean | `true` or `false`. |
| Score (1~5) | Editorial score from 1 to 5. |
| Tag | A controlled label used for matching and filtering. |
| Array | A list of values. |

## Score Values

| Score | Meaning |
|---|---|
| 5 | Excellent |
| 4 | Good |
| 3 | Average |
| 2 | Weak |
| 1 | Poor |

## Area Fields

| Field | Type | Required | Definition | Allowed Values / Notes |
|---|---|---:|---|---|
| Area | Text | Yes | The stay area name used for recommendation and display. | Examples: Hongdae, Myeongdong, Gangnam, Insadong, Seoul Station, Dongdaemun, Jamsil, Seongsu, Itaewon, Mapo / Gongdeok. |
| District | Text | Optional | The administrative district or broad location context for the area. | Example: Mapo-gu. Use only when verified. |
| Nearest Station | Text | Optional | The main station used to access the area or hotel. | Use the most practical station for travelers, not only the closest by distance. |
| Airport Access | Score (1~5) | Yes | How convenient the area is from Incheon Airport. | Score by AREX, airport bus, transfers, late-night risk, and luggage burden. |
| Subway Access | Score (1~5) | Yes | How useful and convenient subway access is for travelers staying in the area. | Consider useful lines, transfer convenience, and station density. |
| Walking Difficulty | Score (1~5) | Yes | How easy walking is for travelers. Higher score means easier walking. | Consider distance, slopes, stairs, crossings, crowding, and luggage. |
| Nightlife | Score (1~5) | Yes | Strength of nightlife access and nightlife-focused travel fit. | High score can conflict with Quiet Stay. |
| Shopping | Score (1~5) | Yes | Strength of shopping access. | Consider practical shopping, fashion, malls, markets, and tourist shopping. |
| Food | Score (1~5) | Yes | Strength of restaurant and practical meal access. | Include convenient everyday food and destination dining. |
| Cafes | Score (1~5) | Yes | Strength of cafe access and cafe-focused travel fit. | Include density, variety, and traveler appeal. |
| Luxury | Score (1~5) | Yes | Strength of luxury hotel, shopping, dining, and premium-stay fit. | Area-level luxury, not hotel-specific luxury. |
| Budget | Score (1~5) | Yes | Suitability for cost-sensitive travelers. | Higher score means better budget fit. |
| Quiet Stay | Score (1~5) | Yes | Suitability for travelers who want calm surroundings and lower night noise risk. | High nightlife areas usually score lower. |
| Traditional Culture | Score (1~5) | Yes | Access to traditional culture, palaces, old Seoul, galleries, tea houses, and cultural routes. | Area-level culture fit. |
| Family Friendly | Score (1~5) | Yes | Area suitability for families. | Consider quietness, walking burden, transit, food, safety, and hotel practicality. |

## Hotel Fields

| Field | Type | Required | Definition | Allowed Values / Notes |
|---|---|---:|---|---|
| Hotel Name | Text | Yes | Official or display hotel name used in the database. | Must be verified before production use. |
| Hotel Class | Tag | Yes | General category of the hotel. | Budget, Mid-range, Mid-upscale, Lifestyle, Luxury, Serviced residence, Business hotel. |
| Area | Text | Yes | Stay area where the hotel belongs. | Must match an Area field where possible. |
| Luxury Level | Score (1~5) | Yes | Hotel-level luxury and premium-stay fit. | Use hotel positioning, facilities, room quality, and service expectations. |
| Airport Access | Score (1~5) | Yes | Hotel-specific airport access convenience. | Consider AREX, airport bus, taxi, pickup, transfers, and luggage. |
| Suitcase Friendly | Score (1~5) | Yes | How easy the hotel is with large luggage. | Consider station exit, elevators, distance, entrance, slope, and crowds. |
| Family Friendly | Score (1~5) | Yes | Hotel suitability for families. | Consider room types, quietness, stroller/luggage route, and surrounding streets. |
| Business Friendly | Score (1~5) | Yes | Hotel suitability for business travelers. | Consider desk/work setup, quietness, business district access, and transit reliability. |
| Walking Difficulty | Score (1~5) | Yes | Ease of walking route to the hotel. Higher score means easier walking. | Consider luggage and weather. |
| Noise Level | Score (1~5) | Yes | Night and room-noise suitability. Higher score means lower noise risk. | Use review patterns after verification. |
| Budget Level | Tag | Yes | Price-position category, not live price. | Budget, Mid, Upper-mid, High, Luxury, Verification Needed. |
| Best For | Array | Yes | User types or priorities the hotel fits well. | Use controlled tags from Recommended For. |
| Not Ideal For | Array | Yes | User types or priorities the hotel does not fit well. | Must be specific and practical. |

## Review Fields

| Field | Type | Required | Definition | Allowed Values / Notes |
|---|---|---:|---|---|
| Strengths | Array | Optional | Repeated positive patterns that support hotel fit. | Summarize patterns only. Do not copy reviews. |
| Weaknesses | Array | Optional | Repeated negative patterns that affect hotel fit. | Summarize patterns only. Do not copy reviews. |
| Frequently Praised | Array | Optional | Review themes repeatedly praised across sources. | Must be recurring, not one-off. |
| Common Complaints | Array | Optional | Review themes repeatedly criticized across sources. | Must be recurring and relevant to recommendation. |
| Korea Inside Notes | Text | Optional | Editorial interpretation for foreign visitors. | Explain how review patterns affect user fit. |

## Decision Engine Fields

| Field | Type | Required | Definition | Allowed Values / Notes |
|---|---|---:|---|---|
| Travel Purpose | Tag | Yes | The user's main trip style or purpose. | First Time, Family, Couple, Solo, Business. |
| Priority | Array | Yes | User-selected priorities used for scoring. | Airport Access, Shopping, Food, Cafes, Nightlife, Traditional Culture, Quiet Stay, Luxury, Budget, Large Suitcases. |
| Recommended Area | Text | Yes | Primary area selected by the engine. | Must come from Area values. |
| Recommended Hotel | Text | Optional | Primary hotel selected by the engine. | Required only when hotel recommendation is enabled. |
| Alternative Area | Array | Optional | Other areas that may also fit. | Each alternative should include a short reason. |
| Alternative Hotel | Array | Optional | Other hotels that may also fit. | Each alternative should include a short reason. |
| Recommendation Reason | Array | Yes | Clear explanation of why the option was recommended. | Must be visible to users. |
| Next Action | Tag | Yes | The action the user should take after the recommendation. | Compare hotels, Open Booking, Open Agoda, Check walking distance, Check airport route, Verification Needed. |

## Validation Rules

### Allowed Values

Controlled fields must use approved values.

Travel Purpose:

- First Time
- Family
- Couple
- Solo
- Business

Priority:

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

Recommended For / Best For tags:

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

Score fields:

- 1
- 2
- 3
- 4
- 5

Budget Level:

- Budget
- Mid
- Upper-mid
- High
- Luxury
- Verification Needed

### Required Fields

Area database required fields:

- Area
- Airport Access
- Subway Access
- Walking Difficulty
- Nightlife
- Shopping
- Food
- Cafes
- Luxury
- Budget
- Quiet Stay
- Traditional Culture
- Family Friendly

Hotel database required fields:

- Hotel Name
- Hotel Class
- Area
- Luxury Level
- Airport Access
- Suitcase Friendly
- Family Friendly
- Business Friendly
- Walking Difficulty
- Noise Level
- Budget Level
- Best For
- Not Ideal For

Decision result required fields:

- Travel Purpose
- Priority
- Recommended Area
- Recommendation Reason
- Next Action

### Optional Fields

Optional fields may be blank during early research, but must be marked clearly if not verified.

Optional fields:

- District
- Nearest Station
- Recommended Hotel
- Alternative Area
- Alternative Hotel
- Review Strengths
- Review Weaknesses
- Frequently Praised
- Common Complaints
- Korea Inside Notes

### Future Fields

Future fields may be added only when the data source and use case are clear.

Possible future fields:

- Source URL
- Last Reviewed Date
- Review Tag IDs
- Booking URL
- Agoda URL
- Affiliate Eligible
- User Feedback Score
- AI Review Summary Status
- Data Confidence

Future fields must not change the meaning of existing fields.

## Naming Rules

Use the same field name everywhere.

Do not create synonyms for the same concept.

Examples:

- Use `Suitcase Friendly`, not `Luggage Friendly`.
- Use `Walking Difficulty`, not `Walking Burden`.
- Use `Noise Level`, not `Night Noise`, for hotel-level noise.
- Use `Quiet Stay`, not `Quietness`, for area-level quiet-stay fit.

## Production Rule

If a field is not verified, mark it as `Verification Needed`.

Do not invent values to complete a record.

Do not copy review text into any database field.

Every recommendation must be traceable to defined fields and visible recommendation reasons.
