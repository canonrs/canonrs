use leptos::prelude::*;
use super::dropdown_menu_ui::*;

// Exemplo 1: Dropdown normal com items
#[component]
pub fn BasicExample() -> impl IntoView {
    let (last_clicked, set_last_clicked) = signal("None".to_string());
    
    view! {
        <div>
            <DropdownMenu id="dropdown-ex".to_string()>
                <DropdownMenuTrigger
                    target_dropdown_id="dropdown-ex".to_string()
                    id="dropdown-trigger-ex".to_string()
                >
                    "Options ▼"
                </DropdownMenuTrigger>
                <DropdownMenuContent>
                    <DropdownMenuGroup>
                        <DropdownMenuLabel>"Account"</DropdownMenuLabel>
                        <div 
                            data-dropdown-menu-item=""
                            on:click={move |_| set_last_clicked.set("Profile".to_string())}
                        >
                            "Profile"
                        </div>
                        <div 
                            data-dropdown-menu-item=""
                            on:click={move |_| set_last_clicked.set("Settings".to_string())}
                        >
                            "Settings"
                        </div>
                    </DropdownMenuGroup>
                    <DropdownMenuSeparator />
                    <div 
                        data-dropdown-menu-item=""
                        on:click={move |_| set_last_clicked.set("Logout".to_string())}
                    >
                        "Logout"
                    </div>
                </DropdownMenuContent>
            </DropdownMenu>
            
            <p style="margin-top: 1rem;">
                "Last clicked: " {move || last_clicked.get()}
            </p>
        </div>
    }
}

// Exemplo 2: Dropdown com checkboxes
#[component]
pub fn CheckboxExample() -> impl IntoView {
    let (selected, set_selected) = signal(vec![true, true, false]);
    
    view! {
        <div>
            <DropdownMenu id="dropdown-checkbox-ex".to_string()>
                <DropdownMenuTrigger
                    target_dropdown_id="dropdown-checkbox-ex".to_string()
                    id="dropdown-checkbox-trigger-ex".to_string()
                >
                    "Columns ▼"
                </DropdownMenuTrigger>
                <DropdownMenuContent>
                    <DropdownMenuGroup>
                        <div
                            data-dropdown-menu-checkbox-item=""
                            aria-checked={move || if selected.get()[0] { "true" } else { "false" }}
                            on:click={move |_| set_selected.update(|s| s[0] = !s[0])}
                        >
                            "✓ ID"
                        </div>
                        <div
                            data-dropdown-menu-checkbox-item=""
                            aria-checked={move || if selected.get()[1] { "true" } else { "false" }}
                            on:click={move |_| set_selected.update(|s| s[1] = !s[1])}
                        >
                            "✓ Name"
                        </div>
                        <div
                            data-dropdown-menu-checkbox-item=""
                            aria-checked={move || if selected.get()[2] { "true" } else { "false" }}
                            on:click={move |_| set_selected.update(|s| s[2] = !s[2])}
                        >
                            "✓ Email"
                        </div>
                    </DropdownMenuGroup>
                </DropdownMenuContent>
            </DropdownMenu>
            
            <p style="margin-top: 1rem;">
                "Selected: " 
                {move || format!("ID: {}, Name: {}, Email: {}", 
                    selected.get()[0], 
                    selected.get()[1], 
                    selected.get()[2]
                )}
            </p>
        </div>
    }
}
