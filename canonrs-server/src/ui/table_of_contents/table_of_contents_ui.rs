#![allow(unreachable_pub, dead_code)]

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
                let is_child = item.level > 1;
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
    // Constrói a árvore achatando tudo em um Vec e usando índices
    // stack guarda (level, index_no_flat)
    let flat: Vec<TocNode> = items.into_iter()
        .map(|item| TocNode { item, children: Vec::new() })
        .collect();

    // índices dos pais para cada nó
    let n = flat.len();
    let mut parent: Vec<Option<usize>> = vec![None; n];
    let mut stack: Vec<(u8, usize)> = Vec::new(); // (level, flat_idx)

    for i in 0..n {
        let level = flat[i].item.level;
        while stack.last().map(|(l, _)| *l >= level).unwrap_or(false) {
            stack.pop();
        }
        if let Some(&(_, p)) = stack.last() {
            parent[i] = Some(p);
        }
        stack.push((level, i));
    }

    // monta a árvore de trás para frente para evitar borrow conflict
    // extrai todos os nós em ordem reversa e insere nos pais
    let mut nodes: Vec<Option<TocNode>> = flat.into_iter().map(Some).collect();

    for i in (0..n).rev() {
        if let Some(p) = parent[i] {
            let child = nodes[i].take().unwrap();
            nodes[p].as_mut().unwrap().children.insert(0, child);
        }
    }

    nodes.into_iter().enumerate()
        .filter(|(i, _)| parent[*i].is_none())
        .filter_map(|(_, n)| n)
        .collect()
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
                <div class="toc-item-row">
                    {has_children.then(|| view! {
                        <TocExpandButtonPrimitive>
                            <span data-rs-toc-expand-icon="" />
                        </TocExpandButtonPrimitive>
                    })}
                    <TocLinkPrimitive href=format!("#{}", item.id)>
                        {item.text}
                    </TocLinkPrimitive>
                </div>
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
