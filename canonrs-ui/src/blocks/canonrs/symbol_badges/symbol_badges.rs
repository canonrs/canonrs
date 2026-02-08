use leptos::prelude::*;

#[derive(Clone, Debug)]
pub enum SymbolVariant {
    SSR,
    Stable,
    Experimental,
}

#[derive(Clone)]
pub struct Symbol {
    pub variant: SymbolVariant,
    pub label: String,
}

#[component]
pub fn SymbolBadges(symbols: Vec<Symbol>) -> impl IntoView {
    view! {
        <div class="symbol-badges">
            {symbols.into_iter().map(|symbol| {
                let class = match symbol.variant {
                    SymbolVariant::SSR => "symbol-badge symbol-ssr",
                    SymbolVariant::Stable => "symbol-badge symbol-stable",
                    SymbolVariant::Experimental => "symbol-badge symbol-experimental",
                };
                view! {
                    <span class={class}>{symbol.label}</span>
                }
            }).collect_view()}
        </div>
    }
}
