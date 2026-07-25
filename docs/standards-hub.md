# Korea Inside Standards Hub

## Document Metadata

Layer : L1
Status : Active
Authority Type : Standards Index
Primary Responsibility : Documentation map, document roles, reference order, and source-of-truth navigation
Source of Truth For : Standards organization, document role boundaries, current documentation inventory, and reference order
Not Responsible For : File-change approval, product philosophy, business strategy, current tasks, Git state, reference records, or history
Higher Priority Documents : Current User Instruction, AGENTS.md
Related Documents : PROJECT.md, AGENTS.md, docs/product-constitution.md, docs/business-operating-system.md, docs/conflict-resolution-standard.md
Change Policy : Owner approval required before change
Last Reviewed : 2026-07-25
Review Trigger : Documentation role, source-of-truth mapping, document inventory, or reference-order change

## 1. Purpose

This hub explains which Markdown document to use for each kind of decision.

It does not authorize document creation, deletion, movement, renaming, merging, refactoring, staging, commit, or push.

Keep professional rules in their specialized documents. Root `AGENTS.md` contains durable Codex behavior and approval rules; it should link to professional standards instead of copying them.

## 2. Execution Authority

Use this order to decide whether Codex may act:

1. The user's current direct instruction
2. A task-specific Codex instruction or Implementation Contract approved by the user
3. Root `AGENTS.md` scope, protection, approval, and Git rules
4. The relevant specialized standard
5. Page specifications and implementation references

No handoff, TODO, ROADMAP, standard, page specification, or historical record grants execution authority by itself.

## 3. Current State and Context Sources

Use this order to determine what is currently true:

1. Current state re-verified in the actual repository or operating interface
2. The latest handoff explicitly designated by the user
3. Current research, official sources, and measurement data
4. Historical handoffs, Decision Log, Project Memory, and historical records

If no latest handoff is explicitly designated, do not infer one from a dated report or historical record.

If the actual Git or operating state differs from a handoff, verify the actual state and report the difference.

Historical records preserve context. They are not current execution authority.

## 4. Current Context and Planning Documents

### Latest Handoff

The latest handoff, when explicitly designated by the user, is the primary handoff reference for current state, completed work, next-task context, and Git or deployment state.

It does not authorize Codex to start a new task. A next task named in the handoff remains a candidate until the user approves execution.

Do not read every historical handoff for every task. Consult a historical handoff only to recover:

- the reason for an earlier decision
- before-and-after context
- completed history
- background missing from the latest handoff

If a latest and historical handoff conflict, use the latest. Never re-run a historical next task, Git state, approval state, or Codex instruction.

### TODO

`project/TODO.md` contains reviewable backlog candidates. It does not select or approve the next task.

### ROADMAP

`project/ROADMAP.md` contains long-term direction and stages. It is not a page queue or automatic development sequence.

### Historical Records

Decision logs, project memory, changelogs, reports, and history documents are consulted selectively for context and preserved as records.

## 5. Core Documents

| Document | Role |
|---|---|
| `../AGENTS.md` | Durable Codex behavior, scope, approval, Git, protection, and QA rules |
| `../PROJECT.md` | Stable project identity, purpose, and high-level document entry points |
| `product-constitution.md` | Product mission, vision, principles, and decision criteria |
| `business-operating-system.md` | Business principles, growth, KPI, and monetization direction |
| `conflict-resolution-standard.md` | Interpretation order when current documents conflict |
| `implementation-contract-standard.md` | Required format and boundaries for approved implementation work |

## 6. Professional Standards

### Codex and AI

| Document | Role |
|---|---|
| `codex-guidelines.md` | Supplementary execution judgment; does not repeat or override `AGENTS.md` |
| `ai-development-constitution.md` | Durable AI development principles |
| `ai-decision-framework.md` | Decision sequence and trade-off framework |
| `ai-collaboration-protocol.md` | Human–AI collaboration responsibilities |
| `ai-self-audit.md` | AI self-audit criteria |
| `ai-evaluation-standard.md` | AI output evaluation criteria |

### Design and UX

| Document | Role |
|---|---|
| `design-system.md` | Site-wide visual language, color roles, typography, spacing, images, infographics, responsive design, and design QA |
| `component-library.md` | Reusable component structures and variants |
| `golden-page-template.md` | Detailed page structure and decision flow |
| `page-template-standard.md` | Minimum page skeleton checklist |
| `anti-pattern-standard.md` | Cross-page design and implementation anti-patterns |
| `header.md` | Common header specification |
| `footer.md` | Common footer specification |

### Content and SEO

