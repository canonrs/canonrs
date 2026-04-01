Alert is a feedback component that enforces accessibility semantics through variant-driven behavior. AlertVariant determines role and aria-live configuration. This ensures correct announcement behavior without manual ARIA assignment.

AlertPrimitive encapsulates semantic logic. It assigns roles such as alert and configures live regions. AlertTitlePrimitive and AlertDescriptionPrimitive define structured content. The UI layer acts as a pass-through, preserving primitive authority.

SSR compatibility is guaranteed. All semantic attributes are rendered on the server. No client-side correction is required. This ensures immediate accessibility for assistive technologies.

Variants control behavior. Destructive alerts use assertive announcements, while default alerts use polite ones. This mapping is enforced at compile time, preventing misuse.

The component is composable. Title and description are optional but structured. This allows flexible content while maintaining semantic integrity.

Common issues prevented:
- Missing role="alert"
- Incorrect aria-live usage
- Inconsistent alert behavior

The system eliminates reliance on developer discipline. Semantics are encoded, not optional.

Performance remains optimal. No runtime logic is required beyond rendering primitives. Hydration does not modify structure.

Use cases include error messages, warnings, and status notifications. The component ensures that messages are both visible and accessible.

Key properties:
- Variant-driven semantics
- SSR-safe rendering
- Accessible by default
- Consistent behavior
- No manual ARIA wiring

Alert in CanonRS is a semantic contract that guarantees correct feedback delivery across all environments.
