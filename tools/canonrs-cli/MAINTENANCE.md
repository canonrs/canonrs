# CanonRS CLI - Maintenance Guide

Critical scenarios requiring CLI updates.

---

## 1. Dependency Version Change

**Trigger:** Leptos, Axum, or Tokio major version bump in CanonRS framework.

**Action:**
```bash
# Update template in src/commands/new.rs
# Update Cargo.toml dependencies section (lines ~15-25)
```

**Test:**
```bash
cargo build --package canonrs-cli
canonrs new test_app
cd test_app && canonrs dev
```

---

## 2. Profile Configuration Change

**Trigger:** New LLVM/Rust behavior requires different LTO/strip settings.

**Action:**
```bash
# Edit src/workspace/profiles.rs
# Adjust canonrs-ssr, canonrs-csr, or canonrs-hybrid profiles
```

**Test:**
```bash
canonrs new test_ssr && cd test_ssr
canonrs build
# Verify binary size and no linker errors
```

---

## 3. Leptos Config Structure Change

**Trigger:** `cargo-leptos` updates `[[workspace.metadata.leptos]]` format.

**Action:**
```bash
# Edit src/workspace/generator.rs
# Update workspace_cargo template (lines ~15-30)
```

**Test:**
```bash
canonrs new test_app && cd test_app
canonrs dev
# Verify hot-reload works
```

---

## 4. New CanonRS Mode Added

**Trigger:** Framework adds new rendering mode (e.g., `static`, `islands-enhanced`).

**Action:**
1. Add mode to `src/detect.rs` enum
2. Add profile to `src/workspace/profiles.rs`
3. Update docs (CLI_QUICKSTART.md)

**Test:**
```bash
# Create canonrs.toml with new mode
canonrs dev
# Verify correct profile applied
```

---

## When NOT to Update CLI

- ❌ Minor framework bugfixes
- ❌ New components/utilities in CanonRS
- ❌ Documentation-only changes
- ❌ Example app updates

CLI updates only when **build pipeline** or **project structure** changes.
