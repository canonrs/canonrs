# Command

id: command
label: Command
family: interactive
category: Action
intent: Command palette for quick actions
description: Command palette
composable: true
capabilities: OpenClose, Typeahead
required_parts: CommandInput, CommandList
optional_parts: CommandItem, CommandGroup, CommandSeparator, CommandEmpty
tags: command, palette, spotlight, search, shortcut
keywords: 
pain: Command palette lacks keyboard navigation and ARIA consistency
promise: Command palette semantics and selection fully enforced
why: CommandPrimitive enforces listbox semantics with structured input, grouping and items. SelectionState and ActivityState control highlight and selection behavior. This guarantees consistent keyboard navigation and accessibility.
rules: CR-001, CR-004
use_cases: command palette, quick navigation
related: dropdown_menu, context_menu, menubar

## before
// ❌ Typical
view! {
  <input placeholder="Search" />
  <div class="list">
    <div>"Item"</div>
  </div>
}

## after
// ✅ CanonRS
view! {
  <Command>
    <CommandInput placeholder="Search..." />
    <CommandList>
      <CommandItem>"Item"</CommandItem>
    </CommandList>
  </Command>
}
