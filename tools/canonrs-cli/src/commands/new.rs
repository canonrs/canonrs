use anyhow::{Result, Context};
use colored::*;
use std::fs;
use std::path::Path;

pub fn execute(name: &str) -> Result<()> {
    let app_dir = std::env::current_dir()?.join(name);

    if app_dir.exists() {
        anyhow::bail!("Directory '{}' already exists", name);
    }

    println!("{}", format!("🚀 Creating CanonRS app '{}'...", name).cyan());

    create_structure(&app_dir, name)?;

    println!("{}", "✅ Done!".green());
    println!();
    println!("Next steps:");
    println!("  {} {}", "cd".yellow(), name);
    println!("  {}", "canonrs dev".yellow());

    Ok(())
}

fn create_structure(app_dir: &Path, name: &str) -> Result<()> {
    // Directories
    for dir in &[
        "src/pages",
        "src/layout",
        "style",
        "public/style",
        "public/pkg",
    ] {
        fs::create_dir_all(app_dir.join(dir))?;
    }

    // Cargo.toml
    fs::write(app_dir.join("Cargo.toml"), cargo_toml(name))
        .context("Failed to write Cargo.toml")?;

    // canonrs.toml
    fs::write(app_dir.join("canonrs.toml"), canonrs_toml())
        .context("Failed to write canonrs.toml")?;

    // src/lib.rs
    fs::write(app_dir.join("src/lib.rs"), lib_rs(name))
        .context("Failed to write src/lib.rs")?;

    // src/main.rs
    fs::write(app_dir.join("src/main.rs"), main_rs(name))
        .context("Failed to write src/main.rs")?;

    // src/pages/home.rs
    fs::write(app_dir.join("src/pages/home.rs"), home_rs())
        .context("Failed to write src/pages/home.rs")?;

    // src/pages/mod.rs
    fs::write(app_dir.join("src/pages/mod.rs"), "pub mod home;\\n")
        .context("Failed to write src/pages/mod.rs")?;

    // style/main.css
    fs::write(app_dir.join("style/main.css"), style_css())
        .context("Failed to write style/main.css")?;

    // Makefile
    fs::write(app_dir.join("Makefile"), makefile(name))
        .context("Failed to write Makefile")?;

    // .gitignore
    fs::write(app_dir.join(".gitignore"), gitignore())
        .context("Failed to write .gitignore")?;

    // postcss.config.cjs
    fs::write(app_dir.join("postcss.config.cjs"), postcss_config())
        .context("Failed to write postcss.config.cjs")?;

    println!("  {} project structure created", "✓".green());
    Ok(())
}

fn cargo_toml(name: &str) -> String {
    format!(r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[[bin]]
name = "{name}"
path = "src/main.rs"

[dependencies]
leptos = {{ version = "0.8", default-features = false }}
leptos_meta = {{ version = "0.8", default-features = false }}
leptos_router = {{ version = "0.8", default-features = false }}
leptos_axum = {{ version = "0.8", optional = true }}
axum = {{ version = "0.8", optional = true }}
tower-http = {{ version = "0.6", optional = true, features = ["fs", "compression-br", "compression-gzip"] }}
tokio = {{ version = "1", features = ["full"], optional = true }}
canonrs = {{ path = "../../packages-rust/rs-canonrs/canonrs", default-features = false }}
console_error_panic_hook = "0.1"
wasm-bindgen = "0.2"
serde = {{ version = "1", features = ["derive"] }}

[features]
default = []
hydrate = ["leptos/hydrate", "canonrs/hydrate"]
ssr = ["canonrs/ssr", "leptos/ssr", "leptos_meta/ssr", "leptos_router/ssr", "leptos_axum", "axum", "tower-http", "tokio"]

[package.metadata.leptos]
bin-features = ["ssr"]
lib-features = ["hydrate"]
lib-profile-release = "wasm-release"

[profile.wasm-release]
inherits = "release"
opt-level = "z"
lto = "fat"
codegen-units = 1
panic = "abort"
strip = "symbols"

[profile.release]
lto = false
strip = false
codegen-units = 16
opt-level = 2

[profile.dev]
opt-level = 1
"#, name = name)
}

fn canonrs_toml() -> &'static str {
    r#"[app]
mode = "ssr"
"#
}

