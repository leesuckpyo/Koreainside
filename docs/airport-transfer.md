# Airport Transfer Page Standard

## Purpose

Airport Transfer is a decision-support page for helping visitors choose the most suitable way to travel from the airport to their hotel or destination.

The page does not replace detailed navigation apps. Korea Inside helps users:

- choose the best transfer option
- understand why that option is recommended
- continue detailed route guidance in Naver Map

The core user question is:

> How do I get from the airport to my hotel?

Airport Transfer should answer this question quickly, clearly, and practically.

## User Input

The recommendation flow requires the following inputs.

### Destination

Users enter either:

- hotel name
- area name

Example areas:

- Hongdae
- Myeongdong
- Gangnam
- Insadong
- Seoul Station
- Dongdaemun

### Arrival Terminal

Users select one arrival terminal:

- ICN Terminal 1
- ICN Terminal 2

### Arrival Time

Users enter arrival time in 24-hour format.

Examples:

- 14:30
- 22:10
- 01:20

### Number of Travelers

Users select:

- 1
- 2
- 3
- 4+

### Number of Suitcases

Users select:

- None
- 1
- 2
- 3
- 4+

## Recommendation Engine

The recommendation engine uses the user's situation to decide the most suitable airport transfer option.

Decision factors include:

- destination area or hotel location
- arrival terminal
- arrival time
- number of travelers
- number of suitcases
- walking burden after arrival
- late-night availability
- group size
- family suitability
- cost sensitivity
- convenience needs

Supported transfer options:

- AREX
- Airport Bus
- Night Bus
- Regular Taxi
- Large Taxi
- Airport Pickup

Airport Transfer should focus on helping users decide what to take. Transport explanations should support the decision, not dominate the page.

Recommended content balance:

- 80% decision support
- 20% transport explanation

## Output Result

The page should return a clear recommended route, recommendation reasons, alternatives, and next action.

### Recommended Route

Example:

1 : AREX All Stop
2 : Hongik University Station
3 : Exit 1
4 : 320m Walk
5 : ABC Hotel

### Recommendation Reason

Reasons should be short and scannable.

Examples:

- Fastest
- No Transfer
- Suitable for Large Suitcase
- Lowest Cost
- Best for Late-night Arrival
- Best for Families

### Alternative Routes

Alternative routes are possible options that are not the primary recommendation.

Examples:

- Airport Bus
- Taxi
- Airport Pickup

### Open in Naver Map

The page should provide an Open in Naver Map action so users can continue with detailed route guidance.

Korea Inside recommends the transfer method. Naver Map provides the live route guidance.

### Airport Pickup

Airport Pickup may be recommended when the user's situation makes public transportation or taxi use difficult.

Typical pickup recommendation cases:

- family with children
- large luggage
- late-night arrival
- large group
- hotel with difficult access
- heavy rain or difficult walking conditions

## Transport Options

Transport descriptions should remain short and practical.

### AREX

- Fast
- Comfortable
- Best for Hongdae and Seoul Station

### Airport Bus

- Good for hotels
- Easier with luggage
- Reduces transfer burden

### Night Bus

- Useful after midnight
- Availability depends on route and time

### Taxi

- Convenient
- More expensive than public transportation
- Suitability depends on luggage and group size

### Airport Pickup

- May fit families
- May fit large luggage
- May fit late-night arrivals
- May help when the hotel is difficult to reach

## Real Travel Problems

Airport Transfer should address real travel problems that affect the transfer decision.

Examples:

- Late-night arrival
- Missed last train
- Too many suitcases
- Family with children
- Large group
- Early morning flight
- Hotel on a hill
- Long walking distance
- No English
- Heavy rain

Each problem should connect to a practical recommendation.

## FAQ

### Should I take AREX or Airport Bus?

AREX is usually better when speed and rail access are important. Airport Bus is often better when the hotel is close to a bus stop or the user wants to avoid luggage transfers.

### Can I use a taxi with four large suitcases?

A regular taxi may not be suitable for four travelers with large suitcases. A large taxi or Airport Pickup may be more practical.

### Is the Night Bus safe?

Night Bus can be a practical late-night option, but users should check route availability and stop location before relying on it.

### How much does a taxi cost?

Taxi cost depends on destination, time, traffic, taxi type, and surcharges. The page should avoid fixed fare claims unless they are reviewed against an official or reliable source.

### Do I need Airport Pickup?

Airport Pickup may be useful for families, large groups, late-night arrivals, heavy luggage, or hotels that are difficult to reach by public transportation.

### Which option is best for families?

Families should prioritize low walking burden, luggage convenience, and fewer transfers. Airport Bus, large taxi, or Airport Pickup may be better than rail-only routes depending on the destination.

