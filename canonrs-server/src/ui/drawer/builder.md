# Drawer

id: drawer
label: Drawer
family: family-a-overlay
category: Overlay
intent: Slide-in panel for supplementary content
description: Slide-out drawer component
composable: true
capabilities: OpenClose, FocusTrap, KeyboardEsc, AriaModal
required_parts: DrawerContent
optional_parts: DrawerOverlay
tags: drawer, slide, lateral, panel, mobile
keywords: 
pain: Slide panels lack consistent direction and accessibility semantics
promise: Drawer direction and visibility enforced via typed contract
why: DrawerPrimitive encodes side and visibility with DrawerSide and VisibilityState. ARIA attributes ensure accessibility compliance. This guarantees predictable slide behavior.
rules: CR-001, CR-004
use_cases: mobile navigation, side panels
related: dialog, alert_dialog, sheet, modal, confirm_dialog, tooltip, hover_card, popover


file: drawer_ui.css
tokens: drawer-*, size-*, space-*, radius-*, motion-*, font-*
foundation: spacing, size, radius, motion, typography
states: open, closed
island: drawer_island.rs

## before
// ❌ Typical
view! {
  <div class="drawer left">"Content"</div>
}

## after
// ✅ CanonRS
view! {
  <Drawer>
    <DrawerOverlay />
    <DrawerContent aria_labelledby="title" />
  </Drawer>
}
