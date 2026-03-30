# Canon Rule #238: Workspace Dependency Graph Must Be Acyclic

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-02-13

**Category:** component-architecture
**Tags:** dependencies, graph, architecture, workspace
**Language:** EN

---

**Intro:**
Circular dependencies break build stability and violate architectural layering. Dependency graph must remain acyclic.

**Problem:**
dependency cycles exist causing instability and architectural violations

**Solution:**
enforce directed acyclic graph with strict dependency direction

**Signals:**
- cycle detected
- build instability
- resolution error

**Search Intent:**
how to fix circular dependency rust workspace

**Keywords:**
rust dependency cycle fix, workspace graph acyclic, cargo dependency cycle error, frontend architecture layering

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