use leptos::prelude::*;
use super::form_ui::*;
use canonrs_core::primitives::FieldValidationState;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display:flex;flex-direction:column;gap:2rem;max-width:480px;">
            <Form>
                <FormSection aria_label="Personal Info".to_string()>
                    <FormField>
                        <FormLabel html_for="full-name" required=true>"Full Name"</FormLabel>
                        <input type="text" id="full-name" name="full_name" data-rs-input="" />
                        <FormHint>"Enter your full legal name"</FormHint>
                    </FormField>
                    <FormField validation=FieldValidationState::Invalid>
                        <FormLabel html_for="email" required=true>"Email"</FormLabel>
                        <input type="email" id="email" name="email" data-rs-input="" />
                        <FormError>"Please enter a valid email address"</FormError>
                    </FormField>
                    <FormField validation=FieldValidationState::Valid>
                        <FormLabel html_for="username">"Username"</FormLabel>
                        <input type="text" id="username" name="username" data-rs-input="" />
                        <FormHint>"Username is available"</FormHint>
                    </FormField>
                </FormSection>
                <FormActions>
                    <button type="submit" data-rs-button="">"Submit"</button>
                    <button type="reset" data-rs-button="">"Reset"</button>
                </FormActions>
            </Form>
        </div>
    }
}
