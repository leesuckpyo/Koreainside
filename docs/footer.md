# Common Footer

Last updated: July 1, 2026

## Purpose

The common footer identifies KR Inside as part of the Go Inside platform and gives visitors stable access to practical guide categories.

The footer should support trust, navigation and future multilingual expansion without relying on logo images.

## Brand Area

Use HTML text for the footer brand:

```text
Go Inside
KR Inside
Don't just visit. Go Inside.
```

Do not place the brand text only inside an image.

## Link Groups

Use three groups:

### Guides

| Label | Current target |
| --- | --- |
| Arrival | `arrival.html` |
| eSIM | `esim.html` |
| T-money | `tmoney.html` |
| WOWPASS | `wowpass.html` |
| Maps | `maps.html` |
| Accommodation | `accommodation.html` |

`accommodation.html` currently forwards visitors to the existing Seoul accommodation decision guide. Replace it with a full accommodation hub when the category expands beyond Seoul.

### Platform

| Label | Current state |
| --- | --- |
| About | Display-only until an About page exists. |
| Travel Tips | `checklist.html` |
| Contact | Display-only until a Contact page exists. |

### Language

| Label | Current state |
| --- | --- |
| English | Current root English pages; future `/en/` only after an approved multilingual implementation. |
| Japanese | Display-only until `/ja/` exists through explicit approval. |
| French | Display-only until `/fr/` exists through explicit approval. |
| German | Display-only until `/de/` exists through explicit approval. |
| Traditional Chinese | Display-only until `/zh-tw/` exists through explicit approval. |

Language expansion must follow the root `AGENTS.md` multilingual target structure. The language folders and translated pages do not currently exist in the repository, and this document does not approve creating them.

Do not add Korean, Vietnamese or other footer language links unless root `AGENTS.md` or a separate approved multilingual decision changes the target language structure.

## Bottom Notice

Use this text:

```text
&copy; 2026 Go Inside. KR Inside is part of the Go Inside platform.
Information is provided for travel planning purposes and should be checked with official sources when necessary.
```

## Design Requirements

- Navy background.
- White text with teal accent.
- Mobile layout is one column.
- Desktop layout is three to four columns.
- Avoid decorative images and heavy visual effects.
- Keep focus styles visible for links.

## Accessibility Requirements

- Use a semantic `footer` element.
- Use `nav` for footer navigation groups with clear `aria-label` values.
- Link text must be descriptive.
- Display-only future links should not be fake clickable links.
- Maintain readable color contrast.

## Implementation Scope

This document is a common UI standard only. It does not automatically approve HTML, CSS, JavaScript, navigation or site-wide footer changes.

Common footer changes affect multiple pages. Before live implementation, confirm the target files, affected scope and diff, then wait for explicit user approval.
