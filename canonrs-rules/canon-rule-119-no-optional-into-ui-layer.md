# Canon Rule #119: No #[prop(optional, into)] in UI Layer

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** components, design-system
**Version:** 1.1.0
**Date:** 2025-01-22

---

## Principle

**UI layer components MUST NOT use `#[prop(optional, into)]` — use explicit `Option<T>` types instead.**

---

## The Problem

The combination `#[prop(optional, into)]` creates catastrophic type ambiguity in Leptos:

1. **Compiler cannot infer** whether you're passing `T` or `Option<T>`
2. **Multiplies E0308 errors** across every call site
3. **Forces users to add `.into()` or `Some()` everywhere**
4. **Breaks intuitive API design** — users expect `Option<T>` to mean "optional"

**Real symptoms from production:**
```rust
error[E0308]: mismatched types
  --> src/ui/component.rs:42:31
   |
42 |         <Component class="foo" />
   |                           ^^^^^ expected `Option<String>`, found `&str`
```

This error appears **hundreds of times** when `#[prop(optional, into)]` is used in UI components.

---

## Forbidden Patterns

### Forbidden: Using prop optional into in UI Layer
```rust
// UI Component (FORBIDDEN)
#[component]
pub fn Button(
    children: Children,
    #[prop(optional, into)] class: Option<String>,  // ❌ NEVER DO THIS
    #[prop(optional, into)] id: Option<String>,     // ❌ NEVER DO THIS
) -> impl IntoView {
    view! {
        <ButtonPrimitive class={class.unwrap_or_default()}>
            {children()}
        </ButtonPrimitive>
    }
}

// User code — FAILS with E0308
view! {
    <Button class="my-class" />  // Error: expected Option<String>, found &str
}
```

**Why this fails:**
- Leptos cannot infer whether `"my-class"` should be `Some("my-class".into())` or `"my-class".into()`
- User must write `class=Some("my-class".to_string())` everywhere
- Completely breaks ergonomics

---

## Canonical Pattern

### Canonical: Explicit Option in UI
```rust
// UI Component (CORRECT)
#[component]
pub fn Button(
    children: Children,
    #[prop(optional)] class: Option<String>,  // ✅ Just Option<String>
    #[prop(optional)] id: Option<String>,     // ✅ Just Option<String>
) -> impl IntoView {
    view! {
        <ButtonPrimitive
            class={class}
            id={id}
        >
            {children()}
        </ButtonPrimitive>
    }
}

// User code — WORKS naturally
view! {
    <Button class="my-class".to_string() />  // ✅ Clean, explicit
    <Button class=Some("my-class".to_string()) />  // ✅ Also works
    <Button />  // ✅ None is implicit
}
```

**Why this works:**
- User explicitly converts `&str → String` when needed
- No type ambiguity at call site
- Component passes `Option` directly to Primitive
- Clean separation of concerns

---

## Rationale

### Architectural Reasons

1. **Type inference boundary:**
   - UI components are called by application code (uncontrolled context)
   - Application code should not pay type inference tax
   - Primitives handle pass-through without conversion (see Canon Rule #124)

2. **Ergonomics vs Safety:**
   - `#[prop(optional, into)]` trades call-site clarity for implementation convenience
   - In UI layer, this trade-off is backwards
   - Better: call site stays clear, Primitive handles pass-through

3. **Error multiplication:**
   - One `#[prop(optional, into)]` → 10-50 E0308 errors in application code
   - Each call site must add `.into()` or `Some()`
   - Violates locality principle

### What This Rule Protects

- **Call-site ergonomics** — users write natural code
- **Type clarity** — no ambiguous conversions
- **Compiler performance** — fewer inference attempts
- **Maintenance** — errors stay localized to UI layer

---

## Enforcement

### Static Analysis (Recommended)
```rust
// Validator pseudocode
for component in ui_layer_components {
    for prop in component.props {
        if prop.has_attribute("optional") && prop.has_attribute("into") {
            emit_error!(
                "Canon Rule #119: UI components cannot use #[prop(optional, into)]"
            );
        }
    }
}
```

### CI Check
```bash
# Grep for forbidden pattern in UI layer
rg '#\[prop\(optional.*into\)' packages-rust/rs-design/src/ui/ && exit 1
```

### Code Review Checklist

- [ ] No `#[prop(optional, into)]` in `src/ui/` directory
- [ ] All optional props use plain `Option<T>`
- [ ] Components pass `Option` directly to Primitives

---

## Exceptions

**No exceptions. This rule applies to ALL components in UI layer.**

**Note:** Primitives also CANNOT use `#[prop(into)]` per Canon Rule #124. This creates a consistent, zero-conversion architecture across all layers.

---

## Related Rules

- **Canon Rule #75:** Primitives have zero styling logic
- **Canon Rule #89:** Primitives are SSR-safe
- **Canon Rule #117:** Event handler boundaries
- **Canon Rule #124:** Primitive Contract Types (Primitives CANNOT use `#[prop(into)]`)

---

## Version History

- **1.1.0** — Removed Primitive exception, aligned with Canon Rule #124 (2025-01-22)
- **1.0.0** — Initial version with Primitive exception (2025-01-22)
