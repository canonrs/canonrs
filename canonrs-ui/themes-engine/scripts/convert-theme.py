import sys
import re

css = sys.stdin.read()

# Separar light e dark
light_section = re.search(r':root\s*{([^}]+(?:[^}]+}[^}]*)*)}', css, re.DOTALL)
dark_section = re.search(r'\.dark\s*{([^}]+(?:[^}]+}[^}]*)*)}', css, re.DOTALL)

def extract_colors(section_text):
    pattern = r'--(\w+(?:-\w+)*): hsl\(([\d.]+)\s+([\d.]+)%\s+([\d.]+)%\);'
    matches = re.findall(pattern, section_text)
    
    colors = []
    for name, h, s, l in matches:
        parts = name.split('-')
        if parts[0] in ['background', 'foreground', 'card', 'popover', 'primary', 'secondary', 'muted', 'accent', 'destructive', 'border', 'input', 'ring', 'chart', 'sidebar', 'shadow']:
            camel = parts[0] + ''.join(p.capitalize() for p in parts[1:])
            colors.append(f"        {camel}: {{ h: {h}, s: {s}, l: {l} }},")
    
    return '\n'.join(colors)

if light_section:
    print("    light: {")
    print("      colors: {")
    print(extract_colors(light_section.group(1)))
    print("      },")
    print("    },")

if dark_section:
    print("    dark: {")
    print("      colors: {")
    print(extract_colors(dark_section.group(1)))
    print("      },")
    print("    },")
