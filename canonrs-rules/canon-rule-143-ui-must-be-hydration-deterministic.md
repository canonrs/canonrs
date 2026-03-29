# Canon Rule #143 — UI Must Be Hydration-Deterministic

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** hydration, ssr, ui
**Version:** 1.0.0
**Date:** 2026-01-16

---
## Context
SSR + Hydration requires **bit-for-bit identical HTML** between server and client at hydration time.

Reactive closures or runtime-dependent logic inside rendered markup cause fatal hydration mismatches.

## Rule
UI components **MUST render deterministic, static HTML during SSR**.

Reactive logic **MUST NOT** influence rendered structure or text during hydration.

## Forbidden Patterns
```rust
{move || signal.get()}
{match context.get() { ... }}
{if runtime_condition { ... }}
```

## Allowed Patterns
- Static literals (`"🌓"`)
- CSS-driven state via `data-*` attributes
- Event handlers mutating state **after hydration**

## Correct Pattern
```rust
<Button on_click=toggle>
    "🌓"
</Button>
```

## Rationale
- Prevents hydration panics
- Ensures SSR safety
- Keeps UI predictable
- Separates rendering from behavior

## Scope
- UI components
- Blocks
- Layouts

## Exception
None.

Hydration determinism is non-negotiable.
