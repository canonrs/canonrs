//! State — mutação de data-rs-state no DOM
//!
//! `State` é o enum canônico de tokens. Use-o sempre que possível.
//! `add/remove/has` aceitam &str para casos de tokens dinâmicos ou legado.

use wasm_bindgen::JsValue;
use web_sys::Element;

/// Tokens canônicos de estado do CanonRS.
/// Representa todos os valores válidos de `data-rs-state`.
/// Tokens dinâmicos ou específicos de componente usam state::add(&str) diretamente.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    // Visibility
    Open,
    Closed,
    Hidden,
    Visible,
    // Activity
    Active,
    Inactive,
    // Selection
    Selected,
    Unselected,
    // Expansion
    Expanded,
    Collapsed,
    // Interaction
    Focused,
    Hover,
    Disabled,
    // Form / Toggle
    Checked,
    Unchecked,
    On,
    Off,
    // Async
    Loading,
    Idle,
    Error,
    Submitting,
    // Feedback
    Copied,
    Paused,
    // Transition
    Entering,
    Exiting,
}

impl State {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open        => "open",
            Self::Closed      => "closed",
            Self::Hidden      => "hidden",
            Self::Visible     => "visible",
            Self::Active      => "active",
            Self::Inactive    => "inactive",
            Self::Selected    => "selected",
            Self::Unselected  => "unselected",
            Self::Expanded    => "expanded",
            Self::Collapsed   => "collapsed",
            Self::Focused     => "focused",
            Self::Hover       => "hover",
            Self::Disabled    => "disabled",
            Self::Checked     => "checked",
            Self::Unchecked   => "unchecked",
            Self::On          => "on",
            Self::Off         => "off",
            Self::Loading     => "loading",
            Self::Idle        => "idle",
            Self::Error       => "error",
            Self::Submitting  => "submitting",
            Self::Copied      => "copied",
            Self::Paused      => "paused",
            Self::Entering    => "entering",
            Self::Exiting     => "exiting",
        }
    }
}

/// Adiciona um token tipado ao data-rs-state
pub fn set(el: &Element, token: State) { add(el, token.as_str()); }

/// Remove um token tipado do data-rs-state
pub fn unset(el: &Element, token: State) { remove(el, token.as_str()); }

/// Verifica se um token tipado está presente
pub fn is(el: &Element, token: State) -> bool { has(el, token.as_str()) }

pub fn is_valid(el: &Element) -> bool {
    let v: &JsValue = el.as_ref();
    !v.is_null() && !v.is_undefined() && el.is_connected()
}

pub fn add(el: &Element, token: &str) {
    if !is_valid(el) { return; }
    let cur = el.get_attribute("data-rs-state").unwrap_or_default();
    if cur.split_whitespace().any(|t| t == token) { return; }
    let next = if cur.is_empty() { token.to_string() } else { format!("{} {}", cur, token) };
    let _ = el.set_attribute("data-rs-state", &next);
}

pub fn remove(el: &Element, token: &str) {
    if !is_valid(el) { return; }
    let cur = el.get_attribute("data-rs-state").unwrap_or_default();
    if !cur.split_whitespace().any(|t| t == token) { return; }
    let next = cur.split_whitespace().filter(|t| *t != token).collect::<Vec<_>>().join(" ");
    let _ = el.set_attribute("data-rs-state", &next);
}

pub fn has(el: &Element, token: &str) -> bool {
    if !is_valid(el) { return false; }
    el.get_attribute("data-rs-state")
        .unwrap_or_default()
        .split_whitespace()
        .any(|t| t == token)
}

pub fn open(el: &Element) {
    remove(el, "closed");
    add(el, "open");
}

pub fn close(el: &Element) {
    remove(el, "open");
    add(el, "closed");
}

pub fn toggle(el: &Element) {
    if is_open(el) { close(el); } else { open(el); }
}

pub fn is_open(el: &Element) -> bool {
    has(el, "open")
}

pub fn expand(el: &Element) {
    remove(el, "collapsed");
    add(el, "expanded");
}

pub fn collapse(el: &Element) {
    remove(el, "expanded");
    add(el, "collapsed");
}

pub fn is_expanded(el: &Element) -> bool {
    has(el, "expanded")
}

pub fn set_scroll_lock(locked: bool) {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        if let Some(body) = doc.body() {
            if locked { let _ = body.set_attribute("data-rs-scroll-lock", "true"); }
            else      { let _ = body.remove_attribute("data-rs-scroll-lock"); }
        }
    }
}

// Aliases para compatibilidade
pub fn add_state(el: &Element, token: &str) { add(el, token); }
pub fn remove_state(el: &Element, token: &str) { remove(el, token); }

#[cfg(test)]
mod tests {
    // Nota: testes de DOM rodam via wasm-pack test --headless
    // Testes de lógica pura podem rodar com cargo test

    #[test]
    fn state_enum_as_str() {
        use super::State;
        assert_eq!(State::Open.as_str(),       "open");
        assert_eq!(State::Closed.as_str(),     "closed");
        assert_eq!(State::Active.as_str(),     "active");
        assert_eq!(State::Inactive.as_str(),   "inactive");
        assert_eq!(State::Expanded.as_str(),   "expanded");
        assert_eq!(State::Collapsed.as_str(),  "collapsed");
        assert_eq!(State::Focused.as_str(),    "focused");
        assert_eq!(State::Hover.as_str(),      "hover");
        assert_eq!(State::Disabled.as_str(),   "disabled");
        assert_eq!(State::Selected.as_str(),   "selected");
        assert_eq!(State::Unselected.as_str(), "unselected");
        assert_eq!(State::Hidden.as_str(),     "hidden");
        assert_eq!(State::Visible.as_str(),    "visible");
        assert_eq!(State::Checked.as_str(),    "checked");
        assert_eq!(State::Unchecked.as_str(),  "unchecked");
        assert_eq!(State::On.as_str(),         "on");
        assert_eq!(State::Off.as_str(),        "off");
        assert_eq!(State::Loading.as_str(),    "loading");
        assert_eq!(State::Idle.as_str(),       "idle");
        assert_eq!(State::Error.as_str(),      "error");
        assert_eq!(State::Submitting.as_str(), "submitting");
        assert_eq!(State::Copied.as_str(),     "copied");
        assert_eq!(State::Paused.as_str(),     "paused");
        assert_eq!(State::Entering.as_str(),   "entering");
        assert_eq!(State::Exiting.as_str(),    "exiting");
    }

    #[test]
    fn state_enum_equality() {
        use super::State;
        assert_eq!(State::Open, State::Open);
        assert_ne!(State::Open, State::Closed);
    }
}
