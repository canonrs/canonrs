# Canon Rule #97: Leptos 0.8 Requires Floating Nightly Toolchain

**Status:** ENFORCED  
**Severity:** MEDIUM  
**Scope:** leptos, workspace
**Version:** 1.0.0  
**Date:** 2025-01-15

---

## Principle

**Leptos 0.8 applications MUST use unpinned `nightly` toolchain and unpinned minor versions in Cargo.toml.**

Leptos 0.8 depends on unstable Rust features that evolve rapidly. Pinning toolchain dates or minor versions creates incompatibilities that manifest as cryptic build errors.

---

## The Problem

When using pinned nightly or pinned Leptos minor versions:

- Build fails with "rustc X.Y is not supported by leptos"
- Or: "feature `edition2024` is required but not stabilized"
- Or: Dependency resolution conflicts with newer crates
- Error messages don't clearly indicate toolchain mismatch

### Real Symptoms
```
error: rustc 1.86.0-nightly is not supported by the following packages:
  leptos@0.8.15 requires rustc 1.88
```

Or:
```
error: feature `edition2024` is required
The package requires the Cargo feature called `edition2024`,
but that feature is not stabilized in this version of Cargo.
```

Or:
```
error: failed to select a version for the requirement `leptos_axum = "^0.8.15"`
candidate versions found which didn't match: 0.8.7, 0.8.6, 0.8.5, ...
```

---

## Anti-Pattern (FORBIDDEN)
```toml
# ❌ FORBIDDEN: Pinned nightly date
# rust-toolchain.toml
[toolchain]
channel = "nightly-2024-11-01"  # ← Breaks with new Leptos releases

# ❌ FORBIDDEN: Pinned minor version
[workspace.dependencies]
leptos = { version = "0.8.15", features = ["nightly"] }  # ← Breaks
leptos_axum = "0.8.15"  # ← May not exist or be incompatible
```

This fails because:
- Leptos 0.8.x releases require progressively newer nightly features
- Minor version `0.8.15` may require rustc 1.88 (doesn't exist yet)
- Older nightlies lack required unstable features
- Cargo can't resolve exact minor + floating nightly

---

## Canonical Pattern (REQUIRED)
```toml
# ✅ REQUIRED: Floating nightly
# rust-toolchain.toml
[toolchain]
channel = "nightly"  # ← Always latest
components = ["rustfmt", "clippy"]

# ✅ REQUIRED: Floating minor version
[workspace.dependencies]
leptos = { version = "0.8", features = ["nightly"] }  # ← No minor
leptos_router = { version = "0.8", features = ["nightly"] }
leptos_meta = { version = "0.8" }
leptos_axum = { version = "0.8" }
```

Let Cargo resolve to latest compatible minor version automatically.

---

## Why This Is Required

Leptos 0.8 development model:

1. Uses unstable Rust features (edition2024, type system extensions)
2. Tracks latest nightly closely
3. Releases minor versions frequently (0.8.x)
4. Requires specific rustc versions per release

Pinning creates incompatibility matrix:

| Pinned Nightly | Pinned Leptos | Result |
|----------------|---------------|---------|
| nightly-2024-11-01 | 0.8.15 | ❌ rustc too old |
| nightly-2025-01-10 | 0.8.7 | ❌ leptos too old |
| nightly | 0.8 | ✅ Cargo resolves |

Only floating both allows Cargo to find compatible versions.

---

## Acceptable Pinning Scenarios

Only pin when:

1. **Reproducing a specific bug**
```toml
   # Temporary for bug report
   channel = "nightly-2025-01-10"
   leptos = "=0.8.12"
```

2. **Production releases** (pin in CI, not in source)
```yaml
   # .github/workflows/deploy.yml
   - run: rustup override set nightly-2025-01-10
```

3. **Archival projects** (not actively developed)
```toml
   # Last-known-good for legacy app
   channel = "nightly-2024-12-01"
```

Never pin in active development or in committed source for libraries.

---

## Diagnostic Checklist

If you see rustc version errors:
```bash
# 1. Check rust-toolchain.toml
cat rust-toolchain.toml
# Should be: channel = "nightly" (no date)

# 2. Check Leptos versions
grep "leptos.*=" Cargo.toml
# Should be: version = "0.8" (no minor)

# 3. Update toolchain
rustup update nightly
rustup default nightly

# 4. Clean and rebuild
cargo clean
cargo build

# 5. Check Cargo.lock for resolved versions
grep "name = \"leptos\"" Cargo.lock -A 2
```

---

## Enforcement

- All Leptos 0.8 projects MUST use `channel = "nightly"` without date
- Cargo.toml MUST use `version = "0.8"` without minor for Leptos crates
- CI MUST use `rustup update` before builds
- Production deploys MAY pin in CI scripts, not in source
- Pinned versions MUST be documented with expiration date

---

## Canonical Justification

> **Leptos 0.8 is pre-1.0 and tracks nightly closely.**  
> Stability comes from semantic versioning (0.8 API contract),  
> not from toolchain pinning (which creates brittleness).

This rule exists to:
- Align with Leptos 0.8's development model
- Prevent "works on my machine" due to toolchain drift
- Let Cargo's version resolution work as designed
- Reduce CI/CD complexity (no toolchain matrix)

---

## Migration Path for Pinned Projects
```bash
# 1. Remove date from rust-toolchain.toml
sed -i 's/nightly-[0-9-]*/nightly/' rust-toolchain.toml

# 2. Remove minor versions from Cargo.toml
sed -i 's/leptos.*=.*"0\.8\.[0-9]*"/leptos = { version = "0.8"/' Cargo.toml

# 3. Update and clean
rustup update nightly
cargo clean
rm Cargo.lock

# 4. Rebuild
cargo build
```

---

## Related Canon Rules

- Canon Rule #94 — Leptos Workspace Features Must Be Explicit
- Canon Rule #93 — Leptos WASM Dev Builds Must Use Release Mode
- Canon Rule #100 — Build Orchestrators Must Be Workspace-Scoped

---

## Version History

- **1.0.0** (2025-01-15): Initial rule based on Workbench migration toolchain incompatibility debugging
