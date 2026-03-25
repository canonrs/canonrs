# Animate

## When to use
Use to wrap any element with entrance/exit animations. Driven entirely by CSS — no JS animation libraries required.

## Variants
- `FadeIn` / `FadeOut`
- `SlideIn` / `SlideOut`
- `ScaleIn` / `ScaleOut`

## Props
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| animation_type | AnimationType | FadeIn | Animation style |
| duration | Option<String> | None | CSS duration |
| easing | Option<String> | None | CSS easing |
| delay | Option<String> | None | CSS delay |

## Behavior
- Applies `data-rs-animate` and `data-rs-animation` attributes
- Animation triggered via CSS on mount
- No JavaScript animation engine

## Edge cases
- For conditional show/hide use with `data-rs-state` pattern
- Avoid stacking multiple Animate wrappers
