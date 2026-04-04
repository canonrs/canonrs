# Banner

id: banner
label: Banner
family: family-e-feedback
category: Feedback
intent: Display persistent page-level messages
description: Banner message
composable: true
capabilities: OpenClose
required_parts: 
optional_parts: BannerContent, BannerActions, BannerClose
tags: banner, announcement, notification, top, message
keywords: 
pain: Banner messages lack consistent visibility and accessibility behavior
promise: Visibility and ARIA behavior enforced by state and variant
why: BannerVariant controls semantic role and aria-live behavior. VisibilityState ensures correct open/hidden state without runtime logic. This guarantees accessible, consistent page-level messaging.
rules: CR-001, CR-004
use_cases: system announcements, warnings
related: toast, alert, callout, inline_notice, status_dot, inline_meta, badge


file: banner_ui.css
tokens: banner-*, space-*, radius-*, font-*
foundation: spacing, radius, typography
states: open, closed
island: banner_island.rs

## before
// ❌ Typical
view! {
  <div class="banner">"Maintenance scheduled"</div>
}

## after
// ✅ CanonRS
view! {
  <Banner>
    <BannerContent>"Maintenance scheduled"</BannerContent>
  </Banner>
}
