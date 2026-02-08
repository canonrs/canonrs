---
component: Table
layer: UI
status: Stable
since: v0.1
last_review: 2025-01-15
ownership: canonrs
keywords:
  - design system
  - leptos
  - ssr
  - table
  - data grid
path primitive: src/primitives/table.rs
path UI: src/ui/table/
---

# Table — UI Component Documentation

## 1. Conceptual Introduction

The **Table** is a UI component that renders structured tabular data in semantic HTML format. It composes the primitives `TablePrimitive`, `TableHeaderPrimitive`, `TableBodyPrimitive`, `TableRowPrimitive`, `TableHeadPrimitive`, and `TableCellPrimitive` to create accessible, responsive, and token-styled tables.

The Table exists in CanonRS to:
- Guarantee semantic HTML structure (`<table>`, `<thead>`, `<tbody>`, `<tr>`, `<th>`, `<td>`)
- Apply consistent styling via CSS tokens
- Provide ergonomic API for Leptos developers
- Support visual states (hover, selection) via data-attributes

**What it does NOT do:**
- Pagination (use blocks/DataTable)
- Sorting/filtering (use blocks/DataTable)
- Virtualization (use VirtualTable)
- Remote data fetching

---

## 2. Architectural Responsibility (Contract)

### Responsibility

- Render semantic HTML `<table>` in SSR
- Expose components `Table`, `TableHeader`, `TableBody`, `TableFooter`, `TableRow`, `TableHead`, `TableCell`, `TableCaption`
- Apply `data-*` attributes for CSS hooks
- Support `class` prop for external styling
- Maintain accessible DOM structure (implicit ARIA via native elements)

### Non-Responsibility

- Manage data (that's the parent component's job)
- Execute business logic (sorting, filtering)
- Access browser APIs (`window`, `document`)
- Implement virtual scrolling
- Control selection state (uses `selected` prop passed by parent)

---

## 3. Position in CanonRS Ecosystem
```text
User Query → DataTable (Block) → Table (UI) → TablePrimitive → HTML <table> → CSS [data-table]
```

**Flow:**
1. Blocks (like `DataTable`) manage data and state
2. `Table` UI composes primitives with friendly API
3. Primitives render pure HTML with `data-*`
4. Shell CSS applies tokens via `[data-table-*]` selectors
5. Browser renders styled table

**SSR:** All HTML is generated on the server. Hydration is not necessary (purely visual component).

---

## 4. Applied Tokens (REQUIRED)

### Layout
- `--space-md` — cell padding
- `--list-item-height` — `<th>` height

### Typography
- `--font-size-sm` — table size
- `--font-weight-semibold` — headers
- `--font-weight-medium` — footer

### Color
- `--border` — borders (HSL)
- `--muted` — header/footer/hover background (HSL + alpha)
- `--foreground` — header text (HSL)
- `--muted-foreground` — caption (HSL)

### Border
- `--border-width-hairline` — 1px (lines)
- `--border-width-thin` — 2px (header)

### Motion
- `--motion-duration-fast` — hover transition
- `--motion-ease-standard` — easing

---

## 5. Technical Structure (How It Works)

### Generated HTML (SSR)
```html
<div data-table>
  <table>
    <thead data-table-header>
      <tr data-table-row>
        <th data-table-head>Header</th>
      </tr>
    </thead>
    <tbody data-table-body>
      <tr data-table-row data-state="unselected">
        <td data-table-cell>Cell</td>
      </tr>
    </tbody>
  </table>
</div>
```

**Contract:**
- `data-table` → wrapper with scroll
- `data-table-header/body/footer` → semantic sections
- `data-table-row` → rows with `data-state="selected|unselected"`
- `data-table-head/cell` → cells

**Hydration:** Not needed. Component is static (except CSS hover).

---

## 6. Execution Flow
```text
SSR Render (Leptos)
 → HTML <table> with data-* attributes
 → Shell CSS loads (inline in <head>)
 → Browser applies [data-table-*] rules
 → Hover/selection via CSS pseudo-classes (:hover, [data-state])
```

