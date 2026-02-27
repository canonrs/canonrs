use leptos::prelude::*;
use canonrs_ui::blocks::{Card, AlertBlock, AlertVariant, StatCard, CalloutBlock, CalloutType, EmptyState, ToolbarBlock, PageHeader, Breadcrumb, BreadcrumbItem, ButtonGroupBlock, FormBlock, FieldBlock, FormActionsBlock, DataTableBlock, List, ListItem, CodeBlockBlock};
use rs_canonrs::domain::{CanonNode, CanonBlockType};

/// Renderiza um CanonNode recursivamente.
/// Desacoplado do builder — não conhece drag, selection, drop zones.
/// É o renderer oficial de páginas CanonRS.
pub fn render_node(node: &CanonNode) -> AnyView {
    leptos::logging::log!("render_node: {:?} children={}", node.block, node.children.len());
    let children_views: Vec<AnyView> = node.children.iter()
        .map(|c| render_node(c))
        .collect();

    match &node.block {
        CanonBlockType::Slot { name } => {
            let name = name.clone();
            view! {
                <div data-canon-slot=name>
                    {children_views}
                </div>
            }.into_any()
        }

        CanonBlockType::Card => view! {
            <div data-canon-block="card" style="padding: 1rem; background: var(--theme-surface-bg); border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md); box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                {if children_views.is_empty() {
                    view! { <div style="font-size: 0.8rem; color: var(--theme-surface-fg-muted);">"Card"</div> }.into_any()
                } else {
                    children_views.into_iter().collect_view().into_any()
                }}
            </div>
        }.into_any(),

        CanonBlockType::Alert => view! {
            <AlertBlock variant=AlertVariant::Info>
                "Alert"
            </AlertBlock>
        }.into_any(),

        CanonBlockType::Header => view! {
            <header data-canon-block="header" class="canon-header">{children_views}</header>
        }.into_any(),

        CanonBlockType::Footer => view! {
            <footer data-canon-block="footer" class="canon-footer">{children_views}</footer>
        }.into_any(),

        CanonBlockType::Section => view! {
            <div data-canon-block="section" style="padding: 1rem; border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md); background: var(--theme-surface-bg);">
                {children_views}
            </div>
        }.into_any(),

        CanonBlockType::Form => view! {
            <div data-canon-block="form" style="display: flex; flex-direction: column; gap: var(--space-md); min-height: 2rem;">
                {if children_views.is_empty() {
                    view! { <div style="font-size: 0.8rem; color: var(--theme-surface-fg-muted); padding: 0.5rem;">""</div> }.into_any()
                } else {
                    children_views.into_iter().collect_view().into_any()
                }}
            </div>
        }.into_any(),

        CanonBlockType::Field => view! {
            <div data-canon-block="field" style="display: flex; flex-direction: column; gap: var(--space-xs);">
                <label style="font-size: 0.8rem; font-weight: 500; color: var(--theme-surface-fg);">"Field"</label>
                <input
                    type="text"
                    style="padding: 0.5rem 0.75rem; border: 1px solid var(--theme-surface-border); border-radius: var(--radius-sm); background: var(--theme-surface-bg); color: var(--theme-surface-fg); font-size: 0.85rem;"
                    placeholder="Value..."
                />
            </div>
        }.into_any(),

        CanonBlockType::FormActions => view! {
            <div data-canon-block="form-actions" style="display: flex; gap: var(--space-sm); justify-content: flex-end;">
                <button style="padding: 0.4rem 1rem; border-radius: var(--radius-sm); border: 1px solid var(--theme-surface-border); background: transparent; cursor: pointer; font-size: 0.8rem;">"Cancel"</button>
                <button style="padding: 0.4rem 1rem; border-radius: var(--radius-sm); border: none; background: var(--theme-action-primary-bg); color: var(--theme-action-primary-fg); cursor: pointer; font-size: 0.8rem;">"Submit"</button>
            </div>
        }.into_any(),

        CanonBlockType::Toolbar => view! {
            <div data-canon-block="toolbar" style="display: flex; gap: var(--space-sm); align-items: center; padding: 0.5rem; background: var(--theme-surface-bg); border: 1px solid var(--theme-surface-border); border-radius: var(--radius-sm);">
                "Toolbar"
                {children_views}
            </div>
        }.into_any(),

        CanonBlockType::Breadcrumb => view! {
            <div data-canon-block="breadcrumb" style="display: flex; gap: 0.25rem; align-items: center; font-size: 0.8rem; color: var(--theme-surface-fg-muted);">
                "Home" <span>"›"</span> "Page" <span>"›"</span> "Current"
            </div>
        }.into_any(),

        CanonBlockType::StatCard => view! {
            <StatCard label="Metric".to_string() value="1,234".to_string() change="↑ 12%".to_string() />
        }.into_any(),

        CanonBlockType::Callout => view! {
            <CalloutBlock variant=CalloutType::Info title="Callout".to_string()>"Callout message"</CalloutBlock>
        }.into_any(),

        CanonBlockType::EmptyState => view! {
            <EmptyState title="No data yet".to_string() description="Add content here".to_string() />
        }.into_any(),

        CanonBlockType::Skeleton => view! {
            <div data-canon-block="skeleton" style="display: flex; flex-direction: column; gap: 0.5rem;">
                <div style="height: 1rem; background: var(--theme-surface-muted); border-radius: var(--radius-sm); width: 60%; animation: pulse 1.5s ease-in-out infinite;" />
                <div style="height: 1rem; background: var(--theme-surface-muted); border-radius: var(--radius-sm); width: 80%;" />
                <div style="height: 1rem; background: var(--theme-surface-muted); border-radius: var(--radius-sm); width: 40%;" />
            </div>
        }.into_any(),

        CanonBlockType::DataTable | CanonBlockType::Table => view! {
            <div data-canon-block="table" style="border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md); overflow: hidden;">
                <div style="display: grid; grid-template-columns: repeat(3, 1fr); background: var(--theme-surface-muted); padding: 0.5rem 0.75rem; font-size: 0.75rem; font-weight: 600; color: var(--theme-surface-fg-muted);">
                    <span>"Name"</span><span>"Status"</span><span>"Date"</span>
                </div>
                <div style="display: grid; grid-template-columns: repeat(3, 1fr); padding: 0.5rem 0.75rem; font-size: 0.8rem; border-top: 1px solid var(--theme-surface-border);">
                    <span>"Item A"</span><span>"Active"</span><span>"2025-01"</span>
                </div>
            </div>
        }.into_any(),

        CanonBlockType::List => view! {
            <ul data-canon-block="list" style="list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 0.25rem;">
                <li style="padding: 0.4rem 0.5rem; border-bottom: 1px solid var(--theme-surface-border); font-size: 0.85rem;">"Item 1"</li>
                <li style="padding: 0.4rem 0.5rem; border-bottom: 1px solid var(--theme-surface-border); font-size: 0.85rem;">"Item 2"</li>
                <li style="padding: 0.4rem 0.5rem; font-size: 0.85rem;">"Item 3"</li>
            </ul>
        }.into_any(),

        CanonBlockType::CodeBlock => view! {
            <pre data-canon-block="code-block" style="padding: 1rem; background: var(--theme-surface-muted); border-radius: var(--radius-md); font-size: 0.8rem; font-family: monospace; overflow-x: auto; margin: 0;">
                <code>"fn main() {
    println!("Hello, CanonRS!");
}"</code>
            </pre>
        }.into_any(),

        CanonBlockType::PageHeader => view! {
            <PageHeader title="Page Title".to_string() subtitle="Description".to_string() />
        }.into_any(),

        CanonBlockType::ButtonGroup => view! {
            <div data-canon-block="button-group" style="display: flex; gap: 0.5rem;">
                <button style="padding: 0.4rem 0.75rem; border-radius: var(--radius-sm); border: 1px solid var(--theme-surface-border); background: var(--theme-surface-bg); cursor: pointer; font-size: 0.8rem;">"Action 1"</button>
                <button style="padding: 0.4rem 0.75rem; border-radius: var(--radius-sm); border: 1px solid var(--theme-surface-border); background: var(--theme-surface-bg); cursor: pointer; font-size: 0.8rem;">"Action 2"</button>
            </div>
        }.into_any(),

        CanonBlockType::Dialog => view! {
            <div data-canon-block="dialog" style="border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md); padding: 1.5rem; background: var(--theme-surface-bg); box-shadow: var(--shadow-lg);">
                <div style="font-size: 1rem; font-weight: 600; margin-bottom: 1rem; color: var(--theme-surface-fg);">"Dialog Title"</div>
                {children_views}
            </div>
        }.into_any(),

        CanonBlockType::Drawer => view! {
            <div data-canon-block="drawer" style="border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md); padding: 1rem; background: var(--theme-surface-bg);">
                <div style="font-size: 0.9rem; font-weight: 600; margin-bottom: 0.75rem; color: var(--theme-surface-fg);">"Drawer"</div>
                {children_views}
            </div>
        }.into_any(),

        CanonBlockType::Popover => view! {
            <div data-canon-block="popover" style="border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md); padding: 0.75rem; background: var(--theme-surface-bg); box-shadow: var(--shadow-md); display: inline-block;">
                {children_views}
            </div>
        }.into_any(),

        CanonBlockType::CommandPanel => view! {
            <div data-canon-block="command-panel" style="border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md); overflow: hidden; background: var(--theme-surface-bg);">
                <div style="padding: 0.5rem 0.75rem; border-bottom: 1px solid var(--theme-surface-border); font-size: 0.8rem; color: var(--theme-surface-fg-muted);">"⌘ Search commands..."</div>
                <div style="padding: 0.5rem; font-size: 0.8rem; color: var(--theme-surface-fg);">"No results"</div>
            </div>
        }.into_any(),

        CanonBlockType::Layout => view! {
            <div data-canon-block="layout" style="display: flex; flex-direction: column; gap: var(--space-md);">
                {children_views}
            </div>
        }.into_any(),
    }
}

/// Renderiza lista de raízes (slots do layout)
pub fn render_document(nodes: &[CanonNode]) -> impl IntoView {
    let views: Vec<AnyView> = nodes.iter().map(|n| render_node(n)).collect();
    view! {
        <div data-canon-document="">
            {views}
        </div>
    }
}
