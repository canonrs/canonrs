use crate::ui::input::input_boundary::Input;
use leptos::prelude::*;
use super::field_ui::{Field, FieldLabel, FieldDescription, FieldError, FieldSet, FieldGroup, FieldContent};
use super::variants::{FieldOrientation, FieldValidation};

#[component]
pub fn FieldShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <FieldSet>
                    <FieldGroup>
                        <Field>
                            <FieldLabel html_for="field-email"><span>{"Email"}</span></FieldLabel>
                            <FieldDescription><span>{"Enter your email address."}</span></FieldDescription>
                            <FieldContent>
                                <Input placeholder="john@example.com" />
                            </FieldContent>
                        </Field>
                    </FieldGroup>
                </FieldSet>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Validation, label and error state unified in a single contract."
            </p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Error state"</span>
                <div data-rs-showcase-preview-row="">
                    <FieldSet>
                        <FieldGroup>
                            <Field _validation=FieldValidation::Error>
                                <FieldLabel html_for="field-email2"><span>{"Email"}</span></FieldLabel>
                                <FieldContent>
                                    <Input placeholder="invalid@" />
                                </FieldContent>
                                <FieldError errors=vec!["Please enter a valid email.".to_string()] />
                            </Field>
                        </FieldGroup>
                    </FieldSet>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Horizontal"</span>
                <div data-rs-showcase-preview-row="">
                    <FieldSet>
                        <FieldGroup>
                            <Field orientation=FieldOrientation::Horizontal>
                                <FieldLabel html_for="field-name"><span>{"Name"}</span></FieldLabel>
                                <FieldContent>
                                    <Input placeholder="John Doe" />
                                </FieldContent>
                            </Field>
                        </FieldGroup>
                    </FieldSet>
                </div>
            </div>

        </div>
    }
}
