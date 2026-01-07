# Canon Rule #22: Leptos Has Three Runtimes, Not One

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All Leptos applications in monorepo

---

## 🎯 Objective

Understand that Leptos operates in three fundamentally different execution modes (CSR, Hydrate, SSR), each with distinct behavior, lifecycle, and limitations.

---

## 📋 The Three Runtimes

### 1. CSR (Client-Side Rendering)

**What runs:** WASM only  
**Where:** Browser  
**Feature flag:** `csr`
```rust
#[cfg(feature = "csr")]
fn client_only_code() {
    // Runs ONLY in browser
    // Has access to: window, document, DOM APIs
    // Does NOT run on server
}
```

**Characteristics:**
- Initial page is empty HTML shell
- All rendering happens in browser
- No SEO without external solutions
- Fast subsequent navigation
- No server resources needed for UI

### 2. Hydrate

**What runs:** Server renders HTML → Browser "hydrates" with WASM  
**Where:** Server (render) + Browser (attach events)  
**Feature flags:** `ssr` (server) + `hydrate` (client)
```rust
#[component]
fn MyComponent() -> impl IntoView {
    // This code runs:
    // 1. On server (generates HTML)
    // 2. On client (attaches event handlers)
    
    let (count, set_count) = create_signal(0);
    
    view! {
        <button on:click=move |_| set_count.update(|n| *n + 1)>
            {count}
        </button>
    }
}
```

**Characteristics:**
- Server sends full HTML (SEO-friendly)
- Browser downloads WASM and "hydrates" (attaches interactivity)
- Code runs TWICE (server + client)
- State must be serializable
- Most complex runtime

### 3. SSR (Server-Side Rendering Only)

**What runs:** Server only, no WASM  
**Where:** Server  
**Feature flag:** `ssr`
```rust
#[cfg(feature = "ssr")]
async fn server_only_endpoint() -> Result<String> {
    // Runs ONLY on server
    // Has access to: database, filesystem, secrets
    // Never exposed to client
}
```

**Characteristics:**
- Pure server rendering
- No client-side interactivity
- Traditional request/response
- Good for admin panels, static content
- Smallest client bundle

---

## 🧠 Key Differences

| Aspect | CSR | Hydrate | SSR |
|--------|-----|---------|-----|
| **Initial HTML** | Empty | Full | Full |
| **Interactivity** | Immediate | After hydration | None |
| **SEO** | Poor | Good | Good |
| **Code runs where** | Client | Both | Server |
| **Bundle size** | Large | Large | Small |
| **Server load** | None | Medium | High |
| **First Paint** | Slow | Fast | Fast |

---

## 🚨 Common Mistakes

### Mistake 1: Assuming Code Runs Once
```rust
#[component]
fn BadCounter() -> impl IntoView {
    // ❌ WRONG: This runs on server AND client
    println!("Component created");  // Prints twice in hydrate mode
    
    let (count, set_count) = create_signal(0);
    view! { <button>{count}</button> }
}
```

**Reality in Hydrate:**
1. Server: `println!` → server logs
2. Client: `println!` → browser console
3. Result: Two different signal instances

### Mistake 2: Using Browser APIs Without Guards
```rust
#[component]
fn BadComponent() -> impl IntoView {
    // ❌ WRONG: window doesn't exist on server
    let width = window().inner_width().unwrap();
    
    view! { <div>{width}</div> }
}
```

**Fix:**
```rust
#[component]
fn GoodComponent() -> impl IntoView {
    let width = create_signal(0);
    
    #[cfg(feature = "hydrate")]
    {
        width.set(window().inner_width().unwrap());
    }
    
    view! { <div>{width}</div> }
}
```

### Mistake 3: Feature Flag Confusion
```rust
// ❌ WRONG: Cargo.toml
[features]
default = ["csr"]
ssr = ["leptos/ssr"]
hydrate = ["leptos/hydrate"]
```

**Problem:** Can't have both `ssr` AND `hydrate` active simultaneously in same binary.

