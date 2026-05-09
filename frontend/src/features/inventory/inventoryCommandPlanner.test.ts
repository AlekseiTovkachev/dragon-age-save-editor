import { describe, expect, it } from "vitest";
import { item } from "../../test/factories";
import { planInventoryDraftCommands } from "./inventoryCommandPlanner";
import type { InventoryItemDraft } from "./useInventoryEditor";

const draft = (overrides: Partial<InventoryItemDraft> = {}): InventoryItemDraft => ({
  itemMetadataDraft: { material: "1", item_level: "0", stack_size: "1" },
  itemPropertiesDraft: [],
  propertyDraft: { property_id: "", power: "" },
  ...overrides,
});

describe("planInventoryDraftCommands", () => {
  it("plans no commands for unchanged drafts", () => {
    const source = item({
      stackable: true,
      item_stacksize: 3,
      item_level: 0,
      properties: [{ id: 7, name: "Damage", power: 1 }],
    });

    expect(planInventoryDraftCommands({
      container: "backpack",
      entries: [{
        index: 0,
        item: source,
        draft: draft({
          itemMetadataDraft: { material: "1", item_level: "0", stack_size: "3" },
          itemPropertiesDraft: [{ id: 7, name: "Damage", power: "1", sourceIndex: 0 }],
        }),
      }],
    })).toEqual([]);
  });

  it("plans stack size and metadata changes", () => {
    const source = item({ stackable: true, item_stacksize: 3, item_level: 0, material: 1, properties: [] });

    expect(planInventoryDraftCommands({
      container: "backpack",
      entries: [{
        index: 0,
        item: source,
        draft: draft({ itemMetadataDraft: { material: "2", item_level: "4", stack_size: "9" } }),
      }],
    })).toEqual([
      { command: "set_backpack_item_stack_size", index: 0, stack_size: 9 },
      {
        command: "patch_item_metadata",
        container: "backpack",
        index: 0,
        patch: { material: 2, item_level: 4 },
      },
    ]);
  });

  it("plans property id, power, remove, and add changes with source indexes preserved", () => {
    const source = item({
      stackable: false,
      item_level: 0,
      properties: [
        { id: 1, name: "(Base Item): Weapon", power: 0 },
        { id: 7, name: "Damage", power: 1 },
        { id: 8, name: "Defense", power: 2 },
      ],
    });

    expect(planInventoryDraftCommands({
      container: "backpack",
      entries: [{
        index: 2,
        item: source,
        draft: draft({
          itemPropertiesDraft: [
            { id: 1, name: "(Base Item): Weapon", power: "0", sourceIndex: 0 },
            { id: 9, name: "Fire", power: "3", sourceIndex: 1 },
            { id: 10, name: "Cold", power: "4", sourceIndex: null },
          ],
        }),
      }],
    })).toEqual([
      { command: "set_item_property_id", container: "backpack", index: 2, property_index: 1, property_id: 9 },
      { command: "set_item_property_power", container: "backpack", index: 2, property_index: 1, power: 3 },
      { command: "remove_item_property", container: "backpack", index: 2, property_index: 2 },
      { command: "add_item_property", container: "backpack", index: 2, property_id: 10, power: 4 },
    ]);
  });

  it("rejects invalid stack and property numbers", () => {
    expect(() => planInventoryDraftCommands({
      container: "backpack",
      entries: [{
        index: 0,
        item: item({ stackable: true }),
        draft: draft({ itemMetadataDraft: { material: "1", item_level: "0", stack_size: "500" } }),
      }],
    })).toThrow("Stack size must be a whole number from 1 to 99.");

    expect(() => planInventoryDraftCommands({
      container: "backpack",
      entries: [{
        index: 0,
        item: item({ properties: [{ id: 7, name: "Damage", power: 1 }] }),
        draft: draft({ itemPropertiesDraft: [{ id: 7, name: "Damage", power: "nope", sourceIndex: 0 }] }),
      }],
    })).toThrow("Property 1 power must be a valid number.");
  });
});
