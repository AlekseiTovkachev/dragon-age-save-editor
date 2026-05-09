import type {
  Ability,
  Character,
  CraftingRecipe,
  IndexedItem,
  InventoryContainer,
  Item,
  PlotBooleanFlag,
  PlotIntegerFlag,
  SaveCommand,
  SaveCommandResult,
  SaveSummary,
  SelectableItemProperty,
} from "../types";

type MockState = {
  summary: SaveSummary | null;
  character: Character;
  backpack: IndexedItem[];
  recipes: number[];
  plotBooleans: Record<number, boolean>;
  plotIntegers: Record<number, number>;
};

const abilities: Record<"skills" | "talents" | "spells", Ability[]> = {
  skills: [{ id: 4001, name: "Coercion", tree: "Communication", ability_type: "Skill", core_ids: [] }],
  talents: [{ id: 100, name: "Powerful", tree: "Warrior", ability_type: "Talent", core_ids: [] }],
  spells: [{ id: 200, name: "Flame Blast", tree: "Primal", ability_type: "Spell", core_ids: [] }],
};

const itemProperties: SelectableItemProperty[] = [
  { id: 7, name: "Increase Damage" },
  { id: 8, name: "Increase Dexterity" },
  { id: 9, name: "Fire Damage" },
];

const recipeCatalog: CraftingRecipe[] = [
  { id: 1, name: "Health Poultice", category: "Potions" },
  { id: 2, name: "Lyrium Potion", category: "Potions" },
  { id: 3, name: "Fire Bomb", category: "Bombs" },
];

const plotBooleanCatalog: PlotBooleanFlag[] = [
  { id: 1, name: "act1_helped_mages", description: "Helped the mages", category: "Act 1" },
];

const plotIntegerCatalog: PlotIntegerFlag[] = [
  {
    id: 10,
    name: "act1_major_choice",
    description: "Act 1 major choice",
    category: "Act 1",
    options: [
      { value: 0, label: "Unset" },
      { value: 2, label: "Picked" },
    ],
  },
];

let state = createState("dao");

function createItem(overrides: Partial<Item> = {}): Item {
  return {
    resref: "gen_im_arm_starfang",
    name: "Starfang",
    wiki_url: "https://dragonage.fandom.com/wiki/Starfang",
    category: { value: "weapons.longswords", label: "Weapons > Longswords" },
    stackable: true,
    object_id: null,
    equipment_slot: null,
    item_cost: 0,
    item_stacksize: 3,
    item_level: 0,
    material: 1,
    material_profile: { family: "metal", target: "weapon" },
    material_info: null,
    material_options: [{ code: 1, tier: 1, name: "Iron", family: "metal", target: "weapon" }],
    properties: [{ id: 7, name: "Increase Damage", power: 1 }],
    ...overrides,
  };
}

function createCharacter(): Character {
  return {
    name: "Aedan",
    template_resref: null,
    approval: null,
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
    equipment: [createItem({ name: "Equipped Sword", stackable: false, item_stacksize: null })],
    skills: abilities.skills,
    talents: abilities.talents,
    spells: abilities.spells,
  };
}

function createState(game: "dao" | "da2"): MockState {
  return {
    summary: {
      source_path: `/mock/${game}.das`,
      dirty: false,
      preferred_game: game,
      money: 100,
      main_character_name: game === "da2" ? "Hawke" : "Aedan",
      companion_count: 1,
      backpack_count: 2,
    },
    character: createCharacter(),
    backpack: [
      { index: 0, item: createItem() },
      { index: 1, item: createItem({ name: "Health Poultice", resref: "gen_im_potion", item_stacksize: 1 }) },
    ],
    recipes: [1],
    plotBooleans: { 1: false },
    plotIntegers: { 10: 0 },
  };
}

function selectedGame(): "dao" | "da2" {
  return globalThis.localStorage?.getItem("smokeGame") === "da2" ? "da2" : "dao";
}

function summaryResult(): SaveCommandResult {
  return { result: "summary", summary: state.summary! };
}

function failingCommand() {
  return globalThis.localStorage?.getItem("smokeFailCommand");
}

function validationIsForcedInvalid() {
  return globalThis.localStorage?.getItem("smokeInvalidValidation") === "1";
}

function knownAbility(id: number): Ability {
  return Object.values(abilities)
    .flat()
    .find((ability) => ability.id === id) ?? { id, name: `Ability ${id}`, tree: null, ability_type: null, core_ids: [] };
}

function updateBackpackItem(index: number, update: (item: Item) => Item): Item {
  state.backpack = state.backpack.map((entry) =>
    entry.index === index ? { ...entry, item: update(entry.item) } : entry,
  );
  return state.backpack.find((entry) => entry.index === index)!.item;
}

function updateInventoryItem(container: InventoryContainer, index: number, update: (item: Item) => Item): Item {
  if (container === "backpack") {
    return updateBackpackItem(index, update);
  }
  state.character = {
    ...state.character,
    equipment: state.character.equipment.map((item, itemIndex) => itemIndex === index ? update(item) : item),
  };
  return state.character.equipment[index];
}

function markDirty() {
  if (state.summary) {
    state.summary = { ...state.summary, dirty: true };
  }
}

