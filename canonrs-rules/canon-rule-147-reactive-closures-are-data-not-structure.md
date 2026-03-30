# Canon Rule #147 — Reactive Closures Are Data, Not Structure

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-01-16

**Category:** state-reactivity
**Tags:** closures, reactivity, ssr, ui
**Language:** EN

---

**Intro:**
Reactive closures evaluated differently in SSR and CSR cause structural instability when used for rendering decisions. They must be limited to data binding only.

**Problem:**
reactive closures used for structure cause hydration mismatch and instability

**Solution:**
restrict reactive closures to data binding and use Show for structure

**Signals:**
- hydration mismatch
- unstable dom
- render inconsistency

**Search Intent:**
how to use reactive closures correctly leptos

**Keywords:**
reactive closures leptos rules, data binding vs structure, leptos show conditional rendering, hydration safe closures

---

## Context
Reactive closures in Leptos are evaluated at different times in SSR and CSR.
Using them to control DOM structure or textual nodes creates instability.

## Rule
Reactive closures **MUST ONLY be used for data binding**, never for:
- Conditional rendering
- Structural decisions
- Text node substitution

## Allowed Usage
```rust
<input value={move || value.get()} />
```

## Forbidden Usage
```rust
<div>
    {move || if open.get() { view! { <Panel/> } } }
</div>
```

## Correct Pattern
```rust
<Show when=move || open.get()>
    <Panel />
</Show>
```

## Rationale
- Preserves DOM shape
- Aligns SSR and CSR execution
- Makes intent explicit
- Prevents hydration panics

## Scope
- UI
- Components
- Blocks

## Exception
None.

Reactive closures bind values, not structure.