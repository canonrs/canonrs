# Canon Rule #103: Critical Runtime JS Must Be Inline in SSR

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.1.0  
**Date:** 2026-01-15

**Category:** core-runtime
**Tags:** ssr, hydration, scripts, runtime
**Language:** EN

---

**Intro:**
External runtime scripts in Leptos SSR environments execute unpredictably due to script ordering issues. This leads to silent failures where listeners do not attach correctly after hydration.

**Problem:**
external scripts execute after hydration causing event listeners to fail

**Solution:**
inline critical runtime javascript before hydration scripts in ssr head

**Signals:**
- silent failure
- listeners not working
- race condition

**Search Intent:**
how to fix ssr script ordering

**Keywords:**
leptos ssr script order, hydration script race condition, inline runtime js ssr, leptos event listener fail

---

## Principle

**Critical runtime JS must execute before hydration.**

In SSR environments, script ordering is not stable.  
Inline scripts are the only deterministic mechanism.

---

## Problem Statement

When using Leptos SSR with `cargo-leptos`, external script tags can be:
- Reordered by `AutoReload` hot-reload injection
- Executed after WASM hydration starts
- Subject to race conditions with dynamic DOM updates

**Real-world symptom:**
```
✅ <script src="/runtime.js"> appears in HTML
✅ HTTP 200 - file loads successfully
✅ Console shows script executed
❌ Event listeners don't work
❌ Clipboard API fails silently
❌ No error messages
```

**Root cause:** Script executes AFTER hydration attaches React-like event system,  
causing listener registration to be overwritten or ignored.

---

## Rule

- Critical runtime JS MUST be:
  - **Inline** (not external file)
  - Placed in `<head>` before `<HydrationScripts/>`
  - Executed synchronously (no `defer`, no `async`)

- External scripts MUST NOT be used for:
  - Clipboard API (`navigator.clipboard`, `execCommand`)
  - Drag & Drop (`dragstart`, `drop` listeners)
  - Keyboard shortcuts (`keydown`, `keyup` global listeners)
  - Theme bootstrap (anti-flash scripts)
  - Any event delegation on `document` or `window`

---

## Forbidden
```rust
// ❌ External script in SSR shell
view! {
    <head>
        <script src="/runtime.js"></script>
        <HydrationScripts options/>
    </head>
}
```
```rust
// ❌ Using leptos_meta::Script (still subject to reordering)
use leptos_meta::Script;
view! {
    <Script src="/runtime.js"/>
}
```

---

## Required
```rust
// ✅ Inline critical runtime
#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <script>
                    // Runtime executes immediately, before any hydration
                    console.log("🟢 Canon runtime loaded");
                    document.addEventListener("click", e => {{
                        const btn = e.target.closest("[data-copy-button]");
                        if (!btn) return;
                        const text = btn.getAttribute("data-copy-text");
                        navigator.clipboard.writeText(text);
                        btn.textContent = "Copied!";
                        setTimeout(() => btn.textContent = "Copy", 1500);
                    }}, true);
                </script>
                
                <HydrationScripts options/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
```

---

## Detection

How to identify if your code violates this rule:
```bash
# 1. Check if runtime script is external
grep -r '<script src.*runtime' shell/src/lib.rs

# 2. Verify script appears in HTML but doesn't work
curl -s http://localhost:3003/ | grep -o '<script.*runtime.*'

# 3. Test: Click feature → no effect, no error
# 4. Console: script loaded ✅ but listeners silent ❌
```

**Diagnostic pattern:**
- Browser DevTools → Network: script loads (200 OK)
- Browser DevTools → Console: no errors
- Feature doesn't work intermittently or consistently
- Hard refresh sometimes fixes it temporarily

---

## Migration Path

**From external to inline:**
```bash
# 1. Read current external script
cat static/runtime.js

# 2. Minify to single line (optional)
# Remove newlines, reduce whitespace

# 3. Embed in shell() lib.rs
# Wrap in <script>...</script> inside <head>

# 4. Remove external file reference
# Delete <script src="..."> line

# 5. Test: feature works immediately after page load
```

---

## Trade-offs

| Aspect | External Script | Inline Script |
|--------|----------------|---------------|
| **Browser cache** | ✅ Cached across pages | ❌ Downloaded with HTML |
| **Execution order** | ❌ Non-deterministic | ✅ Guaranteed |
| **Hot-reload safety** | ❌ Reordered by tooling | ✅ Immune to reordering |
| **File size** | ✅ Separate asset | ❌ Increases HTML size |
| **Dev experience** | ✅ Easy to edit | ⚠️ Edit in Rust file |

**Canon decision:**  
For critical runtime (< 5KB), determinism > cache benefits.

---

## Canonical Justification

> Inline scripts are the only race-free execution model in SSR.  
> Runtime correctness is not negotiable.  
> Cache optimization is premature for infrastructure code.

---

## References

- Leptos SSR hydration model: hydration attaches after initial render
- `AutoReload` behavior: injects WebSocket + reload logic dynamically
- Event delegation pattern: requires listeners before first interaction

---