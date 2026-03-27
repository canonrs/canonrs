# CanonRS — Layouts

> AUTO-GENERATED — do not edit manually

---

## `section`

- **Label:** Section ▤
- **Description:** Self-contained section with header, content and footer
- **Category:** layout
- **Regions:** header, content, footer
- **Use when:** content sections within a page, feature blocks

### Slots

| Slot | Description |
|------|-------------|
| `header` | Section title area |
| `content` | Section content |
| `footer` | Section footer actions |

### Slot Rules

| Slot | Accepts |
|------|---------|
| `header` | Any |
| `content` | Any |
| `footer` | Action |

---

## `page-layout`

- **Label:** Page ▭
- **Description:** Page layout with optional sidebar and aside
- **Category:** layout
- **Regions:** sidebar, content, aside
- **Use when:** documentation pages, articles, content with optional sidebar

### Slots

| Slot | Description |
|------|-------------|
| `sidebar` | Navigation sidebar |
| `content` | Primary content |
| `aside` | Contextual panel |

### Slot Rules

| Slot | Accepts |
|------|---------|
| `sidebar` | Nav |
| `content` | Any |
| `aside` | Any |

---

## `wizard-layout`

- **Label:** Wizard 📋
- **Description:** Multi-step form with header, stepper, content and footer
- **Category:** layout
- **Regions:** header, stepper, content, footer
- **Use when:** multi-step forms, onboarding flows, guided setup

### Slots

| Slot | Description |
|------|-------------|
| `header` | Wizard title and progress |
| `stepper` | Step indicators |
| `content` | Step content |
| `footer` | Navigation actions |

### Slot Rules

| Slot | Accepts |
|------|---------|
| `header` | Any |
| `stepper` | Any |
| `content` | Form |
| `footer` | Action |

---

## `split-view`

- **Label:** Split View ◧
- **Description:** Left context panel and right action/detail panel
- **Category:** layout
- **Regions:** left, right
- **Use when:** master-detail views, side-by-side comparisons, email clients

### Slots

| Slot | Description |
|------|-------------|
| `left` | Context or list panel |
| `right` | Detail or action panel |

### Slot Rules

| Slot | Accepts |
|------|---------|
| `left` | Nav |
| `right` | Action |

---

## `dashboard`

- **Label:** Dashboard ⬛
- **Description:** App shell with header, sidebar and main content area
- **Category:** layout
- **Regions:** header, sidebar, content
- **Use when:** admin panels, data-heavy apps, internal tools with sidebar navigation

### Slots

| Slot | Description |
|------|-------------|
| `header` | Top navigation bar |
| `sidebar` | Left navigation panel |
| `content` | Primary content area |

### Slot Rules

| Slot | Accepts |
|------|---------|
| `header` | Nav |
| `sidebar` | Nav |
| `content` | Any |

---

## `marketing`

- **Label:** Marketing 🌐
- **Description:** Public page with header, hero, main content and footer
- **Category:** layout
- **Regions:** header, hero, content, footer
- **Use when:** public-facing pages, landing pages, product sites

### Slots

| Slot | Description |
|------|-------------|
| `header` | Site header with navigation |
| `hero` | Hero/banner section |
| `content` | Main content sections |
| `footer` | Site footer |

### Slot Rules

| Slot | Accepts |
|------|---------|
| `header` | Nav |
| `hero` | Any |
| `content` | Any |
| `footer` | Action |

---

## `fullscreen`

- **Label:** Fullscreen ⬜
- **Description:** Optional header with full canvas content area
- **Category:** layout
- **Regions:** header, content
- **Use when:** focused editors, canvas tools, immersive experiences

### Slots

| Slot | Description |
|------|-------------|
| `header` | Optional top bar |
| `content` | Full canvas area |

### Slot Rules

| Slot | Accepts |
|------|---------|
| `header` | Nav |
| `content` | Any |

---

