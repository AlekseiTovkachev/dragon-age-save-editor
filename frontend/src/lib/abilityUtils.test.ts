import { describe, expect, it } from "vitest";
import {
  abilityIsLocked,
  coreAbilityOptions,
  groupedAbilities,
  isUselessDa2Talent,
  missingPrerequisiteChain,
  visibleAbilities,
} from "./abilityUtils";
import type { Ability, AbilityListKind } from "../types";

function ability(id: number, overrides: Partial<Ability> = {}): Ability {
  return {
    id,
    name: `Ability ${id}`,
    tree: null,
    ability_type: null,
    core_ids: [],
    ...overrides,
  };
}

const emptyDrafts: Record<AbilityListKind, Ability[]> = {
  skills: [],
  talents: [],
  spells: [],
};

describe("abilityUtils", () => {
  it("groups weapon talents by tree label", () => {
    const groups = groupedAbilities(
      "talents",
      [ability(1, { tree: "Archery" })],
      [],
    );

    expect(groups).toEqual([{ label: "Archery Talents", abilities: [expect.objectContaining({ id: 1 })] }]);
  });

  it("filters low DA2 talent placeholders except the preserved exception", () => {
    expect(isUselessDa2Talent(ability(1))).toBe(true);
    const abilities = [ability(1), ability(700000), ability(101000)];

    expect(visibleAbilities(true, "talents", abilities).map((entry) => entry.id)).toEqual([700000, 101000]);
  });

  it("locks core abilities required by selected dependents", () => {
    const drafts = {
      ...emptyDrafts,
      talents: [ability(10), ability(11, { core_ids: [10] })],
    };

    expect(abilityIsLocked("talents", 10, drafts)).toBe(true);
    expect(abilityIsLocked("talents", 11, drafts)).toBe(false);
  });

  it("builds missing prerequisite chains in dependency order", () => {
    const core = ability(10);
    const child = ability(11, { core_ids: [10] });
    const known = new Map([core, child].map((entry) => [entry.id, entry]));

    expect(missingPrerequisiteChain(child, known, new Set()).map((entry) => entry.id)).toEqual([10]);
  });

  it("hides already selected core abilities from add options", () => {
    const available = {
      ...emptyDrafts,
      talents: [ability(10), ability(11), ability(12, { core_ids: [10] })],
    };
    const drafts = {
      ...emptyDrafts,
      talents: [ability(10)],
    };

    expect(coreAbilityOptions(false, "talents", available, drafts).map((entry) => entry.id)).toEqual([11]);
  });
});
