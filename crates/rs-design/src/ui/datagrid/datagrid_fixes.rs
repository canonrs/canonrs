// FIX 1: Filterable section (around line 242-255)
// ANTES:
{if is_filterable {
    view! { <input ... /> }
} else {
    view! { <></> }
}}

// DEPOIS:
{if is_filterable {
    view! { <input ... /> }.into_any()
} else {
    view! { <></> }.into_any()
}}

// FIX 2: Pagination section (around line 280-293)
// ANTES:
if !cfg.pagination {
    view! { <div>...</div> }.into_any()
} else {
    render_table_body(...).into_any()
}

// DEPOIS:
if !cfg.pagination {
    view! { <div>...</div> }.into_any()
} else {
    view! { {render_table_body(...)} }.into_any()
}

// FIX 3: render_table_body closure (around line 376)
// ANTES:
{rows.into_iter().enumerate().map(|(rel_idx, row)| {
    ...
    class=move || {
        if config.hoverable { ... }  // config moved here
    }

// DEPOIS:
{rows.into_iter().enumerate().map(|(rel_idx, row)| {
    let cfg = Arc::clone(&config);  // Clone before closure
    ...
    class=move || {
        if cfg.hoverable { ... }  // Use cfg instead
    }
