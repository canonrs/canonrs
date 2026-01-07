# Canon Rule #27: Hydration Determinism (IDs, Time, Random, Order)

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All SSR Leptos applications

---

## 🎯 Objective

Ensure server-rendered HTML matches client-side hydration exactly by enforcing deterministic rendering. Non-deterministic sources (random IDs, timestamps, render order) cause hydration mismatches, breaking reactivity and causing visual glitches.

---

## 📋 Requirements

### 1. IDs Must Be Deterministic

**NEVER:**
```rust
// ❌ WRONG: Non-deterministic ID
let id = format!("item-{}", js_sys::Math::random());
let id = format!("node-{}", Date::now());
let id = uuid::Uuid::new_v4().to_string();
```

**ALWAYS:**
```rust
// ✅ CORRECT: Deterministic ID from data
let id = format!("item-{}", item.database_id);
let id = format!("user-{}", user.uuid);

// ✅ CORRECT: Counter-based ID (stable across SSR + hydration)
#[component]
fn ListItem(index: usize, item: Item) -> impl IntoView {
    let id = format!("item-{}", index);
    view! { <div id=id>{item.name}</div> }
}
```

### 2. No Client-Only APIs in SSR Code

**Server cannot execute:**
- `window.*`
- `document.*`
- `localStorage`
- `Math.random()`
- `Date.now()` (without sync)

**Pattern:**
```rust
use leptos::prelude::*;

#[component]
fn SafeComponent() -> impl IntoView {
    // ✅ CORRECT: Feature-gated
    #[cfg(feature = "hydrate")]
    let client_only = window().location().href();
    
    #[cfg(not(feature = "hydrate"))]
    let client_only = String::new();
    
    view! { <div>{client_only}</div> }
}
```

### 3. Render Order Must Be Stable

**NEVER:**
```rust
// ❌ WRONG: HashMap iteration order is non-deterministic
for (key, value) in hash_map.iter() {
    view! { <div>{key}: {value}</div> }
}

// ❌ WRONG: Async data without Suspense
let data = create_resource(|| fetch_data());
view! { <div>{move || data.get()}</div> }
```

**ALWAYS:**
```rust
// ✅ CORRECT: Stable iteration order
let mut items: Vec<_> = hash_map.iter().collect();
items.sort_by_key(|(k, _)| *k);
for (key, value) in items {
    view! { <div>{key}: {value}</div> }
}

// ✅ CORRECT: Suspense for async data
view! {
    <Suspense fallback=|| view! { <div>"Loading..."</div> }>
        {move || data.get().map(|d| view! { <div>{d}</div> })}
    </Suspense>
}
```

### 4. Time/Date Must Match Server and Client

**NEVER:**
```rust
// ❌ WRONG: Server renders UTC, client renders local
let now = chrono::Utc::now();
view! { <time>{now.to_string()}</time> }
```

**ALWAYS:**
```rust
// ✅ CORRECT: Serialize timestamp, hydrate client-side
#[component]
fn Timestamp(timestamp: i64) -> impl IntoView {
    let formatted = create_memo(move |_| {
        #[cfg(feature = "hydrate")]
        {
            let date = js_sys::Date::new(&JsValue::from_f64(timestamp as f64));
            date.to_locale_string("en-US", &JsValue::NULL).as_string().unwrap()
        }
        #[cfg(not(feature = "hydrate"))]
        {
            format!("{}", timestamp) // Placeholder for SSR
        }
    });
    
    view! { <time datetime=timestamp>{formatted}</time> }
}
```

### 5. Effects Do Not Run on Server

**Key principle:**
```rust
// ✅ Effects are client-only
create_effect(move |_| {
    // This NEVER runs on server
    log::info!("This only runs in browser");
});

// ⚠️ create_resource runs on BOTH
let data = create_resource(|| async {
    // Runs on server during SSR
    // Runs on client during hydration
    fetch_data().await
});
```

---

## 🚫 Prohibited Patterns

**NEVER:**
1. Use `Math.random()` for IDs
2. Use `Date.now()` without server-client sync
3. Iterate over `HashMap` without sorting
4. Access `window`/`document` in non-gated code
5. Render different content on server vs client
6. Use `localStorage` in component body

---

## 🔍 Validation

### Manual Check:
```bash
# Search for non-deterministic patterns
grep -r "Math.random\|Date.now\|Uuid::new_v4" src/

# Search for unguarded browser APIs
grep -r "window\(\)\|document\(\)" src/ | grep -v "#\[cfg"

# Search for HashMap iteration in views
grep -r "\.iter()" src/ | grep "view!"
```

### Runtime Check:
1. Enable SSR: `cargo leptos build --release`
2. Compare server HTML with hydrated DOM
3. Check browser console for hydration warnings:
   - `Hydration mismatch`
   - `Expected node`
   - `Attribute mismatch`

---

## ✅ Correct Patterns

### Pattern 1: Deterministic Keys from Data
```rust
#[component]
fn UserList(users: Vec<User>) -> impl IntoView {
    view! {
        <For
            each=move || users.clone()
            key=|user| user.id  // ✅ Database ID is stable
            children=|user| view! { <UserCard user=user/> }
        />
    }
}
```

### Pattern 2: Client-Only Rendering
```rust
#[component]
fn ClientOnlyComponent() -> impl IntoView {
    view! {
        <Show
            when=|| cfg!(feature = "hydrate")
            fallback=|| view! { <div>"SSR Placeholder"</div> }
        >
            <BrowserOnlyFeature/>
        </Show>
    }
}
```

### Pattern 3: Synced Timestamps
```rust
#[server]
async fn get_server_time() -> Result<i64, ServerFnError> {
    Ok(chrono::Utc::now().timestamp())
}

#[component]
fn Clock() -> impl IntoView {
    let server_time = create_resource(|| (), |_| get_server_time());
    
    view! {
        <Suspense>
            {move || server_time.get().map(|t| view! {
                <time>{format_timestamp(t)}</time>
            })}
        </Suspense>
    }
}
```

---

## 📚 References

- Leptos SSR Guide: https://book.leptos.dev/ssr/
- Hydration spec: https://github.com/leptos-rs/leptos/blob/main/docs/book/src/ssr/21_hydration_bugs.md
- Related: Rule #22 (Three Runtimes), Rule #26 (Effects)

---

## 🧠 Rationale

Hydration mismatches break:
- Reactivity (signals don't attach)
- Event handlers (listeners orphaned)
- Visual consistency (flash of wrong content)

Non-deterministic rendering is the #1 cause of "works in dev, breaks in prod" SSR bugs.

---

**Examples of Real Bugs:**

1. **Random IDs:** Drag-drop breaks because `data-drag-id` doesn't match
2. **HashMap order:** Tabs render in wrong sequence on hydration
3. **Date.now():** "Posted 2 seconds ago" becomes "Posted 5 minutes ago"
4. **window.location:** Server renders blank, client renders URL

**Fix:** Apply this rule religiously. SSR + CSR must be byte-identical.

---

**Last Updated:** 2025-01-07  
**Version:** 1.0
