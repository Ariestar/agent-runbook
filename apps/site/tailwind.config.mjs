/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
  theme: {
    extend: {
      colors: {
        background: 'rgb(var(--color-background) / <alpha-value>)',
        surface: 'rgb(var(--color-background) / <alpha-value>)',
        'surface-dim': 'rgb(var(--color-surface-dim) / <alpha-value>)',
        'surface-low': 'rgb(var(--color-surface-low) / <alpha-value>)',
        'surface-mid': 'rgb(var(--color-surface-mid) / <alpha-value>)',
        'surface-high': 'rgb(var(--color-surface-high) / <alpha-value>)',
        'surface-bright': 'rgb(var(--color-surface-bright) / <alpha-value>)',
        primary: 'rgb(var(--color-primary) / <alpha-value>)',
        acid: 'rgb(var(--color-acid) / <alpha-value>)',
        'acid-dim': 'rgb(var(--color-acid-dim) / <alpha-value>)',
        'acid-dark': 'rgb(var(--color-acid-dark) / <alpha-value>)',
        secondary: 'rgb(var(--color-secondary) / <alpha-value>)',
        muted: 'rgb(var(--color-muted) / <alpha-value>)',
        'muted-variant': 'rgb(var(--color-muted-variant) / <alpha-value>)',
        outline: 'rgb(var(--color-outline) / <alpha-value>)',
        'outline-variant': 'rgb(var(--color-outline-variant) / <alpha-value>)',
        danger: 'rgb(var(--color-danger) / <alpha-value>)',
        warning: 'rgb(var(--color-warning) / <alpha-value>)',
        info: 'rgb(var(--color-info) / <alpha-value>)',
      },
      fontFamily: {
        mono: ['JetBrains Mono', 'ui-monospace', 'SFMono-Regular', 'Menlo', 'monospace'],
      },
      fontSize: {
        display: ['48px', { lineHeight: '1.1', letterSpacing: '-0.04em', fontWeight: '800' }],
        headline: ['24px', { lineHeight: '1.2', letterSpacing: '-0.02em', fontWeight: '700' }],
        title: ['20px', { lineHeight: '1.2', letterSpacing: '-0.01em', fontWeight: '600' }],
        body: ['14px', { lineHeight: '1.55', letterSpacing: '0em', fontWeight: '400' }],
        code: ['12px', { lineHeight: '1.4', letterSpacing: '0em', fontWeight: '500' }],
        label: ['11px', { lineHeight: '1', letterSpacing: '0.15em', fontWeight: '700' }],
      },
      backgroundImage: {
        scanline: 'linear-gradient(to bottom, rgba(255,255,255,0), rgba(255,255,255,0) 50%, rgba(0,0,0,0.24) 50%, rgba(0,0,0,0.24))',
        'micro-grid': 'linear-gradient(to right, rgb(var(--color-outline-variant)) 1px, transparent 1px), linear-gradient(to bottom, rgb(var(--color-outline-variant)) 1px, transparent 1px)',
        'dot-grid': 'radial-gradient(rgb(var(--color-outline)) 1px, transparent 1px)',
      },
      boxShadow: {
        hard: '4px 4px 0px 0px rgb(var(--color-outline-variant))',
        'hard-acid': '4px 4px 0px 0px rgb(var(--color-acid-dark))',
        glow: '0 0 14px rgb(var(--color-acid) / 0.35)',
        insetglow: 'inset 0 0 24px rgb(var(--color-acid) / 0.05)',
      },
    },
  },
  plugins: [],
};
