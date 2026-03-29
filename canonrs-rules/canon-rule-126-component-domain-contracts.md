# Canon Rule #126: Component Domain Contracts

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** components
**Version:** 1.0.0
**Date:** 2025-01-22

---

## Principle

**Components own domain state, implement business logic, and orchestrate workflows. Every Component MUST be classified as Stateful or Orchestrator.**

---

## The Problem

Without formal component layer types, developers create components that:

1. Mix domain logic with visual concerns (validation + tabs state in one component)
2. Use Primitives directly instead of UI layer
3. Manage global state locally (duplicating cart, user session)
4. Emit DOM events instead of domain events
5. Know about routing/navigation instead of using callbacks

**Real symptoms:**
- Business logic scattered across UI components
- Validation rules duplicated in multiple places
- API calls in UI layer components
- State management chaos (local vs global unclear)
- Components tightly coupled to routing

**Without this rule:**
- Domain logic leaks into UI layer
- Components become untestable (mixed concerns)
- State boundaries blur
- Refactoring becomes impossible

---

## Component Types

### Type 1: StatefulComponent

**Definition:** Owns domain state and implements business logic for a single feature.

**Use when:** Managing user input, validation, API calls, or feature-specific state.

**Examples:** `LoginForm`, `ProductSearch`, `CommentForm`, `UserProfile`, `ShoppingCartSummary`

#### CAN:
- Use `RwSignal` for domain state (form values, loading, errors)
- Call APIs and fetch data
- Use `use_context()` to access global state
- Validate data according to business rules
- Emit domain events: `on_login`, `on_submit`, `on_save`, `on_delete`
  - Type: `Callback<DomainType>` (e.g., `Callback<User>`, `Callback<Order>`)
- Use UI components from Layer 2 (Button, Input, Select)
- Accept domain props: `user: User`, `product_id: String`
- Manage local error state and success messages

