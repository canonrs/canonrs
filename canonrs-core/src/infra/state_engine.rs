//! @canon-level: strict
//! @canon-owner: primitives-team
//! StateEngine — centraliza aplicação de state em componentes
//! Garante que data-rs-state, aria-* e hidden derivam do mesmo enum

use crate::meta::{
    VisibilityState, ActivityState, SelectionState,
    ToggleState, StateKind, DisabledState,
};

/// Visibilidade — root + content (Dialog, Drawer, Sheet, Accordion)
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

/// Trigger — aria-expanded pertence ao trigger, não ao root
pub fn trigger_attrs(state: VisibilityState) -> TriggerAttrs {
    let open = state == VisibilityState::Open;
    TriggerAttrs {
        data_rs_state: state.as_str(),
        aria_expanded: if open { "true" } else { "false" },
    }
}

pub struct TriggerAttrs {
    pub data_rs_state: &'static str,
    pub aria_expanded: &'static str,
}

/// Atividade — Tabs content
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

/// Seleção — Select items, Listbox
pub fn selection_attrs(state: SelectionState) -> SelectionAttrs {
    let selected = state == SelectionState::Selected;
    SelectionAttrs {
        data_rs_state: if selected { Some("selected") } else { None },
        aria_selected: if selected { Some("true") } else { None },
    }
}

pub struct SelectionAttrs {
    pub data_rs_state: Option<&'static str>,
    pub aria_selected: Option<&'static str>,
}

/// Toggle — aria-pressed + checked
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

/// Disabled — separado de visibility/selection
pub fn disabled_attrs(state: DisabledState) -> DisabledAttrs {
    let disabled = state.as_bool();
    DisabledAttrs {
        data_rs_disabled: if disabled { Some("") } else { None },
        aria_disabled: state.aria(),
        disabled,
    }
}

pub struct DisabledAttrs {
    pub data_rs_disabled: Option<&'static str>,
    pub aria_disabled: Option<&'static str>,
    pub disabled: bool,
}

/// resolve_state retorna ResolvedState (não mais string crua)
pub fn resolve_state(kind: StateKind) -> ResolvedState {
    ResolvedState {
        data_rs_state: kind.as_str(),
    }
}

pub struct ResolvedState {
    pub data_rs_state: &'static str,
}

/// Form validation — data-rs-state + aria-busy + aria-invalid
pub fn validation_attrs(state: crate::primitives::form::FormValidationState) -> ValidationAttrs {
    use crate::primitives::form::FormValidationState;
    ValidationAttrs {
        data_rs_state: state.as_str(),
        aria_busy: if state == FormValidationState::Submitting { Some("true") } else { None },
        aria_invalid: if state == FormValidationState::Error { Some("true") } else { None },
    }
}

pub struct ValidationAttrs {
    pub data_rs_state: &'static str,
    pub aria_busy: Option<&'static str>,
    pub aria_invalid: Option<&'static str>,
}

pub fn loading_attrs(state: crate::meta::LoadingState) -> LoadingAttrs {
    let is_loading = state == crate::meta::LoadingState::Loading;
    LoadingAttrs {
        data_rs_state:  state.as_str(),
        aria_busy:      if is_loading { Some("true") } else { None },
        hidden:         false,
    }
}

pub struct LoadingAttrs {
    pub data_rs_state: &'static str,
    pub aria_busy:     Option<&'static str>,
    pub hidden:        bool,
}

/// Value Object — estado projetado para DOM
/// Calculado fora do primitive, passado como prop única
#[derive(Clone, Default)]
pub struct UiStateAttrs {
    pub data_rs_state:    Option<&'static str>,
    pub data_rs_disabled: Option<&'static str>,
    pub data_rs_loading:  Option<&'static str>,
    pub disabled:         bool,
    pub aria_disabled:    Option<&'static str>,
    pub aria_busy:        Option<&'static str>,
    pub aria_pressed:     Option<&'static str>,
}

impl UiStateAttrs {
    pub fn from_button(
        disabled: crate::meta::DisabledState,
        loading:  crate::meta::LoadingState,
        pressed:  Option<crate::meta::ToggleState>,
    ) -> Self {
        let d  = disabled_attrs(disabled);
        let la = loading_attrs(loading);
        let ta = pressed.map(toggle_attrs);
        let state = if d.disabled {
            Some("disabled")
        } else if la.aria_busy.is_some() {
            Some("loading")
        } else {
            ta.as_ref().map(|t| t.data_rs_state)
        };
        Self {
            data_rs_state:    state,
            data_rs_disabled: d.data_rs_disabled,
            data_rs_loading:  Some(la.data_rs_state),
            disabled:         d.disabled,
            aria_disabled:    d.aria_disabled,
            aria_busy:        la.aria_busy,
            aria_pressed:     ta.as_ref().map(|t| t.aria_pressed),
        }
    }
}
