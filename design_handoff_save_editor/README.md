# Handoff: Dragon Age Save Editor — UI Redesign

## Overview

This is a redesign of the Dragon Age save editor's UI. It replaces the old topbar + 4-tab + 2-pane layout with a sidebar shell, a screenshot-prominent save identity card, a party-rail-driven character screen, a single-pane sortable inventory table with inline expand-to-edit, an ability browser (tree list + rank ladder), and DAO-imported plot-flag choice cards.

## About the design files

The files in this bundle are **design references created in HTML/CSS/vanilla JS** — prototypes that show the intended look, layout, and behavior. They are **not production code to copy directly**.

The task is to **recreate these designs in the existing target codebase** (React + TypeScript, Vite, the existing `useSaveEditorApp` / `useCharacterEditor` / `useInventoryEditor` hooks, the existing `api.ts` data layer). Match the visuals pixel-by-pixel, but route logic and data through the existing hooks and API; do not introduce new state-management patterns.

The CSS in `styles.css` + `editor.css` IS portable — it can be dropped into `src/styles/` largely as-is. The `app.js` render functions are scaffolding only — port their structure into React components.

## Fidelity

**High-fidelity.** Final colors, typography, spacing, and interactions are settled. Recreate pixel-perfectly. The only design decisions still open are listed in `INTEGRATION.md` § "Open decisions to confirm".

## Files in this bundle

- `Editor.html` — entry point; full app shell and section markup
- `styles.css` — design tokens (colors, fonts, radii, chips, buttons); foundation
- `editor.css` — section-specific styles (sidebar, party rail, inventory table, plot cards, tweaks panel)
- `app.js` — vanilla render functions; reference for component structure and event wiring
- `data.js` — mock party / abilities / items / plot decisions; reference for data shape
- `tweaks.js` — design-tool-only edit-mode panel; **DO NOT PORT** (drop entirely or expose select knobs as a real Settings menu)
- `INTEGRATION.md` — the canonical port spec: component-by-component mapping, hook changes, recommended PR order, file-by-file checklist

**Read `INTEGRATION.md` first.** This README is orientation; INTEGRATION.md is the build instructions.

## Screens

### 1. App Shell
- Two-column grid: 280px sidebar + flexible main area
- Sidebar contains save identity card, primary nav (Characters / Inventory / Recipes / Plot Flags), and bottom save actions (Open / Reset / Save As)
- Sidebar nav items: 0.55rem × 0.65rem padding, gold-accent active state, mono count badge on the right
- Main area has a section header with crumb + h1 + actions, and a body with 1.1rem × 1.5rem padding

### 2. Save Identity Card (sidebar)
- Screenshot is the dominant element — 16:9 aspect, scanline overlay, slot label bottom-right
- Character name in display font (`IM Fell English SC`), gold color
- File path in mono, ellipsis-truncated
- Dirty status chip + level mono-pill in a row beneath
- Hover/focus expands a popover preview to the right

### 3. Characters
- Sub-tabs: Overview / Abilities / Equipment
- Left party rail (220px): list of party members; each card has level badge, name, optional class line (tweakable), unsaved-changes pip
- Overview tab: progress card (level/xp/approval), attributes card (3-col grid), point pools card (4-col grid)
- Abilities tab: kind tabs (Skills/Talents/Spells), search input, points chip; below that a 220px tree list + rank ladder. Owned ranks have a gold border; locked ranks are dimmed and have a "Required" disabled button
- Equipment tab: same sortable table as Inventory but scoped to the selected character. **Save format has no slot info — flat list, no slot labels.**

### 4. Inventory
- Toolbar: search, category chips (All / Weapons / Armor / Consumable / Misc), item count
- Party gold pill in section header (gold pill with inline editable amount)
- Sortable table with columns: Item, Category, Tier (right), Stack (right), Cost (right), chevron
- Click a row → expands inline editor below it (no right pane). Editor has Material/Item level/Cost/Stack fields + properties chips with add/remove
- Active row has a gold left-edge accent bar

### 5. Plot Flags
- Blue-tinted DAO-import banner at top (these are the Warden's choices imported into DA2)
- Toolbar with search + era chips (Origin / Act 1 / Companions / World / Origins DLC)
- 2-column grid of choice cards. Each card: question text + era tag-line + PLT_id mono code, vertical list of options with radio markers, footer with option count + modified pip

## Design tokens

Defined in `styles.css` `:root`:

- **Colors:** `--ink`, `--ink-2/3/4` (parchment scale), `--bg`, `--bg-1/2` (deep blacks), `--gold`, `--gold-2`, `--blood`, `--blood-2`, `--rune` (faint blue), `--line`, `--line-strong`
- **Fonts:** `--font-display` (`IM Fell English SC` for headings), `--font-body` (`IM Fell English`), `--font-ui` (`Inter` for controls), `--font-mono` (`JetBrains Mono`)
- **Radii:** `--r` (8px cards), `--r-sm` (4px controls)
- **Chips:** `.chip` + variants `.gold` `.blood` `.rune`

Accent color is tweakable in the prototype; lock to `--gold-2: #c9a64a` for production unless theming is a goal.

## Interactions

- **Sub-tab switching:** instant, no transition
- **Party rail click:** swaps detail content; rail entry gets gold border; "dirty" pip lights blood-red when that character has unsaved edits
- **Numeric input dirty-state:** input border turns blood-red while value differs from initial; should clear on save
- **Inventory row expand:** only one row open at a time; click open row again to collapse
- **Plot option click:** sets `picked` index, marks `modified: true`, rerenders card

## State management

Reuse existing hooks:
- `useSaveEditorApp` — top-level save state, dirty flag, save/reset/open
- `useCharacterEditor` — per-character draft (stats, pools, abilities). Already has the actions needed for the abilities browser
- `useInventoryEditor` — item list, gold, item drafts. Used by both the Inventory tab and the Equipment tab

No new state shape. The only addition is per-section UI state (search, sort, expanded row, active sub-tab) which is local component state.

## Recommended order (from INTEGRATION.md)

1. Drop CSS into `src/styles/`, build `AppShell`
2. Plot Flags (smallest, easy warm-up)
3. Inventory table component (reused for Equipment tab)
4. Character overview + party rail
5. Abilities browser
6. Wire Equipment tab to reuse the inventory table
7. Delete `Topbar.tsx`, the old item editor right-pane, and slot-grid markup
8. Visual QA pass

## What to confirm with the design owner

Listed in `INTEGRATION.md` § "Open decisions to confirm". The big ones:
- Equipment tab as flat list (vs. legacy slot grid)
- Whether tweak knobs (compact rail, accent) become a Settings menu or are dropped entirely
- Recipes tab styling (left as a stub in the prototype)
