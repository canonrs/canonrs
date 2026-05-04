//! @canon-level: strict
//! @canon-owner: primitives-team
//! ComponentMeta — metadata layer para AI/RAG/Decision Engine
use serde::{Serialize, Deserialize};

/// Typed capability enum — substitui &["open-close"] strings hardcoded
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    OpenClose,
    Selected,
    Active,
    Pressed,
    Disabled,
    Loading,
    Value,
    Multiple,
    Orientation,
    // Pure behaviors — sem props derivadas
    FocusTrap,
    KeyboardEsc,
    AriaModal,
    KeyboardArrows,
    Roving,
    Typeahead,
    VirtualScroll,
    DragDrop,
    Resize,
    Overflow,
    Animation,
}

impl Capability {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OpenClose      => "open-close",
            Self::Selected       => "selected",
            Self::Active         => "active",
            Self::Pressed        => "pressed",
            Self::Disabled       => "disabled",
            Self::Loading        => "loading",
            Self::Value          => "value",
            Self::Multiple       => "multiple",
            Self::Orientation    => "orientation",
            Self::FocusTrap      => "focus-trap",
            Self::KeyboardEsc    => "keyboard-esc",
            Self::AriaModal      => "aria-modal",
            Self::KeyboardArrows => "keyboard-arrows",
            Self::Roving         => "roving",
            Self::Typeahead      => "typeahead",
            Self::VirtualScroll  => "virtual-scroll",
            Self::DragDrop       => "drag-drop",
            Self::Resize         => "resize",
            Self::Overflow       => "overflow",
            Self::Animation      => "animation",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentMeta {
    pub id:             &'static str,
    pub name:           &'static str,
    pub family:         ComponentFamily,
    pub intent:         &'static str,
    pub capabilities:   &'static [Capability],
    pub composable:     bool,
    pub required_parts: &'static [&'static str],
    pub optional_parts: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComponentFamily {
    Overlay,
    Input,
    Feedback,
    Navigation,
    Layout,
    DataDisplay,
    Typography,
    Interactive,
    Utility,
}

impl ComponentFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Overlay     => "overlay",
            Self::Input       => "input",
            Self::Feedback    => "feedback",
            Self::Navigation  => "navigation",
            Self::Layout      => "layout",
            Self::DataDisplay => "data-display",
            Self::Typography  => "typography",
            Self::Interactive => "interactive",
            Self::Utility     => "utility",
        }
    }
}

pub trait HasMeta {
    fn meta() -> &'static ComponentMeta;
}

// ── TO DATA ATTR TRAIT ───────────────────────────────────────────────────────
// Cada estado tem seu próprio atributo DOM — sem colisão de namespace.

pub trait ToDataAttr {
    fn to_data_attr(&self) -> (&'static str, &'static str);
}

