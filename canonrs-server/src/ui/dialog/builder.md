# Dialog

id: dialog
label: Dialog
family: overlay
category: Overlay
intent: Display critical content requiring user interaction
description: Modal dialog component
composable: true
capabilities: OpenClose, FocusTrap, KeyboardEsc, AriaModal
required_parts: DialogContent, DialogTitle
optional_parts: DialogOverlay, DialogDescription, DialogClose
tags: dialog, modal, popup, window, overlay, confirmation
keywords: 
pain: Dialogs break focus trap and accessibility roles
promise: Dialog accessibility and lifecycle enforced via primitives
why: DialogPrimitive defines overlay, portal and content with ARIA compliance. VisibilityState guarantees correct open/close synchronization. This ensures safe modal behavior across SSR and client.
rules: CR-001, CR-004
use_cases: modals, forms
related: alert_dialog, drawer, sheet, modal, confirm_dialog, tooltip, hover_card, popover

## before
// ❌ Typical
view! {
  <div class="modal">"Content"</div>
}

## after
// ✅ CanonRS
view! {
  <Dialog>
    <DialogTrigger>"Open"</DialogTrigger>
    <DialogPortal>
      <DialogOverlay />
      <DialogContent>
        <DialogTitle>"Title"</DialogTitle>
      </DialogContent>
    </DialogPortal>
  </Dialog>
}
