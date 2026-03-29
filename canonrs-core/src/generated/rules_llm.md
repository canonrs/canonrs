# CanonRS — Canon Rules

> AUTO-GENERATED — do not edit manually

---

## CR-001 — Component Types

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components, architecture
- **Version:** 1.0.0

### Anti-patterns

- | **Type 1 - Pure**        | ❌          | ❌      | ❌           | Label, Badge    |
- | **Type 2 - Stateful**    | ✅ RwSignal | ❌      | ❌           | Toggle, Tabs    |

---

## CR-002 — Ownership Rules

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** leptos, state, primitives
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ WRONG
- ### ❌ WRONG
- ### ❌ WRONG

---

## CR-003 — Lists and Iteration

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** leptos, state, ui
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ NEVER DO THIS
- ### ❌ ALSO WRONG
- // ❌ WRONG

---

## CR-004 — Anti-Hydration Rules

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** hydration, ssr
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ Violation #1: Conditional with Async State
- ### ❌ Violation #2: <For> in Dropdown
- ### ❌ Violation #3: RwSignal Props

---

## CR-005 — SSR Effects and Browser API Safety

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** ssr, behavior
- **Version:** 1.0.0

### Anti-patterns

- // ❌ FORBIDDEN without guard
- // ❌ FORBIDDEN in component body
- // ❌ FORBIDDEN without guard

---

## CR-006 — Visual State Declaration

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components, css
- **Version:** 1.0.0

### Anti-patterns

- ## ❌ WRONG Patterns
- // ❌ FORBIDDEN - Concrete colors in UI
- // ❌ FORBIDDEN - JS-controlled styling

---

## CR-007 — Theme and Token Governance

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** design-system, tokens, theming
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ What developers try (breaks at scale)
- | **rs-design** | ✅ YES | ❌ NO | ❌ NO |
- | **rs-tailwind** | ❌ NO | ❌ NO | ✅ YES |

---

## CR-008 — Overlay Islands (Client-Only Architecture)

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** hydration, csr, ssr
- **Version:** 1.0.0

### Problem

**Overlays dinâmicos** (com listas reativas) + **SSR** = **Hydration Hell**   1. SSR gera HTML estático 2. Client tenta hidratar 3. Lista dinâmica altera DOM 4. Leptos detecta mismatch 5. **Panic**   | Approach | Why It Failed | |----------|--------------| | `cfg!(feature = "ssr")` | Compile-time, n

### Anti-patterns

- - ❌ Componente não visível em SSR
- ### ❌ Errado: Lista dinâmica sem Island
- ### ❌ Errado: cfg!() para runtime

---

## CR-009 — Clipboard and Browser APIs

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** interactive, behavior
- **Version:** 1.0.0

### Problem

`navigator.clipboard.writeText()` fails in Leptos callbacks because:  1. **Requires document focus** - async callbacks lose focus 2. **HTTPS only** - dev environments may not have it 3. **User gesture in stack** - callbacks break the chain 4. **Promise handling** - needs `spawn_local` (SSR unsafe)  

### Anti-patterns

- | Modern clipboard API | ❌ NO | Fails in callbacks |
- // ❌ FAILS in callbacks

---

## CR-010 — Modal Reactive State Management

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components, state
- **Version:** 1.0.0

### Problem

Passing `String` props to modals = **values never update**: ```rust // ❌ WRONG fn EditDialog( open: RwSignal<bool>, name: String,  // ← Static! Won't react ) -> impl IntoView { view! { <Input value=name />  // ← Never updates when parent changes } } ```  **Why it fails:** - Props are **captured at c

### Anti-patterns

- // ❌ WRONG
- ### ❌ DON'T

---

## CR-011 — Multi-Callback Ownership

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** interactive, behavior
- **Version:** 1.0.0

### Problem

**Multiple callbacks = moved value errors:** ```rust // ❌ WRONG DataTableColumn::actions(|t: &TokenRow| { let name = t.name.clone();  vec![ RowAction { on_click: Callback::new(move |_| { copy_to_clipboard(&name);  // ← name moved here }), }, RowAction { on_click: Callback::new(move |_| { edit_signal

### Anti-patterns

- // ❌ WRONG
- ### ❌ DON'T

---

## CR-012 — canon-rule-12-select-vs-combobox

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** components, ui
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ É PROIBIDO

---

## CR-013 — canon-rule-13-specialization-vs-substitution

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components, architecture
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ É PROIBIDO
- // ❌ ERRADO - MaskedInput como default
- // ❌ PROIBIDO

---

## CR-014 — canon-rule-14-datatable-vs-virtualtable

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** state, ui, components
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ É PROIBIDO
- | **SSR**         | ✅ Total            | ❌ Client-only      |
- ### ❌ DataTable virtualizado

---

## CR-015 — canon-rule-15-pagination-vs-virtualization

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** layout, ui
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ É PROIBIDO
- | **SEO**          | ✅ URLs indexáveis   | ❌ Client-only       |
- | **Deep links**   | ✅ `/page/5`         | ❌ Não suporta       |

---

## CR-016 — canon-rule-16-client-vs-server-filtering

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** state, architecture
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ É PROIBIDO
- | **SEO**          | ❌ Client-only           | ✅ URLs indexáveis       |
- ### ❌ Client-side com dataset grande

---

## CR-017 — canon-rule-17-human-vs-machine-scale

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components, ui
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ É PROIBIDO
- ### ❌ Escalar componente human-scale além do limite
- ### ❌ Usar componente machine-scale para dados human-scale

---

## CR-018 — Client vs Server

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** state, architecture
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ É PROIBIDO

---

## CR-019 — canon-rule-19-streaming-vs-snapshot

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** state, ssr
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ É PROIBIDO

---

## CR-020 — canon-rule-20-realtime-vs-eventual

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** state, ssr
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ É PROIBIDO
- setInterval(() => fetch(), 100); // ❌

---

## CR-021 — Canonical Color Tokens vs Semantic Intents

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** design-system, tokens
- **Version:** 1.0.0

### Anti-patterns

- `success` / `success-foreground`
- `warning` / `warning-foreground`
- `info` / `info-foreground`

---

## CR-022 — Tailwind v4 + Rust Integration

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** build, css
- **Version:** 1.0.0

### Problem

```rust // Tailwind v4 JIT CANNOT generate these from .rs files view! { <button class="bg-[hsl(var(--color-primary))]">  // NOT COMPILED <div class="h-[var(--size-control-md)]">         // NOT COMPILED <span class="text-[hsl(var(--color-accent))]">   // NOT COMPILED } ```  **Why:** Tailwind's conten

### Anti-patterns

- ### ❌ What DOESN'T Work
- // ❌ FORBIDDEN
- // ❌ Still doesn't work

---

## CR-023 — State Tokens (Hover, Focus, Disabled, Pressed)

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** tokens, state, css
- **Version:** 1.0.0

### Anti-patterns

- #### ❌ INCORRECT: Color Variants
- hover:bg-primary-hover \      // ❌ Requires theme definition
- hover:border-primary-hover";  // ❌ Not portable

---

## CR-024 — Density & Size Scaling

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** tokens, design-system
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ Hardcoded Pixel Values
- ### ❌ Inconsistent Scales
- ### ❌ Density-Unaware Components

---

## CR-025 — Theme Presets Contract

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** tokens, design-system, theming
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ Typography
- ### ❌ Spacing
- ### ❌ Border Radius

---

## CR-026 — Elevation & Shadow System

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** motion, tokens, design-system
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ Theme-Specific Shadows
- ### ❌ Hardcoded Shadow Values
- ### ❌ Arbitrary Z-Index

---

## CR-027 — Motion & Timing Tokens

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** motion, tokens
- **Version:** 1.0.0

### Anti-patterns

- // ❌ Triggers layout recalculation
- ### ❌ Hardcoded Timing
- ### ❌ Transition All

---

## CR-028 — Responsive Grid Contract

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** layout, css
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ Custom Breakpoints
- ### ❌ Desktop-First Approach
- ### ❌ Fixed Pixel Widths

---

## CR-029 — Typography Contract

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** tokens, design-system
- **Version:** 1.0.0

### Anti-patterns

- Web fonts (Google Fonts, Adobe Fonts)
- Custom font files (@font-face)
- Icon fonts (use SVG icons instead)

---

## CR-030 — Iconography System

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** tokens, design-system
- **Version:** 1.0.0

### Anti-patterns

- Font Awesome (icon font)
- Material Icons (inconsistent licensing)
- Custom icon fonts

---

## CR-031 — Accessibility Contract (ARIA + Roles)

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** accessibility, ui
- **Version:** 1.0.0

---

## CR-032 — Theme Persistence Contract

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** state, theming
- **Version:** 1.0.0

---

## CR-033 — Density & Accessibility Mapping

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** accessibility, tokens
- **Version:** 1.0.0

---

## CR-034 — Theme & Density Enforcement (Lint Rules)

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** governance, theming
- **Version:** 1.0.0

---

## CR-035 — Token Usage Validation

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** design-system, tokens, governance
- **Version:** 1.0.0

---

## CR-036 — Component Compliance Levels

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** components, governance
- **Version:** 1.0.0

---

## CR-037 — Provider Taxonomy & Boundaries

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** providers, architecture
- **Version:** 1.0.0

---

## CR-038 — Theme Engine Contract

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** tokens, theming
- **Version:** 1.0.0

---

## CR-039 — Settings UI Compliance

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** state, ui, theming
- **Version:** 1.0.0

---

## CR-040 — Legacy Exception & Annotation Contract

- **Severity:** LOW
- **Status:** ENFORCED
- **Scope:** governance
- **Version:** 1.0.0

---

## CR-041 — Leptos Resource Consumption Contract

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** leptos, state
- **Version:** 1.0.0

### Problem

```rust pub fn MyComponent() -> impl IntoView { let data = Resource::new(|| (), |_| async { fetch_data().await });  view! { <Suspense fallback=|| view! { <div>"Loading..."</div> }> {move || Suspend::new(async move { // ❌ FATAL ERROR: Resource is not a Future match data.await { Ok(result) => view! { 

### Anti-patterns

- ### ❌ WRONG: Treating Resource as Future
- // ❌ FATAL ERROR: Resource is not a Future
- | `.with_untracked()` | Read without tracking | ✅ | ✅ | ❌ |

---

## CR-042 — UI Data Surfaces

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** state, ui, components
- **Version:** 1.0.0

### Problem

``` ui/ ├── virtual_table/     // For client-side Vec<T> ├── streaming_table/   // For server streaming ├── paginated_table/   // For page/limit API └── infinite_table/    // For cursor-based ```  **Why this fails:** - 4 components doing the same job (render rows) - Duplicate viewport logic - Duplic

### Anti-patterns

- ### ❌ WRONG: Strategy as Component
- // ❌ WRONG: ui/tokens_table.rs
- // ❌ WRONG: components/workflow.rs

---

## CR-043 — Domain Components and Commands

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components, architecture
- **Version:** 1.0.0

### Problem

```rust pub fn Workflow() -> impl IntoView { let steps = Resource::new(|| (), |_| fetch_steps());  view! { <Transition> {move || steps.get().map(|steps| view! { {steps.into_iter().map(|step| view! { <div> <span>{step.label}</span> <button on:click=move |_| { // ❌ Command inside domain component tran

### Anti-patterns

- ### ❌ WRONG: Command Inside Domain Component
- // ❌ Command inside domain component
- - ❌ No `spawn_local`

---

## CR-044 — Orchestrators

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** state, architecture
- **Version:** 1.0.0

### Problem

```rust // Domain component knows about commands pub fn WorkflowView() -> impl IntoView { let steps = Resource::new(|| (), |_| fetch_steps());  // ❌ Domain component emitting commands let on_click = move |_| { spawn_local(async { transition_step().await }); };  view! { <div> {/* rendering */} <butto

### Anti-patterns

- ### ❌ WRONG: Tight Coupling
- // ❌ Domain component emitting commands
- <button on:click=on_click>"Complete"</button>  // ❌

---

## CR-045 — Demo Components & Ephemeral State

- **Severity:** LOW
- **Status:** ENFORCED
- **Scope:** components, state
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ DON'T Use Demo Patterns When:
- // ❌ WRONG for a demo
- | Will this be deployed? | ❌ | ✅ |

---

## CR-046 — Command Palette & Intent Surfaces

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** interactive, ui
- **Version:** 1.0.0

### Problem

```rust // Every action becomes a button view! { <div> <button on:click=complete>"Complete Step"</button> <button on:click=fail>"Fail Step"</button> <button on:click=reset>"Reset Step"</button> <button on:click=start>"Start Step"</button> <button on:click=unblock>"Unblock Step"</button> <button on:c

### Anti-patterns

- ### ❌ WRONG: Button Proliferation
- // ❌ WRONG: Renders different trees in SSR vs client
- | Search | Find items | ❌ Different (find vs execute) |

---

## CR-047 — Tree, Context & Selection

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components, state
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ WRONG: Flat List Navigation
- **Execute Domain Actions**
- // ❌ WRONG

---

## CR-048 — Context Provider Pattern

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** providers, architecture
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ WITHOUT Context Provider
- - ❌ Every component needs selection prop
- - ❌ Adding new consumer = update all call sites

---

## CR-049 — Drag & Drop as Intent (Not Action)

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** interactive, behavior
- **Version:** 1.0.0

### Problem

```rust // DragHandle executes business logic directly fn on_drop(item_id: String, target_id: String) { // WRONG: Component knows about database database::move_item(item_id, target_id).await; // WRONG: Component knows about workflow workflow::update_step_order(item_id).await; } ```  **Why this is wr

