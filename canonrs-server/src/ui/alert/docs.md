Alert in CanonRS is a semantic feedback component designed to eliminate accessibility errors and enforce correct messaging behavior. Traditional alert implementations rely on CSS classes and manual ARIA attributes, which often leads to incorrect roles or missing live region behavior. CanonRS solves this by encoding alert semantics into the AlertVariant type.

The Alert component is not just a container. It is a contract that defines how messages are announced to assistive technologies. Depending on the variant, the primitive automatically assigns the correct role and aria-live configuration. For example, destructive alerts may use assertive live regions, while informational alerts use polite announcements. This removes ambiguity and ensures that users receive feedback in the correct context.

The architecture separates semantics from presentation. AlertPrimitive handles role assignment, live region behavior, and accessibility mapping. AlertTitlePrimitive and AlertDescriptionPrimitive define structure and meaning. The UI layer simply forwards props, ensuring no duplication of logic.

SSR safety is inherent. Alerts are static messages, but their semantics must be present in the server-rendered HTML. CanonRS ensures that roles and attributes are embedded during SSR, so screen readers can interpret them immediately. There is no reliance on client-side scripts to fix or enhance accessibility.

The composable design allows optional parts such as AlertTitle and AlertDescription. This provides flexibility while maintaining structure. Developers can include only what is needed, but the system still guarantees correct semantics.

A common problem in typical systems is silent failure. Developers may use a div with a class like alert-error, but forget to add role="alert". This results in messages that are visually correct but inaccessible. CanonRS prevents this by making variant selection responsible for semantics. Incorrect configurations are not possible without explicitly bypassing the system.

Consistency is a key benefit. All alerts across an application behave the same way because they are governed by the same primitive logic. This reduces cognitive load and ensures uniform user experience.

Performance is unaffected because the component does not rely on heavy runtime logic. All behavior is encoded declaratively. Rendering is straightforward, and hydration does not introduce changes.

In real scenarios such as error handling, status updates, or informational messages, Alert provides a reliable mechanism for communicating with users. It ensures that critical messages are not only visible but also announced correctly.

CanonRS Alert represents a shift from styling-based feedback to contract-driven feedback. It guarantees that every alert is accessible, consistent, and semantically correct without requiring manual intervention.
