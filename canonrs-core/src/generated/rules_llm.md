# CanonRS — Canon Rules

> AUTO-GENERATED — do not edit manually

---

## CR-001 — Component Types

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

components mix state, effects, and browser APIs without clear classification

### Solution

classify components into pure, stateful, or interactive with strict rules

### Signals

- hydration errors
- unsafe effects
- browser api panic

---

## CR-002 — Ownership Rules

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

improper ownership and reactive patterns break closures and rendering

### Solution

use StoredValue, ChildrenFn, and move closures for correct ownership and reactivity

### Signals

- move error
- fnonce error
- stale data

---

## CR-003 — Lists and Iteration

- **Category:** state-reactivity
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

using map and collect_view breaks ownership and hydration in lists

### Solution

use component isolation with For and place StoredValue inside components

### Signals

- fnonce error
- hydration mismatch
- moved callback

---

## CR-004 — Anti-Hydration Rules

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

ssr and client render different html structures causing hydration failure

### Solution

ensure identical ssr and client output using suspense, isolated components, and stable props

### Signals

- hydration panic
- text node error
- dom mismatch

---

## CR-005 — SSR Effects and Browser API Safety

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

browser apis and effects execute on server causing runtime failures

### Solution

guard all browser and async logic with wasm cfg conditions

### Signals

- window undefined
- spawn_local panic
- context not found

---

## CR-006 — Visual State Declaration

- **Category:** design-system
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

components mix state and styling causing theme and css inconsistencies

### Solution

expose state via data attributes and delegate visuals to css tokens

### Signals

- theme break
- dark mode fail
- style drift

---

## CR-007 — Theme and Token Governance

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

apps directly depend on design system tokens causing coupling and inconsistency

### Solution

enforce token distribution via dedicated bridge and semantic token usage

### Signals

- build break
- design drift
- theme inconsistency

---

## CR-008 — Overlay Islands (Client-Only Architecture)

- **Category:** core-runtime
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

SSR and client render produce different DOM structures when dynamic lists are inside overlays

### Solution

use runtime browser detection to isolate client-only overlay rendering

### Signals

- hydration panic
- dom mismatch warning
- dropdown breaks after load

---

## CR-009 — Clipboard and Browser APIs

- **Category:** behavior
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

clipboard api fails in callbacks due to focus and async constraints

### Solution

use document execcommand with wasm guard for reliable clipboard operations

### Signals

- notallowederror
- document not focused
- clipboard fail

---

## CR-010 — Modal Reactive State Management

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

modal props are static and do not react to parent state changes

### Solution

use signal props with local state and guarded effects for synchronization

### Signals

- stale data
- no update
- reactivity lost

---

## CR-011 — Multi-Callback Ownership

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

values moved into one callback cannot be reused in others

### Solution

clone values before each callback to ensure independent ownership

### Signals

- use of moved value
- e0382 error
- closure failure

---

## CR-012 — canon-rule-12-select-vs-combobox

- **Category:** component-architecture
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

select and combobox are used interchangeably causing architectural issues

### Solution

choose explicitly between select and combobox based on requirements

### Signals

- wrong component choice
- performance issue
- ux mismatch

---

## CR-013 — canon-rule-13-specialization-vs-substitution

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

specialized components are used to replace base components instead of extending them

### Solution

keep base components generic and create explicit specialized components that extend semantics

### Signals

- god component
- magic flags
- component overload

---

## CR-014 — canon-rule-14-datatable-vs-virtualtable

- **Category:** component-architecture
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

datatable and virtualtable are used interchangeably without considering architectural constraints

### Solution

choose explicitly based on scale semantics ssr and performance requirements

### Signals

- performance issue
- ssr break
- wrong component

---

## CR-015 — canon-rule-15-pagination-vs-virtualization

- **Category:** component-architecture
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

pagination and virtualization are combined or misused causing architectural conflicts

### Solution

choose one strategy explicitly based on ux navigation or performance needs

### Signals

- mixed strategies
- performance issue
- seo conflict

---

## CR-016 — canon-rule-16-client-vs-server-filtering

- **Category:** state-reactivity
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

filtering strategy is chosen incorrectly or mixed causing performance and data issues

### Solution

choose filtering strategy explicitly based on data location and scale

### Signals

- slow filtering
- overfetching
- api overload

---

## CR-017 — canon-rule-17-human-vs-machine-scale

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

components are used outside their intended scale causing performance or complexity issues

### Solution

choose components explicitly based on data scale and cognitive constraints

### Signals

- slow rendering
- over engineering
- wrong component

---

## CR-018 — Client vs Server

- **Category:** state-reactivity
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

sorting is performed in wrong layer causing performance or scalability issues

### Solution

sort data where it resides based on dataset size and architecture

### Signals

- slow sorting
- overload
- inefficient queries

---

## CR-019 — canon-rule-19-streaming-vs-snapshot

- **Category:** state-reactivity
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

data delivery model is misused causing inefficiency and complexity

### Solution

choose streaming or snapshot based on data update frequency and requirements

### Signals

- polling misuse
- latency
- over complexity

---

## CR-020 — canon-rule-20-realtime-vs-eventual

- **Category:** state-reactivity
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

real time and eventual consistency are misused causing incorrect ux and system design

### Solution

choose real time or eventual consistency explicitly based on guarantees required

### Signals

- polling misuse
- latency issue
- over engineering

---

## CR-021 — Canonical Color Tokens vs Semantic Intents

- **Category:** design-system
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

non canonical color tokens are used causing inconsistency and incompatibility

### Solution

use only canonical token set and map semantic intents in application layer

### Signals

- token drift
- variant misuse
- design inconsistency

---

## CR-022 — Tailwind v4 + Rust Integration

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

tailwind jit fails to parse rust syntax causing missing styles

### Solution

predefine all utilities in css and avoid arbitrary values in rust code

### Signals

- missing utility
- css not generated
- jit fail

---

## CR-023 — State Tokens (Hover, Focus, Disabled, Pressed)

- **Category:** design-system
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

state styles rely on color variants causing inconsistency across themes

### Solution

use opacity based state tokens and consistent variables for all states

### Signals

- state inconsistency
- hover mismatch
- theme break

---

## CR-024 — Density & Size Scaling

- **Category:** design-system
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

component sizes and density are inconsistent causing layout issues

### Solution

use modular scale and density driven tokens for all sizing

### Signals

- layout inconsistency
- size mismatch
- ui drift

---

## CR-025 — Theme Presets Contract

- **Category:** design-system
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

themes override non color system properties causing inconsistency and fragmentation

### Solution

restrict themes to defining color palettes only and enforce system wide constants for all other properties

### Signals

- theme drift
- inconsistent spacing
- design fragmentation

---

## CR-026 — Elevation & Shadow System

- **Category:** design-system
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

shadow definitions vary across components causing inconsistent elevation hierarchy

### Solution

use canonical shadow tokens and fixed elevation scale across all components

### Signals

- shadow inconsistency
- z index conflict
- visual hierarchy break

---

## CR-027 — Motion & Timing Tokens

- **Category:** design-system
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

animations use hardcoded timing and easing causing inconsistency and accessibility issues

### Solution

use canonical motion tokens for duration and easing across all components

### Signals

- inconsistent animation
- motion mismatch
- accessibility issue

---

## CR-028 — Responsive Grid Contract

- **Category:** component-architecture
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

custom breakpoints and inconsistent grids break responsive layout predictability

### Solution

use canonical mobile-first breakpoints and 12-column grid system

### Signals

- layout break
- overflow
- inconsistent spacing

---

## CR-029 — Typography Contract

- **Category:** design-system
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

inconsistent typography usage breaks readability and system consistency

### Solution

enforce canonical font stacks, sizes, weights, and line heights

### Signals

- mixed font sizes
- poor readability
- contrast issues

---

## CR-030 — Iconography System

- **Category:** design-system
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

multiple icon systems or raster icons break consistency and accessibility

### Solution

use single svg icon library with standardized sizing and aria patterns

### Signals

- icon inconsistency
- raster usage
- missing aria labels

---

## CR-031 — Accessibility Contract (ARIA + Roles)

- **Category:** accessibility
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

components lack accessibility compliance breaking usability for assistive tech

### Solution

enforce aria roles, keyboard navigation, and wcag 2.1 aa standards

### Signals

- missing aria
- keyboard trap
- poor contrast

---

## CR-032 — Theme Persistence Contract

- **Category:** state-reactivity
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

theme persistence via client storage breaks ssr and causes flash

### Solution

persist theme using http cookies with server synchronization

### Signals

- flash of wrong theme
- hydration mismatch
- theme reset

---

## CR-033 — Density & Accessibility Mapping

- **Category:** accessibility
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

ui density scaling breaks accessibility requirements and usability constraints

### Solution

apply density scaling using tokens while enforcing wcag compliant touch targets and text sizes

### Signals

- small touch target
- text unreadable
- layout overflow

---

## CR-034 — Theme & Density Enforcement (Lint Rules)

- **Category:** governance
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

theme and density rules are not enforced causing inconsistent implementations

### Solution

use linting and ci checks to enforce canonical theme and density rules

### Signals

- rule violation
- hardcoded values
- ci failure

---

## CR-035 — Token Usage Validation

- **Category:** design-system
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

components use hardcoded values instead of tokens causing inconsistency

### Solution

validate all visual properties against canonical token system via automated checks

### Signals

- hardcoded color
- px usage
- token violation

---

## CR-036 — Component Compliance Levels

- **Category:** governance
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

components lack compliance classification causing inconsistent rule enforcement

### Solution

define compliance levels to control rule strictness and allowed exceptions

### Signals

- inconsistent rules
- partial compliance
- architecture drift

---

## CR-037 — Provider Taxonomy & Boundaries

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

providers mix responsibilities causing coupling and incorrect state handling

### Solution

define strict provider boundaries with stateless design system layer

### Signals

- state coupling
- provider misuse
- ssr issue

---

## CR-038 — Theme Engine Contract

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

multiple components write css variables causing inconsistency

### Solution

centralize css variable computation in a single theme engine

### Signals

- css conflict
- variable override
- theme inconsistency

---

## CR-039 — Settings UI Compliance

- **Category:** component-architecture
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

settings ui handles persistence and logic causing coupling

### Solution

expose state changes via callbacks and delegate persistence to app layer

### Signals

- coupling
- state misuse
- side effects

---

## CR-040 — canon-rule-40-legacy-exception-annotation

- **Category:** governance
- **Severity:** LOW
- **Status:** ENFORCED

### Problem

legacy exceptions are not documented causing hidden rule violations

### Solution

require explicit annotation for all legacy exceptions

### Signals

- hidden violation
- legacy drift
- rule bypass

---

## CR-041 — Leptos Resource Consumption Contract

- **Category:** state-reactivity
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

resource is awaited instead of accessed reactively causing render failure

### Solution

consume resource via reactive methods like get and transition

### Signals

- frozen ui
- no render
- silent failure

---

## CR-042 — UI Data Surfaces

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

multiple components implement same rendering logic for different data strategies

### Solution

use single renderer with adapter based data sources

### Signals

- duplicate component
- code redundancy
- complex api

---

## CR-043 — Domain Components and Commands

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

domain components execute commands directly causing coupling between ui and state mutation

### Solution

separate read and write using domain components command components and orchestrators

