use std::fs;
use std::path::Path;

struct FontFace {
    family: &'static str,
    weight: u32,
    style: &'static str,
    subsets: &'static [&'static str],
    npm_package: &'static str,
}

const FONTS: &[FontFace] = &[
    FontFace { family: "IBM Plex Sans",  weight: 400, style: "normal", subsets: &["latin", "latin-ext"], npm_package: "ibm-plex-sans" },
    FontFace { family: "IBM Plex Sans",  weight: 500, style: "normal", subsets: &["latin", "latin-ext"], npm_package: "ibm-plex-sans" },
    FontFace { family: "IBM Plex Sans",  weight: 600, style: "normal", subsets: &["latin", "latin-ext"], npm_package: "ibm-plex-sans" },
    FontFace { family: "IBM Plex Sans",  weight: 700, style: "normal", subsets: &["latin", "latin-ext"], npm_package: "ibm-plex-sans" },
];

// Fonte npm base path (relativo ao cwd do tokens-engine, que roda de canonrs-tokens/)
const FONTS_NPM_BASE: &str = "fonts/node_modules/@fontsource";

// Onde as fontes serao copiadas para ser servidas pelo canonrs-server
const FONTS_OUT_DIR: &str = "../canonrs-server/styles/fonts";

// URL base que o browser vai usar para requisitar as fontes
const FONTS_URL_BASE: &str = "/fonts";

pub fn generate(output_dir: &Path) {
    // Garante diretorio de saida das fontes no server
    let fonts_out = Path::new(FONTS_OUT_DIR);
    fs::create_dir_all(fonts_out).expect("Failed to create fonts output dir");

    let mut css = String::from("/* AUTO-GENERATED - FONTS - @font-face declarations */\n\n");

    for font in FONTS {
        for subset in font.subsets {
            let filename = format!(
                "{}-{}-{}-{}.woff2",
                font.npm_package, subset, font.weight, font.style
            );
            let filename_woff = format!(
                "{}-{}-{}-{}.woff",
                font.npm_package, subset, font.weight, font.style
            );

            let src_woff2 = format!("{}/{}/files/{}", FONTS_NPM_BASE, font.npm_package, filename);
            let src_woff  = format!("{}/{}/files/{}", FONTS_NPM_BASE, font.npm_package, filename_woff);

            let src_path_woff2 = Path::new(&src_woff2);
            let src_path_woff  = Path::new(&src_woff);

            // Copia woff2
            if src_path_woff2.exists() {
                fs::copy(src_path_woff2, fonts_out.join(&filename))
                    .expect("Failed to copy woff2");
            } else {
                println!("  ⚠ Missing: {}", src_woff2);
                continue;
            }

            // Copia woff (fallback)
            let has_woff = src_path_woff.exists();
            if has_woff {
                fs::copy(src_path_woff, fonts_out.join(&filename_woff))
                    .expect("Failed to copy woff");
            }

            // Gera @font-face
            let unicode_range = if *subset == "latin-ext" {
                "U+0100-02AF, U+0304, U+0308, U+0329, U+1E00-1E9F, U+1EF2-1EFF, U+2020, U+20A0-20AB, U+20AD-20CF, U+2113, U+2C60-2C7F, U+A720-A7FF"
            } else {
                "U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+0304, U+0308, U+0329, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD"
            };

            if has_woff {
                css.push_str(&format!(
                    "@font-face {{\n  font-family: '{}';\n  font-style: {};\n  font-weight: {};\n  font-display: swap;\n  src: url('{}/{}') format('woff2'),\n       url('{}/{}') format('woff');\n  unicode-range: {};\n}}\n\n",
                    font.family, font.style, font.weight,
                    FONTS_URL_BASE, filename,
                    FONTS_URL_BASE, filename_woff,
                    unicode_range
                ));
            } else {
                css.push_str(&format!(
                    "@font-face {{\n  font-family: '{}';\n  font-style: {};\n  font-weight: {};\n  font-display: swap;\n  src: url('{}/{}') format('woff2');\n  unicode-range: {};\n}}\n\n",
                    font.family, font.style, font.weight,
                    FONTS_URL_BASE, filename,
                    unicode_range
                ));
            }
        }
    }

    fs::write(output_dir.join("fonts.css"), &css).expect("Failed to write fonts.css");
    println!("  ✓ fonts.css ({} @font-face declarations)", FONTS.len() * 2);
}
