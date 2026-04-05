use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum StatusDotIslandVariant {
    Online,
    #[default]
    Offline,
    Away,
    Busy,
    DoNotDisturb,
}

impl StatusDotIslandVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Online       => "online",
            Self::Offline      => "offline",
            Self::Away         => "away",
            Self::Busy         => "busy",
            Self::DoNotDisturb => "do-not-disturb",
        }
    }
    pub fn aria_label(&self) -> &'static str {
        match self {
            Self::Online       => "Online",
            Self::Offline      => "Offline",
            Self::Away         => "Away",
            Self::Busy         => "Busy",
            Self::DoNotDisturb => "Do not disturb",
        }
    }
    pub fn state(&self) -> &'static str {
        match self {
            Self::Online => "active",
            _            => "inactive",
        }
    }
}

#[island]
pub fn StatusDotIsland(
    #[prop(optional)] variant: Option<StatusDotIslandVariant>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class   = class.unwrap_or_default();
    let variant = variant.unwrap_or_default();
    let aria    = variant.aria_label();

    view! {
        <span data-rs-status-dot-wrapper="" style="display:inline-flex;align-items:center;gap:var(--space-xs);">
            <span
                data-rs-status-dot=""
                data-rs-component="StatusDot"
                data-rs-variant=variant.as_str()
                data-rs-state=variant.state()
                role="img"
                aria-label=aria
                class=class
            ></span>
            {label.map(|l| view! { <span data-rs-status-dot-label="" style="white-space:nowrap;">{l}</span> })}
        </span>
    }
}
