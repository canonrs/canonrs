use rs_canonrs::domain::{CanonBlockType, CanonDocument, CanonNode};

/// Exporta um CanonDocument para código Leptos/Rust válido
pub fn export_document_to_rs(doc: &CanonDocument) -> String {
    let mut out = String::new();

    out.push_str("// ============================================================\n");
    out.push_str(&format!("// CanonRS Export — {} (layout: {})\n", doc.id, doc.layout));
    out.push_str("// ============================================================\n\n");
    out.push_str("use leptos::prelude::*;\n");
    out.push_str("use canonrs_ui::ui::*;\n\n");
    out.push_str("#[component]\n");
    out.push_str("pub fn ExportedLayout() -> impl IntoView {\n");
    out.push_str("    view! {\n");

    for node in &doc.nodes {
        render_node_rs(node, 2, &mut out);
    }

    out.push_str("    }\n");
    out.push_str("}\n");
    out
}

fn indent(depth: usize) -> String {
    "    ".repeat(depth)
}

fn render_node_rs(node: &CanonNode, depth: usize, out: &mut String) {
    let pad = indent(depth);
    match &node.block {
        CanonBlockType::Slot { name } => {
            out.push_str(&format!("{}// Slot: {}\n", pad, name));
            out.push_str(&format!("{}<div data-slot=\"{}\">\n", pad, name));
            for child in &node.children {
                render_node_rs(child, depth + 1, out);
            }
            out.push_str(&format!("{}</div>\n", pad));
        }
        CanonBlockType::Card => render_container(node, depth, out, "<Card>", "</Card>"),
        CanonBlockType::Section => render_container(node, depth, out, "<Section>", "</Section>"),
        CanonBlockType::Layout => render_container(node, depth, out, "<div data-layout=\"\">", "</div>"),
        CanonBlockType::Dialog => render_container(node, depth, out, "<Dialog>", "</Dialog>"),
        CanonBlockType::Drawer => render_container(node, depth, out, "<Drawer>", "</Drawer>"),
        CanonBlockType::Popover => render_container(node, depth, out, "<Popover>", "</Popover>"),
        CanonBlockType::Header => render_leaf(depth, out, "<Header />"),
        CanonBlockType::Footer => render_leaf(depth, out, "<Footer />"),
        CanonBlockType::PageHeader => render_leaf(depth, out, "<PageHeader title=\"\" description=\"\" />"),
        CanonBlockType::Breadcrumb => render_leaf(depth, out, "<Breadcrumb items=vec![] />"),
        CanonBlockType::Toolbar => render_container(node, depth, out, "<Toolbar aria_label=\"\">", "</Toolbar>"),
        CanonBlockType::ButtonGroup => render_container(node, depth, out, "<ButtonGroup>", "</ButtonGroup>"),
        CanonBlockType::Form => render_container(node, depth, out, "<Form>", "</Form>"),
        CanonBlockType::Field => render_leaf(depth, out, "<Field label=\"\" />"),
        CanonBlockType::FormActions => render_container(node, depth, out, "<FormActions>", "</FormActions>"),
        CanonBlockType::Alert => render_leaf(depth, out, "<Alert variant=AlertVariant::Info />"),
        CanonBlockType::Callout => render_leaf(depth, out, "<Callout variant=CalloutVariant::Default />"),
        CanonBlockType::StatCard => render_leaf(depth, out, "<StatCard label=\"Metric\" value=\"0\" />"),
        CanonBlockType::EmptyState => render_leaf(depth, out, "<EmptyState title=\"No data\" description=\"\" />"),
        CanonBlockType::DataTable => render_leaf(depth, out, "<DataTable columns=vec![] rows=vec![] />"),
        CanonBlockType::CodeBlock => render_leaf(depth, out, "<CodeBlock language=\"rust\" code=\"\" />"),
        CanonBlockType::List => render_container(node, depth, out, "<List>", "</List>"),
        CanonBlockType::Skeleton => render_leaf(depth, out, "<Skeleton />"),
        CanonBlockType::Table => render_leaf(depth, out, "<Table columns=vec![] rows=vec![] />"),
        CanonBlockType::CommandPanel => render_leaf(depth, out, "<CommandPanel placeholder=\"Search...\" items=vec![] />"),
    }
}

fn render_container(node: &CanonNode, depth: usize, out: &mut String, open: &str, close: &str) {
    let pad = indent(depth);
    if node.children.is_empty() {
        let self_closing = open.replace('>', " />");
        out.push_str(&format!("{}{}\n", pad, self_closing));
    } else {
        out.push_str(&format!("{}{}\n", pad, open));
        for child in &node.children {
            render_node_rs(child, depth + 1, out);
        }
        out.push_str(&format!("{}{}\n", pad, close));
    }
}

fn render_leaf(depth: usize, out: &mut String, tag: &str) {
    let pad = indent(depth);
    out.push_str(&format!("{}{}\n", pad, tag));
}
