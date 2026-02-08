# CanonRS Canon Rules - Immutable Laws

**Version:** 4.0.0 (Feb 2026)  
**Status:** Enforced at compile-time  
**Violations:** Build failure

---

## 🔒 Rule 1: Bin Separation (CRITICAL)

**LAW:** Separate bins for SSR and CSR.
```rust
// src/bin/csr.rs
#![cfg(target_arch = "wasm32")]
fn main() { app::hydrate(); }

// src/bin/ssr.rs
#![cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() { start_server().await; }
```

**WHY:** Prevents SSR code in WASM (143MB → 3MB)  
**VIOLATION:** ❌ WASM bloat, CI failure

---

## 🔒 Rule 2: No Default Features on Leptos

**LAW:** `default-features = false` on ALL leptos deps.
```toml
# ✅ CORRECT
leptos = { version = "0.8", default-features = false }

# ❌ WRONG
leptos = { version = "0.8" }
```

**WHY:** `default` includes `ssr` → leaks to WASM  
**VIOLATION:** ❌ WASM bloat

---

## 🔒 Rule 3: WASM Size Limit

**LAW:** Optimized WASM ≤ 10MB.
```bash
if [ $SIZE -gt 10485760 ]; then exit 1; fi
```

**VIOLATION:** ❌ CI failure

---

## 🔒 Rule 4: Crate Boundaries

| Crate | Purpose | Cannot Import |
|-------|---------|---------------|
| `canonrs-shared` | Types, tokens | web-sys, wasm-bindgen |
| `canonrs-ui` | SSR-safe components | axum, tokio |
| `canonrs-ui-interactive` | CSR wrappers | axum, tokio |
| `canonrs-csr` | WASM behaviors | ❌ Compiles ONLY wasm32 |
| `canonrs` (facade) | Public API | Nothing new |
```rust
// canonrs-csr/src/lib.rs
#[cfg(all(not(target_arch = "wasm32"), not(doc)))]
compile_error!("canonrs-csr is WASM-only");
```

**VIOLATION:** ❌ Compile error (by design)

---

## 🔒 Rule 5: UI vs Interactive Separation

**LAW:** `canonrs-ui` = SSR-safe markup only, `canonrs-ui-interactive` = CSR wrappers with callbacks.

### canonrs-ui (SSR-safe)
```rust
// button_ui.rs
#[component]
pub fn Button(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
) -> impl IntoView {
    view! {
        <button data-button data-variant={variant.as_str()}>
            {children.map(|c| c())}
        </button>
    }
}
```

### canonrs-ui-interactive (CSR-only)
```rust
// button_interactive.rs
use canonrs_ui::ui::button::{Button, ButtonVariant};

#[component]
pub fn ButtonInteractive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(optional)] on_click: Option<Callback<()>>, // ✅ Adds behavior
) -> impl IntoView {
    view! {
        <Button
            variant=variant
            on:click=move |_| {
                if let Some(ref handler) = on_click {
                    handler.run(());
                }
            }
        >
            {children.map(|c| c())}
        </Button>
    }
}
```

**RULES:**
- ✅ Interactive WRAPS UI base, never reimplements
- ✅ Interactive adds callbacks, UI has none
- ✅ Callbacks use DOMAIN types (String, bool), not DOM events (MouseEvent)
- ✅ No state in Interactive (no RwSignal, StoredValue)
- ❌ NEVER `<div on:*>` wrapper around component
- ❌ NEVER import UI base in apps (use Interactive)

**VIOLATION:** ❌ WASM symbol leakage, compile error

---

## 🔒 Rule 6: Import Only the Facade

**LAW:** Apps import `canonrs` facade ONLY.
```rust
// ✅ CORRECT
use canonrs::ui::Button;
use canonrs::interactive::ButtonInteractive;

// ❌ WRONG
use canonrs_ui::ui::Button;
use canonrs_ui_interactive::ButtonInteractive;
```

