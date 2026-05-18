import { act, renderHook } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import type { SaveCommandResult } from "../../types";
import { useCraftingEditor } from "./useCraftingEditor";

const mocks = vi.hoisted(() => ({
  executeCommand: vi.fn(),
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

const run = async (action: () => Promise<void>) => {
  await action();
  return true;
};

describe("useCraftingEditor", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.executeCommand.mockImplementation(async (command: { command: string }) => {
      if (command.command === "list_crafting_recipes") {
        return { result: "crafting_recipes", recipe_ids: [1, 2] };
      }
      return { result: "crafting_recipes", recipe_ids: [1, 2, 3] };
    });
  });

  it("plans recipe replacement commands from drafts", async () => {
    const { result } = renderHook(() =>
      useCraftingEditor({ run, refreshSummary: vi.fn(async () => undefined) }),
    );

    await act(async () => {
      await result.current.refreshCraftingRecipes();
    });
    act(() => {
      result.current.handleToggle(3, true);
    });

    expect(result.current.planCommands()).toEqual({
      batch: [{ command: "replace_crafting_recipe_list", recipe_ids: [1, 2, 3] }],
    });
  });
});
