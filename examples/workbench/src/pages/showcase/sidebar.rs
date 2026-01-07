use leptos::prelude::*;
use rs_design::*;
use rs_design::use_theme;

#[component]
pub fn ComponentsSidebar() -> impl IntoView {
    let tab_value = RwSignal::new("colors".to_string());

    view! {
        <Sidebar>
            <SidebarHeader>
                <h2 class="px-3 py-2 text-lg font-semibold">Theme Tokens</h2>
            </SidebarHeader>

            <SidebarContent>
                <Tabs value=tab_value>
                    <TabsList class="grid grid-cols-4 w-full mx-3 mb-4">
                        <TabsTrigger value="colors">"Colors"</TabsTrigger>
                        <TabsTrigger value="radius">"Radius"</TabsTrigger>
                        <TabsTrigger value="shadows">"Shadows"</TabsTrigger>
                        <TabsTrigger value="typo">"Typography"</TabsTrigger>
                    </TabsList>
                    
                    <TabsContent value="colors">
                        <ColorsTab />
                    </TabsContent>
                    
                    <TabsContent value="radius">
                        <RadiusTab />
                    </TabsContent>
                    
                    <TabsContent value="shadows">
                        <ShadowsTab />
                    </TabsContent>
                    
                    <TabsContent value="typo">
                        <TypographyTab />
                    </TabsContent>
                </Tabs>
            </SidebarContent>

            <SidebarFooter>
                <div class="px-3 py-2 text-xs text-muted-foreground">
                    RS-Design Theme System
                </div>
            </SidebarFooter>
        </Sidebar>
    }
}

#[component]
fn ColorsTab() -> impl IntoView {
    let theme = use_theme();
    
    // Signals para TODAS as cores
    let primary = RwSignal::new("#6366f1".to_string());
    let primary_fg = RwSignal::new("#ffffff".to_string());
    let secondary = RwSignal::new("#64748b".to_string());
    let secondary_fg = RwSignal::new("#ffffff".to_string());
    let destructive = RwSignal::new("#ef4444".to_string());
    let destructive_fg = RwSignal::new("#ffffff".to_string());
    let muted = RwSignal::new("#f1f5f9".to_string());
    let muted_fg = RwSignal::new("#64748b".to_string());
    let card = RwSignal::new("#ffffff".to_string());
    let card_fg = RwSignal::new("#0f172a".to_string());
    let accent = RwSignal::new("#f59e0b".to_string());
    let accent_fg = RwSignal::new("#ffffff".to_string());
    
    // Atualizar TODAS as cores quando tema mudar
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        let _ = theme.preset.get();
        let _ = theme.mode.get();
        
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Some(html) = doc.document_element() {
                    if let Ok(Some(style)) = window.get_computed_style(&html) {
                        // Atualizar TODAS as cores
                        if let Ok(val) = style.get_property_value("--color-primary") {
                            if !val.is_empty() { primary.set(hsl_to_hex(&val)); }
                        }
                        if let Ok(val) = style.get_property_value("--color-primary-foreground") {
                            if !val.is_empty() { primary_fg.set(hsl_to_hex(&val)); }
                        }
                        if let Ok(val) = style.get_property_value("--color-secondary") {
                            if !val.is_empty() { secondary.set(hsl_to_hex(&val)); }
                        }
                        if let Ok(val) = style.get_property_value("--color-secondary-foreground") {
                            if !val.is_empty() { secondary_fg.set(hsl_to_hex(&val)); }
                        }
                        if let Ok(val) = style.get_property_value("--color-destructive") {
                            if !val.is_empty() { destructive.set(hsl_to_hex(&val)); }
                        }
                        if let Ok(val) = style.get_property_value("--color-destructive-foreground") {
                            if !val.is_empty() { destructive_fg.set(hsl_to_hex(&val)); }
                        }
                        if let Ok(val) = style.get_property_value("--color-muted") {
                            if !val.is_empty() { muted.set(hsl_to_hex(&val)); }
                        }
                        if let Ok(val) = style.get_property_value("--color-muted-foreground") {
                            if !val.is_empty() { muted_fg.set(hsl_to_hex(&val)); }
                        }
                        if let Ok(val) = style.get_property_value("--color-card") {
                            if !val.is_empty() { card.set(hsl_to_hex(&val)); }
                        }
                        if let Ok(val) = style.get_property_value("--color-card-foreground") {
                            if !val.is_empty() { card_fg.set(hsl_to_hex(&val)); }
                        }
                        if let Ok(val) = style.get_property_value("--color-accent") {
                            if !val.is_empty() { accent.set(hsl_to_hex(&val)); }
                        }
                        if let Ok(val) = style.get_property_value("--color-accent-foreground") {
                            if !val.is_empty() { accent_fg.set(hsl_to_hex(&val)); }
                        }
                    }
                }
            }
        }
    });
    
    view! {
        <div class="px-3 space-y-6">
            <SidebarGroup>
                <SidebarGroupLabel>"Primary Colors"</SidebarGroupLabel>
                <SidebarGroupContent>
                    <div class="space-y-2">
                        <ColorPicker label="Primary" value=primary />
                        <ColorPicker label="Primary Foreground" value=primary_fg />
                    </div>
                </SidebarGroupContent>
            </SidebarGroup>
            
            <SidebarGroup>
                <SidebarGroupLabel>"Secondary Colors"</SidebarGroupLabel>
                <SidebarGroupContent>
                    <div class="space-y-2">
                        <ColorPicker label="Secondary" value=secondary />
                        <ColorPicker label="Secondary Foreground" value=secondary_fg />
                    </div>
                </SidebarGroupContent>
            </SidebarGroup>
            
            <SidebarGroup>
                <SidebarGroupLabel>"Destructive Colors"</SidebarGroupLabel>
                <SidebarGroupContent>
                    <div class="space-y-2">
                        <ColorPicker label="Destructive" value=destructive />
                        <ColorPicker label="Destructive Foreground" value=destructive_fg />
                    </div>
                </SidebarGroupContent>
            </SidebarGroup>
            
            <SidebarGroup>
                <SidebarGroupLabel>"Muted Colors"</SidebarGroupLabel>
                <SidebarGroupContent>
                    <div class="space-y-2">
                        <ColorPicker label="Muted" value=muted />
                        <ColorPicker label="Muted Foreground" value=muted_fg />
                    </div>
                </SidebarGroupContent>
            </SidebarGroup>
            
            <SidebarGroup>
                <SidebarGroupLabel>"Card Colors"</SidebarGroupLabel>
                <SidebarGroupContent>
                    <div class="space-y-2">
                        <ColorPicker label="Card" value=card />
                        <ColorPicker label="Card Foreground" value=card_fg />
                    </div>
                </SidebarGroupContent>
            </SidebarGroup>
            
            <SidebarGroup>
                <SidebarGroupLabel>"Accent Colors"</SidebarGroupLabel>
                <SidebarGroupContent>
                    <div class="space-y-2">
                        <ColorPicker label="Accent" value=accent />
                        <ColorPicker label="Accent Foreground" value=accent_fg />
                    </div>
                </SidebarGroupContent>
            </SidebarGroup>
        </div>
    }
}

