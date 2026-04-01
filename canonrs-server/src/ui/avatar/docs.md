# Avatar

## Overview

### What it is
Avatar is a data display component designed to represent a user profile image with a reliable fallback mechanism. It ensures that user identity is consistently rendered even when image loading fails.

### The problem it solves
Traditional avatar implementations rely on event handlers to detect image failures, which often leads to inconsistent fallback behavior and broken UI states.

### How CanonRS enforces it
Avatar uses VisibilityState to control rendering between AvatarImage and AvatarFallback. This guarantees deterministic fallback behavior without manual logic.

## Usage

### Basic usage
```rust
view! {
  <Avatar>
    <AvatarFallback>"AB"</AvatarFallback>
  </Avatar>
}
```

### With variants
Shape is controlled via AvatarShape enum.

### With sizes
Size is defined via AvatarSize enum.

### With states
Status indicators reflect user presence.

### Composition patterns
Avatar combines image, fallback, status, and badge.

## Variants

### Circle
Default rounded avatar.

### Square
Sharp edges.

### Rounded
Soft corners.

## Sizes

### Xs, Sm, Md, Lg, Xl
Predefined size scale.

## States

### Online
Shows active indicator.

### Offline
No indicator.

### Busy
Shows busy status.

### Away
Shows idle status.

## API Reference

### Props
- size: AvatarSize
- shape: AvatarShape
- status: Option<AvatarStatus>
- animated: bool
- badge: Option<i32>
- class: String

### Types
- AvatarSize
- AvatarShape
- AvatarStatus

## Governance

### Canon Rules applied
CR-001, CR-004

### What the compiler enforces
Correct composition and fallback behavior.

### Before vs After
Before: manual error handling.  
After: state-driven rendering.

## Use Cases

### User profile
Display user identity.

### Team lists
Show multiple users consistently.

## Related Components

### Icon
Use for symbolic representation.

### Logo
Use for brand identity.
