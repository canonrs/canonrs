# Canon Rule #221: Leptos.toml Critical Fields Must Align With Workspace Metadata

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-02-03

**Category:** build-tooling
**Tags:** config, workspace, leptos, build
**Language:** EN

---

**Intro:**
Divergent configuration fields create inconsistent builds and runtime behavior across tools. Critical fields must remain aligned.

**Problem:**
critical config fields differ causing inconsistent builds and runtime mismatch

**Solution:**
ensure critical fields match exactly between workspace and local configs

**Signals:**
- feature mismatch
- port conflict
- build divergence

**Search Intent:**
how to align leptos config workspace and local

**Keywords:**
leptos config alignment, workspace metadata consistency, frontend build config mismatch, cargo leptos config issue

---

## Principle

**`site-addr`, `reload-port`, `bin-features`, and `lib-features` MUST match exactly between workspace metadata and product Leptos.toml. Path fields (`style-file`, `assets-dir`) MAY differ due to context (root-relative vs product-relative).**

---

## Problem

Without alignment on critical fields:
- Features diverge: SSR builds with hydrate features, or vice-versa
- Ports mismatch (covered by Rule #219, but reinforced here)
- Build flags inconsistent between `cargo leptos build` and `cargo build --features`

**Observable symptoms**:
```rust
// Workspace says: lib-features = ["hydrate"]
// Leptos.toml says: lib-features = ["hydrate", "csr"]

// Result: cargo leptos build uses "hydrate"
//         cargo build uses "hydrate,csr"
// Divergent binaries!
```

---

## Forbidden Pattern

### Forbidden
```toml
# ROOT Cargo.toml
[[workspace.metadata.leptos]]
name = "canonrs-site"
bin-features = ["ssr"]
lib-features = ["hydrate"]

# products/canonrs-site/Leptos.toml
[package.metadata.leptos]
bin-features = ["ssr", "debug"]  # ❌ DIVERGENT
lib-features = ["hydrate", "csr"] # ❌ DIVERGENT
```

**Why this violates**: Feature flags define compilation boundaries. Divergence creates two different binaries depending on build tool.

---

## Canonical Pattern

### Canonical
```toml
# ROOT Cargo.toml
[[workspace.metadata.leptos]]
name = "canonrs-site"
site-addr = "127.0.0.1:3000"
reload-port = 3001
bin-features = ["ssr"]
lib-features = ["hydrate"]
style-file = "products/canonrs-site/style/output.css"  # Root-relative
assets-dir = "products/canonrs-site/public"            # Root-relative

# products/canonrs-site/Leptos.toml
[package.metadata.leptos]
site-addr = "127.0.0.1:3000"     # ✅ MATCHES
reload-port = 3001                # ✅ MATCHES
bin-features = ["ssr"]            # ✅ MATCHES
lib-features = ["hydrate"]        # ✅ MATCHES
style-file = "public/style/output.css"  # ✅ Product-relative (OK)
assets-dir = "public"                    # ✅ Product-relative (OK)
```

**Why this complies**: Critical runtime/compilation fields are identical. Path fields differ legitimately due to invocation context.

---

## Rationale

### Architectural invariants
1. **Feature flag determinism**: Compilation must be reproducible regardless of tool
2. **Configuration DRY**: Critical values defined once, paths adjusted for context
3. **Tool interoperability**: `cargo leptos`, `cargo build`, and CI all produce same binary

### Bugs prevented
- Hydration mismatches (SSR built with different features than WASM)
- Port conflicts (different tools bind different ports)
- Deployment failures (CI builds different binary than local)

### Why not opinion
Feature flags alter the compiled artifact at the ABI level. Divergent flags = different binaries = undefined behavior.

---

## Enforcement

### CI validation
```bash
#!/bin/bash
# check-critical-alignment.sh

CRITICAL_FIELDS=("site-addr" "reload-port" "bin-features" "lib-features")

for field in "${CRITICAL_FIELDS[@]}"; do
    workspace_val=$(grep -A 20 "workspace.metadata.leptos" Cargo.toml | grep "$field")
    local_val=$(grep "$field" products/*/Leptos.toml)
    
    # Compare (ignoring path context)
    if [[ "$workspace_val" != "$local_val" ]]; then
        echo "❌ Mismatch on $field"
        exit 1
    fi
done
```

### Review checklist
```markdown
- [ ] Ports match between workspace and Leptos.toml
- [ ] bin-features are identical
- [ ] lib-features are identical
- [ ] Paths are context-appropriate (root vs product)
```

---

## Exceptions

**Paths are exempt**: `style-file`, `assets-dir`, `site-root` MAY differ because:
- Workspace metadata uses root-relative paths
- Leptos.toml uses product-relative paths
- Both resolve to the same filesystem location

All other fields have **no exceptions**.

---

## Version History

- **1.0.0** (2026-02-03) — Initial version