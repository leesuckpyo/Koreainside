# AGENTS.md

## Document Metadata

Layer : L1
Status : Active
Authority Type : Behavior Rules
Primary Responsibility : AI and Codex working rules
Source of Truth For : Scope control, approval rules, implementation workflow, git workflow, protected actions, and Codex behavior requirements
Not Responsible For : Product philosophy, business strategy, documentation index, page-specific standards, reference records, history records
Higher Priority Documents : Current User Instruction
Related Documents : PROJECT.md, docs/product-constitution.md, docs/business-operating-system.md, docs/standards-hub.md
Change Policy : Owner approval required before change
Last Reviewed : 2026-07-09
Review Trigger : Codex workflow, approval policy, protected file policy, git workflow, or AI behavior rule changes

## Project Base and Implementation Contract

Project Base is an operating concept, not a separate Markdown file.

Before any Codex work starts, Codex must read the Project Base documents:

- PROJECT.md
- AGENTS.md
- docs/product-constitution.md
- docs/business-operating-system.md
- docs/standards-hub.md

Codex must not implement from conversation memory.

Codex must judge from the current user instruction and the current repository state.

If the current instruction, repository state, or documentation is unclear, incomplete, or conflicting, Codex must follow this rule:

Unknown → STOP → Report → Ask → Never Assume.

No implementation, file modification, deletion, movement, renaming, refactoring, formatting, staging, commit, or push may begin without an explicit Implementation Contract approved by the owner.

An Implementation Contract must define:

- Task goal
- Target files
- Allowed changes
- Prohibited changes
- Required source documents
- Validation method
- Git boundary
- Stop conditions

Use docs/implementation-contract-standard.md as the standard format for all owner-approved Implementation Contracts.

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

## User Instruction First and Scope Control

### User Instruction First Rule

Codex must follow the user's explicit instruction first.

Codex may create, modify, delete, move, rename, refactor, optimize, format, document, or reorganize only what the user explicitly instructed.

Codex must not perform extra work proactively, automatically, or as a helpful improvement.

If Codex believes an additional action is necessary, Codex must stop, explain why the action is needed, and wait for explicit user approval.

### Scope Control Rule

Each task is limited to the files, sections, and actions explicitly approved in the current instruction.

Files not named in the current instruction are out of scope.

Sections not named in the current instruction are out of scope.

Actions not named in the current instruction are out of scope.

### User-Directed Change Exception

Existing features, SEO structure, FAQ, content, navigation, CSS, JavaScript, assets, and documentation are protected by default.

However, if the user explicitly instructs deletion, replacement, restructuring, or modification, Codex may perform that action within the approved scope only.

This means:

- Do not delete or change existing work by default.
- Do delete or change existing work when the user explicitly instructs it.
- If the instruction is unclear, stop and ask for approval.

### Creation Rule

Codex must create files, folders, pages, documents, components, assets, or projects only when the user explicitly instructs it to do so.

If creation appears necessary to complete the task, Codex must not create it automatically.

Codex must report the reason and wait for explicit user approval.

### Modification Rule

Codex must modify only the files and sections explicitly requested by the user.

Do not edit, refactor, rename, reformat, optimize, clean up, or improve unrelated files, pages, components, CSS rules, JavaScript code, navigation structures, documentation files, assets, or metadata.

Even if Codex finds an issue outside the requested scope, Codex must not fix it automatically.

Codex must report the issue separately and wait for user approval.

### No Helpful Extra Work Rule

Codex must not make helpful improvements outside the requested task.

The following actions require explicit user instruction:

- Creating files
- Creating folders
- Creating pages
- Creating documents
- Modifying unrelated files
- Editing shared CSS
- Editing shared JavaScript
- Changing navigation
- Changing SEO metadata
- Renaming classes
- Moving sections
- Adding sections
- Deleting sections
- Rewriting page structure
- Normalizing code style
- Applying backup files
- Applying patch files
- Restoring from older Git versions
- Updating multiple pages for consistency

### Markdown Cleanup and Conflict Reporting Rule

Markdown documentation is not an append-only file.

When editing Markdown documentation, Codex must first read the existing document structure and existing rules.

When editing Markdown documentation, Codex must not simply add new rules on top of existing conflicting, duplicated, unnecessary, outdated, overbroad, or ambiguous rules.

If existing Markdown text is problematic, Codex must report it during the Markdown modification task.

Problematic Markdown text includes:

- Duplicated rules
- Rules that conflict with the current user instruction
- Outdated project rules
- Unnecessary rules
- Overbroad absolute prohibitions
- Ambiguous instructions
- Rules that may cause future task conflicts
- Rules that no longer match the current Korea Inside workflow

For each problematic text or section, Codex must report:

1. The problematic text or section
2. The reason it is a problem
3. The proposed action: delete, replace, merge, or keep
4. Whether the action is included in the diff

Codex must not silently keep conflicting rules and add another rule above them.

When the current user instruction explicitly asks for Markdown cleanup, Codex may delete, replace, or merge problematic Markdown text within the approved files only.

