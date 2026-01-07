# Rust Rules #1: Docker Build Cache Invalidation

**Status:** Normative  
**Applies to:** All Rust services in monorepo with multi-stage Docker builds

---

## The Problem

**Código modificado** + **Docker layer cache** = **Binário obsoleto**

### Why?

1. Docker copia source code em layer COPY
2. Layer é cached se timestamp não muda
3. `cargo build` usa cached layer
4. Binário compilado é antigo
5. **Exit 0 silencioso** sem logs/panic esperados

### Real Example

```bash
# Modificar main.rs
echo 'panic!("TEST");' >> src/main.rs

# Build com cache
docker build -t service .

# Executar
docker run service
# Exit: 0 (sem panic)

# Strings não contém "TEST"
strings /usr/local/bin/service | grep TEST
# (vazio)
```

**Root Cause:** COPY layer estava cached, `cargo build` não recompilou.

---

## The Correct Solution: Always Use --no-cache for Source Changes

### Pattern

```bash
# ❌ ERRADO: Build incremental após modificar código
docker build -t service .

# ✅ CORRETO: Forçar rebuild completo
docker build --no-cache -t service .

# ✅ ALTERNATIVA: Limpar builder cache antes
docker builder prune -af
docker build -t service .
```

### Why This Works

1. `--no-cache` invalida TODAS as layers
2. COPY sempre pega versão mais recente
3. `cargo build` recompila tudo
4. Binário reflete código atual
5. **Behavior esperado garantido**

---

## Mandatory Checks Before Build

### Checklist

- [ ] Modifiquei código Rust (src/, lib.rs, main.rs)?
- [ ] Se SIM: usar `--no-cache` ou `builder prune`
- [ ] Após build: testar execução imediata
- [ ] Verificar logs esperados aparecem
- [ ] Confirmar `strings` contém mensagens de debug

### Verification Commands

```bash
# 1. Build sem cache
docker build --no-cache -f path/to/Dockerfile -t service .

# 2. Extrair binário
docker create --name temp service
docker cp temp:/usr/local/bin/service /tmp/service
docker rm temp

# 3. Verificar strings esperadas
strings /tmp/service | grep "EXPECTED_STRING"

# 4. Executar e validar comportamento
/tmp/service 2>&1
# Deve mostrar logs/panic esperados
```

---

## When Cache is Safe

### ✅ Can Use Cache

- Modificou apenas Dockerfile (RUN, ENV)
- Modificou apenas dependências (Cargo.toml)
- Build inicial (nenhum cache existe)
- CI/CD pipeline controlado

### ❌ Must Use --no-cache

- Modificou qualquer arquivo .rs
- Modificou configuração que afeta compilação
- Debug de comportamento inesperado
- Após merge/rebase com mudanças de código
- Sempre que binário não reflete código esperado

---

## Docker Compose Integration

### Development compose.yml

```yaml
services:
  service:
    build:
      context: ../..
      dockerfile: products/service/Dockerfile
      # IMPORTANTE: hot reload via volume, não rebuild
    volumes:
      - ../../products/service/src:/build/products/service/src:ro
    command: cargo watch -x 'run -p service'
```

**Key:** Em dev, use `cargo watch` + volume mount, não rebuilds Docker.

### Production/Testing

```bash
# Sempre --no-cache para imagem final
docker compose build --no-cache service
docker compose up service
```

---

## CI/CD Requirements

### Build Pipeline

```yaml
# .github/workflows/build.yml
- name: Build Docker image
  run: |
    docker build \
      --no-cache \
      --pull \
      -t $IMAGE_NAME \
      -f $DOCKERFILE_PATH \
      .
```

**Rationale:** CI deve sempre produzir imagem determinística.

---

## Debugging Checklist

### "Binário não reflete código modificado"

1. Verificar timestamp do arquivo:

```bash
   stat products/service/src/main.rs | grep Modify
```

2. Ver quando imagem foi criada:

```bash
   docker images service --format "{{.CreatedAt}}"
```

3. Se imagem é ANTERIOR ao arquivo:

```bash
   docker build --no-cache ...
```

4. Extrair e verificar strings:

```bash
   docker cp $(docker create service):/usr/local/bin/service /tmp/
   strings /tmp/service | grep "DEBUG_STRING"
```

### "Exit 0 sem logs"

**Sintomas:**

- Container inicia e para imediatamente
- `docker logs` vazio
- `docker run -it` não mostra output
- Exit code 0

**Causa provável:** Binário desatualizado por cache.

**Solução:**

```bash
docker build --no-cache ...
```

---

## Comparison: Development vs Production

| Aspect       | Development                | Production              |
| ------------ | -------------------------- | ----------------------- |
| **Strategy** | Volume mount + cargo watch | Docker build --no-cache |
| **Rebuild**  | Automático (hot reload)    | Manual/CI               |
| **Cache**    | Não usa Docker build       | Sempre --no-cache       |
| **Speed**    | Instant (incremental)      | Slower (full rebuild)   |
| **Accuracy** | 100% (código live)         | 100% (--no-cache)       |

---

## Monorepo-Specific Considerations

### Context Path

```bash
# Build context DEVE ser raiz do monorepo
cd /opt/docker/monorepo
docker build -f products/service/Dockerfile .

# ❌ ERRADO: context no subdir
cd products/service
docker build .  # Não encontra packages-rust/
```

### Cargo Workspace

```dockerfile
# COPY ordem CRÍTICA para cache efetivo
COPY Cargo.toml Cargo.lock ./
COPY packages-rust/ ./packages-rust/
COPY products/ ./products/
# Agora modificar products/ invalida apenas último layer
```

