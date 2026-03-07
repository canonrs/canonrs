use leptos::prelude::*;
use super::page_header_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <PageHeader>
            <PageHeaderContent>
                <PageHeaderTitle>"Page Title"</PageHeaderTitle>
                <PageHeaderDescription>"Page description"</PageHeaderDescription>
            </PageHeaderContent>
        </PageHeader>
    }
}
