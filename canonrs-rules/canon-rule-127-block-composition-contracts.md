# Canon Rule #127: Block Composition Contracts

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2025-01-22

**Category:** component-architecture
**Tags:** blocks, composition, layout, structure
**Language:** EN

---

**Intro:**
Blocks without strict boundaries accumulate domain logic, routing, and global state, becoming unmanageable. This breaks separation between structure and behavior.

**Problem:**
blocks contain business logic state or routing instead of pure composition

**Solution:**
restrict blocks to semantic or interactive composition without domain responsibilities

**Signals:**
- api call in block
- routing in block
- global state usage

**Search Intent:**
how to structure block components in frontend architecture

**Keywords:**
block component architecture, semantic vs interactive blocks, frontend composition patterns, design system block layer

---

## Principle

**Blocks compose Components into semantic page sections without domain state or routing. Every Block MUST be classified as Semantic or Interactive.**

---

## The Problem

Without formal block types, developers create blocks that:

1. Manage global state (user session, cart) locally
2. Contain routing logic (navigation, URL handling)
3. Define page-level layout (header positioning, z-index)
4. Implement business logic instead of composing components
5. Blur boundary between Block and Component layers

**Real symptoms:**
- Blocks with API calls
- Blocks managing authentication state
- Blocks defining CSS Grid layouts for entire pages
- Blocks with complex validation logic
- Blocks that are untestable (too many responsibilities)

**Without this rule:**
- Block layer becomes dumping ground for "big components"
- No clear distinction from Component layer
- Code reviews struggle to classify
- Refactoring becomes guesswork

---

## Block Types

### Type 1: SemanticBlock

**Definition:** Structural page section with semantic HTML and passive composition.

**Use when:** Defining page regions like heroes, footers, feature grids, pricing tables.

**Examples:** `HeroBlock`, `FooterBlock`, `FeatureGridBlock`, `PricingTableBlock`, `TestimonialBlock`

#### CAN:
- Use semantic HTML: `<section>`, `<header>`, `<footer>`, `<aside>`, `<nav>`, `<article>`
- Compose UI components and StatefulComponents
- Accept content via props (strings, enums, simple data)
- Accept `ChildrenFn` for flexible slots
- Define block-level structure (grid of cards, list of features)
- Use data attributes for styling hooks (`data-hero`, `data-feature-grid`)