### Signals

- spawn_local in view
- button triggers mutation in domain
- mixed read write logic

---

## CR-044 — Orchestrators

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

domain components handle commands directly instead of delegating coordination

### Solution

use orchestrator layer to connect domain components and command components via callbacks and refresh signals

### Signals

- command in domain component
- resource mixed with mutation
- tight coupling read write

---

## CR-045 — Demo Components & Ephemeral State

- **Category:** governance
- **Severity:** LOW
- **Status:** ENFORCED

### Problem

demo components mimic production patterns causing unnecessary complexity and confusion

### Solution

use simple signals and direct state mutation for demos while isolating them from production code

### Signals

- resource in demo
- fake async logic
- over engineered examples

---

## CR-046 — Command Palette & Intent Surfaces

- **Category:** behavior
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

ui relies on buttons for all actions causing clutter and poor scalability

### Solution

introduce command registry and command palette as intent surface separate from ui

### Signals

- too many buttons
- no keyboard navigation
- hard to discover actions

---

## CR-047 — Tree, Context & Selection

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

tree components execute actions or contain business logic instead of exposing selection

### Solution

separate tree selection from actions using selection context and command registry

### Signals

- tree executes action
- button inside tree node
- navigation coupled with logic

---

## CR-048 — Context Provider Pattern

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

prop drilling across components makes state management complex and tightly coupled

### Solution

use context providers to share global state and eliminate prop drilling

### Signals

- prop drilling
- deep prop chains
- duplicated state passing

---

## CR-049 — Drag & Drop as Intent (Not Action)

- **Category:** behavior
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

drag and drop handlers execute business logic directly causing tight coupling

### Solution

emit semantic drop events and convert them into commands in application layer

### Signals

- api call in drag handler
- database update in drop
- coupled drag logic

---

## CR-050 — Provider Singleton & Runtime Separation

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

duplicate providers create inconsistent contexts and break interaction flow

### Solution

enforce single provider per interaction scope and restrict creation to app layer

### Signals

- context mismatch
- silent failure
- drag drop broken
- multiple providers

---

## CR-051 — Callbacks as Commands

- **Category:** behavior
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

callbacks mutate state directly instead of emitting commands

### Solution

callbacks must create commands executed via a command history system

### Signals

- no undo support
- side effects in callback
- direct mutation
- api call in callback

---

## CR-052 — Command History as First-Class Runtime

- **Category:** state-reactivity
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

local command histories break global undo and state consistency

### Solution

provide a single command history provider at the application root

### Signals

- no global undo
- duplicate history
- state inconsistency

---

## CR-053 — Client-Only Runtime Islands

- **Category:** core-runtime
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

browser dependent code executes during ssr causing panic and dom mismatch

### Solution

guard client code with cfg wasm32 and provide ssr safe fallback rendering

### Signals

- unwrap window panic
- hydration mismatch
- web_sys used in ssr

---

## CR-054 — canon-rule-54-render-must-be-total

- **Category:** state-reactivity
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

render closures assume runtime state and panic during ssr

### Solution

make render total by handling all states and move strict assumptions to user actions

### Signals

- expect in disabled
- unwrap in class
- panic during ssr

---

## CR-055 — Canonical CSS Entry Points

- **Category:** styling-css
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

relative css imports tie consumers to internal repo structure

### Solution

use canonical packages that re export css artifacts via workspace resolution

### Signals

- ../../packages-rust import
- css breaks after refactor
- postcss resolve error

---

## CR-056 — Monorepo CSS Build Pipeline

- **Category:** build-tooling
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

css is consumed directly from generators instead of build artifacts

### Solution

enforce build pipeline that copies generated css into canonical dist packages

### Signals

- css from crates
- missing dist css
- inconsistent builds

---

## CR-057 — PostCSS Canon Configuration

- **Category:** build-tooling
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

postcss misconfiguration causes unresolved imports and broken css order

### Solution

use esm config with postcss import first and enforce canonical ordering

### Signals

- tailwind import error
- css order broken
- module resolution fail

---

## CR-058 — Leptos Assets Dev Constraint

- **Category:** build-tooling
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

nested asset paths are not served causing missing css and silent failures

### Solution

copy compiled assets to root of assets directory and reference directly

### Signals

- 1 byte css file
- missing styles in dev
- assets not loading

---

## CR-059 — CSS Cascade Ownership

- **Category:** styling-css
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

multiple css bundles create undefined cascade precedence

### Solution

enforce single stylesheet entrypoint with defined cascade ownership

### Signals

- style conflicts
- random overrides
- multiple css files

---

## CR-060 — CSS Artifacts Are Immutable

- **Category:** build-tooling
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

manual edits in generated css create divergence from source

### Solution

treat css artifacts as immutable and regenerate on every change

### Signals

- manual css patch
- dist css edited
- inconsistent builds

---

## CR-061 — No Relative CSS Imports

- **Category:** styling-css
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

relative css imports couple files to directory structure

### Solution

use canonical package imports instead of relative paths

### Signals

- ../../css import
- broken imports after move
- path fragility

---

## CR-062 — Single Source of Truth for Design Tokens

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

multiple token sources create divergence and inconsistency

### Solution

define tokens in one canonical file and propagate via build pipeline

### Signals

- multiple tokens.css
- different values
- sync issues

---

## CR-063 — Leptos Reactivity - No .get() Outside Closures

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

reactive values accessed outside tracking context prevent updates

### Solution

wrap memo and signal access inside closures to ensure reactive tracking

### Signals

- outside reactive context error
- stale ui
- memo not updating

---

## CR-064 — CSS Build Pipeline is Mandatory

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

css is not generated because build step is missing

### Solution

run explicit tailwind build pipeline before serving application

### Signals

- unstyled page
- missing css file
- tailwind not applied

---

## CR-065 — data-theme Sync is ThemeProvider Responsibility

- **Category:** design-system
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

theme state is not reflected in dom attributes so css does not apply

### Solution

sync theme signals to dom attributes using client side effect

### Signals

- theme not applying
- missing data-theme
- css tokens inactive

---

## CR-066 — Workbench Setup Checklist

- **Category:** governance
- **Severity:** LOW
- **Status:** ENFORCED

### Problem

developers start without validating setup leading to hidden configuration failures

### Solution

enforce pre development checklist validating tokens css and assets pipeline

### Signals

- unstyled app
- missing css pipeline
- invalid setup

---

## CR-067 — Leptos CSR Does NOT Load CSS via `<Stylesheet />`

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

stylesheet component does not inject css in csr mode

### Solution

load css via html link tags instead of stylesheet component in csr

### Signals

- no styles loaded
- stylesheet ignored
- unstyled csr app

---

## CR-068 — Asset Must Exist in Final dist/

- **Category:** build-tooling
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

assets referenced in app do not exist in final build output

### Solution

verify all referenced assets physically exist in dist before runtime

### Signals

- 404 asset
- missing css file
- silent build failure

---

## CR-069 — Trunk Only Serves What's in dist/

- **Category:** build-tooling
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

assets outside dist are not served causing 404 errors

### Solution

ensure all assets are built or copied into dist before serving

### Signals

- 404 styles.css
- public not served
- asset not found

---

## CR-070 — CSS Pipeline Requires Health Checks

- **Category:** build-tooling
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

css pipeline failures are silent and only detected at runtime

### Solution

implement automated checks to validate css existence content and size

### Signals

- empty css file
- missing classes
- silent failure

---

## CR-071 — Debug Theme by Verifying File First

- **Category:** governance
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

developers debug application logic before verifying css file existence

### Solution

always validate css file existence serving and content before debugging code

### Signals

- missing css
- 404 stylesheet
- theme not applying

---

## CR-072 — Layout H1 Prohibition

- **Category:** accessibility
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

layouts include h1 causing multiple h1 elements per page

### Solution

restrict h1 usage to page components only and use alternative tags in layouts

### Signals

- multiple h1
- seo issues
- screen reader confusion

---

## CR-073 — ComponentPage Template Contract

- **Category:** component-architecture
- **Severity:** LOW
- **Status:** ENFORCED

### Problem

component pages duplicate layout structure instead of using centralized template

### Solution

use ComponentPage template and pass only data as props from product layer

### Signals

- duplicate layout code
- inconsistent component pages
- template drift

---

## CR-074 — Block Semantic HTML

- **Category:** accessibility
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

blocks lack semantic html and proper aria leading to accessibility issues

### Solution

use section landmarks aria-labelledby and strict heading hierarchy

### Signals

- axe violations
- screen reader confusion
- invalid heading hierarchy

---

## CR-075 — Primitive CSS Prohibition

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

primitives include css and viewport logic violating separation of concerns

### Solution

restrict primitives to semantic html and move styling to ui layer

### Signals

- tailwind in primitives
- viewport logic in context
- style leakage

---

## CR-076 — Navigation vs Action Component Contract

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

anchors nested in buttons causing invalid html and interaction conflicts

### Solution

separate navigation components using a tags and action components using button tags

### Signals

- button link nesting
- keyboard navigation issues
- aria conflicts

---

## CR-077 — Monorepo Path Stability

- **Category:** build-tooling
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

hardcoded paths break when monorepo structure changes

### Solution

calculate paths relative to config files and avoid absolute paths

### Signals

- file not found after refactor
- broken imports
- postcss path errors

---

## CR-078 — Token Definition Completeness

- **Category:** design-system
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

components use partial tokens and hardcoded values breaking consistency

### Solution

enforce 100 percent token usage for all applicable properties

### Signals

- hardcoded values
- theme inconsistencies
- partial token usage

---

## CR-079 — ComponentPage Template Contract

- **Category:** design-system
- **Severity:** LOW
- **Status:** ENFORCED

### Problem

generic classes break layout consistency across component pages

### Solution

use canon-page and canon-rail classes as mandatory layout contract

### Signals

- content not centered
- inconsistent page width
- layout drift

---

## CR-080 — Workspace Watch Configuration

- **Category:** build-tooling
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

changes in path dependencies do not trigger recompilation

### Solution

configure watch-additional-files for all local dependencies

### Signals

- stale build
- no recompilation
- changes ignored

---

## CR-081 — Flex Layout Ownership

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

components define flex behavior internally instead of parent context

### Solution

assign flex responsibility to ui wrappers and parent containers

### Signals

- wrong width calculation
- flex not growing
- layout mismatch

---

## CR-082 — CSS Build Pipeline Health

- **Category:** build-tooling
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

css changes not reflected because build pipeline is not executed

### Solution

run css build pipeline and verify compiled output before testing

### Signals

- stale styles
- css not updating
- postcss not triggered

---

## CR-083 — Layout Zones Contract

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

components are placed without spatial rules causing layout inconsistency

### Solution

enforce five canonical layout zones with strict nesting and responsibilities

### Signals

- layout chaos
- content outside container
- z-index conflicts

---

## CR-084 — SVG Dark Mode Contract

- **Category:** styling-css
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

external svg images do not inherit css color leading to dark mode issues

### Solution

use css filters inline svg or masks to control svg coloring

### Signals

- icons stay black
- dark mode contrast issues
- svg color mismatch

---

## CR-085 — Leptos Asset Pipeline in Dev Mode

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

assets not served in dev because trunk hooks are ignored

### Solution

place assets in public directory and configure leptos asset serving

