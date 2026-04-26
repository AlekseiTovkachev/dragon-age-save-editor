import type { CraftingRecipe } from "../types";

export type RecipeChecklistGroup = {
  category: string;
  ids: number[];
};

export function sortedRecipeChecklistIds(
  availableRecipes: CraftingRecipe[],
  savedRecipeIds: number[],
  draftRecipeIds: number[],
): number[] {
  return Array.from(new Set([
    ...availableRecipes.map((recipe) => recipe.id),
    ...savedRecipeIds,
    ...draftRecipeIds,
  ])).sort((left, right) => left - right);
}

export function recipeLabel(availableRecipes: CraftingRecipe[], recipeId: number): string {
  const recipe = availableRecipes.find((entry) => entry.id === recipeId);
  return recipe ? `${recipe.name} (${recipe.id})` : `Recipe ${recipeId}`;
}

export function recipeCategory(availableRecipes: CraftingRecipe[], recipeId: number): string {
  return availableRecipes.find((entry) => entry.id === recipeId)?.category ?? "Other";
}

export function recipeIsKnown(availableRecipes: CraftingRecipe[], recipeId: number): boolean {
  return availableRecipes.some((entry) => entry.id === recipeId);
}

export function groupedRecipeChecklistIds(
  availableRecipes: CraftingRecipe[],
  savedRecipeIds: number[],
  draftRecipeIds: number[],
): RecipeChecklistGroup[] {
  const groups = new Map<string, number[]>();
  for (const recipeId of sortedRecipeChecklistIds(availableRecipes, savedRecipeIds, draftRecipeIds)) {
    const category = recipeCategory(availableRecipes, recipeId);
    groups.set(category, [...(groups.get(category) ?? []), recipeId]);
  }
  return Array.from(groups, ([category, ids]) => ({ category, ids }));
}
