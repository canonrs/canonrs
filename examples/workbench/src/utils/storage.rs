#[cfg(target_arch = "wasm32")]
use leptos::web_sys;

#[cfg(target_arch = "wasm32")]
pub fn get_local_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_local_storage() -> Option<()> {
    None
}

#[cfg(target_arch = "wasm32")]
pub fn set_item(key: &str, value: &str) -> Result<(), String> {
    get_local_storage()
        .ok_or("LocalStorage not available")?
        .set_item(key, value)
        .map_err(|e| format!("{:?}", e))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn set_item(_key: &str, _value: &str) -> Result<(), String> {
    Err("LocalStorage not available in SSR".to_string())
}

#[cfg(target_arch = "wasm32")]
pub fn get_item(key: &str) -> Option<String> {
    get_local_storage()?.get_item(key).ok()?
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_item(_key: &str) -> Option<String> {
    None
}
