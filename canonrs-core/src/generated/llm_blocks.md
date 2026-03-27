# CanonRS — Blocks

> AUTO-GENERATED — do not edit manually

---

## `container`

- **Label:** Container
- **Description:** Max-width centered container
- **Category:** layout
- **Variant:** structure
- **Container:** yes
- **Regions:** content

### Region Rules

| Region | Accepts |
|--------|---------|
| `content` | Any |

### Props

| Prop | Type | Default | Scope |
|------|------|---------|-------|
| `max-width` | Text | `1200px` | visual |
| `padding` | Number | `-` | visual |

### Typical Children

card, grid, stack, columns, container

---

## `footer`

- **Label:** Footer
- **Description:** Page footer block
- **Category:** page
- **Variant:** page
- **Container:** yes
- **Regions:** left, center, right

### Region Rules

| Region | Accepts |
|--------|---------|
| `left` | Nav |
| `center` | Any |
| `right` | Action |

### Typical Children

header, footer, toolbar, page-header

---

## `stack`

- **Label:** Stack
- **Description:** Flex stack container vertical or horizontal
- **Category:** layout
- **Variant:** structure
- **Container:** yes
- **Regions:** items

### Region Rules

| Region | Accepts |
|--------|---------|
| `items` | Any |

### Props

| Prop | Type | Default | Scope |
|------|------|---------|-------|
| `flex-direction` | Select(column:Vertical,row:Horizontal) | `column` | visual |
| `gap` | Number | `0.5rem` | visual |
| `align-items` | Select(stretch:Stretch,flex-start:Start,center:Center,flex-end:End) | `stretch` | visual |

### Typical Children

card, grid, stack, columns, container

---

## `header`

- **Label:** Header
- **Description:** Page header with left center right regions
- **Category:** page
- **Variant:** page
- **Container:** yes
- **Regions:** logo, nav, center, actions

### Region Rules

| Region | Accepts |
|--------|---------|
| `logo` | Any |
| `nav` | Nav |
| `center` | Any |
| `actions` | Action |

### Typical Children

header, footer, toolbar, page-header

---

## `grid`

- **Label:** Grid
- **Description:** CSS grid layout with N columns
- **Category:** layout
- **Variant:** structure
- **Container:** yes
- **Regions:** items

### Region Rules

| Region | Accepts |
|--------|---------|
| `items` | Any |

### Props

| Prop | Type | Default | Scope |
|------|------|---------|-------|
| `grid-columns` | Number | `3` | structural |
| `grid-template-columns` | Select(repeat(1,1fr):1,repeat(2,1fr):2,repeat(3,1fr):3,repeat(4,1fr):4,repeat(5,1fr):5,repeat(6,1fr):6) | `repeat(3,1fr)` | visual |
| `gap` | Number | `1rem` | visual |
| `row-gap` | Number | `-` | visual |

### Presets

- **2 Columns:** `grid-columns=2`, `grid-template-columns=repeat(2,1fr)`, `gap=1rem`
- **3 Columns:** `grid-columns=3`, `grid-template-columns=repeat(3,1fr)`, `gap=1rem`
- **4 Columns:** `grid-columns=4`, `grid-template-columns=repeat(4,1fr)`, `gap=1rem`
- **Sidebar:** `grid-template-columns=240px 1fr`, `gap=1.5rem`

### Typical Children

card, grid, stack, columns, container

---

## `columns`

- **Label:** Columns
- **Description:** Two equal columns
- **Category:** layout
- **Variant:** structure
- **Container:** yes
- **Regions:** columns

### Region Rules

| Region | Accepts |
|--------|---------|
| `columns` | Any |

### Props

| Prop | Type | Default | Scope |
|------|------|---------|-------|
| `gap` | Number | `1rem` | visual |

### Presets

- **Equal 2:** `gap=1rem`
- **Equal 3:** `gap=1rem`

### Typical Children

card, grid, stack, columns, container

---

## `sidebar-layout`

- **Label:** Sidebar Layout
- **Description:** Block-level sidebar and main content
- **Category:** layout
- **Variant:** structure
- **Container:** yes
- **Regions:** nav, main

### Region Rules

| Region | Accepts |
|--------|---------|
| `nav` | Nav |
| `main` | Any |

### Typical Children

card, grid, stack, columns, container

---

## `timeline`

- **Label:** Timeline
- **Description:** Chronological timeline block
- **Category:** data
- **Variant:** feature
- **Container:** yes
- **Regions:** header, items, footer

### Region Rules

| Region | Accepts |
|--------|---------|
| `header` | Any |
| `items` | Any |
| `footer` | Action |

### Typical Children

button, input, select, badge, stat-card

---

## `command-panel`

- **Label:** Command Panel
- **Description:** Command palette overlay block
- **Category:** overlay
- **Variant:** overlay
- **Container:** no
- **Regions:** search, results, footer

### Region Rules

| Region | Accepts |
|--------|---------|
| `search` | Form |
| `results` | Any |
| `footer` | Action |

### Typical Children

button, form, markdown, list

---

## `wizard`

- **Label:** Wizard
- **Description:** Multi-step form wizard block
- **Category:** form
- **Variant:** feature
- **Container:** yes
- **Regions:** steps, body, actions

### Region Rules

| Region | Accepts |
|--------|---------|
| `steps` | Any |
| `body` | Form |
| `actions` | Action |

### Typical Children

input, select, checkbox, switch, textarea, radio-group

---

## `split`

- **Label:** Split
- **Description:** Aside and main two-panel block
- **Category:** layout
- **Variant:** structure
- **Container:** yes
- **Regions:** aside, main

### Region Rules

| Region | Accepts |
|--------|---------|
| `aside` | Nav |
| `main` | Any |

### Typical Children

card, grid, stack, columns, container

---

## `detail-panel`

- **Label:** Detail Panel
- **Description:** Master-detail panel layout
- **Category:** layout
- **Variant:** structure
- **Container:** yes
- **Regions:** aside, content

### Region Rules

| Region | Accepts |
|--------|---------|
| `aside` | Any |
| `content` | Any |

### Typical Children

card, grid, stack, columns, container

---

## `filter-bar`

- **Label:** Filter Bar
- **Description:** Filters and actions bar
- **Category:** data
- **Variant:** feature
- **Container:** yes
- **Regions:** filters, actions

### Region Rules

| Region | Accepts |
|--------|---------|
| `filters` | Form |
| `actions` | Action |

### Typical Children

button, input, select, badge, stat-card

---

## `stat-card`

- **Label:** Stat Card
- **Description:** Metric stat display block
- **Category:** data
- **Variant:** feature
- **Container:** no
- **Regions:** icon, label, value, change

### Region Rules

| Region | Accepts |
|--------|---------|
| `icon` | Any |
| `label` | Any |
| `value` | Any |
| `change` | Any |

### Typical Children

button, input, select, badge, stat-card

---

## `list`

- **Label:** List
- **Description:** Vertical list container
- **Category:** data
- **Variant:** feature
- **Container:** yes
- **Regions:** header, items, footer

### Region Rules

| Region | Accepts |
|--------|---------|
| `header` | Any |
| `items` | Any |
| `footer` | Action |

### Typical Children

button, input, select, badge, stat-card

---

