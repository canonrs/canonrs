# Canon Rule #12: Cargo Workspace ≠ Target Directory

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All Rust workspaces in monorepo

---

## 🎯 Objective

Understand that Cargo workspaces centralize build artifacts in a single `target/` directory, not in individual crate directories.

---

## 📋 The Problem

### Classic Symptom:
```bash
# Build a crate in workspace
cargo build -p my-service --release

# Try to run it
./target/release/my-service
# Error: No such file or directory

# Binary is actually here:
./products/target/release/my-service
# ✅ Works
```

---

## 🧠 Why This Happens

**Cargo Workspace Behavior:**
- Workspace controls the build graph
- ALL crates in workspace share ONE `target/` directory
- Location determined by workspace root, not crate location
- Prevents duplicate compilation of shared dependencies

---

## ✅ Correct Approach

### Find Target Directory:
```bash
# Always verify target location
cargo metadata --format-version 1 | jq -r '.target_directory'
# Output: /opt/docker/monorepo/products/target
```

### Workspace Structure:
```
monorepo/
├── Cargo.toml              # Workspace root
├── products/
│   ├── Cargo.toml          # Workspace manifest
│   ├── service-a/
│   │   └── Cargo.toml      # Member crate
│   └── target/             # ✅ Shared target (HERE!)
│       └── release/
│           ├── service-a   # ✅ Binary location
│           └── service-b
└── packages-rust/
    └── shared-lib/
        └── Cargo.toml      # Workspace member
```

---

## 🔍 Verification Commands
```bash
# 1. Find workspace root
cargo locate-project --workspace --message-format plain

# 2. Find target directory
cargo metadata --no-deps | jq -r '.target_directory'

# 3. List all built binaries
find $(cargo metadata --no-deps | jq -r '.target_directory') \
  -type f -executable -name 'service-*'

# 4. Run binary from correct location
TARGET_DIR=$(cargo metadata --no-deps | jq -r '.target_directory')
$TARGET_DIR/release/my-service
```

---

## 🐛 Common Errors

### Error 1: Wrong Path Assumption
```bash
# ❌ WRONG
cd products/my-service
cargo build --release
./target/release/my-service  # Doesn't exist here

# ✅ CORRECT
cd products/my-service
cargo build --release
../../products/target/release/my-service
```

### Error 2: Dockerfile Using Wrong Path
```dockerfile
# ❌ WRONG
COPY target/release/service /app/

# ✅ CORRECT (from workspace root)
COPY products/target/release/service /app/
```

### Error 3: CI/CD Path Issues
```yaml
# ❌ WRONG
- run: |
    cd services/api
    cargo build --release
    ./target/release/api  # Fails

# ✅ CORRECT
- run: |
    cargo build --release -p api
    ./products/target/release/api
```

---

## 📝 Best Practices

### 1. Use Workspace-Aware Commands
```bash
# Build from anywhere in workspace
cargo build -p service-name --release

# Don't rely on cwd for target location
```

### 2. Scripts Should Query Metadata
```bash
#!/bin/bash
TARGET_DIR=$(cargo metadata --no-deps --format-version 1 | jq -r '.target_directory')
BINARY="$TARGET_DIR/release/$SERVICE_NAME"

if [[ -f "$BINARY" ]]; then
    exec "$BINARY"
else
    echo "Binary not found: $BINARY"
    exit 1
fi
```

### 3. Docker Multi-Stage Builds
```dockerfile
FROM rust:1.83 AS builder
WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY products/ ./products/
COPY packages-rust/ ./packages-rust/

RUN cargo build --release -p my-service

# Use correct path from workspace
FROM scratch
COPY --from=builder \
  /build/products/target/release/my-service \
  /app/my-service

ENTRYPOINT ["/app/my-service"]
```

---

## 🔗 Related Rules

- **Canon Rule #18** (Docker Cache): Build artifacts location affects caching
- **Canon Rule #13** (Binary Size): Where to check binary size

---

**Key Insight:** Workspace centralizes compilation. Always query `cargo metadata` instead of assuming paths.

---

**Last Updated:** 2025-01-07  
**Version:** 1.0
