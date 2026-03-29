# Canon Rule #102: Runtime JS Is Shell Infrastructure

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** interactive, behavior
**Version:** 1.0.0  
**Date:** 2026-01-15

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
