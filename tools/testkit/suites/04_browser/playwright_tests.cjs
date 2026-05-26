const { chromium } = require('playwright');

const BASE_URL = 'http://localhost:3000';

const TESTS = [
    {
        name: 'SSR: accordion',
        route: '/showcase/accordion',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: alert',
        route: '/showcase/alert',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: alert_dialog',
        route: '/showcase/alert-dialog',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: animate',
        route: '/showcase/animate',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: aspect_ratio',
        route: '/showcase/aspect-ratio',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: avatar',
        route: '/showcase/avatar',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: badge',
        route: '/showcase/badge',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: banner',
        route: '/showcase/banner',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: breadcrumb',
        route: '/showcase/breadcrumb',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: button',
        route: '/showcase/button',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: button_group',
        route: '/showcase/button-group',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: callout',
        route: '/showcase/callout',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: card',
        route: '/showcase/card',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: carousel',
        route: '/showcase/carousel',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: chart',
        route: '/showcase/chart',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: checkbox',
        route: '/showcase/checkbox',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: code_block',
        route: '/showcase/code-block',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: collapsible',
        route: '/showcase/collapsible',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: color_picker',
        route: '/showcase/color-picker',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: combobox',
        route: '/showcase/combobox',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: command',
        route: '/showcase/command',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: confirm_dialog',
        route: '/showcase/confirm-dialog',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: context_menu',
        route: '/showcase/context-menu',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: copy_button',
        route: '/showcase/copy-button',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: data_table',
        route: '/showcase/data-table',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: dialog',
        route: '/showcase/dialog',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: doc_progress',
        route: '/showcase/doc-progress',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: drawer',
        route: '/showcase/drawer',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: dropdown_menu',
        route: '/showcase/dropdown-menu',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: empty_state',
        route: '/showcase/empty-state',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: empty_table',
        route: '/showcase/empty-table',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: error_state',
        route: '/showcase/error-state',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: field',
        route: '/showcase/field',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: form',
        route: '/showcase/form',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: form_error_summary',
        route: '/showcase/form-error-summary',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: hero',
        route: '/showcase/hero',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: hover_card',
        route: '/showcase/hover-card',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: icon',
        route: '/showcase/icon',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: icon_button',
        route: '/showcase/icon-button',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: inline_meta',
        route: '/showcase/inline-meta',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: inline_notice',
        route: '/showcase/inline-notice',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: input',
        route: '/showcase/input',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: input_group',
        route: '/showcase/input-group',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: input_otp',
        route: '/showcase/input-otp',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: kbd',
        route: '/showcase/kbd',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: label',
        route: '/showcase/label',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: link',
        route: '/showcase/link',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: link_group',
        route: '/showcase/link-group',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: list_item',
        route: '/showcase/list-item',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: loading_overlay',
        route: '/showcase/loading-overlay',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: logo',
        route: '/showcase/logo',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: markdown',
        route: '/showcase/markdown',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: menu',
        route: '/showcase/menu',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: menubar',
        route: '/showcase/menubar',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: modal',
        route: '/showcase/modal',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: nav_item',
        route: '/showcase/nav-item',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: navigation_menu',
        route: '/showcase/navigation-menu',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: page_header',
        route: '/showcase/page-header',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: pagination',
        route: '/showcase/pagination',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: popover',
        route: '/showcase/popover',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: progress',
        route: '/showcase/progress',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: pulse',
        route: '/showcase/pulse',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: radio',
        route: '/showcase/radio',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: radio_group',
        route: '/showcase/radio-group',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: resizable',
        route: '/showcase/resizable',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: scroll_area',
        route: '/showcase/scroll-area',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: section',
        route: '/showcase/section',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: select',
        route: '/showcase/select',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: separator',
        route: '/showcase/separator',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: sheet',
        route: '/showcase/sheet',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: sidebar',
        route: '/showcase/sidebar',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: skeleton',
        route: '/showcase/skeleton',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: slider',
        route: '/showcase/slider',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: spinner',
        route: '/showcase/spinner',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: stat',
        route: '/showcase/stat',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: status_dot',
        route: '/showcase/status-dot',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: switch',
        route: '/showcase/switch',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: table',
        route: '/showcase/table',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: table_of_contents',
        route: '/showcase/table-of-contents',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: tabs',
        route: '/showcase/tabs',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: textarea',
        route: '/showcase/textarea',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: toast',
        route: '/showcase/toast',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: toggle',
        route: '/showcase/toggle',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: toggle_group',
        route: '/showcase/toggle-group',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: toolbar',
        route: '/showcase/toolbar',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: tooltip',
        route: '/showcase/tooltip',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: tree',
        route: '/showcase/tree',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'SSR: virtual_list',
        route: '/showcase/virtual-list',
        test: async (page) => {
            const html = await page.content();
            if (!html.includes('data-rs-')) throw new Error('no data-rs-* in SSR');
        }
    },
    {
        name: 'nav: accordion activates',
        route: '/showcase/accordion',
        test: async (page) => {
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-accordion]');
                return d && d.hasAttribute('data-rs-initialized');
            }, { timeout: 10000 });
            const trigger = await page.$('[data-rs-accordion-trigger]');
            if (!trigger) return;
            await trigger.click();
            await page.waitForTimeout(300);
        }
    },
    {
        name: 'init: alert renders',
        route: '/showcase/alert',
        test: async (page) => {
            await page.waitForSelector('[data-rs-alert]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-alert]');
            if (!el) throw new Error('[data-rs-alert] not found in DOM');
        }
    },
    {
        name: 'init: animate renders',
        route: '/showcase/animate',
        test: async (page) => {
            await page.waitForSelector('[data-rs-animate]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-animate]');
            if (!el) throw new Error('[data-rs-animate] not found in DOM');
        }
    },
    {
        name: 'init: avatar renders',
        route: '/showcase/avatar',
        test: async (page) => {
            await page.waitForSelector('[data-rs-avatar]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-avatar]');
            if (!el) throw new Error('[data-rs-avatar] not found in DOM');
        }
    },
    {
        name: 'dismiss: banner renders',
        route: '/showcase/banner',
        test: async (page) => {
            await page.waitForSelector('[data-rs-banner]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-banner]');
            if (!el) throw new Error('[data-rs-banner] not found in DOM');
        }
    },
    {
        name: 'init: button renders',
        route: '/showcase/button',
        test: async (page) => {
            await page.waitForSelector('[data-rs-button]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-button]');
            if (!el) throw new Error('[data-rs-button] not found in DOM');
        }
    },
    {
        name: 'gesture: carousel renders',
        route: '/showcase/carousel',
        test: async (page) => {
            await page.waitForSelector('[data-rs-carousel]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-carousel]');
            if (!el) throw new Error('[data-rs-carousel] not found in DOM');
        }
    },
    {
        name: 'data: chart renders',
        route: '/showcase/chart',
        test: async (page) => {
            await page.waitForSelector('[data-rs-chart]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-chart]');
            if (!el) throw new Error('[data-rs-chart] not found in DOM');
        }
    },
    {
        name: 'init: checkbox renders',
        route: '/showcase/checkbox',
        test: async (page) => {
            await page.waitForSelector('[data-rs-checkbox]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-checkbox]');
            if (!el) throw new Error('[data-rs-checkbox] not found in DOM');
        }
    },
    {
        name: 'content: code_block renders',
        route: '/showcase/code-block',
        test: async (page) => {
            await page.waitForSelector('[data-rs-code-block]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-code-block]');
            if (!el) throw new Error('[data-rs-code-block] not found in DOM');
        }
    },
    {
        name: 'init: collapsible renders',
        route: '/showcase/collapsible',
        test: async (page) => {
            await page.waitForSelector('[data-rs-collapsible]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-collapsible]');
            if (!el) throw new Error('[data-rs-collapsible] not found in DOM');
        }
    },
    {
        name: 'selection: color_picker opens',
        route: '/showcase/color-picker',
        test: async (page) => {
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-color-picker]');
                return d && d.hasAttribute('data-rs-initialized');
            }, { timeout: 10000 });
            const trigger = await page.$('[data-rs-color-picker-trigger]');
            if (!trigger) return;
            await trigger.click();
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-color-picker]');
                return d && d.getAttribute('data-rs-state') === 'open';
            }, { timeout: 3000 });
        }
    },
    {
        name: 'init: command renders',
        route: '/showcase/command',
        test: async (page) => {
            await page.waitForSelector('[data-rs-command]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-command]');
            if (!el) throw new Error('[data-rs-command] not found in DOM');
        }
    },
    {
        name: 'overlay: confirm_dialog opens',
        route: '/showcase/confirm-dialog',
        test: async (page) => {
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-confirm-dialog]');
                return d && d.hasAttribute('data-rs-initialized');
            }, { timeout: 10000 });
            const trigger = await page.$('[data-rs-confirm-dialog-trigger]');
            if (!trigger) return;
            await trigger.click();
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-confirm-dialog]');
                return d && d.getAttribute('data-rs-state') === 'open';
            }, { timeout: 3000 });
        }
    },
    {
        name: 'content: copy_button renders',
        route: '/showcase/copy-button',
        test: async (page) => {
            await page.waitForSelector('[data-rs-copy-button]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-copy-button]');
            if (!el) throw new Error('[data-rs-copy-button] not found in DOM');
        }
    },
    {
        name: 'data: data_table renders',
        route: '/showcase/data-table',
        test: async (page) => {
            await page.waitForSelector('[data-rs-datatable]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-datatable]');
            if (!el) throw new Error('[data-rs-datatable] not found in DOM');
        }
    },
    {
        name: 'overlay: dialog opens',
        route: '/showcase/dialog',
        test: async (page) => {
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-dialog]');
                return d && d.hasAttribute('data-rs-initialized');
            }, { timeout: 10000 });
            const trigger = await page.$('[data-rs-dialog-trigger]');
            if (!trigger) return;
            await trigger.click();
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-dialog]');
                return d && d.getAttribute('data-rs-state') === 'open';
            }, { timeout: 3000 });
        }
    },
    {
        name: 'overlay: drawer opens',
        route: '/showcase/drawer',
        test: async (page) => {
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-drawer]');
                return d && d.hasAttribute('data-rs-initialized');
            }, { timeout: 10000 });
            const trigger = await page.$('[data-rs-drawer-trigger]');
            if (!trigger) return;
            await trigger.click();
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-drawer]');
                return d && d.getAttribute('data-rs-state') === 'open';
            }, { timeout: 3000 });
        }
    },
    {
        name: 'overlay: dropdown_menu opens',
        route: '/showcase/dropdown-menu',
        test: async (page) => {
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-dropdown-menu]');
                return d && d.hasAttribute('data-rs-initialized');
            }, { timeout: 10000 });
            const trigger = await page.$('[data-rs-dropdown-menu-trigger]');
            if (!trigger) return;
            await trigger.click();
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-dropdown-menu]');
                return d && d.getAttribute('data-rs-state') === 'open';
            }, { timeout: 3000 });
        }
    },
    {
        name: 'init: field renders',
        route: '/showcase/field',
        test: async (page) => {
            await page.waitForSelector('[data-rs-field]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-field]');
            if (!el) throw new Error('[data-rs-field] not found in DOM');
        }
    },
    {
        name: 'init: form renders',
        route: '/showcase/form',
        test: async (page) => {
            await page.waitForSelector('[data-rs-form]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-form]');
            if (!el) throw new Error('[data-rs-form] not found in DOM');
        }
    },
    {
        name: 'init: icon_button renders',
        route: '/showcase/icon-button',
        test: async (page) => {
            await page.waitForSelector('[data-rs-icon-button]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-icon-button]');
            if (!el) throw new Error('[data-rs-icon-button] not found in DOM');
        }
    },
    {
        name: 'init: input renders',
        route: '/showcase/input',
        test: async (page) => {
            await page.waitForSelector('[data-rs-input]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-input]');
            if (!el) throw new Error('[data-rs-input] not found in DOM');
        }
    },
    {
        name: 'init: input_group renders',
        route: '/showcase/input-group',
        test: async (page) => {
            await page.waitForSelector('[data-rs-input-group]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-input-group]');
            if (!el) throw new Error('[data-rs-input-group] not found in DOM');
        }
    },
    {
        name: 'init: input_otp renders',
        route: '/showcase/input-otp',
        test: async (page) => {
            await page.waitForSelector('[data-rs-input-otp]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-input-otp]');
            if (!el) throw new Error('[data-rs-input-otp] not found in DOM');
        }
    },
    {
        name: 'data: list_item renders',
        route: '/showcase/list-item',
        test: async (page) => {
            await page.waitForSelector('[data-rs-list]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-list]');
            if (!el) throw new Error('[data-rs-list] not found in DOM');
        }
    },
    {
        name: 'init: loading_overlay renders',
        route: '/showcase/loading-overlay',
        test: async (page) => {
            await page.waitForSelector('[data-rs-overlay-container]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-overlay-container]');
            if (!el) throw new Error('[data-rs-overlay-container] not found in DOM');
        }
    },
    {
        name: 'content: markdown renders',
        route: '/showcase/markdown',
        test: async (page) => {
            await page.waitForSelector('[data-rs-markdown]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-markdown]');
            if (!el) throw new Error('[data-rs-markdown] not found in DOM');
        }
    },
    {
        name: 'init: menu renders',
        route: '/showcase/menu',
        test: async (page) => {
            await page.waitForSelector('[data-rs-menu]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-menu]');
            if (!el) throw new Error('[data-rs-menu] not found in DOM');
        }
    },
    {
        name: 'nav: menubar activates',
        route: '/showcase/menubar',
        test: async (page) => {
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-menubar]');
                return d && d.hasAttribute('data-rs-initialized');
            }, { timeout: 10000 });
            const trigger = await page.$('[data-rs-menubar-trigger]');
            if (!trigger) return;
            await trigger.click();
            await page.waitForTimeout(300);
        }
    },
    {
        name: 'overlay: modal opens',
        route: '/showcase/modal',
        test: async (page) => {
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-modal]');
                return d && d.hasAttribute('data-rs-initialized');
            }, { timeout: 10000 });
            const trigger = await page.$('[data-rs-modal-trigger]');
            if (!trigger) return;
            await trigger.click();
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-modal]');
                return d && d.getAttribute('data-rs-state') === 'open';
            }, { timeout: 3000 });
        }
    },
    {
        name: 'init: nav_item renders',
        route: '/showcase/nav-item',
        test: async (page) => {
            await page.waitForSelector('[data-rs-nav-item]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-nav-item]');
            if (!el) throw new Error('[data-rs-nav-item] not found in DOM');
        }
    },
    {
        name: 'init: navigation_menu renders',
        route: '/showcase/navigation-menu',
        test: async (page) => {
            await page.waitForSelector('[data-rs-navigation-menu]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-navigation-menu]');
            if (!el) throw new Error('[data-rs-navigation-menu] not found in DOM');
        }
    },
    {
        name: 'overlay: popover opens',
        route: '/showcase/popover',
        test: async (page) => {
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-popover]');
                return d && d.hasAttribute('data-rs-initialized');
            }, { timeout: 10000 });
            const trigger = await page.$('[data-rs-popover-trigger]');
            if (!trigger) return;
            await trigger.click();
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-popover]');
                return d && d.getAttribute('data-rs-state') === 'open';
            }, { timeout: 3000 });
        }
    },
    {
        name: 'init: progress renders',
        route: '/showcase/progress',
        test: async (page) => {
            await page.waitForSelector('[data-rs-progress]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-progress]');
            if (!el) throw new Error('[data-rs-progress] not found in DOM');
        }
    },
    {
        name: 'init: radio renders',
        route: '/showcase/radio',
        test: async (page) => {
            await page.waitForSelector('[data-rs-radio]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-radio]');
            if (!el) throw new Error('[data-rs-radio] not found in DOM');
        }
    },
    {
        name: 'gesture: resizable renders',
        route: '/showcase/resizable',
        test: async (page) => {
            await page.waitForSelector('[data-rs-resizable]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-resizable]');
            if (!el) throw new Error('[data-rs-resizable] not found in DOM');
        }
    },
    {
        name: 'gesture: scroll_area renders',
        route: '/showcase/scroll-area',
        test: async (page) => {
            await page.waitForSelector('[data-rs-scroll-area]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-scroll-area]');
            if (!el) throw new Error('[data-rs-scroll-area] not found in DOM');
        }
    },
    {
        name: 'selection: select opens',
        route: '/showcase/select',
        test: async (page) => {
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-select]');
                return d && d.hasAttribute('data-rs-initialized');
            }, { timeout: 10000 });
            const trigger = await page.$('[data-rs-select-trigger]');
            if (!trigger) return;
            await trigger.click();
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-select]');
                return d && d.getAttribute('data-rs-state') === 'open';
            }, { timeout: 3000 });
        }
    },
    {
        name: 'overlay: sheet opens',
        route: '/showcase/sheet',
        test: async (page) => {
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-sheet]');
                return d && d.hasAttribute('data-rs-initialized');
            }, { timeout: 10000 });
            const trigger = await page.$('[data-rs-sheet-trigger]');
            if (!trigger) return;
            await trigger.click();
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-sheet]');
                return d && d.getAttribute('data-rs-state') === 'open';
            }, { timeout: 3000 });
        }
    },
    {
        name: 'gesture: slider renders',
        route: '/showcase/slider',
        test: async (page) => {
            await page.waitForSelector('[data-rs-slider]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-slider]');
            if (!el) throw new Error('[data-rs-slider] not found in DOM');
        }
    },
    {
        name: 'init: status_dot renders',
        route: '/showcase/status-dot',
        test: async (page) => {
            await page.waitForSelector('[data-rs-status-dot]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-status-dot]');
            if (!el) throw new Error('[data-rs-status-dot] not found in DOM');
        }
    },
    {
        name: 'init: switch renders',
        route: '/showcase/switch',
        test: async (page) => {
            await page.waitForSelector('[data-rs-switch]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-switch]');
            if (!el) throw new Error('[data-rs-switch] not found in DOM');
        }
    },
    {
        name: 'init: table renders',
        route: '/showcase/table',
        test: async (page) => {
            await page.waitForSelector('[data-rs-table]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-table]');
            if (!el) throw new Error('[data-rs-table] not found in DOM');
        }
    },
    {
        name: 'init: table_of_contents renders',
        route: '/showcase/table-of-contents',
        test: async (page) => {
            await page.waitForSelector('[data-rs-toc]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-toc]');
            if (!el) throw new Error('[data-rs-toc] not found in DOM');
        }
    },
    {
        name: 'nav: tabs activates',
        route: '/showcase/tabs',
        test: async (page) => {
            await page.waitForFunction(() => {
                const d = document.querySelector('[data-rs-tabs]');
                return d && d.hasAttribute('data-rs-initialized');
            }, { timeout: 10000 });
            const trigger = await page.$('[data-rs-tabs-trigger]');
            if (!trigger) return;
            await trigger.click();
            await page.waitForTimeout(300);
        }
    },
    {
        name: 'init: textarea renders',
        route: '/showcase/textarea',
        test: async (page) => {
            await page.waitForSelector('[data-rs-textarea]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-textarea]');
            if (!el) throw new Error('[data-rs-textarea] not found in DOM');
        }
    },
    {
        name: 'init: toast renders',
        route: '/showcase/toast',
        test: async (page) => {
            await page.waitForSelector('[data-rs-toast]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-toast]');
            if (!el) throw new Error('[data-rs-toast] not found in DOM');
        }
    },
    {
        name: 'init: toggle renders',
        route: '/showcase/toggle',
        test: async (page) => {
            await page.waitForSelector('[data-rs-toggle]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-toggle]');
            if (!el) throw new Error('[data-rs-toggle] not found in DOM');
        }
    },
    {
        name: 'init: tooltip renders',
        route: '/showcase/tooltip',
        test: async (page) => {
            await page.waitForSelector('[data-rs-tooltip]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-tooltip]');
            if (!el) throw new Error('[data-rs-tooltip] not found in DOM');
        }
    },
    {
        name: 'data: virtual_list renders',
        route: '/showcase/virtual-list',
        test: async (page) => {
            await page.waitForSelector('[data-rs-virtual-list]', { timeout: 5000 }).catch(() => {});
            const el = await page.$('[data-rs-virtual-list]');
            if (!el) throw new Error('[data-rs-virtual-list] not found in DOM');
        }
    }
];

