//! Runtime region validation — dev only
//! Law: Block/Layout slots are always Option<ChildrenFn>
//! Required regions validated at render time in debug builds

/// Validate block regions in debug mode only.
/// Call with block id and list of provided region names (those where slot.is_some()).
/// Emits warning for each required region that is missing.
///
/// # Example
/// ```
/// validate_block_regions!("hero", &[
///     if header.is_some() { "header" } else { "" },
///     if content.is_some() { "content" } else { "" },
/// ]);
/// ```
#[macro_export]
macro_rules! validate_block_regions {
    ($block_id:expr, $provided:expr) => {
        #[cfg(debug_assertions)]
        {
            use canonrs_core::block_types::BlockDefinition;
            if let Some(def) = BlockDefinition::find($block_id) {
                let provided: &[&str] = $provided;
                for req in def.regions_required {
                    if !provided.contains(req) {
                        leptos::logging::warn!(
                            "[CanonRS][Block:'{}'] required region '{}' not provided — Law: Block slots are Option<ChildrenFn>",
                            $block_id,
                            req
                        );
                    }
                }
            }
        }
    };
}
