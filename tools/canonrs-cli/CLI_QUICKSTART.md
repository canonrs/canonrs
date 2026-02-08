# CanonRS CLI - Quick Start

Command-line interface for CanonRS Framework.

## Installation
```bash
cargo install canonrs-cli
```

Or from source:
```bash
cd /opt/docker/monorepo
cargo build --release --package canonrs-cli
sudo ln -sf target/release/canonrs /usr/local/bin/canonrs
```

---

## Commands

### `canonrs new <name>`
Creates new CanonRS app with SSR-safe defaults.
```bash
canonrs new my_app
cd my_app
canonrs dev
```

### `canonrs dev`
Starts development server with hot-reload on http://localhost:3000
```bash
canonrs dev
```

### `canonrs build`
Production build with optimized profile.
```bash
canonrs build
```

Binary: `.canonrs/workspace/target/release/`

### `canonrs doctor`
Health check for CanonRS environment.
```bash
canonrs doctor
```

---

## Configuration

### `canonrs.toml`
Simple config in project root:
```toml
[app]
name = "my_app"
mode = "ssr"  # ssr, csr, or hybrid
```

**Modes:**
- `ssr` → Server-side rendering (Leptos + Axum)
- `csr` → Client-side only (WASM)
- `hybrid` → Islands architecture

---

## Project Structure
```
my_app/
├── canonrs.toml
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── pages/
├── style/
├── public/
└── .canonrs/        # Hidden, managed by framework
```

---

## Rules

### ❌ No Profiles in App Cargo.toml
Profiles are framework-managed. Adding `[profile.*]` blocks build:
```toml
# ❌ FORBIDDEN in your Cargo.toml
[profile.release]
lto = true
```

Framework sets correct profiles based on `canonrs.toml` mode.

---

## Troubleshooting

### Port 3000 in use
```bash
lsof -ti:3000 | xargs kill -9
canonrs dev
```

### cargo-leptos not found
```bash
cargo install cargo-leptos
canonrs doctor
```

### wasm32 target missing
```bash
rustup target add wasm32-unknown-unknown
canonrs doctor
```

### Build fails with linker errors
**This is a framework bug.** Report at:
https://github.com/canonrs/canonrs/issues

Include `canonrs doctor` output and error message.

---

## Next Steps

- Read `ARCHITECTURE.md` for internals
- Check examples in `/examples`
- Report issues on GitHub
