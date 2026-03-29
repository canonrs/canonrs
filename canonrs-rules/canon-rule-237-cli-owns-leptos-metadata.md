# Canon Rule #237: CLI Owns Leptos Metadata Blocks

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** cli, workspace, leptos
**Version:** 1.0.0
**Date:** 2026-02-13

---

## Principle

All `[[workspace.metadata.leptos]]` blocks are generated and synchronized exclusively by the CanonRS CLI.

Manual edits are forbidden.

---

## Problem

Manual editing of leptos metadata causes:

- Port divergence (Rule #219)
- site-root collisions (Rule #220)
- Feature leaks (Rule #223)
- Path errors (Rule #226)
- Hydration mismatches
- Build nondeterminism

Leptos metadata defines runtime architecture, not convenience config.

---

## Forbidden Pattern

```toml
[[workspace.metadata.leptos]]
name = "canonrs-site"
site-addr = "127.0.0.1:3009"  # ❌ manual change
```

---

## Canonical Pattern

Leptos metadata exists inside a CLI managed region:

```toml
# --- BEGIN CLI MANAGED LEPTOS ---
[[workspace.metadata.leptos]]
name = "canonrs-site"
...
# --- END CLI MANAGED LEPTOS ---
```

Only the CLI modifies this region.

---

## Enforcement

- CI verifies no diff inside managed block.
- CLI regenerates metadata on `canonrs sync`.
- Build fails if workspace metadata checksum changes unexpectedly.

---

## Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
