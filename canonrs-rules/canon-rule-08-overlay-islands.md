# Canon Rule #08: Overlay Islands (Client-Only Architecture)

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** hydration, csr, ssr
**Version:** 1.0.0
**Date:** 2025-01-16

---

---

## The Problem

**Dynamic overlays** (with reactive lists) + **SSR** = **Hydration Hell**

### Why?

1. SSR generates static HTML  
2. Client tries to hydrate  
3. Dynamic list changes DOM  
4. Leptos detects mismatch  
5. **Panic**

### Attempted Solutions (All Failed)

| Approach | Why It Failed |
|----------|--------------|
| `cfg!(feature = "ssr")` | Compile-time, not runtime |
| `#[cfg(feature = "hydrate")]` | Code does not exist in SSR, breaks composition |
| `is_server()` | Edge case: may return `true` on client in some builds |
| `.map().collect_view()` | `Vec<View>` is not `Sync` |
| `StoredValue<Vec<View>>` | Same problem, does not compile |
| `<For>` inside overlay | Direct hydration mismatch |

---

## The Correct Solution: Browser Detection

### Pattern
```rust
use leptos::prelude::*;

#[component]
pub fn DynamicOverlay(items: Vec<String>) -> impl IntoView {
    let items = StoredValue::new(items);
    let is_browser = RwSignal::new(false);
    
    // Runtime browser detection
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if web_sys::window().is_some() {
            is_browser.set(true);
        }
    });

    view! {
        <Show
            when=move || is_browser.get()
            fallback=|| view! {
                // SSR placeholder (same size as real component)
                <div class="w-20 h-8"></div>
            }
        >
            <DropdownMenu>
                <DropdownMenuContent>
                    {move || {
                        items.get_value()
                            .iter()
                            .map(|item| view! { <MenuItem>{item}</MenuItem> })
                            .collect_view()
                    }}
                </DropdownMenuContent>
            </DropdownMenu>
        </Show>
    }
}
```

### Why This Works

1. **SSR:** `window` does not exist → `is_browser = false` → renders placeholder  
2. **Client:** `window` exists → Effect runs → `is_browser = true` → renders overlay  
3. **Zero hydration mismatch:** Placeholder in SSR, overlay on client  
4. **Pure runtime:** Does not depend on feature flags  
5. **Robust:** Works in all build modes (dev/watch/release/SSR)  

---

## Affected Components

### Must Use Island Pattern

- ✅ `DropdownMenu` (with dynamic list)  
- ✅ `Popover` (with dynamic content)  
- ✅ `ContextMenu` (with dynamic items)  
- ✅ `CommandPalette` (always dynamic)  
- ✅ `Select` (with dynamic options)  
- ✅ `DataTableViewOptions` (column toggle)  

### Can Use SSR Normally

- ✅ `Table` (static data)  
- ✅ `Card` (fixed layout)  
- ✅ `Button` (no overlay)  
- ✅ `Input` (form)  
- ✅ `Checkbox` (simple control)  

---

## Implementation Checklist

- [ ] Component uses overlay (Dropdown/Popover/Dialog)?  
- [ ] Overlay contains dynamic list (`<For>` or `.map()`)?  
- [ ] If YES to both: apply Island Pattern  
- [ ] SSR placeholder has **same size** as real component  
- [ ] Tested in SSR + hydration + client  

---

## Edge Cases

### "is_server() returns true on client"

**Symptom:** `is_server()` remains `true` even after hydration  

**Cause:** Known Leptos bug in some build setups  

**Solution:** Use `web_sys::window()` instead of `is_server()`  

### "Flash of placeholder"

**Symptom:** User sees placeholder before overlay appears  

**Cause:** Normal — Effect needs to run first  

**Solution:**  
- Placeholder must have similar size/style  
- Add `opacity-0` to placeholder if needed  
- Or accept as trade-off for correct SSR  

---

## Performance Implications

### SSR

- ✅ Smaller payload (no overlay in HTML)  
- ✅ Faster First Paint  
- ❌ Component not visible in SSR  

### Client

- ✅ Zero hydration errors  
- ✅ Overlay fully functional  
- ⚠️ Slight delay until Effect runs (imperceptible)  

---

## Comparison: CanonRS vs shadcn

| Aspect | shadcn (React) | CanonRS (Leptos) |
|--------|----------------|------------------|
| **Strategy** | Implicit client-only | Explicit island |
| **SSR** | Not real (Next.js client components) | True SSR |
| **Governance** | Not documented | Canonical rule |
| **Robustness** | "Just works" | Conscious engineering |

**Verdict:** CanonRS is **more rigorous**, not inferior.

---

## Examples

### ✅ Correct: DataTableViewOptions
```rust
// SSR: placeholder
// Client: dropdown with dynamic checkboxes
<Show when=move || is_browser.get() ...>
    <DropdownMenu>
        {move || columns.iter().map(...).collect_view()}
    </DropdownMenu>
</Show>
```

### ❌ Wrong: Dynamic list without Island
```rust
// BREAKS hydration
<DropdownMenu>
    <For each=|| items .../>
</DropdownMenu>
```

### ❌ Wrong: cfg!() for runtime
```rust
// DOES NOT WORK
<Show when=|| !cfg!(feature = "ssr") ...>
```

---

## Normative Status

- Violations **MUST** block PRs  
- Exceptions require explicit approval  
- All overlay components **MUST** document if they need Islands  
- Design system **MUST** provide Island utilities  

---

**Author:** Canon Working Group  
**Replaces:** None (first formal definition)
