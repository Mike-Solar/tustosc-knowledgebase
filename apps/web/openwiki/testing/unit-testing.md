---
type: concept
title: Unit Testing
description: The Vitest unit test setup and the two existing suites (cn() class merge and commit message validation), with focused run commands.
tags: [testing, vitest, unit-tests]
---

# Unit Testing

Unit tests run through **Vitest** via the single script `pnpm test` (`vitest run`, no watch mode by default). All tests live under `tests/unit/`. There is no Vitest config file — the framework defaults are used and tests import source files directly with relative paths.

## Current suites

| Suite | File | Behavior under test | Command |
| --- | --- | --- | --- |
| `cn()` class merge | `tests/unit/ui-utils.test.ts` | `cn('px-2', conditional, 'px-4', 'text-foreground')` resolves Tailwind conflicts: `px-4` wins over `px-2`, falsy values are dropped, result is `'px-4 text-foreground'` | `pnpm test -- tests/unit/ui-utils.test.ts` |
| Commit message validation | `tests/unit/validate-commit-message.test.ts` | Five behaviors of `validateCommitMessage` (see [Commit Message Validation](/openwiki/tooling/commit-message-validation.md)): accepts a normal Chinese commit, rejects a normal commit with a body, rejects a non-Chinese summary, requires a BREAKING CHANGE body for breaking commits, accepts a breaking commit with body | `pnpm test -- tests/unit/validate-commit-message.test.ts` |

The `cn()` test is the only coverage of the UI kit's class-merge seam ([shadcn-vue UI Kit](/openwiki/presentation/ui-kit.md)); the commit-message test pins the collaboration contract's parser.

## How tests import source

- `ui-utils.test.ts` imports `cn` from `../../app/lib/utils` (a `.ts` module).
- `validate-commit-message.test.ts` imports `validateCommitMessage` from `../../scripts/validate-commit-message.mjs` (an `.mjs` ESM module) — the validator exports a pure function specifically so it can be tested this way.

## Conventions

- **Fail-first (TDD)**: AGENTS.md requires writing a failing test for new functions and behavior before implementation; configuration files, pure directory notes, and generated files are exempt.
- **Behavior over implementation**: tests exercise public interfaces (e.g. use cases through `execute()`), never private details — the historical `get-home-page-content.test.ts`/`present-home-page.test.ts` were the models for this.
- Tests are *not* lint-ignored (the ESLint ignores in `eslint.config.mjs` cover only `app/components/ui/**`, `CardNav/**`, `SpecularButton/**`), so test files must pass `pnpm run lint:check`.

## Validation

- Full suite: `pnpm test`.
- Single file (focused): `pnpm test -- tests/unit/<file>.test.ts`.
- CI runs `pnpm test` after `lint:check` and `typecheck`; pre-push does **not** run tests (see [Git Hooks and CI](/openwiki/tooling/hooks-and-ci.md)).
