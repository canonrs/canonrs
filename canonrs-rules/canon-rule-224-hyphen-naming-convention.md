# Canon Rule #224: Workspace Package Names Must Use Hyphens Consistently

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-03

**Category:** governance
**Tags:** naming, workspace, cargo, convention
**Language:** EN

---

**Intro:**
Inconsistent naming between hyphens and underscores causes tool resolution issues and build failures. Naming must be uniform.

**Problem:**
mixed hyphen and underscore naming causes tool resolution errors

**Solution:**
use hyphens consistently across package bin and config names

**Signals:**
- binary not found
- name mismatch
- build error

**Search Intent:**
how to fix cargo naming hyphen underscore issue

**Keywords:**
cargo naming convention hyphen, rust package naming rules, workspace naming consistency, binary name mismatch rust

---

## Principle

**All package names, bin names, and leptos config names MUST use hyphens (`-`), never underscores (`_`), with the understanding that Rust internally converts hyphens to underscores for module names.**

---

## Problem

Without hyphen consistency:
- cargo-leptos searches for `canonrs-site` but finds `canonrs_site`
- Binary outputs have unpredictable names (`canonrs_site` vs `canonrs-site`)
- `use canonrs_site::App` vs `canonrs-site` package name creates confusion
- CI scripts fail with "file not found"

**Observable symptoms**:
```bash
cargo leptos build
Error: Could not read "target/debug/canonrs-site"

ls target/debug/
canonrs_site  # ❌ Name mismatch
```

---

## Forbidden Pattern

### Forbidden
```toml
# ROOT Cargo.toml
[[workspace.metadata.leptos]]
name = "canonrs_site"           # ❌ UNDERSCORE
bin-package = "canonrs-site"    # ❌ INCONSISTENT

# products/canonrs-site/Cargo.toml
[package]
name = "canonrs-site"           # ❌ INCONSISTENT

[[bin]]
name = "canonrs_site"           # ❌ UNDERSCORE

# Leptos.toml
output-name = "canonrs_site"    # ❌ UNDERSCORE
```

**Why this violates**: Mixed naming creates unpredictable tool behavior. Cargo and cargo-leptos have different resolution rules.

---

## Canonical Pattern

### Canonical
```toml
# ROOT Cargo.toml
[[workspace.metadata.leptos]]
name = "canonrs-site"           # ✅ HYPHEN
bin-package = "canonrs-site"    # ✅ HYPHEN
lib-package = "canonrs-site"    # ✅ HYPHEN

# products/canonrs-site/Cargo.toml
[package]
name = "canonrs-site"           # ✅ HYPHEN

[[bin]]
name = "canonrs-site"           # ✅ HYPHEN
path = "src/main.rs"

# Leptos.toml
[package]
name = "canonrs-site"           # ✅ HYPHEN

[package.metadata.leptos]
output-name = "canonrs-site"    # ✅ HYPHEN
```

**Why this complies**: Single naming convention across all tools. Rust automatically converts to `canonrs_site` for module imports.

**Usage in code**:
```rust
// imports use underscore (auto-converted by Rust)
use canonrs_site::App;

// But package/bin/config all use hyphen
```

---

## Rationale

### Architectural invariants
1. **Tool compatibility**: cargo, cargo-leptos, rustc all handle hyphens correctly
2. **File system**: Hyphens are valid in all contexts (URLs, files, DNS)
3. **Rust convention**: `cargo new` defaults to hyphens

### Bugs prevented
- "Binary not found" errors at runtime
- CI deployment failures (wrong artifact name)
- cargo-leptos unable to locate output
- Developer confusion (three different names for same thing)

### Why not opinion
Cargo's naming convention is hyphen-first. Fighting this creates friction with the entire Rust ecosystem.

---

## Enforcement

### CI validation
```bash
#!/bin/bash
# check-naming-consistency.sh

# Check workspace metadata
if grep -E 'name = "[^"]*_[^"]*"' Cargo.toml | grep workspace.metadata; then
    echo "❌ Underscore found in workspace.metadata.leptos name"
    exit 1
fi

# Check all Cargo.toml files
find . -name Cargo.toml -exec grep -H 'name = "[^"]*_[^"]*"' {} \; && {
    echo "❌ Underscore found in package names"
    exit 1
}

# Check Leptos.toml files
find . -name Leptos.toml -exec grep -H 'name = "[^"]*_[^"]*"' {} \; && {
    echo "❌ Underscore found in Leptos config"
    exit 1
}

echo "✅ All names use hyphens"
```

### Linter rule
```rust
// workspace-lint/src/naming.rs
fn validate_hyphen_convention(config: &CargoToml) -> Result<()> {
    if config.package.name.contains('_') {
        bail!("Package name must use hyphens: {}", config.package.name);
    }
    
    for bin in &config.bin {
        if bin.name.contains('_') {
            bail!("Bin name must use hyphens: {}", bin.name);
        }
    }
    
    Ok(())
}
```

### cargo-deny config
```toml
# deny.toml
[bans]
deny = [
    { name = "*_*", reason = "Package names must use hyphens" },
]
```

---

## Exceptions

**No exceptions. This rule is absolute.**

Even internal crates use hyphens: `canonrs-ssr`, `canonrs-csr`, `canonrs-shared` (not `canonrs_ssr`).

**Note**: In Rust code, imports automatically use underscores:
```rust
use canonrs_site::App;  // ✅ This is automatic, not a violation
```

---

## Version History

- **1.0.0** (2026-02-03) — Initial version