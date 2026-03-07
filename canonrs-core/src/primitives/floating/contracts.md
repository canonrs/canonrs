# Floating Primitive - Contracts

## Purpose
Calculate and apply positioning for anchored overlay components (Popover, Tooltip, Dropdown, HoverCard).

## Responsibilities
1. Calculate anchor element position via `getBoundingClientRect()`
2. Compute floating element position based on placement
3. Detect viewport collisions
4. Apply flip strategy when needed
5. Write CSS custom properties: `--floating-x`, `--floating-y`, `--floating-placement`

## Non-Responsibilities
- Visual styling (CSS owns this)
- State management (Controller owns this)
- Animation (CSS transitions own this)

## Architecture
```
Anchor Element (trigger)
    ↓ getBoundingClientRect()
use_floating_position(anchor_id, config)
    ↓ calculates position
CSS vars (--floating-x, --floating-y)
    ↓ consumed by
Overlay CSS (transform: translate())
```

## Usage Example
```rust
use_floating_position(
    "trigger-button",
    FloatingConfig {
        placement: Placement::Bottom,
        offset: 8.0,
        flip: true,
    }
);
```

## Canon Rules Applied
- Rule #166: Overlay Positioning Is Component-Owned (CSS vars, not inline styles)
- Rule #27: SSR/CSR Markup Structurally Identical (no DOM manipulation)
- Rule #169: Token Cascade Is Architecture (CSS vars are the interface)
