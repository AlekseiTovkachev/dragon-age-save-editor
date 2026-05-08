# Dragon Age Save Editor — UI Redesign Integration Spec

**Audience:** an AI coding agent (or human dev) integrating the redesigned UI from `Editor.html` into the existing React/TypeScript codebase under `src/`.

**Source of truth:** the prototype lives in this project at:
- `Editor.html` — markup + structure
- `styles.css` — design tokens (CSS variables, typography, primitives)
- `editor.css` — layout & component styles
- `app.js` — vanilla render logic (reference only — do NOT copy verbatim, port to React)
- `data.js` — mock data (do NOT port — real data comes from `api.ts`)
- `tweaks.js` — design-tool only, **drop in production port**

**Codebase to modify:** the React app under `src/`, entry `src/App.tsx`, state hook `src/features/app/useSaveEditorApp.ts`, panel components under `src/features/<area>/`. Do not change `src/api.ts`, `src/types.ts`, or any `*.test.*` expectations beyond what is strictly needed; the data layer and tests are stable.

---

## 1. Design tokens & global styles

Copy the CSS variables, typography rules, and button/input/chip primitives from `styles.css` and `editor.css` into `src/styles/`. Keep them split:

```
src/styles/
  tokens.css      ← :root vars from styles.css (palette, fonts, radii, shadows)
  base.css        ← typography, body, scrollbars (already exists; merge)
  shell.css       ← .app, .app-sidebar, .app-main, .id-card, .nav-item
  characters.css  ← .party-rail, .party-card, .char-header, .char-tab, etc.
  abilities.css   ← .abilities-layout, .tree-list, .ranks-panel, .rank-row
  inventory.css   ← .inv-toolbar, .inv-table, .inv-thead, .inv-row, .inv-expand
  plot-flags.css  ← .dao-banner, .plot-grid, .plot-card, .plot-opt
  forms.css       ← .num, .inp, .search-input, .chip, .btn (merge with existing)
  tweaks.css      ← OMIT in production (panel is design-tool only)
```

Import order in `src/main.tsx` or `src/styles.css`:
1. `tokens.css`
2. `base.css`
3. component css files

