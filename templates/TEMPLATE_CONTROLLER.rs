//! ============================================================================
//! CANONRS TEMPLATE —  (CSR ONLY)
//! ============================================================================
//!
//! REGRA DE USO DO TEMPLATE:
//! - Template = copiar LITERALMENTE
//! - Zero interpretação
//! - Zero "acho que é assim"
//! - Apenas substituir nomes específicos (Example → YourController)
//!
//! O QUE PODE USAR:
//! - Estado local (RwSignal, create_signal)
//! - Timers, async, effects
//! - Integração com Browser APIs (clipboard, focus, scroll)
//! - Coordenação entre múltiplos UIs
//! - Orquestração de callbacks
//!
//! O QUE NÃO PODE USAR:
//! - ❌ Renderizar HTML diretamente
//! - ❌ Definir estrutura visual
//! - ❌ CSS ou data-* attributes
//! - ❌ Ser usado no SSR
//!
//! Controller coordena. UI renderiza. Primitive estrutura.
//! ============================================================================

use leptos::prelude::*;

pub struct ExampleController {
    pub value: RwSignal<bool>,
}

impl ExampleController {
    pub fn new() -> Self {
        Self {
            value: RwSignal::new(false),
        }
    }

    pub fn toggle(&self) {
        self.value.update(|v| *v = !*v);
    }
}
