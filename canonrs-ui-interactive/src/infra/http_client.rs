#[cfg(target_arch = "wasm32")]
use serde_json::Value;

#[cfg(target_arch = "wasm32")]
pub async fn get_json(url: &str) -> Result<Value, String> {
    use leptos::wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::Response;

    let window = web_sys::window().ok_or("no window")?;
    let resp_value = JsFuture::from(window.fetch_with_str(url))
        .await.map_err(|e| format!("{:?}", e))?;
    let resp: Response = resp_value.dyn_into().map_err(|e| format!("{:?}", e))?;
    let json = JsFuture::from(resp.json().map_err(|e| format!("{:?}", e))?)
        .await.map_err(|e| format!("{:?}", e))?;
    let text = js_sys::JSON::stringify(&json).map_err(|e| format!("{:?}", e))?;
    let s = text.as_string().ok_or("not string")?;
    serde_json::from_str(&s).map_err(|e| e.to_string())
}

#[cfg(target_arch = "wasm32")]
pub async fn post_json(url: &str, body: String) -> Result<(Value, u16), String> {
    use leptos::wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, RequestMode, Response};

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&leptos::wasm_bindgen::JsValue::from_str(&body));

    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|e| format!("{:?}", e))?;
    request.headers().set("Content-Type", "application/json")
        .map_err(|e| format!("{:?}", e))?;

    let window = web_sys::window().ok_or("no window")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await.map_err(|e| format!("{:?}", e))?;
    let resp: Response = resp_value.dyn_into().map_err(|e| format!("{:?}", e))?;
    let status = resp.status();
    let json = JsFuture::from(resp.json().map_err(|e| format!("{:?}", e))?)
        .await.map_err(|e| format!("{:?}", e))?;
    let text = js_sys::JSON::stringify(&json).map_err(|e| format!("{:?}", e))?;
    let s = text.as_string().ok_or("not string")?;
    let v: Value = serde_json::from_str(&s).map_err(|e| e.to_string())?;
    Ok((v, status))
}

#[cfg(target_arch = "wasm32")]
pub async fn delete_json(url: &str) -> Result<(), String> {
    use leptos::wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, RequestMode};

    let opts = RequestInit::new();
    opts.set_method("DELETE");
    opts.set_mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|e| format!("{:?}", e))?;
    let window = web_sys::window().ok_or("no window")?;
    let _ = JsFuture::from(window.fetch_with_request(&request)).await;
    Ok(())
}
