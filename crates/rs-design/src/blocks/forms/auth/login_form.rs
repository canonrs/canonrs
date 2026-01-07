//! @canon-level: loose
//! @canon-exceptions: [#21]
//! @canon-justification: Legacy auth form
//! @canon-owner: auth-team
//! @canon-target-date: 2025-03-01

//! LoginForm Block - Enterprise Ready
use crate::ui::input::{InputType, InputValidation};
use crate::ui::button::{ButtonVariant, ButtonType};

use leptos::prelude::*;
use crate::ui::*;
use crate::blocks::feedback::{FormError, FormSuccess};
use crate::tokens::animations::AnimationVariant;

#[component]
pub fn LoginForm(
    #[prop(optional)] on_submit: Option<Callback<(String, String)>>,
) -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (email_touched, set_email_touched) = signal(false);
    let (password_touched, set_password_touched) = signal(false);
    
    let email_error = move || {
        if email_touched.get() && email.get().is_empty() {
            Some("Email é obrigatório".to_string())
        } else if email_touched.get() && !email.get().contains('@') {
            Some("Email inválido".to_string())
        } else {
            None
        }
    };
    
    let password_error = move || {
        if password_touched.get() && password.get().is_empty() {
            Some("Senha é obrigatória".to_string())
        } else if password_touched.get() && password.get().len() < 6 {
            Some("Mínimo 6 caracteres".to_string())
        } else {
            None
        }
    };
    
    let form_valid = move || {
        email_error().is_none() && password_error().is_none() && 
        !email.get().is_empty() && !password.get().is_empty()
    };
    
    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        set_email_touched.set(true);
        set_password_touched.set(true);
        
        if form_valid() {
            set_loading.set(true);
            if let Some(cb) = &on_submit {
                cb(email.get(), password.get());
            }
        }
    };
    
    view! {
        <Animate variant=AnimationVariant::FadeUp>
            <Card class="w-full max-w-md mx-auto p-8">
                <form on:submit=handle_submit class="space-y-6">
                    <div class="text-center space-y-2">
                        <h2 class="text-2xl font-bold">Bem-vindo</h2>
                        <p class="text-sm text-gray-600">Entre com suas credenciais</p>
                    </div>
                    
                    <div class="space-y-2">
                        <Label>"Email"</Label>
                        <Input
                            placeholder="seu@email.com"
                            value=email
                            on_input=move |v: String| {
                                set_email.set(v);
                                set_email_touched.set(true);
                            }
                        />
                        <FormError message=email_error() />
                    </div>
                    
                    <div class="space-y-2">
                        <Label>"Senha"</Label>
                        <Input
                            input_type=InputType::Password
                            placeholder="••••••••"
                            value=password
                            on_input=move |v: String| {
                                set_password.set(v);
                                set_password_touched.set(true);
                            }
                        />
                        <FormError message=password_error() />
                    </div>
                    
                    {move || error.get().map(|msg| view! {
                        <FormError message=Some(msg) />
                    })}
                    
                    <Button
                        button_type=ButtonType::Submit
                        full_width=true
                    >
                        {move || if loading.get() { "Entrando..." } else { "Entrar" }}
                    </Button>
                </form>
            </Card>
        </Animate>
    }
}
