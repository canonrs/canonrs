# Breadcrumb

## Overview

### What it is
Breadcrumb is a navigation component used to represent the current location within a hierarchy. It provides a structured trail that helps users understand their position and navigate back through levels.

It enforces accessibility and semantic correctness through activity state mapping.

### The problem it solves
Typical breadcrumb implementations fail to correctly indicate the current page, often missing proper ARIA attributes. This results in accessibility issues and inconsistent navigation behavior.

Breadcrumb solves this by enforcing activity state and mapping it automatically to aria-current.

### How CanonRS enforces it
ActivityState defines whether a breadcrumb item is active. The primitive maps this state to aria-current, ensuring correct semantics without manual intervention.

## Usage

### Basic usage
```rust
view! {
  <Breadcrumb>
    <BreadcrumbItem>
      <BreadcrumbLink href="#">"Home"</BreadcrumbLink>
    </BreadcrumbItem>
  </Breadcrumb>
}
```

### With variants
No visual variants are defined.

### With sizes
Size is inherited from context.

### With states
Activity state defines current page.

### Composition patterns
Breadcrumb uses items, links, separators, and page indicators.

## Variants

### Default
Single consistent behavior.

## Sizes

### Default
Inherited.

## States

### Active
Represents current page.

### Inactive
Represents navigable link.

## API Reference

### Props
- class: String

### Types
- ActivityState

## Governance

### Canon Rules applied
CR-001, CR-004

### What the compiler enforces
Correct activity state and structure.

### Before vs After
Before: manual aria-current handling.  
After: automatic state mapping.

## Use Cases

### Navigation trails
Show hierarchy in applications.

### Page hierarchy
Provide context for deep navigation.

## Related Components

### NavigationMenu
Use for primary navigation.

### Sidebar
Use for persistent navigation.