**Trade-off:** Granularidade de cache vs simplicidade de `--no-cache`.

---

## Performance Implications

### --no-cache Impact

| Build Type    | Time  | When to Use              |
| ------------- | ----- | ------------------------ |
| Cached        | ~10s  | Dependencies não mudaram |
| Partial cache | ~1min | Só código mudou          |
| --no-cache    | ~5min | Garantia de fresh build  |

**Recommendation:** Use `--no-cache` sempre que houver QUALQUER dúvida.

---

## Normative Status

- Violations **MUST** block deployment
- All build scripts **MUST** document cache strategy
- CI/CD **MUST** use `--no-cache` ou equivalente
- Debugging **MUST** start com rebuild --no-cache

---

## Examples

### ✅ Correto: Script de build robusto

```bash
#!/bin/bash
set -e

echo "Building with --no-cache to ensure fresh binary..."
docker build \
  --no-cache \
  --pull \
  -f products/core-auth/backend-api/Dockerfile \
  -t core-auth-backend-api \
  .

echo "Verifying binary..."
docker run --rm core-auth-backend-api --version
```

### ❌ Errado: Build incremental após código mudar

```bash
# Modificou main.rs
vim src/main.rs

# Build sem --no-cache
docker build -t service .  # ❌ Pode usar cache obsoleto

# Executar
docker run service  # ❌ Comportamento inesperado
```

### ✅ Correto: Development workflow

```bash
# docker-compose.dev.yml
services:
  backend-api:
    build:
      context: ../..
      dockerfile: products/core-auth/backend-api/Dockerfile.dev
    volumes:
      - ../../products/core-auth/backend-api/src:/build/products/core-auth/backend-api/src:ro
    command: cargo watch -x 'run -p core-auth-backend-api'
```

---

**Author:** DevOps Working Group  
**Date:** 2025-01-06  
**Version:** 1.0  
**Replaces:** None  
**Related:** Canon Rule #8 (Overlay Islands)

---

## ⚠️ Additional Root Cause: Wrong Binary Target

**Even with `--no-cache`, Docker can still produce an unexpected binary** if:

- `Cargo.toml` defines `[[bin]]` with custom `path`
- `src/bin/*.rs` exists alongside `src/main.rs`
- Binary name matches, but entrypoint file is different
- Cargo silently picks wrong source file

### Real Example

```toml
# Cargo.toml
[[bin]]
name = "service"
path = "src/bin/alternative.rs"  # ← Hidden definition

# You edit src/main.rs
# But cargo builds src/bin/alternative.rs
# Result: Changes never appear, even with --no-cache
```

### Mandatory Verification

```bash
# 1. Find exact binary source path
cargo metadata --no-deps --format-version 1 \
  | jq -r '.packages[].targets[] | select(.kind[] == "bin") | "\(.name): \(.src_path)"'

# 2. Verify your changes are in the CORRECT file
# Expected output:
# service: /path/to/products/service/src/main.rs

# 3. Check for multiple bin definitions
grep -n "^\[\[bin\]\]" Cargo.toml
```

### Diagnostic Steps

If binary behavior doesn't match code after `--no-cache`:

```bash
# Step 1: Extract and verify binary contains expected strings
docker create --name verify service
docker cp verify:/usr/local/bin/service /tmp/service-binary
docker rm verify
strings /tmp/service-binary | grep "YOUR_DEBUG_STRING"

# Step 2: If strings missing, check Cargo resolution
cd products/service
cargo build --release -vv 2>&1 | grep "Compiling.*main"

# Step 3: Verify only ONE main.rs/bin exists
find . -name "main.rs" -o -path "*/bin/*.rs" | grep -v target
```

### Normative Requirements

Every production Rust service **MUST**:

- [ ] Have exactly ONE binary entrypoint
- [ ] Document `[[bin]]` path explicitly in Cargo.toml comments
- [ ] Include verification in README:

```bash
  # Verify binary source
  cargo metadata --no-deps | jq '.packages[].targets[] | select(.kind[] == "bin")'
```

- [ ] Add debug `println!` in `main()` first line for build verification
- [ ] Test extracted binary matches expected behavior before deployment

### Failure Mode

**Symptom:** Binary exits silently (exit 0) with no logs, even with:

- `--no-cache` build
- `panic!()` in code
- `RUST_BACKTRACE=1`
- Manual execution

**Root Cause:** Cargo compiled different file than you edited.

**Resolution:**

1. Run `cargo metadata` to find actual source
2. Verify `[[bin]]` definition in Cargo.toml
3. Check for `src/bin/` directory
4. Remove ambiguous entrypoints
5. Rebuild with `--no-cache`

---

## Integration: Cache + Binary Target

Both issues combine when:

```mermaid
graph TD
    A[Edit src/main.rs] --> B[docker build]
    B --> C{Cache hit on COPY?}
    C -->|Yes| D[Old binary used]
    C -->|No| E[cargo build runs]
    E --> F{Correct [[bin]] path?}
    F -->|No| G[Compiles wrong file]
    F -->|Yes| H[Correct binary]
    D --> I[Exit 0 silent]
    G --> I
    H --> J[Expected behavior]
```

**Defense:**

1. Always use `--no-cache` after code changes
2. Always verify binary source path in Cargo.toml
3. Always test extracted binary with expected debug output

---

**Last Updated:** 2025-01-06  
**Version:** 1.1  
**Added:** Binary target verification section
