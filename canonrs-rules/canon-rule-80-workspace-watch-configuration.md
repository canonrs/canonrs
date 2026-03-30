# Canon Rule #80: Workspace Watch Configuration

**Status:** ENFORCED


**Severity:** MEDIUM
**Scope:** workspace, build
**Version:** 1.0.0
**Date:** 2025-01-16

---


---

## Principle

`cargo-leptos watch` does NOT automatically monitor path dependencies outside the current workspace.

When using local design system packages (e.g., `rs-design`), changes will NOT trigger recompilation unless explicitly configured via `watch-additional-files`.

---

## Problem Statement

### Default Behavior
```toml
# workbench/Cargo.toml
[dependencies]
rs-design = { path = "../../../../../packages-rust/rs-design" }
```

**What happens:**
1. Edit file in `packages-rust/rs-design/src/ui/sidebar.rs`
2. `cargo-leptos watch` does NOT detect change
3. No recompilation occurs
4. Browser shows stale code
5. Developer wastes time debugging "why isn't this working?"

**Root cause:** `cargo-leptos` only watches files listed in workspace members by default.

---

## Canonical Solution

### Add to Cargo.toml
```toml
[package.metadata.leptos]
# ... other config ...
watch-additional-files = ["../../../../../packages-rust/rs-design/src"]
```

**Effect:** Now changes in `rs-design/src` trigger recompilation.

---

## Configuration Syntax

### Single Path
```toml
[package.metadata.leptos]
watch-additional-files = ["../../../../../packages-rust/rs-design/src"]
```

### Multiple Paths
```toml
[package.metadata.leptos]
watch-additional-files = [
    "../../../../../packages-rust/rs-design/src",
    "../../../../../packages-rust/rs-tailwind/dist",
    "../../shared-components/src"
]
```

### Path Requirements

- **MUST** be relative to Cargo.toml location
- **MUST** point to directories (not individual files)
- **SHOULD** point to `src/` for Rust packages
- **CAN** include multiple dependency paths

---

## Path Calculation
```bash
# From: /monorepo/examples/_wip/workbench/Cargo.toml
# To:   /monorepo/packages-rust/rs-design/src

# Calculate relative path:
cd /monorepo/examples/_wip/workbench
realpath --relative-to=. /monorepo/packages-rust/rs-design/src

# Result: ../../../../../packages-rust/rs-design/src
```

### Common Monorepo Layouts
```
monorepo/
├─ packages-rust/
│  └─ rs-design/
│     └─ src/  ← Watch this
└─ examples/
   └─ _wip/
      └─ workbench/
         └─ Cargo.toml  ← From here

Relative path: ../../../../../packages-rust/rs-design/src
```

---

## Verification

### Before Configuration
```bash
# 1. Start watch
cargo leptos watch

# 2. In another terminal, edit dependency
vim ../../../../../packages-rust/rs-design/src/ui/sidebar.rs

# 3. Save file
# Result: No recompilation ❌
```

### After Configuration
```bash
# 1. Add watch-additional-files to Cargo.toml

# 2. Restart watch
pkill -f "cargo leptos"
cargo leptos watch

# 3. Edit dependency
vim ../../../../../packages-rust/rs-design/src/ui/sidebar.rs

# 4. Save file
# Result: Recompilation triggered ✓
# Output: "Compiling rs-design v0.1.0"
```

---

## Complete Configuration Example
```toml
[package]
name = "canonrs-workbench"
version = "0.1.0"
edition = "2021"

[dependencies]
rs-design = { path = "../../../../../packages-rust/rs-design" }
leptos = "0.8"

[package.metadata.leptos]
output-name = "workbench"
site-root = "target/site"
site-pkg-dir = "pkg"
site-addr = "127.0.0.1:3003"
assets-dir = "public"
reload-port = 3004
env = "DEV"
bin-features = ["ssr"]
lib-features = ["hydrate"]

# ✅ CRITICAL: Watch local dependencies
watch-additional-files = ["../../../../../packages-rust/rs-design/src"]
```

---

## When To Use

### Always Required For

- Local design system packages (`rs-design`, `rs-ui`, etc.)
- Shared component libraries
- Local utility crates used across examples
- Any path dependency outside workspace

### NOT Required For

- Published crates from crates.io
- Dependencies within same workspace
- Generated code in `target/`
- Third-party npm packages

---

