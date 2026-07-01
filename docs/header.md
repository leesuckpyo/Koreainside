# Common Header

Last updated: July 1, 2026

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
- Planned language expansion: EN, JP, KR, TW, VN

Future language versions should use dedicated URLs, not query-string switching. Follow `docs/multilingual-seo-strategy.md` before enabling language navigation.

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

Future page migrations should reuse the same header classes and include `common.js`.
