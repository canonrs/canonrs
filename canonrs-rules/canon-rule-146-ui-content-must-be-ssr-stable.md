# Canon Rule #146 — UI Content Must Be SSR-Stable

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** ssr, hydration, ui
**Version:** 1.0.0
**Date:** 2026-01-16

---
## Context
Leptos SSR performs DOM walking during hydration.
Any difference between server-rendered HTML and initial client HTML causes fatal hydration errors.

Dynamic UI content rendered via reactive closures inside structural nodes is a primary source of mismatch.

## Rule
UI components **MUST render SSR-stable content**.
Reactive closures **MUST NOT** be used to generate structural or textual content that differs between SSR and CSR.

## Mandatory Pattern
```rust
<Button>
    "🌓"
</Button>
```

State changes are handled via callbacks, not conditional rendering.

## Forbidden Patterns
```rust
<Button>
    {move || if theme.get() == Dark { "🌙" } else { "☀️" }}
</Button>
```

## Rationale
- Guarantees hydration safety
- Prevents unreachable panic
- Makes SSR deterministic
- Separates state mutation from render shape

## Scope
- UI components
- Blocks
- Layout shells

## Exception
None.

If content can differ, it must not be reactive.
