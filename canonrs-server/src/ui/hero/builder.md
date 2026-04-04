# Hero UI

id: hero-ui
label: Hero UI
family: family-h-layout
category: Display
intent: Semantic UI elements inside HeroBlock
description: Hero typography and label components
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: hero, title, subtitle, description, heading
keywords: 
pain: Hero sections lack consistent structure and semantic grouping
promise: Hero content structured via explicit semantic primitives
why: HeroPrimitive defines a structural block with dedicated parts like title, description and actions. Each part is explicitly declared. This guarantees predictable layout composition.
rules: CR-001, CR-004
use_cases: landing pages, marketing headers
related: 


file: hero_ui.css
tokens: hero-*, space-*, font-*
foundation: spacing, typography
states: 
island: hero_island.rs

## before
// ❌ Typical
view! {
  <div class="hero">
    <h1>"Title"</h1>
  </div>
}

## after
// ✅ CanonRS
view! {
  <HeroTitle>"Title"</HeroTitle>
}
