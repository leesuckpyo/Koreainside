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
Last Reviewed : 2026-09-18
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

The user's current direct instruction is the highest execution authority.

A task-specific Codex instruction approved by the user serves as the Implementation Contract for that task. A separate document-format Implementation Contract does not need to be created for every task.

An instruction that changes repository files must define at least:

- Task goal
- Modification scope
- Protected or prohibited scope
- Validation criteria

A read-only inspection may proceed from the user's direct instruction without a separate Implementation Contract.

If the instruction, scope, protection boundary, or validation criteria are unclear or conflicting, Codex must not interpret them into permission to modify files. Report the difference and ask the user.

Use `docs/implementation-contract-standard.md` when the user requests or approves a separate formal Implementation Contract.

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

### Creation Rule

Codex must create files, folders, pages, documents, components, assets, or projects only when the user explicitly instructs it to do so.

If creation appears necessary but is not approved, report the reason and wait for explicit user approval before creating it.

### Modification Rule

Do not edit, refactor, rename, reformat, optimize, clean up, or improve unrelated files, pages, components, CSS rules, JavaScript code, navigation structures, documentation files, assets, or metadata.

Report issues outside the requested scope separately and wait for user approval before fixing them.

### No Helpful Extra Work Rule

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
4. Confirm the approved files, sections, actions, protection boundaries, and validation criteria. The current user instruction may already provide this approval.
5. Explain the impact before changing protected files, shared components, URLs, or structure. If approval is missing or the scope must change, present the proposed diff and wait for explicit approval before editing.
6. Apply only the approved changes. Existing approval does not extend to additional files or actions.
7. Complete self-review and the minimum verification required by the selected QA level.
8. Show the full resulting diff without abbreviation and report using Reporting Format.
9. Stop after the approved task. Perform Git actions only as explicitly authorized under Approval & Git Workflow.

Scope approval precedes editing; review of the resulting diff follows implementation. Neither grants commit or push approval.

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

## Execution Authority and Current Context

### Execution Authority

Use this order to decide whether Codex may act:

1. The user's current direct instruction
2. A task-specific Codex instruction or Implementation Contract approved by the user
3. The scope, protection, approval, and Git rules in root `AGENTS.md`
4. The relevant specialized standard
5. Page specifications and implementation references

No handoff, TODO, ROADMAP, standard, page specification, or historical record grants execution authority by itself.

### Current State and Context Sources

Use this order to determine what is currently true:

1. Current state re-verified in the actual repository or operating interface
2. The latest handoff explicitly designated by the user
3. Current research, official sources, and measurement data
4. Historical handoffs and historical records

The latest handoff is the primary handoff reference for current state, completed work, and next-task context. It does not let Codex automatically start a new task without the user's execution approval.

Even when a latest handoff names a next task, that task remains a candidate until the user approves it.

If the actual Git or operating state differs from the latest handoff, verify the actual state and report the difference.

A historical handoff is context only and never current execution authority.

Codex performs only the work currently approved by the user.

The word "next" means the next procedure inside the currently approved task. It does not authorize Codex to select another page, feature, document, or backlog item.

A new task may be selected only by:

- the user's current direct instruction
- a task-specific Codex instruction or Implementation Contract approved by the user

Use the latest handoff only for current state, execution context, next-task context, and Git or deployment status. Do not read every historical handoff for every task.

Consult a historical handoff only when:

- the reason for an earlier decision is needed
- before-and-after context must be restored
- completed history must be verified
- background omitted from the latest handoff is required

If the latest and a historical handoff conflict, use the latest handoff. Never re-run a historical next task, Git state, approval state, or Codex instruction as current work. Historical handoffs remain project records and are not deletion targets.

TODO and ROADMAP documents are planning references, not execution approval. A TODO item is a reviewable backlog candidate. A ROADMAP describes long-term direction and stages.

## Image Production Execution Boundary

Generate or edit an image only when the user directly asks to create or modify an image, illustration, infographic, or other visual asset.

Statements such as "an image is needed," "review the infographic direction," "consider the image placement," "finish the infographic tomorrow," or "list the required images" are planning or discussion, not production approval.

