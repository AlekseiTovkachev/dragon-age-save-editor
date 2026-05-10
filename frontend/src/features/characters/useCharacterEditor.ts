/* eslint-disable react-hooks/set-state-in-effect -- Character form drafts intentionally mirror the loaded character snapshot. */
import { useCallback, useEffect, useMemo, useState } from "react";
import { executeCommand, expectResult } from "../../api";
import { useDraftCheckpoint } from "../../hooks/useDraftCheckpoint";
import {
  abilityIsLocked,
  abilityLabel,
  allKnownAbilities,
  cloneAbilities,
  coreAbilityOptions,
  isUselessDa2Talent,
  missingPrerequisiteChain,
  visibleAbilities,
  visibleTreeAbilities,
} from "../../lib/abilityUtils";
import { MAIN_TARGET, targetKey } from "../../lib/itemUtils";
import type {
  Ability,
  AbilityListKind,
  Character,
  CharacterSummary,
  CharacterTarget,
  SaveSummary,
} from "../../types";
import type { AsyncRun } from "../shared/types";
import { planCharacterDraftCommands } from "./characterCommandPlanner";
import type { CharacterDraft } from "./characterCommandPlanner";

type UseCharacterEditorOptions = {
  summary: SaveSummary | null;
  run: AsyncRun;
  refreshSummary: () => Promise<unknown>;
};

const EMPTY_ABILITIES: Record<AbilityListKind, Ability[]> = {
  skills: [],
  talents: [],
  spells: [],
};

type CharacterDraftCheckpoint = CharacterDraft;

const cloneCharacterCheckpoint = (checkpoint: CharacterDraftCheckpoint): CharacterDraftCheckpoint => ({
  statsDraft: { ...checkpoint.statsDraft },
  levelDraft: checkpoint.levelDraft,
  experienceDraft: checkpoint.experienceDraft,
  approvalDraft: checkpoint.approvalDraft,
  pointPoolsDraft: { ...checkpoint.pointPoolsDraft },
  abilityDrafts: {
    skills: cloneAbilities(checkpoint.abilityDrafts.skills),
    talents: cloneAbilities(checkpoint.abilityDrafts.talents),
    spells: cloneAbilities(checkpoint.abilityDrafts.spells),
  },
});