Codex must show all deletions, replacements, and merges in the diff.

Codex must not commit before explicit user approval.

### Backup and Restore Rule

Backup ZIPs, older Git versions, patch files, and external files are reference materials only unless the user explicitly approves their use.

Codex must not overwrite current project files from any backup, ZIP, patch, or older version without:

1. Comparing against the current file
2. Showing the diff
3. Receiving explicit user approval

### Required Work Sequence

For every task, Codex must follow this sequence:

1. Run `git status`.
2. Confirm the working tree state.
3. Read the relevant Markdown documentation.
4. Identify the exact files and actions allowed by the current user instruction.
5. Modify only the approved files and sections.
6. Show the full diff without abbreviation.
7. Wait for user approval.
8. Commit only after explicit user approval.

### Stop Conditions

Codex must stop and ask for approval if:

- The requested task requires editing additional files.
- The requested task requires creating new files, folders, pages, documents, components, assets, or projects not explicitly instructed.
- The Markdown documentation appears outdated or incorrect.
- The current file conflicts with the documented rule.
- The requested change affects shared layout, navigation, CSS, JavaScript, SEO, or multiple pages.
- The task requires restoring from backup, ZIP, patch, or Git history.
- Codex is unsure whether a file or action is within scope.

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
- Keep work limited to the current user instruction.
- Avoid large batches of unrelated changes.
- Implement the smallest change necessary.
- Protect existing work.
- If instructions are ambiguous, stop and ask instead of making assumptions.
- If a requested change is technically possible but likely harmful, explain the risks and safer alternatives before implementation, then wait for approval.
- If technical debt must be introduced, explain it explicitly.

Scope control rules:

- Edit the files and sections explicitly requested or approved in the current user instruction.
- Treat unrelated pages and components as out of scope unless the user explicitly includes them.
- Treat Header, Footer, Hero, navigation, shared JavaScript, common CSS, and shared components as out of scope unless the user explicitly includes them.
- Refactor CSS or rename classes only when the user explicitly instructs or approves that action.
- Move, rename, delete, or reorganize files only when explicitly instructed or approved.
- Stop and ask when the requested scope is unclear.

File scope limits per task unless explicitly approved:

- HTML: maximum 2 files
- CSS: maximum 1 file
- JavaScript: maximum 1 file
- Markdown: no strict limit

Stop and request approval if the task needs to exceed these limits.

---

## Approval & Git Workflow

For every implementation:

1. Analyze the request.
2. Identify the exact files, sections, and actions approved by the current user instruction.
3. Modify only the approved files and sections.
4. Show the full diff without abbreviation.
5. Wait for explicit user approval before commit or follow-up work.
6. Stop immediately after the requested task is complete.

For documentation-rule changes, limit edits to named documents, show the full diff without abbreviation, and wait for Product Owner approval before commit or follow-up work.

Approval rules:

- Stop and wait for explicit user approval.
- Treat approval as explicit only when the Product Owner clearly gives it.
- Proceed to the next task only after a new user instruction.
- Create a local commit only when the user asks for or approves a commit.
- Do not push automatically after commit.
- Push to GitHub only when the Product Owner explicitly instructs or approves the push.
- If the Product Owner says they will push manually in GitHub Desktop or by another direct method, do not push from Codex.
- Sync or publish only when explicitly instructed.

Completion rules:

- After applying the approved change, perform only the minimum verification required by the selected QA level, report the result, and stop immediately.
- Do not continue into automatic QA, screenshot generation, extra verification, extra improvements, refactoring, commits, pushes, or another task unless the user explicitly requests it.
- Do not proceed to the next task without a new user instruction.

After explicit commit approval:

- Use a clear commit message describing the implemented task.
- Stop immediately after the local commit.
- Do not treat commit approval as push approval.

Before pushing:

- Verify `git status`.
- Verify the working tree is clean.
- Verify the ahead/behind state.
- Verify the latest commit.
- Verify the files included in the commit.
- Verify there are no unexpected changes or untracked files.
- Do not force push, rebase, merge, or pull unless the Product Owner explicitly instructs it.

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

- Modify protected files only when explicitly requested.
- Modify navigation only with approval.
- Modify `style.css` only with approval.
- Modify shared JavaScript only with approval.
- Rename, move, or delete existing files only with approval.
- Delete or replace images only when requested.
- Rewrite working code only when the user approves the specific benefit.

If a requested implementation requires modifying a protected file:

1. Explain why the modification is necessary.
2. List every affected file.
3. Explain the expected impact.
4. Identify technical, SEO, maintenance, and user experience risks when relevant.
5. Show the complete diff.
6. Wait for approval.

When the user explicitly approves creating new content, prefer new HTML pages or Markdown documentation instead of modifying existing production pages whenever that is the safer option.

---

## Architecture Rules

Respect the existing project structure. Reorganize folders, rename files, move files, change URLs, or introduce redirects only with explicit approval.

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

HTML First Policy:

