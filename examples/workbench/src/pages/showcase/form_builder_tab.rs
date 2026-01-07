use leptos::prelude::*;
use rs_design::ui::form_builder::{
    FormBuilder, FormSchema, FieldDef, FieldType, SelectOption,
    FieldDependency, DependencyCondition,
};

#[component]
pub fn FormBuilderTab() -> impl IntoView {
    // Example 1: Simple Contact Form
    let contact_schema = FormSchema::new("contact", "Contact Us")
        .field(
            FieldDef::new("name", "Full Name", FieldType::Text)
                .placeholder("John Doe")
                .required()
                .min_length(2)
                .max_length(100)
        )
        .field(
            FieldDef::new("email", "Email Address", FieldType::Email)
                .placeholder("john@example.com")
                .required()
                .email()
        )
        .field(
            FieldDef::new("phone", "Phone Number", FieldType::Tel)
                .placeholder("+1 (555) 123-4567")
                .pattern(r"^\+?[1-9]\d{1,14}$")
                .help_text("Include country code")
        )
        .field(
            FieldDef::new("subject", "Subject", FieldType::Select)
                .required()
                .options(vec![
                    SelectOption::new("General Inquiry", "general"),
                    SelectOption::new("Support", "support"),
                    SelectOption::new("Sales", "sales"),
                    SelectOption::new("Feedback", "feedback"),
                ])
        )
        .field(
            FieldDef::new("message", "Message", FieldType::TextArea)
                .placeholder("Tell us more...")
                .required()
                .min_length(10)
                .max_length(500)
        )
        .submit_label("Send Message")
        .cancel_label("Clear");
    
    // Example 2: User Registration with Conditional Fields
    let registration_schema = FormSchema::new("registration", "Create Account")
        .field(
            FieldDef::new("username", "Username", FieldType::Text)
                .placeholder("johndoe")
                .required()
                .min_length(3)
                .max_length(20)
                .pattern(r"^[a-zA-Z0-9_]+$")
                .help_text("Letters, numbers, and underscores only")
        )
        .field(
            FieldDef::new("email", "Email", FieldType::Email)
                .placeholder("john@example.com")
                .required()
                .email()
        )
        .field(
            FieldDef::new("password", "Password", FieldType::Password)
                .required()
                .min_length(8)
                .help_text("Minimum 8 characters")
        )
        .field(
            FieldDef::new("account_type", "Account Type", FieldType::Radio)
                .required()
                .options(vec![
                    SelectOption::new("Personal", "personal"),
                    SelectOption::new("Business", "business"),
                ])
        )
        .field(
            FieldDef::new("company", "Company Name", FieldType::Text)
                .placeholder("Acme Corp")
                .required()
                .depends_on(FieldDependency {
                    field_id: "account_type".to_string(),
                    condition: DependencyCondition::Equals("business".to_string()),
                })
        )
        .field(
            FieldDef::new("newsletter", "Subscribe to Newsletter", FieldType::Checkbox)
                .default_value("false")
        )
        .submit_label("Create Account");
    
    // Example 3: Event Booking Form
    let booking_schema = FormSchema::new("booking", "Book Event")
        .field(
            FieldDef::new("event_name", "Event Name", FieldType::Text)
                .required()
        )
        .field(
            FieldDef::new("event_date", "Event Date", FieldType::Date)
                .required()
        )
        .field(
            FieldDef::new("event_time", "Event Time", FieldType::Time)
                .required()
        )
        .field(
            FieldDef::new("attendees", "Number of Attendees", FieldType::Number)
                .required()
                .min(1.0)
                .max(1000.0)
        )
        .field(
            FieldDef::new("budget", "Budget ($)", FieldType::Range)
                .default_value("5000")
                .min(1000.0)
                .max(50000.0)
        )
        .submit_label("Book Event");
    
    let (active_form, set_active_form) = signal("contact");
    
    let on_contact_submit = Callback::new(move |values| {
        leptos::logging::log!("📧 Contact form submitted: {:?}", values);
    });
    
    let on_registration_submit = Callback::new(move |values| {
        leptos::logging::log!("👤 Registration submitted: {:?}", values);
    });
    
    let on_booking_submit = Callback::new(move |values| {
        leptos::logging::log!("📅 Booking submitted: {:?}", values);
    });
    
    let on_cancel = Callback::new(move |_| {
        leptos::logging::log!("❌ Form cancelled");
    });
    
    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-2xl font-bold mb-2">"Form Builder"</h2>
                <p class="text-muted-foreground">"Schema-based forms with validation"</p>
            </div>
            
            <div class="flex gap-2 border-b">
                <button
                    class=move || {
                        let base = "px-4 py-2 text-sm font-medium border-b-2 -mb-px";
                        if active_form.get() == "contact" {
                            format!("{} border-primary text-primary", base)
                        } else {
                            format!("{} border-transparent hover:border-gray-300", base)
                        }
                    }
                    on:click=move |_| set_active_form.set("contact")
                >
                    "Contact Form"
                </button>
                
                <button
                    class=move || {
                        let base = "px-4 py-2 text-sm font-medium border-b-2 -mb-px";
                        if active_form.get() == "registration" {
                            format!("{} border-primary text-primary", base)
                        } else {
                            format!("{} border-transparent hover:border-gray-300", base)
                        }
                    }
                    on:click=move |_| set_active_form.set("registration")
                >
                    "Registration"
                </button>
                
                <button
                    class=move || {
                        let base = "px-4 py-2 text-sm font-medium border-b-2 -mb-px";
                        if active_form.get() == "booking" {
                            format!("{} border-primary text-primary", base)
                        } else {
                            format!("{} border-transparent hover:border-gray-300", base)
                        }
                    }
                    on:click=move |_| set_active_form.set("booking")
                >
                    "Event Booking"
                </button>
            </div>
            
            <div class="grid grid-cols-2 gap-6">
                <div class="border rounded-lg p-6 bg-card">
                    {move || {
                        match &*active_form.get() {
                            "contact" => view! {
                                <FormBuilder
                                    schema=contact_schema.clone()
                                    on_submit=on_contact_submit
                                    on_cancel=on_cancel
                                />
                            }.into_any(),
                            "registration" => view! {
                                <FormBuilder
                                    schema=registration_schema.clone()
                                    on_submit=on_registration_submit
                                />
                            }.into_any(),
                            "booking" => view! {
                                <FormBuilder
                                    schema=booking_schema.clone()
                                    on_submit=on_booking_submit
                                />
                            }.into_any(),
                            _ => view! { <></> }.into_any(),
                        }
                    }}
                </div>
                
                <div class="space-y-4">
                    <div class="p-4 bg-blue-50 border border-blue-200 rounded">
                        <p class="text-sm font-semibold text-blue-900 mb-2">"Features"</p>
                        <ul class="text-xs text-blue-700 space-y-1">
                            <li>"✅ 16 field types (text, email, select, date, etc)"</li>
                            <li>"✅ Schema-based validation"</li>
                            <li>"✅ Required, min/max length, pattern, email, url"</li>
                            <li>"✅ Conditional fields (dependencies)"</li>
                            <li>"✅ Custom validators"</li>
                            <li>"✅ Auto-validation on blur"</li>
                            <li>"✅ Error messages"</li>
                            <li>"✅ Loading state"</li>
                            <li>"✅ Reset functionality"</li>
                        </ul>
                    </div>
                    
                    <div class="p-4 bg-green-50 border border-green-200 rounded">
                        <p class="text-sm font-semibold text-green-900 mb-2">"Examples"</p>
                        <ul class="text-xs text-green-700 space-y-1">
                            <li>"📧 Contact Form - Simple 5 field form"</li>
                            <li>"👤 Registration - Conditional company field"</li>
                            <li>"📅 Event Booking - Date/time/range inputs"</li>
                        </ul>
                    </div>
                    
                    <div class="p-4 bg-purple-50 border border-purple-200 rounded">
                        <p class="text-sm font-semibold text-purple-900 mb-2">"Try It"</p>
                        <ul class="text-xs text-purple-700 space-y-1">
                            <li>"1. Fill in the form fields"</li>
                            <li>"2. Click outside field to trigger validation"</li>
                            <li>"3. Try leaving required fields empty"</li>
                            <li>"4. In Registration, select 'Business' account"</li>
                            <li>"5. Watch 'Company Name' field appear"</li>
                            <li>"6. Submit and check console (F12)"</li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}
