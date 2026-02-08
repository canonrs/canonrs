# CanonRS Build Pipeline

## Overview
O `build.rs` do pacote `canonrs` orquestra a geração e embedding de CSS durante a compilação.

## Pipeline (3 etapas)

### 1. Execute tokens-engine
```rust
Command::new("cargo")
    .args(["run", "--bin", "tokens-engine"])
    .current_dir("../canonrs-tokens")
```

**O que faz:**
- Gera todas as famílias de tokens CSS (.generated/)
- Processa temas (light/dark)
- Cria canonrs.css (entry point)
- Cria canonrs.bundle.css (single-file)

**Output:**
```
canonrs-ui/styles/
├── .generated/
│   ├── family-*.css (11 arquivos)
│   └── themes.css
├── canonrs.css (~4KB)
└── canonrs.bundle.css (~315KB)
```

### 2. Copy CSS to OUT_DIR
```rust
fs::copy(
    "../canonrs-ui/styles/canonrs.bundle.css",
    "${OUT_DIR}/canonrs.css"
)
```

**Por quê bundle.css?**
- Single-file = sem @imports runtime
- Embedding = incluso no binário
- Zero network requests
- Funciona offline

**Localização:**
```
target/debug/build/canonrs-<hash>/out/canonrs.css
```

### 3. Rerun triggers
```rust
println!("cargo:rerun-if-changed=../canonrs-tokens/src");
println!("cargo:rerun-if-changed=../canonrs-ui/styles");
```

**Quando rebuilda:**
- Tokens Rust mudam
- CSS base muda
- Temas mudam
- Componentes CSS mudam

## Fluxo Completo
```
cargo build
    ↓
build.rs executa
    ↓
tokens-engine roda
    ↓
CSS gerado em canonrs-ui/styles/
    ↓
bundle.css copiado para OUT_DIR
    ↓
CSS embedded no binário canonrs
    ↓
Apps usam: use canonrs::ui::*
```

## Como o CSS é servido

### Leptos (SSR)
```rust
use leptos_meta::Stylesheet;

view! {
    <Stylesheet href="/canonrs.css"/>
}
```

### Axum (File serving)
```rust
// O canonrs.css é copiado para public/ durante build
// Servidor serve de public/canonrs.css
```

## Debugging

### CSS não atualiza?
```bash
# Force rebuild
cargo clean -p canonrs
cargo build -p canonrs
```

### tokens-engine falha?
```bash
# Rode manualmente
cd packages-rust/rs-canonrs/canonrs-tokens
cargo run --bin tokens-engine
```

### CSS não existe no OUT_DIR?
```bash
# Verifique o source
ls -lh packages-rust/rs-canonrs/canonrs-ui/styles/canonrs.bundle.css

# Verifique o target
find target -name "canonrs.css" -path "*/out/*"
```

## Otimização

### Por que não gerar CSS on-demand?
- ❌ Runtime overhead
- ❌ Filesystem access
- ❌ Cache invalidation complexo

### Por que embedding?
- ✅ Zero config para usuários
- ✅ CSS sempre sincronizado com componentes
- ✅ Offline-first
- ✅ Single source of truth

## Alternativas (não usadas)

### include_str! no código
❌ Não funciona com arquivos gerados dinamicamente

### Copy no Cargo.toml
❌ Não pode executar comandos

### Procedural macro
❌ Muito complexo, executa tarde demais

### build.rs é o único jeito de executar código arbitrário antes da compilação.

## Enterprise Trade-offs

| Abordagem | Build time | Runtime | Debugging | Adotamos? |
|-----------|-----------|---------|-----------|-----------|
| build.rs + embedding | Lento | Rápido | Difícil | ✅ Sim |
| Runtime generation | Rápido | Lento | Fácil | ❌ Não |
| Prebuilt CSS (manual) | - | Rápido | Médio | ❌ Não |

**Decisão:** Priorizar runtime sobre build time.

## Verificação

Após `cargo build -p canonrs`, deve existir:
```bash
# 1. CSS gerado
ls canonrs-ui/styles/canonrs.bundle.css

# 2. CSS no OUT_DIR
find target -path "*/canonrs-*/out/canonrs.css"

# 3. Log de sucesso
# cargo:warning=✅ CanonRS CSS embedded: 323584 bytes
```

## Troubleshooting

### "CSS not found after tokens-engine ran"
```bash
cd canonrs-tokens
cargo run --bin tokens-engine
ls ../canonrs-ui/styles/canonrs.bundle.css
```

### "Failed to run tokens-engine"
```bash
# Verifique deps
cd canonrs-tokens
cargo check --bin tokens-engine
```

### CSS desatualizado no app
```bash
# Force full rebuild
cargo clean
cargo build
```
