use leptos::prelude::*;
use rs_design::*;
use rs_design::ui::input::{Input, InputType};
use rs_design::ui::dialog::{Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle, DialogDescription};

use super::database_server::update_component_status;

#[component]
pub fn EditComponentDialog(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into)] component_id: Signal<String>,
    #[prop(into)] component_name: Signal<String>,
    #[prop(into)] component_tipo: Signal<String>,
    #[prop(into)] tokens_canonicos: Signal<i32>,
    #[prop(into)] tokens_familia_c: Signal<i32>,
    #[prop(into)] familias_aplicadas: Signal<String>,
    #[prop(into)] has_readme: Signal<bool>,
    #[prop(into)] folder_structure: Signal<bool>,
    #[prop(into)] status: Signal<String>,
) -> impl IntoView {
    let local_tokens = RwSignal::new(String::new());
    let local_familia = RwSignal::new(String::new());
    let local_familias = RwSignal::new(String::new());
    let local_readme = RwSignal::new(false);
    let local_structure = RwSignal::new(false);
    let local_status = RwSignal::new(String::new());
    
    let saving = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);

    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if open.get() {
            local_tokens.set(tokens_canonicos.get().to_string());
            local_familia.set(tokens_familia_c.get().to_string());
            local_familias.set(familias_aplicadas.get());
            local_readme.set(has_readme.get());
            local_structure.set(folder_structure.get());
            local_status.set(status.get());
        }
    });

    let on_save = move |_| {
        saving.set(true);
        error_msg.set(None);
        
        let name = component_name.get();
        let tokens = local_tokens.get().parse::<i32>().unwrap_or(0);
        let familia = local_familia.get().parse::<i32>().unwrap_or(0);
        let familias = local_familias.get();
        let readme = local_readme.get();
        let structure = local_structure.get();
        let stat = local_status.get();
        
        leptos::logging::log!("Would save - spawn_local disabled");
    };

    view! {
        <Dialog open=open>
            <DialogContent class="sm:max-w-[600px]">
                <DialogHeader>
                    <DialogTitle>"Edit Component: " {move || component_tipo.get()}</DialogTitle>
                    <DialogDescription>
                        "Update canonical compliance for " <code class="font-mono">{move || component_name.get()}</code>
                    </DialogDescription>
                </DialogHeader>
                
                {move || error_msg.get().map(|msg| view! {
                    <div class="bg-red-50 border border-red-200 text-red-800 px-4 py-3 rounded">
                        {msg}
                    </div>
                })}
                
                <div class="grid gap-4 py-4">
                    <div class="grid gap-2">
                        <Label>"Tokens Canônicos (%)"</Label>
                        <Input 
                            value=local_tokens
                            input_type=InputType::Number
                            placeholder="0-100"
                        />
                    </div>
                    
                    <div class="grid gap-2">
                        <Label>"Famílias (%)"</Label>
                        <Input 
                            value=local_familia
                            input_type=InputType::Number
                            placeholder="0-100"
                        />
                    </div>
                    
                    <div class="grid gap-2">
                        <Label>"Famílias Aplicadas"</Label>
                        <Input 
                            value=local_familias
                            placeholder="A,B,C"
                        />
                        <p class="text-xs text-gray-500">"Separadas por vírgula (ex: A,B,C)"</p>
                    </div>
                    
                    <div class="flex items-center gap-2">
                        <input 
                            type="checkbox"
                            id="readme"
                            checked=move || local_readme.get()
                            on:change=move |ev| {
                                local_readme.set(event_target_checked(&ev));
                            }
                            class="h-4 w-4"
                        />
                        <Label attr:r#for="readme">"README.md criado"</Label>
                    </div>
                    
                    <div class="flex items-center gap-2">
                        <input 
                            type="checkbox"
                            id="structure"
                            checked=move || local_structure.get()
                            on:change=move |ev| {
                                local_structure.set(event_target_checked(&ev));
                            }
                            class="h-4 w-4"
                        />
                        <Label attr:r#for="structure">"Estrutura de pastas correta"</Label>
                    </div>
                    
                    <div class="grid gap-2">
                        <Label>"Status"</Label>
                        <select 
                            class="w-full h-10 px-3 rounded-md border"
                            on:change=move |ev| {
                                local_status.set(event_target_value(&ev));
                            }
                        >
                            <option value="draft" selected=move || local_status.get() == "draft">"Draft"</option>
                            <option value="review" selected=move || local_status.get() == "review">"Review"</option>
                            <option value="stable" selected=move || local_status.get() == "stable">"Stable"</option>
                            <option value="deprecated" selected=move || local_status.get() == "deprecated">"Deprecated"</option>
                        </select>
                    </div>
                </div>
                
                <DialogFooter>
                    <Button 
                        on:click=on_save
                        disabled=saving.get()
                    >
                        {move || if saving.get() { "Saving..." } else { "Save changes" }}
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}
