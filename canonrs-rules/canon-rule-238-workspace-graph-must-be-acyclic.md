# Canon Rule #238: Workspace Dependency Graph Must Be Acyclic

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** architecture, workspace
**Version:** 1.0.0
**Date:** 2026-02-13

---

## Principle

The workspace crate dependency graph must be a Directed Acyclic Graph (DAG).

No circular dependencies are allowed at any layer.

---

## Problem

Circular dependencies cause:

- Incremental build instability
- Cargo resolution errors
- Implicit feature activation
- Architectural boundary violations
- Logical layer inversion (UI importing products, etc.)

---

## Forbidden Pattern

```
canonrs-ui → canonrs-site
canonrs-site → canonrs-ui   ❌ cycle
```

Or transitive:

```
A → B → C → A  ❌
```

---

## Canonical Pattern

Layering example:

```
tokens-engine
   ↓
rs-design
   ↓
canonrs
   ↓
products/*
```

Dependency direction always flows downward.

Never upward.

---

## Enforcement

CI validation:

```bash
cargo metadata --format-version 1 \
| jq '.packages[].dependencies'
```

Static detection of back-references via graph traversal.

---

## Exceptions

None.

Architectural cycles are fatal errors.

---

## Version History

- 1.0.0 — Initial definition