**Fix:**
```toml
# Separate build targets
[features]
# Server binary
ssr = ["leptos/ssr", "dep:axum"]

# Client WASM
hydrate = ["leptos/hydrate"]
csr = ["leptos/csr"]

# Never enable ssr+hydrate together
```

---

## ✅ Correct Patterns

### Pattern 1: Runtime-Specific Code
```rust
#[component]
fn RuntimeAwareComponent() -> impl IntoView {
    // Server-only data fetching
    #[cfg(feature = "ssr")]
    let data = create_resource(|| (), |_| async {
        fetch_from_database().await
    });
    
    // Client-only event handling
    #[cfg(feature = "hydrate")]
    {
        create_effect(move |_| {
            window().on("resize", |_| {
                // Handle resize
            });
        });
    }
    
    view! { <div>/* ... */</div> }
}
```

### Pattern 2: Server Functions
```rust
#[server(GetUser, "/api")]
pub async fn get_user(id: String) -> Result<User, ServerFnError> {
    // This ONLY runs on server
    // Automatically creates API endpoint
    // Client gets generated fetch() call
    let user = database::get_user(&id).await?;
    Ok(user)
}

#[component]
fn UserProfile() -> impl IntoView {
    let user = create_resource(
        || (),
        |_| async { get_user("123".to_string()).await }
    );
    
    view! {
        <Suspense fallback=|| view! { "Loading..." }>
            {move || user.get().map(|u| view! { <div>{u.name}</div> })}
        </Suspense>
    }
}
```

### Pattern 3: Shared State (Correct)
```rust
// ✅ CORRECT: State survives hydration
#[component]
fn App() -> impl IntoView {
    // Signal created on both server and client
    let (count, set_count) = create_signal(0);
    
    // Provide context for child components
    provide_context(count);
    
    view! {
        <Counter />
    }
}

#[component]
fn Counter() -> impl IntoView {
    // Use context - works in both runtimes
    let count = use_context::<ReadSignal<i32>>().unwrap();
    
    view! { <div>{count}</div> }
}
```

---

## 🔍 Debugging Runtime Issues

### Check Active Features
```bash
# See what features are actually compiled
cargo build --features hydrate -vv 2>&1 | grep "feature="

# Verify WASM build
wasm-pack build --target web --features hydrate
```

### Runtime Detection
```rust
pub fn current_runtime() -> &'static str {
    #[cfg(all(feature = "ssr", not(feature = "hydrate")))]
    return "SSR";
    
    #[cfg(all(feature = "hydrate", not(feature = "ssr")))]
    return "Hydrate (client)";
    
    #[cfg(feature = "csr")]
    return "CSR";
    
    "Unknown"
}
```

### Log Lifecycle
```rust
#[component]
fn DebugComponent() -> impl IntoView {
    logging::log!("Component render start - Runtime: {}", current_runtime());
    
    create_effect(move |_| {
        logging::log!("Effect running - Runtime: {}", current_runtime());
    });
    
    view! { <div>"Check logs"</div> }
}
```

---

## 📝 Build Configuration

### Correct Cargo.toml
```toml
[package]
name = "frontend"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
leptos = { version = "0.6" }
leptos_axum = { version = "0.6", optional = true }

[features]
# Server-side rendering
ssr = [
    "leptos/ssr",
    "leptos_axum",
]

# Client-side hydration
hydrate = [
    "leptos/hydrate",
]

# Pure client-side rendering
csr = [
    "leptos/csr",
]

# Don't mix ssr+hydrate in default
default = []
```

### Correct Build Commands
```bash
# Build server binary
cargo build --release --features ssr --no-default-features

# Build client WASM
cargo leptos build --release
# OR manually:
wasm-pack build --target web --features hydrate --no-default-features
```

---

## 🔗 Related Rules

- **Canon Rule #23** (Provider Scope): Where providers can exist
- **Canon Rule #24** (Context Lifetime): When context is available
- **Canon Rule #25** (Effect Execution): How many times effects run

---

**Key Insight:** Leptos is not "one framework with options" - it's three distinct execution models sharing a syntax. Code that works in CSR may silently break in Hydrate, and vice versa.

---

**Last Updated:** 2025-01-07  
**Version:** 1.0
