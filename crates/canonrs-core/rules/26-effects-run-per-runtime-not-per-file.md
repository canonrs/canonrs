# Canon Rule #26: Effects Run Per Runtime, Not Per File

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All Leptos applications using effects (create_effect, create_memo, create_resource)

---

## 🎯 Objective

Eliminate confusion about "duplicate effect execution" and hydration mismatches by clarifying that effects execute independently in each runtime context (server, browser, hydrate).

---

## 📋 The Rule

**Effects execute per-runtime, not per-source-file. Code that runs on both server and browser will execute effects twice—once per runtime.**

Common misconceptions:

- ❌ "My effect runs twice, it's a bug"
- ❌ "I wrote the effect once, it should only run once"
- ❌ "Hydration is breaking my effects"

Reality:

- ✅ Effect runs on server during SSR → produces HTML
- ✅ Same effect runs in browser during hydration → reconciles with DOM
- ✅ This is correct Leptos behavior, not a bug

---

## 🧠 The Three Runtimes (Review)

From **Canon Rule #22**, Leptos has three execution contexts:

| Runtime               | When              | Where              | Purpose                         |
| --------------------- | ----------------- | ------------------ | ------------------------------- |
| **Server**            | First request     | Actix/Axum handler | Generate initial HTML           |
| **Browser (Hydrate)** | Page load         | WASM in browser    | Attach reactivity to SSR'd HTML |
| **Browser (CSR)**     | Client navigation | WASM in browser    | Render new components           |

**Key insight:** Server and Hydrate both execute your component code, including effects.

---

## 🔍 Why Effects "Run Twice"

### Example: Simple Counter

```rust
#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    create_effect(move |_| {
        logging::log!("Effect triggered, count = {}", count.get());
    });

    view! {
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            "Count: " {count}
        </button>
    }
}
```

**What happens:**

1. **Server (SSR):**

   - Component runs
   - Effect executes: `"Effect triggered, count = 0"` (server console)
   - HTML generated: `<button>Count: 0</button>`

2. **Browser (Hydrate):**
   - WASM loads, component re-runs
   - Effect executes: `"Effect triggered, count = 0"` (browser console)
   - Reactive system attaches to existing DOM

**Result:** Log appears twice (once server, once browser). This is correct.

---

## 🚨 Common Failure Modes

### Failure 1: Server-Only Side Effects in Effects

```rust
#[component]
pub fn DataFetcher() -> impl IntoView {
    let (data, set_data) = create_signal(None);

    create_effect(move |_| {
        // ❌ WRONG: Database query in effect
        let db_result = query_database();
        set_data.set(Some(db_result));
    });

    view! {
        <div>{move || format!("{:?}", data.get())}</div>
    }
}
```

**Problem:**

- Server: Effect runs, query succeeds
- Browser: Effect runs, `query_database()` fails (no DB access in WASM)
- Hydration mismatch: Server rendered data, browser has `None`

**Fix:** Use `create_resource` or guard with `#[cfg(feature = "ssr")]`:

```rust
#[component]
pub fn DataFetcher() -> impl IntoView {
    let data = create_resource(
        || (),
        |_| async {
            #[cfg(feature = "ssr")]
            {
                query_database().await
            }
            #[cfg(not(feature = "ssr"))]
            {
                fetch_from_api().await
            }
        }
    );

    view! {
        <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            {move || data.get().map(|d| view! { <div>{format!("{:?}", d)}</div> })}
        </Suspense>
    }
}
```

### Failure 2: Browser-Only APIs in Effects

```rust
#[component]
pub fn ScrollTracker() -> impl IntoView {
    let (scroll_y, set_scroll_y) = create_signal(0);

    create_effect(move |_| {
        // ❌ WRONG: window doesn't exist on server
        let y = window().scroll_y().unwrap();
        set_scroll_y.set(y as i32);
    });

    view! {
        <div>"Scroll Y: " {scroll_y}</div>
    }
}
```

**Problem:**

- Server: Panics at `window()` (no browser API)
- Browser: Works fine

