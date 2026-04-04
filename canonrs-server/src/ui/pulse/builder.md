# Pulse

id: pulse
label: Pulse
family: family-e-feedback
category: Feedback
intent: Animated attention indicator
description: Pulse animation wrapper
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: pulse, animation, glow, attention, highlight, ping
keywords: 
pain: Animations applied inconsistently with arbitrary classes and speeds
promise: Animation variant, size and speed strictly typed
why: PulsePrimitive encodes variant, size and speed as enums mapped to data attributes. This removes class-based inconsistencies. It guarantees predictable animation behavior.
rules: CR-001, CR-004
use_cases: status indicators, attention highlights
related: progress, spinner, skeleton, loading_overlay, doc_progress


file: pulse_ui.css
tokens: pulse-*, size-*, motion-*
foundation: size, motion
states: active
island: pulse_island.rs

pillar: progress

badges: SSR Safe, Hydration Safe, Token Driven, Deterministic API, Zero Drift

## before
// ❌ Typical
view! {
  <span class="pulse-fast big"></span>
}

## after
// ✅ CanonRS
view! {
  <Pulse variant=PulseVariant::Emphasized speed=PulseSpeed::Fast />
}
