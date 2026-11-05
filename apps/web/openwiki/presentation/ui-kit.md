---
type: concept
title: shadcn-vue UI Kit
description: The generated shadcn-vue reka-nova UI kit (button, input, separator, sheet, sidebar, skeleton, tooltip), the cn() class-merge boundary, and its current usage status.
tags: [presentation, ui, shadcn, reka-ui]
---

# shadcn-vue UI Kit

`app/components/ui/` is a shadcn-vue kit generated in the **reka-nova** style (per `components.json`: `"style": "reka-nova"`, font `geist-sans`, icon library `lucide`, base color `stone`, TypeScript on). The kit wraps `reka-ui` primitives with Tailwind v4 classes and the project's `cn()` class-merge boundary. It is upstream-maintained generated code: ESLint ignores `app/components/ui/**` (see [Project Toolchain](/openwiki/tooling/project-toolchain.md)), and AGENTS.md treats it as a vendor surface the project only consumes.

## Component families

| Family | Entry | Notes |
| --- | --- | --- |
| Button | `ui/button/Button.vue` + `index.ts` | cva `buttonVariants` (variants: default/outline/secondary/ghost/destructive/link; sizes: default/xs/sm/lg/icon/icon-xs/icon-sm/icon-lg; cva defaults variant `default`, size `default`). Renders via reka-ui `Primitive` (`as` defaults to `button`) with `data-slot="button"` |
| Input | `ui/input/Input.vue` | `useVModel(props, 'modelValue', emits, { passive: true, defaultValue: props.defaultValue })` over a native input with `data-slot="input"`, emitting `update:modelValue` |
| Separator | `ui/separator/Separator.vue` | reka-ui `Separator`, `reactiveOmit` for class delegation, defaults `orientation: 'horizontal'`, `decorative: true` |
| Sheet | `ui/sheet/*` | Dialog-style overlay family (Trigger/Content/Header/Footer/Title/Description/Overlay/Close). `Sheet.vue` forwards `DialogRootProps`/`DialogRootEmits` via `useForwardPropsEmits`; `SheetContent` defaults `side: 'right'`, `showCloseButton: true` and closes with a `Button variant="ghost" size="icon-sm"` containing `XIcon`; `SheetOverlay` uses `bg-black/10 supports-backdrop-filter:backdrop-blur-xs`. Used by the Sidebar for mobile |
| Sidebar | `ui/sidebar/*` | 20+ components (Provider, Content, Header, Footer, Group*, Menu*, Inset, Rail, Trigger, Separator, Input, Skeleton) plus `utils.ts` |
| Skeleton | `ui/skeleton/Skeleton.vue` | `bg-muted rounded-md animate-pulse` div |
| Tooltip | `ui/tooltip/Tooltip*.vue` | reka-ui `TooltipRoot` with `useForwardPropsEmits` |

## Sidebar context

`ui/sidebar/utils.ts` defines the shared sidebar state through reka-ui's `createContext` (`useSidebar`/`provideSidebarContext`) with `state` (expanded/collapsed), `open`, `isMobile`, `openMobile`, and `toggleSidebar`. Constants: `SIDEBAR_COOKIE_NAME = 'sidebar_state'`, `SIDEBAR_COOKIE_MAX_AGE` (7 days), `SIDEBAR_WIDTH = '16rem'`, `SIDEBAR_WIDTH_MOBILE = '18rem'`, `SIDEBAR_WIDTH_ICON = '3rem'`, `SIDEBAR_KEYBOARD_SHORTCUT = 'b'`. `SidebarProvider` derives `defaultOpen` from the cookie not containing `sidebar_state=false`, detects mobile via `useMediaQuery('(max-width: 768px)')`, listens for Ctrl/Cmd+B keydown to toggle, and wraps children in `TooltipProvider :delay-duration="0"`. `Sidebar` defaults `side: 'left'`, `variant: 'sidebar'`, `collapsible: 'offcanvas'` and renders inside a mobile `Sheet` on small screens; `SidebarMenuButton` shows its `TooltipContent` only when `state === 'collapsed' && !isMobile`; `SidebarMenuSkeleton` randomizes placeholder text width as `Math.floor(Math.random() * 40) + 50`%.

## The cn() boundary

`app/lib/utils.ts` exports `cn(...inputs: ClassValue[])` = `twMerge(clsx(inputs))` — the single class-merge seam all UI components use (Tailwind conflict resolution + conditional classes). Its behavior is covered by the project's only utility test, `tests/unit/ui-utils.test.ts` (asserts `cn('px-2', undefined, 'px-4', 'text-foreground') === 'px-4 text-foreground'`). `components.json` maps `utils` to `~/lib/utils` and `ui` to `~/components/ui`.

## Current usage status

The kit is **unused by any page or layout today**: grep of `app/` shows only internal ui→ui imports (e.g. `Sidebar.vue` importing `Sheet`/`SheetContent`), no page imports. The kit was scaffolded for future knowledge-space UI (sidebar navigation, sheets, inputs, tooltips) and is ready to consume via auto-imported components (Nuxt auto-imports `~/components`, per `nuxt.config.ts` `components.dirs`).

## Extension point

- Consume any kit component by name in a Vue template (Nuxt auto-import), e.g. `<UButton>` is not used — the kit exports are `<Button>`, `<Input>`, `<Sidebar>`, etc.
- Regenerate/extend the kit with the shadcn-vue CLI using `components.json` settings; the generated files stay under `app/components/ui/` and remain lint-ignored.
- The `buttonVariants` cva object is also exported from `ui/button/index.ts` for programmatic variant use.

## Focused validation

The narrowest check for kit wiring is `pnpm test` (the `cn()` test proves the class-merge seam) plus `pnpm typecheck`. There are no component tests for the kit itself (upstream-maintained). See [Unit Testing](/openwiki/testing/unit-testing.md) and [Theming](/openwiki/presentation/theming.md) for the CSS variables the kit's utility classes depend on.
