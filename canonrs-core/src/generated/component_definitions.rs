// AUTO-GENERATED — do not edit. Edit components.toml instead.
use crate::domain::block_definition::{BlockPropDef, BlockPreset, PropFieldType, PropScope};

const BUTTON_PROPS: &[BlockPropDef] = &[
	BlockPropDef { key: "label", label: "Label", field: PropFieldType::Text, default: Some("Button"), scope: PropScope::Visual, css: None },
	BlockPropDef { key: "variant", label: "Variant", field: PropFieldType::Select(&[("default", "default"), ("primary", "primary"), ("secondary", "secondary"), ("destructive", "destructive"), ("outline", "outline"), ("ghost", "ghost")]), default: Some("default"), scope: PropScope::Visual, css: None },
	BlockPropDef { key: "size", label: "Size", field: PropFieldType::Select(&[("sm", "sm"), ("md", "md"), ("lg", "lg")]), default: Some("md"), scope: PropScope::Visual, css: None },
];

const INPUT_PROPS: &[BlockPropDef] = &[
	BlockPropDef { key: "label", label: "Label", field: PropFieldType::Text, default: Some("Label"), scope: PropScope::Visual, css: None },
	BlockPropDef { key: "placeholder", label: "Placeholder", field: PropFieldType::Text, default: Some("Enter value"), scope: PropScope::Visual, css: None },
	BlockPropDef { key: "name", label: "Name", field: PropFieldType::Text, default: Some("field"), scope: PropScope::Visual, css: None },
];

const TABS_PROPS: &[BlockPropDef] = &[
	BlockPropDef { key: "default_value", label: "Default Tab", field: PropFieldType::Text, default: Some("tab1"), scope: PropScope::Visual, css: None },
];

pub static COMPONENT_DEFINITIONS_GENERATED: &[crate::domain::component_definition::ComponentDefinition] = &[
	crate::domain::component_definition::ComponentDefinition { id: "button", label: "Button", icon: "⬚", description: "Action button", requires_config: true, props_schema: BUTTON_PROPS, presets: &[] },
	crate::domain::component_definition::ComponentDefinition { id: "dialog", label: "Dialog", icon: "▢", description: "Modal dialog", requires_config: false, props_schema: &[], presets: &[] },
	crate::domain::component_definition::ComponentDefinition { id: "input", label: "Input", icon: "▱", description: "Text input", requires_config: true, props_schema: INPUT_PROPS, presets: &[] },
	crate::domain::component_definition::ComponentDefinition { id: "carousel", label: "Carousel", icon: "◁▷", description: "Image carousel", requires_config: false, props_schema: &[], presets: &[] },
	crate::domain::component_definition::ComponentDefinition { id: "tabs", label: "Tabs", icon: "⊟", description: "Tabbed navigation", requires_config: true, props_schema: TABS_PROPS, presets: &[] },
];
