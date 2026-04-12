# DAO Manual Real-Game Checklist

Use this checklist against a copied Dragon Age: Origins save first. Do not edit the only copy of a save you care about.

Tester:

Date:

App build/version:

Game version/DLC/mods:

Original save path:

Edited save path:

Test result: Pass / Fail / Blocked

Notes:



## Pre-Flight

- [ ] Back up the original DAO save folder, including `.das`, `.das.met`, and `screen.dds` if present.

  Notes:


- [ ] Confirm DAO can load the original save before editing.

  Notes:


- [ ] Confirm the app starts without console, startup, or missing-data errors.

  Notes:


- [ ] Confirm test edits are intentionally easy to recognize in-game, but not extreme enough to risk obvious game overflow or broken progression.

  Notes:


## Open And Overview

- [ ] Open a DAO `.das` file through `Open Save`.

  Expected: the app accepts the save and shows the overview.

  Notes:


- [ ] Confirm the overview values look plausible: main character name, companion count, backpack item count, save status, and source path.

  Notes:


- [ ] If the save folder has `screen.dds`, confirm the save preview appears.

  Notes:


- [ ] Confirm the initial status is `Saved` / not dirty before making edits.

  Notes:


## Money

- [ ] Record the current in-game money before editing.

  Original value:

  Notes:


- [ ] Change Money in the Overview tab, click `Apply`, and confirm the status changes to dirty/unsaved.

  Edited value:

  Notes:


- [ ] Click `Reset` after changing the draft but before applying another value, and confirm it returns to the last applied value.

  Notes:


## Characters

- [ ] Open the Characters tab and confirm the party list contains the expected main character and companions.

  Notes:


- [ ] Select the main character and record baseline stats, level, and point pools.

  Original Strength:

  Original Dexterity:

  Original Willpower:

  Original Magic:

  Original Cunning:

  Original Constitution:

  Original Level:

  Original Attribute Points:

  Original Skill Points:

  Original Talent Points:

  Original Specialization Points:

  Notes:


- [ ] Edit one or more core stats for the main character, click `Apply`, and confirm the UI keeps the applied values.

  Edited values:

  Notes:


- [ ] Edit level and available point pools for the main character, click `Apply`, and confirm the UI keeps the applied values.

  Edited values:

  Notes:


- [ ] Select at least one companion and confirm their fields load without replacing the main character's draft values unexpectedly.

  Companion tested:

  Notes:


- [ ] Edit one safe companion value, click `Apply`, and confirm the UI keeps the applied value.

  Edited value:

  Notes:


- [ ] Click `Reset` after changing a character draft but before applying, and confirm it returns to the last applied value.

  Notes:


## Abilities

- [ ] Open the Abilities tab and confirm Skills, Talents, and Spells lists load for the selected character.

  Character tested:

  Notes:


- [ ] Add one available skill, talent, or spell that should be visible in-game, then click `Apply`.

  Ability added:

  Notes:


- [ ] Remove one non-core ability if available, then click `Apply`.

  Ability removed:

  Notes:


- [ ] Confirm locked/dependent ability entries cannot be deleted from the UI.

  Locked ability checked:

  Notes:


- [ ] Click `Reset` after changing ability drafts but before applying, and confirm it restores the last applied list.

  Notes:


## Inventory - Backpack

- [ ] Open the Inventory tab, choose `Backpack`, and confirm the item list count matches the overview backpack count.

  Notes:


- [ ] Select a backpack item and record its name, resref, material, item level, and properties.

  Item:

  Resref:

  Original Material:

  Original Item Level:

  Original Properties:

  Notes:


- [ ] Change the item's material if available, click `Apply`, and confirm the UI updates the material name/type consistently.

  Edited material:

  Notes:


- [ ] Change the item's item level, click `Apply`, and confirm the UI keeps the applied value.

  Edited item level:

  Notes:


