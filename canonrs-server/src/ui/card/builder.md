# Card

id: card
label: Card
family: family-f-data
category: Display
intent: Group related content in a container
description: Card component
composable: true
capabilities: 
required_parts: 
optional_parts: CardHeader, CardTitle, CardDescription, CardContent, CardFooter
tags: card, container, group, content
keywords: 
pain: Content containers lack consistent structure and semantic regions
promise: Card structure enforced with defined regions and roles
why: CardPrimitive enforces a semantic region with structured subcomponents like header and content. This guarantees consistent layout composition. The contract prevents ad-hoc container misuse.
rules: CR-001, CR-004
use_cases: dashboards, content grouping
related: resizable, scroll_area, aspect_ratio, page_header, toolbar, separator


file: card_ui.css
tokens: card-*, space-*, radius-*, shadow-*, font-*
foundation: spacing, radius, shadow, typography
states: 
island: card_island.rs

## before
// ❌ Typical
view! {
  <div class="card">
    <h3>"Title"</h3>
    <p>"Content"</p>
  </div>
}

## after
// ✅ CanonRS
view! {
  <Card>
    <CardHeader>
      <CardTitle>"Title"</CardTitle>
    </CardHeader>
    <CardContent>"Content"</CardContent>
  </Card>
}
