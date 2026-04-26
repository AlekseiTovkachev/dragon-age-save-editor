import { act, renderHook, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { character, indexedItem, recipe, summary } from "../../test/factories";
import type { SaveCommandResult } from "../../types";
import { useSaveEditorApp } from "./useSaveEditorApp";

const mocks = vi.hoisted(() => ({
  executeCommand: vi.fn(),
  hasDocument: vi.fn(),
  openDocument: vi.fn(),
  open: vi.fn(),
  save: vi.fn(),
}));

vi.mock("../../api", () => ({
  executeCommand: mocks.executeCommand,
  hasDocument: mocks.hasDocument,
  openDocument: mocks.openDocument,
  toErrorMessage: (caught: unknown) => caught instanceof Error ? caught.message : String(caught),
  expectResult: (response: SaveCommandResult, result: SaveCommandResult["result"]) => {
    if (response.result !== result) {
      throw new Error(`Expected ${result}, received ${response.result}`);
    }
    return response;
  },
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
  open: mocks.open,
  save: mocks.save,
}));

function installCommandResponses() {
  let currentCharacter = character();
  let currentItem = indexedItem().item;
  mocks.executeCommand.mockImplementation(async (command: { command: string; [key: string]: unknown }) => {
    switch (command.command) {
      case "validate":
        return { result: "validation", report: { is_valid: true, findings: [] } };
      case "get_summary":
        return { result: "summary", summary: summary() };
      case "get_document_assets":
        return { result: "document_assets", assets: { screenshot_data_url: "data:image/png;base64,abc" } };
      case "list_characters":
        return { result: "characters", characters: [{ target: "main_character", name: "Hero" }] };
      case "get_character":
        return { result: "character", target: "main_character", character: currentCharacter };
      case "list_available_abilities":
        return { result: "available_abilities", list: "talents", abilities: [] };
      case "list_available_item_properties":
        return { result: "available_item_properties", properties: [{ id: 7, name: "Damage" }] };
      case "list_crafting_recipes":
        return { result: "crafting_recipes", recipe_ids: [1] };
      case "list_available_crafting_recipes":
        return { result: "available_crafting_recipes", recipes: [recipe(1)] };
      case "list_plot_flags":
        return { result: "plot_flags", booleans: [], integers: [] };
      case "list_available_plot_flags":
        return { result: "available_plot_flags", booleans: [], integers: [] };
      case "list_backpack_items":
        return { result: "items", items: [{ index: 0, item: currentItem }] };
      case "set_money":
        return { result: "summary", summary: summary({ money: command.money as number, dirty: true }) };
      case "patch_core_stats":
      case "patch_point_pools":
      case "set_experience":
      case "set_approval":
        return { result: "character", target: "main_character", character: currentCharacter };
      case "set_level":
        currentCharacter = character({ level: command.level as number });
        return { result: "character", target: "main_character", character: currentCharacter };
      case "replace_ability_list":
        return { result: "character", target: "main_character", character: currentCharacter };
      case "replace_crafting_recipe_list":
        return { result: "crafting_recipes", recipe_ids: command.recipe_ids as number[] };
      case "patch_plot_flags":
        return { result: "plot_flags", booleans: command.booleans, integers: command.integers };
      case "set_backpack_item_stack_size":
        currentItem = { ...currentItem, item_stacksize: command.stack_size as number };
        return { result: "item", container: "backpack", index: command.index, item: currentItem };
      case "patch_item_metadata":
        currentItem = {
          ...currentItem,
          ...command.patch as Partial<typeof currentItem>,
        };
        return { result: "item", container: "backpack", index: command.index, item: currentItem };
      case "add_item_property":
      case "remove_item_property":
      case "set_item_property_id":
      case "set_item_property_power":
        return { result: "item", container: "backpack", index: command.index, item: currentItem };
      case "apply_batch":
        for (const nestedCommand of command.commands as Array<{ command: string; [key: string]: unknown }>) {
          await mocks.executeCommand(nestedCommand);
        }
        return { result: "summary", summary: summary({ dirty: true }) };
      case "save_as":
        return { result: "saved", output_path: "C:/out.das", summary: summary({ dirty: false }) };
      default:
        throw new Error(`unhandled command: ${command.command}`);
    }
  });
}

describe("useSaveEditorApp", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.hasDocument.mockResolvedValue(false);
    installCommandResponses();
  });

  it("opens, validates, hydrates feature data, and returns to the character overview", async () => {
    mocks.open.mockResolvedValue("C:/save.das");
    mocks.openDocument.mockResolvedValue(summary({ preferred_game: "da2" }));
    const { result } = renderHook(() => useSaveEditorApp());

    await act(async () => {
      await result.current.handleOpen();
    });

    expect(result.current.summary?.preferred_game).toBe("da2");
    expect(result.current.section).toBe("characters");
    expect(result.current.characterTab).toBe("overview");
    expect(mocks.executeCommand).toHaveBeenCalledWith({ command: "validate" });
    expect(mocks.executeCommand).toHaveBeenCalledWith({ command: "list_characters" });
    expect(mocks.executeCommand).toHaveBeenCalledWith({ command: "list_available_item_properties" });
  });

  it("clears loaded state when open validation fails", async () => {
    mocks.open.mockResolvedValue("C:/bad.das");
    mocks.openDocument.mockResolvedValue(summary());
    mocks.executeCommand.mockImplementation(async (command: { command: string }) => {
      if (command.command === "validate") {
        return { result: "validation", report: { is_valid: false, findings: [] } };
      }
      return { result: "summary", summary: summary() };
    });
    const { result } = renderHook(() => useSaveEditorApp());

    await act(async () => {
      await result.current.handleOpen();
    });

    expect(result.current.summary).toBeNull();
    expect(result.current.operation.error).toContain("invalid save structure");
  });

  it("hydrates an existing document on mount", async () => {
    mocks.hasDocument.mockResolvedValue(true);
    const { result } = renderHook(() => useSaveEditorApp());

    await waitFor(() => expect(result.current.summary).not.toBeNull());

    expect(mocks.executeCommand).toHaveBeenCalledWith({ command: "get_summary" });
    expect(mocks.executeCommand).toHaveBeenCalledWith({ command: "get_document_assets" });
    expect(mocks.executeCommand).toHaveBeenCalledWith({ command: "list_characters" });
  });

  it("resets drafts to the initial hydration baseline", async () => {
    mocks.open.mockResolvedValue("C:/save.das");
    mocks.openDocument.mockResolvedValue(summary());
    const { result } = renderHook(() => useSaveEditorApp());

    await act(async () => {
      await result.current.handleOpen();
    });
    await waitFor(() => expect(result.current.characterPanel.state.levelDraft).toBe("1"));

    act(() => {
      result.current.characterPanel.actions.setLevelDraft("9");
    });
    expect(result.current.characterPanel.state.levelDraft).toBe("9");

    act(() => {
      result.current.resetToCommittedDrafts();
    });

    expect(result.current.characterPanel.state.levelDraft).toBe("1");
  });

  it("commits frontend draft values to the backend and makes them the next reset baseline", async () => {
    mocks.open.mockResolvedValue("C:/save.das");
    mocks.openDocument.mockResolvedValue(summary());
    const { result } = renderHook(() => useSaveEditorApp());

    await act(async () => {
      await result.current.handleOpen();
    });
    await waitFor(() => expect(result.current.characterPanel.state.levelDraft).toBe("1"));
    mocks.executeCommand.mockClear();

    act(() => {
      result.current.characterPanel.actions.setLevelDraft("7");
    });
    await act(async () => {
      await result.current.commitDrafts();
    });
    act(() => {
      result.current.characterPanel.actions.setLevelDraft("8");
    });

    act(() => {
      result.current.resetToCommittedDrafts();
    });

    expect(result.current.characterPanel.state.levelDraft).toBe("7");
    expect(mocks.executeCommand).toHaveBeenCalledWith({
      command: "apply_batch",
      commands: [{ command: "set_level", target: "main_character", level: 7 }],
    });
  });

  it("does not checkpoint drafts when a commit command fails", async () => {
    mocks.open.mockResolvedValue("C:/save.das");
    mocks.openDocument.mockResolvedValue(summary());
    const { result } = renderHook(() => useSaveEditorApp());

    await act(async () => {
      await result.current.handleOpen();
    });
    await waitFor(() => expect(result.current.characterPanel.state.levelDraft).toBe("1"));
    mocks.executeCommand.mockImplementation(async (command: { command: string }) => {
      if (command.command === "apply_batch") {
        throw new Error("level rejected");
      }
      if (command.command === "get_summary") {
        return { result: "summary", summary: summary() };
      }
      return { result: "character", target: "main_character", character: character() };
    });

    act(() => {
      result.current.characterPanel.actions.setLevelDraft("7");
    });
    await act(async () => {
      await result.current.commitDrafts();
    });
    act(() => {
      result.current.characterPanel.actions.setLevelDraft("8");
      result.current.resetToCommittedDrafts();
    });

    expect(result.current.operation.error).toBe("level rejected");
    expect(result.current.characterPanel.state.levelDraft).toBe("1");
  });

  it("resets money and item stack drafts without backend commands", async () => {
    mocks.open.mockResolvedValue("C:/save.das");
    mocks.openDocument.mockResolvedValue(summary({ money: 100 }));
    const { result } = renderHook(() => useSaveEditorApp());

    await act(async () => {
      await result.current.handleOpen();
    });
    act(() => {
      result.current.setSection("inventory");
    });
    await waitFor(() => expect(result.current.inventoryPanel.state.itemIndex).toBe(0));
    mocks.executeCommand.mockClear();

    act(() => {
      result.current.inventoryPanel.actions.setMoneyDraft("999");
      result.current.inventoryPanel.actions.setItemMetadataDraft((current) => ({ ...current, stack_size: "42" }));
    });
    act(() => {
      result.current.resetToCommittedDrafts();
    });

    expect(result.current.inventoryPanel.state.moneyDraft).toBe("100");
    expect(result.current.inventoryPanel.state.itemMetadataDraft.stack_size).toBe("3");
    expect(mocks.executeCommand).not.toHaveBeenCalled();
  });

  it("commits money and item stack drafts to the backend", async () => {
    mocks.open.mockResolvedValue("C:/save.das");
    mocks.openDocument.mockResolvedValue(summary({ money: 100 }));
    const { result } = renderHook(() => useSaveEditorApp());

    await act(async () => {
      await result.current.handleOpen();
    });
    act(() => {
      result.current.setSection("inventory");
    });
    await waitFor(() => expect(result.current.inventoryPanel.state.itemIndex).toBe(0));
    mocks.executeCommand.mockClear();

    act(() => {
      result.current.inventoryPanel.actions.setMoneyDraft("999");
      result.current.inventoryPanel.actions.setItemMetadataDraft((current) => ({ ...current, stack_size: "42" }));
    });
    await act(async () => {
      await result.current.commitDrafts();
    });

    expect(mocks.executeCommand).toHaveBeenCalledWith({ command: "set_money", money: 999 });
    expect(mocks.executeCommand).toHaveBeenCalledWith({
      command: "apply_batch",
      commands: [{ command: "set_backpack_item_stack_size", index: 0, stack_size: 42 }],
    });
  });
});