**Key tokens to preserve** (DAO palette, DA2 chrome):
- `--gold-2` (accent — gold #c9a64a) — used by chips, dirty borders, active states
- `--ink`, `--ink-2`, `--ink-3`, `--ink-4` — text scale (high → low contrast)
- `--surface`, `--surface-2`, `--surface-strong`, `--line` — backgrounds and borders
- `--font-display` (IM Fell English SC), `--font-ui` (Inter), `--font-mono` (JetBrains Mono)

Google Fonts already linked in `Editor.html` head; mirror in `index.html`.

---

## 2. Component port — file-by-file

For each redesigned area, this section gives:
- **Replaces:** the existing files/components being supplanted
- **New components:** what to create
- **Data wiring:** which existing hook / state slice feeds the new UI
- **Markup reference:** which DOM block in `Editor.html` to mirror
- **Logic reference:** which function in `app.js` to translate to React

### 2.1 App shell (sidebar nav)

**Replaces:** the top-of-page layout in `src/App.tsx` and the entire `src/components/Topbar.tsx` + `src/components/MainTabs.tsx`.

**New components:**
- `src/components/AppShell.tsx` — outer `<div class="app">` with sidebar slot + main slot
- `src/components/Sidebar/Sidebar.tsx` — full sidebar: identity card + nav + save actions
- `src/components/Sidebar/SaveIdentityCard.tsx` — screenshot card with hover popover, character name, path, dirty chip
- `src/components/Sidebar/NavList.tsx` — section nav (Characters / Inventory / Recipes / Plot Flags) with counts
- `src/components/Sidebar/SaveActions.tsx` — Open / Reset Drafts / Save As buttons (was the toolbar)

**Data wiring:** `useSaveEditorApp()` already exposes everything needed:
- `app.summary` → `main_character_name`, `source_path`, `preferred_game`, `dirty`
- `app.screenshotDataUrl` → identity card image
- `app.handleOpen`, `app.handleSaveAs`, `app.commitDrafts`, `app.resetToCommittedDrafts` → save actions
- `app.visibleSections`, `app.section`, `app.setSection` → nav state
- `app.operation.busy` → disable buttons

**Counts in nav (e.g. "42" next to Inventory):** add to `useSaveEditorApp` return value:
```ts
const sectionCounts = {
  characters: app.characterPanel.state.characters.length,
  inventory: app.inventoryPanel.state.items.length,
  recipes: app.craftingPanel.state.craftingRecipeDrafts.length,
  plot_flags: app.plotFlagsPanel.state.groupedPlotIntegers.reduce((n, g) => n + g.flags.length, 0),
};
```

**Markup reference:** `Editor.html` lines for `<aside class="app-sidebar">` block.

**Notes:**
- The screenshot popover (hover/focus to enlarge) is pure CSS — see `.id-card`, `.shot-popover` in `editor.css`. Keep that interaction.
- Active nav item uses `is-active` class. Use `aria-current="page"` for a11y.
- Dirty chip on the identity card: `summary.dirty ? "Unsaved changes" : "Saved copy ready"` — already present in old `Topbar.tsx`.

---

### 2.2 Characters — header + party rail + overview

**Replaces:** the entire left/right split in `src/features/characters/CharacterPanel.tsx`. Keep the file but rewrite its top-level render. The `CharacterOverview` and `CharacterAbilities` inner components also change significantly.

**New components:**
- `src/features/characters/PartyRail.tsx` — vertical list of party members with level badge + dirty pip. **No portraits, no class info by default** (per design decision).
- `src/features/characters/CharacterHeader.tsx` — name, class line, chips (Level / XP / unspent points)
- `src/features/characters/CharacterSubtabs.tsx` — Overview / Abilities / Equipment switcher
- Keep `CharacterOverview` as a sub-component but restyle it with the new `.card-2` + `.grid-progress` / `.grid-attrs` / `.grid-pools` markup.

**Data wiring:** unchanged — `CharacterPanelState` and `CharacterPanelActions` from `useCharacterEditor` already cover all draft fields. The dirty-per-field logic is **new** — see §6.

**Markup reference:** `Editor.html` `<section data-section="characters">` block (overview tab inside it).

**Per-field dirty visual:** the input gets a red bottom border (`.inp.dirty`) only when its current value differs from its committed value. Track this per field; do **not** apply on initial load just because the character has any pending change.

```tsx
// inside CharacterOverview field:
<NumericInput
  className={value !== committedValue ? "inp dirty" : "inp"}
  ...
/>
```

---

### 2.3 Characters — Abilities browser

**Replaces:** `CharacterAbilities` and `AbilityGroup` in `src/features/characters/CharacterPanel.tsx` (lines ~210–408). This is the **biggest single rewrite**.

**Old shape:** 3 cramped columns (Skills / Talents / Spells), each with a dropdown + grouped expandable list.

**New shape:** kind tabs at top (Skills | Talents | Spells), one column for trees on the left, ranks panel on the right.

**New components:**
- `src/features/characters/abilities/AbilitiesPanel.tsx` — top-level layout with kind tabs + search + points chip
- `src/features/characters/abilities/KindTabs.tsx` — Skills / Talents / Spells switcher
- `src/features/characters/abilities/TreeList.tsx` — left column, one row per tree, shows `owned/total` rank pip
- `src/features/characters/abilities/RankLadder.tsx` — right column, list of ranks for selected tree, with Add/Remove buttons

**Data shape mapping (real data → prototype):**
- The prototype's `ABILITY_TREES.skills[].ranks[]` is fake. In your real data, **a "tree" is a group of related abilities** — your existing `groupedAbilities()` in `lib/abilityUtils.ts` already does this grouping. Each ability in a group is a "rank" (ordinal within the tree).
- "Owned" = ability is in `state.abilityDrafts[kind]`
- "Locked" = `actions.abilityIsLocked(kind, abilityId)` — required by another selected ability
- For each tree, `total` = `group.abilities.length`, `owned` = count where `selected === true`

**Logic reference:** `app.js` `renderAbilities()` — the structure (tree list highlights selected tree, rank ladder shows ranks for selected tree, click to toggle) is correct. Translate the toggle handlers to call `actions.handleVisibleAbilityAdd` / `actions.handleAbilityRemove`.

**Search:** filter `treeAbilities` by name match across both tree label and rank names — same as prototype's `filtered = trees.filter(...)`.

**Points chip:** `c.point_pools.skill_points` / `talent_points` / `specialization_points` — show "0" when null.

---

### 2.4 Inventory — sortable table with inline expand

**Replaces:**
- `src/features/inventory/InventoryPanel.tsx` (full rewrite of layout — no more two-pane)
- `src/components/ItemList.tsx` — delete or repurpose
- `src/components/ItemEditor.tsx` — keep but restyle to fit inline-expand block

**New components:**
- `src/features/inventory/InventoryTable.tsx` — the toolbar + table + tbody with inline expand. **Reusable** — used by both the Inventory tab and the character Equipment tab.
- `src/features/inventory/InventoryToolbar.tsx` — search + category filter chips + count
- `src/features/inventory/InventoryRow.tsx` — single `<button class="inv-row">` row
- `src/features/inventory/InlineItemEditor.tsx` — the `.inv-expand` block (overview fields + properties chips). This **wraps `ItemEditor.tsx`** — keep the existing form-control logic, just re-style the wrapper.

**Sortable header:** click toggles ascending/descending. State: `{ sort: 'name' | 'cat' | 'tier' | 'stack' | 'cost', dir: 1 | -1 }`. Lift to parent so it persists across re-renders.

**Inline expand:** clicking a row toggles `expandedResref`. Only one row expanded at a time. The expand block sits **between rows** in the DOM (as a sibling, not a child) — see prototype.

**Click handling:** clicking inside the `.inv-expand` block must NOT collapse the row. In React: `onClick={(e) => { if (e.target.closest('.inv-expand')) return; toggleExpand(item); }}` or stop propagation on the expand block.

**Data wiring:**
- `state.items` (already `IndexedItem[]`)
- `state.itemIndex` → replace with `expandedIndex` (same idea, just renamed for clarity)
- All existing actions on `useInventoryEditor` (clone, remove, property add/remove/update, metadata patch) hook into the inline editor unchanged.

**Category chips:** the prototype has `["All", "Weapons", "Armor", "Consumable", "Quest"]`. Map to your `ItemCategory.value`/`label` enum — derive the chip list from the unique categories present in `state.items` plus an "All" prepended.

**Party gold (Money):** moves from the tab strip into the Inventory section header (`.head-actions .gold-pill`). Wire to `inventoryPanel.actions.setMoneyDraft` and `state.moneyDraft`.

---

### 2.5 Characters — Equipment tab

**Replaces:** `character-equipment-layout` block in `CharacterPanel.tsx` (the two-pane ItemList + ItemEditor).

**New:** reuse `<InventoryTable>` from §2.4 with:
- Items source: equipment list for the current character (your `state.items` already returns equipment when the active container is `equipment`)
- `allowRemove={false}`, `allowClone={false}` on the inline editor (you can't remove from equipment)
- No party gold pill, no category filter (just search)
- A help line above: "Items currently carried by this character. The save format doesn't track equipment slots, so items are listed flat — click any row to edit material, properties, and metadata."

**Data wiring:** the existing `inventoryPanel` already swaps containers when the character changes — confirm that container=equipment is set when the equipment subtab is active. If not, add a side effect in `useSaveEditorApp` or `useInventoryEditor` that sets the container based on `characterTab`.

---

### 2.6 Plot Flags — choice cards with DAO banner

**Replaces:** `src/features/plotFlags/PlotFlagsPanel.tsx` (the two-column fieldset wall).

**New components:**
- `src/features/plotFlags/PlotFlagsPanel.tsx` (rewritten) — top: DAO banner + toolbar (search + era chips), then `.plot-grid` of cards
- `src/features/plotFlags/PlotChoiceCard.tsx` — one card per integer flag; question text + radio-style options + "modified" pip in footer
- `src/features/plotFlags/PlotBooleanCard.tsx` — for boolean flags, render as a card with a single yes/no toggle styled the same way

**DAO banner copy** (preserve verbatim; this addresses a real user confusion):
> **These are imported from DAO.** DA2 reads the Warden's choices from your DAO save — edit them here to change how the world remembers you. Hawke's own decisions stay in DA2 dialogue/quest data, not these flags.

**Era filter:** group flags by category (existing `state.groupedPlotIntegers[].category`). Render a chip strip: `["All", ...uniqueCategories]`.

**Modified count:** count drafts where `plotIntegerDrafts[id]` differs from the original or `plotBooleanDrafts[id]` differs.

**Data wiring:** unchanged — `usePlotFlagsEditor` returns the right shape. Only render when `summary.preferred_game === "da2"` (existing guard).

---

### 2.7 Recipes

**Status:** prototype left this as a stub. Port pattern from Plot Flags: same toolbar + sectioned card pattern, checkbox per recipe within a category card. Keep `useCraftingEditor` unchanged.

---

## 3. State / hook changes

Most of the redesign is **presentational**. Existing hooks (`useSaveEditorApp`, `useCharacterEditor`, `useInventoryEditor`, `usePlotFlagsEditor`, `useCraftingEditor`) keep their shape. Three small additions:

1. **Per-field dirty tracking** (Character Overview): each draft field needs to know its committed counterpart so the input can flag itself dirty independently. Add a helper:
   ```ts
   // src/lib/dirty.ts
   export const isDirty = (draft: string, committed: number | null) =>
     draft !== String(committed ?? "");
   ```

2. **Section counts** for the sidebar nav (see §2.1).

3. **Inline expand state** in InventoryPanel — local component state, doesn't need to live in the hook.

---

## 4. Removed / deprecated

After the port, these can be deleted:
- `src/components/Topbar.tsx` (replaced by Sidebar)
- `src/components/MainTabs.tsx` (replaced by NavList in Sidebar; money input moves to Inventory header)
- `src/components/ItemList.tsx` (replaced by InventoryTable rows)
- The `topbar`, `topbar-*`, `nav-link`, `main-tabbar`, `tab-money-control` rules in `src/styles/layout.css` and `src/styles/tabs.css`

Keep:
- `src/components/ui.tsx` primitives — they're still useful, just used inside the new wrappers
- `src/components/ItemEditor.tsx` — wrap inside `<InlineItemEditor>` rather than rewriting

---

## 5. Tests

The redesign should not break behavioral tests. Verify:
- `useCharacterEditor.test.tsx`, `useInventoryEditor.test.tsx`, `usePlotFlagsEditor.test.tsx` — pass unchanged
- `ItemList.test.tsx`, `MainTabs.test.tsx`, `Topbar.test.tsx` — **delete** (components removed)
- `ItemEditor.test.tsx` — keep, but if it asserts on outer wrapper class names, relax to query by role/text
- Add at least one render test for `InventoryTable` (rows render, click expands, sort header toggles dir)

---

## 6. Recommended PR order

1. **Tokens + global CSS** — drop in vars, fonts, base styles. App still works, just looks slightly different.
2. **Sidebar shell** — replace Topbar + MainTabs with Sidebar. Removes the visible topbar.
3. **Plot Flags** — smallest panel, easy port, lets you exercise the new card+chip patterns.
4. **Inventory table** — extract `<InventoryTable>` reusable. Remove two-pane.
5. **Equipment tab** — drop in `<InventoryTable>` with `allowRemove={false}`.
6. **Character overview + party rail** — restyle, add per-field dirty.
7. **Abilities browser** — biggest rewrite, save for last.
8. **Recipes polish** — apply the Plot Flags pattern.
9. **Cleanup** — delete dead components, prune CSS, update tests.

Each PR should keep the app fully functional. No flag-day swap.

---

## 7. Open decisions to confirm before integrating

- [ ] **Equipment as flat list (no slots)** — confirm this is the intended direction. Some users may expect slot labels. The prototype reflects "we don't actually know which slot."
- [ ] **Tweaks panel disposition** — drop entirely, or convert a subset (compact rail, accent color) to a real Settings menu in the sidebar?
- [ ] **Section counts in nav** — show always, or only when > 0?
- [ ] **Character class line** — currently hidden in the party rail (was a tweak). Confirm permanently hidden, or surface only on the detail header?
- [ ] **Dark-only theme** — the redesign is dark. Is light mode a future concern, or out of scope?

---

## 8. Files in this project (handoff checklist)

| File | Role | Port? |
|---|---|---|
| `Editor.html` | Reference markup | Reference only |
| `styles.css` | Tokens + base | Yes — split into `src/styles/tokens.css` etc. |
| `editor.css` | Component styles | Yes — split per component |
| `app.js` | Render logic | Translate to React, do not copy |
| `data.js` | Mock data | Discard — use real `api.ts` |
| `tweaks.js` | Design-tool only | Discard |
| `Wireframes.html` | Earlier exploration | Reference only |
| `tabs/*` | Wireframe partials | Reference only |