export function useCharacterEditor({ summary, run, refreshSummary }: UseCharacterEditorOptions) {
  const [characters, setCharacters] = useState<CharacterSummary[]>([]);
  const [characterKey, setCharacterKey] = useState("main");
  const [character, setCharacter] = useState<Character | null>(null);
  const [statsDraft, setStatsDraft] = useState<Record<string, string>>({});
  const [levelDraft, setLevelDraft] = useState("");
  const [experienceDraft, setExperienceDraft] = useState("");
  const [approvalDraft, setApprovalDraft] = useState("");
  const [pointPoolsDraft, setPointPoolsDraft] = useState<Record<string, string>>({});
  const [abilityDrafts, setAbilityDrafts] = useState<Record<AbilityListKind, Ability[]>>(EMPTY_ABILITIES);
  const [availableAbilities, setAvailableAbilities] = useState<Record<AbilityListKind, Ability[]>>(EMPTY_ABILITIES);
  const [selectedAbilityToAdd, setSelectedAbilityToAdd] = useState<Record<AbilityListKind, string>>({
    skills: "",
    talents: "",
    spells: "",
  });
  const draftCheckpoint = useDraftCheckpoint<CharacterDraftCheckpoint>({ clone: cloneCharacterCheckpoint });

  const isDa2 = summary?.preferred_game === "da2";
  const selectedCharacterTarget = useMemo(
    () => characters.find((entry) => targetKey(entry.target) === characterKey)?.target ?? MAIN_TARGET,
    [characters, characterKey],
  );
  const visibleAbilityKinds = useMemo<AbilityListKind[]>(
    () => (isDa2 ? ["talents", "spells"] : ["skills", "talents", "spells"]),
    [isDa2],
  );

  const checkpointFromCharacter = useCallback((source: Character): CharacterDraftCheckpoint => ({
    statsDraft: {
      strength: source.core_stats.strength.toString(),
      dexterity: source.core_stats.dexterity.toString(),
      willpower: source.core_stats.willpower.toString(),
      magic: source.core_stats.magic.toString(),
      cunning: source.core_stats.cunning.toString(),
      constitution: source.core_stats.constitution.toString(),
    },
    levelDraft: source.level?.toString() ?? "",
    experienceDraft: source.experience?.toString() ?? "",
    approvalDraft: source.approval?.toString() ?? "",
    pointPoolsDraft: {
      attribute_points: source.point_pools.attribute_points?.toString() ?? "",
      skill_points: source.point_pools.skill_points?.toString() ?? "",
      talent_points: source.point_pools.talent_points?.toString() ?? "",
      specialization_points: source.point_pools.specialization_points?.toString() ?? "",
    },
    abilityDrafts: {
      skills: cloneAbilities(source.skills),
      talents: cloneAbilities(source.talents),
      spells: cloneAbilities(source.spells),
    },
  }), []);

  const applyCheckpoint = useCallback((checkpoint: CharacterDraftCheckpoint) => {
    setStatsDraft({ ...checkpoint.statsDraft });
    setLevelDraft(checkpoint.levelDraft);
    setExperienceDraft(checkpoint.experienceDraft);
    setApprovalDraft(checkpoint.approvalDraft);
    setPointPoolsDraft({ ...checkpoint.pointPoolsDraft });
    setAbilityDrafts({
      skills: cloneAbilities(checkpoint.abilityDrafts.skills),
      talents: cloneAbilities(checkpoint.abilityDrafts.talents),
      spells: cloneAbilities(checkpoint.abilityDrafts.spells),
    });
  }, []);

  const syncCharacterDrafts = useCallback((source: Character) => {
    const checkpoint = checkpointFromCharacter(source);
    draftCheckpoint.checkpoint(checkpoint);
    applyCheckpoint(checkpoint);
  }, [applyCheckpoint, checkpointFromCharacter, draftCheckpoint]);

  const checkpointDrafts = useCallback(() => {
    draftCheckpoint.checkpoint({
      statsDraft: { ...statsDraft },
      levelDraft,
      experienceDraft,
      approvalDraft,
      pointPoolsDraft: { ...pointPoolsDraft },
      abilityDrafts: {
        skills: cloneAbilities(abilityDrafts.skills),
        talents: cloneAbilities(abilityDrafts.talents),
        spells: cloneAbilities(abilityDrafts.spells),
      },
    });
  }, [abilityDrafts, approvalDraft, draftCheckpoint, experienceDraft, levelDraft, pointPoolsDraft, statsDraft]);

  const resetToCommittedDrafts = useCallback(() => {
    const checkpoint = draftCheckpoint.reset();
    if (checkpoint) {
      applyCheckpoint(checkpoint);
    }
  }, [applyCheckpoint, draftCheckpoint]);

  const resetLoadedDrafts = useCallback(() => {
    if (character) {
      syncCharacterDrafts(character);
    }
  }, [character, syncCharacterDrafts]);

  useEffect(() => {
    if (character) {
      syncCharacterDrafts(character);
    }
  }, [character, syncCharacterDrafts]);

  const refreshCharacters = useCallback(async () => {
    const response = expectResult(await executeCommand({ command: "list_characters" }), "characters");
    setCharacters(response.characters);
    if (response.characters[0]) {
      setCharacterKey((current) =>
        response.characters.some((entry) => targetKey(entry.target) === current)
          ? current
          : targetKey(response.characters[0].target),
      );
    }
  }, []);

  const loadCharacter = useCallback(async (target: CharacterTarget) => {
    const response = expectResult(await executeCommand({ command: "get_character", target }), "character");
    setCharacter(response.character);
  }, []);

  useEffect(() => {
    if (summary) {
      void loadCharacter(selectedCharacterTarget);
    }
  }, [loadCharacter, selectedCharacterTarget, summary]);

  const refreshAvailableAbilities = useCallback(async (preferredGame: SaveSummary["preferred_game"]) => {
    for (const list of ["skills", "talents", "spells"] as AbilityListKind[]) {
      const response = expectResult(
        await executeCommand({ command: "list_available_abilities", list }),
        "available_abilities",
      );
      const sortedAbilities = response.abilities
        .filter((ability) => !(preferredGame === "da2" && list === "talents" && isUselessDa2Talent(ability)))
        .sort((left, right) =>
          abilityLabel(left).localeCompare(abilityLabel(right), undefined, { sensitivity: "base" }),
        );
      setAvailableAbilities((current) => ({ ...current, [list]: sortedAbilities }));
      setSelectedAbilityToAdd((current) => ({
        ...current,
        [list]: sortedAbilities[0] ? sortedAbilities[0].id.toString() : "",
      }));
    }
  }, []);

  const currentDraft = useCallback((): CharacterDraft => ({
    statsDraft,
    levelDraft,
    experienceDraft,
    approvalDraft,
    pointPoolsDraft,
    abilityDrafts,
  }), [abilityDrafts, approvalDraft, experienceDraft, levelDraft, pointPoolsDraft, statsDraft]);

  const commitPlannedDraftCommands = useCallback(async (includeAbilities: boolean) => {
    if (!character) {
      return false;
    }
    return run(async () => {
      const plannedCommands = planCharacterDraftCommands({
        target: selectedCharacterTarget,
        character,
        draft: currentDraft(),
      }).filter((command) => includeAbilities || command.command !== "replace_ability_list");

      if (plannedCommands.length > 0) {
        await executeCommand({ command: "apply_batch", commands: plannedCommands });
        await loadCharacter(selectedCharacterTarget);
      }
      await refreshSummary();
    });
  }, [character, currentDraft, loadCharacter, refreshSummary, run, selectedCharacterTarget]);

  const commitCharacterFields = useCallback(async () => {
    return commitPlannedDraftCommands(false);
  }, [commitPlannedDraftCommands]);

  const resetCharacterDraftToLoaded = useCallback(() => {
    resetLoadedDrafts();
  }, [resetLoadedDrafts]);

  const commitAbilityDrafts = useCallback(async () => {
    if (!character) {
      return false;
    }
    return run(async () => {
      const plannedCommands = planCharacterDraftCommands({
        target: selectedCharacterTarget,
        character,
        draft: currentDraft(),
      }).filter((command) => command.command === "replace_ability_list");

      if (plannedCommands.length > 0) {
        await executeCommand({ command: "apply_batch", commands: plannedCommands });
        await loadCharacter(selectedCharacterTarget);
      }
      await refreshSummary();
    });
  }, [character, currentDraft, loadCharacter, refreshSummary, run, selectedCharacterTarget]);

  const commitDrafts = useCallback(async () => {
    if (!await commitCharacterFields()) {
      return false;
    }
    if (!await commitAbilityDrafts()) {
      return false;
    }
    checkpointDrafts();
    return true;
  }, [checkpointDrafts, commitAbilityDrafts, commitCharacterFields]);

  const resetAbilityDrafts = useCallback(() => {
    resetLoadedDrafts();
  }, [resetLoadedDrafts]);

  const handleAbilityRemove = useCallback((list: AbilityListKind, abilityId: number) => {
    if (abilityIsLocked(list, abilityId, abilityDrafts)) {
      return;
    }
    setAbilityDrafts((current) => ({
      ...current,
      [list]: current[list].filter((ability) => ability.id !== abilityId),
    }));
  }, [abilityDrafts]);

  const handleAbilityAdd = useCallback((list: AbilityListKind) => {
    const options = coreAbilityOptions(isDa2, list, availableAbilities, abilityDrafts);
    const selectedId = Number(selectedAbilityToAdd[list]);
    const selected = options.find((ability) => ability.id === selectedId) ?? options[0];
    if (!selected) {
      return;
    }
    setAbilityDrafts((current) => {
      if (current[list].some((ability) => ability.id === selected.id)) {
        return current;
      }
      return {
        ...current,
        [list]: [...current[list], selected],
      };
    });
  }, [abilityDrafts, availableAbilities, isDa2, selectedAbilityToAdd]);

  const handleVisibleAbilityAdd = useCallback((list: AbilityListKind, abilityId: number) => {
    const known = allKnownAbilities(isDa2, list, availableAbilities, abilityDrafts);
    const knownById = new Map(known.map((ability) => [ability.id, ability]));
    const selected = knownById.get(abilityId);
    if (!selected) {
      return;
    }
    setAbilityDrafts((current) => {
      if (current[list].some((ability) => ability.id === selected.id)) {
        return current;
      }
      const selectedIds = new Set(current[list].map((ability) => ability.id));
      const chain = missingPrerequisiteChain(selected, knownById, selectedIds);
      const additions = [...chain, selected].filter((ability, index, abilities) => {
        return !selectedIds.has(ability.id) && abilities.findIndex((entry) => entry.id === ability.id) === index;
      });
      return {
        ...current,
        [list]: [...current[list], ...additions],
      };
    });
  }, [abilityDrafts, availableAbilities, isDa2]);

  const clear = useCallback(() => {
    setCharacters([]);
    setCharacterKey("main");
    setCharacter(null);
    setStatsDraft({});
    setLevelDraft("");
    setExperienceDraft("");
    setApprovalDraft("");
    setPointPoolsDraft({});
    setAbilityDrafts(EMPTY_ABILITIES);
    setAvailableAbilities(EMPTY_ABILITIES);
    setSelectedAbilityToAdd({ skills: "", talents: "", spells: "" });
    draftCheckpoint.clear();
  }, [draftCheckpoint]);

  return {
    characters,
    characterKey,
    setCharacterKey,
    character,
    selectedCharacterTarget,
    isDa2,
    statsDraft,
    setStatsDraft,
    levelDraft,
    setLevelDraft,
    experienceDraft,
    setExperienceDraft,
    approvalDraft,
    setApprovalDraft,
    pointPoolsDraft,
    setPointPoolsDraft,
    abilityDrafts,
    availableAbilities,
    selectedAbilityToAdd,
    setSelectedAbilityToAdd,
    visibleAbilityKinds,
    refreshCharacters,
    refreshAvailableAbilities,
    loadCharacter,
    commitCharacterFields,
    resetCharacterDraftToLoaded,
    commitAbilityDrafts,
    resetAbilityDrafts,
    abilityIsLocked: (list: AbilityListKind, abilityId: number) => abilityIsLocked(list, abilityId, abilityDrafts),
    handleAbilityRemove,
    handleAbilityAdd,
    visibleAbilities: (list: AbilityListKind, abilities: Ability[]) => visibleAbilities(isDa2, list, abilities),
    coreAbilityOptions: (list: AbilityListKind) => coreAbilityOptions(isDa2, list, availableAbilities, abilityDrafts),
    visibleTreeAbilities: (list: AbilityListKind) =>
      visibleTreeAbilities(isDa2, list, availableAbilities, abilityDrafts),
    handleVisibleAbilityAdd,
    commitDrafts,
    resetToCommittedDrafts,
    clear,
  };
}
