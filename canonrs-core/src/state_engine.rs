//! @canon-level: strict
//! @canon-owner: primitives-team
//! StateEngine — centraliza aplicação de state em componentes
//! Garante que data-rs-state, aria-* e hidden derivam do mesmo enum

use crate::meta::{
    VisibilityState, ActivityState, SelectionState,
    ToggleState, StateKind,
};

/// Retorna (data-rs-state, aria-hidden, aria-expanded, hidden)
/// para componentes de visibilidade (Dialog, Drawer, Sheet, Accordion)
pub fn visibility_attrs(state: VisibilityState) -> VisibilityAttrs {
    let open = state == VisibilityState::Open;
    VisibilityAttrs {
        data_rs_state: state.as_str(),
        aria_hidden: if open { "false" } else { "true" },
        aria_expanded: if open { "true" } else { "false" },
        hidden: !open,
    }
}

pub struct VisibilityAttrs {
    pub data_rs_state: &'static str,
    pub aria_hidden: &'static str,
    pub aria_expanded: &'static str,
    pub hidden: bool,
}

/// Retorna (data-rs-state, aria-selected, hidden)
/// para componentes de atividade (Tabs)
pub fn activity_attrs(state: ActivityState) -> ActivityAttrs {
    let active = state == ActivityState::Active;
    ActivityAttrs {
        data_rs_state: state.as_str(),
        aria_selected: if active { "true" } else { "false" },
        hidden: !active,
    }
}

pub struct ActivityAttrs {
    pub data_rs_state: &'static str,
    pub aria_selected: &'static str,
    pub hidden: bool,
}

/// Retorna (data-rs-state, aria-selected)
/// para componentes de seleção (Select items)
pub fn selection_attrs(state: SelectionState) -> SelectionAttrs {
    let selected = state == SelectionState::Selected;
    SelectionAttrs {
        data_rs_state: state.as_str(),
        aria_selected: if selected { Some("true") } else { None },
    }
}

pub struct SelectionAttrs {
    pub data_rs_state: &'static str,
    pub aria_selected: Option<&'static str>,
}

/// Retorna (data-rs-state, aria-pressed, checked)
/// para Toggle
pub fn toggle_attrs(state: ToggleState) -> ToggleAttrs {
    let on = state == ToggleState::On;
    ToggleAttrs {
        data_rs_state: state.as_str(),
        aria_pressed: if on { "true" } else { "false" },
        checked: on,
    }
}

pub struct ToggleAttrs {
    pub data_rs_state: &'static str,
    pub aria_pressed: &'static str,
    pub checked: bool,
}

/// Resolve state a partir de StateKind unificado
pub fn resolve_state(kind: StateKind) -> &'static str {
    kind.as_str()
}
