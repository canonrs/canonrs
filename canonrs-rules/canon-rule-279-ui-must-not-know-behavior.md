# Canon Rule #279: UI Layer Must Not Be Aware of Behavior Layer

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** behavior, ui, architecture
**Version:** 1.0.0  
**Date:** 2026-02-13

---

## Principle

**UI must never import, reference, register, or conditionally depend on Behavior modules.**

---

## Problem

If UI references Behavior:

- Layer boundaries collapse
- SSR determinism risks increase
- Runtime coupling emerges
- Testability degrades

---

## Forbidden Pattern

```rust
use crate::behavior::resize;

if resize_enabled {
    register_resize();
}
```

UI does not orchestrate runtime.

---

## Canonical Pattern

```rust
<XPrimitive
    attr:data-resizable="true"
/>
```

Behavior registry reacts independently at runtime.

---

## Rationale

Preserves:
- Strict layering
- Predictable SSR
- Independent evolution of runtime bridge

---

## Enforcement

- Dependency graph audit
- UI crate must not depend on behavior crate

---

## Exceptions

> No exceptions.

---

## Version History

- 1.0.0 — Initial version
