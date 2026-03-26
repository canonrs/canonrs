//! style_mapper — binding BlockProps / StyleProps → CSS custom properties
//!
//! Pipeline:  Block props → StyleProps → CSS variables (--rs-*)
//! Priority:  Style Contract > Block Props > Default

use crate::style_contract::props::StyleProps;
use std::collections::HashMap;

/// Mapa de CSS variables canônicas Canon RS.
/// Zero hardcode no renderer — tudo passa por aqui.
#[derive(Debug, Default)]
pub struct CssVarMap {
    pub vars: HashMap<&'static str, String>,
}

impl CssVarMap {
    pub fn new() -> Self { Self::default() }

    pub fn insert(&mut self, key: &'static str, value: impl Into<String>) {
        self.vars.insert(key, value.into());
    }

    /// Serializa como string CSS inline: "--rs-padding: 1rem; --rs-gap: 0.5rem;"
    pub fn to_inline_style(&self) -> String {
        let mut parts: Vec<String> = self.vars
            .iter()
            .map(|(k, v)| format!("{}: {};", k, v))
            .collect();
        parts.sort();
        parts.join(" ")
    }
}

/// Converte props do nó (HashMap<String,String>) → CssVarMap.
/// Prioridade: style_contract > block_props > default (aplicado externamente).
pub fn map_props_to_css_vars(props: &HashMap<String, String>) -> CssVarMap {
    let mut map = CssVarMap::new();

    let bindings: &[(&str, &'static str)] = &[
        // Spacing
        ("padding",         "--rs-padding"),
        ("padding-x",       "--rs-padding-x"),
        ("padding-y",       "--rs-padding-y"),
        ("gap",             "--rs-gap"),
        ("margin",          "--rs-margin"),
        ("margin-x",        "--rs-margin-x"),
        ("margin-y",        "--rs-margin-y"),
        // Layout
        ("width",           "--rs-width"),
        ("height",          "--rs-height"),
        ("min-width",       "--rs-min-width"),
        ("max-width",       "--rs-max-width"),
        ("min-height",      "--rs-min-height"),
        ("max-height",      "--rs-max-height"),
        ("grid-columns",    "--rs-grid-columns"),
        ("grid-template-columns", "--rs-grid-template-columns"),
        ("flex-direction",  "--rs-flex-direction"),
        ("align-items",     "--rs-align-items"),
        ("justify-content", "--rs-justify-content"),
        // Typography
        ("font-size",       "--rs-font-size"),
        ("font-weight",     "--rs-font-weight"),
        ("line-height",     "--rs-line-height"),
        ("letter-spacing",  "--rs-letter-spacing"),
        ("text-align",      "--rs-text-align"),
        // Color
        ("color",           "--rs-color"),
        ("background",      "--rs-background"),
        ("border-color",    "--rs-border-color"),
        ("border-radius",   "--rs-border-radius"),
        ("border-width",    "--rs-border-width"),
        // Misc
        ("opacity",         "--rs-opacity"),
        ("z-index",         "--rs-z-index"),
        ("shadow",          "--rs-shadow"),
    ];

    for (prop_key, css_var) in bindings {
        if let Some(val) = props.get(*prop_key) {
            map.insert(css_var, val.clone());
        }
    }

    map
}

/// Converte StyleProps struct → CssVarMap (style contract path).
pub fn map_style_props_to_css_vars(style: &StyleProps) -> CssVarMap {
    // StyleProps já tem to_class() — aqui geramos vars brutas para o renderer
    let map = CssVarMap::new();

    if let Some(ref spacing) = style.spacing {
        // Spacing fields mapeados via Debug/Display se disponível
        // Expansão futura: spacing.padding → --rs-padding
        let _ = spacing; // placeholder até Spacing impl Display
    }
    if let Some(ref layout) = style.layout {
        let _ = layout;
    }
    if let Some(ref typo) = style.typography {
        let _ = typo;
    }
    if let Some(ref color) = style.color {
        let _ = color;
    }

    map
}

/// Resolve estilo final do nó aplicando prioridade:
/// Style Contract > Block Props > (defaults externos)
pub fn resolve_style(
    block_props: &HashMap<String, String>,
    style_contract: Option<&StyleProps>,
) -> CssVarMap {
    // Base: block props
    let mut map = map_props_to_css_vars(block_props);

    // Override: style contract (maior prioridade)
    if let Some(style) = style_contract {
        let contract_map = map_style_props_to_css_vars(style);
        for (k, v) in contract_map.vars {
            map.vars.insert(k, v);
        }
    }

    map
}
