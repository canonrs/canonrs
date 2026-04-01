use leptos::prelude::*;
use super::form_ui::{
    Form, FormSection, FormField, FormLabel,
    FormHint, FormError, FormActions,
};
use super::form_ui::{FormValidationState, FieldValidationState};
use crate::ui::input::Input;
use crate::ui::button::button_ui::Button;

#[component]
pub fn FormShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Form>
                    <FormSection>
                        <FormField>
                            <FormLabel html_for="name" required=true>"Full name"</FormLabel>
                            <Input placeholder="John Doe" />
                            <FormHint>"Your full legal name."</FormHint>
                        </FormField>
                        <FormField>
                            <FormLabel html_for="email" required=true>"Email"</FormLabel>
                            <Input placeholder="john@example.com" />
                        </FormField>
                    </FormSection>
                    <FormActions>
                        <Button>"Submit"</Button>
                    </FormActions>
                </Form>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Form lifecycle and validation state enforced at container level."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With validation"</span>
                <div data-rs-showcase-preview-row="">
                    <Form validation=FormValidationState::Error>
                        <FormSection>
                            <FormField validation=FieldValidationState::Invalid>
                                <FormLabel html_for="email2">"Email"</FormLabel>
                                <Input placeholder="invalid@" />
                                <FormError>"Please enter a valid email address."</FormError>
                            </FormField>
                            <FormField validation=FieldValidationState::Valid>
                                <FormLabel html_for="name2">"Name"</FormLabel>
                                <Input placeholder="John Doe" />
                            </FormField>
                        </FormSection>
                        <FormActions>
                            <Button>"Submit"</Button>
                        </FormActions>
                    </Form>
                </div>
            </div>
        </div>
    }
}
