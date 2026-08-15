# Korea Inside Content Writing Standard

## Document Metadata

Layer : L2
Status : Active
Authority Type : Content Writing Standard
Primary Responsibility : Site-wide editorial voice, information-type labeling, writing modules, and content QA
Source of Truth For : Brand voice, first-screen answer structure, fact-versus-judgment separation, page-type writing balance, recommendation wording, affiliate disclosure and CTA placement, and editorial QA
Not Responsible For : SEO metadata and indexing, visual design, affiliate enrollment, revenue performance, affiliate status, administrator classification, implementation approval, Git workflow, current priorities, page-specific copy, reference records, or history
Higher Priority Documents : Current User Instruction, AGENTS.md, PROJECT.md, docs/product-constitution.md
Related Documents : docs/business-operating-system.md, docs/seo-standard.md, docs/design-system.md, docs/golden-page-template.md, docs/decision-engine.md, docs/knowledge-management.md
Change Policy : Owner approval required before change
Last Reviewed : 2026-07-25
Review Trigger : Brand voice change, recurring editorial inconsistency, new page family, or content QA change

## 1. Brand Voice

Korea Inside writes as a practical guide that helps international visitors solve Korean travel problems and make decisions.

- Put judgment and problem-solving before tourism promotion.
- Minimize hype, exclamation, emotional sales language, and advertising copy.
- Give a usable conclusion instead of transferring the entire decision back to the reader.
- Explain why an option fits, where it does not fit, and what exception changes the answer.
- Never describe unexperienced information as personal experience.
- Keep sentences and paragraphs short, direct, and easy to translate.
- Use one consistent name for the same place, product, card, app, or procedure within and across pages.

## 2. First-screen Answer

Answer the searcher's main question before a long introduction.

When relevant, the opening should establish:

- the safest default recommendation
- the strongest practical alternative
- the condition that changes the default
- who the answer suits
- who should avoid it

Do not begin with broad destination description, history, or promotional atmosphere when the user needs a decision.

## 3. Information Types

Keep the following information types distinct:

| Type | Meaning | Writing rule |
|---|---|---|
| Official fact | Published by a responsible authority or provider | Name the source and include the confirmation date for changeable claims. |
| Directly verified fact | Checked by the editor or project through a reliable first-hand method | State what was checked and when without expanding the claim beyond the check. |
| Actual experience | A real, attributable experience | Do not generalize one experience into a universal fact. |
| Editorial judgment | Korea Inside's conclusion based on stated criteria | Show the criteria, advantages, limitations, and exceptions. |
| Reader action | A step the visitor should take | Use direct, short instructions and identify prerequisites or risks. |

Do not blend experience, inference, marketing claims, or community reports into official fact.

## 4. Page-type Writing Modules

Use these proportions as review guides, not exact word-count requirements.

### Procedure and Transport Guides

- Official information and procedure: about 60%
- Experience or editorial judgment: about 30%
- Direct action guidance: about 10%

### Comparison Pages

- Objective comparison: about 45%
- Advantages, disadvantages, and editorial judgment: about 30%
- Traveler-type recommendations: about 20%
- Action guidance: about 5%

### Problem-solving Pages

- Immediate answer and solution: about 40%
- Supporting reason: about 35%
- Exceptions and failure conditions: about 15%
- Checklist: about 10%

Do not force every page family into the same section count or content ratio.

## 5. Common Writing Rules

- State units for prices, fares, time, distance, and operating intervals.
- Mark estimates as estimates.
- For changeable information, provide the official source and confirmation date.
- Publish recommendation criteria.
- Do not invent ratings, rankings, probabilities, reviews, testimonials, or personal experience.
- Present disadvantages and unsuitable users, not only strengths.
- Keep details that affect a real choice; remove repetition, generalities, and encyclopedia-style background that does not.
- Use tables when side-by-side comparison reduces uncertainty.
- Use short steps and checklists for procedures.
- Keep essential information as visible semantic HTML, not image-only text.
- Place affiliate selection and commercial booking prompts after the reader has enough information to decide.
- Do not let commercial CTA copy appear before the decision-support content.

## 6. Recommendation Pattern

A useful recommendation normally answers:

1. What is the default choice?
2. Why does it fit?
3. What is the best alternative?
4. Which constraint changes the answer?
5. What are the main limitations?
6. Who should choose or avoid each option?
7. What should the reader verify or do next?

Use the criteria defined in `decision-engine.md` when the recommendation is rule-based.

