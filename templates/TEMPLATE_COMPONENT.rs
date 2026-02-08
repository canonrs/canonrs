//! ============================================================================
//! CANONRS TEMPLATE — COMPONENT
//! ============================================================================
//!
//! O QUE É UM COMPONENT:
//! - Camada de COORDENAÇÃO
//! - Conecta UI Components com estado, providers ou controllers
//! - Onde existe lógica de interação
//!
//! O QUE PODE USAR:
//! - UI Components (Button, Input, Dialog, etc)
//! - Signals (RwSignal, Signal, Memo)
//! - Providers (Theme, Density, Language, etc)
//! - Controllers (CSR-only)
//! - Callbacks
//! - Lógica de estado LOCAL ao componente
//!
//! O QUE NÃO PODE USAR:
//! - ❌ Primitives diretamente
//! - ❌ CSS inline
//! - ❌ HTML estrutural de página (section, main, layout)
//! - ❌ Providers próprios (Component NÃO cria provider)
//! - ❌ Efeitos globais
//!
//! COMPONENT COORDENA.
//! UI RENDERIZA.
//! PROVIDER DECIDE.
//!
//! Se virar estrutural → é BLOCK.
//! Se virar lógica pesada → é CONTROLLER.
//! ============================================================================

use leptos::prelude::*;
use crate::ui::{Button, Dialog};
use crate::providers::use_theme;

#[component]
pub fn ExampleComponent(
    #[prop(default = String::new())] label: String,
    #[prop(default = Callback::new(|_| {}))] on_confirm: Callback<()>,
) -> impl IntoView {
    // Estado LOCAL do componente
    let open = create_rw_signal(false);

    // Uso de Provider (sem criar)
    let theme = use_theme();

    let open_dialog = Callback::new(move |_| {
        open.set(true);
    });

    let confirm = Callback::new(move |_| {
        open.set(false);
        on_confirm.call(());
    });

    view! {
        <>
            <Button on_click={open_dialog}>
                {if label.is_empty() { "Open".to_string() } else { label }}
            </Button>

            <Dialog open=open>
                <div class="space-y-4">
                    <p>"Tema atual: " {theme.mode().to_string()}</p>

                    <div class="flex gap-2">
                        <Button on_click={confirm}>
                            "Confirm"
                        </Button>

                        <Button
                            variant="ghost"
                            on_click={Callback::new(move |_| open.set(false))}
                        >
                            "Cancel"
                        </Button>
                    </div>
                </div>
            </Dialog>
        </>
    }
}
