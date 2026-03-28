# canonrs-core

> Single Source of Truth for the CanonRS design system.
> All metadata, constraints, and API contracts are generated from `*_ui.rs` headers.

---

## Architecture
```
*_ui.rs  (@canon-* headers + Leptos props)
        ↓
    build.rs  (orchestrates all generators)
        ↓
src/generated/
  ├── catalog.rs            — CatalogEntry with kind, parts, regions, accept rules
  ├── component_meta.rs     — ComponentMeta with intent, capabilities, required parts
  ├── block_meta.rs         — BlockMeta for all blocks and layouts
  ├── block_definitions.rs  — BlockDefinition with regions, props, presets, AcceptRule
  ├── layout_definitions.rs — LayoutDefinition with slots and descriptions
  ├── component_definitions.rs — ComponentDefinition unifying meta + catalog
  ├── llm_components.md     — LLM context for UI components
  ├── llm_blocks.md         — LLM context for blocks
  └── llm_layouts.md        — LLM context for layouts
```

---

## SSOT Pipeline

The single source of truth flows in one direction only:
```
NEVER edit generated files manually
ALWAYS edit *_ui.rs headers or Leptos props
```

### Component headers (`*_ui.rs`)

Every UI component declares its contract via `@canon-*` doc comments:
```rust
//! @canon-id: button
//! @canon-label: Button
//! @canon-family: interactive
//! @canon-category: Action
//! @canon-intent: Trigger an action or event
//! @canon-description: Action button with variant and size
//! @canon-composable: false
//! @canon-capabilities: Disabled
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: button, action, submit, click, cta
```

### Block/Layout headers

Blocks and layouts extend the schema with structural metadata:
```rust
//! @canon-id: card
//! @canon-type: block
//! @canon-category: layout
//! @canon-variant: structure
//! @canon-container: true
//! @canon-regions: header, content, footer
//! @canon-label: Card
//! @canon-description: Container with header/content/footer regions
//! @canon-tags: card, container, box, content
//! @canon-slot-accepts: header=Any,content=Any,footer=Action
```

---

## Generated Artifacts

### `catalog.rs`

Unified registry of all components, blocks, and layouts with:
- `CatalogKind` — `Component | Block | Layout`
- `CatalogAcceptRule` — declarative accept rules per entry
- `CatalogRegionRule` — per-region accept rules derived from `@canon-slot-accepts`
```rust
CatalogEntry {
    id: "block.card",
    kind: CatalogKind::Block,
    category: CatalogCategory::Layout,
    parts: &[],
    regions: &["header", "content", "footer"],
    accepts: &[CatalogAcceptRule::AnyComponent, CatalogAcceptRule::AnyBlock],
    region_rules: &[
        CatalogRegionRule { region: "header",  accepts: &[CatalogAcceptRule::AnyComponent] },
        CatalogRegionRule { region: "content", accepts: &[CatalogAcceptRule::AnyComponent, CatalogAcceptRule::AnyBlock] },
        CatalogRegionRule { region: "footer",  accepts: &[CatalogAcceptRule::ComponentCategory(CatalogCategory::Action)] },
    ],
}
```

### `component_meta.rs`

Runtime statics for AI/RAG/Decision Engine:
```rust
pub static ACCORDION_META: ComponentMeta = ComponentMeta {
    id: "accordion",
    name: "Accordion",
    family: ComponentFamily::Layout,
    intent: "Expand and collapse content sections",
    capabilities: &[Capability::OpenClose, Capability::Multiple],
    composable: true,
    required_parts: &["AccordionItem", "AccordionTrigger", "AccordionContent"],
    optional_parts: &[],
};
```

### `block_definitions.rs`

Full structural definitions for blocks with semantic `AcceptRule` per region:
```rust
BlockDefinition {
    id: "card",
    regions: &[
        BlockRegion { id: "header",  accepts: &[AcceptRule::Any], layout: RegionLayout::Vertical },
        BlockRegion { id: "content", accepts: &[AcceptRule::Any], layout: RegionLayout::Vertical },
        BlockRegion { id: "footer",  accepts: &[AcceptRule::Category(BlockCategory::Form)], layout: RegionLayout::Horizontal },
    ],
    ...
}
```

