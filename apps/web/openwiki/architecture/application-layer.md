---
type: concept
title: Application Layer
description: Contract and current state of the root application layer (dto, ports, use-cases, errors) in the Clean Architecture skeleton.
tags: [architecture, application, clean-architecture]
---

# Application Layer

The `application/` layer sits between `domain/` and `interface-adapters/`. It answers what business goal a user is trying to accomplish: each use case orchestrates domain objects and abstract ports, then returns a DTO. It must work identically whether driven by a page, a server API, or a script.

## Current state

`application/` is an **empty scaffold**. Only `application/errors/README.md` exists; `dto/`, `ports/`, and `use-cases/` are empty directories.

The historical example use case `application/use-cases/get-home-page-content.ts` (plus `dto/home-page.ts` and `ports/home-page.ts`) was removed in commit `301a6ff` alongside the rest of the home-page example chain.

## Intended responsibilities

Per [AGENTS.md](/AGENTS.md) and [docs/architecture/clean-architecture.md](/docs/architecture/clean-architecture.md):

- **`dto/`** — data structures at the use-case boundary (use-case input and output).
- **`ports/`** — input ports and abstractions of external capabilities the use cases need.
- **`use-cases/`** — one business goal per use case, e.g. `get-document-detail.ts`. A use case calls domain objects and ports, then outputs a DTO.
- **`errors/`** — application-boundary errors for use-case orchestration; the README forbids passing third-party exceptions straight through to pages.

## Hard constraints

- `application` may depend on `domain` only, plus its own DTO/port/error types. It must not import Vue components, routes, browser APIs, `useFetch`, database clients, or Milkdown (AGENTS.md).
- A use case must not return database records — database records belong to `infrastructure`. The use case converts domain objects into DTOs.
- Ports are interfaces or types; their concrete implementations live in `infrastructure/`.

## Naming rule

`application/use-cases/get-document-detail.ts` expresses the action; directory context (`use-cases`) is not repeated in filenames. Use-case names are verbs.

## Extension point

The canonical example in [clean-architecture.md](/docs/architecture/clean-architecture.md) — "document detail" — creates `application/use-cases/get-document-detail.ts` after the [Domain layer](/openwiki/architecture/domain-layer.md) declares the repository abstraction, and before the [Interface-Adapters layer](/openwiki/architecture/interface-adapters-layer.md) controller is written. The full chain and recipe are documented on the [Development Workflow](/openwiki/workflows/development-workflow.md) page.

## Focused validation

<!-- openwiki: broken internal link [vue|nuxt|@milkdown|pinia] file "vue|nuxt|@milkdown|pinia" does not exist. Fix the href or restore the target, then delete this comment. -->
No application-layer tests exist yet because the layer is empty. New use cases are tested through their public `execute()` interface with real (or in-memory) dependencies, per the TDD convention in AGENTS.md; see [Unit Testing](/openwiki/testing/unit-testing.md). The dependency-direction grep `rg -n "from ['\"](vue|nuxt|@milkdown|pinia)" domain application` must stay empty.
