export type CommandErrorCode =
  | "invalid_target"
  | "missing_field"
  | "type_mismatch"
  | "missing_stat_row"
  | "unsupported_numeric_value"
  | "numeric_range"
  | "lookup_failed"
  | "unknown_ability"
  | "invalid_ability_kind"
  | "missing_core_ability"
  | "invalid_item_index"
  | "missing_item_resref"
  | "backpack_resref_mismatch"
  | "invalid_property_index"
  | "invalid_property_array_parity"
  | "unsupported_game_for_clone"
  | "item_is_stackable"
  | "item_is_not_stackable"
  | "invalid_stack_size"
  | "unsupported_plot_flags"
  | "invalid_save_state"
  | "io"
  | "extract";

export type CommandError = {
  code: CommandErrorCode;
  message: string;
};

export type CharacterTarget = "main_character" | { companion: { index: number } };

export type AbilityListKind = "skills" | "talents" | "spells";

export type InventoryContainer = "backpack" | { equipment: { target: CharacterTarget } };

export type SaveSummary = {
  source_path: string;
  dirty: boolean;
  preferred_game: "dao" | "dao_awakening" | "da2" | null;
  money: number;
  main_character_name: string;
  companion_count: number;
  backpack_count: number;
};

export type DocumentAssets = {
  screenshot_data_url: string | null;
};

export type ValidationSeverity = "error" | "warning";

export type ValidationCode =
  | "missing_field"
  | "type_mismatch"
  | "invalid_numeric_value"
  | "invalid_list_entry"
  | "invalid_property_array_parity";

export type ValidationFinding = {
  severity: ValidationSeverity;
  code: ValidationCode;
  path: string;
  message: string;
};

export type ValidationReport = {
  is_valid: boolean;
  findings: ValidationFinding[];
};

export type SelectableItemProperty = {
  id: number;
  name: string | null;
};

export type CraftingRecipe = {
  id: number;
  name: string;
  category: string;
};

export type PlotBooleanValue = {
  id: number;
  value: boolean;
};

export type PlotIntegerValue = {
  id: number;
  value: number;
};

export type PlotBooleanFlag = {
  id: number;
  name: string;
  description: string;
  category: string;
};

export type PlotIntegerFlag = {
  id: number;
  name: string;
  description: string;
  category: string;
  options: PlotIntegerOption[];
};

export type PlotIntegerOption = {
  value: number;
  label: string;
};

export type CharacterSummary = {
  target: CharacterTarget;
  name: string;
};

export type CoreStats = {
  strength: number;
  dexterity: number;
  willpower: number;
  magic: number;
  cunning: number;
  constitution: number;
};

export type PointPools = {
  attribute_points: number | null;
  skill_points: number | null;
  talent_points: number | null;
  specialization_points: number | null;
};

export type Ability = {
  id: number;
  name: string | null;
  tree: string | null;
  ability_type: string | null;
  core_ids: number[];
};

export type ItemProperty = {
  id: number;
  name: string | null;
  power: number;
};

export type MaterialInfo = {
  code: number;
  tier: number;
  name: string;
  family: MaterialFamily;
  target: MaterialTarget;
};

export type MaterialFamily = "metal" | "wood" | "leather";

export type MaterialTarget = "armor" | "weapon" | "shield";

export type MaterialProfile = {
  family: MaterialFamily;
  target: MaterialTarget;
};

export type ItemCategory = {
  value: string;
  label: string;
};

export type Item = {
  resref: string | null;
  name: string | null;
  wiki_url: string | null;
  category: ItemCategory;
  stackable: boolean;
  object_id: number | null;
  equipment_slot: number | null;
  item_cost: number | null;
  item_stacksize: number | null;
  item_level: number | null;
  material: number | null;
  material_profile: MaterialProfile | null;
  material_info: MaterialInfo | null;
  material_options: MaterialInfo[];
  properties: ItemProperty[];
};

