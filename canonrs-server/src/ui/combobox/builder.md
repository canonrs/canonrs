# Combobox

id: combobox
label: Combobox
family: input
category: Form
intent: Search and select from a list
description: Searchable combo box
composable: true
capabilities: OpenClose, Disabled
required_parts: ComboboxTrigger, ComboboxList, ComboboxItem
optional_parts: 
tags: combobox, search, autocomplete, filter, combo
keywords: 
pain: Searchable selects break ARIA roles and keyboard navigation
promise: Combobox roles and interaction fully enforced by structure
why: ComboboxPrimitive defines proper ARIA roles and input behavior. SelectionState and VisibilityState control dropdown interaction. This guarantees accessible and predictable combobox behavior.
rules: CR-001, CR-004
use_cases: autocomplete, search dropdown
related: radio, radio_group, color_picker, slider

## before
// ❌ Typical
view! {
  <input type="text" />
  <ul><li>"Option"</li></ul>
}

## after
// ✅ CanonRS
view! {
  <Combobox>
    <ComboboxTrigger>"Select..."</ComboboxTrigger>
    <ComboboxList>
      <ComboboxItem value="1">"Option"</ComboboxItem>
    </ComboboxList>
  </Combobox>
}
