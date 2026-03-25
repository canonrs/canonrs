//! @canon-level: api
//! AlertDialog API Contract

/// AlertDialog root component
///
/// Props:
/// - `class: Option<String>`
/// - `children: Children`
pub struct AlertDialogApi;

/// AlertDialogTrigger props:
/// - `class: Option<String>`
/// - `children: Children`
pub struct AlertDialogTriggerApi;

/// AlertDialogOverlay props:
/// - `class: Option<String>`
pub struct AlertDialogOverlayApi;

/// AlertDialogContent props:
/// - `class: Option<String>`
/// - `children: Children`
///
/// Behavior: role=alertdialog + aria-live=assertive
/// aria-labelledby/describedby gerados automaticamente pelo behavior via DOM
pub struct AlertDialogContentApi;

/// AlertDialogTitle props:
/// - `class: Option<String>`
/// - `children: Children`
pub struct AlertDialogTitleApi;

/// AlertDialogDescription props:
/// - `class: Option<String>`
/// - `children: Children`
pub struct AlertDialogDescriptionApi;

/// AlertDialogClose props:
/// - `class: Option<String>`
/// - `children: Children`
pub struct AlertDialogCloseApi;
