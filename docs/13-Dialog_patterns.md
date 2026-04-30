# 13 — Dialog Patterns (CanonRS)

## Regra de Ouro

    Dialog e ConfirmDialog sao DOM-driven.
    Nunca vivem dentro de regioes reativas.
    Triggers nunca vivem dentro de slot!() ou For().

---

## 1. Estrutura obrigatoria

    ProjectsPage (owner dos signals)
      ├── ConfirmDeleteProject  ← dialog FIXO, fora do render dinamico
      └── ProjectsList          ← lista reativa, sem dialog dentro

    Signals declarados no Page — passados como props para os filhos.
    Dialog nunca recria porque esta fora da lista.

---

## 2. ConfirmDialog — implementacao correta

    // signals no Page
    let delete_id = RwSignal::new(Option::<i64>::None);
    let projects  = RwSignal::new(Vec::<Project>::new());

    // Action retorna o ID para permitir update local
    let delete = Action::new(move |id: &i64| {
        let id = *id;
        async move { delete_project(id).await.map(|_| id) }
    });

    // componente isolado — nunca recriado
    <ConfirmDeleteProject delete_id=delete_id delete=delete projects=projects />

---

## 3. Effect no ConfirmDeleteProject

    Effect::new(move |_| {
        let _ = delete.version().get();  // dependencia explicita
        if let Some(result) = delete.value().get_untracked() {
            match result {
                Ok(id) => {
                    projects.update(|list| list.retain(|p| p.id != id));
                    delete_id.set(None);
                    canonrs::confirm_dialog_close("confirm-delete");
                }
                Err(e) => {
                    error_msg.set(Some(e.to_string()));
                    canonrs::confirm_dialog_close("confirm-delete");
                }
            }
        }
    });

    REGRA: version().get() como dependencia explicita garante
    que o Effect re-executa em toda nova action — mesmo resultado igual.

---

## 4. Trigger na linha da tabela

    fn project_row(p: Project, delete_id: RwSignal<Option<i64>>) -> AnyView {
        let pid = StoredValue::new(p.id);  // StoredValue — nao Move
        view! {
            <ConfirmDialogTrigger
                variant=ConfirmDialogVariant::Destructive
                target="confirm-delete"
                on:click=move |_| {
                    delete_id.set(Some(pid.get_value()));
                }>
                "Delete"
            </ConfirmDialogTrigger>
        }.into_any()
    }

    REGRA: project_row e funcao normal (nao component).
    REGRA: pid usa StoredValue — nao captura por move.
    REGRA: For com key=|p| p.id garante reuso de nos DOM.

---

## 5. Update local — sem refetch

    CORRETO:
      Ok(id) => projects.update(|list| list.retain(|p| p.id != id));

    ERRADO:
      Ok(_) => version.update(|v| *v += 1);  // refetch destroe DOM

    ERRADO:
      Ok(id) => projects.update(...);  // update local
      version.update(...);             // + refetch = double render

    Escolha UM: update local OU refetch. Nunca os dois.

---

## 6. O que NUNCA funciona

    ConfirmDialogTrigger dentro de slot!(move || ...)  — runtime quebra apos re-render
    ConfirmDialogTrigger dentro de For sem key estavel — idem
    confirm_dialog_open() via JS                       — nao existe no window
    CustomEvent rs:confirm-dialog:open no elemento     — runtime nao escuta
    CustomEvent rs:confirm-dialog:open no document     — runtime nao escuta
    data-* custom em Button                            — canonrs remove atributos
    Closure::once em on:click dentro de slot reativo   — morre apos primeiro uso
    projects.update + version.update juntos            — double render

---

## 7. Modelo mental

    React:   state controla UI
    CanonRS: DOM controla UI, state e auxiliar

    React:   open={bool} — dialog controlado por state
    CanonRS: click no trigger — dialog controlado pelo runtime

    React:   re-render e seguro
    CanonRS: re-render dentro de runtime DOM-driven destrói bindings

---

## 8. Checklist

    [ ] Dialog declarado fora de qualquer slot! ou For
    [ ] Trigger fora de slot! reativo
    [ ] project_row e funcao normal com StoredValue para IDs
    [ ] For usa key estavel (id do item)
    [ ] delete Action retorna ID para update local
    [ ] Effect usa version().get() como dependencia
    [ ] Nao mistura update local com refetch

## Referencia

    products/core-scraper/core-scraper-frontend-leptos/src/pages/projects.rs
    products/seo-agent/seo-agent-frontend-leptos/src/pages/projects.rs
