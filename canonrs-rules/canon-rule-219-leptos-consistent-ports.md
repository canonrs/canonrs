# Canon Rule #219: Leptos Product Must Declare Consistent Ports Across Workspace and Local Config

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** leptos, workspace
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**`site-addr` and `reload-port` MUST match exactly between `[[workspace.metadata.leptos]]` and product `Leptos.toml`.**

---

## Problem

Without port consistency:
- Server starts on wrong port (e.g., 3000 vs 3004)
- Hot reload connects to wrong port, fails silently
- Developer confusion: "cargo leptos serve" uses different port than "cargo run"
- CI/CD deploys to unexpected ports

**Observable symptoms**:
```
# Terminal 1
cargo leptos serve
🚀 Listening on http://127.0.0.1:3000

# Terminal 2 (user expects 3004)
curl http://127.0.0.1:3004
Connection refused
```

---

## Forbidden Pattern

### Forbidden
```toml
# ROOT Cargo.toml
[[workspace.metadata.leptos]]
name = "canonrs-site"
site-addr = "127.0.0.1:3000"
reload-port = 3001

# products/canonrs-site/Leptos.toml
[package.metadata.leptos]
site-addr = "127.0.0.1:3004"  # ❌ DIVERGENT
reload-port = 3005             # ❌ DIVERGENT
```

**Why this violates**: `cargo-leptos` may use either config depending on invocation context, causing unpredictable behavior.

---

## Canonical Pattern

### Canonical
```toml
# ROOT Cargo.toml
[[workspace.metadata.leptos]]
name = "canonrs-site"
site-addr = "127.0.0.1:3000"
reload-port = 3001

# products/canonrs-site/Leptos.toml
[package.metadata.leptos]
site-addr = "127.0.0.1:3000"  # ✅ MATCHES
reload-port = 3001             # ✅ MATCHES
```

**Why this complies**: Single source of truth for runtime ports, regardless of build invocation.

---

## Rationale

### Architectural invariants
1. **Deterministic deployment**: Port configuration must be unambiguous
2. **Hot reload contract**: Reload port mismatch breaks development loop
3. **Multi-product workspaces**: Each product needs unique ports, but internal consistency

### Bugs prevented
- Silent hot reload failures (connects to wrong port)
- Production deploys on dev ports
- Port conflicts when running multiple products

### Why not opinion
Port binding is an OS-level contract. Divergent configuration creates race conditions and undefined behavior at runtime.

---

## Enforcement

### CI validation
```bash
#!/bin/bash
# check-leptos-ports.sh

for product in products/*/Leptos.toml; do
    product_name=$(basename $(dirname $product))
    
    # Extract ports from workspace
    workspace_site_addr=$(grep -A 20 "name = \"$product_name\"" Cargo.toml | grep site-addr | head -1)
    workspace_reload=$(grep -A 20 "name = \"$product_name\"" Cargo.toml | grep reload-port | head -1)
    
    # Extract from local
    local_site_addr=$(grep site-addr $product)
    local_reload=$(grep reload-port $product)
    
    if [[ "$workspace_site_addr" != "$local_site_addr" ]]; then
        echo "❌ Port mismatch in $product_name"
        exit 1
    fi
done
```

### Build-time check
```toml
# Cargo.toml (via build.rs)
[package.metadata.leptos.validation]
enforce-port-consistency = true
```

---

## Exceptions

**No exceptions. This rule is absolute.**

Ports must be deterministic. If a product needs different ports in different environments, use environment variables at runtime, not config divergence.

---

## Version History

- **1.0.0** (2026-02-03) — Initial version
