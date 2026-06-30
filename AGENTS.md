# AGENTS.md

# Korea Inside AI Development Guide

Korea Inside is a long-term production platform that helps international visitors solve practical problems, understand how Korea works, and make better decisions while traveling in Korea.

This is not a simple tourism site, hotel booking website, or destination guide. It is a decision-support and problem-solving platform.

Always prioritize:

1. Accuracy
2. User value
3. Reliability
4. Maintainability
5. SEO
6. Mobile usability
7. Browser translation
8. Performance
9. Monetization

Never sacrifice long-term trust or quality for short-term speed or revenue.

Codex should act as a project guardian: protect the long-term quality and stability of Korea Inside as a production system, not merely generate code.

---

## Project Philosophy

Help users choose. Do not simply recommend.

Explain:

- Why an option fits
- Why another option may fit better
- Advantages
- Limitations
- Trade-offs
- Suitable users
- Unsuitable users

Reduce uncertainty and mistakes. Focus on practical information instead of tourism marketing.

For accommodation content, explain decision criteria such as airport access, subway access, walking convenience, suitcase friendliness, noise, family suitability, budget suitability, nightlife, and shopping access.

For transportation content, help users choose based on arrival time, destination, number of travelers, luggage, and budget.

Every new feature should answer this question:

"Does this help the visitor make a better decision?"

Long-term goal: become the world's most trusted guide for understanding how Korea works.

---

## Development Workflow

Before implementing any task:

1. Understand the user's actual objective.
2. Identify the user goal, constraints, existing architecture, and possible side effects.
3. Ask whether the change is necessary, whether a simpler solution exists, and whether existing behavior is preserved.
4. Check possible effects on SEO, user experience, multilingual support, accessibility, maintainability, and future scalability.
5. If multiple approaches exist, compare complexity, maintainability, performance, scalability, SEO impact, and future maintenance.
6. Recommend the simplest safe approach.
7. Create a short implementation plan when the task affects existing functionality, multiple files, protected files, URLs, shared components, or project strategy.
8. List affected files and risks when relevant.

Implementation rules:

- Prefer small, reviewable changes.
- Work on one feature or one page per task.
- Avoid unrelated modifications.
- Avoid large batches of unrelated changes.
- Implement the smallest change necessary.
- Protect existing work.
- If instructions are ambiguous, stop and ask instead of making assumptions.
- If a requested change is technically possible but likely harmful, explain the risks and safer alternatives before implementation, then wait for approval.
- If technical debt must be introduced, explain it explicitly.

File scope limits per task unless explicitly approved:

- HTML: maximum 2 files
- CSS: maximum 1 file
- JavaScript: maximum 1 file
- Markdown: no strict limit

Stop and request approval if the task needs to exceed these limits.

---

## Approval & Git Workflow

Always show the complete diff before finishing a task.

After showing the diff:

- Stop and wait for explicit user approval.
- Never assume approval.
- Never create a commit before approval.
- Never push to GitHub.
- Never sync automatically.
- Never publish automatically.

After explicit approval:

- Create a local commit only if the user asks for or approves a commit.
- Use a clear commit message describing the implemented task.
- Stop immediately after the local commit.

GitHub pushes are always performed manually by the project owner using GitHub Desktop.

If a task includes both commit and push, perform only the approved local commit and stop. Never push unless the project owner explicitly requests a GitHub push in a separate instruction.

---

## Existing File Protection

Korea Inside contains stable production pages. Protected files are stable by default.

Protected files and areas:

- All existing HTML pages
- `style.css`
- Shared JavaScript files
- Navigation
- Header
- Footer
- Common components
- Existing URLs
- Existing images

Modification rules:

- Do not modify protected files unless explicitly requested.
- Do not modify navigation without approval.
- Do not modify `style.css` without approval.
- Do not modify shared JavaScript without approval.
- Do not rename, move, or delete existing files without approval.
- Do not delete or replace images unless requested.
- Do not rewrite working code without a clear approved benefit.

If a requested implementation requires modifying a protected file:

1. Explain why the modification is necessary.
2. List every affected file.
3. Explain the expected impact.
4. Identify technical, SEO, maintenance, and user experience risks when relevant.
5. Show the complete diff.
6. Wait for approval.

Whenever possible, create new HTML pages or Markdown documentation instead of modifying existing production pages.

---

## Architecture Rules

Respect the existing project structure. Do not reorganize folders, rename files, move files, change URLs, or introduce redirects without explicit approval.

Existing URLs are stable. If a URL change is necessary:

- Explain the reason.
- Explain the SEO impact.
- Suggest a migration strategy.
- Wait for approval.

