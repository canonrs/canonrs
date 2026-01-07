use crate::tokens::SPACING;
use crate::ui::input::{InputType, InputValidation};
use crate::ui::{Button, Card, Input, Label};
use leptos::prelude::*;

#[component]
pub fn LoginForm01(
    #[prop(default = String::new(), into)] class: String,
    #[prop(optional)] on_submit: Option<Callback<(String, String)>>,
) -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if let Some(callback) = on_submit {
            callback.run((email.get(), password.get()));
        }
    };

    view! {
        <div class=format!("flex flex-col gap-6 {}", class)>
            <Card>
                <div class=format!("p-6 space-y-[{}]", SPACING.md)>
                    <div class="space-y-2">
                        <h2 class="text-2xl font-bold">"Login to your account"</h2>
                        <p class="text-sm text-muted-foreground">
                            "Enter your email below to login to your account"
                        </p>
                    </div>

                    <form on:submit=handle_submit class="space-y-4">
                        <div class="space-y-2">
                            <Label html_for="email".to_string()>
                                "Email"
                            </Label>
                            <Input
                                input_type=InputType::Email
                                placeholder="m@example.com"
                                class="w-full".to_string()
                            />
                        </div>

                        <div class="space-y-2">
                            <div class="flex items-center">
                                <Label html_for="password".to_string()>
                                    "Password"
                                </Label>

                                <a
                                    href="#"
                                    class="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                                >
                                    "Forgot your password?"
                                </a>
                            </div>
                            <Input
                                input_type=InputType::Password
                                class="w-full".to_string()
                            />
                        </div>

                        <div class="space-y-4">
                            <Button class="w-full".to_string()>
                                "Login"
                            </Button>
                            <Button class="w-full".to_string()>
                                "Login with Google"
                            </Button>
                            <p class="text-center text-sm text-muted-foreground">
                                "Don't have an account? "
                                <a href="#" class="underline underline-offset-4 hover:text-primary">
                                    "Sign up"
                                </a>
                            </p>
                        </div>
                    </form>
                </div>
            </Card>
        </div>
    }
}
