
const { chromium } = require('playwright');
const http = require('http');

const BASE_URL = 'http://localhost:3000';

const ROUTES = ["/showcase/dialog", "/showcase/modal", "/showcase/drawer", "/showcase/sheet", "/showcase/popover", "/showcase/tooltip", "/showcase/dropdown-menu", "/showcase/accordion", "/showcase/tabs", "/showcase/select", "/showcase/checkbox", "/showcase/switch", "/showcase/slider", "/showcase/carousel"];

function fetchSSR(url) {
    return new Promise((resolve, reject) => {
        http.get(url, (res) => {
            let data = '';
            res.on('data', chunk => data += chunk);
            res.on('end', () => resolve(data));
        }).on('error', reject);
    });
}

function extractAttrs(html, attr) {
    const re = new RegExp(`${attr}="([^"]*)"`, 'g');
    const vals = new Set();
    let m;
    while ((m = re.exec(html)) !== null) vals.add(m[1]);
    return vals;
}

function extractAttrKeys(html) {
    const re = /data-rs-[a-z][a-z0-9-]+(?:="[^"]*")?/g;
    const keys = new Set();
    let m;
    while ((m = re.exec(html)) !== null) {
        const key = m[0].split('=')[0];
        keys.add(key);
    }
    return keys;
}

async function checkRoute(page, route) {
    const errors = [];
    const url = BASE_URL + route;

    // 1. SSR HTML — sem JS
    let ssrHtml;
    try { ssrHtml = await fetchSSR(url); }
    catch(e) { return [`[CR-HYD-000] ${route} — servidor nao responde`]; }

    // 2. DOM hidratado — com JS
    const consoleErrors = [];
    page.on('console', msg => {
        const t = msg.text();
        if (msg.type() === 'error' && !t.includes('favicon') && !t.includes('collect')) {
            consoleErrors.push(t);
        }
    });

    await page.goto(url, { waitUntil: 'domcontentloaded', timeout: 10000 });
    // aguarda hydration
    await page.waitForTimeout(2000);

    const hydratedHtml = await page.content();

    // CR-HYD-100: hydration mismatch no console
    const mismatches = consoleErrors.filter(e =>
        e.includes('hydration') || e.includes('mismatch') || e.includes('did not match')
    );
    if (mismatches.length > 0) {
        errors.push(`[CR-HYD-100] ${route} — hydration mismatch: ${mismatches[0]}`);
    }

    // CR-HYD-101: data-rs-interaction deve existir no SSR
    const ssrInteractions    = extractAttrs(ssrHtml, 'data-rs-interaction');
    const hydratedInteractions = await page.evaluate(() => {
        return Array.from(document.querySelectorAll('[data-rs-interaction]'))
            .map(el => el.getAttribute('data-rs-interaction'));
    });
    if (ssrInteractions.size === 0 && hydratedInteractions.length > 0) {
        errors.push(`[CR-HYD-101] ${route} — data-rs-interaction ausente no SSR mas presente no DOM\n              garantia: interaction roots DEVEM existir no SSR`);
    }

    // CR-HYD-102: data-rs-uid deve existir no SSR
    const ssrUids    = extractAttrs(ssrHtml, 'data-rs-uid');
    const hydratedUids = await page.evaluate(() => {
        return Array.from(document.querySelectorAll('[data-rs-uid]'))
            .map(el => el.getAttribute('data-rs-uid'));
    });
    if (ssrUids.size === 0 && hydratedUids.length > 0) {
        errors.push(`[CR-HYD-102] ${route} — data-rs-uid ausente no SSR\n              garantia: uids DEVEM ser determinísticos no SSR`);
    }

    // CR-HYD-103: data-rs-state inicial deve existir no SSR
    const ssrStates = extractAttrs(ssrHtml, 'data-rs-state');
    const hydratedStates = await page.evaluate(() => {
        const els = document.querySelectorAll('[data-rs-interaction][data-rs-state]');
        return Array.from(els).map(el => el.getAttribute('data-rs-state'));
    });
    if (ssrStates.size === 0 && hydratedStates.length > 0) {
        errors.push(`[CR-HYD-103] ${route} — data-rs-state ausente no SSR\n              garantia: estado inicial DEVE ser emitido no servidor`);
    }

    // CR-HYD-104: WASM panic
    const panics = consoleErrors.filter(e =>
        e.includes('panicked') || e.includes('wasm trap') || e.includes('RuntimeError')
    );
    if (panics.length > 0) {
        errors.push(`[CR-HYD-104] ${route} — WASM panic: ${panics[0]}`);
    }

    return errors;
}

(async () => {
    if (!require('playwright')) { console.log('[SKIP]'); process.exit(0); }
    const browser = await chromium.launch({ headless: true });
    let passed = 0, failed = 0;
    const failures = [];

    for (const route of ROUTES) {
        const page = await browser.newPage();
        try {
            const errors = await checkRoute(page, route);
            if (errors.length > 0) {
                errors.forEach(e => console.log('[FAIL] ' + e));
                failures.push(route);
                failed++;
            } else {
                console.log('[OK] ' + route);
                passed++;
            }
        } catch(e) {
            console.log('[FAIL] ' + route + ': ' + e.message);
            failures.push(route);
            failed++;
        } finally {
            await page.close();
        }
    }

    await browser.close();
    console.log('\n==================================================');
    console.log('[OK] ' + passed + ' routes clean');
    if (failed > 0) {
        console.log('[FAIL] ' + failed + ' routes failed');
        process.exit(1);
    }
    console.log('[OK] Hydration governance canonical');
    process.exit(0);
})();
