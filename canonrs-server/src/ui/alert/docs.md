# Alert

## When to use
Use to communicate status messages, warnings, errors, or confirmations inline within a page.

## Variants
- `Default` — neutral message
- `Info` — informational
- `Success` — positive outcome
- `Warning` — caution required
- `Error` — critical issue

## Props
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| variant | AlertVariant | Default | Visual style |
| class | Option<String> | None | CSS class |
| id | Option<String> | None | HTML id |

## Accessibility
- Uses `role="alert"` for errors, `role="status"` for others
- `aria-live` set automatically based on variant
- Close button has `aria-label="Close alert"`

## Behavior
- Dismissal handled by behavior engine via `data-rs-close`
- No manual event handlers required

## Edge cases
- Do not use for toasts or transient notifications — use Toast instead
- Always include AlertTitle for screen reader context
