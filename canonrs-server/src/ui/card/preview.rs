use leptos::prelude::*;
use super::card_ui::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};

#[component]
pub fn CardShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Card>
                    <CardHeader>
                        <CardTitle>"Getting Started"</CardTitle>
                        <CardDescription>"Everything you need to build with CanonRS."</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <p>"Card structure enforced with defined regions and roles. Compose header, content and footer independently."</p>
                    </CardContent>
                    <CardFooter>
                        <span>"Last updated: today"</span>
                    </CardFooter>
                </Card>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Card structure enforced with defined regions and roles."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-md);width:100%;">
                    <Card>
                        <CardHeader>
                            <CardTitle>"Header only"</CardTitle>
                        </CardHeader>
                    </Card>
                    <Card>
                        <CardContent>
                            <p>"Content only — no header or footer."</p>
                        </CardContent>
                    </Card>
                    <Card>
                        <CardHeader>
                            <CardTitle>"Full card"</CardTitle>
                            <CardDescription>"With all three regions."</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <p>"Body content goes here."</p>
                        </CardContent>
                        <CardFooter>
                            <span>"Footer action"</span>
                        </CardFooter>
                    </Card>
                </div>
            </div>
        </div>
    }
}
