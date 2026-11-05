---
type: concept
title: Nuxt Convention Directories
description: How the Nuxt convention directories (app, server, shared) and nuxt.config.ts wiring shape the application, including modules, aliases, and styling hooks.
tags: [architecture, nuxt, configuration]
---

# Nuxt Convention Directories

The repository keeps Nuxt's convention directories (`app/`, `server/`, `shared/`) separate from the Clean Architecture rings. `app/` and `server/` are the outermost presentation/server entry rings; `shared/` holds side-effect-free code safe for both. See [Architecture Overview](/openwiki/architecture/overview.md) for the dependency-direction model.

## app/ — presentation layer

| Directory | Contract (READMEs) | Status |
| --- | --- | --- |
| `app/app.vue` | Nuxt application root | Implemented — see [App Shell and Routing](/openwiki/presentation/app-shell.md) |
| `app/assets/styles/` | Build-processed global styles | Implemented — see [Theming and Global Styles](/openwiki/presentation/theming.md) |
| `app/components/` | Vue presentation components | Implemented — see [Custom Visual Components](/openwiki/presentation/visual-components.md) and [shadcn-vue UI Kit](/openwiki/presentation/ui-kit.md) |
| `app/composables/` | Nuxt/Vue lifecycle adaptation | Empty (no README even) |
| `app/layouts/` | Common page shells | Implemented — `default.vue` |
| `app/middleware/` | Route middleware only; no domain rules or data access | Empty scaffold |
| `app/pages/` | Route entrypoints | Implemented — `index.vue`, `workspace.vue` placeholders |
| `app/plugins/` | Client/Nuxt app plugins; orchestration goes through the composition root | Empty scaffold |
| `app/stores/` | Pinia presentation state; domain state stays in the domain layer | Empty scaffold |
| `app/types/` | Presentation-layer types only; cross-boundary types go to `shared/types` | Empty scaffold |
| `app/utils/` | Side-effect-free page helpers; no domain rules, HTTP, or persistence | Empty scaffold |
| `app/error.vue` | Error page | Implemented — see [App Shell and Routing](/openwiki/presentation/app-shell.md) |
| `app/lib/utils.ts` | `cn()` class-merge boundary for the UI kit | Implemented — see [shadcn-vue UI Kit](/openwiki/presentation/ui-kit.md) |

## server/ — Nuxt server entrypoints

All four subdirectories are empty scaffolds with README contracts:

- `server/api/` — Nuxt server API route entrypoints; each entry performs protocol conversion and enters the business kernel through an application use case (never direct domain access).
- `server/middleware/` — server middleware.
- `server/plugins/` — server plugins.
- `server/utils/` — server-side utilities.

## shared/ — cross-boundary base code

All four subdirectories are empty scaffolds with README contracts:

- `shared/constants/` — constants usable by both app and server.
- `shared/schemas/` — validation schemas usable by both sides.
- `shared/types/` — types usable by both sides; presentation-only types stay in `app/types`.
- `shared/utils/` — side-effect-free general utilities.

## nuxt.config.ts wiring

[nuxt.config.ts](/nuxt.config.ts) configures the framework:

- **Modules**: `@nuxt/ui` and `@pinia/nuxt`.
- **CSS entry**: `~/assets/styles/main.css` (Tailwind v4 via the `@tailwindcss/vite` plugin, registered under `vite.plugins`).
- **Components**: auto-import directory `~/components` scoped to `.vue` files.
- **Fonts**: Google font providers disabled (`fonts.providers.google: false`, `googleicons: false`).
- **Icons**: provider `none` (the app uses `@lucide/vue` components directly rather than the icon module).
- **App head**: favicon link to `/icon/favicon.svg` with `type: 'image/svg+xml'`.
- **Aliases**: `@domain`, `@application`, `@interface-adapters`, `@infrastructure` mapped to the root Clean Architecture directories (see [Architecture Overview](/openwiki/architecture/overview.md)).
- **compatibilityDate**: `2025-07-15`.
- **Devtools**: disabled.

## Public assets

`public/` is the static web root: `robots.txt` (allow all), `icon/favicon.svg` (brand mark — linked as the site favicon in `nuxt.config.ts` and used as the CardNav logo), and `favicon.ico`. No other static assets exist.

## Ignored runtime artifacts

`.gitignore` excludes Nuxt build outputs (`.output`, `.nuxt`, `.nitro`, `.data`, `.cache`, `dist`), `node_modules`, logs, `.env*`, and local agent/browser artifacts (`.learnings/`, `output/`, `.playwright-cli/`). None of these are source; do not document or commit them.

## TypeScript

`tsconfig.json` is a solution-style file referencing the four Nuxt-generated configs (`.nuxt/tsconfig.{app,server,shared,node}.json`) produced by `nuxt prepare` (run automatically by the `postinstall` script). Run `pnpm typecheck` after scaffolding changes; the generated configs must exist or typecheck fails.