// ── STATE ENUMS POR DOMÍNIO ──────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum VisibilityState {
    #[default] Closed,
    Open,
}
impl VisibilityState {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Open => "open", Self::Closed => "closed" }
    }
}
impl From<bool> for VisibilityState {
    fn from(b: bool) -> Self { if b { Self::Open } else { Self::Closed } }
}
impl ToDataAttr for VisibilityState {
    fn to_data_attr(&self) -> (&'static str, &'static str) {
        ("data-rs-visibility", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ActivityState {
    #[default] Inactive,
    Active,
}
impl ActivityState {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Active => "active", Self::Inactive => "inactive" }
    }
}
impl From<bool> for ActivityState {
    fn from(b: bool) -> Self { if b { Self::Active } else { Self::Inactive } }
}
impl ToDataAttr for ActivityState {
    fn to_data_attr(&self) -> (&'static str, &'static str) {
        ("data-rs-activity", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SelectionState {
    #[default] Unselected,
    Selected,
}
impl SelectionState {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Selected => "selected", Self::Unselected => "unselected" }
    }
}
impl From<bool> for SelectionState {
    fn from(b: bool) -> Self { if b { Self::Selected } else { Self::Unselected } }
}
impl ToDataAttr for SelectionState {
    fn to_data_attr(&self) -> (&'static str, &'static str) {
        ("data-rs-selection", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ToggleState {
    #[default] Off,
    On,
}
impl ToggleState {
    pub fn as_str(&self) -> &'static str {
        match self { Self::On => "on", Self::Off => "off" }
    }
}
impl From<bool> for ToggleState {
    fn from(b: bool) -> Self { if b { Self::On } else { Self::Off } }
}
impl ToDataAttr for ToggleState {
    fn to_data_attr(&self) -> (&'static str, &'static str) {
        ("data-rs-toggle", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum NavigationState {
    #[default] Inactive,
    Current,
}
impl NavigationState {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Current => "current", Self::Inactive => "inactive" }
    }
}
impl From<bool> for NavigationState {
    fn from(b: bool) -> Self { if b { Self::Current } else { Self::Inactive } }
}
impl ToDataAttr for NavigationState {
    fn to_data_attr(&self) -> (&'static str, &'static str) {
        ("data-rs-navigation", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum DisabledState {
    #[default] Enabled,
    Disabled,
}
impl DisabledState {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Enabled => "enabled", Self::Disabled => "disabled" }
    }
    pub fn as_bool(&self) -> bool { *self == Self::Disabled }
    pub fn aria(&self) -> Option<&'static str> {
        if self.as_bool() { Some("true") } else { None }
    }
}
impl From<bool> for DisabledState {
    fn from(d: bool) -> Self { if d { Self::Disabled } else { Self::Enabled } }
}
impl ToDataAttr for DisabledState {
    fn to_data_attr(&self) -> (&'static str, &'static str) {
        ("data-rs-disabled", self.as_str())
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum LoadingState {
    #[default] Idle,
    Loading,
}
impl LoadingState {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Idle => "idle", Self::Loading => "loading" }
    }
}
impl From<bool> for LoadingState {
    fn from(b: bool) -> Self { if b { Self::Loading } else { Self::Idle } }
}
impl ToDataAttr for LoadingState {
    fn to_data_attr(&self) -> (&'static str, &'static str) {
        ("data-rs-loading", self.as_str())
    }
}


// ── ARIA + HTML HELPERS ──────────────────────────────────────────────────────
// Métodos diretos nos enums — elimina necessidade de state_engine structs

impl VisibilityState {
    pub fn aria_hidden(&self) -> &'static str {
        if *self == Self::Open { "false" } else { "true" }
    }
    pub fn aria_expanded(&self) -> &'static str {
        if *self == Self::Open { "true" } else { "false" }
    }
    pub fn hidden(&self) -> bool { *self != Self::Open }
}

impl ActivityState {
    pub fn aria_selected(&self) -> &'static str {
        if *self == Self::Active { "true" } else { "false" }
    }
    pub fn hidden(&self) -> bool { *self != Self::Active }
}

impl SelectionState {
    pub fn aria_selected(&self) -> Option<&'static str> {
        if *self == Self::Selected { Some("true") } else { None }
    }
}

impl ToggleState {
    pub fn aria_pressed(&self) -> &'static str {
        if *self == Self::On { "true" } else { "false" }
    }
    pub fn checked(&self) -> bool { *self == Self::On }
}

impl DisabledState {
    pub fn aria_disabled(&self) -> Option<&'static str> {
        if *self == Self::Disabled { Some("true") } else { None }
    }
    pub fn disabled(&self) -> bool { *self == Self::Disabled }
    pub fn href_or<'a>(&self, href: &'a str) -> &'a str {
        if *self == Self::Disabled { "#" } else { href }
    }
    pub fn tabindex(&self) -> &'static str {
        if *self == Self::Disabled { "-1" } else { "0" }
    }
}

impl LoadingState {
    pub fn aria_busy(&self) -> Option<&'static str> {
        if *self == Self::Loading { Some("true") } else { None }
    }
}

// ── STATE KIND (unificador sem ambiguidade) ───────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StateKind {
    Visibility(VisibilityState),
    Activity(ActivityState),
    Selection(SelectionState),
    Toggle(ToggleState),
    Navigation(NavigationState),
    Disabled(DisabledState),
    Loading(LoadingState),
}

