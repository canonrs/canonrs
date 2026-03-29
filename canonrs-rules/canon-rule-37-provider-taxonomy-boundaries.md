# Canon Rule #37: Provider Taxonomy & Boundaries

**Status:** ENFORCED

**Severity:** HIGH
**Scope:** providers, architecture
**Version:** 1.0.0
**Date:** 2025-01-16


## Principle
The design system defines **exactly three canonical providers**: Theme, Density, and Language. Each provider is **orthogonal**, **stateless** (on the design system layer), and **SSR-safe**. Persistence, business logic, and HTTP concerns belong exclusively to the app layer.

## The Three Canonical Providers

### 1️⃣ ThemeProvider

**Responsibility:**
- Apply `data-theme` attribute to `<html>`
- Apply `class="light|dark"` based on resolved mode
- Expose `ThemeContext { mode, preset }`

**Does NOT:**
- ❌ Persist to cookies/localStorage
- ❌ Call server functions
- ❌ Make HTTP requests
- ❌ Know about user preferences

**Location:** `packages-rust/rs-design/src/providers/theme_provider.rs`

**Props:**
```rust
#[component]
pub fn ThemeProvider(
    #[prop(optional)] initial_mode: Option<String>,
    #[prop(optional)] initial_preset: Option<String>,
    children: Children,
) -> impl IntoView
```

**Context:**
```rust
#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub mode: RwSignal<ThemeMode>,
    pub preset: RwSignal<String>,
}

pub fn use_theme() -> ThemeContext;
```

**DOM Effects:**
```html
<html class="dark" data-theme="ocean">
```















































































