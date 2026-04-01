# Aspect Ratio

## Overview

### What it is
Aspect Ratio is a layout component that enforces a consistent width-to-height ratio for its children. It ensures that content such as images or videos maintains its proportions regardless of container size.

### The problem it solves
Without a controlled aspect ratio, media elements often cause layout shifts during resizing or loading. This leads to inconsistent rendering and poor user experience.

### How CanonRS enforces it
AspectRatioPrimitive encodes ratio_w and ratio_h into structural attributes, guaranteeing layout stability without runtime calculations.

## Usage

### Basic usage
```rust
view! {
  <AspectRatio>
    <img src="img.png" />
  </AspectRatio>
}
```

### With variants
No variants are defined.

### With sizes
Size is determined by parent container.

### With states
No state transitions.

### Composition patterns
Used as a wrapper around media elements.

## Variants

### Default
Single enforced behavior.

## Sizes

### Default
Inherited.

## States

### Static
No dynamic state.

## API Reference

### Props
- ratio_w: f32
- ratio_h: f32
- class: String

### Types
None

## Governance

### Canon Rules applied
CR-001, CR-004

### What the compiler enforces
Valid ratio structure.

### Before vs After
Before: manual padding hacks.  
After: structural ratio enforcement.

## Use Cases

### Video containers
Maintain 16:9 ratio.

### Image previews
Ensure consistent thumbnails.

## Related Components

### Card
Use Card for content containers with layout semantics.
