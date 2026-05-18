import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { executeCommand, expectResult } from "../../api";
import { useDraftCheckpoint } from "../../hooks/useDraftCheckpoint";
import {
  groupedRecipeChecklistIds,
  recipeIsKnown,
  recipeLabel,
  sortedRecipeChecklistIds,
} from "../../lib/recipeUtils";
import type { CraftingRecipe, SaveCommand } from "../../types";
import type { AsyncRun } from "../shared/types";

type UseCraftingEditorOptions = {
  run: AsyncRun;
  refreshSummary: () => Promise<unknown>;
};

const cloneRecipeDrafts = (draft: number[]) => [...draft];

type CraftingCommandPlan = {
  batch: SaveCommand[];
};

function sameRecipeIds(left: number[], right: number[]) {
  return left.length === right.length && left.every((value, index) => value === right[index]);
}

export function useCraftingEditor({ run, refreshSummary }: UseCraftingEditorOptions) {
  const [craftingRecipes, setCraftingRecipes] = useState<number[]>([]);
  const [craftingRecipeDrafts, setCraftingRecipeDrafts] = useState<number[]>([]);
  const [availableCraftingRecipes, setAvailableCraftingRecipes] = useState<CraftingRecipe[]>([]);
  const craftingRecipeDraftsRef = useRef(craftingRecipeDrafts);
  const draftCheckpoint = useDraftCheckpoint<number[]>({ clone: cloneRecipeDrafts });

  useEffect(() => {
    craftingRecipeDraftsRef.current = craftingRecipeDrafts;
  }, [craftingRecipeDrafts]);

  const refreshCraftingRecipes = useCallback(async () => {
    const response = expectResult(await executeCommand({ command: "list_crafting_recipes" }), "crafting_recipes");
    setCraftingRecipes(response.recipe_ids);
    setCraftingRecipeDrafts(response.recipe_ids);
    draftCheckpoint.checkpoint(response.recipe_ids);
  }, [draftCheckpoint]);

  const refreshAvailableCraftingRecipes = useCallback(async () => {
    const response = expectResult(
      await executeCommand({ command: "list_available_crafting_recipes" }),
      "available_crafting_recipes",
    );
    setAvailableCraftingRecipes(response.recipes);
  }, []);

  const handleToggle = useCallback((recipeId: number, checked: boolean) => {
    setCraftingRecipeDrafts((current) => {
      if (checked) {
        return current.includes(recipeId) ? current : [...current, recipeId];
      }
      return current.filter((id) => id !== recipeId);
    });
  }, []);

  const commitRecipeDrafts = useCallback(async () => {
    return run(async () => {
      const response = expectResult(
        await executeCommand({
          command: "replace_crafting_recipe_list",
          recipe_ids: craftingRecipeDraftsRef.current,
        }),
        "crafting_recipes",
      );
      setCraftingRecipes(response.recipe_ids);
      setCraftingRecipeDrafts(response.recipe_ids);
      await refreshSummary();
    });
  }, [refreshSummary, run]);

  const planCommands = useCallback((): CraftingCommandPlan => {
    if (sameRecipeIds(craftingRecipes, craftingRecipeDrafts)) {
      return { batch: [] };
    }
    return {
      batch: [{ command: "replace_crafting_recipe_list", recipe_ids: craftingRecipeDrafts }],
    };
  }, [craftingRecipeDrafts, craftingRecipes]);

  const resetLoadedDrafts = useCallback(() => {
    setCraftingRecipeDrafts(craftingRecipes);
  }, [craftingRecipes]);

  const checkpointDrafts = useCallback(() => {
    draftCheckpoint.checkpoint(craftingRecipeDrafts);
  }, [craftingRecipeDrafts, draftCheckpoint]);

  const commitDrafts = useCallback(async () => {
    if (!await commitRecipeDrafts()) {
      return false;
    }
    checkpointDrafts();
    return true;
  }, [checkpointDrafts, commitRecipeDrafts]);

  const resetToCommittedDrafts = useCallback(() => {
    const checkpoint = draftCheckpoint.reset();
    if (checkpoint) {
      setCraftingRecipeDrafts(checkpoint);
    }
  }, [draftCheckpoint]);

  const clear = useCallback(() => {
    setCraftingRecipes([]);
    setCraftingRecipeDrafts([]);
    setAvailableCraftingRecipes([]);
    draftCheckpoint.clear();
  }, [draftCheckpoint]);

  const sortedRecipeIds = useMemo(
    () => sortedRecipeChecklistIds(availableCraftingRecipes, craftingRecipes, craftingRecipeDrafts),
    [availableCraftingRecipes, craftingRecipeDrafts, craftingRecipes],
  );
  const groupedRecipeIds = useMemo(
    () => groupedRecipeChecklistIds(availableCraftingRecipes, craftingRecipes, craftingRecipeDrafts),
    [availableCraftingRecipes, craftingRecipeDrafts, craftingRecipes],
  );
  const getRecipeLabel = useCallback(
    (recipeId: number) => recipeLabel(availableCraftingRecipes, recipeId),
    [availableCraftingRecipes],
  );
  const getRecipeIsKnown = useCallback(
    (recipeId: number) => recipeIsKnown(availableCraftingRecipes, recipeId),
    [availableCraftingRecipes],
  );

  return {
    craftingRecipeDrafts,
    availableCraftingRecipes,
    sortedRecipeIds,
    groupedRecipeIds,
    recipeLabel: getRecipeLabel,
    recipeIsKnown: getRecipeIsKnown,
    refreshCraftingRecipes,
    refreshAvailableCraftingRecipes,
    handleToggle,
    commitRecipeDrafts,
    resetLoadedDrafts,
    planCommands,
    commitDrafts,
    resetToCommittedDrafts,
    clear,
  };
}