### Signals

- 404 assets
- missing files in dev
- trunk hooks ignored

---

## CR-086 — Children vs ChildrenFn Contract

- **Category:** state-reactivity
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

incorrect children type leads to fnonce errors and reactivity issues

### Solution

use childrenfn for reactive wrappers and children for static slots

### Signals

- expected fn found fnonce
- sync trait error
- render failure

---

## CR-087 — Leptos SSR Script Placement

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

scripts execute before hydration leading to broken event bindings

### Solution

inject scripts inside shell after app rendering

### Signals

- event not firing
- phantom behavior
- hydration mismatch

---

## CR-089 — Primitives Must Never Touch Browser APIs

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

primitives access browser apis causing ssr and hydration issues

### Solution

restrict primitives to html and delegate behavior to runtime js

### Signals

- window usage in primitives
- hydration errors
- ssr breakage

---

## CR-090 — Hydration Is DOM Replacement, Not Enhancement

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

developers assume dom persists after hydration causing broken behavior

### Solution

treat hydration as dom replacement and avoid relying on node identity

### Signals

- event lost after hydration
- inconsistent behavior
- silent failures

---

## CR-091 — Markdown and Code Blocks Are Render-Only

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

markdown components include behavior instead of pure rendering

### Solution

limit markdown components to structure and delegate behavior externally

### Signals

- event handlers in markdown
- clipboard logic inside render
- ssr coupling

---

## CR-092 — If It Works in Prod but Not Dev, Suspect Hydration Order

- **Category:** governance
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

developers misdiagnose issues caused by hydration order differences

### Solution

use heuristic to identify hydration timing problems early

### Signals

- works in prod fails in dev
- no errors
- inconsistent behavior

---

## CR-093 — Leptos WASM Dev Builds Must Use Release Mode

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

debug wasm builds cause large binaries and browser stalls

### Solution

use release mode for leptos wasm development builds

### Signals

- browser freeze
- slow hydration
- large wasm size

---

## CR-094 — Leptos Workspace Features Must Be Explicit

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

workspace level features are missing causing leptos macro type inference failures

### Solution

declare leptos features explicitly in workspace dependencies instead of member crates

### Signals

- E0282 error
- cannot infer type
- view macro failure

---

## CR-095 — SSR Requires Complete HTML Shell

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

missing html shell prevents SSR rendering and hydration

### Solution

provide full html head and body structure through shell function

### Signals

- ERR_EMPTY_RESPONSE
- leptos_meta panic
- missing head tag

---

## CR-096 — SSR Requires Explicit Provider Tree

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

missing providers cause context not found panics during SSR

### Solution

wrap app with all required providers in correct dependency order

### Signals

- context not found
- server panic
- empty response

---

## CR-097 — Leptos 0.8 Requires Floating Nightly Toolchain

- **Category:** build-tooling
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

pinned toolchain or versions cause build incompatibility errors

### Solution

use floating nightly toolchain and unpinned minor versions in dependencies

### Signals

- rustc not supported
- edition2024 error
- dependency conflict

---

## CR-098 — Axum + Leptos SSR Closures Must Own State

- **Category:** component-architecture
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

shared state across closures causes borrow of moved value errors

### Solution

clone state before each closure and move ownership into closures

### Signals

- E0382 error
- borrow of moved value
- closure ownership issue

---

## CR-099 — CSR Islands Must Not Own Routing or Providers

- **Category:** core-runtime
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

islands create routing or providers causing conflicts with SSR host

### Solution

restrict islands to local rendering and delegate routing and providers to host

### Signals

- hydration mismatch
- router conflict
- context already exists

---

## CR-100 — Build Orchestrators Must Be Workspace-Scoped

- **Category:** build-tooling
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

build configuration in member crates is ignored or conflicts in workspace builds

### Solution

define all build orchestrator metadata in workspace root cargo.toml

### Signals

- metadata not found
- hot reload broken
- asset 404
- config conflict

---

## CR-101 — Workbench Assets Must Be Product-Scoped

- **Category:** governance
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

illustrative assets stored in design system create coupling and misuse of scope

### Solution

keep illustrative assets inside product scope and restrict design system to primitives

### Signals

- asset coupling
- design drift
- wrong ownership

---

## CR-102 — Runtime JS Is Shell Infrastructure

- **Category:** behavior
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

components directly execute runtime js causing coupling and inconsistent behavior

### Solution

move all runtime javascript to shell layer and keep components declarative

### Signals

- event duplication
- dom coupling
- inconsistent behavior

---

## CR-103 — Critical Runtime JS Must Be Inline in SSR

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

external scripts execute after hydration causing event listeners to fail

### Solution

inline critical runtime javascript before hydration scripts in ssr head

### Signals

- silent failure
- listeners not working
- race condition

---

## CR-104 — AutoReload Breaks Script Order Guarantees

- **Category:** build-tooling
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

autoreload injects scripts unpredictably breaking execution order

### Solution

avoid relying on external script order and inline critical runtime logic

### Signals

- intermittent failure
- hot reload bug
- timing issue

---

## CR-105 — canon-rule-105-visual-indicators-must-have-a-single-owner

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

multiple layers render the same visual indicator causing duplication

### Solution

assign a single owner component for each visual indicator

### Signals

- duplicate visuals
- css hacks
- misalignment

---

## CR-106 — canon-rule-106-ui-neutralizes-hostile-css-not-primitives

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

css fixes applied in primitives break layer separation

### Solution

handle css neutralization only in ui layer while keeping primitives pure

### Signals

- css leakage
- primitive contamination
- layer violation

---

## CR-107 — Token Architecture & Theme Specificity

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

color tokens in root prevent themes from overriding values

### Solution

separate structural tokens in root and color tokens in theme selectors

### Signals

- theme override fail
- color mismatch
- specificity conflict

---

## CR-108 — Visual Surfaces Contract

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

components define appearance instead of using semantic surface tokens

### Solution

enforce surface types and token-based styling for all components

### Signals

- design drift
- inconsistent ui
- dark mode break

---

## CR-111 — Model First, CSS Second

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

css is used to fix structural and ownership issues in component architecture

### Solution

fix the underlying model and component hierarchy instead of applying css workarounds

### Signals

- css hacks
- important usage
- layout overlap

---

## CR-112 — UI Owns Visual Style

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

visual styling is applied in primitives instead of ui layer

### Solution

restrict styling to ui layer and keep primitives purely structural

### Signals

- style leakage
- duplicate css
- primitive coupling

---

## CR-113 — States Are Data, Not Style

- **Category:** design-system
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

state is encoded as visual style instead of semantic data

### Solution

express state via data attributes and map to visuals only in css

### Signals

- style coupling
- theme break
- implicit state

---

## CR-114 — Single Visual Authority

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

multiple elements render the same visual signal causing conflicts

### Solution

assign a single component as the visual authority for each signal

### Signals

- duplicate borders
- misaligned visuals
- css conflicts

---

## CR-115 — Reset Awareness & CSS Boundaries

- **Category:** styling-css
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

components do not account for global css reset causing inconsistent behavior

### Solution

handle reset effects in ui layer while keeping primitives untouched

### Signals

- style inconsistency
- reset conflict
- layout break

---

## CR-116 — WASM Externref Table Limits

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

unbounded creation of externrefs causes wasm runtime crashes

### Solution

avoid per-component callbacks and delegate behavior to global shell handlers

### Signals

- table grow error
- runtime crash
- externref overflow

---

## CR-117 — Design System Callbacks Are Props, Not Handlers

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

inline handlers create externrefs leading to wasm runtime limits

### Solution

pass callbacks as props and delegate execution to shell via data attributes

### Signals

- externref overflow
- runtime crash
- event handler issue

---

## CR-118 — View<\_> Type Boundary Prohibition

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

open view type inference causes variance cycles and compilation errors

### Solution

restrict view<_> to components and use impl into view or anyview elsewhere

### Signals

- variance cycle
- e0391 error
- type mismatch

---

## CR-119 — No #[prop(optional, into)] in UI Layer

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

optional into props create type ambiguity and compiler errors

### Solution

use explicit option types in ui components and avoid implicit conversions

### Signals

- e0308 error
- type mismatch
- ambiguous props

---

## CR-120 — DOM Events vs Semantic Callbacks Boundary

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

dom events are exposed in ui layer and callbacks are incorrectly wrapped or typed

### Solution

restrict dom events to primitives and expose only semantic callbacks in ui components

### Signals

- e0308 type mismatch
- some callback wrapping
- on:click in ui api

---

## CR-121 — StoredValue for Non-Copy Values in view! Closures

- **Category:** state-reactivity
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

non-copy values in view closures cause FnOnce errors and break reactivity

### Solution

wrap all non-copy values in StoredValue before view and access via get_value or with_value

### Signals

- E0525 error
- expected Fn found FnOnce
- closure moves value
- re-render failure

---

## CR-122 — No Conditional Rendering with .then() Closures

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

conditional rendering with then closures creates FnOnce errors and hydration issues

### Solution

use Show component for all conditional rendering instead of then closures

### Signals

- FnOnce error
- hydration mismatch
- type inference failure
- expected element found nothing

---

## CR-123 — Component Architecture Taxonomy and Contracts

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

components are not classified leading to boundary violations across layers

### Solution

enforce strict taxonomy with five layers and explicit contracts per component type

### Signals

- primitive with state
- ui with css tokens
- component confusion

---

## CR-124 — Primitive Contract Types

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

primitives contain logic state or type conversion violating architectural boundaries

### Solution

enforce pure interactive or container primitive types with strict no state no conversion rules

### Signals

- primitive with state
- prop into usage
- unwrap in primitive

---

## CR-125 — UI Component Contracts

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

ui components mix responsibilities and use ambiguous prop patterns

### Solution

enforce adapter controlled and composite ui types with strict callback and prop rules

### Signals

- prop optional into
- mouseevent in api
- primitive leakage

---

## CR-126 — Component Domain Contracts

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

components mix domain logic with ui or primitive concerns

### Solution

separate domain logic into stateful and orchestrator components with strict boundaries

### Signals

- api calls in ui
- primitive usage in component
- routing inside component

---

## CR-127 — Block Composition Contracts

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

blocks contain business logic state or routing instead of pure composition

### Solution

restrict blocks to semantic or interactive composition without domain responsibilities

### Signals

- api call in block
- routing in block
- global state usage

---

## CR-128 — Layout Shell and Zone Contracts

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

layouts mix structure with logic causing tight coupling and poor reuse

### Solution

enforce shell and zone layout types with strict structural-only responsibilities

### Signals

- layout with api calls
- auth logic in layout
- state in layout
- non reusable layout

---

## CR-129 — SSR Event Safety

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

dynamic event handlers in ssr lists break hydration determinism

### Solution

avoid event handlers in iterators and use delegation or csr only patterns

### Signals

- unreachable panic
- hydration error
- event binding failure

---

## CR-130 — Controlled UI Contract

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

ui components receive plain values instead of signals breaking reactivity

### Solution

require ui components to accept signal or rwsignal for reactive state props

### Signals

- ui not updating
- stale state
- manual rerender

---

## CR-131 — Reactive Boundary Ownership

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

non copy values moved into reactive boundaries cause fnonce errors

### Solution

clone simple values or use storedvalue for complex or stateful values

### Signals

