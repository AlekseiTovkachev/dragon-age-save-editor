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
  return ability.id < 100000 && ability.id !== 700000;
}
