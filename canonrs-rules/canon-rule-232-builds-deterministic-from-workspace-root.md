# Canon Rule #232: Product Builds Must Be Deterministic From Workspace Root

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** build
**Version:** 1.0.0  
**Date:** 2025-02-04

---

## Principle

**All product builds MUST be executed from workspace root with zero dependence on current working directory.**

- `cwd = /workspace/root` always
- No relative path assumptions
- Reproducible across environments

---

## Problem

Without deterministic builds:

- "Works on my machine" syndrome
- CI failures due to path assumptions
- Flaky builds depending on shell history
- Makefiles assume wrong working directory
- Relative imports break unpredictably

### Symptom Examples
```bash
# Works
cd /workspace/products/app && make build  ✅

# Breaks
cd /workspace && make -C products/app build  ❌
# Error: ../../../assets/style.css not found
```

---

## Forbidden Pattern

### Forbidden
```makefile
# Makefile assumes cwd=product dir
build:
	trunk build --release
	cp ../../../assets/style.css dist/
	# ❌ Breaks if cwd ≠ expected
```

**Why this violates:**
- Fragile relative paths
- Breaks in CI/containers
- Non-reproducible

---

## Canonical Pattern

### Canonical
```makefile
# Makefile with explicit root reference
WORKSPACE_ROOT := $(shell git rev-parse --show-toplevel)
PRODUCT_DIR := $(WORKSPACE_ROOT)/products/app
ASSETS_DIR := $(WORKSPACE_ROOT)/assets

build:
	cd $(PRODUCT_DIR) && trunk build --release
	cp $(ASSETS_DIR)/style.css $(PRODUCT_DIR)/dist/
```

**Or use workspace configuration:**
```toml
# Cargo.toml
[[workspace.metadata.leptos]]
assets-dir = "assets"  # Relative to workspace root
site-root = "products/app/target/site"
```

**Why this complies:**
- No cwd assumptions
- Works from any directory
- CI-friendly

---

## Rationale

### Invariants Protected
1. **Reproducibility**: Same command = same result
2. **CI/CD Reliability**: No environment-specific paths
3. **Developer Experience**: Works regardless of shell state
4. **Container Builds**: No hidden dependencies on host paths

### Why Not Opinion
Build determinism is fundamental to:
- Continuous Integration
- Supply chain security (reproducible builds)
- Multi-developer workflows
- Automated deployments

### What This Enables
- Docker builds without volume mounts
- CI caching (paths are stable)
- Parallel builds (no race conditions)
- Build from any directory

---

## Enforcement

### Makefile Pattern (Required)
```makefile
# Always determine workspace root
WORKSPACE_ROOT := $(shell git rev-parse --show-toplevel)

# Or use relative upward traversal with validation
WORKSPACE_ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST)))/../..)

# Fail if not in git repo
ifeq ($(WORKSPACE_ROOT),)
$(error "Must be run from within git repository")
endif
```

### CI Validation (Required)
```yaml
# Test build from different directories
- name: Test Build Reproducibility
  run: |
    cd /tmp
    make -C /workspace/products/app build
    
    cd /workspace
    make -C products/app build
    
    # Both should produce identical artifacts
    diff /tmp/result /workspace/result
```

### Pre-Commit Check
```bash
# Test from random directory
cd $(mktemp -d)
make -C $WORKSPACE/products/app build || \
  echo "❌ Build is CWD-dependent"
```

---

## Exceptions

**No exceptions for product builds.**

Internal tool scripts may use cwd if:
1. Clearly documented as developer-only
2. Never used in CI/CD
3. Not part of release process

---

## Related Rules

- **Rule #221**: Separate Build Bins by Target
- **Rule #226**: Workspace-Relative Build Paths
- **Rule #228**: Feature Flags Control Compilation

---

## Implementation Checklist

For each product Makefile:
- [ ] Defines `WORKSPACE_ROOT` explicitly
- [ ] All paths relative to `WORKSPACE_ROOT`
- [ ] Fails fast if not in workspace
- [ ] Tested from multiple directories
- [ ] CI builds from workspace root

---

## Version History

- **1.0.0** (2025-02-04) — Initial version, consolidates Rules #221, #226, #228
