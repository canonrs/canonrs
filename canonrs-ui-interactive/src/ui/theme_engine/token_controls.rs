use leptos::prelude::*;
use super::{ThemeState, ThemeTokens, HslColor, ActiveMode};
use canonrs_ui::ui::tabs::{Tabs, TabsList, TabsTrigger, TabsContent};
use canonrs_ui::ui::button::{Button, ButtonVariant, ButtonSize};

fn section(label: &'static str) -> impl IntoView {
    view! {
        <div style="font-size:0.65rem;font-weight:700;text-transform:uppercase;letter-spacing:0.1em;color:var(--theme-surface-fg-muted);padding:0.6rem 1rem 0.3rem;border-top:1px solid var(--theme-surface-border);margin-top:0.25rem;">
            {label}
        </div>
    }
}

fn color_token<GL, SL, GD, SD>(
    label: &'static str,
    gl: GL, sl: SL,
    gd: GD, sd: SD,
    theme: ThemeState,
) -> impl IntoView
where
    GL: Fn(&ThemeTokens) -> HslColor + Copy + Send + Sync + 'static,
    SL: Fn(&mut ThemeTokens, HslColor) + Copy + Send + Sync + 'static,
    GD: Fn(&ThemeTokens) -> HslColor + Copy + Send + Sync + 'static,
    SD: Fn(&mut ThemeTokens, HslColor) + Copy + Send + Sync + 'static,
{
    let a1 = theme.active.clone(); let a2 = theme.active.clone(); let a3 = theme.active.clone();
    let a4 = theme.active.clone(); let a5 = theme.active.clone(); let a6 = theme.active.clone();
    let a7 = theme.active.clone(); let a8 = theme.active.clone(); let a9 = theme.active.clone();
    let t1 = theme.clone(); let t2 = theme.clone(); let t3 = theme.clone();
    let t4 = theme.clone(); let t5 = theme.clone(); let t6 = theme.clone();
    let t7 = theme.clone(); let t8 = theme.clone(); let t9 = theme.clone();

    let get_h  = move || { if a1.get() == ActiveMode::Light { gl(&t1.light.get()).h as f64 } else { gd(&t1.dark.get()).h as f64 } };
    let get_h2 = move || { if a2.get() == ActiveMode::Light { gl(&t2.light.get()).h as f64 } else { gd(&t2.dark.get()).h as f64 } };
    let get_s  = move || { if a3.get() == ActiveMode::Light { gl(&t3.light.get()).s as f64 } else { gd(&t3.dark.get()).s as f64 } };
    let get_s2 = move || { if a4.get() == ActiveMode::Light { gl(&t4.light.get()).s as f64 } else { gd(&t4.dark.get()).s as f64 } };
    let get_l  = move || { if a5.get() == ActiveMode::Light { gl(&t5.light.get()).l as f64 } else { gd(&t5.dark.get()).l as f64 } };
    let get_l2 = move || { if a6.get() == ActiveMode::Light { gl(&t6.light.get()).l as f64 } else { gd(&t6.dark.get()).l as f64 } };

    let update_h = move |v: f64| { if a7.get() == ActiveMode::Light { t7.light.try_update(|s| { let mut c = gl(s); c.h = v as f32; sl(s, c); }); } else { t7.dark.try_update(|s| { let mut c = gd(s); c.h = v as f32; sd(s, c); }); } };
    let update_s = move |v: f64| { if a8.get() == ActiveMode::Light { t8.light.try_update(|s| { let mut c = gl(s); c.s = v as f32; sl(s, c); }); } else { t8.dark.try_update(|s| { let mut c = gd(s); c.s = v as f32; sd(s, c); }); } };
    let update_l = move |v: f64| { if a9.get() == ActiveMode::Light { t9.light.try_update(|s| { let mut c = gl(s); c.l = v as f32; sl(s, c); }); } else { t9.dark.try_update(|s| { let mut c = gd(s); c.l = v as f32; sd(s, c); }); } };

    view! {
        <div style="padding:0.4rem 1rem;border-bottom:1px solid var(--theme-surface-border);">
            <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:0.35rem;">
                <span style="font-size:0.7rem;font-family:monospace;color:var(--theme-surface-fg-muted);">{label}</span>
                <div style=format!("width:1.5rem;height:1.5rem;border-radius:3px;background:var(--theme-{});border:1px solid var(--theme-surface-border);", label)></div>
            </div>
            <div style="display:grid;grid-template-columns:0.8rem 1fr 2.5rem;align-items:center;gap:0.25rem;margin-bottom:0.2rem;">
                <span style="font-size:0.6rem;color:var(--theme-surface-fg-muted);">"H"</span>
                <input type="range" min="0" max="360" step="0.5"
                    prop:value=move || get_h()
                    style="width:100%;accent-color:var(--theme-action-primary-bg);cursor:pointer;"
                    on:input=move |ev| update_h(event_target_value(&ev).parse::<f64>().unwrap_or(0.0))
                />
                <span style="font-size:0.6rem;font-family:monospace;text-align:right;">{move || format!("{:.0}", get_h2())}</span>
            </div>
            <div style="display:grid;grid-template-columns:0.8rem 1fr 2.5rem;align-items:center;gap:0.25rem;margin-bottom:0.2rem;">
                <span style="font-size:0.6rem;color:var(--theme-surface-fg-muted);">"S"</span>
                <input type="range" min="0" max="100" step="0.5"
                    prop:value=move || get_s()
                    style="width:100%;accent-color:var(--theme-action-primary-bg);cursor:pointer;"
                    on:input=move |ev| update_s(event_target_value(&ev).parse::<f64>().unwrap_or(0.0))
                />
                <span style="font-size:0.6rem;font-family:monospace;text-align:right;">{move || format!("{:.1}", get_s2())}</span>
            </div>
            <div style="display:grid;grid-template-columns:0.8rem 1fr 2.5rem;align-items:center;gap:0.25rem;">
                <span style="font-size:0.6rem;color:var(--theme-surface-fg-muted);">"L"</span>
                <input type="range" min="0" max="100" step="0.5"
                    prop:value=move || get_l()
                    style="width:100%;accent-color:var(--theme-action-primary-bg);cursor:pointer;"
                    on:input=move |ev| update_l(event_target_value(&ev).parse::<f64>().unwrap_or(0.0))
                />
                <span style="font-size:0.6rem;font-family:monospace;text-align:right;">{move || format!("{:.1}", get_l2())}</span>
            </div>
        </div>
    }
}

