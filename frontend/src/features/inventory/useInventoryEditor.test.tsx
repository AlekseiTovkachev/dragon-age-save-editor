import { act, renderHook, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { indexedItem, item, summary } from "../../test/factories";
import type { InventoryContainer, SaveCommandResult } from "../../types";
import { useInventoryEditor } from "./useInventoryEditor";

const mocks = vi.hoisted(() => ({
  executeCommand: vi.fn(),
  openUrl: vi.fn(),
}));

vi.mock("../../api", () => ({
  executeCommand: mocks.executeCommand,
  expectResult: (response: SaveCommandResult, result: SaveCommandResult["result"]) => {
    if (response.result !== result) {
      throw new Error(`Expected ${result}, received ${response.result}`);
    }
    return response;
  },
}));

vi.mock("@tauri-apps/plugin-opener", () => ({
  openUrl: mocks.openUrl,
}));

const run = async (action: () => Promise<void>) => {
  await action();
  return true;
};

describe("useInventoryEditor", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.executeCommand.mockImplementation(async (command: { command: string }) => {
      if (command.command === "list_backpack_items") {
        return { result: "items", items: [indexedItem()] };
      }
      if (command.command === "list_available_item_properties") {
        return {
          result: "available_item_properties",
          properties: [
            { id: 7, name: "Damage" },
            { id: 8, name: "Defense" },
            { id: 9, name: "Fire" },
          ],
        };
      }
      if (command.command === "set_money") {
        return { result: "summary", summary: summary({ money: 777 }) };
      }
      if (command.command === "clone_backpack_item") {
        return { result: "item", container: "backpack", index: 3, item: item() };
      }
      return { result: "item", container: "backpack", index: 0, item: item() };
    });
  });

  it("plans stack size, metadata, and property mutations in order", async () => {
    const setError = vi.fn();
    const { result } = renderHook(() =>
      useInventoryEditor({
        summary: summary(),
        container: "backpack",
        isBackpackInventory: true,
        run,
        setError,
      }),
    );

    await act(async () => {
      await result.current.refreshAvailableItemProperties();
      await result.current.refreshItems();
    });
    await waitFor(() => expect(result.current.selectedItem).not.toBeNull());

    act(() => {
      result.current.setItemMetadataDraft((current) => ({ ...current, stack_size: "5", item_level: "2" }));
      result.current.handlePropertyUpdateDraft("id", 0, "8");
      result.current.handlePropertyUpdateDraft("power", 0, "2.5");
      result.current.setPropertyDraft({ property_id: "9", power: "3" });
    });
    act(() => {
      result.current.handlePropertyAddDraft();
    });

    expect(result.current.planCommands()).toEqual({
      clones: [],
      removes: [],
      batch: [
        { command: "set_backpack_item_stack_size", index: 0, stack_size: 5 },
        { command: "patch_item_metadata", container: "backpack", index: 0, patch: { item_level: 2 } },
        { command: "set_item_property_id", container: "backpack", index: 0, property_index: 0, property_id: 8 },
        { command: "set_item_property_power", container: "backpack", index: 0, property_index: 0, power: 2.5 },
        { command: "add_item_property", container: "backpack", index: 0, property_id: 9, power: 3 },
      ],
    });
  });

  it("plans money and item draft commands", async () => {
    const { result } = renderHook(() =>
      useInventoryEditor({
        summary: summary({ money: 100 }),
        container: "backpack",
        isBackpackInventory: true,
        run,
        setError: vi.fn(),
      }),
    );

    await act(async () => {
      await result.current.refreshItems();
    });
    await waitFor(() => expect(result.current.selectedItem).not.toBeNull());

    act(() => {
      result.current.setMoneyDraft("777");
      result.current.setItemMetadataDraft((current) => ({ ...current, stack_size: "5" }));
    });

    expect(result.current.planCommands()).toEqual({
      clones: [],
      removes: [],
      batch: [
        { command: "set_money", money: 777 },
        { command: "set_backpack_item_stack_size", index: 0, stack_size: 5 },
      ],
    });
  });

  it("reports invalid stack size before planning edit commands", async () => {
    const setError = vi.fn();
    const { result } = renderHook(() =>
      useInventoryEditor({
        summary: summary(),
        container: "backpack",
        isBackpackInventory: true,
        run,
        setError,
      }),
    );

    await act(async () => {
      await result.current.refreshItems();
    });
    act(() => {
      result.current.setItemMetadataDraft((current) => ({ ...current, stack_size: "500" }));
    });

    expect(() => result.current.planCommands()).toThrow("Stack size must be a whole number from 1 to 99.");
  });

  it("keeps item drafts when switching selections", async () => {
    mocks.executeCommand.mockImplementation(async (command: { command: string }) => {
      if (command.command === "list_backpack_items") {
        return {
          result: "items",
          items: [
            indexedItem(0, { name: "Potion", item_stacksize: 3 }),
            indexedItem(1, { name: "Bomb", item_stacksize: 2 }),
          ],
        };
      }
      return { result: "item", container: "backpack", index: 0, item: item() };
    });
    const { result } = renderHook(() =>
      useInventoryEditor({
        summary: summary(),
        container: "backpack",
        isBackpackInventory: true,
        run,
        setError: vi.fn(),
      }),
    );

    await act(async () => {
      await result.current.refreshItems();
    });
    await waitFor(() => expect(result.current.itemIndex).toBe(0));

    act(() => {
      result.current.setItemMetadataDraft((current) => ({ ...current, stack_size: "12" }));
      result.current.setItemIndex(1);
    });
    await waitFor(() => expect(result.current.itemMetadataDraft.stack_size).toBe("2"));

    act(() => {
      result.current.setItemIndex(0);
    });

    await waitFor(() => expect(result.current.itemMetadataDraft.stack_size).toBe("12"));
  });

  it("hides stale items while a new container is loading", async () => {
    const equipmentContainer = { equipment: { target: { companion: { index: 2 } } } } as const;
    mocks.executeCommand.mockImplementation(async (command: { command: string }) => {
      if (command.command === "list_backpack_items") {
        return { result: "items", items: [indexedItem(0, { name: "Backpack Potion" })] };
      }
      if (command.command === "list_equipment_items") {
        return { result: "items", items: [indexedItem(2, { name: "Heavy Chainmail" })] };
      }
      return { result: "item", container: "backpack", index: 0, item: item() };
    });
    const { result, rerender } = renderHook(
      ({ container }: { container: InventoryContainer }) =>
        useInventoryEditor({
          summary: summary(),
          container,
          isBackpackInventory: container === "backpack",
          run,
          setError: vi.fn(),
        }),
      { initialProps: { container: "backpack" as InventoryContainer } },
    );

    await act(async () => {
      await result.current.refreshItems();
    });
    await waitFor(() => expect(result.current.items[0]?.item.name).toBe("Backpack Potion"));

    rerender({ container: equipmentContainer });

    expect(result.current.items).toEqual([]);
    expect(result.current.selectedItem).toBeNull();

    await act(async () => {
      await result.current.refreshItems();
    });

    expect(result.current.items[0]?.item.name).toBe("Heavy Chainmail");
  });

  it("queues backpack removal until global apply", async () => {
    mocks.executeCommand.mockImplementation(async (command: { command: string }) => {
      if (command.command === "list_backpack_items") {
        return {
          result: "items",
          items: [
            indexedItem(0, { name: "Sword" }),
            indexedItem(1, { name: "Shield" }),
          ],
        };
      }
      return { result: "summary", summary: summary({ dirty: true }) };
    });
    const { result } = renderHook(() =>
      useInventoryEditor({
        summary: summary(),
        container: "backpack",
        isBackpackInventory: true,
        run,
        setError: vi.fn(),
      }),
    );

    await act(async () => {
      await result.current.refreshItems();
    });
    await waitFor(() => expect(result.current.itemIndex).toBe(0));

    await act(async () => {
      await result.current.handleBackpackRemove();
    });

    expect(result.current.items.map((entry) => entry.index)).toEqual([1]);
    expect(mocks.executeCommand).not.toHaveBeenCalledWith({
      command: "apply_batch",
      commands: [{ command: "remove_backpack_item", index: 0 }],
    });

    expect(result.current.planCommands()).toEqual({ clones: [], removes: [0], batch: [] });
  });

  it("queues backpack clone until global apply", async () => {
    mocks.executeCommand.mockImplementation(async (command: { command: string; index?: number }) => {
      if (command.command === "list_backpack_items") {
        return {
          result: "items",
          items: [indexedItem(0, { name: "Sword", stackable: false, item_stacksize: null })],
        };
      }
      if (command.command === "clone_backpack_item") {
        return {
          result: "item",
          container: "backpack",
          index: 1,
          item: item({ name: "Sword", stackable: false, item_stacksize: null }),
        };
      }
      return { result: "summary", summary: summary({ dirty: true }) };
    });
    const { result } = renderHook(() =>
      useInventoryEditor({
        summary: summary(),
        container: "backpack",
        isBackpackInventory: true,
        run,
        setError: vi.fn(),
      }),
    );

    await act(async () => {
      await result.current.refreshItems();
    });
    await waitFor(() => expect(result.current.canCloneBackpackItem).toBe(true));

    await act(async () => {
      await result.current.handleBackpackClone();
    });

    expect(result.current.itemIndex).toBe(0);
    expect(mocks.executeCommand).not.toHaveBeenCalledWith({ command: "clone_backpack_item", index: 0 });

    expect(result.current.planCommands()).toEqual({
      clones: [{ tempIndex: -1, sourceIndex: 0, batch: [] }],
      removes: [],
      batch: [],
    });
  });

  it("queues repeated backpack clones from the selected source item", async () => {
    mocks.executeCommand.mockImplementation(async (command: { command: string }) => {
      if (command.command === "list_backpack_items") {
        return {
          result: "items",
          items: [indexedItem(0, { name: "Ring of Ages", stackable: false, item_stacksize: null })],
        };
      }
      return { result: "summary", summary: summary({ dirty: true }) };
    });
    const { result } = renderHook(() =>
      useInventoryEditor({
        summary: summary(),
        container: "backpack",
        isBackpackInventory: true,
        run,
        setError: vi.fn(),
      }),
    );

    await act(async () => {
      await result.current.refreshItems();
    });
    await waitFor(() => expect(result.current.canCloneBackpackItem).toBe(true));

    await act(async () => {
      await result.current.handleBackpackClone();
    });
    await waitFor(() => expect(result.current.canCloneBackpackItem).toBe(true));
    await act(async () => {
      await result.current.handleBackpackClone();
    });

    expect(result.current.planCommands()).toEqual({
      clones: [
        { tempIndex: -1, sourceIndex: 0, batch: [] },
        { tempIndex: -2, sourceIndex: 0, batch: [] },
      ],
      removes: [],
      batch: [],
    });
  });

  it("plans queued backpack clones and removals", async () => {
    mocks.executeCommand.mockImplementation(async (command: { command: string }) => {
      if (command.command === "list_backpack_items") {
        return {
          result: "items",
          items: [
            indexedItem(0, { name: "Sword", stackable: false, item_stacksize: null }),
            indexedItem(1, { name: "Shield", stackable: false, item_stacksize: null }),
          ],
        };
      }
      return { result: "summary", summary: summary({ dirty: true }) };
    });
    const { result } = renderHook(() =>
      useInventoryEditor({
        summary: summary(),
        container: "backpack",
        isBackpackInventory: true,
        run,
        setError: vi.fn(),
      }),
    );

    await act(async () => {
      await result.current.refreshItems();
    });
    await waitFor(() => expect(result.current.canCloneBackpackItem).toBe(true));

    await act(async () => {
      await result.current.handleBackpackClone();
    });
    act(() => {
      result.current.setItemIndex(1);
    });
    await act(async () => {
      await result.current.handleBackpackRemove();
    });

    expect(result.current.planCommands()).toEqual({
      clones: [{ tempIndex: -1, sourceIndex: 0, batch: [] }],
      removes: [1],
      batch: [],
    });
  });

  it("discards queued backpack structure changes on reset", async () => {
    mocks.executeCommand.mockImplementation(async (command: { command: string }) => {
      if (command.command === "list_backpack_items") {
        return {
          result: "items",
          items: [
            indexedItem(0, { name: "Sword" }),
            indexedItem(1, { name: "Shield" }),
          ],
        };
      }
      return { result: "summary", summary: summary({ dirty: true }) };
    });
    const { result } = renderHook(() =>
      useInventoryEditor({
        summary: summary(),
        container: "backpack",
        isBackpackInventory: true,
        run,
        setError: vi.fn(),
      }),
    );

    await act(async () => {
      await result.current.refreshItems();
    });
    await waitFor(() => expect(result.current.items.map((entry) => entry.index)).toEqual([0, 1]));

    await act(async () => {
      await result.current.handleBackpackRemove();
    });
    expect(result.current.items.map((entry) => entry.index)).toEqual([1]);

    act(() => {
      result.current.resetToCommittedDrafts();
    });

    expect(result.current.items.map((entry) => entry.index)).toEqual([0, 1]);

    expect(result.current.planCommands()).toEqual({ clones: [], removes: [], batch: [] });
  });

  it("plans cached drafts for multiple selected items", async () => {
    const backpackItems = [
      indexedItem(0, { name: "Potion", item_stacksize: 3 }),
      indexedItem(1, { name: "Bomb", item_stacksize: 2 }),
    ];
    mocks.executeCommand.mockImplementation(async (command: { command: string; index?: number; stack_size?: number }) => {
      if (command.command === "list_backpack_items") {
        return { result: "items", items: backpackItems };
      }
      if (command.command === "set_backpack_item_stack_size") {
        return {
          result: "item",
          container: "backpack",
          index: command.index,
          item: item({ item_stacksize: command.stack_size }),
        };
      }
      if (command.command === "set_money") {
        return { result: "summary", summary: summary({ money: 100, dirty: true }) };
      }
      return { result: "item", container: "backpack", index: command.index ?? 0, item: item() };
    });
    const { result } = renderHook(() =>
      useInventoryEditor({
        summary: summary(),
        container: "backpack",
        isBackpackInventory: true,
        run,
        setError: vi.fn(),
      }),
    );

    await act(async () => {
      await result.current.refreshItems();
    });
    await waitFor(() => expect(result.current.itemIndex).toBe(0));

    act(() => {
      result.current.setItemMetadataDraft((current) => ({ ...current, stack_size: "12" }));
      result.current.setItemIndex(1);
    });
    await waitFor(() => expect(result.current.itemMetadataDraft.stack_size).toBe("2"));
    act(() => {
      result.current.setItemMetadataDraft((current) => ({ ...current, stack_size: "8" }));
    });

    expect(result.current.planCommands()).toEqual({
      clones: [],
      removes: [],
      batch: [
        { command: "set_backpack_item_stack_size", index: 0, stack_size: 12 },
        { command: "set_backpack_item_stack_size", index: 1, stack_size: 8 },
      ],
    });
  });

  it("removes properties by original backend index when hidden properties exist", async () => {
    mocks.executeCommand.mockImplementation(async (command: { command: string }) => {
      if (command.command === "list_backpack_items") {
        return {
          result: "items",
          items: [
            indexedItem(0, {
              properties: [
                { id: 1, name: "(Base Item): Weapon", power: 0 },
                { id: 7, name: "Damage", power: 1 },
                { id: 2, name: "(Damage Type): Fire", power: 0 },
              ],
            }),
          ],
        };
      }
      return { result: "item", container: "backpack", index: 0, item: item() };
    });
    const { result } = renderHook(() =>
      useInventoryEditor({
        summary: summary(),
        container: "backpack",
        isBackpackInventory: true,
        run,
        setError: vi.fn(),
      }),
    );

    await act(async () => {
      await result.current.refreshItems();
    });
    await waitFor(() => expect(result.current.itemPropertiesDraft).toHaveLength(3));

    act(() => {
      result.current.handlePropertyRemoveDraft(1);
    });
    expect(result.current.planCommands()).toEqual({
      clones: [],
      removes: [],
      batch: [{ command: "remove_item_property", container: "backpack", index: 0, property_index: 1 }],
    });
  });
});
