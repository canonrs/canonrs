
use leptos::prelude::*;
use canonrs_core::TocItem;
use canonrs_core::primitives::table_of_contents::*;
#[cfg(feature = "ssr")]
use canonrs_core::VisibilityState;

#[component]
pub fn TableOfContents(
    items: Vec<TocItem>,
    #[prop(default = TocMode::Simple)] mode: TocMode,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = "On this page".to_string())] title: String,
) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        view! {
            <TocPrimitive class=class mode=mode>
                <TocTitlePrimitive>
                    {title}
                </TocTitlePrimitive>
                {match mode {
                    TocMode::Simple  => render_simple(items).into_any(),
                    TocMode::Expand  => render_expand(items).into_any(),
                    TocMode::Nested  => render_nested(items).into_any(),
                }}
            </TocPrimitive>
        }.into_any()
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = items;
        let _ = mode;
        let _ = class;
        let _ = title;
        view! { <nav data-rs-toc="" data-rs-component="TableOfContents"></nav> }.into_any()
    }
}

#[cfg(feature = "ssr")]
fn render_simple(items: Vec<TocItem>) -> impl IntoView {
    view! {
        <TocListPrimitive>
            {items.into_iter().map(|item| {
                view! {
                    <TocItemPrimitive
                        data_level=item.level.to_string()
                        data_target=item.id.clone()
                        state=TocItemState::Idle
                        is_child=false
                        has_children=false
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

#[cfg(feature = "ssr")]
fn render_expand(items: Vec<TocItem>) -> impl IntoView {
    view! {
        <TocListPrimitive>
            {items.into_iter().map(|item| {
                let is_child = item.level > 2;
                view! {
                    <TocItemPrimitive
                        data_level=item.level.to_string()
                        data_target=item.id.clone()
                        state=TocItemState::Idle
                        is_child=is_child
                        has_children=false
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

#[cfg(feature = "ssr")]
fn render_nested(items: Vec<TocItem>) -> impl IntoView {
    let tree = build_tree(items);
    view! {
        <TocListPrimitive>
            {render_tree_nodes(tree)}
        </TocListPrimitive>
    }
}

#[derive(Clone)]
#[cfg(feature = "ssr")]
struct TocNode {
    item: TocItem,
    children: Vec<TocNode>,
}

#[cfg(feature = "ssr")]
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

#[cfg(feature = "ssr")]
fn get_node_mut<'a>(nodes: &'a mut Vec<TocNode>, path: &[usize]) -> Option<&'a mut TocNode> {
    if path.is_empty() { return None; }
    let mut current = nodes.get_mut(path[0])?;
    for &idx in &path[1..] {
        current = current.children.get_mut(idx)?;
    }
    Some(current)
}

#[cfg(feature = "ssr")]
fn render_tree_nodes(nodes: Vec<TocNode>) -> Vec<AnyView> {
    nodes.into_iter().map(|node| {
        let has_children = !node.children.is_empty();
        let item = node.item;
        let children = node.children;
        view! {
            <TocItemPrimitive
                data_level=item.level.to_string()
                data_target=item.id.clone()
                state=TocItemState::Idle
                is_child=false
                has_children=has_children
            >
                {has_children.then(|| view! {
                    <TocExpandButtonPrimitive>
                        <span data-rs-toc-expand-icon="" />
                    </TocExpandButtonPrimitive>
                })}
                <TocLinkPrimitive href=format!("#{}", item.id)>
                    {item.text}
                </TocLinkPrimitive>
                {has_children.then(|| view! {
                    <TocSubtreePrimitive state=VisibilityState::Closed>
                        {render_tree_nodes(children)}
                    </TocSubtreePrimitive>
                })}
            </TocItemPrimitive>
        }.into_any()
    }).collect()
}

#[component]
pub fn TableOfContentsPreview() -> leptos::prelude::AnyView {
    view! { <div data-toc="">"Table of Contents"</div> }.into_any()
}
