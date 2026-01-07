# Canon Rule #53: Client-Only Runtime Islands

**Status:** Normative  
**Severity:** Critical  
**Applies to:** All SSR-enabled Leptos applications

---

## The Principle

**Runtime code that depends on browser APIs MUST be guarded with `#[cfg(target_arch = "wasm32")]`.**

SSR (Server-Side Rendering) runs in a Rust server environment without:
- DOM APIs (`window`, `document`, `HTMLElement`)
- Browser events (`MouseEvent`, `KeyboardEvent`)
- Web APIs (`localStorage`, `fetch`, `WebSocket`)
- WASM-specific features

Client-only code creates "islands" - sections that only exist in the browser.

---

## The Problem

### ❌ Wrong Pattern (Unguarded Client Code)
```rust
#[component]
pub fn MyComponent() -> impl IntoView {
    // 🚫 PANIC in SSR!
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    view! {
        <div on:click=move |_| {
            // 🚫 PANIC in SSR!
            window.alert_with_message("Clicked");
        }>
    }
}
```

**Why this breaks:**
- SSR has no `window` or `document`
- `.unwrap()` panics on server
- Event handlers execute during SSR (not just client)
- Hydration mismatches

---

## The Solution

### ✅ Correct Pattern (Guarded Islands)
```rust
#[component]
pub fn MyComponent() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::window;
        
        let on_click = move |_| {
            if let Some(w) = window() {
                let _ = w.alert_with_message("Clicked");
            }
        };
        
        view! {
            <div on:click=on_click>"Click me"</div>
        }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        view! {
            <div>"Click me"</div>
        }
    }
}
```

---

## Client-Only Patterns

### 1. Event Listeners (ALWAYS Client-Only)
```rust
#[component]
pub fn CommandHistoryProvider(children: Children) -> impl IntoView {
    let history = CommandHistory::new();
    provide_context(history);
    
    // ✅ Keyboard shortcuts - client only
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        
        let closure = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if e.ctrl_key() && e.key() == "z" {
                history.undo();
            }
        }) as Box<dyn FnMut(_)>);
        
        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback(
                "keydown",
                closure.as_ref().unchecked_ref()
            );
        }
        
        closure.forget();
    }
    
    children()
}
```

### 2. DOM Manipulation (ALWAYS Client-Only)
```rust
#[component]
pub fn AutoFocus() -> impl IntoView {
    let input_ref = NodeRef::<html::Input>::new();
    
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::prelude::*;
        
        Effect::new(move |_| {
            if let Some(input) = input_ref.get() {
                let _ = input.focus();
            }
        });
    }
    
    view! {
        <input node_ref=input_ref />
    }
}
```

### 3. Browser Storage (ALWAYS Client-Only)
```rust
#[component]
pub fn PersistentState() -> impl IntoView {
    let (value, set_value) = signal(String::new());
    
    #[cfg(target_arch = "wasm32")]
    {
        // Load from localStorage
        Effect::new(move |_| {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(saved)) = storage.get_item("my_key") {
                        set_value.set(saved);
                    }
                }
            }
        });
        
        // Save to localStorage
        Effect::new(move |_| {
            let v = value.get();
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("my_key", &v);
                }
            }
        });
    }
    
    view! {
        <input
            prop:value=value
            on:input=move |e| set_value.set(event_target_value(&e))
        />
    }
}
```

### 4. Timers & Intervals (ALWAYS Client-Only)
```rust
#[component]
pub fn AutoRefresh() -> impl IntoView {
    let (count, set_count) = signal(0);
    
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        
        let closure = Closure::wrap(Box::new(move || {
            set_count.update(|c| *c += 1);
        }) as Box<dyn Fn()>);
        
        if let Some(window) = web_sys::window() {
            let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                1000
            );
        }
        
        closure.forget();
    }
    
    view! {
        <div>"Count: " {count}</div>
    }
}
```

---

## SSR-Safe Patterns

### ✅ Reactive Signals (Safe in SSR)
```rust
// ✅ Signals work in both SSR and client
let (count, set_count) = signal(0);
```

### ✅ Context Providers (Safe in SSR)
```rust
// ✅ Contexts work in both SSR and client
provide_context(MyContext::new());
```

### ✅ View Rendering (Safe in SSR)
```rust
// ✅ View macros work in both SSR and client
view! {
    <div>"Hello"</div>
}
```

### ✅ Props & Children (Safe in SSR)
```rust
#[component]
pub fn MyComponent(
    value: String,
    children: Children,
) -> impl IntoView {
    // ✅ Props and children work in both
    view! {
        <div>{value} {children()}</div>
    }
}
```

---

## Hydration Safety

### ❌ Wrong (Hydration Mismatch)
```rust
#[component]
pub fn DateTime() -> impl IntoView {
    // 🚫 Different on server vs client!
    let now = chrono::Local::now();
    
    view! {
        <div>{now.to_string()}</div>
    }
}
```

