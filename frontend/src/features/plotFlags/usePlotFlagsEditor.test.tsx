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
          booleans: [{ id: 1, value: false }, { id: 2026, value: false }, { id: 2005, value: false }],
          integers: [{ id: 10, value: 0 }, { id: 1000, value: 1 }, { id: 1001, value: 2 }],
        };
      }
      if (command.command === "list_available_plot_flags") {
        return {
          result: "available_plot_flags",
          booleans: [
            { id: 1, name: "bool_a", description: "A", category: "Act 1" },
            { id: 2026, name: "alistair_warden", description: "Alistair and Warden", category: "Landsmeet" },
            { id: 2005, name: "human_noble", description: "Human Noble", category: "Warden" },
          ],
          integers: [
            {
              id: 10,
              name: "choice_a",
              description: "Choice",
              category: "Act 1",
              options: [{ value: 2, label: "Picked" }],
            },
            {
              id: 1000,
              name: "gender",
              description: "Gender",
              category: "Warden",
              options: [{ value: 1, label: "Male" }, { value: 2, label: "Female" }],
            },
            {
              id: 1001,
              name: "race",
              description: "Race",
              category: "Warden",
              options: [{ value: 2, label: "Elf" }, { value: 3, label: "Human" }],
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
        booleans: [{ id: 1, value: true }, { id: 2026, value: false }, { id: 2005, value: false }],
        integers: [{ id: 10, value: 2 }, { id: 1000, value: 1 }, { id: 1001, value: 2 }],
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
        booleans: [{ id: 1, value: true }, { id: 2026, value: false }, { id: 2005, value: false }],
        integers: [{ id: 10, value: 2 }, { id: 1000, value: 1 }, { id: 1001, value: 2 }],
      }],
    });
  });

  it("keeps direct user edits visible instead of applying frontend implications", async () => {
    const { result } = renderHook(() => usePlotFlagsEditor());

    await act(async () => {
      await result.current.refreshPlotFlags();
      await result.current.refreshAvailablePlotFlags();
    });
    act(() => {
      result.current.handleBooleanToggle(2026, true);
    });

    expect(result.current.plotBooleanDrafts[2005]).toBe(false);
    expect(result.current.plotIntegerDrafts[1000]).toBe(1);
    expect(result.current.plotIntegerDrafts[1001]).toBe(2);
    expect(result.current.hasPlotWarnings).toBe(true);
  });
});
