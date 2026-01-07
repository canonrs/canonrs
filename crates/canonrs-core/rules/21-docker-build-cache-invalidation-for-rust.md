# Canon Rule #21: Docker Build Cache Invalidation for Rust

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All Rust services in monorepo with multi-stage Docker builds

---

## 🎯 Objective

Prevent stale binaries in Docker images caused by layer caching when Rust source code changes. Ensure compiled binaries always reflect current source code state.

---

## 📋 The Problem

**Symptom Chain:**

1. Modify `main.rs` or any `.rs` file
2. Run `docker build -t service .`
3. Container exits immediately with `exit 0`
4. No logs appear despite logging code
5. Binary responds once then dies
6. Host binary works, Docker binary fails

**Root Cause:**

- Docker caches `COPY` layer based on file metadata
- If timestamps unchanged, layer is reused
- `cargo build` runs against cached (old) source
- New code never compiled into image
- Binary contains previous version

**Real Example:**

```bash
# Add panic to code
echo 'panic!("TEST");' >> src/main.rs

# Build (cache hit on COPY)
docker build -t service .

# Run
docker run service
# Exit: 0 (no panic triggered)

# Proof
strings /app/binary | grep TEST
# (empty - old binary still in image)
```

---

## ✅ Required Solution

### Always Use `--no-cache` After Source Changes

```bash
# ❌ WRONG: Incremental build after code change
docker build -t service .

# ✅ CORRECT: Force complete rebuild
docker build --no-cache -t service .

# ✅ ALTERNATIVE: Clear cache first
docker builder prune -af
docker build -t service .
```

**Why This Works:**

1. Invalidates ALL layers unconditionally
2. `COPY` fetches latest file content
3. `cargo build` recompiles everything
4. Binary reflects current source code
5. Expected behavior guaranteed

---

## 🔍 Mandatory Pre-Build Checklist

Before ANY Docker build:

- [ ] Did I modify ANY `.rs` file?
- [ ] If YES: use `--no-cache` or `builder prune`
- [ ] After build: test execution immediately
- [ ] Verify expected logs appear
- [ ] Confirm debug strings present: `strings /app/binary | grep "EXPECTED"`

---

## 📦 Verification Commands

```bash
# 1. Build without cache
docker build --no-cache -f path/to/Dockerfile -t service .

# 2. Extract binary from image
docker create --name verify service
docker cp verify:/app/service /tmp/service-test
docker rm verify

# 3. Verify strings in binary
strings /tmp/service-test | grep "expected_log_message"

# 4. Test execution
/tmp/service-test 2>&1 | head -20
# Must show expected behavior
```

---

## ⚠️ When Cache is Safe vs Unsafe

### ✅ Cache Safe (can skip `--no-cache`):

- Modified only `Dockerfile` (RUN, ENV, etc)
- Modified only `Cargo.toml` dependencies
- Initial build (no cache exists)
- CI/CD pipeline (controlled environment)

### ❌ Cache Unsafe (MUST use `--no-cache`):

- Modified ANY `.rs` file
- Modified configuration affecting compilation
- Debugging unexpected container behavior
- After git merge/rebase with code changes
- Binary behavior doesn't match source expectations

---

## 🏗️ Development vs Production

| Aspect       | Development               | Production            |
| ------------ | ------------------------- | --------------------- |
| **Strategy** | Volume mount + hot reload | `--no-cache` build    |
| **Rebuild**  | Automatic (cargo-watch)   | Manual/CI             |
| **Cache**    | Not using Docker build    | Always `--no-cache`   |
| **Speed**    | Instant (incremental)     | Slower (full rebuild) |
| **Accuracy** | 100% (live source)        | 100% (`--no-cache`)   |

### Development Setup (Recommended)

```yaml
# docker-compose.dev.yml
services:
  backend:
    build:
      context: ../..
      dockerfile: products/service/Dockerfile.dev
    volumes:
      - ../../products/service/src:/build/products/service/src:ro
    command: cargo watch -x 'run -p service'
```

**Key:** Use `cargo watch` + volume mounts in dev, NOT Docker rebuilds.

### Production Build

```bash
# Always --no-cache for production images
docker build \
  --no-cache \
  --pull \
  -t service:production \
  -f products/service/Dockerfile \
  .
```

---

## 🐛 Debugging Stale Binary

### Symptoms:

- Container exits with `exit 0` immediately
- `docker logs` shows nothing or old logs
- `docker run -it` produces no output
- Changes to code don't appear in behavior

### Diagnosis Steps:

```bash
# 1. Check source file timestamp
stat products/service/src/main.rs | grep Modify

# 2. Check image creation time
docker images service --format "{{.CreatedAt}}"

# 3. If image OLDER than source file:
docker build --no-cache -f products/service/Dockerfile -t service .

# 4. Extract and verify binary
docker create --name check service
docker cp check:/app/service /tmp/service-check
docker rm check
strings /tmp/service-check | grep "DEBUG_MARKER"
```

---

## 🚫 Additional Root Cause: Wrong Binary Target

**Even with `--no-cache`, wrong binary can be compiled if:**

- `Cargo.toml` has multiple `[[bin]]` definitions
- Custom `path` points to different source file
- `src/bin/*.rs` exists alongside `src/main.rs`
- Cargo silently picks wrong entrypoint

### Example Problem:

```toml
# Hidden in Cargo.toml
[[bin]]
name = "service"
path = "src/bin/old.rs"  # ← Wrong file!

# You edit src/main.rs but Cargo builds src/bin/old.rs
# Result: Changes never appear
```

### Verification:

```bash
# Find actual binary source path
cargo metadata --no-deps --format-version 1 \
  | jq -r '.packages[].targets[] | select(.kind[] == "bin") | "\(.name): \(.src_path)"'

# Expected output:
# service: /path/to/products/service/src/main.rs

# Check for multiple bin definitions
grep -n "^\[\[bin\]\]" Cargo.toml
```

---

## ✅ Normative Requirements

Every production Rust service **MUST:**

1. Document build process includes `--no-cache` verification
2. Include build verification in CI/CD:

```yaml
- name: Build with cache invalidation
  run: docker build --no-cache -t $IMAGE .
```

3. Add debug marker to `main()` first line:

```rust
   tracing::info!("SERVICE_NAME v{} starting", env!("CARGO_PKG_VERSION"));
```

4. Verify extracted binary before deployment:

```bash
   strings /tmp/binary | grep "SERVICE_NAME"
```

5. Have exactly ONE binary entrypoint per crate
6. Document `[[bin]]` path in Cargo.toml comments if non-standard

---

## 📊 Performance Impact

| Build Type    | Time  | When to Use            |
| ------------- | ----- | ---------------------- |
| Cached        | ~10s  | Dependencies unchanged |
| Partial cache | ~1min | Only code changed      |
| `--no-cache`  | ~5min | Guarantee fresh build  |

**Recommendation:** Use `--no-cache` whenever ANY doubt exists about binary freshness.

---

## 🔗 Related Rules

- **Canon Rule #1** (Overlay Islands): Build context isolation
- **Canon Rule #12** (Workspace Target): Binary location in workspace
- **Canon Rule #13** (Binary Size): Detecting incomplete builds
- **Canon Rule #14** (Exit 0): Understanding process termination

---

## 💡 Key Insight

**Docker layer caching is timestamp-based, not content-based.**

Touching a file without changing content can invalidate cache.  
NOT touching a file WHILE changing content can preserve stale cache.

The only 100% reliable method: `--no-cache` after source modifications.

---

**Violations:** MUST block deployment  
**Last Updated:** 2025-01-07  
**Version:** 1.0

---
