Alert Dialog is a specialized overlay component that enforces role="alertdialog" and assertive announcement behavior. It is designed for destructive or critical actions requiring user confirmation.

AlertDialogPrimitive defines the core behavior. It manages open/close state, role assignment, and accessibility. AlertDialogContentPrimitive, Title, and Description define structure and semantics.

Focus trapping is automatic. Interaction outside the dialog is prevented. This ensures accessibility compliance.

SSR consistency is guaranteed. The dialog structure and attributes are rendered server-side. Hydration does not alter DOM structure.

Triggers and actions use data attributes. data-rs-dialog-trigger opens the dialog. data-rs-dialog-close closes it. This removes manual event wiring.

Required structure:
- AlertDialogContent
- AlertDialogTitle

Optional:
- Overlay
- Description

The component prevents misuse of generic dialogs for critical flows. It enforces correct semantics and behavior.

Common issues avoided:
- Missing alertdialog role
- No focus trapping
- Incorrect announcement behavior

Performance is efficient. Declarative attributes handle behavior without heavy runtime logic.

Use cases include delete confirmations and irreversible actions.

Key properties:
- Enforced accessibility semantics
- SSR-safe rendering
- Focus management built-in
- Declarative interaction
- Consistent behavior

Alert Dialog ensures that critical interactions are handled correctly, preventing user error and accessibility violations.
