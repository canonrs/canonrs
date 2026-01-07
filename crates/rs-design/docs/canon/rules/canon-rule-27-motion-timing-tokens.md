# Canon Rule #27: Motion & Timing Tokens

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-01

## Principle
Motion creates **purposeful transitions** that guide user attention and provide feedback. All animations use **system-wide timing tokens** that respect user preferences and maintain consistency across components.

## Duration Tokens

### Canonical Timing Scale
```css
:root {
  /* Micro-interactions (instant feedback) */
  --motion-duration-instant: 0ms;
  --motion-duration-fast: 150ms;
  
  /* Standard transitions */
  --motion-duration-normal: 300ms;
  --motion-duration-comfortable: 400ms;
  
  /* Deliberate animations */
  --motion-duration-slow: 500ms;
  --motion-duration-slower: 700ms;
  
  /* Page transitions */
  --motion-duration-page: 1000ms;
}
```

### Usage Guidelines

**Instant (0ms):** Immediate state changes
```rust
// Color changes, visibility toggles
"transition-colors duration-[var(--motion-duration-instant)]"
```

**Fast (150ms):** Hover states, focus rings
```rust
// Interactive feedback
"transition-all duration-[var(--motion-duration-fast)]"
```

**Normal (300ms):** Default transitions
```rust
// Component state changes
"transition-all duration-[var(--motion-duration-normal)]"
```

**Slow (500ms+):** Deliberate animations
```rust
// Slide-ins, expansions
"transition-transform duration-[var(--motion-duration-slow)]"
```

## Easing Functions

### Canonical Easing Tokens
```css
:root {
  /* Standard easings */
  --motion-ease-linear: linear;
  --motion-ease-in: cubic-bezier(0.4, 0, 1, 1);
  --motion-ease-out: cubic-bezier(0, 0, 0.2, 1);
  --motion-ease-in-out: cubic-bezier(0.4, 0, 0.2, 1);
  
  /* Material Design easings */
  --motion-ease-standard: cubic-bezier(0.4, 0, 0.2, 1);      /* Balanced */
  --motion-ease-emphasized: cubic-bezier(0.2, 0, 0, 1);      /* Snappy exit */
  --motion-ease-decelerate: cubic-bezier(0, 0, 0.2, 1);      /* Enter screen */
  --motion-ease-accelerate: cubic-bezier(0.4, 0, 1, 1);      /* Exit screen */
  
  /* Playful easings */
  --motion-ease-bounce: cubic-bezier(0.68, -0.55, 0.265, 1.55);
  --motion-ease-spring: cubic-bezier(0.175, 0.885, 0.32, 1.275);
}
```

### Easing Application

**Entering Elements:** Use decelerate
```rust
"ease-[var(--motion-ease-decelerate)]"  // Slides in smoothly
```

**Exiting Elements:** Use accelerate
```rust
"ease-[var(--motion-ease-accelerate)]"  // Snaps away
```

**State Changes:** Use standard
```rust
"ease-[var(--motion-ease-standard)]"    // Balanced feel
```

**Playful UI:** Use bounce/spring sparingly
```rust
"ease-[var(--motion-ease-spring)]"      // Attention-grabbing
```

## Component Motion Patterns

### Button Interactions
```rust
const BUTTON_MOTION: &str = "\
    transition-all \
    duration-[var(--motion-duration-fast)] \
    ease-[var(--motion-ease-standard)]";
```

### Modal/Dialog Animations
```rust
// Backdrop fade
const BACKDROP_MOTION: &str = "\
    transition-opacity \
    duration-[var(--motion-duration-normal)]";

// Content scale + fade
const DIALOG_MOTION: &str = "\
    transition-all \
    duration-[var(--motion-duration-comfortable)] \
    ease-[var(--motion-ease-emphasized)] \
    data-[state=open]:animate-in \
    data-[state=closed]:animate-out \
    data-[state=open]:fade-in-0 \
    data-[state=closed]:fade-out-0 \
    data-[state=open]:zoom-in-95 \
    data-[state=closed]:zoom-out-95";
```

### Dropdown Menus
```rust
const DROPDOWN_MOTION: &str = "\
    data-[state=open]:animate-in \
    data-[state=closed]:animate-out \
    data-[state=closed]:fade-out-0 \
    data-[state=open]:fade-in-0 \
    data-[state=closed]:zoom-out-95 \
    data-[state=open]:zoom-in-95 \
    duration-[var(--motion-duration-fast)]";
```

