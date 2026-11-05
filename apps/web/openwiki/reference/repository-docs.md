---
type: concept
title: Repository Documentation Library
description: Map of the docs directory (architecture, guides, adr, specs, plans), the roles of README, AGENTS.md, CONTRIBUTING.md, and LICENSE, and the historical-note convention.
tags: [reference, documentation, governance]
---

# Repository Documentation Library

The repository's long-lived documentation is split between root governance files and the `docs/` tree. These files are the authoritative intent record for the Clean Architecture skeleton and the collaboration contract.

## Root governance files

| File | Role |
| --- | --- |
| `README.md` | Project positioning (Yuque-like knowledge base for the TUST Open Atom Open Source Association), tech stack, quick start, command table, commit type table, structure overview; license section pointing to `LICENSE` |
| `AGENTS.md` | The stable, executable collaboration contract: directory responsibilities, dependency direction, naming rules, code style, Markdown/editor (Milkdown) policy, doc governance, validation commands, hooks/CI rules, commit message rules. Also carries the OpenWiki section (generated wiki is just-in-time context; regenerate rather than hand-edit) |
| `CONTRIBUTING.md` | Contributor-facing flow: fork → branch → pnpm install → dev → commit rules → PR |
| `LICENSE` | AGPL-3.0 (34 KB, standard text); referenced by README's License section. Static file — no per-page change surface beyond version bumps |
| `CLAUDE.md` | Thin pointer to AGENTS.md for Claude Code agents (OpenWiki marker block) |

## docs/ tree

| Path | Contents |
| --- | --- |
| `docs/architecture/clean-architecture.md` | Newcomer explainer: why four layers, layer responsibilities, the canonical document-detail call chain, where new features land, common mistakes |
| `docs/architecture/directory-map.md` | Full directory tree with one-line responsibilities and the filename naming rule |
| `docs/guides/development-workflow.md` | Daily workflow: install/start, page entrypoints, modification flow, verification layers, commit message rules, doc-update triggers, cannot-do list |
| `docs/adr/0001-clean-architecture.md` | ADR-0001 (accepted 2026-08-14): adopting the four-layer Clean Architecture; rejected alternatives (all-in-`app/`, per-module four-layer copies); consequences and the naming constraint |
| `docs/specs/2026-08-14-knowledgebase-architecture-design.md` | Skeleton-phase design spec: goals, fixed boundaries, option analysis, directory responsibilities, dependency direction, example chain, toolchain rules |
| `docs/plans/2026-08-14-knowledgebase-skeleton.md` | The staged implementation plan (Tasks 1–5 with verification steps and atomic-commit guidance); kept as historical record |

## Historical-note convention

`docs/specs/` and `docs/plans/` files that predate the home-page example removal carry a "历史设计说明/历史计划说明" banner at the top explaining that the example file paths no longer match source. Treat file paths in specs/plans as historical references; the wiki's [Architecture](/openwiki/architecture/overview.md) pages and `docs/architecture/` describe the current state.

## When to update which doc

Per docs/guides/development-workflow.md: directory boundaries/dependency direction → `docs/architecture/`; newcomer steps → `docs/guides/`; important trade-offs → `docs/adr/`; stable collaboration rules → `AGENTS.md`; entrypoints/commands → `README.md`. Code behavior, commands, public flows, or architecture changes must update the corresponding docs in the same change (AGENTS.md).

## OpenWiki integration

- `AGENTS.md` and `CLAUDE.md` carry `OPENWIKI:START`/`OPENWIKI:END` marker blocks; the scheduled `.github/workflows/openwiki-update.yml` regenerates the `openwiki/` index and, via `add-paths`, also updates those markers and the workflow itself (see [Git Hooks and CI](/openwiki/tooling/hooks-and-ci.md)).
- Wiki diagrams and regeneration behavior are governed by the skills documented on [OpenWiki Skills Library](/openwiki/reference/openwiki-skills.md).
