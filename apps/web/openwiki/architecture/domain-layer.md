---
type: concept
title: Domain Layer
description: Contract and current state of the root domain layer (entities, value-objects, services, repositories, errors) in the Clean Architecture skeleton.
tags: [architecture, domain, clean-architecture]
---

# Domain Layer

The `domain/` layer is the innermost Clean Architecture ring. It answers what the knowledge-base business *is* and which rules must always hold. It must be framework-free and testable without Nuxt or a browser.

## Current state

`domain/` is an **empty scaffold**. It contains three README-only directories and two empty directories:

| Directory | Contents | Status |
| --- | --- | --- |
| `domain/entities/` | (none) | Empty — no entity files |
| `domain/value-objects/` | `README.md` | Placeholder only |
| `domain/services/` | `README.md` | Placeholder only |
| `domain/repositories/` | (none) | Empty — no repository abstractions |
| `domain/errors/` | `README.md` | Placeholder only |

The historical example entity `domain/entities/home-content.ts` (with its repository abstraction `domain/repositories/home-content.ts`) was removed in commit `301a6ff` when the home-page example chain was deleted, so the layer is again a pure contract.

## Intended responsibilities

Per [AGENTS.md](/AGENTS.md) and [docs/architecture/clean-architecture.md](/docs/architecture/clean-architecture.md):

- **`entities/`** — things with identity and lifecycle, e.g. knowledge documents, knowledge spaces.
- **`value-objects/`** — immutable concepts expressed through value equality that encapsulate domain constraints, e.g. document path, version number, member role. The README requires: value objects must not depend on frameworks or infrastructure.
- **`services/`** — domain rules that span multiple entities and do not naturally belong to a single entity.
- **`repositories/`** — declare only the *capabilities* the domain needs (e.g. `read` a document detail). No database, HTTP, or ORM details.
- **`errors/`** — domain error types recognizable by upper layers; the README forbids exposing raw HTTP or database errors here.

## Hard constraints

- `domain` must not import Vue, Nuxt, Pinia, Milkdown, HTTP clients, or database drivers (AGENTS.md dependency rules; enforced by review, not by lint).
- Files in `domain/` must not leak infrastructure concepts; a database record is never a domain object.
- The layer owns the vocabulary other layers use: entities, value objects, domain services, and repository interfaces.

## Naming rule

Directories already express context, so filenames express only responsibility — `domain/repositories/document.ts`, never `document-repository.ts`. Extra qualifiers are allowed only to disambiguate protocols, third-party implementations, or runtime differences (canonical examples from AGENTS.md: `persistence/in-memory-document.ts`, `markdown/milkdown.ts`, `http/client.ts`).

## Scaffold READMEs

The README files in empty directories (e.g. `domain/errors/README.md`) exist solely so Git can track the directories (per docs/plans/2026-08-14-knowledgebase-skeleton.md) and are the exception to the "no filler READMEs" rule — they must never be replaced by business logic.

## Extension point

The first real feature should define its entity and value objects here first, then declare its repository capability in `domain/repositories/`, before the [Application layer](/openwiki/architecture/application-layer.md) use case is written. The complete recipe is on the [Development Workflow](/openwiki/workflows/development-workflow.md) page.

## Focused validation

<!-- openwiki: broken internal link [vue|nuxt|@milkdown|pinia] file "vue|nuxt|@milkdown|pinia" does not exist. Fix the href or restore the target, then delete this comment. -->
There are currently no domain tests because there is no domain code. When a new domain behavior is added, AGENTS.md requires writing a failing test first (Vitest; see [Unit Testing](/openwiki/testing/unit-testing.md)), and a dependency-direction grep is the project's own check: `rg -n "from ['\"](vue|nuxt|@milkdown|pinia)" domain application` must return nothing.
