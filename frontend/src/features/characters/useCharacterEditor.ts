/* eslint-disable react-hooks/set-state-in-effect -- Character form drafts intentionally mirror the loaded character snapshot. */
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import type { SetStateAction } from "react";
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
  SaveCommand,
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

const EMPTY_DRAFT: CharacterDraft = {
  statsDraft: {},
  levelDraft: "",
  experienceDraft: "",
  approvalDraft: "",
  pointPoolsDraft: {},
  abilityDrafts: EMPTY_ABILITIES,
};

type CharacterDraftMap = Record<string, CharacterDraft>;
type DraftSetter<T> = SetStateAction<T>;
type CharacterCommandPlan = {
  batch: SaveCommand[];
};

const cloneCharacterDraft = (draft: CharacterDraft): CharacterDraft => ({
  statsDraft: { ...draft.statsDraft },
  levelDraft: draft.levelDraft,
  experienceDraft: draft.experienceDraft,
  approvalDraft: draft.approvalDraft,
  pointPoolsDraft: { ...draft.pointPoolsDraft },
  abilityDrafts: {
    skills: cloneAbilities(draft.abilityDrafts.skills),
    talents: cloneAbilities(draft.abilityDrafts.talents),
    spells: cloneAbilities(draft.abilityDrafts.spells),
  },
});

const cloneCharacterDraftMap = (map: CharacterDraftMap): CharacterDraftMap =>
  Object.fromEntries(Object.entries(map).map(([key, draft]) => [key, cloneCharacterDraft(draft)]));

const draftFromCharacter = (source: Character): CharacterDraft => ({
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
});