fn hsl_to_hex(hsl: &str) -> String {
    let parts: Vec<&str> = hsl.trim().split_whitespace().collect();
    if parts.len() != 3 {
        return "#000000".to_string();
    }
    
    let h: f64 = parts[0].parse().unwrap_or(0.0);
    let s: f64 = parts[1].trim_end_matches('%').parse().unwrap_or(0.0) / 100.0;
    let l: f64 = parts[2].trim_end_matches('%').parse().unwrap_or(0.0) / 100.0;
    
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    
    let (r1, g1, b1) = if h_prime >= 0.0 && h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime >= 1.0 && h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime >= 2.0 && h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime >= 3.0 && h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime >= 4.0 && h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    
    let r = ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    let g = ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    let b = ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

#[component]
fn RadiusTab() -> impl IntoView {
    view! {
        <div class="px-3 space-y-4">
            <div class="text-xs text-muted-foreground mb-2">"SM"</div>
            <div class="w-full h-16 bg-primary rounded-sm"></div>
            <div class="text-xs text-muted-foreground mb-2">"MD"</div>
            <div class="w-full h-16 bg-primary rounded-md"></div>
            <div class="text-xs text-muted-foreground mb-2">"LG"</div>
            <div class="w-full h-16 bg-primary rounded-lg"></div>
        </div>
    }
}

#[component]
fn ShadowsTab() -> impl IntoView {
    view! {
        <div class="px-3 space-y-4">
            <div class="text-xs text-muted-foreground mb-2">"SM"</div>
            <div class="w-full h-16 bg-card border rounded shadow-sm"></div>
            <div class="text-xs text-muted-foreground mb-2">"MD"</div>
            <div class="w-full h-16 bg-card border rounded shadow-md"></div>
            <div class="text-xs text-muted-foreground mb-2">"LG"</div>
            <div class="w-full h-16 bg-card border rounded shadow-lg"></div>
        </div>
    }
}

#[component]
fn TypographyTab() -> impl IntoView {
    view! {
        <div class="px-3 space-y-4">
            <div class="text-sm">"Small Text"</div>
            <div class="text-base">"Base Text"</div>
            <div class="text-lg">"Large Text"</div>
            <div class="pt-2 border-t">
                <div class="font-normal">"Normal"</div>
                <div class="font-medium">"Medium"</div>
                <div class="font-bold">"Bold"</div>
            </div>
        </div>
    }
}
