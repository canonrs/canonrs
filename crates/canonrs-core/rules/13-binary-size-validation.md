# Canon Rule #13: Binary Size Validation for Rust

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All Rust services using Axum/Tokio/Hyper

---

## 🎯 Objective

Detect incomplete or incorrectly linked Rust binaries by validating expected size ranges for common dependency combinations.

---

## 📋 The Problem

**Symptom:**
- Binary is ~300-500 KB
- Runs on host (sometimes)
- Dies silently in container with `exit 0`
- No logs, no panic, no error

**Diagnosis:**
- Dynamic linking against system libs
- Incomplete dead code elimination
- Runtime dependencies on glibc/openssl not satisfied in container

---

## 📊 Expected Binary Sizes

### Static MUSL Builds (Correct):

| Stack | Debug | Release | Release + Strip |
|-------|-------|---------|-----------------|
| Axum + Tokio + Hyper | 25-40 MB | 12-18 MB | 8-15 MB |
| Actix-web | 20-35 MB | 10-15 MB | 7-12 MB |
| Minimal (no async) | 5-10 MB | 2-5 MB | 1-3 MB |

### Dynamic Builds (Warning Sign):

| Stack | Size | Issue |
|-------|------|-------|
| Axum + Tokio (dynamic) | 300-800 KB | ❌ Missing runtime libs |
| Any async runtime | < 1 MB | ❌ Likely broken |

---

## 🚨 Red Flags

### Size Too Small:
```bash
ls -lh target/release/service
# 437K  ❌ WAY too small for Axum+Tokio
```

**Immediate Actions:**
1. Check linking type: `file target/release/service`
2. Check dependencies: `ldd target/release/service`
3. Rebuild with static MUSL

### Size Unexpectedly Large:
```bash
ls -lh target/debug/service
# 180M  ⚠️  Debug build, not release
```

**Check:** Ensure using `--release` flag.

---

## ✅ Validation Commands
```bash
# 1. Check binary size
ls -lh target/x86_64-unknown-linux-musl/release/service

# 2. Verify static linking
file target/x86_64-unknown-linux-musl/release/service
# Expected: statically linked, stripped

# 3. Check for dynamic dependencies (should fail for static)
ldd target/x86_64-unknown-linux-musl/release/service 2>&1
# Expected: "not a dynamic executable"

# 4. List embedded dependencies
cargo tree -p service --edges normal
```

---

## 🔧 Correction: Build Static MUSL

### Cargo Configuration:
```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
rustflags = ["-C", "target-feature=+crt-static"]
```

### Build Command:
```bash
# Install MUSL target
rustup target add x86_64-unknown-linux-musl

# Build static binary
cargo build --release --target x86_64-unknown-linux-musl

# Verify size
ls -lh target/x86_64-unknown-linux-musl/release/service
# Should be 8-15 MB for Axum+Tokio
```

### Dockerfile (Correct):
```dockerfile
FROM rust:1.83-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev pkgconfig

WORKDIR /build
COPY . .

RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release \
      --target x86_64-unknown-linux-musl \
      -p service

# Verify size in build
RUN ls -lh target/x86_64-unknown-linux-musl/release/service && \
    SIZE=$(stat -c%s target/x86_64-unknown-linux-musl/release/service) && \
    if [ $SIZE -lt 5000000 ]; then \
      echo "ERROR: Binary too small ($SIZE bytes)"; \
      exit 1; \
    fi

FROM scratch
COPY --from=builder \
  /build/target/x86_64-unknown-linux-musl/release/service \
  /app/service

ENTRYPOINT ["/app/service"]
```

---

## 📝 CI/CD Integration
```yaml
# .github/workflows/build.yml
- name: Build and validate binary
  run: |
    cargo build --release --target x86_64-unknown-linux-musl
    
    SIZE=$(stat -c%s target/x86_64-unknown-linux-musl/release/service)
    echo "Binary size: $SIZE bytes"
    
    if [ $SIZE -lt 5000000 ]; then
      echo "❌ Binary suspiciously small"
      exit 1
    fi
    
    if [ $SIZE -gt 50000000 ]; then
      echo "⚠️  Binary unexpectedly large (debug build?)"
      exit 1
    fi
    
    file target/x86_64-unknown-linux-musl/release/service | grep -q "statically linked" || {
      echo "❌ Binary not statically linked"
      exit 1
    }
```

---

## 🐛 Debugging Small Binaries
```bash
# 1. Check what was actually compiled
cargo build --release -vv 2>&1 | grep "Running.*rustc"

# 2. Check for conditional compilation issues
cargo build --release -vv 2>&1 | grep "Compiling.*service"

# 3. Verify dependencies are included
nm -D target/release/service | grep tokio
# Should show tokio symbols

# 4. Compare with known-good build
ls -lh target/x86_64-unknown-linux-musl/release/
```

---

## 🔗 Related Rules

- **Canon Rule #12** (Workspace Target): Where to find binaries
- **Canon Rule #18** (Docker Cache): Stale binaries from cache
- **Canon Rule #20** (MUSL): Why static linking matters

---

**Key Insight:** Binary size is a reliable indicator of build correctness. A 400KB Axum+Tokio binary is mathematically impossible.

---

**Last Updated:** 2025-01-07  
**Version:** 1.0
