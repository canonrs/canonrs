# CanonRS Enforcement Guards

## Compile-Time Guards

### Mutually Exclusive Features
**Location**: `canonrs/src/lib.rs`
```rust
#[cfg(all(feature = "ssr", feature = "hydrate"))]
compile_error!("Features `ssr` and `hydrate` are mutually exclusive.");
```
Prevents activating both SSR and hydrate in the same build.

### SSR-in-WASM Prevention
**Location**: `canonrs-server/Cargo.toml`

`syntect`, `pulldown-cmark`, `axum`, and `tokio` are declared `optional = true` and only activated by the `ssr` feature. They physically cannot enter a hydrate build graph.

---

## Build-Time Guards

### WASM Size Limit
**Location**: `products/canonrs-builder/Makefile`
```makefile
MAX_WASM_MB := 10
```

Fails the build if the WASM bundle exceeds 10MB. Prevents silent regressions from heavy dependencies leaking into client builds.

### Validate hydrate graph
```bash
cargo check -p canonrs-builder --features hydrate
cargo tree -p canonrs --features hydrate | grep -E "syntect|pulldown|axum|tokio"
# Must return empty
```

---

## Architectural Rules

| Rule | Enforcement |
|------|-------------|
| `syntect` never in WASM | Optional dep, `ssr` feature only |
| `pulldown-cmark` never in WASM | Optional dep, `ssr` feature only |
| `canonrs-client` never in SSR | Optional dep, `hydrate` feature only |
| `behaviors` never in SSR | `#[cfg(target_arch = "wasm32")]` in `canonrs-client` |
| Public API stable across targets | Stub pattern in `render_markdown` and `render_with_prefix` |

---

## Compliance Checklist

Before every commit:
- [ ] `cargo check -p canonrs-builder --features ssr` passes
- [ ] `cargo check -p canonrs-builder --features hydrate` passes
- [ ] `cargo check -p canonrs-server --features hydrate` passes
- [ ] No `syntect` or `pulldown-cmark` in hydrate dependency tree
- [ ] WASM bundle < 10MB

---

## Incident History

### 2025-02-04: WASM 143MB Regression
**Cause**: SSR-only code leaked into WASM build.
**Impact**: Bundle grew 71x (2MB → 143MB).
**Fix**: All heavy deps made optional, activated only by `ssr` feature.
**Lesson**: Feature flags control what enters the build graph. `cfg` controls what compiles. Both are required.
