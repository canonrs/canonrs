# Tooltip

id: tooltip
label: Tooltip
family: overlay
category: Overlay
intent: Show brief label on hover/focus
description: Hover tooltip
composable: true
capabilities: OpenClose
required_parts: TooltipTrigger, TooltipContent
optional_parts: TooltipProvider
tags: tooltip, hint, hover, info, help
keywords: 
pain: Tooltip requires id wiring and loses sync between trigger and content
promise: Trigger-content relationship enforced without manual wiring
why: TooltipPrimitive uses visibility state and DOM structure instead of ids. aria-describedby is optional but supported. This guarantees consistent overlay behavior.
rules: CR-001, CR-004
use_cases: hints, help text
related: dialog, alert_dialog, drawer, sheet, modal, confirm_dialog, hover_card, popover

## before
// ❌ Typical
view! {
  <span title="Info">"Hover"</span>
}

## after
// ✅ CanonRS
view! {
  <Tooltip>
    <TooltipTrigger>"Hover"</TooltipTrigger>
    <TooltipContent>"Info"</TooltipContent>
  </Tooltip>
}
