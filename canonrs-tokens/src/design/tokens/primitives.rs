pub struct PrimitiveToken {
    pub name: &'static str,
    pub value: &'static str,
}

pub const PRIMITIVE_VALUES: &[PrimitiveToken] = &[

    // ======================================================================
    // Neutral palette (raw HSL)
    // ======================================================================

    PrimitiveToken { name: "primitive-neutral-950", value: "220 16% 11%" },
    PrimitiveToken { name: "primitive-neutral-900", value: "217 32% 17%" },
    PrimitiveToken { name: "primitive-neutral-800", value: "215 25% 27%" },
    PrimitiveToken { name: "primitive-neutral-700", value: "217 19% 35%" },
    PrimitiveToken { name: "primitive-neutral-600", value: "215 14% 48%" },
    PrimitiveToken { name: "primitive-neutral-500", value: "220 9% 60%" },
    PrimitiveToken { name: "primitive-neutral-400", value: "218 11% 73%" },
    PrimitiveToken { name: "primitive-neutral-300", value: "216 12% 84%" },
    PrimitiveToken { name: "primitive-neutral-200", value: "220 13% 91%" },
    PrimitiveToken { name: "primitive-neutral-100", value: "220 14% 96%" },
    PrimitiveToken { name: "primitive-neutral-50",  value: "210 20% 98%" },

    PrimitiveToken { name: "primitive-white", value: "0 0% 100%" },
    PrimitiveToken { name: "primitive-black", value: "0 0% 0%" },

    // ======================================================================
    // Accent palettes (raw HSL)
    // ======================================================================

    PrimitiveToken { name: "primitive-amber-500", value: "38 92% 50%" },
    PrimitiveToken { name: "primitive-amber-600", value: "32 95% 44%" },

    PrimitiveToken { name: "primitive-red-500", value: "0 84% 60%" },
    PrimitiveToken { name: "primitive-red-600", value: "0 72% 51%" },

    PrimitiveToken { name: "primitive-green-500", value: "142 71% 45%" },
    PrimitiveToken { name: "primitive-green-600", value: "142 76% 36%" },

    // ======================================================================
    // Spacing — numeric scale only
    // ======================================================================

    PrimitiveToken { name: "primitive-space-0", value: "0" },
    PrimitiveToken { name: "primitive-space-1", value: "0.25rem" },
    PrimitiveToken { name: "primitive-space-2", value: "0.5rem" },
    PrimitiveToken { name: "primitive-space-3", value: "0.75rem" },
    PrimitiveToken { name: "primitive-space-4", value: "1rem" },
    PrimitiveToken { name: "primitive-space-5", value: "1.25rem" },
    PrimitiveToken { name: "primitive-space-6", value: "1.5rem" },
    PrimitiveToken { name: "primitive-space-8", value: "2rem" },
    PrimitiveToken { name: "primitive-space-10", value: "2.5rem" },
    PrimitiveToken { name: "primitive-space-12", value: "3rem" },
    PrimitiveToken { name: "primitive-space-16", value: "4rem" },

    // ======================================================================
    // Radius — numeric scale
    // ======================================================================

    PrimitiveToken { name: "primitive-radius-0", value: "0" },
    PrimitiveToken { name: "primitive-radius-1", value: "0.25rem" },
    PrimitiveToken { name: "primitive-radius-2", value: "0.375rem" },
    PrimitiveToken { name: "primitive-radius-3", value: "0.5rem" },
    PrimitiveToken { name: "primitive-radius-full", value: "9999px" },

    // ======================================================================
    // Shadows — physical light model
    // ======================================================================

    PrimitiveToken { name: "primitive-shadow-sm", value: "0 1px 2px rgb(0 0 0 / 0.05)" },
    PrimitiveToken { name: "primitive-shadow-md", value: "0 4px 6px rgb(0 0 0 / 0.1)" },
    PrimitiveToken { name: "primitive-shadow-lg", value: "0 10px 15px rgb(0 0 0 / 0.15)" },

    // ======================================================================
    // Typography — raw values
    // ======================================================================

    PrimitiveToken { name: "primitive-font-sans", value: "ui-sans-serif, system-ui, sans-serif" },
    PrimitiveToken { name: "primitive-font-mono", value: "ui-monospace, monospace" },

    PrimitiveToken { name: "primitive-font-size-1", value: "0.75rem" },
    PrimitiveToken { name: "primitive-font-size-2", value: "0.875rem" },
    PrimitiveToken { name: "primitive-font-size-3", value: "1rem" },
    PrimitiveToken { name: "primitive-font-size-4", value: "1.125rem" },
    PrimitiveToken { name: "primitive-font-size-5", value: "1.25rem" },
    PrimitiveToken { name: "primitive-font-size-6", value: "1.5rem" },

    PrimitiveToken { name: "primitive-font-weight-400", value: "400" },
    PrimitiveToken { name: "primitive-font-weight-500", value: "500" },
    PrimitiveToken { name: "primitive-font-weight-600", value: "600" },
    PrimitiveToken { name: "primitive-font-weight-700", value: "700" },

    PrimitiveToken { name: "primitive-line-height-1", value: "1.25" },
    PrimitiveToken { name: "primitive-line-height-2", value: "1.5" },
    PrimitiveToken { name: "primitive-line-height-3", value: "1.75" },

    // ======================================================================
    // Motion — time & curve
    // ======================================================================

    PrimitiveToken { name: "primitive-motion-duration-fast", value: "150ms" },
    PrimitiveToken { name: "primitive-motion-duration-normal", value: "200ms" },
    PrimitiveToken { name: "primitive-motion-duration-slow", value: "300ms" },

    PrimitiveToken { name: "primitive-motion-ease-linear", value: "linear" },
    PrimitiveToken { name: "primitive-motion-ease-standard", value: "cubic-bezier(0.4, 0.0, 0.2, 1)" },

    // ======================================================================
    // Z-index
    // ======================================================================

    PrimitiveToken { name: "primitive-z-base", value: "0" },
    PrimitiveToken { name: "primitive-z-dropdown", value: "1000" },
    PrimitiveToken { name: "primitive-z-overlay", value: "1300" },
    PrimitiveToken { name: "primitive-z-modal", value: "1400" },
];
