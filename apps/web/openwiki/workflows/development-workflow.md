---
type: concept
title: Development Workflow
description: Recipes for common changes (page, component, first business feature, theming) and the verification gate sequence enforced by hooks and CI.
tags: [workflows, contribution, verification]
---

# Development Workflow

This page collects the recurring change recipes for the repository and the verification gates every change passes. The procedural baseline is [docs/guides/development-workflow.md](/docs/guides/development-workflow.md); AGENTS.md holds the stable rules.

## Common recipes

### Add or edit a page (e.g. a new knowledge-space page)

1. Create `app/pages/<name>.vue` — it automatically renders inside `layouts/default.vue` (the brand shell) via `NuxtPage`; see [App Shell and Routing](/openwiki/presentation/app-shell.md).
2. Compose only composables and presentation components in the page (AGENTS.md: `app/pages` composes, components display, composables adapt Nuxt lifecycles). Do not call `$fetch`, instantiate repositories, or write business rules in the page.
3. Reuse the global `.page-placeholder` class or scoped styles; do not extend the dead `.hero-*` styles (see [Theming](/openwiki/presentation/theming.md)).
4. Validate: `pnpm dev`, check the route and narrow screens (no horizontal overflow), then `pnpm run lint:check` and `pnpm typecheck`.

### Add or edit a visual component

1. Follow the mount → `requestAnimationFrame` loop → resize → teardown pattern of the existing WebGL components, or the GSAP timeline pattern of CardNav; see [Custom Visual Components](/openwiki/presentation/visual-components.md).
2. Components are props-driven; colors/sizes come from the consumer (layout or page). Keep business rules out.
3. If the component is vendor-style/upstream-maintained, extend the ESLint ignores in `eslint.config.mjs`; otherwise it must pass lint.
4. Validate manually (no component test harness exists) plus `pnpm run lint:check`/`pnpm typecheck`.

### Add the first business feature (full Clean Architecture chain)

The documented "document detail" recipe from [docs/architecture/clean-architecture.md](/docs/architecture/clean-architecture.md), which maps 1:1 to the wiki's layer pages:

1. **Domain** ([page](/openwiki/architecture/domain-layer.md)): define the entity/value objects and declare the repository capability in `domain/repositories/`.
2. **Application** ([page](/openwiki/architecture/application-layer.md)): write `application/use-cases/get-document-detail.ts` (single business goal, returns a DTO), with `application/dto/` and `application/ports/` as needed. Write the failing test first (`tests/unit/get-document-detail.test.ts`).
3. **Interface-adapters** ([page](/openwiki/architecture/interface-adapters-layer.md)): create controller (route-param conversion), mapper, presenter, and view model.
4. **Infrastructure** ([page](/openwiki/architecture/infrastructure-layer.md)): implement the data source (`infrastructure/persistence/` or `http/`) and the composition root `infrastructure/nuxt/<feature>.ts` that wires repository → use case → controller.
5. **Presentation**: composable in `app/composables/`, page in `app/pages/`, display components in `app/components/<feature>/`.
<!-- openwiki: broken internal link [vue|nuxt|@milkdown|pinia] file "vue|nuxt|@milkdown|pinia" does not exist. Fix the href or restore the target, then delete this comment. -->
6. Validate: `pnpm test` (fail-first suites), the dependency-direction grep `rg -n "from ['\"](vue|nuxt|@milkdown|pinia)" domain application` must return nothing, then `pnpm run lint:check`, `pnpm typecheck`, `pnpm build`.

### Change theming or brand colors

Edit the `:root` variables and `@theme inline` mapping in `app/assets/styles/main.css` — see [Theming and Global Styles](/openwiki/presentation/theming.md) for the token table and the dead-style warning.

### Change commit rules

Edit `scripts/validate-commit-message.mjs` **and** `tests/unit/validate-commit-message.test.ts` together (fail-first), then docs — see [Commit Message Validation](/openwiki/tooling/commit-message-validation.md).

## Verification gates

| Gate | Runs | Purpose |
| --- | --- | --- |
| pre-commit | `lint-staged` (ESLint --fix + Prettier on staged code/CSS) | Fast format/lint on staged files |
| commit-msg | `commitlint --edit` + custom validator | Enforce `type(scope): 中文摘要` and body rules |
| pre-push | `lint:check` + `typecheck` | Cheap full-project cross-file check |
| CI | `lint:check` + `typecheck` + `test` + `build` | Full parity on push/PR |

Hooks never run Markdown checks, spell checks, or builds. Full details on [Git Hooks and CI](/openwiki/tooling/hooks-and-ci.md).

## Commit conventions

- Header: `type(scope): 中文摘要`; `type` from `feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert`; `scope` lowercase letters/digits/hyphens; summary must contain Chinese.
- Normal commits: header only, no body.
- Breaking commits: `type(scope)!:` plus a `BREAKING CHANGE: ...` body line.
- One atomic commit per completed stage, only after verification passes; no force-push or history rewrite; don't push without being asked (AGENTS.md).

## Cannot-do list

- No business rules written directly into pages/components.
- No Vue/Nuxt/Milkdown/Pinia/HTTP/database imports in `domain/` or `application/`.
- No database records passed as page props (convert via DTO/mapper/presenter).
- No "顺手整理" (incidental) changes to unrelated modules.
- No claiming build/test/typecheck passes without fresh verification.
- No hand-editing generated `openwiki/` pages (the scheduled workflow regenerates them — see [OpenWiki Skills Library](/openwiki/reference/openwiki-skills.md)).
