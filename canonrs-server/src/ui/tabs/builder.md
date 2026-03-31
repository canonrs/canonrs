# Tabs

id: tabs
label: Tabs
family: navigation
category: Navigation
intent: Switch between related content panels
description: Tabbed navigation
composable: true
capabilities: Active
required_parts: TabsList, TabsTrigger, TabsContent
optional_parts: 
tags: tabs, navigation, tab, sections, panels
keywords: 
pain: Tabs require manual state sync between triggers and panels
promise: Active state governs trigger and content without manual wiring
why: TabsPrimitive uses ActivityState to synchronize triggers and panels. ARIA roles and visibility are derived automatically. This guarantees consistent tab behavior.
rules: CR-001, CR-004
use_cases: panel navigation, settings
related: collapsible, table_of_contents

## before
// ❌ Typical
view! {
  <button>"Tab"</button>
  <div>"Content"</div>
}

## after
// ✅ CanonRS
view! {
  <Tabs>
    <TabsList>
      <TabsTrigger value="a" active=true>"Tab"</TabsTrigger>
    </TabsList>
    <TabsContent value="a" active=true>"Content"</TabsContent>
  </Tabs>
}
