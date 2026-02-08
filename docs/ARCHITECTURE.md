# CanonRS Architecture

## Overview

CanonRS is a Cargo workspace containing 6 specialized crates that form an enterprise-grade UI component library for Leptos with SSR support.
```
┌─────────────────────────────────────────────────────────┐
│                    canonrs (facade)                     │
│              Stable public API & boundaries              │
└─────────────────────────────────────────────────────────┘
           │        │         │         │         │
           ▼        ▼         ▼         ▼         ▼
    ┌──────────┐ ┌──────┐ ┌─────────┐ ┌────────┐ ┌─────┐
    │ canonrs- │ │canon-│ │canonrs- │ │canonrs-│ │canon│
    │   ui     │ │ rs-  │ │   ui-   │ │  csr   │ │ rs- │
    │          │ │shared│ │interact.│ │        │ │prov.│
    └──────────┘ └──────┘ └─────────┘ └────────┘ └─────┘
    SSR + CSR   Pure     CSR-focused  CSR-only   SSR+CSR
                Types                             Context
```

---

## Crate Responsibilities

### canonrs (facade)
**Purpose**: Stable public API orchestration

**Exports**:
- `canonrs::ui::*` → SSR-safe components
- `canonrs::ui_interactive::*` → Interactive components (feature-gated)
- `canonrs::providers::*` → Context providers
- `canonrs::layouts::*` → Layout primitives
- `canonrs::blocks::*` → High-level blocks
- `canonrs::shared::*` → Pure types

**Key Decision**: Module `ui_interactive` always exists, content controlled by feature `with-interactive`.

---

### canonrs-shared
**Purpose**: Pure data structures and types

**Contains**:
- Design tokens
- Type definitions
- Enums, traits
- Zero runtime dependencies

**Rules**:
- NO leptos
- NO web-sys
- NO js-sys
- Pure Rust only

**Compiles**: All targets (SSR, CSR, WASM, native)

---

### canonrs-ui
**Purpose**: SSR-safe UI components

**Contains**:
- Primitives (Button, Input, Tree, etc.)
- Layouts (PageHeader, Section, etc.)
- Blocks (Card, Hero, etc.)

**Characteristics**:
- Deterministic HTML rendering
- No callbacks with behavior
- No DOM manipulation
- No client-side state management

**Compiles**: SSR + CSR

**Example**:
```rust
#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
        <button data-button>
            {children()}
        </button>
    }
}
```

---

### canonrs-ui-interactive
**Purpose**: Interactive component wrappers

**Contains**:
- Components with callbacks (`Callback<T>`)
- Client-side state (`RwSignal`, `StoredValue`)
- Event handlers with behavior
- DOM access (guarded by `#[cfg(feature = "hydrate")]`)

**Characteristics**:
- Wraps canonrs-ui components
- Adds interactivity layer
- Domain-typed callbacks (not DOM events)

**Compiles**: SSR + CSR (but content only active in hydrate)

**Key Architectural Decision**:
- Crate compiles in both SSR and CSR builds
- DOM access guarded internally with `#[cfg(feature = "hydrate")]`
- NOT guarded by `#![cfg(target_arch = "wasm32")]` (this would break SSR facade)

**Why this works**:
- Module exists in SSR (import compiles)
- Code only executes in hydrate (runtime safe)
- No stubs needed (Rust cfg does it correctly)

**Example**:
```rust
#[component]
pub fn ButtonInteractive(
    on_click: Callback<()>,
    children: Children,
) -> impl IntoView {
    view! {
        <Button on:click=move |_| on_click.run(())>
            {children()}
        </Button>
    }
}
```

---

### canonrs-providers
**Purpose**: Context providers with SSR/hydrate separation

**Contains**:
- `ThemeProvider`
- `LanguageProvider`
- `DensityProvider`
- `CanonRSRoot` (provider orchestrator)
- Hydration helpers

**Structure**:
```
canonrs-providers/
├── src/
│   ├── theme/
│   │   ├── theme_provider.rs
│   │   └── theme_types.rs
│   ├── root/
│   ├── language/
│   ├── density/
│   ├── hydration/
│   ├── prelude.rs
│   └── lib.rs
└── PROVIDERS_CHARACTERISTICS.md
```

**SSR Safety Pattern**:
```rust
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    // Context creation works in SSR
    let mode = create_rw_signal(ThemeMode::Dark);
    provide_context(ThemeContext { mode });
    
    // DOM access only in hydrate
    #[cfg(feature = "hydrate")]
    {
        Effect::new(move |_| {
            let html = document().document_element().unwrap();
            // DOM manipulation
        });
    }
    
    children()
}
```

**Compiles**: SSR + CSR

**See**: `PROVIDERS_CHARACTERISTICS.md` for provider criteria

---

### canonrs-csr
**Purpose**: Client-side runtime

**Contains**:
- WASM entry points
- Routing logic
- Hydration bootstrap
- Browser-specific behaviors

**Rules**:
- Can use web-sys, js-sys freely
- Never imported by SSR code

**Compiles**: CSR only (`#[cfg(target_arch = "wasm32")]`)

---

## Feature Flags

### `ssr`
**Activates**: Server-side rendering
- Compiles for x86_64 (or host target)
- Includes axum, tokio, tower
- NO WASM dependencies

