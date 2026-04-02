# Canon Rule #303: No Random or Time-Based Rendering

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** core-runtime
**Tags:** hydration, determinism, ssr
**Language:** EN

---

**Intro:**
Rendering must be deterministic between SSR and client. Randomness or time-based values introduce divergence and break hydration.

**Problem:**
render output differs between server and client

**Solution:**
remove randomness or isolate to client-only execution

**Signals:**
- hydration mismatch
- flickering UI
- inconsistent DOM

**Search Intent:**
how to prevent random rendering issues in ssr

**Keywords:**
ssr determinism, hydration mismatch random, time rendering bug, frontend consistency

---

## Principle

Rendering must be deterministic.

---

## Problem

Random or time-based rendering:

- produces different DOM trees
- breaks hydration alignment
- causes runtime panic
- invalidates SSR guarantees

This is a structural violation of Canon runtime.

---

## Patterns

### Forbidden Pattern

```
let id = rand::random();
let now = Date::now();
```

---

### Canonical Pattern

```
let id = stable_id;
```

```
if is_client() {
  generate_runtime_value();
}
```

---

## Contract

### Enforcement

- no randomness in SSR
- no time-based rendering in SSR
- all values must be deterministic

---

### Exceptions

Client-only islands.

---

## Version History

- 1.0.0 — Initial definition
