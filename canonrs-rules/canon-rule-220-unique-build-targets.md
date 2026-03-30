# Canon Rule #220: Workspace Metadata Must Define Unique Build Targets Per Product

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-03

**Category:** build-tooling
**Tags:** workspace, build, targets, leptos
**Language:** EN

---

**Intro:**
Shared build directories cause artifact collisions and corrupted outputs across products. Isolation is required for deterministic builds.

**Problem:**
multiple products share same build output causing overwrite and corruption

**Solution:**
assign unique site root per product to isolate build outputs

**Signals:**
- artifact overwrite
- wrong wasm
- build conflict

**Search Intent:**
how to fix shared build output leptos workspace

**Keywords:**
leptos site root conflict, workspace build isolation, wasm artifact overwrite, frontend multi product build

---

## Principle

**Each `[[workspace.metadata.leptos]]` block MUST specify a unique `site-root` to prevent output collision.**

---

## Problem

Without unique build targets:
- Products overwrite each other's outputs
- `target/site/pkg/` contains mixed WASM from multiple products
- Hot reload serves wrong product's assets
- CI artifacts are corrupted (mixed binaries)

**Observable symptoms**:
```bash
cargo leptos build --package canonrs-site
# Output: target/site/

cargo leptos build --package canonrs-workbench
# Output: target/site/  ❌ OVERWRITES PREVIOUS

ls target/site/pkg/
canonrs_workbench.wasm  # ❌ Where is canonrs_site.wasm?
```

---

## Forbidden Pattern

### Forbidden
```toml
[[workspace.metadata.leptos]]
name = "canonrs-site"
site-root = "target/site"      # ❌ SHARED
site-pkg-dir = "pkg"

[[workspace.metadata.leptos]]
name = "canonrs-workbench"
site-root = "target/site"      # ❌ COLLISION
site-pkg-dir = "pkg"
```

**Why this violates**: Both products write to same directory, last build wins.

---

## Canonical Pattern

### Canonical
```toml
[[workspace.metadata.leptos]]
name = "canonrs-site"
site-root = "target/site"           # ✅ UNIQUE
site-pkg-dir = "pkg"

[[workspace.metadata.leptos]]
name = "canonrs-workbench"
site-root = "target/site-workbench" # ✅ UNIQUE
site-pkg-dir = "pkg"

[[workspace.metadata.leptos]]
name = "core-auth-frontend"
site-root = "target/site-auth"      # ✅ UNIQUE
site-pkg-dir = "pkg"
```

**Why this complies**: Each product has isolated output directory, no collision possible.

---

## Rationale

### Architectural invariants
1. **Build isolation**: Each product is independently deployable
2. **Artifact integrity**: Mixed outputs break deployment pipelines
3. **Parallel builds**: Unique targets enable `cargo build -j`

### Bugs prevented
- Deployed wrong product to production (mixed WASM)
- Hot reload serves stale or wrong product
- CI cache corruption (artifacts from different products)

### Why not opinion
File system collisions are race conditions. Shared output directories violate the principle of build isolation.

---

## Enforcement

### CI validation
```bash
#!/bin/bash
# check-unique-site-roots.sh

site_roots=$(grep -h "site-root" Cargo.toml | sort)
unique_roots=$(echo "$site_roots" | uniq)

if [[ "$site_roots" != "$unique_roots" ]]; then
    echo "❌ Duplicate site-root detected"
    echo "$site_roots"
    exit 1
fi
```

### Linter rule
```rust
// workspace-lint/src/leptos.rs
fn validate_site_roots(metadata: &[LeptosMetadata]) -> Result<()> {
    let roots: HashSet<_> = metadata.iter().map(|m| &m.site_root).collect();
    
    if roots.len() != metadata.len() {
        bail!("Duplicate site-root in workspace.metadata.leptos");
    }
    
    Ok(())
}
```

---

## Exceptions

**No exceptions. This rule is absolute.**

Even for single-product workspaces, use unique `site-root` to future-proof against adding more products.

---

## Version History

- **1.0.0** (2026-02-03) — Initial version