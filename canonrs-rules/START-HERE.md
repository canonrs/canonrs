# Canon Design System — START HERE

**Audience:** Developers, Architects, AI Systems  
**Scope:** rs-design Canon Rules (#1–#71)

---

## The Purpose

This document is the **single entry point** to the Canon.

You do **NOT** start by reading all rules.  
You start by **identifying the problem type**, then follow the correct path.

> **Canon Rules are law.**
> This file tells you **where to look first**.

---

## 🔍 First Question (Always)

**What kind of problem are you facing?**

Choose ONE path below.

---

## 🧭 Path 1 — CSS, Theme, Styles, White Page

Symptoms:
- Page renders but unstyled
- Dark / light mode not working
- Theme toggle does nothing
- Classes seem applied but visuals don't change
- Everything works in code, nothing works visually

➡️ Go to: **SYMPTOMS.md → CSS & THEME**

Primary Rules involved:
- #55 Canonical CSS Entry Points
- #56 Monorepo CSS Build Pipeline
- #57 PostCSS Canon Config
- #58 Leptos Assets Dev Constraint
- #59 CSS Cascade Ownership
- #60 CSS Artifacts Are Immutable
- #61 No Relative CSS Imports

---

## 🧭 Path 2 — Build, Serve, Assets, dist/

Symptoms:
- 404 on CSS or assets
- File exists in repo but not in browser
- Build serves app but not styles
- Hot reload unreliable
- Works yesterday, broken today

➡️ Go to: **SYMPTOMS.md → BUILD & ASSETS**

Primary Rules involved:
- #55 Canonical CSS Entry Points
- #56 Monorepo CSS Build Pipeline
- #58 Leptos Assets Dev Constraint

---

## 🧭 Path 3 — CSS Import Order & PostCSS

Symptoms:
- PostCSS warnings about @import order
- Canon tokens not applied
- Tailwind compiles but Canon missing
- @import statements failing

➡️ Go to: **SYMPTOMS.md → CSS IMPORTS**

Primary Rules involved:
- #55 Canonical CSS Entry Points
- #57 PostCSS Canon Config
- #61 No Relative CSS Imports

---

## 🧭 Path 4 — New App / New Workbench

Symptoms:
- Starting a new example or app
- Cloned repo, unsure if setup is correct
- Want to avoid hours of debugging later

➡️ Mandatory: **Apply Rules #55, #56, #58 in order**

---

## Golden Rule

> **Never start debugging application logic before verifying:**
> 1. File exists in canonical package  
> 2. File is imported correctly  
> 3. File contains expected content  

This order is **non-negotiable**.

---

## What NOT to Do

- ❌ Do not use relative paths to import CSS
- ❌ Do not import from `packages-rust/` or `crates/`
- ❌ Do not assume PostCSS "handled it"
- ❌ Do not put CSS in subfolders under `public/`

Canon is **deterministic**.  
Follow the path, do not improvise.

---

## Next Step

Once you know the problem type:

→ Read **SYMPTOMS.md**  
→ Follow the Debug Order  
→ Apply the referenced Rules  

That is the intended workflow.
