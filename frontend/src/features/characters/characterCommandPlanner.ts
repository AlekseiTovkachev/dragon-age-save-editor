import { parseNumber } from "../../lib/format";
import type { Ability, AbilityListKind, Character, CharacterTarget, SaveCommand } from "../../types";

export type CharacterDraft = {
  statsDraft: Record<string, string>;
  levelDraft: string;
  experienceDraft: string;
  approvalDraft: string;
  pointPoolsDraft: Record<string, string>;
  abilityDrafts: Record<AbilityListKind, Ability[]>;
};

export type CharacterDraftCommand = Extract<
  SaveCommand,
  | { command: "patch_core_stats" }
  | { command: "patch_point_pools" }
  | { command: "set_level" }
  | { command: "set_experience" }
  | { command: "set_approval" }
  | { command: "replace_ability_list" }
>;

type CharacterCommandPlanInput = {
  target: CharacterTarget;
  character: Character;
  draft: CharacterDraft;
};

const abilityKinds: AbilityListKind[] = ["skills", "talents", "spells"];

function abilityIds(abilities: Ability[]) {
  return abilities.map((ability) => ability.id);
}

function sameIds(left: number[], right: number[]) {
  return left.length === right.length && left.every((value, index) => value === right[index]);
}

export function planCharacterDraftCommands({
  target,
  character,
  draft,
}: CharacterCommandPlanInput): CharacterDraftCommand[] {
  const commands: CharacterDraftCommand[] = [];
  const coreStatsPatch: Extract<CharacterDraftCommand, { command: "patch_core_stats" }>["patch"] = {};

  for (const key of ["strength", "dexterity", "willpower", "magic", "cunning", "constitution"] as const) {
    const value = parseNumber(draft.statsDraft[key]);
    if (value !== null && character.core_stats[key] !== value) {
      coreStatsPatch[key] = value;
    }
  }
  if (Object.keys(coreStatsPatch).length > 0) {
    commands.push({ command: "patch_core_stats", target, patch: coreStatsPatch });
  }

  const level = parseNumber(draft.levelDraft);
  if (level === null) {
    throw new Error("Level must be a valid number.");
  }
  if (character.level !== level) {
    commands.push({ command: "set_level", target, level });
  }

  const experience = parseNumber(draft.experienceDraft);
  if (draft.experienceDraft.trim() !== "" && experience === null) {
    throw new Error("Experience must be a valid number.");
  }
  if (experience !== null && character.experience !== experience) {
    commands.push({ command: "set_experience", target, experience });
  }

  const pointPoolsPatch: Extract<CharacterDraftCommand, { command: "patch_point_pools" }>["patch"] = {};
  for (const key of ["attribute_points", "skill_points", "talent_points", "specialization_points"] as const) {
    const value = parseNumber(draft.pointPoolsDraft[key]);
    if (value !== null && character.point_pools[key] !== value) {
      pointPoolsPatch[key] = value;
    }
  }
  if (Object.keys(pointPoolsPatch).length > 0) {
    commands.push({ command: "patch_point_pools", target, patch: pointPoolsPatch });
  }

  if (target !== "main_character" && character.approval !== null) {
    const approval = parseNumber(draft.approvalDraft);
    if (approval === null) {
      throw new Error("Approval must be a valid number.");
    }
    if (character.approval !== approval) {
      commands.push({ command: "set_approval", target, approval });
    }
  }

  for (const list of abilityKinds) {
    const draftedIds = abilityIds(draft.abilityDrafts[list]);
    const loadedIds = abilityIds(character[list]);
    if (!sameIds(draftedIds, loadedIds)) {
      commands.push({
        command: "replace_ability_list",
        target,
        list,
        ability_ids: draftedIds,
      });
    }
  }

  return commands;
}
