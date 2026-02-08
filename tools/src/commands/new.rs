use std::path::Path;
use std::fs;
use anyhow::{Result, Context};
use colored::*;
use crate::detect::Mode;
use crate::workspace::generator;

pub fn execute(name: &str) -> Result<()> {
    let app_dir = Path::new(name);
    
    if app_dir.exists() {
        anyhow::bail!("Directory '{}' already exists", name);
    }
    
    println!("{}", "Creating new CanonRS app...".green().bold());
    
    // Create app structure
    fs::create_dir_all(app_dir.join("src/pages"))?;
    fs::create_dir_all(app_dir.join("public/style"))?;
    fs::create_dir_all(app_dir.join("style"))?;
    
    // Create canonrs.toml
    let canonrs_toml = format!(r#"[app]
name = "{}"
mode = "ssr"  # ssr, csr, or hybrid
"#, name);
    fs::write(app_dir.join("canonrs.toml"), canonrs_toml)?;
    
    let lib_name = name.replace("-", "_");
    
    // Create simple Cargo.toml (NO profiles, NO canonrs for now)
    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[[bin]]
name = "{}"
path = "src/main.rs"

[dependencies]
leptos = {{ version = "0.8", features = ["csr", "ssr"] }}
leptos_meta = {{ version = "0.8" }}
leptos_router = {{ version = "0.8" }}
leptos_axum = {{ version = "0.8", optional = true }}
axum = {{ version = "0.8", optional = true }}
tokio = {{ version = "1", features = ["full"], optional = true }}
tower = {{ version = "0.5", optional = true }}
tower-http = {{ version = "0.6", features = ["fs"], optional = true }}
console_error_panic_hook = "0.1"
wasm-bindgen = "0.2"

[features]
hydrate = ["leptos/hydrate"]
ssr = ["leptos/ssr", "leptos_meta/ssr", "leptos_router/ssr", "leptos_axum", "axum", "tokio", "tower", "tower-http"]
"#, name, name);
    fs::write(app_dir.join("Cargo.toml"), cargo_toml)?;
    
    // Create src/lib.rs WITHOUT canonrs
    let lib_rs = format!(r#"use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::StaticSegment;

mod pages;

pub fn shell(options: LeptosOptions) -> impl IntoView {{
    view! {{
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }}
}}

#[component]
pub fn App() -> impl IntoView {{
    provide_meta_context();
    
    view! {{
        <Stylesheet id="main" href="/style/output.css"/>
        <Title text="{} - CanonRS"/>
        
        <Router>
            <Routes fallback=|| view!{{ "Not found" }}>
                <Route path=StaticSegment("") view=pages::home::HomePage/>
            </Routes>
        </Router>
    }}
}}

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {{
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}}
"#, name);
    fs::write(app_dir.join("src/lib.rs"), lib_rs)?;
    
    // Create src/main.rs
    let main_rs = format!(r#"#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {{
    use axum::Router;
    use leptos::config::get_configuration;
    use leptos_axum::{{generate_route_list, LeptosRoutes}};
    use tower_http::services::ServeDir;
    use {}::{{App, shell}};
    
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {{
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        }})
        .nest_service("/style", ServeDir::new("public/style"))
        .with_state(leptos_options);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🚀 Server running on http://{{}}", addr);
    axum::serve(listener, app).await.unwrap();
}}

#[cfg(not(feature = "ssr"))]
pub fn main() {{}}
"#, lib_name);
    fs::write(app_dir.join("src/main.rs"), main_rs)?;
    
    // Create src/pages/mod.rs
    fs::write(app_dir.join("src/pages/mod.rs"), "pub mod home;\n")?;
    
    // Create src/pages/home.rs
    let home_rs = r#"use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div style="padding: 2rem; font-family: system-ui;">
            <h1>"Hello CanonRS!"</h1>
            <p>"Your app is running."</p>
        </div>
    }
}
"#;
    fs::write(app_dir.join("src/pages/home.rs"), home_rs)?;
    
    // Create empty style/main.css
    fs::write(app_dir.join("style/main.css"), "/* Your styles here */\n")?;
    
    // Generate .canonrs workspace
    generator::generate_workspace(app_dir, Mode::SSR)
        .context("Failed to generate workspace")?;
    
    println!("{}", format!("✅ Created {} (SSR mode detected)", name).green());
    println!("\nNext steps:");
    println!("  cd {}", name);
    println!("  canonrs dev");
    
    Ok(())
}
