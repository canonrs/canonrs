# Canon Rule #15: Scratch Images Require Self-Contained Binaries

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All services using `FROM scratch` base images

---

## 🎯 Objective

Understand and properly handle the constraints of `scratch` images, which contain absolutely nothing except what you explicitly copy into them.

---

## 📋 What `scratch` Means

**`scratch` contains ZERO files:**

- ❌ No `curl` / `wget`
- ❌ No `ca-certificates`
- ❌ No `libc` / `glibc`
- ❌ No shell (`/bin/sh`)
- ❌ No filesystem utilities
- ❌ No certificate store

**Only contains:** What you `COPY` into the image.

---

## 🚫 Direct Consequences

### 1. External Healthchecks Impossible

```yaml
# ❌ FAILS: curl doesn't exist in scratch
healthcheck:
  test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
```

**Why:** No `curl` binary available to execute.

### 2. Shell Commands Fail

```dockerfile
# ❌ FAILS: no /bin/sh
ENTRYPOINT ["/bin/sh", "-c", "exec /app/service"]

# ✅ CORRECT: direct binary execution
ENTRYPOINT ["/app/service"]
```

### 3. TLS/HTTPS Requires Embedded Certificates

If your binary makes HTTPS calls:

```dockerfile
# Must include CA certificates
FROM alpine AS certs
RUN apk add --no-cache ca-certificates

FROM scratch
COPY --from=certs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/service /app/service
```

---

## ✅ Solutions

### Option A: Internal Healthcheck (Recommended for scratch)

```yaml
# No external healthcheck - rely on process state
services:
  backend:
    image: service
    # No healthcheck defined
    restart: unless-stopped
```

**Rationale:** Container is healthy if process is running. Docker monitors process state automatically.

### Option B: Dummy Healthcheck

```yaml
healthcheck:
  test: ["CMD-SHELL", "exit 0"]
  interval: 5s
```

**Use when:** Other services use `depends_on: service_healthy` but actual health verification not needed.

### Option C: Use Minimal Base Image Instead

```dockerfile
# Instead of scratch, use distroless or alpine
FROM gcr.io/distroless/static-debian12
# Has ca-certificates, basic libs, NO shell

FROM alpine:3.19
# Has shell, curl, minimal utilities
```

---

## 🏗️ Architecture Decision

### When to Use `scratch`:

✅ **Use scratch when:**

- Binary is statically linked (MUSL)
- No external healthcheck needed
- No runtime dependencies
- Maximum security/minimalism desired
- Binary size <50MB

✅ **Use distroless when:**

- Need ca-certificates for HTTPS
- Want some debug capabilities
- Binary is statically linked
- Need slightly more than scratch

✅ **Use alpine when:**

- Need shell for debugging
- Need external healthcheck tools
- Dynamic linking acceptable
- Development/testing phase

---

## 📝 Complete Working Example

```dockerfile
# Multi-stage build with scratch
FROM rust:1.83-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev pkgconfig

WORKDIR /build
COPY . .

RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

# Production image
FROM scratch

COPY --from=builder \
  /build/target/x86_64-unknown-linux-musl/release/service \
  /app/service

EXPOSE 3000

ENTRYPOINT ["/app/service"]
```

**Corresponding compose:**

```yaml
services:
  backend:
    image: service
    container_name: backend
    ports:
      - "3000:3000"
    environment:
      - RUST_LOG=info
    restart: unless-stopped
    # NO healthcheck - scratch has no curl
```

---

## 🐛 Common Errors

### Error 1: Healthcheck Blocks Stack

```yaml
# docker-compose.yml
services:
  backend:
    image: service # scratch image
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]

  frontend:
    depends_on:
      backend:
        condition: service_healthy # ❌ NEVER becomes healthy
```

**Result:** Frontend never starts, backend marked "unhealthy" forever.

**Fix:** Remove healthcheck or use dummy `exit 0`.

### Error 2: Shell Entrypoint

```dockerfile
FROM scratch
COPY service /app/service
ENTRYPOINT ["/bin/sh", "-c", "/app/service"]  # ❌ /bin/sh doesn't exist
```

**Result:** `exec /bin/sh: no such file or directory`

**Fix:** Direct binary execution:

```dockerfile
ENTRYPOINT ["/app/service"]
```

### Error 3: Missing CA Certificates

```rust
// Code makes HTTPS request
let response = reqwest::get("https://api.example.com").await?;
```

**Result:** TLS handshake fails, certificate validation errors.

**Fix:** Copy CA certs into image or compile with `rustls` (native TLS).

---

## 🔍 Verification

```bash
# List contents of scratch image
docker run --rm --entrypoint ls service /
# Should show ONLY what you copied

# Try to run shell (should fail)
docker run --rm -it service /bin/sh
# Error: no such file or directory

# Verify binary is static
docker create --name check service
docker cp check:/app/service /tmp/service
file /tmp/service
# Should show: statically linked, no dynamic libraries
ldd /tmp/service 2>&1
# Should show: not a dynamic executable
docker rm check
```

---

## ✅ Best Practices

1. **Document Image Base:**

```dockerfile
   # Using scratch - no healthcheck support
   FROM scratch
```

2. **Static Linking Verification:**

```bash
   # In CI/CD
   ldd target/x86_64-unknown-linux-musl/release/service || echo "✅ Static"
```

3. **Size Optimization:**

```dockerfile
   # Strip debug symbols
   RUN strip target/x86_64-unknown-linux-musl/release/service
```

4. **Security Scanning:**

```bash
   # Scratch images have minimal attack surface
   docker scan service
   # Should show: 0 vulnerabilities (no base OS)
```

---

## 🔗 Related Rules

- **Canon Rule #13** (Binary Size): Static binaries are larger
- **Canon Rule #16** (Healthcheck Design): Healthcheck orchestration patterns
- **Canon Rule #20** (MUSL): Static linking requirements

---

**Key Insight:** `scratch` is not a "minimal Linux" - it's literally nothing. Every capability must be explicitly provided by your binary.

---

**Violations:** May cause deployment failures  
**Last Updated:** 2025-01-07  
**Version:** 1.0

---
