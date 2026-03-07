# canonrs-client

WASM and hydration layer of the CanonRS design system.

Only compiled when targeting `wasm32` or using the `hydrate` feature. Never included in SSR builds.

## Responsibility

Provides interactive components, browser behaviors, and client-side theme management. All code in this crate assumes it runs in a browser environment.

## Modules

### `behaviors/`
Browser-native behavior implementations for all interactive components. Each behavior file wires a component's DOM interactions using `web-sys`.

Examples:
- `accordion_behavior` — expand/collapse with keyboard support
- `drag_drop_behavior` — drag and drop primitives
- `sidebar_behavior` — responsive sidebar toggle
- `virtual_list_behavior/` — windowed list rendering
- `theme_toggle_behavior` — client-side theme switching

The `auto_init.rs` and `behavior_registry.rs` handle automatic behavior registration on hydration. Called via `canonrs::behaviors::init_canonrs_behaviors()` in the hydrate entry point.

### `ui/`
Interactive UI component implementations that require browser APIs:
- `avatar` — image loading with fallback
- `button` — interactive button with ripple/state
- `calendar/` — date picker with keyboard navigation
- `command/` — command palette
- `data_table/` — sortable, filterable data table
- `sidebar/` — responsive sidebar
- `toast/` — notification toasts

### `themes/`
Client-side theme engine:
- `engine` — applies theme tokens to CSS variables at runtime
- `registry` — theme registry for dynamic theme switching

## Features

| Feature | Effect |
|---------|--------|
| `hydrate` | Activates Leptos hydration mode |

## Target Restriction

`behaviors/` is only compiled on `wasm32`:
```rust
#[cfg(target_arch = "wasm32")]
pub mod behaviors;
```
