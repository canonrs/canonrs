#!/usr/bin/env python3
"""
incremental.py — Incremental Certification
Detecta arquivos alterados e roda apenas as suites relevantes
"""
import os, sys, subprocess, glob

_CANONRS_ROOT = os.environ.get("GITHUB_WORKSPACE", "/opt/docker/monorepo/packages-rust/rs-canonrs")
TESTKIT_DIR   = os.path.join(_CANONRS_ROOT, "tools/testkit")

# mapa: path pattern -> suites afetadas
DEPENDENCY_MAP = [
    ("canonrs-core/src/primitives/",     ["01_architecture", "09_usage", "17_signal_contracts"]),
    ("canonrs-server/src/ui/",           ["01_architecture", "09_usage", "11_hydration_contracts", "16_dom_topology"]),
    ("canonrs-server/src/blocks/",       ["01_architecture", "13_dom_driven_contracts"]),
    ("canonrs-interactions",             ["03_runtime", "10_runtime_contracts", "19_runtime_groups"]),
    ("canonrs-interactions-overlay",     ["12_overlay_contracts", "16_dom_topology"]),
    ("canonrs-core/src/",                ["01_architecture", "02_lifecycle", "15_island_contracts"]),
    ("tools/canonrs-orchestrator/",      ["08_orchestrator", "20_build_parity"]),
    ("canonrs-client/",                  ["14_build_artifacts", "20_build_parity"]),
]

def get_changed_files():
    """Retorna lista de arquivos alterados no PR ou ultimo commit."""
    base = os.environ.get("GITHUB_BASE_REF", "")
    if base:
        result = subprocess.run(
            ["git", "diff", "--name-only", f"origin/{base}...HEAD"],
            capture_output=True, text=True,
            cwd=_CANONRS_ROOT
        )
    else:
        result = subprocess.run(
            ["git", "diff", "--name-only", "HEAD~1"],
            capture_output=True, text=True,
            cwd=_CANONRS_ROOT
        )
    return result.stdout.strip().splitlines()

def get_affected_suites(changed_files):
    """Retorna set de suites afetadas pelos arquivos alterados."""
    affected = set()
    for path in changed_files:
        for pattern, suites in DEPENDENCY_MAP:
            if pattern in path:
                affected.update(suites)
    return affected

def run_suite(suite_id):
    run_all = os.path.join(TESTKIT_DIR, "suites", suite_id, "run_all.sh")
    if not os.path.exists(run_all):
        print(f"[SKIP] {suite_id} — run_all.sh not found")
        return 0
    result = subprocess.run(["bash", run_all], cwd=_CANONRS_ROOT)
    return result.returncode

def run():
    changed = get_changed_files()
    if not changed:
        print("[INCREMENTAL] No changed files detected — running full governance")
        affected = {s for _, suites in DEPENDENCY_MAP for s in suites}
    else:
        print(f"[INCREMENTAL] Changed files: {len(changed)}")
        for f in changed[:10]:
            print(f"  {f}")
        affected = get_affected_suites(changed)

    if not affected:
        print("[INCREMENTAL] No suites affected — skipping")
        return 0

    print(f"\n[INCREMENTAL] Running {len(affected)} affected suites:")
    for s in sorted(affected):
        print(f"  {s}")

    failed = 0
    for suite_id in sorted(affected):
        print(f"\n[RUN] {suite_id}")
        rc = run_suite(suite_id)
        if rc != 0:
            failed += 1

    print(f"\n[INCREMENTAL] {len(affected) - failed} passed, {failed} failed")
    return 1 if failed > 0 else 0

if __name__ == "__main__":
    sys.exit(run())