- fnonce error
- closure move error
- ownership violation

---

## CR-132 — Layout Composition Over Abstraction

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

layouts handle composition logic causing ownership and hydration issues

### Solution

restrict layouts to structural wrappers and move composition to application layer

### Signals

- children move error
- hydration mismatch
- slot complexity

---

## CR-133 — Children Consumption Locality

- **Category:** state-reactivity
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

children is forwarded across layers causing ownership and fnonce errors

### Solution

consume children immediately at the closest render point

### Signals

- move error
- fnonce closure
- render failure

---

## CR-134 — Layouts Are CSS and Semantics Only

- **Category:** styling-css
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

layouts include behavior or state breaking structural determinism

### Solution

restrict layouts to semantic containers and css structure only

### Signals

- state in layout
- event in layout
- layout coupling

---

## CR-135 — UI vs Layout Responsibility Boundary

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

ui and layout responsibilities mix causing architectural coupling

### Solution

enforce strict boundary where layout handles zones and ui handles behavior

### Signals

- provider in layout
- behavior in layout
- coupled layers

---

## CR-136 — Controllers Are CSR-Only

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

controllers executed during ssr cause hydration mismatch and runtime errors

### Solution

gate controllers with wasm target and execute only in csr

### Signals

- hydration mismatch
- window undefined
- double execution

---

## CR-137 — Providers Must Have a Single Owner

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

multiple providers create duplicated state and inconsistent ui

### Solution

ensure each provider has a single owning component controlling lifecycle

### Signals

- duplicate provider
- state desync
- multiple contexts

---

## CR-138 — Children Must Be Consumed Immediately

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

children forwarding breaks ownership and causes render instability

### Solution

consume children immediately in the receiving component

### Signals

- move error
- ownership violation
- render failure

---

## CR-139 — Slots Are UI-Level Only

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

slots are defined in layouts causing ownership and composition issues

### Solution

restrict slots to ui components and keep layouts purely compositional

### Signals

- ownership explosion
- fnonce issues
- layout complexity

---

## CR-140 — Layouts Are Structural, Not Behavioral

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

behavior in layouts breaks ssr determinism and reuse

### Solution

keep layouts strictly structural with no state or logic

### Signals

- state in layout
- event handler in layout
- logic in layout

---

## CR-141 — CSR Composition Belongs to the App Layer

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

csr logic outside app layer breaks ssr boundaries and architecture separation

### Solution

centralize all csr composition and orchestration in the application layer only

### Signals

- hydration mismatch
- cfg wasm leakage
- ssr crash
- logic duplication

---

## CR-142 — canon-rule-142-children-must-always-be-optional

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

mandatory children props create rigid and breaking component apis

### Solution

declare children as optional and safely render when present

### Signals

- component requires children
- unwrap children panic
- composition break

---

## CR-143 — canon-rule-143-ui-must-be-hydration-deterministic

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

non deterministic ui output breaks hydration and causes mismatches

### Solution

render static deterministic html during ssr and defer behavior to events

### Signals

- hydration panic
- dom mismatch
- runtime error

---

## CR-144 — canon-rule-144-providers-expose-state-apps-own-interaction

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

providers include interaction logic causing coupling and ownership violations

### Solution

limit providers to state exposure and mutation apis only

### Signals

- provider renders ui
- implicit interaction
- state coupling

---

## CR-145 — canon-rule-145-ui-must-not-mutate-global-state-implicitly

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

ui components mutate global state implicitly causing hidden side effects

### Solution

enforce explicit callbacks for all state mutations from ui components

### Signals

- hidden state change
- implicit side effect
- context mutation in ui

---

## CR-146 — canon-rule-146-ui-content-must-be-ssr-stable

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

dynamic content in ssr causes hydration mismatch and runtime errors

### Solution

ensure all rendered content is static during ssr and defer changes to events

### Signals

- hydration panic
- unreachable error
- dom mismatch

---

## CR-147 — canon-rule-147-reactive-closures-are-data-not-structure

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

reactive closures used for structure cause hydration mismatch and instability

### Solution

restrict reactive closures to data binding and use Show for structure

### Signals

- hydration mismatch
- unstable dom
- render inconsistency

---

## CR-148 — canon-rule-148-ui-must-never-infer-intent-from-state

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

ui infers intent from state changes causing implicit behavior

### Solution

express intent explicitly via callbacks or commands and update state after

### Signals

- implicit logic
- duplicated behavior
- side effects

---

## CR-149 — canon-rule-149-controllers-must-be-temporal-not-structural

- **Category:** behavior
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

controllers define structure instead of managing temporal logic

### Solution

limit controllers to async flows and coordination and move rendering to ui

### Signals

- controller renders ui
- structure in controller
- coupling ui logic

---

## CR-150 — canon-rule-150-ui-must-be-deterministic-under-ssr-and-hydration

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

non deterministic rendering breaks ssr hydration contract

### Solution

ensure identical rendering output during ssr and initial hydration

### Signals

- hydration failure
- dom mismatch
- runtime divergence

---

## CR-151 — canon-rule-151-visual-feedback-must-never-encode-application-state

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

visual feedback encodes application logic causing hidden coupling

### Solution

treat visuals as projection of state and never as decision source

### Signals

- logic based on icon
- state inferred from ui
- implicit coupling

---

## CR-152 — canon-rule-152-provider-callback-hydration-ownership

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

callbacks capturing provider context are dropped before hydration causing runtime errors

### Solution

use inline event handlers or hydration safe callback mechanisms

### Signals

- callback removed
- unreachable panic
- hydration failure

---

## CR-153 — canon-rule-153-layouts-must-be-event-free-during-hydration

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

event listeners in layouts during hydration cause runtime mismatch and failures

### Solution

remove all event bindings from layouts and delegate to csr controllers or ui

### Signals

- unreachable panic
- callback removed
- hydration failure
- dom instability

---

## CR-154 — Deterministic Layout via Canonical CSS (SSR-Safe)

- **Category:** styling-css
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

layout is controlled by runtime logic instead of css causing hydration mismatch

### Solution

define all layout positioning in css and use data attributes for state

### Signals

- hydration mismatch
- layout drift
- runtime positioning

---

## CR-155 — CSS Token Contract Is Architecture

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

mixed css token formats break theming and runtime css evaluation

### Solution

enforce numeric hsl format for all tokens and apply functions at consumption

### Signals

- css not applied
- invalid hsl value
- theme broken
- rgba fallback

---

## CR-156 — CSS Variable Scope Is Non-Negotiable

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

css variables defined in wrong scope resolve to empty values

### Solution

define variable mappings in the same selector scope as source variables

### Signals

- empty css variable
- theme not applied
- silent css failure

---

## CR-157 — Design System CSS Must Be Build-Time Flattened

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

css is served with runtime imports causing missing styles and silent failures

### Solution

flatten all css at build time into a single file with no @import statements

### Signals

- missing styles
- silent import failure
- css loaded but empty

---

## CR-158 — Design System Packages Are Immutable Contracts (No Direct File Imports)

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

apps import design system files directly causing coupling and breakage

### Solution

consume design system only through versioned package exports

### Signals

- path import
- ci break
- coupling

---

## CR-159 — UI CSS Must Be Fail-Safe, Token-Only, and Attribute-Scoped

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

ui css depends on structure or tokens that may be missing causing fragile styles

### Solution

enforce token-only css scoped by data attributes with fail-safe defaults

### Signals

- missing styles
- fragile css
- inconsistent rendering

---

## CR-160 — First App Must Not Require CSS Build

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

first app requires css build step causing onboarding friction

### Solution

provide prebuilt css so first app runs without any build step

### Signals

- build required
- onboarding friction
- toolchain dependency

---

## CR-161 — Canonical CSS Load Order Is Mandatory

- **Category:** styling-css
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

css files are loaded in wrong order causing cascade issues

### Solution

enforce strict canonical load order for all css layers

### Signals

- style override
- theme broken
- visual inconsistency

---

## CR-162 — Providers Are Infrastructure, Not UI

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

providers are used as ui components causing architectural leakage

### Solution

keep providers as infrastructure and let ui consume their context

### Signals

- provider as ui
- coupling
- state leakage

---

## CR-163 — DOM Effects Are Hydration-Only

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

dom effects run during ssr causing hydration mismatch

### Solution

guard all dom effects with hydrate feature or runtime checks

### Signals

- hydration mismatch
- runtime error
- dom panic

---

## CR-164 — WASM and JS Assets Must Be Served by SSR

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

wasm and js assets are not served causing hydration failure

### Solution

explicitly serve pkg assets from ssr server routes

### Signals

- 404 wasm
- no hydration
- static html only

---

## CR-165 — CanonRS Workbench Is a Canonical Reference

- **Category:** governance
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

architecture ignores working reference leading to inconsistency

### Solution

treat workbench implementation as canonical source of truth

### Signals

- reinventing patterns
- inconsistency
- architecture drift

---

## CR-166 — Dist Is Read-Only

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

dist files are manually edited causing non reproducible builds

### Solution

apply all fixes in source and regenerate dist via build process

### Signals

- manual edit
- inconsistent build
- hidden bug

---

## CR-167 — CanonRS CSS Has a Single Entrypoint

- **Category:** styling-css
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

multiple css entrypoints cause ordering bugs and missing styles

### Solution

expose a single canonical css entrypoint for all applications

### Signals

- missing tokens
- import drift
- css inconsistency

---

## CR-168 — UI Must Declare Required Tokens

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

ui references undefined tokens causing broken or invisible styles

### Solution

ensure all tokens are defined before being used by ui components

### Signals

- missing variable
- invisible ui
- broken style

---

## CR-169 — Token Import Order Is Architectural

- **Category:** styling-css
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

tokens are imported in wrong order causing inconsistent styles

### Solution

enforce canonical import order from core tokens to layouts and blocks

### Signals

- partial styles
- broken components
- order conflict

---

## CR-170 — HTML and CSS Must Share the Same Contract

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

html attributes do not match css selectors causing broken styling

### Solution

ensure html and css share identical contract using data attributes

### Signals

- no styles applied
- selector mismatch
- ui broken

---

## CR-171 — Phantom Variables Are Forbidden

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

undefined css variables are used causing silent failures

### Solution

ensure all variables are declared and traceable in token definitions

### Signals

- undefined variable
- silent failure
- broken ui

---

## CR-172 — The Site Does Not Own Design

- **Category:** governance
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

applications define design rules causing fragmentation

### Solution

centralize all design definitions within the design system packages

### Signals

- design drift
- inconsistency
- fragmentation

---

## CR-173 — CSS Is a First-Class System

- **Category:** governance
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

css is treated as ad hoc leading to inconsistent and untraceable styling

### Solution

treat css as a governed system with strict architectural rules

### Signals

- css hacks
- inconsistent styles
- debug difficulty

---

## CR-174 — canon-rule-174-tokens-are-compile-time-contracts

- **Category:** build-tooling
- **Severity:** BLOCKING
- **Status:** ENFORCED

### Problem

tokens are not validated at build time causing inconsistencies and errors

### Solution

enforce token definitions and usage through compile time validation

### Signals

- token drift
- invalid token
- build failure

---

## CR-175 — Dark/Light Is a CSS Concern, Not a Component Concern

- **Category:** design-system
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

components implement dark light logic causing duplication and coupling

