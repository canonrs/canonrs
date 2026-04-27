//! Action helpers — padrão Tier S para reagir a Action results

use leptos::prelude::*;

/// Reage ao resultado de uma Action usando trigger explícito.
///
/// Retorna um RwSignal<u32> que deve ser incrementado no on:click.
/// RwSignal é Copy — pode ser capturado por múltiplos closures.
///
/// Uso:
/// ```rust
/// let trigger = use_action_result(create, move |result| match result {
///     Ok(p)  => { ... }
///     Err(e) => { ... }
/// });
///
/// // no botão:
/// on:click=move |_| {
///     trigger.update(|v| *v += 1);
///     create.dispatch(());
/// }
/// ```
pub fn use_action_result<T, E, F>(
    action: Action<(), Result<T, E>>,
    on_result: F,
) -> RwSignal<u32>
where
    T: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static,
    F: Fn(Result<T, E>) + Send + Sync + 'static,
{
    let trigger      = RwSignal::new(0_u32);
    let last_handled = StoredValue::new(0_u32);
    let version      = action.version();
    let last_ver     = StoredValue::new(version.get_untracked());
    let on_result    = std::sync::Arc::new(on_result);

    Effect::new(move |_| {
        let trig = trigger.get();
        let ver  = version.get();

        if action.pending().get() { return; }
        if trig == last_handled.get_value() { return; }
        if ver <= last_ver.get_value() { return; }

        if let Some(result) = action.value().get_untracked() {
            last_handled.set_value(trig);
            last_ver.set_value(ver);
            let cb = on_result.clone();
            untrack(move || cb(result));
        }
    });

    trigger
}
