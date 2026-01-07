# CanonRS Architecture

## Overview
CanonRS is a unified design system and runtime framework for Leptos applications.

## Structure
```
canonrs/
├── crates/
│   ├── rs-design/          # Core design system (primitives, tokens, themes)
│   ├── canonrs-core/       # Runtime contracts, rules, security
│   ├── canonrs-ui/         # High-level UI components
│   ├── canonrs-blocks/     # Reusable composite blocks
│   ├── canonrs-themes/     # Theme engine
│   └── canonrs-leptos/     # Leptos SSR-safe adapters
│
├── examples/
│   ├── workbench/          # Full demo (ex frontend-leptos)
│   └── leptos-basic/       # Minimal starter
│
└── docs/
    ├── canonrs/
    │   ├── index.md        # Operating model (bootloader)
    │   └── rules/          # Canon rules
    └── ARCHITECTURE.md     # This file
```

## Dependency Flow
```
canonrs-leptos → canonrs-ui → rs-design
       ↓              ↓
  canonrs-core ──────┘
```

## Design Principles

1. **One Repository** - Single source of truth
2. **Workspace Architecture** - Cargo workspace for all crates
3. **SSR-First** - Leptos with hydration determinism
4. **Zero-Trust Security** - Rules enforced at compile time
5. **Rule-Driven** - Every decision backed by Canon Rule

## Migration from Monorepo

- `packages-rust/rs-design` → `crates/rs-design`
- `frontend-leptos` → `examples/workbench`
- `_rules/` → `crates/canonrs-core/rules/` + `docs/canonrs/rules/`

## For AI Assistants

When operating on this codebase:
1. Read `docs/canonrs/index.md` first
2. Consult Canon Rules by number (e.g., Rule #22)
3. Never assume - verify against rules
4. Propose new rules when gaps found
