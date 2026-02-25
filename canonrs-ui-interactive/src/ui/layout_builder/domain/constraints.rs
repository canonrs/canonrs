// ─── Constraint Engine ────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NodeCategory {
    Layout, Form, FormField, Nav, Content, Text, Page, Overlay,
}

pub fn allowed_children(category: NodeCategory) -> &'static [NodeCategory] {
    match category {
        NodeCategory::Layout => &[
            NodeCategory::Layout, NodeCategory::Form, NodeCategory::Nav,
            NodeCategory::Content, NodeCategory::Overlay, NodeCategory::Text,
        ],
        NodeCategory::Form => &[NodeCategory::FormField, NodeCategory::Nav, NodeCategory::Content],
        NodeCategory::Page => &[NodeCategory::Nav, NodeCategory::Content],
        NodeCategory::Overlay => &[NodeCategory::Form, NodeCategory::Content, NodeCategory::Nav],
        _ => &[],
    }
}

pub fn slot_accepts(category: NodeCategory) -> bool {
    category != NodeCategory::FormField
}

pub fn region_placeholder(region_id: &str) -> &'static str {
    match region_id {
        "header"           => "Drop text or navigation",
        "footer"           => "Drop actions or links",
        "content" | "body" => "Drop components or text",
        "actions"          => "Drop buttons or actions",
        "left"             => "Drop navigation",
        "right"            => "Drop actions",
        "center"           => "Drop content",
        "fields"           => "Drop form fields",
        _                  => "Drop components here",
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcceptRule {
    Category(NodeCategory),
    Block(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RegionLayout {
    Single, Flex, Grid(u8),
}

impl Default for RegionLayout {
    fn default() -> Self { Self::Single }
}

#[derive(Debug)]
pub struct BlockRegion {
    pub id: &'static str,
    pub label: &'static str,
    pub accepts: &'static [AcceptRule],
    pub max_children: Option<usize>,
    pub layout: RegionLayout,
}

impl BlockRegion {
    pub const fn new(id: &'static str, label: &'static str, accepts: &'static [AcceptRule]) -> Self {
        Self { id, label, accepts, max_children: None, layout: RegionLayout::Single }
    }
    pub fn accepts_block(&self, block_id: &str, category: NodeCategory) -> bool {
        for rule in self.accepts {
            match rule {
                AcceptRule::Category(c) if *c == category => return true,
                AcceptRule::Block(id) if *id == block_id  => return true,
                _ => {}
            }
        }
        false
    }
    pub fn is_full(&self, current_count: usize) -> bool {
        self.max_children.map(|max| current_count >= max).unwrap_or(false)
    }
}

#[derive(Clone, Debug)]
pub struct BlockDef {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
    pub category: NodeCategory,
    pub is_container: bool,
    pub regions: &'static [BlockRegion],
}

impl PartialEq for BlockDef {
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}

impl BlockDef {
    pub fn can_accept(&self, child_category: NodeCategory) -> bool {
        if !self.is_container { return false; }
        allowed_children(self.category).contains(&child_category)
    }
    pub fn has_regions(&self) -> bool { !self.regions.is_empty() }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComponentDef {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
    pub category: NodeCategory,
    pub description: &'static str,
}