## Korea Inside Humanization Standard

This standard applies only to Korea Inside Global English content. It does not govern Korea Inside Japan or Japanese-language localization, which require separate localized standards.

When a task requests "humanization," "humanize," "humanization," "natural copy," removal of AI or mechanical writing, human-sounding sentences, or a natural travel-guide voice, use the rules below together with the existing Brand Voice, Information Types, Common Writing Rules, and Recommendation Pattern.

### Definition

Humanization means converting mechanical, AI-generated, database-like, process-oriented, or recommendation-engine copy into natural editorial travel writing.

Humanization is not synonym replacement.

BAD:

> Choose Gangnam when premium shopping and business are priorities.

ALSO BAD:

> Gangnam is good for travelers who prioritize premium shopping and business.

GOOD:

> Gangnam makes much more sense when meetings, shopping and dinner are already south of the river. Staying nearby avoids repeated cross-city journeys and leaves more of the day for the plans that brought you there.

The GOOD version works because it explains the thought in a human order:

situation → reason → practical consequence.

### Language Humanization

Prefer:

- ordinary and familiar words
- natural sentence rhythm
- concrete travel situations
- clear cause and effect
- practical consequences
- meaningful trade-offs
- smooth transitions between paragraphs

Reduce repetitive analytical wording such as:

- optimize
- prioritize
- evaluate
- verify
- determine
- recommended for
- best suited for
- strongest default
- decision
- alternative
- use case
- travel style
- fit
- criteria

These words are not absolutely banned. The problem is repeated analytical, instructional, or classification-style use.

### Context Humanization

Do not repeatedly write condition → command → recommendation.

Avoid patterns such as:

- "If X, choose Y."
- "If A matters, choose B."
- "For C, choose D."

Prefer natural explanation:

> Jamsil becomes more appealing when several days already revolve around Lotte World, Seoul Sky and southeastern Seoul. If those plans appear only once, a more central hotel usually leaves the rest of the trip easier.

The paragraph should explain why a choice works before telling the reader what conclusion to draw.

### Human Thought Order

Whenever possible, explain information in this order:

1. What situation the traveler is actually in
2. Why the location, service, or choice matters
3. What changes in the real trip
4. The practical trade-off, exception, or conclusion

Do not begin every paragraph by telling the reader what to do. The reader should understand the reason naturally before reaching the recommendation.

### Structural Humanization

Humanization applies to page structure as well as sentences.

Avoid AI-generated repetition such as:

Hero recommendation → Quick Decision cards → second Quick Answer → traveler-style matrix → comparison table → Best for / Watch out → scenarios → Final Recommendation.

One search question should normally receive one clear primary answer. Later sections must deepen that answer with new information instead of repeating the same recommendation in another visual format.

Cards and tables remain useful when they perform the side-by-side comparison described in Common Writing Rules or serve a necessary visual function under `design-system.md`. They are exceptions, not the default structure for restating prose.

### Editorial Tone

The final page should feel like a travel guide edited by a knowledgeable person.

It should not feel like:

- an AI itinerary generator
- a scoring engine
- a database export
- a SaaS dashboard
- a consultant report
- a decision matrix
- an instruction manual

The reader should feel that someone understands the travel situation and is explaining why a choice may or may not work. Keep the tone editorial rather than slang-heavy or overly casual.

The distinctions among fact, direct verification, experience, judgment, and reader action remain governed by Information Types; humanization must not blur them.

### No Fake Humanity

Human writing does not mean fabricated experience. The existing Brand Voice and Common Writing Rules remain controlling.

Never invent statements such as:

- "I stayed here..."
- "We found..."
- "On my last visit..."
- "Our experience was..."

unless the experience was explicitly verified and provided.

Humanization means natural reasoning, vocabulary, rhythm, and context. It does not mean pretending to have personal experience, inventing reviews or ratings, or adding unverified prices, facilities, or other facts.

### Preservation Rules

Humanization must not silently change:

- factual meaning
- primary search intent
- title
- `h1`
- URL
- canonical
- robots
- sitemap status
- existing internal-link destinations
- hotel names
- affiliate URLs
- affiliate IDs
- tracking attributes
- `data-link-stage`
- `rel`
- `target`
- affiliate disclosure
- verified facts
- image `src`
- image `alt`
- structured data, except when an explicitly approved visible-copy change also requires synchronization
- common Header
- Navigation
- Footer
- `common.js`

