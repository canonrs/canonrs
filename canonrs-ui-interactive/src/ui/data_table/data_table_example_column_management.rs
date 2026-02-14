use leptos::prelude::*;
use canonrs_ui::ui::dropdown_menu::{DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuLabel, DropdownMenuSeparator};
use canonrs_ui::ui::button::{Button, ButtonVariant};
use super::{DataTableInteractive, ColumnDef, DataTableRequest, DataTableResponse, use_column_reorder, use_column_resize, use_column_pin, PinPosition};

#[derive(Clone, Debug, PartialEq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub price: f64,
    pub stock: u32,
    pub status: String,
    pub sku: String,
    pub brand: String,
    pub weight: f64,
}

#[component]
pub fn DataTableColumnManagementExample() -> impl IntoView {
    let visible_columns = RwSignal::new(vec![
        ("name",     true),
        ("category", true),
        ("price",    true),
        ("stock",    true),
        ("status",   true),
        ("sku",      true),
        ("brand",    true),
        ("weight",   true),
    ]);

    let density_mode      = RwSignal::new("comfortable");
    let zebra_mode        = RwSignal::new(false);
    let hover_mode        = RwSignal::new(true);
    let sticky_mode       = RwSignal::new(false);
    let column_drag_mode  = RwSignal::new(false);
    let column_resize_mode = RwSignal::new(false);
    let column_pin_mode   = RwSignal::new(false);

    let column_widths = RwSignal::new(std::collections::HashMap::from([
        ("name".to_string(),     200u32),
        ("category".to_string(), 130u32),
        ("price".to_string(),     90u32),
        ("stock".to_string(),     80u32),
        ("status".to_string(),   120u32),
        ("sku".to_string(),      110u32),
        ("brand".to_string(),    120u32),
        ("weight".to_string(),    90u32),
    ]));
    let pinned_columns = RwSignal::new(std::collections::HashMap::from([("name".to_string(), PinPosition::Left)]));

    let fetch_products = move |req: DataTableRequest| -> Result<DataTableResponse<Product>, String> {
        let mut all_products = vec![
            Product { id: 1,  name: "Laptop Pro".to_string(),         category: "Electronics".to_string(),  price: 1299.99, stock: 15,  status: "Active".to_string(),       sku: "LAP-001".to_string(), brand: "TechBrand".to_string(),  weight: 2.1  },
            Product { id: 2,  name: "Wireless Mouse".to_string(),      category: "Accessories".to_string(),  price: 29.99,   stock: 150, status: "Active".to_string(),       sku: "MOU-002".to_string(), brand: "ClickCo".to_string(),    weight: 0.1  },
            Product { id: 3,  name: "USB-C Cable".to_string(),         category: "Accessories".to_string(),  price: 12.99,   stock: 0,   status: "Out of Stock".to_string(), sku: "CAB-003".to_string(), brand: "CableMax".to_string(),   weight: 0.05 },
            Product { id: 4,  name: "Monitor 27\"".to_string(),        category: "Electronics".to_string(),  price: 349.99,  stock: 8,   status: "Active".to_string(),       sku: "MON-004".to_string(), brand: "ViewTech".to_string(),   weight: 5.2  },
            Product { id: 5,  name: "Keyboard Mechanical".to_string(), category: "Accessories".to_string(),  price: 89.99,   stock: 42,  status: "Active".to_string(),       sku: "KEY-005".to_string(), brand: "TypeMaster".to_string(), weight: 1.1  },
            Product { id: 6,  name: "Webcam HD".to_string(),           category: "Electronics".to_string(),  price: 79.99,   stock: 23,  status: "Active".to_string(),       sku: "CAM-006".to_string(), brand: "VisionPro".to_string(),  weight: 0.3  },
            Product { id: 7,  name: "Headphones".to_string(),          category: "Audio".to_string(),        price: 149.99,  stock: 67,  status: "Active".to_string(),       sku: "HDP-007".to_string(), brand: "SoundWave".to_string(),  weight: 0.4  },
            Product { id: 8,  name: "USB Hub".to_string(),             category: "Accessories".to_string(),  price: 49.99,   stock: 30,  status: "Active".to_string(),       sku: "HUB-008".to_string(), brand: "ConnectAll".to_string(), weight: 0.2  },
            Product { id: 9,  name: "SSD 1TB".to_string(),             category: "Storage".to_string(),      price: 89.99,   stock: 12,  status: "Active".to_string(),       sku: "SSD-009".to_string(), brand: "SpeedDisk".to_string(),  weight: 0.08 },
            Product { id: 10, name: "Mouse Pad".to_string(),           category: "Accessories".to_string(),  price: 19.99,   stock: 200, status: "Active".to_string(),       sku: "PAD-010".to_string(), brand: "DeskPro".to_string(),    weight: 0.3  },
            Product { id: 11, name: "Desk Lamp".to_string(),           category: "Office".to_string(),       price: 34.99,   stock: 5,   status: "Low Stock".to_string(),    sku: "LMP-011".to_string(), brand: "BrightSpace".to_string(), weight: 0.9 },
            Product { id: 12, name: "Standing Desk".to_string(),       category: "Furniture".to_string(),    price: 499.99,  stock: 3,   status: "Low Stock".to_string(),    sku: "DSK-012".to_string(), brand: "ErgoDesk".to_string(),   weight: 32.0 },
        ];

        if !req.filter_query.is_empty() {
            let query = req.filter_query.to_lowercase();
            all_products.retain(|p| {
                p.name.to_lowercase().contains(&query)     ||
                p.category.to_lowercase().contains(&query) ||
                p.status.to_lowercase().contains(&query)   ||
                p.sku.to_lowercase().contains(&query)      ||
                p.brand.to_lowercase().contains(&query)
            });
        }

        if let Some(ref col) = req.sort_column {
            all_products.sort_by(|a, b| {
                let cmp = match col.as_str() {
                    "name"     => a.name.cmp(&b.name),
                    "category" => a.category.cmp(&b.category),
                    "price"    => a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal),
                    "stock"    => a.stock.cmp(&b.stock),
                    "status"   => a.status.cmp(&b.status),
                    "sku"      => a.sku.cmp(&b.sku),
                    "brand"    => a.brand.cmp(&b.brand),
                    "weight"   => a.weight.partial_cmp(&b.weight).unwrap_or(std::cmp::Ordering::Equal),
                    _          => std::cmp::Ordering::Equal,
                };
                if req.sort_ascending { cmp } else { cmp.reverse() }
            });
        }

        let total = all_products.len();
        let start = (req.page - 1) * req.page_size;
        let end   = (start + req.page_size).min(total);
        let data  = all_products.into_iter().skip(start).take(end - start).collect();
        Ok(DataTableResponse { data, total, page: req.page, page_size: req.page_size })
    };

    let columns_signal = Signal::derive(move || {
        visible_columns.get().into_iter()
            .filter(|(_, visible)| *visible)
            .map(|(col_id, _)| match col_id {
                "name"     => ColumnDef::new("name",     "Product Name", |p: &Product| p.name.clone()),
                "category" => ColumnDef::new("category", "Category",     |p: &Product| p.category.clone()),
                "price"    => ColumnDef::new("price",    "Price",        |p: &Product| format!("${:.2}", p.price)),
                "stock"    => ColumnDef::new("stock",    "Stock",        |p: &Product| p.stock.to_string()),
                "status"   => ColumnDef::new("status",   "Status",       |p: &Product| p.status.clone()),
                "sku"      => ColumnDef::new("sku",      "SKU",          |p: &Product| p.sku.clone()),
                "brand"    => ColumnDef::new("brand",    "Brand",        |p: &Product| p.brand.clone()),
                "weight"   => ColumnDef::new("weight",   "Weight (kg)",  |p: &Product| format!("{:.2}", p.weight)),
                _          => ColumnDef::new(col_id, col_id, |_: &Product| String::new()),
            })
            .collect()
    });

    let toggle_column = move |col_id: &'static str| {
        visible_columns.update(|cols| {
            if let Some(pos) = cols.iter().position(|(id, _)| *id == col_id) {
                cols[pos].1 = !cols[pos].1;
            }
        });
    };

    use_column_reorder(
        "column-drag-container".to_string(),
        Signal::derive(move || column_drag_mode.get()),
        move |from_id: String, to_id: String| {
            visible_columns.update(|cols| {
                let from_pos = cols.iter().position(|(id, _)| *id == from_id.as_str());
                let to_pos   = cols.iter().position(|(id, _)| *id == to_id.as_str());
                if let (Some(from), Some(to)) = (from_pos, to_pos) {
                    let item = cols.remove(from);
                    cols.insert(to, item);
                }
            });
        },
    );

    use_column_resize(
        format!("{}-resize", "products-table"),
        Signal::derive(move || column_resize_mode.get()),
        move |col_id: String, width: u32| {
            column_widths.update(|widths| { widths.insert(col_id, width); });
        },
    );

    use_column_pin(
        format!("{}-resize", "products-table"),
        Signal::derive(move || column_pin_mode.get()),
        move |col_id: String, pin_pos: PinPosition| {
            pinned_columns.update(|pins| { pins.insert(col_id, pin_pos); });
        },
    );

    view! {
        <div class="space-y-4">
            <div class="flex flex-wrap gap-4 items-center p-4 border rounded">
                <div class="font-semibold">"Column Management"</div>

                <DropdownMenu>
                    <DropdownMenuTrigger target_dropdown_id="columns-dropdown".to_string()>
                        <Button variant=ButtonVariant::Outline>"Columns ▼"</Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent>
                        <DropdownMenuLabel>"Toggle Columns"</DropdownMenuLabel>
                        <DropdownMenuSeparator />
                        <div class="p-2 space-y-2">
                            {move || {
                                visible_columns.get().iter().map(|(col_id, visible)| {
                                    let id = *col_id;
                                    let is_checked = *visible;
                                    view! {
                                        <div class="flex items-center gap-2 px-2 py-1.5 hover:bg-muted rounded">
                                            <input
                                                type="checkbox"
                                                id=format!("col-{}", id)
                                                prop:checked=is_checked
                                                on:change=move |_| toggle_column(id)
                                                style="cursor: pointer;"
                                            />
                                            <label for=format!("col-{}", id) style="cursor: pointer;">{id}</label>
                                        </div>
                                    }
                                }).collect_view()
                            }}
                        </div>
                    </DropdownMenuContent>
                </DropdownMenu>

                <div class="flex gap-1">
                    {move || {
                        let d = density_mode.get();
                        view! {
                            <>
                                <div on:click=move |_| density_mode.set("compact")>
                                    <Button variant=if d == "compact" { ButtonVariant::Solid } else { ButtonVariant::Ghost }>"Compact"</Button>
                                </div>
                                <div on:click=move |_| density_mode.set("comfortable")>
                                    <Button variant=if d == "comfortable" { ButtonVariant::Solid } else { ButtonVariant::Ghost }>"Comfortable"</Button>
                                </div>
                                <div on:click=move |_| density_mode.set("spacious")>
                                    <Button variant=if d == "spacious" { ButtonVariant::Solid } else { ButtonVariant::Ghost }>"Spacious"</Button>
                                </div>
                            </>
                        }
                    }}
                </div>

                <div class="flex items-center gap-2">
                    <input type="checkbox" id="zebra-toggle"  prop:checked=move || zebra_mode.get()         on:change=move |_| zebra_mode.update(|v| *v = !*v)         style="cursor: pointer;" />
                    <label for="zebra-toggle"  style="cursor: pointer;">"Zebra"</label>
                </div>
                <div class="flex items-center gap-2">
                    <input type="checkbox" id="hover-toggle"  prop:checked=move || hover_mode.get()         on:change=move |_| hover_mode.update(|v| *v = !*v)         style="cursor: pointer;" />
                    <label for="hover-toggle"  style="cursor: pointer;">"Row Hover"</label>
                </div>
                <div class="flex items-center gap-2">
                    <input type="checkbox" id="sticky-toggle" prop:checked=move || sticky_mode.get()        on:change=move |_| sticky_mode.update(|v| *v = !*v)        style="cursor: pointer;" />
                    <label for="sticky-toggle" style="cursor: pointer;">"Sticky Header"</label>
                </div>
                <div class="flex items-center gap-2">
                    <input type="checkbox" id="drag-toggle"   prop:checked=move || column_drag_mode.get()   on:change=move |_| column_drag_mode.update(|v| *v = !*v)   style="cursor: pointer;" />
                    <label for="drag-toggle"   style="cursor: pointer;">"Column Drag"</label>
                </div>
                <div class="flex items-center gap-2">
                    <input type="checkbox" id="resize-toggle" prop:checked=move || column_resize_mode.get() on:change=move |_| column_resize_mode.update(|v| *v = !*v) style="cursor: pointer;" />
                    <label for="resize-toggle" style="cursor: pointer;">"Column Resize"</label>
                </div>
                <div class="flex items-center gap-2">
                    <input type="checkbox" id="pin-toggle"    prop:checked=move || column_pin_mode.get()    on:change=move |_| column_pin_mode.update(|v| *v = !*v)    style="cursor: pointer;" />
                    <label for="pin-toggle"    style="cursor: pointer;">"Column Pin"</label>
                </div>
            </div>

            <div style="overflow-x: auto; width: 100%;">
                <DataTableInteractive
                    columns=columns_signal
                    fetch_data=fetch_products
                    page_size=10
                    id="products-table"
                    density=Signal::derive(move || density_mode.get().to_string())
                    zebra=Signal::derive(move || zebra_mode.get())
                    row_hover=Signal::derive(move || hover_mode.get())
                    sticky_header=Signal::derive(move || sticky_mode.get())
                    draggable=Signal::derive(move || column_drag_mode.get())
                    resizable=Signal::derive(move || column_resize_mode.get())
                    pinnable=Signal::derive(move || column_pin_mode.get())
                    column_widths=column_widths
                    pinned_columns=pinned_columns
                />
            </div>
        </div>
    }
}