#[component]
pub fn TokenControls(theme: ThemeState) -> impl IntoView {
    let t_light = theme.clone();
    let t_dark = theme.clone();
    let t_radius_display = theme.clone();
    let t_radius_prop = theme.clone();
    let t_radius_input = theme.clone();

    view! {
        <div style="padding-bottom:2rem;">
            <div style="display:flex;gap:0.4rem;padding:0.5rem 1rem;border-bottom:1px solid var(--theme-surface-border);">
                <Button variant=ButtonVariant::Solid size=ButtonSize::Sm on:click=move |_| { t_light.active.try_update(|a| *a = ActiveMode::Light); }>
                    "☀ Light"
                </Button>
                <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm on:click=move |_| { t_dark.active.try_update(|a| *a = ActiveMode::Dark); }>
                    "🌙 Dark"
                </Button>
            </div>

            <Tabs id="token-tabs" default="colors">
                <TabsList>
                    <TabsTrigger id="tab-colors" value="colors">"🎨 Colors"</TabsTrigger>
                    <TabsTrigger id="tab-tokens" value="tokens">"⚙ Tokens"</TabsTrigger>
                </TabsList>
                <TabsContent value="colors">
                    {section("Surfaces")}
                    {color_token("surface-bg", |s| s.surface_bg.clone(), |s,v| s.surface_bg=v, |s| s.surface_bg.clone(), |s,v| s.surface_bg=v, theme.clone())}
                    {color_token("surface-fg", |s| s.surface_fg.clone(), |s,v| s.surface_fg=v, |s| s.surface_fg.clone(), |s,v| s.surface_fg=v, theme.clone())}
                    {color_token("surface-elevated", |s| s.surface_elevated.clone(), |s,v| s.surface_elevated=v, |s| s.surface_elevated.clone(), |s,v| s.surface_elevated=v, theme.clone())}
                    {color_token("surface-border", |s| s.surface_border.clone(), |s,v| s.surface_border=v, |s| s.surface_border.clone(), |s,v| s.surface_border=v, theme.clone())}
                    {color_token("surface-fg-muted", |s| s.surface_fg_muted.clone(), |s,v| s.surface_fg_muted=v, |s| s.surface_fg_muted.clone(), |s,v| s.surface_fg_muted=v, theme.clone())}
                    {section("Actions")}
                    {color_token("action-primary-bg", |s| s.action_primary_bg.clone(), |s,v| s.action_primary_bg=v, |s| s.action_primary_bg.clone(), |s,v| s.action_primary_bg=v, theme.clone())}
                    {color_token("action-primary-fg", |s| s.action_primary_fg.clone(), |s,v| s.action_primary_fg=v, |s| s.action_primary_fg.clone(), |s,v| s.action_primary_fg=v, theme.clone())}
                    {color_token("action-secondary-bg", |s| s.action_secondary_bg.clone(), |s,v| s.action_secondary_bg=v, |s| s.action_secondary_bg.clone(), |s,v| s.action_secondary_bg=v, theme.clone())}
                    {color_token("action-accent-bg", |s| s.action_accent_bg.clone(), |s,v| s.action_accent_bg=v, |s| s.action_accent_bg.clone(), |s,v| s.action_accent_bg=v, theme.clone())}
                    {color_token("action-focus-ring", |s| s.action_focus_ring.clone(), |s,v| s.action_focus_ring=v, |s| s.action_focus_ring.clone(), |s,v| s.action_focus_ring=v, theme.clone())}
                    {section("States")}
                    {color_token("state-success-bg", |s| s.state_success_bg.clone(), |s,v| s.state_success_bg=v, |s| s.state_success_bg.clone(), |s,v| s.state_success_bg=v, theme.clone())}
                    {color_token("state-success-border", |s| s.state_success_border.clone(), |s,v| s.state_success_border=v, |s| s.state_success_border.clone(), |s,v| s.state_success_border=v, theme.clone())}
                    {color_token("state-warning-bg", |s| s.state_warning_bg.clone(), |s,v| s.state_warning_bg=v, |s| s.state_warning_bg.clone(), |s,v| s.state_warning_bg=v, theme.clone())}
                    {color_token("state-warning-border", |s| s.state_warning_border.clone(), |s,v| s.state_warning_border=v, |s| s.state_warning_border.clone(), |s,v| s.state_warning_border=v, theme.clone())}
                    {color_token("state-error-bg", |s| s.state_error_bg.clone(), |s,v| s.state_error_bg=v, |s| s.state_error_bg.clone(), |s,v| s.state_error_bg=v, theme.clone())}
                    {color_token("state-error-border", |s| s.state_error_border.clone(), |s,v| s.state_error_border=v, |s| s.state_error_border.clone(), |s,v| s.state_error_border=v, theme.clone())}
                    {section("Charts")}
                    {color_token("chart-1", |s| s.chart_1.clone(), |s,v| s.chart_1=v, |s| s.chart_1.clone(), |s,v| s.chart_1=v, theme.clone())}
                    {color_token("chart-2", |s| s.chart_2.clone(), |s,v| s.chart_2=v, |s| s.chart_2.clone(), |s,v| s.chart_2=v, theme.clone())}
                    {color_token("chart-3", |s| s.chart_3.clone(), |s,v| s.chart_3=v, |s| s.chart_3.clone(), |s,v| s.chart_3=v, theme.clone())}
                    {color_token("chart-4", |s| s.chart_4.clone(), |s,v| s.chart_4=v, |s| s.chart_4.clone(), |s,v| s.chart_4=v, theme.clone())}
                    {color_token("chart-5", |s| s.chart_5.clone(), |s,v| s.chart_5=v, |s| s.chart_5.clone(), |s,v| s.chart_5=v, theme.clone())}
                    {section("Sidebar")}
                    {color_token("sidebar-bg", |s| s.sidebar_bg.clone(), |s,v| s.sidebar_bg=v, |s| s.sidebar_bg.clone(), |s,v| s.sidebar_bg=v, theme.clone())}
                    {color_token("sidebar-primary-bg", |s| s.sidebar_primary_bg.clone(), |s,v| s.sidebar_primary_bg=v, |s| s.sidebar_primary_bg.clone(), |s,v| s.sidebar_primary_bg=v, theme.clone())}
                    {color_token("sidebar-accent-bg", |s| s.sidebar_accent_bg.clone(), |s,v| s.sidebar_accent_bg=v, |s| s.sidebar_accent_bg.clone(), |s,v| s.sidebar_accent_bg=v, theme.clone())}
                </TabsContent>
                <TabsContent value="tokens">
                    {section("Shape")}
                    <div style="padding:0.5rem 1rem;">
                        <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:0.3rem;">
                            <span style="font-size:0.7rem;font-family:monospace;color:var(--theme-surface-fg-muted);">"--radius"</span>
                            <span style="font-size:0.7rem;font-family:monospace;">{move || format!("{:.3}rem", t_radius_display.radius.get())}</span>
                        </div>
                        <input type="range" min="0" max="1.5" step="0.025"
                            prop:value=move || t_radius_prop.radius.get() as f64
                            style="width:100%;accent-color:var(--theme-action-primary-bg);cursor:pointer;"
                            on:input=move |ev| {
                                let v = event_target_value(&ev).parse::<f64>().unwrap_or(0.375);
                                t_radius_input.radius.try_update(|r| *r = v as f32);
                            }
                        />
                    </div>
                </TabsContent>
            </Tabs>
        </div>
    }
}
