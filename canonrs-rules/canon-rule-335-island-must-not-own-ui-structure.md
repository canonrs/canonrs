# Canon Rule #335: Island Must Not Own UI Structure When Content Is Compositional

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-05

**Category:** islands-architecture
**Tags:** islands, ssr, children, composition, structure
**Language:** EN

---

**Intro:**
Islands must control state, not render UI structure. When content is compositional — cards, tables, grids, rich components — it belongs in the SSR shell. Passing rich content as `Vec<String>` or serialized data to an island destroys composability and type safety.

**Problem:**
Islands receive `Vec<TabItem { content: String }>` forcing all content to be serialized as plain strings, losing Leptos component composition.

**Solution:**
Island wraps children via `Children`. SSR defines structure. Island provides context for state.

**Signals:**
- island prop contains HTML as string
- `Vec<T>` with content fields passed to island
- island renders cards, tables, or grids internally

**Search Intent:**
leptos island children rich content, island vec string content, island composition pattern

**Keywords:**
leptos island children, island SSR composition, island structure separation

---

## Principle

UI = f(state). Islands own the function. SSR owns the structure. Never invert this.

---

## Problem
```rust
// ❌ broken — content serialized as String, loses composition
#[island]
pub fn TabsIsland(tabs: Vec<TabItem>, active: String) -> impl IntoView {
    // TabItem { value: String, label: String, content: String }
    // content: String cannot hold <Card />, <Table />, <Grid />
}
```

---

## Patterns

### Forbidden Pattern
- `content: String` inside island props for rich UI content
- island renders `<Card>`, `<Table>`, `<Grid>` internally from serialized data
- `Vec<T>` where T contains UI content fields

### Canonical Pattern
```rust
// ✅ Island owns state only
#[island]
pub fn TabsRootIsland(children: Children, #[prop(into)] initial: String) -> impl IntoView {
    let active = RwSignal::new(initial);
    provide_context(TabsContext(active));
    view! { <div data-rs-tabs="">{children()}</div> }
}

// ✅ SSR defines structure with rich components
view! {
    <TabsRootIsland initial="proof">
        <TabsTriggerIsland value="proof">"Proof"</TabsTriggerIsland>
        <TabsContentIsland value="proof">
            <Card>...</Card>
            <Table>...</Table>
        </TabsContentIsland>
    </TabsRootIsland>
}
```

---

## Contract

### Enforcement
- island props must never contain UI content as String
- rich content must be passed via `Children`
- island must not render layout components (Grid, Card, Table) from internal data

### Exceptions
- `TabsIsland` with `Vec<TabItem>` is allowed only when content is plain text (e.g. preview demos)

---

## Version History
- 1.0.0 - Initial definition