### Solution

```rust // 1. Design System: Emits events (NO business logic) use rs_design::ui::drag_drop::{DragDropProvider, DragHandle, DropEvent};  pub fn MyApp() -> impl IntoView { let on_drop = Callback::new(|event: DropEvent| { // 2. Application Layer: Converts event → command let command = MoveItemCommand {

### Anti-patterns

- ### ❌ Wrong Pattern (Coupled)
- ### ❌ Wrong (Coupled)

---

## CR-050 — Provider Singleton & Runtime Separation

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** providers, architecture
- **Version:** 1.0.0

### Problem

```rust // App layer <App> <DragDropProvider>  // ✅ Provider 1 <Dashboard> <DragDropProvider>  // 🚫 Provider 2 (DUPLICATE!) ... </DragDropProvider> </Dashboard> </DragDropProvider> </App> ```  **Why this breaks:** - Two separate `DragContext` instances created - `DragHandle` uses Context 1 - `DropZo

### Solution

```rust // App layer ONLY pub fn App() -> impl IntoView { view! { <Router> <DragDropProvider>  // ✅ ONE provider at app root <DragDropCallbacksProvider>  // ✅ Nested correctly <Dashboard />  // ✅ NO provider here <WorkflowEditor />  // ✅ NO provider here </DragDropCallbacksProvider> </DragDropProvid

### Anti-patterns

- ### ❌ Wrong Pattern (Duplicated Provider)
- ### ❌ Wrong Pattern (Provider in Design System)
- | **Design System (rs-design)** | ❌ NO | ✅ YES | UI primitives, emit events |

---

## CR-051 — Callbacks as Commands

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** interactive, behavior
- **Version:** 1.0.0

### Problem

```rust // Dashboard callback executes mutation directly let on_widget_drag = Callback::new(move |event: WidgetDragEvent| { // 🚫 WRONG: Direct state mutation set_widgets.update(|ws| { if let Some(w) = ws.iter_mut().find(|w| w.id == event.widget_id) { w.position = event.to_position; } });  // 🚫 WRONG

### Solution

```rust // 1. Define command struct MoveWidgetCommand { widget_id: String, from_position: WidgetPosition, to_position: WidgetPosition, }  impl Command for MoveWidgetCommand { fn execute(&self, state: &mut AppState) { if let Some(w) = state.widgets.iter_mut().find(|w| w.id == self.widget_id) { w.posi

### Anti-patterns

- ### ❌ Wrong Pattern (Direct Mutation)
- ## Forbidden Patterns ❌
- ### ❌ Callback Testing (Hard)

---

## CR-052 — Command History as First-Class Runtime

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** interactive, state
- **Version:** 1.0.0

### Problem

```rust // Each component creates its own history pub fn Dashboard() -> impl IntoView { let command_history = CommandHistory::new(); // 🚫 Local  view! { <div on:click=move |_| { command_history.execute(SomeCommand {}); }> } } ```  **Why this breaks:** - No global undo/redo - Cannot undo across compo

### Solution

