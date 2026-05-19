use leptos::prelude::*;
use canonrs_core::slot;
use crate::blocks::card::{CardBlock, CardVariant};
use crate::ui::card::{CardHeader, CardTitle, CardDescription, CardContent, CardFooter};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn CardShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <CardBlock
                header=slot!(|| view! {
                    <CardHeader>
                        <CardTitle>"Getting Started"</CardTitle>
                        <CardDescription>"Everything you need to build with CanonRS."</CardDescription>
                    </CardHeader>
                }.into_any())
                content=slot!(|| view! {
                    <CardContent><p>"Card structure enforced with defined regions and roles."</p></CardContent>
                }.into_any())
                footer=slot!(|| view! {
                    <CardFooter><span>"Last updated: today"</span></CardFooter>
                }.into_any())
            />
            <p data-rs-showcase-preview-anchor="">
                "Card structure enforced with defined regions and roles."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <Stack direction=StackDirection::Vertical gap=StackGap::Md>
                    <CardBlock
                        header=slot!(|| view! {
                            <CardHeader><CardTitle>"Header only"</CardTitle></CardHeader>
                        }.into_any())
                    />
                    <CardBlock
                        content=slot!(|| view! {
                            <CardContent><p>"Content only — no header or footer."</p></CardContent>
                        }.into_any())
                    />
                    <CardBlock
                        variant=CardVariant::Outlined
                        header=slot!(|| view! {
                            <CardHeader>
                                <CardTitle>"Full card"</CardTitle>
                                <CardDescription>"With all three regions."</CardDescription>
                            </CardHeader>
                        }.into_any())
                        content=slot!(|| view! {
                            <CardContent><p>"Body content goes here."</p></CardContent>
                        }.into_any())
                        footer=slot!(|| view! {
                            <CardFooter><span>"Footer action"</span></CardFooter>
                        }.into_any())
                    />
                </Stack>
            </Stack>
        </Stack>
    }
}
