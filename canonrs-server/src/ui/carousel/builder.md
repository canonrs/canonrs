# Carousel

id: carousel
label: Carousel
family: family-g-composite
category: Display
intent: Cycle through items horizontally
description: Image carousel slider
composable: true
capabilities: KeyboardArrows
required_parts: CarouselTrack, CarouselItem
optional_parts: CarouselPrev, CarouselNext, CarouselIndicators
tags: carousel, slider, gallery, images, slideshow
keywords: 
pain: Carousels break accessibility and state synchronization across slides
promise: Slide state and navigation semantics enforced via structured primitives
why: CarouselPrimitive defines roles and slide semantics including aria labels and state. ActivityState and VisibilityState control active and hidden slides. This ensures accessibility and predictable slideshow behavior.
rules: CR-001, CR-004
use_cases: image galleries, feature sliders
related: avatar, icon, logo, code_block, markdown, chart, stat, inline_meta, kbd, badge


file: carousel_ui.css
tokens: carousel-*, size-*, space-*, radius-*, motion-*, font-*
foundation: spacing, size, radius, motion, typography
states: active, inactive
island: carousel_island.rs

pillar: content_display

badges: SSR Safe, Hydration Safe, Token Driven, Deterministic API, Zero Drift, Island Architecture

## before
// ❌ Typical
view! {
  <div class="carousel">
    <div class="slide active">"Slide 1"</div>
  </div>
}

## after
// ✅ CanonRS
view! {
  <Carousel>
    <CarouselTrack>
      <CarouselItem>"Slide 1"</CarouselItem>
    </CarouselTrack>
  </Carousel>
}