### Slide Transitions
```rust
// Sidebar slide-in
const SIDEBAR_MOTION: &str = "\
    transition-transform \
    duration-[var(--motion-duration-comfortable)] \
    ease-[var(--motion-ease-emphasized)] \
    data-[state=open]:translate-x-0 \
    data-[state=closed]:-translate-x-full";
```

## Reduced Motion Support

### System Preference Detection
```css
@media (prefers-reduced-motion: reduce) {
  :root {
    /* Override all durations to instant */
    --motion-duration-fast: 0ms;
    --motion-duration-normal: 0ms;
    --motion-duration-comfortable: 0ms;
    --motion-duration-slow: 0ms;
    --motion-duration-slower: 0ms;
    --motion-duration-page: 0ms;
  }
  
  * {
    /* Disable all animations */
    animation-duration: 0ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0ms !important;
  }
}
```

### Rust Implementation
```rust
#[component]
pub fn AnimatedComponent() -> impl IntoView {
    let prefers_reduced_motion = RwSignal::new(false);
    
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(media_query) = window.match_media("(prefers-reduced-motion: reduce)") {
                if let Some(query) = media_query {
                    prefers_reduced_motion.set(query.matches());
                }
            }
        }
    });
    
    let motion_class = move || {
        if prefers_reduced_motion.get() {
            ""  // No animation
        } else {
            "transition-all duration-[var(--motion-duration-normal)]"
        }
    };
    
    view! {
        <div class=motion_class>
    }
}
```

## Animation Keyframes

### Fade Animations
```css
@keyframes fade-in {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes fade-out {
  from { opacity: 1; }
  to { opacity: 0; }
}
```

### Scale Animations
```css
@keyframes zoom-in {
  from { transform: scale(0.95); opacity: 0; }
  to { transform: scale(1); opacity: 1; }
}

@keyframes zoom-out {
  from { transform: scale(1); opacity: 1; }
  to { transform: scale(0.95); opacity: 0; }
}
```

### Slide Animations
```css
@keyframes slide-in-from-right {
  from { transform: translateX(100%); }
  to { transform: translateX(0); }
}

@keyframes slide-out-to-right {
  from { transform: translateX(0); }
  to { transform: translateX(100%); }
}
```

## Performance Optimization

### GPU-Accelerated Properties
```rust
// Only animate transform and opacity for 60fps
const PERFORMANT_MOTION: &str = "\
    transition-transform \
    transition-opacity \
    will-change-transform"; // Hint to browser
```

### Avoid Animating
```rust
// ❌ Triggers layout recalculation
"transition-all"  // Animates width, height, etc.

// ✅ GPU-accelerated only
"transition-transform transition-opacity"
```

## Choreography

### Stagger Delays
```css
/* Stagger list item animations */
.list-item:nth-child(1) { animation-delay: 0ms; }
.list-item:nth-child(2) { animation-delay: 50ms; }
.list-item:nth-child(3) { animation-delay: 100ms; }
.list-item:nth-child(4) { animation-delay: 150ms; }
```

### Sequential Animations
```rust
// Fade in → Slide in → Scale
"animate-[fade-in_300ms_ease-out,slide-in_300ms_200ms_ease-out,scale-in_200ms_400ms_ease-out]"
```

## Prohibited Patterns

### ❌ Hardcoded Timing
```rust
// FORBIDDEN
"duration-300"  // Use var(--motion-duration-normal)
"ease-in-out"   // Use var(--motion-ease-standard)
```

### ❌ Transition All
```rust
// FORBIDDEN (performance)
"transition-all duration-300"

// CORRECT (specific properties)
"transition-transform transition-opacity duration-[var(--motion-duration-normal)]"
```

### ❌ Excessive Animation
```rust
// FORBIDDEN (motion sickness risk)
"animate-spin animate-bounce animate-pulse"  // Too much!
```

## Validation Checklist
- [ ] All durations use CSS variable tokens
- [ ] Easings match motion purpose (enter/exit/state)
- [ ] Reduced motion preference supported
- [ ] Only transform/opacity animated for performance
- [ ] No animations exceed 1000ms
- [ ] Stagger delays used for list animations

## References
- [WCAG 2.1: Animation from Interactions](https://www.w3.org/WAI/WCAG21/Understanding/animation-from-interactions.html)
- [Material Motion System](https://m3.material.io/styles/motion/overview)
- [Framer Motion Easings](https://www.framer.com/motion/transition/)
- [Canon Rule #23: State Tokens](./canon-rule-23-state-tokens.md)
