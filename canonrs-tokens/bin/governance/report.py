"""
report.py — Structured diagnostics
Tier S: JSON output + CLI + severity levels
"""

import json
from dataclasses import dataclass, asdict, field
from typing import List, Optional


@dataclass
class Diagnostic:
    component: str
    layer: str        # primitive | ui | boundary | preview | css | token | behavior | ast | layout
    rule: str         # CR-350, STATE-ENGINE, etc
    severity: str     # error | warning | info
    file: str
    line: Optional[int]
    message: str


@dataclass
class ValidationResult:
    total_ok:      int = 0
    total_errors:  int = 0
    total_warnings: int = 0
    diagnostics:   List[Diagnostic] = field(default_factory=list)

    def add_error(self, component: str, layer: str, rule: str, file: str, message: str, line: int = None):
        self.diagnostics.append(Diagnostic(component, layer, rule, "error", file, line, message))
        self.total_errors += 1

    def add_warning(self, component: str, layer: str, rule: str, file: str, message: str, line: int = None):
        self.diagnostics.append(Diagnostic(component, layer, rule, "warning", file, line, message))
        self.total_warnings += 1

    def to_json(self) -> str:
        return json.dumps({
            "summary": {
                "ok":       self.total_ok,
                "errors":   self.total_errors,
                "warnings": self.total_warnings,
            },
            "diagnostics": [asdict(d) for d in self.diagnostics]
        }, indent=2)

    def print_cli(self):
        for d in self.diagnostics:
            prefix = "❌" if d.severity == "error" else "⚠️ "
            loc = f":{d.line}" if d.line else ""
            print(f"  {prefix} [{d.rule}] {d.component}/{d.layer} {d.file}{loc}")
            print(f"     {d.message}")