async function executeSingle(command: SaveCommand): Promise<SaveCommandResult> {
  if (failingCommand() === command.command) {
    throw { code: "io", message: `Mocked ${command.command} failure` };
  }

  switch (command.command) {
    case "validate":
      return {
        result: "validation",
        report: {
          is_valid: !validationIsForcedInvalid(),
          findings: validationIsForcedInvalid()
            ? [{
                severity: "error",
                code: "missing_field",
                path: "SAVEGAME_PARTYLIST",
                message: "Mocked invalid save",
              }]
            : [],
        },
      };
    case "get_summary":
      return summaryResult();
    case "get_document_assets":
      return { result: "document_assets", assets: { screenshot_data_url: null } };
    case "list_characters":
      return { result: "characters", characters: [{ target: "main_character", name: state.character.name }] };
    case "get_character":
      return { result: "character", target: command.target, character: state.character };
    case "list_available_abilities":
      return { result: "available_abilities", list: command.list, abilities: abilities[command.list] };
    case "list_available_item_properties":
      return { result: "available_item_properties", properties: itemProperties };
    case "list_available_crafting_recipes":
      return { result: "available_crafting_recipes", recipes: recipeCatalog };
    case "list_available_plot_flags":
      return { result: "available_plot_flags", booleans: plotBooleanCatalog, integers: plotIntegerCatalog };
    case "list_backpack_items":
      return { result: "items", items: state.backpack };
    case "list_equipment_items":
      return { result: "items", items: state.character.equipment.map((item, index) => ({ index, item })) };
    case "list_crafting_recipes":
      return { result: "crafting_recipes", recipe_ids: state.recipes };
    case "list_plot_flags":
      return {
        result: "plot_flags",
        booleans: Object.entries(state.plotBooleans).map(([id, value]) => ({ id: Number(id), value })),
        integers: Object.entries(state.plotIntegers).map(([id, value]) => ({ id: Number(id), value })),
      };
    case "patch_core_stats":
      state.character = {
        ...state.character,
        core_stats: { ...state.character.core_stats, ...command.patch },
      };
      markDirty();
      return summaryResult();
    case "patch_point_pools":
      state.character = {
        ...state.character,
        point_pools: { ...state.character.point_pools, ...command.patch },
      };
      markDirty();
      return summaryResult();
    case "set_level":
      state.character = { ...state.character, level: command.level };
      markDirty();
      return summaryResult();
    case "set_experience":
      state.character = { ...state.character, experience: command.experience };
      markDirty();
      return summaryResult();
    case "set_approval":
      state.character = { ...state.character, approval: command.approval };
      markDirty();
      return summaryResult();
    case "replace_ability_list":
      state.character = {
        ...state.character,
        [command.list]: command.ability_ids.map(knownAbility),
      };
      markDirty();
      return summaryResult();
    case "set_money":
      state.summary = { ...state.summary!, money: command.money };
      markDirty();
      return summaryResult();
    case "set_backpack_item_stack_size":
      updateBackpackItem(command.index, (item) => ({ ...item, item_stacksize: command.stack_size }));
      markDirty();
      return { result: "item", container: "backpack", index: command.index, item: state.backpack[command.index].item };
    case "patch_item_metadata":
      {
        const item = updateInventoryItem(command.container, command.index, (entry) => ({ ...entry, ...command.patch }));
        markDirty();
        return { result: "item", container: command.container, index: command.index, item };
      }
    case "add_item_property":
      {
        const item = updateInventoryItem(command.container, command.index, (entry) => ({
          ...entry,
          properties: [
            ...entry.properties,
            {
              id: command.property_id,
              name: itemProperties.find((property) => property.id === command.property_id)?.name ?? null,
              power: command.power,
            },
          ],
        }));
        markDirty();
        return { result: "item", container: command.container, index: command.index, item };
      }
    case "remove_item_property":
      {
        const item = updateInventoryItem(command.container, command.index, (entry) => ({
          ...entry,
          properties: entry.properties.filter((_, index) => index !== command.property_index),
        }));
        markDirty();
        return { result: "item", container: command.container, index: command.index, item };
      }
    case "set_item_property_power":
      {
        const item = updateInventoryItem(command.container, command.index, (entry) => ({
          ...entry,
          properties: entry.properties.map((property, index) =>
            index === command.property_index ? { ...property, power: command.power } : property,
          ),
        }));
        markDirty();
        return { result: "item", container: command.container, index: command.index, item };
      }
    case "set_item_property_id":
      {
        const item = updateInventoryItem(command.container, command.index, (entry) => ({
          ...entry,
          properties: entry.properties.map((property, index) =>
            index === command.property_index
              ? {
                  ...property,
                  id: command.property_id,
                  name: itemProperties.find((option) => option.id === command.property_id)?.name ?? null,
                }
              : property,
          ),
        }));
        markDirty();
        return { result: "item", container: command.container, index: command.index, item };
      }
    case "replace_crafting_recipe_list":
      state.recipes = command.recipe_ids;
      markDirty();
      return { result: "crafting_recipes", recipe_ids: state.recipes };
    case "patch_plot_flags":
      for (const flag of command.booleans) state.plotBooleans[flag.id] = flag.value;
      for (const flag of command.integers) state.plotIntegers[flag.id] = flag.value;
      markDirty();
      return executeSingle({ command: "list_plot_flags" });
    case "save_as":
      if (state.summary) {
        state.summary = { ...state.summary, dirty: false, source_path: command.output_path };
      }
      return { result: "saved", output_path: command.output_path, summary: state.summary! };
    case "apply_batch":
      for (const nested of command.commands) {
        await executeSingle(nested);
      }
      return summaryResult();
    default:
      markDirty();
      return summaryResult();
  }
}

export async function mockOpenDocument(): Promise<SaveSummary> {
  state = createState(selectedGame());
  return state.summary!;
}

export async function mockHasDocument(): Promise<boolean> {
  return false;
}

export async function mockExecuteCommand(command: SaveCommand): Promise<SaveCommandResult> {
  return executeSingle(command);
}

export async function mockOpenDialog(): Promise<string> {
  return `/mock/${selectedGame()}.das`;
}

export async function mockSaveDialog(): Promise<string> {
  return `/mock/${selectedGame()}-edited.das`;
}
