# Canon Rule #147 — Reactive Closures Are Data, Not Structure

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** leptos, ssr, ui
**Version:** 1.0.0
**Date:** 2026-01-16

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
