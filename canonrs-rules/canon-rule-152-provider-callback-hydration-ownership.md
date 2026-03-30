# Canon Rule 152 — Provider Callback Hydration Ownership

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-01-16

**Category:** core-runtime
**Tags:** hydration, callbacks, providers, wasm
**Language:** EN

---

**Intro:**
Heap-owned callbacks capturing provider context can be dropped before hydration reattachment, causing runtime panics. This breaks deterministic hydration in SSR environments.

**Problem:**
callbacks capturing provider context are dropped before hydration causing runtime errors

**Solution:**
use inline event handlers or hydration safe callback mechanisms

**Signals:**
- callback removed
- unreachable panic
- hydration failure

**Search Intent:**
how to fix provider callback hydration errors

**Keywords:**
leptos provider callback error, hydration ownership issue, callback dropped before attach, wasm hydration panic

---

## Status

**Mandatory**

## Context

In SSR + Hydration environments (Leptos/Tachys), callbacks attached to DOM events
are **reattached** during hydration. If a callback is **heap-owned** (`Callback::new`)
and captures a **Provider context**, it may be **dropped before reattachment**,
causing runtime panics such as:

callback removed before attaching
unreachable

pgsql
Copiar código

This is not a visual mismatch issue.
It is a **lifetime + ownership violation** during hydration.

## Rule

**Callbacks that capture Provider context MUST NOT be created via `Callback::new`
inside SSR execution paths.**

Instead, they must be attached using **inline `on:event` handlers**
or created via hydration-safe mechanisms.

## Allowed Patterns

### App / Layout Layer (preferred)

```rust
<Button
    on:click=move |_| {
        let current = theme_ctx.mode.get();
        theme_ctx.set_mode(next_mode(current));
    }
>
    "🌓"
</Button>
on:event handlers are hydration-safe

Runtime owns attachment lifecycle

No heap-owned callback is dropped prematurely

Stable Callback (advanced)
rust
Copiar código
let toggle = use_callback(move |_| {
    theme_ctx.set_mode(...);
});
Allowed when abstraction is required

Must be explicitly hydration-stable

Forbidden Patterns ❌
rust
Copiar código
let toggle = Callback::new(move |_| {
    theme_ctx.set_mode(...);
});
When:

Used in App/Layout/Shell code

Captures Provider context

Runs in SSR + hydrate environments

This WILL cause hydration panics.

Scope
Applies to:

ThemeProvider

DensityProvider

LanguageProvider

LayoutProvider

Any CanonRS Provider

Rationale
Providers are resolved during hydration

Heap-owned callbacks may be dropped before DOM reattachment

on:event keeps ownership inside the runtime

Prevents non-deterministic hydration crashes

Canonical Guidance
App-layer: use on:event

UI Components: may expose Callback props

Providers: must never be mutated by unstable callbacks

Primitives: never own callbacks

Outcome
Following this rule guarantees:

Deterministic hydration

No Tachys runtime panics

Correct Provider ownership semantics

This rule is non-negotiable for SSR-safe CanonRS applications.
```