### Solution

keep components theme agnostic and delegate context resolution to css tokens

### Signals

- duplicate logic
- theme inconsistency
- hydration mismatch

---

## CR-176 — Governance Is the Single Source of Truth

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

multiple layers define rules instead of central governance causing inconsistency

### Solution

route all contract resolution and validation exclusively through governance layer

### Signals

- logic duplication
- inconsistency
- drift

---

## CR-177 — canonrs.css Is a Generated Artifact, Never a Design Surface

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

generated css is manually edited causing non deterministic builds

### Solution

edit only source files and regenerate css through build process

### Signals

- manual edit
- build mismatch
- lost changes

---

## CR-178 — Contracts Are Read-Only APIs (DEPRECATED)

- **Category:** governance
- **Severity:** NONE
- **Status:** DEPRECATED

### Problem

deprecated rules are referenced causing fragmentation and duplication

### Solution

reference only canonical active rules and ignore deprecated ones

### Signals

- duplicate rule
- confusion
- invalid reference

---

## CR-179 — No Visual Surface Tokens in Core or Families

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

visual surface values are defined in core or family tokens breaking theme separation

### Solution

define only semantic intent in core and resolve appearance in theme layer

### Signals

- dark mode break
- visual drift
- theme coupling

---

## CR-180 — Semantic Token Names Must Follow Canonical Suffixes

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

semantic tokens use inconsistent suffixes causing lookup failures

### Solution

enforce only canonical suffixes bg fg and border for semantic tokens

### Signals

- missing style
- token mismatch
- naming drift

---

## CR-181 — Token Cascade Is Architecture, Not Convention

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

token cascade is treated as convention causing violations and inconsistencies

### Solution

enforce token cascade through build time validation and strict layering

### Signals

- layer violation
- build failure
- inconsistent usage

---

## CR-182 — Semantic Role Must Affect Visual Contrast

- **Category:** design-system
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

different semantic roles share identical visuals causing user confusion

### Solution

ensure each semantic role has distinct visual contrast and background

### Signals

- no contrast
- user confusion
- semantic collapse

---

## CR-183 — Overlay Positioning Is a Primitive, Not a Controller Concern

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

overlay positioning is implemented in controllers causing duplication and instability

### Solution

move all positioning logic into dedicated primitives

### Signals

- inline style
- duplication
- hydration break

---

## CR-184 — Floating Overlays Expose Position Only via CSS Variables

- **Category:** styling-css
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

overlay positioning is exposed through multiple mechanisms causing inconsistency

### Solution

expose all overlay position values exclusively via css custom properties

### Signals

- inline styles
- layout drift
- inconsistent positioning

---

## CR-185 — Overlays Must Be Anchor-Relative

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

overlays are positioned relative to viewport instead of anchor element

### Solution

position overlays strictly relative to explicit anchor elements

### Signals

- floating overlay
- misaligned ui
- position drift

---

## CR-186 — Overlay Visibility Is Controlled Only via data-state

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

overlay visibility is controlled imperatively causing hydration mismatch

### Solution

control overlay visibility exclusively via data-state attributes and css

### Signals

- hydration mismatch
- dom divergence
- visibility bug

---

## CR-187 — Floating Primitives Enforce CSS-Variable-Only Positioning

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

primitives mutate layout properties directly causing inconsistency

### Solution

restrict primitives to writing only css variables for positioning

### Signals

- inline top left
- layout mutation
- contract violation

---

## CR-188 — Overlay Positioning Is Not UI Logic

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

overlay positioning logic is implemented in ui or controllers causing duplication

### Solution

delegate positioning entirely to dedicated primitives

### Signals

- duplicate logic
- geometry in ui
- inconsistent overlays

---

## CR-191 — Pages Must Delegate Wiring to Page Behaviors Only

- **Category:** behavior
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

pages contain logic and wiring causing coupling and growth

### Solution

delegate all wiring to page behaviors and keep pages logic free

### Signals

- logic in page
- event listener
- coupling

---

## CR-192 — Page Behaviors Must Not Implement New Logic

- **Category:** behavior
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

page behaviors implement logic causing duplication and architecture collapse

### Solution

restrict page behaviors to wiring and delegate logic to framework behaviors

### Signals

- logic in behavior
- duplication
- unscalable

---

## CR-193 — Framework Behaviors Are the Only Source of Interactivity

- **Category:** behavior
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

interactivity is implemented outside framework behaviors causing inconsistency

### Solution

implement all interactivity exclusively within framework behaviors

### Signals

- event in ui
- inconsistent behavior
- duplication

---

## CR-194 — UI Components Are Pure Render Functions

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

ui components contain logic or side effects causing hydration issues

### Solution

ensure ui components are pure render functions with no side effects

### Signals

- effect in ui
- dom access
- hydration error

---

## CR-195 — Interactive Components Require Explicit ID

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

interactive components use non deterministic or optional ids causing hydration and behavior issues

### Solution

require explicit deterministic id prop provided by consumer for all interactive components

### Signals

- hydration mismatch
- missing id
- behavior attach fail

---

## CR-196 — SSR/CSR Separation for WASM Bloat Prevention

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

ssr and csr code are compiled together causing wasm bloat and inefficiency

### Solution

separate modules strictly by compilation target using feature flags

### Signals

- large wasm
- slow build
- memory bloat

---

## CR-197 — Feature Flags Are Architectural Boundaries

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

feature flags are misused causing cross target code leakage and instability

### Solution

treat feature flags as hard architectural boundaries defining code location

### Signals

- type mismatch
- duplicate crates
- hydration panic

---

## CR-198 — Runtime Crates Must Not Leak Cross-Target Types

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

runtime types leak across targets causing type conflicts and instability

### Solution

use shared contract types and isolate runtime specific implementations

### Signals

- type mismatch
- duplicate dependency
- integration failure

---

## CR-199 — Server Adapters Are Integration Code

- **Category:** governance
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

server adapter logic leaks into framework causing coupling and instability

### Solution

isolate server adapters as integration layer separate from framework

### Signals

- framework coupling
- runtime leak
- migration issue

---

## CR-200 — Shared Crates Must Be Zero-Dependency Runtime

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

shared crates include runtime dependencies causing architecture leakage

### Solution

restrict shared crates to pure types with no runtime dependencies

### Signals

- runtime import
- dependency leak
- build failure

---

## CR-201 — Presets Are the Only Source of Color Values

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

color values are defined outside presets causing duplication and inconsistency

### Solution

define all color values exclusively within preset definitions

### Signals

- color drift
- palette break
- hardcoded color

---

## CR-202 — Themes Resolve Context, Not Palette

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

themes define color values directly causing palette inconsistency

### Solution

themes must reference preset tokens and never define color values

### Signals

- theme override
- duplicate color
- semantic drift

---

## CR-203 — Semantic Tokens Are the Only Bridge Between Theme and Families

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

components or families bypass semantic tokens causing coupling and context loss

### Solution

enforce semantic tokens as the only bridge between theme and families

### Signals

- token bypass
- context loss
- refactor break

---

## CR-204 — Theme Files Must Be Last in the Cascade

- **Category:** styling-css
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

theme files are imported before other css causing overrides to fail

### Solution

import theme files last in the css cascade after all tokens and components

### Signals

- theme not applied
- wrong colors
- override failure

---

## CR-205 — UI Tokens Must Bind to Semantic Tokens, Never to Presets

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

family tokens reference preset tokens directly bypassing semantic layer

### Solution

bind ui tokens exclusively to semantic tokens not preset tokens

### Signals

- theme ignored
- context loss
- token bypass

---

## CR-206 — UI Inventory Is Fixed and Canonical

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

ui component set changes without control causing breaking changes and drift

### Solution

enforce fixed ui inventory with build time validation and versioning

### Signals

- unexpected component
- inventory drift
- build failure

---

## CR-207 — Preset Switching Must Never Change Component CSS

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

components depend on preset values causing rebuild and coupling

### Solution

ensure components use tokens so preset changes only affect variable values

### Signals

- hardcoded color
- rebuild required
- theme break

---

## CR-208 — A Working Theme Toggle Is Proof of Correct Token Architecture

- **Category:** governance
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

theme toggle breaks visuals due to incorrect token architecture

### Solution

fix token cascade and mappings instead of modifying javascript or components

### Signals

- wrong colors
- toggle bug
- visual mismatch

---

## CR-209 — Axum Version Must Match Adapter Version

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

axum versions differ between adapter and app causing type mismatch

### Solution

use exact axum version defined by adapter without override

### Signals

- type mismatch
- build error
- duplicate crate

---

## CR-210 — cdylib Is Mandatory for Hydration

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

cdylib crate type is missing causing wasm generation failure

### Solution

declare cdylib crate type to ensure proper wasm output

### Signals

- no wasm
- hydration fail
- build error

---

## CR-211 — SSR Meta Requires Explicit HTML Shell

- **Category:** core-runtime
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

ssr meta is used without html shell causing runtime failure

### Solution

provide explicit html shell with head and body for ssr rendering

### Signals

- runtime panic
- missing head
- meta error

---

## CR-212 — Hydration Bootstrap Is Tooling-Owned

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

application controls wasm bootstrap causing lifecycle conflicts

### Solution

delegate bootstrap to tooling and expose only entrypoint

### Signals

- double init
- unexpected behavior
- bootstrap conflict

---

## CR-213 — Streaming SSR Requires Explicit Executor Initialization

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

streaming ssr runs without executor causing runtime failures

### Solution

initialize async executor explicitly before rendering

### Signals

- runtime panic
- stream hang
- async failure

---

## CR-214 — lib.rs Owns Application Structure and UI Semantics

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

application structure is defined outside lib rs causing architectural drift

### Solution

centralize all ui composition and routing in lib rs

### Signals

- ui in main
- routing drift
- meta inconsistency

---

## CR-215 — main.rs Owns Runtime and Server Bootstrap Exclusively

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

main rs contains application logic causing coupling and instability

### Solution

restrict main rs to runtime initialization and server bootstrap only

### Signals

- logic in main
- feature entanglement
- bootstrap error

---

## CR-216 — App Components Must Not Define HTML Structure

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

components define html structure causing invalid ssr output

### Solution

limit html document structure to shell and keep components content only

### Signals

- invalid html
- hydration error
- meta failure

---

## CR-217 — Entry Points Must Be Explicit, Public, and Isolated

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

entrypoints are implicit or private causing build ambiguity

### Solution

define public feature isolated entrypoints for ssr and csr

### Signals

- unresolved import
- feature leak
- build failure

---

## CR-218 — Legacy Rendering APIs Are Forbidden

- **Category:** governance
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

deprecated rendering apis are used causing architectural inconsistency

### Solution

use only canonical rendering integration paths and remove legacy apis

### Signals

- legacy api
- hydration mismatch
- integration break

---

## CR-219 — Leptos Product Must Declare Consistent Ports Across Workspace and Local Config

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

ports differ between workspace and local config causing runtime inconsistency

### Solution

ensure site addr and reload port match across all configurations

### Signals

- wrong port
- connection refused
- reload failure

---

## CR-220 — Workspace Metadata Must Define Unique Build Targets Per Product

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

multiple products share same build output causing overwrite and corruption

### Solution

assign unique site root per product to isolate build outputs

### Signals

- artifact overwrite
- wrong wasm
- build conflict

