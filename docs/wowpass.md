# WOWPASS Page Planning

Last updated: 2026-07-02

## Purpose

This document records the revised UX and content strategy for `wowpass.html`.

WOWPASS should not be explained as a product only. The page should explain WOWPASS as one money-management option for travelers in Korea: useful for some visitors, unnecessary for others, and best understood through real Korean money, real kiosks, real cards, and short decision guidance.

This is documentation only. This task does not modify `wowpass.html`, `style.css`, `common.js`, routes, navigation, or image assets.

## Current Page Direction

Current `wowpass.html` already has useful content that should be preserved:

- SEO metadata, canonical URL, title, and FAQ JSON-LD.
- Existing FAQ section.
- Existing images for WOWPASS card, WOWPASS machine, T-money card, and WOWPASS use flow.
- Practical warnings that availability and supported services can change.
- Related links to T-money, arrival, payments, maps, and eSIM guides.

Future updates should improve and reorder the existing content rather than removing sections or reducing useful information.

## Revised UX Principle

The page should create an "아~ 이거구나" moment before it asks users to compare features.

Preferred user flow:

1. Recognize Korean money.
2. Understand approximate value and exchange feeling.
3. Explain that many travelers can use their home credit/debit cards in Korea.
4. Explain why some travelers still choose WOWPASS.
5. Show how WOWPASS works with images.
6. Give short guidance on who may need it and who may not.
7. Keep FAQ for detailed questions.

Avoid making the page feel like a broad `WOWPASS vs T-money` battle. T-money comparison is still useful, but it should support a decision instead of defining the entire page.

## Korean Money Concept

The WOWPASS page should introduce Korean money before explaining card charging and spending. This helps users understand why KRW numbers look large and what a top-up amount roughly means.

Use actual or official/reference banknote images if available. If banknote images are not available during implementation, use a clear placeholder with descriptive alt text and a TODO note until proper assets are added.

Current Korean banknotes to show:

| Banknote | Recognition purpose | Approx. USD | Approx. EUR | Approx. GBP | Approx. AUD | Approx. JPY |
|---|---|---:|---:|---:|---:|---:|
| KRW 1,000 | Smallest common banknote; helps users see that Korean prices often use thousands. | USD 0.64 | EUR 0.57 | GBP 0.48 | AUD 0.93 | JPY 105 |
| KRW 5,000 | Mid-small note used for everyday small payments. | USD 3.22 | EUR 2.83 | GBP 2.42 | AUD 4.67 | JPY 523 |
| KRW 10,000 | Common note that helps users estimate daily spending and top-ups. | USD 6.43 | EUR 5.65 | GBP 4.84 | AUD 9.34 | JPY 1,045 |
| KRW 50,000 | Largest common banknote; useful for explaining cash exchange and prepaid charging. | USD 32.17 | EUR 28.26 | GBP 24.21 | AUD 46.72 | JPY 5,225 |

Exchange-rate notes:

- Values above are approximate orientation values, not live exchange quotes.
- Last reviewed: 2026-07-02.
- Rates were checked against Wise mid-market KRW converter pages for USD, EUR, GBP, AUD, and JPY.
- Exchange rates change often. Before publishing the values in HTML, refresh the table, update the reviewed date, and avoid implying that WOWPASS top-up rates equal the table values.
- The money concept should help users understand scale, not promise savings.

Official money source:

- Bank of Korea, "Introduction to Banknotes", confirms current KRW 1,000, KRW 5,000, KRW 10,000, and KRW 50,000 banknotes: https://www.bok.or.kr/eng/main/contents.do?menuNo=400112

## Recommended Section Flow

| Order | Section intent | Image role | Content guidance |
|---|---|---|---|
| 1 | Korean money recognition | Recognition + value image | Show KRW banknotes first. Explain that Korean prices often look large because prices are written in won. |
| 2 | Approximate value feeling | Value image | Give rounded USD/EUR/GBP/AUD/JPY values and a visible exchange-rate-change note. |
| 3 | Home cards still work for many travelers | Decision guidance | Explain that many visitors can use home credit/debit cards in Korea, but card acceptance, foreign transaction fees, issuer blocks, and exchange rates depend on the card. |
| 4 | Why some travelers choose WOWPASS | Recognition + decision image | Present WOWPASS as a prepaid budgeting and spending option, not a required item. |
| 5 | How WOWPASS works | Usage image | Show card, kiosk, app, and payment flow with short text. Prefer image, short meaning, practical explanation, next action. |
| 6 | Who may need it / who may not | Decision cards or compact table | Keep this short. The goal is decision clarity, not a long comparison page. |
| 7 | FAQ | Existing FAQ component | Preserve FAQ and keep visible FAQ text aligned with FAQ JSON-LD. |

