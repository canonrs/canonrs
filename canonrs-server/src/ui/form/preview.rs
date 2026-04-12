use leptos::prelude::*;
use super::form_ui::{Form, FormActions, FormSection, FormField, FormLabel, FormHint, FormError, FieldValidationState, FormValidationState};
use crate::ui::button::button_ui::Button;
use crate::ui::input::Input;

#[component]
pub fn FormShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Form>
                    <FormSection>
                        <FormField>
                            <FormLabel html_for="name" required=true><span>{"Full name"}</span></FormLabel>
                            <FormHint><span>{"Your full legal name."}</span></FormHint>
                            <Input placeholder="John Doe" />
                        </FormField>
                        <FormField>
                            <FormLabel html_for="email"><span>{"Email"}</span></FormLabel>
                            <Input placeholder="john@example.com" />
                        </FormField>
                    </FormSection>
                    <FormActions>
                        <Button><span>{"Submit"}</span></Button>
                    </FormActions>
                </Form>
            </div>
            <p data-rs-showcase-preview-anchor="">
                <span>{"Form lifecycle and validation state enforced at container level."}</span>
            </p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label=""><span>{"With validation"}</span></span>
                <div data-rs-showcase-preview-row="">
                    <Form validation=FormValidationState::Error>
                        <FormSection>
                            <FormField validation=FieldValidationState::Invalid>
                                <FormLabel html_for="email2"><span>{"Email"}</span></FormLabel>
                                <FormError><span>{"Please enter a valid email address."}</span></FormError>
                                <Input placeholder="invalid@" />
                            </FormField>
                            <FormField validation=FieldValidationState::Valid>
                                <FormLabel html_for="name2"><span>{"Name"}</span></FormLabel>
                                <Input placeholder="John Doe" />
                            </FormField>
                        </FormSection>
                        <FormActions>
                            <Button><span>{"Submit"}</span></Button>
                        </FormActions>
                    </Form>
                </div>
            </div>

        </div>
    }
}
