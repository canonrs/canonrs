# Canon Rule #96: SSR Requires Explicit Provider Tree

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** providers, leptos, ssr
**Version:** 1.0.0  
**Date:** 2025-01-15

---

## Principle

**Leptos SSR applications MUST explicitly wrap the component tree with all required Providers in the App component.**

SSR cannot infer or auto-inject context providers. Every provider that child components depend on must be explicitly present in the tree, or the server will panic with "Context not found" errors.

---

## The Problem

When components use contexts (Theme, Density, etc.) but the App component doesn't provide them:

- Server panics: "ThemeContext not found. Make sure ThemeProvider wraps your app."
- Error occurs at first render, not at build time
- Browser shows ERR_EMPTY_RESPONSE
- No indication which provider is missing until you read panic message

### Real Symptoms
```
thread 'tokio-runtime-worker' panicked at rs-design/src/providers/theme_types.rs:25:10:
ThemeContext not found. Make sure ThemeProvider wraps your app.
```

Or:
```
thread 'tokio-runtime-worker' panicked at rs-design/src/providers/density_types.rs:50:10:
DensityContext not found. Make sure DensityProvider is in the component tree.
```

---

## Anti-Pattern (FORBIDDEN)
```rust
// ❌ FORBIDDEN: No providers
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Title text="My App"/>
        <Router>
            <Routes>
                <Route path="" view=HomePage/>
            </Routes>
        </Router>
    }
}

// HomePage uses ThemeContext but it's not provided
#[component]
fn HomePage() -> impl IntoView {
    let theme = use_theme(); // ← PANIC: ThemeContext not found
    view! { <div class=theme.class>"Hello"</div> }
}
```

This fails because:
- SSR renders top-down
- Contexts must exist before children access them
- No auto-wiring of providers in SSR

---

## Canonical Pattern (REQUIRED)
```rust
// ✅ REQUIRED: Explicit provider tree
use rs_design::providers::{ThemeProvider, DensityProvider};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="My App"/>
        <ThemeProvider>
            <DensityProvider>
                <Router>
                    <Routes>
                        <Route path="" view=HomePage/>
                    </Routes>
                </Router>
            </DensityProvider>
        </ThemeProvider>
    }
}

// Now HomePage can safely use contexts
#[component]
fn HomePage() -> impl IntoView {
    let theme = use_theme(); // ✅ Works
    let density = use_density(); // ✅ Works
    view! { <div>"Hello"</div> }
}
```

---

## Provider Ordering Rules

Providers MUST be nested in dependency order:
```rust
// ✅ CORRECT: Dependencies inside dependents
<ThemeProvider>           // ← Outer: no dependencies
    <DensityProvider>     // ← Uses theme
        <Router>          // ← Uses both
            <App/>
        </Router>
    </DensityProvider>
</ThemeProvider>

// ❌ WRONG: Dependent outside dependency
<DensityProvider>         // ← Tries to use theme
    <ThemeProvider>       // ← Not available yet
        <Router>
```

Common provider order for CanonRS apps:

1. `ThemeProvider` (outermost - no dependencies)
2. `DensityProvider` (depends on theme)
3. Domain providers (SelectionProvider, DragDropProvider, etc.)
4. `Router` (innermost - uses all contexts)

---

## Why SSR Cannot Auto-Provide

CSR differences that don't apply to SSR:

| CSR Behavior | SSR Behavior |
|--------------|--------------|
| Can mount providers lazily | Must exist before render |
| Can inject via root mount | No injection point |
| Runtime provider discovery | Compile-time tree only |

SSR is static tree generation. There is no "mount point" where providers can be injected after the fact.

---

## Common Providers in CanonRS
```rust
// Standard CanonRS provider stack
use rs_design::providers::{
    ThemeProvider,
    DensityProvider,
};
use rs_design::ui::selection::SelectionProvider;
use rs_design::ui::drag_drop::{DragDropProvider, DragDropCallbacksProvider};
use rs_design::SidebarProvider;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <ThemeProvider>
            <DensityProvider>
                <DragDropProvider>
                    <DragDropCallbacksProvider>
                        <SelectionProvider<String> mode=SelectionMode::Multiple>
                            <SidebarProvider>
                                <Router>
                                    <AppRoutes/>
                                </Router>
                            </SidebarProvider>
                        </SelectionProvider<String>>
                    </DragDropCallbacksProvider>
                </DragDropProvider>
            </DensityProvider>
        </ThemeProvider>
    }
}
```

---

## Diagnostic Checklist

If you see "Context not found" panic:
```bash
# 1. Identify missing provider from panic message
# Example: "ThemeContext not found" → need ThemeProvider

# 2. Check App component structure
grep -A 20 "pub fn App" src/lib.rs

# 3. Verify provider is imported
grep "ThemeProvider" src/lib.rs

# 4. Check provider wraps Router
# Provider must be OUTSIDE Router, not inside

# 5. Verify provider order (dependencies first)
```

---

## Enforcement

- All SSR apps MUST wrap Router with required providers
- Providers MUST be ordered by dependency (no deps first)
- New providers MUST be documented with their dependencies
- CI MUST test SSR render to catch missing providers

---

## Canonical Justification

> **SSR is static tree generation with no runtime injection.**  
> Contexts are compile-time tree structure, not runtime services.  
> Providers must be explicit because SSR has no mounting phase.

This rule exists to:
- Make provider dependencies explicit and auditable
- Prevent "works in dev, breaks in prod" SSR issues
- Encode SSR's static nature as governance
- Eliminate context discovery debugging cycles

---

## Related Canon Rules

- Canon Rule #94 — Leptos Workspace Features Must Be Explicit
- Canon Rule #95 — SSR Requires Complete HTML Shell
- Canon Rule #99 — CSR Islands Must Not Own Routing or Providers

---

## Version History

- **1.0.0** (2025-01-15): Initial rule based on Workbench migration provider panic debugging
