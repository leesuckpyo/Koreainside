# KR Inside Design System Audit

Last updated: 2026-07-02

## Purpose

This document audits the current KR Inside HTML/CSS component patterns and records how future pages should reuse them.

This is documentation only. It does not introduce new HTML, CSS, JavaScript, assets, routes, or visual changes.

## Scope

- Checked project-wide HTML pages and `style.css`.
- Compared current repeated patterns with existing documentation in `docs/component-library.md`, `docs/golden-page-template.md`, `docs/header.md`, and `docs/footer.md`.
- Focused on reusable structure, duplicated component families, and practical design-system consistency.

## Hybrid Recognition UX Direction

KR Inside should not become Western-style only. The design direction is a hybrid of global usability and Korean travel recognition.

| Layer | What it means | Design implication |
|---|---|---|
| Global UX | Clean layout, readable structure, mobile-first rhythm, simple cards, clear headings, and fast scanning. | Keep the current KR Inside spacing, typography, table, card, and section patterns. Do not add visual clutter or decorative complexity. |
| Korean Experience | Real objects, real app screens, rich practical details, and visual recognition of how Korea works. | Use real or official/reference images for practical objects so visitors can recognize what they will see in Korea. |

Major practical sections should create an "아~ 이거구나" moment: the user should instantly recognize the object, place, screen, or action before reading a long explanation.

Examples:

- This is a T-money card.
- This is a WOWPASS kiosk.
- This is Korean money.
- This is Hongdae street.
- This is how Naver Map works.

## Image-As-Information Standard

Images are information, not decoration. Every important image must have a defined role before it is added.

| Image role | Purpose | Examples |
|---|---|---|
| Recognition image | Helps the user identify a real object or place. | T-money card, WOWPASS card, WOWPASS kiosk, Korean banknotes, Naver Map app screen. |
| Usage image | Shows how something is used in context. | Card tapping, kiosk charging, app balance screen, subway gate use. |
| Value image | Helps the user understand scale, price, budget, or money feeling. | Korean banknotes with approximate foreign-currency values. |
| Decision image | Helps the user choose between practical options. | T-money card beside WOWPASS card with short fit guidance. |
| Location image | Helps the user recognize a district, station, airport area, or pickup point. | Hongdae street, Myeongdong street, airport arrival area, kiosk location context. |

Real objects should use actual, official, or clearly referenced images whenever available. This applies to WOWPASS cards, WOWPASS kiosks, Korean banknotes, app screens, and T-money cards.

AI-generated images may be used for mood or place context when a real-object image is not required. Acceptable uses include Hongdae, Myeongdong, Gangnam, Ikseon-dong, airport atmosphere, and general neighborhood mood. Do not use AI images to represent official cards, kiosks, banknotes, app screens, maps, or payment interfaces.

For browser translation and accessibility, important information in an image must also appear as visible semantic HTML near the image. Do not rely on image text, `alt`, `title`, or `figcaption` alone for essential instructions, labels, warnings, comparisons, or values.

## Show First, Explain Later

Practical Korea guides should prefer this order inside major sections:

1. Image or visual recognition point.
2. Short meaning.
3. Practical explanation.
4. Next action or decision guidance.

Avoid long text before the user understands what they are looking at. Avoid manual-like writing unless the section is explicitly a procedure. Short looping demonstrations can be useful, but they should make the user feel "아~ 이렇게 사용하는구나." They are not full tutorials and should not become long step-by-step animations.

## Comparison Limits

Comparisons are useful only when they help the visitor choose. Do not turn guide pages into broad VS pages by default.

Use comparison when:

- Two options are commonly confused.
- The visitor must choose one before travel or at arrival.
- A side-by-side view reduces mistakes.

Keep comparisons compact. Prefer short decision guidance, fit/avoid notes, and one practical table over repeated long comparison sections.

## Reference Pages

Current highest-quality visual and structural references:

