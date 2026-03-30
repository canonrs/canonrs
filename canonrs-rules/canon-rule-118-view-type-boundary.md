# Canon Rule #118: View<\_> Type Boundary Prohibition

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2025-01-21

**Category:** component-architecture
**Tags:** rust, types, view, inference
**Language:** EN

---

**Intro:**
Using View<_> outside component boundaries causes type inference explosions and compilation failures. Open inference propagates across the codebase, breaking type resolution.

**Problem:**
open view type inference causes variance cycles and compilation errors

**Solution:**
restrict view<_> to components and use impl into view or anyview elsewhere

**Signals:**
- variance cycle
- e0391 error
- type mismatch

**Search Intent:**
how to fix rust view type errors

**Keywords:**
rust view type inference, leptos anyview pattern, view generic error rust, type boundary leptos

---

## Critical Rule

**`View<_>` IS NOT A BOUNDARY TYPE**

`View<_>` with open inference MUST ONLY exist as direct return of `#[component]`.

Violation = Variance cycles / Inference explosions / Refactoring brittleness

---

## The Problem (Real Production Case)

### Symptom

```rust
// Change ONE line in workflow/demo.rs
fn render_step_buttons<F>(...) -> View<_> { ... }

// Result: 197 compilation errors across ENTIRE codebase
error[E0391]: cycle detected when computing variances
```

### Root Cause

`View<_>` with generic `_` creates **open type inference graph**:

- TypedBuilder propagates variance
- Generic functions propagate constraints
- Cross-file type unification fails
- Single change = cascading failure

---

## Prohibited Patterns

### ❌ FORBIDDEN #1: Helper Returns View<\_>

```rust
// ❌ WRONG - Open inference boundary
fn render_step_buttons<F>(status: RwSignal<StepStatus>, on_change: F) -> View<_>
where
    F: Fn(StepStatus) + 'static + Copy,
{
    let content: View<_> = match status.get() {
        Status::A => view! { <Button>A</Button> },
        Status::B => view! { <Button>B</Button> },
    };
    content
}
```

**Why it breaks:**

- `View<_>` forces global inference
- `F` generic propagates to TypedBuilder
- ButtonPropsBuilder → HeaderPropsBuilder → cycle

---

### ❌ FORBIDDEN #2: View<\_> in Intermediates

```rust
// ❌ WRONG - Type leaks across match arms
let content: View<_> = match x {
    A => view! { <div/> },
    B => view! { <span/> },
};
```

**Why it breaks:**

- `View<Div>` ≠ `View<Span>`
- Compiler cannot unify
- Error: mismatched types

---

### Forbidden #3: Vec View

```rust
// ❌ WRONG - Incompatible collection type
let mut result: Vec<View> = Vec::new();
result.push(view! { <div/> }.into_view());
```

**Why it breaks:**

- `View<View<HtmlElement<Div>>>` ≠ `View`
- Double wrapping
- Expected `View`, found `View<View<...>>`

---

### ❌ FORBIDDEN #4: View<\_> in Closures

```rust
// ❌ WRONG - Closure return type leaks
let renderer = move || -> View<_> {
    view! { <div/> }
};
```

**Why it breaks:**

- Closure inference conflicts with component inference
- Type parameter escapes local scope

---

## Correct Patterns

### ✅ CORRECT #1: Component Returns View<\_>

```rust
// ✅ ALLOWED - Component is inference boundary
#[component]
pub fn MyComponent() -> impl IntoView {
    view! { <div>"Hello"</div> }
}
```

**Why it works:**

- `#[component]` macro closes inference
- `impl IntoView` is opaque boundary
- No propagation beyond component

---

### Correct #2: Helper Returns impl IntoView

```rust
// ✅ CORRECT - Closed boundary
fn render_step_buttons(
    status: RwSignal<StepStatus>,
    on_change: Callback<StepStatus>,
) -> impl IntoView {
    move || match status.get() {
        Status::A => view! { <Button>A</Button> }.into_any(),
        Status::B => view! { <Button>B</Button> }.into_any(),
    }
}
```

**Why it works:**

- `impl IntoView` is opaque
- `move ||` captures without generics
- `.into_any()` unifies types
- No TypedBuilder propagation

---

### Correct #3: Match with AnyView

```rust
// ✅ CORRECT - Unified type
move || match x {
    A => view! { <div/> }.into_any(),
    B => view! { <span/> }.into_any(),
}
```

**Why it works:**

- `AnyView` is unified type
- All arms return same concrete type
- No inference needed

---

### Correct #4: Vec AnyView

```rust
// ✅ CORRECT - Homogeneous collection
let mut result: Vec<AnyView> = Vec::new();
result.push(view! { <div inner_html={h}></div> }.into_any());
```

**Why it works:**

- `AnyView` is concrete type
- `.into_any()` converts explicitly
- No double wrapping

---

### Correct #5: Closure Returns AnyView

```rust
// ✅ CORRECT - Closed return type
let renderer = move || -> AnyView {
    view! { <div/> }.into_any()
};
```

