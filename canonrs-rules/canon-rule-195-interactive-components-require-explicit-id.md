# Canon Rule #195: Interactive Components Require Explicit ID

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-02-02

**Category:** component-architecture
**Tags:** components, hydration, id, behavior
**Language:** EN

---

**Intro:**
Auto-generated or optional IDs break SSR and CSR determinism, causing hydration mismatches and unreliable behavior attachment. Stable identifiers must be explicit.

**Problem:**
interactive components use non deterministic or optional ids causing hydration and behavior issues

**Solution:**
require explicit deterministic id prop provided by consumer for all interactive components

**Signals:**
- hydration mismatch
- missing id
- behavior attach fail

**Search Intent:**
how to fix hydration issues with component ids

**Keywords:**
deterministic id ssr csr, component id requirement pattern, hydration id mismatch, frontend behavior registry id

---

## Principle

**Interactive components that attach client-side behaviors MUST require an explicit `id` prop without default value or automatic generation.**

- ID must be deterministic across SSR and CSR
- ID must be provided by the component consumer
- No UUID, no global counters, no automatic fallbacks

---

## Problem

What breaks **without** this rule:

- **Hydration mismatch**: Auto-generated IDs differ between server and client render passes
- **Non-deterministic behavior attachment**: Registry cannot reliably match DOM elements to behaviors
- **Conditional rendering breaks**: Components rendered conditionally get different IDs on SSR vs CSR
- **Testing fragility**: Non-deterministic IDs make assertions impossible
- **Accessibility violations**: `aria-labelledby` and form associations break with changing IDs

---

## Forbidden Pattern

### Forbidden
```rust
// Auto-generated ID with UUID
#[component]
pub fn Combobox(
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let id = id.unwrap_or_else(|| {
        format!("combobox-{}", uuid::Uuid::new_v4())
    });
    // ...
}

// Auto-generated ID with global counter
static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[component]
pub fn Combobox(
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let id = id.unwrap_or_else(|| {
        format!("combobox-{}", COUNTER.fetch_add(1, Ordering::Relaxed))
    });
    // ...
}

// Default empty string
#[component]
pub fn Combobox(
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    // Behavior registry skips elements with empty id
}
```

Why this violates the rule:
- UUID changes between SSR and CSR
- Counter produces different sequences if render order changes
- Empty ID causes behavior registry to skip the element
- Developer gets no compile-time feedback that ID is required

---

## Canonical Pattern

### Canonical
```rust
#[component]
pub fn Combobox(
    id: String, // Required, no default, no Option<T>
    options: Vec<ComboboxOption>,
    #[prop(default = "Select...".to_string())] placeholder: String,
) -> impl IntoView {
    view! {
        <ComboboxPrimitive
            id=id
            data-combobox
        >
            // ...
        </ComboboxPrimitive>
    }
}

// Usage
view! {
    <Combobox id="user-role-select" options=roles />
}
```

Why this complies:
- Compiler enforces ID at call site
- Same ID in SSR and CSR (deterministic)
- Behavior registry reliably finds element
- Tests use predictable IDs
- Accessibility attributes reference stable IDs

---

## Rationale

Why this rule exists **architecturally**:

### Invariants it protects
- **SSR/CSR determinism**: Same component tree must produce identical DOM structure
- **Behavior attachment contract**: Registry depends on stable, unique IDs to match elements

### Contracts it enforces
- Component consumer declares intent (explicit ID = explicit control)
- No hidden side effects (no global state mutation)

### Bugs it prevents
- Hydration mismatches that cause silent failures or panics
- Race conditions in behavior initialization
- Flaky tests from non-deterministic IDs
- Broken accessibility relationships

### Why it is not opinion
SSR and CSR must produce identical markup for hydration to work. Any non-deterministic ID generation violates this fundamental constraint.

---

## Enforcement

How this rule is validated:

- **Code review**: All interactive components must have `id: String` (not `Option<String>`, not default)
- **Component checklist**: When creating UI with behavior, verify ID is required prop
- **Behavior registry**: Logs warning if element has `data-*` attribute but no `id`
- **Testing**: Integration tests fail if hydration mismatch occurs

Future enforcement:
- Lint rule to detect `#[prop(optional)] id` or `#[prop(default = ...)] id` in components with `data-*` attributes

---

## Exceptions

**No exceptions. This rule is absolute.**

If a component does not attach behaviors (e.g., pure presentational `<Button>` without `data-*` attribute), it does not need an `id` prop. But any component that participates in the behavior registry MUST require explicit ID.

---

## Version History

- **1.0.0** — Initial version (2026-02-02)