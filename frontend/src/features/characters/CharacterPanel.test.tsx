import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { character, indexedItem } from "../../test/factories";
import { CharacterPanel } from "./CharacterPanel";
import type { CharacterPanelActions, CharacterPanelState } from "./CharacterPanel";
import type { InventoryPanelActions, InventoryPanelState } from "../inventory/InventoryPanel";

function characterState(): CharacterPanelState {
  return {
    characters: [{ target: "main_character", name: "Hero" }],
    characterKey: "main",
    character: character(),
    levelDraft: "1",
    experienceDraft: "10",
    approvalDraft: "0",
    statsDraft: {},
    pointPoolsDraft: {},
    visibleAbilityKinds: [],
    selectedAbilityToAdd: { skills: "", talents: "", spells: "" },
    availableAbilities: { skills: [], talents: [], spells: [] },
    abilityDrafts: { skills: [], talents: [], spells: [] },
  };
}

function characterActions(): CharacterPanelActions {
  return {
    setCharacterKey: vi.fn(),
    setLevelDraft: vi.fn(),
    setExperienceDraft: vi.fn(),
    setApprovalDraft: vi.fn(),
    setStatsDraft: vi.fn(),
    setPointPoolsDraft: vi.fn(),
    coreAbilityOptions: () => [],
    visibleTreeAbilities: () => [],
    setSelectedAbilityToAdd: vi.fn(),
    handleAbilityAdd: vi.fn(),
    visibleAbilities: (_list, abilities) => abilities,
    abilityIsLocked: () => false,
    handleAbilityRemove: vi.fn(),
    handleVisibleAbilityAdd: vi.fn(),
  };
}

function inventoryState(): InventoryPanelState {
  const entry = indexedItem(0, {
    name: "Sword",
    stackable: false,
    properties: [{ id: 7, name: "Damage", power: 1 }],
  });

  return {
    moneyDraft: "100",
    items: [entry],
    itemIndex: 0,
    selectedItem: entry.item,
    canEditStackSize: false,
    canCloneBackpackItem: true,
    canEditMaterial: true,
    itemMetadataDraft: { material: "1", item_level: "1", stack_size: "1" },
    propertyDraft: { property_id: "7", power: "" },
    itemPropertiesDraft: [{ id: 7, name: "Damage", power: "1", sourceIndex: 0 }],
    availableItemProperties: [{ id: 7, name: "Damage" }],
  };
}

function inventoryActions(): InventoryPanelActions {
  return {
    setMoneyDraft: vi.fn(),
    setItemIndex: vi.fn(),
    setItemMetadataDraft: vi.fn(),
    setPropertyDraft: vi.fn(),
    handlePropertyAddDraft: vi.fn(),
    handlePropertyRemoveDraft: vi.fn(),
    handlePropertyUpdateDraft: vi.fn(),
    handleBackpackRemove: vi.fn(),
    handleBackpackClone: vi.fn(),
    handleWikiOpen: vi.fn(),
  };
}

describe("CharacterPanel equipment tab", () => {
  it("uses the inventory table with an equipment-only inline editor", () => {
    render(
      <CharacterPanel
        state={characterState()}
        actions={characterActions()}
        inventoryState={inventoryState()}
        inventoryActions={inventoryActions()}
        characterTab="equipment"
        setCharacterTab={vi.fn()}
        canEdit
        busy={false}
      />,
    );

    expect(
      screen.getByText(/Items currently carried by this character\. The save format doesn't track equipment slots/),
    ).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Sort by Item/ })).toBeInTheDocument();
    expect(screen.queryByRole("searchbox", { name: /Search inventory/ })).not.toBeInTheDocument();
    expect(screen.queryByText(/Party Gold/)).not.toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /Sword/ }));

    expect(screen.getByRole("heading", { name: "Overview", level: 3 })).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Clone Item" })).not.toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Remove Item" })).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Add" })).toBeInTheDocument();
  });
});
