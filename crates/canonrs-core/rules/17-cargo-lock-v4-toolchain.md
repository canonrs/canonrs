# Canon Rule #17: Cargo.lock v4 = Toolchain Incompatibility

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All Rust projects

---

## 🎯 Objective

Recognize and resolve toolchain version mismatches indicated by Cargo.lock version errors.

---

## 📋 The Error
```bash
error: failed to parse lock file at: /path/Cargo.lock

Caused by:
  lock file version `4` was found, but this version of Cargo 
  does not understand this lock file, perhaps Cargo needs to be updated?
```

---

## 🧠 What This Means

**Cargo.lock Versions:**
- v1: Cargo < 1.38
- v2: Cargo 1.38 - 1.47
- v3: Cargo 1.48+
- **v4: Cargo 1.77+ (nightly features)**

**Why v4 Exists:**
- `edition2024` support
- Unstable features in dependencies
- New resolver algorithms

---

## 🚨 Common Causes

### Cause 1: Dependency Using edition2024
```toml
# In transitive dependency
[package]
edition = "2024"  # Requires nightly
```

**Example:**
```bash
cargo tree | grep backon
# backon v1.6.0 uses edition2024
```

### Cause 2: Workspace Member Using Nightly Features
```toml
#![feature(async_closure)]  # Requires nightly
```

---

## ✅ Solutions

### Option A: Use Nightly Toolchain (Recommended)
```bash
# Install nightly
rustup toolchain install nightly

# Set default
rustup default nightly

# Or per-directory
echo "nightly" > rust-toolchain.toml
```

**Dockerfile:**
```dockerfile
FROM rustlang/rust:nightly-alpine AS builder
# Rest of build...
```

### Option B: Downgrade Dependency (Not Recommended)
```toml
# Only if you control the dependency
[dependencies]
problematic-crate = "1.5.0"  # Older version without edition2024
```

**Issues:**
- Loses features/fixes
- May have security vulnerabilities
- Not sustainable

### Option C: Pin Cargo Version
```bash
# If you MUST use stable
cargo +1.83 build
```

**Limitation:** Won't work if dependencies require nightly.

---

## 🔍 Diagnosis Steps
```bash
# 1. Check current Rust version
rustc --version

# 2. Check Cargo.lock version
head -n 3 Cargo.lock
# version = 4

# 3. Find dependencies requiring nightly
cargo tree -e features | grep edition
cargo metadata | jq '.packages[] | select(.edition == "2024")'

# 4. Check for feature gates
grep -r "#!\[feature" . --include="*.rs"
```

---

## 📝 Prevention

### Workspace Configuration:
```toml
# rust-toolchain.toml (in repo root)
[toolchain]
channel = "nightly-2025-01-01"  # Pin nightly date
components = ["rustfmt", "clippy"]
```

**Benefits:**
- Consistent across team
- Reproducible builds
- CI uses same version

### CI/CD:
```yaml
# .github/workflows/build.yml
- name: Install Rust nightly
  uses: actions-rs/toolchain@v1
  with:
    toolchain: nightly
    override: true

- name: Build
  run: cargo build --release
```

---

## 🚫 Don't Try To "Fix" Cargo.lock

**❌ WRONG:**
```bash
# Manually edit Cargo.lock version
sed -i 's/version = 4/version = 3/' Cargo.lock
cargo build
```

**Why This Fails:**
- Cargo regenerates lockfile
- Underlying incompatibility remains
- Build will fail anyway

---

## ✅ Correct Approach
```bash
# 1. Accept that nightly is required
rustup default nightly

# 2. Update Dockerfile
FROM rustlang/rust:nightly-alpine AS builder

# 3. Document in README
echo "Requires Rust nightly due to edition2024 dependencies" >> README.md

# 4. Pin nightly version for stability
echo 'nightly-2025-01-01' > rust-toolchain
```

---

## 🔗 Related Rules

- **Canon Rule #18** (Docker Cache): Rebuild after toolchain change
- **Canon Rule #20** (MUSL): Nightly supports MUSL targets

---

**Key Insight:** Cargo.lock v4 is not a bug - it's a feature compatibility indicator. Upgrade your toolchain, don't downgrade your dependencies.

---

**Last Updated:** 2025-01-07  
**Version:** 1.0
