import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { CraftingPanel } from "./CraftingPanel";
import type { CraftingPanelActions, CraftingPanelState } from "./CraftingPanel";

const state: CraftingPanelState = {
  sortedRecipeIds: [1, 2, 99],
  groupedRecipeIds: [
    { category: "Potions", ids: [1, 2] },
    { category: "Traps", ids: [99] },
  ],
  craftingRecipeDrafts: [2, 99],
};

function renderCraftingPanel(overrides: Partial<CraftingPanelState> = {}) {
  const actions: CraftingPanelActions = {
    handleToggle: vi.fn(),
    recipeIsKnown: (recipeId) => recipeId !== 99,
    recipeLabel: (recipeId) => {
      const labels: Record<number, string> = {
        1: "Lesser Health Poultice (1)",
        2: "Potent Lyrium Potion (2)",
        99: "Recipe 99",
      };
      return labels[recipeId] ?? `Recipe ${recipeId}`;
    },
  };

  render(
    <CraftingPanel
      state={{ ...state, ...overrides }}
      actions={actions}
      canEdit
      busy={false}
    />,
  );

  return actions;
}

describe("CraftingPanel", () => {
  it("renders recipe category cards and preserves unknown recipes as disabled checked rows", () => {
    renderCraftingPanel();

    expect(screen.getByRole("heading", { name: "Potions" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Traps" })).toBeInTheDocument();
    expect(screen.getByText("2 of 3 known")).toBeInTheDocument();
    expect(screen.getByRole("checkbox", { name: "Potent Lyrium Potion (2)" })).toBeChecked();
    expect(screen.getByRole("checkbox", { name: "Recipe 99 (unknown, preserved)" })).toBeDisabled();
  });

  it("filters recipes by search text and category chips", () => {
    renderCraftingPanel();

    fireEvent.change(screen.getByRole("searchbox", { name: "Search recipes" }), { target: { value: "lyrium" } });
    expect(screen.getByRole("checkbox", { name: "Potent Lyrium Potion (2)" })).toBeInTheDocument();
    expect(screen.queryByRole("checkbox", { name: "Lesser Health Poultice (1)" })).not.toBeInTheDocument();

    fireEvent.change(screen.getByRole("searchbox", { name: "Search recipes" }), { target: { value: "" } });
    fireEvent.click(screen.getByRole("button", { name: "Traps" }));
    expect(screen.queryByRole("heading", { name: "Potions" })).not.toBeInTheDocument();
    expect(screen.getByRole("checkbox", { name: "Recipe 99 (unknown, preserved)" })).toBeInTheDocument();
  });

  it("toggles known recipe checkboxes", () => {
    const actions = renderCraftingPanel();

    fireEvent.click(screen.getByRole("checkbox", { name: "Lesser Health Poultice (1)" }));
    expect(actions.handleToggle).toHaveBeenCalledWith(1, true);

    fireEvent.click(screen.getByRole("checkbox", { name: "Potent Lyrium Potion (2)" }));
    expect(actions.handleToggle).toHaveBeenCalledWith(2, false);
  });
});
