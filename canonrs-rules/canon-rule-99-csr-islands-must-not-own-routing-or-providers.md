# Canon Rule #99: CSR Islands Must Not Own Routing or Providers

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** ssr, csr, wasm
**Version:** 1.0.0  
**Date:** 2025-01-15

---

## Principle

**WASM islands in an SSR-first application MUST NOT create their own Router or provide global Providers.**

Islands are guests in the SSR host's DOM. They mount at specific nodes, render local UI, and unmount cleanly. Routing and providers are the host's responsibility.

---

## The Problem

When WASM islands try to own routing or providers:

- Router conflicts with SSR's router (two routers fighting)
- Provider contexts leak between islands and host
- Hydration mismatches (SSR expects one tree, CSR creates another)
- Islands cannot be lazy-loaded independently
- Memory leaks on unmount (providers not cleaned up)

### Real Symptoms
```
✗ Hydration mismatch: expected <Router>, found <div>
✗ Context "Theme" already exists in tree
✗ Cannot unmount island: router still active
✗ Navigation broken after island mount
```

---

## Anti Pattern
```rust
// ❌ FORBIDDEN: Island creates router
#[wasm_bindgen]
pub fn mount_markdown(selector: &str) {
    mount_to(
        selector,
        || view! {
            <Router>  // ← WRONG: Island creating router
                <Routes>
                    <Route path="/docs" view=MarkdownViewer/>
                </Routes>
            </Router>
        }
    );
}

// ❌ FORBIDDEN: Island provides global context
#[wasm_bindgen]
pub fn mount_editor(selector: &str) {
    mount_to(
        selector,
        || view! {
            <ThemeProvider>  // ← WRONG: Island providing context
                <CodeEditor/>
            </ThemeProvider>
        }
    );
}
```

This fails because:
- SSR already has a Router managing navigation
- SSR already has ThemeProvider in component tree
- Island's router/provider conflicts with host
- Unmounting the island leaves orphaned contexts

---

## Canonical Pattern
```rust
// ✅ REQUIRED: Island is just a component
#[wasm_bindgen]
pub fn mount_markdown(selector: &str) {
    mount_to(
        selector,
        || view! {
            <MarkdownViewer content=initial_content />
        }
    );
}

// ✅ REQUIRED: Island uses host's context
#[wasm_bindgen]
pub fn mount_editor(selector: &str) {
    mount_to(
        selector,
        || view! {
            <CodeEditor/>  // Uses theme from host's ThemeProvider
        }
    );
}

// ✅ Host provides router and contexts
#[component]
pub fn App() -> impl IntoView {
    view! {
        <ThemeProvider>
            <Router>
                <Routes>
                    <Route path="/docs" view=DocsPage/>
                </Routes>
            </Router>
        </ThemeProvider>
    }
}

#[component]
fn DocsPage() -> impl IntoView {
    view! {
        <div>
            <h1>"Documentation"</h1>
            <div id="markdown-mount"></div>  // ← Island mounts here
        </div>
    }
}
```

Host manages routing, providers, and island mount points.

---

## Island Responsibilities Allowed

Islands MAY:

- Render local UI components
- Manage local state (useState, RwSignal)
- Use contexts provided by host (use_theme, use_density)
- Handle local events (onClick, onChange)
- Make API calls
- Update their own DOM subtree

Islands MUST NOT:

- Create Router
- Provide ThemeProvider, DensityProvider, or other global contexts
- Modify DOM outside their mount node
- Listen to global route changes (host handles this)
- Persist state in global storage (use host's state management)

---

## Host Responsibilities Required

SSR host MUST:

- Provide single Router for entire app
- Provide all global contexts (Theme, Density, etc.)
- Create mount points for islands (`<div id="island-mount">`)
- Lazy-load island WASM on demand
- Unmount islands when route changes
- Pass initial props to islands via data attributes

---

## Communication Patterns

### Host To Island Props
```rust
// Host
view! {
    <div
        id="editor-mount"
        data-theme="dark"
        data-lang="rust"
    ></div>
}

// Island
#[wasm_bindgen]
pub fn mount_editor(selector: &str) {
    let el = document().query_selector(selector).unwrap().unwrap();
    let theme = el.get_attribute("data-theme").unwrap();
    let lang = el.get_attribute("data-lang").unwrap();

    mount_to(el, move || view! {
        <CodeEditor theme=theme lang=lang/>
    });
}
```

### Island To Host Events
```rust
// Island dispatches custom event
#[wasm_bindgen]
pub fn mount_editor(selector: &str) {
    mount_to(selector, || view! {
        <CodeEditor on_save=move |code| {
            let event = CustomEvent::new("editor:save").unwrap();
            event.detail().set("code", code);
            window().dispatch_event(&event);
        }/>
    });
}

// Host listens for event
window.addEventListener("editor:save", (e) => {
    console.log("Code saved:", e.detail.code);
});
```

---

## Lazy Loading Islands
```rust
// Host lazy-loads island
#[component]
fn DocsPage() -> impl IntoView {
    let load_island = create_action(|_| async {
        // Dynamic import
        let module = js_sys::eval(
            r#"import("/islands/markdown/markdown.js")"#
        ).await;
        module.mount_markdown("#markdown-mount");
    });

    view! {
        <div>
            <button on:click=move |_| load_island.dispatch(())>
                "Load Markdown Viewer"
            </button>
            <div id="markdown-mount"></div>
        </div>
    }
}
```

---

## Diagnostic Checklist

If islands cause hydration or routing issues:
```bash
# 1. Check island code for Router
grep -r "Router" islands/*/src/

# 2. Check island code for Providers
grep -r "Provider" islands/*/src/

# 3. Verify island only uses mount_to
grep -r "mount_to" islands/*/src/

# 4. Check host provides contexts
grep -r "ThemeProvider\|DensityProvider" shell/src/lib.rs

# 5. Verify islands don't modify global state
grep -r "window\." islands/*/src/
```

---

## Enforcement

- All WASM islands MUST use `mount_to()` only
- Islands MUST NOT create Router
- Islands MUST NOT provide global contexts
- Islands MUST NOT access DOM outside mount node
- Host MUST provide all routing and contexts
- Code review MUST verify island isolation

---

## Canonical Justification

> **Islands are components, not applications.**  
> They render at specific DOM nodes managed by the host.  
> Router and providers are architectural concerns, not component concerns.

This rule exists to:
- Maintain SSR's architectural authority
- Enable true lazy loading of islands
- Prevent context/router conflicts
- Allow islands to mount/unmount cleanly
- Keep islands small and focused

---

## Related Canon Rules

- Canon Rule #95 — SSR Requires Complete HTML Shell
- Canon Rule #96 — SSR Requires Explicit Provider Tree
- Canon Rule #90 — Hydration Is DOM Replacement

---

## Version History

- **1.0.0** (2025-01-15): Initial rule based on Workbench migration SSR-first + islands architecture
