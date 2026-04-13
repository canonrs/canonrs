use leptos::prelude::*;
use super::form_boundary::{Form, FormActions, FormSection, FormField, FormLabel, FormHint, FormError};
use crate::ui::button::button_boundary::Button;
use crate::ui::input::input_boundary::Input;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn FormShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Form>
                <FormSection>
                    <FormField>
                        <FormLabel html_for="name" required=true><span>"Full name"</span></FormLabel>
                        <FormHint><span>"Your full legal name."</span></FormHint>
                        <Input placeholder="John Doe" />
                    </FormField>
                    <FormField>
                        <FormLabel html_for="email"><span>"Email"</span></FormLabel>
                        <Input placeholder="john@example.com" />
                    </FormField>
                </FormSection>
                <FormActions>
                    <Button><span>"Submit"</span></Button>
                </FormActions>
            </Form>
            <p data-rs-showcase-preview-anchor="">
                "Form lifecycle and validation state enforced at container level."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"With validation"</span>
                <Form>
                    <FormSection>
                        <FormField >
                            <FormLabel html_for="email2"><span>"Email"</span></FormLabel>
                            <FormError><span>"Please enter a valid email address."</span></FormError>
                            <Input placeholder="invalid@" />
                        </FormField>
                        <FormField >
                            <FormLabel html_for="name2"><span>"Name"</span></FormLabel>
                            <Input placeholder="John Doe" />
                        </FormField>
                    </FormSection>
                    <FormActions>
                        <Button><span>"Submit"</span></Button>
                    </FormActions>
                </Form>
            </Stack>
        </Stack>
    }
}
