//! @canon-level: api
//! Alert API Contract

/// Alert root component
///
/// Props:
/// - `variant: AlertVariant` — Default, Destructive, Warning, Success
/// - `class: Option<String>`
/// - `children: Children`
pub struct AlertApi;

/// AlertTitle props:
/// - `class: Option<String>`
/// - `children: Children`
pub struct AlertTitleApi;

/// AlertDescription props:
/// - `class: Option<String>`
/// - `children: Children`
pub struct AlertDescriptionApi;

/// AlertCloseButton props:
/// - `class: Option<String>`
/// - `children: Children` — Close button content (e.g. icon)
///
/// Behavior: seta `data-rs-state="closed"` no closest `[data-rs-alert]`
/// CSS esconde via `[data-rs-alert][data-rs-state="closed"] { display: none }`
pub struct AlertCloseButtonApi;