Any change to these items requires explicit approval. Search intent and SEO elements remain governed by `seo-standard.md`; visual components remain governed by `design-system.md`. This list defines the preservation boundary for humanization rather than replacing those standards.

### Affiliate Preservation Check

Never infer whether affiliate or OTA links exist. Before humanizing a page, inspect the actual current repository HEAD and record the existing count and exact destinations of:

- Expedia links
- Booking.com links
- Agoda links
- other affiliate links
- tracking attributes
- disclosures

After humanization, compare the recorded before-and-after state. The current repository HEAD at the start of the approved task is the source of truth.

Do not restore links from memory or assumptions. Do not remove existing commercial links unless explicitly approved. Affiliate placement and disclosure remain governed by the Affiliate Publication Policy below and `business-operating-system.md`.

### Finalized Copy Rule

When finalized English copy is supplied in the task:

- apply it exactly
- preserve punctuation and intended paragraph structure
- do not simplify it
- do not improve it
- do not shorten it
- do not translate it
- do not replace words because another phrase appears "better"

Codex's role in that case is implementation and verification, not autonomous copywriting.

### Final Humanization QA

Before reporting a humanization task complete, ask:

- Does this sound like a person explaining the trip?
- Does each paragraph add something new?
- Is the same recommendation repeated elsewhere?
- Is the reader being ordered around unnecessarily?
- Did we merely replace one mechanical word with another?
- Could a familiar everyday word replace an analytical expression?
- Does the paragraph explain why before reaching the conclusion?
- Are meaningful trade-offs preserved?
- Does the copy still feel natural when translated?
- Were facts, links, SEO, and affiliate data preserved?
- Was any first-hand experience invented?

If any answer exposes a problem, revise within the approved scope before completion. If the revision would exceed that scope, stop and request approval.

### 운영 정의

Korea Inside에서 “인간화”란 단순한 단어 치환이 아니다.

AI형 반복 구조와 명령형·분류형 문장을 제거하고, 사람이 실제 여행 상황을 설명하는 순서에 따라 일상적으로 사용하는 자연스러운 단어와 문맥으로 문장 전체를 다시 구성하는 것을 의미한다.

기본 흐름:

상황 → 이유 → 실제 여행에서 생기는 변화 → 장단점 또는 결론

## 7. Affiliate Publication Policy

- Place an affiliate CTA only in a context where the reader is making the relevant decision.
- Do not let an affiliate CTA appear before the content needed to make an informed decision.
- Do not use fixed or floating booking CTAs.
- Do not present an affiliate element as an oversized advertising banner.
- Use an editorial card that follows the existing card, spacing, and typography system.
- Do not repeat the same Expedia, Booking.com, and Agoda set across a page.
- Use one primary booking conversion area per page in principle.
- Place the primary booking area where the reader has completed the relevant area, accommodation, or transport decision.
- In the body, allow only a single contextual link or card that directly solves the specific problem being discussed.
- A contextual body link or card must not repeat the same OTA set used in the primary booking area.
- Omit the primary booking area when booking conversion is not central to the page purpose.
- Do not automatically change the placement on an existing approved page. Apply this policy when the user approves a page-specific change.

Affiliate enrollment, revenue performance, status, tracking operations, and user-cost disclosure facts belong to `business-operating-system.md`.

On-screen affiliate disclosure must not hide the possibility of commission. State that a user pays no additional cost only when that program structure has been officially verified.

The affiliate placement examples in `golden-page-template.md` are implementation references subordinate to this standard and `business-operating-system.md`.

## 8. Relationship to Other Standards

- Search intent, title, `h1`, canonical, indexing, sitemap, internal links, and structured data belong to `seo-standard.md`.
- Color, typography, spacing, cards, images, infographics, and responsive visual QA belong to `design-system.md`.
- Detailed page flow belongs to `golden-page-template.md`.
- Source registration and review cycles belong to `knowledge-management.md`.

Do not duplicate those standards here.

## 9. Content QA

Before content approval, verify:

- Does the first screen answer the main question?
- Is the conclusion clear?
- Are facts, direct verification, experience, judgment, and actions distinguishable?
- Are exceptions and unsuitable cases included?
- Are recommendation criteria visible?
- Is repeated or introductory filler removed?
- Do changeable claims have an official source and confirmation date?
- Are units and estimates clearly labeled?
- Is the voice consistent with other Korea Inside pages?
- Are essential statements visible as text?
- Does affiliate wording wait until the reader can make an informed decision?
