import { act, renderHook, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { character, indexedItem, recipe, summary } from "../../test/factories";
import type {
  PlotBooleanFlag,
  PlotBooleanValue,
  PlotIntegerFlag,
  PlotIntegerValue,
  SaveCommand,
  SaveCommandResult,
} from "../../types";
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

function isBatchEditCommand(command: SaveCommand) {
  return [
    "set_money",
    "patch_core_stats",
    "patch_point_pools",
    "set_level",
    "set_experience",
    "set_approval",
    "replace_ability_list",
    "replace_crafting_recipe_list",
    "patch_plot_flags",
    "patch_item_metadata",
    "set_backpack_item_stack_size",
    "add_item_property",
    "remove_item_property",
    "set_item_property_power",
    "set_item_property_id",
  ].includes(command.command);
}

type CommandResponseOptions = {
  currentSummary?: ReturnType<typeof summary>;
  plotBooleans?: PlotBooleanValue[];
  plotIntegers?: PlotIntegerValue[];
  availablePlotBooleans?: PlotBooleanFlag[];
  availablePlotIntegers?: PlotIntegerFlag[];
};

function installCommandResponses(options: CommandResponseOptions = {}) {
  const currentSummary = options.currentSummary ?? summary();
  const plotBooleans = options.plotBooleans ?? [];
  const plotIntegers = options.plotIntegers ?? [];
  const availablePlotBooleans = options.availablePlotBooleans ?? [];
  const availablePlotIntegers = options.availablePlotIntegers ?? [];
  let currentCharacter = character();
  let currentBackpackItem = indexedItem().item;
  let currentEquipmentItem = indexedItem(0, { name: "Equipped Sword", stackable: false, item_stacksize: null }).item;
  mocks.executeCommand.mockImplementation(async (command: { command: string; [key: string]: unknown }) => {
    switch (command.command) {
      case "validate":
        return { result: "validation", report: { is_valid: true, findings: [] } };
      case "get_summary":
        return { result: "summary", summary: currentSummary };
      case "get_document_assets":
        return { result: "document_assets", assets: { screenshot_data_url: "data:image/png;base64,abc" } };
      case "list_characters":
        return {
          result: "characters",
          characters: [
            { target: "main_character", name: "Hero" },
            { target: { companion: { index: 0 } }, name: "Alistair" },
          ],
        };
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
        return { result: "plot_flags", booleans: plotBooleans, integers: plotIntegers };
      case "list_available_plot_flags":
        return { result: "available_plot_flags", booleans: availablePlotBooleans, integers: availablePlotIntegers };
      case "list_backpack_items":
        return { result: "items", items: [{ index: 0, item: currentBackpackItem }] };
      case "list_equipment_items":
        return { result: "items", items: [{ index: 0, item: currentEquipmentItem }] };
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
        currentBackpackItem = { ...currentBackpackItem, item_stacksize: command.stack_size as number };
        return { result: "item", container: "backpack", index: command.index, item: currentBackpackItem };
      case "patch_item_metadata":
        if (command.container === "backpack") {
          currentBackpackItem = {
            ...currentBackpackItem,
            ...command.patch as Partial<typeof currentBackpackItem>,
          };
          return { result: "item", container: "backpack", index: command.index, item: currentBackpackItem };
        }
        currentEquipmentItem = {
          ...currentEquipmentItem,
          ...command.patch as Partial<typeof currentEquipmentItem>,
        };
        return { result: "item", container: command.container, index: command.index, item: currentEquipmentItem };
      case "remove_backpack_item":
        return { result: "summary", summary: summary({ dirty: true, backpack_count: 0 }) };
      case "add_item_property":
      case "remove_item_property":
      case "set_item_property_id":
      case "set_item_property_power":
        return {
          result: "item",
          container: command.container,
          index: command.index,
          item: command.container === "backpack" ? currentBackpackItem : currentEquipmentItem,
        };
      case "apply_batch":
        for (const nestedCommand of command.commands as Array<{ command: string; [key: string]: unknown }>) {
          if (!isBatchEditCommand(nestedCommand as SaveCommand)) {
            throw new Error(`Command ${nestedCommand.command} is not supported in apply_batch`);
          }
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

  it("does not repeatedly rehydrate when the loaded state rerenders", async () => {
    mocks.open.mockResolvedValue("C:/save.das");
    mocks.openDocument.mockResolvedValue(summary());
    const { result } = renderHook(() => useSaveEditorApp());

    await act(async () => {
      await result.current.handleOpen();
    });
    await waitFor(() => expect(result.current.characterPanel.state.levelDraft).toBe("1"));
    mocks.executeCommand.mockClear();

    act(() => {
      result.current.characterPanel.actions.setLevelDraft("2");
    });
    act(() => {
      result.current.characterPanel.actions.setLevelDraft("3");
    });

    expect(mocks.executeCommand).not.toHaveBeenCalledWith({ command: "get_summary" });
    expect(mocks.executeCommand).not.toHaveBeenCalledWith({ command: "list_characters" });
  });

  it("skips DA2-only plot flag hydration for DAO saves", async () => {
    mocks.open.mockResolvedValue("C:/save.das");
    mocks.openDocument.mockResolvedValue(summary({ preferred_game: "dao" }));
    const { result } = renderHook(() => useSaveEditorApp());

    await act(async () => {
      await result.current.handleOpen();
    });

    expect(mocks.executeCommand).not.toHaveBeenCalledWith({ command: "list_plot_flags" });
    expect(mocks.executeCommand).not.toHaveBeenCalledWith({ command: "list_available_plot_flags" });
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
    expect(result.current.characterPanel.state.levelDraft).toBe("7");
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

    expect(mocks.executeCommand).toHaveBeenCalledWith({
      command: "apply_batch",
      commands: [
        { command: "set_money", money: 999 },
        { command: "set_backpack_item_stack_size", index: 0, stack_size: 42 },
      ],
    });
    expect(result.current.inventoryPanel.state.moneyDraft).toBe("999");
  });

  it("prompts before Save As when drafts are pending and cancels without saving", async () => {
    mocks.open.mockResolvedValue("C:/save.das");
    mocks.openDocument.mockResolvedValue(summary({ dirty: false }));
    mocks.save.mockResolvedValue("C:/out.das");
    const { result } = renderHook(() => useSaveEditorApp());

    await act(async () => {
      await result.current.handleOpen();
    });
    await waitFor(() => expect(result.current.characterPanel.state.levelDraft).toBe("1"));
    act(() => {
      result.current.characterPanel.actions.setLevelDraft("7");
    });

    await act(async () => {
      await result.current.handleSaveAs();
    });

    expect(result.current.saveAsPrompt.open).toBe(true);
    expect(mocks.save).not.toHaveBeenCalled();

    act(() => {
      result.current.saveAsPrompt.onCancel();
    });

    expect(result.current.saveAsPrompt.open).toBe(false);
    expect(result.current.characterPanel.state.levelDraft).toBe("7");
    expect(mocks.executeCommand).not.toHaveBeenCalledWith({ command: "save_as", output_path: "C:/out.das" });
  });

  it("applies pending drafts before Save As after confirmation", async () => {
    mocks.open.mockResolvedValue("C:/save.das");
    mocks.openDocument.mockResolvedValue(summary({ dirty: false }));
    mocks.save.mockResolvedValue("C:/out.das");
    const { result } = renderHook(() => useSaveEditorApp());

    await act(async () => {
      await result.current.handleOpen();
    });
    await waitFor(() => expect(result.current.characterPanel.state.levelDraft).toBe("1"));
    act(() => {
      result.current.characterPanel.actions.setLevelDraft("7");
    });

    await act(async () => {
      await result.current.handleSaveAs();
    });
    await act(async () => {
      await result.current.saveAsPrompt.onConfirm();
    });

    expect(mocks.executeCommand).toHaveBeenCalledWith({
      command: "apply_batch",
      commands: [{ command: "set_level", target: "main_character", level: 7 }],
    });
    expect(mocks.executeCommand).toHaveBeenCalledWith({ command: "save_as", output_path: "C:/out.das" });
    expect(result.current.saveAsPrompt.open).toBe(false);
  });

  it("blocks Save As confirmation when DA2 plot drafts have warnings", async () => {
    installCommandResponses({
      currentSummary: summary({ preferred_game: "da2", dirty: false }),
      plotBooleans: [{ id: 2005, value: false }],
      plotIntegers: [{ id: 1001, value: 3 }],
      availablePlotBooleans: [{ id: 2005, name: "Human Noble", description: "", category: "Warden" }],
      availablePlotIntegers: [{
        id: 1001,
        name: "Warden race",
        description: "",
        category: "Warden",
        options: [
          { value: 2, label: "Elf" },
          { value: 3, label: "Human" },
        ],
      }],
    });
    mocks.open.mockResolvedValue("C:/save.das");
    mocks.openDocument.mockResolvedValue(summary({ preferred_game: "da2", dirty: false }));
    mocks.save.mockResolvedValue("C:/out.das");
    const { result } = renderHook(() => useSaveEditorApp());

    await act(async () => {
      await result.current.handleOpen();
    });
    act(() => {
      result.current.plotFlagsPanel.actions.handleBooleanToggle(2005, true);
      result.current.plotFlagsPanel.actions.handleIntegerChange(1001, 2);
    });
    expect(result.current.hasPlotWarnings).toBe(true);

    await act(async () => {
      await result.current.handleSaveAs();
    });
    await act(async () => {
      await result.current.saveAsPrompt.onConfirm();
    });

    expect(result.current.operation.error).toContain("Resolve DA2 plot warnings");
    expect(mocks.save).not.toHaveBeenCalled();
    expect(mocks.executeCommand).not.toHaveBeenCalledWith(expect.objectContaining({ command: "apply_batch" }));
    expect(mocks.executeCommand).not.toHaveBeenCalledWith({ command: "save_as", output_path: "C:/out.das" });
    expect(result.current.saveAsPrompt.open).toBe(false);
  });

  it("aborts Save As when draft apply fails", async () => {
    mocks.open.mockResolvedValue("C:/save.das");
    mocks.openDocument.mockResolvedValue(summary({ dirty: false }));
    mocks.save.mockResolvedValue("C:/out.das");
    const { result } = renderHook(() => useSaveEditorApp());

    await act(async () => {
      await result.current.handleOpen();
    });
    await waitFor(() => expect(result.current.characterPanel.state.levelDraft).toBe("1"));
    act(() => {
      result.current.characterPanel.actions.setLevelDraft("7");
    });
    mocks.executeCommand.mockImplementation(async (command: { command: string }) => {
      if (command.command === "apply_batch") {
        throw new Error("draft rejected");
      }
      return { result: "validation", report: { is_valid: true, findings: [] } };
    });

    await act(async () => {
      await result.current.handleSaveAs();
    });
    await act(async () => {
      await result.current.saveAsPrompt.onConfirm();
    });

    expect(result.current.operation.error).toBe("draft rejected");
    expect(mocks.save).not.toHaveBeenCalled();
    expect(mocks.executeCommand).not.toHaveBeenCalledWith({ command: "save_as", output_path: "C:/out.das" });
  });

  it("applies cross-panel drafts in one merged batch", async () => {
    mocks.open.mockResolvedValue("C:/save.das");
    mocks.openDocument.mockResolvedValue(summary({ money: 100 }));
    const { result } = renderHook(() => useSaveEditorApp());

    await act(async () => {
      await result.current.handleOpen();
    });
    await waitFor(() => expect(result.current.characterPanel.state.levelDraft).toBe("1"));
    act(() => {
      result.current.setSection("inventory");
    });
    await waitFor(() => expect(result.current.inventoryPanel.state.itemIndex).toBe(0));
    mocks.executeCommand.mockClear();

    act(() => {
      result.current.characterPanel.actions.setLevelDraft("7");
      result.current.inventoryPanel.actions.setMoneyDraft("999");
      result.current.inventoryPanel.actions.setItemMetadataDraft((current) => ({ ...current, stack_size: "42" }));
    });
    await act(async () => {
      await result.current.commitDrafts();
    });

    const applyBatchCalls = mocks.executeCommand.mock.calls
      .map(([command]) => command)
      .filter((command) => command.command === "apply_batch");
    expect(applyBatchCalls[0]).toEqual({
      command: "apply_batch",
      commands: [
        { command: "set_level", target: "main_character", level: 7 },
        { command: "set_money", money: 999 },
        { command: "set_backpack_item_stack_size", index: 0, stack_size: 42 },
      ],
    });
  });

  it("resets drafts across panels in one action", async () => {
    mocks.open.mockResolvedValue("C:/save.das");
    mocks.openDocument.mockResolvedValue(summary({ money: 100 }));
    const { result } = renderHook(() => useSaveEditorApp());

    await act(async () => {
      await result.current.handleOpen();
    });
    await waitFor(() => expect(result.current.characterPanel.state.levelDraft).toBe("1"));
    act(() => {
      result.current.setSection("inventory");
    });
    await waitFor(() => expect(result.current.inventoryPanel.state.itemIndex).toBe(0));

    act(() => {
      result.current.characterPanel.actions.setLevelDraft("9");
      result.current.inventoryPanel.actions.setMoneyDraft("999");
      result.current.inventoryPanel.actions.setItemMetadataDraft((current) => ({ ...current, stack_size: "42" }));
      result.current.craftingPanel.actions.handleToggle(2, true);
    });
    act(() => {
      result.current.resetToCommittedDrafts();
    });

    expect(result.current.characterPanel.state.levelDraft).toBe("1");
    expect(result.current.inventoryPanel.state.moneyDraft).toBe("100");
    expect(result.current.inventoryPanel.state.itemMetadataDraft.stack_size).toBe("3");
    expect(result.current.craftingPanel.state.craftingRecipeDrafts).toEqual([1]);
  });

  it("applies cached backpack drafts after switching to equipment", async () => {
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

    act(() => {
      result.current.inventoryPanel.actions.setItemMetadataDraft((current) => ({ ...current, stack_size: "42" }));
      result.current.setSection("characters");
      result.current.setCharacterTab("equipment");
    });
    await waitFor(() => expect(result.current.inventoryPanel.state.itemMetadataDraft.item_level).toBe("1"));
    mocks.executeCommand.mockClear();

    act(() => {
      result.current.inventoryPanel.actions.setItemMetadataDraft((current) => ({ ...current, item_level: "5" }));
    });
    await act(async () => {
      await result.current.commitDrafts();
    });

    expect(mocks.executeCommand).toHaveBeenCalledWith({
      command: "apply_batch",
      commands: [
        { command: "set_backpack_item_stack_size", index: 0, stack_size: 42 },
        {
          command: "patch_item_metadata",
          container: { equipment: { target: "main_character" } },
          index: 0,
          patch: { item_level: 5 },
        },
      ],
    });
  });

  it("sends backpack removals outside apply_batch", async () => {
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

    await act(async () => {
      await result.current.inventoryPanel.actions.handleBackpackRemove();
    });
    mocks.executeCommand.mockClear();
    await act(async () => {
      await result.current.commitDrafts();
    });

    const applyBatchCalls = mocks.executeCommand.mock.calls
      .map(([command]) => command)
      .filter((command) => command.command === "apply_batch");
    expect(applyBatchCalls.flatMap((command) => command.commands)).not.toEqual(
      expect.arrayContaining([expect.objectContaining({ command: "remove_backpack_item" })]),
    );
    expect(mocks.executeCommand).toHaveBeenCalledWith({ command: "remove_backpack_item", index: 0 });
  });

  it("loads equipment items for the active character equipment tab", async () => {
    mocks.open.mockResolvedValue("C:/save.das");
    mocks.openDocument.mockResolvedValue(summary());
    const { result } = renderHook(() => useSaveEditorApp());

    await act(async () => {
      await result.current.handleOpen();
    });
    act(() => {
      result.current.setCharacterTab("equipment");
    });

    await waitFor(() =>
      expect(mocks.executeCommand).toHaveBeenCalledWith({
        command: "list_equipment_items",
        target: "main_character",
      }),
    );

    act(() => {
      result.current.characterPanel.actions.setCharacterKey("companion:0");
    });

    await waitFor(() =>
      expect(mocks.executeCommand).toHaveBeenCalledWith({
        command: "list_equipment_items",
        target: { companion: { index: 0 } },
      }),
    );
  });
});
