import { describe, expect, it } from "vitest";
import { groupedRecipeChecklistIds, recipeIsKnown, recipeLabel } from "./recipeUtils";
import type { CraftingRecipe } from "../types";

const recipes: CraftingRecipe[] = [
  { id: 2, name: "Poultice", category: "Potions" },
  { id: 1, name: "Trap", category: "Traps" },
];

describe("recipeUtils", () => {
  it("labels known and unknown recipes", () => {
    expect(recipeLabel(recipes, 2)).toBe("Poultice (2)");
    expect(recipeLabel(recipes, 99)).toBe("Recipe 99");
    expect(recipeIsKnown(recipes, 99)).toBe(false);
  });

  it("groups available, saved, and draft ids while preserving unknowns", () => {
    expect(groupedRecipeChecklistIds(recipes, [99], [2])).toEqual([
      { category: "Traps", ids: [1] },
      { category: "Potions", ids: [2] },
      { category: "Other", ids: [99] },
    ]);
  });
});
