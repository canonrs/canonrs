# CanonRS Build Flow

**Version:** 3.0.0 (Feb 2026)

---

## Quick Start
```bash
# Build CSS completo
cd /opt/docker/monorepo/packages-rust/rs-canonrs
./scripts/core/generate-families.sh  # Gera families CSS
./scripts/core/bundle-css.sh         # Bundle final

# Build site
cd /opt/docker/monorepo/products/canonrs-site
make build
```

---

## Build Steps

### 1. Generate Families (Rust → CSS)
```bash
cd canonrs-tokens
cargo run
```

**Input:** `canonrs-shared/src/design/tokens/families/*.rs`  
**Output:** `canonrs-ui/styles/.generated/family-*.css`

10 families generated:
- overlay, selection, forms, navigation, feedback
- data, composite, layout, state, layers

---

### 2. Bundle CSS
```bash
./scripts/core/bundle-css.sh
```

**Output:** `canonrs-ui/styles/canonrs.bundle.css`

---

### 3. Build Site
```bash
cd products/canonrs-site

# CSS
npm run build:css

# SSR bin
cargo build --bin canonrs-site-ssr --release --features ssr

# CSR bin (WASM)
cargo build --bin canonrs-site-csr \
  --target wasm32-unknown-unknown \
  --release --features hydrate

# Optimize WASM
wasm-opt -Oz target/wasm32-unknown-unknown/release/deps/canonrs_site_csr-*.wasm \
  -o public/pkg/app.wasm
```

**Final sizes:**
- WASM: ~3-5MB (optimized)
- CSS: ~300KB (bundled)

---

## Validation
```bash
make validate  # Check architecture rules
```

Validates:
- No `axum` in WASM
- No `tokio` in WASM
- Correct bin separation

---

## Development
```bash
cd products/canonrs-site
make dev  # Watch mode
```

---

## Token Flow
```
tokens-engine (Rust - Complete CSS Cascade)
    ↓
families/*.rs → .generated/family-*.css
    ↓
bundle-css.sh → canonrs.bundle.css
    ↓
PostCSS (site) → output.css
```

---

## See Also

- [ARCHITECTURE.md](./ARCHITECTURE.md) - System design
- [Makefile](../../products/canonrs-site/Makefile) - Build commands
