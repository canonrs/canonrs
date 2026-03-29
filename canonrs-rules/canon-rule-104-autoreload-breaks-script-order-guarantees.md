# Canon Rule #104: AutoReload Breaks Script Order Guarantees

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** leptos, build, ssr
**Version:** 1.1.0  
**Date:** 2026-01-15

---

## Principle

**AutoReload may reorder or inject scripts unpredictably.**

Hot-reload tooling alters DOM execution order and must not be trusted  
for runtime-critical logic.

---

## Problem Statement

Leptos SSR's `<AutoReload/>` component provides hot-reload during development.  
It works by:
1. Injecting a WebSocket connection to the dev server
2. Listening for file changes
3. Dynamically inserting/updating script tags in `<head>`

**The hidden cost:**  
Even if you write scripts in a specific order in `shell()`, AutoReload  
injects wrapper code that can execute BETWEEN your intended script order.

**Real scenario that cost 18 hours:**
```rust
// Developer writes this order:
<script src="/runtime.js"></script>
<HydrationScripts options/>

// Browser receives this order:
<script>/* AutoReload WebSocket */</script>
<script src="/runtime.js"></script>
<script>/* Hydration starts */</script>
<script>/* AutoReload wrapper continues */</script>
```

Result: Runtime script loads but listeners register in wrong lifecycle phase.

---

## Rule

- `AutoReload` MUST be treated as non-deterministic.
- Script tag order in `shell()` source ≠ execution order in browser.
- Runtime-critical JS MUST NOT rely on external script ordering when `AutoReload` is active.
- Production builds MUST NOT include `AutoReload`.

---

## Symptoms of Violation

**Pattern A: Intermittent Failures**
- Feature works after hard refresh (Ctrl+Shift+R)
- Feature breaks after hot-reload triggers
- Works in production, fails in dev

**Pattern B: Silent Failures**
```
✅ Script tag appears in HTML source
✅ HTTP 200 - file downloaded
✅ No console errors
✅ No network errors
❌ Event listeners don't fire
❌ Clipboard API does nothing
❌ Drag & drop fails
```

**Pattern C: Timing-Dependent**
- Adding `setTimeout` "fixes" it
- Adding `console.log` changes behavior
- Works on slow connections, fails on fast

---

## Detection
```bash
# 1. Check if AutoReload is active
grep -n "AutoReload" shell/src/lib.rs

# 2. Inspect actual HTML order in browser
curl -s http://localhost:3003/ | grep -n '<script'

# 3. Compare source order vs browser order
# Source:  runtime.js (line 38) → HydrationScripts (line 39)
# Browser: AutoReload (injected) → runtime.js → more AutoReload

# 4. Test behavior difference
# Dev:  make dev  → feature broken
# Prod: cargo leptos build --release → feature works
```

**Diagnostic questions:**
- Does removing `<AutoReload/>` fix it? → This rule applies
- Does inline script fix it? → Rule #103 applies
- Does it work in production? → AutoReload is the culprit

---

## Required Mitigation

### Option A: Inline Critical Scripts (Recommended)
```rust
// Remove AutoReload dependency entirely for critical runtime
view! {
    <head>
        <script>
            // Inline = immune to AutoReload reordering
            document.addEventListener("click", /* ... */);
        </script>
        <HydrationScripts options/>
    </head>
}
```

### Option B: Disable AutoReload in Dev (Not Recommended)
```rust
#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <head>
            // ⚠️ Loses hot-reload benefits
            // <AutoReload options=options.clone()/>
            <script src="/runtime.js"></script>
            <HydrationScripts options/>
        </head>
    }
}
```

### Option C: Conditional AutoReload (Compromise)
```rust
view! {
    <head>
        <script>/* inline critical runtime */</script>
        
        #[cfg(debug_assertions)]
        <AutoReload options=options.clone()/>
        
        <HydrationScripts options/>
    </head>
}
```

---

## Migration Path

**Identifying affected code:**
```bash
# Find external scripts that might be reordered
grep -rn '<script src' shell/src/lib.rs

# Find AutoReload usage
grep -rn 'AutoReload' shell/src/

# Check if runtime depends on execution order
grep -rn 'addEventListener.*document\|window' static/
```

**Step-by-step fix:**
1. Identify critical runtime scripts (clipboard, drag, shortcuts)
2. Inline them before `<HydrationScripts/>`
3. Keep non-critical scripts external (analytics, etc.)
4. Test in dev with AutoReload active
5. Verify production build still works

---

## Trade-offs

| Approach | Dev Experience | Runtime Safety | Bundle Size |
|----------|---------------|----------------|-------------|
| **External + AutoReload** | ✅ Fast hot-reload | ❌ Race conditions | ✅ Cached |
| **Inline + AutoReload** | ✅ Fast hot-reload | ✅ Deterministic | ⚠️ +2-5KB HTML |
| **External - AutoReload** | ❌ Manual refresh | ✅ Deterministic | ✅ Cached |

**Canon decision:**  
Runtime safety > dev convenience for production-critical features.

---

## Why This Happens (Technical Deep Dive)

AutoReload implementation:
```rust
// Simplified internal behavior
pub fn AutoReload() -> impl IntoView {
    view! {
        <script>
            // WebSocket connection injected HERE
            const ws = new WebSocket("ws://localhost:3001");
            ws.onmessage = () => location.reload();
        </script>
    }
}
```

The `view!` macro expands this into multiple script injections,  
and Leptos SSR doesn't guarantee atomic ordering of sibling elements  
during hydration preparation phase.

**Not a bug, but a design trade-off:**  
AutoReload prioritizes developer experience over execution determinism.

---

## Canonical Justification

> Development tooling is allowed to be unstable.  
> Runtime correctness is not.

Hot-reload is a productivity tool, not infrastructure.  
Critical runtime must be independent of development tooling.

---

## References

- Leptos AutoReload source: https://github.com/leptos-rs/leptos
- SSR script injection order: non-deterministic by design
- Production builds: AutoReload compiled out via feature flags

---

## See Also

- **Rule #103:** Critical Runtime JS Must Be Inline in SSR
- **Rule #102:** Runtime JS Is Shell Infrastructure

---
