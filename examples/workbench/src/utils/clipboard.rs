use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = document, js_name = execCommand)]
    fn exec_command(command: &str) -> bool;
}

pub fn copy_to_clipboard(text: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Ok(textarea) = document.create_element("textarea") {
                    if let Ok(textarea) = textarea.dyn_into::<web_sys::HtmlTextAreaElement>() {
                        textarea.set_value(text);
                        
                        if let Some(body) = document.body() {
                            let _ = body.append_child(&textarea);
                            textarea.select();
                            let _ = exec_command("copy");
                            let _ = body.remove_child(&textarea);
                        }
                    }
                }
            }
        }
    }
}
