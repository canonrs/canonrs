# CanonRS Interaction Data — Guia de Referência

## Arquitetura

    init_data(el)
      → data_table::init  — [data-rs-datatable]
      → virtual_list::init — [data-rs-virtual-list]
      → list_item::init   — [data-rs-list]
      → chart::init       — [data-rs-chart]

Todo engine começa com:

    if !lifecycle::init_guard(&root) { return; }

---

## runtime/context

    use crate::runtime::context;

    context::find_root(&target, "[data-rs-datatable]");
    context::propagate_owner(&root);

find_root: busca o root pelo closest() primeiro, depois por data-rs-owner → data-rs-uid.
propagate_owner: propaga data-rs-uid do root para todos os filhos via data-rs-owner.
Usar quando eventos bubblam de filhos que precisam encontrar o root.

---

## engines/chart_engine

    use crate::engines::chart_engine::{read_chart_data, parse_chart_data, set_canvas_dpi, draw_chart, Series};

    pub type Series = Vec<(String, Vec<f64>, String, bool)>;
    // (name, data, color, active)

    read_chart_data(&root)         // le data-rs-chart-data do root
    parse_chart_data(&json)        // Option<(Vec<String>, Series)>
    set_canvas_dpi(&canvas, &root, height)  // ajusta DPR e dimensoes
    draw_chart(&canvas, "line", &labels, &series, show_grid, height)
    // chart_type: "line" | "bar"

---

## data_table

Funcionalidades do engine:

    bind_filter          — search input filtra rows em tempo real
    bind_sort            — click no header ordena colunas
    bind_pagination      — prev/next, esconde rows alem da pagina
    bind_density         — compact/comfortable/spacious
    bind_col_toggle      — mostrar/esconder colunas
    bind_selection       — click/shift+click/ctrl+click/select-all
    bind_bulk_bar        — aparece quando ha selecao
    bind_bulk_actions    — dispara rs-datatable-bulk-action
    bind_row_actions     — dispara rs-datatable-action
    bind_context_menu    — rightclick na row
    bind_column_resize   — drag no handle do header (injetado via DOM)

DOM como fonte de verdade:

    data-rs-selected-ids       — ids selecionados separados por virgula
    data-rs-current-row        — row_id da ultima action
    data-rs-current-label      — label da ultima action
    data-rs-current-action     — action clicada
    data-rs-current-bulk-action — bulk action clicada
    data-rs-sort-col           — coluna de sort ativa
    data-rs-sort-asc           — direcao do sort
    data-rs-current-page       — pagina atual
    data-rs-page-size          — itens por pagina
    data-rs-total-pages        — total de paginas
    data-rs-selectable         — "true" se selecao ativa
    data-rs-density            — compact/comfortable/spacious

Eventos disparados:

    rs-datatable-action        — detail: { action, rowId, label }
    rs-datatable-bulk-action   — detail: { action, ids[] }
    rs-selection-change        — detail: { ids[], count }

Padrao de escuta na pagina:

    // escutar uma vez no mount
    root.add_event_listener("rs-datatable-action", |e| {
        let action = e.detail().action;
        let row_id = e.detail().rowId;
        match action { "edit" => ..., "delete" => ..., _ => {} }
    });

row_id_fn:

    // padrao: usa indice como id
    // para id real do item:
    <DataTable row_id_fn=Arc::new(|s: &Source| s.id.to_string()) />

NUNCA fazer:

    - RwSignal para linha selecionada — usar data-rs-selected-ids
    - on:click por linha para capturar id — usar rs-datatable-action
    - registrar listener dentro de Effect — registrar uma vez no mount

---

## virtual_list

    bind_scroll     — virtualiza rendering de listas longas
    bind_resize     — recalcula quando container redimensiona

DOM:

    data-rs-virtual-list       — root
    data-rs-virtual-item       — item renderizado
    data-rs-item-height        — altura fixa do item em px
    data-rs-total-items        — total de items

---

## list_item

    bind_selection  — ativa item clicado, desativa outros
    bind_keyboard   — Arrow Up/Down, Home, End, Enter

DOM:

    data-rs-list               — root
    data-rs-list-item          — item
    data-rs-value              — valor do item

---

## chart

    bind_legend      — toggle de series via click
    bind_tooltip     — hover mostra valores no crosshair
    bind_resize      — redraw quando container redimensiona
    bind_datatable_sync — sincroniza highlight com datatable

DOM:

    data-rs-chart              — root
    data-rs-chart-type         — "line" | "bar"
    data-rs-chart-data         — JSON com labels e series
    data-rs-chart-height       — altura em px
    data-rs-chart-grid         — "hidden" para ocultar grid
    data-rs-chart-legend       — "hidden" para ocultar legenda
    data-rs-chart-sync-table   — id do datatable para sincronizar
    data-rs-chart-canvas       — elemento canvas
    data-rs-chart-tooltip      — elemento tooltip
    data-rs-chart-crosshair    — elemento crosshair
    data-rs-chart-legend       — elemento legenda

Eventos disparados:

    canon:chart:hover  — detail: { index }
    canon:chart:leave  — sem detail

---

## Template minimo de engine data

    //! MeuEngine Interaction
    //! Core: dom/{lifecycle, state, query, attrs}

    use canonrs_interactions_core::dom::{lifecycle, state, query, attrs};
    use crate::runtime::context;

    pub fn init(root: web_sys::Element) {
        if !lifecycle::init_guard(&root) { return; }

        bind_events(&root);
    }

    fn bind_events(root: &web_sys::HtmlElement) {
        // usar context::find_root para eventos que bubblam
        // usar attrs::get_usize para ler configuracao do DOM
        // usar state::add/remove para estado
        // disparar CustomEvent para comunicacao com a pagina
    }

---

## Proibido

    - usar crate::shared — usar canonrs_interactions_core diretamente
    - duplicar is_initialized/mark_initialized — usar lifecycle::init_guard
    - reimplementar find_root — usar context::find_root
    - signals ou estado reativo — DOM e a fonte de verdade
    - registrar listener dentro de Effect — registrar uma vez no init