Do not invoke an image-generation or image-editing tool merely because the task mentions an image or infographic.

Before production, the approved scope must make the target, purpose, aspect ratio or dimensions, visual direction, and requested change sufficiently clear. If any material production requirement is ambiguous, stop and ask instead of inventing it.

## Specialized Standards Reference

Keep detailed professional rules in their source-of-truth documents instead of duplicating them here.

- Product principles: `docs/product-constitution.md`
- Business principles: `docs/business-operating-system.md`
- Documentation roles and reference order: `docs/standards-hub.md`
- Content writing and editorial QA: `docs/content-writing-standard.md`
- Visual design and component rules: `docs/design-system.md`
- Travel Guide family design: `docs/design-travel-guide.md`
- SEO: `docs/seo-standard.md`
- Multilingual SEO: `docs/multilingual-seo-strategy.md`
- Decision logic: `docs/decision-engine.md`
- Research and source management: `docs/knowledge-management.md`
- Affiliate publication policy: `docs/content-writing-standard.md`
- Business and monetization principles: `docs/business-operating-system.md`

These documents provide domain standards. They do not expand the approved task scope or authorize file changes.

Before creating a public page or revising an existing page layout, Codex must:

1. Identify the page's parent navigation category and page family.
2. Read `docs/design-system.md`.
3. Read the Active Family Design Standard for that page family.
4. Reuse an approved same-role family component when one exists.
5. If the Family Design Standard does not cover the required UI, stop, report the reason, and obtain explicit user approval before creating a different design.

If the parent navigation category or page family has no Active Family Design Standard, Codex must stop, report `New Page Family Design Standard required`, ask the user for approval, and not invent a design before approval.

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
- Avoid large batches of unrelated changes.
- Implement the smallest change necessary.
- If a requested change is technically possible but likely harmful, explain the risks and safer alternatives before implementation, then wait for approval.
- If technical debt must be introduced, explain it explicitly.

Apply the User Instruction First and Scope Control section and the Existing File Protection section throughout implementation.

File scope limits per task unless explicitly approved:

- HTML: maximum 2 files
- CSS: maximum 1 file
- JavaScript: maximum 1 file
- Markdown: no strict limit

Stop and request approval if the task needs to exceed these limits.

---

## Approval & Git Workflow

Use Required Work Sequence for implementation, including documentation-rule changes.

Approval rules:

- Treat approval as explicit only when the Product Owner clearly gives it.
- Proceed to the next task only after a new user instruction.
- Stage only explicitly approved files using named paths. Never use `git add .` or `git add -A`.
- Create a local commit only when the user asks for or approves a commit.
- Do not push automatically after commit.
- Push to GitHub only when the Product Owner explicitly instructs or approves the push.
- If the Product Owner says they will push manually in GitHub Desktop or by another direct method, do not push from Codex.
- Sync or publish only when explicitly instructed.

Completion rules:

- After applying the approved change, perform only the minimum verification required by the selected QA level, report the result, and stop immediately.
- Do not continue into automatic QA, screenshot generation, extra verification, extra improvements, refactoring, commits, pushes, or another task unless the user explicitly requests it.

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
- Hero
- Common components
- Existing URLs
- Existing images
- Mobile hamburger menu
- Common navigation structure
- Menu width, height, position, transparency, and scrolling behavior
- Accordion behavior
- `common.js` events and related common CSS
- Existing backup folders

Preserve pre-existing user modifications and untracked files. Do not modify, delete, revert, or stage them without explicit approval for those files and actions.

### Authentication and Secret Protection

- Protect the existing Google Search Console OAuth implementation and Windows Credential Manager handling.
- Never print or expose Vercel Access Tokens, API tokens, passwords, account or banking information, authentication data, or other secrets in logs, command output, reports, screenshots, or documentation.
- Do not issue, rotate, reset, delete, or replace existing credentials unless the user explicitly approves that exact action.
- Do not change OAuth, credential storage, token handling, or authentication settings without explicit user approval.
- If verification can reveal a secret, report only whether the configuration exists or passes; redact the value.

Modification rules:

