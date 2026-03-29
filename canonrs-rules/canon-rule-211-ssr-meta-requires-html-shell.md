# Canon Rule #211: SSR Meta Requires Explicit HTML Shell

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** ssr, ui
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**When using `leptos_meta` in SSR, an explicit HTML shell MUST be provided.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Without an explicit shell:

- Runtime panics occur
- Meta tags cannot resolve a `<head>`
- HTML structure becomes ambiguous
- Errors surface only at runtime

---

## Forbidden Pattern

### ❌ Forbidden

```rust
provide_meta_context();
view! { <Title/> <Meta/> <Body/> }
```

No enclosing HTML document.

---

## Canonical Pattern

### ✅ Canonical

```rust
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
```

---

## Rationale

Meta rendering requires a concrete document boundary.

- Head/body separation is mandatory
- Implicit containers are invalid
- SSR must be structurally explicit

---

## Enforcement

- SSR builds must expose `shell()`
- Missing shell fails review
- Runtime panic considered violation

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
