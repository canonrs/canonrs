# Canon Rule #95: SSR Requires Complete HTML Shell

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2025-01-15

**Category:** core-runtime
**Tags:** ssr, hydration, html, leptos
**Language:** EN

---

**Intro:**
Leptos SSR requires a full HTML document structure to support meta injection and hydration. Missing html head or body tags causes runtime panics and empty responses.

**Problem:**
missing html shell prevents SSR rendering and hydration

**Solution:**
provide full html head and body structure through shell function

**Signals:**
- ERR_EMPTY_RESPONSE
- leptos_meta panic
- missing head tag

**Search Intent:**
how to fix leptos ssr empty response

**Keywords:**
leptos ssr html shell, leptos_meta head tag error, hydration scripts placement leptos, ssr empty response fix

---

## Principle

**Leptos SSR applications MUST provide a complete HTML document shell with explicit `<html>`, `<head>`, and `<body>` tags.**

Without this shell, leptos_meta components panic at runtime, SSR fails silently, and hydration cannot occur. The shell is structural, not cosmetic.

---

## The Problem

When SSR components use `leptos_meta` (Title, Meta, etc.) without a complete HTML shell:

- Runtime panic: "you are using leptos_meta without a </head> tag"
- Server returns empty responses (ERR_EMPTY_RESPONSE)
- No error message in build, only at first request
- Hydration scripts have nowhere to attach

### Real Symptoms
```
thread 'tokio-runtime-worker' panicked at leptos_meta/src/lib.rs:250:18:
you are using leptos_meta without a </head> tag
```

Browser shows:
```
ERR_EMPTY_RESPONSE
```

---

## Anti Pattern
```rust
// ❌ FORBIDDEN: No HTML shell
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Title text="My App"/>
        <Router>
            <Routes>...</Routes>
        </Router>
    }
}
```

This fails because:
- leptos_meta expects `<head>` to exist
- SSR has no document structure to render into
- Hydration scripts cannot be injected

---

## Canonical Pattern
```rust
// ✅ REQUIRED: Complete HTML shell
#[cfg(feature = "ssr")]
pub fn shell(options: leptos::config::LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Title text="My App"/>
        <Router>...</Router>
    }
}
```

Server uses `shell()` instead of `App()`:
```rust
let app = Router::new()
    .leptos_routes(&leptos_options, routes, {
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
    })
    .with_state(leptos_options);
```

---

## Required Shell Components

### Minimum Required
```html
<!DOCTYPE html>
<html>
    <head>
        <AutoReload options/>
        <HydrationScripts options/>
        <MetaTags/>
    </head>
    <body>
        <App/>
    </body>
</html>
```

### Recommended Production
```html
<!DOCTYPE html>
<html lang="en" data-theme="light">
    <head>
        <meta charset="utf-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1"/>
        <link rel="stylesheet" href="/app.css"/>
        <AutoReload options/>
        <HydrationScripts options/>
        <MetaTags/>
    </head>
    <body>
        <App/>
        <script src="/runtime.js"></script>
    </body>
</html>
```

---

## Why This Is Required

SSR operates in two phases:

1. **Server render**: Generates HTML string
   - Needs `<head>` to inject meta tags
   - Needs `<body>` to place app content
   - Needs `<!DOCTYPE>` for valid HTML5

2. **Client hydration**: Attaches JS to existing DOM
   - HydrationScripts must be in `<head>`
   - App component must be in `<body>`
   - DOM structure must match server output

Without the shell:
- Phase 1 has no target to render into
- Phase 2 has no structure to hydrate
- leptos_meta has no `<head>` to populate

---

## Diagnostic Checklist

If you see "leptos_meta without </head> tag":
```bash
# 1. Verify shell() exists with #[cfg(feature = "ssr")]
grep -A 5 "pub fn shell" src/lib.rs

# 2. Check server uses shell(), not App()
grep "shell(" src/main.rs

# 3. Verify shell has complete structure
# Must have: <!DOCTYPE html>, <html>, <head>, <body>

# 4. Check MetaTags placement
# Must be inside <head>
```

---

## Enforcement

- All Leptos SSR apps MUST implement `shell()` function
- `shell()` MUST include `<!DOCTYPE html><html><head><body>`
- Server routes MUST call `shell()`, not `App()` directly
- `MetaTags` MUST be inside `<head>`
- `App` component MUST be inside `<body>`

---

## Canonical Justification

> **SSR is HTML document generation, not component mounting.**  
> The server produces a complete HTML document.  
> Partial DOM injection is a CSR pattern and does not apply to SSR.

This rule exists to:
- Make SSR's document-centric nature explicit
- Prevent "empty response" debugging cycles
- Align with browser expectations for valid HTML
- Enable proper hydration anchoring

---

## Related Canon Rules

- Canon Rule #94 — Leptos Workspace Features Must Be Explicit
- Canon Rule #96 — SSR Requires Explicit Provider Tree
- Canon Rule #90 — Hydration Is DOM Replacement

---

## Version History

- **1.0.0** (2025-01-15): Initial rule based on Workbench migration SSR shell requirement