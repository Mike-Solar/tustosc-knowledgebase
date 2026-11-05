---
type: concept
title: Theming and Global Styles
description: The Tailwind v4 global stylesheet — design tokens, @theme mapping, shell classes, responsive breakpoints — and the legacy dead hero styles still present.
tags: [presentation, styling, tailwind, design-tokens]
---

# Theming and Global Styles

All global styling lives in a single entry: `app/assets/styles/main.css`, registered in `nuxt.config.ts` (`css: ['~/assets/styles/main.css']`) and compiled through Tailwind v4 (`@tailwindcss/vite` plugin). The file is the most-changed file in repository history (9 commits touched it), so treat it as the primary theme surface.

## File structure

1. **Imports**: `@import 'tailwindcss';` and `@import 'tw-animate-css';`, plus `@custom-variant dark (&:is(.dark *));` so dark variants activate under a `.dark` ancestor.
2. **`:root` design tokens** — light color scheme (`color-scheme: light`), font stack (Geist → Avenir Next → Noto Sans SC → Microsoft YaHei → ui-sans-serif → system-ui → -apple-system → BlinkMacSystemFont → Segoe UI → sans-serif), and semantic variables: `--background`/`--foreground` (`#f7f8fb`/`#151922`), `--card`, `--popover`, `--primary` (`#171b25`), `--secondary`, `--muted`, `--accent` (`#e86f4f` — the association's accent orange), `--destructive` (`#c94848`), `--border` (`#dfe4ec`), `--input` (`#dfe4ec`), `--ring` (`#e6a38d`), `--radius` (`0.75rem`), plus a full `--sidebar-*` set (background `#eef1f5`, primary/foreground/accent/border/ring).
3. **`@theme inline` mapping** — bridges the CSS variables into Tailwind utilities (`--color-background: var(--background)`, `--color-sidebar-*`, `--font-heading`). This is why classes like `bg-background`, `text-muted-foreground`, `bg-sidebar` work.
4. **Base element styles** — box-sizing reset, `html`/`body` min-width 320px and background, font inheritance, link color reset.
5. **Shell classes** — `.site-shell` (flex column, min-height 100vh), `.site-header`/`.site-footer` (width `min(1240px, calc(100% - 64px))`), `.site-card-nav` positioning overrides for the layout's `CardNav` (`top: 24px !important`, `width: min(1180px, calc(100% - 48px)) !important`, `max-width: none !important`), `.card-nav-brand*`, `.site-card-nav-action`, and the home backdrop `.home-aurora-backdrop` (`position: fixed; inset: 0; z-index: -1; pointer-events: none`).
6. **`.page-placeholder`** — the centered full-viewport grid used by both placeholder pages (width `min(1240px, calc(100% - 64px))`, min-height `calc(100svh - 232px)`, `h1` with `clamp(30px, 5vw, 56px)`).
7. **Responsive breakpoints** — `@media (max-width: 840px)` reflows `.site-card-nav`'s top bar into a CSS grid `32px minmax(0, 1fr) auto` (hamburger left, centered brand, CTA right), shrinks `.card-nav-brand` to 12px with a 24px brand mark, and hides `.site-nav`; `@media (max-width: 520px)` narrows `.site-header`/`.site-footer`/`.hero-section` to `calc(100% - 32px)` and stacks the footer spans vertically.
8. **`@layer base`** — Tailwind base layer applying `border-border` and `outline-ring/50` to all elements, `bg-background text-foreground` to body.

## Legacy dead styles (do not extend)

`main.css` still contains styles for the **removed** home-page hero (removed in commit `301a6ff` / `6db3e37`): `.hero-section`, `.hero-copy`, `.hero-actions`, `.primary-action`, `.hero-support`, `.hero-visual`, `.hero-orb-frame*`, `.hero-orb-caption`, `.hero-dock`, `.home-dock-panel`, `.home-orb`, `.home-page`, plus `.site-nav`, `.brand`, `.brand-mark`, `.header-action` — remnants of the removed HomeHero/HomeCategoryList/HomeDocumentList example chain. Grep evidence: no `.vue` file references `hero-section`, `site-nav`, `brand`, `header-action`, or `primary-action` today. These rules are dead weight and a trap for new work — new pages should use `.page-placeholder` or new scoped styles, and a cleanup refactor may remove the hero block safely (verify with grep before removing).

## Dark mode

Dark mode is *wired but not activated*: the `dark` custom variant and dark-prefixed classes in the UI kit (e.g. `dark:bg-input/30`) exist, but no `.dark` class is ever applied at runtime (no toggle, no `color-scheme: dark` block in `main.css`). The design is currently light-only in practice.

## Change recipes

- **Change a brand color**: update the corresponding `:root` variable (e.g. `--accent: #e86f4f`); the `@theme inline` mapping propagates it to all utilities.
- **Change the font stack**: edit the `:root` font-family and the `--font-heading` token in `@theme inline`.
- **Add a page layout class**: extend `.page-placeholder` or add a scoped style in the page; do not reuse the dead `.hero-*` classes.
- **Responsive behavior**: extend the 840px/520px breakpoints; the shell relies on these exact widths.

## Focused validation

Styling is validated manually (`pnpm dev`, check narrow screens for horizontal overflow — a documented acceptance criterion in docs/plans) and by `pnpm run lint:check` (CSS formatting via Prettier lint-staged). There is no automated visual test suite.
