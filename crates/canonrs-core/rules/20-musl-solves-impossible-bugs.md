# Canon Rule #20: MUSL Resolves 90% of "Impossible" Container Bugs

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All Rust services deployed in containers

---

## 🎯 Objective

Recognize runtime dependency issues and default to static MUSL linking to eliminate "works on host, fails in container" problems.

---

## 📋 The Pattern

**Symptom:**
- Binary runs perfectly on host
- Same binary fails silently in container
- Exit code 0, no logs, no panic
- `strace` shows missing libs or failed syscalls

**Root Cause:**
- Dynamic linking against system libraries
- Container lacks required runtime dependencies
- glibc version mismatch
- OpenSSL version incompatibility

---

## 🧠 Why MUSL Solves This

**Dynamic Binary (problematic):**
```bash
ldd /app/service
# libssl.so.3 => not found
# libcrypto.so.3 => not found
# libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6
```

**Static MUSL Binary (solution):**
```bash
ldd /app/service
# not a dynamic executable

file /app/service
# statically linked, stripped
```

**Result:** Binary contains ALL dependencies. No runtime requirements.

---

## ✅ Default Solution Template

### Dockerfile:
```dockerfile
FROM rust:1.83-alpine AS builder

RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig \
    build-base

WORKDIR /build
COPY . .

RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release \
      --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder \
  /build/target/x86_64-unknown-linux-musl/release/service \
  /app/service

EXPOSE 3000
ENTRYPOINT ["/app/service"]
```

---

## 🚨 When to Suspect Runtime Issues

### Checklist:

- [ ] Binary works on host
- [ ] Binary fails in container
- [ ] Exit code 0 (no error)
- [ ] No logs appear
- [ ] Binary size < 1MB (too small for dependencies)
- [ ] `file` shows "dynamically linked"
- [ ] `ldd` shows missing libraries

**If ANY 3+ checked:** Runtime dependency issue. Use MUSL.

---

## 🔧 Verification Commands
```bash
# 1. Check if binary is static
file target/x86_64-unknown-linux-musl/release/service
# Expected: statically linked

# 2. Verify no dynamic dependencies
ldd target/x86_64-unknown-linux-musl/release/service 2>&1
# Expected: "not a dynamic executable"

# 3. Check binary size (should be larger)
ls -lh target/x86_64-unknown-linux-musl/release/service
# Expected: 8-15 MB for typical Axum service

# 4. Test in minimal container
docker run --rm -v $(pwd)/target/x86_64-unknown-linux-musl/release:/app \
  scratch /app/service
# Should work without ANY base image dependencies
```

---

## 📊 Comparison

| Aspect | Dynamic (glibc) | Static (MUSL) |
|--------|-----------------|---------------|
| **Binary Size** | 500KB - 2MB | 8MB - 15MB |
| **Runtime Deps** | glibc, openssl, etc | None |
| **Container Base** | Requires Debian/Ubuntu | Works in `scratch` |
| **Portability** | Limited | Universal |
| **Debugging** | Complex (missing libs) | Simple (self-contained) |
| **Security** | Depends on base image | Minimal attack surface |

---

## 🐛 Real-World Example

### Before (Dynamic):
```dockerfile
FROM rust:1.83 AS builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:12-slim
COPY --from=builder /build/target/release/service /app/
CMD ["/app/service"]
```

**Issue:**
```bash
docker run service
# (exits immediately, no output)

docker run -it --entrypoint bash service
root@container:/# ldd /app/service
# libssl.so.3 => not found  ❌
```

### After (MUSL):
```dockerfile
FROM rust:1.83-alpine AS builder
RUN apk add musl-dev
WORKDIR /build
COPY . .
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder \
  /build/target/x86_64-unknown-linux-musl/release/service \
  /app/service
ENTRYPOINT ["/app/service"]
```

**Result:**
```bash
docker run service
# 2025-01-07T01:00:00Z INFO Starting service...
# ✅ Works perfectly
```

---

## 📝 Cargo Configuration
```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-musl]
linker = "rust-lld"
rustflags = [
  "-C", "target-feature=+crt-static",
  "-C", "link-arg=-static",
]

[build]
target = "x86_64-unknown-linux-musl"
```

---

## 🔗 Related Rules

- **Canon Rule #13** (Binary Size): MUSL binaries are larger
- **Canon Rule #14** (Exit 0): Silent failures from missing libs
- **Canon Rule #15** (Scratch): MUSL enables scratch images

---

**Key Insight:** If it works on host but not in container, assume runtime dependency mismatch. MUSL eliminates this entire class of problems.

---

**Last Updated:** 2025-01-07  
**Version:** 1.0
