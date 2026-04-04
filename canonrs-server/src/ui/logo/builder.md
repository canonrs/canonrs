# Site Logo

id: site-logo
label: Site Logo
family: family-f-data
category: Brand
intent: Brand identity link pointing to home
description: CanonRS logo combining SVG icon, wordmark and optional tagline
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: logo, brand, home, identity, canonrs, wordmark, icon, tagline
keywords: 
pain: Brand logos implemented inconsistently across layouts and break navigation semantics
promise: Brand identity structure and navigation behavior enforced in a single contract
why: LogoPrimitive enforces anchor semantics, size and variant at the root element. Icon, wordmark and tagline are explicitly structured parts. This guarantees consistent brand rendering and correct navigation behavior.
rules: CR-001, CR-004
use_cases: site header, app navigation
related: avatar, icon, code_block, markdown, chart, stat, inline_meta, kbd, badge, carousel


file: logo_ui.css
tokens: logo-*, size-*, font-*
foundation: size, typography
states: 
island: logo_island.rs

pillar: content_display

badges: SSR Safe, Hydration Safe, Token Driven, Deterministic API, Zero Drift

## before
// ❌ Typical
view! {
  <a href="/" class="logo">
    <img src="/logo.svg" />
    <span>"Brand"</span>
  </a>
}

## after
// ✅ CanonRS
view! {
  <Logo>
    <LogoWordmarkPrimitive>"Brand"</LogoWordmarkPrimitive>
  </Logo>
}
