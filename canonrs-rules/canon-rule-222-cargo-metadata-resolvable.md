# Canon Rule #222: Workspace Members Must Be Fully Resolvable by Cargo Metadata

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** workspace, build
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**All paths in `[workspace.members]` MUST resolve to valid `Cargo.toml` files and appear in `cargo metadata --format-version 1` output.**

---

## Problem

Without resolvable members:
- `cargo build --workspace` fails silently or partially
- IDE tooling (rust-analyzer) cannot find crates
- `cargo test --workspace` skips broken members
- Dependency graph is incomplete, causing version conflicts

**Observable symptoms**:
```bash
cargo metadata --format-version 1 | jq '.workspace_members | length'
4  # Expected 5 members

cargo build --workspace
error: package `canonrs-site` not found in workspace
```

---

## Forbidden Pattern

### Forbidden
```toml
[workspace]
members = [
    "canonrs-site",                    # ❌ Missing 'products/' prefix
    "packages-rust/canonrs-ssr",       # ❌ Missing 'rs-canonrs/' level
    "products/canonrs-workbench/src",  # ❌ Points to src/, not root
]
```

**Why this violates**: Cargo cannot find `Cargo.toml` at these paths from workspace root.

**Validation**:
```bash
ls canonrs-site/Cargo.toml
# ls: cannot access 'canonrs-site/Cargo.toml': No such file or directory
```

---

## Canonical Pattern

### Canonical
```toml
[workspace]
members = [
    "packages-rust/rs-canonrs/canonrs",
    "packages-rust/rs-canonrs/canonrs-ssr",
    "packages-rust/rs-canonrs/canonrs-csr",
    "packages-rust/rs-canonrs/canonrs-shared",
    "products/canonrs-site",
    "products/canonrs-workbench",
]
resolver = "2"
```

**Why this complies**: Each path points to directory containing `Cargo.toml`.

**Validation**:
```bash
for member in $(grep -A 10 'members =' Cargo.toml | grep '"' | tr -d '", '); do
    if [[ ! -f "$member/Cargo.toml" ]]; then
        echo "❌ Invalid member: $member"
        exit 1
    fi
done
```

---

## Rationale

### Architectural invariants
1. **Workspace integrity**: Cargo must know all crates to resolve deps correctly
2. **Tool compatibility**: rust-analyzer, cargo-leptos, CI all depend on metadata
3. **Build reproducibility**: Missing members cause non-deterministic builds

### Bugs prevented
- Partial builds (some crates ignored)
- IDE autocomplete failures (crates not indexed)
- Version resolution errors (incomplete dependency graph)
- CI cache misses (workspace structure unknown)

### Why not opinion
`cargo metadata` is the canonical source of truth for Rust tooling. Invalid members break the entire ecosystem contract.

---

## Enforcement

### CI validation (strict)
```bash
#!/bin/bash
# validate-workspace-members.sh

# Get expected members from Cargo.toml
expected=$(grep -A 50 'members =' Cargo.toml | grep '"' | tr -d '", ' | sort)

# Get actual members from cargo metadata
actual=$(cargo metadata --format-version 1 --no-deps | \
         jq -r '.workspace_members[]' | \
         cut -d' ' -f1 | sort)

if [[ "$expected" != "$actual" ]]; then
    echo "❌ Workspace member mismatch"
    echo "Expected:"
    echo "$expected"
    echo "Actual:"
    echo "$actual"
    exit 1
fi

echo "✅ All members resolvable"
```

### Build-time check
```bash
# In CI pipeline
cargo metadata --format-version 1 > /dev/null || {
    echo "❌ cargo metadata failed - invalid workspace"
    exit 1
}
```

### Pre-commit hook
```bash
#!/bin/bash
# .git/hooks/pre-commit

for member in $(grep -A 50 'members =' Cargo.toml | grep '"' | tr -d '", '); do
    if [[ ! -f "$member/Cargo.toml" ]]; then
        echo "❌ Cannot commit: invalid workspace member $member"
        exit 1
    fi
done
```

---

## Exceptions

**No exceptions. This rule is absolute.**

Every entry in `members = [...]` must resolve to a valid Cargo.toml. Use glob patterns (`"products/*"`) only if ALL directories match.

---

## Version History

- **1.0.0** (2026-02-03) — Initial version
