# Tauri Manual QA Checklist

Use copied saves only. Keep the original `.das`, `.das.met`, and `screen.dds` files untouched until the edited copy has been checked in game.

Run the desktop app:

```powershell
npm run tauri dev
```

## Startup And Shell

- [ ] App window opens centered and fully visible, with no clipped bottom edge.
- [ ] Header shows app title, save filename/path when loaded, game label, dirty/saved state, and screenshot/no-screenshot state.
- [ ] Horizontal tabs are visible and usable.
- [ ] Money display sits beside the tabs and stays compact.
- [ ] `Open Save`, `Commit Changes`, `Reset Drafts`, and `Save As` are visible in the header after loading a save.
- [ ] `Save As` is disabled before any committed backend edit makes the save dirty.
- [ ] Error/warning banners are readable and dismissible if triggered.

## DAO Save

- [ ] Open a vanilla DAO save.
- [ ] `Characters`, `Inventory`, and `Recipes` tabs are visible.
- [ ] `Plot Flags` tab is hidden.
- [ ] Character overview fits without page-level scrolling at the intended Tauri window size.
- [ ] Main character does not show an approval field.
- [ ] Companion characters show approval when available.
- [ ] Progress, attributes, and point pools fields are readable and not clipped.
- [ ] Ability trees are collapsed by default or compact enough to scan.
- [ ] Ability tree groups expand and collapse correctly.
- [ ] Add/remove ability controls preserve prerequisite locking.
- [ ] Equipment item list and editor render correctly.
- [ ] Inventory item list shows only item name and amount when `item_stacksize > 1`.
- [ ] Inventory item row text is vertically centered.
- [ ] Money panel is compact and does not dominate the tab/header row.
- [ ] Item overview and properties columns align at the top.
- [ ] Item overview does not show duplicate name or stackable fields.
- [ ] Property dropdown uses the dark DAO styling, not a white background.
- [ ] Property table columns remain aligned while editing IDs and powers.
- [ ] If an item has many properties, only the property list scrolls; overview and add-property controls stay usable.
- [ ] Recipes render in compact columns at desktop width.
- [ ] Recipe rows are readable and toggle correctly.

## DA2 Save

- [ ] Open a DA2 save.
- [ ] `Plot Flags` tab is visible.
- [ ] DA2 abilities load into the expected combined talent/spell behavior.
- [ ] DA2 item property IDs and powers display correctly.
- [ ] Editing DA2 item property powers preserves float/bitcast behavior after commit and reload.
- [ ] DA2 crafting recipes load and toggle correctly.
- [ ] Plot flag integer groups render readable radio options.
- [ ] Plot flag boolean groups render readable checkbox rows.
- [ ] Plot flag changes can be drafted, committed, saved, and reloaded.

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