#### CANNOT:
- Use Primitives directly (MUST go through UI layer)
- Define CSS or data attributes (use UI components)
- Know about routing (use callbacks instead: `on_success: Callback<()>`)
- Manage global state locally (cart, user session — use context)
- Coordinate multiple features (that's OrchestratorComponent)

#### Canonical Example:
```rust
// ✅ CORRECT: StatefulComponent
#[component]
pub fn LoginForm(
    on_login: Callback<User>,  // ✅ Domain callback
    on_cancel: Callback<()>,
) -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    
    let handle_submit = move |_| {
        set_loading.set(true);
        set_error.set(None);
        
        // Validation (business logic)
        if email.get().is_empty() {
            set_error.set(Some("Email required".to_string()));
            set_loading.set(false);
            return;
        }
        
        // API call (domain logic)
        spawn_local(async move {
            match api::login(email.get(), password.get()).await {
                Ok(user) => on_login.run(user),  // ✅ Emit domain event
                Err(e) => set_error.set(Some(e.to_string())),
            }
            set_loading.set(false);
        });
    };
    
    view! {
        <form on:submit=handle_submit>
            <Input
                value=email.get()
                on_input=move |v| set_email.set(v)
                placeholder="Email"
            />
            <Input
                value=password.get()
                on_input=move |v| set_password.set(v)
                placeholder="Password"
                input_type="password"
            />
            
            <Show when=move || error.get().is_some()>
                <div class="error">{error.get()}</div>
            </Show>
            
            <Button on_click=handle_submit loading=loading.get()>
                "Login"
            </Button>
            <Button variant=ButtonVariant::Ghost on_click=move |_| on_cancel.run(())>
                "Cancel"
            </Button>
        </form>
    }
}
```

#### Forbidden Example 1: Using Primitive directly
```rust
// ❌ WRONG: Primitive in Component layer
#[component]
pub fn LoginForm() -> impl IntoView {
    view! {
        <form>
            <InputPrimitive />  // ❌ Should use Input (UI layer)
            <ButtonPrimitive>"Login"</ButtonPrimitive>  // ❌ Should use Button
        </form>
    }
}
```

#### Forbidden Example 2: Routing in Component
```rust
// ❌ WRONG: Navigation in Component
#[component]
pub fn LoginForm() -> impl IntoView {
    let navigate = use_navigate();  // ❌ Routing knowledge
    
    let handle_login = move |user: User| {
        navigate("/dashboard");  // ❌ Component knows routes
    };
    
    view! { /* ... */ }
}
```

#### Forbidden Example 3: Managing global state locally
```rust
// ❌ WRONG: Cart state in local component
#[component]
pub fn ProductCard() -> impl IntoView {
    let (cart, set_cart) = signal(vec![]);  // ❌ Cart is global, not local
    
    let add_to_cart = move |product: Product| {
        set_cart.update(|c| c.push(product));
    };
    
    view! { /* ... */ }
}
```

---

### Type 2: OrchestratorComponent

**Definition:** Coordinates multiple StatefulComponents or manages complex workflows.

**Use when:** Multi-step processes, dashboards, or cross-feature coordination needed.

**Examples:** `CheckoutWizard`, `MultiStepForm`, `Dashboard`, `OnboardingFlow`, `ReportBuilder`

#### CAN:
- Manage workflow state (current step, progress, completion)
- Coordinate multiple StatefulComponents
- Use `provide_context()` to share state with children
- Handle cross-component communication
- Emit workflow events: `on_complete`, `on_step_change`, `on_cancel`
- Use `RwSignal` for orchestration state (step index, validation status)
- Use UI components and StatefulComponents

#### CANNOT:
- Render Primitives directly
- Define global layout (that's Layout layer, Rule #128)
- Manage application-wide state (user session, theme — use app-level context)
- Know about routing (use callbacks)

#### Canonical Example:
```rust
// ✅ CORRECT: OrchestratorComponent
#[component]
pub fn CheckoutWizard(
    on_complete: Callback<Order>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    let (step, set_step) = signal(1);  // ✅ Workflow state
    let (cart_validated, set_cart_validated) = signal(false);
    let (shipping_info, set_shipping_info) = signal(None::<ShippingInfo>);
    let (payment_info, set_payment_info) = signal(None::<PaymentInfo>);
    
    // Provide context for child components
    provide_context(CheckoutContext {
        step: step.read_only(),
        set_step,
    });
    
    let handle_cart_review = move |valid: bool| {
        set_cart_validated.set(valid);
        if valid {
            set_step.set(2);
        }
    };
    
    let handle_shipping = move |info: ShippingInfo| {
        set_shipping_info.set(Some(info));
        set_step.set(3);
    };
    
    let handle_payment = move |info: PaymentInfo| {
        set_payment_info.set(Some(info));
        
        // Create order (orchestration logic)
        let order = Order {
            shipping: shipping_info.get().unwrap(),
            payment: info,
        };
        
        on_complete.run(order);  // ✅ Emit workflow completion
    };
    
    view! {
        <div data-wizard="">
            <ProgressBar current=step.get() total=3 />
            
            <Show when=move || step.get() == 1>
                <CartReview on_continue=handle_cart_review />
            </Show>
            
            <Show when=move || step.get() == 2>
                <ShippingForm on_submit=handle_shipping />
            </Show>
            
            <Show when=move || step.get() == 3>
                <PaymentForm on_submit=handle_payment />
            </Show>
            
            <Button
                variant=ButtonVariant::Ghost
                on_click=move |_| on_cancel.run(())
            >
                "Cancel"
            </Button>
        </div>
    }
}
```

#### Forbidden Example 1: Layout concerns in Orchestrator
```rust
// ❌ WRONG: Layout in Orchestrator
#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div style="display: grid; grid-template-columns: 200px 1fr;">  // ❌ Layout
            <Sidebar />
            <MainContent />
        </div>
    }
}
```

#### Forbidden Example 2: Global state management
```rust
// ❌ WRONG: Application state in Orchestrator
#[component]
pub fn Dashboard() -> impl IntoView {
    let (user, set_user) = signal(None);  // ❌ User is app-level, not orchestrator
    let (theme, set_theme) = signal(Theme::Light);  // ❌ Theme is app-level
    
    view! { /* ... */ }
}
```

---

## Universal Component Contracts

**ALL Components MUST follow these contracts:**

### 1. Never Use Primitives Directly
**MUST use UI layer components — never Primitives.**
```rust
// ✅ CORRECT
<Button on_click=save>"Save"</Button>
<Input value=name on_input=set_name />

// ❌ WRONG
<ButtonPrimitive on:click=save>"Save"</ButtonPrimitive>
<InputPrimitive value=name />
```

### 2. Emit Domain Callbacks
**Use `Callback<DomainType>` — NEVER `Callback<MouseEvent>`.**
```rust
// ✅ CORRECT
on_submit: Callback<FormData>
on_login: Callback<User>
on_save: Callback<Product>
on_delete: Callback<String>  // ID

// ❌ WRONG
on_click: Callback<MouseEvent>
on_change: Callback<Event>
```

### 3. No Routing Knowledge
**Use callbacks for navigation — don't call `use_navigate()`.**
```rust
// ✅ CORRECT
#[component]
pub fn LoginForm(
    on_success: Callback<User>,  // Parent handles navigation
) -> impl IntoView { /* ... */ }

// ❌ WRONG
#[component]
pub fn LoginForm() -> impl IntoView {
    let navigate = use_navigate();
    let handle_login = move |user| navigate("/dashboard");  // ❌
    // ...
}
```

### 4. Domain State vs Visual State
**Distinguish clearly: domain state (data, validation) vs visual state (loading, errors).**
```rust
// ✅ CORRECT
let (email, set_email) = signal(String::new());  // Domain state
let (loading, set_loading) = signal(false);  // Visual state
let (error, set_error) = signal(None::<String>);  // Visual state

// ❌ WRONG (mixing concerns)
let (tab_index, set_tab_index) = signal(0);  // Visual state in domain component
```

### 5. Global vs Local State
**Use context for global state — don't duplicate in components.**
```rust
// ✅ CORRECT: Use global context
let cart = use_context::<CartContext>().unwrap();

// ❌ WRONG: Local cart state
let (cart, set_cart) = signal(CartState::default());
```

---

## Rationale

### Why Component Layer Exists

1. **Business logic centralization** — one place for domain rules
2. **Testability** — components can be tested with mock callbacks
3. **State ownership** — clear boundaries for state management
4. **Separation from UI** — domain logic independent of visual presentation
5. **Reusability** — components work with any UI theme/style

### What This Rule Protects

- **Domain integrity** — business logic stays pure
- **Testing simplicity** — no DOM dependencies in tests
- **Architectural clarity** — layer boundaries enforced
- **Refactoring safety** — domain logic changes don't break UI

---

## Enforcement

### Static Analysis
```rust
// Check 1: No Primitives in Component layer
for import in component.imports {
    if import.path.contains("primitives/") {
        emit_error!("Canon Rule #126: Components cannot use Primitives directly");
    }
}

// Check 2: No routing
if component.contains("use_navigate") || component.contains("use_location") {
    emit_error!("Canon Rule #126: Components cannot use routing directly");
}

// Check 3: Domain callbacks only
for prop in component.props {
    if prop.type.contains("Callback<MouseEvent>") {
        emit_error!("Canon Rule #126: Use domain callbacks, not DOM events");
    }
}
```

### CI Checks
```bash
# No Primitive usage in components
rg 'ButtonPrimitive|InputPrimitive|SelectPrimitive' products/*/src/components/ && exit 1

# No routing in components
rg 'use_navigate|use_location' products/*/src/components/ && exit 1

# No layout in components
rg 'display: grid|display: flex' products/*/src/components/ && exit 1
```

---

## Exceptions

**No exceptions. This rule is absolute.**

If a component needs to use Primitives or routing, it's misclassified or the architecture needs reconsideration.

---

## Quick Reference

| Type | State | API Calls | Coordination | Example |
|------|-------|-----------|--------------|---------|
| **Stateful** | ✅ Domain + Visual | ✅ Yes | ❌ No | `LoginForm`, `ProductSearch` |
| **Orchestrator** | ✅ Workflow only | ⚠️ Rare | ✅ Yes | `CheckoutWizard`, `Dashboard` |

---

## Related Rules

- **Canon Rule #119:** No `#[prop(optional, into)]` in UI layer
- **Canon Rule #120:** DOM Events vs Semantic Callbacks
- **Canon Rule #124:** Primitive Contract Types
- **Canon Rule #125:** UI Component Contracts (previous rule)
- **Canon Rule #127:** Block Composition Contracts (next rule)

---

## Version History

- **1.0.0** — Initial canonical version (2025-01-22)
