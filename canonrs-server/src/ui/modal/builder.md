# Modal

id: modal
label: Modal
family: overlay
category: Overlay
intent: Generic modal container
description: Modal window component
composable: true
capabilities: OpenClose, FocusTrap, AriaModal
required_parts: 
optional_parts: ModalOverlay, ModalContent, ModalClose
tags: modal, window, popup, overlay, dialog
keywords: 
pain: Modals desync visibility, aria-hidden and focus management
promise: Modal visibility and accessibility fully synchronized via state
why: ModalPrimitive maps VisibilityState to aria-hidden and hidden attributes. Trigger and content share the same state contract. This guarantees consistent open/close behavior and accessibility.
rules: CR-001, CR-004
use_cases: dialogs, overlays
related: dialog, alert_dialog, drawer, sheet, confirm_dialog, tooltip, hover_card, popover

## before
// ❌ Typical
view! {
  {if open { view! { <div class="modal">"Content"</div> } }}
}

## after
// ✅ CanonRS
view! {
  <Modal state=VisibilityState::Open>
    <ModalContent>"Content"</ModalContent>
  </Modal>
}
