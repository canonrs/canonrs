# CanonRS Rules — Classification by Tag

Index of all Canon Rules grouped by scope tag.

**Total rules:** 276  
**Total tags:** 26  
**Last updated:** 2026-03-28

---

## Tags

- [accessibility](#accessibility)
- [architecture](#architecture)
- [behavior](#behavior)
- [build](#build)
- [cli](#cli)
- [components](#components)
- [csr](#csr)
- [css](#css)
- [design-system](#design-system)
- [governance](#governance)
- [hydration](#hydration)
- [interactive](#interactive)
- [layout](#layout)
- [leptos](#leptos)
- [motion](#motion)
- [overlays](#overlays)
- [pages](#pages)
- [primitives](#primitives)
- [providers](#providers)
- [ssr](#ssr)
- [state](#state)
- [theming](#theming)
- [tokens](#tokens)
- [ui](#ui)
- [wasm](#wasm)
- [workspace](#workspace)

---

## accessibility

**3 rules**

- `canon-rule-31-accessibility-contract.md` — #31: Accessibility Contract (ARIA + Roles) _CRITICAL_
- `canon-rule-33-density-accessibility-mapping.md` — #33: Density & Accessibility Mapping _HIGH_
- `canon-rule-72-layout-h1-prohibition.md` — #72: Layout H1 Prohibition _MEDIUM_

---

## architecture

**68 rules**

- `canon-rule-01-types.md` — #01: Component Types _HIGH_
- `canon-rule-13-specialization-vs-substitution.md` — #13 — Specialization vs Substitution _HIGH_
- `canon-rule-16-client-vs-server-filtering.md` — #16 — Client-side vs Server-side Filtering _MEDIUM_
- `canon-rule-18-client-vs-server-sorting.md` — #18 — Sorting: Client vs Server _MEDIUM_
- `canon-rule-37-provider-taxonomy-boundaries.md` — #37: Provider Taxonomy & Boundaries _HIGH_
- `canon-rule-43-domain-components-and-commands.md` — #43: Domain Components and Commands _HIGH_
- `canon-rule-44-orchestrators.md` — #44: Orchestrators _HIGH_
- `canon-rule-48-context-provider.md` — #48: Context Provider Pattern _HIGH_
- `canon-rule-50-provider-singleton-pattern.md` — #50: Provider Singleton & Runtime Separation _HIGH_
- `canon-rule-59-css-cascade-ownership.md` — #59: CSS Cascade Ownership _HIGH_
- `canon-rule-62-single-source-of-truth-tokens.md` — #62: Single Source of Truth for Design Tokens _CRITICAL_
- `canon-rule-75-primitive-css-prohibition.md` — #75: Primitive CSS Prohibition _CRITICAL_
- `canon-rule-77-monorepo-path-stability.md` — #77: Monorepo Path Stability _HIGH_
- `canon-rule-83-layout-zones-contract.md` — #83: Layout Zones Contract _CRITICAL_
- `canon-rule-98-axum-leptos-ssr-closures-must-own-state.md` — #98: Axum + Leptos SSR Closures Must Own State _MEDIUM_
- `canon-rule-105-visual-indicators-must-have-a-single-owner.md` — #105 — Visual Indicators Must Have a Single Owner _HIGH_
- `canon-rule-107-token-architecture-theme-specificity.md` — #107: Token Architecture & Theme Specificity _CRITICAL_
- `canon-rule-111-model-first-css-second.md` — #111: Model First, CSS Second _CRITICAL_
- `canon-rule-118-view-type-boundary.md` — #118: View<\_> Type Boundary Prohibition _CRITICAL_
- `canon-rule-123-component-architecture-taxonomy.md` — #123: Component Architecture Taxonomy and Contracts _CRITICAL_
- `canon-rule-135-ui-vs-layout-responsibility-boundary.md` — #135: UI vs Layout Responsibility Boundary _CRITICAL_
- `canon-rule-140-layouts-are-structural-not-behavioral.md` — #140: Layouts Are Structural, Not Behavioral _CRITICAL_
- `canon-rule-141-csr-composition-belongs-to-app-layer.md` — #141: CSR Composition Belongs to the App Layer _CRITICAL_
- `canon-rule-142-children-must-always-be-optional.md` — #142 — Children Must Always Be Optional in Public Components _HIGH_
- `canon-rule-144-providers-expose-state-apps-own-interaction.md` — #144 — Providers Expose State, Applications Own Interaction _HIGH_
- `canon-rule-145-ui-must-not-mutate-global-state-implicitly.md` — #145 — UI Components Must Not Mutate Global State Implicitly _HIGH_
- `canon-rule-148-ui-must-never-infer-intent-from-state.md` — #148 — UI Must Never Infer Intent From State _HIGH_
- `canon-rule-149-controllers-must-be-temporal-not-structural.md` — #149 — Controllers Must Be Temporal, Not Structural _HIGH_
- `canon-rule-158-design-system-packages-immutable-contracts.md` — #158: Design System Packages Are Immutable Contracts (No Direct File Imports) _CRITICAL_
- `canon-rule-162-providers-not-ui.md` — #162: Providers Are Infrastructure, Not UI _HIGH_
- `canon-rule-165-workbench-is-canon.md` — #165: CanonRS Workbench Is a Canonical Reference _HIGH_
- `canon-rule-167-canonrs-css-single-entrypoint.md` — #167: CanonRS CSS Has a Single Entrypoint _HIGH_
- `canon-rule-172-site-does-not-own-design.md` — #172: The Site Does Not Own Design _HIGH_
- `canon-rule-173-css-is-a-first-class-system.md` — #173: CSS Is a First-Class System _HIGH_
- `canon-rule-174-tokens-are-compile-time-contracts.md` — #174 — Tokens Are Compile-Time Contracts _BLOCKING_
- `canon-rule-176-governance-is-the-single-source-of-truth.md` — #176: Governance Is the Single Source of Truth _CRITICAL_
- `canon-rule-181-token-cascade-is-architecture-not-convention.md` — #181: Token Cascade Is Architecture, Not Convention _CRITICAL_
- `canon-rule-183-overlay-positioning-is-a-primitive.md` — #183: Overlay Positioning Is a Primitive, Not a Controller Concern _CRITICAL_
- `canon-rule-188-overlay-positioning-is-not-ui-logic.md` — #188: Overlay Positioning Is Not UI Logic _HIGH_
- `canon-rule-191-pages-use-page-behaviors-only.md` — #191: Pages Must Delegate Wiring to Page Behaviors Only _CRITICAL_
- `canon-rule-192-page-behaviors-do-not-implement-logic.md` — #192: Page Behaviors Must Not Implement New Logic _CRITICAL_
- `canon-rule-193-framework-behaviors-are-the-only-source-of-interactivity.md` — #193: Framework Behaviors Are the Only Source of Interactivity _CRITICAL_
- `canon-rule-194-ui-components-are-pure-render-functions.md` — #194: UI Components Are Pure Render Functions _CRITICAL_
- `canon-rule-196-ssr-csr-separation-wasm-bloat-prevention.md` — #196: SSR/CSR Separation for WASM Bloat Prevention _CRITICAL_
- `canon-rule-197-feature-flags-are-architectural-boundaries.md` — #197: Feature Flags Are Architectural Boundaries _CRITICAL_
- `canon-rule-198-runtime-crates-must-not-leak-cross-target-types.md` — #198: Runtime Crates Must Not Leak Cross-Target Types _CRITICAL_
- `canon-rule-199-server-adapters-are-integration-code.md` — #199: Server Adapters Are Integration Code _HIGH_
- `canon-rule-200-shared-crates-must-be-zero-dependency-runtime.md` — #200: Shared Crates Must Be Zero-Dependency Runtime _CRITICAL_
- `canon-rule-207-preset-switching-never-changes-component-css.md` — #207: Preset Switching Must Never Change Component CSS _CRITICAL_
- `canon-rule-208-working-theme-toggle-proves-correct-token-architecture.md` — #208: A Working Theme Toggle Is Proof of Correct Token Architecture _HIGH_
- `canon-rule-214-lib-owns-app-structure.md` — #214: lib.rs Owns Application Structure and UI Semantics _CRITICAL_
- `canon-rule-215-main-owns-runtime-bootstrap.md` — #215: main.rs Owns Runtime and Server Bootstrap Exclusively _CRITICAL_
- `canon-rule-234-tokens-engine-single-source-of-truth.md` — #234: Tokens Engine Is the Single Source of CSS Truth _CRITICAL_
- `canon-rule-236-cli-defines-workspace-topology.md` — #236: CanonRS CLI Defines Workspace Topology _CRITICAL_
- `canon-rule-238-workspace-graph-must-be-acyclic.md` — #238: Workspace Dependency Graph Must Be Acyclic _CRITICAL_
- `canon-rule-241-tokens-engine-is-build-step-not-runtime.md` — #241: Tokens Engine Is Build Infrastructure, Not Runtime Code _CRITICAL_
- `canon-rule-242-cascade-order-is-a-contract.md` — #242: Cascade Order Is a Contract, Not a Preference _CRITICAL_
- `canon-rule-243-data-theme-is-the-only-activation-boundary.md` — #243: data-theme Is the Only Theme Activation Boundary _CRITICAL_
- `canon-rule-244-semantic-layer-is-mandatory-abstraction.md` — #244: Semantic Layer Is a Mandatory Abstraction _CRITICAL_
- `canon-rule-245-families-must-not-leak-global-state.md` — #245: Families Must Not Leak Global State _HIGH_
- `canon-rule-249-products-must-not-depend-on-generated-css.md` — #249: Products Must Not Depend on .generated CSS _CRITICAL_
- `canon-rule-250-design-system-must-not-depend-on-products.md` — #250: Design System Must Not Depend on Products _CRITICAL_
- `canon-rule-253-token-cascade-order-is-immutable.md` — #253: Token Cascade Order Is Immutable _CRITICAL_
- `canon-rule-259-products-must-not-define-build-topology.md` — #259: Products Must Not Define Build Topology _CRITICAL_
- `canon-rule-262-no-runtime-css-regeneration.md` — #262: No Runtime CSS Regeneration _CRITICAL_
- `canon-rule-263-workspace-must-declare-crate-boundary-visibility.md` — #263: Workspace Crate Boundaries Must Be Explicitly Declared _CRITICAL_
- `canon-rule-279-ui-must-not-know-behavior.md` — #279: UI Layer Must Not Be Aware of Behavior Layer _CRITICAL_
- `canon-rule-281-no-cross-crate-layer-leaks.md` — #281: Cross-Crate Layer Leakage Is Forbidden _CRITICAL_

---

## behavior

**22 rules**

- `canon-rule-05-ssr-effects.md` — #05: SSR Effects and Browser API Safety _CRITICAL_
- `canon-rule-09-clipboard-apis.md` — #09: Clipboard and Browser APIs _MEDIUM_
- `canon-rule-11-multi-callback-ownership.md` — #11: Multi-Callback Ownership _HIGH_
- `canon-rule-49-drag-drop-as-intent.md` — #49: Drag & Drop as Intent (Not Action) _MEDIUM_
- `canon-rule-51-callbacks-as-commands.md` — #51: Callbacks as Commands _MEDIUM_
- `canon-rule-102-runtime-js-is-shell-infrastructure.md` — #102: Runtime JS Is Shell Infrastructure _CRITICAL_
- `canon-rule-120-dom-events-vs-semantic-callbacks.md` — #120: DOM Events vs Semantic Callbacks Boundary _HIGH_
- `canon-rule-129-ssr-event-safety.md` — #129: SSR Event Safety _CRITICAL_
- `canon-rule-136-controllers-are-csr-only.md` — #136: Controllers Are CSR-Only _CRITICAL_
- `canon-rule-191-pages-use-page-behaviors-only.md` — #191: Pages Must Delegate Wiring to Page Behaviors Only _CRITICAL_
- `canon-rule-192-page-behaviors-do-not-implement-logic.md` — #192: Page Behaviors Must Not Implement New Logic _CRITICAL_
- `canon-rule-193-framework-behaviors-are-the-only-source-of-interactivity.md` — #193: Framework Behaviors Are the Only Source of Interactivity _CRITICAL_
- `canon-rule-195-interactive-components-require-explicit-id.md` — #195: Interactive Components Require Explicit ID _HIGH_
- `canon-rule-264-behavior-registry-is-single-entry-point.md` — #264: Behavior Registry Is the Single Runtime Entry Point _CRITICAL_
- `canon-rule-265-interactive-owns-state-behavior-does-not.md` — #265: Interactive Owns All State — Behavior Owns None _CRITICAL_
- `canon-rule-268-behavior-cleanup-is-mandatory.md` — #268: Behavior Cleanup Is Mandatory and Deterministic _CRITICAL_
- `canon-rule-269-custom-events-are-runtime-contracts.md` — #269: Custom Events Are Public Runtime Contracts _HIGH_
- `canon-rule-270-behavior-dom-scope-isolation.md` — #270: Behavior Must Never Traverse Beyond Its Container Scope _CRITICAL_
- `canon-rule-271-behavior-idempotent-registration.md` — #271: Behavior Execution Must Be Idempotent _CRITICAL_
- `canon-rule-272-no-global-mutable-wasm-state.md` — #272: Global Mutable State Is Forbidden in wasm Scope _CRITICAL_
- `canon-rule-273-mutationobserver-explicit-justification.md` — #273: MutationObserver Requires Explicit Architectural Justification _HIGH_
- `canon-rule-279-ui-must-not-know-behavior.md` — #279: UI Layer Must Not Be Aware of Behavior Layer _CRITICAL_

---

## build

**55 rules**

- `canon-rule-22-tailwind-v4-rust-integration.md` — #22: Tailwind v4 + Rust Integration _CRITICAL_
- `canon-rule-55-canonical-css-entry-points.md` — #55: Canonical CSS Entry Points _HIGH_
- `canon-rule-56-monorepo-css-build-pipeline.md` — #56: Monorepo CSS Build Pipeline _HIGH_
- `canon-rule-57-postcss-canon-config.md` — #57: PostCSS Canon Configuration _MEDIUM_
- `canon-rule-58-leptos-assets-dev-constraint.md` — #58: Leptos Assets Dev Constraint _MEDIUM_
- `canon-rule-60-css-artifacts-are-immutable.md` — #60: CSS Artifacts Are Immutable _HIGH_
- `canon-rule-61-no-relative-css-imports.md` — #61: No Relative CSS Imports _MEDIUM_
- `canon-rule-64-css-build-pipeline-mandatory.md` — #64: CSS Build Pipeline is Mandatory _CRITICAL_
- `canon-rule-66-workbench-setup-checklist.md` — #66: Workbench Setup Checklist _LOW_
- `canon-rule-68-asset-must-exist-in-dist.md` — #68: Asset Must Exist in Final dist/ _HIGH_
- `canon-rule-69-trunk-only-serves-dist.md` — #69: Trunk Only Serves What's in dist/ _HIGH_
- `canon-rule-70-css-pipeline-health-checks.md` — #70: CSS Pipeline Requires Health Checks _MEDIUM_
- `canon-rule-80-workspace-watch-configuration.md` — #80: Workspace Watch Configuration _MEDIUM_
- `canon-rule-82-css-build-pipeline-health.md` — #82: CSS Build Pipeline Health _HIGH_
- `canon-rule-85-leptos-asset-pipeline-dev.md` — #85: Leptos Asset Pipeline in Dev Mode _CRITICAL_
- `canon-rule-93-leptos-wasm-dev-builds-must-use-release-mode.md` — #93: Leptos WASM Dev Builds Must Use Release Mode _CRITICAL_
- `canon-rule-100-build-orchestrators-must-be-workspace-scoped.md` — #100: Build Orchestrators Must Be Workspace-Scoped _MEDIUM_
- `canon-rule-101-workbench-assets-must-be-product-scoped.md` — #101: Workbench Assets Must Be Product-Scoped _MEDIUM_
- `canon-rule-104-autoreload-breaks-script-order-guarantees.md` — #104: AutoReload Breaks Script Order Guarantees _HIGH_
- `canon-rule-157-design-system-css-build-time-flattened.md` — #157: Design System CSS Must Be Build-Time Flattened _CRITICAL_
- `canon-rule-160-css-bootstrap-no-build.md` — #160: First App Must Not Require CSS Build _CRITICAL_
- `canon-rule-164-pkg-served-by-ssr.md` — #164: WASM and JS Assets Must Be Served by SSR _CRITICAL_
- `canon-rule-166-dist-is-readonly.md` — #166: Dist Is Read-Only _CRITICAL_
- `canon-rule-174-tokens-are-compile-time-contracts.md` — #174 — Tokens Are Compile-Time Contracts _BLOCKING_
- `canon-rule-177-canonrs-css-generated-artifact-never-design-surface.md` — #177: canonrs.css Is a Generated Artifact, Never a Design Surface _CRITICAL_
- `canon-rule-196-ssr-csr-separation-wasm-bloat-prevention.md` — #196: SSR/CSR Separation for WASM Bloat Prevention _CRITICAL_
- `canon-rule-197-feature-flags-are-architectural-boundaries.md` — #197: Feature Flags Are Architectural Boundaries _CRITICAL_
- `canon-rule-204-theme-files-must-be-last-in-cascade.md` — #204: Theme Files Must Be Last in the Cascade _HIGH_
- `canon-rule-209-axum-version-must-match-adapter.md` — #209: Axum Version Must Match Adapter Version _CRITICAL_
- `canon-rule-210-cdylib-mandatory-for-hydration.md` — #210: cdylib Is Mandatory for Hydration _CRITICAL_
- `canon-rule-217-entrypoints-must-be-explicit.md` — #217: Entry Points Must Be Explicit, Public, and Isolated _CRITICAL_
- `canon-rule-220-unique-build-targets.md` — #220: Workspace Metadata Must Define Unique Build Targets Per Product _CRITICAL_
- `canon-rule-222-cargo-metadata-resolvable.md` — #222: Workspace Members Must Be Fully Resolvable by Cargo Metadata _CRITICAL_
- `canon-rule-224-hyphen-naming-convention.md` — #224: Workspace Package Names Must Use Hyphens Consistently _CRITICAL_
- `canon-rule-225-workspace-exact-version-policy-v2.md` — #225 (v2.0): Workspace Dependency Version Policy Must Prevent ABI Drift Without Blocking Patch Compatibility _CRITICAL_
- `canon-rule-227-workspace-dependency-inheritance.md` — #227: Products Must Inherit Workspace Dependencies _HIGH_
- `canon-rule-228-wasm-target-configuration.md` — #228: Workspace Must Define WASM Target Configuration _CRITICAL_
- `canon-rule-229-design-system-css-artifact-isolation.md` — #229: Design System CSS Must Be Consumed as an Independent Artifact _CRITICAL_
- `canon-rule-230-wasm-artifact-budget-is-hard-contract.md` — #230: WASM Artifact Budget Is a Hard Contract _CRITICAL_
- `canon-rule-231-ssr-link-isolation-refined.md` — #231 (Refined): SSR Crates Must Be Link-Time Isolated From WASM Graph _CRITICAL_
- `canon-rule-232-builds-deterministic-from-workspace-root.md` — #232: Product Builds Must Be Deterministic From Workspace Root _HIGH_
- `canon-rule-233-css-cascade-is-build-enforced-order.md` — #233: CSS Cascade Order Is Build-Enforced Architecture _CRITICAL_
- `canon-rule-240-generated-artifacts-must-not-be-committed.md` — #240: Generated Artifacts Must Not Be Committed _CRITICAL_
- `canon-rule-246-css-bundle-must-be-layer-free.md` — #246: CSS Bundle Must Be Layer-Free _CRITICAL_
- `canon-rule-248-tokens-engine-is-the-only-source-of-truth.md` — #248: Tokens Engine Is the Only Source of Truth _CRITICAL_
- `canon-rule-251-artifacts-must-be-statically-identifiable.md` — #251: All Build Artifacts Must Be Statically Identifiable _HIGH_
- `canon-rule-254-canonrs-must-build-without-products.md` — #254: CanonRS Must Build Independently of Products _CRITICAL_
- `canon-rule-257-tokens-engine-is-mandatory-pre-build-step.md` — #257: Tokens Engine Is a Mandatory Pre-Build Step _CRITICAL_
- `canon-rule-258-mode-drives-build-profiles.md` — #258: Mode Drives Build Profiles _CRITICAL_
- `canon-rule-259-products-must-not-define-build-topology.md` — #259: Products Must Not Define Build Topology _CRITICAL_
- `canon-rule-261-css-size-drift-must-be-monitored.md` — #261: CSS Bundle Size Drift Must Be Monitored _HIGH_
- `canon-rule-262-no-runtime-css-regeneration.md` — #262: No Runtime CSS Regeneration _CRITICAL_
- `canon-rule-264-behavior-registry-is-single-entry-point.md` — #264: Behavior Registry Is the Single Runtime Entry Point _CRITICAL_
- `canon-rule-266-primitives-must-not-generate-ids.md` — #266: Primitives Must Never Generate IDs _HIGH_
- `canon-rule-280-interactive-must-be-feature-flag-safe.md` — #280: Interactive Must Be Safe Under Feature Flag Removal _CRITICAL_

---

## cli

**7 rules**

- `canon-rule-236-cli-defines-workspace-topology.md` — #236: CanonRS CLI Defines Workspace Topology _CRITICAL_
- `canon-rule-237-cli-owns-leptos-metadata.md` — #237: CLI Owns Leptos Metadata Blocks _CRITICAL_
- `canon-rule-239-cli-sync-must-be-idempotent.md` — #239: CLI Sync Operations Must Be Idempotent _HIGH_
- `canon-rule-240-generated-artifacts-must-not-be-committed.md` — #240: Generated Artifacts Must Not Be Committed _CRITICAL_
- `canon-rule-255-cli-authority-over-workspace-generation.md` — #255: CLI Is the Sole Authority Over Workspace Generation _CRITICAL_
- `canon-rule-258-mode-drives-build-profiles.md` — #258: Mode Drives Build Profiles _CRITICAL_
- `canon-rule-260-cli-autodiscovery-must-be-explicit-or-fail.md` — #260: CLI Autodiscovery Must Be Explicit or Fail _HIGH_

---

## components

**40 rules**

- `canon-rule-01-types.md` — #01: Component Types _HIGH_
- `canon-rule-06-visual-state.md` — #06: Visual State Declaration _HIGH_
- `canon-rule-10-modal-state.md` — #10: Modal Reactive State Management _HIGH_
- `canon-rule-12-select-vs-combobox.md` — #12 — Select vs Combobox _MEDIUM_
- `canon-rule-13-specialization-vs-substitution.md` — #13 — Specialization vs Substitution _HIGH_
- `canon-rule-14-datatable-vs-virtualtable.md` — #14 — DataTable vs VirtualTable _MEDIUM_
- `canon-rule-17-human-vs-machine-scale.md` — #17 — Human-scale vs Machine-scale Components _HIGH_
- `canon-rule-36-component-compliance-levels.md` — #36: Component Compliance Levels _MEDIUM_
- `canon-rule-42-ui-data-surfaces.md` — #42: UI Data Surfaces _HIGH_
- `canon-rule-43-domain-components-and-commands.md` — #43: Domain Components and Commands _HIGH_
- `canon-rule-45-demo-components.md` — #45: Demo Components & Ephemeral State _LOW_
- `canon-rule-47-tree-context-selection.md` — #47: Tree, Context & Selection _HIGH_
- `canon-rule-73-componentpage-template-contract.md` — #73: ComponentPage Template Contract _LOW_
- `canon-rule-74-block-semantic-html.md` — #74: Block Semantic HTML _HIGH_
- `canon-rule-76-navigation-vs-action-contract.md` — #76: Navigation vs Action Component Contract _HIGH_
- `canon-rule-79-componentpage-template-contract.md` — #79: ComponentPage Template Contract _LOW_
- `canon-rule-91-markdown-render-only.md` — #91: Markdown and Code Blocks Are Render-Only _HIGH_
- `canon-rule-108-visual-surfaces-contract.md` — #108: Visual Surfaces Contract _CRITICAL_
- `canon-rule-111-model-first-css-second.md` — #111: Model First, CSS Second _CRITICAL_
- `canon-rule-113-states-are-data-not-style.md` — #113: States Are Data, Not Style _HIGH_
- `canon-rule-114-single-visual-authority.md` — #114: Single Visual Authority _HIGH_
- `canon-rule-115-reset-awareness-and-boundaries.md` — #115: Reset Awareness & CSS Boundaries _MEDIUM_
- `canon-rule-116-wasm-externref-table-limits.md` — #116: WASM Externref Table Limits _CRITICAL_
- `canon-rule-117-design-system-callbacks-are-props-not-handlers.md` — #117: Design System Callbacks Are Props, Not Handlers _CRITICAL_
- `canon-rule-118-view-type-boundary.md` — #118: View<\_> Type Boundary Prohibition _CRITICAL_
- `canon-rule-119-no-optional-into-ui-layer.md` — #119: No #[prop(optional, into)] in UI Layer _CRITICAL_
- `canon-rule-122-no-conditional-rendering-with-closures.md` — #122: No Conditional Rendering with .then() Closures _HIGH_
- `canon-rule-126-component-domain-contracts.md` — #126: Component Domain Contracts _CRITICAL_
- `canon-rule-127-block-composition-contracts.md` — #127: Block Composition Contracts _HIGH_
- `canon-rule-130-controlled-ui-contract.md` — #130: Controlled UI Contract _HIGH_
- `canon-rule-138-children-must-be-consumed-immediately.md` — #138: Children Must Be Consumed Immediately _CRITICAL_
- `canon-rule-139-slots-are-ui-level-only.md` — #139: Slots Are UI-Level Only _HIGH_
- `canon-rule-142-children-must-always-be-optional.md` — #142 — Children Must Always Be Optional in Public Components _HIGH_
- `canon-rule-151-visual-feedback-must-never-encode-application-state.md` — #151 — Visual Feedback Must Never Encode Application State _HIGH_
- `canon-rule-170-html-css-contract-must-match.md` — #170: HTML and CSS Must Share the Same Contract _CRITICAL_
- `canon-rule-175-dark-light-css-concern-not-component.md` — #175: Dark/Light Is a CSS Concern, Not a Component Concern _HIGH_
- `canon-rule-195-interactive-components-require-explicit-id.md` — #195: Interactive Components Require Explicit ID _HIGH_
- `canon-rule-203-semantic-tokens-bridge-theme-families.md` — #203: Semantic Tokens Are the Only Bridge Between Theme and Families _CRITICAL_
- `canon-rule-205-ui-tokens-bind-semantic-never-presets.md` — #205: UI Tokens Must Bind to Semantic Tokens, Never to Presets _CRITICAL_
- `canon-rule-207-preset-switching-never-changes-component-css.md` — #207: Preset Switching Must Never Change Component CSS _CRITICAL_

---

## csr

**8 rules**

- `canon-rule-08-overlay-islands.md` — #08: Overlay Islands (Client-Only Architecture) _HIGH_
- `canon-rule-53-client-only-runtime-islands.md` — #53: Client-Only Runtime Islands _HIGH_
- `canon-rule-67-leptos-csr-css-loading.md` — #67: Leptos CSR Does NOT Load CSS via `<Stylesheet />` _CRITICAL_
- `canon-rule-99-csr-islands-must-not-own-routing-or-providers.md` — #99: CSR Islands Must Not Own Routing or Providers _HIGH_
- `canon-rule-136-controllers-are-csr-only.md` — #136: Controllers Are CSR-Only _CRITICAL_
- `canon-rule-141-csr-composition-belongs-to-app-layer.md` — #141: CSR Composition Belongs to the App Layer _CRITICAL_
- `canon-rule-214-lib-owns-app-structure.md` — #214: lib.rs Owns Application Structure and UI Semantics _CRITICAL_
- `canon-rule-217-entrypoints-must-be-explicit.md` — #217: Entry Points Must Be Explicit, Public, and Isolated _CRITICAL_

---

## css

**55 rules**

- `canon-rule-06-visual-state.md` — #06: Visual State Declaration _HIGH_
- `canon-rule-22-tailwind-v4-rust-integration.md` — #22: Tailwind v4 + Rust Integration _CRITICAL_
- `canon-rule-23-state-tokens.md` — #23: State Tokens (Hover, Focus, Disabled, Pressed) _HIGH_
- `canon-rule-28-responsive-grid-contract.md` — #28: Responsive Grid Contract _MEDIUM_
- `canon-rule-55-canonical-css-entry-points.md` — #55: Canonical CSS Entry Points _HIGH_
- `canon-rule-56-monorepo-css-build-pipeline.md` — #56: Monorepo CSS Build Pipeline _HIGH_
- `canon-rule-57-postcss-canon-config.md` — #57: PostCSS Canon Configuration _MEDIUM_
- `canon-rule-59-css-cascade-ownership.md` — #59: CSS Cascade Ownership _HIGH_
- `canon-rule-60-css-artifacts-are-immutable.md` — #60: CSS Artifacts Are Immutable _HIGH_
- `canon-rule-61-no-relative-css-imports.md` — #61: No Relative CSS Imports _MEDIUM_
- `canon-rule-64-css-build-pipeline-mandatory.md` — #64: CSS Build Pipeline is Mandatory _CRITICAL_
- `canon-rule-67-leptos-csr-css-loading.md` — #67: Leptos CSR Does NOT Load CSS via `<Stylesheet />` _CRITICAL_
- `canon-rule-70-css-pipeline-health-checks.md` — #70: CSS Pipeline Requires Health Checks _MEDIUM_
- `canon-rule-75-primitive-css-prohibition.md` — #75: Primitive CSS Prohibition _CRITICAL_
- `canon-rule-82-css-build-pipeline-health.md` — #82: CSS Build Pipeline Health _HIGH_
- `canon-rule-84-svg-dark-mode-contract.md` — #84: SVG Dark Mode Contract _HIGH_
- `canon-rule-106-ui-neutralizes-hostile-css-not-primitives.md` — #106 — UI Neutralizes Hostile CSS, Not Primitives _HIGH_
- `canon-rule-107-token-architecture-theme-specificity.md` — #107: Token Architecture & Theme Specificity _CRITICAL_
- `canon-rule-108-visual-surfaces-contract.md` — #108: Visual Surfaces Contract _CRITICAL_
- `canon-rule-111-model-first-css-second.md` — #111: Model First, CSS Second _CRITICAL_
- `canon-rule-112-ui-owns-visual-style.md` — #112: UI Owns Visual Style _CRITICAL_
- `canon-rule-113-states-are-data-not-style.md` — #113: States Are Data, Not Style _HIGH_
- `canon-rule-114-single-visual-authority.md` — #114: Single Visual Authority _HIGH_
- `canon-rule-115-reset-awareness-and-boundaries.md` — #115: Reset Awareness & CSS Boundaries _MEDIUM_
- `canon-rule-134-layouts-are-css-and-semantics-only.md` — #134: Layouts Are CSS and Semantics Only _HIGH_
- `canon-rule-154-css-layout-deterministic.md` — #154: Deterministic Layout via Canonical CSS (SSR-Safe) _CRITICAL_
- `canon-rule-155-css-token-contract-is-architecture.md` — #155: CSS Token Contract Is Architecture _CRITICAL_
- `canon-rule-156-css-variable-scope-is-non-negotiable.md` — #156: CSS Variable Scope Is Non-Negotiable _CRITICAL_
- `canon-rule-157-design-system-css-build-time-flattened.md` — #157: Design System CSS Must Be Build-Time Flattened _CRITICAL_
- `canon-rule-159-css-ui-fail-safe-template.md` — #159: UI CSS Must Be Fail-Safe, Token-Only, and Attribute-Scoped _CRITICAL_
- `canon-rule-160-css-bootstrap-no-build.md` — #160: First App Must Not Require CSS Build _CRITICAL_
- `canon-rule-161-css-load-order.md` — #161: Canonical CSS Load Order Is Mandatory _CRITICAL_
- `canon-rule-166-dist-is-readonly.md` — #166: Dist Is Read-Only _CRITICAL_
- `canon-rule-167-canonrs-css-single-entrypoint.md` — #167: CanonRS CSS Has a Single Entrypoint _HIGH_
- `canon-rule-169-token-order-is-architectural.md` — #169: Token Import Order Is Architectural _HIGH_
- `canon-rule-171-no-phantom-variables.md` — #171: Phantom Variables Are Forbidden _CRITICAL_
- `canon-rule-173-css-is-a-first-class-system.md` — #173: CSS Is a First-Class System _HIGH_
- `canon-rule-175-dark-light-css-concern-not-component.md` — #175: Dark/Light Is a CSS Concern, Not a Component Concern _HIGH_
- `canon-rule-184-floating-overlays-expose-position-only-via-css-vars.md` — #184: Floating Overlays Expose Position Only via CSS Variables _CRITICAL_
- `canon-rule-186-overlay-visibility-is-data-state.md` — #190: Overlay Visibility Is Controlled Only via data-state _CRITICAL_
- `canon-rule-204-theme-files-must-be-last-in-cascade.md` — #204: Theme Files Must Be Last in the Cascade _HIGH_
- `canon-rule-229-design-system-css-artifact-isolation.md` — #229: Design System CSS Must Be Consumed as an Independent Artifact _CRITICAL_
- `canon-rule-233-css-cascade-is-build-enforced-order.md` — #233: CSS Cascade Order Is Build-Enforced Architecture _CRITICAL_
- `canon-rule-234-tokens-engine-single-source-of-truth.md` — #234: Tokens Engine Is the Single Source of CSS Truth _CRITICAL_
- `canon-rule-242-cascade-order-is-a-contract.md` — #242: Cascade Order Is a Contract, Not a Preference _CRITICAL_
- `canon-rule-243-data-theme-is-the-only-activation-boundary.md` — #243: data-theme Is the Only Theme Activation Boundary _CRITICAL_
- `canon-rule-244-semantic-layer-is-mandatory-abstraction.md` — #244: Semantic Layer Is a Mandatory Abstraction _CRITICAL_
- `canon-rule-246-css-bundle-must-be-layer-free.md` — #246: CSS Bundle Must Be Layer-Free _CRITICAL_
- `canon-rule-247-entry-order-is-architectural.md` — #247: CSS Entry Order Is Architectural, Not Cosmetic _CRITICAL_
- `canon-rule-249-products-must-not-depend-on-generated-css.md` — #249: Products Must Not Depend on .generated CSS _CRITICAL_
- `canon-rule-252-theme-switching-must-be-attribute-driven.md` — #252: Theme Switching Must Be Attribute-Driven _HIGH_
- `canon-rule-253-token-cascade-order-is-immutable.md` — #253: Token Cascade Order Is Immutable _CRITICAL_
- `canon-rule-257-tokens-engine-is-mandatory-pre-build-step.md` — #257: Tokens Engine Is a Mandatory Pre-Build Step _CRITICAL_
- `canon-rule-261-css-size-drift-must-be-monitored.md` — #261: CSS Bundle Size Drift Must Be Monitored _HIGH_
- `canon-rule-278-primitive-must-not-encode-design-tokens.md` — #278: Primitive Must Not Encode Design Tokens in Rust _CRITICAL_

---

## design-system

**30 rules**

- `canon-rule-07-token-governance.md` — #07: Theme and Token Governance _CRITICAL_
- `canon-rule-21-canonical-color-tokens.md` — #21: Canonical Color Tokens vs Semantic Intents _HIGH_
- `canon-rule-24-density-size-scaling.md` — #24: Density & Size Scaling _MEDIUM_
- `canon-rule-25-theme-presets-contract.md` — #25: Theme Presets Contract _HIGH_
- `canon-rule-26-elevation-shadow-system.md` — #26: Elevation & Shadow System _MEDIUM_
- `canon-rule-29-typography-contract.md` — #29: Typography Contract _HIGH_
- `canon-rule-30-iconography-system.md` — #30: Iconography System _MEDIUM_
- `canon-rule-35-token-usage-validation.md` — #35: Token Usage Validation _HIGH_
- `canon-rule-62-single-source-of-truth-tokens.md` — #62: Single Source of Truth for Design Tokens _CRITICAL_
- `canon-rule-78-token-definition-completeness.md` — #78: Token Definition Completeness _HIGH_
- `canon-rule-81-flex-layout-ownership.md` — #81: Flex Layout Ownership _HIGH_
- `canon-rule-84-svg-dark-mode-contract.md` — #84: SVG Dark Mode Contract _HIGH_
- `canon-rule-101-workbench-assets-must-be-product-scoped.md` — #101: Workbench Assets Must Be Product-Scoped _MEDIUM_
- `canon-rule-117-design-system-callbacks-are-props-not-handlers.md` — #117: Design System Callbacks Are Props, Not Handlers _CRITICAL_
- `canon-rule-119-no-optional-into-ui-layer.md` — #119: No #[prop(optional, into)] in UI Layer _CRITICAL_
- `canon-rule-123-component-architecture-taxonomy.md` — #123: Component Architecture Taxonomy and Contracts _CRITICAL_
- `canon-rule-155-css-token-contract-is-architecture.md` — #155: CSS Token Contract Is Architecture _CRITICAL_
- `canon-rule-156-css-variable-scope-is-non-negotiable.md` — #156: CSS Variable Scope Is Non-Negotiable _CRITICAL_
- `canon-rule-157-design-system-css-build-time-flattened.md` — #157: Design System CSS Must Be Build-Time Flattened _CRITICAL_
- `canon-rule-158-design-system-packages-immutable-contracts.md` — #158: Design System Packages Are Immutable Contracts (No Direct File Imports) _CRITICAL_
- `canon-rule-159-css-ui-fail-safe-template.md` — #159: UI CSS Must Be Fail-Safe, Token-Only, and Attribute-Scoped _CRITICAL_
- `canon-rule-176-governance-is-the-single-source-of-truth.md` — #176: Governance Is the Single Source of Truth _CRITICAL_
- `canon-rule-179-no-visual-surfaces-in-core-or-families.md` — #179: No Visual Surface Tokens in Core or Families _CRITICAL_
- `canon-rule-180-semantic-token-names-canonical-suffixes.md` — #180: Semantic Token Names Must Follow Canonical Suffixes _CRITICAL_
- `canon-rule-182-semantic-role-must-affect-visual-contrast.md` — #182: Semantic Role Must Affect Visual Contrast _HIGH_
- `canon-rule-201-presets-only-color-source.md` — #201: Presets Are the Only Source of Color Values _CRITICAL_
- `canon-rule-203-semantic-tokens-bridge-theme-families.md` — #203: Semantic Tokens Are the Only Bridge Between Theme and Families _CRITICAL_
- `canon-rule-206-ui-inventory-is-fixed.md` — #206: UI Inventory Is Fixed and Canonical _CRITICAL_
- `canon-rule-229-design-system-css-artifact-isolation.md` — #229: Design System CSS Must Be Consumed as an Independent Artifact _CRITICAL_
- `canon-rule-254-canonrs-must-build-without-products.md` — #254: CanonRS Must Build Independently of Products _CRITICAL_

---

## governance

**11 rules**

- `canon-rule-34-theme-density-enforcement.md` — #34: Theme & Density Enforcement (Lint Rules) _HIGH_
- `canon-rule-35-token-usage-validation.md` — #35: Token Usage Validation _HIGH_
- `canon-rule-36-component-compliance-levels.md` — #36: Component Compliance Levels _MEDIUM_
- `canon-rule-40-legacy-exception-annotation.md` — #40: Legacy Exception & Annotation Contract _LOW_
- `canon-rule-176-governance-is-the-single-source-of-truth.md` — #176: Governance Is the Single Source of Truth _CRITICAL_
- `canon-rule-178-contracts-are-read-only-apis.md` — #178: Contracts Are Read-Only APIs (DEPRECATED) _NONE_
- `canon-rule-187-floating-primitives-expose-css-vars-only.md` — #187: Floating Primitives Enforce CSS-Variable-Only Positioning _CRITICAL_
- `canon-rule-206-ui-inventory-is-fixed.md` — #206: UI Inventory Is Fixed and Canonical _CRITICAL_
- `canon-rule-208-working-theme-toggle-proves-correct-token-architecture.md` — #208: A Working Theme Toggle Is Proof of Correct Token Architecture _HIGH_
- `canon-rule-248-tokens-engine-is-the-only-source-of-truth.md` — #248: Tokens Engine Is the Only Source of Truth _CRITICAL_
- `canon-rule-256-generated-workspace-is-ephemeral.md` — #256: Generated Workspace Is Ephemeral and Immutable _CRITICAL_

---

## hydration

**19 rules**

- `canon-rule-04-hydration.md` — #04: Anti-Hydration Rules _CRITICAL_
- `canon-rule-08-overlay-islands.md` — #08: Overlay Islands (Client-Only Architecture) _HIGH_
- `canon-rule-53-client-only-runtime-islands.md` — #53: Client-Only Runtime Islands _HIGH_
- `canon-rule-87-leptos-ssr-script-placement.md` — #87: Leptos SSR Script Placement _CRITICAL_
- `canon-rule-90-hydration-is-dom-replacement.md` — #90: Hydration Is DOM Replacement, Not Enhancement _CRITICAL_
- `canon-rule-92-ssr-debug-heuristic.md` — #92: If It Works in Prod but Not Dev, Suspect Hydration Order _MEDIUM_
- `canon-rule-103-critical-runtime-js-must-be-inline-in-ssr.md` — #103: Critical Runtime JS Must Be Inline in SSR _CRITICAL_
- `canon-rule-129-ssr-event-safety.md` — #129: SSR Event Safety _CRITICAL_
- `canon-rule-143-ui-must-be-hydration-deterministic.md` — #143 — UI Must Be Hydration-Deterministic _CRITICAL_
- `canon-rule-146-ui-content-must-be-ssr-stable.md` — #146 — UI Content Must Be SSR-Stable _CRITICAL_
- `canon-rule-150-ui-must-be-deterministic-under-ssr-and-hydration.md` — #150 — UI Must Be Deterministic Under SSR and Hydration _CRITICAL_
- `canon-rule-154-css-layout-deterministic.md` — #154: Deterministic Layout via Canonical CSS (SSR-Safe) _CRITICAL_
- `canon-rule-163-hydration-effects-only.md` — #163: DOM Effects Are Hydration-Only _CRITICAL_
- `canon-rule-186-overlay-visibility-is-data-state.md` — #190: Overlay Visibility Is Controlled Only via data-state _CRITICAL_
- `canon-rule-195-interactive-components-require-explicit-id.md` — #195: Interactive Components Require Explicit ID _HIGH_
- `canon-rule-210-cdylib-mandatory-for-hydration.md` — #210: cdylib Is Mandatory for Hydration _CRITICAL_
- `canon-rule-212-hydration-bootstrap-tooling-owned.md` — #212: Hydration Bootstrap Is Tooling-Owned _CRITICAL_
- `canon-rule-152-provider-callback-hydration-ownership.md` — 152 — Provider Callback Hydration Ownership _CRITICAL_
- `canon-rule-153-layouts-must-be-event-free-during-hydration.md` — 153 — Layouts Must Be Event-Free During Hydration _CRITICAL_

---

## interactive

**16 rules**

- `canon-rule-09-clipboard-apis.md` — #09: Clipboard and Browser APIs _MEDIUM_
- `canon-rule-11-multi-callback-ownership.md` — #11: Multi-Callback Ownership _HIGH_
- `canon-rule-46-command-palette-intent-surfaces.md` — #46: Command Palette & Intent Surfaces _MEDIUM_
- `canon-rule-49-drag-drop-as-intent.md` — #49: Drag & Drop as Intent (Not Action) _MEDIUM_
- `canon-rule-51-callbacks-as-commands.md` — #51: Callbacks as Commands _MEDIUM_
- `canon-rule-52-command-history-runtime.md` — #52: Command History as First-Class Runtime _MEDIUM_
- `canon-rule-102-runtime-js-is-shell-infrastructure.md` — #102: Runtime JS Is Shell Infrastructure _CRITICAL_
- `canon-rule-231-ssr-link-isolation-refined.md` — #231 (Refined): SSR Crates Must Be Link-Time Isolated From WASM Graph _CRITICAL_
- `canon-rule-265-interactive-owns-state-behavior-does-not.md` — #265: Interactive Owns All State — Behavior Owns None _CRITICAL_
- `canon-rule-267-data-attributes-are-contract-not-style.md` — #267: Data Attributes Are Contract, Not Styling Mechanism _HIGH_
- `canon-rule-269-custom-events-are-runtime-contracts.md` — #269: Custom Events Are Public Runtime Contracts _HIGH_
- `canon-rule-274-interactive-feature-isolation.md` — #274: Interactive Features Must Be Strictly Isolated _CRITICAL_
- `canon-rule-275-interactive-container-bound-hooks.md` — #275: Interactive Hooks Must Be Container-Bound _CRITICAL_
- `canon-rule-276-interactive-ssr-determinism.md` — #276: Interactive Must Remain Deterministic Under SSR Without wasm _CRITICAL_
- `canon-rule-277-interactive-must-not-emit-dom-events.md` — #277: Interactive Must Not Emit DOM Events _HIGH_
- `canon-rule-280-interactive-must-be-feature-flag-safe.md` — #280: Interactive Must Be Safe Under Feature Flag Removal _CRITICAL_

---

## layout

**15 rules**

- `canon-rule-15-pagination-vs-virtualization.md` — #15 — Pagination vs Virtualization _MEDIUM_
- `canon-rule-28-responsive-grid-contract.md` — #28: Responsive Grid Contract _MEDIUM_
- `canon-rule-72-layout-h1-prohibition.md` — #72: Layout H1 Prohibition _MEDIUM_
- `canon-rule-81-flex-layout-ownership.md` — #81: Flex Layout Ownership _HIGH_
- `canon-rule-83-layout-zones-contract.md` — #83: Layout Zones Contract _CRITICAL_
- `canon-rule-128-layout-shell-and-zone-contracts.md` — #128: Layout Shell and Zone Contracts _HIGH_
- `canon-rule-132-layout-composition-over-abstraction.md` — #132: Layout Composition Over Abstraction _HIGH_
- `canon-rule-134-layouts-are-css-and-semantics-only.md` — #134: Layouts Are CSS and Semantics Only _HIGH_
- `canon-rule-135-ui-vs-layout-responsibility-boundary.md` — #135: UI vs Layout Responsibility Boundary _CRITICAL_
- `canon-rule-138-children-must-be-consumed-immediately.md` — #138: Children Must Be Consumed Immediately _CRITICAL_
- `canon-rule-139-slots-are-ui-level-only.md` — #139: Slots Are UI-Level Only _HIGH_
- `canon-rule-140-layouts-are-structural-not-behavioral.md` — #140: Layouts Are Structural, Not Behavioral _CRITICAL_
- `canon-rule-149-controllers-must-be-temporal-not-structural.md` — #149 — Controllers Must Be Temporal, Not Structural _HIGH_
- `canon-rule-154-css-layout-deterministic.md` — #154: Deterministic Layout via Canonical CSS (SSR-Safe) _CRITICAL_
- `canon-rule-153-layouts-must-be-event-free-during-hydration.md` — 153 — Layouts Must Be Event-Free During Hydration _CRITICAL_

---

## leptos

**33 rules**

- `canon-rule-02-ownership.md` — #02: Ownership Rules _HIGH_
- `canon-rule-03-lists.md` — #03: Lists and Iteration _MEDIUM_
- `canon-rule-41-leptos-resource-consumption.md` — #41: Leptos Resource Consumption Contract _MEDIUM_
- `canon-rule-58-leptos-assets-dev-constraint.md` — #58: Leptos Assets Dev Constraint _MEDIUM_
- `canon-rule-63-leptos-reactivity-closures.md` — #63: Leptos Reactivity - No .get() Outside Closures _HIGH_
- `canon-rule-67-leptos-csr-css-loading.md` — #67: Leptos CSR Does NOT Load CSS via `<Stylesheet />` _CRITICAL_
- `canon-rule-85-leptos-asset-pipeline-dev.md` — #85: Leptos Asset Pipeline in Dev Mode _CRITICAL_
- `canon-rule-86-children-childrenfn-contract.md` — #86: Children vs ChildrenFn Contract _CRITICAL_
- `canon-rule-87-leptos-ssr-script-placement.md` — #87: Leptos SSR Script Placement _CRITICAL_
- `canon-rule-90-hydration-is-dom-replacement.md` — #90: Hydration Is DOM Replacement, Not Enhancement _CRITICAL_
- `canon-rule-93-leptos-wasm-dev-builds-must-use-release-mode.md` — #93: Leptos WASM Dev Builds Must Use Release Mode _CRITICAL_
- `canon-rule-94-leptos-workspace-features-must-be-explicit.md` — #94: Leptos Workspace Features Must Be Explicit _CRITICAL_
- `canon-rule-95-ssr-requires-complete-html-shell.md` — #95: SSR Requires Complete HTML Shell _CRITICAL_
- `canon-rule-96-ssr-requires-explicit-provider-tree.md` — #96: SSR Requires Explicit Provider Tree _HIGH_
- `canon-rule-97-leptos-08-requires-floating-nightly-toolchain.md` — #97: Leptos 0.8 Requires Floating Nightly Toolchain _MEDIUM_
- `canon-rule-98-axum-leptos-ssr-closures-must-own-state.md` — #98: Axum + Leptos SSR Closures Must Own State _MEDIUM_
- `canon-rule-100-build-orchestrators-must-be-workspace-scoped.md` — #100: Build Orchestrators Must Be Workspace-Scoped _MEDIUM_
- `canon-rule-104-autoreload-breaks-script-order-guarantees.md` — #104: AutoReload Breaks Script Order Guarantees _HIGH_
- `canon-rule-121-storedvalue-for-noncopy-in-view.md` — #121: StoredValue for Non-Copy Values in view! Closures _CRITICAL_
- `canon-rule-122-no-conditional-rendering-with-closures.md` — #122: No Conditional Rendering with .then() Closures _HIGH_
- `canon-rule-131-reactive-boundary-ownership.md` — #131: Reactive Boundary Ownership _HIGH_
- `canon-rule-132-layout-composition-over-abstraction.md` — #132: Layout Composition Over Abstraction _HIGH_
- `canon-rule-133-children-consumption-locality.md` — #133: Children Consumption Locality _CRITICAL_
- `canon-rule-138-children-must-be-consumed-immediately.md` — #138: Children Must Be Consumed Immediately _CRITICAL_
- `canon-rule-140-layouts-are-structural-not-behavioral.md` — #140: Layouts Are Structural, Not Behavioral _CRITICAL_
- `canon-rule-147-reactive-closures-are-data-not-structure.md` — #147 — Reactive Closures Are Data, Not Structure _HIGH_
- `canon-rule-219-leptos-consistent-ports.md` — #219: Leptos Product Must Declare Consistent Ports Across Workspace and Local Config _CRITICAL_
- `canon-rule-220-unique-build-targets.md` — #220: Workspace Metadata Must Define Unique Build Targets Per Product _CRITICAL_
- `canon-rule-221-critical-config-alignment.md` — #221: Leptos.toml Critical Fields Must Align With Workspace Metadata _HIGH_
- `canon-rule-223-feature-flag-isolation.md` — #223: Feature Flag Scopes Must Not Leak Between Products _CRITICAL_
- `canon-rule-226-root-relative-paths.md` — #226: Workspace Leptos Paths Must Be Root-Relative _HIGH_
- `canon-rule-237-cli-owns-leptos-metadata.md` — #237: CLI Owns Leptos Metadata Blocks _CRITICAL_
- `canon-rule-152-provider-callback-hydration-ownership.md` — 152 — Provider Callback Hydration Ownership _CRITICAL_

---

## motion

**2 rules**

- `canon-rule-26-elevation-shadow-system.md` — #26: Elevation & Shadow System _MEDIUM_
- `canon-rule-27-motion-timing-tokens.md` — #27: Motion & Timing Tokens _MEDIUM_

---

## overlays

**4 rules**

- `canon-rule-183-overlay-positioning-is-a-primitive.md` — #183: Overlay Positioning Is a Primitive, Not a Controller Concern _CRITICAL_
- `canon-rule-184-floating-overlays-expose-position-only-via-css-vars.md` — #184: Floating Overlays Expose Position Only via CSS Variables _CRITICAL_
- `canon-rule-187-floating-primitives-expose-css-vars-only.md` — #187: Floating Primitives Enforce CSS-Variable-Only Positioning _CRITICAL_
- `canon-rule-185-overlays-must-be-anchor-relative.md` — #189: Overlays Must Be Anchor-Relative _CRITICAL_

---

## pages

**2 rules**

- `canon-rule-191-pages-use-page-behaviors-only.md` — #191: Pages Must Delegate Wiring to Page Behaviors Only _CRITICAL_
- `canon-rule-192-page-behaviors-do-not-implement-logic.md` — #192: Page Behaviors Must Not Implement New Logic _CRITICAL_

---

## primitives

**15 rules**

- `canon-rule-02-ownership.md` — #02: Ownership Rules _HIGH_
- `canon-rule-75-primitive-css-prohibition.md` — #75: Primitive CSS Prohibition _CRITICAL_
- `canon-rule-89-primitives-no-browser-apis.md` — #89: Primitives Must Never Touch Browser APIs _CRITICAL_
- `canon-rule-106-ui-neutralizes-hostile-css-not-primitives.md` — #106 — UI Neutralizes Hostile CSS, Not Primitives _HIGH_
- `canon-rule-112-ui-owns-visual-style.md` — #112: UI Owns Visual Style _CRITICAL_
- `canon-rule-120-dom-events-vs-semantic-callbacks.md` — #120: DOM Events vs Semantic Callbacks Boundary _HIGH_
- `canon-rule-124-primitive-contract-types.md` — #124: Primitive Contract Types _CRITICAL_
- `canon-rule-183-overlay-positioning-is-a-primitive.md` — #183: Overlay Positioning Is a Primitive, Not a Controller Concern _CRITICAL_
- `canon-rule-184-floating-overlays-expose-position-only-via-css-vars.md` — #184: Floating Overlays Expose Position Only via CSS Variables _CRITICAL_
- `canon-rule-187-floating-primitives-expose-css-vars-only.md` — #187: Floating Primitives Enforce CSS-Variable-Only Positioning _CRITICAL_
- `canon-rule-188-overlay-positioning-is-not-ui-logic.md` — #188: Overlay Positioning Is Not UI Logic _HIGH_
- `canon-rule-185-overlays-must-be-anchor-relative.md` — #189: Overlays Must Be Anchor-Relative _CRITICAL_
- `canon-rule-266-primitives-must-not-generate-ids.md` — #266: Primitives Must Never Generate IDs _HIGH_
- `canon-rule-267-data-attributes-are-contract-not-style.md` — #267: Data Attributes Are Contract, Not Styling Mechanism _HIGH_
- `canon-rule-278-primitive-must-not-encode-design-tokens.md` — #278: Primitive Must Not Encode Design Tokens in Rust _CRITICAL_

---

## providers

**9 rules**

- `canon-rule-37-provider-taxonomy-boundaries.md` — #37: Provider Taxonomy & Boundaries _HIGH_
- `canon-rule-48-context-provider.md` — #48: Context Provider Pattern _HIGH_
- `canon-rule-50-provider-singleton-pattern.md` — #50: Provider Singleton & Runtime Separation _HIGH_
- `canon-rule-65-theme-provider-dom-sync.md` — #65: data-theme Sync is ThemeProvider Responsibility _HIGH_
- `canon-rule-96-ssr-requires-explicit-provider-tree.md` — #96: SSR Requires Explicit Provider Tree _HIGH_
- `canon-rule-137-providers-must-have-single-owner.md` — #137: Providers Must Have a Single Owner _HIGH_
- `canon-rule-144-providers-expose-state-apps-own-interaction.md` — #144 — Providers Expose State, Applications Own Interaction _HIGH_
- `canon-rule-162-providers-not-ui.md` — #162: Providers Are Infrastructure, Not UI _HIGH_
- `canon-rule-152-provider-callback-hydration-ownership.md` — 152 — Provider Callback Hydration Ownership _CRITICAL_

---

## ssr

**34 rules**

- `canon-rule-04-hydration.md` — #04: Anti-Hydration Rules _CRITICAL_
- `canon-rule-05-ssr-effects.md` — #05: SSR Effects and Browser API Safety _CRITICAL_
- `canon-rule-08-overlay-islands.md` — #08: Overlay Islands (Client-Only Architecture) _HIGH_
- `canon-rule-19-streaming-vs-snapshot.md` — #19 — Streaming vs Snapshot Data _MEDIUM_
- `canon-rule-20-realtime-vs-eventual.md` — #20 — Real-time UI vs Eventual Consistency _MEDIUM_
- `canon-rule-87-leptos-ssr-script-placement.md` — #87: Leptos SSR Script Placement _CRITICAL_
- `canon-rule-89-primitives-no-browser-apis.md` — #89: Primitives Must Never Touch Browser APIs _CRITICAL_
- `canon-rule-90-hydration-is-dom-replacement.md` — #90: Hydration Is DOM Replacement, Not Enhancement _CRITICAL_
- `canon-rule-92-ssr-debug-heuristic.md` — #92: If It Works in Prod but Not Dev, Suspect Hydration Order _MEDIUM_
- `canon-rule-95-ssr-requires-complete-html-shell.md` — #95: SSR Requires Complete HTML Shell _CRITICAL_
- `canon-rule-96-ssr-requires-explicit-provider-tree.md` — #96: SSR Requires Explicit Provider Tree _HIGH_
- `canon-rule-98-axum-leptos-ssr-closures-must-own-state.md` — #98: Axum + Leptos SSR Closures Must Own State _MEDIUM_
- `canon-rule-99-csr-islands-must-not-own-routing-or-providers.md` — #99: CSR Islands Must Not Own Routing or Providers _HIGH_
- `canon-rule-103-critical-runtime-js-must-be-inline-in-ssr.md` — #103: Critical Runtime JS Must Be Inline in SSR _CRITICAL_
- `canon-rule-104-autoreload-breaks-script-order-guarantees.md` — #104: AutoReload Breaks Script Order Guarantees _HIGH_
- `canon-rule-121-storedvalue-for-noncopy-in-view.md` — #121: StoredValue for Non-Copy Values in view! Closures _CRITICAL_
- `canon-rule-129-ssr-event-safety.md` — #129: SSR Event Safety _CRITICAL_
- `canon-rule-136-controllers-are-csr-only.md` — #136: Controllers Are CSR-Only _CRITICAL_
- `canon-rule-143-ui-must-be-hydration-deterministic.md` — #143 — UI Must Be Hydration-Deterministic _CRITICAL_
- `canon-rule-146-ui-content-must-be-ssr-stable.md` — #146 — UI Content Must Be SSR-Stable _CRITICAL_
- `canon-rule-147-reactive-closures-are-data-not-structure.md` — #147 — Reactive Closures Are Data, Not Structure _HIGH_
- `canon-rule-150-ui-must-be-deterministic-under-ssr-and-hydration.md` — #150 — UI Must Be Deterministic Under SSR and Hydration _CRITICAL_
- `canon-rule-163-hydration-effects-only.md` — #163: DOM Effects Are Hydration-Only _CRITICAL_
- `canon-rule-164-pkg-served-by-ssr.md` — #164: WASM and JS Assets Must Be Served by SSR _CRITICAL_
- `canon-rule-199-server-adapters-are-integration-code.md` — #199: Server Adapters Are Integration Code _HIGH_
- `canon-rule-209-axum-version-must-match-adapter.md` — #209: Axum Version Must Match Adapter Version _CRITICAL_
- `canon-rule-211-ssr-meta-requires-html-shell.md` — #211: SSR Meta Requires Explicit HTML Shell _HIGH_
- `canon-rule-213-streaming-ssr-requires-executor.md` — #213: Streaming SSR Requires Explicit Executor Initialization _CRITICAL_
- `canon-rule-214-lib-owns-app-structure.md` — #214: lib.rs Owns Application Structure and UI Semantics _CRITICAL_
- `canon-rule-215-main-owns-runtime-bootstrap.md` — #215: main.rs Owns Runtime and Server Bootstrap Exclusively _CRITICAL_
- `canon-rule-216-app-must-not-define-html.md` — #216: App Components Must Not Define HTML Structure _HIGH_
- `canon-rule-217-entrypoints-must-be-explicit.md` — #217: Entry Points Must Be Explicit, Public, and Isolated _CRITICAL_
- `canon-rule-218-legacy-rendering-apis-forbidden.md` — #218: Legacy Rendering APIs Are Forbidden _HIGH_
- `canon-rule-153-layouts-must-be-event-free-during-hydration.md` — 153 — Layouts Must Be Event-Free During Hydration _CRITICAL_

---

## state

**32 rules**

- `canon-rule-02-ownership.md` — #02: Ownership Rules _HIGH_
- `canon-rule-03-lists.md` — #03: Lists and Iteration _MEDIUM_
- `canon-rule-10-modal-state.md` — #10: Modal Reactive State Management _HIGH_
- `canon-rule-14-datatable-vs-virtualtable.md` — #14 — DataTable vs VirtualTable _MEDIUM_
- `canon-rule-16-client-vs-server-filtering.md` — #16 — Client-side vs Server-side Filtering _MEDIUM_
- `canon-rule-18-client-vs-server-sorting.md` — #18 — Sorting: Client vs Server _MEDIUM_
- `canon-rule-19-streaming-vs-snapshot.md` — #19 — Streaming vs Snapshot Data _MEDIUM_
- `canon-rule-20-realtime-vs-eventual.md` — #20 — Real-time UI vs Eventual Consistency _MEDIUM_
- `canon-rule-23-state-tokens.md` — #23: State Tokens (Hover, Focus, Disabled, Pressed) _HIGH_
- `canon-rule-32-theme-persistence-contract.md` — #32: Theme Persistence Contract _MEDIUM_
- `canon-rule-39-settings-ui-compliance.md` — #39: Settings UI Compliance _MEDIUM_
- `canon-rule-41-leptos-resource-consumption.md` — #41: Leptos Resource Consumption Contract _MEDIUM_
- `canon-rule-42-ui-data-surfaces.md` — #42: UI Data Surfaces _HIGH_
- `canon-rule-44-orchestrators.md` — #44: Orchestrators _HIGH_
- `canon-rule-45-demo-components.md` — #45: Demo Components & Ephemeral State _LOW_
- `canon-rule-47-tree-context-selection.md` — #47: Tree, Context & Selection _HIGH_
- `canon-rule-52-command-history-runtime.md` — #52: Command History as First-Class Runtime _MEDIUM_
- `canon-rule-54-render-must-be-total.md` — #54 — Render Must Be Total _CRITICAL_
- `canon-rule-63-leptos-reactivity-closures.md` — #63: Leptos Reactivity - No .get() Outside Closures _HIGH_
- `canon-rule-86-children-childrenfn-contract.md` — #86: Children vs ChildrenFn Contract _CRITICAL_
- `canon-rule-113-states-are-data-not-style.md` — #113: States Are Data, Not Style _HIGH_
- `canon-rule-121-storedvalue-for-noncopy-in-view.md` — #121: StoredValue for Non-Copy Values in view! Closures _CRITICAL_
- `canon-rule-130-controlled-ui-contract.md` — #130: Controlled UI Contract _HIGH_
- `canon-rule-131-reactive-boundary-ownership.md` — #131: Reactive Boundary Ownership _HIGH_
- `canon-rule-133-children-consumption-locality.md` — #133: Children Consumption Locality _CRITICAL_
- `canon-rule-137-providers-must-have-single-owner.md` — #137: Providers Must Have a Single Owner _HIGH_
- `canon-rule-144-providers-expose-state-apps-own-interaction.md` — #144 — Providers Expose State, Applications Own Interaction _HIGH_
- `canon-rule-145-ui-must-not-mutate-global-state-implicitly.md` — #145 — UI Components Must Not Mutate Global State Implicitly _HIGH_
- `canon-rule-148-ui-must-never-infer-intent-from-state.md` — #148 — UI Must Never Infer Intent From State _HIGH_
- `canon-rule-151-visual-feedback-must-never-encode-application-state.md` — #151 — Visual Feedback Must Never Encode Application State _HIGH_
- `canon-rule-213-streaming-ssr-requires-executor.md` — #213: Streaming SSR Requires Explicit Executor Initialization _CRITICAL_
- `canon-rule-265-interactive-owns-state-behavior-does-not.md` — #265: Interactive Owns All State — Behavior Owns None _CRITICAL_

---

## theming

**16 rules**

- `canon-rule-07-token-governance.md` — #07: Theme and Token Governance _CRITICAL_
- `canon-rule-25-theme-presets-contract.md` — #25: Theme Presets Contract _HIGH_
- `canon-rule-32-theme-persistence-contract.md` — #32: Theme Persistence Contract _MEDIUM_
- `canon-rule-34-theme-density-enforcement.md` — #34: Theme & Density Enforcement (Lint Rules) _HIGH_
- `canon-rule-38-theme-engine-contract.md` — #38: Theme Engine Contract _CRITICAL_
- `canon-rule-39-settings-ui-compliance.md` — #39: Settings UI Compliance _MEDIUM_
- `canon-rule-65-theme-provider-dom-sync.md` — #65: data-theme Sync is ThemeProvider Responsibility _HIGH_
- `canon-rule-71-debug-theme-verify-file-first.md` — #71: Debug Theme by Verifying File First _MEDIUM_
- `canon-rule-156-css-variable-scope-is-non-negotiable.md` — #156: CSS Variable Scope Is Non-Negotiable _CRITICAL_
- `canon-rule-161-css-load-order.md` — #161: Canonical CSS Load Order Is Mandatory _CRITICAL_
- `canon-rule-179-no-visual-surfaces-in-core-or-families.md` — #179: No Visual Surface Tokens in Core or Families _CRITICAL_
- `canon-rule-201-presets-only-color-source.md` — #201: Presets Are the Only Source of Color Values _CRITICAL_
- `canon-rule-202-themes-resolve-context-not-palette.md` — #202: Themes Resolve Context, Not Palette _CRITICAL_
- `canon-rule-235-ui-must-consume-semantic-not-theme.md` — #235: UI Layer Must Consume Semantic Tokens, Never Theme Directly _CRITICAL_
- `canon-rule-243-data-theme-is-the-only-activation-boundary.md` — #243: data-theme Is the Only Theme Activation Boundary _CRITICAL_
- `canon-rule-252-theme-switching-must-be-attribute-driven.md` — #252: Theme Switching Must Be Attribute-Driven _HIGH_

---

## tokens

**45 rules**

- `canon-rule-07-token-governance.md` — #07: Theme and Token Governance _CRITICAL_
- `canon-rule-21-canonical-color-tokens.md` — #21: Canonical Color Tokens vs Semantic Intents _HIGH_
- `canon-rule-23-state-tokens.md` — #23: State Tokens (Hover, Focus, Disabled, Pressed) _HIGH_
- `canon-rule-24-density-size-scaling.md` — #24: Density & Size Scaling _MEDIUM_
- `canon-rule-25-theme-presets-contract.md` — #25: Theme Presets Contract _HIGH_
- `canon-rule-26-elevation-shadow-system.md` — #26: Elevation & Shadow System _MEDIUM_
- `canon-rule-27-motion-timing-tokens.md` — #27: Motion & Timing Tokens _MEDIUM_
- `canon-rule-29-typography-contract.md` — #29: Typography Contract _HIGH_
- `canon-rule-30-iconography-system.md` — #30: Iconography System _MEDIUM_
- `canon-rule-33-density-accessibility-mapping.md` — #33: Density & Accessibility Mapping _HIGH_
- `canon-rule-35-token-usage-validation.md` — #35: Token Usage Validation _HIGH_
- `canon-rule-38-theme-engine-contract.md` — #38: Theme Engine Contract _CRITICAL_
- `canon-rule-62-single-source-of-truth-tokens.md` — #62: Single Source of Truth for Design Tokens _CRITICAL_
- `canon-rule-65-theme-provider-dom-sync.md` — #65: data-theme Sync is ThemeProvider Responsibility _HIGH_
- `canon-rule-71-debug-theme-verify-file-first.md` — #71: Debug Theme by Verifying File First _MEDIUM_
- `canon-rule-78-token-definition-completeness.md` — #78: Token Definition Completeness _HIGH_
- `canon-rule-84-svg-dark-mode-contract.md` — #84: SVG Dark Mode Contract _HIGH_
- `canon-rule-107-token-architecture-theme-specificity.md` — #107: Token Architecture & Theme Specificity _CRITICAL_
- `canon-rule-108-visual-surfaces-contract.md` — #108: Visual Surfaces Contract _CRITICAL_
- `canon-rule-155-css-token-contract-is-architecture.md` — #155: CSS Token Contract Is Architecture _CRITICAL_
- `canon-rule-159-css-ui-fail-safe-template.md` — #159: UI CSS Must Be Fail-Safe, Token-Only, and Attribute-Scoped _CRITICAL_
- `canon-rule-168-ui-must-declare-required-tokens.md` — #168: UI Must Declare Required Tokens _CRITICAL_
- `canon-rule-169-token-order-is-architectural.md` — #169: Token Import Order Is Architectural _HIGH_
- `canon-rule-171-no-phantom-variables.md` — #171: Phantom Variables Are Forbidden _CRITICAL_
- `canon-rule-174-tokens-are-compile-time-contracts.md` — #174 — Tokens Are Compile-Time Contracts _BLOCKING_
- `canon-rule-179-no-visual-surfaces-in-core-or-families.md` — #179: No Visual Surface Tokens in Core or Families _CRITICAL_
- `canon-rule-180-semantic-token-names-canonical-suffixes.md` — #180: Semantic Token Names Must Follow Canonical Suffixes _CRITICAL_
- `canon-rule-182-semantic-role-must-affect-visual-contrast.md` — #182: Semantic Role Must Affect Visual Contrast _HIGH_
- `canon-rule-201-presets-only-color-source.md` — #201: Presets Are the Only Source of Color Values _CRITICAL_
- `canon-rule-202-themes-resolve-context-not-palette.md` — #202: Themes Resolve Context, Not Palette _CRITICAL_
- `canon-rule-203-semantic-tokens-bridge-theme-families.md` — #203: Semantic Tokens Are the Only Bridge Between Theme and Families _CRITICAL_
- `canon-rule-205-ui-tokens-bind-semantic-never-presets.md` — #205: UI Tokens Must Bind to Semantic Tokens, Never to Presets _CRITICAL_
- `canon-rule-207-preset-switching-never-changes-component-css.md` — #207: Preset Switching Must Never Change Component CSS _CRITICAL_
- `canon-rule-234-tokens-engine-single-source-of-truth.md` — #234: Tokens Engine Is the Single Source of CSS Truth _CRITICAL_
- `canon-rule-235-ui-must-consume-semantic-not-theme.md` — #235: UI Layer Must Consume Semantic Tokens, Never Theme Directly _CRITICAL_
- `canon-rule-240-generated-artifacts-must-not-be-committed.md` — #240: Generated Artifacts Must Not Be Committed _CRITICAL_
- `canon-rule-241-tokens-engine-is-build-step-not-runtime.md` — #241: Tokens Engine Is Build Infrastructure, Not Runtime Code _CRITICAL_
- `canon-rule-242-cascade-order-is-a-contract.md` — #242: Cascade Order Is a Contract, Not a Preference _CRITICAL_
- `canon-rule-244-semantic-layer-is-mandatory-abstraction.md` — #244: Semantic Layer Is a Mandatory Abstraction _CRITICAL_
- `canon-rule-245-families-must-not-leak-global-state.md` — #245: Families Must Not Leak Global State _HIGH_
- `canon-rule-247-entry-order-is-architectural.md` — #247: CSS Entry Order Is Architectural, Not Cosmetic _CRITICAL_
- `canon-rule-248-tokens-engine-is-the-only-source-of-truth.md` — #248: Tokens Engine Is the Only Source of Truth _CRITICAL_
- `canon-rule-253-token-cascade-order-is-immutable.md` — #253: Token Cascade Order Is Immutable _CRITICAL_
- `canon-rule-257-tokens-engine-is-mandatory-pre-build-step.md` — #257: Tokens Engine Is a Mandatory Pre-Build Step _CRITICAL_
- `canon-rule-278-primitive-must-not-encode-design-tokens.md` — #278: Primitive Must Not Encode Design Tokens in Rust _CRITICAL_

---

## ui

**39 rules**

- `canon-rule-03-lists.md` — #03: Lists and Iteration _MEDIUM_
- `canon-rule-12-select-vs-combobox.md` — #12 — Select vs Combobox _MEDIUM_
- `canon-rule-14-datatable-vs-virtualtable.md` — #14 — DataTable vs VirtualTable _MEDIUM_
- `canon-rule-15-pagination-vs-virtualization.md` — #15 — Pagination vs Virtualization _MEDIUM_
- `canon-rule-17-human-vs-machine-scale.md` — #17 — Human-scale vs Machine-scale Components _HIGH_
- `canon-rule-31-accessibility-contract.md` — #31: Accessibility Contract (ARIA + Roles) _CRITICAL_
- `canon-rule-39-settings-ui-compliance.md` — #39: Settings UI Compliance _MEDIUM_
- `canon-rule-42-ui-data-surfaces.md` — #42: UI Data Surfaces _HIGH_
- `canon-rule-46-command-palette-intent-surfaces.md` — #46: Command Palette & Intent Surfaces _MEDIUM_
- `canon-rule-54-render-must-be-total.md` — #54 — Render Must Be Total _CRITICAL_
- `canon-rule-74-block-semantic-html.md` — #74: Block Semantic HTML _HIGH_
- `canon-rule-76-navigation-vs-action-contract.md` — #76: Navigation vs Action Component Contract _HIGH_
- `canon-rule-81-flex-layout-ownership.md` — #81: Flex Layout Ownership _HIGH_
- `canon-rule-91-markdown-render-only.md` — #91: Markdown and Code Blocks Are Render-Only _HIGH_
- `canon-rule-105-visual-indicators-must-have-a-single-owner.md` — #105 — Visual Indicators Must Have a Single Owner _HIGH_
- `canon-rule-112-ui-owns-visual-style.md` — #112: UI Owns Visual Style _CRITICAL_
- `canon-rule-120-dom-events-vs-semantic-callbacks.md` — #120: DOM Events vs Semantic Callbacks Boundary _HIGH_
- `canon-rule-125-ui-component-contracts.md` — #125: UI Component Contracts _CRITICAL_
- `canon-rule-135-ui-vs-layout-responsibility-boundary.md` — #135: UI vs Layout Responsibility Boundary _CRITICAL_
- `canon-rule-142-children-must-always-be-optional.md` — #142 — Children Must Always Be Optional in Public Components _HIGH_
- `canon-rule-143-ui-must-be-hydration-deterministic.md` — #143 — UI Must Be Hydration-Deterministic _CRITICAL_
- `canon-rule-145-ui-must-not-mutate-global-state-implicitly.md` — #145 — UI Components Must Not Mutate Global State Implicitly _HIGH_
- `canon-rule-146-ui-content-must-be-ssr-stable.md` — #146 — UI Content Must Be SSR-Stable _CRITICAL_
- `canon-rule-147-reactive-closures-are-data-not-structure.md` — #147 — Reactive Closures Are Data, Not Structure _HIGH_
- `canon-rule-148-ui-must-never-infer-intent-from-state.md` — #148 — UI Must Never Infer Intent From State _HIGH_
- `canon-rule-149-controllers-must-be-temporal-not-structural.md` — #149 — Controllers Must Be Temporal, Not Structural _HIGH_
- `canon-rule-150-ui-must-be-deterministic-under-ssr-and-hydration.md` — #150 — UI Must Be Deterministic Under SSR and Hydration _CRITICAL_
- `canon-rule-151-visual-feedback-must-never-encode-application-state.md` — #151 — Visual Feedback Must Never Encode Application State _HIGH_
- `canon-rule-162-providers-not-ui.md` — #162: Providers Are Infrastructure, Not UI _HIGH_
- `canon-rule-168-ui-must-declare-required-tokens.md` — #168: UI Must Declare Required Tokens _CRITICAL_
- `canon-rule-170-html-css-contract-must-match.md` — #170: HTML and CSS Must Share the Same Contract _CRITICAL_
- `canon-rule-193-framework-behaviors-are-the-only-source-of-interactivity.md` — #193: Framework Behaviors Are the Only Source of Interactivity _CRITICAL_
- `canon-rule-194-ui-components-are-pure-render-functions.md` — #194: UI Components Are Pure Render Functions _CRITICAL_
- `canon-rule-206-ui-inventory-is-fixed.md` — #206: UI Inventory Is Fixed and Canonical _CRITICAL_
- `canon-rule-211-ssr-meta-requires-html-shell.md` — #211: SSR Meta Requires Explicit HTML Shell _HIGH_
- `canon-rule-216-app-must-not-define-html.md` — #216: App Components Must Not Define HTML Structure _HIGH_
- `canon-rule-235-ui-must-consume-semantic-not-theme.md` — #235: UI Layer Must Consume Semantic Tokens, Never Theme Directly _CRITICAL_
- `canon-rule-267-data-attributes-are-contract-not-style.md` — #267: Data Attributes Are Contract, Not Styling Mechanism _HIGH_
- `canon-rule-279-ui-must-not-know-behavior.md` — #279: UI Layer Must Not Be Aware of Behavior Layer _CRITICAL_

---

## wasm

**11 rules**

- `canon-rule-93-leptos-wasm-dev-builds-must-use-release-mode.md` — #93: Leptos WASM Dev Builds Must Use Release Mode _CRITICAL_
- `canon-rule-99-csr-islands-must-not-own-routing-or-providers.md` — #99: CSR Islands Must Not Own Routing or Providers _HIGH_
- `canon-rule-116-wasm-externref-table-limits.md` — #116: WASM Externref Table Limits _CRITICAL_
- `canon-rule-196-ssr-csr-separation-wasm-bloat-prevention.md` — #196: SSR/CSR Separation for WASM Bloat Prevention _CRITICAL_
- `canon-rule-210-cdylib-mandatory-for-hydration.md` — #210: cdylib Is Mandatory for Hydration _CRITICAL_
- `canon-rule-212-hydration-bootstrap-tooling-owned.md` — #212: Hydration Bootstrap Is Tooling-Owned _CRITICAL_
- `canon-rule-228-wasm-target-configuration.md` — #228: Workspace Must Define WASM Target Configuration _CRITICAL_
- `canon-rule-230-wasm-artifact-budget-is-hard-contract.md` — #230: WASM Artifact Budget Is a Hard Contract _CRITICAL_
- `canon-rule-231-ssr-link-isolation-refined.md` — #231 (Refined): SSR Crates Must Be Link-Time Isolated From WASM Graph _CRITICAL_
- `canon-rule-264-behavior-registry-is-single-entry-point.md` — #264: Behavior Registry Is the Single Runtime Entry Point _CRITICAL_
- `canon-rule-268-behavior-cleanup-is-mandatory.md` — #268: Behavior Cleanup Is Mandatory and Deterministic _CRITICAL_

---

## workspace

**30 rules**

- `canon-rule-56-monorepo-css-build-pipeline.md` — #56: Monorepo CSS Build Pipeline _HIGH_
- `canon-rule-66-workbench-setup-checklist.md` — #66: Workbench Setup Checklist _LOW_
- `canon-rule-68-asset-must-exist-in-dist.md` — #68: Asset Must Exist in Final dist/ _HIGH_
- `canon-rule-69-trunk-only-serves-dist.md` — #69: Trunk Only Serves What's in dist/ _HIGH_
- `canon-rule-77-monorepo-path-stability.md` — #77: Monorepo Path Stability _HIGH_
- `canon-rule-80-workspace-watch-configuration.md` — #80: Workspace Watch Configuration _MEDIUM_
- `canon-rule-94-leptos-workspace-features-must-be-explicit.md` — #94: Leptos Workspace Features Must Be Explicit _CRITICAL_
- `canon-rule-97-leptos-08-requires-floating-nightly-toolchain.md` — #97: Leptos 0.8 Requires Floating Nightly Toolchain _MEDIUM_
- `canon-rule-100-build-orchestrators-must-be-workspace-scoped.md` — #100: Build Orchestrators Must Be Workspace-Scoped _MEDIUM_
- `canon-rule-198-runtime-crates-must-not-leak-cross-target-types.md` — #198: Runtime Crates Must Not Leak Cross-Target Types _CRITICAL_
- `canon-rule-200-shared-crates-must-be-zero-dependency-runtime.md` — #200: Shared Crates Must Be Zero-Dependency Runtime _CRITICAL_
- `canon-rule-219-leptos-consistent-ports.md` — #219: Leptos Product Must Declare Consistent Ports Across Workspace and Local Config _CRITICAL_
- `canon-rule-220-unique-build-targets.md` — #220: Workspace Metadata Must Define Unique Build Targets Per Product _CRITICAL_
- `canon-rule-221-critical-config-alignment.md` — #221: Leptos.toml Critical Fields Must Align With Workspace Metadata _HIGH_
- `canon-rule-222-cargo-metadata-resolvable.md` — #222: Workspace Members Must Be Fully Resolvable by Cargo Metadata _CRITICAL_
- `canon-rule-223-feature-flag-isolation.md` — #223: Feature Flag Scopes Must Not Leak Between Products _CRITICAL_
- `canon-rule-224-hyphen-naming-convention.md` — #224: Workspace Package Names Must Use Hyphens Consistently _CRITICAL_
- `canon-rule-225-workspace-exact-version-policy-v2.md` — #225 (v2.0): Workspace Dependency Version Policy Must Prevent ABI Drift Without Blocking Patch Compatibility _CRITICAL_
- `canon-rule-226-root-relative-paths.md` — #226: Workspace Leptos Paths Must Be Root-Relative _HIGH_
- `canon-rule-227-workspace-dependency-inheritance.md` — #227: Products Must Inherit Workspace Dependencies _HIGH_
- `canon-rule-228-wasm-target-configuration.md` — #228: Workspace Must Define WASM Target Configuration _CRITICAL_
- `canon-rule-236-cli-defines-workspace-topology.md` — #236: CanonRS CLI Defines Workspace Topology _CRITICAL_
- `canon-rule-237-cli-owns-leptos-metadata.md` — #237: CLI Owns Leptos Metadata Blocks _CRITICAL_
- `canon-rule-238-workspace-graph-must-be-acyclic.md` — #238: Workspace Dependency Graph Must Be Acyclic _CRITICAL_
- `canon-rule-239-cli-sync-must-be-idempotent.md` — #239: CLI Sync Operations Must Be Idempotent _HIGH_
- `canon-rule-250-design-system-must-not-depend-on-products.md` — #250: Design System Must Not Depend on Products _CRITICAL_
- `canon-rule-255-cli-authority-over-workspace-generation.md` — #255: CLI Is the Sole Authority Over Workspace Generation _CRITICAL_
- `canon-rule-256-generated-workspace-is-ephemeral.md` — #256: Generated Workspace Is Ephemeral and Immutable _CRITICAL_
- `canon-rule-263-workspace-must-declare-crate-boundary-visibility.md` — #263: Workspace Crate Boundaries Must Be Explicitly Declared _CRITICAL_
- `canon-rule-281-no-cross-crate-layer-leaks.md` — #281: Cross-Crate Layer Leakage Is Forbidden _CRITICAL_

---
