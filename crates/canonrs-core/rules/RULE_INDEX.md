# Canon Rules - Semantic Index

Quick reference for requesting specific knowledge.

## 🏗️ Architecture (Core Design)

- **#03** - BFF Mandatory Boundary
- **#04** - Port Allocation Strategy
- **#05** - Core Services Non-Exposure
- **#06** - Traefik Single Ingress
- **#07** - Environment Classification

## 🔐 Security (Zero Trust)

- **#08** - Auth Core Hardening
- **#09** - Token Audience Validation
- **#10** - mTLS Core-to-Core
- **#11** - Key Rotation & kid

## 🐳 Docker + Rust (Build System)

- **#12** - Cargo Workspace ≠ Target Directory
- **#13** - Binary Size Validation
- **#14** - Exit 0 ≠ Crash
- **#15** - Scratch Images Requirements
- **#18** - Docker Cache Invalidation
- **#19** - CLI vs Compose (Never Mix)
- **#20** - MUSL for Container Compatibility
- **#21** - Rust Build Cache

## ⚡ Leptos Runtime (SSR Model)

- **#22** - Three Runtimes (server, browser, hydrate)
- **#23** - Providers Exist Exactly Once
- **#24** - Context Is Lexical
- **#25** - Pages Orchestrate, Blocks Execute
- **#26** - Effects Run Per Runtime
- **#27** - Hydration Determinism

---

## 📖 How to Use

**For new chat:**
1. Start with `CANON_RUNTIME_CONTRACT.md`
2. Reference this index when needed

**When problem occurs:**
1. Identify category (Architecture/Security/Docker/Leptos)
2. Request specific Rule Capsule
3. Apply full rule only if capsule insufficient

**Example:**
> "Hydration mismatch appearing. Apply Rule Capsule #27."
