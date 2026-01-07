# Canon Rule #36: Component Compliance Levels

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-02

## Principle
Components are classified by **compliance level** to balance canon purity with real-world constraints. Levels define which rules can be relaxed, under what conditions, and with what governance.

## Compliance Levels

### Level 1: Strict (Canon Pure)

**Definition:** 100% compliance with ALL canon rules. Zero exceptions.

**Requirements:**
- ✅ All tokens from canonical set
- ✅ No hardcoded values
- ✅ SSR-safe
- ✅ Fully accessible (WCAG 2.1 AA)
- ✅ Density-aware
- ✅ Theme-compatible
- ✅ Type-safe props
- ✅ Documented with examples

**Use Cases:**
- Core design system components
- Public-facing components
- Components in component library showcase
- Reusable primitives

**Examples:**
```
ui/button/
ui/input/
ui/select/
primitives/focus-trap/
```

**Marker:**
```rust
/// @canon-level strict
/// Compliance: 100%
#[component]
pub fn Button(...) -> impl IntoView { ... }
```

---

### Level 2: Standard (Production Grade)

**Definition:** 95%+ compliance. Minor exceptions allowed with explicit justification.

**Allowed Exceptions:**
- ✅ Non-canonical variants for specific business needs (with approval)
- ✅ Hardcoded values if token doesn't exist (must file token request)
- ✅ Simplified accessibility for internal tools (document why)
- ✅ Performance optimizations that bypass some rules

**Requirements:**
- Must document ALL exceptions in component header
- Exceptions require team review
- Must have migration path to Level 1

**Use Cases:**
- Business-specific components
- Dashboard widgets
- Internal admin tools
- Feature-specific UI

**Examples:**
```
features/analytics-chart/
features/user-profile-card/
internal/admin-panel/
```

**Marker:**
```rust
/// @canon-level standard
/// Compliance: 96%
/// Exceptions:
///   - Uses custom color for brand requirement (approved 2025-01-01)
///   - Non-canonical "info" variant (legacy compatibility)
#[component]
pub fn AnalyticsChart(...) -> impl IntoView { ... }
```

---

### Level 3: Loose (Legacy/Migration)

**Definition:** 70%+ compliance. Significant exceptions allowed during migration or for legacy compatibility.

**Allowed Exceptions:**
- ✅ localStorage instead of cookies (migration in progress)
- ✅ Non-canonical colors (legacy brand requirements)
- ✅ Fixed pixel sizes (pre-density era)
- ✅ Incomplete accessibility (must document plan)
- ✅ Non-SSR compatible (client-only acceptable)

**Requirements:**
- Must have **migration plan** documented
- Must have **target date** for Level 2 promotion
- Exceptions tracked in component database
- Cannot be used in new features (legacy only)

**Use Cases:**
- Legacy components being migrated
- Third-party integrations
- White-label customer overrides
- Temporary prototypes

**Examples:**
```
legacy/old-dashboard/
integrations/salesforce-widget/
customer/acme-custom-theme/
```

**Marker:**
```rust
/// @canon-level loose
/// Compliance: 73%
/// Migration Status: In Progress
/// Target Level: Standard by 2025-Q2
/// Exceptions:
///   - Uses localStorage (migrating to cookies)
///   - Hardcoded brand colors (white-label requirement)
///   - Non-density-aware (requires refactor)
#[component]
pub fn LegacyDashboard(...) -> impl IntoView { ... }
```

---

## Compliance Matrix

