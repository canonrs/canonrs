# Canon Rule #216: App Components Must Not Define HTML Structure

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-02-03

**Category:** component-architecture
**Tags:** html, ssr, components, structure
**Language:** EN

---

**Intro:**
Embedding document structure in components causes invalid markup and hydration failures. HTML boundaries must remain in shell.

**Problem:**
components define html structure causing invalid ssr output

**Solution:**
limit html document structure to shell and keep components content only

**Signals:**
- invalid html
- hydration error
- meta failure

**Search Intent:**
how to prevent html structure in components

**Keywords:**
ssr html structure separation, component document boundary, frontend html shell pattern, leptos invalid html fix

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