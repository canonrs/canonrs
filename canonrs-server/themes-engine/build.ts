import { ThemeDefinition, HSLColor } from './schema';
import * as presets from '../tokens/themes/presets';

function hslToString(color: HSLColor): string {
  return `${color.h} ${color.s}% ${color.l}%`;
}

function generateThemeCSS(theme: ThemeDefinition): string {
  const lines: string[] = [];
  
  lines.push(`/* ${theme.name} - ${theme.description} */`);
  
  // Light mode (se existir)
  if (theme.modes.light) {
    lines.push(`[data-theme="${theme.id}"] {`);
    Object.entries(theme.modes.light.colors).forEach(([key, value]) => {
      const cssVar = key.replace(/([A-Z])/g, '-$1').toLowerCase();
      lines.push(`  --color-${cssVar}: ${hslToString(value)};`);
    });
    lines.push('}\n');
  }
  
  // Dark mode (sempre existe)
  if (theme.modes.dark) {
    lines.push(`[data-theme="${theme.id}"].dark {`);
    Object.entries(theme.modes.dark.colors).forEach(([key, value]) => {
      const cssVar = key.replace(/([A-Z])/g, '-$1').toLowerCase();
      lines.push(`  --color-${cssVar}: ${hslToString(value)};`);
    });
    lines.push('}\n');
  }
  
  return lines.join('\n');
}

console.log('/* AUTO-GENERATED - DO NOT EDIT */');
console.log('/* Generated from TypeScript theme definitions */');
console.log('/* Run: npm run build:themes to regenerate */\n');

const themes = Object.values(presets).filter(
  (preset): preset is ThemeDefinition =>
    typeof preset === 'object' && preset !== null && 'id' in preset
);

const css = themes.map(generateThemeCSS).join('\n');
console.log(css);