### `hydrate`
**Activates**: Client-side hydration
- Compiles for wasm32-unknown-unknown
- Activates DOM access in providers
- Activates interactive behaviors

### `with-interactive`
**Controls**: Whether `canonrs::ui_interactive` has content
- In SSR: Module exists, but empty (allows imports)
- In hydrate: Module has all interactive components

**Usage in app**:
```toml
[features]
ssr = [
  "canonrs/ssr",
  "canonrs/with-interactive",  # Allows imports in SSR
]

hydrate = [
  "canonrs/hydrate",
  "canonrs/with-interactive",  # Activates behavior
]
```

### `with-csr`
**Controls**: Whether `canonrs::csr` runtime exists
- Only used in pure CSR apps (no SSR)

---

## Build Targets

### SSR Build
```bash
cargo build --no-default-features --features=ssr
```

**Includes**:
- canonrs-ui (full)
- canonrs-ui-interactive (compiles but DOM guarded)
- canonrs-providers (SSR path)
- canonrs-shared

**Excludes**:
- canonrs-csr (never compiles)

### CSR Build (Hydrate)
```bash
cargo build --target wasm32-unknown-unknown --features=hydrate
```

**Includes**:
- canonrs-ui (full)
- canonrs-ui-interactive (full, behaviors active)
- canonrs-providers (hydrate path)
- canonrs-shared

**Excludes**:
- SSR-only dependencies (axum, tokio)

---

## Critical Architectural Decisions

### Decision 1: Interactive compiles in SSR

**Problem**: If `canonrs-ui-interactive` doesn't compile in SSR, facade can't reexport it, app needs `#[cfg]`.

**Solution**: Crate compiles in both, DOM access guarded internally.

**Trade-off**:
- ✅ App code clean (no cfg)
- ✅ Facade stable
- ⚠️ Interactive symbols present in SSR binary (dead code, eliminated by linker)

**Rationale**: DX and architectural clarity > binary size (which is optimized anyway).

---

### Decision 2: Providers in separate crate

**Problem**: Providers touch DOM, need SSR/hydrate bifurcation, violate UI purity.

**Solution**: Move to `canonrs-providers` with internal guards.

**Benefit**:
- canonrs-ui stays pure (SSR-safe, no side effects)
- Providers explicitly handle runtime differences
- Clear separation of concerns

---

### Decision 3: Facade uses features, not target_arch

**Problem**: Using `#[cfg(target_arch = "wasm32")]` in facade or library crates breaks SSR compilation.

**Solution**: Use `#[cfg(feature = "...")]` everywhere, let Cargo handle targets.

**Rule**: Target decides how code runs, feature decides if code exists.

---

### Decision 4: No stubs, no mocks

**Problem**: Empty stub functions create false contracts.

**Solution**: Module exists, content controlled by feature. If feature not active, module is genuinely empty.

**Benefit**: Fail-fast, honest contracts, no runtime traps.

---

## App Usage Patterns

### Basic SSR-safe component
```rust
use canonrs::ui::Button;

view! {
    <Button>"Click me"</Button>
}
```

### Interactive component (requires hydrate)
```rust
use canonrs::ui_interactive::tree::Tree;

let (nodes, set_nodes) = create_signal(vec![...]);
let on_select = Callback::new(|id| { ... });

view! {
    <Tree nodes=nodes on_select=on_select />
}
```

### Providers
```rust
use canonrs::providers::{CanonRSRoot, ThemeProvider};

view! {
    <CanonRSRoot>
        <ThemeProvider>
            <App />
        </ThemeProvider>
    </CanonRSRoot>
}
```

---

## Dependency Graph
```
canonrs (facade)
├── canonrs-ui
│   └── canonrs-shared
├── canonrs-ui-interactive
│   ├── canonrs-ui
│   └── canonrs-shared
├── canonrs-providers
│   └── leptos
├── canonrs-csr (optional)
│   └── web-sys, js-sys
└── canonrs-shared
```

**Rules**:
- No circular dependencies
- canonrs-shared has zero deps (except std)
- Interactive can depend on UI, never reverse

---

## Validation
```bash
# Validate workspace
cargo check --workspace

# Test SSR build (no WASM)
cargo build --no-default-features --features=ssr

# Test CSR build (WASM-only)
cargo build --target wasm32-unknown-unknown --features=hydrate

# Run app
cd products/canonrs-site
make dev
```

---

## Migration Path

Components mature from Interactive → UI:

1. **Experimental**: Lives in `canonrs-ui-interactive`, CSR-focused
2. **Stabilized**: Extract SSR-safe core, move to `canonrs-ui`
3. **Interactive wrapper**: Keep in `canonrs-ui-interactive`, wraps UI core

**Example**: Tree
- `canonrs-ui::tree::Tree` → SSR-safe, no callbacks
- `canonrs-ui-interactive::tree::Tree` → Wraps UI tree, adds callbacks

---

## Summary

**Crates**: 6 specialized
**Targets**: SSR (x86_64) + CSR (wasm32)
**Features**: Control runtime boundaries
**Facade**: Stable contracts, no stubs
**DX**: Clean app code, no cfg guards
**Safety**: SSR never panics on JS APIs

This architecture is production-ready, enterprise-grade, and maintainable.
