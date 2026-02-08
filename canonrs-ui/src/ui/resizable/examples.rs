use leptos::prelude::*;
use super::resizable_ui::*;
use super::resizable_panel_ui::*;
use super::resizable_handle_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <Resizable>
            <ResizablePanel>"Left Panel"</ResizablePanel>
            <ResizableHandle />
            <ResizablePanel>"Right Panel"</ResizablePanel>
        </Resizable>
    }
}