**Fix:** Guard with runtime check:

```rust
#[component]
pub fn ScrollTracker() -> impl IntoView {
    let (scroll_y, set_scroll_y) = create_signal(0);

    create_effect(move |_| {
        #[cfg(not(feature = "ssr"))]
        {
            use leptos::window;
            if let Ok(y) = window().scroll_y() {
                set_scroll_y.set(y as i32);
            }
        }
    });

    view! {
        <div>"Scroll Y: " {scroll_y}</div>
    }
}
```

### Failure 3: Non-Idempotent Effects

```rust
#[component]
pub fn IdGenerator() -> impl IntoView {
    let (id, set_id) = create_signal(String::new());

    create_effect(move |_| {
        // ❌ WRONG: Generates different UUID each time
        let uuid = Uuid::new_v4().to_string();
        set_id.set(uuid);
    });

    view! {
        <div id=id>"Content"</div>
    }
}
```

**Problem:**

- Server: Generates UUID `abc123`, renders `<div id="abc123">`
- Browser: Generates different UUID `def456`, tries to hydrate with `id="def456"`
- Hydration fails: IDs don't match

**Fix:** Generate ID once, store in signal:

```rust
#[component]
pub fn IdGenerator() -> impl IntoView {
    // ✅ Generate ID at component creation, not in effect
    let id = Uuid::new_v4().to_string();
    let (id_signal, _) = create_signal(id);

    view! {
        <div id=id_signal>"Content"</div>
    }
}
```

Or use `create_memo` to ensure same value:

```rust
let id = create_memo(move |_| {
    // This memo runs once per runtime, but returns same deterministic value
    format!("element-{}", some_deterministic_input.get())
});
```

---

## ✅ Correct Patterns

### Pattern 1: Runtime-Specific Effects

```rust
#[component]
pub fn Analytics() -> impl IntoView {
    let (page_views, set_page_views) = create_signal(0);

    // Server-only effect
    #[cfg(feature = "ssr")]
    create_effect(move |_| {
        log_to_server_analytics(page_views.get());
    });

    // Browser-only effect
    #[cfg(not(feature = "ssr"))]
    create_effect(move |_| {
        if let Some(gtag) = window().get("gtag") {
            gtag.call1(&JsValue::NULL, &JsValue::from("event"))
                .expect("gtag call failed");
        }
    });

    view! {
        <div>"Page views: " {page_views}</div>
    }
}
```

**Key:** Each runtime has its own effect. No shared execution.

### Pattern 2: Idempotent Effects

```rust
#[component]
pub fn ThemeApplier() -> impl IntoView {
    let theme = use_context::<ReadSignal<Theme>>().unwrap();

    create_effect(move |_| {
        let theme_class = match theme.get() {
            Theme::Dark => "dark-mode",
            Theme::Light => "light-mode",
        };

        // ✅ Idempotent: same input = same output
        // Both server and browser will compute same class
        document()
            .body()
            .unwrap()
            .set_class_name(theme_class);
    });

    view! { <div>"Theme applied"</div> }
}
```

**Why it works:** Effect produces identical result in both runtimes. No mismatch.

### Pattern 3: Deferred Browser Effects

```rust
#[component]
pub fn LazyLoader() -> impl IntoView {
    let (mounted, set_mounted) = create_signal(false);

    // Effect only runs after browser hydration
    create_effect(move |_| {
        if !mounted.get() {
            #[cfg(not(feature = "ssr"))]
            {
                // Browser-only initialization
                set_mounted.set(true);
                load_heavy_library();
            }
        }
    });

    view! {
        <Show
            when=move || mounted.get()
            fallback=|| view! { <div>"Loading..."</div> }
        >
            <HeavyComponent />
        </Show>
    }
}
```

---

## 🔬 Debugging "Duplicate" Effects

### Symptom: Log Appears Twice

```rust
create_effect(move |_| {
    logging::log!("Effect ran");  // Appears twice in console
});
```

**Diagnosis:**

