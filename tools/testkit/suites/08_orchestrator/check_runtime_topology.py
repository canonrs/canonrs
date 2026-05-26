#!/usr/bin/env python3
"""check_runtime_topology.py"""
import subprocess, sys, os
DIR = os.path.dirname(os.path.abspath(__file__))
def get_site_url():
    import yaml, os
    registry = os.path.join(os.environ.get("GITHUB_WORKSPACE", "/opt/docker/monorepo/packages-rust/rs-canonrs"), 
                            "../../tools/testkit/testkit_registry.yaml")
    try:
        with open(registry) as f:
            data = yaml.safe_load(f)
        port = data.get("products", {}).get("canonrs-site", {}).get("port", 3004)
        return f"http://localhost:{port}"
    except Exception:
        return "http://localhost:3004"

def server_available(url):
    import urllib.request
    try:
        urllib.request.urlopen(url, timeout=3)
        return True
    except Exception:
        return False

def run():
    url = get_site_url()
    if not server_available(url):
        print(f"[SKIP] runtime_topology — server not running on {url}")
        return 0
    env = dict(os.environ)
    env["NODE_PATH"] = "/usr/lib/node_modules"
    env["CANON_BASE_URL"] = url
    try:
        result = subprocess.run(["node", os.path.join(DIR, "runtime_topology_tests.cjs")], env=env, timeout=180)
        return result.returncode
    except subprocess.TimeoutExpired:
        print("[FAIL] timeout"); return 1
    except Exception as e:
        print(f"[FAIL] {e}"); return 1
if __name__ == "__main__": sys.exit(run())
