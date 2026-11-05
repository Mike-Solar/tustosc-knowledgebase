---
type: entrypoint
title: Quickstart — knowledgebase-ui wiki
description: "Entry point for the knowledgebase-ui code wiki: repository map, section navigation, task-routing table, and backlog."
tags: [quickstart, navigation]
---

# Quickstart

`knowledgebase-ui` is a Yuque-like (类语雀) knowledge-base web UI for the TUST Open Atom Open Source Association (天津科技大学开放原子开源协会), built on Nuxt 4 + Vue 3 + TypeScript with pnpm. It is deliberately a **long-lived collaboration skeleton**: the implemented surface today is a polished presentation shell (card navigation, WebGL visuals, global theming) plus a strict tooling/verification contract, while the business core exists as empty [Clean Architecture](architecture/overview.md) scaffolds (`domain/`, `application/`, `interface-adapters/`, `infrastructure/`) awaiting the first real feature. Milkdown editor dependencies are locked but not wired.

Source and tests are authoritative; this wiki is an evidence index. Start with [Architecture Overview](architecture/overview.md) for the layer map, then use the routing table below.

## Wiki sections

| Section | Pages | Covers |
| --- | --- | --- |
| **Architecture** | [Overview](architecture/overview.md), [Domain](architecture/domain-layer.md), [Application](architecture/application-layer.md), [Interface-Adapters](architecture/interface-adapters-layer.md), [Infrastructure](architecture/infrastructure-layer.md), [Nuxt Conventions](architecture/nuxt-conventions.md) | Layer contracts, dependency direction, `@domain`/`@application`/`@interface-adapters`/`@infrastructure` aliases, app/server/shared convention dirs, nuxt.config wiring |
| **Presentation** | [App Shell](presentation/app-shell.md), [Visual Components](presentation/visual-components.md), [shadcn-vue UI Kit](presentation/ui-kit.md), [Theming](presentation/theming.md) | The implemented UI: layout/routes/error page, Aurora/Radar/CardNav/SpecularButton, the ui kit + `cn()`, Tailwind v4 tokens and legacy dead styles |
| **Tooling** | [Project Toolchain](tooling/project-toolchain.md), [Commit Message Validation](tooling/commit-message-validation.md), [Git Hooks and CI](tooling/hooks-and-ci.md) | Scripts/engines/lint config, the custom commit validator, husky hooks, ci.yml, the OpenWiki update workflow |
| **Testing** | [Unit Testing](testing/unit-testing.md) | Vitest setup and the two existing suites |
| **Workflows** | [Development Workflow](workflows/development-workflow.md) | Change recipes and the verification ladder |
| **Reference** | [Repository Docs](reference/repository-docs.md), [OpenWiki Skills](reference/openwiki-skills.md) | docs/ governance, LICENSE/AGENTS/CONTRIBUTING roles, the two tracked OpenWiki skills |

## Task routing

