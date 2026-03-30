# Canon Rule #122: No Conditional Rendering with .then() Closures

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** leptos, components
**Version:** 1.0.0
**Date:** 2025-01-22

---

## Principle

**Conditional rendering MUST use `<Show>` component — never `condition.then(|| view!)`.**

---

## The Problem

Using `bool.then(|| view!)` for conditional rendering causes three structural failures:

1. **FnOnce violations** — closure captures values by move
2. **Type inference failures** — return type ambiguity
3. **SSR hydration mismatches** — inconsistent DOM structure

**Real symptoms from production:**
```rust
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
  --> src/components/modal.rs:23:9
   |
23 |         {open.then(|| view! {
   |          ^^^^ this closure implements `FnOnce`, not `Fn`
   |               because it moves the variable `content` out
```

**Additional symptoms:**
- Hydration warnings: "expected element, found nothing"
- Closures that work once, fail on re-render
- Type inference loops in reactive contexts

---

## Forbidden Patterns

### Forbidden: Using then for conditional views
```rust
#[component]
pub fn Modal(open: Signal<bool>, content: String) -> impl IntoView {
    view! {
        {open.get().then(|| view! {  // ❌ FORBIDDEN
            <div class="modal">
                {content}  // Moves `content` — FnOnce error
            </div>
        })}
    }
}
```

**Why this fails:**
- `.then()` returns `Option<T>`
- Closure captures `content` by move
- Closure is `FnOnce`, not `Fn`
- Cannot re-render when `open` changes

### Forbidden: Nested then conditions
```rust
#[component]
pub fn Dashboard(user: User) -> impl IntoView {
    view! {
        {user.is_admin.then(|| view! {  // ❌ First .then()
            {user.has_access.then(|| view! {  // ❌ Nested .then()
                <AdminPanel />
            })}
        })}
    }
}
```

**Why this fails:**
- Double FnOnce violation
- Type inference explosion
- Hydration structure unpredictable

### Forbidden: then with complex children
```rust
#[component]
pub fn List(items: Vec<Item>, show_details: bool) -> impl IntoView {
    view! {
        <ul>
            {items.into_iter().map(|item| {
                view! {
                    <li>
                        {item.name}
                        {show_details.then(|| view! {  // ❌ Inside iterator
                            <div>{item.description}</div>
                        })}
                    </li>
                }
            }).collect_view()}
        </ul>
    }
}
```

**Why this fails:**
- Closure per list item — each can fail independently
- `show_details` captured by move per item
- Becomes unmaintainable quickly

---

## Canonical Pattern

### Canonical: Use Show component
```rust
#[component]
pub fn Modal(open: Signal<bool>, content: String) -> impl IntoView {
    view! {
        <Show when=move || open.get()>  // ✅ CORRECT
            <div class="modal">
                {content}
            </div>
        </Show>
    }
}
```

**Why this works:**
- `<Show>` handles closure lifecycle correctly
- Content evaluated lazily when condition true
- No FnOnce issues
- SSR-safe hydration

### Canonical: Nested conditions with Show
```rust
#[component]
pub fn Dashboard(user: User) -> impl IntoView {
    let is_admin = user.is_admin;
    let has_access = user.has_access;
    
    view! {
        <Show when=move || is_admin>  // ✅ First condition
            <Show when=move || has_access>  // ✅ Nested condition
                <AdminPanel />
            </Show>
        </Show>
    }
}
```

**Why this works:**
- Each `<Show>` manages its own reactivity
- Clear nesting structure
- Predictable hydration

### Canonical: Conditional with fallback
```rust
#[component]
pub fn Content(user: Option<User>) -> impl IntoView {
    view! {
        <Show
            when=move || user.is_some()
            fallback=|| view! { <div>"Please log in"</div> }
        >
            <UserProfile user=user.unwrap() />
        </Show>
    }
}
```

**Why this works:**
- `fallback` provides else-branch semantically
- No manual `if/else` needed
- SSR renders correct branch

### Canonical: Multiple conditions
```rust
#[component]
pub fn StatusBadge(status: Signal<Status>) -> impl IntoView {
    view! {
        <Show when=move || matches!(status.get(), Status::Success)>
            <span class="badge-success">"Success"</span>
        </Show>
        <Show when=move || matches!(status.get(), Status::Error)>
            <span class="badge-error">"Error"</span>
        </Show>
        <Show when=move || matches!(status.get(), Status::Pending)>
            <span class="badge-pending">"Pending"</span>
        </Show>
    }
}
```