impl StateKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Visibility(s) => s.as_str(),
            Self::Activity(s)   => s.as_str(),
            Self::Selection(s)  => s.as_str(),
            Self::Toggle(s)     => s.as_str(),
            Self::Navigation(s) => s.as_str(),
            Self::Disabled(s)   => s.as_str(),
            Self::Loading(s)    => s.as_str(),
        }
    }

    pub fn to_data_attr(&self) -> (&'static str, &'static str) {
        match self {
            Self::Visibility(s) => s.to_data_attr(),
            Self::Activity(s)   => s.to_data_attr(),
            Self::Selection(s)  => s.to_data_attr(),
            Self::Toggle(s)     => s.to_data_attr(),
            Self::Navigation(s) => s.to_data_attr(),
            Self::Disabled(s)   => s.to_data_attr(),
            Self::Loading(s)    => s.to_data_attr(),
        }
    }
}

// ── CAPABILITY → PROPS MAPPING ───────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct CapabilityPropDef {
    pub key:     &'static str,
    pub label:   &'static str,
    pub kind:    CapabilityPropKind,
    pub default: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CapabilityPropKind {
    Bool,
    String,
    Enum(&'static [&'static str]),
    // Typed state — alinha Capability com runtime enum
    DisabledState,
    LoadingState,
    VisibilityState,
    ActivityState,
    SelectionState,
    ToggleState,
}

pub fn capability_to_props_enum(cap: Capability) -> &'static [CapabilityPropDef] {
    match cap {
        Capability::OpenClose => &[
            CapabilityPropDef { key: "open",         label: "Open",         kind: CapabilityPropKind::VisibilityState, default: Some("closed") },
            CapabilityPropDef { key: "default-open", label: "Default Open", kind: CapabilityPropKind::Bool,            default: Some("false") },
        ],
        Capability::Selected => &[
            CapabilityPropDef { key: "selected", label: "Selected", kind: CapabilityPropKind::SelectionState, default: Some("unselected") },
        ],
        Capability::Active => &[
            CapabilityPropDef { key: "active", label: "Active", kind: CapabilityPropKind::ActivityState, default: Some("inactive") },
        ],
        Capability::Pressed => &[
            CapabilityPropDef { key: "pressed", label: "Pressed", kind: CapabilityPropKind::ToggleState, default: Some("off") },
        ],
        Capability::Disabled => &[
            CapabilityPropDef { key: "disabled", label: "Disabled", kind: CapabilityPropKind::DisabledState, default: Some("enabled") },
        ],
        Capability::Loading => &[
            CapabilityPropDef { key: "loading", label: "Loading", kind: CapabilityPropKind::LoadingState, default: Some("idle") },
        ],
        Capability::Value => &[
            CapabilityPropDef { key: "value", label: "Value", kind: CapabilityPropKind::String, default: None },
        ],
        Capability::Multiple => &[
            CapabilityPropDef { key: "multiple", label: "Multiple", kind: CapabilityPropKind::Bool, default: Some("false") },
        ],
        Capability::Orientation => &[
            CapabilityPropDef { key: "orientation", label: "Orientation", kind: CapabilityPropKind::Enum(&["horizontal", "vertical"]), default: Some("horizontal") },
        ],
        Capability::FocusTrap | Capability::KeyboardEsc | Capability::AriaModal |
        Capability::KeyboardArrows | Capability::Roving | Capability::Typeahead |
        Capability::VirtualScroll | Capability::DragDrop | Capability::Resize |
        Capability::Overflow | Capability::Animation => &[],
    }
}

pub fn derive_props_from_capabilities(caps: &[Capability]) -> Vec<CapabilityPropDef> {
    let mut seen   = std::collections::HashSet::new();
    let mut result = Vec::new();
    for cap in caps {
        for prop in capability_to_props_enum(*cap) {
            if seen.insert(prop.key) {
                result.push(prop.clone());
            }
        }
    }
    result
}

pub fn derive_props_from_meta(meta: &ComponentMeta) -> Vec<CapabilityPropDef> {
    derive_props_from_capabilities(meta.capabilities)
}
