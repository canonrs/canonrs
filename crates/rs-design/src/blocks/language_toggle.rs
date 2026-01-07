use crate::providers::{LanguageContext, Language};
use leptos::prelude::*;
use crate::ui::button::*;

#[component]
pub fn LanguageToggle(
    #[prop(optional)] on_language_change: Option<Callback<String>>,
) -> impl IntoView {
    let lang_ctx = use_context::<LanguageContext>().expect("LanguageContext not found");
    let current_lang = lang_ctx.current;
    
    let languages = vec![
        ("en", "English"),
        ("pt", "Português"),
        ("es", "Español"),
    ];
    
    let change_language = move |lang_code: String| {
        let new_lang = Language::new(&lang_code);
        current_lang.set(new_lang);
        
        // Notify parent (for cookie persistence)
        if let Some(callback) = on_language_change {
            callback.run(lang_code);
        }
    };

    view! {
        <div class="flex gap-2">
            {languages.into_iter().map(|(code, name)| {
                let code_owned = code.to_string();
                let code_for_click = code_owned.clone();
                
                view! {
                    <Button
                        variant=ButtonVariant::Ghost
                        size=ButtonSize::Sm
                        on:click=move |_| change_language(code_for_click.clone())
                    >
                        {name}
                        {move || {
                            if current_lang.get().code == code_owned {
                                view! { <span class="ml-1">"✓"</span> }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }}
                    </Button>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