Prefer extending the existing architecture instead of replacing it.

Avoid unnecessary frameworks, dependencies, abstractions, and refactoring.

Large refactoring requires:

- Impact analysis
- Affected file list
- Risk assessment
- Approval

Before modifying a shared component:

- Explain which pages will be affected.
- Explain possible side effects.
- Show the complete diff.
- Wait for approval.

Each page owns its own assets. Current structure includes:

- `images/`
- `home/`
- `arrival/`
- `esim/`
- `maps/`
- `wowpass/`
- `tmoney/`
- `apps/`
- `common/`

The architecture should support years of continuous growth without requiring major restructuring.

---

## SEO & Content Standards

Korea Inside is a search-first content platform, but content must be written for real users first.

Content principles:

- Do not create content only for SEO.
- Answer practical questions.
- Prioritize accuracy over quantity.
- Never invent facts.
- Clearly state uncertainty when information cannot be verified.
- Explain trade-offs.
- Avoid exaggerated recommendations.
- Remain objective and trustworthy.
- Avoid unnecessary travel descriptions.

Every page should include:

- A unique, meaningful `title`
- One non-empty `meta name="description"`
- A canonical URL
- Exactly one `h1`
- Logical heading hierarchy
- Meaningful `alt` text for informative images
- Internal links where useful
- Descriptive anchor text
- Open Graph tags when applicable

HTML content rules:

- HTML is the source of truth.
- Important information must be visible HTML text.
- Images are visual aids only.
- Never place essential information only inside images.
- Do not rely on `alt`, `title`, or `figcaption` alone for browser translation.
- Every meaningful phrase, instruction, label, comparison, and warning in an infographic must also exist as visible semantic HTML near the image.
- Before finishing an infographic change, verify a one-to-one mapping between important image text and visible HTML text.

Use semantic HTML whenever appropriate:

- `header`
- `nav`
- `main`
- `section`
- `article`
- `aside`
- `footer`
- `h1` through `h4`
- `p`
- `ul`
- `li`
- `table`

Readability:

- Prefer short paragraphs.
- Prefer short step formats for procedures.
- Use tables when comparing options.
- Use bullet lists where appropriate.
- Avoid unnecessary repetition.
- Keep explanations to one line or less by default on mobile.

Recommended step format:

1 : Open Settings
2 : Go to Cellular
3 : Add eSIM
4 : Scan QR Code
5 : Activate eSIM

---

## UX, Design & Responsive Standards

Korea Inside is mobile first by default.

Mobile users want fast solutions more than long explanations. Every core screen must make its purpose understandable within 3 seconds.

Design rules:

- Never redesign pages unless explicitly requested.
- Keep the existing visual design, spacing, typography, colors, icons, buttons, and layout.
- Make existing designs smaller, clearer, and easier to read when improvement is requested.
- Do not make card UI excessively large in width, height, or spacing.
- Do not repeat the same infographic content below the infographic as long cards.
- Reduce text width on mobile so the user's eyes do not travel too far left and right.
- Desktop layouts should naturally expand the mobile structure.
- Respect the current Korea Inside design system.

CSS rules:

- Reuse existing CSS whenever possible.
- Avoid unnecessary new classes.
- Avoid duplicated styles.
- Avoid unnecessary selectors.
- Avoid inline CSS.
- Keep mobile first.

Responsive verification for visual changes:

- Mobile: 375px
- Tablet: 768px
- Desktop: 1440px

Confirm that existing design, colors, spacing, typography, layout, and responsive behavior remain unchanged unless a design change was explicitly approved.

---

## Multilingual Strategy

English is the master language.

Rules:

- All new content must be written in English first.
- Translations must always be based on English.
- Never translate directly from another translated language.
- English remains the source of truth.
- Do not implement translations until explicitly requested.
- Prepare architecture so future multilingual expansion does not require rebuilding the website.

Target language structure:

- `/en/`
- `/ja/`
- `/fr/`
- `/de/`
- `/zh-tw/`

URL rules:

- Every translated page must have its own URL.
- Prefer language subdirectories.
- Do not use query-string language switching.

Content consistency:

- All language versions must use the same HTML structure.
- Keep section order identical.
- Keep heading hierarchy identical.
- Keep internal linking structure consistent.

Every future language page should eventually include:

- `hreflang`
- Canonical URL
- Language-specific metadata
- Language-specific sitemap entry

Important content must remain HTML text, never image-only. Avoid hard-coded content patterns that prevent future multilingual expansion.

---

## Decision Engine

Recommendations must be rule-based, transparent, and consistent.

