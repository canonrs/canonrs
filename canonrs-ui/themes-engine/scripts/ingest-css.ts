#!/usr/bin/env tsx
import * as fs from 'fs';
import * as path from 'path';

interface HSLColor {
  h: number;
  s: number;
  l: number;
}

function parseHSL(value: string): HSLColor {
  const match = value.match(/hsl\(([\d.]+)\s+([\d.]+)%\s+([\d.]+)%\)/);
  if (!match) throw new Error(`Invalid HSL: ${value}`);
  return {
    h: parseFloat(match[1]),
    s: parseFloat(match[2]),
    l: parseFloat(match[3]),
  };
}

function parseCSS(cssContent: string) {
  const lightVars: Record<string, string> = {};
  const darkVars: Record<string, string> = {};
  
  // Parse :root
  const rootMatch = cssContent.match(/:root\s*\{([^}]+)\}/s);
  if (rootMatch) {
    const vars = rootMatch[1].match(/--[\w-]+:\s*[^;]+;/g) || [];
    vars.forEach(v => {
      const [key, value] = v.split(':').map(s => s.trim());
      lightVars[key.replace('--', '')] = value.replace(';', '');
    });
  }
  
  // Parse .dark
  const darkMatch = cssContent.match(/\.dark\s*\{([^}]+)\}/s);
  if (darkMatch) {
    const vars = darkMatch[1].match(/--[\w-]+:\s*[^;]+;/g) || [];
    vars.forEach(v => {
      const [key, value] = v.split(':').map(s => s.trim());
      darkVars[key.replace('--', '')] = value.replace(';', '');
    });
  }
  
  return { lightVars, darkVars };
}

function mapToTheme(vars: Record<string, string>, mode: 'light' | 'dark') {
  const colors: Record<string, HSLColor> = {};
  const contextual: any = { sidebar: {} };
  const charts: Record<string, HSLColor> = {};
  const typography: any = {};
  const radius: any = {};
  const shadows: any = {};
  const spacing: any = {};
  
  Object.entries(vars).forEach(([key, value]) => {
    // Colors
    if (['background', 'foreground', 'card', 'card-foreground', 'popover', 'popover-foreground',
         'primary', 'primary-foreground', 'secondary', 'secondary-foreground',
         'muted', 'muted-foreground', 'accent', 'accent-foreground',
         'destructive', 'destructive-foreground', 'border', 'input', 'ring'].includes(key)) {
      const camelKey = key.replace(/-([a-z])/g, (_, c) => c.toUpperCase());
      colors[camelKey] = parseHSL(value);
    }
    
    // Sidebar contextual
    if (key.startsWith('sidebar-')) {
      const subKey = key.replace('sidebar-', '').replace(/-([a-z])/g, (_, c) => c.toUpperCase());
      contextual.sidebar[subKey] = parseHSL(value);
    }
    
    // Charts
    if (key.startsWith('chart-')) {
      const num = key.replace('chart-', '');
      charts[`chart${num}`] = parseHSL(value);
    }
    
    // Typography
    if (key.startsWith('font-')) {
      const subKey = key.replace('font-', '').replace(/-([a-z])/g, (_, c) => c.toUpperCase());
      typography[subKey] = value;
    }
    
    // Radius
    if (key === 'radius') {
      radius.base = parseFloat(value);
    }
    
    // Shadows
    if (key.startsWith('shadow-') && !key.includes('color')) {
      const subKey = key.replace('shadow-', '');
      if (['x', 'y', 'blur', 'spread', 'opacity'].includes(subKey)) {
        if (!shadows.base) shadows.base = {};
        shadows.base[subKey] = parseFloat(value);
      }
    }
    
    if (key === 'shadow-color') {
      if (!shadows.base) shadows.base = {};
      shadows.base.color = parseHSL(value);
    }
    
    // Spacing
    if (key === 'spacing') {
      spacing.base = parseFloat(value);
    }
  });
  
  return { colors, contextual, charts, typography, radius, shadows, spacing };
}

function generateTS(themeName: string, light: any, dark: any): string {
  return `import { ThemeDefinition } from '../../token-engine/schema';

export const ${themeName}: ThemeDefinition = {
  id: '${themeName.replace(/([A-Z])/g, '-$1').toLowerCase().replace(/^-/, '')}',
  name: '${themeName.replace(/([A-Z])/g, ' $1').trim()}',
  description: 'Generated theme',
  modes: {
    light: ${JSON.stringify(light, null, 6).replace(/"([^"]+)":/g, '$1:')},
    dark: ${JSON.stringify(dark, null, 6).replace(/"([^"]+)":/g, '$1:')},
  },
};
`;
}

// Main
const args = process.argv.slice(2);
if (args.length === 0) {
  console.error('Usage: ingest-css <input.css>');
  process.exit(1);
}

const inputFile = args[0];
const themeName = path.basename(inputFile, '.css').replace(/-([a-z])/g, (_, c) => c.toUpperCase());
const cssContent = fs.readFileSync(inputFile, 'utf-8');

const { lightVars, darkVars } = parseCSS(cssContent);
const light = mapToTheme(lightVars, 'light');
const dark = mapToTheme(darkVars, 'dark');

const tsContent = generateTS(themeName, light, dark);
const outputPath = path.join(__dirname, '../../tokens/themes/presets', `${path.basename(inputFile, '.css')}.ts`);

fs.writeFileSync(outputPath, tsContent);
console.log(`✅ Generated: ${outputPath}`);
