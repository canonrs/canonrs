# WASM Optimization Guide

## Requirements

### 1. Workspace Profile
Add to root `Cargo.toml`:
```toml
[profile.wasm-release]
inherits = "release"
opt-level = "z"
lto = "fat"
codegen-units = 1
panic = "abort"
strip = "symbols"
```

### 2. Leptos Metadata
Add to root `Cargo.toml`:
```toml
[[workspace.metadata.leptos]]
# ... other config
lib-profile-release = "wasm-release"
wasm-opt = true
wasm-opt-features = ["-Oz", "--enable-bulk-memory", "--enable-nontrapping-float-to-int"]
```

### 3. Install wasm-opt
```bash
cargo install wasm-opt
```

### 4. Build Commands
```bash
# Production
cargo leptos build --release

# Dev (optimized)
cargo leptos watch --release
```

## Results
- Target: <3MB WASM
- Actual: ~1.5MB with current setup

## Known Issues
- `cargo-leptos 0.3.2`: watch mode ignores wasm-opt without `--release` flag
- Always use `--release` flag even in dev mode
