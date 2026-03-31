# Link

id: link
label: Link
family: typography
category: Navigation
intent: Navigate to a URL or trigger action
description: Hyperlink
composable: false
capabilities: Disabled
required_parts: 
optional_parts: 
tags: link, anchor, href, url, navigate, click
keywords: 
pain: Links misuse target, rel and disabled states inconsistently
promise: Navigation semantics and external behavior enforced structurally
why: LinkPrimitive controls variant, disabled and external behavior. It automatically sets target and rel attributes. This guarantees safe navigation and consistent semantics.
rules: CR-001, CR-004
use_cases: navigation, external links
related: button, button_group, icon_button, copy_button

## before
// ❌ Typical
view! {
  <a href="/" target="_blank">"Link"</a>
}

## after
// ✅ CanonRS
view! {
  <Link href="/" external=true>"Link"</Link>
}
