---
type: concept
title: Project Toolchain
description: package.json scripts and engines, pnpm workspace whitelist, ESLint and Prettier configuration, tsconfig references, and the narrowest validation commands.
tags: [tooling, package-manager, lint, typescript]
---

# Project Toolchain

The project is a single pnpm-managed Nuxt 4 application. `package.json` (`name: "nuxt-app"`) declares `"type": "module"`, is private, and requires Node `>=22` and pnpm `>=10` (`engines`; also reflected in `pnpm-workspace.yaml`, which only whitelists build scripts for `@parcel/watcher`, `esbuild`, `unrs-resolver`, and `vue-demi`).

## Scripts

| Script | Command | Purpose |
| --- | --- | --- |
| `build` | `nuxt build` | Production build (CI step) |
| `dev` | `nuxt dev` | Development server |
| `generate` | `nuxt generate` | Static site generation |
| `preview` | `nuxt preview` | Preview the built app |
| `postinstall` | `nuxt prepare` | Generate `.nuxt` tsconfigs |
| `lint` | `eslint . --fix` | Auto-fix (never used in gates) |
| `lint:check` | `eslint .` | Read-only lint (pre-push, CI) |
| `format` | `prettier --write .` | Format everything |
| `test` | `vitest run` | Run the unit test suite |
| `typecheck` | `nuxt typecheck` | Vue/TS type checking |
| `check:pre-push` | `pnpm run lint:check && pnpm typecheck` | Pre-push gate |
| `prepare` | `husky` | Install husky hooks |

`lint-staged` runs `eslint --fix` + `prettier --write` on staged `.vue/.js/.jsx/.cjs/.mjs/.ts/.tsx/.cts/.mts` and `prettier --write` on staged `.css/.scss/.less` only — Markdown/JSON/YAML are deliberately excluded (AGENTS.md: no Markdown lint or unrelated format gates).

## Dependencies

- **Runtime**: `nuxt` ^4.4.8, `vue` ^3.5.35, `vue-router` ^5.1.0, `@nuxt/ui` ^4.8.2, `@pinia/nuxt` ^0.11.3 + `pinia` ^3.0.4, `@vueuse/core` ^14.4.0, `reka-ui` ^2.10.3, `class-variance-authority`, `clsx`, `tailwind-merge`, `tw-animate-css`, `@lucide/vue`, plus the visual stack (`gsap` ^3.15.0, `ogl` ^1.0.11, `motion-v` ^1.10.3).
- **Milkdown (editor, locked, unused)**: `@milkdown/core`, `@milkdown/vue`, `@milkdown/preset-commonmark`, `@milkdown/preset-gfm`, `@milkdown/plugin-history` — all `^7.22.1`. No editor feature is wired; Milkdown may only enter through `infrastructure/markdown` (see [Infrastructure Layer](/openwiki/architecture/infrastructure-layer.md)).
- **Dev**: `vitest` ^4.1.10, `typescript` ^6.0.3, `vue-tsc` ^3.3.9, `eslint` ^9.39.4, `@nuxt/eslint-config` ^1.16.0, `prettier` ^3.8.4, `tailwindcss` ^4.3.1, `@tailwindcss/vite` ^4.3.1, `@commitlint/cli` ^21.0.2, `@commitlint/config-conventional` ^21.0.2, `husky` ^9.1.7, `lint-staged` ^17.0.7.

## Lint and format configuration

- `eslint.config.mjs` — `createConfigForNuxt()` (Nuxt default rules, no extra stylistic gates), with ignores for `app/components/ui/**`, `app/components/CardNav/**`, `app/components/SpecularButton/**` (upstream-maintained/vendor code).
- `.prettierrc` — 2-space indent, single quotes, semicolons, `trailingComma: 'es5'`, `printWidth: 100`, `vueIndentScriptAndStyle: true`.
- `.editorconfig` — UTF-8, LF, 2-space indent; Markdown keeps trailing whitespace.

## TypeScript

`tsconfig.json` is solution-style: `"files": []` plus `references` to the four Nuxt-generated configs (`.nuxt/tsconfig.app.json`, `.nuxt/tsconfig.server.json`, `.nuxt/tsconfig.shared.json`, `.nuxt/tsconfig.node.json`) produced by `nuxt prepare` (postinstall). Root-level TS files in `domain/`, `application/`, etc. are type-checked through this generated setup — run `pnpm typecheck` after adding files to those layers.

## Nuxt wiring

See [Nuxt Convention Directories](/openwiki/architecture/nuxt-conventions.md) for `nuxt.config.ts` (modules, css entry, components auto-import, fonts/icons providers disabled, `@domain`/`@application`/`@interface-adapters`/`@infrastructure` aliases, favicon, `compatibilityDate`).

## Narrowest validation commands

| Intent | Command |
| --- | --- |
| Fastest sanity (staged files only) | `pnpm exec lint-staged` (pre-commit) |
| Read-only lint | `pnpm run lint:check` |
| Type checking | `pnpm typecheck` |
| Unit tests | `pnpm test` |
| Single test file | `pnpm test -- tests/unit/<file>.test.ts` |
| Pre-push gate | `pnpm run check:pre-push` |
| Full CI parity | `pnpm run lint:check && pnpm typecheck && pnpm test && pnpm build` |

There is no Markdown lint, spell check, or other format gate; hooks do not run builds (see [Git Hooks and CI](/openwiki/tooling/hooks-and-ci.md)).
 (see [Git Hooks and CI](/openwiki/tooling/hooks-and-ci.md)).
