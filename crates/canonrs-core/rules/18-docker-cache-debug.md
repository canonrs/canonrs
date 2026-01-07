# Canon Rule #18: Docker Cache Invalides Debugging

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All Docker builds during debugging

---

## 🎯 Objective

Eliminate cache as a variable during debugging by always using `--no-cache` when troubleshooting unexpected behavior.

---

## 📋 The Rule

**If you modified ANY `.rs` file and didn't use `--no-cache`, you don't know what's running.**

---

## 🚨 Why This Matters

### Cache Invalidation is Timestamp-Based

Docker doesn't analyze file content:
- File modified → layer invalidated
- File NOT touched → layer cached
- Even if content changed via `git checkout`

### Example Failure Mode:
```bash
# 1. Build image
docker build -t service .

# 2. Modify main.rs
echo 'panic!("test");' >> src/main.rs

# 3. Git checkout old version (timestamp unchanged)
git checkout main src/main.rs

# 4. Build again (cache hit!)
docker build -t service .

# 5. Run - no panic (old binary still in image)
docker run service
```

---

## ✅ Absolute Rule During Debug
```bash
# ❌ NEVER during debugging
docker build -t service .

# ✅ ALWAYS during debugging
docker build --no-cache -t service .

# ✅ Alternative
docker builder prune -af
docker build -t service .
```

**No exceptions. No discussion.**

---

## 🔍 When Cache is Acceptable

### ✅ Safe Scenarios:
- Initial build (no cache exists)
- Dependency-only changes (`Cargo.toml`)
- Dockerfile-only changes (ENV, COPY order)
- CI/CD pipeline (controlled, fresh environment)

### ❌ Never Trust Cache When:
- Debugging unexpected behavior
- Binary doesn't match source expectations
- After git operations (checkout, merge, rebase)
- Container exits unexpectedly
- Logs don't match code

---

## 📝 Debugging Workflow
```bash
# 1. Reproduce issue
docker run service
# Exits with unexpected behavior

# 2. Modify code to debug
echo 'println!("DEBUG MARKER");' >> src/main.rs

# 3. ALWAYS rebuild without cache
docker build --no-cache -t service .

# 4. Test
docker run service
# Should see "DEBUG MARKER"

# 5. Verify binary contains change
docker create --name verify service
docker cp verify:/app/service /tmp/service
strings /tmp/service | grep "DEBUG MARKER"
docker rm verify
```

---

## 🐛 Cache Invalidation Checklist

Before trusting a build during debugging:

- [ ] Used `--no-cache` or `builder prune`
- [ ] Verified strings in binary: `strings /app/binary | grep MARKER`
- [ ] Checked image timestamp: `docker images --format "{{.CreatedAt}}"`
- [ ] Confirmed image timestamp AFTER source modification

---

## 🔧 CI/CD Integration
```yaml
# Always use --no-cache for production
- name: Build production image
  run: docker build --no-cache -t $IMAGE .

# Or use BuildKit cache mounts (advanced)
- name: Build with explicit cache
  run: |
    docker buildx build \
      --cache-from type=registry,ref=$CACHE_IMAGE \
      --cache-to type=registry,ref=$CACHE_IMAGE \
      -t $IMAGE .
```

---

## 🔗 Related Rules

- **Canon Rule #52** (detailed version of this rule)
- **Canon Rule #13** (Binary Size): Detect stale binaries
- **Canon Rule #14** (Exit 0): Debug unexpected termination

---

**Key Insight:** During debugging, cache is your enemy. Eliminate it as a variable by always rebuilding from scratch.

---

**Last Updated:** 2025-01-07  
**Version:** 1.0