export type Character = {
  name: string;
  template_resref: string | null;
  approval: number | null;
  level: number | null;
  experience: number | null;
  core_stats: CoreStats;
  point_pools: PointPools;
  equipment: Item[];
  skills: Ability[];
  talents: Ability[];
  spells: Ability[];
};

export type IndexedItem = {
  index: number;
  item: Item;
};

export type SaveCommand =
  | { command: "validate" }
  | { command: "get_summary" }
  | { command: "get_document_assets" }
  | { command: "get_character"; target: CharacterTarget }
  | { command: "list_available_abilities"; list: AbilityListKind }
  | { command: "list_available_item_properties" }
  | { command: "list_available_crafting_recipes" }
  | { command: "list_available_plot_flags" }
  | { command: "list_characters" }
  | { command: "list_backpack_items" }
  | { command: "list_equipment_items"; target: CharacterTarget }
  | { command: "list_crafting_recipes" }
  | { command: "list_plot_flags" }
  | { command: "set_money"; money: number }
  | { command: "patch_core_stats"; target: CharacterTarget; patch: Partial<CoreStats> }
  | { command: "patch_point_pools"; target: CharacterTarget; patch: Partial<PointPools> }
  | { command: "set_level"; target: CharacterTarget; level: number }
  | { command: "set_experience"; target: CharacterTarget; experience: number }
  | { command: "set_approval"; target: CharacterTarget; approval: number }
  | {
      command: "replace_ability_list";
      target: CharacterTarget;
      list: AbilityListKind;
      ability_ids: number[];
    }
  | { command: "replace_crafting_recipe_list"; recipe_ids: number[] }
  | { command: "patch_plot_flags"; booleans: PlotBooleanValue[]; integers: PlotIntegerValue[] }
  | {
      command: "patch_item_metadata";
      container: InventoryContainer;
      index: number;
      patch: {
        item_cost?: number | null;
        material?: number | null;
        item_level?: number | null;
      };
    }
  | { command: "remove_backpack_item"; index: number }
  | { command: "clone_backpack_item"; index: number }
  | { command: "set_backpack_item_stack_size"; index: number; stack_size: number }
  | {
      command: "replace_backpack_item";
      index: number;
      replacement: {
        resref: string;
        item_cost?: number | null;
        material?: number | null;
        item_level?: number | null;
      };
    }
  | {
      command: "add_item_property";
      container: InventoryContainer;
      index: number;
      property_id: number;
      power: number;
    }
  | {
      command: "remove_item_property";
      container: InventoryContainer;
      index: number;
      property_index: number;
    }
  | {
      command: "set_item_property_power";
      container: InventoryContainer;
      index: number;
      property_index: number;
      power: number;
    }
  | {
      command: "set_item_property_id";
      container: InventoryContainer;
      index: number;
      property_index: number;
      property_id: number;
    }
  | { command: "save_as"; output_path: string };

export type SaveCommandResult =
  | { result: "validation"; report: ValidationReport }
  | { result: "summary"; summary: SaveSummary }
  | { result: "document_assets"; assets: DocumentAssets }
  | { result: "available_abilities"; list: AbilityListKind; abilities: Ability[] }
  | { result: "available_item_properties"; properties: SelectableItemProperty[] }
  | { result: "available_crafting_recipes"; recipes: CraftingRecipe[] }
  | { result: "available_plot_flags"; booleans: PlotBooleanFlag[]; integers: PlotIntegerFlag[] }
  | { result: "characters"; characters: CharacterSummary[] }
  | { result: "items"; items: IndexedItem[] }
  | { result: "crafting_recipes"; recipe_ids: number[] }
  | { result: "plot_flags"; booleans: PlotBooleanValue[]; integers: PlotIntegerValue[] }
  | { result: "character"; target: CharacterTarget; character: Character }
  | { result: "item"; container: InventoryContainer; index: number; item: Item }
  | { result: "saved"; output_path: string; summary: SaveSummary };
