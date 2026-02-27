//! # MarketingLayout — Regions: header, hero, main, footer
use leptos::prelude::*;

#[component]
pub fn MarketingLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] hero: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = Signal::derive(|| String::new()))] header_zone_id: Signal<String>,
    #[prop(default = Signal::derive(|| String::new()))] hero_zone_id: Signal<String>,
    #[prop(default = Signal::derive(|| String::new()))] main_zone_id: Signal<String>,
    #[prop(default = Signal::derive(|| String::new()))] footer_zone_id: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="layout-marketing" data-layout="marketing" data-layout-version="1">
            <header class="layout-marketing-header"
                data-layout-region="header"
                data-region-hint="Drop logo, nav or actions"
                data-drop-zone=move || (!header_zone_id.get().is_empty()).then_some("")
                data-zone-id=move || (!header_zone_id.get().is_empty()).then(|| header_zone_id.get())>
                {header.map(|h| h())}
            </header>
            <section class="layout-marketing-hero"
                data-layout-region="hero"
                data-region-hint="Drop hero banner"
                data-drop-zone=move || (!hero_zone_id.get().is_empty()).then_some("")
                data-zone-id=move || (!hero_zone_id.get().is_empty()).then(|| hero_zone_id.get())>
                {hero.map(|h| h())}
            </section>
            <main class="layout-marketing-main"
                data-layout-region="main"
                data-region-hint="Drop page sections"
                data-drop-zone=move || (!main_zone_id.get().is_empty()).then_some("")
                data-zone-id=move || (!main_zone_id.get().is_empty()).then(|| main_zone_id.get())>
                {children()}
            </main>
            <footer class="layout-marketing-footer"
                data-layout-region="footer"
                data-region-hint="Drop footer links"
                data-drop-zone=move || (!footer_zone_id.get().is_empty()).then_some("")
                data-zone-id=move || (!footer_zone_id.get().is_empty()).then(|| footer_zone_id.get())>
                {footer.map(|f| f())}
            </footer>
        </div>
    }
}
