# Canon Rule #216: App Components Must Not Define HTML Structure

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** ui, ssr
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**Application components MUST NOT define `<html>`, `<head>`, or `<body>` structure.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

When components define HTML structure:

- Meta injection breaks
- Hydration becomes undefined
- Multiple head/body contexts appear
- SSR output becomes invalid HTML

---

## Forbidden Pattern

### Forbidden

```rust
#[component]
fn App() -> impl IntoView {
    view! {
        <Html/>
        <Head/>
        <Body>...</Body>
    }
}
```

Structural HTML inside UI components.

---

## Canonical Pattern

### Canonical

```rust
#[component]
fn App() -> impl IntoView {
    view! {
        <Title/>
        <Meta/>
        <Router/>
    }
}
```

Structural HTML belongs to the shell only.

---

## Rationale

HTML structure is a document-level concern.

- UI components describe content, not documents
- SSR shells own the document boundary
- Prevents invalid markup and runtime panics

---

## Enforcement

- Static scan for `<html>` in components
- SSR runtime validation
- Review enforcement

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
