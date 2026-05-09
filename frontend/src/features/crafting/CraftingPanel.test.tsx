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

/** Click the expand toggle for a card (aria-expanded button that contains the heading text) */
function clickCardHead(name: string) {
  const btn = screen.getAllByRole("button", { name: new RegExp(name, "i") })
    .find((el) => el.hasAttribute("aria-expanded"));
  if (!btn) throw new Error(`No card-head button found for "${name}"`);
  fireEvent.click(btn);
}

describe("CraftingPanel", () => {
  it("renders recipe category cards collapsed by default", () => {
    renderCraftingPanel();

    expect(screen.getByRole("heading", { name: "Potions" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Traps" })).toBeInTheDocument();
    expect(screen.getByText("2 of 3 known")).toBeInTheDocument();
    // Rows are not visible while collapsed
    expect(screen.queryByRole("checkbox", { name: "Potent Lyrium Potion (2)" })).not.toBeInTheDocument();
  });

  it("expands a card on header click and shows recipe rows", () => {
    renderCraftingPanel();

    clickCardHead("Potions");
    expect(screen.getByRole("checkbox", { name: "Potent Lyrium Potion (2)" })).toBeChecked();

    clickCardHead("Traps");
    // Unknown recipe preserved as disabled checked
    expect(screen.getByRole("checkbox", { name: "Recipe 99 (unknown, preserved)" })).toBeDisabled();
  });

  it("filters recipes by search text and category chips", () => {
    renderCraftingPanel();

    // Expand Potions first so rows are visible
    clickCardHead("Potions");

    fireEvent.change(screen.getByRole("searchbox", { name: "Search recipes" }), { target: { value: "lyrium" } });
    expect(screen.getByRole("checkbox", { name: "Potent Lyrium Potion (2)" })).toBeInTheDocument();
    expect(screen.queryByRole("checkbox", { name: "Lesser Health Poultice (1)" })).not.toBeInTheDocument();

    fireEvent.change(screen.getByRole("searchbox", { name: "Search recipes" }), { target: { value: "" } });
    fireEvent.click(screen.getByRole("button", { name: "Traps", hidden: false }));
    // After filtering to Traps category, Potions card is gone
    expect(screen.queryByRole("heading", { name: "Potions" })).not.toBeInTheDocument();
    // Expand Traps card to see rows
    clickCardHead("Traps");
    expect(screen.getByRole("checkbox", { name: "Recipe 99 (unknown, preserved)" })).toBeInTheDocument();
  });

  it("toggles known recipe checkboxes", () => {
    const actions = renderCraftingPanel();

    // Expand Potions to access checkboxes
    clickCardHead("Potions");

    fireEvent.click(screen.getByRole("checkbox", { name: "Lesser Health Poultice (1)" }));
    expect(actions.handleToggle).toHaveBeenCalledWith(1, true);

    fireEvent.click(screen.getByRole("checkbox", { name: "Potent Lyrium Potion (2)" }));
    expect(actions.handleToggle).toHaveBeenCalledWith(2, false);
  });
});