## WOWPASS Positioning

Core explanation:

WOWPASS is a prepaid money-management option for visitors who want to load KRW, track spending, use a card-like payment method, and combine this with a T-money transportation function.

Do not present WOWPASS as automatically better than a foreign credit card, debit card, cash, or T-money. The best option depends on traveler needs.

Useful reasons to mention:

- Exchange or top-up convenience when official current conditions support the claim.
- Budget control through prepaid spending.
- T-money function for public transportation.
- One-card simplicity for travelers who prefer not to manage cash, a separate transport card, and foreign cards separately.

Limitations to mention:

- Some travelers can simply use their home credit/debit card plus T-money.
- Supported currencies, machines, fees, locations, limits, refunds, and top-up methods can change.
- Users should confirm current official WOWPASS terms before relying on a specific feature.
- Transportation balance may need to be managed separately from prepaid payment balance if current WOWPASS rules require it.

## Who May Need WOWPASS

WOWPASS may fit travelers who:

- Want prepaid budget control in KRW.
- Prefer a traveler-focused card and app instead of relying only on a foreign card.
- Plan to use both payments and public transportation.
- Want a kiosk/app-based way to manage Korean spending.
- Are uncomfortable estimating Korean money values on the spot.

WOWPASS may not fit travelers who:

- Are comfortable using a home credit/debit card in Korea.
- Want only subway and bus access, where T-money may be simpler.
- Are staying briefly and expect very limited spending.
- Do not want to manage another card, app, kiosk process, or refund step.
- Need guaranteed acceptance everywhere, which should never be promised.

## Image Requirements

Use actual, official, or clearly referenced images for:

- WOWPASS card.
- WOWPASS kiosk or machine.
- Korean banknotes.
- WOWPASS app screens.
- T-money card.

Existing relevant assets:

- `images/wowpass/wowpass-card.png`
- `images/wowpass/wowpass-machine.png`
- `images/wowpass/wowpass-guide.png`
- `images/wowpass/wowpass-use-flow.png`
- `images/wowpass/tmoney-card.png`

Missing or not yet confirmed assets:

- Korean banknote images.
- WOWPASS app screen images.

If missing assets block a future HTML update, use clear placeholders only with visible TODO notes and descriptive alt text. Do not use AI images for cards, kiosks, banknotes, app screens, or official payment interfaces.

## Animation Principle

Animations are not tutorials. If animation is added later, it should be short, looping, lightweight, and focused on recognition.

Good animation intent:

- "Ah, this is how I tap the card."
- "Ah, this is how the kiosk/card/app relationship works."
- "Ah, I load money first, then spend from the balance."

Avoid:

- Long step-by-step manuals.
- Heavy JavaScript.
- Decorative motion that does not help a traveler understand what they will see or do.
- Animation that contains essential text not repeated in visible HTML.

## SEO and Content Notes

Future HTML work should preserve:

- Existing URL: `wowpass.html`.
- Existing canonical URL unless the site URL changes by approved project strategy.
- Existing useful FAQ content.
- Visible, semantic HTML for every important instruction, comparison, warning, and value.

Potential future title direction, only after approval:

- Shift from a pure `WOWPASS vs T-money` title toward a money-management frame, while still covering T-money comparison where it helps search intent.

No metadata should be changed without reviewing SEO impact and showing the complete diff first.

## Source Notes

Sources to verify before future HTML implementation:

- Bank of Korea banknotes: https://www.bok.or.kr/eng/main/contents.do?menuNo=400112
- Official WOWPASS website for current card features, supported top-up/exchange methods, fees, limits, machine locations, app features, and refund rules: https://www.wowpass.io/
- Official T-money website for transport-card behavior and compatibility notes: https://eng.tmoney.co.kr/en/aeb/main/main/readMain.dev
- Wise KRW to USD reference: https://wise.com/gb/currency-converter/krw-to-usd-rate
- Wise KRW to EUR reference: https://wise.com/gb/currency-converter/krw-to-eur-rate
- Wise KRW to GBP reference: https://wise.com/gb/currency-converter/krw-to-gbp-rate
- Wise KRW to AUD reference: https://wise.com/gb/currency-converter/krw-to-aud-rate
- Wise KRW to JPY reference: https://wise.com/gb/currency-converter/krw-to-jpy-rate

## Quality Notes

Estimated page strategy quality before this documentation update: 72/100.

Estimated page strategy quality after this documentation update: 90/100.

Remaining gap: `wowpass.html` itself still uses a VS-first structure. A future approved HTML task can update the page order and add a Korean money recognition section without removing existing useful sections.
