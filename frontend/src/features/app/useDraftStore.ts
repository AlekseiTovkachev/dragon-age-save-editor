import { useCallback } from "react";
import { executeCommand, expectResult } from "../../api";
import type { SaveCommand, SaveSummary } from "../../types";
import type { useCharacterEditor } from "../characters/useCharacterEditor";
import type { useCraftingEditor } from "../crafting/useCraftingEditor";
import type { useInventoryEditor } from "../inventory/useInventoryEditor";
import type { usePlotFlagsEditor } from "../plotFlags/usePlotFlagsEditor";
import type { AsyncRun } from "../shared/types";

type CharacterEditor = ReturnType<typeof useCharacterEditor>;
type InventoryEditor = ReturnType<typeof useInventoryEditor>;
type CraftingEditor = ReturnType<typeof useCraftingEditor>;
type PlotFlagsEditor = ReturnType<typeof usePlotFlagsEditor>;

type UseDraftStoreOptions = {
  preferredGame: SaveSummary["preferred_game"] | null | undefined;
  run: AsyncRun;
  refreshSummary: () => Promise<unknown>;
  characterEditor: CharacterEditor;
  inventoryEditor: InventoryEditor;
  craftingEditor: CraftingEditor;
  plotFlagsEditor: PlotFlagsEditor;
};

function remapItemIndex(command: SaveCommand, fromIndex: number, toIndex: number): SaveCommand {
  if ("index" in command && command.index === fromIndex) {
    return { ...command, index: toIndex } as SaveCommand;
  }
  return command;
}

export function useDraftStore({
  preferredGame,
  run,
  refreshSummary,
  characterEditor,
  inventoryEditor,
  craftingEditor,
  plotFlagsEditor,
}: UseDraftStoreOptions) {
  const hasPendingDrafts = useCallback(() => {
    const characterPlan = characterEditor.planCommands();
    const inventoryPlan = inventoryEditor.planCommands();
    const craftingPlan = craftingEditor.planCommands();
    const plotFlagPlan = preferredGame === "da2" ? plotFlagsEditor.planCommands() : { batch: [] };
    return (
      characterPlan.batch.length > 0 ||
      inventoryPlan.batch.length > 0 ||
      (inventoryPlan.clones?.length ?? 0) > 0 ||
      (inventoryPlan.removes?.length ?? 0) > 0 ||
      craftingPlan.batch.length > 0 ||
      plotFlagPlan.batch.length > 0
    );
  }, [characterEditor, craftingEditor, inventoryEditor, plotFlagsEditor, preferredGame]);

  const apply = useCallback(async () => {
    return run(async () => {
      const characterPlan = characterEditor.planCommands();
      const inventoryPlan = inventoryEditor.planCommands();
      const craftingPlan = craftingEditor.planCommands();
      const plotFlagPlan = preferredGame === "da2" ? plotFlagsEditor.planCommands() : { batch: [] };
      const batch = [
        ...characterPlan.batch,
        ...inventoryPlan.batch,
        ...craftingPlan.batch,
        ...plotFlagPlan.batch,
      ];
      const clones = inventoryPlan.clones ?? [];
      const removes = inventoryPlan.removes ?? [];
      const hasInventoryStructureChanges = clones.length > 0 || removes.length > 0;

      if (batch.length > 0) {
        await executeCommand({ command: "apply_batch", commands: batch });
      }

      for (const clone of clones) {
        const cloneResponse = expectResult(
          await executeCommand({ command: "clone_backpack_item", index: clone.sourceIndex }),
          "item",
        );
        const cloneCommands = clone.batch.map((command) => remapItemIndex(command, clone.tempIndex, cloneResponse.index));
        if (cloneCommands.length > 0) {
          await executeCommand({ command: "apply_batch", commands: cloneCommands });
        }
      }

      if (removes.length > 0) {
        await executeCommand({
          command: "apply_batch",
          commands: removes.map((index) => ({ command: "remove_backpack_item" as const, index })),
        });
      }

      if (batch.length > 0 || hasInventoryStructureChanges) {
        await refreshSummary();
        await characterEditor.refreshLoadedCharacters();
        await inventoryEditor.refreshItems();
        await craftingEditor.refreshCraftingRecipes();
        if (preferredGame === "da2") {
          await plotFlagsEditor.refreshPlotFlags();
        }
      }

      characterEditor.markDraftsCommitted();
      inventoryEditor.markDraftsCommitted(hasInventoryStructureChanges);
      craftingEditor.markDraftsCommitted();
      plotFlagsEditor.markDraftsCommitted();
    });
  }, [
    characterEditor,
    craftingEditor,
    inventoryEditor,
    plotFlagsEditor,
    preferredGame,
    refreshSummary,
    run,
  ]);

  const reset = useCallback(() => {
    characterEditor.resetToCommittedDrafts();
    inventoryEditor.resetToCommittedDrafts();
    craftingEditor.resetToCommittedDrafts();
    plotFlagsEditor.resetToCommittedDrafts();
  }, [characterEditor, craftingEditor, inventoryEditor, plotFlagsEditor]);

  return { apply, reset, hasPendingDrafts };
}
