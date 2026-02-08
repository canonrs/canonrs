//! ============================================================================
//! CANONRS TEMPLATE — UI COMPONENT
//! ============================================================================
//!
//! REGRA DE USO DO TEMPLATE:
//! - Template = copiar LITERALMENTE
//! - Zero interpretação
//! - Zero "acho que é assim"
//! - Apenas substituir nomes específicos (Example → Button, Input, etc)
//!
//! REGRA CRÍTICA SSR — NUNCA USE CALLBACK COM DEFAULT:
//! - ❌ ERRADO: #[prop(default = Callback::new(|_| {}))] on_click: Callback<ev::MouseEvent>
//!   Motivo: Causa erro "RuntimeError: unreachable" na hidratação
//!   Componentes que tinham esse erro: Button, Input, Switch, Checkbox (todos corrigidos)
//!
//! - ✅ CORRETO: #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>
//!   Componentes corretos: Button, Input, Switch, Checkbox
//!
//! REGRA CRÍTICA — BRANCHING ESTRUTURAL OBRIGATÓRIO:
//! UI NÃO pode interceptar eventos com if let dentro do handler.
//!
//! - ❌ ERRADO (viola semântica DOM):
//!   on:click=move |ev| {
//!       if let Some(handler) = on_click {
//!           handler.run(ev);
//!       }
//!   }
//!   Motivo: Handler sempre existe no DOM mesmo sem callback
//!   Componente corrigido: Button (tinha esse padrão errado)
//!
//! - ✅ CORRETO (branching estrutural):
//!   if let Some(handler) = on_click {
//!       Either::Left(view! {
//!           <ExamplePrimitive
//!               on:click=move |ev| handler.run(ev)
//!           >
//!               {children.map(|c| c())}
//!           </ExamplePrimitive>
//!       })
//!   } else {
//!       Either::Right(view! {
//!           <ExamplePrimitive>
//!               {children.map(|c| c())}
//!           </ExamplePrimitive>
//!       })
//!   }
//!   Componentes corretos: Button, Switch, Checkbox, Input
//!
//! CASO ESPECIAL — MÚLTIPLOS EVENTOS OPCIONAIS:
//! Para componentes com 2+ eventos (ex: Input com on_input, on_change, on_blur):
//! - Branch principal: Input sem handlers vs Input com handlers
//! - Dentro do branch "com handlers": pode usar if let nos handlers inline
//!
//! Exemplo (Input):
//! if on_input.is_none() && on_change.is_none() && on_blur.is_none() {
//!     Either::Left(view! { <InputPrimitive ... /> })
//! } else {
//!     Either::Right(view! {
//!         <InputPrimitive
//!             on:input=move |ev| if let Some(h) = &on_input { h.run(ev); }
//!             on:change=move |ev| if let Some(h) = &on_change { h.run(ev); }
//!             on:blur=move |ev| if let Some(h) = &on_blur { h.run(ev); }
//!         />
//!     })
//! }
//!
//! REGRA — UI NÃO INTERCEPTA DISABLED:
//! - ❌ ERRADO:
//!   let handle_click = move |ev| {
//!       if !disabled { // UI não deve checar disabled
//!           handler.run(ev);
//!       }
//!   };
//!   Motivo: <button disabled> já bloqueia eventos no browser
//!   Componente corrigido: Button (tinha if !disabled antes)
//!
//! - ✅ CORRETO:
//!   on:click=move |ev| handler.run(ev)
//!   // Browser gerencia disabled automaticamente
//!   Componentes corretos: Button, Switch, Checkbox
//!
//! REGRA CRÍTICA — NUNCA PASSAR Option<Callback> PARA COMPONENTES:
//! UI coordenador decide. Componentes filhos NUNCA recebem Option<Callback>.
//!
//! - ❌ ERRADO (DataTable antes da correção):
//!   <Button on_click=Some(Callback::new(...)) />
//!   <TableRow on_click=Some(row_handler) />
//!   Motivo: Empurra decisão para dentro do componente, viola contrato
//!
//! - ✅ CORRETO (DataTable após correção):
//!   if let Some(handler) = on_click {
//!       view! { <Button on_click=handler /> }
//!   } else {
//!       view! { <Button /> }
//!   }
//!   Motivo: Decisão externa, componente recebe Callback direto ou nada
//!
//! REGRA — PRESENÇA CONDICIONAL DENTRO DE CHILDREN:
//! Dentro de children de outro componente, use .then() para Option<View>.
//!
//! - ❌ ERRADO:
//!   <TableRow>
//!       {if selectable {
//!           view! { <TableCell>...</TableCell> }.into_view()
//!       } else {
//!           ().into_view()
//!       }}
//!   </TableRow>
//!   Motivo: if/else retorna tipos incompatíveis dentro de children slot
//!
//! - ✅ CORRETO:
//!   <TableRow>
//!       {selectable.then(|| view! {
//!           <TableCell>...</TableCell>
//!       })}
//!   </TableRow>
//!   Motivo: .then() retorna Option<View>, que view! macro expande corretamente
//!
//! EXEMPLO COMPLETO — DataTable Pattern (componente complexo):
//! - Branching estrutural para TableRow com/sem on_row_click (Either)
//! - .then() para células condicionais (selectable)
//! - Callbacks adaptadores (MouseEvent → String/usize)
//! - NUNCA passa Option<Callback> para Button/Checkbox/TableRow
//!
//! Ver: /opt/docker/monorepo/packages-rust/rs-design/src/ui/data_table/data_table_ui.rs
//!
//! ============================================================================

use leptos::prelude::*;
use leptos::either::Either;
use crate::primitives::ExamplePrimitive;

#[component]
pub fn ExampleUI(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    if let Some(handler) = on_click {
        Either::Left(view! {
            <ExamplePrimitive
                id={id}
                class={class}
                disabled={disabled}
                on:click=move |_| handler(())
            >
                {children.map(|c| c())}
            </ExamplePrimitive>
        })
    } else {
        Either::Right(view! {
            <ExamplePrimitive
                id={id}
                class={class}
                disabled={disabled}
            >
                {children.map(|c| c())}
            </ExamplePrimitive>
        })
    }
}
