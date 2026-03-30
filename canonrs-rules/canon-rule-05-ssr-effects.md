# Canon Rule #05: SSR Effects and Browser API Safety

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** core-runtime
**Tags:** ssr, effects, browser api, wasm
**Language:** EN

---

**Intro:**
Using browser APIs or async runtime features during SSR leads to runtime panics and undefined behavior. Leptos components must isolate client-only logic to prevent crashes and security issues.

**Problem:**
browser apis and effects execute on server causing runtime failures

**Solution:**
guard all browser and async logic with wasm cfg conditions

**Signals:**
- window undefined
- spawn_local panic
- context not found

**Search Intent:**
how to prevent leptos ssr panic

**Keywords:**
leptos ssr effects, browser api ssr panic, leptos cfg wasm guard, spawn_local ssr error

---

## Critical Rule
**BROWSER APIs MUST NEVER RUN ON SERVER**

Violation = Runtime panic / Silent data corruption / Security breach

---

## API Blacklist Server Fatal

### Category A: DOM APIs
```rust
// ❌ FORBIDDEN without guard
window()
document()
web_sys::Element
web_sys::HtmlElement
localStorage
sessionStorage
navigator
location
history
```

### Category B: Leptos Router (Client-Contextual)
**Rule:** Router hooks are client-contextual, not SSR-safe APIs.  
They require active router context which only exists during client navigation.

```rust
// ❌ FORBIDDEN in component body
let navigate = use_navigate(); // Panic on SSR
let params = use_params(); // Context not available

// ✅ CORRECT: Inside event handler
view! {
    <button on:click=move |_| {
        let navigate = use_navigate(); // Safe in client event
        navigate("/home");
    }>
}
```

### Category C: Async Runtime
```rust
// ❌ FORBIDDEN without guard
leptos::task::spawn_local(async { ... }); // Panic on SSR
wasm_bindgen_futures::spawn_local(async { ... });

// ✅ CORRECT
#[cfg(target_arch = "wasm32")]
leptos::task::spawn_local(async { ... });
```

### Category D: JS Interop
```rust
// ❌ FORBIDDEN
#[wasm_bindgen]
extern "C" {
    fn myJsFunction(); // Undefined on server
}

// ✅ CORRECT
#[cfg(target_arch = "wasm32")]
{
    myJsFunction();
}
```

---

## Guard Patterns Mandatory

### Pattern #1: Effect Guard (Most Common)
```rust
// ❌ WRONG - Runs on server!
Effect::new(move |_| {
    let elem = document().get_element_by_id("root");
});

// ✅ CORRECT
#[cfg(target_arch = "wasm32")]
Effect::new(move |_| {
    let elem = document().get_element_by_id("root");
});
```

### Pattern #2: Conditional Block
```rust
#[cfg(target_arch = "wasm32")]
{
    // Multiple browser API calls
    let window = web_sys::window().unwrap();
    let doc = window.document().unwrap();
    let body = doc.body().unwrap();
    // ...
}
```

### Pattern #3: Function-Level Guard
```rust
#[cfg(target_arch = "wasm32")]
fn setup_scroll_lock() {
    document().body().unwrap()
        .style()
        .set_property("overflow", "hidden")
        .unwrap();
}

// Call safely anywhere
#[cfg(target_arch = "wasm32")]
setup_scroll_lock();
```

### Pattern #4: SSR Placeholder (Last Resort)
```rust
#[component]
pub fn BrowserOnlyWidget() -> impl IntoView {
    if leptos::is_server() {
        return view! { <div>"Loading..."</div> }.into_any();
    }
    
    // Client-only code
    view! { <ComplexBrowserStuff /> }.into_any()
}
```

---

## Decision Matrix

| API Type          | SSR Safe? | Guard Required | Pattern              |
|-------------------|-----------|----------------|----------------------|
| `RwSignal::new()` | ✅ Yes    | ❌ No          | Use freely           |
| `Effect::new()`   | ❌ No     | ✅ Yes         | `#[cfg(...)]`        |
| `use_context()`   | ✅ Yes    | ❌ No          | SSR-safe             |
| `use_navigate()`  | ❌ No     | ✅ Yes         | Event handler only   |
| `window()`        | ❌ No     | ✅ Yes         | `#[cfg(...)]`        |
| `spawn_local()`   | ❌ No     | ✅ Yes         | `#[cfg(...)]`        |
| `<Suspense>`      | ✅ Yes    | ❌ No          | Use freely           |

---

## Real Production Errors

