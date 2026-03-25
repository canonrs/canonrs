# Accordion

## When to use
Use to show/hide sections of related content. Ideal for FAQs, settings panels, and navigation menus.

## Variants
- `Single` — only one item open at a time (default)
- `Multiple` — multiple items can be open simultaneously

## Props
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| selection | AccordionSelection | Single | Open mode |
| collapsible | bool | true | Allow closing open item |
| class | String | "" | CSS class |
| id | Option<String> | None | HTML id |

## Accessibility
- Trigger uses `<button>` with `aria-expanded`
- Content uses `aria-hidden` when closed
- Keyboard: Enter/Space to toggle, Arrow keys to navigate

## Behavior
- State managed by behavior engine via `data-rs-accordion`
- No JavaScript required on SSR render
- Animations via CSS transitions on `data-rs-state`

## Edge cases
- `collapsible=false` prevents closing the last open item
- Nested accordions are not supported
