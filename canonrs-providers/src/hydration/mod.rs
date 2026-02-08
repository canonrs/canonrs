//! Hydration helpers and guards

/// Check if code is running in browser (hydrate context)
#[cfg(feature = "hydrate")]
pub fn is_browser() -> bool {
    true
}

#[cfg(not(feature = "hydrate"))]
pub fn is_browser() -> bool {
    false
}

/// Check if currently hydrating
pub fn is_hydrating() -> bool {
    cfg!(feature = "hydrate")
}
