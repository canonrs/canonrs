# Canon Rule #227: Products Must Inherit Workspace Dependencies

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** workspace, build
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**Product crates MUST use `{ workspace = true }` for all dependencies declared in `[workspace.dependencies]`, never redeclaring versions.**

---

## Problem

Without workspace inheritance:
- Version drift: Products use different versions than workspace declares
- Duplicate dependency declarations cause conflicts
- Updates require changing multiple Cargo.toml files
- Violates DRY (Don't Repeat Yourself)

**Observable symptoms**:
```bash
cargo tree -p canonrs-site -d
# Shows multiple versions of same crate

error: failed to select a version for `axum`
candidate versions: 0.8.7, 0.8.8
required by canonrs-site, leptos_axum
```

---

## Forbidden Pattern

### ❌ Forbidden
```toml
# ROOT Cargo.toml
[workspace.dependencies]
leptos = "=0.8.15"
axum = "=0.8.8"

# products/canonrs-site/Cargo.toml
[dependencies]
leptos = "=0.8.15"              # ❌ DUPLICATED
axum = { version = "=0.8.8", optional = true }  # ❌ DUPLICATED
```

**Why this violates**: Version is declared twice. If workspace updates to 0.8.16, product still uses 0.8.15.

---

## Canonical Pattern

### ✅ Canonical
```toml
# ROOT Cargo.toml
[workspace.dependencies]
leptos = "=0.8.15"
leptos_meta = "=0.8.5"
leptos_router = "=0.8.11"
leptos_axum = "=0.8.7"
axum = "=0.8.8"
tower-http = { version = "=0.6.8", features = ["fs"] }
tokio = { version = "=1.42.0", features = ["full"] }

# products/canonrs-site/Cargo.toml
[dependencies]
leptos = { workspace = true }                    # ✅ INHERITED
leptos_meta = { workspace = true }               # ✅ INHERITED
leptos_router = { workspace = true }             # ✅ INHERITED
leptos_axum = { workspace = true, optional = true }
axum = { workspace = true, optional = true }
tower-http = { workspace = true, optional = true }
tokio = { workspace = true, optional = true }

# Local-only deps (not in workspace)
canonrs = { path = "../../packages-rust/rs-canonrs/canonrs" }
```

**Why this complies**: Single source of truth for versions. Workspace update propagates automatically.

---

## Rationale

### Architectural invariants
1. **Version consistency**: All crates use exact same version
2. **Update atomicity**: Change version once, applies everywhere
3. **Dependency graph**: Cargo resolver sees unified versions

### Bugs prevented
- Version conflicts between products
- Forgotten updates (one product on old version)
- Build cache invalidation (different versions = different artifacts)
- ABI conflicts (see Rule #225)

### Why not opinion
Cargo workspaces exist specifically to solve this problem. Not using them defeats the purpose of having a workspace.

---

## Enforcement

### CI validation
```bash
#!/bin/bash
# check-workspace-inheritance.sh

# Get deps declared in workspace
workspace_deps=$(grep -A 100 '\[workspace.dependencies\]' Cargo.toml | \
                 grep -E '^\w+' | awk -F'=' '{print $1}' | tr -d ' ')

# Check each product
for product in products/*/Cargo.toml; do
    for dep in $workspace_deps; do
        # If product uses this dep, check it inherits from workspace
        if grep -q "^$dep " "$product"; then
            if ! grep -q "$dep.*workspace.*true" "$product"; then
                echo "❌ $product: $dep must use { workspace = true }"
                exit 1
            fi
        fi
    done
done

echo "✅ All products inherit workspace dependencies"
```

### cargo-deny config
```toml
# deny.toml
[bans]
deny = [
    { duplicate-versions = "deny" },
]

# Deny any product declaring version for workspace dep
[[bans.deny]]
pattern = "leptos.*version"
reason = "Must use { workspace = true }"
```

### Review checklist
```markdown
- [ ] No version= in product Cargo.toml for workspace deps
- [ ] All shared deps use { workspace = true }
- [ ] Optional deps add `optional = true` but still inherit version
- [ ] cargo tree shows no duplicate versions
```

---

## Exceptions

### Product-specific dependencies
```toml
# products/canonrs-site/Cargo.toml
[dependencies]
# Workspace deps - INHERITED
leptos = { workspace = true }

# Product-specific - OK to declare version
web-sys = { version = "0.3", features = ["Window", "Storage"] }
canonrs = { path = "../../packages-rust/rs-canonrs/canonrs" }
```

**Allowed if**:
- Dependency is NOT in `[workspace.dependencies]`
- Dependency is unique to this product
- Path dependencies (local crates)

### Dev/build dependencies
```toml
[dev-dependencies]
# Can declare versions if NOT in workspace.dependencies
criterion = "0.5"

[build-dependencies]
# Can declare versions if NOT in workspace.dependencies
cc = "1.0"
```

**All workspace-declared deps: No exceptions.**

---

## Version History

- **1.0.0** (2026-02-03) — Initial version
