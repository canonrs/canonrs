# Select

id: select
label: Select
family: input
category: Form
intent: Choose one option from a list
description: Dropdown select input
composable: true
capabilities: OpenClose, Disabled
required_parts: SelectTrigger, SelectContent, SelectItem
optional_parts: SelectValue, SelectSeparator
tags: select, dropdown, choose, options, list, combo
keywords: 
pain: Select components require manual state sync between trigger and options
promise: Selection, visibility and interaction fully governed by structure
why: SelectPrimitive encodes open state, selection and disabled behavior via data attributes. Trigger and content are synchronized without id wiring. This guarantees consistent dropdown behavior.
rules: CR-001, CR-004
use_cases: forms, filters
related: combobox, radio, radio_group, color_picker, slider

## before
// ❌ Typical
view! {
  <select>
    <option>"A"</option>
  </select>
}

## after
// ✅ CanonRS
view! {
  <Select>
    <SelectTrigger>
      <SelectValue placeholder="Select..." />
    </SelectTrigger>
    <SelectContent>
      <SelectItem value="a">"A"</SelectItem>
    </SelectContent>
  </Select>
}
