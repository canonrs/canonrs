# Canon Rule #102: Runtime JS Is Shell Infrastructure

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-01-15

**Category:** behavior
**Tags:** runtime, js, events, shell
**Language:** EN

---

**Intro:**
Embedding runtime JavaScript logic inside components breaks separation of concerns and leads to inconsistent behavior. Interaction logic like clipboard and event handling must be centralized.

**Problem:**
components directly execute runtime js causing coupling and inconsistent behavior

**Solution:**
move all runtime javascript to shell layer and keep components declarative

**Signals:**
- event duplication
- dom coupling
- inconsistent behavior

**Search Intent:**
how to separate runtime js leptos

**Keywords:**
leptos runtime js separation, shell infrastructure js, component event handling architecture, js behavior separation ui

---

## Principle

**Components declare intent. The shell executes effects.**

Runtime JavaScript (clipboard, drag, shortcuts) is infrastructure,  
not component logic.

---

## Rule

- Rust components MAY:
  - Render `data-*` attributes
  - Declare interaction intent

- Rust components MUST NOT:
  - Call clipboard APIs
  - Touch DOM imperatively
  - Attach JS listeners

- All runtime JS MUST live in the shell.

---

## Forbidden
```rust
on:click=move |_| navigator.clipboard.write_text(...)
```

---

## Required
```js
document.addEventListener("click", e => {
  const btn = e.target.closest("[data-copy-button]");
  if (!btn) return;
  navigator.clipboard.writeText(btn.dataset.copyText);
});
```

---

## Canonical Justification

> Runtime behavior is a deployment concern, not a component concern.

---