---

## CR-221 — Leptos.toml Critical Fields Must Align With Workspace Metadata

- **Category:** build-tooling
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

critical config fields differ causing inconsistent builds and runtime mismatch

### Solution

ensure critical fields match exactly between workspace and local configs

### Signals

- feature mismatch
- port conflict
- build divergence

---

## CR-222 — Workspace Members Must Be Fully Resolvable by Cargo Metadata

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

workspace members do not resolve to valid cargo toml files causing incomplete builds

### Solution

ensure all workspace member paths resolve and appear in cargo metadata output

### Signals

- missing package
- partial build
- metadata mismatch

---

## CR-223 — Feature Flag Scopes Must Not Leak Between Products

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

feature flags leak between products causing incorrect builds and bloat

### Solution

define explicit package scoped feature flags with no cross contamination

### Signals

- feature leak
- wrong build
- dependency bloat

---

## CR-224 — Workspace Package Names Must Use Hyphens Consistently

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

mixed hyphen and underscore naming causes tool resolution errors

### Solution

use hyphens consistently across package bin and config names

### Signals

- binary not found
- name mismatch
- build error

---

## CR-225 — Workspace Dependency Version Policy Must Prevent ABI Drift Without Blocking Patch Compatibility

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

dependency versions drift causing abi conflicts and inconsistent builds

### Solution

pin minor versions allow patch updates and enforce lockfile with duplicate prevention

### Signals

- duplicate crate
- type mismatch
- abi conflict

---

## CR-226 — Workspace Leptos Paths Must Be Root-Relative

- **Category:** build-tooling
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

paths are product relative causing missing assets and build failures

### Solution

define all leptos paths relative to workspace root

### Signals

- 404 css
- missing assets
- build error

---

## CR-227 — Products Must Inherit Workspace Dependencies

- **Category:** build-tooling
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

dependencies are redeclared in products causing version drift

### Solution

use workspace true inheritance for all shared dependencies

### Signals

- version conflict
- duplicate dependency
- build failure

---

## CR-228 — Workspace Must Define WASM Target Configuration

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

wasm target is not configured causing build failures

### Solution

define wasm target configuration in cargo config with optimization flags

### Signals

- target not found
- build fail
- wasm error

---

## CR-229 — Design System CSS Must Be Consumed as an Independent Artifact

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

design system css is integrated into product build pipelines causing coupling

### Solution

consume design system css as standalone prebuilt artifact

### Signals

- path coupling
- build dependency
- css override

---

## CR-230 — WASM Artifact Budget Is a Hard Contract

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

wasm bundle grows without limits causing performance degradation

### Solution

define and enforce wasm size budget at build time with failure on exceed

### Signals

- large wasm
- slow load
- bundle bloat

---

## CR-231 — SSR Crates Must Be Link-Time Isolated From WASM Graph

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

server dependencies leak into wasm graph causing bloat and invalid builds

### Solution

enforce strict separation of ssr and wasm graphs through features bins and dependency isolation

### Signals

- wasm bloat
- unexpected dependency
- build anomaly

---

## CR-232 — Product Builds Must Be Deterministic From Workspace Root

- **Category:** build-tooling
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

builds depend on working directory causing inconsistent and failing builds

### Solution

execute all builds from workspace root using explicit root relative paths

### Signals

- path error
- ci failure
- inconsistent build

---

## CR-233 — CSS Cascade Order Is Build-Enforced Architecture

- **Category:** styling-css
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

css layers are loaded in wrong order causing inconsistencies and overrides

### Solution

enforce strict cascade order at bundler level with fixed sequence

### Signals

- token override
- theme break
- specificity issue

---

## CR-234 — Tokens Engine Is the Single Source of CSS Truth

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

tokens are defined outside canonical engine causing inconsistency

### Solution

generate all tokens exclusively from tokens engine and consume as artifact

### Signals

- token override
- duplicate definition
- css conflict

---

## CR-235 — UI Layer Must Consume Semantic Tokens, Never Theme Directly

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

ui consumes theme tokens directly causing tight coupling and instability

### Solution

restrict ui to semantic tokens only and isolate theme mapping

### Signals

- theme coupling
- hardcoded color
- refactor break

---

## CR-236 — CanonRS CLI Defines Workspace Topology

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

workspace structure is modified manually causing inconsistency and breakage

### Solution

delegate all workspace topology changes exclusively to cli

### Signals

- metadata mismatch
- structure drift
- build failure

---

## CR-237 — CLI Owns Leptos Metadata Blocks

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

leptos metadata is edited manually causing inconsistency and failures

### Solution

restrict all metadata changes to cli managed regions only

### Signals

- port mismatch
- config drift
- build issue

---

## CR-238 — Workspace Dependency Graph Must Be Acyclic

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

dependency cycles exist causing instability and architectural violations

### Solution

enforce directed acyclic graph with strict dependency direction

### Signals

- cycle detected
- build instability
- resolution error

---

## CR-239 — CLI Sync Operations Must Be Idempotent

- **Category:** governance
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

cli operations produce different outputs on repeated runs causing instability

### Solution

ensure cli sync produces identical results across executions

### Signals

- git diff
- unstable output
- rebuild loop

---

## CR-240 — Generated Artifacts Must Not Be Committed

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

generated files are committed causing conflicts drift and stale artifacts

### Solution

exclude all generated outputs via gitignore and regenerate during build or ci

### Signals

- merge conflict
- diff noise
- stale artifact

---

## CR-241 — Tokens Engine Is Build Infrastructure, Not Runtime Code

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

tokens engine is used at runtime causing dependency leakage and architectural violation

### Solution

restrict tokens engine to build time and prevent runtime imports

### Signals

- dependency leak
- runtime bloat
- build graph issue

---

## CR-242 — Cascade Order Is a Contract, Not a Preference

- **Category:** styling-css
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

css cascade order changes causing token conflicts and unstable ui

### Solution

enforce fixed cascade order generated by system and forbid manual changes

### Signals

- override failure
- theme leak
- visual instability

---

## CR-243 — data-theme Is the Only Theme Activation Boundary

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

themes use multiple activation mechanisms causing conflicts and leaks

### Solution

restrict theme activation exclusively to data theme attribute

### Signals

- theme leak
- partial override
- inconsistent behavior

---

## CR-244 — Semantic Layer Is a Mandatory Abstraction

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

components use theme tokens directly causing coupling and instability

### Solution

enforce semantic layer as mandatory bridge between theme and ui

### Signals

- token bypass
- theme coupling
- refactor break

---

## CR-245 — Families Must Not Leak Global State

- **Category:** design-system
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

family tokens override global layers causing unpredictable state

### Solution

restrict families to domain scoped tokens without overriding system layers

### Signals

- override conflict
- theme instability
- cascade break

---

## CR-246 — CSS Bundle Must Be Layer-Free

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

css bundle contains layer or import directives causing instability

### Solution

flatten bundle fully and remove all structural directives

### Signals

- layer directive
- import leak
- runtime instability

---

## CR-247 — CSS Entry Order Is Architectural, Not Cosmetic

- **Category:** styling-css
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

entry order changes cause instability and incorrect overrides

### Solution

enforce fixed entry generation order and forbid manual edits

### Signals

- override break
- token shadowing
- inconsistent ui

---

## CR-248 — Tokens Engine Is the Only Source of Truth

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

generated token files are edited manually causing drift and inconsistency

### Solution

restrict all token generation to engine and forbid manual edits

### Signals

- diff drift
- inconsistent css
- build mismatch

---

## CR-249 — Products Must Not Depend on .generated CSS

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

products depend on internal generated css causing coupling and fragility

### Solution

consume only final bundled css artifact and forbid internal layer usage

### Signals

- path coupling
- import break
- refactor failure

---

## CR-250 — Design System Must Not Depend on Products

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

design system depends on product crates causing cycles and instability

### Solution

enforce one directional dependency flow from core to products only

### Signals

- dependency cycle
- build instability
- feature leakage

---

## CR-251 — All Build Artifacts Must Be Statically Identifiable

- **Category:** build-tooling
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

artifacts use dynamic naming causing cache corruption and instability

### Solution

use deterministic naming conventions tied to source identity

### Signals

- cache issue
- mismatch artifact
- deployment confusion

---

## CR-252 — Theme Switching Must Be Attribute-Driven

- **Category:** design-system
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

themes are applied via javascript mutations causing instability and conflicts

### Solution

apply themes exclusively via dom data attributes and css cascade

### Signals

- hydration mismatch
- style conflict
- debug difficulty

---

## CR-253 — Token Cascade Order Is Immutable

- **Category:** styling-css
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

token cascade order is modified causing resolution failures

### Solution

enforce immutable cascade order through generators and ci checks

### Signals

- token mismatch
- silent regression
- style break

---

## CR-254 — CanonRS Must Build Independently of Products

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

design system depends on products causing coupling and circular dependencies

### Solution

ensure design system builds independently without product context

### Signals

- build failure
- dependency cycle
- artifact coupling

---

## CR-255 — CLI Is the Sole Authority Over Workspace Generation

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

workspace structure is modified manually causing instability and drift

### Solution

delegate all workspace generation exclusively to cli

### Signals

- config drift
- build mismatch
- manual override

---

## CR-256 — Generated Workspace Is Ephemeral and Immutable

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

generated workspace is treated as source causing drift and instability

### Solution

treat workspace as ephemeral artifact and regenerate via cli

### Signals

- manual edit
- git commit workspace
- build inconsistency

---

## CR-257 — Tokens Engine Is a Mandatory Pre-Build Step

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

tokens engine is not executed before build causing missing css

### Solution

enforce tokens engine execution before any build or dev process

### Signals

- missing css
- style break
- build warning

---

## CR-258 — Mode Drives Build Profiles

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

build profiles are defined manually causing mismatch with execution mode

### Solution

derive build profiles exclusively from mode configuration via cli

### Signals

- profile mismatch
- build inconsistency
- mode conflict

---

## CR-259 — Products Must Not Define Build Topology

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

products define build topology causing inconsistency and architectural drift

### Solution

delegate all build topology control exclusively to framework cli

### Signals

- config drift
- workspace mismatch
- build inconsistency

---

## CR-260 — CLI Autodiscovery Must Be Explicit or Fail

- **Category:** governance
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

cli uses silent fallback paths causing hidden misconfiguration

### Solution

enforce explicit discovery with hard failure when root is not found

### Signals

- misconfiguration
- unexpected path
- partial build

---

## CR-261 — CSS Bundle Size Drift Must Be Monitored

- **Category:** build-tooling
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

css bundle grows silently causing performance issues and bloat

### Solution

define size baseline and enforce limits via ci checks

### Signals

- bundle bloat
- slow load
- unexpected growth

---

## CR-262 — No Runtime CSS Regeneration

- **Category:** build-tooling
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

css is generated or mutated at runtime causing inconsistency and mismatch

### Solution

generate css only at build time and serve as static artifact

### Signals

- hydration mismatch
- style drift
- cache inconsistency

---

## CR-263 — Workspace Crate Boundaries Must Be Explicitly Declared

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

crates do not declare roles causing unclear responsibilities and coupling

### Solution

require explicit role metadata declaration for every crate

### Signals

- role ambiguity
- dependency confusion
- coupling risk

---

