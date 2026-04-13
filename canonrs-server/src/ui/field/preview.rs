use leptos::prelude::*;
use super::field_boundary::{Field, FieldLabel, FieldDescription, FieldError, FieldSet, FieldGroup, FieldContent};
use super::variants::{FieldOrientation, FieldValidation};
use crate::ui::input::input_boundary::Input;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn FieldShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <FieldSet>
                <FieldGroup>
                    <Field>
                        <FieldLabel html_for="field-email"><span>"Email"</span></FieldLabel>
                        <FieldDescription><span>"Enter your email address."</span></FieldDescription>
                        <FieldContent>
                            <Input placeholder="john@example.com" />
                        </FieldContent>
                    </Field>
                </FieldGroup>
            </FieldSet>
            <p data-rs-showcase-preview-anchor="">
                "Validation, label and error state unified in a single contract."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Error state"</span>
                <FieldSet>
                    <FieldGroup>
                        <Field _validation=FieldValidation::Error>
                            <FieldLabel html_for="field-email2"><span>"Email"</span></FieldLabel>
                            <FieldContent>
                                <Input placeholder="invalid@" />
                            </FieldContent>
                            <FieldError errors=vec!["Please enter a valid email.".to_string()] />
                        </Field>
                    </FieldGroup>
                </FieldSet>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Horizontal"</span>
                <FieldSet>
                    <FieldGroup>
                        <Field orientation=FieldOrientation::Horizontal>
                            <FieldLabel html_for="field-name"><span>"Name"</span></FieldLabel>
                            <FieldContent>
                                <Input placeholder="John Doe" />
                            </FieldContent>
                        </Field>
                    </FieldGroup>
                </FieldSet>
            </Stack>
        </Stack>
    }
}
