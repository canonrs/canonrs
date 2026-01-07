use leptos::prelude::*;
use rs_design::*;

use rs_design::ui::dialog::*;
#[component]
pub fn EditTokenDialog(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into)] token_id: Signal<String>,
    #[prop(into)] token_name: Signal<String>,
    #[prop(into)] token_value: Signal<String>,
    #[prop(into)] token_scope: Signal<String>,
    #[prop(into)] token_category: Signal<String>,
    #[prop(into)] token_status: Signal<String>,
) -> impl IntoView {
    let name = RwSignal::new(String::new());
    let value = RwSignal::new(String::new());
    let scope = RwSignal::new(String::new());
    let category = RwSignal::new(String::new());
    let status = RwSignal::new(String::new());
    #[cfg(target_arch = "wasm32")]

    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if open.get() {
            name.set(token_name.get());
            value.set(token_value.get());
            scope.set(token_scope.get());
            category.set(token_category.get());
            status.set(token_status.get());
        }
    });

    let on_save = move |_| {
        leptos::logging::log!("Save token {}: name={}, value={}, scope={}, category={}, status={}", 
            token_id.get(), name.get(), value.get(), scope.get(), category.get(), status.get());
        open.set(false);
    };

    view! {
        <Dialog open=open>
            <DialogContent class="sm:max-w-[500px]">
                <DialogHeader>
                    <DialogTitle>"Edit Token"</DialogTitle>
                    <DialogDescription>
                        "Make changes to the token. Click save when you're done."
                    </DialogDescription>
                </DialogHeader>
                <div class="grid gap-4 py-4">
                    <div class="grid gap-2">
                        <Label>"Name"</Label>
                        <Input value=name placeholder="Token name" />
                    </div>
                    <div class="grid gap-2">
                        <Label>"Value"</Label>
                        <Input value=value placeholder="Token value" />
                    </div>
                    <div class="grid gap-2">
                        <Label>"Scope"</Label>
                        <Input value=scope placeholder="canonical/contextual" />
                    </div>
                    <div class="grid gap-2">
                        <Label>"Category"</Label>
                        <Input value=category placeholder="Category" />
                    </div>
                    <div class="grid gap-2">
                        <Label>"Status"</Label>
                        <Input value=status placeholder="active/draft/deprecated" />
                    </div>
                </div>
                <DialogFooter>
                    <Button on:click=on_save>"Save changes"</Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}
