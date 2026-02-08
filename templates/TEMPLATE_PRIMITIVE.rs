//! ============================================================================
//! CANONRS TEMPLATE — PRIMITIVE
//! ============================================================================
//!
//! REGRA DE USO DO TEMPLATE:
//! - Template = copiar LITERALMENTE
//! - Zero interpretação
//! - Zero "acho que é assim"
//! - Apenas substituir nomes específicos (Example → YourPrimitive)
//!
//! O QUE PODE USAR:
//! - HTML semântico puro (button, div, input, label, etc)
//!   Exemplo: ButtonPrimitive usa <button>, LabelPrimitive usa <label>
//!
//! - Props estruturais simples (id, class, disabled, role, tabindex)
//!   Exemplo: CheckboxPrimitive recebe checked, disabled, name, value
//!
//! - data-* attributes para comunicação visual com CSS
//!   Exemplo: attr:data-checkbox="", attr:data-state={if checked { "checked" } else { "unchecked" }}
//!
//! - aria-* attributes para acessibilidade
//!   Exemplo: attr:aria-checked={if checked { "true" } else { "false" }}
//!
//! - Signals APENAS como leitura direta (Signal<T>), sem lógica
//!
//! - Atributos HTML diretos sem prefixo quando palavra reservada
//!   Exemplo: LabelPrimitive usa for={for_id} (não attr:for pois "for" é palavra reservada)
//!
//! O QUE NÃO PODE USAR:
//! - ❌ CSS inline ou lógica de estilo
//!
//! - ❌ Estado interno (use_state, RwSignal interno, create_signal)
//!
//! - ❌ use_effect, create_resource, spawn_local, Effect::new
//!   Violação: ButtonPrimitive antigamente tinha Effect::new para anexar eventos
//!
//! - ❌ Event handlers (on:click, on:input, on:change, etc)
//!   Violação: InputPrimitive antigamente tinha on:input, on:change, on:blur
//!   Correto: InputPrimitive hoje não tem handlers nenhum
//!
//! - ❌ Callbacks de negócio
//!   Exemplo ruim: #[prop(default = Callback::new(|_| {}))] on_click: Callback<ev::MouseEvent>
//!   Motivo: Causa erro de hidratação SSR
//!
//! - ❌ Browser APIs (window, document, clipboard, timers)
//!
//! - ❌ Controllers
//!
//! - ❌ Decidir comportamento ou fluxo
//!
//! - ❌ prop:* (props reativas causam hidratação)
//!   Violação: LabelPrimitive antigamente tinha prop:htmlFor={for_id}
//!   Correto: LabelPrimitive hoje usa for={for_id} (atributo direto)
//!
//! ATENÇÃO HIDRATAÇÃO:
//! - Prefira attr:* sempre que possível
//!   Exemplo: attr:aria-disabled={if disabled { "true" } else { "false" }}
//!
//! - Use atributo direto (sem prefixo) apenas quando attr: não funciona
//!   Exemplo: for={for_id} em LabelPrimitive (palavra reservada)
//!
//! - Nunca use prop:* - causa problemas de hidratação SSR
//!   Componentes testados: Button, Input, Label, Switch, Checkbox - todos sem prop:*
//!
//! Primitive NÃO decide. Primitive NÃO coordena. Primitive NÃO tem opinião.
//! Primitive é APENAS HTML semântico puro.
//! ============================================================================

use leptos::prelude::*;

#[component]
pub fn ExamplePrimitive(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            id={id}
            class={class}
            attr:data-disabled={if disabled { "true" } else { "" }}
            attr:aria-disabled={if disabled { "true" } else { "" }}
        >
            {children.map(|c| c())}
        </div>
    }
}
