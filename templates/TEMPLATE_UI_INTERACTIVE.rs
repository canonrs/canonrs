//! Template para Componentes Interactive - CanonRS
//! 
//! Este template define o padrão CANÔNICO para criar componentes interativos
//! no crate `canonrs-ui-interactive`.
//!
//! ═══════════════════════════════════════════════════════════════════════════
//! REGRA DE OURO
//! ═══════════════════════════════════════════════════════════════════════════
//! 
//! Interactive componentes são WRAPPERS que adicionam comportamento (callbacks)
//! a componentes SSR-safe do `canonrs-ui`. Eles NUNCA criam nova estrutura DOM.
//!
//! ═══════════════════════════════════════════════════════════════════════════
//! ✅ PERMITIDO (O QUE VOCÊ DEVE FAZER)
//! ═══════════════════════════════════════════════════════════════════════════
//!
//! 1. Importar componente base do `canonrs-ui`
//! 2. Adicionar callbacks tipados com tipos de DOMÍNIO (não DOM events)
//! 3. Converter eventos DOM → tipos de domínio dentro do handler
//! 4. Colocar eventos DIRETAMENTE no componente base (on:click, on:input, etc)
//! 5. Usar mesmas props do componente base + callbacks extras
//! 6. Extrair valores reais do DOM (event_target_value, checked state, etc)
//!
//! ═══════════════════════════════════════════════════════════════════════════
//! ❌ PROIBIDO (O QUE VOCÊ NUNCA DEVE FAZER)
//! ═══════════════════════════════════════════════════════════════════════════
//!
//! 1. ❌ NUNCA criar wrapper `<div on:*>` ao redor do componente
//! 2. ❌ NUNCA usar `Callback<MouseEvent>` ou outros tipos de DOM raw
//! 3. ❌ NUNCA criar novo estado local (RwSignal, Signal, StoredValue)
//! 4. ❌ NUNCA adicionar elementos HTML novos (divs, spans, wrappers)
//! 5. ❌ NUNCA compor outros componentes complexos (Popover, Modal, etc)
//! 6. ❌ NUNCA criar layout ou estrutura visual
//! 7. ❌ NUNCA passar eventos DOM diretamente pro callback (converter primeiro)
//! 8. ❌ NUNCA usar Either/Option para renderização condicional
//!
//! ═══════════════════════════════════════════════════════════════════════════
//! EXCEÇÃO ÚNICA (Canon Rule explícita)
//! ═══════════════════════════════════════════════════════════════════════════
//!
//! Componentes "Interactive" podem compor primitives diretamente APENAS quando
//! representam fluxos modais atômicos que requerem orquestração de callbacks
//! entre múltiplos elementos.
//!
//! Exemplo: ConfirmDialogInteractive (orquestra Dialog + Buttons com callbacks)
//!
//! Esta exceção deve ser:
//! - Documentada explicitamente no arquivo
//! - Justificada tecnicamente
//! - Aprovada como Canon Rule
//!
//! ═══════════════════════════════════════════════════════════════════════════

use leptos::prelude::*;
use leptos::ev;
use canonrs_ui::ui::example::ExampleComponent; // ✅ Importa do canonrs-ui

