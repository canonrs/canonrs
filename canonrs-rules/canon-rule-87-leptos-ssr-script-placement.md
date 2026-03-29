# Canon Rule #87: Leptos SSR Script Placement

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** hydration, ssr, leptos
**Version:** 1.0.0  
**Date:** 2026-01-14

---

## Principle

**Imperative JavaScript MUST be injected in `shell()`, NOT in `index.html`.**

In Leptos SSR apps, `index.html` is a static template that does NOT participate in the hydration cycle. Scripts placed there execute before the real DOM exists, causing "phantom behavior" where events are registered but never fire.

---

## The Problem

### ❌ Anti-Pattern: Script in `index.html`
```html
<!-- index.html -->
<!DOCTYPE html>
<html>
<head>...</head>
<body>
    <script>
    document.addEventListener("click", (e) => {
        const btn = e.target.closest("[data-copy-button]");
        // ❌ btn is never found, even though HTML has the button
    });
    </script>
</body>
</html>
```

**Why it fails:**
- Script runs before SSR DOM is hydrated
- Event listener attaches to wrong/stale DOM tree
- After hydration, the real DOM has no listener
- Result: clicks detected but nothing happens

**Symptoms:**
- Event seems to fire intermittently
- `console.log` shows click detected but action fails
- DOM inspection shows correct attributes but no behavior
- Works in production build but not dev

---

## Canonical Solution

### ✅ Pattern: Script in `shell()`
```rust
#[cfg(feature = "ssr")]
pub fn shell(options: leptos::config::LeptosOptions) -> impl IntoView {
    use leptos_meta::*;

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
                
                // ✅ Script AFTER <App/>, inside shell()
                <script>
                document.addEventListener("click", (e) => {{
                    const btn = e.target.closest("[data-copy-button]");
                    if (!btn) return;
                    
                    const text = btn.getAttribute("data-copy-text");
                    if (!text) return;
                    
                    // clipboard logic...
                }}, true); // capture phase for hydrated apps
                </script>
            </body>
        </html>
    }
}
```

**Why it works:**
- Script executes AFTER SSR DOM is rendered
- Has access to actual hydrated elements
- Event delegation survives hydration
- Works in both dev and production

---

## Decision Matrix

| Script Location | SSR | Hydration | Event Access | Verdict |
|----------------|-----|-----------|--------------|---------|
| `index.html` | ❌ Ignored | ❌ Replaced | ❌ Stale DOM | **FORBIDDEN** |
| `shell()` inline | ✅ Rendered | ✅ Preserved | ✅ Real DOM | **CORRECT** |
| `shell()` external | ✅ Rendered | ✅ Preserved | ✅ Real DOM | **ACCEPTABLE** |

---

## Scope of Application

**Scripts that MUST be in `shell()`:**
- Event delegation (clicks, keyboard shortcuts)
- Clipboard operations
- Global observers (IntersectionObserver, MutationObserver)
- Analytics / tracking
- Third-party integrations (e.g., Stripe, Google Analytics)

**Scripts that CAN be in `index.html`:**
- Theme flash prevention (must run before render)
- Critical path CSS loading
- Feature detection that doesn't touch DOM

---

## Real-World Example: Copy Button

### Problem Statement
Copy button with `data-copy-button` and `data-copy-text` attributes renders correctly in DOM, but clicking does nothing. Console shows no errors.

### Root Cause
Event listener in `index.html` attached to pre-hydration DOM. After hydration, listener was orphaned.

### Solution
```rust
// src/app.rs
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html>
            <body>
                <App/>
                <script>
                document.addEventListener("click", (e) => {{
                    const btn = e.target.closest("[data-copy-button]");
                    if (!btn) return;
                    
                    const text = btn.getAttribute("data-copy-text");
                    const textarea = document.createElement("textarea");
                    textarea.value = text;
                    textarea.style.position = "fixed";
                    textarea.style.opacity = "0";
                    
                    document.body.appendChild(textarea);
                    textarea.select();
                    document.execCommand("copy");
                    document.body.removeChild(textarea);
                    
                    btn.textContent = "Copied!";
                    setTimeout(() => btn.textContent = "Copy", 1500);
                }}, true);
                </script>
            </body>
        </html>
    }
}
```

**Result:** Copy functionality works reliably in dev and production.

---

## Debugging Protocol

When imperative JS "doesn't work" in Leptos SSR:
```bash
# 1. Check script location
grep -r "<script>" src/app.rs
# Should be in shell(), not index.html

# 2. Verify shell() is being used
grep "pub fn shell" src/app.rs

# 3. Confirm SSR feature is enabled
cargo check --features ssr

# 4. Test in browser console
document.querySelectorAll('[data-copy-button]')
# Should return NodeList with elements

# 5. Manually test event
document.addEventListener('click', e => console.log(e.target), true)
# Should log all clicks
```

---

## Integration with Monorepo

When scripts are shared across multiple Leptos apps:
```rust
// Option A: Inline in each shell()
pub fn shell() -> impl IntoView {
    view! {
        <body>
            <App/>
            <script>{include_str!("../scripts/runtime.js")}</script>
        </body>
    }
}

// Option B: External with correct path
pub fn shell() -> impl IntoView {
    view! {
        <body>
            <App/>
            <script src="/scripts/runtime.js"></script>
        </body>
    }
}
```

**IMPORTANT:** External scripts must be in `assets-dir` (see Canon Rule #85).

---

## Canonical Justification

> **Leptos SSR has a specific hydration model.**  
> Scripts that don't respect this model create "phantom behaviors" where the app appears broken despite correct DOM structure.

This rule enforces:
- **Predictable script execution** relative to hydration
- **No "works sometimes" bugs** caused by race conditions
- **Clear separation** between static templates and dynamic rendering
- **Maintainable debugging** by eliminating invisible failure modes

---

## Related Canon Rules

- Canon Rule #85 — Leptos Asset Pipeline in Dev Mode
- Canon Rule #58 — Leptos Assets Dev Constraint (subfolder limitation)
- Canon Rule #04 — Hydration Safety in SSR

---

## Version History

- **1.0.0** (2026-01-14): Initial rule based on Copy Button implementation debugging
