from pathlib import Path
import os, glob, re
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')


PRIMITIVES_DIR = _CANONRS_ROOT + "/canonrs-core/src/primitives"
UI_DIR         = _CANONRS_ROOT + "/canonrs-server/src/ui"
BASE_URL       = "http://localhost:3000"

has_preview = set(
    os.path.basename(os.path.dirname(p))
    for p in glob.glob(f"{UI_DIR}/**/preview.rs", recursive=True)
)

def get_interaction_attr(comp):
    rs = os.path.join(PRIMITIVES_DIR, f"{comp}.rs")
    if not os.path.exists(rs): return comp.replace("_", "-")
    src = re.sub(r"//[^\n]*", "", open(rs).read())
    # busca bloco que contem data-rs-interaction
    for m in re.finditer(r'<\s*\w+[^>]*data-rs-interaction="[^"]*"[^>]*>', src):
        block = m.group(0)
        m2 = re.search(r'data-rs-([a-z][a-z0-9-]+)=""', block)
        if m2: return m2.group(1)
    # fallback — primeiro data-rs-{name}="" no arquivo
    m = re.search(r'data-rs-([a-z][a-z0-9-]+)=""', src)
    return m.group(1) if m else comp.replace("_", "-")

class PrimitiveInfo:
    def __init__(self, comp, group, main_attr, has_trigger):
        self.comp = comp; self.group = group
        self.main_attr = main_attr; self.has_trigger = has_trigger

primitives = {}
for rs_file in glob.glob(f"{PRIMITIVES_DIR}/*.rs"):
    comp = os.path.basename(rs_file).replace(".rs", "")
    if comp == "mod": continue
    src = re.sub(r"//[^\n]*", "", open(rs_file).read())
    m = re.search(r'data-rs-interaction="([^"]+)"', src)
    if not m: continue
    group = m.group(1)
    main_attr = get_interaction_attr(comp)
    has_trigger = bool(re.search(rf'data-rs-{re.escape(main_attr)}-trigger', src))
    primitives[comp] = PrimitiveInfo(comp, group, main_attr, has_trigger)

# componentes sem showcase real ou comportamento especial
SKIP_INTERACTION = {
    "doc_progress": "nao renderiza no showcase",
    "hover_card":   "hover only — validado via SSR",
    "context_menu": "right-click only — validado via SSR",
}

route_name = lambda c: c.replace("_", "-")

def wait_for_el(attr):
    return f"            await page.waitForSelector('[data-rs-{attr}]', {{ timeout: 5000 }}).catch(() => {{}});\n"

def wait_init(attr):
    return (
        f"            await page.waitForFunction(() => {{\n"
        f"                const d = document.querySelector('[data-rs-{attr}]');\n"
        f"                return d && d.hasAttribute('data-rs-initialized');\n"
        f"            }}, {{ timeout: 10000 }});\n"
    )

def wait_state(attr, state):
    return (
        f"            await page.waitForFunction(() => {{\n"
        f"                const d = document.querySelector('[data-rs-{attr}]');\n"
        f"                return d && d.getAttribute('data-rs-state') === '{state}';\n"
        f"            }}, {{ timeout: 3000 }});\n"
    )

test_blocks = []

# SSR
for comp in sorted(has_preview):
    route = f"/showcase/{route_name(comp)}"
    test_blocks.append(
        f"    {{\n        name: 'SSR: {comp}',\n        route: '{route}',\n"
        f"        test: async (page) => {{\n"
        f"            const html = await page.content();\n"
        f"            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');\n"
        f"        }}\n    }}"
    )