/// ExampleInteractive - Versão interativa do ExampleComponent
///
/// Este componente adiciona callbacks ao ExampleComponent base,
/// convertendo eventos DOM em tipos de domínio apropriados.
///
/// # Callbacks
/// - `on_action`: Acionado quando o usuário interage com o componente
///
/// # Exemplo
/// ```rust
/// <ExampleInteractive
///     value="test"
///     on_action=Callback::new(|data: String| {
///         log!("Action triggered with: {}", data);
///     })
/// />
/// ```
#[component]
pub fn ExampleInteractive(
    // ✅ Props do componente base (mesmas do SSR)
    #[prop(default = String::new())] value: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    
    // ✅ Callbacks TIPADOS com tipos de DOMÍNIO (não DOM events)
    #[prop(optional)] on_action: Option<Callback<String>>,    // ✅ String (domínio)
    // ❌ on_action: Option<Callback<ev::MouseEvent>>,        // ❌ MouseEvent (DOM)
    
    // ✅ Outros callbacks conforme necessário
    #[prop(optional)] on_change: Option<Callback<bool>>,      // ✅ bool (domínio)
    #[prop(optional)] on_complete: Option<Callback<()>>,      // ✅ () quando não há dados
) -> impl IntoView {
    // ❌ PROIBIDO: criar estado local
    // let (local_state, set_local_state) = signal(false);
    
    // ❌ PROIBIDO: StoredValue ou outros estados
    // let stored = StoredValue::new(value);
    
    // ✅ CORRETO: Usar componente base diretamente com eventos
    view! {
        <ExampleComponent
            value=value
            disabled=disabled
            class=class
            id=id
            
            // ✅ Evento direto no componente (não em wrapper)
            on:click=move |ev: ev::MouseEvent| {
                if !disabled {
                    // ✅ Converter DOM → Domínio
                    let domain_data = extract_data_from_event(&ev);
                    
                    if let Some(ref handler) = on_action {
                        // ✅ Chamar callback com tipo de domínio
                        handler.run(domain_data);
                    }
                }
            }
            
            // ✅ Outro evento se necessário
            on:change=move |ev: ev::Event| {
                if let Some(ref handler) = on_change {
                    // ✅ Extrair valor real do DOM
                    let checked = event_target_checked(&ev);
                    handler.run(checked);
                }
            }
        />
    }
    
    // ❌ ERRADO: Wrapper div
    // view! {
    //     <div on:click=move |ev| { ... }>
    //         <ExampleComponent ... />
    //     </div>
    // }
}

// ✅ Função auxiliar para conversão DOM → Domínio
fn extract_data_from_event(ev: &ev::MouseEvent) -> String {
    // Extrai dados relevantes do evento
    // Converte para tipo de domínio apropriado
    "extracted_data".to_string()
}

// ═══════════════════════════════════════════════════════════════════════════
// CHECKLIST ANTES DE COMMIT
// ═══════════════════════════════════════════════════════════════════════════
//
// [ ] Componente importa do canonrs-ui?
// [ ] Callbacks usam tipos de DOMÍNIO (não MouseEvent/Event)?
// [ ] Eventos estão DIRETOS no componente (sem <div> wrapper)?
// [ ] Não há estado local (Signal, RwSignal, StoredValue)?
// [ ] Não há elementos HTML novos?
// [ ] Conversão DOM → Domínio está implementada?
// [ ] Exemplo de uso documentado?
// [ ] Se compõe primitives, tem justificativa EXCEPTION RULE?
//
// ═══════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════
// LEPTOS 0.8 - REGRAS CRÍTICAS
// ═══════════════════════════════════════════════════════════════════════════
//
// ✅ Children SEMPRE precisa .map(|c| c())
// view! { {children.map(|c| c())} }
//
// ✅ Props Option<T> precisam unwrap
// class=class.unwrap_or_default()
// id=id.unwrap_or_default()
//
// ✅ StoredValue para state em closures (evita FnOnce)
// let stored = StoredValue::new(value);
// move || stored.get_value()
//
// ✅ Either para branches condicionais
// {move || {
//     if show {
//         Either::Left(view! { <div>A</div> })
//     } else {
//         Either::Right(view! { <div>B</div> })
//     }
// }}
//
// ❌ NUNCA .then(|| view! {...}) em closures
// ❌ NUNCA {children} direto
// ❌ NUNCA move || que consome variáveis (causa FnOnce error)
// ❌ NUNCA branches if/else com tipos diferentes
//
// ═══════════════════════════════════════════════════════════════════════════
// ERROS COMUNS E SOLUÇÕES
// ═══════════════════════════════════════════════════════════════════════════
//
// ERROR E0282 (type annotations needed)
// → Use Either::Left/Right para branches condicionais
//
// ERROR E0525 (FnOnce vs FnMut)
// → Use StoredValue::new() ou clone antes do closure
//
// ERROR E0308 (Option<T> não é T)
// → Use .unwrap_or_default() ou .unwrap_or(value)
//
// ERROR "IntoRender not satisfied for Option<Children>"
// → Use {children.map(|c| c())} sempre
//
// ═══════════════════════════════════════════════════════════════════════════
