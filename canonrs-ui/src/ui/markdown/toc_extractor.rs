use pulldown_cmark::{Parser, Event, Tag, TagEnd, HeadingLevel};
use canonrs_shared::TocItem;

pub struct TocExtractor;

impl TocExtractor {
    pub fn extract_toc(markdown: &str) -> Vec<TocItem> {
        let parser = Parser::new(markdown);
        let mut toc_items = Vec::new();
        let mut in_heading = false;
        let mut current_level = 0u8;
        let mut current_text = String::new();
        let mut heading_counter = 0;

        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    in_heading = true;
                    current_level = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                    current_text.clear();
                }
                Event::End(TagEnd::Heading { .. }) => {
                    if in_heading {
                        let id = if current_text.is_empty() {
                            format!("heading-{}", heading_counter)
                        } else {
                            slugify(&current_text)
                        };
                        toc_items.push(TocItem::new(id, current_text.clone(), current_level));
                        heading_counter += 1;
                        in_heading = false;
                    }
                }
                Event::Text(text) if in_heading => {
                    current_text.push_str(&text);
                }
                _ => {}
            }
        }

        toc_items
    }
}

pub fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() { c }
            else if c.is_whitespace() || c == '-' { '-' }
            else { '_' }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

impl TocExtractor {
    pub fn extract_toc_with_prefix(markdown: &str, id_prefix: &str) -> Vec<TocItem> {
        let parser = Parser::new(markdown);
        let mut toc_items = Vec::new();
        let mut in_heading = false;
        let mut current_level = 0u8;
        let mut current_text = String::new();
        let mut heading_counter = 0;

        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    in_heading = true;
                    current_level = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                    current_text.clear();
                }
                Event::End(TagEnd::Heading { .. }) => {
                    if in_heading {
                        let slug = if current_text.is_empty() {
                            format!("heading-{}", heading_counter)
                        } else {
                            slugify(&current_text)
                        };
                        let id = if id_prefix.is_empty() {
                            slug
                        } else {
                            format!("{}-{}", id_prefix, slug)
                        };
                        toc_items.push(TocItem::new(id, current_text.clone(), current_level));
                        heading_counter += 1;
                        in_heading = false;
                    }
                }
                Event::Text(text) if in_heading => {
                    current_text.push_str(&text);
                }
                _ => {}
            }
        }

        toc_items
    }
}