**Why it works:**

- `AnyView` is explicit
- No inference leakage

---

## Decision Matrix

| Context                 | Allowed Return Type     | Pattern                              |
| ----------------------- | ----------------------- | ------------------------------------ |
| `#[component]` function | `impl IntoView`         | Direct `view! {}`                    |
| Helper function         | `impl IntoView`         | `move \|\| match { ... }.into_any()` |
| Match arm               | `AnyView`               | `.into_any()` each arm               |
| Vec collection          | `Vec<AnyView>`          | `.into_any()` before push            |
| Closure                 | `AnyView` or `impl ...` | Explicit type annotation             |
| Intermediate variable   | ❌ NEVER `View<_>`      | Use closure or `AnyView`             |

---

## Real Production Error (Your Case)

### Error

```
error[E0391]: cycle detected when computing the variances for items in this crate
note: ...which requires computing function signature of `render_step_buttons`
note: ...which requires computing the variances of `ButtonPropsBuilder`
note: ...which again requires computing the variances for items in this crate
```

### Broken Code

```rust
// workflow/demo.rs:69
fn render_step_buttons<F>(status: RwSignal<StepStatus>, on_change: F) -> View<_>
where
    F: Fn(StepStatus) + 'static + Copy,
{
    let content: View<_> = match status.get() {
        // ...
    };
    content
}
```

### Fix

```rust
fn render_step_buttons(
    status: RwSignal<StepStatus>,
    on_change: Callback<StepStatus>,
) -> impl IntoView {
    move || match status.get() {
        StepStatus::Completed => view! {
            <Button on_click=move |_| { on_change.run(StepStatus::Pending); }>
                "Reset"
            </Button>
        }.into_any(),
        StepStatus::Pending => view! {
            <Button on_click=move |_| { on_change.run(StepStatus::Active); }>
                "Start"
            </Button>
        }.into_any(),
        // ... other arms with .into_any()
    }
}
```

### Why Fix Works

1. ❌ Removed generic `F` → ✅ concrete `Callback<StepStatus>`
2. ❌ Removed `View<_>` return → ✅ `impl IntoView`
3. ❌ Removed intermediate `let content: View<_>` → ✅ `move ||`
4. ❌ Removed implicit inference → ✅ explicit `.into_any()`

Result: **197 errors → 0 errors**

---

## Validation Checklist

### Static Analysis

```bash
# Find violations
rg "-> View<_>" src/ --type rust | grep -v "#\[component\]"
rg "let.*: View<_>" src/ --type rust
rg "Vec<View>" src/ --type rust
```

### CI Check (Recommended)

```bash
#!/bin/bash
# .github/workflows/canon-rule-116.sh

violations=$(rg "-> View<_>" src/ --type rust | grep -v "#\[component\]" | wc -l)

if [ $violations -gt 0 ]; then
    echo "❌ Canon Rule #116 violated: View<_> used outside component"
    exit 1
fi
```

---

## Migration Strategy

### Phase 1: Audit

```bash
# Find all View<_> usage
rg "View<_>" src/ --type rust -l > view_violations.txt
```

### Phase 2: Isolate

```rust
// Temporary feature flag
#[cfg(not(feature = "strict_view_boundary"))]
mod legacy {
    // Old code with View<_>
}
```

### Phase 3: Refactor

Priority order:

1. **Helpers** → `impl IntoView`
2. **Match/If** → `AnyView`
3. **Collections** → `Vec<AnyView>`
4. **Closures** → explicit `AnyView`

### Phase 4: Enforce

```toml
# Cargo.toml
[features]
default = ["strict_view_boundary"]
strict_view_boundary = []
```

---

## Related Rules

This rule **closes the gap** between:

- **#54 Render Must Be Total** - prevents runtime panics → this prevents compile-time explosions
- **#63 Leptos Reactivity Closures** - defines when to access signals → this defines what to return
- **#86 Children/ChildrenFn Contract** - defines ownership boundaries → this defines type boundaries
- **#13 Specialization vs Substitution** - prevents over-generics → this prevents type leakage
- **#43/44 Domain Components/Orchestrators** - separates concerns → this enforces separation structurally

---

## Quick Reference Card

```rust
// ✅ ALWAYS SAFE
#[component]
pub fn Foo() -> impl IntoView { view! {} }

fn helper() -> impl IntoView {
    move || view! {}.into_any()
}

let items: Vec<AnyView> = vec![
    view! {}.into_any(),
];

// ❌ NEVER ALLOWED
fn helper() -> View<_> { ... }           // ❌ Open inference
let x: View<_> = match { ... };          // ❌ Type leakage
let items: Vec<View> = vec![];           // ❌ Incompatible collection
let f = || -> View<_> { ... };           // ❌ Closure escape
```

---

**Enforcement Level:** CRITICAL
**Last Updated:** 2025-01-21
**Status:** Active immediately

This rule is **non-negotiable** for production stability.