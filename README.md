# CanonRS

**A Rust web framework that makes SSR boring again.**

CanonRS removes build-time footguns — linker crashes, LTO traps, broken profiles —
so you can focus on shipping features instead of fighting Cargo.

If the build breaks, it’s a framework bug — not your fault.

---

## Why CanonRS exists

Modern Rust SSR often means:

- Linker crashes when using `Children`
- LTO + strip breaking builds
- Mysterious LLVM errors
- Hours lost tweaking Cargo profiles

**CanonRS absorbs all of that.**

No manual profiles.  
No LTO guessing.  
No build voodoo.

---

## Quick start

```bash
cargo install canonrs-cli

canonrs new my_app
cd my_app
canonrs dev
```

Open http://localhost:3000 🚀

That’s it.

---

## What CanonRS handles for you

- ✅ SSR-safe build profiles
- ✅ Automatic LTO / strip management
- ✅ Zero-config workspace generation
- ✅ Hot reload with correct settings
- ✅ Design system and UI primitives
- ✅ Type-safe theming via tokens

You never touch:
- Cargo profiles
- LTO flags
- strip settings
- codegen-units
- workspace internals

---

## Core principles

CanonRS follows strict architectural rules:

- **Build configuration is framework responsibility**
- **Developers must not manage profiles**
- **SSR + `Children` must never use LTO**
- **If `canonrs dev` fails, it’s a framework bug**

These are not conventions — they are enforced.

---

## Project layout (high level)

```
canonrs/
├── crates/              # Framework crates (source of truth)
├── tools/canonrs-cli/   # CanonRS CLI
├── docs/                # Documentation
└── examples/            # Example applications
```

Your app stays clean.  
Build complexity stays hidden.

---

## Documentation

- CLI Quick Start → tools/canonrs-cli/CLI_QUICKSTART.md
- Architecture → tools/canonrs-cli/ARCHITECTURE.md
- Canon Rules → design principles and invariants
- Build Flow → SSR and pipeline internals

You don’t need to read these to get started — only if you’re curious.

---

## Example

```rust
use canonrs::prelude::*;

#[component]
pub fn HelloWorld() -> impl IntoView {
    view! {
        <Box>
            <Text>"Hello, CanonRS!"</Text>
        </Box>
    }
}
```

No configuration required.

---

## CLI commands

- `canonrs new <name>` — create a new app
- `canonrs dev` — start dev server with hot reload
- `canonrs build` — production build
- `canonrs doctor` — environment health check

---

## Status

CanonRS is under active development.  
APIs may evolve, but core principles are stable.

Feedback and early adopters are welcome.

---

## Contributing

1. Fork the repo
2. Create a feature branch
3. Run tests: `cargo test --workspace`
4. Open a pull request

Framework bugs are framework responsibility — report them.

---

## Community

- Issues → https://github.com/canonrs/canonrs/issues
- Discussions → https://github.com/canonrs/canonrs/discussions

---

## License

MIT

---

**CanonRS**  
Making Rust SSR predictable, boring, and reliable.