**VIOLATION:** ❌ Breaks on refactor

---

## 🔒 Rule 7: Leptos 0.8 Patterns

**LAW:** Follow Leptos 0.8 rules strictly.
```rust
// ✅ CORRECT
let value = RwSignal::new(0);
{children.map(|c| c())}
prop=option.unwrap_or_default()

{move || {
    if cond {
        Either::Left(view! { <div>A</div> })
    } else {
        Either::Right(view! { <div>B</div> })
    }
}}

// ❌ WRONG
let (val, set) = signal(0);           // doesn't exist
{children}                            // missing .map()
{move || cond.then(|| view! {...})}   // Option not supported
```

**VIOLATION:** ❌ E0282, E0525, E0308 compile errors

---

## 📁 Where to Put Code

### UI Component (SSR-safe)
**Location:** `canonrs-ui/src/ui/my_component/my_component_ui.rs`
```rust
#[component]
pub fn MyComponent(children: Children) -> impl IntoView {
    view! { <div>{children()}</div> }
}
```

### Interactive Wrapper (CSR-only)
**Location:** `canonrs-ui-interactive/src/ui/my_component/my_component_interactive.rs`
```rust
use canonrs_ui::ui::my_component::MyComponent;

#[component]
pub fn MyComponentInteractive(
    children: Children,
    on_action: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <MyComponent on:click=move |_| {
            if let Some(ref h) = on_action { h.run("data".into()); }
        }>
            {children()}
        </MyComponent>
    }
}
```

### CSR Behavior
**Location:** `canonrs-csr/src/behaviors/my_behavior.rs`
```rust
use web_sys::window;

pub fn init_my_behavior() {
    let window = window().expect("no window");
    // WASM-only code
}
```

### Token
**Location:** `canonrs-shared/src/design/tokens/families/family_x.rs`
```rust
pub const FAMILY_X: &[FamilyToken] = &[
    FamilyToken::new("x-primary", "var(--semantic-primary)"),
];
```

---

## 🚫 Import Prohibitions in WASM
```toml
# ❌ FORBIDDEN in wasm32:
axum = "*"
tokio = { version = "*", features = ["full"] }
tower = "*"
leptos_axum = "*"
```

**WHY:** Pull in thread pools, FS, network → dead code bloat

---

## ✅ Validation
```bash
cd products/canonrs-site
make validate

# Checks: bin separation, size limits, no SSR in WASM
```

---

## 🔥 Common Violations

| Error | Cause | Fix |
|-------|-------|-----|
| WASM 100MB+ | SSR leaked | `cargo tree --target wasm32` |
| E0282 | Wrong pattern | Use `Either::Left/Right` |
| E0525 | FnOnce | Use `StoredValue` or clone |
| IntoRender | Missing .map() | `{children.map(|c| c())}` |

---

**These rules are not suggestions. They are immutable laws.**

**Last Updated:** Feb 2026

---

## 🔒 Rule 8: Workspace Structure (Feb 2026)

**LAW:** CanonRS is organized as a Cargo workspace with specialized crates.
```
rs-canonrs/                      # Workspace root
├── Cargo.toml                   # [workspace] manifest
├── target/                      # Shared build artifacts
├── canonrs/                     # Facade - public API
├── canonrs-shared/              # Pure types, no runtime deps
├── canonrs-ui/                  # SSR-safe UI components
├── canonrs-ui-interactive/      # Interactive wrappers (CSR/hydrate)
├── canonrs-providers/           # Context providers (SSR + hydrate)
└── canonrs-csr/                 # Client-side runtime (WASM-only)
```

**WHY:** 
- Type-safe SSR/CSR separation at compile-time
- Prevents `js-sys` panics in SSR
- Enables independent publishing to crates.io
- Single `/target` directory (no space duplication)

---

## 🔒 Rule 9: Provider SSR Safety (CRITICAL)

**LAW:** Providers must work in SSR without browser APIs.

