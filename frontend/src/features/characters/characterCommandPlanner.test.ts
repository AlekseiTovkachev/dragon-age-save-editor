import { describe, expect, it } from "vitest";
import { ability, character } from "../../test/factories";
import { planCharacterDraftCommands } from "./characterCommandPlanner";
import type { CharacterDraft } from "./characterCommandPlanner";

const draftFromCharacter = (overrides: Partial<CharacterDraft> = {}): CharacterDraft => ({
  statsDraft: {
    strength: "10",
    dexterity: "11",
    willpower: "12",
    magic: "13",
    cunning: "14",
    constitution: "15",
  },
  levelDraft: "1",
  experienceDraft: "10",
  approvalDraft: "0",
  pointPoolsDraft: {
    attribute_points: "1",
    skill_points: "2",
    talent_points: "3",
    specialization_points: "4",
  },
  abilityDrafts: {
    skills: [ability(4001)],
    talents: [ability(100)],
    spells: [ability(200)],
  },
  ...overrides,
});

describe("planCharacterDraftCommands", () => {
  it("plans no commands for unchanged main character drafts", () => {
    expect(planCharacterDraftCommands({
      target: "main_character",
      character: character(),
      draft: draftFromCharacter(),
    })).toEqual([]);
  });

  it("plans changed stats, progress, point pools, and abilities", () => {
    expect(planCharacterDraftCommands({
      target: "main_character",
      character: character(),
      draft: draftFromCharacter({
        statsDraft: { ...draftFromCharacter().statsDraft, strength: "21" },
        levelDraft: "5",
        experienceDraft: "1234",
        pointPoolsDraft: { ...draftFromCharacter().pointPoolsDraft, talent_points: "9" },
        abilityDrafts: {
          skills: [ability(4001), ability(4002)],
          talents: [ability(100)],
          spells: [ability(200)],
        },
      }),
    })).toEqual([
      { command: "patch_core_stats", target: "main_character", patch: { strength: 21 } },
      { command: "set_level", target: "main_character", level: 5 },
      { command: "set_experience", target: "main_character", experience: 1234 },
      { command: "patch_point_pools", target: "main_character", patch: { talent_points: 9 } },
      {
        command: "replace_ability_list",
        target: "main_character",
        list: "skills",
        ability_ids: [4001, 4002],
      },
    ]);
  });

  it("plans companion approval only for companions with approval rows", () => {
    const companionTarget = { companion: { index: 0 } } as const;
    expect(planCharacterDraftCommands({
      target: companionTarget,
      character: character({ approval: 10 }),
      draft: draftFromCharacter({ approvalDraft: "17" }),
    })).toContainEqual({ command: "set_approval", target: companionTarget, approval: 17 });

    expect(planCharacterDraftCommands({
      target: "main_character",
      character: character({ approval: 10 }),
      draft: draftFromCharacter({ approvalDraft: "17" }),
    })).not.toContainEqual({ command: "set_approval", target: "main_character", approval: 17 });
  });

  it("rejects invalid level, experience, and approval drafts", () => {
    expect(() => planCharacterDraftCommands({
      target: "main_character",
      character: character(),
      draft: draftFromCharacter({ levelDraft: "nope" }),
    })).toThrow("Level must be a valid number.");

    expect(() => planCharacterDraftCommands({
      target: "main_character",
      character: character(),
      draft: draftFromCharacter({ experienceDraft: "nope" }),
    })).toThrow("Experience must be a valid number.");

    expect(() => planCharacterDraftCommands({
      target: { companion: { index: 0 } },
      character: character({ approval: 10 }),
      draft: draftFromCharacter({ approvalDraft: "nope" }),
    })).toThrow("Approval must be a valid number.");
  });
});
