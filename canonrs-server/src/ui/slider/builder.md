# Slider

id: slider
label: Slider
family: input
category: Form
intent: Select a value within a range
description: Range slider input
composable: true
capabilities: Value, Disabled
required_parts: SliderTrack, SliderThumb
optional_parts: SliderRange
tags: slider, range, interval, volume, value, drag
keywords: 
pain: Sliders allow invalid values and break accessibility attributes
promise: Value clamped and ARIA attributes enforced automatically
why: SliderPrimitive clamps value within min/max and maps percent and aria-valuenow. Track and thumb are structurally defined. This guarantees consistent interaction behavior.
rules: CR-001, CR-004
use_cases: volume control, range selection
related: combobox, radio, radio_group, color_picker

## before
// ❌ Typical
view! {
  <input type="range" value="200" />
}

## after
// ✅ CanonRS
view! {
  <Slider min=0.0 max=100.0 value=50.0 />
}
