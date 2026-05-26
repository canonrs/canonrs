#!/usr/bin/env python3
"""
generate_report.py — HTML Governance Report Generator
Gera relatorio HTML com metricas de governance por suite
"""
import os, sys, json, datetime, subprocess, glob

_CANONRS_ROOT = os.environ.get("GITHUB_WORKSPACE", "/opt/docker/monorepo/packages-rust/rs-canonrs")
TESTKIT_DIR   = os.path.join(_CANONRS_ROOT, "tools/testkit")
SUITES_DIR    = os.path.join(TESTKIT_DIR, "suites")
OUTPUT_DIR    = os.environ.get("REPORT_DIR", "/tmp/canonrs-reports")

os.makedirs(OUTPUT_DIR, exist_ok=True)

SUITES = [
    ("01_architecture",       "Architecture"),
    ("02_lifecycle",          "Lifecycle"),
    ("03_runtime",            "Runtime"),
    ("08_orchestrator",       "Orchestrator"),
    ("09_usage",              "Usage"),
    ("10_runtime_contracts",  "Runtime Contracts"),
    ("11_hydration_contracts","Hydration"),
    ("12_overlay_contracts",  "Overlay"),
    ("13_dom_driven_contracts","DOM Driven"),
    ("14_build_artifacts",    "Build Artifacts"),
    ("15_island_contracts",   "Island"),
    ("16_dom_topology",       "DOM Topology"),
    ("17_signal_contracts",   "Signals"),
    ("18_canvas_graph_contracts","Canvas Graph"),
    ("19_runtime_groups",     "Runtime Groups"),
    ("20_build_parity",       "Build Parity"),
]

def run_suite(suite_id):
    run_all = os.path.join(SUITES_DIR, suite_id, "run_all.sh")
    if not os.path.exists(run_all):
        return {"status": "skip", "output": "run_all.sh not found", "duration": 0}
    import time
    start = time.time()
    result = subprocess.run(
        ["bash", run_all],
        capture_output=True, text=True, timeout=120
    )
    duration = round(time.time() - start, 2)
    status = "pass" if result.returncode == 0 else "fail"
    return {"status": status, "output": result.stdout + result.stderr, "duration": duration}

def generate_html(results):
    sha     = os.environ.get("GITHUB_SHA", "local")[:8]
    branch  = os.environ.get("GITHUB_REF_NAME", "local")
    run_id  = os.environ.get("GITHUB_RUN_ID", "0")
    ts      = datetime.datetime.utcnow().strftime("%Y-%m-%d %H:%M UTC")

    passed  = sum(1 for r in results if r["status"] == "pass")
    failed  = sum(1 for r in results if r["status"] == "fail")
    skipped = sum(1 for r in results if r["status"] == "skip")
    total   = len(results)
    score   = round((passed / total) * 100) if total > 0 else 0

    rows = ""
    for r in results:
        color = {"pass": "#22c55e", "fail": "#ef4444", "skip": "#94a3b8"}[r["status"]]
        icon  = {"pass": "✅", "fail": "❌", "skip": "⏭"}[r["status"]]
        rows += f"""
        <tr>
          <td>{icon} {r["name"]}</td>
          <td style="color:{color};font-weight:bold">{r["status"].upper()}</td>
          <td>{r["duration"]}s</td>
          <td><pre style="max-height:100px;overflow:auto;font-size:11px">{r["output"][-500:] if r["output"] else ""}</pre></td>
        </tr>"""

    html = f"""<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>CanonRS Governance Report</title>
  <style>
    body {{ font-family: system-ui; background: #0f172a; color: #e2e8f0; padding: 2rem; }}
    h1 {{ color: #7c3aed; }}
    .meta {{ color: #94a3b8; font-size: 0.9rem; margin-bottom: 2rem; }}
    .score {{ font-size: 4rem; font-weight: bold; color: {"#22c55e" if score >= 90 else "#f59e0b" if score >= 70 else "#ef4444"}; }}
    .summary {{ display: flex; gap: 2rem; margin: 1rem 0 2rem; }}
    .stat {{ background: #1e293b; padding: 1rem 2rem; border-radius: 8px; text-align: center; }}
    .stat-num {{ font-size: 2rem; font-weight: bold; }}
    .stat-label {{ color: #94a3b8; font-size: 0.8rem; }}
    table {{ width: 100%; border-collapse: collapse; }}
    th {{ background: #1e293b; padding: 0.75rem; text-align: left; }}
    td {{ padding: 0.5rem 0.75rem; border-bottom: 1px solid #1e293b; vertical-align: top; }}
    tr:hover {{ background: #1e293b33; }}
    pre {{ background: #0f172a; padding: 0.5rem; border-radius: 4px; color: #94a3b8; }}
  </style>
</head>
<body>
  <h1>CanonRS Governance Report</h1>
  <div class="meta">
    SHA: {sha} | Branch: {branch} | Run: {run_id} | Generated: {ts}
  </div>
  <div class="score">{score}%</div>
  <div class="summary">
    <div class="stat"><div class="stat-num" style="color:#22c55e">{passed}</div><div class="stat-label">PASSED</div></div>
    <div class="stat"><div class="stat-num" style="color:#ef4444">{failed}</div><div class="stat-label">FAILED</div></div>
    <div class="stat"><div class="stat-num" style="color:#94a3b8">{skipped}</div><div class="stat-label">SKIPPED</div></div>
    <div class="stat"><div class="stat-num">{total}</div><div class="stat-label">TOTAL</div></div>
  </div>
  <table>
    <thead><tr><th>Suite</th><th>Status</th><th>Duration</th><th>Output</th></tr></thead>
    <tbody>{rows}</tbody>
  </table>
</body>
</html>"""

    out = os.path.join(OUTPUT_DIR, "governance_report.html")
    with open(out, "w") as f:
        f.write(html)

    # JSON para CI
    json_out = os.path.join(OUTPUT_DIR, "governance_report.json")
    with open(json_out, "w") as f:
        json.dump({
            "sha": sha, "branch": branch, "timestamp": ts,
            "score": score, "passed": passed, "failed": failed,
            "skipped": skipped, "total": total,
            "suites": results
        }, f, indent=2)

    print(f"[REPORT] {out}")
    print(f"[REPORT] Score: {score}% ({passed}/{total})")
    return score

def run():
    results = []
    for suite_id, name in SUITES:
        print(f"[RUN] {name}...")
        r = run_suite(suite_id)
        r["suite"] = suite_id
        r["name"]  = name
        results.append(r)

    score = generate_html(results)
    return 0 if score == 100 else 1

if __name__ == "__main__":
    sys.exit(run())
