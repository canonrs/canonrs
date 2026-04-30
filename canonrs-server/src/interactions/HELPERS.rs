// canonrs_helpers — central reference
//
// Todos os helpers disponiveis no CanonRS
// Fonte: canonrs-server/src/interactions/ + canonrs-client/src/hooks/

// ── IMPORTS ─────────────────────────────────────────────────────────────────

use canonrs::dialog_close;          // fecha Dialog pelo uid
use canonrs::confirm_dialog_close;  // fecha ConfirmDialog pelo uid
use canonrs::input_reset;           // reseta Input pelo name
use canonrs::use_action_result;     // reage a Action com trigger
use canonrs::hooks::use_select;     // bridge Select DOM -> RwSignal (so hydrate)
use canonrs::slot;                  // ChildrenFn sem boilerplate

// ── dialog_close ─────────────────────────────────────────────────────────────
//
// Fecha um Dialog pelo uid apos acao async concluir.
// SSR-safe: no-op no servidor.
//
// canonrs::dialog_close("dialog-new-project");

// ── confirm_dialog_close ──────────────────────────────────────────────────────
//
// Fecha um ConfirmDialog pelo uid.
// SSR-safe: no-op no servidor.
//
// canonrs::confirm_dialog_close("confirm-delete-project");

// ── input_reset ───────────────────────────────────────────────────────────────
//
// Reseta um Input pelo name apos submit.
// SSR-safe: no-op no servidor.
//
// canonrs::input_reset("project-name");

// ── use_action_result ─────────────────────────────────────────────────────────
//
// Reage ao resultado de uma Action com trigger explicito.
// Retorna RwSignal<u32> que deve ser incrementado no on:click.
//
// let trigger = canonrs::use_action_result(create, move |result| match result {
//     Ok(p)  => { canonrs::dialog_close("dialog-new-project"); }
//     Err(e) => { error.set(Some(e.to_string())); }
// });
//
// on:click=move |_| {
//     trigger.update(|v| *v += 1);
//     create.dispatch(());
// }

// ── use_select ────────────────────────────────────────────────────────────────
//
// Bridge entre Select DOM-driven e RwSignal<String>.
// Disponivel APENAS em hydrate (wasm32).
//
// let selected = RwSignal::new(String::new());
// let node_ref = canonrs::hooks::use_select(selected);
// <Select node_ref=node_ref> ... </Select>

// ── slot! ─────────────────────────────────────────────────────────────────────
//
// Converte closure em ChildrenFn (Arc<dyn Fn() -> AnyView>).
// Elimina Arc::new boilerplate em slots de Block e Layout.
//
// slot!(|| view! { <Foo /> }.into_any())           -- estatico
// slot!(move || view! { <Foo val=val /> }.into_any()) -- captura escopo
//
// slot! pode ser aninhado dentro de outro slot!.
// Sempre termina com .into_any() dentro do view!.
