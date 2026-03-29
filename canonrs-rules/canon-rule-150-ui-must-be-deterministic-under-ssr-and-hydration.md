# Canon Rule #150 — UI Must Be Deterministic Under SSR and Hydration

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** ssr, hydration, ui
**Version:** 1.0.0
**Date:** 2026-01-16

---
## Context
SSR and hydration require **byte-for-byte compatible HTML**.
Any divergence between server render and client initial render results in hydration failure.

Dynamic logic inside UI render paths breaks this contract.

## Rule
UI rendering **MUST be fully deterministic** during:
- SSR render
- Initial hydration

UI **MUST NOT** depend on:
- runtime-only values
- reactive closures in render output
- client-only signals during SSR

## Forbidden Pattern
```rust
view! {
    <Button>{move || theme.get_icon()}</Button>
}
```

## Correct Pattern
```rust
view! {
    <Button>"🌓"</Button>
}
```

Dynamic behavior must be handled **after hydration**, via explicit events.

## Rationale
- Prevents hydration panic
- Guarantees SSR correctness
- Makes rendering predictable
- Enforces Canon SSR discipline

## Scope
- UI
- Components
- Layouts

## Exception
None.

If it renders, it must render identically everywhere.