| Intent | Wiki page | Source entrypoints / symbols | Focused tests | Minimal validation |
| --- | --- | --- | --- | --- |
| Understand the architecture and where a feature lands | [Architecture Overview](architecture/overview.md) + layer pages | `nuxt.config.ts` aliases; `AGENTS.md`; `docs/architecture/clean-architecture.md` | none yet (layers empty) | `pnpm run lint:check`, `pnpm typecheck` |
| Change nav cards / header / footer | [App Shell](presentation/app-shell.md) | `app/layouts/default.vue` (`cardNavItems`, slots), `app/components/CardNav/CardNav.vue` | none | `pnpm dev` + narrow-screen check, `pnpm run lint:check` |
| Tune a WebGL visual (Aurora/Radar/SpecularButton) | [Visual Components](presentation/visual-components.md) | `app/components/Aurora/Aurora.vue`, `Radar/Radar.vue`, `SpecularButton/SpecularButton.vue` | none (browser-only) | `pnpm dev` |
| Add or regenerate shadcn kit components | [shadcn-vue UI Kit](presentation/ui-kit.md) | `components.json`, `app/components/ui/**`, `app/lib/utils.ts` (`cn`) | `tests/unit/ui-utils.test.ts` | `pnpm test`, `pnpm typecheck` |
| Change colors/fonts/breakpoints | [Theming](presentation/theming.md) | `app/assets/styles/main.css` (`:root`, `@theme inline`, 840/520px) | none | `pnpm dev`; do not extend dead `.hero-*` rules |
| Add the first business feature (full chain) | [Infrastructure Layer](architecture/infrastructure-layer.md) + [Development Workflow](workflows/development-workflow.md) | `domain/repositories/document.ts` → `application/use-cases/get-document-detail.ts` → `interface-adapters/controllers/document-detail.ts` → `infrastructure/nuxt/document-detail.ts` (composition root) | failing-first unit tests per AGENTS.md | `pnpm test`, `pnpm typecheck`, `pnpm run lint:check` |
| Understand commit message rules / validator | [Commit Message Validation](tooling/commit-message-validation.md) | `scripts/validate-commit-message.mjs` (`validateCommitMessage`, `HEADER_PATTERN`, `BREAKING_BODY_PATTERN`), `commitlint.config.js`, `.husky/commit-msg` | `tests/unit/validate-commit-message.test.ts` (5 cases) | `pnpm test -- tests/unit/validate-commit-message.test.ts` |
| Understand hooks / CI / wiki regeneration | [Git Hooks and CI](tooling/hooks-and-ci.md) | `.husky/pre-commit`, `.husky/commit-msg`, `.husky/pre-push`, `.github/workflows/ci.yml`, `.github/workflows/openwiki-update.yml` | none (infra) | push / open PR |
| Run the right checks for a change | [Project Toolchain](tooling/project-toolchain.md) | `package.json` scripts (`lint:check`, `typecheck`, `test`, `check:pre-push`, `build`) | — | see the commands table there |
| Write a failing-first test | [Unit Testing](testing/unit-testing.md) | `tests/unit/*.test.ts`, vitest | — | `pnpm test -- tests/unit/<file>.test.ts` |
| Update project docs / ADR / plans | [Repository Docs](reference/repository-docs.md) | `docs/architecture/*`, `docs/guides/*`, `docs/adr/*`, `docs/specs/*`, `docs/plans/*` | — | keep `AGENTS.md`/`README.md` in sync |
| Author/validate wiki diagrams or add an OpenWiki connector | [OpenWiki Skills](reference/openwiki-skills.md) | `skills/mermaid-diagrams/SKILL.md`, `skills/write-connector/SKILL.md` | — | run the OpenWiki update workflow |

## Key facts in one glance

- **Framework**: Nuxt 4 (`^4.4.8`), Vue 3, TypeScript 6, pnpm ≥ 10, Node ≥ 22; `name: nuxt-app`, `"type": "module"`.
- **Routes**: `/` and `/workspace`, both minimal placeholders under the shared `layouts/default.vue` shell; errors render `app/error.vue`.
- **Four empty business layers** with documented contracts; the historical home-page example was removed in commit `301a6ff`.
- **Enforced commit format**: `type(scope): 中文摘要`; scope lowercase `[a-z0-9-]`; breaking commits need `!` + `BREAKING CHANGE:` body; normal commits cannot have a body.
- **Verification**: pre-commit lint-staged → commit-msg (commitlint + custom validator) → pre-push (`lint:check` + `typecheck`) → CI (full `lint:check` + `typecheck` + `test` + `build`, no CD).
- **OpenWiki**: `openwiki/` is generated by the scheduled `.github/workflows/openwiki-update.yml` (daily + manual); do not hand-edit generated pages — update source/docs and let the workflow regenerate.

## Backlog

No valid deferrals: every tracked substantial component and workflow (including `skills/` and `LICENSE`) is documented. Intentionally unimplemented areas — Milkdown editing, real business features, server APIs — are documented as intended-but-empty scaffolds with evidence on the [Architecture](architecture/overview.md) pages rather than deferred.