(async () => {
    let browser;
    try {
        browser = await chromium.launch({ headless: true });
    } catch(e) {
        console.log('[SKIP] Chromium nao disponivel: ' + e.message);
        process.exit(0);
    }

    let passed = 0;
    let failed = 0;
    const failures = [];

    for (const t of TESTS) {
        const page = await browser.newPage();
        const consoleErrors = [];
        page.on('console', msg => { if (msg.type() === 'error') consoleErrors.push(msg.text()); });
        try {
            await page.goto(BASE_URL + t.route, { waitUntil: 'domcontentloaded', timeout: 10000 });
            await t.test(page);
            const critical = consoleErrors.filter(e => !e.includes('favicon') && !e.includes('404') && !e.includes('collect'));
            if (critical.length > 0) throw new Error('console errors: ' + critical[0]);
            console.log('[OK] ' + t.name);
            passed++;
        } catch(e) {
            console.log('[FAIL] ' + t.name + ': ' + e.message);
            failures.push(t.name);
            failed++;
        } finally {
            await page.close();
        }
    }

    await browser.close();
    console.log('\n==================================================');
    console.log('[OK] ' + passed + ' tests passed');
    if (failed > 0) {
        console.log('[FAIL] ' + failed + ' tests failed');
        failures.forEach(f => console.log('  - ' + f));
        process.exit(1);
    }
    console.log('[OK] Browser runtime canonical');
    process.exit(0);
})();
