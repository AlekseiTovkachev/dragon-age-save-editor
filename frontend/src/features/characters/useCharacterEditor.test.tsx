import { act, renderHook, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ability, character } from "../../test/factories";
import type { SaveCommandResult } from "../../types";
import { useCharacterEditor } from "./useCharacterEditor";

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

describe("useCharacterEditor", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.executeCommand.mockImplementation(async (command: { command: string }) => {
      if (command.command === "get_character") {
        return { result: "character", target: "main_character", character: character() };
      }
      if (command.command === "list_available_abilities") {
        return { result: "available_abilities", list: "talents", abilities: [] };
      }
      if (command.command === "replace_ability_list") {
        return { result: "character", target: "main_character", character: character() };
      }
      return { result: "character", target: "main_character", character: character() };
    });
  });

  it("commits character overview edits in command order", async () => {
    const refreshSummary = vi.fn(async () => undefined);
    const { result } = renderHook(() =>
      useCharacterEditor({ summary: null, run, refreshSummary }),
    );

    await act(async () => {
      await result.current.loadCharacter("main_character");
    });
    await waitFor(() => expect(result.current.character).not.toBeNull());

    act(() => {
      result.current.setStatsDraft((current) => ({ ...current, strength: "21" }));
      result.current.setLevelDraft("5");
      result.current.setExperienceDraft("1234");
      result.current.setPointPoolsDraft((current) => ({ ...current, talent_points: "9" }));
      result.current.setApprovalDraft("17");
    });

    await act(async () => {
      await result.current.commitCharacterFields();
    });

    expect(mocks.executeCommand).toHaveBeenCalledWith({
      command: "apply_batch",
      commands: [
        { command: "patch_core_stats", target: "main_character", patch: { strength: 21 } },
        { command: "set_level", target: "main_character", level: 5 },
        { command: "set_experience", target: "main_character", experience: 1234 },
        { command: "patch_point_pools", target: "main_character", patch: { talent_points: 9 } },
      ],
    });
    const commands = mocks.executeCommand.mock.calls.map(([command]) => command);
    expect(JSON.stringify(commands)).not.toContain("set_approval");
    expect(refreshSummary).toHaveBeenCalledTimes(1);
  });

  it("commits approval edits for companions", async () => {
    const companionTarget = { companion: { index: 0 } } as const;
    mocks.executeCommand.mockImplementation(async (command: { command: string }) => {
      if (command.command === "list_characters") {
        return { result: "characters", characters: [{ target: companionTarget, name: "Morrigan" }] };
      }
      if (command.command === "get_character") {
        return { result: "character", target: companionTarget, character: character({ approval: 10 }) };
      }
      return { result: "character", target: companionTarget, character: character({ approval: 10 }) };
    });
    const refreshSummary = vi.fn(async () => undefined);
    const { result } = renderHook(() =>
      useCharacterEditor({ summary: null, run, refreshSummary }),
    );

    await act(async () => {
      await result.current.refreshCharacters();
    });
    await waitFor(() => expect(result.current.selectedCharacterTarget).toEqual(companionTarget));
    await act(async () => {
      await result.current.loadCharacter(result.current.selectedCharacterTarget);
    });
    await waitFor(() => expect(result.current.character).not.toBeNull());

    act(() => {
      result.current.setApprovalDraft("17");
    });

    await act(async () => {
      await result.current.commitCharacterFields();
    });

    expect(mocks.executeCommand).toHaveBeenCalledWith({
      command: "apply_batch",
      commands: [{ command: "set_approval", target: companionTarget, approval: 17 }],
    });
  });

  it("hydrates character drafts as part of loading a character", async () => {
    const companionTarget = { companion: { index: 0 } } as const;
    mocks.executeCommand.mockResolvedValue({
      result: "character",
      target: companionTarget,
      character: character({ approval: 10, level: 8 }),
    });
    const refreshSummary = vi.fn(async () => undefined);
    const { result } = renderHook(() =>
      useCharacterEditor({ summary: null, run, refreshSummary }),
    );

    await act(async () => {
      await result.current.loadCharacter(companionTarget);
    });

    expect(result.current.character?.approval).toBe(10);
    expect(result.current.approvalDraft).toBe("10");
    expect(result.current.levelDraft).toBe("8");
  });

  it("skips ability replacement when ability drafts are unchanged", async () => {
    const loaded = character({
      skills: [ability(1)],
      talents: [ability(2)],
      spells: [ability(3)],
    });
    mocks.executeCommand.mockImplementation(async (command: { command: string }) => {
      if (command.command === "get_character") {
        return { result: "character", target: "main_character", character: loaded };
      }
      return { result: "character", target: "main_character", character: loaded };
    });
    const refreshSummary = vi.fn(async () => undefined);
    const { result } = renderHook(() =>
      useCharacterEditor({ summary: null, run, refreshSummary }),
    );

    await act(async () => {
      await result.current.loadCharacter("main_character");
    });
    await waitFor(() => expect(result.current.abilityDrafts.skills).toHaveLength(1));

    await act(async () => {
      await result.current.commitAbilityDrafts();
    });

    expect(mocks.executeCommand).not.toHaveBeenCalledWith(expect.objectContaining({ command: "apply_batch" }));
  });
});
