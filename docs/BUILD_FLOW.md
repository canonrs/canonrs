# CanonRS Build Flow

## CSS Pipeline

The CSS bundle is generated at compile time via `canonrs/build.rs` and embedded into the binary.
```
canonrs-tokens/src/design/  →  tokens-engine binary  →  canonrs-server/styles/canonrs.bundle.css
                                                                    ↓
                                                    canonrs/build.rs (embed via include_str!)
                                                                    ↓
                                                         canonrs::canonrs_css() → &'static str
```

### Regenerate CSS manually
```bash
cd packages-rust/rs-canonrs
cargo run -p canonrs-tokens --bin tokens-engine
```

Output: `canonrs-server/styles/canonrs.bundle.css`

---

## App Build (canonrs-builder)
```bash
cd products/canonrs-builder

# Development
make dev

# SSR binary
cargo build --features ssr

# WASM (hydrate)
cargo build --target wasm32-unknown-unknown --features hydrate
```

---

## Validation
```bash
# Check SSR compiles clean
cargo check -p canonrs-builder --features ssr

# Check hydrate compiles clean (no syntect/pulldown in graph)
cargo check -p canonrs-builder --features hydrate

# Check server crate standalone
cargo check -p canonrs-server --features hydrate
```

---

## CSS in Production

The CSS is served via a dedicated route registered in `main.rs`:
```rust
.route("/canonrs.css", axum::routing::get(|| async {
    ([(header::CONTENT_TYPE, "text/css")], canonrs::canonrs_css())
}))
```

No external CSS files needed. The bundle is embedded in the binary at compile time.

---

## See Also

- [ARCHITECTURE.md](./ARCHITECTURE.md) — System design
- [canonrs-tokens/README.md](../canonrs-tokens/README.md) — Token pipeline
