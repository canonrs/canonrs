use leptos::callback::Callback;
use leptos::prelude::*;
use leptos::ev::MouseEvent;
use super::drag_drop::{DragHandle, DropZone};
use crate::ui::button::Button;
use super::types::{DashboardConfig, WidgetDef as Widget, WidgetContent, WidgetRemoveEvent};

#[component]
pub fn Dashboard(
    config: DashboardConfig,
    widgets: Vec<Widget>,
    #[prop(optional)] on_widget_remove: Option<Callback<WidgetRemoveEvent>>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let widgets_signal = RwSignal::new(widgets);

    view! {
        <DropZone class={class.unwrap_or_default()}>
            <div
                attr:data-dashboard-grid=""
                attr:data-columns={config.columns}
            >
                <For
                    each=move || widgets_signal.get()
                    key=|w| w.id.clone()
                    children=move |widget| {
                        let widget_id = widget.id.clone();
                        let position = widget.position.clone();
                        let on_remove = on_widget_remove.clone();

                        let wid = widget_id.clone();
                        view! {
                            <DragHandle>
                                <div
                                    attr:data-dashboard-widget=""
                                    attr:data-widget-id={wid.clone()}
                                    attr:data-grid-x={position.x}
                                    attr:data-grid-y={position.y}
                                    attr:data-grid-width={position.width}
                                    attr:data-grid-height={position.height}
                                    attr:data-resizable={widget.resizable}
                                    attr:data-removable={widget.removable}
                                >
                                    <div data-widget-header="">
                                        <h3 data-widget-title="">{widget.title.clone()}</h3>
                                        <Show when=move || widget.removable>
                                            {
                                                let wid = widget_id.clone();
                                                view! {
                                                    <Button
                                                        attr:data-widget-remove=""
                                                        on_click=Callback::new(move |_: MouseEvent| {
                                                            if let Some(ref handler) = on_remove {
                                                                handler.run(WidgetRemoveEvent {
                                                                    widget_id: wid.clone(),
                                                                });
                                                            }
                                                        })
                                                    >
                                                        "✕"
                                                    </Button>
                                                }
                                            }
                                        </Show>
                                    </div>

                                    <div data-widget-content="">
                                        {match &widget.content {
                                            WidgetContent::Empty => {
                                                view! { <div data-widget-empty="">"Empty widget"</div> }.into_view()
                                            }
                                            WidgetContent::Text(text) => {
                                                view! { <div data-widget-text="">{text.clone()}</div> }.into_view()
                                            }
                                            WidgetContent::Html(html) => {
                                                view! { <div data-widget-html="" inner_html={html.clone()}></div> }.into_view()
                                            }
                                        }}
                                    </div>
                                </div>
                            </DragHandle>
                        }
                    }
                />
            </div>
        </DropZone>
    }
}