### `api.rs` (per component)

Generated in `canonrs-server/src/ui/*/api.rs`:
```rust
pub const BUTTON_API: ComponentApi = ComponentApi {
    id: "button",
    props: &[
        PropDef { name: "children", kind: PropType::Children, required: true, default: None },
        PropDef { name: "variant",  kind: PropType::Enum(&["primary","solid","secondary",...]), required: false, default: Some("primary") },
        PropDef { name: "size",     kind: PropType::Bool, required: false, default: Some("md") },
        PropDef { name: "disabled", kind: PropType::Bool, required: false, default: Some("false") },
    ],
};
```

---

## Constraint Engine

`src/infra/constraint_engine.rs` — validates composition at runtime:
```rust
// Can a block be inserted into a region?
ConstraintEngine::can_insert("card", "footer", "button", 0)
// → ValidationResult::Valid

// Does a component have all required parts?
ConstraintEngine::validate_parts("accordion", &["AccordionItem", "AccordionTrigger"])
// → ValidationResult::Invalid([MissingParts { missing: ["AccordionContent"] }])

// What can go into a region?
ConstraintEngine::valid_children_for("card", "content")
// → ["button", "input", "badge", ...]

// Can a catalog entry nest inside another?
ConstraintEngine::catalog_can_nest_in_region(&dashboard, "header", &button)
// → true (header accepts Navigation category)
```

### Running tests
```bash
cargo test constraint_engine
```

---

## Build Generators

Located in `build/generators/`:

| Generator | Output | Source |
|-----------|--------|--------|
| `gen_catalog.rs` | `catalog.rs` | `@canon-*` headers |
| `gen_meta.rs` | `component_meta.rs`, `block_meta.rs` | `@canon-*` headers |
| `gen_definitions.rs` | `block_definitions.rs`, `layout_definitions.rs` | `@canon-*` headers + props |
| `gen_api/` | `*/api.rs` | Leptos `#[component]` props |
| `gen_component_definitions.rs` | `component_definitions.rs` | `@canon-*` headers |
| `gen_llm.rs` | `llm_*.md` | All of the above |

---

## Adding a New Component

1. Create `canonrs-server/src/ui/<name>/<name>_ui.rs` with `@canon-*` headers
2. Implement the Leptos `#[component]`
3. Run `cargo build` — all generated files update automatically

## Adding a New Block

1. Create `canonrs-server/src/blocks/<name>/<name>_block.rs` with `@canon-*` headers
2. Add `@canon-slot-accepts` for region rules
3. Run `cargo build`

---

## LLM Context

Three markdown files are generated for LLM consumption:

- `llm_components.md` — intent, capabilities, relationships, usage context
- `llm_blocks.md` — regions, props, presets, region rules, typical children
- `llm_layouts.md` — slots, slot rules, use when context

These files are the recommended input for AI page generation tasks.

---

## Crate Structure
```
canonrs-core/
├── build/
│   ├── types.rs         — shared build types
│   ├── utils.rs         — helpers (pascal_to_kebab, to_const_name)
│   ├── parsers.rs       — parse @canon-* headers, props, presets
│   └── generators/      — all code generators
├── src/
│   ├── infra/
│   │   ├── constraint_engine.rs  — runtime validation engine
│   │   ├── state_engine.rs       — aria-* / data-rs-state
│   │   └── dom_contract.rs       — required parts validation
│   ├── primitives/      — 82 pure HTML primitives
│   ├── generated/       — auto-generated (do not edit)
│   ├── catalog_types.rs — CatalogEntry, CatalogAcceptRule, PropType
│   ├── block_types.rs   — BlockDefinition, AcceptRule, BlockRegion
│   ├── meta_types.rs    — ComponentMeta, Capability, ComponentFamily
│   └── lib.rs
└── build.rs             — orchestrates all generators
```