fn lib_rs(name: &str) -> String {
    let component_name = to_pascal_case(name);
    format!(r#"use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::StaticSegment;
use canonrs::providers::prelude::*;

pub fn shell(options: leptos::config::LeptosOptions) -> impl IntoView {{
    view! {{
        <!DOCTYPE html>
        <html lang="en" data-theme="canonrs-theme">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <leptos_meta::AutoReload options=options.clone()/>
                <leptos_axum::HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <{component_name}App/>
            </body>
        </html>
    }}
}}

#[component]
pub fn {component_name}App() -> impl IntoView {{
    provide_meta_context();
    view! {{
        <Stylesheet id="canonrs" href="/canonrs.css"/>
        <Stylesheet id="app" href="/style/output.css"/>
        <Title text="{name}"/>
        <CanonRSRoot>
            <Router>
                <Routes fallback=|| view! {{ "Not found" }}>
                    <Route path=StaticSegment("") view=pages::home::HomePage/>
                </Routes>
            </Router>
        </CanonRSRoot>
    }}
}}

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {{
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body({component_name}App);
    #[cfg(target_arch = "wasm32")]
    canonrs::behaviors::init_canonrs_behaviors();
}}

mod pages;
"#, name = name, component_name = component_name)
}

fn main_rs(name: &str) -> String {
    let component_name = to_pascal_case(name);
    format!(r#"#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {{
    use axum::{{Router, http::header}};
    use leptos::prelude::*;
    use leptos_axum::{{generate_route_list, LeptosRoutes}};
    use tower_http::compression::CompressionLayer;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options.clone();

    let app = Router::new()
        .route("/canonrs.css", axum::routing::get(|| async {{
            (
                [(header::CONTENT_TYPE, "text/css")],
                canonrs::canonrs_css()
            )
        }}))
        .leptos_routes(
            &leptos_options,
            generate_route_list({crate_name}::{component_name}App),
            {{
                let opts = leptos_options.clone();
                move || {crate_name}::shell(opts.clone())
            }},
        )
        .fallback(leptos_axum::file_and_error_handler({{
            let opts = leptos_options.clone();
            move |_| {crate_name}::shell(opts.clone())
        }}))
        .layer(CompressionLayer::new())
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🚀 {{}} running on http://{{addr}}", "{name}");
    axum::serve(listener, app.into_make_service()).await.unwrap();
}}

#[cfg(not(feature = "ssr"))]
fn main() {{
    println!("Enable 'ssr' feature");
}}
"#, name = name, crate_name = name.replace('-', "_"), component_name = component_name)
}

fn home_rs() -> &'static str {
    r#"use leptos::prelude::*;
use canonrs::layouts::{PageHeader, Section};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Section>
            <PageHeader
                title="Welcome"
                subtitle="Your CanonRS app is ready."
            />
        </Section>
    }
}
"#
}

fn style_css() -> &'static str {
    r#"@import "canonrs.css";
@import "tailwindcss";
"#
}

fn makefile(name: &str) -> String {
    format!(r#".DEFAULT_GOAL := dev

WASM := /opt/docker/monorepo/target/site/pkg/{name}.wasm
WASM_DIR := /opt/docker/monorepo/target/site/pkg

dev:
	canonrs dev

build:
	canonrs build
	wasm-opt -Oz --strip-debug --strip-producers $(WASM) -o $(WASM)
	@echo "✅ WASM: $$(ls -lh $(WASM) | awk '{{print $$5}}')"

.PHONY: dev build
"#, name = name.replace('-', "_"))
}

fn gitignore() -> &'static str {
    r#"/target
/.canonrs/workspace/target
node_modules
style/output.css
public/style/output.css
public/pkg
"#
}

fn postcss_config() -> &'static str {
    r#"const path = require('path');

module.exports = {
  plugins: {
    'postcss-import': {
      path: [
        path.resolve(__dirname, 'style'),
        path.resolve(__dirname, '../../packages-rust/rs-canonrs/canonrs-server/styles')
      ]
    },
    '@tailwindcss/postcss': {},
    autoprefixer: {}
  }
};
"#
}

fn to_pascal_case(s: &str) -> String {
    s.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect()
}
