# Color Picker

id: color-picker
label: Color Picker
family: family-c-forms
category: Form
intent: Select a color value
description: Color picker input
composable: true
capabilities: Value, Disabled
required_parts: 
optional_parts: ColorPickerSwatch, ColorPickerInput
tags: color-picker, color, palette, rgb, hex
keywords: 
pain: Color inputs lack consistent selection and accessibility behavior
promise: Color selection and state enforced via structured primitives
why: ColorPickerPrimitive uses SelectionState and VisibilityState for interaction control. The contract ensures consistent value handling and accessibility. This prevents ad-hoc color input implementations.
rules: CR-001, CR-004
use_cases: theme customization, design tools
related: combobox, radio, radio_group, slider


file: color_picker_ui.css
tokens: color-picker-*, size-*, space-*, radius-*, motion-*, font-*, state-*
foundation: spacing, size, radius, motion, typography, interaction
states: selected, disabled
island: color_picker_island.rs

## before
// ❌ Typical
view! {
  <input type="color" value="#000" />
}

## after
// ✅ CanonRS
view! {
  <ColorPicker value="#000000" />
}