## Multiple Workbench Setup
```toml
# Example 1: minimal/Cargo.toml
[package.metadata.leptos]
watch-additional-files = ["../../../packages-rust/rs-design/src"]

# Example 2: workbench/Cargo.toml
[package.metadata.leptos]
watch-additional-files = ["../../../../../packages-rust/rs-design/src"]

# Example 3: leptos-tailwindv4/Cargo.toml
[package.metadata.leptos]
watch-additional-files = ["../../../packages-rust/rs-design/src"]
```

**Note:** Paths differ based on example location in monorepo.

---

## Common Mistakes

### Mistake 1: Absolute Paths
```toml
# ❌ WRONG
watch-additional-files = ["/opt/docker/monorepo/packages-rust/rs-design/src"]
```

**Problem:** Breaks on different machines
**Solution:** Always use relative paths

### Mistake 2: Wrong Base Directory
```toml
# ❌ WRONG (from workspace root instead of Cargo.toml)
watch-additional-files = ["packages-rust/rs-design/src"]
```

**Problem:** Path doesn't exist relative to Cargo.toml
**Solution:** Calculate from Cargo.toml location

### Mistake 3: Watching `target/`
```toml
# ❌ WRONG
watch-additional-files = ["../../../../../packages-rust/rs-design/target"]
```

**Problem:** Causes infinite recompile loops
**Solution:** Only watch `src/`, never `target/`

### Mistake 4: Individual Files
```toml
# ❌ SUBOPTIMAL
watch-additional-files = [
    "../../../../../packages-rust/rs-design/src/ui/sidebar.rs",
    "../../../../../packages-rust/rs-design/src/ui/button.rs"
]
```

**Problem:** Maintenance nightmare
**Solution:** Watch entire `src/` directory

---

## Debugging Watch Issues

### Check if path is correct
```bash
cd examples/_wip/workbench
ls -la ../../../../../packages-rust/rs-design/src
# Should list files, not error
```

### Check cargo-leptos output
```bash
cargo leptos watch --verbose
# Look for "Watching additional paths: ..."
```

### Manually trigger recompilation
```bash
# Force rebuild to test if code change works
cargo leptos build

# If build succeeds but watch fails, it's a watch config issue
```

### Test with simple file change
```bash
# Add comment to watched file
echo "// test" >> ../../../../../packages-rust/rs-design/src/lib.rs

# Watch terminal for "Compiling rs-design"
```

---

## Performance Considerations

### Impact on Build Times

- **Minimal:** Only checks file timestamps
- **No CPU overhead** when files unchanged
- **Instant detection** when files change

### Number of Watch Paths
```toml
# ✅ GOOD: Specific packages
watch-additional-files = [
    "../../../../../packages-rust/rs-design/src",
    "../../../../../packages-rust/rs-components/src"
]

# ⚠️ AVOID: Watching too many paths
watch-additional-files = [
    "../../../../../packages-rust",  # Watches EVERYTHING
]
```

**Guideline:** Watch only packages you actively edit.

---

## Integration With Other Tools

### With CSS Watch
```bash
# Terminal 1: Watch Rust (includes rs-design)
cargo leptos watch

# Terminal 2: Watch CSS
npm run watch:css
```

Both can run simultaneously.

### With VS Code
```json
// .vscode/tasks.json
{
  "label": "Watch Workbench + rs-design",
  "type": "shell",
  "command": "cargo leptos watch",
  "problemMatcher": ["$rustc"],
  "isBackground": true
}
```

---

## Enforcement Checklist

- [ ] All local path dependencies listed
- [ ] Paths are relative to Cargo.toml
- [ ] Paths point to `src/` directories
- [ ] Paths verified with `ls -la`
- [ ] Watch detects changes (test with edit)
- [ ] No infinite recompile loops

---

## Canonical Justification

**Developer experience is a feature.**

Waiting 10 seconds to realize code didn't recompile wastes:
- Developer focus
- Iteration speed
- Debugging time

Configuring watch once saves hours over project lifetime.

**Canon mandates:** Explicit watch configuration for all local dependencies.

---

## Canon References

- Canon Rule #64 — CSS Build Pipeline Mandatory
- Canon Rule #82 — CSS Build Pipeline Health
- Canon Rule #56 — Monorepo CSS Build Pipeline

---

## Related Symptoms

If you see:
- Code changes don't trigger recompilation
- `cargo leptos watch` ignores dependency edits
- Manual `cargo build` works, watch doesn't
- Recompilation only happens on workbench file changes

→ **This rule is violated.**

Go to: **SYMPTOMS.md → WATCH CONFIGURATION FAILURES**