## CR-264 — Behavior Registry Is the Single Runtime Entry Point

- **Category:** behavior
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

behaviors are initialized in multiple places causing duplication and instability

### Solution

centralize all behavior initialization through a single registry bootstrap

### Signals

- duplicate listeners
- init conflict
- runtime anomaly

---

## CR-265 — Interactive Owns All State — Behavior Owns None

- **Category:** state-reactivity
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

behavior holds state causing divergence and inconsistent rendering

### Solution

restrict all state to interactive layer and keep behavior stateless

### Signals

- state divergence
- hydration mismatch
- debug difficulty

---

## CR-266 — Primitives Must Never Generate IDs

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

primitives generate ids causing mismatch between ssr and csr

### Solution

require ids to be passed externally and forbid internal generation

### Signals

- hydration warning
- id mismatch
- runtime conflict

---

## CR-267 — Data Attributes Are Contract, Not Styling Mechanism

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

classes are used to represent state causing coupling and ambiguity

### Solution

use data attributes to represent structural and behavioral contracts

### Signals

- class misuse
- state ambiguity
- coupling issue

---

## CR-268 — Behavior Cleanup Is Mandatory and Deterministic

- **Category:** behavior
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

event listeners and side effects are not cleaned up causing leaks and duplication

### Solution

implement deterministic cleanup using lifecycle hooks for all behaviors

### Signals

- memory leak
- duplicate listener
- event storm

---

## CR-269 — Custom Events Are Public Runtime Contracts

- **Category:** behavior
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

custom events lack naming contract causing instability and breakage

### Solution

use canonical namespaced event pattern canon component action

### Signals

- event mismatch
- silent failure
- integration break

---

## CR-270 — Behavior Must Never Traverse Beyond Its Container Scope

- **Category:** behavior
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

behavior queries global dom causing cross component interference

### Solution

restrict all dom queries to container scoped selectors

### Signals

- unexpected interaction
- coupling
- runtime bug

---

## CR-271 — Behavior Execution Must Be Idempotent

- **Category:** behavior
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

behavior registration runs multiple times causing duplication and instability

### Solution

guard registration using container flags to ensure single execution

### Signals

- duplicate event
- multiple trigger
- runtime anomaly

---

## CR-272 — Global Mutable State Is Forbidden in wasm Scope

- **Category:** state-reactivity
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

global mutable state exists causing race conditions and coupling

### Solution

store all state in reactive signals or scoped structures

### Signals

- race condition
- memory leak
- state conflict

---

## CR-273 — MutationObserver Requires Explicit Architectural Justification

- **Category:** behavior
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

mutationobserver is used without control causing performance degradation

### Solution

require explicit architectural approval before using mutationobserver

### Signals

- performance drop
- layout thrash
- observer spam

---

## CR-274 — Interactive Features Must Be Strictly Isolated

- **Category:** state-reactivity
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

features share state causing coupling and unpredictable regressions

### Solution

scope all signals and state per feature module

### Signals

- hidden dependency
- regression chain
- state conflict

---

## CR-275 — Interactive Hooks Must Be Container-Bound

- **Category:** state-reactivity
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

hooks attach globally causing interference and coupling

### Solution

bind all hooks explicitly to container id

### Signals

- unexpected behavior
- cross interaction
- state leak

---

## CR-276 — Interactive Must Remain Deterministic Under SSR Without wasm

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

interactive rendering depends on wasm causing mismatch between ssr and csr

### Solution

gate all wasm logic and ensure deterministic ssr output

### Signals

- hydration mismatch
- ssr divergence
- runtime inconsistency

---

## CR-277 — Interactive Must Not Emit DOM Events

- **Category:** behavior
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

interactive layer dispatches dom events directly breaking separation of concerns

### Solution

restrict dom event dispatching exclusively to behavior layer

### Signals

- custom event dispatch
- event bubbling misuse
- runtime coupling

---

## CR-278 — Primitive Must Not Encode Design Tokens in Rust

- **Category:** design-system
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

design tokens are hardcoded in rust primitives breaking theming and token cascade

### Solution

move all visual decisions to css using data attributes and token system

### Signals

