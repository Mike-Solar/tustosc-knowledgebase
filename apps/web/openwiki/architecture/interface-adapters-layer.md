---
type: concept
title: Interface-Adapters Layer
description: Contract and current state of the interface-adapters layer (controllers, mappers, presenters, view-models) in the Clean Architecture skeleton.
tags: [architecture, interface-adapters, clean-architecture]
---

# Interface-Adapters Layer

The `interface-adapters/` layer converts external input into use-case input and use-case output into the shape pages or APIs consume. It is the translation ring between the [Application layer](/openwiki/architecture/application-layer.md) and the [Nuxt convention directories](/openwiki/architecture/nuxt-conventions.md) (`app/`, `server/`).

## Current state

`interface-adapters/` is an **empty scaffold**: `controllers/`, `mappers/`, `presenters/`, and `view-models/` are all empty directories with no files. The historical example files (`controllers/home-page.ts`, `mappers/home-page.ts`, `presenters/home-page.ts`, `view-models/home-page.ts`) were removed in commit `301a6ff`.

## Intended responsibilities

Per [AGENTS.md](/AGENTS.md) and [docs/architecture/clean-architecture.md](/docs/architecture/clean-architecture.md):

- **`controllers/`** — coordinate an entry point (page or API route) with a use case: translate input (e.g. route params) and hand the result to a presenter.
- **`mappers/`** — field and format conversion between models (DTO to view model, external payload to domain input).
- **`presenters/`** — the output boundary; convert use-case results into view models.
- **`view-models/`** — stable data structures pages depend on.

## Core rule: pages do not format DTOs

Pages must not assemble dates, quantity copy, or permission state from a use-case DTO; that conversion belongs to mappers/presenters. Pages receive a `DocumentDetailViewModel`-style object and never see entity internals, database records, or third-party responses (AGENTS.md: "跨层数据必须通过 DTO、mapper 或 presenter 转换，不能把领域实体、数据库记录或第三方响应直接暴露给页面").

## Naming rule

Directory context already says `controllers`, `presenters`, etc., so filenames carry only the responsibility: `controllers/document-detail.ts`, `presenters/document-detail.ts`, `view-models/document-detail.ts` — never `document-detail-controller.ts` or `document-detail-presenter.ts`.

## Position in the call chain

In the canonical document-detail chain, the adapter layer sits between the composition root and the use case:

```text
infrastructure/nuxt/document-detail.ts  (creates controller)
  → interface-adapters/controllers/document-detail.ts  (execute)
  → application/use-cases/get-document-detail.ts  (returns DTO)
  → interface-adapters/presenters/document-detail.ts  (calls mapper)
  → interface-adapters/view-models/document-detail.ts  → app/components/document/*.vue
```

The full sequence, including the [Infrastructure](/openwiki/architecture/infrastructure-layer.md) and [Application](/openwiki/architecture/application-layer.md) participants, is drawn on the Infrastructure Layer page.

## Extension point

When adding a feature, the adapter step comes after the [Application layer](/openwiki/architecture/application-layer.md) use case exists and before the page is composed: create the controller (route-param conversion), mapper (DTO ↔ view model), presenter (output boundary), and view model (page contract). The step-by-step recipe is on the [Development Workflow](/openwiki/workflows/development-workflow.md) page.

## Focused validation

Adapter-layer behavior is validated by presenter/mapper unit tests (the historical `tests/unit/present-home-page.test.ts` was the model). Presenter tests construct a DTO, assert the resulting view model shape, and must fail before the presenter exists (TDD per AGENTS.md; see [Unit Testing](/openwiki/testing/unit-testing.md)).
