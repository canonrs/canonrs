# Stat

id: stat
label: Stat
family: family-f-data
category: Display
intent: Display a key metric with label
description: Metric stat display
composable: true
capabilities: 
required_parts: StatValue, StatLabel
optional_parts: StatDelta, StatIcon
tags: stat, metric, number, kpi, indicator, value
keywords: 
pain: Metrics displayed without consistent structure or alignment
promise: Metric layout and semantics enforced via structured primitives
why: StatPrimitive enforces composition of value, label and optional delta. Size, alignment and trend are encoded via attributes. This guarantees consistent KPI display.
rules: CR-001, CR-004
use_cases: dashboards, analytics
related: avatar, icon, logo, code_block, markdown, chart, inline_meta, kbd


file: stat_ui.css
tokens: stat-*, space-*, font-*, size-*
foundation: spacing, size, radius, typography
states: 
island: stat_island.rs

## before
// ❌ Typical
view! {
  <div>
    <h1>"100"</h1>
    <p>"Users"</p>
  </div>
}

## after
// ✅ CanonRS
view! {
  <Stat>
    <StatValue>"100"</StatValue>
    <StatLabel>"Users"</StatLabel>
  </Stat>
}
