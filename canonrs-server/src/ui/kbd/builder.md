# Kbd

id: kbd
label: Kbd
family: typography
category: Display
intent: Display keyboard shortcut
description: Keyboard shortcut display
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: kbd, keyboard, shortcut, ctrl, cmd
keywords: 
pain: Keyboard shortcuts displayed inconsistently with ad-hoc styling
promise: Shortcut representation standardized via size and variant enums
why: KbdPrimitive encodes size and variant into data attributes. Group and separator primitives enforce consistent composition. This guarantees uniform shortcut display.
rules: CR-001, CR-004
use_cases: shortcuts, docs
related: avatar, icon, logo, code_block, markdown, chart, stat, inline_meta

## before
// ❌ Typical
view! {
  <span class="kbd">Ctrl + K</span>
}

## after
// ✅ CanonRS
view! {
  <KbdGroup>
    <Kbd>"Ctrl"</Kbd>
    <KbdSeparator />
    <Kbd>"K"</Kbd>
  </KbdGroup>
}
