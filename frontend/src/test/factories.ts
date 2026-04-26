import type { Ability, Character, CraftingRecipe, IndexedItem, Item, SaveSummary } from "../types";

export function summary(overrides: Partial<SaveSummary> = {}): SaveSummary {
  return {
    source_path: "C:/saves/save.das",
    dirty: false,
    preferred_game: "dao",
    money: 100,
    main_character_name: "Hero",
    companion_count: 1,
    backpack_count: 1,
    ...overrides,
  };
}

export function ability(id: number, overrides: Partial<Ability> = {}): Ability {
  return {
    id,
    name: `Ability ${id}`,
    tree: null,
    ability_type: null,
    core_ids: [],
    ...overrides,
  };
}

export function character(overrides: Partial<Character> = {}): Character {
  return {
    name: "Hero",
    template_resref: null,
    approval: 0,
    level: 1,
    experience: 10,
    core_stats: {
      strength: 10,
      dexterity: 11,
      willpower: 12,
      magic: 13,
      cunning: 14,
      constitution: 15,
    },
    point_pools: {
      attribute_points: 1,
      skill_points: 2,
      talent_points: 3,
      specialization_points: 4,
    },
    equipment: [],
    skills: [ability(4001)],
    talents: [ability(100)],
    spells: [ability(200)],
    ...overrides,
  };
}

export function item(overrides: Partial<Item> = {}): Item {
  return {
    resref: "gen_swd",
    name: "Sword",
    wiki_url: null,
    category: { value: "weapon", label: "Weapon" },
    stackable: true,
    object_id: null,
    equipment_slot: null,
    item_cost: 5,
    item_stacksize: 3,
    item_level: 1,
    material: 1,
    material_profile: { family: "metal", target: "weapon" },
    material_info: null,
    material_options: [{ code: 1, tier: 1, name: "Iron", family: "metal", target: "weapon" }],
    properties: [{ id: 7, name: "Damage", power: 1 }],
    ...overrides,
  };
}

export function indexedItem(index = 0, overrides: Partial<Item> = {}): IndexedItem {
  return { index, item: item(overrides) };
}

export function recipe(id: number, name = `Recipe ${id}`, category = "Potions"): CraftingRecipe {
  return { id, name, category };
}