### ❌ WRONG (panics in SSR):
```rust
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let doc = document(); // 💥 js-sys panic in SSR!
    // ...
}
```

### ✅ CORRECT (SSR-safe):
```rust
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    // Context works in both SSR and CSR
    let mode = create_rw_signal(ThemeMode::Dark);
    provide_context(ThemeContext { mode });
    
    // Effects only run in hydrate
    #[cfg(feature = "hydrate")]
    {
        Effect::new(move |_| {
            let doc = document(); // ✅ Safe: only in hydrate
            // DOM manipulation
        });
    }
    
    children()
}
```

**RULES:**
- ✅ Use `#[cfg(feature = "hydrate")]` NOT `#[cfg(target_arch = "wasm32")]`
- ✅ Context creation works in SSR
- ✅ DOM access ONLY inside `#[cfg(feature = "hydrate")]`
- ✅ Effects use `Effect::new()` inside hydrate guards
- ❌ NEVER call `window()`, `document()` outside guards

**WHY:** Leptos SSR compiles for x86_64, not wasm32. Target arch checks don't protect SSR.

**VIOLATION:** ❌ Runtime panic: "cannot access imported statics on non-wasm targets"

---

## 🔒 Rule 10: Signal Creation in Components

**LAW:** Never create signals outside `#[component]`.

### ❌ WRONG (panics in SSR):
```rust
pub fn example() -> impl IntoView {
    let (state, set) = signal(0); // 💥 No Owner!
    view! { ... }
}
```

### ✅ CORRECT:
```rust
#[component]
fn Example() -> impl IntoView {
    let state = create_rw_signal(0); // ✅ Has Owner
    view! { ... }
}

pub fn example() -> impl IntoView {
    view! { <Example /> }
}
```

**WHY:** Signals require a reactive Owner context, which only exists inside `#[component]`.

**VIOLATION:** ❌ Runtime panic: "spawn_local called outside of LocalSet"

---

## 🔒 Rule 11: Crate Dependencies

**LAW:** Strict dependency hierarchy enforced.
```
canonrs (facade)
├── canonrs-ui
├── canonrs-ui-interactive
├── canonrs-providers
├── canonrs-csr
└── canonrs-shared

canonrs-ui
└── canonrs-shared

canonrs-ui-interactive
├── canonrs-ui
└── canonrs-shared

canonrs-providers
└── (leptos only)

canonrs-csr
└── (web-sys, js-sys only)

canonrs-shared
└── (NO dependencies except std)
```

**PROHIBITED:**
- ❌ canonrs-ui → canonrs-ui-interactive (circular)
- ❌ canonrs-shared → anything (must stay pure)
- ❌ canonrs-providers → web-sys (use cfg guards)

**VIOLATION:** ❌ Compile error: cyclic dependency

---

## 📦 Publishing Order

**LAW:** Publish crates in dependency order.
```bash
# 1. Pure types (no deps)
cargo publish -p canonrs-shared

# 2. UI layer (depends on shared)
cargo publish -p canonrs-ui

# 3. Providers (depends on leptos only)
cargo publish -p canonrs-providers

# 4. Interactive (depends on ui + shared)
cargo publish -p canonrs-ui-interactive

# 5. CSR (WASM-only, no deps on other canonrs crates)
cargo publish -p canonrs-csr

# 6. Facade (depends on all)
cargo publish -p canonrs
```

**VIOLATION:** ❌ Publish fails if dependency not published

---

## 🔍 Workspace Validation
```bash
# Validate entire workspace
cd packages-rust/rs-canonrs
cargo check --workspace

# Test SSR build (no WASM)
cargo build --no-default-features --features=ssr

# Test CSR build (WASM-only)
cargo build --target wasm32-unknown-unknown --features=hydrate
```

---

**Rule 8-11 Status:** ENFORCED (Feb 2026)
**Workspace Migration:** COMPLETE
