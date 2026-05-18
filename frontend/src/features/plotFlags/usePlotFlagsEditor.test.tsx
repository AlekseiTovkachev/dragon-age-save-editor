import { act, renderHook } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import type { SaveCommandResult } from "../../types";
import { usePlotFlagsEditor } from "./usePlotFlagsEditor";

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

describe("usePlotFlagsEditor", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.executeCommand.mockImplementation(async (command: { command: string }) => {
      if (command.command === "list_plot_flags") {
        return {
          result: "plot_flags",
          booleans: [{ id: 1, value: false }],
          integers: [{ id: 10, value: 0 }],
        };
      }
      if (command.command === "list_available_plot_flags") {
        return {
          result: "available_plot_flags",
          booleans: [{ id: 1, name: "bool_a", description: "A", category: "Act 1" }],
          integers: [
            {
              id: 10,
              name: "choice_a",
              description: "Choice",
              category: "Act 1",
              options: [{ value: 2, label: "Picked" }],
            },
          ],
        };
      }
      return {
        result: "plot_flags",
        booleans: [{ id: 1, value: true }],
        integers: [{ id: 10, value: 2 }],
      };
    });
  });

  it("plans patch payloads from boolean and integer drafts", async () => {
    const { result } = renderHook(() => usePlotFlagsEditor());

    await act(async () => {
      await result.current.refreshPlotFlags();
      await result.current.refreshAvailablePlotFlags();
    });
    act(() => {
      result.current.handleBooleanToggle(1, true);
      result.current.handleIntegerChange(10, 2);
    });
    expect(result.current.planCommands()).toEqual({
      batch: [{
        command: "patch_plot_flags",
        booleans: [{ id: 1, value: true }],
        integers: [{ id: 10, value: 2 }],
      }],
    });
  });

  it("plans plot flag patch commands from drafts", async () => {
    const { result } = renderHook(() => usePlotFlagsEditor());

    await act(async () => {
      await result.current.refreshPlotFlags();
      await result.current.refreshAvailablePlotFlags();
    });
    act(() => {
      result.current.handleBooleanToggle(1, true);
      result.current.handleIntegerChange(10, 2);
    });

    expect(result.current.planCommands()).toEqual({
      batch: [{
        command: "patch_plot_flags",
        booleans: [{ id: 1, value: true }],
        integers: [{ id: 10, value: 2 }],
      }],
    });
  });
});
