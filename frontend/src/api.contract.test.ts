import { describe, expect, it } from "vitest";
import { expectResult } from "./api";
import type { CommandError, SaveCommand, SaveCommandResult } from "./types";

const commandFixtures = {
  validate: { command: "validate" },
  get_summary: { command: "get_summary" },
  get_document_assets: { command: "get_document_assets" },
  get_character: { command: "get_character", target: "main_character" },
  list_available_abilities: { command: "list_available_abilities", list: "talents" },
  list_available_item_properties: { command: "list_available_item_properties" },
  list_available_crafting_recipes: { command: "list_available_crafting_recipes" },
  list_available_plot_flags: { command: "list_available_plot_flags" },
  list_characters: { command: "list_characters" },
  list_backpack_items: { command: "list_backpack_items" },
  list_equipment_items: { command: "list_equipment_items", target: { companion: { index: 0 } } },
  list_crafting_recipes: { command: "list_crafting_recipes" },
  list_plot_flags: { command: "list_plot_flags" },
  set_money: { command: "set_money", money: 123 },
  patch_core_stats: { command: "patch_core_stats", target: "main_character", patch: { strength: 20 } },
  patch_point_pools: { command: "patch_point_pools", target: "main_character", patch: { talent_points: 2 } },
  set_level: { command: "set_level", target: "main_character", level: 7 },
  set_experience: { command: "set_experience", target: "main_character", experience: 1000 },
  set_approval: { command: "set_approval", target: { companion: { index: 0 } }, approval: 50 },
  replace_ability_list: {
    command: "replace_ability_list",
    target: "main_character",
    list: "talents",
    ability_ids: [100],
  },
  replace_crafting_recipe_list: { command: "replace_crafting_recipe_list", recipe_ids: [1, 2] },
  patch_plot_flags: { command: "patch_plot_flags", booleans: [{ id: 1, value: true }], integers: [{ id: 2, value: 3 }] },
  patch_item_metadata: {
    command: "patch_item_metadata",
    container: { equipment: { target: { companion: { index: 0 } } } },
    index: 0,
    patch: { item_level: 2, material: null },
  },
  remove_backpack_item: { command: "remove_backpack_item", index: 0 },
  clone_backpack_item: { command: "clone_backpack_item", index: 0 },
  set_backpack_item_stack_size: { command: "set_backpack_item_stack_size", index: 0, stack_size: 9 },
  replace_backpack_item: { command: "replace_backpack_item", index: 0, replacement: { resref: "gen_swd" } },
  add_item_property: { command: "add_item_property", container: "backpack", index: 0, property_id: 7, power: 1 },
  remove_item_property: { command: "remove_item_property", container: "backpack", index: 0, property_index: 1 },
  set_item_property_power: { command: "set_item_property_power", container: "backpack", index: 0, property_index: 1, power: 2 },
  set_item_property_id: { command: "set_item_property_id", container: "backpack", index: 0, property_index: 1, property_id: 8 },
  apply_batch: {
    command: "apply_batch",
    commands: [
      { command: "set_money", money: 123 },
      {
        command: "patch_core_stats",
        target: { companion: { index: 0 } },
        patch: { strength: 20, magic: 18 },
      },
      { command: "set_backpack_item_stack_size", index: 0, stack_size: 9 },
    ],
  },
  save_as: { command: "save_as", output_path: "C:/mock/save-copy.das" },
} satisfies { [K in SaveCommand["command"]]: Extract<SaveCommand, { command: K }> };

const errorFixtures = {
  invalid_stack_size: {
    code: "invalid_stack_size",
    message: "invalid stack size 500; stack size must be between 1 and 99",
  },
  no_stat_row_template: {
    code: "no_stat_row_template",
    message: "cannot insert stat row for MainCharacter: no stat row template exists",
  },
} satisfies Record<string, CommandError>;