#### CANNOT:
- Have domain state (`RwSignal` for user data, cart, etc.)
- Call APIs or fetch data
- Use `use_context()` for application state
- Manage routing or navigation
- Define z-index or position (that's Layout layer)
- Implement business logic (validation, calculation)
- Know about authentication or authorization

#### Canonical Example:
```rust
// ✅ CORRECT: SemanticBlock
#[component]
pub fn HeroBlock(
    title: String,
    subtitle: String,
    image_url: String,
    cta: ChildrenFn,  // CTA button provided by caller
) -> impl IntoView {
    view! {
        <section data-hero="" class="hero-block">
            <div data-hero-content="">
                <h1>{title}</h1>
                <p>{subtitle}</p>
                <div data-hero-cta="">
                    {cta()}
                </div>
            </div>
            <div data-hero-image="">
                <img src={image_url} alt="Hero" />
            </div>
        </section>
    }
}
```

#### Forbidden Example 1: State in SemanticBlock
```rust
// ❌ WRONG: Domain state in Semantic Block
#[component]
pub fn HeroBlock() -> impl IntoView {
    let user = use_context::<User>().unwrap();  // ❌ State
    
    view! {
        <section data-hero="">
            <h1>"Welcome, "{user.name}</h1>  // ❌ Domain data
        </section>
    }
}
```

#### Forbidden Example 2: Routing in SemanticBlock
```rust
// ❌ WRONG: Navigation in Semantic Block
#[component]
pub fn HeroBlock() -> impl IntoView {
    let navigate = use_navigate();  // ❌ Routing
    
    view! {
        <section data-hero="">
            <Button on_click=move |_| navigate("/signup")>  // ❌
                "Get Started"
            </Button>
        </section>
    }
}
```

---

### Type 2: InteractiveBlock

**Definition:** Feature-complete section with local interactions but no global state.

**Use when:** Sections need interaction but don't manage application state (comments, search, filters).

**Examples:** `CommentSectionBlock`, `SearchBlock`, `FilterBarBlock`, `ChatWidgetBlock`, `FeedbackFormBlock`

#### CAN:
- Compose multiple StatefulComponents
- Manage block-level interactions (expand/collapse section)
- Emit block-level events: `on_filter_change`, `on_comment_submit`, `on_search`
  - Type: `Callback<BlockData>` where `BlockData` is block-specific
- Have local visual state (section expanded, filter panel open)
- Use `RwSignal` for **block UI state only** (not domain state)

#### CANNOT:
- Manage global application state (user, cart, session)
- Call authentication APIs
- Know about page structure (header height, sidebar width)
- Define page transitions or routing
- Implement complex business logic (that's Component layer)
- Fetch global data (user preferences, app config)

#### Canonical Example:
```rust
// ✅ CORRECT: InteractiveBlock
#[component]
pub fn CommentSectionBlock(
    post_id: String,
    on_comment_submit: Callback<Comment>,  // ✅ Block-level event
) -> impl IntoView {
    let (expanded, set_expanded) = signal(true);  // ✅ Block UI state
    
    view! {
        <section data-comments="" data-expanded={expanded.get()}>
            <header data-comments-header="">
                <h3>"Comments"</h3>
                <Button
                    variant=ButtonVariant::Ghost
                    on_click=move |_| set_expanded.update(|e| *e = !*e)
                >
                    {move || if expanded.get() { "Collapse" } else { "Expand" }}
                </Button>
            </header>
            
            <Show when=move || expanded.get()>
                <CommentList post_id=post_id.clone() />
                <CommentForm
                    post_id=post_id.clone()
                    on_submit=move |comment| {
                        on_comment_submit.run(comment);
                    }
                />
            </Show>
        </section>
    }
}
```

#### Forbidden Example 1: Global state in InteractiveBlock
```rust
// ❌ WRONG: Global state in Block
#[component]
pub fn SearchBlock() -> impl IntoView {
    let (search_history, set_search_history) = signal(vec![]);  // ❌ Global data
    
    provide_context(SearchContext {  // ❌ Global context
        history: search_history,
    });
    
    view! { /* ... */ }
}
```

#### Forbidden Example 2: Business logic in InteractiveBlock
```rust
// ❌ WRONG: Validation logic in Block
#[component]
pub fn FilterBarBlock() -> impl IntoView {
    let validate_price = |min: f64, max: f64| -> Result<(), String> {  // ❌
        if min > max {
            return Err("Min must be less than max".to_string());
        }
        // Complex validation...
        Ok(())
    };
    
    view! { /* ... */ }
}
```

---

## Universal Block Contracts

**ALL Blocks MUST follow these contracts:**

### 1. No Global State Management
**Never use `provide_context()` for application-wide state.**
```rust
// ✅ CORRECT: Emit events, let parent handle state
on_submit: Callback<FormData>

// ❌ WRONG: Provide global context
provide_context(UserContext { user });
```

### 2. No Routing
**Use callbacks for navigation — never call `use_navigate()`.**
```rust
// ✅ CORRECT: Callback for navigation
on_cta_click: Callback<()>

// ❌ WRONG: Direct navigation
let navigate = use_navigate();
navigate("/signup");
```

### 3. Semantic HTML Required
**Use appropriate semantic elements for structure.**
```rust
// ✅ CORRECT
<section data-hero="">
<footer data-footer="">
<aside data-sidebar="">

// ❌ WRONG
<div data-hero="">  // Should be <section>
<div data-footer="">  // Should be <footer>
```

### 4. No Layout Concerns
**Don't define page-level positioning or z-index.**
```rust
// ✅ CORRECT: Block-level structure
<section data-comments="" class="comment-section">

// ❌ WRONG: Page layout
<section style="position: fixed; top: 0; z-index: 9999;">
```

### 5. Composition Over Implementation
**Compose Components — don't reimplement their logic.**
```rust
// ✅ CORRECT: Compose existing components
<CommentList post_id=id />
<CommentForm on_submit=handler />

// ❌ WRONG: Reimplement form logic
let (text, set_text) = signal(String::new());
// ... validation, API call, etc.
```

---

## Rationale

### Why Block Layer Exists

1. **Semantic structure** — groups related components into meaningful sections
2. **Reusability** — blocks can be reused across pages
3. **Composition boundary** — clear separation from Component and Layout layers
4. **Content modeling** — maps to CMS content blocks
5. **Testing** — blocks testable independently of page context

### What This Rule Protects

- **Layer boundaries** — blocks don't become "mega components"
- **State locality** — prevents global state leaking into blocks
- **Semantic HTML** — enforces proper document structure
- **Composition clarity** — blocks compose, don't implement

---

## Enforcement

### Static Analysis
```rust
// Check 1: No global state
if block.contains("provide_context") {
    emit_error!("Canon Rule #127: Blocks cannot provide global context");
}

// Check 2: No routing
if block.contains("use_navigate") || block.contains("use_location") {
    emit_error!("Canon Rule #127: Blocks cannot use routing");
}

// Check 3: Semantic HTML
if block.contains("<div data-hero=") {
    emit_warning!("Canon Rule #127: Use <section> instead of <div> for semantic blocks");
}
```

### CI Checks
```bash
# No routing in blocks
rg 'use_navigate|use_location' packages-rust/rs-design/src/blocks/ && exit 1

# No provide_context in blocks
rg 'provide_context' packages-rust/rs-design/src/blocks/ && exit 1

# No API calls in blocks
rg 'api::|fetch_|async fn' packages-rust/rs-design/src/blocks/ && exit 1
```

---

## Exceptions

### Exception 1: Block UI State (Allowed)
```rust
// ✅ ALLOWED: Block-level UI state
let (section_expanded, set_section_expanded) = signal(true);
let (filter_open, set_filter_open) = signal(false);
```

**When allowed:**
- State controls block visibility/expansion only
- State does not represent domain data
- State is not shared across blocks

### Exception 2: Data Props (Allowed)
```rust
// ✅ ALLOWED: Accepting data props
#[component]
pub fn PricingTableBlock(
    plans: Vec<PricingPlan>,  // Data passed in, not fetched
    on_select: Callback<String>,
) -> impl IntoView { /* ... */ }
```

**When allowed:**
- Data passed as props, not fetched internally
- Block displays data, doesn't manage it

**No other exceptions exist.**

---

## Quick Reference

| Type | State | Composition | Events | Example |
|------|-------|-------------|--------|---------|
| **Semantic** | ❌ None | ✅ Passive | ❌ None | `HeroBlock`, `FooterBlock` |
| **Interactive** | ✅ UI only | ✅ Active | ✅ Block-level | `CommentSection`, `SearchBlock` |

---

## Related Rules

- **Canon Rule #124:** Primitive Contract Types
- **Canon Rule #125:** UI Component Contracts
- **Canon Rule #126:** Component Domain Contracts (previous rule)
- **Canon Rule #128:** Layout Shell and Zone Contracts (next rule)

---

## Version History

- **1.0.0** — Initial canonical version (2025-01-22)