### ✅ Correct (Client-Only Render)
```rust
#[component]
pub fn DateTime() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let (now, set_now) = signal(chrono::Local::now().to_string());
        
        Effect::new(move |_| {
            set_now.set(chrono::Local::now().to_string());
        });
        
        view! {
            <div>{now}</div>
        }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        view! {
            <div>"Loading..."</div>
        }
    }
}
```

---

## Common Client-Only APIs

### Browser APIs (ALWAYS Guard)
- `window()` / `document()`
- `localStorage` / `sessionStorage`
- `navigator` / `location`
- `history.pushState()`
- `requestAnimationFrame()`
- Canvas / WebGL
- WebSocket / WebRTC
- Geolocation / Camera

### Event Listeners (ALWAYS Guard)
- `addEventListener()`
- `removeEventListener()`
- Global event handlers (keydown, resize, scroll)

### DOM Manipulation (ALWAYS Guard)
- `.focus()` / `.blur()`
- `.scrollIntoView()`
- `.querySelector()`
- Element properties (`.value`, `.checked`)

### Wasm-Only (ALWAYS Guard)
- `wasm_bindgen` closures
- `js_sys` / `web_sys` APIs
- `Closure::wrap()`
- `.forget()` / `.into_js_value()`

---

## Testing Strategy

### SSR Tests (No Guards Needed)
```rust
#[test]
fn test_ssr_render() {
    let html = leptos::ssr::render_to_string(|| view! {
        <MyComponent />
    });
    
    assert!(html.contains("expected content"));
}
```

### Client Tests (WASM Required)
```rust
#[wasm_bindgen_test]
fn test_client_interaction() {
    // Mount component
    // Simulate click
    // Assert state change
}
```

---

## Provider Guard Pattern

Providers that use client APIs should guard only the client parts:
```rust
#[component]
pub fn DragDropProvider(children: Children) -> impl IntoView {
    // ✅ Context creation works in SSR
    let context = DragContext::new();
    provide_context(context);
    
    // ✅ Event setup is client-only
    #[cfg(target_arch = "wasm32")]
    {
        setup_drag_listeners(&context);
    }
    
    children()
}
```

---

## Real-World Examples

### CommandHistoryProvider
```rust
#[component]
pub fn CommandHistoryProvider(children: Children) -> impl IntoView {
    let history = CommandHistory::new();
    provide_context(history);
    
    // ✅ Keyboard shortcuts - client only
    #[cfg(target_arch = "wasm32")]
    {
        setup_keyboard_shortcuts(history);
    }
    
    children()
}
```

### DragHandle
```rust
#[component]
pub fn DragHandle(children: Children) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let context = use_drag_context();
        
        let on_drag_start = move |ev: web_sys::DragEvent| {
            context.start_drag(item_id.clone(), None);
        };
        
        view! {
            <div draggable="true" on:dragstart=on_drag_start>
                {children()}
            </div>
        }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        view! {
            <div>{children()}</div>
        }
    }
}
```

---

## Decision Tree
```
Does code use browser API?
├─ YES → #[cfg(target_arch = "wasm32")]
└─ NO → Safe in SSR

Does code create event listener?
├─ YES → #[cfg(target_arch = "wasm32")]
└─ NO → Safe in SSR

Does code manipulate DOM?
├─ YES → #[cfg(target_arch = "wasm32")]
└─ NO → Safe in SSR

Does code use wasm_bindgen?
├─ YES → #[cfg(target_arch = "wasm32")]
└─ NO → Safe in SSR
```

---

## Normative Requirements

**MUST:**
- Client APIs MUST be guarded with `#[cfg(target_arch = "wasm32")]`
- Event listeners MUST be client-only
- DOM manipulation MUST be client-only
- SSR and client MUST render identical initial HTML

**MUST NOT:**
- MUST NOT use `.unwrap()` on `window()` or `document()`
- MUST NOT create hydration mismatches
- MUST NOT call browser APIs in SSR
- MUST NOT use `wasm_bindgen` without guards

**SHOULD:**
- Use `if let Some(window) = window()` pattern
- Provide fallback SSR rendering where appropriate
- Test both SSR and client paths
- Document which components are client-only

---

## Anti-Patterns ❌

### 🚫 Unguarded DOM Access
```rust
let window = window().unwrap(); // PANIC in SSR!
```

### 🚫 Hydration Mismatch
```rust
let random = rand::random::<u32>(); // Different on server vs client
view! { <div>{random}</div> }
```

### 🚫 Client State in SSR
```rust
localStorage.get_item("key") // Doesn't exist in SSR
```

---

**Author:** Canon Working Group  
**Date:** 2026-01-04  
**Version:** 1.0  
**Related:** Leptos SSR, Hydration, WASM
