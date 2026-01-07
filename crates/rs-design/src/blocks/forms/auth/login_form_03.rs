use crate::ui::input::{InputType, InputValidation};
use crate::ui::{Button, Card, Input, Label};
use leptos::prelude::*;

#[component]
pub fn LoginForm03(
    #[prop(default = String::new(), into)] class: String,
    #[prop(optional)] on_submit: Option<Callback<(String, String)>>,
    #[prop(default = "Acme Inc.".to_string(), into)] company_name: String,
    #[prop(optional)] logo: Option<Children>,
) -> impl IntoView {
    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if let Some(callback) = on_submit {
            callback.run(("".to_string(), "".to_string()));
        }
    };

    view! {
        <div class="bg-muted flex min-h-screen flex-col items-center justify-center gap-6 p-6 md:p-10">
            <div class="flex w-full max-w-sm flex-col gap-6">
                // Logo/Brand
                <a href="#" class="flex items-center gap-2 self-center font-medium">
                    {match logo {
                        Some(logo_fn) => logo_fn().into_any(),
                        None => view! {
                            <div class="bg-primary text-primary-foreground flex size-6 items-center justify-center rounded-md">
                                <span class="text-xs">"A"</span>
                            </div>
                        }.into_any()
                    }}
                    <span>{company_name}</span>
                </a>

                // Form Card
                <div class=format!("flex flex-col gap-6 {}", class)>
                    <Card>
                        <div class="p-6 space-y-4">
                            <div class="text-center space-y-2">
                                <h1 class="text-xl font-semibold">"Welcome back"</h1>
                                <p class="text-sm text-muted-foreground">
                                    "Login with your Apple or Google account"
                                </p>
                            </div>

                            <form on:submit=handle_submit class="space-y-4">
                                // Social buttons
                                <div class="space-y-2">
                                    <Button class="w-full".to_string()>
                                        "Login with Apple"
                                    </Button>
                                    <Button class="w-full".to_string()>
                                        "Login with Google"
                                    </Button>
                                </div>

                                // Separator
                                <div class="relative">
                                    <div class="absolute inset-0 flex items-center">
                                        <span class="w-full border-t" />
                                    </div>
                                    <div class="relative flex justify-center text-xs uppercase">
                                        <span class="bg-card px-2 text-muted-foreground">
                                            "Or continue with"
                                        </span>
                                    </div>
                                </div>

                                // Email
                                <div class="space-y-2">
                                    <Label html_for="email".to_string()>"Email"</Label>
                                    <Input
                                        input_type=InputType::Email
                                        placeholder="m@example.com"
                                        class="w-full".to_string()
                                    />
                                </div>

                                // Password
                                <div class="space-y-2">
                                    <div class="flex items-center">
                                        <Label html_for="password".to_string()>"Password"</Label>
                                        <a

                                            href="#"
                                            class="ml-auto text-sm underline-offset-4 hover:underline"
                                        >
                                            "Forgot your password?"
                                        </a>
                                    </div>
                                    <Input
                                        input_type=InputType::Password
                                        class="w-full".to_string()
                                    />
                                </div>

                                // Submit
                                <Button class="w-full".to_string()>"Login"</Button>

                                <p class="text-center text-sm text-muted-foreground">
                                    "Don't have an account? "
                                    <a href="#" class="underline">"Sign up"</a>
                                </p>
                            </form>
                        </div>
                    </Card>

                    <p class="px-6 text-center text-xs text-muted-foreground">
                        "By clicking continue, you agree to our "
                        <a href="#" class="underline">"Terms of Service"</a>
                        " and "
                        <a href="#" class="underline">"Privacy Policy"</a>
                        "."
                    </p>
                </div>
            </div>
        </div>
    }
}
