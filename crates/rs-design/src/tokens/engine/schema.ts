export interface HSLColor {
  h: number;
  s: number;
  l: number;
}

export interface Typography {
  fontSans: string;
  fontSerif: string;
  fontMono: string;
  letterSpacing: number;
}

export interface Spacing {
  radius: number;
  spacing: number;
}

export interface Shadow {
  color: HSLColor;
  opacity: number;
  blur: number;
  spread: number;
  offsetX: number;
  offsetY: number;
}

export interface ThemeColors {
  primary: HSLColor;
  primaryForeground: HSLColor;
  secondary: HSLColor;
  secondaryForeground: HSLColor;
  accent: HSLColor;
  accentForeground: HSLColor;
  background: HSLColor;
  foreground: HSLColor;
  card: HSLColor;
  cardForeground: HSLColor;
  popover: HSLColor;
  popoverForeground: HSLColor;
  muted: HSLColor;
  mutedForeground: HSLColor;
  destructive: HSLColor;
  destructiveForeground: HSLColor;
  border: HSLColor;
  input: HSLColor;
  ring: HSLColor;
  chart1: HSLColor;
  chart2: HSLColor;
  chart3: HSLColor;
  chart4: HSLColor;
  chart5: HSLColor;
  sidebarBackground: HSLColor;
  sidebarForeground: HSLColor;
  sidebarPrimary: HSLColor;
  sidebarPrimaryForeground: HSLColor;
  sidebarAccent: HSLColor;
  sidebarAccentForeground: HSLColor;
  sidebarBorder: HSLColor;
  sidebarRing: HSLColor;
}

export interface ThemeMode {
  colors: ThemeColors;
  typography: Typography;
  spacing: Spacing;
  shadow: Shadow;
}

export interface ThemeDefinition {
  version: number;
  id: string;
  name: string;
  author: string;
  description: string;
  modes: {
    light: ThemeMode;
    dark: ThemeMode;
  };
}