| Document | Role |
|---|---|
| `content-writing-standard.md` | Brand voice, first-screen answer, fact-versus-judgment separation, writing modules, and content QA |
| `seo-standard.md` | Search intent, representative URL, metadata, canonical, indexing, sitemap, internal links, and structured data |
| `multilingual-seo-strategy.md` | Language URL, hreflang, localization, and multilingual search architecture |
| `knowledge-management.md` | Source registration, review, provenance, and knowledge maintenance |
| `decision-engine.md` | Transparent recommendation rules and inputs |

### Architecture, Quality, and Release

| Document | Role |
|---|---|
| `project-architecture.md` | Technical architecture and boundaries |
| `category-map.md` | Project structure and category map |
| `definition-of-ready.md` | Readiness criteria before implementation |
| `code-review-standard.md` | Code-change review criteria |
| `review-checklist.md` | Release and page review checklist |
| `risk-management.md` | Risk identification and response |
| `measurement-framework.md` | Product and content measurement |
| `release-strategy.md` | Release preparation and publication |
| `evolution-standard.md` | Post-release evolution |
| `lifecycle-management.md` | Asset and document lifecycle states |
| `change-management.md` | Controlled change procedure |

### Data, Assets, and Intelligence

| Document | Role |
|---|---|
| `asset-library.md` | Asset registration and reuse |
| `data-dictionary.md` | Shared data definitions |
| `korea-inside-intelligence.md` | Intelligence-system overview |
| `korea-inside-intelligence/event-schema.md` | Intelligence event schema |
| `korea-inside-intelligence/tracking-plan.md` | Intelligence tracking plan |
| `assets/esim-decision-flow.md` | eSIM infographic specification |
| `assets/payments-comparison.md` | Payments infographic specification |

## 7. Product, Strategy, and Governance References

| Document | Role |
|---|---|
| `project-charter.md` | Project charter |
| `product-vision.md` | Product vision detail |
| `founder-principles.md` | Founder principles |
| `competitive-moat-strategy.md` | Competitive moat strategy |
| `master-playbook.md` | Workflow playbooks by task type |
| `successor-guide.md` | Stable onboarding and context-recovery guide |
| `project-memory.md` | Historical approved decision context |
| `decision-log.md` | Official decision records |

These documents guide decisions but do not authorize repository changes by themselves.

## 8. Page and Domain Specifications

Page-specific documents apply only to their named page or domain:

- `accommodation.md`
- `airport-transfer.md`
- `apps.md`
- `checklist.md`
- `esim.md`
- `payments.md`
- `stay-guide.md`
- `stay-area-database.md`
- `tmoney.md`
- `wowpass.md`
- `home-hero.md`
- `home-journey.md`
- `home-quality-review.md`
- `content-where-to-stay-in-seoul.md`
- `hotel-database.md`
- `hotel-scoring-rules.md`

A page specification is reference material, not permission to start or change that page.

## 9. Project Logs

| Document | Role |
|---|---|
| `project/TODO.md` | Reviewable backlog candidates |
| `project/ROADMAP.md` | Long-term direction and stages |
| `project/IDEAS.md` | Unapproved ideas |
| `project/BUGS.md` | Known bug candidates |
| `project/BUSINESS.md` | Business notes |
| `project/DECISIONS.md` | Project decision summaries |
| `project/CHANGELOG.md` | Change record |
| `project/HISTORY.md` | Historical milestones |

Logs and records do not expand the current approved task.

## 10. Role Boundaries

- `AGENTS.md` governs Codex behavior, approval, scope, Git, protected actions, and QA.
- `codex-guidelines.md` supports practical execution judgment.
- `design-system.md` governs visual language and design QA.
- `content-writing-standard.md` governs editorial voice and content QA.
- `seo-standard.md` governs search architecture and page-level SEO.
- The latest handoff informs current context only when explicitly designated; it never grants execution authority.
- TODO is backlog; ROADMAP is long-term direction.
- Page specifications govern only their named subject.
- Historical records explain the past and never act as current commands.

Do not copy the same rule into multiple documents. Link to the source-of-truth document and keep only the minimum boundary statement needed to avoid misinterpretation.

## 11. Maintenance Rules

- Preserve historical handoffs and dated completion records.
- Do not place a current commit hash, Git state, ahead/behind count, next page, temporary approval status, or completed one-time instruction in a permanent standard.
- Do not create a new standard when an existing source-of-truth document can be improved.
- Update this index when a standard is deliberately added, renamed, moved, deprecated, or archived.
- Any file move, merge, rename, deletion, or documentation architecture change requires explicit approval and a complete diff.
