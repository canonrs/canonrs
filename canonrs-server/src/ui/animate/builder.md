# Animate

id: animate
label: Animate
family: family-i-animation
category: Display
intent: Apply CSS animations to children
description: Animation wrapper component
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: animate, animation, transition, motion
keywords: 
pain: Animations rely on fragile class names and inconsistent timing values
promise: Animation type and easing enforced through typed enums
why: AnimationName and AnimationEasing define allowed motion patterns. The primitive encodes animation parameters into data-rs attributes, avoiding class-based drift. This ensures consistent animation behavior across SSR and client.
rules: CR-001, CR-004
use_cases: page transitions, modal animations
related: carousel


file: animate_ui.css
tokens: animate-*, motion-*, transform-*, opacity-*
foundation: motion, interaction
states: active, inactive
island: animate_island.rs

## before
// ❌ Typical
view! {
  <div class="fade-in ease-in-out duration-300">"Content"</div>
}

## after
// ✅ CanonRS
view! {
  <Animate animation=AnimationName::FadeIn duration="300ms">
    "Content"
  </Animate>
}
