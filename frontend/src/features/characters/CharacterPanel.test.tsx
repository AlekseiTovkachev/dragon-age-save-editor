import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ability, character, indexedItem } from "../../test/factories";
import { CharacterPanel } from "./CharacterPanel";
import type { CharacterPanelActions, CharacterPanelState } from "./CharacterPanel";
import type { InventoryPanelActions, InventoryPanelState } from "../inventory/InventoryPanel";
import type { Ability, AbilityListKind } from "../../types";

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

function characterActions(overrides: Partial<CharacterPanelActions> = {}): CharacterPanelActions {
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
    ...overrides,
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

function renderCharacterPanel(
  state: CharacterPanelState,
  actions: CharacterPanelActions,
  characterTab: "overview" | "abilities" | "equipment" = "abilities",
) {
  return render(
    <CharacterPanel
      state={state}
      actions={actions}
      inventoryState={inventoryState()}
      inventoryActions={inventoryActions()}
      characterTab={characterTab}
      setCharacterTab={vi.fn()}
      canEdit
      busy={false}
    />,
  );
}

const abilityCatalog: Record<AbilityListKind, Ability[]> = {
  skills: [
    ability(4001, { name: "Combat Training", tree: "Combat Training", ability_type: "Skill" }),
    ability(4002, { name: "Improved Combat Training", tree: "Combat Training", ability_type: "Skill", core_ids: [4001] }),
    ability(4101, { name: "Coercion", tree: "Social", ability_type: "Skill" }),
  ],
  talents: [
    ability(100, { name: "Shield Bash", tree: "Weapon and Shield", ability_type: "Warrior" }),
    ability(101, { name: "Shield Pummel", tree: "Weapon and Shield", ability_type: "Warrior", core_ids: [100] }),
    ability(120, { name: "Riposte", tree: "Dual Weapon", ability_type: "Rogue" }),
  ],
  spells: [
    ability(200, { name: "Flame Blast", tree: "Primal", ability_type: "Mage" }),
    ability(201, { name: "Flaming Weapons", tree: "Primal", ability_type: "Mage", core_ids: [200] }),
  ],
};

function abilityBrowserState(overrides: Partial<CharacterPanelState> = {}): CharacterPanelState {
  const skills = [abilityCatalog.skills[0]];
  const talents = [abilityCatalog.talents[0]];
  const spells = [abilityCatalog.spells[0]];

  return {
    ...characterState(),
    character: character({ skills, talents, spells }),
    visibleAbilityKinds: ["skills", "talents", "spells"],
    availableAbilities: abilityCatalog,
    abilityDrafts: { skills, talents, spells },
    ...overrides,
  };
}

function abilityBrowserActions(overrides: Partial<CharacterPanelActions> = {}): CharacterPanelActions {
  return characterActions({
    visibleTreeAbilities: (list) => abilityCatalog[list],
    ...overrides,
  });
}

describe("CharacterPanel equipment tab", () => {
  it("renders the redesigned party rail, character header, subtabs, and overview cards", () => {
    render(
      <CharacterPanel
        state={characterState()}
        actions={characterActions()}
        inventoryState={inventoryState()}
        inventoryActions={inventoryActions()}
        characterTab="overview"
        setCharacterTab={vi.fn()}
        canEdit
        busy={false}
      />,
    );

    expect(screen.getByRole("complementary", { name: "Party members" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Hero", level: 2 })).toBeInTheDocument();
    expect(screen.getByRole("navigation", { name: "Character sections" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Progress", level: 3 })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Attributes", level: 3 })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Point Pools", level: 3 })).toBeInTheDocument();
  });

  it("marks overview numeric inputs dirty per field", () => {
    const state = {
      ...characterState(),
      levelDraft: "2",
      statsDraft: { strength: "10", magic: "99" },
      pointPoolsDraft: { attribute_points: "1", skill_points: "5" },
    };

    render(
      <CharacterPanel
        state={state}
        actions={characterActions()}
        inventoryState={inventoryState()}
        inventoryActions={inventoryActions()}
        characterTab="overview"
        setCharacterTab={vi.fn()}
        canEdit
        busy={false}
      />,
    );

    expect(screen.getByLabelText("Level")).toHaveClass("dirty");
    expect(screen.getByLabelText("Strength")).not.toHaveClass("dirty");
    expect(screen.getByLabelText("Magic")).toHaveClass("dirty");
    expect(screen.getByLabelText("Attribute Points")).not.toHaveClass("dirty");
    expect(screen.getByLabelText("Skill Points")).toHaveClass("dirty");
    expect(screen.getByText("Modified")).toBeInTheDocument();
  });

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

describe("CharacterPanel abilities tab", () => {
  it("renders ability kind tabs and switches the active rank ladder", () => {
    renderCharacterPanel(abilityBrowserState(), abilityBrowserActions());

    expect(screen.getByRole("navigation", { name: "Ability lists" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Skills/ })).toHaveClass("is-active");
    expect(screen.getByRole("heading", { name: "Combat Training ranks", level: 3 })).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /Talents/ }));

    expect(screen.getByRole("button", { name: /Talents/ })).toHaveClass("is-active");
    expect(screen.getByRole("heading", { name: "Weapon and Shield Talents ranks", level: 3 })).toBeInTheDocument();
    expect(screen.queryByRole("heading", { name: "Combat Training ranks", level: 3 })).not.toBeInTheDocument();
  });

  it("filters by tree name and by rank ability name", () => {
    renderCharacterPanel(abilityBrowserState(), abilityBrowserActions());

    fireEvent.click(screen.getByRole("button", { name: /Talents/ }));
    expect(screen.getByRole("option", { name: /Weapon and Shield Talents/ })).toBeInTheDocument();
    expect(screen.getByRole("option", { name: /Dual Weapon Talents/ })).toBeInTheDocument();

    fireEvent.change(screen.getByRole("searchbox", { name: "Search abilities" }), {
      target: { value: "dual weapon" },
    });
    expect(screen.queryByRole("option", { name: /Weapon and Shield Talents/ })).not.toBeInTheDocument();
    expect(screen.getByRole("option", { name: /Dual Weapon Talents/ })).toBeInTheDocument();
    expect(screen.getByText("Riposte")).toBeInTheDocument();

    fireEvent.change(screen.getByRole("searchbox", { name: "Search abilities" }), {
      target: { value: "shield pummel" },
    });
    expect(screen.getByRole("option", { name: /Weapon and Shield Talents/ })).toBeInTheDocument();
    expect(screen.queryByRole("option", { name: /Dual Weapon Talents/ })).not.toBeInTheDocument();
    expect(screen.getByText("Shield Pummel")).toBeInTheDocument();
  });

  it("changes the rank ladder when a tree is selected", () => {
    renderCharacterPanel(abilityBrowserState(), abilityBrowserActions());

    fireEvent.click(screen.getByRole("button", { name: /Talents/ }));
    expect(screen.getByRole("heading", { name: "Weapon and Shield Talents ranks", level: 3 })).toBeInTheDocument();

    fireEvent.click(screen.getByRole("option", { name: /Dual Weapon Talents/ }));

    expect(screen.getByRole("heading", { name: "Dual Weapon Talents ranks", level: 3 })).toBeInTheDocument();
    expect(screen.getByText("Riposte")).toBeInTheDocument();
    expect(screen.queryByText("Shield Bash")).not.toBeInTheDocument();
  });

  it("calls ability add and remove handlers with the active kind and ability id", () => {
    const handleVisibleAbilityAdd = vi.fn();
    const handleAbilityRemove = vi.fn();
    renderCharacterPanel(
      abilityBrowserState(),
      abilityBrowserActions({
        handleVisibleAbilityAdd,
        handleAbilityRemove,
      }),
    );

    fireEvent.click(screen.getByRole("button", { name: /Talents/ }));
    fireEvent.click(screen.getByRole("button", { name: "Add Shield Pummel" }));
    fireEvent.click(screen.getByRole("button", { name: "Remove Shield Bash" }));

    expect(handleVisibleAbilityAdd).toHaveBeenCalledWith("talents", 101);
    expect(handleAbilityRemove).toHaveBeenCalledWith("talents", 100);
  });

  it("disables removal for locked owned abilities and shows the required state", () => {
    const handleAbilityRemove = vi.fn();
    renderCharacterPanel(
      abilityBrowserState(),
      abilityBrowserActions({
        abilityIsLocked: (list, abilityId) => list === "talents" && abilityId === 100,
        handleAbilityRemove,
      }),
    );

    fireEvent.click(screen.getByRole("button", { name: /Talents/ }));

    expect(screen.getByText("Locked ranks are required by another ability.")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Required Shield Bash" })).toBeDisabled();

    fireEvent.click(screen.getByRole("button", { name: "Required Shield Bash" }));

    expect(handleAbilityRemove).not.toHaveBeenCalled();
  });
});