export function useCharacterEditor({ summary, run, refreshSummary }: UseCharacterEditorOptions) {
  const [characters, setCharacters] = useState<CharacterSummary[]>([]);
  const [characterKey, setCharacterKey] = useState("main");
  const [loadedCharacters, setLoadedCharacters] = useState<Record<string, Character>>({});
  const [drafts, setDrafts] = useState<CharacterDraftMap>({});
  const [availableAbilities, setAvailableAbilities] = useState<Record<AbilityListKind, Ability[]>>(EMPTY_ABILITIES);
  const [selectedAbilityToAdd, setSelectedAbilityToAdd] = useState<Record<AbilityListKind, string>>({
    skills: "",
    talents: "",
    spells: "",
  });
  const draftCheckpoint = useDraftCheckpoint<CharacterDraftMap>({ clone: cloneCharacterDraftMap });
  const draftsRef = useRef(drafts);

  useEffect(() => {
    draftsRef.current = drafts;
  }, [drafts]);

  const isDa2 = summary?.preferred_game === "da2";
  const selectedCharacterTarget = useMemo(
    () => characters.find((entry) => targetKey(entry.target) === characterKey)?.target ?? MAIN_TARGET,
    [characters, characterKey],
  );
  const visibleAbilityKinds = useMemo<AbilityListKind[]>(
    () => (isDa2 ? ["talents", "spells"] : ["skills", "talents", "spells"]),
    [isDa2],
  );

  const character = loadedCharacters[characterKey] ?? null;
  const currentDraft = drafts[characterKey] ?? EMPTY_DRAFT;
  const statsDraft = currentDraft.statsDraft;
  const levelDraft = currentDraft.levelDraft;
  const experienceDraft = currentDraft.experienceDraft;
  const approvalDraft = currentDraft.approvalDraft;
  const pointPoolsDraft = currentDraft.pointPoolsDraft;
  const abilityDrafts = currentDraft.abilityDrafts;

  const updateCurrentDraft = useCallback(
    (mutate: (current: CharacterDraft) => CharacterDraft) => {
      setDrafts((prev) => {
        const current = prev[characterKey] ? cloneCharacterDraft(prev[characterKey]) : cloneCharacterDraft(EMPTY_DRAFT);
        return { ...prev, [characterKey]: mutate(current) };
      });
    },
    [characterKey],
  );

  const applySetter = <T,>(setter: DraftSetter<T>, current: T): T =>
    typeof setter === "function" ? (setter as (value: T) => T)(current) : setter;

  const setStatsDraft = useCallback(
    (setter: DraftSetter<Record<string, string>>) =>
      updateCurrentDraft((draft) => ({ ...draft, statsDraft: applySetter(setter, draft.statsDraft) })),
    [updateCurrentDraft],
  );
  const setLevelDraft = useCallback(
    (setter: DraftSetter<string>) =>
      updateCurrentDraft((draft) => ({ ...draft, levelDraft: applySetter(setter, draft.levelDraft) })),
    [updateCurrentDraft],
  );
  const setExperienceDraft = useCallback(
    (setter: DraftSetter<string>) =>
      updateCurrentDraft((draft) => ({ ...draft, experienceDraft: applySetter(setter, draft.experienceDraft) })),
    [updateCurrentDraft],
  );
  const setApprovalDraft = useCallback(
    (setter: DraftSetter<string>) =>
      updateCurrentDraft((draft) => ({ ...draft, approvalDraft: applySetter(setter, draft.approvalDraft) })),
    [updateCurrentDraft],
  );
  const setPointPoolsDraft = useCallback(
    (setter: DraftSetter<Record<string, string>>) =>
      updateCurrentDraft((draft) => ({ ...draft, pointPoolsDraft: applySetter(setter, draft.pointPoolsDraft) })),
    [updateCurrentDraft],
  );
  const setAbilityDrafts = useCallback(
    (setter: DraftSetter<Record<AbilityListKind, Ability[]>>) =>
      updateCurrentDraft((draft) => ({ ...draft, abilityDrafts: applySetter(setter, draft.abilityDrafts) })),
    [updateCurrentDraft],
  );

  const seedDraftForCharacter = useCallback((key: string, source: Character) => {
    setDrafts((prev) => {
      if (prev[key]) {
        return prev;
      }
      return { ...prev, [key]: draftFromCharacter(source) };
    });
  }, []);

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
    const key = targetKey(target);
    setCharacterKey(key);
    setLoadedCharacters((prev) => ({ ...prev, [key]: response.character }));
    seedDraftForCharacter(key, response.character);
  }, [seedDraftForCharacter]);

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

  const commitPlannedDraftCommands = useCallback(async (includeAbilities: boolean) => {
    if (!character) {
      return false;
    }
    return run(async () => {
      const plannedCommands = planCharacterDraftCommands({
        target: selectedCharacterTarget,
        character,
        draft: currentDraft,
      }).filter((command) => includeAbilities || command.command !== "replace_ability_list");

      if (plannedCommands.length > 0) {
        await executeCommand({ command: "apply_batch", commands: plannedCommands });
        await loadCharacter(selectedCharacterTarget);
      }
      await refreshSummary();
    });
  }, [character, currentDraft, loadCharacter, refreshSummary, run, selectedCharacterTarget]);

  const planCommands = useCallback((): CharacterCommandPlan => {
    const targetsByKey = new Map(characters.map((entry) => [targetKey(entry.target), entry.target]));
    targetsByKey.set("main", MAIN_TARGET);
    const batch = Object.entries(drafts).flatMap(([key, draft]) => {
      const source = loadedCharacters[key];
      const target = targetsByKey.get(key);
      if (!source || !target) {
        return [];
      }
      return planCharacterDraftCommands({ target, character: source, draft });
    });
    return { batch };
  }, [characters, drafts, loadedCharacters]);

  const commitCharacterFields = useCallback(async () => {
    return commitPlannedDraftCommands(false);
  }, [commitPlannedDraftCommands]);

  const commitAbilityDrafts = useCallback(async () => {
    if (!character) {
      return false;
    }
    return run(async () => {
      const plannedCommands = planCharacterDraftCommands({
        target: selectedCharacterTarget,
        character,
        draft: currentDraft,
      }).filter((command) => command.command === "replace_ability_list");

      if (plannedCommands.length > 0) {
        await executeCommand({ command: "apply_batch", commands: plannedCommands });
        await loadCharacter(selectedCharacterTarget);
      }
      await refreshSummary();
    });
  }, [character, currentDraft, loadCharacter, refreshSummary, run, selectedCharacterTarget]);

  const checkpointDrafts = useCallback(() => {
    draftCheckpoint.checkpoint(draftsRef.current);
  }, [draftCheckpoint]);

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

  const resetToCommittedDrafts = useCallback(() => {
    const checkpoint = draftCheckpoint.reset();
    if (checkpoint) {
      setDrafts(checkpoint);
      return;
    }
    setDrafts(() => {
      const next: CharacterDraftMap = {};
      for (const [key, source] of Object.entries(loadedCharacters)) {
        next[key] = draftFromCharacter(source);
      }
      return next;
    });
  }, [draftCheckpoint, loadedCharacters]);

  const handleAbilityRemove = useCallback((list: AbilityListKind, abilityId: number) => {
    if (abilityIsLocked(list, abilityId, abilityDrafts)) {
      return;
    }
    setAbilityDrafts((current) => ({
      ...current,
      [list]: current[list].filter((ability) => ability.id !== abilityId),
    }));
  }, [abilityDrafts, setAbilityDrafts]);

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
  }, [abilityDrafts, availableAbilities, isDa2, selectedAbilityToAdd, setAbilityDrafts]);

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
  }, [abilityDrafts, availableAbilities, isDa2, setAbilityDrafts]);

  const clear = useCallback(() => {
    setCharacters([]);
    setCharacterKey("main");
    setLoadedCharacters({});
    setDrafts({});
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
    commitAbilityDrafts,
    abilityIsLocked: (list: AbilityListKind, abilityId: number) => abilityIsLocked(list, abilityId, abilityDrafts),
    handleAbilityRemove,
    handleAbilityAdd,
    visibleAbilities: (list: AbilityListKind, abilities: Ability[]) => visibleAbilities(isDa2, list, abilities),
    coreAbilityOptions: (list: AbilityListKind) => coreAbilityOptions(isDa2, list, availableAbilities, abilityDrafts),
    visibleTreeAbilities: (list: AbilityListKind) =>
      visibleTreeAbilities(isDa2, list, availableAbilities, abilityDrafts),
    handleVisibleAbilityAdd,
    planCommands,
    commitDrafts,
    resetToCommittedDrafts,
    clear,
  };
}
