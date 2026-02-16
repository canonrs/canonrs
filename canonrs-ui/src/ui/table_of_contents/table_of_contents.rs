//! TableOfContents - Enterprise standalone component
//! 3 modes: simple | expand | nested
//! SSR-safe, behavior-driven scroll-spy

use leptos::prelude::*;
use canonrs_shared::TocItem;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TocMode {
    #[default]
    Simple,    // flat list, scroll-spy only
    Expand,    // flat list + auto-expand sub-levels on parent active
    Nested,    // hierarchical tree, collapsible sub-levels
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
    view! {
        <nav
            data-toc=""
            data-toc-mode=mode.as_str()
            id=id
            class=class
        >
            <p data-toc-title="">{title}</p>
            {match mode {
                TocMode::Simple  => render_simple(items).into_any(),
                TocMode::Expand  => render_expand(items).into_any(),
                TocMode::Nested  => render_nested(items).into_any(),
            }}
        </nav>
    }
}

// ── Mode 1: Simple ────────────────────────────────────────────────────────────

fn render_simple(items: Vec<TocItem>) -> impl IntoView {
    view! {
        <ul data-toc-list="">
            {items.into_iter().map(|item| {
                view! {
                    <li
                        data-toc-item=""
                        data-level=item.level.to_string()
                        data-state="idle"
                        data-target=item.id.clone()
                    >
                        <a data-toc-link="" href=format!("#{}", item.id)>
                            {item.text}
                        </a>
                    </li>
                }
            }).collect::<Vec<_>>()}
        </ul>
    }
}

// ── Mode 2: Expand ────────────────────────────────────────────────────────────

fn render_expand(items: Vec<TocItem>) -> impl IntoView {
    view! {
        <ul data-toc-list="">
            {items.into_iter().map(|item| {
                let is_child = item.level > 2;
                view! {
                    <li
                        data-toc-item=""
                        data-level=item.level.to_string()
                        data-state="idle"
                        data-target=item.id.clone()
                        data-child=if is_child { "true" } else { "false" }
                    >
                        <a data-toc-link="" href=format!("#{}", item.id)>
                            {item.text}
                        </a>
                    </li>
                }
            }).collect::<Vec<_>>()}
        </ul>
    }
}

// ── Mode 3: Nested ────────────────────────────────────────────────────────────

fn render_nested(items: Vec<TocItem>) -> impl IntoView {
    let tree = build_tree(items);
    view! {
        <ul data-toc-list="" data-toc-tree="">
            {render_tree_nodes(tree)}
        </ul>
    }
}

#[derive(Clone)]
struct TocNode {
    item: TocItem,
    children: Vec<TocNode>,
}

fn build_tree(items: Vec<TocItem>) -> Vec<TocNode> {
    let mut roots: Vec<TocNode> = Vec::new();
    let mut stack: Vec<(u8, usize)> = Vec::new(); // (level, index in roots/children)

    for item in items {
        let node = TocNode { item: item.clone(), children: Vec::new() };
        let level = item.level;

        // Pop stack until we find a parent
        while stack.last().map(|(l, _)| *l >= level).unwrap_or(false) {
            stack.pop();
        }

        if stack.is_empty() {
            roots.push(node);
            stack.push((level, roots.len() - 1));
        } else {
            // Navigate to parent and push child
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
            <li
                data-toc-item=""
                data-level=item.level.to_string()
                data-state="idle"
                data-target=item.id.clone()
                attr:data-has-children={has_children.then(|| "true")}
            >
                {has_children.then(|| view! {
                    <button
                        type="button"
                        data-toc-expand-btn=""
                        aria-expanded="false"
                    >
                        "›"
                    </button>
                })}
                <a data-toc-link="" href=format!("#{}", item.id)>
                    {item.text}
                </a>
                {has_children.then(|| view! {
                    <ul data-toc-subtree="" data-state="closed">
                        {render_tree_nodes(children)}
                    </ul>
                })}
            </li>
        }.into_any()
    }).collect()
}