- [ ] Add one item property with a visible or measurable effect, click `Apply`, and confirm the property remains listed.

  Property added:

  Power:

  Notes:


- [ ] Edit one existing item property power, click `Apply`, and confirm the property remains listed with the new power.

  Property edited:

  New power:

  Notes:


- [ ] Remove one item property, click `Apply`, and confirm it disappears from the UI.

  Property removed:

  Notes:


- [ ] Use `Remove Item` on a low-risk backpack item, then confirm the backpack list and overview count decrease by one.

  Item removed:

  Notes:


- [ ] Click `Reset` after changing inventory drafts but before applying, and confirm it restores the last applied item values.

  Notes:


## Inventory - Equipment

- [ ] Switch Inventory to `Equipment`, select the main character, and confirm equipped items load.

  Notes:


- [ ] Select an equipped item and record its name, slot, material, item level, and properties.

  Item:

  Slot:

  Original Material:

  Original Item Level:

  Original Properties:

  Notes:


- [ ] Change equipped item material or item level, click `Apply`, and confirm the UI keeps the applied value.

  Edited value:

  Notes:


- [ ] Add, edit, or remove one equipped item property, click `Apply`, and confirm the UI keeps the applied property list.

  Property change:

  Notes:


- [ ] Select at least one companion in the equipment dropdown and confirm their equipped items load.

  Companion tested:

  Notes:


## Save And Reload In App

- [ ] Use `Save As` and write the edited `.das` to a new test output path, not over the original save.

  Output path:

  Notes:


- [ ] Close and restart the app, then open the edited `.das`.

  Notes:


- [ ] Confirm edited money, character values, abilities, backpack changes, and equipment changes still appear after reloading the edited file in the app.

  Notes:


- [ ] Confirm the reloaded edited save status starts as saved/not dirty.

  Notes:


## Install Edited Save Into DAO

- [ ] Put the edited `.das` into a DAO save slot folder that contains compatible metadata, or replace a backed-up copied slot for testing.

  Slot folder:

  Notes:


- [ ] Start DAO and confirm the edited save appears in the Load Game menu.

  Notes:


- [ ] Load the edited save.

  Expected: the save loads without crashing, infinite loading, or returning to menu.

  Notes:


## In-Game Verification

- [ ] Confirm money matches the edited value.

  In-game value:

  Notes:


- [ ] Confirm main character stats, level, and point pools match the edited values where DAO exposes them.

  Notes:


- [ ] Confirm companion edits match the edited values where DAO exposes them.

  Notes:


- [ ] Confirm added abilities are present and usable or visible in the correct in-game screen.

  Notes:


- [ ] Confirm removed abilities are gone, if the game UI should show the removal.

  Notes:


- [ ] Confirm edited backpack item material/item level/properties are reflected in item stats, damage, armor, bonuses, or tooltips.

  Notes:


- [ ] Confirm removed backpack item is gone.

  Notes:


- [ ] Confirm edited equipped item material/item level/properties are reflected in item stats, damage, armor, bonuses, or tooltips.

  Notes:


- [ ] Move between areas or trigger a normal save/load cycle to confirm the edited save remains stable.

  Notes:


- [ ] Create a new in-game save from the edited save, then load that new save.

  New save:

  Notes:


## Negative And Edge Checks

- [ ] Try entering non-numeric text in a numeric field and confirm the app shows an error instead of applying bad data.

  Field tested:

  Notes:


- [ ] Try canceling `Open Save` and confirm the current app state is unchanged.

  Notes:


- [ ] Try canceling `Save As` and confirm no file is written and the current document remains open.

  Notes:


- [ ] Try opening an unsupported or intentionally wrong file type if available and confirm the app reports an error without losing the current document unexpectedly.

  File tested:

  Notes:


## Final Result

- [ ] All selected DAO real-game checks passed.

  Notes:


- [ ] Any failures have enough detail to reproduce: original save, edited save, exact field/value, game location, and observed behavior.

  Notes:


Failures / follow-up bugs:



