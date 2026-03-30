# Canon Rule #279: UI Layer Must Not Be Aware of Behavior Layer

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13

**Category:** component-architecture
**Tags:** ui, behavior, layering, ssr
**Language:** EN

---

**Intro:**
When UI depends on behavior modules, architectural boundaries collapse and SSR determinism is compromised. Layers must remain strictly isolated.

**Problem:**
ui layer references behavior layer causing coupling and ssr issues

**Solution:**
enforce strict separation where ui communicates only via attributes and behavior reacts independently

**Signals:**
- behavior import in ui
- conditional runtime logic
- tight coupling

**Search Intent:**
how to separate ui and behavior layers

**Keywords:**
ui behavior separation architecture, ssr safe layering frontend, data attribute driven behavior, frontend decoupling patterns

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