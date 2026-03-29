# Canon Rule #223: Feature Flag Scopes Must Not Leak Between Products

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** leptos, workspace
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**Each `[[workspace.metadata.leptos]]` block MUST define `bin-features` and `lib-features` that apply ONLY to its specified `bin-package` and `lib-package`, with no cross-contamination.**

---

## Problem

Without feature isolation:
- SSR packages (`canonrs-ssr`) compile with hydrate features
- Hydrate builds include server-only dependencies (tokio, axum)
- WASM binaries bloated with unused SSR code
- Violates Rule #196 (SSR/CSR separation) at workspace level

**Observable symptoms**:
```bash
cargo build --package canonrs-ssr --features hydrate
# ❌ SSR package compiling with CSR features

cargo tree -p canonrs-site --features hydrate | grep tokio
# ❌ WASM build includes tokio (SSR-only dep)
```

---

## Forbidden Pattern

### ❌ Forbidden
```toml
# ROOT Cargo.toml
[[workspace.metadata.leptos]]
name = "canonrs-site"
# ❌ No bin-package/lib-package specified
bin-features = ["ssr"]
lib-features = ["hydrate"]

# Result: ALL workspace members get these features!

[[workspace.metadata.leptos]]
name = "canonrs-workbench"
bin-features = ["ssr", "debug"]  # ❌ Leaks to canonrs-site
lib-features = ["hydrate"]
```

**Why this violates**: Without explicit package binding, features apply globally or unpredictably.

---

## Canonical Pattern

### ✅ Canonical
```toml
[[workspace.metadata.leptos]]
name = "canonrs-site"
bin-package = "canonrs-site"      # ✅ EXPLICIT
lib-package = "canonrs-site"      # ✅ EXPLICIT
bin-features = ["ssr"]            # Only for canonrs-site bin
lib-features = ["hydrate"]        # Only for canonrs-site lib

[[workspace.metadata.leptos]]
name = "canonrs-workbench"
bin-package = "canonrs-workbench" # ✅ EXPLICIT
lib-package = "canonrs-workbench" # ✅ EXPLICIT
bin-features = ["ssr"]            # Only for workbench bin
lib-features = ["hydrate"]        # Only for workbench lib

# canonrs-ssr, canonrs-csr are NOT in leptos metadata
# They are NOT products, they are libraries
```

**Why this complies**: Each product's features are isolated to its own package. Library crates (`canonrs-ssr`) are never directly built with Leptos features.

---

## Rationale

### Architectural invariants
1. **Feature boundaries**: SSR and hydrate must never coexist in same artifact
2. **Package isolation**: Products are independent, libraries are context-free
3. **Build determinism**: Feature flags are explicit per package

### Bugs prevented
- SSR code in WASM (hydration mismatches)
- Tokio/Axum in browser builds (bloat + compile errors)
- Violates Canon Rule #196 (SSR/CSR separation)
- Violates Canon Rule #197 (feature flag boundaries)

### Why not opinion
Feature flags control compilation at the ABI level. Cross-contamination creates binaries that violate architectural contracts.

---

## Enforcement

### CI validation
```bash
#!/bin/bash
# check-feature-isolation.sh

# Extract all bin-package and lib-package names
packages=$(grep -E "(bin-package|lib-package)" Cargo.toml | \
           awk -F'"' '{print $2}' | sort -u)

# Verify each is a member
for pkg in $packages; do
    if ! cargo metadata --format-version 1 --no-deps | \
         jq -r '.workspace_members[]' | grep -q "$pkg"; then
        echo "❌ Feature leak: $pkg not in workspace"
        exit 1
    fi
done

# Verify SSR/CSR libs are NOT in leptos metadata
if grep -q "canonrs-ssr" Cargo.toml | grep "bin-package"; then
    echo "❌ SSR library should not be a leptos product"
    exit 1
fi
```

### Static analysis
```rust
// workspace-lint/src/features.rs
fn validate_feature_isolation(metadata: &WorkspaceMetadata) -> Result<()> {
    for leptos_config in &metadata.leptos_configs {
        let bin_pkg = &leptos_config.bin_package;
        let lib_pkg = &leptos_config.lib_package;
        
        // Verify packages exist
        ensure!(
            metadata.members.contains(bin_pkg),
            "bin-package {} not in workspace", bin_pkg
        );
        
        // Verify no -ssr/-csr libs are products
        ensure!(
            !bin_pkg.ends_with("-ssr") && !bin_pkg.ends_with("-csr"),
            "SSR/CSR libraries cannot be leptos products"
        );
    }
    
    Ok(())
}
```

---

## Exceptions

**No exceptions. This rule is absolute.**

Library crates (`canonrs-ssr`, `canonrs-csr`) NEVER appear in `[[workspace.metadata.leptos]]`. Only final products (apps) do.

---

## Version History

- **1.0.0** (2026-02-03) — Initial version
