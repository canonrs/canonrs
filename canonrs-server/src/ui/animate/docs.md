# Animate

## Overview

### What it is
Animate is a utility component in CanonRS designed to apply consistent, governed motion to UI elements. It wraps children and applies animation behavior using typed enums instead of fragile class names. The component ensures that animation parameters such as easing and duration are controlled and predictable. This eliminates inconsistencies between different parts of the UI.

### The problem it solves
Traditional animation relies on arbitrary CSS classes and inline styles, leading to inconsistent timing, easing, and naming conventions. Developers often mix different animation systems, creating visual drift across the application. This also introduces SSR and hydration inconsistencies when animations depend on runtime conditions. Animate solves this by enforcing animation contracts through typed enums.

### How CanonRS enforces it
AnimationName and AnimationEasing are strictly typed enums that define allowed animation patterns. The primitive encodes animation behavior into data-rs attributes instead of class names. This ensures that animations are deterministic and SSR-safe. The compiler guarantees that only valid animation configurations are used.

## Usage

### Basic usage
```rust
view! {
  <Animate>"Content"</Animate>
}
```
This applies a default fade-in animation. The component ensures consistent behavior without additional configuration. No manual class management is required. The animation is SSR-safe.

### With variants
Variants are defined through AnimationName. Each variant represents a specific motion pattern. This ensures consistency across the system. Developers cannot introduce arbitrary animations.

### With sizes
Animate does not define sizes directly. Duration and delay act as timing controls. These parameters influence perceived animation scale. This keeps the API focused and predictable.

### With states
Animations are applied declaratively without runtime toggles. State is encoded in attributes, not signals. This prevents hydration mismatch. The animation lifecycle remains consistent.

### Composition patterns
Animate wraps any component or element. It is often used with overlays, modals, and transitions. Composition does not break animation guarantees. Behavior remains isolated and predictable.

## Variants

### FadeIn
FadeIn is the default animation and provides a smooth opacity transition. It is suitable for most UI elements entering the viewport. The easing ensures a natural feel. This variant is widely used for page transitions.
```rust
<Animate animation=AnimationName::FadeIn>
```

### SlideIn
SlideIn introduces directional movement. It is useful for panels and modals. The motion communicates spatial hierarchy. It enhances user understanding of layout changes.
```rust
<Animate animation=AnimationName::SlideIn>
```

## Sizes

### Duration
Duration controls how long the animation lasts. Short durations create snappy interactions. Longer durations emphasize transitions. The value is passed as a string.
```rust
<Animate duration="300ms">
```

## States

### Active
Active state represents when animation is applied. It is deterministic and does not rely on runtime checks. The DOM reflects the animation state directly. This ensures SSR consistency.

### Idle
Idle state means no animation is applied. The component still renders normally. This allows predictable fallback behavior. No visual glitches occur.

## API Reference

### Props
- animation: AnimationName (default: FadeIn)
- easing: AnimationEasing (default: EaseInOut)
- duration: String
- delay: String
- class: String

### Types
- AnimationName
- AnimationEasing

## Governance

### Canon Rules applied
- CR-001
- CR-004

### What the compiler enforces
- Valid animation types
- Consistent easing
- No arbitrary class usage

### Before vs After
Before: class-based animation  
After: typed animation contract

## Use Cases

### page transitions
Animate ensures smooth transitions between pages. It standardizes motion across the application. This improves perceived performance. The behavior is predictable.

### modal animations
Animate enhances modal entry and exit transitions. It ensures consistent motion patterns. This improves user experience. No manual animation logic is needed.

## Related Components

### carousel
Carousel manages motion across multiple items. Animate handles individual transitions. Use carousel for sequence-based motion. Use animate for isolated transitions.
