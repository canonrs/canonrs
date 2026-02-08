# CanonRS

**Full-stack Rust framework with SSR-safe defaults**

CanonRS is a batteries-included Rust web framework that absorbs build complexity so you can focus on shipping features, not fighting configuration.

## Why CanonRS?

- ✅ **Zero-config SSR** - Server-side rendering that just works
- ✅ **Smart profiles** - Framework manages LTO/strip/codegen automatically
- ✅ **Design system included** - Professional UI components out of the box
- ✅ **Type-safe theming** - CSS variables with Rust type safety
- ✅ **Hot reload** - Instant feedback during development
- ✅ **Production-ready** - Optimized builds with correct settings

## Quick Start

### Installation
```bash
cargo install canonrs-cli
```

### Create Your First App
```bash
canonrs new my_app
cd my_app
canonrs dev
```

Your app is now running at `http://localhost:3000` 🚀

## What's Included

### Framework Crates

- **`canonrs`** - Core framework and runtime
- **`canonrs-ui`** - UI component library with design tokens
- **`canonrs-ui-interactive`** - Interactive components with state
- **`canonrs-behaviors`** - Reusable component behaviors
- **`canonrs-providers`** - Context providers (theme, language, density)
- **`canonrs-tokens`** - Design token system and CSS generation
- **`canonrs-server`** - Server-side utilities
- **`canonrs-shared`** - Shared types and utilities

### Tooling

- **`canonrs-cli`** - Command-line interface for project management
- **Token engine** - Automatic CSS generation from design tokens
- **Theme system** - Type-safe theming with presets

## Documentation

- [CLI Quick Start](tools/canonrs-cli/CLI_QUICKSTART.md)
- [Architecture](tools/canonrs-cli/ARCHITECTURE.md)
- [Canon Rules](CANON_RULES.md) - Framework design principles
- [WASM Optimization](WASM_OPTIMIZATION.md)
- [Build Flow](docs/BUILD_FLOW.md)

## Project Structure
```
canonrs/
├── canonrs/                    # Core framework
├── canonrs-ui/                 # UI components + design system
│   ├── src/                    # Rust components
│   ├── styles/                 # CSS styles
│   └── themes/                 # Theme presets
├── canonrs-tokens/             # Token system
├── canonrs-providers/          # Context providers
├── canonrs-behaviors/          # Reusable behaviors
├── canonrs-server/             # Server utilities
├── canonrs-shared/             # Shared code
├── canonrs-ui-interactive/     # Interactive components
├── tools/canonrs-cli/          # CLI tool
├── docs/                       # Documentation
├── scripts/                    # Build scripts
└── templates/                  # Component templates
```

## Design Philosophy

### Canon Rules

CanonRS follows strict **Canon Rules** to ensure consistency and maintainability:

1. **Profiles are framework responsibility** - No manual profile configuration
2. **UI with `Children` must not use LTO in SSR** - Prevents linker crashes
3. **If build fails, it's framework's fault** - Clear error attribution

See [CANON_RULES.md](CANON_RULES.md) for full details.

### Design Token System

CanonRS includes a complete design token system:

- **Semantic tokens** - `color.accent.primary`, `spacing.md`
- **Type-safe** - Compile-time validation
- **CSS generation** - Automatic bundle creation
- **Theme support** - Light/dark modes out of the box

## Examples

### Basic Component
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

### With Design Tokens
```rust
#[component]
pub fn Card() -> impl IntoView {
    view! {
        <Box 
            padding="spacing.md"
            background="color.surface.primary"
            border_radius="radius.md"
        >
            <Text color="color.text.primary">
                "Card content"
            </Text>
        </Box>
    }
}
```

## Development

### Prerequisites

- Rust 1.94+ (nightly recommended)
- Node.js 18+ (for token engine)
- cargo-leptos

### Building from Source
```bash
git clone https://github.com/canonrs/canonrs.git
cd canonrs
cargo build
```

### Running Tests
```bash
cargo test --workspace
```

## CLI Commands

- `canonrs new <name>` - Create new app
- `canonrs dev` - Start development server
- `canonrs build` - Production build
- `canonrs doctor` - Health check

See [CLI documentation](tools/canonrs-cli/CLI_QUICKSTART.md) for details.

## Contributing

We welcome contributions! Please read our contributing guidelines first.

### Development Setup

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test --workspace`
5. Submit a pull request

## Community

- [GitHub Issues](https://github.com/canonrs/canonrs/issues)
- [Discussions](https://github.com/canonrs/canonrs/discussions)

## License

MIT License - see [LICENSE](LICENSE-MIT) for details.

## Credits

Built with ❤️ using:
- [Leptos](https://leptos.dev) - Reactive UI framework
- [Axum](https://github.com/tokio-rs/axum) - Web server
- [Tokio](https://tokio.rs) - Async runtime

---

**CanonRS** - Making Rust web development feel like magic ✨