| Reference | Why it matters |
|---|---|
| `where-to-stay-in-seoul.html` | Strong decision-guide structure, FAQ schema, area comparison, and stay-card patterns. |
| `accommodation.html` | Recently aligned accommodation decision page using hero, quick answer, decision cards, warning box, checklist, comparison table, FAQ, and breadcrumb schema. |
| `maps.html` | Strong app-comparison layout with visual assets, maps-specific cards, checklist, and compact tables. |
| `payments.html` | Practical guide layout with quick answer, payment decision cards, warning/tip boxes, FAQ, and comparison-oriented sections. |
| `tmoney.html` | Practical transport-card topic that should use real-card recognition and short usage guidance. |
| `wowpass.html` | Existing WOWPASS card, machine, and use-flow imagery; future direction should shift from product comparison to traveler money-management guidance. |
| `index.html` | Home hero, common header behavior, common footer, core CTA button styles, and homepage entry-card pattern. |

## Component Inventory

| Item | Current usage pages | Reusable? | Standard component? | Improvements |
|---|---|---|---|---|
| Header | All main HTML pages use a semantic `<header>` and the `.header` family. `index.html` and `esim.html` also use `data-common-header` with `common.js`. | Yes | Yes, but implementation is not fully centralized. | Keep the existing `.header`, `.header__inner`, `.brand`, `.nav`, `.nav-toggle`, and `.language-switcher` classes. Future migrations should align active-link state, ARIA labels, and `data-common-header` behavior page by page after approval. |
| Footer | All main HTML pages use a semantic `<footer>` and the `.footer` family. | Yes | Yes | Continue using the documented footer structure in `docs/footer.md`. Keep footer brand text as visible HTML. Standardize future footer link groups without changing existing navigation unless approved. |
| Hero | All major pages use a hero pattern. Variants include `hero-banner`, `page-hero`, `airport-page-hero`, `maps-hero`, `esim-hero`, and page-specific stay/payment hero patterns. | Yes, by page type | Partially | Define when to use each approved variant: homepage media hero, standard guide hero, two-column decision hero, app/media hero, and stay-cluster hero. Avoid creating another hero family unless no existing variant fits. |
| Quick Answer | Used on decision pages such as `accommodation.html`, `apps.html`, `payments.html`, `where-to-stay-in-seoul.html`, `airport-bus.html`, `arex.html`, `rental-car.html`, `taxi.html`, `esim.html`, and several Seoul area pages. | Yes | Partially | Reuse the current near-top answer section pattern. Standardize labels such as `Quick answer`, `Best for`, `Choose this if`, and `Watch out` as visible HTML text. |
| Comparison Table | Used across `accommodation.html`, `apps.html`, `payments.html`, `maps.html`, `esim.html`, `tmoney.html`, `wowpass.html`, `airport.html`, `checklist.html`, `where-to-stay-in-seoul.html`, and Seoul area pages. CSS includes `.info-table`, `.table-scroll`, `.esim-table`, and stay/payment table variants. | Yes | Yes, with variants | Prefer `.table-scroll` plus semantic `<table>` for comparisons. Use `<th scope>` where possible. Avoid horizontal overflow on 360px by testing table wrappers. |
| Decision Cards | Used in `accommodation.html`, `payments.html`, `maps.html`, `airport.html`, `arrival.html`, `stay-guide.html`, `where-to-stay-in-seoul.html`, `wowpass.html`, and Seoul area pages. Class families include `.airport-action-card`, `.stay-decision-card`, `.payments-card`, `.maps-stack-card`, and `guide-card`. | Yes | Partially | Consolidate future card usage around existing card families before adding new CSS. Cards should include option name, best-fit user, main advantage, and limitation. |
| Warning Box | Used in `accommodation.html`, `apps.html`, `checklist.html`, `payments.html`, and `wowpass.html`. CSS includes `.warning-box`. | Yes | Yes | Reserve warning boxes for risk, limitation, or mistake-prevention content. Keep labels visible as text and avoid using color alone to communicate risk. |
| Tip Box | Used in `airport-bus.html`, `apps.html`, `arex.html`, `checklist.html`, `payments.html`, `rental-car.html`, `taxi.html`, and `tmoney.html`. CSS includes `.tip-box`. | Yes | Yes | Use tip boxes for practical non-critical advice. Do not mix warnings and tips in the same component. |
| Checklist | Class-based checklist UI appears in `accommodation.html`, `checklist.html`, and `maps.html`; checklist-like lists appear in many guide pages. CSS includes `.checklist`, `.checklist__item`, `.checklist__checkbox`, and checked state styles. | Yes | Yes | Reuse `.checklist` for action-oriented booking/setup checks. Keep each item short and scannable. If interactive checked states are expanded later, document JS behavior before implementation. |
| FAQ | FAQ sections appear on most major guide pages, including accommodation, airport, transport, apps, eSIM, maps, payments, taxi, T-money, WOWPASS, stay guide, and Seoul area pages. Variants include `.airport-faq`, `.payments-faq`, `.esim-faq`, and `<details>`. | Yes | Yes, with naming variants | Keep visible FAQ HTML aligned with FAQ JSON-LD. Future work should standardize FAQ wrapper naming while preserving existing page styles. |
| CTA Buttons | Most pages use `.btn`, `.btn--primary`, `.btn--ghost`, `.btn--text`, `airport-pill`, `chip`, or page-specific CTA rows. | Yes | Partially | Use `.btn` for main actions, pills for in-page navigation, and chips for compact tags/links. Avoid inventing new button visuals unless the action hierarchy cannot be expressed with existing styles. |
| Breadcrumb | Major guide pages use `.page-hero__breadcrumb`; `esim.html` has `.esim-breadcrumb`; structured data commonly uses `BreadcrumbList`. | Yes | Yes, with one variant exception | Keep visual breadcrumb near the hero and maintain matching Breadcrumb JSON-LD where SEO value exists. Consider aligning `esim-breadcrumb` naming in a future approved cleanup. |
| Section spacing | Global `.section`, `.section__header`, `.section__title`, `.section__subtitle` exist. Page families also use `.airport-section`, `.esim-section`, payment/stay/map sections, and gray modifiers. | Yes | Partially | Use existing section families by page type. Avoid new section spacing rules unless the page cannot be expressed with `.section` or the approved page-family section class. |
| Recognition media | Used where real objects or app screens help the user understand quickly, including maps, eSIM, T-money, and WOWPASS assets. | Yes | Partially | Use real-object images with visible HTML meaning and a clear image role. Avoid decorative image drops. Prefer "show first, explain later" for object-heavy Korea guides. |
| Card spacing | Cards across home, airport, maps, payments, stay, WOWPASS, and KII dashboard share padding, border, radius, shadow, and hover patterns but use different class names. | Yes | Partially | Prefer existing card classes inside the same page family. Future CSS cleanup could introduce shared card tokens or utility classes, but only after impact review. |
| Color palette | `style.css` defines core tokens: `--blue-deep`, `--blue-accent`, `--blue-light`, `--blue-muted`, gray scale, `--white`, and maps/card shadow tokens. | Yes | Yes | Continue using root variables. Avoid one-off colors unless tied to an approved visual need. Where custom colors exist, map them to existing tokens during future cleanup. |
| Typography | Global type uses `--font-sans` with Inter/system fallback. Section headings, hero titles, eyebrows, and cards use consistent weights and sizes with some page-specific clamp values. | Yes | Yes | Keep visible text concise on mobile. Avoid new font families. Reduce page-specific heading scale only when text fitting or hierarchy requires it. |
| Border radius | Root tokens include `--radius-sm` 8px, `--radius-md` 12px, `--radius-lg` 16px, and `--radius-xl` 20px. Pills use `999px`; a few one-off values exist. | Yes | Yes | Use radius tokens for cards, panels, tables, and forms. Reserve `999px` for pills/chips only. Avoid new arbitrary radius values. |
| Shadow | Root tokens include `--shadow-sm`, `--shadow-md`, `--shadow-lg`; maps also uses `--maps-card-shadow`. | Yes | Yes | Use token shadows for cards and elevated panels. Keep shadows subtle. Future cleanup can reduce custom box-shadow values after visual regression checks. |
| Responsive breakpoints | Current CSS uses breakpoints around 480, 540, 640, 720, 768, 860, 900, 960, 1024, and 1120px. Project QA expects 360/375, 768, and 1440px checks. | Yes | Partially | Keep existing breakpoints for current pages. For new work, prefer existing breakpoint families and verify at 360px, 768px, and 1440px. Consolidation should be a separate approved CSS cleanup. |

