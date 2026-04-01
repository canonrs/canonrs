# Breadcrumb — LLM Context

## Identity

### component_id
breadcrumb

### type
navigation

### family
navigation

### pillar
hierarchical navigation

### canonical_import
canonrs::ui::breadcrumb

## Contract

### What the compiler enforces
Breadcrumb enforces activity state mapping and structure.

### What cannot drift
- Current page semantics
- Navigation structure

### What is guaranteed at compile time
- Correct aria-current mapping
- Valid hierarchy

## Definition

### Precise technical definition
Breadcrumb is a composable navigation trail that encodes hierarchy and current location using activity state.

### Distinction from HTML element
Unlike nav, it enforces state and semantics.

### Distinction from similar components
Sidebar provides full navigation; Breadcrumb shows path.

## API

### Props
- class: String

### Variants
None

### Sizes
None

### States
- Active
- Inactive

## Patterns

### When to use
Hierarchical navigation display.

### When NOT to use
Primary navigation menus.

### Anti-patterns
Manual aria-current handling.

## Composition

### Works with
Items, links, separators.

### Does not work with
Unstructured elements.

### Requires
BreadcrumbItem structure.

## Governance

### Canon Rules
CR-001, CR-004

### Violations prevented
- Incorrect current page marking
- Broken navigation semantics

### SSR safety
Static rendering.

### Hydration safety
No runtime logic.

## Examples

### Minimal
```rust
view! {
  <Breadcrumb>
    <BreadcrumbItem>
      <BreadcrumbLink href="#">"Home"</BreadcrumbLink>
    </BreadcrumbItem>
  </Breadcrumb>
}
```

### With all props
```rust
view! {
  <Breadcrumb>
    <BreadcrumbItem>
      <BreadcrumbLink href="#" state=ActivityState::Inactive>"Home"</BreadcrumbLink>
    </BreadcrumbItem>
    <BreadcrumbItem>
      <BreadcrumbPage>"Page"</BreadcrumbPage>
    </BreadcrumbItem>
  </Breadcrumb>
}
```

### Common mistake + correction
Wrong: missing aria-current  
Correct: use BreadcrumbPage
