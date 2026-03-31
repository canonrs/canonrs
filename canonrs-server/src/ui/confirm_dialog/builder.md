# Confirm Dialog

id: confirm-dialog
label: Confirm Dialog
family: overlay
category: Overlay
intent: Ask user to confirm an action
description: Confirmation dialog
composable: false
capabilities: OpenClose, FocusTrap, AriaModal
required_parts: 
optional_parts: 
tags: confirm-dialog, confirm, confirmation, cancel, ok
keywords: 
pain: Confirmation dialogs miss ARIA roles and destructive intent signaling
promise: Confirmation semantics and intent enforced via variant and structure
why: ConfirmDialogPrimitive enforces role="alertdialog" with variant-driven semantics. VisibilityState controls lifecycle and accessibility attributes. This guarantees proper focus and urgency communication.
rules: CR-001, CR-004
use_cases: delete confirmation, critical actions
related: dialog, alert_dialog, drawer, sheet, modal, tooltip, hover_card, popover

## before
// ❌ Typical
view! {
  <div class="modal">"Are you sure?"</div>
}

## after
// ✅ CanonRS
view! {
  <ConfirmDialog />
}