## Duplicate Component Report

| Duplicate area | Current duplicated forms | Impact | Recommended direction |
|---|---|---|---|
| Card shells | `.airport-action-card`, `.stay-decision-card`, `.payments-card`, `.maps-stack-card`, `.guide-card`, home entry cards, WOWPASS cards | Similar visual behavior is repeated with page-specific names. This is maintainable now but can grow CSS duplication. | Reuse the nearest page-family card first. If a new shared card is ever created, document it and migrate only with page-by-page approval. |
| FAQ wrappers | `.airport-faq`, `.payments-faq`, `.esim-faq`, page-specific `<details>` styling | FAQ UX is consistent, but CSS naming differs by page family. | Keep existing wrappers. Future standard could be `.faq-list` plus page-family modifiers after approval. |
| Hero variants | `hero-banner`, `page-hero`, `airport-page-hero`, `maps-hero`, `esim-hero`, stay/payment-specific heroes | The project has several strong hero patterns, but new pages may accidentally create another variant. | Choose an existing hero by content type before adding CSS. Document the chosen reference page in each page doc. |
| Table wrappers | `.info-table`, `.table-scroll`, `.esim-table`, stay tables, payment tables, maps compact tables | Comparison content is strong, but mobile overflow rules may vary. | Use semantic tables with wrappers. Treat no-horizontal-scroll QA as required for every table-heavy page. |
| CTA patterns | `.btn`, `.btn--primary`, `.btn--ghost`, `.btn--text`, `.airport-pill`, `.chip`, page-specific action rows | Button hierarchy can blur if pills, chips, and buttons are used interchangeably. | Use buttons for commands/primary actions, pills for in-page navigation, and chips for compact filters/tags/related links. |
| Header markup | Header HTML repeats across pages; only `index.html` and `esim.html` currently use `data-common-header` with `common.js`. | Repeated static header is stable but harder to update globally. | Do not centralize automatically. Future migration should be explicit, page-scoped, and tested for menu behavior. |
| Footer markup | Footer HTML repeats across pages. | Stable and SEO-safe, but global footer changes require many page edits. | Keep current footer. Use `docs/footer.md` as source for future manual alignment. |
| Warning/tip labels | `.warning-box` and `.tip-box` exist, but some pages also use page-specific caution or note copy. | Message severity may become inconsistent. | Prefer standard `.warning-box` for risks and `.tip-box` for helpful notes. |

