//! gen_showcase.rs — gera src/generated/showcase.json
//! SOURCE: canonrs-server/src/ui/*/builder.yaml

use std::path::Path;
use std::fs;
use crate::build::types::ShowcaseEntry;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct BuilderYaml {
    id:                Option<String>,
    label:             Option<String>,
    category:          Option<String>,
    description:       Option<String>,
    keywords:          Option<String>,
    pain:              Option<String>,
    promise:           Option<String>,
    why:               Option<String>,
    before:            Option<String>,
    after:             Option<String>,
    rules:             Option<Vec<String>>,
    use_cases:         Option<Vec<String>>,
    related:           Option<Vec<String>>,
    badges:            Option<Vec<String>>,
    pillar:            Option<String>,
    file:              Option<String>,
    tokens:            Option<String>,
    foundation:        Option<String>,
    states:            Option<Vec<String>>,
    boundary:          Option<String>,
    block:             Option<Vec<String>>,
    blocks_primitives: Option<Vec<String>>,
}

pub(crate) fn generate_showcase(ui_dir: &Path, out_path: &Path) {
    let mut entries: Vec<ShowcaseEntry> = vec![];

    let dir_entries = match fs::read_dir(ui_dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in dir_entries.flatten() {
        let path = entry.path();
        if !path.is_dir() { continue; }

        let builder_file = path.join("builder.yaml");
        let content = match fs::read_to_string(&builder_file) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let builder: BuilderYaml = match serde_yaml::from_str(&content) {
            Ok(b) => b,
            Err(e) => {
                println!("cargo:warning=Showcase YAML parse error in {:?}: {}", builder_file, e);
                continue;
            }
        };

        let id = match builder.id {
            Some(v) if !v.is_empty() => v,
            _ => {
                println!("cargo:warning=Showcase missing id in {:?}", builder_file);
                continue;
            }
        };

        let primitive_src = {
            let p = ui_dir.parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .map(|p| p.join("canonrs-core/src/primitives"))
                .unwrap_or_default()
                .join(format!("{}.rs", id.replace('-', "_")));
            fs::read_to_string(&p).unwrap_or_default()
        };

        let ui_src = fs::read_to_string(
            path.join(format!("{}_ui.rs", id.replace('-', "_")))
        ).unwrap_or_default();

        let boundary_src = fs::read_to_string(
            path.join(format!("{}_boundary.rs", id.replace('-', "_")))
        ).unwrap_or_default();

        entries.push(ShowcaseEntry {
            id,
            label:         builder.label.unwrap_or_default(),
            category:      builder.category.unwrap_or_default(),
            description:   builder.description.unwrap_or_default(),
            keywords:      builder.keywords.unwrap_or_default(),
            pain:          builder.pain.unwrap_or_default(),
            promise:       builder.promise.unwrap_or_default(),
            why:           builder.why.unwrap_or_default(),
            before:        builder.before.unwrap_or_default(),
            after:         builder.after.unwrap_or_default(),
            rules:         builder.rules.unwrap_or_default(),
            use_cases:     builder.use_cases.unwrap_or_default(),
            related:       builder.related.unwrap_or_default(),
            badges:        builder.badges.unwrap_or_default(),
            pillar:        builder.pillar.unwrap_or_default(),
            primitive_src,
            ui_src,
            boundary_src,
            block:             builder.block.unwrap_or_default(),
            blocks_primitives: builder.blocks_primitives.unwrap_or_default(),
        });
    }

    entries.sort_by(|a, b| a.id.cmp(&b.id));

    let json = serde_json::to_string_pretty(&entries)
        .expect("showcase json serialize failed");

    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent).ok();
    }
    fs::write(out_path, json).expect("showcase json write failed");
    println!("cargo:warning=CanonRS Showcase: showcase.json ({} components)", entries.len());
}
