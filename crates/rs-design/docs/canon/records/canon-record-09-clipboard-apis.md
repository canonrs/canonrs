# Clipboard & Browser APIs (Canon Rule #9)

**Status:** Normative  
**Applies to:** Clipboard, Browser APIs requiring user context  
**Date:** 2025-12-30

---

## The Problem

`navigator.clipboard.writeText()` fails in Leptos callbacks because:

1. **Requires document focus** - async callbacks lose focus
2. **HTTPS only** - dev environments may not have it
3. **User gesture in stack** - callbacks break the chain
4. **Promise handling** - needs `spawn_local` (SSR unsafe)

### Attempted Solutions (All Failed)

| Approach | Why It Failed |
|----------|--------------|
| `navigator.clipboard` | Document not focused in callbacks |
| `spawn_local` + Promise | Panic on SSR |
| `wasm_bindgen_futures` | Extra dependency, still needs focus |
| Focus management | Unreliable, race conditions |

**Error:**
```
NotAllowedError: Failed to execute 'writeText' on 'Clipboard': 
Document is not focused.
```

---

## ✅ The Correct Solution: document.execCommand

### Why This Works

- **No focus required** - works in background tabs
- **Synchronous** - no Promise/async needed
- **SSR-safe** - with `#[cfg]` guard
- **No extra deps** - just `wasm_bindgen`

### Canonical Implementation
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = document, js_name = execCommand)]
    fn exec_command(command: &str) -> bool;
}

pub fn copy_to_clipboard(text: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Ok(textarea) = document.create_element("textarea") {
                    if let Ok(textarea) = textarea.dyn_into::<web_sys::HtmlTextAreaElement>() {
                        textarea.set_value(text);
                        
                        // Hide off-screen
                        let style = textarea.style();
                        let _ = style.set_property("position", "absolute");
                        let _ = style.set_property("left", "-9999px");
                        
                        if let Some(body) = document.body() {
                            let _ = body.append_child(&textarea);
                            textarea.select();
                            
                            // Copy!
                            let _ = exec_command("copy");
                            
                            // Cleanup
                            let _ = body.remove_child(&textarea);
                        }
                    }
                }
            }
        }
    }
}
```

---

## Required Setup

### Cargo.toml
```toml
[dependencies]
wasm-bindgen = "0.2"

[dependencies.web-sys]
version = "0.3"
features = [
    "Window",
    "Document",
    "Element",
    "HtmlElement",
    "HtmlTextAreaElement",
    "CssStyleDeclaration",
]
```

### Project Structure
```
src/
  utils/
    clipboard.rs  ← Implementation here
    mod.rs        ← pub mod clipboard; pub use clipboard::*;
  lib.rs          ← mod utils;
```

---

## Usage in Actions
```rust
use crate::utils::copy_to_clipboard;

DataTableColumn::actions(move |row| {
    let value = row.value.clone();
    
    vec![
        RowAction {
            label: "Copy value".to_string(),
            on_click: Callback::new(move |_| {
                copy_to_clipboard(&value);
            }),
            variant: RowActionVariant::Default,
        },
    ]
})
```

---

## Decision Matrix

| Scenario | Use This Pattern | Why |
|----------|------------------|-----|
| Copy text in callback | ✅ YES | Works without focus |
| Copy in async context | ✅ YES | Synchronous, no spawn_local |
| Copy in dropdown action | ✅ YES | Dropdown closes, document unfocused |
| Copy on button click | ✅ YES | Always works |
| Modern clipboard API | ❌ NO | Fails in callbacks |

---

## Alternative: Modern API (Not Recommended)
```rust
// ❌ FAILS in callbacks
#[cfg(target_arch = "wasm32")]
{
    let clipboard = window.navigator().clipboard();
    let promise = clipboard.write_text(text);
    
    // Requires spawn_local (SSR panic risk)
    wasm_bindgen_futures::spawn_local(async move {
        let _ = JsFuture::from(promise).await;
    });
}
```

**Problems:**
- Needs `wasm_bindgen_futures` dependency
- Requires `spawn_local` (needs `#[cfg]` guard)
- Still fails if document not focused
- More complex error handling

---

## Browser Compatibility

| Browser | execCommand | clipboard API |
|---------|-------------|---------------|
| Chrome | ✅ Always | ⚠️ Focus needed |
| Firefox | ✅ Always | ⚠️ Focus needed |
| Safari | ✅ Always | ⚠️ Focus needed |
| Edge | ✅ Always | ⚠️ Focus needed |

**Verdict:** `execCommand` is deprecated but MORE reliable for our use case.

---

## Normative Status

- Violations **MUST** block PRs
- `navigator.clipboard` in callbacks **FORBIDDEN**
- All clipboard operations **MUST** use this pattern
- Alternative implementations require explicit approval

---

**Author:** Canon Working Group  
**Date:** 2025-12-30  
**Version:** 1.0  
**Status:** Production-Critical
