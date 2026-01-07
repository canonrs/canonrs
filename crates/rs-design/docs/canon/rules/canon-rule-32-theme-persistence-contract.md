# Canon Rule #32: Theme Persistence Contract

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-02

## Principle
Theme state MUST persist across sessions using **HTTP cookies** (SSR-safe), not browser storage. The contract defines how themes are saved, loaded, and synchronized between server and client.

## Cookie Schema

### Required Cookies
```
theme-mode=<mode>; Path=/; Max-Age=31536000; SameSite=Lax
theme-preset=<preset>; Path=/; Max-Age=31536000; SameSite=Lax
```

**Parameters:**
- `theme-mode`: `"light"` | `"dark"` | `"system"`
- `theme-preset`: Any valid preset ID from `ThemeRegistry`
- `Max-Age`: 1 year (31536000 seconds)
- `SameSite`: Lax (CSRF protection)
- `Path`: `/` (available everywhere)

### Prohibited Storage Methods

❌ `localStorage` - Not SSR-safe, causes flash  
❌ `sessionStorage` - Lost on tab close  
❌ `IndexedDB` - Overcomplicated for theme  
❌ Client-only state - No SSR hydration  

**Exception:** Fallback to `localStorage` only if cookies are disabled by user.

## Architecture Layers

### 1️⃣ Server (SSR)

**Responsibilities:**
- Read cookies from HTTP headers
- Pass initial theme to client
- Generate anti-flash script in `<head>`

**Location:** `apps/*/src/theme_server.rs`
```rust
#[server]
pub async fn get_theme_from_cookie() -> Result<(String, String), ServerFnError> {
    use axum::http::header::COOKIE;
    let headers = extract::<axum::http::HeaderMap>().await?;
    
    // Parse cookies...
    Ok((mode, preset))
}

#[server]
pub async fn set_theme_cookie(mode: String, preset: String) -> Result<(), ServerFnError> {
    let response = expect_context::<ResponseOptions>();
    response.append_header(SET_COOKIE, ...);
    Ok(())
}
```

### 2️⃣ Design System (rs-design)

**Responsibilities:**
- Accept initial theme as props
- Apply theme to DOM (`data-theme` + `class`)
- Provide context to components

**Location:** `packages-rust/rs-design/src/providers/theme_provider.rs`
```rust
#[component]
pub fn ThemeProvider(
    #[prop(optional)] initial_mode: Option<String>,
    #[prop(optional)] initial_preset: Option<String>,
    children: Children,
) -> impl IntoView {
    // Apply to DOM, NO cookie writing here
}
```

**❌ FORBIDDEN in rs-design:**
- Server functions
- Cookie manipulation
- HTTP dependencies

### 3️⃣ Application (App Root)

**Responsibilities:**
- Call server functions on theme change
- Render anti-flash script in `shell()`
- Connect UI components with persistence

**Location:** `apps/*/src/app.rs`
```rust
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <head>
            <script>
                {r#"
                (function() {
                    const getCookie = (name) => { /* ... */ };
                    const mode = getCookie('theme-mode') || 'system';
                    const preset = getCookie('theme-preset') || 'default';
                    
                    // Apply BEFORE any rendering
                    document.documentElement.classList.add(resolveMode(mode));
                    document.documentElement.setAttribute('data-theme', preset);
                })();
                "#}
            </script>
            <link rel="stylesheet" href="/app.css"/>
        </head>
    }
}
```

## Anti-Flash Script

### Requirements

1. **Inline in `<head>`** - Cannot be external file
2. **Before CSS** - Must execute before stylesheets load
3. **Synchronous** - No async/await
4. **Self-contained** - No dependencies
5. **Cookie-based** - Read from `document.cookie`

### Template
```javascript
(function() {
    const getCookie = (name) => {
        const value = `; ${document.cookie}`;
        const parts = value.split(`; ${name}=`);
        if (parts.length === 2) return parts.pop().split(';').shift();
        return null;
    };
    
    const savedMode = getCookie('theme-mode') || 'system';
    const savedPreset = getCookie('theme-preset') || 'default';
    
    let resolvedMode = savedMode;
    if (savedMode === 'system') {
        resolvedMode = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    }
    
    const html = document.documentElement;
    html.classList.remove('light', 'dark');
    html.classList.add(resolvedMode);
    html.setAttribute('data-theme', savedPreset);
})();
```

