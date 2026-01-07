# CanonRS

Enterprise-grade design system and runtime framework for Leptos.

## Quick Start
```bash
# Add to your Cargo.toml
[dependencies]
rs-design = { git = "https://github.com/your-org/canonrs", path = "crates/rs-design" }
canonrs-ui = { git = "https://github.com/your-org/canonrs", path = "crates/canonrs-ui" }
canonrs-leptos = { git = "https://github.com/your-org/canonrs", path = "crates/canonrs-leptos" }
```
```rust
use leptos::prelude::*;
use rs_design::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Button>"Click me"</Button>
    }
}
```

## Architecture

- **rs-design** - Core design system (primitives, tokens, themes)
- **canonrs-core** - Runtime contracts, rules, security
- **canonrs-ui** - High-level UI components
- **canonrs-blocks** - Reusable composite blocks
- **canonrs-themes** - Theme engine
- **canonrs-leptos** - Leptos SSR-safe adapters

See [ARCHITECTURE.md](docs/ARCHITECTURE.md) for details.

## Examples

- `examples/leptos-basic` - Minimal starter
- `examples/workbench` - Full demo with all features

## Canon Rules

CanonRS is rule-driven. Every architectural decision is backed by a Canon Rule.

**For humans:** Start with [Operating Model](docs/canonrs/index.md)

**For AI:** Read `docs/canonrs/index.md` first, consult rules on-demand

## Development
```bash
# Build all crates
cargo build --workspace

# Run workbench
cd examples/workbench
trunk serve

# Run minimal example
cd examples/leptos-basic
trunk serve
```

## Documentation

- [Architecture](docs/ARCHITECTURE.md)
- [Canon Rules Index](docs/canonrs/index.md)
- [rs-design Docs](crates/rs-design/docs/)

## License

MIT

---

## 🔄 Development Model

### Source of Truth
`rs-design` is developed in our internal monorepo and synced here as **versioned snapshots**.

**Do NOT edit `crates/rs-design/` directly in this repo.**

### Sync Process
```bash
# When releasing a new version:
./scripts/sync-rs-design.sh 0.1.0-alpha
git add crates/rs-design
git commit -m "chore: sync rs-design v0.1.0-alpha"
git tag v0.1.0-alpha
git push --tags
```

This ensures:
- ✅ Single source of truth
- ✅ Controlled public releases
- ✅ Clean version history
- ✅ No divergence

