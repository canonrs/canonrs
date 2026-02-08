//! ============================================================================
//! CANONRS TEMPLATE — BLOCK
//! ============================================================================
//!
//! O QUE É UM BLOCK:
//! - Unidade de composição de PÁGINA
//! - Combina múltiplos UI Components
//! - Define ESTRUTURA, não comportamento
//! - Pode representar uma seção inteira da tela
//!
//! O QUE PODE USAR:
//! - UI Components (NUNCA Primitives diretamente)
//! - HTML estrutural simples (section, header, footer, main)
//! - Children e slots OPCIONAIS
//! - Props declarativas (title, description, variant)
//! - data-* attributes para CSS estrutural
//!
//! O QUE NÃO PODE USAR:
//! - ❌ Estado interno (RwSignal, create_signal)
//! - ❌ Controllers
//! - ❌ Browser APIs
//! - ❌ Callbacks obrigatórios
//! - ❌ Lógica de domínio
//! - ❌ Timers, async, effects
//! - ❌ Providers
//!
//! BLOCK NÃO DECIDE.
//! BLOCK NÃO CONTROLA.
//! BLOCK APENAS COMPÕE.
//!
//! Qualquer lógica = COMPONENT ou CONTROLLER.
//! ============================================================================

use leptos::prelude::*;
use crate::ui::{Card, Button};

#[component]
pub fn ExampleBlock(
    #[prop(default = String::new())] title: String,
    #[prop(default = String::new())] description: String,
    #[prop(optional)] header: Option<Children>,
    #[prop(optional)] footer: Option<Children>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <section class="canon-block">
            {header.map(|h| view! {
                <header class="canon-block-header">
                    {h()}
                </header>
            })}

            <Card>
                <div class="canon-block-body">
                    {(!title.is_empty()).then(|| view! {
                        <h2 class="canon-block-title">{title}</h2>
                    })}

                    {(!description.is_empty()).then(|| view! {
                        <p class="canon-block-description">{description}</p>
                    })}

                    {children.map(|c| c())}
                </div>
            </Card>

            {footer.map(|f| view! {
                <footer class="canon-block-footer">
                    {f()}
                </footer>
            })}
        </section>
    }
}
