# CanonRS Token Engine

Processes tokens into consumable artifacts.

## Source of Truth
Theme presets are authored exclusively in CSS.

## Input
```
src/token-engine/ingest/css/*.css
```

## Output
```
src/tokens/themes/presets/*.ts (auto-generated)
style/generated-themes.css
```

## Rules
- Do NOT edit generated TS files manually
- All theme changes must be done in CSS
- Token Engine is the single authority for conversion
- TS exists only for type safety and Rust/JS consumption

## Workflow
```
CSS → Token Engine → Typed ThemeDefinition → Runtime
```

## Commands
```bash
# Ingest CSS to TS
npm run ingest:css ingest/css/theme-name.css

# Build final themes.css
npm run build:themes
```

## Pipeline
1. Create theme in TweakCN
2. Export CSS
3. Save to `ingest/css/`
4. Run `npm run ingest:css`
5. Run `npm run build:themes`
6. TS generated, validated, consumed by Rust
