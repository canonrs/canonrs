//! TableOfContents UI - Enterprise component using primitives
//! 3 modes: simple | expand | nested
//! SSR-safe, behavior-driven scroll-spy

use leptos::prelude::*;
use canonrs_shared::TocItem;
use canonrs_shared::shared::navigation_context::{NavigationState, HeadingHierarchy};
use crate::primitives::table_of_contents::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TocMode {
    #[default]
    Simple,
    Expand,
    Nested,
}

impl TocMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            TocMode::Simple => "simple",
            TocMode::Expand => "expand",
            TocMode::Nested => "nested",
        }
    }
}

// ── Main Component ────────────────────────────────────────────────────────────

#[component]
pub fn TableOfContents(
    items: Vec<TocItem>,
    #[prop(default = TocMode::Simple)] mode: TocMode,
    #[prop(into, default = String::new())] id: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = "On this page".to_string())] title: String,
) -> impl IntoView {
    // Populate NavigationContext from TOC items
    if let Some(nav_state) = use_context::<RwSignal<NavigationState>>() {
        let hierarchy = HeadingHierarchy::from_toc_items(&items);
        nav_state.update(|s| {
            s.heading_hierarchy = hierarchy;
        });
    }

    view! {
        <TocPrimitive
            id=id
            class=class
            data_toc_mode=mode.as_str().to_string()
        >
            <TocTitlePrimitive>
                {title}
            </TocTitlePrimitive>
            {match mode {
                TocMode::Simple  => render_simple(items).into_any(),
                TocMode::Expand  => render_expand(items).into_any(),
                TocMode::Nested  => render_nested(items).into_any(),
            }}
        </TocPrimitive>
    }
}

// ── Mode 1: Simple ────────────────────────────────────────────────────────────

fn render_simple(items: Vec<TocItem>) -> impl IntoView {
    view! {
        <TocListPrimitive>
            {items.into_iter().map(|item| {
                view! {
                    <TocItemPrimitive
                        data_level=item.level.to_string()
                        data_target=item.id.clone()
                        data_state="idle".to_string()
                        data_child="false".to_string()
                        data_has_children="false".to_string()
                    >
                        <TocLinkPrimitive href=format!("#{}", item.id)>
                            {item.text}
                        </TocLinkPrimitive>
                    </TocItemPrimitive>
                }
            }).collect::<Vec<_>>()}
        </TocListPrimitive>
    }
}

// ── Mode 2: Expand ────────────────────────────────────────────────────────────

fn render_expand(items: Vec<TocItem>) -> impl IntoView {
    view! {
        <TocListPrimitive>
            {items.into_iter().map(|item| {
                let is_child = item.level > 2;
                view! {
                    <TocItemPrimitive
                        data_level=item.level.to_string()
                        data_target=item.id.clone()
                        data_state="idle".to_string()
                        data_child=if is_child { "true".to_string() } else { "false".to_string() }
                        data_has_children="false".to_string()
                    >
                        <TocLinkPrimitive href=format!("#{}", item.id)>
                            {item.text}
                        </TocLinkPrimitive>
                    </TocItemPrimitive>
                }
            }).collect::<Vec<_>>()}
        </TocListPrimitive>
    }
}

// ── Mode 3: Nested ────────────────────────────────────────────────────────────

fn render_nested(items: Vec<TocItem>) -> impl IntoView {
    let tree = build_tree(items);
    view! {
        <TocListPrimitive>
            {render_tree_nodes(tree)}
        </TocListPrimitive>
    }
}

#[derive(Clone)]
struct TocNode {
    item: TocItem,
    children: Vec<TocNode>,
}

fn build_tree(items: Vec<TocItem>) -> Vec<TocNode> {
    let mut roots: Vec<TocNode> = Vec::new();
    let mut stack: Vec<(u8, usize)> = Vec::new();

    for item in items {
        let node = TocNode { item: item.clone(), children: Vec::new() };
        let level = item.level;

        while stack.last().map(|(l, _)| *l >= level).unwrap_or(false) {
            stack.pop();
        }

        if stack.is_empty() {
            roots.push(node);
            stack.push((level, roots.len() - 1));
        } else {
            let path: Vec<usize> = stack.iter().map(|(_, i)| *i).collect();
            let parent = get_node_mut(&mut roots, &path);
            if let Some(p) = parent {
                p.children.push(node);
                let child_idx = p.children.len() - 1;
                stack.push((level, child_idx));
            }
        }
    }

    roots
}

fn get_node_mut<'a>(nodes: &'a mut Vec<TocNode>, path: &[usize]) -> Option<&'a mut TocNode> {
    if path.is_empty() { return None; }
    let mut current = nodes.get_mut(path[0])?;
    for &idx in &path[1..] {
        current = current.children.get_mut(idx)?;
    }
    Some(current)
}

fn render_tree_nodes(nodes: Vec<TocNode>) -> Vec<AnyView> {
    nodes.into_iter().map(|node| {
        let has_children = !node.children.is_empty();
        let item = node.item;
        let children = node.children;

        view! {
            <TocItemPrimitive
                data_level=item.level.to_string()
                data_target=item.id.clone()
                data_state="idle".to_string()
                data_child="false".to_string()
                data_has_children=if has_children { "true".to_string() } else { "false".to_string() }
            >
                {has_children.then(|| view! {
                    <TocExpandButtonPrimitive aria_expanded="false".to_string() />
                })}
                <TocLinkPrimitive href=format!("#{}", item.id)>
                    {item.text}
                </TocLinkPrimitive>
                {has_children.then(|| view! {
                    <TocSubtreePrimitive data_state="closed".to_string()>
                        {render_tree_nodes(children)}
                    </TocSubtreePrimitive>
                })}
            </TocItemPrimitive>
        }.into_any()
    }).collect()
}
