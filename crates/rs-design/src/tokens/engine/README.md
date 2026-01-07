# Theme Engine

Sistema de build-time para geração de CSS a partir de definições TypeScript.

## Arquitetura
```
ThemeDefinition (TS) → Build → CSS Variables → Runtime (data-theme)
```

## Como funciona

1. **Design Time**: Definir temas em `themes/presets/*.ts`
2. **Build Time**: Rodar `npm run build:themes`
3. **Runtime**: `ThemeEngine` aplica `data-theme` attribute

## Adicionar novo tema

1. Criar `themes/presets/meu-tema.ts`:
```typescript
import { ThemeDefinition } from '../../engine/schema';

export const meuTema: ThemeDefinition = {
  version: 1,
  id: 'meu-tema',
  name: 'Meu Tema',
  // ...
};
```

2. Exportar em `themes/presets/index.ts`:
```typescript
export { meuTema } from './meu-tema';
```

3. Adicionar no Registry (Rust):
```rust
// src/themes/registry.rs
ThemePreset {
    id: "meu-tema",
    label: "Meu Tema",
    description: "...",
},
```

4. Rebuild:
```bash
npm run build:themes
```

## TODO: Auto-sync Registry

Próxima evolução: gerar `themes.manifest.json` no build e carregar dinamicamente no Registry (eliminar duplicação).
