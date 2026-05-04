//! @canon-level: strict
//! @canon-owner: primitives-team
//! StateEngine — centraliza aplicação de state em componentes
//! Cada estado tem seu próprio atributo DOM — sem colisão de namespace.

use crate::meta::{
    VisibilityState, ActivityState, SelectionState,
    ToggleState, StateKind, DisabledState, LoadingState,
    ToDataAttr,
};

// ── Visibility ───────────────────────────────────────────────────────────────

pub struct VisibilityAttrs {
    pub data_rs_visibility: &'static str,
    pub aria_hidden:        &'static str,
    pub aria_expanded:      &'static str,
    pub hidden:             bool,
}

pub fn visibility_attrs(state: VisibilityState) -> VisibilityAttrs {
    let open = state == VisibilityState::Open;
    let (_, value) = state.to_data_attr();
    VisibilityAttrs {
        data_rs_visibility: value,
        aria_hidden:        if open { "false" } else { "true" },
        aria_expanded:      if open { "true" } else { "false" },
        hidden:             !open,
    }
}

// ── Trigger ──────────────────────────────────────────────────────────────────

pub struct TriggerAttrs {
    pub data_rs_visibility: &'static str,
    pub aria_expanded:      &'static str,
}

pub fn trigger_attrs(state: VisibilityState) -> TriggerAttrs {
    let open = state == VisibilityState::Open;
    let (_, value) = state.to_data_attr();
    TriggerAttrs {
        data_rs_visibility: value,
        aria_expanded:      if open { "true" } else { "false" },
    }
}

// ── Activity ─────────────────────────────────────────────────────────────────

pub struct ActivityAttrs {
    pub data_rs_activity: &'static str,
    pub aria_selected:    &'static str,
    pub hidden:           bool,
}

pub fn activity_attrs(state: ActivityState) -> ActivityAttrs {
    let active = state == ActivityState::Active;
    let (_, value) = state.to_data_attr();
    ActivityAttrs {
        data_rs_activity: value,
        aria_selected:    if active { "true" } else { "false" },
        hidden:           !active,
    }
}

// ── Selection ────────────────────────────────────────────────────────────────

pub struct SelectionAttrs {
    pub data_rs_selection: Option<&'static str>,
    pub aria_selected:     Option<&'static str>,
}

pub fn selection_attrs(state: SelectionState) -> SelectionAttrs {
    let selected = state == SelectionState::Selected;
    let (_, value) = state.to_data_attr();
    SelectionAttrs {
        data_rs_selection: if selected { Some(value) } else { None },
        aria_selected:     if selected { Some("true") } else { None },
    }
}

// ── Toggle ───────────────────────────────────────────────────────────────────

pub struct ToggleAttrs {
    pub data_rs_toggle: &'static str,
    pub aria_pressed:   &'static str,
    pub checked:        bool,
}

pub fn toggle_attrs(state: ToggleState) -> ToggleAttrs {
    let on = state == ToggleState::On;
    let (_, value) = state.to_data_attr();
    ToggleAttrs {
        data_rs_toggle: value,
        aria_pressed:   if on { "true" } else { "false" },
        checked:        on,
    }
}

// ── Disabled ─────────────────────────────────────────────────────────────────

pub struct DisabledAttrs {
    pub data_rs_disabled: Option<&'static str>,
    pub aria_disabled:    Option<&'static str>,
    pub disabled:         bool,
}

pub fn disabled_attrs(state: DisabledState) -> DisabledAttrs {
    let disabled = state.as_bool();
    let (_, value) = state.to_data_attr();
    DisabledAttrs {
        data_rs_disabled: if disabled { Some(value) } else { None },
        aria_disabled:    state.aria(),
        disabled,
    }
}

// ── Loading ──────────────────────────────────────────────────────────────────

pub struct LoadingAttrs {
    pub data_rs_loading: &'static str,
    pub aria_busy:       Option<&'static str>,
}

pub fn loading_attrs(state: LoadingState) -> LoadingAttrs {
    let is_loading = state == LoadingState::Loading;
    let (_, value) = state.to_data_attr();
    LoadingAttrs {
        data_rs_loading: value,
        aria_busy:       if is_loading { Some("true") } else { None },
    }
}

// ── resolve_state ────────────────────────────────────────────────────────────

pub struct ResolvedState {
    pub attr:  &'static str,
    pub value: &'static str,
}

pub fn resolve_state(kind: StateKind) -> ResolvedState {
    let (attr, value) = kind.to_data_attr();
    ResolvedState { attr, value }
}

// ── Validation ───────────────────────────────────────────────────────────────

pub struct ValidationAttrs {
    pub data_rs_activity: &'static str,
    pub aria_busy:        Option<&'static str>,
    pub aria_invalid:     Option<&'static str>,
}

pub fn validation_attrs(state: crate::primitives::form::FormValidationState) -> ValidationAttrs {
    use crate::primitives::form::FormValidationState;
    ValidationAttrs {
        data_rs_activity: state.as_str(),
        aria_busy:        if state == FormValidationState::Submitting { Some("true") } else { None },
        aria_invalid:     if state == FormValidationState::Error { Some("true") } else { None },
    }
}

// ── UiStateAttrs (substituído por attrs separados) ───────────────────────────
// Mantido temporariamente para compatibilidade — remover após migração completa

#[derive(Clone, Default)]
pub struct UiStateAttrs {
    pub data_rs_disabled: Option<&'static str>,
    pub data_rs_loading:  &'static str,
    pub data_rs_toggle:   Option<&'static str>,
    pub disabled:         bool,
    pub aria_disabled:    Option<&'static str>,
    pub aria_busy:        Option<&'static str>,
    pub aria_pressed:     Option<&'static str>,
}

impl UiStateAttrs {
    pub fn from_button(
        disabled: DisabledState,
        loading:  LoadingState,
        pressed:  Option<ToggleState>,
    ) -> Self {
        let d  = disabled_attrs(disabled);
        let la = loading_attrs(loading);
        let ta = pressed.map(toggle_attrs);
        Self {
            data_rs_disabled: d.data_rs_disabled,
            data_rs_loading:  la.data_rs_loading,
            data_rs_toggle:   ta.as_ref().map(|t| t.data_rs_toggle),
            disabled:         d.disabled,
            aria_disabled:    d.aria_disabled,
            aria_busy:        la.aria_busy,
            aria_pressed:     ta.as_ref().map(|t| t.aria_pressed),
        }
    }
}
