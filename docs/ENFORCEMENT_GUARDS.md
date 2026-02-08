# CanonRS Enforcement Guards

## Overview
Este documento descreve os **guards enterprise** implementados para prevenir regressões arquiteturais no CanonRS. Estes guards são obrigatórios e **não negociáveis**.

---

## 1. Compile-Time Guards

### 1.1 WASM-Only Enforcement (`canonrs-csr`)
**Localização**: `canonrs-csr/src/lib.rs`
```rust
#[cfg(all(not(target_arch = "wasm32"), not(doc)))]
compile_error!("❌ CANON VIOLATION: canonrs-csr can only be used in WASM builds (target_arch = wasm32)");
```

**Propósito**: Impede que código CSR (client-side) seja compilado para servidor.

**Previne**: 
- Vazamento de código WASM-only para bins SSR
- Binários SSR crescerem com código client inútil
- Quebra de separação SSR/CSR

---

### 1.2 SSR-in-WASM Prevention (`canonrs-ui`)
**Localização**: `canonrs-ui/src/lib.rs`
```rust
#[cfg(all(target_arch = "wasm32", feature = "ssr"))]
compile_error!("❌ CANON VIOLATION: feature 'ssr' cannot be enabled in WASM build");
```

**Propósito**: Impede que features SSR sejam ativadas em builds WASM.

**Previne**:
- Código servidor no bundle WASM
- WASM crescer de 2MB → 143MB (caso real ocorrido)
- Dependências server-only em client bundles

---

## 2. Build-Time Guards

### 2.1 WASM Size Limit
**Localização**: `products/canonrs-site/Makefile`
```makefile
MAX_WASM_MB := 10

check-wasm-size:
	@if [ -f "target/site/pkg/canonrs-site.wasm" ]; then \
		WASM_SIZE=$$(du -m target/site/pkg/canonrs-site.wasm | cut -f1); \
		echo "WASM size: $${WASM_SIZE}MB (max: $(MAX_WASM_MB)MB)"; \
		if [ $$WASM_SIZE -gt $(MAX_WASM_MB) ]; then \
			echo "❌ CANON VIOLATION: WASM too large ($${WASM_SIZE}MB > $(MAX_WASM_MB)MB)"; \
			exit 1; \
		fi; \
	fi
```

**Propósito**: Falha o build se WASM ultrapassar 10MB.

**Previne**:
- Regressões silenciosas de tamanho
- Deploy de bundles gigantes (143MB)
- Performance degradation no cliente

**Uso**:
```bash
make build  # Automatically checks WASM size
```

---

## 3. Architectural Guards

### 3.1 Feature Gates em `canonrs-ui`
**Padrão**: Use `#[cfg(feature = "hydrate")]` para código client-only
```rust
#[cfg(feature = "hydrate")]
{
    use leptos::leptos_dom::helpers::document;
    let html = document().document_element().unwrap();
    // ... código DOM
}

#[cfg(not(feature = "hydrate"))]
let initial_value = Default::default();
```

**Razão**: `canonrs-ui` compila para **SSR E CSR**. Feature gates diferenciam comportamento no mesmo target.

**NÃO USE** `#[cfg(target_arch = "wasm32")]` em `canonrs-ui` - isso quebra SSR.

---

### 3.2 Separation of Concerns
**Regra Canônica**:
```
canonrs-shared → tokens, tipos (SSR + CSR)
canonrs-ui     → componentes Leptos (SSR + CSR safe)
canonrs-csr    → behaviors WASM-only
canonrs-server → utilitários SSR-only
canonrs        → facade pública
```

**Enforcement**: Cada crate declara suas restrições via `compile_error!`.

---

## 4. Workspace Configuration Guards

### 4.1 Unified Workspace
**Localização**: `/opt/docker/monorepo/Cargo.toml`

Todos os crates devem estar em `workspace.members`:
- Previne divergência de versões
- Centraliza dependências comuns
- Facilita audits de segurança

### 4.2 Shared Dependencies
Dependencies usadas por múltiplos crates **devem** estar em `[workspace.dependencies]`:
```toml
[workspace.dependencies]
leptos = { version = "0.8", default-features = false }
serde = { version = "1.0", features = ["derive"] }
# ...
```

---

## 5. Testing Guards

### 5.1 CI Size Check
**TODO**: Adicionar workflow GitHub Actions:
```yaml
- name: Check WASM size
  run: |
    cd products/canonrs-site
    make check-wasm-size
```

---

## 6. Developer Guidelines

### DO ✅
- Sempre use `make build` (inclui size check)
- Adicione `compile_error!` em crates com restrições
- Use feature gates corretamente (`hydrate` vs `target_arch`)
- Mantenha workspace.dependencies sincronizado

### DON'T ❌
- Nunca ignore warnings de size
- Nunca compile `canonrs-csr` fora de WASM
- Nunca use `#[cfg(target_arch)]` em `canonrs-ui`
- Nunca adicione deps sem workspace
- Nunca crie bins multi-target

---

## 7. Incident History

### 2025-02-04: WASM 143MB Regression
**Causa**: Código SSR vazou para build WASM  
**Impacto**: Bundle cresceu 71x (2MB → 143MB)  
**Fix**: Guards 1.1, 1.2, 2.1 implementados  
**Lição**: "O desenvolvedor nunca decide target. O crate decide."

---

## 8. Compliance Checklist

Antes de commit/PR:
- [ ] `cargo check` passa em todos os crates
- [ ] `make build` passa (inclui size check)
- [ ] Nenhum warning de workspace profiles
- [ ] WASM < 10MB
- [ ] Nenhum `compile_error!` violado

---

**Última atualização**: 2025-02-04  
**Responsável**: CanonRS Core Team
