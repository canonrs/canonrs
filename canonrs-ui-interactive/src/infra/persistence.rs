use leptos::prelude::*;
use crate::ui::theme_engine::{ThemeState, ThemeTokens, ActiveMode};

fn get_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

pub fn save_theme(theme: &ThemeState) {
    let Some(storage) = get_storage() else { return };
    let light = theme.light.get();
    let dark  = theme.dark.get();
    let active = theme.active.get();
    let radius = theme.radius.get();
    if let Ok(j) = serde_json::to_string(&light)  { storage.set_item("canonrs_light",  &j).ok(); }
    if let Ok(j) = serde_json::to_string(&dark)   { storage.set_item("canonrs_dark",   &j).ok(); }
    if let Ok(j) = serde_json::to_string(&active) { storage.set_item("canonrs_active", &j).ok(); }
    if let Ok(j) = serde_json::to_string(&radius) { storage.set_item("canonrs_radius", &j).ok(); }
}

pub fn load_theme_into(theme: &ThemeState) {
    let Some(storage) = get_storage() else { return };
    if let Ok(Some(j)) = storage.get_item("canonrs_light") {
        if let Ok(v) = serde_json::from_str::<ThemeTokens>(&j) { theme.light.set(v); }
    }
    if let Ok(Some(j)) = storage.get_item("canonrs_dark") {
        if let Ok(v) = serde_json::from_str::<ThemeTokens>(&j) { theme.dark.set(v); }
    }
    if let Ok(Some(j)) = storage.get_item("canonrs_active") {
        if let Ok(v) = serde_json::from_str::<ActiveMode>(&j) { theme.active.set(v); }
    }
    if let Ok(Some(j)) = storage.get_item("canonrs_radius") {
        if let Ok(v) = serde_json::from_str::<f32>(&j) { theme.radius.set(v); }
    }
}

pub fn persist_theme(theme: ThemeState) {
    let t = theme.clone();
    Effect::new(move |_| {
        let _ = t.active.get();
        let _ = t.light.get();
        let _ = t.dark.get();
        let _ = t.radius.get();
        save_theme(&t);
    });
}
