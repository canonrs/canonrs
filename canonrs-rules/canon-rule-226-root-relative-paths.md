# Canon Rule #226: Workspace Leptos Paths Must Be Root-Relative

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** leptos, workspace
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**All path fields in `[[workspace.metadata.leptos]]` (`style-file`, `assets-dir`) MUST be relative to the workspace root, not to the product directory.**

---

## Problem

Without root-relative paths:
- CSS files return 404 (cargo-leptos runs from workspace root)
- Assets not found during build
- Works locally (if in product dir) but fails in CI
- Hot reload serves wrong files

**Observable symptoms**:
```bash
cd /opt/docker/monorepo  # Workspace root
cargo leptos serve

# Browser console:
Failed to load resource: the server responded with 404 (Not Found)
/style/output.css
```

---

## Forbidden Pattern

### ❌ Forbidden
```toml
# ROOT Cargo.toml
[[workspace.metadata.leptos]]
name = "canonrs-site"
style-file = "style/output.css"      # ❌ Relative to where?
assets-dir = "public"                 # ❌ Ambiguous
site-root = "target/site"
```

**Why this violates**: When `cargo-leptos` runs from workspace root, it looks for `/style/output.css` (doesn't exist) instead of `/products/canonrs-site/style/output.css`.

**Test**:
```bash
ls style/output.css
# ls: cannot access 'style/output.css': No such file or directory

ls products/canonrs-site/style/output.css
# File exists ✓
```

---

## Canonical Pattern

### ✅ Canonical
```toml
# ROOT Cargo.toml
[[workspace.metadata.leptos]]
name = "canonrs-site"
bin-package = "canonrs-site"
lib-package = "canonrs-site"
style-file = "products/canonrs-site/style/output.css"  # ✅ From root
assets-dir = "products/canonrs-site/public"            # ✅ From root
site-root = "target/site"                              # ✅ Root-relative
```

**Why this complies**: Paths resolve correctly when cargo-leptos runs from workspace root.

**Verification**:
```bash
# From workspace root
ls products/canonrs-site/style/output.css  # ✅ Exists
ls products/canonrs-site/public            # ✅ Exists
```

---

## Rationale

### Architectural invariants
1. **Build context**: cargo-leptos always executes from workspace root
2. **CI consistency**: CI clones at root level, not product level
3. **Multi-product**: Multiple products need unambiguous paths

### Bugs prevented
- CSS 404 errors in production
- Assets missing in deployed builds
- "Works on my machine" (when running from product dir)
- Hot reload serving wrong product's files

### Why not opinion
File system paths are absolute or relative to a working directory. Cargo-leptos's working directory is the workspace root by definition.

---

## Enforcement

### CI validation
```bash
#!/bin/bash
# check-leptos-paths.sh

# Extract style-file and assets-dir from workspace metadata
paths=$(grep -A 20 'workspace.metadata.leptos' Cargo.toml | \
        grep -E '(style-file|assets-dir)' | \
        awk -F'"' '{print $2}')

for path in $paths; do
    # Check if path starts with products/ or packages/
    if [[ ! "$path" =~ ^(products|packages) ]]; then
        echo "❌ Path must be root-relative: $path"
        echo "Expected: products/<product>/$path"
        exit 1
    fi
    
    # Verify path exists
    if [[ ! -e "$path" ]]; then
        echo "❌ Path does not exist: $path"
        exit 1
    fi
done

echo "✅ All paths are root-relative and exist"
```

### Build-time check
```bash
# In cargo-leptos wrapper script
for cfg in $(toml get Cargo.toml workspace.metadata.leptos); do
    style_file=$(echo $cfg | jq -r '.style_file')
    
    if [[ ! -f "$style_file" ]]; then
        echo "❌ style-file not found: $style_file"
        echo "Hint: Use products/<name>/style/..."
        exit 1
    fi
done
```

### Review checklist
```markdown
- [ ] style-file starts with `products/<product>/`
- [ ] assets-dir starts with `products/<product>/`
- [ ] Both paths resolve from workspace root
- [ ] `ls <path>` succeeds from root directory
```

---

## Exceptions

### Shared assets (rare)
```toml
[[workspace.metadata.leptos]]
name = "canonrs-site"
style-file = "shared/style/common.css"  # ✅ OK if truly shared
assets-dir = "products/canonrs-site/public"
```

**Allowed only if**:
- Asset is genuinely shared by multiple products
- Path is still root-relative
- Documented why shared

**All other cases: No exceptions.**

---

## Version History

- **1.0.0** (2026-02-03) — Initial version