- inline styles
- var(-- usage in rust
- hardcoded classes

---

## CR-279 — UI Layer Must Not Be Aware of Behavior Layer

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

ui layer references behavior layer causing coupling and ssr issues

### Solution

enforce strict separation where ui communicates only via attributes and behavior reacts independently

### Signals

- behavior import in ui
- conditional runtime logic
- tight coupling

---

## CR-280 — Interactive Must Be Safe Under Feature Flag Removal

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

feature flags change dom structure causing hydration mismatch

### Solution

preserve structural output and control behavior via attributes

### Signals

- hydration mismatch
- dom divergence
- conditional rendering bugs

---

## CR-281 — Cross-Crate Layer Leakage Is Forbidden

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

cross crate access to internal layers breaks modular architecture

### Solution

restrict access to public apis and enforce visibility boundaries

### Signals

- internal module import
- tight coupling
- semver break risk

---

## CR-282 — No Raw Text Nodes in SSR Boundaries

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

text nodes rendered in SSR differ from expected element nodes in hydration phase

### Solution

always wrap text content in explicit HTML elements

### Signals

- hydration panic
- expected text node mismatch
- tachys failed_to_cast_text_node

---

## CR-283 — Stable DOM Structure Required for Hydration

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

ssr and wasm generate different dom trees causing hydration mismatch

### Solution

ensure identical node hierarchy regardless of state

### Signals

- hydration panic
- dom mismatch warning
- inconsistent node tree

---

## CR-284 — Children Must Be Structurally Stable

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

optional children modify node count causing mismatch between ssr and client

### Solution

ensure children count and position remain stable regardless of state

### Signals

- hydration mismatch
- missing node errors
- inconsistent children length

---

## CR-285 — Overlay Must Use Container Pattern

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

overlay implemented as standalone content breaks layering and interaction model

### Solution

use container pattern separating content and overlay layers

### Signals

- overlay not blocking interaction
- overlay rendered as content
- layering issues

---

## CR-286 — Overlay Must Block Interaction

- **Category:** behavior
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

overlay does not block pointer events allowing unintended interaction

### Solution

enforce pointer-event blocking and layering via z-index

### Signals

- click through overlay
- focus leakage
- interaction during loading

---

## CR-287 — State Must Control Visibility

- **Category:** state-reactivity
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

state is defined but does not affect visibility or rendering

### Solution

bind state directly to DOM visibility via attributes or CSS selectors

### Signals

- state changes without visual effect
- inconsistent UI behavior
- hidden elements still interactive

---

## CR-288 — aria-hidden Must Match Visibility

- **Category:** accessibility
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

aria-hidden does not match real visibility state

### Solution

synchronize aria-hidden with actual DOM visibility

### Signals

- screen reader reads hidden content
- accessibility audit failures
- inconsistent aria state

---

## CR-289 — Overlay Must Be Visually Present

- **Category:** design-system
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

overlay exists structurally but has no visual representation

### Solution

ensure overlay has visible feedback such as spinner, backdrop, or indicator

### Signals

- invisible overlay
- user confusion
- no loading feedback

---

## CR-290 — Interactive Components Must Emit Events

- **Category:** behavior
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

components mutate state without emitting events

### Solution

emit standardized events such as rs-change on state updates

### Signals

- no event emitted
- integration failure
- non-reactive components

---

## CR-291 — data-rs-value Is Canonical Output

- **Category:** behavior
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

component state is not accessible externally

### Solution

expose value via data-rs-value attribute

### Signals

- missing data-rs-value
- integration failure
- state inaccessible

---

## CR-292 — Select Is Canonical Interaction Model

- **Category:** behavior
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

components implement inconsistent interaction patterns

### Solution

standardize all interactions on Select model: state + value + event

### Signals

- inconsistent component behavior
- missing events
- integration failures

---

## CR-293 — No Duplicate State Outside DOM

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

state exists in multiple sources causing desynchronization

### Solution

use DOM as single source of truth via data attributes

### Signals

- inconsistent UI state
- mismatch between DOM and logic
- stale values

---

## CR-294 — No Hidden Interactive Elements

- **Category:** accessibility
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

interactive elements exist inside hidden containers

### Solution

disable or remove interactivity when element is hidden

### Signals

- focus leaks
- keyboard navigation issues
- accessibility violations

---

## CR-295 — No Conditional Rendering for Structure

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

if/else modifies node structure between SSR and client

### Solution

use stable structure and conditionally change content or visibility

### Signals

- hydration mismatch
- missing nodes
- runtime panic

---

## CR-296 — Components Must Be Composable

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

components cannot interact with others

### Solution

design components with standardized contracts

### Signals

- isolated components
- integration failure
- duplicated logic

---

## CR-297 — Overlay State Must Be Deterministic

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

overlay visibility depends on implicit logic

### Solution

bind overlay behavior strictly to explicit state

### Signals

- inconsistent overlay visibility
- unpredictable UI
- state mismatch

---

## CR-298 — DOM Is Single Source of Truth

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

state is stored outside DOM and becomes inconsistent

### Solution

use DOM attributes as canonical state representation

### Signals

- state divergence
- inconsistent rendering
- debugging complexity

---

## CR-299 — No Implicit Visual State

- **Category:** design-system
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

visual changes occur without explicit state mapping

### Solution

map all visual changes to explicit state attributes

### Signals

- inconsistent styling
- unclear state representation
- css conflicts

---

## CR-300 — No Side Effects in Primitives

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

primitives contain logic or side effects

### Solution

keep primitives pure and move logic to behaviors

### Signals

- unpredictable rendering
- hydration issues
- side effects in view

---

## CR-301 — Events Must Be Standardized

- **Category:** behavior
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

custom or inconsistent event naming breaks integration

### Solution

use canonical event names such as rs-change

### Signals

- inconsistent event names
- integration issues
- duplicated logic

---

## CR-302 — State Must Be Serializable in DOM

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

component state is hidden in memory and not exposed

### Solution

serialize all state into data-rs-* attributes

### Signals

- hydration mismatch
- integration failure
- state inaccessible

---

## CR-303 — No Random or Time-Based Rendering

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

render output differs between server and client

### Solution

remove randomness or isolate to client-only execution

### Signals

- hydration mismatch
- flickering UI
- inconsistent DOM

---

## CR-304 — No Optional DOM Nodes

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

DOM structure differs between SSR and client

### Solution

always render nodes and control visibility via attributes

### Signals

- hydration panic
- node mismatch
- inconsistent structure

---

## CR-305 — All Text Must Be Wrapped

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

text nodes mismatch between SSR and client

### Solution

wrap all text inside explicit elements

### Signals

- expected text node error
- hydration panic
- string mismatch

---

## CR-306 — Components Must Declare Behavior

- **Category:** behavior
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

behavior is implicit or missing

### Solution

declare behavior via data-rs-behavior

### Signals

- missing interaction
- behavior not attached
- inconsistent UX

---

## CR-307 — No Inline Style for Logic

- **Category:** styling-css
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

logic embedded in style attributes

### Solution

move logic to state attributes and CSS

### Signals

- style conditions
- inconsistent rendering
- duplication

---

## CR-308 — All Interactions Must Emit Events

- **Category:** behavior
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

state changes are not externally observable

### Solution

emit canonical events after state changes

### Signals

- silent updates
- integration failure
- missing reactions

---

## CR-309 — No Component-Specific Event Names

- **Category:** governance
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

components emit custom event names

### Solution

use canonical event names

### Signals

- inconsistent events
- integration complexity
- duplicated logic

---

## CR-310 — Components Must Be SSR Safe

- **Category:** core-runtime
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

component output differs between environments

### Solution

ensure deterministic and environment-independent rendering

### Signals

- hydration mismatch
- runtime panic
- inconsistent UI

---

## CR-311 — No Implicit DOM Order Dependency

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

logic relies on child position

### Solution

use explicit attributes and selectors

### Signals

- fragile components
- order-based bugs
- unpredictable behavior

---

## CR-312 — Behaviors Must Be Idempotent

- **Category:** behavior
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

multiple attachments create duplicated listeners and inconsistent state

### Solution

ensure behavior attachment is idempotent using guards

### Signals

- duplicated events
- multiple listeners firing
- unstable toggle behavior

---

## CR-313 — No DOM Mutation Outside Behavior Layer

- **Category:** component-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

components mutate DOM directly

### Solution

restrict DOM mutation to behavior layer only

### Signals

- unpredictable UI updates
- duplicated logic
- inconsistent DOM state

---

## CR-314 — State Must Not Live in Behavior

- **Category:** behavior
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

state stored inside behavior logic

### Solution

externalize state to DOM attributes

### Signals

- hidden state
- inconsistent behavior
- debugging difficulty

---

## CR-315 — Every State Change Must Update DOM

- **Category:** state-reactivity
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

state changes not reflected in DOM

### Solution

update data-rs-* attributes on every change

### Signals

- stale UI
- mismatch between state and DOM
- integration failure

---

## CR-316 — No Implicit Default State

- **Category:** state-reactivity
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

state defaults are implicit

### Solution

always define explicit initial state

### Signals

- inconsistent initial render
- unpredictable behavior
- missing attributes

---

## CR-317 — All Components Must Be Composable

- **Category:** component-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

components cannot be reused or composed

### Solution

design components with explicit contracts

### Signals

- rigid components
- duplication
- poor reuse

---

## CR-318 — No Hidden Side Effects

- **Category:** state-reactivity
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

side effects occur implicitly

### Solution

make all side effects explicit

### Signals

- unexpected updates
- difficult debugging
- inconsistent state

---

## CR-319 — Behaviors Must Be Stateless

- **Category:** behavior
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

behavior stores internal state

### Solution

derive state from DOM

### Signals

- inconsistent behavior
- state desync
- debugging difficulty

---

## CR-320 — DOM Is the Single Source of Truth

- **Category:** governance
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

multiple sources of truth exist

### Solution

centralize all state in DOM

### Signals

- state divergence
- inconsistent UI
- integration issues

---

## CR-321 — Integration Must Be Declarative

- **Category:** governance
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

integration is manual and imperative

### Solution

use declarative data-rs-* attributes

### Signals

- complex wiring
- tight coupling
- brittle integrations

---

## CR-322 — Island DOM Shape Must Be Static

- **Category:** islands-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

islands with dynamic DOM shape cause hydration mismatch and runtime panic

### Solution

keep island view structure static and fixed

### Signals

- failed_to_cast_marker_node
- hydration panic
- tachys cursor error

---

## CR-323 — Island Props Must Be Serializable

- **Category:** islands-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

custom types without serde cause compile errors in island props

### Solution

derive serde traits or use primitive-compatible types

### Signals

- island compile error
- trait bound not satisfied
- failed to parse path

---

## CR-324 — Island View Must Be Isomorphic

- **Category:** islands-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

SSR and hydrate produce different HTML causing hydration panic

### Solution

initialize signals with deterministic values derived from props

### Signals

- hydration mismatch
- cursor panic
- different attribute values SSR vs client

---

## CR-325 — Dynamic Lists Must Live Outside Islands

- **Category:** islands-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

iterators, For component and inner_html inside islands break hydration

### Solution

move dynamic lists to SSR shell components outside the island

### Signals

- failed_to_cast_marker_node at list position
- hydration panic on collection render
- tachys cursor error inside island

---

## CR-326 — Island Event Handlers Must Be Cfg-Gated

- **Category:** islands-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

web-sys and wasm-bindgen APIs are unavailable during SSR compilation

### Solution

gate all client-only logic with cfg(feature = hydrate)

### Signals

- web_sys not found in SSR build
- wasm_bindgen unavailable
- JsCast not in scope

---

## CR-327 — Signals Are the SSOT for Island State

- **Category:** islands-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

DOM mutation and attribute reading as state source breaks the reactive model

### Solution

derive all state from signals initialized from props

### Signals

- reading data-rs-* attributes as state
- set_attribute called directly
- DOM scanning inside island

---

## CR-328 — Island Crate Must Be Linked in WASM Entry Point

- **Category:** islands-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

islands defined in a separate crate are silently excluded from the WASM bundle

### Solution

add the island crate as a dependency and use it explicitly in the client crate

### Signals

- island function missing from generated JS
- window.__leptos_islands undefined
- island never hydrates

---

## CR-329 — HydrationScripts Must Declare Islands Mode

- **Category:** islands-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

without islands=true the runtime does not connect leptos-island elements to WASM functions

### Solution

pass islands=true to HydrationScripts in the app shell

### Signals

- islands render as static HTML
- no interaction after hydration
- window.__leptos_islands undefined

---

## CR-330 — Island Props Must Use Serde Enums

- **Category:** islands-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

`Option<String>` and `Option<bool>` with `#[prop(optional)]` always serialize as `null` in `data-props`, making the value invisible to the client after hydration.

### Solution

Define an enum with `#[derive(serde::Serialize, serde::Deserialize)]` for every prop that carries meaningful state across the SSR boundary.

### Signals

- prop always appears as `null` in `data-props` HTML attribute
- client island never receives the intended value
- state appears correct in SSR but resets after hydration

---

## CR-331 — Island SSR State Must Be Fully Materialized Without Signals

- **Category:** islands-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

Signal values like `is_hover.get()` return their initial value during SSR but the rendered attribute may be empty if the closure does not account for static initial state separately.

### Solution

Pre-compute an `initial_state` string from all static booleans and use it as a fallback in the reactive closure.

### Signals

- `data-rs-state=""` in SSR output even when initial state was passed
- state appears only after user interaction, not on page load
- CSS that depends on initial state does not apply on first render

---

## CR-332 — Group Override Selector Must Match Base Specificity

- **Category:** css-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

A group selector with fewer conditions than the child's base selector loses the cascade silently. The override is present in the CSS but never applies.

### Solution

Replicate all `:not()` conditions from the base child selector in the group override selector.

### Signals

- group override CSS is present in the bundle but has no visual effect
- computed styles show the child's base rule winning
- adding `!important` fixes it (forbidden — signals a specificity problem)

---

## CR-333 — Island CSS Must Use Descendant Selector Not Child Combinator

- **Category:** css-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

A CSS rule using `>` between a parent component and an island child will never match because the DOM contains intermediate Leptos wrapper elements.

### Solution

Always use the descendant combinator (space) when writing selectors that span across island boundaries.

### Signals

- CSS rule is present in bundle but never applies to island children
- rule works in isolation but breaks when component is wrapped in an island
- DevTools shows the selector does not match despite correct DOM attributes

---

## CR-334 — Island State Must Propagate Via Context

- **Category:** islands-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

Islands cannot share state via SSR component tree. Passing signals as props is not supported. DOM mutation as a workaround breaks the SSOT principle.

### Solution

Root island publishes state via `provide_context`. Child islands consume via `use_context` captured before any closure.

### Signals

- child island does not react to parent state changes
- state shared via DOM attributes instead of signals
- `use_context` called inside event handler closure

---

## CR-335 — Island Must Not Own UI Structure When Content Is Compositional

- **Category:** islands-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

Islands receive `Vec<TabItem { content: String }>` forcing all content to be serialized as plain strings, losing Leptos component composition.

### Solution

Island wraps children via `Children`. SSR defines structure. Island provides context for state.

### Signals

- island prop contains HTML as string
- `Vec<T>` with content fields passed to island
- island renders cards, tables, or grids internally

---

## CR-336 — Token References Must Resolve to Existing Design Tokens

- **Category:** tokens
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

`--tabs-trigger-bg-active: var(--theme-primary-bg)` — `--theme-primary-bg` does not exist in the token system. The tab active state renders incorrectly with no build error.

### Solution

Before declaring a token reference, verify the target token exists. Use the same token chain used by equivalent components (e.g. Button primary uses `--theme-action-primary-bg`).

### Signals

- active state renders with wrong color
- component visually inconsistent with equivalent components
- no build error despite broken visual

---

## CR-337 — Hover State Must Not Override Active State

- **Category:** css-contracts
- **Severity:** MEDIUM
- **Status:** ENFORCED

### Problem

`[data-rs-tabs-trigger]:hover` overrides `[data-rs-tabs-trigger][data-rs-state~="active"]` when the user hovers over the active tab, removing the primary color and replacing it with the muted hover style.

### Solution

All hover selectors on stateful components must exclude the active state via `:not([data-rs-state~="active"])`.

### Signals

- active tab loses its color when hovered
- selected item visually deselects on mouse-over
- hover and active styles conflict

---

## CR-338 — Island Boundary Rule

- **Category:** leptos-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

A `#[island]` wrapping layout structure causes Leptos to emit an empty shell in SSR. The DOM never reaches the browser on first paint. Flex chains, height inheritance, and CSS selectors all fail silently.

### Solution

Split every interactive component into three layers: SSR layout component, minimal init island, and optional interaction module.

### Signals

- component renders empty in SSR curl output
- flex/grid layout collapses despite correct CSS
- `height: 100%` has no effect on island children
- `display: contents` on `leptos-island` is used as a workaround (forbidden)

---

## CR-339 — Pointer Capture Must Register Move and Up on Document

- **Category:** interaction-architecture
- **Severity:** HIGH
- **Status:** ENFORCED

### Problem

`pointermove` registered on the drag handle stops firing when the pointer moves faster than the element boundary. The drag freezes mid-interaction even though `setPointerCapture` was called.

### Solution

Register `pointerdown` on the handle. Register `pointermove` and `pointerup` on `document`. Use `setPointerCapture` on the handle to ensure the pointer ID is tracked correctly.

### Signals

- drag stops when moving fast
- bar does not follow pointer outside element bounds
- `pointerup` not detected when releasing outside the handle
- interaction works correctly on slow movement but breaks on fast movement

---

## CR-340 — Passthrough Island Must Be Zero Logic

- **Category:** island-architecture
- **Severity:** CRITICAL
- **Status:** ENFORCED

### Problem

When passthrough islands perform transformations such as string parsing, enum mapping, default resolution, or conditional rendering, they break the CanonRS architecture by introducing logic into a layer that must remain purely mechanical.

### Solution

Passthrough islands must accept fully typed props (enums, booleans, structured data) and forward them directly to UI components. All transformations must occur before the island (call site) or inside the UI layer.

### Signals

- usage of `match` inside island
- usage of `unwrap_or`, `unwrap_or_default`, or fallback logic
- string → enum conversion inside island
- conditional rendering branches (`if`, `match`) in island
- presence of parsing or normalization logic
- island API accepts `String` where enum exists

---