## Reuse Policy For Future Work

Before creating any new section or component:

1. Search existing HTML and `style.css` for a similar component.
2. Reuse the existing component if it fits the page purpose.
3. Extend an existing page-family component if the structure is close.
4. Create a new component only when no suitable existing pattern exists.
5. If a reusable component is created, document where it should be reused and which pages may benefit later.

## Current Standard Patterns

| Pattern | Preferred existing source |
|---|---|
| Common header | `docs/header.md`, `index.html`, `esim.html`, `.header` CSS family |
| Common footer | `docs/footer.md`, `.footer` CSS family |
| Decision-guide hero | `accommodation.html`, `where-to-stay-in-seoul.html`, `.airport-page-hero` and stay hero patterns |
| Practical quick answer | `accommodation.html`, `payments.html`, `apps.html` |
| Comparison table | `.table-scroll`, `.info-table`, `accommodation.html`, `payments.html`, `maps.html` |
| Decision cards | `accommodation.html`, `where-to-stay-in-seoul.html`, `payments.html`, `maps.html` |
| Warning/tip boxes | `.warning-box`, `.tip-box` |
| Checklist | `.checklist` in `checklist.html`, `accommodation.html`, `maps.html` |
| FAQ | `.airport-faq`, `.payments-faq`, `.esim-faq`, semantic `<details>` |
| CTA hierarchy | `.btn`, `.btn--primary`, `.btn--ghost`, `.btn--text`, `.airport-pill`, `.chip` |
| Recognition image section | Real object or app screenshot, short meaning, practical explanation, next action |

## Maintenance Notes

- This audit did not modify HTML, CSS, JavaScript, images, or existing documentation.
- `style.css` already has a usable token layer for colors, radius, shadow, typography, and layout width.
- The biggest duplication risk is not visual inconsistency today; it is future growth of page-specific cards, FAQ wrappers, and hero variants.
- Any future CSS consolidation should be handled as a separate approved task because shared CSS can affect many production pages.
- Future object-heavy pages should be reviewed against the hybrid recognition UX direction before adding new sections, images, or animations.