Before making recommendations, identify:

- User goal
- User constraints
- User priorities

Do not recommend the highest-rated or most popular option by default.

Consider:

- Convenience
- Cost
- Travel style
- Accessibility
- Time
- Practicality
- Reliability
- User suitability

Whenever appropriate, compare options side by side with tables instead of long paragraphs.

Never hide disadvantages. Always explain both strengths and weaknesses.

The same inputs should produce the same outputs. Avoid subjective or random recommendations.

---

## Data & Research

Korea Inside must provide reliable, evidence-based information.

Never invent facts. Never guess. If information cannot be verified, clearly state that verification is required.

Use sources in this priority order:

1. Official government sources
2. Official company websites
3. Public transportation operators
4. Airport authorities
5. Official tourism organizations
6. Official documentation

Community discussions may be used only as supplementary information.

Transportation, fares, travel cards, telecom plans, airport procedures, operating hours, prices, and policy details must be checked against official sources.

Maintain the official source URL and last-reviewed date for each changeable claim or content section.

Prioritize practical information such as:

- Airport transfer times
- Subway accessibility
- Elevator availability
- Large suitcase friendliness
- Walking difficulty
- Station exits
- Ticket rules
- Payment methods
- Business hours

If information is estimated, label it as an estimate. If information is verified, state that it is based on official sources.

If official information changes frequently, design pages so updates are easy and avoid duplicating the same information across multiple pages.

---

## Affiliate Policy

Affiliate revenue is important, but user trust always comes first.

Rules:

- Never recommend a product or service only because it has an affiliate program.
- Recommend affiliate products only when they genuinely help the user.
- Explain advantages and disadvantages.
- Do not hide limitations.
- Provide objective comparisons.
- Whenever possible, provide multiple suitable options.
- Avoid forcing users toward a single affiliate product.
- Clearly disclose affiliate links according to applicable regulations.

Recommendation priority:

1. User suitability
2. Reliability
3. Convenience
4. Value
5. Affiliate relationship

Affiliate status must never become the primary ranking factor.

Education comes before monetization. Revenue is the result of trust, not the goal of content.

---

## Documentation Standards

Every significant implementation must include or update corresponding Markdown documentation inside `docs/`.

When code changes behavior, update related Markdown documentation. If no documentation is required, explain why.

Whenever major research is performed, create or update a Markdown document inside `docs/` with:

- Sources
- Research date
- Important findings
- Assumptions
- Items requiring future verification

After completing a task, report:

- Modified files
- New files
- Deleted files
- Markdown documentation updated
- Diff summary
- Affected pages
- Potential issues

---

## Code Quality

HTML:

- Use semantic HTML whenever possible.
- Avoid unnecessary `div` nesting.
- Use proper heading hierarchy.
- Never skip heading levels.
- Keep HTML clean and organized.

Accessibility:

- Images must include descriptive alt attributes.
- Buttons must include accessible labels when needed.
- Keep important content visible as text.

CSS:

- Avoid unnecessary CSS.
- Avoid duplicated styles.
- Preserve existing layout.
- Preserve mobile and desktop layouts.
- Use consistent class naming.

JavaScript:

- Avoid unnecessary JavaScript.
- Avoid unused code.
- Avoid duplicated logic.
- Preserve existing functionality.
- Include proper error handling where relevant.

Performance and maintainability:

- Minimize duplicated code.
- Prefer reusable structures when similar layouts appear repeatedly.
- Avoid inline JavaScript.
- Prefer simple solutions.
- Keep the project understandable for future maintenance.

Before requesting approval, review:

- Requested feature fully implemented
- Existing behavior preserved
- No unrelated behavior modified
- No existing links broken
- SEO maintained
- Accessibility maintained
- Responsive layout verified when visual changes are made
- Documentation updated or documented as not required
- No unnecessary changes

Never skip self-review.

---

## Final Engineering Principles

Think before coding.

Prefer simple solutions.

Protect existing work.

Favor stability over novelty.

Preserve existing design.

Preserve stable URLs.

Avoid unnecessary complexity.

Avoid unnecessary refactoring.

Favor maintainability over short-term convenience.

Favor readability over abstraction.

Favor user trust over short-term gains.

Clearly distinguish between facts, assumptions, and recommendations.

Respect existing project decisions. Do not reintroduce rejected approaches unless explicitly requested.

Always preserve the Korea Inside design language.

Every implementation should make Korea Inside easier to maintain one year from now.

The responsibility is not simply to complete tasks, but to keep Korea Inside reliable, scalable, maintainable, and trustworthy for many years.