## Hydration Contract

### Server Render (SSR)
```rust
let (initial_mode, initial_preset) = get_theme_from_cookie();

view! {
    <ThemeProvider initial_mode initial_preset>
        <App />
    </ThemeProvider>
}
```

### Client Hydration
```rust
// ThemeProvider receives initial values from SSR
// Applies them to DOM (already applied by anti-flash script)
// Listens for changes and updates cookies via server function
```

### Synchronization Flow
```
1. User changes theme in UI
   ↓
2. Component calls callback with (mode, preset)
   ↓
3. App calls set_theme_cookie() server function
   ↓
4. Server sets HTTP cookie in response
   ↓
5. ThemeProvider updates DOM (data-theme + class)
   ↓
6. Next page load: cookie is read in SSR
   ↓
7. Anti-flash script applies theme immediately
   ↓
8. ThemeProvider hydrates with same values (no flash)
```

## Security

### CSRF Protection
```
SameSite=Lax
```

Prevents cookie from being sent in cross-site requests (except safe methods like GET).

### XSS Protection

**❌ NEVER:**
```javascript
// DANGEROUS - XSS vulnerability
html.innerHTML = getCookie('theme-preset');
```

**✅ SAFE:**
```javascript
html.setAttribute('data-theme', getCookie('theme-preset'));
```

### Cookie Hijacking

**Not sensitive data** - Theme preference is not a security risk. No HTTPS requirement, but recommended.

## Testing

### SSR Test
```bash
curl -H "Cookie: theme-mode=dark; theme-preset=ocean" http://localhost:3000
# Should return HTML with class="dark" and data-theme="ocean"
```

### Cookie Persistence Test

1. Set theme to Dark + Ocean preset
2. Close browser completely
3. Reopen and navigate to app
4. Verify theme is Dark + Ocean (no flash)

### Flash Prevention Test

1. Set theme to Dark
2. Hard reload (Ctrl+Shift+R)
3. Watch for white flash during load
4. **PASS:** No flash, dark theme from frame 1

### Validation Checklist

- [ ] Cookies set with correct Max-Age (1 year)
- [ ] SameSite=Lax for CSRF protection
- [ ] Anti-flash script in `<head>` before CSS
- [ ] Script reads cookies (not localStorage)
- [ ] SSR passes initial theme to ThemeProvider
- [ ] No flash on page load/reload
- [ ] Theme persists after browser restart
- [ ] Server functions only in app layer

## Migration from localStorage

If migrating from localStorage-based theme:
```javascript
// In anti-flash script, add migration
const legacyMode = localStorage.getItem('theme-mode');
if (legacyMode && !getCookie('theme-mode')) {
    // First visit after migration - convert to cookie
    document.cookie = `theme-mode=${legacyMode}; Path=/; Max-Age=31536000; SameSite=Lax`;
    localStorage.removeItem('theme-mode'); // Clean up
}
```

## Prohibited Patterns

### ❌ Client-Only Persistence
```rust
// WRONG - causes flash
#[component]
pub fn ThemeProvider() -> impl IntoView {
    Effect::new(move |_| {
        localStorage.setItem('theme', mode);
    });
}
```

### ❌ Server Functions in Design System
```rust
// WRONG - violates architecture
// File: packages-rust/rs-design/src/providers/theme_provider.rs
#[server]
fn save_theme() { /* ... */ } // ❌ NO!
```

### ❌ Async Script Loading
```html
<!-- WRONG - script loads after CSS -->
<link rel="stylesheet" href="/app.css">
<script async src="/theme.js"></script>
```

## References

- [MDN: HTTP Cookies](https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies)
- [SameSite Cookie Explained](https://web.dev/samesite-cookies-explained/)
- [Leptos SSR Guide](https://book.leptos.dev/ssr/index.html)
- [Canon Rule #21: Canonical Color Tokens](./canon-rule-21-canonical-color-tokens.md)
- [Canon Rule #25: Theme Presets Contract](./canon-rule-25-theme-presets-contract.md)

---

**Versioning:** Semantic (MAJOR.MINOR.PATCH)  
**Compliance:** Enforced via linting + CI checks  
**Breaking Changes:** Require major version bump
