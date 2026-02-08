# CanonRS Documentation

**Version:** 3.0.0 (Feb 2026)

---

## Quick Links

- [ARCHITECTURE.md](./ARCHITECTURE.md) - System design & Canon Rules
- [BUILD_FLOW.md](./BUILD_FLOW.md) - Build process & commands

---

## Core Principles

1. **SSR ⇄ CSR Isolation** - Separate bins, zero leakage
2. **Token-Driven** - Design decisions via Rust types
3. **Compile-Time Validation** - Architecture violations = build errors
4. **Enterprise-Grade** - Anti-regression guards built-in

---

## Structure
```
canonrs-shared   → Tokens, design system (SSR+CSR)
canonrs-ui       → Leptos components (SSR+CSR safe)
canonrs-csr      → WASM-only behaviors
canonrs-tools    → Build-time tools
canonrs (facade) → Unified public API
```

---

## Getting Started
```bash
# Build CSS
cd /opt/docker/monorepo/packages-rust/rs-canonrs
./scripts/core/generate-families.sh
./scripts/core/bundle-css.sh

# Build site
cd /opt/docker/monorepo/products/canonrs-site
make build

# Validate architecture
make validate
```

---

**Framework:** Leptos 0.8+  
**License:** MIT
