# Hover Card

id: hover-card
label: Hover Card
family: family-a-overlay
category: Overlay
intent: Show rich preview on hover
description: Hover card popup
composable: true
capabilities: OpenClose
required_parts: HoverCardTrigger, HoverCardContent
optional_parts: 
tags: hover-card, preview, card, hover, popup
keywords: 
pain: Hover previews break focus and visibility synchronization
promise: Hover state and positioning enforced via visibility contract
why: HoverCardPrimitive uses VisibilityState and side enums to control display. Trigger and content share synchronized state. This guarantees consistent hover behavior and accessibility.
rules: CR-001, CR-004
use_cases: user previews, tooltips with content
related: dialog, alert_dialog, drawer, sheet, modal, confirm_dialog, tooltip, popover


file: hover_card_ui.css
tokens: hover-card-*, space-*, radius-*, shadow-*, motion-*
foundation: spacing, radius, shadow, motion
states: open, closed
island: hover_card_island.rs

pillar: overlay

badges: SSR Safe, Hydration Safe, Token Driven, Deterministic API, Zero Drift, Island Architecture

## before
// ❌ Typical
view! {
  <div on:mouseover=show>"Hover"</div>
}

## after
// ✅ CanonRS
view! {
  <HoverCard>
    <HoverCardTrigger>"Hover"</HoverCardTrigger>
    <HoverCardContent>"Content"</HoverCardContent>
  </HoverCard>
}