1. Check server logs: Does it appear there?
2. Check browser console: Does it appear there?
3. If both → correct behavior (one per runtime)
4. If browser only but twice → check for duplicate providers (Canon Rule #23)

### Symptom: Hydration Mismatch Warning

```
Warning: Hydration mismatch. Server rendered:
<div>Value A</div>
But browser expected:
<div>Value B</div>
```

**Diagnosis:**

- Effect produced different result in server vs browser
- Common cause: non-deterministic data (timestamps, random IDs, API calls)

**Fix checklist:**

- [ ] Is effect reading from external source (DB, API, `window`)?
- [ ] Is effect generating random data (UUIDs, timestamps)?
- [ ] Is effect idempotent (same inputs = same outputs)?
- [ ] Could effect use `create_resource` instead?

---

## 🎓 Mental Model: Effects Are Runtime-Local

Think of effects like this:

```rust
// Your code (single source file):
create_effect(move |_| {
    logging::log!("Effect");
});

// What Leptos does:

// SERVER RUNTIME:
fn server_execution() {
    create_effect(move |_| {
        logging::log!("Effect");  // Runs here
    });
}

// BROWSER RUNTIME:
fn browser_execution() {
    create_effect(move |_| {
        logging::log!("Effect");  // AND runs here
    });
}
```

**They are separate executions in separate processes.**

---

## 📝 Validation Checklist

When writing effects, ask:

- [ ] Does this effect access server-only resources (DB, filesystem)?

  - → Guard with `#[cfg(feature = "ssr")]` or use `create_resource`

- [ ] Does this effect access browser-only APIs (`window`, `document`)?

  - → Guard with `#[cfg(not(feature = "ssr"))]`

- [ ] Does this effect produce different results on server vs browser?

  - → Make it deterministic or split into runtime-specific effects

- [ ] Does this effect modify external state (files, DB, analytics)?

  - → Ensure it's safe to run twice (idempotent or guarded)

- [ ] Am I surprised this effect runs "twice"?
  - → Revisit Canon Rule #22 (Three Runtimes)

---

## 🚫 Anti-Patterns

### Anti-Pattern 1: Fighting Dual Execution

```rust
// ❌ WRONG: Trying to prevent "duplicate" execution
static EFFECT_RAN: AtomicBool = AtomicBool::new(false);

create_effect(move |_| {
    if !EFFECT_RAN.swap(true, Ordering::SeqCst) {
        do_something();
    }
});
```

**Problem:**

- Server sets flag → effect runs once ✅
- Browser sees new runtime → flag is false → effect runs ✅
- Flag doesn't persist across runtimes
- You've added complexity for no benefit

**Fix:** Accept dual execution or use runtime guards.

### Anti-Pattern 2: Effects for Async Data

```rust
// ❌ WRONG: Effect doing async work
create_effect(move |_| {
    spawn_local(async move {
        let data = fetch_api().await;
        set_data.set(data);
    });
});
```

**Problem:**

- Server: `spawn_local` might not work (no browser event loop)
- Browser: Works, but suspense boundaries won't track loading
- Hydration: Data arrives at different times

**Fix:** Use `create_resource`:

```rust
let data = create_resource(|| (), |_| async { fetch_api().await });
```

---

## 🔗 Related Rules

- **Canon Rule #22** (Three Runtimes): Foundation for understanding dual execution
- **Canon Rule #24** (Context Lifetime): Context availability differs per runtime
- **Canon Rule #25** (Effect Execution): When effects trigger
- **Canon Rule #27** (Hydration Determinism): Ensuring server/browser match

---

## 💡 Key Insight

**Effects don't run "twice" due to a bug—they run once per runtime because your app executes in multiple runtimes.**

If you write code that runs on both server and browser, effects will execute in both. This is fundamental to SSR + hydration, not a Leptos quirk.

Solution: Write effects that are either:

1. **Runtime-specific** (guarded with `#[cfg]`)
2. **Idempotent** (produce same result in all runtimes)
3. **Deferred** (only activate after mount)

---

**Last Updated:** 2025-01-07  
**Version:** 1.0