const resultFixtures = {
  validation: { result: "validation", report: { is_valid: true, findings: [] } },
  summary: {
    result: "summary",
    summary: {
      source_path: "C:/mock/save.das",
      dirty: false,
      preferred_game: "dao",
      money: 10,
      main_character_name: "Aedan",
      companion_count: 1,
      backpack_count: 2,
    },
  },
  document_assets: { result: "document_assets", assets: { screenshot_data_url: null } },
  available_abilities: { result: "available_abilities", list: "talents", abilities: [] },
  available_item_properties: { result: "available_item_properties", properties: [{ id: 7, name: "Damage" }] },
  available_crafting_recipes: { result: "available_crafting_recipes", recipes: [{ id: 1, name: "Potion", category: "Potions" }] },
  available_plot_flags: { result: "available_plot_flags", booleans: [], integers: [] },
  characters: { result: "characters", characters: [{ target: "main_character", name: "Aedan" }] },
  items: { result: "items", items: [] },
  crafting_recipes: { result: "crafting_recipes", recipe_ids: [1] },
  plot_flags: { result: "plot_flags", booleans: [{ id: 1, value: true }], integers: [{ id: 2, value: 3 }] },
  character: {
    result: "character",
    target: "main_character",
    character: {
      name: "Aedan",
      template_resref: null,
      approval: null,
      level: 1,
      experience: 10,
      core_stats: { strength: 10, dexterity: 10, willpower: 10, magic: 10, cunning: 10, constitution: 10 },
      point_pools: { attribute_points: 1, skill_points: 1, talent_points: 1, specialization_points: 1 },
      equipment: [],
      skills: [],
      talents: [],
      spells: [],
    },
  },
  item: {
    result: "item",
    container: "backpack",
    index: 0,
    item: {
      resref: "gen_swd",
      name: "Sword",
      wiki_url: null,
      category: { value: "weapon", label: "Weapon" },
      stackable: false,
      object_id: null,
      equipment_slot: null,
      item_cost: null,
      item_stacksize: null,
      item_level: null,
      material: null,
      material_profile: null,
      material_info: null,
      material_options: [],
      properties: [],
    },
  },
  saved: {
    result: "saved",
    output_path: "C:/mock/save-copy.das",
    summary: {
      source_path: "C:/mock/save-copy.das",
      dirty: false,
      preferred_game: "dao",
      money: 10,
      main_character_name: "Aedan",
      companion_count: 1,
      backpack_count: 2,
    },
  },
} satisfies { [K in SaveCommandResult["result"]]: Extract<SaveCommandResult, { result: K }> };

describe("Tauri command contract", () => {
  it("has representative frontend fixtures for every command and result variant", () => {
    expect(Object.keys(commandFixtures)).toContain("apply_batch");
    expect(Object.keys(resultFixtures)).toContain("summary");
  });

  it("serializes apply_batch with the Rust snake_case wire shape", () => {
    expect(JSON.parse(JSON.stringify(commandFixtures.apply_batch))).toEqual({
      command: "apply_batch",
      commands: [
        { command: "set_money", money: 123 },
        {
          command: "patch_core_stats",
          target: { companion: { index: 0 } },
          patch: { strength: 20, magic: 18 },
        },
        { command: "set_backpack_item_stack_size", index: 0, stack_size: 9 },
      ],
    });
  });

  it("serializes nested equipment containers with the Rust enum wire shape", () => {
    expect(JSON.parse(JSON.stringify(commandFixtures.patch_item_metadata))).toEqual({
      command: "patch_item_metadata",
      container: { equipment: { target: { companion: { index: 0 } } } },
      index: 0,
      patch: { item_level: 2, material: null },
    });
  });

  it("keeps frontend error codes aligned with Rust command errors", () => {
    expect(errorFixtures.invalid_stack_size.code).toBe("invalid_stack_size");
    expect(errorFixtures.no_stat_row_template.code).toBe("no_stat_row_template");
  });

  it("narrows command results by result tag", () => {
    const summary = expectResult(resultFixtures.summary, "summary").summary;
    expect(summary.main_character_name).toBe("Aedan");
    expect(() => expectResult(resultFixtures.summary, "items")).toThrow("Expected items result, received summary.");
  });
});
