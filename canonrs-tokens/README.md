# CanonRS Tokens

Pure design token library with zero dependencies and built-in CSS generation.

## Structure
```
canonrs-tokens/
├── src/                     # Library (zero dependencies)
│   ├── lib.rs
│   └── design/
│       └── tokens/
│           ├── core.rs
│           ├── primitives.rs
│           ├── semantics.rs
│           └── families/    # 11 token families
│               ├── family_a_overlay.rs
│               ├── family_b_selection.rs
│               ├── family_c_forms.rs
│               └── ...
└── bin/                     # CLI tooling
    ├── tokens-engine.rs     # Main orchestrator
    ├── theme_generator.rs   # Theme processing
    ├── theme_mapping.rs     # Semantic mappings
    ├── entry_generator.rs   # canonrs.css builder
    └── bundler.rs           # CSS bundler
```

## Usage

### As Library
```rust
use canonrs_tokens::design::tokens::families::*;

// Access token families
let overlay_tokens = FAMILY_A_OVERLAY;
```

### As CLI Tool
```bash
# Generate all CSS
cargo run --bin tokens-engine

# Build only the library
cargo build --lib
```

## Token Families

1. **family-a-overlay** - Modals, popovers, tooltips
2. **family-b-selection** - Dropdowns, combobox, menus
3. **family-c-forms** - Inputs, textarea, checkbox, radio
4. **family-d-navigation** - Tabs, breadcrumbs, pagination
5. **family-e-feedback** - Alerts, toasts, badges, banners
6. **family-f-data** - Tables, cards, avatars, stats
7. **family-g-composite** - Accordion, carousel, tree
8. **family-h-layout** - Container, grid, spacing
9. **family-i-animation** - Transitions, keyframes
10. **family-s-state** - Hover, focus, active, disabled
11. **family-z-layers** - Z-index hierarchy

## See Also

- [TOKENS_FLOW.md](./TOKENS_FLOW.md) - Step-by-step generation flow
- [Workspace README](../README.md) - Full CanonRS architecture
