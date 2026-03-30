# Canon Rule #131: Reactive Boundary Ownership

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2025-01-22

**Category:** state-reactivity
**Tags:** closure, ownership, storedvalue, reactivity
**Language:** EN

---

**Intro:**
Reactive boundaries re-execute closures multiple times, and moving non-Copy values into them causes FnOnce violations. This leads to compilation failures and unstable reactive behavior.

**Problem:**
non copy values moved into reactive boundaries cause fnonce errors

**Solution:**
clone simple values or use storedvalue for complex or stateful values

**Signals:**
- fnonce error
- closure move error
- ownership violation

**Search Intent:**
how to fix fnonce errors in

**Keywords:**
leptos reactive boundary ownership, storedvalue vs clone rust, closure ownership leptos, reactive closure fnonce error

---

## Principle

**Non-`Copy` values captured by reactive boundaries (`<Show>`, `<For>`, `<Suspense>`, `<Transition>`) MUST either:**
- **be explicitly cloned, OR**
- **use `StoredValue` when cloning is not appropriate**

**Sub-principle:** `StoredValue` is NOT for pass-through props. It is for expensive or stateful values that need stable identity across reactive executions.

---

## The Problem

Reactive boundaries create closures that may execute multiple times. When non-`Copy` values are moved into these closures:

1. Rust's ownership rules require `Fn` trait (callable multiple times)
2. Moving non-`Copy` values makes the closure `FnOnce` (callable once)
3. Leptos expects `Fn`, causing compilation failure
4. Error: `expected Fn, found FnOnce`

**Observable symptoms:**
- `error[E0525]: expected a closure that implements the Fn trait, but this closure only implements FnOnce`
- `closure is FnOnce because it moves the variable X out of its environment`
- Compilation failure, not runtime error

---

## Forbidden Patterns

### ❌ Forbidden — Moving String into Show
```rust
#[component]
pub fn TabsContent(
    children: ChildrenFn,
    class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    
    view! {
        <Show when=is_active>
            <TabsContentPrimitive class=class>  // ❌ Moves class
                {children()}  // ❌ Also moves children
            </TabsContentPrimitive>
        </Show>
    }
}
// Error: closure is FnOnce because it moves `class` and `children`
```

### ❌ Forbidden — Using StoredValue for Pass-Through Props
```rust
// ❌ ANTI-PATTERN! StoredValue is not for simple props
let class = StoredValue::new(class.unwrap_or_default());
let id = StoredValue::new(id.unwrap_or_default());

view! {
    <Show when=is_active>
        <Primitive 
            class=class.get_value()  // ❌ Wrong use of StoredValue
            id=id.get_value()        // ❌ Just clone instead!
        />
    </Show>
}
```

---

## Canonical Pattern

### ✅ Canonical — Clone for Strings
```rust
#[component]
pub fn TabsContent(
    children: ChildrenFn,
    class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    
    view! {
        <Show when=is_active>
            <TabsContentPrimitive class=class.clone()>  // ✅ Explicit clone
                {children()}
            </TabsContentPrimitive>
        </Show>
    }
}
```

### ✅ Canonical — StoredValue for ChildrenFn (Correct Use)
```rust
#[component]
pub fn Dialog(
    children: ChildrenFn,
    open: Signal<bool>,
) -> impl IntoView {
    let children = StoredValue::new(children);  // ✅ ChildrenFn needs StoredValue
    
    view! {
        <Show when=open>
            <DialogPrimitive>
                {children.get_value()()}  // ✅ Call stored function
            </DialogPrimitive>
        </Show>
    }
}
```

### ✅ Canonical — Copy Types (No Extra Work)
```rust
#[component]
pub fn Tabs(
    active_tab: Signal<usize>,  // ✅ usize is Copy
) -> impl IntoView {
    view! {
        <Show when=move || active_tab.get() > 0>
            // ✅ No clone needed - usize is Copy
        </Show>
    }
}
```

---

## Rationale

**The problem:**
Reactive boundaries (`<Show>`, `<For>`, etc.) create closures that Leptos may call multiple times. Rust requires these closures to be `Fn`, which means they cannot move non-`Copy` values out of their environment.

**Why clone is the default solution:**
- Strings are cheap to clone (Rc-based internally)
- Explicit cloning documents intent clearly
- Preserves ownership semantics
- No runtime overhead for reactive tracking

**When StoredValue is justified:**
`StoredValue` should be used ONLY when:
1. The value is expensive to clone (e.g., large `Vec`, complex structs)
2. The value needs stable identity across reactive executions
3. The value is captured by multiple independent closures
4. The value is a closure itself (e.g., `ChildrenFn`, `Callback`)

**When StoredValue is WRONG:**
- ❌ Pass-through props (`class`, `id`, `aria-*`)
- ❌ Simple strings or primitives
- ❌ As a "quick fix" for ownership errors
- ❌ To avoid thinking about ownership

**Critical insight:**
Using `StoredValue` for simple props creates false complexity and obscures data flow. It makes code harder to understand and maintain. Always prefer `clone()` unless you have a specific, documented reason to use `StoredValue`.

---

## Enforcement

**Compiler:**
- Rust's type system automatically catches violations
- Error messages explicitly state "`FnOnce` vs `Fn`"

**Code Review:**
- **CRITICAL**: Challenge every `StoredValue` usage—demand justification
- Ensure `clone()` is used for strings in reactive boundaries
- Verify `ChildrenFn` and expensive types justify `StoredValue`

**Linter:**
- Detect unnecessary `StoredValue` for `String`, `Option<String>`, primitives
- Flag missing `clone()` for `String` in reactive boundaries
- Warn on `StoredValue` without accompanying comment explaining why

---

## Exceptions

**Exception 1: Copy Types**
No special handling needed for `Copy` types:
```rust
let count: i32 = 42;
view! {
    <Show when=is_active>
        <div>{count}</div>  // ✅ Copy type - no clone needed
    </Show>
}
```

**Exception 2: Owned Values Consumed Once**
If the value is only used once and not in a reactive boundary:
```rust
view! {
    <Primitive class=class>  // ✅ Direct move - no boundary
        "Content"
    </Primitive>
}
```

**Exception 3: Closures and Functions**
`ChildrenFn`, `Callback`, and other function types MUST use `StoredValue`:
```rust
let children = StoredValue::new(children);  // ✅ Function type - justified
```

---

## Version History

- **1.0.0** — Initial canonical version (2025-01-22)