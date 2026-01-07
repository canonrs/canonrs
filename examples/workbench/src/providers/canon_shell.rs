use leptos::prelude::*;
use rs_design::{LanguageProvider, ThemeProvider, DensityProvider, ThemeEngine};
use rs_design::ui::drag_drop::{DragDropProvider, DragDropCallbacksProvider};
use crate::providers::command_history::CommandHistoryProvider;

#[component]
pub fn CanonShell(children: Children) -> impl IntoView {
    let initial_language = "en".to_string();
    let initial_theme = "dark".to_string();
    let initial_density = rs_design::types::DensityMode::Comfortable;

    view! {
        <DragDropProvider>
            <DragDropCallbacksProvider>
                <CommandHistoryProvider>
                    <LanguageProvider initial_language=initial_language>
                        <ThemeProvider
                            initial_theme=initial_theme
                            storage_key="canonrs-theme"
                        >
                            <DensityProvider initial_mode=initial_density>
                                <ThemeEngine/>
                                {children()}
                            </DensityProvider>
                        </ThemeProvider>
                    </LanguageProvider>
                </CommandHistoryProvider>
            </DragDropCallbacksProvider>
        </DragDropProvider>
    }
}
