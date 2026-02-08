# CanonRS CLI - Architecture

Internal design and technical decisions.

---

## Workspace Structure

Framework manages hidden `.canonrs/` directory:
```
my_app/
└── .canonrs/              # Git-ignored
    └── workspace/
        ├── Cargo.toml     # Workspace root with profiles
        ├── app/           # Copy of src/ and Cargo.toml
        │   ├── Cargo.toml
        │   └── src/
        └── target/        # Build artifacts
```

**Why copy instead of symlink?**
- Cross-platform (Windows-safe)
- Cargo requires hierarchical workspace members
- Enterprise-compatible

---

## Profile Management

Framework creates 3 canonical profiles in `.canonrs/workspace/Cargo.toml`:

### SSR Profile (`canonrs-ssr`)
```toml
[profile.canonrs-ssr]
inherits = "release"
lto = false           # REQUIRED: Children + SSR + LTO = linker crash
strip = false         # REQUIRED: trait objects need symbols
codegen-units = 16    # REQUIRED: LLVM needs space for closures
opt-level = 2
```

**Why these settings?**
Rust UI + SSR + trait objects (`Children`) + LTO = LLVM linker crash.
This is a real limitation, not a bug.

### CSR Profile (`canonrs-csr`)
```toml
[profile.canonrs-csr]
inherits = "release"
lto = "thin"
strip = "symbols"
codegen-units = 1
opt-level = "z"
```

### Hybrid Profile (`canonrs-hybrid`)
```toml
[profile.canonrs-hybrid]
inherits = "release"
lto = false
strip = false
codegen-units = 16
opt-level = 2
```

---

## Cache System

`.canonrs/workspace/` regenerates **only when**:
- Workspace doesn't exist
- `canonrs.toml` changed
- `Cargo.toml` changed

Otherwise cached and instant.

---

## Design Principles

### 1. Separation of Concerns
- Dev code: `src/` + clean `Cargo.toml`
- Build config: `.canonrs/workspace/` (framework-managed)

### 2. Error Attribution
- User code error → User fixes
- Build pipeline error → Framework bug

### 3. Guardrails Over Flexibility
Profiles forbidden in app `Cargo.toml` to prevent:
- Wrong LTO settings → linker crash
- Developer confusion → lost productivity

### 4. Canon Rule: UI + Children + SSR
Components with `Children` props trigger trait object closures.
LTO + strip + SSR = linker crash.
`canonrs-ssr` profile enforces safe settings automatically.

---

## Contributing

### Testing Changes
```bash
cd /opt/docker/monorepo
cargo build --package canonrs-cli
./target/debug/canonrs new test_app
cd test_app
../target/debug/canonrs dev
```

### Adding New Profiles
1. Edit `tools/canonrs-cli/src/workspace/profiles.rs`
2. Add mode to `detect.rs`
3. Update docs
4. Test with `canonrs doctor`

---

## License

MIT