**Alternative using single <Show> with match:**
```rust
view! {
    {move || match status.get() {
        Status::Success => view! { <span class="badge-success">"Success"</span> }.into_any(),
        Status::Error => view! { <span class="badge-error">"Error"</span> }.into_any(),
        Status::Pending => view! { <span class="badge-pending">"Pending"</span> }.into_any(),
    }}
}
```

---

## Rationale

### Architectural Reasons

1. **Closure trait requirements:**
   - Leptos needs `Fn` for re-rendering
   - `.then()` creates `FnOnce` closures naturally
   - `<Show>` component designed for `Fn` semantics

2. **SSR + Hydration safety:**
   - `<Show>` generates consistent DOM markers
   - Client knows where to hydrate conditional content
   - `.then(|| view!)` has unpredictable structure

3. **Type system clarity:**
   - `<Show>` has explicit type signature
   - `.then()` relies on inference (often fails)
   - Compiler errors are clearer with `<Show>`

### What This Rule Protects

- **Closure correctness** — Fn vs FnOnce invariants
- **Hydration stability** — predictable DOM structure
- **Code maintainability** — explicit conditional semantics
- **Type inference** — reduces compiler workload

---

## Enforcement

### Static Analysis
```rust
// Linter pseudocode
for expr in view_macro_expressions {
    if expr.is_method_call("then") && expr.arg_is_view_macro() {
        emit_error!(
            "Canon Rule #122: Use <Show> for conditional rendering, not .then()"
        );
    }
}
```

### CI Check
```bash
# Grep for forbidden pattern
rg '\.then\(\|\|.*view!' packages-rust/rs-design/src/ && \
    echo "❌ Canon Rule #122: Replace .then() with <Show>" && exit 1
```

### Code Review Checklist

- [ ] No `.then(|| view! {...})` patterns
- [ ] All conditional rendering uses `<Show>`
- [ ] Nested conditions use nested `<Show>` or match expressions
- [ ] Fallback branches use `fallback=|| view! {...}`

---

## Exceptions

### Exception 1 Non view conditionals
```rust
// Conditional string (NOT a view) — ALLOWED
let label = if count > 10 { "Many" } else { "Few" };

view! {
    <div>{label}</div>  // ✅ OK — not conditional rendering
}
```

### Exception 2 Option map simple values
```rust
// Simple value mapping — ALLOWED
view! {
    <div>{user.map(|u| u.name)}</div>  // ✅ OK — maps to String, not view
}
```

### Exception 3 Match expressions
```rust
// Match on enum — ALLOWED (but prefer <Show> when possible)
view! {
    {move || match status.get() {
        Status::Loading => view! { <Spinner /> }.into_any(),
        Status::Error(e) => view! { <Error msg=e /> }.into_any(),
        Status::Success(data) => view! { <Content data=data /> }.into_any(),
    }}
}
```

**Prefer `<Show>` when possible, but match is acceptable for complex enums.**

---

## Migration Guide

### Before (Forbidden)
```rust
view! {
    {is_visible.then(|| view! {
        <div>"Content"</div>
    })}
}
```

### After (Canonical)
```rust
view! {
    <Show when=move || is_visible>
        <div>"Content"</div>
    </Show>
}
```

### Before (Nested)
```rust
view! {
    {user.is_some().then(|| view! {
        {user.unwrap().is_admin.then(|| view! {
            <AdminPanel />
        })}
    })}
}
```

### After (Canonical)
```rust
view! {
    <Show when=move || user.is_some()>
        <Show when=move || user.unwrap().is_admin>
            <AdminPanel />
        </Show>
    </Show>
}
```

---

## Quick Reference

| Pattern | Allowed? | Use Instead |
|---------|----------|-------------|
| `condition.then(\|\| view!)` | ❌ | `<Show when=..>` |
| `<Show when=..>` | ✅ | — |
| `if x { y } else { z }` (values) | ✅ | — |
| `match status { .. }` (views) | ⚠️ | Prefer `<Show>` |
| `option.map(\|x\| x.field)` | ✅ | — |

---

## Related Rules

- **Canon Rule #63:** Leptos Reactivity Closures (Fn vs FnOnce)
- **Canon Rule #121:** StoredValue for Non-Copy in view!
- **Canon Rule #90:** Hydration is DOM Replacement

---

## Version History

- **1.0.0** — Initial canonical version (2025-01-22)
