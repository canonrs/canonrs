# CanonRS CLI

Design system governance as infrastructure.

## Installation
```bash
# Via cargo
cargo install canonrs

# Via homebrew (coming soon)
brew install canonrs/tap/canonrs
```

## Quick Start
```bash
# Initialize canon in your project
canonrs init

# Check compliance
canonrs check

# Generate HTML report
canonrs report --format=html --output=report.html

# Annotate legacy file
canonrs annotate src/Button.tsx \
  --rule=#21 \
  --justification="Legacy component" \
  --owner=ui-team \
  --target-date=2025-Q2
```

## Commands

### `canonrs init`
Initialize canon configuration in your project.

### `canonrs check`
Validate design system compliance.

Options:
- `--level` - Compliance level (strict/standard/loose)
- `--strict` - Fail on warnings
- `--ci` - CI mode with exit codes

### `canonrs report`
Generate compliance reports.

Formats:
- `html` - Interactive HTML report
- `json` - Machine-readable JSON
- `markdown` - Markdown summary

### `canonrs annotate <file>`
Add exception annotations to legacy files.

### `canonrs violations`
List all canon violations.

### `canonrs providers`
Validate provider architecture (Canon Rule #37).

### `canonrs tokens`
Validate token usage (Canon Rule #35).

## Configuration

`.canon/canon.toml`:
```toml
version = "1.0"

[compliance]
level = "standard"
fail_on_warnings = false

[providers]
allowed = ["ThemeProvider", "DensityProvider", "LanguageProvider"]

[tokens]
enforce_colors = true
enforce_sizes = true

[exceptions]
max_percentage = 10
```

## CI Integration

### GitHub Actions
```yaml
- name: Canon Check
  run: canonrs check --ci --strict
```

### GitLab CI
```yaml
canon:
  script:
    - canonrs check --ci
```

## License

MIT
