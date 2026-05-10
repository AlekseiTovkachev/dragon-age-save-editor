import type { Ability, AbilityListKind } from "../types";

export function abilityLabel(ability: Ability): string {
  const name = ability.name ?? `Ability ${ability.id}`;
  const parts = [name, `ID ${ability.id}`];
  if (ability.tree) {
    parts.push(ability.tree);
  }
  if (ability.ability_type) {
    parts.push(ability.ability_type);
  }
  return parts.join("  |  ");
}

export function cloneAbilities(abilities: Ability[]): Ability[] {
  return abilities.map((ability) => ({ ...ability, core_ids: [...ability.core_ids] }));
}

function isWeaponTalent(ability: Ability): boolean {
  return ["Archery", "Dual Weapon", "Two-Handed", "Weapon and Shield"].includes(ability.tree ?? "");
}

function abilityGroupLabel(list: AbilityListKind, ability: Ability, knownAbilities: Ability[]): string {
  if (ability.ability_type === "Class") {
    return "Class Unlocks";
  }
  if (list === "spells") {
    return ability.tree ? `${ability.tree} Spells` : "Other Spells";
  }
  if (list === "skills") {
    return ability.tree ?? ability.ability_type ?? "Other Skills";
  }
  if (isWeaponTalent(ability)) {
    return `${ability.tree} Talents`;
  }
  if (ability.ability_type === "Class") {
    return "Class Unlocks";
  }
  if (ability.ability_type === "Specialization") {
    return "Specialization Unlocks";
  }
  if (list === "talents" && ability.tree) {
    return `${ability.tree} Talents`;
  }

  const coreLabels = ability.core_ids
    .map((coreId) => knownAbilities.find((candidate) => candidate.id === coreId))
    .filter((candidate): candidate is Ability => Boolean(candidate))
    .map((candidate) => candidate.name ?? candidate.tree ?? `Core ${candidate.id}`);
  if (coreLabels.length > 0) {
    return coreLabels.join(" / ");
  }

  return ability.tree ?? "Other Talents";
}

export function groupedAbilities(
  list: AbilityListKind,
  abilities: Ability[],
  availableAbilities: Ability[],
): { label: string; abilities: Ability[] }[] {
  const knownAbilities = [...abilities, ...availableAbilities];
  const groups = new Map<string, Ability[]>();
  for (const ability of abilities) {
    const label = abilityGroupLabel(list, ability, knownAbilities);
    groups.set(label, [...(groups.get(label) ?? []), ability]);
  }
  return Array.from(groups, ([label, entries]) => ({ label, abilities: entries }));
}

export function isUselessDa2Talent(ability: Ability): boolean {
  return (ability.id < 100000 && ability.id !== 700000) || ability.id === 300000;
}

export function visibleAbilities(isDa2: boolean, list: AbilityListKind, abilities: Ability[]): Ability[] {
  if (isDa2 && list === "talents") {
    return abilities.filter((ability) => !isUselessDa2Talent(ability));
  }
  return abilities;
}

export function isCoreAbility(ability: Ability): boolean {
  return ability.core_ids.length === 0;
}

export function abilityIsLocked(
  list: AbilityListKind,
  abilityId: number,
  abilityDrafts: Record<AbilityListKind, Ability[]>,
): boolean {
  return abilityDrafts[list].some((ability) => {
    if (ability.id === abilityId || !ability.core_ids.includes(abilityId)) {
      return false;
    }
    return !ability.core_ids.some(
      (coreId) => coreId !== abilityId && abilityDrafts[list].some((candidate) => candidate.id === coreId),
    );
  });
}

