# Home Hero

Last updated: July 2, 2026

## Purpose

The home hero positions KR Inside as a practical decision-support service for visitors preparing for Korea.

It should make the service clear within a few seconds on mobile:

- What KR Inside is.
- What problems it helps solve.
- Where a first-time visitor should start.

## Approved Hero Message

Use this `h1` structure on the home page:

```text
KR Inside
Everything You Need
Before Visiting Korea
```

The hero brand text is HTML text. Do not place the approved message inside an image.

The `KR Inside` brand text may use inline HTML spans for visual emphasis: `K` in red, `R` in blue, and `Inside` in navy. The letters must remain real HTML text inside the single home-page `h1`.

## Supporting Message Direction

The supporting paragraph should explain that KR Inside focuses on practical pre-trip information:

- Airport arrival.
- Transportation cards.
- eSIM and mobile connectivity.
- Payments and WOWPASS.
- Korea map app usage.

Avoid positioning the home page as a tourism destination list. The promise is problem solving before and during travel.

## Primary CTAs

Use two hero CTA links:

| CTA | Target |
| --- | --- |
| Start with Arrival | `arrival.html` |
| Check eSIM Guide | `esim.html` |

## Hero Entry Cards

The home hero includes four fast-entry cards:

| Card | Target | Role |
| --- | --- | --- |
| Arrival | `arrival.html` | Airport steps, transport choices and first-hour setup. |
| eSIM | `esim.html` | Mobile data setup before landing or at the airport. |
| T-money | `tmoney.html` | Subway and bus card basics. |
| WOWPASS | `wowpass.html` | Prepaid card and visitor payment setup. |

## Design Requirements

- Mobile-first layout.
- Important text remains visible semantic HTML.
- Navy base with teal accents.
- Existing hero image may remain as support, but the message must not depend on the image.
- Avoid adding new heavy visual assets unless they improve user decisions.
- Keep the hero compact enough that desktop screens do not feel empty.
- If cinematic motion is used, animate only the hero image with a subtle one-time CSS transform and disable it with `prefers-reduced-motion`.

## SEO Requirements

- The home hero message is the only `h1` on `index.html`.
- Supporting copy uses `p` text.
- Entry cards use link text and visible descriptions.
- Do not add another `h1` in the header, category cards or later sections.

## Responsive Targets

Check the home hero at:

- 360px mobile.
- 768px tablet.
- 1440px desktop.

The hero should keep readable text, visible CTAs and non-overlapping entry cards at each target width.

## Home Trust Section

The trust section appears directly below the hero and before the practical guide cards.

Use this section title:

```text
Why KR Inside exists
```

The section should explain that travelers often struggle with real usage systems, not sightseeing ideas. It should mention that airport arrival, transportation cards, eSIM setup, payments, maps and booking habits can differ by country, and that KR Inside helps foreign visitors make better decisions in Korea.

Use three trust cards:

| Card title | Message |
| --- | --- |
| Practical, not promotional | Provide real usage standards, cautions and choice criteria instead of promotional recommendations. |
| Before you arrive | Organize eSIM, transportation card, payment method and map app information before airport arrival. |
| Better decisions in Korea | Show strengths, limitations and conditions so users can compare options and choose for themselves. |

Design requirements:

- Use text-first HTML, not images.
- Use an `h2` for the section title.
- Use `h3` for card titles.
- Mobile layout is one column.
- Desktop layout is three columns.
- Keep the visual tone calm: light background, white cards, navy headings and teal accents.

## Home Feature Grid

The feature grid appears below the trust section and introduces problem-solving guide categories.

Use this section title:

```text
Essential guides for visiting Korea
```

The supporting copy should explain that KR Inside covers practical information needed before and after visiting Korea, including airport arrival, connectivity, transportation, payments, maps and accommodation decisions.

Use six feature cards:

| Card title | Current target | Message |
| --- | --- | --- |
| Arrival Guide | `airport.html` | Terminal steps, transport choices and first things to prepare after airport arrival. |
| eSIM Guide | `esim.html` | Compare data-only eSIMs, Korean phone numbers and SIM choices before buying. |
| T-money Guide | `tmoney.html` | How to buy, recharge and use Korea's transport card for subways and buses. |
| WOWPASS Guide | `wowpass.html` | Visitor payment card, currency exchange and transport features. |
| Maps Guide | `maps.html` | Why Google Maps can be limited in Korea and which local map apps to use instead. |
| Accommodation Guide | `accommodation.html` | Location, transit access, suitcase movement and practical stay criteria. |

`accommodation.html` currently forwards visitors to the existing Seoul accommodation decision guide. Replace it with a full accommodation hub when the category expands beyond Seoul.

Design requirements:

- Use text-first HTML, not images.
- Use an `h2` for the section title.
- Use `h3` for card titles.
- Each card includes a short label, title, description and clear Learn more text.
- Mobile layout is one column.
- Tablet layout is two columns.
- Desktop layout is three columns.
- Keep card heights visually balanced.
