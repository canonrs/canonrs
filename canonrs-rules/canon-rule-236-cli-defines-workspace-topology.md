# Canon Rule #236: CanonRS CLI Defines Workspace Topology

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** cli, workspace, architecture
**Version:** 1.0.0  
**Date:** 2026-02-13  

---

## Principle

The CanonRS CLI is the single authority responsible for defining, generating, and reorganizing workspace topology.

Manual structural edits to the workspace are prohibited.

---

## Architectural Boundary

Workspace structure includes:

- `[workspace.members]`
- `[[workspace.metadata.leptos]]`
- product directories
- package directories
- naming conventions
- site-root definitions
- bin/lib package bindings
- feature isolation mappings

Only the CLI may modify these.

---

## Forbidden Pattern

Manual editing:

```toml
[workspace]
members = [
  "products/new-app"  # ❌ added manually
]
```

Manual folder manipulation:

```
mkdir products/new-app   # ❌ outside CLI
```

Manual leptos metadata injection:

```toml
[[workspace.metadata.leptos]]
name = "new-app"  # ❌ manual entry
```

---

## Canonical Flow

All structural changes must follow:

```
canonrs new product
canonrs new package
canonrs reorganize
canonrs sync
```

The CLI updates:

- Cargo.toml
- Leptos metadata
- directory structure
- feature boundaries
- naming compliance
- workspace determinism guarantees

---

## Rationale

Manual topology mutations cause:

- Broken cargo metadata
- Feature leakage
- Duplicate site-roots
- Port mismatches
- Non-deterministic builds
- SSR/CSR contamination
- Naming drift

Workspace topology is architecture, not configuration.

---

## Enforcement

- CI validates workspace integrity via `cargo metadata`
- CI checks that no manual diff exists outside CLI-managed blocks
- Workspace Cargo.toml may contain protected regions:
  
```toml
# --- BEGIN CLI MANAGED ---
# (auto-generated content)
# --- END CLI MANAGED ---
```

Editing inside managed blocks is a violation.

---

## Exceptions

Manual edits allowed only for:

- dependency version bumps (within policy)
- comments
- documentation sections

Structural changes: **No exceptions.**

---

## Related Rules

- Rule #222 — Workspace Members Must Be Fully Resolvable
- Rule #223 — Feature Flag Isolation
- Rule #224 — Naming Consistency
- Rule #232 — Deterministic Root Builds

---

## Version History

- **1.0.0** (2026-02-13) — Initial formalization of CLI authority
