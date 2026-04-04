# Alert Dialog

id: alert-dialog
label: Alert Dialog
family: family-a-overlay
category: Overlay
intent: Confirm destructive actions with user
description: Alert dialog for critical confirmations
composable: true
capabilities: OpenClose, FocusTrap, AriaModal
required_parts: AlertDialogContent, AlertDialogTitle
optional_parts: AlertDialogOverlay, AlertDialogDescription, AlertDialogAction
tags: alert-dialog, confirm, destructive, modal, overlay
keywords: 
pain: Destructive dialogs lack proper role and accessibility enforcement
promise: Alertdialog role and accessibility guaranteed by component contract
why: AlertDialog reuses Dialog but enforces role="alertdialog" and assertive aria-live. The specialized content primitive ensures critical actions are announced correctly. This prevents misuse of generic dialogs for destructive flows.
rules: CR-001, CR-004
use_cases: delete confirmation, critical actions
related: dialog, drawer, sheet, modal, confirm_dialog, tooltip, hover_card, popover


file: alert_dialog_ui.css
tokens: alert-dialog-*, space-*, radius-*, shadow-*, motion-*
foundation: spacing, radius, shadow, motion
states: open, closed
island: alert_dialog_island.rs

## before
// ❌ Typical
view! {
  <div class="modal">
    <p>"Delete account?"</p>
    <button>"Confirm"</button>
  </div>
}

## after
// ✅ CanonRS
view! {
  <AlertDialog>
    <AlertDialogTrigger>"Delete"</AlertDialogTrigger>
    <AlertDialogPortal>
      <AlertDialogContent>
        <AlertDialogTitle>"Confirm"</AlertDialogTitle>
        <AlertDialogDescription>"This cannot be undone"</AlertDialogDescription>
      </AlertDialogContent>
    </AlertDialogPortal>
  </AlertDialog>
}
