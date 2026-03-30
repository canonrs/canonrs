# Canon Rule #73: ComponentPage Template Contract

**Status:** ENFORCED
**Severity:** LOW
**Scope:** components
**Version:** 1.0.0
**Date:** 2025-01-16

---

## Rule

**Component documentation pages MUST use the `ComponentPage` template from the design system. Product code MUST only pass data as props, never reimplementing page structure.**

## Classification

| Aspect | Value |
|--------|-------|
| Category | Architecture |
| Severity | ❌ Critical |
| Scope | Component Documentation |
| Enforcement | Code Review |

## Rationale

### Single Source of Truth
- Template changes propagate to all component pages
- 50+ pages → 1 file to maintain
- Zero code duplication

### Separation of Structure and Content
- **Template** (rs-design): defines structure + blocks
- **Page** (product): provides data only
- Never mix concerns

## Template Location
```
rs-design/
├── src/blocks/canonrs/        # Blocks used by template
│   ├── component_header.rs
│   ├── symbol_badges.rs
│   ├── usage_block.rs
│   └── ...
└── src/pages/canonrs/         # Template itself
    └── component_page.rs
```

**Rule:** Template lives in design system, not product.

## Anti Pattern
```rust
// ❌ WRONG - Reimplementing structure in product
#[component]
pub fn ButtonPage() -> impl IntoView {
    view! {
        <div class="page">
            <div class="main">
                <h1>"Button"</h1>
                <p>"Description"</p>
                <div class="preview">
                    // Preview content
                </div>
                // ... more structure
            </div>
        </div>
    }
}
```

**Problems:**
- Structure duplicated across 50+ pages
- Template changes require 50+ file edits
- Inconsistent styling
- Token governance violation

## Correct Pattern
```rust
// ✅ CORRECT - Using template
use rs_design::pages::canonrs::ComponentPage;
use rs_design::blocks::canonrs::*;

#[component]
pub fn ButtonPage() -> impl IntoView {
    let preview = view! {
        <div class="flex gap-4">
            <Button>"Primary"</Button>
            <Button variant=ButtonVariant::Outline>"Outline"</Button>
        </div>
    }.into_any();

    view! {
        <ComponentPage
            name="Button".to_string()
            description="Primary action trigger".to_string()
            symbols=vec![
                Symbol { 
                    label: "SSR".to_string(), 
                    variant: SymbolVariant::SSR 
                },
            ]
            when_to_use=vec!["Primary actions".to_string()]
            when_not_to_use=vec!["Navigation".to_string()]
            preview=preview
        />
    }
}
```

**Benefits:**
- Page only provides data
- Template handles structure
- Single point of change
- Token governance maintained

## Template API Contract

### Required Props
```rust
name: String           // Component name
description: String    // Short description
```

### Optional Props
```rust
symbols: Option<Vec<Symbol>>              // Technical badges
preview: Option<AnyView>                  // Component preview
when_to_use: Option<Vec<String>>          // Use cases
when_not_to_use: Option<Vec<String>>      // Anti-use cases
api_props: Option<Vec<ApiProp>>           // API documentation
comparison: Option<Vec<ComparisonRow>>    // Technical comparison
rules: Option<Vec<String>>                // Canon rules
anti_patterns: Option<Vec<String>>        // Anti-patterns
```

## Preview Content Pattern

### Converting View to AnyView
```rust
// Pattern: .into_any()
let preview = view! {
    <div class="space-y-4">
        // Your preview content
    </div>
}.into_any();
```

**Rule:** Preview content must use `.into_any()` for type erasure.

## Data Structures

### Symbol
```rust
Symbol {
    label: String,           // "SSR", "Stable", etc.
    variant: SymbolVariant,  // Semantic variant
}

enum SymbolVariant {
    SSR,
    Hydration,
    ClientOnly,
    Stable,
    Beta,
}
```

### ApiProp
```rust
ApiProp {
    name: String,          // Prop name
    prop_type: String,     // Rust type
    description: String,   // Purpose
    required: bool,        // Mandatory?
}
```

### ComparisonRow
```rust
ComparisonRow {
    aspect: String,       // What's compared
    traditional: String,  // Old way
    canon: String,        // Canon way
}
```

## Maintenance

### Updating All Pages
```bash
# Change template once
vim rs-design/src/pages/canonrs/component_page.rs

# Rebuild
cargo build

# All 50+ pages updated automatically
```

### Adding New Block
```rust
// 1. Create block in rs-design
// rs-design/src/blocks/canonrs/new_block.rs

// 2. Add to template
// rs-design/src/pages/canonrs/component_page.rs
{new_data.map(|d| view! { <NewBlock data=d /> })}

// 3. Pages opt-in via prop
new_data=vec![...]
```

## Enforcement

### Code Review Checklist
- [ ] Page imports `ComponentPage` from rs-design
- [ ] Page only passes props (no structure)
- [ ] Preview uses `.into_any()`
- [ ] No duplicate HTML structure

## Exceptions

**None.** All component documentation pages use the template.

- If template is insufficient, extend it
- Never create parallel structure

## Related Rules

- **Canon Rule #74**: Block Semantic HTML
- **Canon Rule #75**: Token Family Boundaries
- **Canon Rule #59**: CSS Cascade Ownership

## Version

- **Created**: 2026-01-12
- **Status**: ✅ Active