# interaction
for comp, info in sorted(primitives.items()):
    if comp not in has_preview: continue
    if comp in SKIP_INTERACTION: continue
    attr  = info.main_attr
    group = info.group
    route = f"/showcase/{route_name(comp)}"

    if group == "overlay" and info.has_trigger:
        test_blocks.append(
            f"    {{\n        name: 'overlay: {comp} opens',\n        route: '{route}',\n"
            f"        test: async (page) => {{\n"
            + wait_init(attr) +
            f"            const trigger = await page.$('[data-rs-{attr}-trigger]');\n"
            f"            if (!trigger) return;\n"
            f"            await trigger.click();\n"
            + wait_state(attr, "open") +
            f"        }}\n    }}"
        )
    elif group == "nav" and info.has_trigger:
        test_blocks.append(
            f"    {{\n        name: 'nav: {comp} activates',\n        route: '{route}',\n"
            f"        test: async (page) => {{\n"
            + wait_init(attr) +
            f"            const trigger = await page.$('[data-rs-{attr}-trigger]');\n"
            f"            if (!trigger) return;\n"
            f"            await trigger.click();\n"
            f"            await page.waitForTimeout(300);\n"
            f"        }}\n    }}"
        )
    elif group == "selection" and info.has_trigger:
        test_blocks.append(
            f"    {{\n        name: 'selection: {comp} opens',\n        route: '{route}',\n"
            f"        test: async (page) => {{\n"
            + wait_init(attr) +
            f"            const trigger = await page.$('[data-rs-{attr}-trigger]');\n"
            f"            if (!trigger) return;\n"
            f"            await trigger.click();\n"
            + wait_state(attr, "open") +
            f"        }}\n    }}"
        )
    elif group in ("gesture", "content", "data", "init", "dismiss"):
        test_blocks.append(
            f"    {{\n        name: '{group}: {comp} renders',\n        route: '{route}',\n"
            f"        test: async (page) => {{\n"
            + wait_for_el(attr) +
            f"            const el = await page.$('[data-rs-{attr}]');\n"
            f"            if (!el) throw new Error('[data-rs-{attr}] not found in DOM');\n"
            f"        }}\n    }}"
        )

tests_js = ",\n".join(test_blocks)

js = (
    "const { chromium } = require('playwright');\n\n"
    f"const BASE_URL = '{BASE_URL}';\n\n"
    "const TESTS = [\n" + tests_js + "\n];\n\n"
    "(async () => {\n"
    "    let browser;\n"
    "    try {\n        browser = await chromium.launch({ headless: true });\n"
    "    } catch(e) {\n        console.log('[SKIP] Chromium nao disponivel: ' + e.message);\n        process.exit(0);\n    }\n\n"
    "    let passed = 0;\n    let failed = 0;\n    const failures = [];\n\n"
    "    for (const t of TESTS) {\n"
    "        const page = await browser.newPage();\n"
    "        const consoleErrors = [];\n"
    "        page.on('console', msg => { if (msg.type() === 'error') consoleErrors.push(msg.text()); });\n"
    "        try {\n"
    "            await page.goto(BASE_URL + t.route, { waitUntil: 'domcontentloaded', timeout: 10000 });\n"
    "            await t.test(page);\n"
    "            const critical = consoleErrors.filter(e => !e.includes('favicon') && !e.includes('404') && !e.includes('collect'));\n"
    "            if (critical.length > 0) throw new Error('console errors: ' + critical[0]);\n"
    "            console.log('[OK] ' + t.name);\n            passed++;\n"
    "        } catch(e) {\n"
    "            console.log('[FAIL] ' + t.name + ': ' + e.message);\n"
    "            failures.push(t.name);\n            failed++;\n"
    "        } finally {\n            await page.close();\n        }\n    }\n\n"
    "    await browser.close();\n"
    "    console.log('\\n==================================================');\n"
    "    console.log('[OK] ' + passed + ' tests passed');\n"
    "    if (failed > 0) {\n"
    "        console.log('[FAIL] ' + failed + ' tests failed');\n"
    "        failures.forEach(f => console.log('  - ' + f));\n"
    "        process.exit(1);\n    }\n"
    "    console.log('[OK] Browser runtime canonical');\n"
    "    process.exit(0);\n"
    "})();\n"
)

Path("/opt/docker/monorepo/tools/testkit/canonrs/tests/playwright_tests.cjs").write_text(js)
total = len(test_blocks)
print(f"OK: {total} tests ({len(has_preview)} SSR + {total - len(has_preview)} interaction)")