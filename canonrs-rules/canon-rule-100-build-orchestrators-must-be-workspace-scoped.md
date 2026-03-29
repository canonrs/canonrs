# Canon Rule #100: Build Orchestrators Must Be Workspace-Scoped

**Status:** ENFORCED  
**Severity:** MEDIUM  
**Scope:** build, workspace, leptos
**Version:** 1.0.0  
**Date:** 2025-01-15

---

## Principle

**Build orchestration configuration (cargo-leptos, trunk, etc.) MUST be declared in the workspace root `Cargo.toml`, not in individual member crates.**

Build orchestrators operate at the workspace level and require centralized configuration. Member-level configuration is ignored or causes conflicts.

---

## The Problem

When build orchestrator configuration exists only in member crates:

- cargo-leptos cannot find configuration
- Build fails with "metadata not found" errors
- Hot reload doesn't work (watches wrong directories)
- Asset paths are incorrect
- Multiple members create conflicting configurations

### Real Symptoms
```
Error: `cargo metadata` exited with an error:
  could not find `leptos` in workspace metadata
```

Or:
```
warning: no [[workspace.metadata.leptos]] found in workspace root
falling back to defaults (probably incorrect)
```

Or hot reload that doesn't trigger, assets that 404, etc.

---

## Anti-Pattern (FORBIDDEN)
```toml
# ❌ FORBIDDEN: Configuration in member crate only
# shell/Cargo.toml
[package.metadata.leptos]
output-name = "my-app"
site-root = "target/site"
site-addr = "127.0.0.1:3000"
# ... etc
```

This fails because:
- cargo-leptos reads `workspace.metadata.leptos`, not `package.metadata.leptos`
- Workspace tools cannot discover member-level config
- Multiple members would conflict
- CI/CD cannot find configuration

---

## Canonical Pattern (REQUIRED)
```toml
# ✅ REQUIRED: Configuration in workspace root
# Cargo.toml (workspace root)
[workspace]
members = ["shell", "islands/markdown"]

[[workspace.metadata.leptos]]
name = "my-app-shell"
bin-package = "my-app-shell"
lib-package = "my-app-shell"
site-root = "target/site"
site-pkg-dir = "pkg"
site-addr = "127.0.0.1:3003"
reload-port = 3004
assets-dir = "shell/public"
style-file = "shell/style/main.css"
env = "DEV"
bin-features = ["ssr"]
lib-features = ["hydrate"]
watch-additional-files = ["packages-rust/rs-design/src"]

# Member crate has NO metadata
# shell/Cargo.toml
[package]
name = "my-app-shell"
# ... dependencies only
```

---

## Multi-Package Workspaces

For workspaces with multiple Leptos applications:
```toml
# Workspace root with multiple apps
[[workspace.metadata.leptos]]
name = "admin-panel"
bin-package = "admin-panel"
lib-package = "admin-panel"
site-root = "target/admin"
site-addr = "127.0.0.1:3001"

[[workspace.metadata.leptos]]
name = "customer-portal"
bin-package = "customer-portal"
lib-package = "customer-portal"
site-root = "target/customer"
site-addr = "127.0.0.1:3002"

# Build specific app
cargo leptos watch --bin-package admin-panel
```

---

## Required Configuration Fields

### Minimum Required
```toml
[[workspace.metadata.leptos]]
bin-package = "my-app"       # Must match binary crate name
lib-package = "my-app"       # Must match library crate name
site-root = "target/site"    # Where final HTML/assets go
```

### Recommended Production
```toml
[[workspace.metadata.leptos]]
name = "my-app"                           # Human-readable name
bin-package = "my-app"                    # Binary crate name
lib-package = "my-app"                    # Library crate name
site-root = "target/site"                 # Build output directory
site-pkg-dir = "pkg"                      # WASM package directory
site-addr = "127.0.0.1:3000"             # Dev server address
reload-port = 3001                        # Hot reload WebSocket port
assets-dir = "shell/public"               # Static assets source
style-file = "shell/style/main.css"       # Tailwind/CSS entry point
env = "DEV"                               # Environment
bin-features = ["ssr"]                    # Server features
lib-features = ["hydrate"]                # Client features
watch-additional-files = [                # Extra watch paths
    "packages-rust/rs-design/src",
    "config/*.toml"
]
```

