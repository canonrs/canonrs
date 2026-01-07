# ThemeToggle

Theme switcher component with Light/Dark/System modes.

## Classification

- **Type:** Type 2 (UI Control + State)
- **Scale:** Human-scale
- **SSR:** ✅ Yes
- **A11y:** ✅ Total
- **Tokens:** Família A (core) + Canônicos
- **State:** Global (ThemeProvider)

## What it IS

- Explicit theme control (`light`, `dark`, `system`)
- Human interface, simple, accessible
- Synced with `<html data-theme="">`
- Integrated with `ThemeProvider`

## What it is NOT

- ❌ Magic button with hidden logic
- ❌ Loose hook
- ❌ Directly coupled to Tailwind
- ❌ Machine-scale component

## Variants

### Icon (Default)
For header/navbar. Cycles: Light → Dark → System → Light
```rust
<ThemeToggle />
```

### Button
For settings/preferences pages.
```rust
<ThemeToggle variant=ThemeToggleVariant::Button />
```

### Dropdown
For detailed configuration.
```rust
<ThemeToggle variant=ThemeToggleVariant::Dropdown />
```

## Sizes
```rust
<ThemeToggle size=ToggleSize::Sm />
<ThemeToggle size=ToggleSize::Md /> // default
<ThemeToggle size=ToggleSize::Lg />
```

## Usage
```rust
use rs_design::{ThemeToggle, ThemeToggleVariant, ToggleSize};

// Header
<ThemeToggle />

// Settings page
<ThemeToggle variant=ThemeToggleVariant::Button size=ToggleSize::Lg />

// Preferences dropdown
<ThemeToggle variant=ThemeToggleVariant::Dropdown />
```

## Accessibility

- `aria-label` changes based on current mode
- Keyboard accessible
- Respects user's system preference
- Works with screen readers

## Canon Compliance

✅ Rule #1 (Types) - Type 2  
✅ Rule #6 (Visual State) - Global state via Provider  
✅ Rule #7 (Token governance) - All tokens applied  
✅ Rule #12 (Specialization) - Icon/Button/Dropdown variants  
✅ Rule #17 (Human-scale) - Simple UX, human-driven  
✅ SSR/Hydration safe

## Architecture

ThemeToggle does NOT manage theme logic. It only:
- Reads current state from `ThemeProvider`
- Emits user intention
- Updates global state

All theme resolution (System → light/dark), persistence (localStorage), and DOM updates happen in `ThemeProvider`.
