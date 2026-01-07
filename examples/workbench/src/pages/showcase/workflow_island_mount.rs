#[cfg(target_arch = "wasm32")]
pub fn mount_workflow_island() {
    use leptos::prelude::*;
    use leptos::mount::mount_to;
    use super::WorkflowClient;
    use wasm_bindgen::JsCast;

    leptos::logging::log!("🔍 Buscando #workflow-island...");
    
    if let Some(element) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("workflow-island"))
    {
        leptos::logging::log!("✅ Elemento encontrado!");
        element.set_inner_html("");
        
        leptos::logging::log!("🎯 Montando com closure...");
        let _root = mount_to(
            element.unchecked_into::<web_sys::HtmlElement>(),
            || view! { <WorkflowClient /> }
        );
        leptos::logging::log!("✅ Mount concluído");
    } else {
        leptos::logging::log!("❌ #workflow-island não encontrado");
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn mount_workflow_island() {}
