# Banner

## Overview

### What it is
Banner is a feedback component used to display persistent, page-level messages in CanonRS. It provides a structured and accessible way to communicate important information to users across an entire interface.

It supports composability and controlled visibility, ensuring consistent behavior across different contexts.

### The problem it solves
Traditional banners often lack consistent accessibility semantics and visibility control. Developers rely on ad hoc logic, leading to inconsistent behavior and poor user experience.

Banner solves this by enforcing visibility and accessibility through structured primitives and state encoding.

### How CanonRS enforces it
BannerVariant defines semantic meaning and ARIA behavior, while VisibilityState ensures correct open and hidden states without runtime logic. This guarantees consistent rendering and accessibility.

## Usage

### Basic usage
```rust
view! {
  <Banner>
    <BannerContent>"Message"</BannerContent>
  </Banner>
}
```

### With variants
Variants define the tone and semantic meaning of the banner.

### With sizes
No size variants are defined.

### With states
Visibility is controlled internally, ensuring consistent behavior without manual toggling.

### Composition patterns
Banner supports content, actions, and close controls, enabling flexible layouts.

## Variants

### Info
Used for informational messages.

### Warning
Used for cautionary messages.

### Destructive
Used for critical alerts.

## Sizes

### Default
Inherited from layout.

## States

### Visible
Banner is displayed and announced correctly.

### Hidden
Banner is not rendered or is visually hidden.

## API Reference

### Props
- variant: BannerVariant
- class: String

### Types
- BannerVariant

## Governance

### Canon Rules applied
CR-001, CR-004

### What the compiler enforces
Correct structure and accessibility semantics.

### Before vs After
Before: inconsistent banner implementations.  
After: structured and accessible banners.

## Use Cases

### System announcements
Used to communicate updates or maintenance.

### Warnings
Used to highlight important notices.

## Related Components

### Alert
Use Alert for localized messages.

### Toast
Use Toast for transient notifications.
