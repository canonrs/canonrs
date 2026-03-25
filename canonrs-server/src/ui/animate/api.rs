//! @canon-level: api
//! Animate API Contract

/// Animate wrapper component
///
/// Props:
/// - `animation_type: AnimationType` — FadeIn (default), FadeOut, SlideIn, SlideOut, ScaleIn, ScaleOut
/// - `duration: Option<String>` — CSS duration e.g. "300ms"
/// - `easing: Option<String>` — CSS easing e.g. "ease-in-out"
/// - `delay: Option<String>` — CSS delay e.g. "100ms"
/// - `class: Option<String>`
/// - `id: Option<String>`
/// - `children: Children`
pub struct AnimateApi;
