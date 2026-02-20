use super::pillar::CanonPillar;

pub const CANON_PILLARS: &[CanonPillar] = &[
    CanonPillar {
        id: "architecture",
        label: "Architecture + UI",
        icon: "/assets/pillars/architeture-ui.svg",
        rule_doc: "INDEX.md",
    },
    CanonPillar {
        id: "ssr-wasm",
        label: "SSR + WASM First",
        icon: "/assets/pillars/ssr-wasm-first.svg",
        rule_doc: "canon-rule-04-hydration.md",
    },
    CanonPillar {
        id: "hydration",
        label: "Hydration Safety",
        icon: "/assets/pillars/hidration.svg",
        rule_doc: "canon-rule-04-hydration.md",
    },
    CanonPillar {
        id: "state",
        label: "Predictable State",
        icon: "/assets/pillars/predictable-state.svg",
        rule_doc: "canon-rule-06-visual-state.md",
    },
    CanonPillar {
        id: "tokens",
        label: "Token Governance",
        icon: "/assets/pillars/token-governance.svg",
        rule_doc: "canon-rule-08-token-governance.md",
    },
    CanonPillar {
        id: "rules",
        label: "Rules over Conventions",
        icon: "/assets/pillars/rules-over-conventions.svg",
        rule_doc: "START-HERE.md",
    },
    CanonPillar {
        id: "stable",
        label: "Stability",
        icon: "/assets/pillars/stable.svg",
        rule_doc: "canon-rule-50-provider-singleton-pattern.md",
    },
];
