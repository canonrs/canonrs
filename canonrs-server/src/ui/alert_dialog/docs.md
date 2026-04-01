Alert Dialog in CanonRS is a specialized overlay component designed for critical user confirmations, particularly destructive actions. Unlike generic dialogs, it enforces stricter accessibility and semantic rules to ensure that users are properly informed before taking irreversible actions.

The primary distinction lies in its role. Alert Dialog uses role="alertdialog", which signals assistive technologies that the content is urgent and requires immediate attention. This is combined with assertive aria-live behavior, ensuring that the message is announced without delay. Traditional dialogs often fail to implement this correctly, leading to accessibility gaps.

CanonRS encodes these requirements into the primitive layer. AlertDialogPrimitive ensures correct role assignment and behavior. Supporting primitives such as AlertDialogContentPrimitive, AlertDialogTitlePrimitive, and AlertDialogDescriptionPrimitive define structure and semantics. The UI layer simply forwards props and children.

Focus management is handled automatically. The component traps focus within the dialog, preventing interaction with the background. This is critical for accessibility and usability. Developers do not need to implement focus trapping manually.

SSR safety is maintained. The dialog structure, including roles and attributes, is rendered on the server. When the dialog is opened, the client does not need to reconcile differences. The system ensures that the DOM remains consistent.

The composition model enforces required parts. AlertDialogContent and AlertDialogTitle are mandatory, ensuring that every dialog has a clear message. Optional parts like overlay and description provide additional context but do not compromise structure.

A common failure in typical implementations is using a generic modal for destructive actions. This results in incorrect semantics and insufficient user warning. CanonRS prevents this by providing a dedicated component with enforced behavior.

Interaction is declarative. Trigger and close actions are defined through attributes such as data-rs-dialog-trigger and data-rs-dialog-close. This eliminates manual event handling and ensures consistent behavior.

Performance is optimized. The component uses minimal runtime logic, relying on declarative attributes and primitives. Hydration does not introduce inconsistencies.

In real-world scenarios such as account deletion or irreversible changes, Alert Dialog ensures that users are clearly informed and required to confirm their actions. It provides a reliable and accessible confirmation flow.

CanonRS Alert Dialog is not just a modal; it is a governed interaction pattern that guarantees correctness, accessibility, and consistency for critical user actions.
