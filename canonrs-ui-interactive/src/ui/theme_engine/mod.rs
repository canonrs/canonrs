pub mod theme_wizard;
pub mod token_controls;
pub mod live_preview;

use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HslColor {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

impl HslColor {
    pub fn new(h: f32, s: f32, l: f32) -> Self { Self { h, s, l } }
    pub fn to_css(&self) -> String { format!("hsl({} {}% {}%)", self.h, self.s, self.l) }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ThemeTokens {
    pub surface_bg:           HslColor,
    pub surface_fg:           HslColor,
    pub surface_elevated:     HslColor,
    pub surface_elevated_fg:  HslColor,
    pub surface_muted:        HslColor,
    pub surface_fg_muted:     HslColor,
    pub surface_border:       HslColor,
    pub action_primary_bg:    HslColor,
    pub action_primary_fg:    HslColor,
    pub action_secondary_bg:  HslColor,
    pub action_secondary_fg:  HslColor,
    pub action_accent_bg:     HslColor,
    pub action_accent_fg:     HslColor,
    pub action_focus_ring:    HslColor,
    pub state_error_bg:       HslColor,
    pub state_error_fg:       HslColor,
    pub state_error_border:   HslColor,
    pub state_success_bg:     HslColor,
    pub state_success_fg:     HslColor,
    pub state_success_border: HslColor,
    pub state_warning_bg:     HslColor,
    pub state_warning_fg:     HslColor,
    pub state_warning_border: HslColor,
    pub overlay_bg:           HslColor,
    pub overlay_fg:           HslColor,
    pub chart_1:              HslColor,
    pub chart_2:              HslColor,
    pub chart_3:              HslColor,
    pub chart_4:              HslColor,
    pub chart_5:              HslColor,
    pub sidebar_bg:           HslColor,
    pub sidebar_fg:           HslColor,
    pub sidebar_border:       HslColor,
    pub sidebar_primary_bg:   HslColor,
    pub sidebar_primary_fg:   HslColor,
    pub sidebar_accent_bg:    HslColor,
    pub sidebar_accent_fg:    HslColor,
}

impl ThemeTokens {
    pub fn to_css_vars(&self) -> Vec<(&'static str, String)> {
        vec![
            ("surface-bg",           self.surface_bg.to_css()),
            ("surface-fg",           self.surface_fg.to_css()),
            ("surface-elevated",     self.surface_elevated.to_css()),
            ("surface-elevated-fg",  self.surface_elevated_fg.to_css()),
            ("surface-muted",        self.surface_muted.to_css()),
            ("surface-fg-muted",     self.surface_fg_muted.to_css()),
            ("surface-border",       self.surface_border.to_css()),
            ("action-primary-bg",    self.action_primary_bg.to_css()),
            ("action-primary-fg",    self.action_primary_fg.to_css()),
            ("action-secondary-bg",  self.action_secondary_bg.to_css()),
            ("action-secondary-fg",  self.action_secondary_fg.to_css()),
            ("action-accent-bg",     self.action_accent_bg.to_css()),
            ("action-accent-fg",     self.action_accent_fg.to_css()),
            ("action-focus-ring",    self.action_focus_ring.to_css()),
            ("state-error-bg",       self.state_error_bg.to_css()),
            ("state-error-fg",       self.state_error_fg.to_css()),
            ("state-error-border",   self.state_error_border.to_css()),
            ("state-success-bg",     self.state_success_bg.to_css()),
            ("state-success-fg",     self.state_success_fg.to_css()),
            ("state-success-border", self.state_success_border.to_css()),
            ("state-warning-bg",     self.state_warning_bg.to_css()),
            ("state-warning-fg",     self.state_warning_fg.to_css()),
            ("state-warning-border", self.state_warning_border.to_css()),
            ("overlay-bg",           self.overlay_bg.to_css()),
            ("overlay-fg",           self.overlay_fg.to_css()),
            ("chart-1",              self.chart_1.to_css()),
            ("chart-2",              self.chart_2.to_css()),
            ("chart-3",              self.chart_3.to_css()),
            ("chart-4",              self.chart_4.to_css()),
            ("chart-5",              self.chart_5.to_css()),
            ("sidebar-bg",           self.sidebar_bg.to_css()),
            ("sidebar-fg",           self.sidebar_fg.to_css()),
            ("sidebar-border",       self.sidebar_border.to_css()),
            ("sidebar-primary-bg",   self.sidebar_primary_bg.to_css()),
            ("sidebar-primary-fg",   self.sidebar_primary_fg.to_css()),
            ("sidebar-accent-bg",    self.sidebar_accent_bg.to_css()),
            ("sidebar-accent-fg",    self.sidebar_accent_fg.to_css()),
        ]
    }

    pub fn default_light() -> Self {
        Self {
            surface_bg:          HslColor::new(0.0,   0.0,   100.0),
            surface_fg:          HslColor::new(0.0,   0.0,   14.90),
            surface_elevated:    HslColor::new(0.0,   0.0,   100.0),
            surface_elevated_fg: HslColor::new(0.0,   0.0,   14.90),
            surface_muted:       HslColor::new(210.0, 20.0,  98.04),
            surface_fg_muted:    HslColor::new(220.0, 8.94,  46.08),
            surface_border:      HslColor::new(220.0, 13.04, 90.98),
            action_primary_bg:   HslColor::new(37.69, 92.13, 50.20),
            action_primary_fg:   HslColor::new(0.0,   0.0,   0.0),
            action_secondary_bg: HslColor::new(220.0, 14.29, 95.88),
            action_secondary_fg: HslColor::new(215.0, 13.79, 34.12),
            action_accent_bg:    HslColor::new(48.0,  100.0, 96.08),
            action_accent_fg:    HslColor::new(22.73, 82.5,  31.37),
            action_focus_ring:   HslColor::new(37.69, 92.13, 50.20),
            state_error_bg:      HslColor::new(0.0,   84.24, 60.20),
            state_error_fg:      HslColor::new(0.0,   0.0,   100.0),
            state_error_border:  HslColor::new(0.0,   84.24, 45.0),
            state_success_bg:    HslColor::new(142.0, 76.0,  36.0),
            state_success_fg:    HslColor::new(0.0,   0.0,   100.0),
            state_success_border:HslColor::new(142.0, 76.0,  25.0),
            state_warning_bg:    HslColor::new(38.0,  92.0,  50.0),
            state_warning_fg:    HslColor::new(0.0,   0.0,   0.0),
            state_warning_border:HslColor::new(38.0,  92.0,  35.0),
            overlay_bg:          HslColor::new(0.0,   0.0,   100.0),
            overlay_fg:          HslColor::new(0.0,   0.0,   14.90),
            chart_1: HslColor::new(37.69, 92.13, 50.20),
            chart_2: HslColor::new(32.13, 94.62, 43.73),
            chart_3: HslColor::new(25.96, 90.48, 37.06),
            chart_4: HslColor::new(22.73, 82.5,  31.37),
            chart_5: HslColor::new(21.71, 77.78, 26.47),
            sidebar_bg:          HslColor::new(210.0, 20.0,  98.04),
            sidebar_fg:          HslColor::new(0.0,   0.0,   14.90),
            sidebar_border:      HslColor::new(220.0, 13.04, 90.98),
            sidebar_primary_bg:  HslColor::new(37.69, 92.13, 50.20),
            sidebar_primary_fg:  HslColor::new(0.0,   0.0,   100.0),
            sidebar_accent_bg:   HslColor::new(48.0,  100.0, 96.08),
            sidebar_accent_fg:   HslColor::new(22.73, 82.5,  31.37),
        }
    }

    pub fn default_dark() -> Self {
        Self {
            surface_bg:          HslColor::new(0.0,   0.0,   9.02),
            surface_fg:          HslColor::new(0.0,   0.0,   89.80),
            surface_elevated:    HslColor::new(0.0,   0.0,   14.90),
            surface_elevated_fg: HslColor::new(0.0,   0.0,   89.80),
            surface_muted:       HslColor::new(0.0,   0.0,   12.16),
            surface_fg_muted:    HslColor::new(0.0,   0.0,   63.92),
            surface_border:      HslColor::new(0.0,   0.0,   25.10),
            action_primary_bg:   HslColor::new(37.69, 92.13, 50.20),
            action_primary_fg:   HslColor::new(0.0,   0.0,   0.0),
            action_secondary_bg: HslColor::new(0.0,   0.0,   14.90),
            action_secondary_fg: HslColor::new(0.0,   0.0,   89.80),
            action_accent_bg:    HslColor::new(22.73, 82.5,  31.37),
            action_accent_fg:    HslColor::new(48.0,  96.64, 76.67),
            action_focus_ring:   HslColor::new(37.69, 92.13, 50.20),
            state_error_bg:      HslColor::new(0.0,   84.24, 60.20),
            state_error_fg:      HslColor::new(0.0,   0.0,   100.0),
            state_error_border:  HslColor::new(0.0,   84.24, 45.0),
            state_success_bg:    HslColor::new(142.0, 76.0,  36.0),
            state_success_fg:    HslColor::new(0.0,   0.0,   100.0),
            state_success_border:HslColor::new(142.0, 76.0,  25.0),
            state_warning_bg:    HslColor::new(38.0,  92.0,  50.0),
            state_warning_fg:    HslColor::new(0.0,   0.0,   0.0),
            state_warning_border:HslColor::new(38.0,  92.0,  35.0),
            overlay_bg:          HslColor::new(0.0,   0.0,   14.90),
            overlay_fg:          HslColor::new(0.0,   0.0,   89.80),
            chart_1: HslColor::new(43.26, 96.41, 56.27),
            chart_2: HslColor::new(32.13, 94.62, 43.73),
            chart_3: HslColor::new(22.73, 82.5,  31.37),
            chart_4: HslColor::new(25.96, 90.48, 37.06),
            chart_5: HslColor::new(22.73, 82.5,  31.37),
            sidebar_bg:          HslColor::new(0.0,   0.0,   5.88),
            sidebar_fg:          HslColor::new(0.0,   0.0,   89.80),
            sidebar_border:      HslColor::new(0.0,   0.0,   25.10),
            sidebar_primary_bg:  HslColor::new(37.69, 92.13, 50.20),
            sidebar_primary_fg:  HslColor::new(0.0,   0.0,   100.0),
            sidebar_accent_bg:   HslColor::new(22.73, 82.5,  31.37),
            sidebar_accent_fg:   HslColor::new(48.0,  96.64, 76.67),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Copy, serde::Serialize, serde::Deserialize)]
pub enum ActiveMode { Light, Dark }

/// ThemeState com sub-signals — cada parte tem owner global independente
#[derive(Clone)]
pub struct ThemeState {
    pub active: ArcRwSignal<ActiveMode>,
    pub light:  ArcRwSignal<ThemeTokens>,
    pub dark:   ArcRwSignal<ThemeTokens>,
    pub radius: ArcRwSignal<f32>,
}

impl ThemeState {
    pub fn new(owner: &leptos::prelude::Owner) -> Self {
        owner.with(|| Self {
            active: ArcRwSignal::new(ActiveMode::Light),
            light:  ArcRwSignal::new(ThemeTokens::default_light()),
            dark:   ArcRwSignal::new(ThemeTokens::default_dark()),
            radius: ArcRwSignal::new(0.375),
        })
    }

    pub fn current_css_vars(&self) -> Vec<(&'static str, String)> {
        match self.active.get() {
            ActiveMode::Light => self.light.get().to_css_vars(),
            ActiveMode::Dark  => self.dark.get().to_css_vars(),
        }
    }
}
