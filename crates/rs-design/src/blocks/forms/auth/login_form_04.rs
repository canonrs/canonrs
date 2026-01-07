use leptos::prelude::*;
use crate::ui::*;
use crate::providers::LanguageContext;

#[component]
pub fn LoginForm04() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    
    let lang_ctx = use_context::<LanguageContext>().expect("LanguageContext not found");
    
    // Tradução simples (migrar para i18n depois)
    let t = Memo::new(move |_| match lang_ctx.current.get().code.as_str() {
        "pt" => ("Email", "Senha", "Entrar", "Esqueceu a senha?"),
        "es" => ("Correo", "Contraseña", "Iniciar sesión", "¿Olvidaste tu contraseña?"),
        _ => ("Email", "Password", "Sign In", "Forgot password?"),
    });

    view! {
        <div class="w-full max-w-sm space-y-6">
            <div class="space-y-2 text-center">
                <h1 class="text-3xl font-bold">{move || t.get().2}</h1>
            </div>
            <div class="space-y-4">
                <div class="space-y-2">
                    <Label>{move || t.get().0}</Label>
                    <Input 
                        input_type=InputType::Email
                        placeholder="name@example.com"
                        value=email
                    />
                </div>
                <div class="space-y-2">
                    <Label>{move || t.get().1}</Label>
                    <Input 
                        input_type=InputType::Password
                        value=password
                    />
                </div>
                <Button class="w-full" variant=ButtonVariant::Solid>
                    {move || t.get().2}
                </Button>
                <div class="text-center text-sm">
                    <a href="#" class="text-primary hover:underline">
                        {move || t.get().3}
                    </a>
                </div>
            </div>
        </div>
    }
}