---

## Watch Additional Files

Critical for monorepos:
```toml
[[workspace.metadata.leptos]]
# ...
watch-additional-files = [
    "/opt/docker/monorepo/packages-rust/rs-design/src",  # Absolute path OK
    "../shared-lib/src",                                  # Relative to workspace
    "config/**/*.toml",                                   # Glob patterns
]
```

Without this:
- Changes in dependency crates don't trigger rebuild
- Hot reload misses external changes
- Development workflow breaks

---

## Why Workspace-Scoped

Build orchestrators need workspace-level view:

1. **Dependency resolution**: Must see all workspace members
2. **Path resolution**: Relative paths are workspace-relative
3. **Watch configuration**: Monitors workspace root
4. **Output coordination**: Prevents multiple apps from conflicting

Member-level config cannot provide this global view.

---

## Other Build Tools

Same principle applies to other orchestrators:

### Trunk
```toml
# ✅ Workspace root
[workspace.metadata.trunk]
public-url = "/app/"
dist = "target/trunk"
```

### wasm-pack
```toml
# ✅ Workspace root
[workspace.metadata.wasm-pack]
profile-dev = "dev"
profile-release = "release"
```

### Custom Build Scripts
```toml
# ✅ Workspace root
[workspace.metadata.my-builder]
config-file = "build-config.toml"
output-dir = "dist"
```

---

## Diagnostic Checklist

If cargo-leptos cannot find config:
```bash
# 1. Check workspace root for metadata
grep -A 10 "workspace.metadata.leptos" Cargo.toml

# 2. Verify bin-package matches actual crate name
grep "name.*=" shell/Cargo.toml

# 3. Check paths are workspace-relative
# assets-dir should be relative to workspace root

# 4. Verify cargo-leptos sees config
cargo leptos --help
# Should show your app in available targets

# 5. Check for conflicting member-level metadata
grep -r "package.metadata.leptos" */Cargo.toml
```

---

## Migration from Member Config
```bash
# 1. Copy metadata from member to workspace root
# From shell/Cargo.toml [package.metadata.leptos]
# To Cargo.toml [[workspace.metadata.leptos]]

# 2. Remove member-level metadata
sed -i '/\[package\.metadata\.leptos\]/,/^$/d' shell/Cargo.toml

# 3. Fix relative paths (now workspace-relative)
# Change: assets-dir = "public"
# To:     assets-dir = "shell/public"

# 4. Test
cargo leptos watch
```

---

## Enforcement

- All cargo-leptos projects MUST use `[[workspace.metadata.leptos]]`
- Member crates MUST NOT have `[package.metadata.leptos]`
- Paths in metadata MUST be workspace-relative
- CI MUST verify metadata exists in workspace root
- New projects MUST use workspace-scoped configuration from start

---

## Canonical Justification

> **Build orchestration is a workspace concern, not a package concern.**  
> Orchestrators coordinate multiple crates, resolve paths globally,  
> and manage output directories that span the workspace.

This rule exists to:
- Centralize build configuration for discoverability
- Prevent conflicting configurations across members
- Enable proper dependency tracking and hot reload
- Align with Cargo's workspace-first design
- Simplify CI/CD (single source of truth)

---

## Related Canon Rules

- Canon Rule #94 — Leptos Workspace Features Must Be Explicit
- Canon Rule #97 — Leptos 0.8 Requires Floating Nightly Toolchain
- Canon Rule #93 — Leptos WASM Dev Builds Must Use Release Mode

---

## Version History

- **1.0.0** (2025-01-15): Initial rule based on Workbench migration cargo-leptos configuration requirements