- When the user requests removal of a section, delete the HTML itself.
- Do not hide removed sections with CSS such as `display: none` unless the user explicitly requests it.
- Keep important content as visible semantic HTML text.

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

Responsive verification for visual changes must follow the Codex QA / Verification Rules below and must not use automated screenshots, Chrome CDP, or scroll verification unless explicitly requested.

- Do not run full responsive QA for minor visual changes unless the Product Owner requests it.
- Use the smallest verification level that matches the change scope and risk.
- Preserve existing design, colors, spacing, typography, layout, and responsive behavior unless a design change was explicitly approved.
- For Level 3 Release QA, use the standard responsive viewports: mobile 375px, tablet 768px, desktop 1440px.

---

## Codex QA / Verification Rules

### QA Efficiency Rule

Verification must be proportional to the scope and risk of the change.

The primary objective is to complete the requested implementation efficiently. Verification supports implementation and must never become the main task.

For every implementation:

1. Complete the approved change.
2. Perform only the minimum verification required by the selected QA level.
3. Report the result.
4. Stop immediately.

Do not delay task completion with unnecessary verification.

User browser verification has priority. Codex should not repeatedly automate checks that the user can verify directly in the browser.

If the Product Owner will verify the result directly in the browser, skip automated visual QA and report:

"User browser verification requested."

Unless the Product Owner explicitly requests it, do not:

- Generate screenshots.
- Perform full responsive QA.
- Test unrelated pages.
- Repeat browser verification.
- Retry Chrome CDP multiple times.
- Perform scroll-position verification.
- Verify sections that were not modified.

### Screenshot Rule

Screenshots are optional.

Generate screenshots only when explicitly requested by the Product Owner.

Default:

- No screenshots.
- No repeated captures.
- No full-page captures.

When screenshots are requested:

- Desktop: one screenshot.
- Mobile: one screenshot.

If screenshot generation fails:

- Report the failure.
- Mark it as "Known QA Limitation".
- Stop.

### Chrome CDP Rule

Chrome CDP is optional.

If Chrome CDP or browser automation fails once:

- Report the failure.
- Mark it as "Known QA Limitation".
- Stop.

Never repeatedly retry browser automation unless explicitly requested.

### QA Scope Rules

#### Level 1 - Quick Verify

Use this for:

- Text changes
- CSS spacing
- Font adjustments
- Icon changes
- Image replacement
- Minor visual improvements
- Single-section HTML edits

Verification:

- Check only the modified page or component.
- Do not inspect unrelated pages.
- Do not generate screenshots unless the user explicitly requests them.
- Do not run responsive QA.
- Stop after confirming there are no obvious implementation errors.

#### Level 2 - Feature Verify

Use this for:

- Navigation
- Hamburger menu
- JavaScript changes
- Interactive components
- CTA behavior

Verification:

- Test only the affected feature.
- Verify desktop and mobile behavior.
- Check console errors if applicable.
- Do not perform release-level QA.

#### Level 3 - Release QA

Use only when:

- Explicitly requested by the Product Owner
- Final release check
- Large multi-page implementation
- Navigation-wide or architecture changes

Verification:

- Desktop.
- Tablet.
- Mobile.
- Navigation flow.
- Critical user journey.
- Console errors.

Do not use Level 3 for ordinary feature development.

### QA Stop Rule

Stop QA and report as "Known QA Limitation" when:

- Browser tooling is unavailable.
- Playwright or headless browser fails due to environment issue.
- The issue is unrelated to the current change.
- The Product Owner has already confirmed the result on a real device.
- Additional QA would not change the implementation decision.

### Product Owner / User Verification Priority

If the Product Owner confirms the result or will verify the browser directly, do not repeat automated visual checks unless explicitly requested.

### Reporting Format

For every task, report:

- Whether approval was received before applying changes.
- Whether the task stopped after the approved change.
- Impact
- Time
- Priority
- Changed files
- Verification level used
- QA result
- Known QA limitations, if any

Important:

- Do not perform full QA for minor changes.
- Do not delay implementation with excessive verification.
- Time is also a quality factor.
- For document changes, modify only the named documents and sections, show the full diff without abbreviation, and wait for Product Owner approval before commit or follow-up work.
- After the approved implementation and required verification level are complete, report the result and stop immediately.
- Do not continue with additional QA, screenshots, improvements, refactoring, commits, pushes, or another task unless explicitly requested by the Product Owner.

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

When documentation is explicitly requested or approved for a significant implementation, update the corresponding Markdown documentation inside `docs/`.

When code changes behavior, update related Markdown documentation only if the user explicitly requests or approves that documentation action. If no documentation update is included, explain why.

Whenever major research documentation is explicitly requested or approved, create or update a Markdown document inside `docs/` with:

- Sources
- Research date
- Important findings
- Assumptions
- Items requiring future verification

After completing a task, report:

- Impact
- Time
- Priority
- Changed files
- Diff summary
- Affected pages
- Verification level used
- QA result
- Known QA limitations, if any

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
- Verification level selected according to Codex QA / Verification Rules
- Documentation action matches the current user instruction or approval
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