```rust // App layer (ONE provider at root) pub fn App() -> impl IntoView { view! { <Router> <DragDropProvider> <DragDropCallbacksProvider> <CommandHistoryProvider>  // ✅ Global <Dashboard /> <FormBuilder /> <DataGrid /> </CommandHistoryProvider> </DragDropCallbacksProvider> </DragDropProvider> </Ro

### Anti-patterns

- ### ❌ Wrong Pattern (Local Command History)
- ## Forbidden Patterns ❌

---

## CR-053 — Client-Only Runtime Islands

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** hydration, csr
- **Version:** 1.0.0

### Problem

```rust pub fn MyComponent() -> impl IntoView { // 🚫 PANIC in SSR! let window = web_sys::window().unwrap(); let document = window.document().unwrap();  view! { <div on:click=move |_| { // 🚫 PANIC in SSR! window.alert_with_message("Clicked"); }> } } ```  **Why this breaks:** - SSR has no `window` or 

### Solution

```rust pub fn MyComponent() -> impl IntoView { { use web_sys::window;  let on_click = move |_| { if let Some(w) = window() { let _ = w.alert_with_message("Clicked"); } };  view! { <div on:click=on_click>"Click me"</div> } }  { view! { <div>"Click me"</div> } } } ```  ---

### Anti-patterns

- ### ❌ Wrong Pattern (Unguarded Client Code)
- ### ❌ Wrong (Hydration Mismatch)
- ## Anti-Patterns ❌

---

## CR-054 — canon-rule-54-render-must-be-total

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** state, ui
- **Version:** 1.0.0

### Problem

In Leptos/Tachys, **attribute closures are evaluated during rendering**, not interaction.  This includes closures used in:  - `disabled=` - `class=` - `style=` - `hidden=` - any reactive attribute or signal-based expression  If these closures call:  - `expect` - `unwrap` - `panic!` - runtime-only as

### Anti-patterns

- ## ❌ Forbidden Pattern (Render Panic)

---

## CR-055 — Canonical CSS Entry Points

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** build, css
- **Version:** 1.0.0

### Anti-patterns

- /* ❌ NEVER ALLOWED */

---

## CR-056 — Monorepo CSS Build Pipeline

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** build, css, workspace
- **Version:** 1.0.0

---

## CR-057 — PostCSS Canon Configuration

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** build, css
- **Version:** 1.0.0

### Anti-patterns

- /* ❌ NEVER */

---

## CR-058 — Leptos Assets Dev Constraint

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** leptos, build
- **Version:** 1.0.0

### Anti-patterns

- public/pkg/*.css  ❌ (1 byte placeholder)
- - ❌ Symlinks
- - ❌ Runtime copying

---

## CR-059 — CSS Cascade Ownership

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** css, architecture
- **Version:** 1.0.0

---

## CR-060 — CSS Artifacts Are Immutable

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** build, css
- **Version:** 1.0.0

---

## CR-061 — No Relative CSS Imports

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** build, css
- **Version:** 1.0.0

---

## CR-062 — Single Source of Truth for Design Tokens

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** design-system, tokens, architecture
- **Version:** 1.0.0

### Problem

``` ├── crates/rs-design/style/tokens.css          ← "Source 1" ├── packages-js/canonrs-design/tokens.css      ← "Source 2" ├── examples/workbench/style/tokens.css        ← "Source 3" └── frontend/styles/tokens.css                 ← "Source 4" ```  **Why this is wrong:** - Tokens diverge between fil

### Solution

``` monorepo/ ├── crates/rs-design/style/ │   ├── tokens.css       ← ÚNICA FONTE DA VERDADE │   ├── main.css │   └── theme.css │ └── examples/workbench/ ├── style/globals.css │   @import "/absolute/path/to/crates/rs-design/style/tokens.css" └── public/workbench.css  ← ARTIFACT (gerado via build) ```

### Anti-patterns

- ### ❌ Wrong Pattern (Multiple Sources)
- | Copy-paste | Many (diverge) | ❌ No | ✅ Yes (but breaks) |
- # ❌ If shows @import, path is wrong

---

## CR-063 — Leptos Reactivity - No .get() Outside Closures

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** leptos, state
- **Version:** 1.0.0

### Problem

```rust let tokens = Memo::new(move |_| fetch_tokens());  // WRONG: .get() outside tracking context let count = tokens.get().len();  view! { <p>{count}</p>  // 🚫 Never updates } ```  **Error message:** ``` you access a reactive_graph::computed::arc_memo::ArcMemo<T> outside a reactive tracking contex

### Solution

```rust let tokens = Memo::new(move |_| fetch_tokens());  view! { <p>{move || tokens.get().len()}</p>  // ✅ Reactive } ```  **Why this works:** - `move ||` creates closure - Leptos tracks `tokens` as dependency - Changes trigger re-render automatically - UI stays synchronized  ---

### Anti-patterns

- ### ❌ Wrong Pattern (Breaks Reactivity)
- // ❌ WRONG
- #### ❌ Wrong: Passing Memo directly

---

## CR-064 — CSS Build Pipeline is Mandatory

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** build, css
- **Version:** 1.0.0

### Problem

```bash cargo leptos serve  cargo leptos serve ```  **What happens:** ```html <!-- HTML references CSS --> <link rel="stylesheet" href="/workbench.css">  <!-- But file doesn't exist --> $ ls public/workbench.css ls: cannot access 'public/workbench.css': No such file or directory ```  **Symptoms:** -

### Solution

```bash npx @tailwindcss/cli \ -i style/globals.css \ -o public/workbench.css \ --minify  cargo leptos build  cargo leptos serve ```  ---

### Anti-patterns

- ### ❌ Wrong Assumption (No Build)
- - ❌ Does NOT run Tailwind
- - ❌ Does NOT process PostCSS

---

## CR-065 — data-theme Sync is ThemeProvider Responsibility

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** tokens, theming, providers
- **Version:** 1.0.0

### Problem

```rust pub fn ThemeProvider(children: Children) -> impl IntoView { let mode = RwSignal::new(ThemeMode::Light); let preset = RwSignal::new("amber-minimal".to_string());  provide_context(ThemeContext { mode, preset });  children()  // 🚫 NO DOM sync } ```  **CSS expects:** ```css [data-theme="amber-mi

### Solution

```rust pub fn ThemeProvider( children: Children, ) -> impl IntoView { let mode = RwSignal::new(ThemeMode::Light); let preset = RwSignal::new(initial_preset.unwrap_or("amber-minimal".to_string()));  // ✅ CRITICAL: Sync to DOM Effect::new(move |_| { if let Some(window) = web_sys::window() { if let So

### Anti-patterns

- ### ❌ Wrong Pattern (State Only)
- - CSS never activates ❌
- - Theme doesn't apply ❌

---

## CR-066 — Workbench Setup Checklist

- **Severity:** LOW
- **Status:** ENFORCED
- **Scope:** workspace, build
- **Version:** 1.0.0

### Problem

```bash vim src/app.rs  ```  **Time wasted:** 2-4 hours debugging what should have been validated upfront.  ---

### Solution

```bash ./scripts/validate-setup.sh  ```  ---

### Anti-patterns

- ### ❌ Wrong Approach (Assume Everything Works)
- # ❌ Any check fails → Fix before proceeding
- - ❌ "Why is page white?" (70% of issues)

---

## CR-067 — Leptos CSR Does NOT Load CSS via `<Stylesheet />`

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** leptos, csr, css
- **Version:** 1.0.0

### Problem

Developers coming from SSR frameworks assume: ```rust view! { <Stylesheet href="/styles.css"/> <App /> } ```  Will inject `<link>` tags into the document. **In CSR, it does not.**  **Why:** - CSR uses `mount_to_body()` which replaces `<body>` content - `<Stylesheet />` generates meta tags during SSR

### Anti-patterns

- // ❌ NEVER IN CSR

---

## CR-068 — Asset Must Exist in Final dist/

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** build, workspace
- **Version:** 1.0.0

### Problem

Applications often reference assets that:  - Were never generated - Were generated in the wrong location - Failed silently during build - Exist in source but not in output  **Result:** 404 errors, missing styles, broken functionality — all silent until runtime.  ---

### Anti-patterns

- <!-- ❌ NEVER ALLOWED -->

---

## CR-069 — Trunk Only Serves What's in dist/

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** workspace, build
- **Version:** 1.0.0

### Problem

Developers assume Trunk works like Vite or Webpack Dev Server: - "Put assets in `public/` and they're served" - "CSS in `style/` is automatically available" - "Trunk will find it"  **Wrong.** Trunk has a different model:  1. Processes `index.html` and its `data-trunk` directives 2. Copies/generates 

### Anti-patterns

- # ❌ WRONG ASSUMPTIONS
- // ❌ WRONG
- <!-- ❌ WRONG -->

---

## CR-070 — CSS Pipeline Requires Health Checks

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** build, css
- **Version:** 1.0.0

### Problem

CSS build pipelines fail in subtle ways: - Build command runs but outputs nothing - File is generated but is empty - Classes are missing due to purge errors - Theme tokens not compiled - Path resolution fails silently  **Without validation, these failures only surface at runtime in production.**  --

### Anti-patterns

- // ❌ NO VALIDATION
- console.error(`❌ ${CSS_PATH} does not exist`);
- console.error(`❌ ${CSS_PATH} is empty`);

---

## CR-071 — Debug Theme by Verifying File First

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** tokens, theming
- **Version:** 1.0.0

### Problem

When dark mode, theming, or styling doesn't work, developers instinctively debug: - React/Leptos state management - CSS-in-JS token generation - DOM class application - JavaScript event handlers - Framework reactivity systems  **95% of the time, the CSS file is missing or broken.**  ---

### Anti-patterns

- echo "   ❌ CSS file missing in dist/"
- echo "   ❌ CSS returns $HTTP_CODE"
- echo "   ❌ Missing: $class"

---

## CR-072 — Layout H1 Prohibition

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** accessibility, layout
- **Version:** 1.0.0

### Anti-patterns

- | Severity | ❌ Critical |
- // ❌ WRONG - Layout with h1
- <h1>"My App"</h1>  // ❌ Layout choosing h1

---

## CR-073 — ComponentPage Template Contract

- **Severity:** LOW
- **Status:** ENFORCED
- **Scope:** components
- **Version:** 1.0.0

### Anti-patterns

- | Severity | ❌ Critical |
- // ❌ WRONG - Reimplementing structure in product

---

## CR-074 — Block Semantic HTML

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components, ui
- **Version:** 1.0.0

### Anti-patterns

- | Severity | ❌ Critical |
- // ❌ WRONG - Skipping h2
- <h3>"API"</h3>  // ❌ Should be h2

---

## CR-075 — Primitive CSS Prohibition

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** primitives, css, architecture
- **Version:** 1.0.0

### Anti-patterns

- // ❌ NEVER ALLOWED IN PRIMITIVES
- pub is_mobile: bool,  // ❌ NO
- pub breakpoint: String,  // ❌ NO

---

## CR-076 — Navigation vs Action Component Contract

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components, ui
- **Version:** 1.0.0

### Anti-patterns

- // ❌ NEVER ALLOWED
- <a href="/tokens">  // ❌ INVALID HTML
- // ❌ HTML INVALID

---

## CR-077 — Monorepo Path Stability

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** workspace, architecture
- **Version:** 1.0.0

### Anti-patterns

- ├─ Cargo.toml  (path = "../../crates/rs-design")  ❌ BROKEN
- └─ postcss.config.js  (../../crates/...)          ❌ BROKEN
- // ❌ NEVER ALLOWED

---

## CR-078 — Token Definition Completeness

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** design-system, tokens
- **Version:** 1.0.0

### Anti-patterns

- // ❌ INCOMPLETE TOKEN USAGE
- width: 16rem;                    // ❌ Hardcoded, should use --sidebar-width
- // ❌ Partial token usage

---

## CR-079 — ComponentPage Template Contract

- **Severity:** LOW
- **Status:** ENFORCED
- **Scope:** components
- **Version:** 1.0.0

### Anti-patterns

- // ❌ NEVER ALLOWED
- <div class="page">  // ❌ Generic class name
- <div class="main">  // ❌ Semantic tag as class

---

## CR-080 — Workspace Watch Configuration

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** workspace, build
- **Version:** 1.0.0

### Anti-patterns

- # Result: No recompilation ❌
- # ❌ WRONG
- # ❌ WRONG (from workspace root instead of Cargo.toml)

---

## CR-081 — Flex Layout Ownership

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** ui, layout, design-system
- **Version:** 1.0.0

### Anti-patterns

- // ❌ NEVER ALLOWED
- <div data-sidebar-inset="" class="flex-1">  // ❌ Primitive has flex
- <main>{children()}</main>  // ❌ No flex wrapper in UI

---

## CR-082 — CSS Build Pipeline Health

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** build, css
- **Version:** 1.0.0

### Anti-patterns

- # ❌ WRONG WORKFLOW
- # Reality: Nothing happens ❌
- # ❌ ERROR

---

## CR-083 — Layout Zones Contract

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** layout, architecture
- **Version:** 1.0.0

### Anti-patterns

- - ❌ NEVER: Direct content
- - ❌ NEVER: Components
- - ❌ NEVER: Primary content

---

## CR-084 — SVG Dark Mode Contract

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** css, tokens, design-system
- **Version:** 1.0.0

### Problem

Developers assume SVG strokes will adapt to dark mode by using `stroke="currentColor"`: ```svg <!-- ❌ DOES NOT WORK with <img> --> <svg> <path stroke="currentColor" /> </svg> ``` ```html <!-- ❌ currentColor is NOT inherited --> <img src="icon.svg" /> ```  **Result:** Icons remain black in dark mode,

### Anti-patterns

- <!-- ❌ DOES NOT WORK with <img> -->
- <!-- ❌ currentColor is NOT inherited -->
- ### ❌ Anti-Pattern 1: `currentColor` in External SVG

---

## CR-085 — Leptos Asset Pipeline in Dev Mode

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** leptos, build
- **Version:** 1.0.0

### Problem

Developers assume `Trunk.toml` hooks will copy assets during `make dev`: ```toml [[hooks]] stage = "pre_build" command = "cp" command_arguments = ["-r", "assets", "dist/"] ```  **Result:** Hook never executes. Assets return 404. Developer wastes hours debugging Trunk.  ---

### Anti-patterns

- | `make dev` | `public/` | Leptos dev server | ❌ No |
- | `cargo leptos watch` | `public/` | Leptos dev server | ❌ No |
- ### ❌ Anti-Pattern 1: Relying on Trunk Hooks in Dev

---

## CR-086 — Children vs ChildrenFn Contract

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** leptos, state
- **Version:** 1.0.0

### Problem

Developers encounter this error repeatedly: ``` error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce` ```  **Root causes:** 1. Using `Children` in wrapper components 2. Trying to call `children()` multiple times 3. Attempting `StoredValue<Children

### Anti-patterns

- ### ❌ Anti-Pattern 1: `{children}` Without `()`
- <div>{children}</div> // ❌ WRONG
- ### ❌ Anti-Pattern 2: `StoredValue<Children>`

---

## CR-087 — Leptos SSR Script Placement

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** hydration, ssr, leptos
- **Version:** 1.0.0

### Problem

```html <!-- index.html --> <!DOCTYPE html> <html> <head>...</head> <body> <script> document.addEventListener("click", (e) => { const btn = e.target.closest("[data-copy-button]"); // ❌ btn is never found, even though HTML has the button }); </script> </body> </html> ```  **Why it fails:** - Script r

### Anti-patterns

- ### ❌ Anti-Pattern: Script in `index.html`
- // ❌ btn is never found, even though HTML has the button
- | `index.html` | ❌ Ignored | ❌ Replaced | ❌ Stale DOM | **FORBIDDEN** |

---

## CR-089 — Primitives Must Never Touch Browser APIs

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** primitives, ssr
- **Version:** 1.0.0

---

## CR-090 — Hydration Is DOM Replacement, Not Enhancement

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** hydration, ssr, leptos
- **Version:** 1.0.0

---

## CR-091 — Markdown and Code Blocks Are Render-Only

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components, ui
- **Version:** 1.0.0

---

## CR-092 — If It Works in Prod but Not Dev, Suspect Hydration Order

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** hydration, ssr
- **Version:** 1.0.0

---

## CR-093 — Leptos WASM Dev Builds Must Use Release Mode

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** leptos, wasm, build
- **Version:** 1.0.0

### Problem

When using default dev/watch mode:  - WASM is built with full debug symbols - Binary size explodes to hundreds of megabytes - Browser must download, validate, compile, and instantiate the module - Main thread is blocked for tens of seconds to minutes - App appears frozen after refresh with no errors

### Anti-patterns

- # ❌ FORBIDDEN for medium/large apps

---

## CR-094 — Leptos Workspace Features Must Be Explicit

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** leptos, workspace
- **Version:** 1.0.0

### Problem

When `features = ["nightly"]` is declared only in member crate dependencies:  - Leptos macros cannot infer types in closures - Every `view!` block fails with E0282 - Error messages are misleading (point to view! code, not features) - No compilation succeeds despite syntactically correct code  ```rus

### Anti-patterns

- # ❌ FORBIDDEN: Features only in member crate

---

## CR-095 — SSR Requires Complete HTML Shell

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** leptos, ssr
- **Version:** 1.0.0

### Problem

When SSR components use `leptos_meta` (Title, Meta, etc.) without a complete HTML shell:  - Runtime panic: "you are using leptos_meta without a </head> tag" - Server returns empty responses (ERR_EMPTY_RESPONSE) - No error message in build, only at first request - Hydration scripts have nowhere to at

### Anti-patterns

- // ❌ FORBIDDEN: No HTML shell

---

## CR-096 — SSR Requires Explicit Provider Tree

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** providers, leptos, ssr
- **Version:** 1.0.0

### Problem

When components use contexts (Theme, Density, etc.) but the App component doesn't provide them:  - Server panics: "ThemeContext not found. Make sure ThemeProvider wraps your app." - Error occurs at first render, not at build time - Browser shows ERR_EMPTY_RESPONSE - No indication which provider is m

### Anti-patterns

- // ❌ FORBIDDEN: No providers
- // ❌ WRONG: Dependent outside dependency

---

## CR-097 — Leptos 0.8 Requires Floating Nightly Toolchain

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** leptos, workspace
- **Version:** 1.0.0

### Problem

When using pinned nightly or pinned Leptos minor versions:  - Build fails with "rustc X.Y is not supported by leptos" - Or: "feature `edition2024` is required but not stabilized" - Or: Dependency resolution conflicts with newer crates - Error messages don't clearly indicate toolchain mismatch  ``` e

### Anti-patterns

- # ❌ FORBIDDEN: Pinned nightly date
- # ❌ FORBIDDEN: Pinned minor version
- | nightly-2024-11-01 | 0.8.15 | ❌ rustc too old |

---

## CR-098 — Axum + Leptos SSR Closures Must Own State

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** leptos, ssr, architecture
- **Version:** 1.0.0

### Problem

When trying to share `LeptosOptions` across multiple router closures:  - Compiler error: "borrow of moved value: `leptos_options`" - Error points to `.with_state()` or second closure - First closure moves the value, subsequent code cannot borrow - Not immediately obvious that the solution is clone-b

### Anti-patterns

- // ❌ FORBIDDEN: Shared reference across closures

---

## CR-099 — CSR Islands Must Not Own Routing or Providers

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** ssr, csr, wasm
- **Version:** 1.0.0

### Problem

When WASM islands try to own routing or providers:  - Router conflicts with SSR's router (two routers fighting) - Provider contexts leak between islands and host - Hydration mismatches (SSR expects one tree, CSR creates another) - Islands cannot be lazy-loaded independently - Memory leaks on unmount

### Anti-patterns

- // ❌ FORBIDDEN: Island creates router
- // ❌ FORBIDDEN: Island provides global context

---

## CR-100 — Build Orchestrators Must Be Workspace-Scoped

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** build, workspace, leptos
- **Version:** 1.0.0

### Problem

When build orchestrator configuration exists only in member crates:  - cargo-leptos cannot find configuration - Build fails with "metadata not found" errors - Hot reload doesn't work (watches wrong directories) - Asset paths are incorrect - Multiple members create conflicting configurations  ``` Err

### Anti-patterns

- # ❌ FORBIDDEN: Configuration in member crate only

---

## CR-101 — Workbench Assets Must Be Product-Scoped

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** build, design-system
- **Version:** 1.0.0

---

## CR-102 — Runtime JS Is Shell Infrastructure

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** interactive, behavior
- **Version:** 1.0.0

---

## CR-103 — Critical Runtime JS Must Be Inline in SSR

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** hydration, ssr
- **Version:** 1.1.0

### Anti-patterns

- Event listeners don't work
- Clipboard API fails silently
- No error messages

---

## CR-104 — AutoReload Breaks Script Order Guarantees

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** leptos, build, ssr
- **Version:** 1.1.0

### Anti-patterns

- Event listeners don't fire
- Clipboard API does nothing
- Drag & drop fails

---

## CR-105 — canon-rule-105-visual-indicators-must-have-a-single-owner

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** ui, architecture
- **Version:** 1.0.0

### Problem

When two layers render parts of the same visual signal:  - The model becomes ambiguous - Styling degenerates into patching - Bugs are "fixed" with offsets or overrides - Architecture silently rots  This is not a CSS issue — it is a **semantic ownership violation**.  ---

---

## CR-106 — canon-rule-106-ui-neutralizes-hostile-css-not-primitives

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** primitives, css
- **Version:** 1.0.0

### Problem

Global resets and base styles may affect primitives unintentionally.  Incorrect responses include: - adding CSS to primitives - adding wrapper hacks - modifying primitive semantics  These actions violate CanonRS layering.  ---

---

## CR-107 — Token Architecture & Theme Specificity

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** tokens, css, architecture
- **Version:** 1.0.0

### Problem

```css /* tokens.css */ :root { --color-primary-bg: hsl(38 92% 50%);  /* Hardcoded default */ --color-bg-surface: hsl(0 0% 100%); --space-lg: 1rem;  /* OK - structural */ }  [data-theme="clean-slate"] { --color-primary-bg: hsl(238 83% 66%);  /* 🚫 CANNOT override :root */ } ```  **Why this breaks:** 

### Solution

```css /* tokens.css - STRUCTURAL ONLY */ :root { /* ━━━ STRUCTURAL (never change with theme) ━━━ */ --space-xs: 0.25rem; --space-sm: 0.5rem; --space-lg: 1rem; --space-xl: 1.5rem;  --font-size-sm: 0.875rem; --font-size-md: 1rem; --font-size-lg: 1.125rem;  --radius-sm: 0.25rem; --radius-md: 0.375rem;

### Anti-patterns

- ### ❌ Wrong Pattern (Conflicting Specificity)
- --color-primary-bg: hsl(38 92% 50%); /* ❌ WRONG - color in :root */
- ### ❌ Colors in :root

---

## CR-108 — Visual Surfaces Contract

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** tokens, css, components
- **Version:** 1.0.0

### Anti-patterns

- - ❌ Cards or highlighted containers
- - ❌ Navigation components
- - ❌ Component previews

---

## CR-111 — Model First, CSS Second

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** architecture, css, components
- **Version:** 1.0.0

### Anti-patterns

- Fix structural overlap
- Hide responsibility conflicts
- "Adjust" incorrect visual authority

---

## CR-112 — UI Owns Visual Style

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** primitives, ui, css
- **Version:** 1.0.0

### Anti-patterns

- CSS in `src/primitives/*`
- Primitive with `class=""`
- Primitive "knowing" how it looks

---

## CR-113 — States Are Data, Not Style

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** css, state, components
- **Version:** 1.0.0

### Anti-patterns

- States encoded as visual classes (`.is-blue`)
- Style logic in Rust component
- Implicit states via DOM (e.g., `:first-child`)

---

## CR-114 — Single Visual Authority

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components, css
- **Version:** 1.0.0

### Anti-patterns

- List with `border-bottom` + trigger with `border-bottom`
- Wrapper drawing visual state
- "Compensations" with negative margin

---

## CR-115 — Reset Awareness & CSS Boundaries

- **Severity:** MEDIUM
- **Status:** ENFORCED
- **Scope:** css, components
- **Version:** 1.0.0

### Anti-patterns

- Fix reset inside primitive
- Use `!important` to "win"
- Create wrappers just to fight CSS

---

## CR-116 — WASM Externref Table Limits

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** wasm, components
- **Version:** 1.0.0

### Problem

When using WASM with JS interop: - Each `#[wasm_bindgen]` export creates table entries - Each `Callback`, `on:*` handler, `web_sys` API call consumes refs - Providers that register global handlers multiply this - Browser refuses to grow table beyond limits  ``` RangeError: WebAssembly.Table.grow(): 

### Anti-patterns

- // ❌ FORBIDDEN - creates externref per component
- // ❌ FORBIDDEN - providers with closures

---

## CR-117 — Design System Callbacks Are Props, Not Handlers

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** components, design-system
- **Version:** 1.0.0

### Problem

Current pattern (FORBIDDEN): ```rust pub fn Button(on_click: Callback<()>) -> impl IntoView { view! { <button on:click=move |_| on_click.run(())>  // ❌ externref! } } ```  Each Button instance = 1 externref 100 buttons = 100 refs = Table.grow() crash  ---

### Anti-patterns

- <button on:click=move |_| on_click.run(())>  // ❌ externref!

---

## CR-118 — View<\_> Type Boundary Prohibition

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** architecture, components
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ FORBIDDEN #1: Helper Returns View<\_>
- // ❌ WRONG - Open inference boundary
- ### ❌ FORBIDDEN #2: View<\_> in Intermediates

---

## CR-119 — No #[prop(optional, into)] in UI Layer

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** components, design-system
- **Version:** 1.1.0

### Problem

The combination `#[prop(optional, into)]` creates catastrophic type ambiguity in Leptos:  1. **Compiler cannot infer** whether you're passing `T` or `Option<T>` 2. **Multiplies E0308 errors** across every call site 3. **Forces users to add `.into()` or `Some()` everywhere** 4. **Breaks intuitive API

### Anti-patterns

- ### ❌ Forbidden: Using `#[prop(optional, into)]` in UI Layer
- #[prop(optional, into)] class: Option<String>,  // ❌ NEVER DO THIS
- #[prop(optional, into)] id: Option<String>,     // ❌ NEVER DO THIS

---

## CR-120 — DOM Events vs Semantic Callbacks Boundary

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** primitives, ui, behavior
- **Version:** 1.0.0

### Problem

Mixing DOM event syntax with callback props creates three structural failures:  1. **Type confusion:** `on:click` vs `on_click` vs `onclick` — unclear boundaries 2. **Unnecessary wrapping:** `Some(Callback::new(...))` patterns proliferate 3. **Primitive leakage:** DOM events appear in UI layer publi

### Anti-patterns

- ### ❌ Forbidden: DOM events in UI layer public API
- on:click: Callback<MouseEvent>,  // ❌ DOM event in UI API
- ### ❌ Forbidden: Wrapping callbacks in Some()

---

## CR-121 — StoredValue for Non-Copy Values in view! Closures

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** leptos, ssr, state
- **Version:** 1.0.0

### Problem

Leptos `view!` macro generates closures that must be `Fn` (callable multiple times). When non-Copy values are moved into these closures, Rust enforces `FnOnce` semantics, causing compile failures.  **The root cause chain:**  1. `view!` needs `Fn` for reactivity (re-render on signal changes) 2. Movin

### Anti-patterns

- ### ❌ Forbidden: Moving String into view! closure
- <div>{title}</div>  // ❌ Moves `title` — FnOnce error
- ### ❌ Forbidden: Moving registry/context into closure

---

## CR-122 — No Conditional Rendering with .then() Closures

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** leptos, components
- **Version:** 1.0.0

### Problem

Using `bool.then(|| view!)` for conditional rendering causes three structural failures:  1. **FnOnce violations** — closure captures values by move 2. **Type inference failures** — return type ambiguity 3. **SSR hydration mismatches** — inconsistent DOM structure  **Real symptoms from production:** 

### Anti-patterns

- ### ❌ Forbidden: Using .then() for conditional views
- {open.get().then(|| view! {  // ❌ FORBIDDEN
- ### ❌ Forbidden: Nested .then() conditions

---

## CR-123 — Component Architecture Taxonomy and Contracts

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** architecture, design-system
- **Version:** 1.0.0

### Problem

Without explicit architectural boundaries, codebases degrade into:  1. **Classification chaos** — "Is this a primitive or UI component?" 2. **Contract violations** — Primitives with business logic, UI with CSS tokens 3. **Boundary leakage** — DOM events in application code, styling in primitives 4. 

### Anti-patterns

- // ❌ WRONG
- // ❌ WRONG: Type conversion in Primitive
- #[prop(optional, into)] class: Option<String>,  // ❌ NEVER use `into`

---

## CR-124 — Primitive Contract Types

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** primitives
- **Version:** 1.0.0

### Problem

Without formal primitive types, developers create "hybrid primitives" that:  1. Mix DOM rendering with state management 2. Apply CSS directly instead of data attributes 3. Convert types (`.into()`, `unwrap_or_default()`) instead of passing through 4. Emit semantic callbacks instead of raw DOM events

### Anti-patterns

- // ❌ WRONG: Type conversion in Primitive
- #[prop(optional, into)] class: Option<String>,  // ❌ NEVER use into
- <button class={class.unwrap_or_default()}>  // ❌ NEVER unwrap

---

## CR-125 — UI Component Contracts

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** ui
- **Version:** 1.0.0

### Problem

Without formal UI layer types, developers create components that:  1. Use Primitives directly in domain code (skipping UI layer) 2. Mix visual state with domain state 3. Define CSS tokens instead of using design system 4. Emit DOM events (`MouseEvent`) instead of semantic callbacks 5. Use `#[prop(op

### Anti-patterns

- // ❌ WRONG: Multiple Primitives in AdapterUI
- <ButtonPrimitive>  // ❌ Calling 2 primitives
- // ❌ WRONG: Domain state in ControlledUI

---

## CR-126 — Component Domain Contracts

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** components
- **Version:** 1.0.0

### Problem

Without formal component layer types, developers create components that:  1. Mix domain logic with visual concerns (validation + tabs state in one component) 2. Use Primitives directly instead of UI layer 3. Manage global state locally (duplicating cart, user session) 4. Emit DOM events instead of d

### Anti-patterns

- // ❌ WRONG: Primitive in Component layer
- <InputPrimitive />  // ❌ Should use Input (UI layer)
- <ButtonPrimitive>"Login"</ButtonPrimitive>  // ❌ Should use Button

---

## CR-127 — Block Composition Contracts

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components
- **Version:** 1.0.0

### Problem

Without formal block types, developers create blocks that:  1. Manage global state (user session, cart) locally 2. Contain routing logic (navigation, URL handling) 3. Define page-level layout (header positioning, z-index) 4. Implement business logic instead of composing components 5. Blur boundary b

### Anti-patterns

- // ❌ WRONG: Domain state in Semantic Block
- let user = use_context::<User>().unwrap();  // ❌ State
- <h1>"Welcome, "{user.name}</h1>  // ❌ Domain data

---

## CR-128 — Layout Shell and Zone Contracts

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** layout
- **Version:** 1.0.0

### Problem

Without formal layout types, developers create layouts that:  1. Contain business logic (authentication, data fetching) 2. Manage domain state (user preferences, cart data) 3. Render domain components directly (forms, dashboards) 4. Mix structural concerns with feature concerns 5. Create untestable,

### Anti-patterns

- // ❌ WRONG: Authentication logic in Shell
- let user = use_context::<User>().unwrap();  // ❌ Domain state
- let logout = move |_| {  // ❌ Business logic

---

## CR-129 — SSR Event Safety

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** hydration, ssr, behavior
- **Version:** 1.0.0

### Problem

When event handlers are attached to dynamically generated nodes during SSR:  1. The server renders static HTML with no event listeners 2. During hydration, the client tries to attach listeners to DOM nodes 3. The DOM walking algorithm expects identical structure between SSR and CSR 4. Dynamic closur

### Anti-patterns

- ### ❌ Forbidden
- ### ❌ Forbidden
- ### ❌ Forbidden

---

## CR-130 — Controlled UI Contract

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components, state
- **Version:** 1.0.0

### Problem

When UI components accept plain types for reactive state:  1. The component cannot react to state changes 2. Parent components must call `.get()` to "adapt" the signal 3. This breaks reactivity—updates don't propagate 4. State becomes stale and UI diverges from reality  **Observable symptoms:** - UI

### Anti-patterns

- ### ❌ Forbidden
- open: bool,  // ❌ Wrong!
- <Dialog open=is_open.get()>  // ❌ Only reads once!

---

## CR-131 — Reactive Boundary Ownership

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** leptos, state
- **Version:** 1.0.0

### Problem

Reactive boundaries create closures that may execute multiple times. When non-`Copy` values are moved into these closures:  1. Rust's ownership rules require `Fn` trait (callable multiple times) 2. Moving non-`Copy` values makes the closure `FnOnce` (callable once) 3. Leptos expects `Fn`, causing co

### Anti-patterns

- ### ❌ Forbidden — Moving String into Show
- <TabsContentPrimitive class=class>  // ❌ Moves class
- {children()}  // ❌ Also moves children

---

## CR-132 — Layout Composition Over Abstraction

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** layout, leptos
- **Version:** 1.0.0

### Problem

When layouts attempt to manage slots, render props, or complex composition:  - Ownership errors with `Children` and `ViewFn` - SSR hydration mismatches - Non-deterministic render order - Tight coupling between layout and app-specific structure - Hard-to-debug move semantics failures  These issues su

### Anti-patterns

- ### ❌ Forbidden

---

## CR-133 — Children Consumption Locality

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** leptos, state
- **Version:** 1.0.0

### Problem

`Children` in Leptos is `FnOnce`. When passed across component layers:  - Move errors occur - Closures created by `view!` cannot safely capture it - SSR rendering panics or fails - Workarounds introduce unnecessary indirection  This caused repeated architectural failures.  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-134 — Layouts Are CSS and Semantics Only

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** css, layout
- **Version:** 1.0.0

### Problem

When layouts include logic beyond structure:  - Layouts become stateful implicitly - UI behavior leaks into structure - CSS zones lose determinism - SSR/CSR boundaries become unclear - Layout reuse across apps breaks  This leads to fragile layouts that cannot be reasoned about statically.  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-135 — UI vs Layout Responsibility Boundary

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** architecture, ui, layout
- **Version:** 1.0.0

### Problem

When UI behavior leaks into layouts, or layouts leak into UI:  - Responsibilities blur - Controllers attach to the wrong layer - CSS zones become coupled to behavior - Refactors break unrelated areas  This was observed in sidebar and header orchestration attempts.  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-136 — Controllers Are CSR-Only

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** ssr, csr, behavior
- **Version:** 1.0.0

### Problem

When Controllers run during SSR:  - Hydration mismatches occur - Event handlers bind to non-existent DOM - Side effects execute twice (SSR + CSR) - Panics happen due to browser-only APIs - Debugging becomes non-deterministic  This directly violates SSR safety guarantees.  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-137 — Providers Must Have a Single Owner

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** state, providers
- **Version:** 1.0.0

### Problem

When Providers are instantiated in multiple places:  - State becomes duplicated - Signals diverge silently - UI desynchronizes - Debugging becomes impossible - Ownership is unclear  This was observed when Providers were placed in both Layouts and Blocks.  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-138 — Children Must Be Consumed Immediately

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** leptos, layout, components
- **Version:** 1.0.0

### Problem

In Leptos, `Children` is implemented as `FnOnce`.  When `Children` is: - stored, - forwarded, - captured by closures, - or passed through multiple components,  the result is:  - Ownership violations - Move errors during compilation - Non-deterministic render behavior - Incompatibility with `view!` m

### Anti-patterns

- ### ❌ Forbidden
- {children()} // ❌ consumed inside closure, not immediately
- ### ❌ Forbidden

---

## CR-139 — Slots Are UI-Level Only

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components, layout
- **Version:** 1.0.0

### Problem

Slots introduce:  - Typed child ownership - Complex lifetime semantics - Fn/FnOnce boundaries - Render-time branching  When used in Layouts:  - Ownership explodes across layers - SSR execution becomes unsafe - Layouts become logic-heavy - Composition becomes brittle  This directly contradicts Canon 

### Anti-patterns

- ### ❌ Forbidden

---

## CR-140 — Layouts Are Structural, Not Behavioral

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** layout, leptos, architecture
- **Version:** 1.0.0

### Problem

When layouts contain behavior:  - State leaks across routes - Ownership rules are violated - SSR execution becomes non-deterministic - Layouts become tightly coupled to app logic - Reuse across products becomes impossible  This manifested as: - Signals inside layouts - Providers with implicit logic 

### Anti-patterns

- ### ❌ Forbidden
- let open = create_rw_signal(true); // ❌ state in layout
- ### ❌ Forbidden

---

## CR-141 — CSR Composition Belongs to the App Layer

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** csr, architecture
- **Version:** 1.0.0

### Problem

When CSR behavior is embedded outside the app layer:  - SSR crashes or hydrates incorrectly - `cfg(target_arch = "wasm32")` leaks everywhere - Design System becomes environment-aware - Logic duplication across products - Impossible-to-test side effects  This occurred when: - Controllers were placed 

### Anti-patterns

- ### ❌ Forbidden
- <SidebarController /> // ❌ CSR logic in layout
- ### ❌ Forbidden

---

## CR-142 — canon-rule-142-children-must-always-be-optional

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components, ui, architecture
- **Version:** 1.0.0

---

## CR-143 — canon-rule-143-ui-must-be-hydration-deterministic

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** hydration, ssr, ui
- **Version:** 1.0.0

---

## CR-144 — canon-rule-144-providers-expose-state-apps-own-interaction

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** providers, state, architecture
- **Version:** 1.0.0

---

## CR-145 — canon-rule-145-ui-must-not-mutate-global-state-implicitly

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** state, ui, architecture
- **Version:** 1.0.0

---

## CR-146 — canon-rule-146-ui-content-must-be-ssr-stable

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** ssr, hydration, ui
- **Version:** 1.0.0

---

## CR-147 — canon-rule-147-reactive-closures-are-data-not-structure

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** leptos, ssr, ui
- **Version:** 1.0.0

---

## CR-148 — canon-rule-148-ui-must-never-infer-intent-from-state

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** state, ui, architecture
- **Version:** 1.0.0

---

## CR-149 — canon-rule-149-controllers-must-be-temporal-not-structural

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** architecture, ui, layout
- **Version:** 1.0.0

---

## CR-150 — canon-rule-150-ui-must-be-deterministic-under-ssr-and-hydration

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** ssr, hydration, ui
- **Version:** 1.0.0

---

## CR-151 — canon-rule-151-visual-feedback-must-never-encode-application-state

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** state, ui, components
- **Version:** 1.0.0

---

## CR-152 — canon-rule-152-provider-callback-hydration-ownership

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** hydration, providers, leptos
- **Version:** 1.0.0

### Anti-patterns

- Forbidden Patterns ❌

---

## CR-153 — canon-rule-153-layouts-must-be-event-free-during-hydration

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** hydration, layout, ssr
- **Version:** 1.0.0

### Problem

Layouts (AppLayout, AppShell, PageLayout, etc.) are rendered:  - On the server (SSR) - Immediately re-walked by the client during hydration  If an event listener exists at this level, the runtime may attempt to attach a callback to a node that:  - Was moved - Was replaced - Was optimized away - Has 

### Anti-patterns

- Any `on:*` DOM event in Layouts
- Business logic inside Layouts
- State mutation in Layouts

---

## CR-154 — Deterministic Layout via Canonical CSS (SSR-Safe)

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** hydration, layout, css
- **Version:** 1.0.0

### Problem

Without this rule, SSR applications suffer from **hydration mismatch panics** caused by DOM divergence between server render and client hydration.  Observable symptoms include:  - Hydration errors when toggling layout elements - Runtime panics in Leptos / React SSR - Inconsistent sidebar positioning

### Anti-patterns

- ### ❌ Forbidden

---

## CR-155 — CSS Token Contract Is Architecture

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** design-system, css, tokens
- **Version:** 1.0.0

### Problem

Without uniform token format:  - **Runtime CSS failures**: `background: hsl(var(--color-primary))` breaks if token contains `hsl()` already - **Theming impossible**: Cannot interpolate or transform wrapped values - **Silent bugs**: Browser falls back to defaults, appearing as "CSS not loading" - **M

### Anti-patterns

- ### ❌ Forbidden
- --color-primary: hsl(221 83% 53%);        /* ❌ wrapped */
- --color-accent: #3b82f6;                  /* ❌ hex */

---

## CR-156 — CSS Variable Scope Is Non-Negotiable

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** design-system, css, theming
- **Version:** 1.0.0

### Problem

Without scope alignment:  - **Variables resolve to empty**: `var(--primary)` returns `""` in browser - **Themes don't apply**: Button stays default browser color despite theme being set - **Silent failures**: CSS loads, HTML correct, but values never populate - **Debugging nightmare**: Everything "l

### Anti-patterns

- ### ❌ Forbidden
- --primary: var(--color-primary);        /* ❌ UNDEFINED */
- --secondary: var(--color-secondary);    /* ❌ UNDEFINED */

---

## CR-157 — Design System CSS Must Be Build-Time Flattened

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** build, css, design-system
- **Version:** 1.0.0

### Problem

Without build-time flattening:  - **Silent import failures**: Browser ignores `@import "./ui/button.css"` if path breaks - **Tokens never load**: Relative paths fail in served dist vs source - **CSS appears loaded**: File downloads but imports fail silently - **Components "have no style"**: HTML cor

### Anti-patterns

- ### ❌ Forbidden
- echo "❌ Unresolved @import in output" && exit 1
- echo "❌ Components missing from output" && exit 1

---

## CR-158 — Design System Packages Are Immutable Contracts (No Direct File Imports)

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** design-system, architecture
- **Version:** 1.0.0

### Problem

Without package boundaries:  - **Silent breakage**: App imports `../../rs-design/style/button.css`, design system refactors → app breaks - **No versioning**: Cannot track which app uses which design system version - **Circular dependencies**: Apps and design system become interdependent - **Build fr

### Anti-patterns

- ### ❌ Forbidden
- // ❌ No one does this
- echo "❌ Direct file import detected" && exit 1

---

## CR-159 — UI CSS Must Be Fail-Safe, Token-Only, and Attribute-Scoped

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** css, tokens, design-system
- **Version:** 1.0.0

### Problem

Without this rule, UI styling becomes fragile and unsafe.  Typical failures observed:  - CSS silently breaks when a Primitive forgets to emit a

---

## CR-160 — First App Must Not Require CSS Build

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** css, build
- **Version:** 1.0.0

### Problem

Requiring CSS build tools in the first app breaks onboarding.  - Immediate friction for new users - Toolchain coupling (Node, Tailwind, PostCSS) - False perception of framework complexity - Non-deterministic first experience  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-161 — Canonical CSS Load Order Is Mandatory

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** css, theming
- **Version:** 1.0.0

### Problem

Incorrect CSS order causes silent visual corruption.  - Tokens overridden by components - Themes partially applied - Layout misalignment - Non-reproducible UI bugs  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-162 — Providers Are Infrastructure, Not UI

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** providers, ui, architecture
- **Version:** 1.0.0

### Problem

Treating providers as UI causes architectural leakage.  - State logic mixed with presentation - Broken composition - Tight coupling to layouts - Unclear responsibility boundaries  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-163 — DOM Effects Are Hydration-Only

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** ssr, hydration
- **Version:** 1.0.0

### Problem

Running DOM effects during SSR causes hydration mismatch.  - Silent crashes - Inconsistent HTML - Impossible-to-debug runtime errors - Broken SSR guarantees  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-164 — WASM and JS Assets Must Be Served by SSR

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** ssr, build
- **Version:** 1.0.0

### Problem

Missing `/pkg` routing breaks hydration.  - 404 on `.wasm` and `.js` - App loads HTML but never hydrates - Silent runtime failure - False UI rendering success  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-165 — CanonRS Workbench Is a Canonical Reference

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** architecture
- **Version:** 1.0.0

### Problem

Ignoring the workbench leads to theoretical architecture drift.  - Duplicate decisions - Conflicting patterns - Reinvented solutions - Fragmented framework behavior  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-166 — Dist Is Read-Only

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** build, css
- **Version:** 1.0.0

### Problem

Editing files in dist/ creates non-reproducible builds and hides architectural errors.  ---

### Anti-patterns

- Editing any file under dist/ directly.

---

## CR-167 — CanonRS CSS Has a Single Entrypoint

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** css, architecture
- **Version:** 1.0.0

### Problem

Multiple CSS entrypoints cause import drift, ordering bugs, and missing tokens.  ---

### Anti-patterns

- Importing individual UI CSS files in applications.

---

## CR-168 — UI Must Declare Required Tokens

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** tokens, ui
- **Version:** 1.0.0

### Problem

Using undefined CSS variables leads to invisible or broken UI.  ---

### Anti-patterns

- UI CSS referencing tokens that do not exist in base tokens.

---

## CR-169 — Token Import Order Is Architectural

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** css, tokens
- **Version:** 1.0.0

### Problem

Out-of-order tokens cause partial styling and broken components.  ---

### Anti-patterns

- Importing UI CSS before core tokens.

---

## CR-170 — HTML and CSS Must Share the Same Contract

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** ui, components
- **Version:** 1.0.0

### Problem

CSS using data-* while HTML renders classes (or vice versa) breaks styling.  ---

### Anti-patterns

- CSS selectors that do not match rendered HTML.

---

## CR-171 — Phantom Variables Are Forbidden

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** css, tokens
- **Version:** 1.0.0

### Problem

Phantom variables create silent UI failures.  ---

### Anti-patterns

- Referencing variables not defined in any token file.

---

## CR-172 — The Site Does Not Own Design

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** architecture
- **Version:** 1.0.0

### Problem

Allowing sites to define UI rules causes framework fragmentation.  ---

### Anti-patterns

- Defining component-level design inside site projects.

---

## CR-173 — CSS Is a First-Class System

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** architecture, css
- **Version:** 1.0.0

### Problem

Treating CSS as secondary leads to untraceable UI bugs.  ---

### Anti-patterns

- Ad-hoc CSS fixes without architectural grounding.

---

## CR-174 — canon-rule-174-tokens-are-compile-time-contracts

- **Severity:** BLOCKING
- **Status:** ENFORCED
- **Scope:** build, tokens, architecture
- **Version:** 1.0.0

---

## CR-175 — Dark/Light Is a CSS Concern, Not a Component Concern

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components, css
- **Version:** 1.0.0

### Problem

When components implement dark/light logic:  - **Duplication** - every component reimplements theme switching - **Inconsistency** - components interpret "dark" differently - **Coupling** - components coupled to theme implementation - **Maintenance explosion** - changing theme strategy requires editi

### Anti-patterns

- ### ❌ Forbidden
- /* ❌ Component knows about dark mode */
- /* ❌ Component has dark-specific selector */

---

## CR-176 — Governance Is the Single Source of Truth

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** governance, architecture, design-system
- **Version:** 1.0.0

### Anti-patterns

- ### ❌ Forbidden
- // ❌ WRONG
- // ❌ WRONG

---

## CR-177 — canonrs.css Is a Generated Artifact, Never a Design Surface

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** build
- **Version:** 1.0.0

### Problem

When developers edit `canonrs.css` directly:  - **Silent data loss** - next build overwrites manual changes - **Source divergence** - output doesn't match source - **Debugging nightmare** - changes appear in browser but not in git - **Team confusion** - some devs edit source, others edit output - **

### Anti-patterns

- ### ❌ Forbidden
- @import "./custom-fix.css";  /* ❌ Manual addition */
- git commit -m "fix: update button colors"  # ❌ Wrong layer

---

## CR-178 — Contracts Are Read-Only APIs (DEPRECATED)

- **Severity:** NONE
- **Status:** DEPRECATED
- **Scope:** governance
- **Version:** 1.0.0

### Anti-patterns

- - ❌ Do NOT reference this rule
- - ❌ Do NOT extend this rule
- - ❌ Do NOT reintroduce this number

---

## CR-179 — No Visual Surface Tokens in Core or Families

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** design-system, tokens, theming
- **Version:** 1.0.0

### Problem

When visual surface values are defined in Core or Family tokens:  - Dark mode breaks (e.g. white or high-luminance surfaces in dark themes) - Themes cannot adapt surfaces independently - Visual intent leaks into foundational layers - Recurrent regressions occur when adding new components or variants

### Anti-patterns

- ### ❌ Forbidden

---

## CR-180 — Semantic Token Names Must Follow Canonical Suffixes

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** design-system, tokens
- **Version:** 1.0.0

### Problem

When semantic tokens use non-standard suffixes:  - **Family references token that doesn't exist** - `--semantic-info-surface` not found - **Theme defines correct name** - `--semantic-info-bg` exists - **CSS breaks silently** - component has no background, no error shown - **Maintenance nightmare** -

### Anti-patterns

- ### ❌ Forbidden
- FamilyToken::new("callout-bg-info", "var(--semantic-info-surface)"), // ❌ -surface doesn't exist
- FamilyToken::new("alert-fg-error", "var(--semantic-error-color)"),   // ❌ -color not allowed

---

## CR-181 — Token Cascade Is Architecture, Not Convention

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** architecture
- **Version:** 1.0.0

### Problem

When token cascade is treated as convention:  - **Developers skip layers** - components reference presets directly - **Inconsistent usage** - some files follow cascade, others don't - **Refactoring breaks** - no guarantee layers are respected - **Onboarding confusion** - "should I use `--color-*` or

### Anti-patterns

- ### ❌ Forbidden (treating cascade as suggestion)
- background: hsl(var(--color-primary)); /* ❌ Skips semantic + family */
- "hsl(38 92% 50%)"  // ❌ Skips semantic, hardcodes color

---

## CR-182 — Semantic Role Must Affect Visual Contrast

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** design-system, tokens
- **Version:** 1.0.0

### Problem

When semantically different components share identical backgrounds:  - **User confusion** - can't distinguish feedback from content - **Semantic collapse** - meaning lost in visual uniformity - **Accessibility failure** - no visual hierarchy - **Enterprise UX violation** - feedback must be immediate

### Anti-patterns

- ### ❌ Forbidden
- --semantic-info-bg: hsl(var(--color-background)); /* ❌ Same as surface */
- // true ❌ - Cannot distinguish semantic roles

---

## CR-183 — Overlay Positioning Is a Primitive, Not a Controller Concern

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** architecture, overlays, primitives
- **Version:** 1.0.0

### Problem

Without this rule:  - Overlays appear centered on screen instead of anchored to triggers - Controllers start performing layout calculations - Inline styles leak into runtime DOM - Tooltip, Popover, Dropdown implementations diverge - SSR/CSR hydration breaks due to structural mismatch  Architectural 

### Anti-patterns

- ### ❌ Forbidden

---

## CR-184 — Floating Overlays Expose Position Only via CSS Variables

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** css, primitives, overlays
- **Version:** 1.1.0

### Anti-patterns

- Inline positioning
- DOM mutation of `top`, `left`, `bottom`, `right`

---

## CR-185 — Overlays Must Be Anchor-Relative

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** overlays, primitives
- **Version:** 1.1.0

### Anti-patterns

- Viewport-centered overlays
- Fixed positioning without anchor
- Positioning without trigger reference

---

## CR-186 — Overlay Visibility Is Controlled Only via data-state

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** hydration, css
- **Version:** 1.1.0

### Anti-patterns

- `display: none` set in Rust
- Conditional rendering
- Inline visibility styles

---

## CR-187 — Floating Primitives Enforce CSS-Variable-Only Positioning

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** overlays, primitives, governance
- **Version:** 1.1.0

### Anti-patterns

- element.style().set_property("top", "120px").unwrap(); // ❌

---

## CR-188 — Overlay Positioning Is Not UI Logic

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** primitives, architecture
- **Version:** 1.0.0

### Problem

Without this rule:  - Controllers accumulate geometry logic - UI code becomes unportable - Multiple components reimplement positioning - Bugs multiply with every new overlay  Observable failures:  - Tooltip logic duplicated in multiple files - Dropdown behavior diverges from Popover - Fixes applied 

### Anti-patterns

- ### ❌ Forbidden

---

## CR-191 — Pages Must Delegate Wiring to Page Behaviors Only

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** pages, behavior, architecture
- **Version:** 1.0.0

### Problem

Without this rule:  - Pages start accumulating logic - Page files grow uncontrollably - Behavior usage becomes inconsistent - The distinction between framework behavior and product wiring blurs  Observable failures:  - Event listeners created inside pages - Pages importing framework primitives or be

### Anti-patterns

- ### ❌ Forbidden
- DataTableBehavior::new(...).init(); // ❌ page calling framework behavior
- el.add_event_listener_with_callback(...); // ❌ logic inside page

---

## CR-192 — Page Behaviors Must Not Implement New Logic

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** architecture, behavior, pages
- **Version:** 1.0.0

### Problem

Without this rule:  - Page behaviors slowly turn into controllers - Logic becomes duplicated across pages - Framework behaviors are bypassed - The architecture collapses into ad-hoc scripts  Observable failures:  - Conditional logic inside page behaviors - Geometry, filtering, or sorting logic in pa

### Anti-patterns

- ### ❌ Forbidden
- reposition_overlay(); // ❌ logic
- let filtered = rows.iter().filter(...); // ❌ logic

---

## CR-193 — Framework Behaviors Are the Only Source of Interactivity

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** behavior, ui, architecture
- **Version:** 1.0.0

### Problem

Without this rule:  - Interactivity leaks into pages - Multiple interaction models coexist - UX becomes inconsistent across products  Observable failures:  - Event listeners in pages - UI components handling their own logic - Different behaviors for the same component  Architectural impact:  - Loss 

### Anti-patterns

- ### ❌ Forbidden

---

## CR-194 — UI Components Are Pure Render Functions

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** ui, architecture
- **Version:** 1.0.0

### Problem

Without this rule:  - UI starts executing logic - Rendering becomes stateful - Hydration mismatches occur - Behavior becomes untraceable  Observable failures:  - Effects inside UI components - DOM access during render - Conditional rendering based on runtime state  Architectural impact:  - Broken SS

### Anti-patterns

- ### ❌ Forbidden
- Effect::new(|| { ... }); // ❌ side effect
- let el = document.get_element_by_id(...); // ❌ DOM access

---

## CR-195 — Interactive Components Require Explicit ID

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** components, behavior, hydration
- **Version:** 1.0.0

### Problem

What breaks **without** this rule:  - **Hydration mismatch**: Auto-generated IDs differ between server and client render passes - **Non-deterministic behavior attachment**: Registry cannot reliably match DOM elements to behaviors - **Conditional rendering breaks**: Components rendered conditionally 

### Anti-patterns

- ### ❌ Forbidden

---

## CR-196 — SSR/CSR Separation for WASM Bloat Prevention

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** architecture, build, wasm
- **Version:** 1.0.0

### Problem

What breaks **without** this rule:  - **WASM bloat**: 22MB+ bundle includes entire SSR rendering engine - **Build time explosion**: Compiling HTML rendering logic to WASM unnecessarily - **Runtime overhead**: Loading megabytes of unused code in browser - **Memory waste**: Client holds full component

### Anti-patterns

- ### ❌ Forbidden
- pub mod ui;           // ❌ Compiles to WASM unnecessarily
- pub mod blocks;       // ❌ HTML rendering in client

---

## CR-197 — Feature Flags Are Architectural Boundaries

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** architecture, build
- **Version:** 1.0.0

### Problem

Misusing feature flags causes:  - SSR code compiled into WASM - CSR code linked into server binaries - Duplicate runtime crates - Non-deterministic build graphs  Observed failures: - `axum::Body` type mismatches - Multiple `axum-core` versions - Hydration panics - WASM bloat  ---

### Anti-patterns

- `cfg(any(feature = "ssr", feature = "hydrate"))`
- Runtime logic inside shared crates
- Feature flags used to “hide” architecture violations

---

## CR-198 — Runtime Crates Must Not Leak Cross-Target Types

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** architecture, workspace
- **Version:** 1.0.0

### Problem

When runtime types leak:  - Multiple versions of the same crate are linked - Type identity breaks (`axum::Body ≠ leptos::Body`) - Errors surface deep in dependency graphs - Integration becomes fragile  ---

### Anti-patterns

- Shared crates depending on `axum`, `leptos`, `web-sys`
- Returning runtime-specific types from framework APIs
- Passing runtime objects across crate boundaries

---

## CR-199 — Server Adapters Are Integration Code

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** architecture, ssr
- **Version:** 1.0.0

### Problem

When adapters leak into framework code:  - Framework becomes locked to one server - Migration cost explodes - Runtime types propagate incorrectly - Build graphs destabilize  ---

### Anti-patterns

- Framework crates importing `axum::*`
- UI or behaviors referencing server extractors
- Adapter-specific types in shared APIs

---

## CR-200 — Shared Crates Must Be Zero-Dependency Runtime

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** architecture, workspace
- **Version:** 1.0.0

### Anti-patterns

- DOM access
- Server adapters
- Async runtimes

---

## CR-201 — Presets Are the Only Source of Color Values

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** design-system, tokens, theming
- **Version:** 1.0.0

### Problem

When themes or semantic tokens define colors directly:  - **Preset is overridden** - palette switching breaks - **Light mode inconsistency** - different colors than dark - **Maintenance nightmare** - color changes require editing multiple files - **Cascade violations** - override order becomes unpre

### Anti-patterns

- ### ❌ Forbidden
- --semantic-action-primary-bg: hsl(210 40% 98%); /* ❌ Hardcoded color */
- --semantic-surface-bg: hsl(0 0% 100%); /* ❌ Hardcoded white */

---

## CR-202 — Themes Resolve Context, Not Palette

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** tokens, theming
- **Version:** 1.0.0

### Problem

When themes define color values instead of referencing presets:  - **Preset switching breaks** - theme hardcoded values override preset - **Duplicate truth** - same color defined in preset AND theme - **Semantic drift** - `--semantic-action-primary-bg` means different things in different contexts - 

### Anti-patterns

- ### ❌ Forbidden
- /* ❌ Theme defining color value */

---

## CR-203 — Semantic Tokens Are the Only Bridge Between Theme and Families

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** design-system, tokens, components
- **Version:** 1.0.0

### Problem

When families or components reference preset/theme tokens directly:  - **Context is lost** - component doesn't know "why" it's using a color - **Theme switching breaks** - components hardwired to specific palette - **Semantic drift** - "primary" means different things in different components - **Ref

### Anti-patterns

- ### ❌ Forbidden
- --button-primary-bg: hsl(var(--color-primary)); /* ❌ Skips semantic layer */
- background: hsl(var(--color-primary)); /* ❌ Component directly using preset */

---

## CR-204 — Theme Files Must Be Last in the Cascade

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** build, css
- **Version:** 1.0.0

### Problem

When theme files are imported early in the cascade:  - **Overrides don't apply** - subsequent imports overwrite theme values - **Context resolution fails** - semantic tokens defined after theme win - **Light/dark broken** - theme class has no effect - **Debugging nightmare** - values appear correct 

### Anti-patterns

- ### ❌ Forbidden
- @import "./.generated/themes.css";          /* ❌ Theme too early */

---

## CR-205 — UI Tokens Must Bind to Semantic Tokens, Never to Presets

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** components, tokens
- **Version:** 1.0.0

### Problem

When family tokens reference presets directly:  - **Context is bypassed** - theme light/dark has no effect - **Semantic meaning is lost** - "primary" becomes "color X" instead of "action intent" - **Refactoring breaks** - changing semantic layer doesn't propagate - **Testing impossible** - can't ver

### Anti-patterns

- ### ❌ Forbidden
- /* ❌ Family directly referencing preset */
- "var(--color-primary)"  // ❌ Skips semantic layer

---

## CR-206 — UI Inventory Is Fixed and Canonical

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** design-system, ui, governance
- **Version:** 1.1.0

### Anti-patterns

- ### ❌ Forbidden

---

## CR-207 — Preset Switching Must Never Change Component CSS

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** architecture, tokens, components
- **Version:** 1.0.0

### Problem

When preset changes require component edits:  - **Not scalable** - can't ship multiple presets to users - **Build-time coupling** - preset becomes a compile-time constant - **User theming impossible** - users can't customize colors - **Maintenance explosion** - adding preset = editing components  Re

### Anti-patterns

- ### ❌ Forbidden
- background: hsl(38 92% 50%);  /* ❌ Amber hardcoded */

---

## CR-208 — A Working Theme Toggle Is Proof of Correct Token Architecture

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** governance, architecture
- **Version:** 1.0.0

### Problem

When theme toggle causes visual bugs:  - **Token cascade is broken** - values not resolving correctly - **Semantic layer incomplete** - tokens missing for some contexts - **Component coupling** - components hardcoded to one mode - **Theme overrides wrong** - cascade order incorrect  Real discovery: 

### Anti-patterns

- ### ❌ Forbidden (when debugging toggle issues)
- // ❌ Attempting to fix visual bugs in JavaScript
- // ❌ Manually forcing colors (workaround for broken tokens)

---

## CR-209 — Axum Version Must Match Adapter Version

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** ssr, build
- **Version:** 1.0.0

### Problem

When Axum versions diverge between the application and the adapter layer:  - Type mismatches between identical-looking types - `Body` incompatibilities at compile time - Multiple `axum_core` versions in the dependency graph - Non-obvious, hard-to-debug build failures  This breaks SSR routing and str

### Anti-patterns

- ### ❌ Forbidden

---

## CR-210 — cdylib Is Mandatory for Hydration

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** build, wasm, hydration
- **Version:** 1.0.0

### Problem

Without `cdylib`:  - WASM artifacts are not generated - `wasm-bindgen` fails late - Hydration silently breaks - Build errors appear far from the root cause  This creates false-positive “successful” builds.  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-211 — SSR Meta Requires Explicit HTML Shell

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** ssr, ui
- **Version:** 1.0.0

### Problem

Without an explicit shell:  - Runtime panics occur - Meta tags cannot resolve a `<head>` - HTML structure becomes ambiguous - Errors surface only at runtime  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-212 — Hydration Bootstrap Is Tooling-Owned

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** hydration, wasm
- **Version:** 1.0.0

### Problem

Manual bootstrap:  - Conflicts with `cargo-leptos` - Breaks tool-controlled lifecycle - Causes double initialization - Produces non-deterministic behavior  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-213 — Streaming SSR Requires Explicit Executor Initialization

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** ssr, state
- **Version:** 1.0.0

### Problem

Without executor initialization:  - Runtime panics occur - Errors are silent or misleading - Streaming hangs or crashes - Debugging becomes impractical  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-214 — lib.rs Owns Application Structure and UI Semantics

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** architecture, ssr, csr
- **Version:** 1.0.0

### Problem

When `lib.rs` is treated as a secondary or optional file:  - UI logic leaks into `main.rs` - SSR and CSR responsibilities blur - Meta configuration becomes inconsistent - Hydration and routing behavior diverge - The application loses a single semantic root  This leads to architectural drift and non-

### Anti-patterns

- ### ❌ Forbidden

---

## CR-215 — main.rs Owns Runtime and Server Bootstrap Exclusively

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** ssr, architecture
- **Version:** 1.0.0

### Problem

When `main.rs` contains application logic:  - Build features become entangled - SSR/CSR branches become unsafe - UI logic is duplicated or conditionally compiled - Bootstrap order becomes fragile  This breaks determinism and makes the application untestable in isolation.  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-216 — App Components Must Not Define HTML Structure

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** ui, ssr
- **Version:** 1.0.0

### Problem

When components define HTML structure:  - Meta injection breaks - Hydration becomes undefined - Multiple head/body contexts appear - SSR output becomes invalid HTML  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-217 — Entry Points Must Be Explicit, Public, and Isolated

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** ssr, csr, build
- **Version:** 1.0.0

### Problem

Implicit or private entrypoints cause:  - Unresolvable imports - Feature flag leakage - Build-time ambiguity - Tooling incompatibility  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-218 — Legacy Rendering APIs Are Forbidden

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** ssr
- **Version:** 1.0.0

### Problem

Using legacy APIs:  - Bypasses canonical shell handling - Breaks meta integration - Causes hydration mismatches - Locks architecture to obsolete patterns  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-219 — Leptos Product Must Declare Consistent Ports Across Workspace and Local Config

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** leptos, workspace
- **Version:** 1.0.0

### Problem

Without port consistency: - Server starts on wrong port (e.g., 3000 vs 3004) - Hot reload connects to wrong port, fails silently - Developer confusion: "cargo leptos serve" uses different port than "cargo run" - CI/CD deploys to unexpected ports  **Observable symptoms**: ``` cargo leptos serve 🚀 Lis

### Anti-patterns

- ### ❌ Forbidden
- site-addr = "127.0.0.1:3004"  # ❌ DIVERGENT
- reload-port = 3005             # ❌ DIVERGENT

---

## CR-220 — Workspace Metadata Must Define Unique Build Targets Per Product

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** leptos, workspace, build
- **Version:** 1.0.0

### Problem

Without unique build targets: - Products overwrite each other's outputs - `target/site/pkg/` contains mixed WASM from multiple products - Hot reload serves wrong product's assets - CI artifacts are corrupted (mixed binaries)  **Observable symptoms**: ```bash cargo leptos build --package canonrs-site

### Anti-patterns

- # Output: target/site/  ❌ OVERWRITES PREVIOUS
- canonrs_workbench.wasm  # ❌ Where is canonrs_site.wasm?
- ### ❌ Forbidden

---

## CR-221 — Leptos.toml Critical Fields Must Align With Workspace Metadata

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** leptos, workspace
- **Version:** 1.0.0

### Problem

Without alignment on critical fields: - Features diverge: SSR builds with hydrate features, or vice-versa - Ports mismatch (covered by Rule #219, but reinforced here) - Build flags inconsistent between `cargo leptos build` and `cargo build --features`  **Observable symptoms**: ```rust // Workspace s

### Anti-patterns

- ### ❌ Forbidden
- bin-features = ["ssr", "debug"]  # ❌ DIVERGENT
- lib-features = ["hydrate", "csr"] # ❌ DIVERGENT

---

## CR-222 — Workspace Members Must Be Fully Resolvable by Cargo Metadata

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** workspace, build
- **Version:** 1.0.0

### Problem

Without resolvable members: - `cargo build --workspace` fails silently or partially - IDE tooling (rust-analyzer) cannot find crates - `cargo test --workspace` skips broken members - Dependency graph is incomplete, causing version conflicts  **Observable symptoms**: ```bash cargo metadata --format-v

### Anti-patterns

- ### ❌ Forbidden
- "canonrs-site",                    # ❌ Missing 'products/' prefix
- "packages-rust/canonrs-ssr",       # ❌ Missing 'rs-canonrs/' level

---

## CR-223 — Feature Flag Scopes Must Not Leak Between Products

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** leptos, workspace
- **Version:** 1.0.0

### Problem

Without feature isolation: - SSR packages (`canonrs-ssr`) compile with hydrate features - Hydrate builds include server-only dependencies (tokio, axum) - WASM binaries bloated with unused SSR code - Violates Rule #196 (SSR/CSR separation) at workspace level  **Observable symptoms**: ```bash cargo bu

### Anti-patterns

- # ❌ SSR package compiling with CSR features
- # ❌ WASM build includes tokio (SSR-only dep)
- ### ❌ Forbidden

---

## CR-224 — Workspace Package Names Must Use Hyphens Consistently

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** workspace, build
- **Version:** 1.0.0

### Problem

Without hyphen consistency: - cargo-leptos searches for `canonrs-site` but finds `canonrs_site` - Binary outputs have unpredictable names (`canonrs_site` vs `canonrs-site`) - `use canonrs_site::App` vs `canonrs-site` package name creates confusion - CI scripts fail with "file not found"  **Observabl

### Anti-patterns

- canonrs_site  # ❌ Name mismatch
- ### ❌ Forbidden
- name = "canonrs_site"           # ❌ UNDERSCORE

---

## CR-225 — Workspace Dependency Version Policy Must Prevent ABI Drift Without Blocking Patch Compatibility

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** workspace, build
- **Version:** 2.0.0

### Problem

Using wide semver ranges:  ```toml axum = "0.8" leptos = "0.8" ```  allows Cargo to resolve different patch versions across the graph:  ``` axum 0.8.7 axum 0.8.9 ```  This causes:  - Type mismatches across crate boundaries - Duplicate symbol linkage - Subtle ABI conflicts - Non-deterministic CI vs l

### Anti-patterns

- ### ❌ Wide semver ranges
- ### ❌ Exact-everything freeze (over-constrained)

---

## CR-226 — Workspace Leptos Paths Must Be Root-Relative

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** leptos, workspace
- **Version:** 1.0.0

### Problem

Without root-relative paths: - CSS files return 404 (cargo-leptos runs from workspace root) - Assets not found during build - Works locally (if in product dir) but fails in CI - Hot reload serves wrong files  **Observable symptoms**: ```bash cd /opt/docker/monorepo  # Workspace root cargo leptos ser

### Anti-patterns

- ### ❌ Forbidden
- style-file = "style/output.css"      # ❌ Relative to where?
- assets-dir = "public"                 # ❌ Ambiguous

---

## CR-227 — Products Must Inherit Workspace Dependencies

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** workspace, build
- **Version:** 1.0.0

### Problem

Without workspace inheritance: - Version drift: Products use different versions than workspace declares - Duplicate dependency declarations cause conflicts - Updates require changing multiple Cargo.toml files - Violates DRY (Don't Repeat Yourself)  **Observable symptoms**: ```bash cargo tree -p cano

### Anti-patterns

- ### ❌ Forbidden
- leptos = "=0.8.15"              # ❌ DUPLICATED
- axum = { version = "=0.8.8", optional = true }  # ❌ DUPLICATED

---

## CR-228 — Workspace Must Define WASM Target Configuration

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** wasm, workspace, build
- **Version:** 1.0.0

### Problem

Without WASM target configuration: - `cargo build --target wasm32-unknown-unknown` fails with "target not found" - WASM builds use default optimization (bloated output) - Inconsistent builds between developers - CI fails on WASM compilation  **Observable symptoms**: ```bash cargo build --target wasm

### Anti-patterns

- ### ❌ Forbidden
- echo "❌ .cargo/config.toml not found"
- echo "❌ WASM target not configured"

---

## CR-229 — Design System CSS Must Be Consumed as an Independent Artifact

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** css, build, design-system
- **Version:** 1.0.0

### Problem

When design system CSS is merged into, imported by, or rebuilt inside product pipelines:  - Products become coupled to internal CanonRS paths - Design system updates force product rebuilds - Utility frameworks (Tailwind) gain authority over design tokens - Build graphs become cyclic or implicit - Ve

### Anti-patterns

- ### ❌ Forbidden
- echo "❌ CanonRS CSS must not be imported into Tailwind"
- echo "❌ Products must not reference CanonRS internal paths"

---

## CR-230 — WASM Artifact Budget Is a Hard Contract

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** build, wasm
- **Version:** 1.0.0

### Problem

Without size budgets:  - WASM bundles grow silently from 2MB → 143MB (71x regression, real incident) - Performance degradation goes unnoticed until production - Client devices download bloated bundles - No early warning system for architectural violations - Developers unaware of cross-compilation le

### Anti-patterns

- ### ❌ Forbidden: No Size Guard
- # ❌ No size validation
- echo "❌ CANON VIOLATION: WASM exceeds budget"; \

---

## CR-231 — SSR Crates Must Be Link-Time Isolated From WASM Graph

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** interactive, wasm, build
- **Version:** 1.1.0

### Anti-patterns

- compile_error!("❌ SSR crate compiled for WASM target");
- echo "❌ SSR dependency leaked into WASM graph"
- ### ❌ Single-bin architecture

---

## CR-232 — Product Builds Must Be Deterministic From Workspace Root

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** build
- **Version:** 1.0.0

### Problem

Without deterministic builds:  - "Works on my machine" syndrome - CI failures due to path assumptions - Flaky builds depending on shell history - Makefiles assume wrong working directory - Relative imports break unpredictably  ```bash cd /workspace/products/app && make build  ✅  cd /workspace && mak

### Anti-patterns

- cd /workspace && make -C products/app build  ❌
- ### ❌ Forbidden: CWD-Dependent Paths
- # ❌ Breaks if cwd ≠ expected

---

## CR-233 — CSS Cascade Order Is Build-Enforced Architecture

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** css, build
- **Version:** 1.0.0

---

## CR-234 — Tokens Engine Is the Single Source of CSS Truth

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** tokens, css, architecture
- **Version:** 1.0.0

### Anti-patterns

- --color-background: hsl(0 0% 100%); /* ❌ Manual token */
- --color-primary: red; /* ❌ Tailwind controlling tokens */

---

## CR-235 — UI Layer Must Consume Semantic Tokens, Never Theme Directly

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** tokens, ui, theming
- **Version:** 1.0.0

### Anti-patterns

- background: var(--theme-surface-bg); /* ❌ Theme direct usage */
- background: hsl(0 0% 100%); /* ❌ Raw color */

---

## CR-236 — CanonRS CLI Defines Workspace Topology

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** cli, workspace, architecture
- **Version:** 1.0.0

### Anti-patterns

- "products/new-app"  # ❌ added manually
- mkdir products/new-app   # ❌ outside CLI
- name = "new-app"  # ❌ manual entry

---

## CR-237 — CLI Owns Leptos Metadata Blocks

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** cli, workspace, leptos
- **Version:** 1.0.0

### Problem

Manual editing of leptos metadata causes:  - Port divergence (Rule #219) - site-root collisions (Rule #220) - Feature leaks (Rule #223) - Path errors (Rule #226) - Hydration mismatches - Build nondeterminism  Leptos metadata defines runtime architecture, not convenience config.  ---

### Anti-patterns

- site-addr = "127.0.0.1:3009"  # ❌ manual change

---

## CR-238 — Workspace Dependency Graph Must Be Acyclic

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** architecture, workspace
- **Version:** 1.0.0

### Problem

Circular dependencies cause:  - Incremental build instability - Cargo resolution errors - Implicit feature activation - Architectural boundary violations - Logical layer inversion (UI importing products, etc.)  ---

### Anti-patterns

- canonrs-site → canonrs-ui   ❌ cycle
- A → B → C → A  ❌

---

## CR-239 — CLI Sync Operations Must Be Idempotent

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** cli, workspace
- **Version:** 1.0.0

### Problem

Non-idempotent generators cause:  - Git noise - Infinite reformat loops - CI instability - Phantom diffs - Timestamp pollution - Rebuild cascades  ---

### Anti-patterns

- git diff → different changes again  ❌
- echo "❌ CLI not idempotent"

---

## CR-240 — Generated Artifacts Must Not Be Committed

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** build, cli, tokens
- **Version:** 1.0.0

### Problem

Committing generated files causes:  - Merge conflicts - Diff pollution - Version skew - Stale artifacts - False architectural drift  ---

---

## CR-241 — Tokens Engine Is Build Infrastructure, Not Runtime Code

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** tokens, architecture
- **Version:** 1.0.0

### Problem

If tokens-engine becomes runtime:  - Product crates depend on build tooling - Runtime binaries grow incorrectly - Architectural layering collapses - Feature flags leak - Workspace graph becomes unstable  ---

---

## CR-242 — Cascade Order Is a Contract, Not a Preference

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** css, tokens, architecture
- **Version:** 1.0.0

### Problem

Changing cascade order causes:  - Semantic override failures - Theme leakage - Token shadowing - Visual instability - Undetectable regressions  ---

### Anti-patterns

- @import "./.generated/semantic.css";  ❌ wrong order

---

## CR-243 — data-theme Is the Only Theme Activation Boundary

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** css, theming, architecture
- **Version:** 1.0.0

### Problem

Multiple activation mechanisms cause:  - Theme leakage - Partial overrides - Conflicting scopes - Unpredictable dark behavior  ---

### Anti-patterns

- :root { --theme-bg: ... }          ❌
- body.dark { ... }                  ❌
- html[data-theme=...] .component    ❌

---

## CR-244 — Semantic Layer Is a Mandatory Abstraction

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** tokens, css, architecture
- **Version:** 1.0.0

### Problem

Direct theme consumption causes:  - Tight coupling to preset vocabulary - Impossible re-theming - Architectural collapse - Token naming chaos  ---

### Anti-patterns

- background: var(--theme-surface-bg); ❌ inside UI
- border: var(--theme-border);         ❌ inside component

---

## CR-245 — Families Must Not Leak Global State

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** tokens, architecture
- **Version:** 1.0.0

### Problem

If families override globals:  - State becomes unpredictable - Visual contract breaks - Theme switching becomes unstable - Cascade loses determinism  ---

---

## CR-246 — CSS Bundle Must Be Layer-Free

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** css, build
- **Version:** 1.0.0

### Problem

If @layer or @import leaks into the bundle:  - CSS order becomes tool-dependent - Runtime cascade becomes unstable - Products gain implicit authority over CanonRS - Tailwind or other processors alter system hierarchy  ---

### Anti-patterns

- @layer components { ... }   ❌
- @import "./something.css";  ❌

---

## CR-247 — CSS Entry Order Is Architectural, Not Cosmetic

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** css, tokens
- **Version:** 1.0.0

### Problem

Changing order causes:  - Semantic shadowing - Root override loss - UI consuming unstable tokens - Non-deterministic visual output  ---

### Anti-patterns

- @import "./.generated/semantic.css";  ❌ wrong order

---

## CR-248 — Tokens Engine Is the Only Source of Truth

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** build, tokens, governance
- **Version:** 1.0.0

### Problem

Manual edits create:  - Drift between Rust token definitions and CSS output - Irreproducible builds - Impossible diff tracking - Silent regression during rebuild  ---

---

## CR-249 — Products Must Not Depend on .generated CSS

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** css, architecture
- **Version:** 1.0.0

### Problem

When products reference internal CSS layers:  - They become coupled to CanonRS internals - Directory refactors break products - Token evolution becomes blocked - Versioning becomes impossible  ---

### Anti-patterns

- <link rel="stylesheet" href="../../rs-canonrs/styles/.generated/root.css">  ❌
- <link rel="stylesheet" href="../../rs-canonrs/styles/canonrs.css">         ❌

---

## CR-250 — Design System Must Not Depend on Products

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** workspace, architecture
- **Version:** 1.0.0

### Problem

Reverse dependency creates:  - Cycles in graph - Hidden feature leakage - Build instability - Impossible artifact isolation  ---

---

## CR-251 — All Build Artifacts Must Be Statically Identifiable

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** build
- **Version:** 1.0.0

### Problem

Dynamic artifact naming causes:  - Cache corruption - Hard-to-debug mismatches - Deployment confusion - CI instability  ---

---

## CR-252 — Theme Switching Must Be Attribute-Driven

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** css, theming
- **Version:** 1.0.0

### Problem

Imperative theme systems cause:  - Inline style pollution - Hydration mismatches - CSS specificity conflicts - Impossible debugging  ---

---

## CR-253 — Token Cascade Order Is Immutable

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** css, tokens, architecture
- **Version:** 1.0.0

### Problem

Changing cascade order:  - Breaks variable resolution - Produces silent style regressions - Invalidates semantic mapping guarantees  ---

---

## CR-254 — CanonRS Must Build Independently of Products

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** build, design-system
- **Version:** 1.0.0

### Problem

If CanonRS depends on product context:  - Circular dependency appears - Artifact governance breaks - CLI orchestration becomes mandatory - Design system cannot be versioned independently  ---

---

## CR-255 — CLI Is the Sole Authority Over Workspace Generation

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** cli, workspace
- **Version:** 1.0.0

### Anti-patterns

- Editing `.canonrs/workspace/Cargo.toml` manually
- Committing workspace topology changes directly
- Defining workspace metadata in product Cargo.toml

---

## CR-256 — Generated Workspace Is Ephemeral and Immutable

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** workspace, governance
- **Version:** 1.0.0

### Anti-patterns

- Adding workspace to git
- Editing generated files
- Injecting manual build flags

---

## CR-257 — Tokens Engine Is a Mandatory Pre-Build Step

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** build, css, tokens
- **Version:** 1.0.0

---

## CR-258 — Mode Drives Build Profiles

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** cli, build
- **Version:** 1.0.0

---

## CR-259 — Products Must Not Define Build Topology

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** architecture, build
- **Version:** 1.0.0

### Anti-patterns

- Defining `[workspace]` inside product
- Overriding leptos metadata
- Custom target management

---

## CR-260 — CLI Autodiscovery Must Be Explicit or Fail

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** cli
- **Version:** 1.0.0

### Anti-patterns

- Hardcoded filesystem fallbacks
- Implicit default paths
- Silent misconfiguration

---

## CR-261 — CSS Bundle Size Drift Must Be Monitored

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** css, build
- **Version:** 1.0.0

### Problem

Silent CSS growth leads to:  - Unbounded token expansion - Duplicate families - Forgotten experimental layers - Performance regressions  ---

### Anti-patterns

- echo "❌ CSS exceeds expected budget"

---

## CR-262 — No Runtime CSS Regeneration

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** build, architecture
- **Version:** 1.0.0

### Problem

Runtime generation introduces:  - Non-deterministic styling - Hydration mismatches - Debug vs release divergence - Impossible caching guarantees  ---

---

## CR-263 — Workspace Crate Boundaries Must Be Explicitly Declared

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** workspace, architecture
- **Version:** 1.0.0

### Problem

Without declared architectural roles:  - Crates drift in responsibility - Dependency layering becomes unclear - Cyclic coupling risk increases - Refactors break invisible contracts  ---

---

## CR-264 — Behavior Registry Is the Single Runtime Entry Point

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** behavior, wasm, build
- **Version:** 1.0.0

### Problem

Without this rule:  - Multiple init systems compete - Behaviors attach twice - Order-dependent bugs appear - MutationObserver conflicts - Inconsistent CSR activation  Observable symptoms:  - `init_x()` called manually per component - Different apps registering behaviors differently - Duplicate liste

### Anti-patterns

- ### ❌ Forbidden

---

## CR-265 — Interactive Owns All State — Behavior Owns None

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** interactive, behavior, state
- **Version:** 1.0.0

### Problem

Without this rule:  - Behavior holds hidden state - State diverges between SSR and CSR - Debugging becomes impossible - Hydration mismatches appear  Observable symptoms:  - `Rc<RefCell<...>>` storing business logic - Sorting state in behavior - Pagination index stored in behavior - Filter strings st

### Anti-patterns

- ### ❌ Forbidden

---

## CR-266 — Primitives Must Never Generate IDs

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** primitives, build
- **Version:** 1.0.0

### Problem

Without this rule:  - SSR and CSR ID mismatch - Non-deterministic markup - Hydration warnings - Broken behavior attachment  Observable symptoms:  - `format!("button-{}", counter)` - Static mut counters - Atomic counters in primitives - Random UUID generation  Architectural impact:  - Hydration insta

### Anti-patterns

- ### ❌ Forbidden

---

## CR-267 — Data Attributes Are Contract, Not Styling Mechanism

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** primitives, ui, interactive
- **Version:** 1.0.0

### Problem

Without this rule:  - CSS and architecture couple incorrectly - Classes abused for state contracts - Visual variants leak into runtime logic - Style and behavior semantics blur  Observable symptoms:  - `class="collapsed"` - `class="variant-solid"` - Using class to represent feature activation  Archi

### Anti-patterns

- ### ❌ Forbidden

---

## CR-268 — Behavior Cleanup Is Mandatory and Deterministic

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** behavior, wasm
- **Version:** 1.0.0

### Problem

Without deterministic cleanup:  - Memory leaks in SPA navigation - Duplicate listeners - Intermittent runtime bugs - Event storms - Hard-to-reproduce state corruption  ---

### Anti-patterns

- ### ❌ Forbidden

---

## CR-269 — Custom Events Are Public Runtime Contracts

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** behavior, interactive
- **Version:** 1.0.0

### Problem

Without formal contract:  - Breaking rename breaks Interactive - Hidden coupling - Silent behavior failures  ---

---

## CR-270 — Behavior Must Never Traverse Beyond Its Container Scope

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** behavior
- **Version:** 1.0.0

### Problem

Global DOM queries cause:  - Cross-component interference - Coupling - Unpredictable runtime effects  ---

---

## CR-271 — Behavior Execution Must Be Idempotent

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** behavior
- **Version:** 1.0.0

### Problem

Hot reload may re-register behavior.  Without idempotence:  - Duplicate listeners - Multiple event triggers  ---

---

## CR-272 — Global Mutable State Is Forbidden in wasm Scope

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** behavior
- **Version:** 1.0.0

### Problem

Global state causes:  - Race conditions - Hidden coupling - Memory leaks  ---

---

## CR-273 — MutationObserver Requires Explicit Architectural Justification

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** behavior
- **Version:** 1.0.0

### Problem

Uncontrolled observers cause:  - Performance degradation - Layout thrashing  ---

---

## CR-274 — Interactive Features Must Be Strictly Isolated

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** interactive
- **Version:** 1.0.0

### Problem

Cross-feature mutation leads to:  - Hidden dependencies - Regression chains  ---

---

## CR-275 — Interactive Hooks Must Be Container-Bound

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** interactive
- **Version:** 1.0.0

### Problem

Global hook attachment breaks isolation.  ---

---

## CR-276 — Interactive Must Remain Deterministic Under SSR Without wasm

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** interactive
- **Version:** 1.0.0

### Problem

SSR mismatch causes hydration errors.  ---

---

## CR-277 — Interactive Must Not Emit DOM Events

- **Severity:** HIGH
- **Status:** ENFORCED
- **Scope:** interactive
- **Version:** 1.0.0

### Problem

Violates separation between runtime and state orchestration.  ---

---

## CR-278 — Primitive Must Not Encode Design Tokens in Rust

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** css, primitives, tokens
- **Version:** 1.0.0

### Problem

When tokens leak into Rust:  - Token cascade is broken - Theming becomes impossible - Structural layer becomes presentation layer - Build-time guarantees are lost  ---

---

## CR-279 — UI Layer Must Not Be Aware of Behavior Layer

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** behavior, ui, architecture
- **Version:** 1.0.0

### Problem

If UI references Behavior:  - Layer boundaries collapse - SSR determinism risks increase - Runtime coupling emerges - Testability degrades  ---

---

## CR-280 — Interactive Must Be Safe Under Feature Flag Removal

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** interactive, build
- **Version:** 1.0.0

### Problem

Feature coupling causes:  - Hidden rendering differences - Inconsistent DOM trees - Hydration mismatches - Regression chains  ---

---

## CR-281 — Cross-Crate Layer Leakage Is Forbidden

- **Severity:** CRITICAL
- **Status:** ENFORCED
- **Scope:** workspace, architecture
- **Version:** 1.0.0

### Problem

Cross-layer leaks create:  - Hidden tight coupling - Refactor paralysis - Broken semver guarantees - Implicit contracts  ---

---

