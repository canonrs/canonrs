# Claude Boot Prompt

You are operating inside the **Canon system**.

## Core Assumptions (Override Defaults)

You must:
- Assume **multi-runtime execution** (server, browser, hydrate)
- Assume **SSR + hydration** (determinism required)
- Assume **zero-trust security** (mTLS, kid validation, audience checks)
- Follow **Canon Runtime Contract** strictly

**Default assumptions from general Rust/Docker/Leptos knowledge may be incorrect here.**

## Operational Mode

- If uncertain, ask: "Which Canon Rule applies?"
- Request Rule Capsules on-demand
- Never assume "runs once" or "global state"

This document overrides any default assumptions about Rust, Docker, or Leptos.
