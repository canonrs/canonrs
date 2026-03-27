# CanonRS — Blocks

> AUTO-GENERATED

---

## `container`

- **Categoria:** layout
- **Variante:** structure
- **Container:** sim
- **Regioes:** content

### Props

| Prop | Tipo | Default | Escopo |
|------|------|---------|--------|
| `max-width` | Text | `1200px` | visual |
| `padding` | Number | `-` | visual |

---

## `footer`

- **Categoria:** page
- **Variante:** page
- **Container:** sim
- **Regioes:** left, center, right

---

## `stack`

- **Categoria:** layout
- **Variante:** structure
- **Container:** sim
- **Regioes:** items

### Props

| Prop | Tipo | Default | Escopo |
|------|------|---------|--------|
| `flex-direction` | Select(column:Vertical,row:Horizontal) | `column` | visual |
| `gap` | Number | `0.5rem` | visual |
| `align-items` | Select(stretch:Stretch,flex-start:Start,center:Center,flex-end:End) | `stretch` | visual |

---

## `header`

- **Categoria:** page
- **Variante:** page
- **Container:** sim
- **Regioes:** logo, nav, center, actions

---

## `grid`

- **Categoria:** layout
- **Variante:** structure
- **Container:** sim
- **Regioes:** items

### Props

| Prop | Tipo | Default | Escopo |
|------|------|---------|--------|
| `grid-columns` | Number | `3` | structural |
| `grid-template-columns` | Select(repeat(1,1fr):1,repeat(2,1fr):2,repeat(3,1fr):3,repeat(4,1fr):4,repeat(5,1fr):5,repeat(6,1fr):6) | `repeat(3,1fr)` | visual |
| `gap` | Number | `1rem` | visual |
| `row-gap` | Number | `-` | visual |

### Presets

- **2 Columns:** `grid-columns=2`, `grid-template-columns=repeat(2,1fr)`, `gap=1rem`
- **3 Columns:** `grid-columns=3`, `grid-template-columns=repeat(3,1fr)`, `gap=1rem`
- **4 Columns:** `grid-columns=4`, `grid-template-columns=repeat(4,1fr)`, `gap=1rem`
- **Sidebar:** `grid-template-columns=240px 1fr`, `gap=1.5rem`

---

## `columns`

- **Categoria:** layout
- **Variante:** structure
- **Container:** sim
- **Regioes:** columns

### Props

| Prop | Tipo | Default | Escopo |
|------|------|---------|--------|
| `gap` | Number | `1rem` | visual |

### Presets

- **Equal 2:** `gap=1rem`
- **Equal 3:** `gap=1rem`

---

## `sidebar-layout`

- **Categoria:** layout
- **Variante:** structure
- **Container:** sim
- **Regioes:** nav, main

---

## `timeline`

- **Categoria:** data
- **Variante:** feature
- **Container:** sim
- **Regioes:** header, items, footer

---

## `command-panel`

- **Categoria:** overlay
- **Variante:** overlay
- **Container:** nao
- **Regioes:** search, results, footer

---

## `wizard`

- **Categoria:** form
- **Variante:** feature
- **Container:** sim
- **Regioes:** steps, body, actions

---

## `split`

- **Categoria:** layout
- **Variante:** structure
- **Container:** sim
- **Regioes:** aside, main

---

## `detail-panel`

- **Categoria:** layout
- **Variante:** structure
- **Container:** sim
- **Regioes:** aside, content

---

## `filter-bar`

- **Categoria:** data
- **Variante:** feature
- **Container:** sim
- **Regioes:** filters, actions

---

## `stat-card`

- **Categoria:** data
- **Variante:** feature
- **Container:** nao
- **Regioes:** icon, label, value, change

---

## `list`

- **Categoria:** data
- **Variante:** feature
- **Container:** sim
- **Regioes:** header, items, footer

---