export function allKnownAbilities(
  isDa2: boolean,
  list: AbilityListKind,
  availableAbilities: Record<AbilityListKind, Ability[]>,
  abilityDrafts: Record<AbilityListKind, Ability[]>,
): Ability[] {
  const byId = new Map<number, Ability>();
  for (const ability of visibleAbilities(isDa2, list, [...availableAbilities[list], ...abilityDrafts[list]])) {
    byId.set(ability.id, ability);
  }
  if (isDa2 && list === "talents") {
    // Drop game-internal ability IDs (e.g. 311002, 312002) that appear in TALENTLIST
    // but have no DB entry in any list. These are companion-specific passive triggers
    // that should never be displayed.
    const dbIds = new Set<number>();
    for (const abilities of Object.values(availableAbilities)) {
      for (const a of abilities) dbIds.add(a.id);
    }
    for (const id of byId.keys()) {
      if (!dbIds.has(id)) byId.delete(id);
    }
  }
  return Array.from(byId.values());
}

export function coreAbilityOptions(
  isDa2: boolean,
  list: AbilityListKind,
  availableAbilities: Record<AbilityListKind, Ability[]>,
  abilityDrafts: Record<AbilityListKind, Ability[]> = { skills: [], talents: [], spells: [] },
): Ability[] {
  const selectedIds = new Set(abilityDrafts[list].map((ability) => ability.id));
  return visibleAbilities(isDa2, list, availableAbilities[list]).filter(
    (ability) => isCoreAbility(ability) && !selectedIds.has(ability.id),
  );
}

function reachesSelectedAbility(
  ability: Ability,
  knownById: Map<number, Ability>,
  selectedIds: Set<number>,
  seen = new Set<number>(),
): boolean {
  if (seen.has(ability.id)) {
    return false;
  }
  seen.add(ability.id);
  return ability.core_ids.some((coreId) => {
    if (selectedIds.has(coreId)) {
      return true;
    }
    const core = knownById.get(coreId);
    return core ? reachesSelectedAbility(core, knownById, selectedIds, seen) : false;
  });
}

export function visibleTreeAbilities(
  isDa2: boolean,
  list: AbilityListKind,
  availableAbilities: Record<AbilityListKind, Ability[]>,
  abilityDrafts: Record<AbilityListKind, Ability[]>,
): Ability[] {
  const known = allKnownAbilities(isDa2, list, availableAbilities, abilityDrafts);
  const knownById = new Map(known.map((ability) => [ability.id, ability]));
  const selectedIds = new Set(abilityDrafts[list].map((ability) => ability.id));
  const selectedOrder = new Map(abilityDrafts[list].map((ability, index) => [ability.id, index]));
  return known
    .filter(
      (ability) =>
        selectedIds.has(ability.id) ||
        (!isCoreAbility(ability) && reachesSelectedAbility(ability, knownById, selectedIds)),
    )
    .sort((left, right) => {
      const leftSelected = selectedOrder.get(left.id);
      const rightSelected = selectedOrder.get(right.id);
      if (leftSelected !== undefined && rightSelected !== undefined) {
        return leftSelected - rightSelected;
      }
      if (leftSelected !== undefined) {
        return -1;
      }
      if (rightSelected !== undefined) {
        return 1;
      }
      return abilityLabel(left).localeCompare(abilityLabel(right), undefined, { sensitivity: "base" });
    });
}

export function missingPrerequisiteChain(
  ability: Ability,
  knownById: Map<number, Ability>,
  selectedIds: Set<number>,
  seen = new Set<number>(),
): Ability[] {
  if (ability.core_ids.length === 0 || seen.has(ability.id)) {
    return [];
  }
  seen.add(ability.id);

  if (ability.core_ids.some((coreId) => selectedIds.has(coreId))) {
    return [];
  }

  for (const coreId of ability.core_ids) {
    const core = knownById.get(coreId);
    if (!core) {
      continue;
    }
    const chain = missingPrerequisiteChain(core, knownById, selectedIds, new Set(seen));
    return [...chain, core].filter((candidate, index, candidates) => {
      return !selectedIds.has(candidate.id) && candidates.findIndex((entry) => entry.id === candidate.id) === index;
    });
  }

  return [];
}