| Rule Category | Strict | Standard | Loose |
|---------------|--------|----------|-------|
| Color Tokens (#21) | 100% | 95% | 70% |
| Typography (#29) | 100% | 100% | 80% |
| Accessibility (#31) | 100% AA | 100% AA | 80% AA |
| Theme Persistence (#32) | Cookies | Cookies | localStorage OK |
| Density (#33) | 100% | 95% | 60% |
| Token Usage (#35) | 100% | 95% | 70% |

## Governance Process

### Promoting a Component

**Loose → Standard:**
1. Fix critical violations (colors, accessibility)
2. Document remaining exceptions
3. Team review + approval
4. Update compliance level marker

**Standard → Strict:**
1. Remove ALL exceptions
2. 100% token coverage
3. Full test coverage
4. Documentation complete
5. Design review + approval

### Demoting a Component

**Only allowed for:**
- Breaking business requirements (temporary)
- Performance critical paths (with measurement)
- Legacy compatibility (migration path required)

**Process:**
1. File exception request (include justification)
2. Team review (must approve)
3. Document exceptions in component
4. Set target date for re-promotion

## Database Schema

### Component Compliance Tracking
```sql
CREATE TABLE components (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    
    -- Compliance level
    canon_level TEXT CHECK(canon_level IN ('strict', 'standard', 'loose')),
    canon_score INTEGER CHECK(canon_score BETWEEN 0 AND 100),
    
    -- Token compliance
    tokens_canonicos_percent INTEGER,
    tokens_familia_c_percent,
    
    -- Exception tracking
    has_exceptions BOOLEAN DEFAULT 0,
    exception_count INTEGER DEFAULT 0,
    exception_details TEXT, -- JSON array
    
    -- Migration status
    migration_status TEXT CHECK(migration_status IN ('none', 'planned', 'in_progress', 'complete')),
    migration_target_level TEXT,
    migration_target_date DATE,
    
    -- Approval
    level_approved_by TEXT,
    level_approved_at DATETIME
);
```

### Exception Schema
```sql
CREATE TABLE component_exceptions (
    id INTEGER PRIMARY KEY,
    component_id INTEGER REFERENCES components(id),
    rule_number INTEGER, -- e.g., 21 for Canon Rule #21
    exception_type TEXT, -- 'color', 'token', 'accessibility', etc.
    justification TEXT,
    approved_by TEXT,
    approved_at DATETIME,
    target_resolution_date DATE,
    status TEXT CHECK(status IN ('active', 'resolved', 'permanent'))
);
```

## Validation

### Automated Checks
```rust
// tools/canon-linter/src/compliance_checker.rs

pub fn check_compliance_level(component: &Component) -> ComplianceReport {
    let mut score = 100;
    let mut violations = Vec::new();
    
    // Check color tokens
    if !component.uses_canonical_colors() {
        score -= 10;
        violations.push("Non-canonical colors");
    }
    
    // Check accessibility
    if !component.meets_wcag_aa() {
        score -= 20;
        violations.push("WCAG AA violations");
    }
    
    // Check density
    if !component.is_density_aware() {
        score -= 15;
        violations.push("Not density-aware");
    }
    
    // Determine level
    let level = match score {
        95..=100 => ComplianceLevel::Strict,
        70..=94 => ComplianceLevel::Standard,
        0..=69 => ComplianceLevel::Loose,
        _ => ComplianceLevel::NonCompliant,
    };
    
    ComplianceReport {
        score,
        level,
        violations,
    }
}
```

### CI Enforcement
```yaml
# .github/workflows/compliance-levels.yml
name: Component Compliance Levels

on: [push, pull_request]

jobs:
  check-levels:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Check Strict Components
        run: |
          # Strict components MUST be 100%
          for file in src/ui/**/*.rs; do
            if grep -q "@canon-level strict" "$file"; then
              score=$(cargo run --bin compliance-checker -- "$file")
              if [ "$score" -lt 100 ]; then
                echo "❌ $file claims strict but scored $score"
                exit 1
              fi
            fi
          done
      
      - name: Check Standard Components
        run: |
          # Standard components MUST be 95%+
          for file in src/features/**/*.rs; do
            if grep -q "@canon-level standard" "$file"; then
              score=$(cargo run --bin compliance-checker -- "$file")
              if [ "$score" -lt 95 ]; then
                echo "❌ $file claims standard but scored $score"
                exit 1
              fi
            fi
          done
      
      - name: Detect Unmarked Components
        run: |
          # All components MUST have a level marker
          for file in src/{ui,features,legacy}/**/*.rs; do
            if ! grep -q "@canon-level" "$file"; then
              echo "⚠️  $file missing compliance level marker"
            fi
          done
```

## White-Label Support

### Customer-Specific Overrides
```rust
/// @canon-level loose
/// White-label: ACME Corp
/// Expires: 2025-12-31
/// 
/// Custom overrides for ACME brand requirements:
///   - Brand colors: #FF6B35 (primary), #004E89 (secondary)
///   - Custom font: "ACME Sans" (fallback to Inter)
///   - Increased spacing (1.2x multiplier)
#[component]
pub fn AcmeButton(
    #[prop(optional)] use_acme_brand: bool,
) -> impl IntoView {
    let class = if use_acme_brand {
        // Non-canonical colors allowed for white-label
        "bg-[#FF6B35] text-white font-acme"
    } else {
        // Canonical fallback
        "bg-primary text-primary-foreground"
    };
    
    view! { <button class=class>{children()}</button> }
}
```

### White-Label Registry
```sql
CREATE TABLE white_label_overrides (
    id INTEGER PRIMARY KEY,
    customer_name TEXT NOT NULL,
    component_name TEXT NOT NULL,
    override_type TEXT, -- 'color', 'font', 'spacing', etc.
    override_value TEXT,
    approved_by TEXT,
    expires_at DATE,
    UNIQUE(customer_name, component_name, override_type)
);
```

## Migration Paths

### Loose → Standard
```
Week 1-2: Audit & Plan
  - List all violations
  - Prioritize fixes
  - Estimate effort

Week 3-4: Fix Critical
  - Accessibility (WCAG AA)
  - Color tokens
  - SSR compatibility

Week 5-6: Fix Non-Critical
  - Density awareness
  - Typography tokens
  - Documentation

Week 7: Testing & Review
  - Automated tests
  - Manual QA
  - Team review

Week 8: Promotion
  - Update marker to standard
  - Close exception tickets
  - Update documentation
```

### Standard → Strict
```
Month 1: Remove All Exceptions
  - Replace non-canonical variants
  - Fix hardcoded values
  - Complete documentation

Month 2: Testing
  - 100% test coverage
  - Accessibility audit
  - Performance testing

Month 3: Review & Promotion
  - Design review
  - Code review
  - Update to strict
```

## Exception Request Template
```markdown
# Canon Exception Request

**Component:** `ui/custom-widget`  
**Current Level:** Standard  
**Rule(s) Affected:** #21 (Color Tokens), #33 (Density)

## Exception Details

### What rule is being violated?
Using hardcoded brand color `#FF6B35` instead of canonical token.

### Why is this exception necessary?
Customer contract requires exact brand color match. No canonical token available.

### What is the business impact if denied?
Loss of $500k/year contract.

### What is the migration path?
1. Create new canonical token `color.brand.acme-orange`
2. Update component to use token
3. Remove exception by Q2 2025

### Approval
- **Requested by:** <REQUESTOR_NAME> (<REQUESTOR_EMAIL>)
- **Approved by:** <APPROVER_NAME> (<APPROVER_ROLE>)
- **Date:** 2025-01-02
- **Expires:** 2025-06-30
```

## Reporting

### Compliance Dashboard
```sql
-- Overall compliance by level
SELECT 
    canon_level,
    COUNT(*) as count,
    AVG(canon_score) as avg_score,
    SUM(exception_count) as total_exceptions
FROM components
GROUP BY canon_level;

-- Components needing promotion
SELECT 
    name,
    canon_level,
    canon_score,
    migration_target_date
FROM components
WHERE migration_status = 'in_progress'
  AND migration_target_date < DATE('now', '+30 days')
ORDER BY migration_target_date;

-- Exception aging report
SELECT 
    c.name,
    ce.rule_number,
    ce.exception_type,
    ce.approved_at,
    JULIANDAY('now') - JULIANDAY(ce.approved_at) as days_open
FROM component_exceptions ce
JOIN components c ON ce.component_id = c.id
WHERE ce.status = 'active'
ORDER BY days_open DESC;
```

## References

- [Canon Rule #21: Canonical Color Tokens](./canon-rule-21-canonical-color-tokens.md)
- [Canon Rule #31: Accessibility Contract](./canon-rule-31-accessibility-contract.md)
- [Canon Rule #32: Theme Persistence Contract](./canon-rule-32-theme-persistence-contract.md)
- [Canon Rule #33: Density & Accessibility Mapping](./canon-rule-33-density-accessibility-mapping.md)
- [Canon Rule #34: Theme & Density Enforcement](./canon-rule-34-theme-density-enforcement.md)
- [Canon Rule #35: Token Usage Validation](./canon-rule-35-token-usage-validation.md)

---

**Enforcement:** Database tracking + CI validation  
**Levels:** Strict (100%) | Standard (95%) | Loose (70%)  
**Migration:** Required for Loose → Standard → Strict

---

## IMPORTANT NOTE

All examples with names, emails, or companies are **PLACEHOLDERS** for documentation purposes:
- `<REQUESTOR_NAME>`, `<APPROVER_NAME>` → Replace with actual person
- `ACME Corp`, `Salesforce` → Example company names (fictional)
- Dates, scores, violations → Example data for illustration

**Real implementation must use:**
- Actual team member names
- Real component names from your codebase
- Actual violation data from linters
- Real approval workflow from your organization
