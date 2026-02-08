use leptos::prelude::*;
use crate::components::markdown::CanonMarkdown;

use wasm_bindgen::JsCast;
#[component]
pub fn CanonDocModal(
    doc_filename: Signal<String>,
) -> impl IntoView {
    let (content, set_content) = signal(None::<Result<String, String>>);
    let (loading, set_loading) = signal(false);

    async fn fetch_markdown(filename: String) -> Result<String, String> {
        let url = format!("/docs/{}", filename);
        
        #[cfg(not(feature = "ssr"))]
        {
            let window = web_sys::window().ok_or("No window")?;
            let resp = wasm_bindgen_futures::JsFuture::from(
                window.fetch_with_str(&url)
            )
            .await
            .map_err(|_| "Fetch failed".to_string())?;
            
            let resp: web_sys::Response = resp.dyn_into().map_err(|_| "Not a response")?;
            let text = wasm_bindgen_futures::JsFuture::from(
                resp.text().map_err(|_| "No text method")?
            )
            .await
            .map_err(|_| "Text failed")?;
            
            text.as_string().ok_or("Not a string".to_string())
        }
        
        #[cfg(feature = "ssr")]
        {
            Err("SSR not supported".to_string())
        }
    }

    view! {
        <div class="canon-doc-modal">
            {
                Effect::new(move || {
                    let filename = doc_filename.get();
                    
                    if filename.is_empty() {
                        set_content.set(None);
                        return;
                    }
                    
                    #[cfg(not(feature = "ssr"))]
                    {
                        set_loading.set(true);
                        wasm_bindgen_futures::spawn_local(async move {
                            let result = fetch_markdown(filename).await;
                            set_content.set(Some(result));
                            set_loading.set(false);
                        });
                    }
                });
            }
            
            {move || if loading.get() {
                view! { <p>"Loading..."</p> }.into_any()
            } else {
                match content.get() {
                    Some(Ok(c)) => view! {
                        <CanonMarkdown content=c show_toc=true class="canon-doc-markdown"/>
                    }.into_any(),
                    Some(Err(e)) => view! {
                        <p class="error">{format!("Error: {}", e)}</p>
                    }.into_any(),
                    None => view! {
                        <p>"Select a document"</p>
                    }.into_any()
                }
            }}
        </div>
    }
}
