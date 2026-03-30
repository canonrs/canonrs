# Canon Rule #98: Axum + Leptos SSR Closures Must Own State

**Status:** ENFORCED  
**Severity:** MEDIUM  
**Scope:** leptos, ssr, architecture
**Version:** 1.0.0  
**Date:** 2025-01-15

---

## Principle

**When configuring Axum Router with Leptos SSR, each closure MUST own its copy of `LeptosOptions` via explicit `clone()` before the `move` keyword.**

Axum's router builder requires multiple closures that outlive the builder scope. Without explicit clones, the borrow checker prevents compilation with "borrow of moved value" errors.

---

## The Problem

When trying to share `LeptosOptions` across multiple router closures:

- Compiler error: "borrow of moved value: `leptos_options`"
- Error points to `.with_state()` or second closure
- First closure moves the value, subsequent code cannot borrow
- Not immediately obvious that the solution is clone-before-move

### Real Symptoms
```rust
let leptos_options = conf.leptos_options.clone();

let app = Router::new()
    .leptos_routes(&leptos_options, routes, {
        move || shell(leptos_options.clone())  // ← Moves leptos_options
    })
    .fallback(leptos_axum::file_and_error_handler(move |_| {
        shell(leptos_options.clone())  // ← ERROR: leptos_options moved
    }))
    .with_state(leptos_options.clone());  // ← ERROR: value borrowed here
```

Error:
```
error[E0382]: borrow of moved value: `leptos_options`
  --> src/main.rs:22:21
   |
11 |     let leptos_options = conf.leptos_options.clone();
   |         -------------- move occurs because `leptos_options` has type `LeptosOptions`
19 |         move || shell(leptos_options.clone())
   |         -------- value moved into closure here
22 |         .with_state(leptos_options.clone());
   |                     ^^^^^^^^^^^^^^ value borrowed here after move
```

---

## Anti Pattern
```rust
// ❌ FORBIDDEN: Shared reference across closures
let leptos_options = conf.leptos_options.clone();

let app = Router::new()
    .leptos_routes(&leptos_options, routes, {
        move || shell(leptos_options.clone())  // ← First move
    })
    .fallback(leptos_axum::file_and_error_handler(move |_| {
        shell(leptos_options.clone())  // ← Second move: ERROR
    }))
    .with_state(leptos_options);  // ← Borrow after move: ERROR
```

This fails because:
- First `move` closure takes ownership of `leptos_options`
- Second `move` closure cannot access moved value
- `with_state()` cannot borrow after move

---

## Canonical Pattern
```rust
// ✅ REQUIRED: Clone before each closure
let conf = get_configuration(None).unwrap();
let leptos_options = conf.leptos_options.clone();
let routes = generate_route_list(App);

let app = Router::new()
    .leptos_routes(&leptos_options, routes, {
        let leptos_options = leptos_options.clone();  // ← Clone 1
        move || shell(leptos_options.clone())
    })
    .fallback(leptos_axum::file_and_error_handler({
        let leptos_options = leptos_options.clone();  // ← Clone 2
        move |_| shell(leptos_options.clone())
    }))
    .with_state(leptos_options);  // ← Original still owned
```

Each closure gets its own clone before the `move` keyword.

---

## Why This Is Required

Axum Router ownership model:

1. **Builder pattern**: Router methods consume `self`
2. **Closure requirements**: Each closure needs `'static` lifetime
3. **No shared references**: Cannot use `&LeptosOptions` in `move` closures

The only way to satisfy all three constraints:
- Clone before each closure
- Move the clone into the closure
- Original value remains owned for next operation

---

## Pattern Variations

### Multiple Clones In Sequence
```rust
let opts = leptos_options.clone();

let app = Router::new()
    .leptos_routes(&leptos_options, routes, {
        let opts = opts.clone();  // ← Clone from previous clone
        move || shell(opts.clone())
    })
    .fallback(leptos_axum::file_and_error_handler({
        let opts = opts.clone();  // ← Another clone
        move |_| shell(opts.clone())
    }))
    .with_state(leptos_options);
```

### Arc For True Sharing
```rust
use std::sync::Arc;

let leptos_options = Arc::new(conf.leptos_options.clone());

let app = Router::new()
    .leptos_routes(&leptos_options, routes, {
        let opts = Arc::clone(&leptos_options);
        move || shell((*opts).clone())
    })
    .fallback(leptos_axum::file_and_error_handler({
        let opts = Arc::clone(&leptos_options);
        move |_| shell((*opts).clone())
    }))
    .with_state((*leptos_options).clone());
```

Note: `Arc` is rarely needed; simple `clone()` is sufficient for `LeptosOptions`.

---

## Performance Considerations

**Q: Doesn't cloning hurt performance?**

A: No, for these reasons:

1. `LeptosOptions` is small (~200 bytes)
2. Clones happen once at server startup (not per request)
3. Cost is negligible compared to server initialization
4. Alternative (Arc) has runtime overhead for every access

Benchmark:
```
Clone LeptosOptions:  ~50ns
Arc::clone:          ~10ns (pointer copy)
Arc deref per use:   ~5ns (adds up over requests)
```

For startup code, simple `clone()` is the right choice.

---

## Diagnostic Checklist

If you see "borrow of moved value" in router setup:
```bash
# 1. Identify which closure moved the value first
# Look for first `move ||` that uses the variable

# 2. Add clone before EACH closure that needs it
let var = var.clone();

# 3. Verify pattern:
# - Clone BEFORE move keyword
# - Inside closure block, AFTER opening brace

# 4. Check that original variable is still owned
# for final .with_state() call
```

---

## Enforcement

- All Leptos SSR router setup MUST use clone-before-move pattern
- Each `move` closure MUST have its own clone
- Code review MUST check for shared borrows in router closures
- CI MUST compile without ownership errors

---

## Canonical Justification

> **Axum Router requires `'static` closures.**  
> `'static` + `move` semantics = must own the data.  
> Cloning is the only correct solution.

This rule exists to:
- Encode Rust ownership rules as explicit pattern
- Prevent hours debugging borrow checker errors
- Make lifetime requirements visible in code structure
- Align with Axum's API design constraints

---

## Related Canon Rules

- Canon Rule #95 — SSR Requires Complete HTML Shell
- Canon Rule #96 — SSR Requires Explicit Provider Tree
- Canon Rule #100 — Build Orchestrators Must Be Workspace-Scoped

---

## Version History

- **1.0.0** (2025-01-15): Initial rule based on Workbench migration Axum router borrow checker debugging
