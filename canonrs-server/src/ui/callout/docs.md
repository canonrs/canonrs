# Callout

## Overview

### What it is
Callout is a feedback component designed to highlight contextual information such as tips, warnings, or notes. It provides structured semantic meaning and visual emphasis for important content.

### The problem it solves
Callouts are often implemented inconsistently, with incorrect roles and lack of semantic clarity. This leads to accessibility issues and unclear intent.

### How CanonRS enforces it
CalloutVariant defines semantic meaning and ARIA behavior. The primitive encodes these semantics directly into the DOM, ensuring consistent accessibility.

## Usage

### Basic usage
```rust
view! {
  <Callout>
    <CalloutDescription>"Info"</CalloutDescription>
  </Callout>
}
```

### With variants
```rust
view! {
  <Callout variant=CalloutVariant::Warning>
    <CalloutTitle>"Warning"</CalloutTitle>
    <CalloutDescription>"Be careful"</CalloutDescription>
  </Callout>
}
```

### With sizes
No size variants.

### With states
Static component.

### Composition patterns
Supports icon, title, and description.

## Variants

### Default
Neutral information.

### Info
Informational content.

### Warning
Cautionary messages.

### Destructive
Critical alerts.

## Sizes

### Default
Inherited.

## States

### Static
No dynamic state.

## API Reference

### Props
- variant: CalloutVariant
- class: String

### Types
- CalloutVariant

## Governance

### Canon Rules applied
CR-001, CR-004

### What the compiler enforces
Semantic correctness and accessibility.

### Before vs After
Before: div-based callouts.  
After: structured semantic component.

## Use Cases

### Tips
Provide helpful hints.

### Warnings
Highlight risks or issues.

## Related Components

### Alert
Use for system messages.

### Banner
Use for page-level messages.
