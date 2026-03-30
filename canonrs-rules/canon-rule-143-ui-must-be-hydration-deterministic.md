# Canon Rule #143 — UI Must Be Hydration-Deterministic

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-01-16

**Category:** core-runtime
**Tags:** hydration, ssr, ui, determinism
**Language:** EN

---

**Intro:**
SSR hydration requires identical HTML between server and client. Reactive logic in rendered output creates mismatches and causes runtime failures.

**Problem:**
non deterministic ui output breaks hydration and causes mismatches

**Solution:**
render static deterministic html during ssr and defer behavior to events

**Signals:**
- hydration panic
- dom mismatch
- runtime error

**Search Intent:**
why hydration mismatch happens in leptos

**Keywords:**
leptos hydration deterministic ui, ssr csr mismatch html, hydration error leptos, deterministic rendering ssr

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