**No runtime JS.** All behavior is pure CSS.

---

## 7. Canonical Use Cases

### Comparison table (ComparisonBlock)
```rust
<Table>
  <TableHeader>
    <TableRow>
      <TableHead>"Feature"</TableHead>
      <TableHead>"Traditional"</TableHead>
      <TableHead>"Canon"</TableHead>
    </TableRow>
  </TableHeader>
  <TableBody>
    {rows.map(|r| view! {
      <TableRow>
        <TableCell>{r.feature}</TableCell>
        <TableCell>{r.traditional}</TableCell>
        <TableCell>{r.canon}</TableCell>
      </TableRow>
    }).collect::<Vec<_>>()}
  </TableBody>
</Table>
```

### Table with selection (DataTable)
```rust
<TableRow selected=is_selected>
  <TableCell>...</TableCell>
</TableRow>
```

### Table with footer
```rust
<TableFooter>
  <TableRow>
    <TableCell>"Total"</TableCell>
    <TableCell>{total}</TableCell>
  </TableRow>
</TableFooter>
```

---

## 8. Anti-Patterns (PROHIBITED)

### ❌ WRONG: Using `<div>` instead of `<table>`
```rust
// DON'T DO THIS
<div class="fake-table">
  <div class="row">...</div>
</div>
```
**Why:** Breaks HTML semantics and accessibility.

### ❌ WRONG: Managing state inside Table
```rust
// DON'T DO THIS
#[component]
pub fn Table(data: Vec<Row>) {
  let (sorted, set_sorted) = signal(data);
  // sorting logic here
}
```
**Why:** Table is presentational. State must come from outside (via props).

### ❌ WRONG: Applying inline styles
```rust
// DON'T DO THIS
<TableCell style="color: red;">...</TableCell>
```
**Why:** Violates token architecture. Use `class` or data-attributes.

### ❌ WRONG: Mixing primitives and raw HTML
```rust
// DON'T DO THIS
<TableBody>
  <tr><td>...</td></tr> // raw HTML
</TableBody>
```
**Why:** Breaks data-attribute contract.

---

## 9. SSR, Hydration and Runtime

### SSR
- All HTML is generated on the server
- Data-attributes applied in initial render
- CSS inline in `<head>` (Shell)

### Hydration
- **Not needed** for basic Table
- If using events (`on:click` on `TableRow`), Leptos hydrates automatically

### Runtime
- Hover: pure CSS (`:hover`)
- Selection: `data-state` controlled by parent (reactive via Leptos signals)

### Hot-reload
- Changes in `.rs` → recompiles Leptos → updates HTML
- Changes in `.css` → rebuild CSS → hard refresh (Ctrl+Shift+F5)

---

## 10. Conformance Checklist

- [x] SSR-safe (pure HTML)
- [x] No imperative JS
- [x] Uses canonical tokens
- [x] Tokens listed (section 4)
- [x] Anti-patterns documented (section 8)
- [x] Rules cited (section 11)
- [x] Isolated primitives (no logic)
- [x] UI composes primitives
- [x] CSS via data-attributes

---

## 11. Canon Rules Applied

- **Canon Rule #102** — Runtime JS Is Shell Infrastructure  
  *Table uses no JS runtime. All behavior is pure CSS.*

- **Canon Rule #103** — Critical Runtime JS Must Be Inline in SSR  
  *Table CSS is inline in `<head>` via Shell.*

- **Canon Rule #104** — AutoReload Breaks Script Order Guarantees  
  *Table doesn't depend on external JS. Hot-reload is safe.*

- **Canon Rule #93** — Leptos WASM Dev Builds Must Use Release Mode  
  *Table compiles in release to avoid WASM overhead.*

- **Canon Rule #100** — Build Orchestrators Must Be Workspace-Scoped  
  *CSS build runs via `npm run build:css` in workspace.*
