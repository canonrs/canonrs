# canonrs-server

The main component library of CanonRS.

Contains all 80+ Leptos UI components, layout primitives, and composite blocks. Compiles on both SSR and WASM targets. Heavy dependencies are strictly opt-in via the `ssr` feature.

## Responsibility

Implements the actual Leptos components whose type contracts are defined in `canonrs-core`. Also hosts the markdown and code highlighting pipeline for SSR rendering.

## Modules

### `ui/`
All UI components. Each component lives in its own subdirectory with this structure:
```
ui/button/
├── button_ui.rs     ← component implementation
├── examples.rs      ← usage examples
└── mod.rs           ← public exports
```

Full component list: accordion, alert, alert_dialog, animate, aspect_ratio, avatar, badge, banner, breadcrumb, button, button_group, callout, card, carousel, chart, checkbox, code_block, collapsible, color_picker, combobox, command, confirm_dialog, context_menu, copy_button, data_table, dialog, doc_progress, drawer, dropdown_menu, empty_state, empty_table, error_state, field, form_error_summary, hover_card, icon, icon_button, inline_notice, input, input_group, input_otp, kbd, label, link, list_item, loading_overlay, **markdown**, menu, menubar, modal, nav_item, navigation_menu, page_header, pagination, popover, progress, pulse, radio, radio_group, resizable, scroll_area, select, separator, sheet, sidebar, skeleton, slider, spinner, stat, status_dot, switch, table, table_of_contents, tabs, textarea, toast, toggle, toggle_group, toolbar, tooltip, tree, virtual_list.

#### Markdown pipeline

The markdown component is the only SSR-heavy component in the library. Its rendering pipeline is gated behind the `ssr` feature:
```
ui/markdown/
├── markdown_ui.rs        ← Leptos component (always compiled)
├── markdown_toolbar.rs   ← toolbar component (always compiled)
├── examples.rs           ← usage examples (always compiled)
├── renderer.rs           ← syntect + pulldown-cmark (ssr only)
└── toc_extractor.rs      ← TOC extraction (ssr only)
```

Public API is stable across targets:
```rust
// Always available — returns RenderedMarkdown::default() on WASM
pub fn render_markdown(md: &str) -> RenderedMarkdown
pub fn render_with_prefix(md: &str, prefix: &str) -> RenderedMarkdown
```

### `layouts/`
Page layout components:
- `section/` — content section with header/footer slots
- `page_layout/` — full page wrapper
- `dashboard/` — dashboard layout with sidebar
- `fullscreen/` — fullscreen layout
- `marketing/` — marketing page layout
- `split_view/` — two-panel split layout
- `wizard/` — multi-step wizard layout

### `blocks/`
Composite blocks built from multiple UI components:
alert, breadcrumb, button_group, callout, card, code_block, command_panel, data_table, dialog, drawer, empty_state, field, footer, form, form_actions, header, list, markdown_surface, page_header, popover, skeleton, stat_card, table, toolbar.

### `providers/`
SSR-side providers:
- `canonrs_root` — root provider with SSR hydration setup
- `layout_provider` — layout context
- `sidebar_provider` — sidebar state
- `theme_script` — inline theme script to prevent FOUC

### `pages/`
Full page components for the CanonRS documentation site.

### `primitives.rs`
Re-exports primitives from `canonrs-core` for internal use.

## Features

| Feature | Activates |
|---------|-----------|
| `ssr` | `syntect`, `pulldown-cmark`, `html-escape`, `axum`, `tokio`, `leptos_axum`, markdown renderer, TOC extractor |
| `hydrate` | Leptos hydration mode |

## SSR-only dependencies

These crates never enter the WASM build graph:

| Crate | Purpose |
|-------|---------|
| `syntect` | Syntax highlighting for `CodeBlock` and `Markdown` |
| `pulldown-cmark` | Markdown parsing |
| `html-escape` | HTML sanitization in markdown output |
