# Canon Rule #67: Leptos CSR Does NOT Load CSS via `<Stylesheet />`

**Status:** ENFORCED


**Severity:** CRITICAL
**Scope:** leptos, csr, css
**Version:** 1.0.0
**Date:** 2025-01-16

---

---

## Principle

In Client-Side Rendering (CSR) mode, `leptos_meta::<Stylesheet />` is **NOT** the source of truth for global CSS.

> **CSR apps mount to existing HTML.**  
> `<Stylesheet />` only works with SSR hydration.

---

## The Problem

Developers coming from SSR frameworks assume:
```rust
view! {
    <Stylesheet href="/styles.css"/>
    <App />
}
```

Will inject `<link>` tags into the document. **In CSR, it does not.**

**Why:**
- CSR uses `mount_to_body()` which replaces `<body>` content
- `<Stylesheet />` generates meta tags during SSR render
- Without SSR, there's no server to inject the `<link>`
- The component renders but produces no DOM effect

**Result:** CSS never loads, app renders unstyled.

---

## Forbidden Pattern (CSR)
```rust
// ❌ NEVER IN CSR
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Stylesheet href="/styles.css"/>  // Does nothing in CSR
        <div class="bg-primary">
            "Hello"
        </div>
    }
}
```

**This compiles. It runs. It silently fails.**

---

## Canonical Pattern (CSR)

### Option 1: HTML `<link>` (Recommended)
```html
<!-- index.html -->
<!DOCTYPE html>
<html>
<head>
    <link data-trunk rel="css" href="dist/styles.css" />
</head>
<body></body>
</html>
```

### Option 2: Direct `<link>` in HTML (No Trunk)
```html
<!DOCTYPE html>
<html>
<head>
    <link rel="stylesheet" href="/styles.css" />
</head>
<body></body>
</html>
```

**Key difference:**
- `data-trunk` → Trunk handles hashing and copying
- Direct `<link>` → Manual cache busting required

---

## When `<Stylesheet />` DOES Work

### SSR with Hydration
```rust
// ✅ WORKS IN SSR
use leptos::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Stylesheet href="/styles.css"/>  // Injected during SSR
        <div>"Hello"</div>
    }
}

// Server renders HTML with <link> already present
// Client hydrates existing DOM
```

**Why it works:**
- Server generates complete HTML including `<head>`
- `<link>` exists before WASM loads
- Client hydration preserves existing elements

---

## Detection Protocol

**Symptom:** Styles not loading in CSR app

**Verification:**
```bash
# 1. Check if using CSR
grep "mount_to_body" src/main.rs
# If yes → CSR mode

# 2. Check HTML source (before JS runs)
curl http://localhost:3001/ | grep "stylesheet"
# Should see <link> in HTML
# If not → CSS not loaded

# 3. Check component code
grep "Stylesheet" src/lib.rs
# If present in CSR → WRONG
```

---

## Correct CSR Configuration

### `main.rs`
```rust
use leptos::mount::mount_to_body;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
```

### `index.html`
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>App</title>
    <link data-trunk rel="rust" data-bin="app-name" />
    <link data-trunk rel="css" href="dist/styles.css" />
</head>
<body></body>
</html>
```

### `lib.rs`
```rust
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    // NO provide_meta_context() needed for CSS
    // NO <Stylesheet /> needed
    
    view! {
        <div class="bg-primary text-white p-4">
            "Styled content"
        </div>
    }
}
```

---

## Framework Comparison

| Framework | CSR CSS Method | SSR CSS Method |
|-----------|---------------|----------------|
| Leptos CSR | HTML `<link>` | `<Stylesheet />` |
| Leptos SSR | HTML `<link>` | `<Stylesheet />` (both work) |
| SolidStart | HTML `<link>` | `<Link />` component |
| Next.js | `globals.css` import | Same |

**Pattern:** CSR always requires HTML-level CSS loading.

---

## Canonical Justification

> **Components that mount after HTML loads cannot modify `<head>`.**  
> This is browser behavior, not framework limitation.

CSR applications must:
- Load critical CSS before JavaScript
- Prevent flash of unstyled content (FOUC)
- Work with disabled JavaScript (progressive enhancement)

The `<link>` in HTML is the only guarantee.

---

## Canon References

- Canon Rule #47 — Asset Must Exist in Final dist/
- Canon Rule #68 — CSS Build is Explicit, Never Implicit
- Canon Rule #71 — Debug Theme by Verifying File First
