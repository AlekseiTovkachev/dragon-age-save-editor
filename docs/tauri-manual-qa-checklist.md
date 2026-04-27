# Tauri Manual QA Checklist

Use copied saves only. Keep the original `.das`, `.das.met`, and `screen.dds` files untouched until the edited copy has been checked in game.

Run the desktop app:

```powershell
npm run tauri dev
```

## Startup And Shell

- [x] App window opens centered and fully visible, with no clipped bottom edge.
- [x] Header shows app title, save filename/path when loaded, game label, dirty/saved state, and screenshot/no-screenshot state.
- [x] Horizontal tabs are visible and usable.
- [x] Money display sits beside the tabs and stays compact.
- [x] `Open Save`, `Commit Changes`, `Reset Drafts`, and `Save As` are visible in the header after loading a save.
- [x] `Save As` is disabled before any committed backend edit makes the save dirty.
- [x] Error/warning banners are readable and dismissible if triggered.

## DAO Save

- [x] Open a vanilla DAO save.
- [x] `Characters`, `Inventory`, and `Recipes` tabs are visible.
- [x] `Plot Flags` tab is hidden.
- [x] Character overview fits without page-level scrolling at the intended Tauri window size.
- [x] Main character does not show an approval field.
- [x] Companion characters show approval when available.
- [x] Progress, attributes, and point pools fields are readable and not clipped.
- [x] Ability trees are collapsed by default or compact enough to scan.
- [x] Ability tree groups expand and collapse correctly.
- [x] Add/remove ability controls preserve prerequisite locking.
- [x] Equipment item list and editor render correctly.
- [x] Inventory item list shows only item name and amount when `item_stacksize > 1`.
- [x] Inventory item row text is vertically centered.
- [x] Money panel is compact and does not dominate the tab/header row.
- [x] Item overview and properties columns align at the top.
- [x] Item overview does not show duplicate name or stackable fields.
- [x] Property dropdown uses the dark DAO styling, not a white background.
- [x] Property table columns remain aligned while editing IDs and powers.
- [x] If an item has many properties, only the property list scrolls; overview and add-property controls stay usable.
- [x] Recipes render in compact columns at desktop width.
- [x] Recipe rows are readable and toggle correctly.

## DA2 Save

- [x] Open a DA2 save.
- [x] `Plot Flags` tab is visible.
- [x] DA2 abilities load into the expected combined talent/spell behavior.
- [x] DA2 item property IDs and powers display correctly.
- [ ] Editing DA2 item property powers preserves float/bitcast behavior after commit and reload.
- [x] DA2 crafting recipes load and toggle correctly.
- [x] Plot flag integer groups render readable radio options.
- [x] Plot flag boolean groups render readable checkbox rows.
- [x] Plot flag changes can be drafted, committed, saved, and reloaded.

## Commit And Reset Model

- [ ] Load a save and note the initial money, stack size, character stat, recipe, and plot flag values.
- [ ] Change money locally, then click `Reset Drafts`; money returns to the loaded value and no backend edit is sent.
- [ ] Change a stack size locally, then click `Reset Drafts`; stack size returns to the loaded value.
- [ ] Change a character stat locally, then click `Reset Drafts`; stat returns to the loaded value.
- [ ] Change a recipe locally, then click `Reset Drafts`; recipe returns to the loaded value.
- [ ] For DA2, change a plot flag locally, then click `Reset Drafts`; plot flag returns to the loaded value.
- [ ] Change money and stack size, then click `Commit Changes`; save state becomes dirty and `Save As` enables.
- [ ] After commit, change the same fields again, then click `Reset Drafts`; fields return to the committed values, not the original loaded values.
- [ ] Commit fails cleanly for an invalid edit and leaves prior committed state intact.

## Save As And Reload

- [ ] After a successful commit, click `Save As` and write to a copy path.
- [ ] `Save As` completion marks the summary as saved/clean.
- [ ] Reopen the saved copy.
- [ ] DAO committed character, inventory, and recipe edits persist after reload.
- [ ] DA2 committed property, recipe, and plot flag edits persist after reload.
- [ ] Original save files remain unchanged.

## Responsive Safeguards

- [ ] Resize or run below fullscreen width if possible.
- [ ] Header buttons remain readable and do not overlap.
- [ ] Cards keep enough space for labels and values.
- [ ] Backpack/inventory rows remain vertically centered.
- [ ] Item editor avoids whole-panel scrolling except where content naturally overflows.
