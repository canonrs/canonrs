# CanonRS Documentation

---

## Quick Links

- [ARCHITECTURE.md](./ARCHITECTURE.md) — System design and crate responsibilities
- [BUILD_FLOW.md](./BUILD_FLOW.md) — Build process and commands
- [ENFORCEMENT_GUARDS.md](./ENFORCEMENT_GUARDS.md) — Compile-time and build-time guards

---

## Core Principles

1. **One dependency** — developers add `canonrs` only, never internal crates
2. **Feature-driven targets** — `ssr` and `hydrate` control what enters the build graph
3. **Stable public API** — same function signatures across all targets
4. **Heavy deps are optional** — `syntect` and `pulldown-cmark` never enter WASM builds

---

## Crate Structure
```
canonrs          → public facade
canonrs-core     → types, providers, primitives (all targets)
canonrs-server   → 80+ UI components, layouts, blocks (SSR + hydrate)
canonrs-client   → behaviors, interactive components (hydrate + wasm32)
canonrs-tokens   → CSS token generator (build tooling only)
```

---

## Getting Started
```bash
# Add to your app
canonrs = { version = "0.1", features = ["ssr"] }
# or
canonrs = { version = "0.1", features = ["hydrate"] }

# Regenerate CSS
cargo run -p canonrs-tokens --bin tokens-engine

# Validate builds
cargo check -p canonrs-builder --features ssr
cargo check -p canonrs-builder --features hydrate
```

---

**Framework:** Leptos 0.8+
**License:** MIT
