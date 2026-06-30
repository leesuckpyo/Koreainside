# Korea Inside Intelligence

## Purpose

Korea Inside Intelligence (KII) is a decision intelligence dashboard for understanding how foreign travelers make practical decisions in Korea.

KII is not a general admin page. It is designed to help Korea Inside answer questions such as:

- Which countries show the strongest travel planning demand?
- Which guides influence accommodation, transport and partner decisions?
- Which stay types, areas and services create the most decision friction?
- Which partner categories have useful commercial intent without reducing user trust?

The v1 dashboard is a foundation only. It uses placeholder data and does not connect to real analytics.

## Data Principles

KII must protect user trust before collecting or reporting data.

Core principles:

- Use anonymous aggregated data only.
- Do not display personal data.
- Do not display raw IP addresses.
- Do not expose individual user journeys.
- Do not use sensitive personal attributes.
- Report trends, patterns and decision categories, not identifiable people.
- Keep partner reporting focused on user suitability and decision quality.

## Anonymous Aggregation Policy

All future KII reporting should aggregate data before it appears in the dashboard.

Recommended aggregation rules:

| Data type | Allowed | Not allowed |
| --- | --- | --- |
| Country trends | Aggregated country-level demand | Individual visitor location history |
| Page performance | Page views and decision events by page | Session replay or personal browsing history |
| Decision flow | Common path patterns | Identifiable user paths |
| Partner interest | Category-level outbound click counts | Personal purchase behavior |
| Reports | Weekly, monthly, seasonal and yearly summaries | Raw visitor logs |

If a metric could identify a person, it should not be shown in KII.

## Dashboard Sections

### 1. Header

The dashboard starts with:

- Korea Inside Intelligence
- Understand How Travelers Decide
- Anonymous aggregated data principle

### 2. Period Filter

The period filter defines the reporting window:

- Today
- 7 Days
- 30 Days
- This Year
- Season
- Custom

In v1, these controls are visual placeholders only.

### 3. Core Metrics

Core metrics summarize the decision funnel:

- Visitors
- Page Views
- Decision Events
- Partner Clicks
- CTR

CTR should be interpreted carefully. A higher CTR is useful only when the partner click follows a helpful decision-support experience.

### 4. Top Countries

Top country reporting should identify market-level demand, not individual users.

Initial placeholder markets:

- USA
- Japan
- Taiwan
- Germany
- France

### 5. Decision Flow

The sample v1 flow is:

Google
-> Stay Guide
-> Accommodation Type
-> Apartment
-> Hongdae
-> Partner Click

Future versions should use aggregated flow patterns to identify where users decide, hesitate or leave.

### 6. Accommodation Insights

Accommodation insight categories:

- Hotel
- Budget Hotel
- Apartment
- Serviced Apartment
- Hanok Stay
- Hostel

These categories should support decision content, stay cluster planning and future partner reports.

### 7. Seasonal Analysis

Seasonal analysis should compare:

- Top country
- Top stay type
- Top area
- Partner interest

Seasonal reporting can help Korea Inside plan content updates before demand peaks.

### 8. Partner Insights

Partner insight categories:

- Booking interest
- Agoda interest
- eSIM interest
- Airport pickup interest
- Transport pass interest

Partner reporting should not rank services by revenue alone. It should also consider user suitability, reliability, convenience and decision context.

### 9. Reports

Planned report types:

- Weekly Report
- Monthly Report
- Seasonal Report
- Yearly Report
- Export PDF
- Export Excel

Exports should use aggregated data only.

## Future Integrations

Future KII integrations may include:

- Privacy-safe analytics
- Search Console performance summaries
- Decision event tracking
- Partner click event tracking
- Stay Cluster funnel analysis
- Seasonal market reports
- Exportable partner reports

Each integration requires separate approval before implementation.

## Partner Report Concept

Partner reports should help answer:

- Which traveler segments show useful interest?
- Which guide pages create qualified partner intent?
- Which countries and seasons match each partner category?
- Which decision factors should be improved before monetization?

Partner reports should remain trust-first. The goal is to explain useful fit, not force users toward affiliate links.

## V1 Implementation Notes

Created files:

- `kii-dashboard.html`
- `docs/korea-inside-intelligence.md`

CSS scope:

- KII dashboard styles should remain scoped to `.kii-dashboard-page`.

V1 limitations:

- Uses placeholder data only.
- Does not connect real analytics.
- Does not export real PDF or Excel files.
- Does not include user accounts or permissions.
- Does not add navigation links.

## Long-Term Direction

KII should become the intelligence layer behind Korea Inside.

The long-term goal is to help the project understand:

- what travelers are trying to decide
- where they become uncertain
- what practical information reduces mistakes
- which partner offers genuinely help users
- which content investments create long-term trust
