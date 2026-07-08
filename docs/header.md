# Common Header

Last updated: July 3, 2026

## Purpose

The common header gives Korea Inside pages a reusable navigation structure while preserving the Go Inside / KR Inside brand relationship.

The header must help visitors move quickly between practical decision areas without turning the site into a general tourism directory.

## Brand Structure

Header brand text is HTML text, not an image:

```text
Go Inside
KR Inside
```

Do not use an `h1` in the header brand area. Each page keeps its only `h1` inside the page hero or main content.

Logo image work is intentionally out of scope for this header update.

## Navigation Order

Use this order for the primary navigation:

1. Home
2. Arrival
3. Transportation
4. Connectivity
5. Money
6. Accommodation
7. Maps
8. Travel Tips

Current home-page link targets:

| Label | Target |
| --- | --- |
| Home | `index.html` |
| Arrival | `arrival.html` |
| Transportation | `airport-transfer.html` |
| Connectivity | `esim.html` |
| Money | `payments.html` |
| Accommodation | `where-to-stay-in-seoul.html` |
| Maps | `maps.html` |
| Travel Tips | `checklist.html` |

If dedicated category hub pages are created later, update these targets without changing the visible order unless there is an approved navigation decision.

## Language Area

The language area is display-only for now.

Current state:

- Visible label: `EN`
- Disabled button state
- Future language expansion must follow the root `AGENTS.md` multilingual target structure: `/en/`, `/ja/`, `/fr/`, `/de/`, `/zh-tw/`.

Future language versions should use dedicated URLs, not query-string switching. Follow root `AGENTS.md` and `docs/multilingual-seo-strategy.md` before enabling language navigation.

The language folders and translated pages do not currently exist in the repository. This document does not approve creating language folders, translated pages, or live header language links without explicit user approval.

## Accessibility Requirements

The common header should include:

- A semantic `nav` element.
- `aria-label="Primary navigation"` on the primary nav.
- A mobile menu button with `aria-expanded`.
- An updated menu button label for open and close states.
- Keyboard focus styles for brand, nav links, language control and menu button.
- Escape key support for closing the mobile menu.

## Scroll Behavior

Home-page hero header behavior:

- Initial state: transparent over the hero image.
- Scrolled state: white background with a light shadow.
- Mobile menu open state: white background with readable dark text.

## Implementation Notes

`common.js` initializes header behavior only when it finds `[data-common-header]`. This allows existing pages to keep their current inline scripts until they are migrated.

Future page migrations should reuse the same header classes and include `common.js` only when the target files, scope and diff are explicitly approved.

This document is a common UI standard only. It does not automatically approve HTML, CSS, JavaScript, navigation or site-wide header changes.

## Mobile Menu QA Record

### July 3, 2026 - Airport mobile menu fix

- Scope: `airport.html`, `common.js`, `style.css`.
- Change: migrate the airport header to the common mobile menu behavior and strengthen mobile menu stacking/scroll behavior.
- Verification level: Level 2 - Feature Verify.
- QA result: Static and DOM-stub verification passed. Confirmed airport header uses `data-common-header`, loads `common.js`, removes the duplicated inline toggle script, opens on toggle click, closes on second click, closes on menu-link click, closes on Escape, and closes when switching to the desktop breakpoint.
- Known QA limitation: In-app browser discovery returned no available browsers, so live 390px click testing, desktop visual confirmation, and browser console inspection could not be completed in this environment.