- Modify protected files only when explicitly requested.
- Modify navigation only with approval.
- Modify `style.css` only with approval.
- Modify shared JavaScript only with approval.
- Rename, move, or delete existing files only with approval.
- Delete or replace images only when requested.
- Rewrite working code only when the user approves the specific benefit.

For protected-file changes, follow Required Work Sequence. Before editing, explain the reason, list every affected file, and identify the expected impact and relevant technical, SEO, maintenance, and user experience risks.

When the user explicitly approves creating new content, prefer new HTML pages or Markdown documentation instead of modifying existing production pages whenever that is the safer option.

---

## Architecture Rules

Respect the existing project structure. Reorganize folders, rename files, move files, change URLs, or introduce redirects only with explicit approval.

Existing URLs are stable. A URL change requires explicit approval under Required Work Sequence; explain the reason, SEO impact, and migration strategy before applying it.

Prefer extending the existing architecture instead of replacing it.

Avoid unnecessary frameworks, dependencies, abstractions, and refactoring.

Large refactoring requires:

- Impact analysis
- Affected file list
- Risk assessment
- Approval

Before modifying a shared component, explain which pages will be affected and the possible side effects. Apply the approval and full-diff requirements in Required Work Sequence.

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

## Domain Standard Boundary

Content, SEO, design, localization, research, recommendation, and affiliate rules belong to the specialized standards listed above.

When implementation touches one of those domains:

- read the relevant standard before editing
- keep important information as visible semantic HTML
- preserve browser translation, accessibility, mobile usability, and existing URLs
- reuse the nearest approved structure and component
- apply only the QA level authorized by this document and the current task

Do not copy an entire specialized standard into this file.

### Content Direction and Implementation Boundary

- Content titles, `h1`, core conclusions, image direction, copy direction, and major section structure follow the user's approval or an approved final specification.
- Codex implements the approved final specification as given.
- Codex must not independently change a title, image direction, copy direction, or recommendation conclusion.
- Do not independently author or rewrite public copy without the user's explicit approval. Content marked `DONE LOCKED` must not be reopened or changed without explicit approval for that content.
- When the user supplies finalized English copy, apply it exactly. Do not autonomously rewrite, shorten, translate, summarize, or improve it.
- If a technical constraint or existing structure conflicts with the approved specification, report the difference and alternatives instead of making an unapproved substitute.
- Within the approved scope, Codex may correct only changes that do not alter the approved purpose, such as a simple typo, grammar error, or clearly broken link.
- Keep content-direction decisions separate from the implementation role.

Detailed voice, wording, recommendation, and editorial QA rules belong to `docs/content-writing-standard.md`.

## Korea Inside Content Humanization Standard

For Korea Inside Global English content, requests for "humanization," "humanize," "natural copy," removal of AI or mechanical writing, human-sounding sentences, or a natural travel-guide voice invoke this standard.

Follow the Humanization Standard in `docs/content-writing-standard.md` for rewriting complete thoughts, natural language, phrase patterns, examples, and editorial QA. Humanization is not synonym replacement.

- Preserve factual meaning, search intent, useful travel information, verified data, internal links, affiliate links, and tracking attributes. Any change to protected content or implementation data still requires explicit approval.
- Apply Content Direction and Implementation Boundary, including finalized-copy and `DONE LOCKED` protection, before any humanization work.

This standard does not govern Korea Inside Japan or Japanese-language localization.

---

## Codex QA / Verification Rules

### QA Efficiency Rule

Verification must be proportional to the scope and risk of the change.

The primary objective is to complete the requested implementation efficiently. Verification supports implementation and must never become the main task.

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

### Reporting Format

For every task, report:

- Whether approval was received before applying changes.
- Whether the task stopped after the approved change.
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

## Documentation Standards

When documentation is explicitly requested or approved for a significant implementation, update the corresponding Markdown documentation inside `docs/`.

When code changes behavior, update related Markdown documentation only if the user explicitly requests or approves that documentation action. If no documentation update is included, explain why.

Whenever major research documentation is explicitly requested or approved, create or update a Markdown document inside `docs/` with:

- Sources
- Research date
- Important findings
- Assumptions
- Items requiring future verification

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