### Error #1: spawn_local Panic
```
thread 'main' panicked at 'cannot call spawn_local on non-wasm target'
```

**Cause:**
```rust
Effect::new(move |_| {
    spawn_local(async { ... }); // Runs on server!
});
```

**Fix:**
```rust
#[cfg(target_arch = "wasm32")]
Effect::new(move |_| {
    spawn_local(async { ... });
});
```

---

### Error #2: use_navigate in Body
```
thread 'main' panicked at 'navigate context not found'
```

**Cause:**
```rust
#[component]
pub fn MyComponent() -> impl IntoView {
    let navigate = use_navigate(); // SSR has no router context
    view! { ... }
}
```

**Fix:**
```rust
#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <button on:click=move |_| {
            let navigate = use_navigate(); // Safe in event
            navigate("/");
        }>
    }
}
```

---

### Error #3: document() in Effect
```
thread 'main' panicked at 'window is undefined'
```

**Cause:**
```rust
Effect::new(move |_| {
    document().body(); // No DOM on server
});
```

**Fix:**
```rust
#[cfg(target_arch = "wasm32")]
Effect::new(move |_| {
    document().body();
});
```

---

### Error #4: localStorage Access
```
ReferenceError: localStorage is not defined
```

**Cause:**
```rust
let value = window()
    .local_storage()
    .unwrap()
    .get_item("key"); // Runs on server
```

**Fix:**
```rust
#[cfg(target_arch = "wasm32")]
{
    let value = window()
        .local_storage()
        .unwrap()
        .get_item("key");
}
```

---

## Common Patterns

### Focus Management
```rust
#[cfg(target_arch = "wasm32")]
Effect::new(move |_| {
    if is_open.get() {
        if let Some(elem) = document()
            .get_element_by_id("dialog")
        {
            let _ = elem.dyn_ref::<HtmlElement>()
                .map(|e| e.focus());
        }
    }
});
```

### Scroll Lock
```rust
#[cfg(target_arch = "wasm32")]
Effect::new(move |_| {
    if is_open.get() {
        document().body()
            .unwrap()
            .style()
            .set_property("overflow", "hidden")
            .ok();
    } else {
        document().body()
            .unwrap()
            .style()
            .set_property("overflow", "")
            .ok();
    }
});
```

### Click Outside
```rust
#[cfg(target_arch = "wasm32")]
Effect::new(move |_| {
    let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
        // Handle click outside
    }) as Box<dyn FnMut(_)>);
    
    document()
        .add_event_listener_with_callback(
            "click",
            closure.as_ref().unchecked_ref()
        )
        .ok();
    
    closure.forget();
});
```

### Keyboard Trap
```rust
#[cfg(target_arch = "wasm32")]
Effect::new(move |_| {
    if is_open.get() {
        let closure = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if e.key() == "Escape" {
                is_open.set(false);
            }
        }) as Box<dyn FnMut(_)>);
        
        document()
            .add_event_listener_with_callback(
                "keydown",
                closure.as_ref().unchecked_ref()
            )
            .ok();
        
        closure.forget();
    }
});
```

---

## Testing SSR Safety

### Test #1: Server Build
```bash
cargo build --features ssr
# Should compile without wasm APIs
```

### Test #2: Check for Violations
```bash
grep -r "window()" src/ --include="*.rs" | grep -v "#\[cfg"
grep -r "document()" src/ --include="*.rs" | grep -v "#\[cfg"
grep -r "use_navigate()" src/ --include="*.rs"
```

### Test #3: Runtime Check
```rust
// Add to component
#[cfg(debug_assertions)]
{
    if leptos::is_server() {
        leptos::logging::log!("⚠️ Running on SERVER");
    }
}
```

---

## References

- [Leptos SSR Guide](https://leptos-rs.github.io/leptos/ssr/index.html)
- [web-sys Documentation](https://rustwasm.github.io/wasm-bindgen/api/web_sys/)
- Canon Rule #3: Lists & Iteration
- Canon Rule #4: Anti-Hydration

---

## Quick Reference Card
```rust
// ✅ ALWAYS SAFE (SSR + Client)
RwSignal::new()
StoredValue::new()
Callback::new()
use_context()
<Suspense>
<For>

// ⚠️ GUARD REQUIRED
#[cfg(target_arch = "wasm32")]
Effect::new()
spawn_local()
window()
document()

// 🚫 NEVER (Context-Only)
use_navigate() // Only in event handlers
use_params() // Only in event handlers
use_location() // Only in event handlers
```

---

**Last Updated:** 2025-12-28 | Canon v1.2