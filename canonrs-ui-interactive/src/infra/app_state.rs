use leptos::prelude::*;
use std::sync::OnceLock;
use rs_canonrs::domain::CanonDocument;
use crate::ui::theme_engine::ThemeState;
use crate::ui::layout_builder::state::builder_engine::BuilderEngine;
use crate::ui::layout_builder::types::{DragContext, CanvasMode, ActiveLayout, init_slots};
use crate::ui::layout_builder::state::drop_zone_types::DragVisualState;
use crate::ui::layout_builder::domain::node::Node;
use uuid::Uuid;

static APP_OWNER: OnceLock<Owner> = OnceLock::new();
static THEME_STATE: OnceLock<ThemeState> = OnceLock::new();
static BUILDER_ENGINE:   OnceLock<RwSignal<BuilderEngine>>   = OnceLock::new();
static BUILDER_TREE:     OnceLock<RwSignal<Vec<Node>>>       = OnceLock::new();
static BUILDER_SELECTED: OnceLock<RwSignal<Option<Uuid>>>    = OnceLock::new();
static BUILDER_DIRTY:    OnceLock<RwSignal<bool>>            = OnceLock::new();
static BUILDER_LAYOUT:   OnceLock<RwSignal<ActiveLayout>>    = OnceLock::new();
static BUILDER_SLOTS:    OnceLock<RwSignal<Vec<Node>>>       = OnceLock::new();
static BUILDER_DRAG_CTX: OnceLock<RwSignal<DragContext>>     = OnceLock::new();
static BUILDER_CANVAS:   OnceLock<RwSignal<CanvasMode>>      = OnceLock::new();
static BUILDER_DRAG_VIS: OnceLock<RwSignal<DragVisualState>> = OnceLock::new();
static WORKSPACE_MODE:   OnceLock<RwSignal<WorkspaceMode>>   = OnceLock::new();

#[derive(Clone, Copy, PartialEq)]
pub enum WorkspaceMode { Builder, Theme }

pub fn app_owner() -> &'static Owner {
    APP_OWNER.get_or_init(Owner::new)
}

pub fn global_theme() -> ThemeState {
    THEME_STATE.get_or_init(|| ThemeState::new(app_owner())).clone()
}

pub fn global_workspace_mode() -> RwSignal<WorkspaceMode> {
    *WORKSPACE_MODE.get_or_init(|| app_owner().with(|| RwSignal::new(WorkspaceMode::Builder)))
}
pub fn global_engine() -> RwSignal<BuilderEngine> {
    *BUILDER_ENGINE.get_or_init(|| app_owner().with(|| RwSignal::new(BuilderEngine::new(CanonDocument::new("dashboard")))))
}
pub fn global_tree() -> RwSignal<Vec<Node>> {
    *BUILDER_TREE.get_or_init(|| app_owner().with(|| RwSignal::new(vec![])))
}
pub fn global_selected() -> RwSignal<Option<Uuid>> {
    *BUILDER_SELECTED.get_or_init(|| app_owner().with(|| RwSignal::new(None)))
}
pub fn global_dirty() -> RwSignal<bool> {
    *BUILDER_DIRTY.get_or_init(|| app_owner().with(|| RwSignal::new(false)))
}
pub fn global_active_layout() -> RwSignal<ActiveLayout> {
    *BUILDER_LAYOUT.get_or_init(|| app_owner().with(|| RwSignal::new(ActiveLayout::Dashboard)))
}
pub fn global_slots() -> RwSignal<Vec<Node>> {
    *BUILDER_SLOTS.get_or_init(|| app_owner().with(|| RwSignal::new(init_slots(&ActiveLayout::Dashboard))))
}
pub fn global_drag_ctx() -> RwSignal<DragContext> {
    *BUILDER_DRAG_CTX.get_or_init(|| app_owner().with(|| RwSignal::new(DragContext::empty())))
}
pub fn global_canvas_mode() -> RwSignal<CanvasMode> {
    *BUILDER_CANVAS.get_or_init(|| app_owner().with(|| RwSignal::new(CanvasMode::Builder)))
}
pub fn global_drag_visual() -> RwSignal<DragVisualState> {
    *BUILDER_DRAG_VIS.get_or_init(|| app_owner().with(|| RwSignal::new(DragVisualState::empty())))
}
