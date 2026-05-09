import { act, renderHook, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { indexedItem, item, summary } from "../../test/factories";
import type { SaveCommandResult } from "../../types";
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

  it("commits stack size, metadata, and property mutations in order", async () => {
    const refreshSummary = vi.fn(async () => summary());
    const setError = vi.fn();
    const { result } = renderHook(() =>
      useInventoryEditor({
        summary: summary(),
        container: "backpack",
        isBackpackInventory: true,
        run,
        setError,
        refreshSummary,
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

    await act(async () => {
      await result.current.commitInventoryItemDrafts();
    });

    expect(mocks.executeCommand).toHaveBeenCalledWith({
      command: "apply_batch",
      commands: [
        { command: "set_backpack_item_stack_size", index: 0, stack_size: 5 },
        { command: "patch_item_metadata", container: "backpack", index: 0, patch: { item_level: 2 } },
        { command: "set_item_property_id", container: "backpack", index: 0, property_index: 0, property_id: 8 },
        { command: "set_item_property_power", container: "backpack", index: 0, property_index: 0, power: 2.5 },
        { command: "add_item_property", container: "backpack", index: 0, property_id: 9, power: 3 },
      ],
    });
    expect(refreshSummary).toHaveBeenCalled();
  });

  it("reports invalid stack size before issuing edit commands", async () => {
    const setError = vi.fn();
    const { result } = renderHook(() =>
      useInventoryEditor({
        summary: summary(),
        container: "backpack",
        isBackpackInventory: true,
        run,
        setError,
        refreshSummary: vi.fn(async () => summary()),
      }),
    );

    await act(async () => {
      await result.current.refreshItems();
    });
    act(() => {
      result.current.setItemMetadataDraft((current) => ({ ...current, stack_size: "500" }));
    });

    await expect(act(async () => result.current.commitInventoryItemDrafts())).rejects.toThrow(
      "Stack size must be a whole number from 1 to 99.",
    );
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
        refreshSummary: vi.fn(async () => summary()),
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

  it("commits cached drafts for multiple selected items", async () => {
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
        refreshSummary: vi.fn(async () => summary()),
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

    await act(async () => {
      await result.current.commitDrafts();
    });

    expect(mocks.executeCommand).toHaveBeenCalledWith({
      command: "apply_batch",
      commands: [
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
        refreshSummary: vi.fn(async () => summary()),
      }),
    );

    await act(async () => {
      await result.current.refreshItems();
    });
    await waitFor(() => expect(result.current.itemPropertiesDraft).toHaveLength(3));

    act(() => {
      result.current.handlePropertyRemoveDraft(1);
    });
    await act(async () => {
      await result.current.commitInventoryItemDrafts();
    });

    expect(mocks.executeCommand).toHaveBeenCalledWith({
      command: "apply_batch",
      commands: [{ command: "remove_item_property", container: "backpack", index: 0, property_index: 1 }],
    });
  });
});
