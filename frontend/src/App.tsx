import { useEffect, useMemo, useState } from "react";
import { open, save } from "@tauri-apps/plugin-dialog";
import { openUrl } from "@tauri-apps/plugin-opener";
import { executeCommand, hasDocument, openDocument, toErrorMessage } from "./api";
import type {
  Ability,
  AbilityListKind,
  Character,
  CharacterSummary,
  CharacterTarget,
  CraftingRecipe,
  IndexedItem,
  InventoryContainer,
  Item,
  ItemProperty,
  PlotBooleanFlag,
  PlotIntegerFlag,
  SelectableItemProperty,
  SaveSummary,
} from "./types";

type Section = "characters" | "inventory" | "recipes" | "plot_flags";
type CharacterTab = "overview" | "abilities" | "equipment";
type ItemPropertyDraft = {
  id: number;
  name: string | null;
  power: string;
};

const MAIN_TARGET: CharacterTarget = "main_character";
const SECTIONS: Section[] = ["characters", "inventory", "recipes", "plot_flags"];
const SECTION_TITLES: Record<Section, string> = {
  characters: "Characters",
  inventory: "Inventory",
  recipes: "Recipes",
  plot_flags: "Plot Flags",
};
const CHARACTER_TABS: CharacterTab[] = ["overview", "abilities", "equipment"];
const CHARACTER_TAB_TITLES: Record<CharacterTab, string> = {
  overview: "Overview",
  abilities: "Abilities",
  equipment: "Equipment",
};

function targetKey(target: CharacterTarget): string {
  return target === "main_character" ? "main" : `companion:${target.companion.index}`;
}

function parseNumber(value: string): number | null {
  if (value.trim() === "") {
    return null;
  }
  const parsed = Number(value);
  return Number.isFinite(parsed) ? parsed : null;
}

function abilityLabel(ability: Ability): string {
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

function itemLabel(item: Item, index: number): string {
  if (item.name) {
    return item.name;
  }
  if (item.resref) {
    return `<${item.resref}>`;
  }
  return `Item ${index}`;
}

function titleCase(value: string): string {
  return value
    .split("_")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ");
}

function gameLabel(value: SaveSummary["preferred_game"]): string {
  switch (value) {
    case "dao":
      return "DAO";
    case "dao_awakening":
      return "DAO Awakening";
    case "da2":
      return "DA2";
    default:
      return "Unknown Game";
  }
}

function cloneAbilities(abilities: Ability[]): Ability[] {
  return abilities.map((ability) => ({ ...ability, core_ids: [...ability.core_ids] }));
}

function toItemPropertyDrafts(properties: ItemProperty[]): ItemPropertyDraft[] {
  return properties.map((property) => ({
    id: property.id,
    name: property.name,
    power: property.power.toString(),
  }));
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

function groupedAbilities(
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

function isUselessDa2Talent(ability: Ability): boolean {
  return ability.id < 100000 && ability.id !== 700000;
}

function App() {
  const [section, setSection] = useState<Section>("characters");
  const [characterTab, setCharacterTab] = useState<CharacterTab>("overview");
  const [summary, setSummary] = useState<SaveSummary | null>(null);
  const [screenshotDataUrl, setScreenshotDataUrl] = useState<string | null>(null);
  const [characters, setCharacters] = useState<CharacterSummary[]>([]);
  const [characterKey, setCharacterKey] = useState("main");
  const [character, setCharacter] = useState<Character | null>(null);
  const [items, setItems] = useState<IndexedItem[]>([]);
  const [itemIndex, setItemIndex] = useState<number | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [moneyDraft, setMoneyDraft] = useState("");
  const [statsDraft, setStatsDraft] = useState<Record<string, string>>({});
  const [levelDraft, setLevelDraft] = useState("");
  const [experienceDraft, setExperienceDraft] = useState("");
  const [approvalDraft, setApprovalDraft] = useState("");
  const [pointPoolsDraft, setPointPoolsDraft] = useState<Record<string, string>>({});
  const [abilityDrafts, setAbilityDrafts] = useState<Record<AbilityListKind, Ability[]>>({
    skills: [],
    talents: [],
    spells: [],
  });
  const [availableAbilities, setAvailableAbilities] = useState<Record<AbilityListKind, Ability[]>>({
    skills: [],
    talents: [],
    spells: [],
  });
  const [selectedAbilityToAdd, setSelectedAbilityToAdd] = useState<Record<AbilityListKind, string>>({
    skills: "",
    talents: "",
    spells: "",
  });
  const [availableItemProperties, setAvailableItemProperties] = useState<SelectableItemProperty[]>([]);
  const [craftingRecipes, setCraftingRecipes] = useState<number[]>([]);
  const [craftingRecipeDrafts, setCraftingRecipeDrafts] = useState<number[]>([]);
  const [availableCraftingRecipes, setAvailableCraftingRecipes] = useState<CraftingRecipe[]>([]);
  const [plotBooleanValues, setPlotBooleanValues] = useState<Record<number, boolean>>({});
  const [plotBooleanDrafts, setPlotBooleanDrafts] = useState<Record<number, boolean>>({});
  const [plotIntegerValues, setPlotIntegerValues] = useState<Record<number, number>>({});
  const [plotIntegerDrafts, setPlotIntegerDrafts] = useState<Record<number, number>>({});
  const [availablePlotBooleans, setAvailablePlotBooleans] = useState<PlotBooleanFlag[]>([]);
  const [availablePlotIntegers, setAvailablePlotIntegers] = useState<PlotIntegerFlag[]>([]);
  const [itemMetadataDraft, setItemMetadataDraft] = useState({
    material: "",
    item_level: "",
    stack_size: "",
  });
  const [itemPropertiesDraft, setItemPropertiesDraft] = useState<ItemPropertyDraft[]>([]);
  const [propertyDraft, setPropertyDraft] = useState({ property_id: "", power: "" });

  const selectedCharacterTarget = useMemo(
    () => characters.find((entry) => targetKey(entry.target) === characterKey)?.target ?? MAIN_TARGET,
    [characters, characterKey],
  );
  const selectedInventoryContainer = useMemo<InventoryContainer>(() => {
    if (section === "characters" && characterTab === "equipment") {
      return { equipment: { target: selectedCharacterTarget } };
    }
    return "backpack";
  }, [characterTab, section, selectedCharacterTarget]);
  const shouldLoadItems = section === "inventory" || (section === "characters" && characterTab === "equipment");
  const selectedItem = useMemo(
    () => items.find((entry) => entry.index === itemIndex)?.item ?? null,
    [itemIndex, items],
  );
  const canEdit = Boolean(summary);
  const isDa2 = summary?.preferred_game === "da2";
  const isBackpackInventory = section === "inventory" && selectedInventoryContainer === "backpack";
  const canEditStackSize = Boolean(
    selectedItem && isBackpackInventory && selectedItem.stackable,
  );
  const canCloneBackpackItem = Boolean(
    selectedItem &&
      isBackpackInventory &&
      !selectedItem.stackable &&
      (summary?.preferred_game === "dao" ||
        summary?.preferred_game === "dao_awakening" ||
        summary?.preferred_game === "da2"),
  );
  const canEditMaterial = Boolean(
    selectedItem?.material_profile && selectedItem.material_options.length > 0,
  );
  const visibleSections = useMemo(
    () => SECTIONS.filter((entry) => entry !== "plot_flags" || summary?.preferred_game === "da2"),
    [summary?.preferred_game],
  );
  const visibleAbilityKinds = useMemo<AbilityListKind[]>(
    () => (isDa2 ? ["talents", "spells"] : ["skills", "talents", "spells"]),
    [isDa2],
  );

  useEffect(() => {
    void hasDocument().then(async (present) => {
      if (present) {
        await refreshSummary();
        await refreshDocumentAssets();
        await refreshCharacters();
        await refreshAvailableAbilities();
        await refreshAvailableItemProperties();
        await refreshCraftingRecipes();
        await refreshAvailableCraftingRecipes();
        await refreshPlotFlags();
        await refreshAvailablePlotFlags();
      }
    });
  }, []);

  useEffect(() => {
    if (summary) {
      setMoneyDraft(summary.money.toString());
    }
  }, [summary]);

  useEffect(() => {
    if (section === "plot_flags" && summary?.preferred_game !== "da2") {
      setSection("characters");
    }
  }, [section, summary?.preferred_game]);

  useEffect(() => {
    if (!character) {
      return;
    }
    setStatsDraft({
      strength: character.core_stats.strength.toString(),
      dexterity: character.core_stats.dexterity.toString(),
      willpower: character.core_stats.willpower.toString(),
      magic: character.core_stats.magic.toString(),
      cunning: character.core_stats.cunning.toString(),
      constitution: character.core_stats.constitution.toString(),
    });
    setLevelDraft(character.level?.toString() ?? "");
    setExperienceDraft(character.experience?.toString() ?? "");
    setApprovalDraft(character.approval?.toString() ?? "");
    setPointPoolsDraft({
      attribute_points: character.point_pools.attribute_points?.toString() ?? "",
      skill_points: character.point_pools.skill_points?.toString() ?? "",
      talent_points: character.point_pools.talent_points?.toString() ?? "",
      specialization_points: character.point_pools.specialization_points?.toString() ?? "",
    });
    setAbilityDrafts({
      skills: cloneAbilities(character.skills),
      talents: cloneAbilities(character.talents),
      spells: cloneAbilities(character.spells),
    });
  }, [character]);

  useEffect(() => {
    if (!selectedItem) {
      return;
    }
    setItemMetadataDraft({
      material: selectedItem.material?.toString() ?? "",
      item_level: selectedItem.item_level?.toString() ?? "",
      stack_size: selectedItem.item_stacksize?.toString() ?? "1",
    });
    setItemPropertiesDraft(toItemPropertyDrafts(selectedItem.properties));
  }, [selectedItem]);

  useEffect(() => {
    if (summary) {
      void loadCharacter(selectedCharacterTarget);
    }
  }, [selectedCharacterTarget, summary]);

  useEffect(() => {
    if (summary && shouldLoadItems) {
      void refreshItems();
    }
  }, [selectedInventoryContainer, shouldLoadItems, summary]);

  async function run(action: () => Promise<void>) {
    setBusy(true);
    setError(null);
    try {
      await action();
    } catch (caught) {
      setError(toErrorMessage(caught));
    } finally {
      setBusy(false);
    }
  }

  async function refreshSummary() {
    const response = await executeCommand({ command: "get_summary" });
    if (response.result === "summary") {
      setSummary(response.summary);
    }
  }

  async function refreshDocumentAssets() {
    const response = await executeCommand({ command: "get_document_assets" });
    if (response.result === "document_assets") {
      setScreenshotDataUrl(response.assets.screenshot_data_url);
    }
  }

  async function refreshCharacters() {
    const response = await executeCommand({ command: "list_characters" });
    if (response.result === "characters") {
      setCharacters(response.characters);
      if (response.characters[0]) {
        setCharacterKey((current) =>
          response.characters.some((entry) => targetKey(entry.target) === current)
            ? current
            : targetKey(response.characters[0].target),
        );
      }
    }
  }

  async function refreshAvailableAbilities() {
    for (const list of ["skills", "talents", "spells"] as AbilityListKind[]) {
      const response = await executeCommand({ command: "list_available_abilities", list });
      if (response.result === "available_abilities") {
        const sortedAbilities = response.abilities
          .filter((ability) => !(summary?.preferred_game === "da2" && list === "talents" && isUselessDa2Talent(ability)))
          .sort((left, right) =>
          abilityLabel(left).localeCompare(abilityLabel(right), undefined, { sensitivity: "base" }),
        );
        setAvailableAbilities((current) => ({ ...current, [list]: sortedAbilities }));
        setSelectedAbilityToAdd((current) => ({
          ...current,
          [list]: sortedAbilities[0] ? sortedAbilities[0].id.toString() : "",
        }));
      }
    }
  }

  async function refreshAvailableItemProperties() {
    const response = await executeCommand({ command: "list_available_item_properties" });
    if (response.result === "available_item_properties") {
      setAvailableItemProperties(response.properties);
      setPropertyDraft((current) => ({
        ...current,
        property_id: current.property_id || response.properties[0]?.id.toString() || "",
      }));
    }
  }

  async function refreshCraftingRecipes() {
    const response = await executeCommand({ command: "list_crafting_recipes" });
    if (response.result === "crafting_recipes") {
      setCraftingRecipes(response.recipe_ids);
      setCraftingRecipeDrafts(response.recipe_ids);
    }
  }

  async function refreshAvailableCraftingRecipes() {
    const response = await executeCommand({ command: "list_available_crafting_recipes" });
    if (response.result === "available_crafting_recipes") {
      setAvailableCraftingRecipes(response.recipes);
    }
  }

  async function refreshPlotFlags() {
    const response = await executeCommand({ command: "list_plot_flags" });
    if (response.result === "plot_flags") {
      const booleanValues = Object.fromEntries(response.booleans.map((entry) => [entry.id, entry.value]));
      const integerValues = Object.fromEntries(response.integers.map((entry) => [entry.id, entry.value]));
      setPlotBooleanValues(booleanValues);
      setPlotBooleanDrafts(booleanValues);
      setPlotIntegerValues(integerValues);
      setPlotIntegerDrafts(integerValues);
    }
  }

  async function refreshAvailablePlotFlags() {
    const response = await executeCommand({ command: "list_available_plot_flags" });
    if (response.result === "available_plot_flags") {
      setAvailablePlotBooleans(response.booleans);
      setAvailablePlotIntegers(response.integers);
    }
  }

  async function loadCharacter(target: CharacterTarget) {
    const response = await executeCommand({ command: "get_character", target });
    if (response.result === "character") {
      setCharacter(response.character);
    }
  }

  async function refreshItems() {
    const equipmentTarget =
      selectedInventoryContainer === "backpack" ? null : selectedInventoryContainer.equipment.target;
    const response = await (selectedInventoryContainer === "backpack"
      ? executeCommand({ command: "list_backpack_items" })
      : executeCommand({
          command: "list_equipment_items",
          target: equipmentTarget ?? MAIN_TARGET,
        }));
    if (response.result === "items") {
      setItems(response.items);
      setItemIndex((current) =>
        current !== null && response.items.some((entry) => entry.index === current)
          ? current
          : response.items[0]?.index ?? null,
      );
    }
  }

  function updateVisibleItem(index: number, item: Item) {
    setItems((current) => current.map((entry) => (entry.index === index ? { ...entry, item } : entry)));
  }

  function sortedRecipeChecklistIds(): number[] {
    return Array.from(new Set([
      ...availableCraftingRecipes.map((recipe) => recipe.id),
      ...craftingRecipes,
      ...craftingRecipeDrafts,
    ])).sort((left, right) => left - right);
  }

  function recipeLabel(recipeId: number): string {
    const recipe = availableCraftingRecipes.find((entry) => entry.id === recipeId);
    return recipe ? `${recipe.name} (${recipe.id})` : `Recipe ${recipeId}`;
  }

  function recipeCategory(recipeId: number): string {
    return availableCraftingRecipes.find((entry) => entry.id === recipeId)?.category ?? "Other";
  }

  function recipeIsKnown(recipeId: number): boolean {
    return availableCraftingRecipes.some((entry) => entry.id === recipeId);
  }

  function groupedRecipeChecklistIds(): { category: string; ids: number[] }[] {
    const groups = new Map<string, number[]>();
    for (const recipeId of sortedRecipeChecklistIds()) {
      const category = recipeCategory(recipeId);
      groups.set(category, [...(groups.get(category) ?? []), recipeId]);
    }
    return Array.from(groups, ([category, ids]) => ({ category, ids }));
  }

  function handleCraftingRecipeToggle(recipeId: number, checked: boolean) {
    setCraftingRecipeDrafts((current) => {
      if (checked) {
        return current.includes(recipeId) ? current : [...current, recipeId];
      }
      return current.filter((id) => id !== recipeId);
    });
  }

  async function handleCraftingRecipesApply() {
    await run(async () => {
      const response = await executeCommand({
        command: "replace_crafting_recipe_list",
        recipe_ids: craftingRecipeDrafts,
      });
      if (response.result === "crafting_recipes") {
        setCraftingRecipes(response.recipe_ids);
        setCraftingRecipeDrafts(response.recipe_ids);
      }
      await refreshSummary();
    });
  }

  function resetCraftingRecipeDrafts() {
    setCraftingRecipeDrafts(craftingRecipes);
  }

  function groupedPlotBooleans(): { category: string; flags: PlotBooleanFlag[] }[] {
    const groups = new Map<string, PlotBooleanFlag[]>();
    for (const flag of availablePlotBooleans) {
      groups.set(flag.category, [...(groups.get(flag.category) ?? []), flag]);
    }
    return Array.from(groups, ([category, flags]) => ({ category, flags }));
  }

  function groupedPlotIntegers(): { category: string; flags: PlotIntegerFlag[] }[] {
    const groups = new Map<string, PlotIntegerFlag[]>();
    for (const flag of availablePlotIntegers) {
      groups.set(flag.category, [...(groups.get(flag.category) ?? []), flag]);
    }
    return Array.from(groups, ([category, flags]) => ({ category, flags }));
  }

  function handlePlotBooleanToggle(id: number, value: boolean) {
    setPlotBooleanDrafts((current) => ({ ...current, [id]: value }));
  }

  function handlePlotIntegerChange(id: number, value: number) {
    setPlotIntegerDrafts((current) => ({ ...current, [id]: value }));
  }

  async function handlePlotFlagsApply() {
    await run(async () => {
      const response = await executeCommand({
        command: "patch_plot_flags",
        booleans: availablePlotBooleans
          .filter((flag) => plotBooleanValues[flag.id] !== undefined || Boolean(plotBooleanDrafts[flag.id]))
          .map((flag) => ({
            id: flag.id,
            value: Boolean(plotBooleanDrafts[flag.id]),
          })),
        integers: availablePlotIntegers
          .filter((flag) => plotIntegerDrafts[flag.id] !== undefined)
          .map((flag) => ({
            id: flag.id,
            value: plotIntegerDrafts[flag.id],
          })),
      });
      if (response.result === "plot_flags") {
        const booleanValues = Object.fromEntries(response.booleans.map((entry) => [entry.id, entry.value]));
        const integerValues = Object.fromEntries(response.integers.map((entry) => [entry.id, entry.value]));
        setPlotBooleanValues(booleanValues);
        setPlotBooleanDrafts(booleanValues);
        setPlotIntegerValues(integerValues);
        setPlotIntegerDrafts(integerValues);
      }
      await refreshSummary();
    });
  }

  function resetPlotFlagDrafts() {
    setPlotBooleanDrafts(plotBooleanValues);
    setPlotIntegerDrafts(plotIntegerValues);
  }

  async function handleOpen() {
    const path = await open({
      title: "Open Dragon Age Save",
      filters: [{ name: "Dragon Age Save", extensions: ["das"] }],
      multiple: false,
    });
    if (!path || Array.isArray(path)) {
      return;
    }
    await run(async () => {
      const opened = await openDocument(path);
      const validationResult = await executeCommand({ command: "validate" });
      if (validationResult.result === "validation" && !validationResult.report.is_valid) {
        setSummary(null);
        setCharacter(null);
        setCharacters([]);
        setItems([]);
        setScreenshotDataUrl(null);
        throw new Error("Failed to open save: validation reported an invalid save structure.");
      }
      setSummary(opened);
      await refreshDocumentAssets();
      await refreshCharacters();
      await refreshAvailableAbilities();
      await refreshAvailableItemProperties();
      await refreshCraftingRecipes();
      await refreshAvailableCraftingRecipes();
      await refreshPlotFlags();
      await refreshAvailablePlotFlags();
      setSection("characters");
      setCharacterTab("overview");
    });
  }

  async function handleSaveAs() {
    if (!summary) {
      return;
    }
    const path = await save({
      title: "Save Edited File As",
      defaultPath: summary.source_path,
      filters: [{ name: "Dragon Age Save", extensions: ["das"] }],
    });
    if (!path) {
      return;
    }
    await run(async () => {
      const validationResult = await executeCommand({ command: "validate" });
      if (validationResult.result === "validation" && !validationResult.report.is_valid) {
        throw new Error("Failed to save: the current document is not structurally valid.");
      }
      const response = await executeCommand({ command: "save_as", output_path: path });
      if (response.result === "saved") {
        setSummary(response.summary);
      }
    });
  }

  async function handleMoneyApply() {
    const money = parseNumber(moneyDraft);
    if (money === null) {
      setError("Money must be a valid number.");
      return;
    }
    await run(async () => {
      const response = await executeCommand({ command: "set_money", money });
      if (response.result === "summary") {
        setSummary(response.summary);
      }
    });
  }

  function resetMoneyDraft() {
    if (summary) {
      setMoneyDraft(summary.money.toString());
    }
  }

  async function handleCharacterApply() {
    await run(async () => {
      let response = await executeCommand({
        command: "patch_core_stats",
        target: selectedCharacterTarget,
        patch: {
          strength: parseNumber(statsDraft.strength) ?? undefined,
          dexterity: parseNumber(statsDraft.dexterity) ?? undefined,
          willpower: parseNumber(statsDraft.willpower) ?? undefined,
          magic: parseNumber(statsDraft.magic) ?? undefined,
          cunning: parseNumber(statsDraft.cunning) ?? undefined,
          constitution: parseNumber(statsDraft.constitution) ?? undefined,
        },
      });
      const level = parseNumber(levelDraft);
      if (level === null) {
        throw new Error("Level must be a valid number.");
      }
      response = await executeCommand({
        command: "set_level",
        target: selectedCharacterTarget,
        level,
      });
      if (response.result === "character") {
        setCharacter(response.character);
      }
      const experience = parseNumber(experienceDraft);
      if (experienceDraft.trim() !== "" && experience === null) {
        throw new Error("Experience must be a valid number.");
      }
      if (experience !== null) {
        response = await executeCommand({
          command: "set_experience",
          target: selectedCharacterTarget,
          experience,
        });
        if (response.result === "character") {
          setCharacter(response.character);
        }
      }
      response = await executeCommand({
        command: "patch_point_pools",
        target: selectedCharacterTarget,
        patch: {
          attribute_points: parseNumber(pointPoolsDraft.attribute_points) ?? undefined,
          skill_points: parseNumber(pointPoolsDraft.skill_points) ?? undefined,
          talent_points: parseNumber(pointPoolsDraft.talent_points) ?? undefined,
          specialization_points: parseNumber(pointPoolsDraft.specialization_points) ?? undefined,
        },
      });
      if (response.result === "character") {
        setCharacter(response.character);
      }
      if (character?.approval !== null) {
        const approval = parseNumber(approvalDraft);
        if (approval === null) {
          throw new Error("Approval must be a valid number.");
        }
        response = await executeCommand({
          command: "set_approval",
          target: selectedCharacterTarget,
          approval,
        });
        if (response.result === "character") {
          setCharacter(response.character);
        }
      }
      await refreshSummary();
    });
  }

  function resetCharacterDraft() {
    if (!character) {
      return;
    }
    setStatsDraft({
      strength: character.core_stats.strength.toString(),
      dexterity: character.core_stats.dexterity.toString(),
      willpower: character.core_stats.willpower.toString(),
      magic: character.core_stats.magic.toString(),
      cunning: character.core_stats.cunning.toString(),
      constitution: character.core_stats.constitution.toString(),
    });
    setLevelDraft(character.level?.toString() ?? "");
    setExperienceDraft(character.experience?.toString() ?? "");
    setApprovalDraft(character.approval?.toString() ?? "");
    setPointPoolsDraft({
      attribute_points: character.point_pools.attribute_points?.toString() ?? "",
      skill_points: character.point_pools.skill_points?.toString() ?? "",
      talent_points: character.point_pools.talent_points?.toString() ?? "",
      specialization_points: character.point_pools.specialization_points?.toString() ?? "",
    });
  }

  async function handleAbilityApplyAll() {
    await run(async () => {
      for (const list of ["skills", "talents", "spells"] as AbilityListKind[]) {
        const response = await executeCommand({
          command: "replace_ability_list",
          target: selectedCharacterTarget,
          list,
          ability_ids: abilityDrafts[list].map((ability) => ability.id),
        });
        if (response.result === "character") {
          setCharacter(response.character);
        }
      }
      await refreshSummary();
    });
  }

  function resetAbilityDrafts() {
    if (!character) {
      return;
    }
    setAbilityDrafts({
      skills: cloneAbilities(character.skills),
      talents: cloneAbilities(character.talents),
      spells: cloneAbilities(character.spells),
    });
  }

  function abilityIsLocked(list: AbilityListKind, abilityId: number): boolean {
    return abilityDrafts[list].some((ability) => {
      if (ability.id === abilityId || !ability.core_ids.includes(abilityId)) {
        return false;
      }
      return !ability.core_ids.some(
        (coreId) => coreId !== abilityId && abilityDrafts[list].some((candidate) => candidate.id === coreId),
      );
    });
  }

  function handleAbilityRemove(list: AbilityListKind, abilityId: number) {
    if (abilityIsLocked(list, abilityId)) {
      return;
    }
    setAbilityDrafts((current) => ({
      ...current,
      [list]: current[list].filter((ability) => ability.id !== abilityId),
    }));
  }

  function handleAbilityAdd(list: AbilityListKind) {
    const options = coreAbilityOptions(list);
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
  }

  function visibleAbilities(list: AbilityListKind, abilities: Ability[]): Ability[] {
    if (isDa2 && list === "talents") {
      return abilities.filter((ability) => !isUselessDa2Talent(ability));
    }
    return abilities;
  }

  function allKnownAbilities(list: AbilityListKind): Ability[] {
    const byId = new Map<number, Ability>();
    for (const ability of visibleAbilities(list, [...availableAbilities[list], ...abilityDrafts[list]])) {
      byId.set(ability.id, ability);
    }
    return Array.from(byId.values());
  }

  function abilityById(list: AbilityListKind): Map<number, Ability> {
    return new Map(allKnownAbilities(list).map((ability) => [ability.id, ability]));
  }

  function selectedAbilityIds(list: AbilityListKind): Set<number> {
    return new Set(abilityDrafts[list].map((ability) => ability.id));
  }

  function isCoreAbility(ability: Ability): boolean {
    return ability.core_ids.length === 0;
  }

  function coreAbilityOptions(list: AbilityListKind): Ability[] {
    return visibleAbilities(list, availableAbilities[list]).filter(isCoreAbility);
  }

  function reachesSelectedAbility(
    list: AbilityListKind,
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
      return core ? reachesSelectedAbility(list, core, knownById, selectedIds, seen) : false;
    });
  }

  function visibleTreeAbilities(list: AbilityListKind): Ability[] {
    const known = allKnownAbilities(list);
    const knownById = new Map(known.map((ability) => [ability.id, ability]));
    const selectedIds = selectedAbilityIds(list);
    const selectedOrder = new Map(abilityDrafts[list].map((ability, index) => [ability.id, index]));
    return known
      .filter((ability) => selectedIds.has(ability.id) || (!isCoreAbility(ability) && reachesSelectedAbility(list, ability, knownById, selectedIds)))
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

  function missingPrerequisiteChain(
    list: AbilityListKind,
    ability: Ability,
    knownById: Map<number, Ability>,
    selectedIds: Set<number>,
    seen = new Set<number>(),
  ): Ability[] {
    if (ability.core_ids.length === 0 || seen.has(ability.id)) {
      return [];
    }
    seen.add(ability.id);

    for (const coreId of ability.core_ids) {
      if (selectedIds.has(coreId)) {
        return [];
      }
    }

    for (const coreId of ability.core_ids) {
      const core = knownById.get(coreId);
      if (!core) {
        continue;
      }
      const chain = missingPrerequisiteChain(list, core, knownById, selectedIds, new Set(seen));
      return [...chain, core].filter((candidate, index, candidates) => {
        return !selectedIds.has(candidate.id) && candidates.findIndex((entry) => entry.id === candidate.id) === index;
      });
    }

    return [];
  }

  function handleVisibleAbilityAdd(list: AbilityListKind, abilityId: number) {
    const knownById = abilityById(list);
    const selected = knownById.get(abilityId);
    if (!selected) {
      return;
    }
    setAbilityDrafts((current) => {
      if (current[list].some((ability) => ability.id === selected.id)) {
        return current;
      }
      const selectedIds = new Set(current[list].map((ability) => ability.id));
      const chain = missingPrerequisiteChain(list, selected, knownById, selectedIds);
      const additions = [...chain, selected].filter((ability, index, abilities) => {
        return !selectedIds.has(ability.id) && abilities.findIndex((entry) => entry.id === ability.id) === index;
      });
      return {
        ...current,
        [list]: [...current[list], ...additions],
      };
    });
  }

  function handlePropertyAddDraft() {
    const propertyId = parseNumber(propertyDraft.property_id);
    const power = parseNumber(propertyDraft.power);
    if (propertyId === null || power === null) {
      setError("Property ID and power must be valid numbers.");
      return;
    }
    const selectedProperty = availableItemProperties.find((property) => property.id === propertyId);
    setItemPropertiesDraft((current) => [
      ...current,
      { id: propertyId, name: selectedProperty?.name ?? null, power: propertyDraft.power.trim() },
    ]);
    setPropertyDraft((current) => ({ ...current, power: "" }));
  }

  function handlePropertyRemoveDraft(propertyIndex: number) {
    setItemPropertiesDraft((current) => current.filter((_, index) => index !== propertyIndex));
  }

  function handlePropertyUpdateDraft(kind: "id" | "power", propertyIndex: number, raw: string) {
    setItemPropertiesDraft((current) =>
      current.map((property, index) => {
        if (index !== propertyIndex) {
          return property;
        }
        if (kind === "id") {
          const value = Number(raw);
          const selectedProperty = availableItemProperties.find((entry) => entry.id === value);
          return { ...property, id: value, name: selectedProperty?.name ?? null };
        }
        return { ...property, power: raw };
      }),
    );
  }

  async function handleInventoryApply() {
    if (itemIndex === null) {
      return;
    }
    await run(async () => {
      let response = null as Awaited<ReturnType<typeof executeCommand>> | null;
      if (canEditStackSize) {
        const stackSize = parseNumber(itemMetadataDraft.stack_size);
        if (stackSize === null || !Number.isInteger(stackSize) || stackSize < 1 || stackSize > 99) {
          throw new Error("Stack size must be a whole number from 1 to 99.");
        }
        response = await executeCommand({
          command: "set_backpack_item_stack_size",
          index: itemIndex,
          stack_size: stackSize,
        });
        if (response.result === "item") {
          updateVisibleItem(response.index, response.item);
        }
      }

      response = await executeCommand({
        command: "patch_item_metadata",
        container: selectedInventoryContainer,
        index: itemIndex,
        patch: {
          material: canEditMaterial ? parseNumber(itemMetadataDraft.material) : undefined,
          item_level: parseNumber(itemMetadataDraft.item_level),
        },
      });
      if (response.result === "item") {
        updateVisibleItem(response.index, response.item);
      }

      const sourceProperties = selectedItem?.properties ?? [];
      const draftProperties = itemPropertiesDraft;
      const sharedCount = Math.min(sourceProperties.length, draftProperties.length);

      for (let index = 0; index < sharedCount; index += 1) {
        const parsedPower = parseNumber(draftProperties[index].power);
        if (parsedPower === null) {
          throw new Error(`Property ${index + 1} power must be a valid number.`);
        }
        if (sourceProperties[index].id !== draftProperties[index].id) {
          response = await executeCommand({
            command: "set_item_property_id",
            container: selectedInventoryContainer,
            index: itemIndex,
            property_index: index,
            property_id: draftProperties[index].id,
          });
        }
        if (sourceProperties[index].power !== parsedPower) {
          response = await executeCommand({
            command: "set_item_property_power",
            container: selectedInventoryContainer,
            index: itemIndex,
            property_index: index,
            power: parsedPower,
          });
        }
      }

      if (sourceProperties.length > draftProperties.length) {
        for (let index = sourceProperties.length - 1; index >= draftProperties.length; index -= 1) {
          response = await executeCommand({
            command: "remove_item_property",
            container: selectedInventoryContainer,
            index: itemIndex,
            property_index: index,
          });
        }
      }

      if (draftProperties.length > sourceProperties.length) {
        for (let index = sourceProperties.length; index < draftProperties.length; index += 1) {
          const parsedPower = parseNumber(draftProperties[index].power);
          if (parsedPower === null) {
            throw new Error(`Property ${index + 1} power must be a valid number.`);
          }
          response = await executeCommand({
            command: "add_item_property",
            container: selectedInventoryContainer,
            index: itemIndex,
            property_id: draftProperties[index].id,
            power: parsedPower,
          });
        }
      }

      if (response.result === "item") {
        updateVisibleItem(response.index, response.item);
      }
      await refreshItems();
      await refreshSummary();
    });
  }

  function resetInventoryDraft() {
    if (!selectedItem) {
      return;
    }
    setItemMetadataDraft({
      material: selectedItem.material?.toString() ?? "",
      item_level: selectedItem.item_level?.toString() ?? "",
      stack_size: selectedItem.item_stacksize?.toString() ?? "1",
    });
    setItemPropertiesDraft(toItemPropertyDrafts(selectedItem.properties));
    setPropertyDraft((current) => ({ ...current, power: "" }));
  }

  async function handleBackpackRemove() {
    if (selectedInventoryContainer !== "backpack" || itemIndex === null) {
      return;
    }
    await run(async () => {
      await executeCommand({ command: "remove_backpack_item", index: itemIndex });
      await refreshSummary();
      await refreshItems();
    });
  }

  async function handleBackpackClone() {
    if (!canCloneBackpackItem || itemIndex === null) {
      return;
    }
    await run(async () => {
      const response = await executeCommand({ command: "clone_backpack_item", index: itemIndex });
      await refreshSummary();
      await refreshItems();
      if (response.result === "item") {
        setItemIndex(response.index);
      }
    });
  }

  async function handleWikiOpen(url: string) {
    await run(async () => {
      await openUrl(url);
    });
  }

  function renderItemList() {
    return (
      <div className="item-list scroll-region">
        {items.map((entry) => (
          <button
            key={entry.index}
            className={entry.index === itemIndex ? "list-row active" : "list-row"}
            onClick={() => setItemIndex(entry.index)}
          >
            <strong>{itemLabel(entry.item, entry.index)}</strong>
            <span>{entry.item.resref ?? "no resref"}</span>
            <span>{entry.item.properties.length} Propert{entry.item.properties.length === 1 ? "y" : "ies"}</span>
          </button>
        ))}
      </div>
    );
  }

  function renderItemEditor(options: { allowRemove: boolean }) {
    return (
      <>
        <div className="panel-heading">
          <h2>{selectedItem ? itemLabel(selectedItem, itemIndex ?? 0) : "Item Detail"}</h2>
          <div className="button-row">
            <button onClick={() => void handleInventoryApply()} disabled={!canEdit || busy || itemIndex === null}>Apply</button>
            <button onClick={() => resetInventoryDraft()} disabled={!canEdit || busy || itemIndex === null}>Reset</button>
          </div>
        </div>
        <div className="panel-scroll-body">
        {selectedItem ? (
          <>
            <div className="field-grid">
              <label><span>Name</span><input value={selectedItem.name ?? ""} disabled /></label>
              <label><span>Resref</span><input value={selectedItem.resref ?? ""} disabled /></label>
              <label><span>Category</span><input value={selectedItem.category.label} disabled /></label>
              <label><span>Stackable</span><input value={selectedItem.stackable ? "Yes" : "No"} disabled /></label>
              {canEditStackSize ? (
                <label>
                  <span>Stack Size</span>
                  <input
                    value={itemMetadataDraft.stack_size}
                    onChange={(event) =>
                      setItemMetadataDraft((current) => ({ ...current, stack_size: event.target.value }))
                    }
                    disabled={!canEdit || busy}
                  />
                </label>
              ) : null}
              <label>
                <span>Wiki</span>
                {selectedItem.wiki_url ? (
                  <a
                    className="field-link"
                    href={selectedItem.wiki_url}
                    onClick={(event) => {
                      event.preventDefault();
                      void handleWikiOpen(selectedItem.wiki_url!);
                    }}
                  >
                    Open item page
                  </a>
                ) : (
                  <input value="No wiki link" disabled />
                )}
              </label>
              {canEditMaterial ? (
                <label>
                  <span>Material</span>
                  <select
                    value={itemMetadataDraft.material}
                    onChange={(event) =>
                      setItemMetadataDraft((current) => ({ ...current, material: event.target.value }))
                    }
                    disabled={!canEdit || busy}
                  >
                    {selectedItem.material_options.map((option) => (
                      <option key={`material-${option.code}`} value={option.code}>
                        {`Tier ${option.tier} - ${option.name}`}
                      </option>
                    ))}
                  </select>
                </label>
              ) : null}
              <label><span>Item Level</span><input value={itemMetadataDraft.item_level} onChange={(event) => setItemMetadataDraft((current) => ({ ...current, item_level: event.target.value }))} disabled={!canEdit || busy} /></label>
            </div>
            {options.allowRemove ? (
              <div className="button-row">
                {canCloneBackpackItem ? (
                  <button onClick={() => void handleBackpackClone()} disabled={!canEdit || busy}>Clone Item</button>
                ) : null}
                <button onClick={() => void handleBackpackRemove()} disabled={!canEdit || busy}>Remove Item</button>
              </div>
            ) : null}
            <div className="properties-section">
              <div className="panel-heading"><h3>Properties</h3></div>
              <div className="property-table">
                <div className="property-row property-header">
                  <span>Property</span>
                  <span>Power</span>
                  <span>Action</span>
                </div>
                <div className="property-list scroll-region">
                  {itemPropertiesDraft.map((property, propertyIndex) => (
                    <div key={`${property.id}-${propertyIndex}`} className="property-row">
                      <select
                        value={property.id}
                        onChange={(event) => handlePropertyUpdateDraft("id", propertyIndex, event.target.value)}
                        disabled={!canEdit || busy}
                      >
                        {availableItemProperties.map((option) => (
                          <option key={`existing-property-${propertyIndex}-${option.id}`} value={option.id}>
                            {option.name ?? `Property ${option.id}`}
                          </option>
                        ))}
                      </select>
                      <input
                        value={property.power}
                        onChange={(event) => handlePropertyUpdateDraft("power", propertyIndex, event.target.value)}
                        disabled={!canEdit || busy}
                      />
                      <button onClick={() => handlePropertyRemoveDraft(propertyIndex)} disabled={!canEdit || busy}>Remove</button>
                    </div>
                  ))}
                </div>
                <div className="property-row add-property">
                  <select value={propertyDraft.property_id} onChange={(event) => setPropertyDraft((current) => ({ ...current, property_id: event.target.value }))} disabled={!canEdit || busy}>
                    {availableItemProperties.map((property) => (
                      <option key={`property-${property.id}`} value={property.id}>
                        {property.name ?? `Property ${property.id}`}
                      </option>
                    ))}
                  </select>
                  <input placeholder="Power" value={propertyDraft.power} onChange={(event) => setPropertyDraft((current) => ({ ...current, power: event.target.value }))} disabled={!canEdit || busy} />
                  <button onClick={() => handlePropertyAddDraft()} disabled={!canEdit || busy}>Add</button>
                </div>
              </div>
            </div>
          </>
        ) : <p className="muted">Select an item to edit metadata and properties.</p>}
        </div>
      </>
    );
  }

  return (
    <div className="app-shell">
      <header className="topbar">
        <div>
          <h1>Dragon Age Save Editor</h1>
          <p>Open, edit, and save as a new file.</p>
        </div>
        <div className="toolbar">
          {summary && screenshotDataUrl ? (
            <div className="topbar-preview" tabIndex={0}>
              <img className="topbar-preview-image" src={screenshotDataUrl} alt="Save screenshot" />
              <div className="topbar-preview-popover">
                <img src={screenshotDataUrl} alt="Save screenshot full preview" />
              </div>
            </div>
          ) : null}
          {summary && !screenshotDataUrl ? <span className="topbar-preview-empty">No screenshot</span> : null}
          <button onClick={() => void handleOpen()} disabled={busy}>Open Save</button>
          <button onClick={() => void handleSaveAs()} disabled={busy || !summary}>Save As</button>
        </div>
      </header>

      <div className="workspace">
        <nav className="main-tabbar">
          {visibleSections.map((entry) => (
            <button
              key={entry}
              className={section === entry ? "nav-link active" : "nav-link"}
              onClick={() => setSection(entry)}
            >
              {SECTION_TITLES[entry]}
            </button>
          ))}
        </nav>

        <main className="content">
          {!summary ? (
            <section className="panel empty-state">
              <h2>No save open</h2>
              <p>Use Open Save to start a single-document editing session.</p>
            </section>
          ) : null}

          {summary && section === "characters" ? (
            <section className="split-section">
              <div className="panel list-panel">
                <h2>Party</h2>
                {characters.map((entry) => (
                  <button
                    key={targetKey(entry.target)}
                    className={targetKey(entry.target) === characterKey ? "list-row active" : "list-row"}
                    onClick={() => setCharacterKey(targetKey(entry.target))}
                  >
                    {entry.name}
                  </button>
                ))}
              </div>
              <div className="panel detail-panel scroll-panel">
                <div className="panel-heading character-heading">
                  <h2>{character?.name ?? "Character"}</h2>
                  <div className="character-tabbar">
                    {CHARACTER_TABS.map((tab) => (
                      <button
                        key={tab}
                        className={characterTab === tab ? "nav-link active" : "nav-link"}
                        onClick={() => setCharacterTab(tab)}
                      >
                        {CHARACTER_TAB_TITLES[tab]}
                      </button>
                    ))}
                  </div>
                </div>
                <div className="panel-scroll-body">
                {character ? (
                  <>
                    {characterTab === "overview" ? (
                      <>
                        <div className="character-field-section">
                          <h3>Progress</h3>
                          <div className="field-grid">
                            <label>
                              <span>Level</span>
                              <input value={levelDraft} onChange={(event) => setLevelDraft(event.target.value)} disabled={!canEdit || busy} />
                            </label>
                            <label>
                              <span>Experience</span>
                              <input
                                value={experienceDraft}
                                onChange={(event) => setExperienceDraft(event.target.value)}
                                disabled={!canEdit || busy}
                                placeholder={character.experience === null ? "Add to save" : undefined}
                              />
                            </label>
                            <label>
                              <span>Approval</span>
                              <input
                                value={approvalDraft}
                                onChange={(event) => setApprovalDraft(event.target.value)}
                                disabled={!canEdit || busy || character?.approval === null}
                                placeholder={character?.approval === null ? "Unavailable for this character" : undefined}
                              />
                            </label>
                          </div>
                        </div>
                        <div className="character-field-section">
                          <h3>Attributes</h3>
                          <div className="field-grid">
                            {Object.entries(statsDraft).map(([key, value]) => (
                              <label key={key}>
                                <span>{titleCase(key)}</span>
                                <input
                                  value={value}
                                  onChange={(event) => setStatsDraft((current) => ({ ...current, [key]: event.target.value }))}
                                  disabled={!canEdit || busy}
                                />
                              </label>
                            ))}
                          </div>
                        </div>
                        <div className="character-field-section">
                          <h3>Point Pools</h3>
                          <div className="field-grid">
                            <label>
                              <span>Attribute Points</span>
                              <input
                                value={pointPoolsDraft.attribute_points ?? ""}
                                onChange={(event) =>
                                  setPointPoolsDraft((current) => ({ ...current, attribute_points: event.target.value }))
                                }
                                disabled={!canEdit || busy}
                                placeholder={character?.point_pools.attribute_points === null ? "Add to save" : undefined}
                              />
                            </label>
                            <label>
                              <span>Skill Points</span>
                              <input
                                value={pointPoolsDraft.skill_points ?? ""}
                                onChange={(event) =>
                                  setPointPoolsDraft((current) => ({ ...current, skill_points: event.target.value }))
                                }
                                disabled={!canEdit || busy}
                                placeholder={character?.point_pools.skill_points === null ? "Add to save" : undefined}
                              />
                            </label>
                            <label>
                              <span>Talent Points</span>
                              <input
                                value={pointPoolsDraft.talent_points ?? ""}
                                onChange={(event) =>
                                  setPointPoolsDraft((current) => ({ ...current, talent_points: event.target.value }))
                                }
                                disabled={!canEdit || busy}
                                placeholder={character?.point_pools.talent_points === null ? "Add to save" : undefined}
                              />
                            </label>
                            <label>
                              <span>Specialization Points</span>
                              <input
                                value={pointPoolsDraft.specialization_points ?? ""}
                                onChange={(event) =>
                                  setPointPoolsDraft((current) => ({ ...current, specialization_points: event.target.value }))
                                }
                                disabled={!canEdit || busy}
                                placeholder={character?.point_pools.specialization_points === null ? "Add to save" : undefined}
                              />
                            </label>
                          </div>
                        </div>
                        <div className="button-row">
                          <button onClick={() => void handleCharacterApply()} disabled={!canEdit || busy}>Apply</button>
                          <button onClick={() => resetCharacterDraft()} disabled={!canEdit || busy}>Reset</button>
                        </div>
                      </>
                    ) : null}

                    {characterTab === "abilities" ? (
                      <>
                        <div className="panel-heading">
                          <h2>Ability Lists</h2>
                          <div className="button-row">
                            <button onClick={() => void handleAbilityApplyAll()} disabled={!canEdit || busy}>Apply</button>
                            <button onClick={() => resetAbilityDrafts()} disabled={!canEdit || busy}>Reset</button>
                          </div>
                        </div>
                        <div className="ability-grid">
                          {visibleAbilityKinds.map((kind) => {
                            const abilityOptions = coreAbilityOptions(kind);
                            const treeAbilities = visibleTreeAbilities(kind);
                            const selectedAbilityValue = abilityOptions.some(
                              (ability) => ability.id.toString() === selectedAbilityToAdd[kind],
                            )
                              ? selectedAbilityToAdd[kind]
                              : abilityOptions[0]?.id.toString() ?? "";
                            return (
                            <div key={kind} className="ability-panel">
                              <div className="panel-heading">
                                <h3>{titleCase(kind)}</h3>
                              </div>
                              <div className="ability-add-row">
                                <select
                                  value={selectedAbilityValue}
                                  onChange={(event) =>
                                    setSelectedAbilityToAdd((current) => ({ ...current, [kind]: event.target.value }))
                                  }
                                  disabled={!canEdit || busy}
                                >
                                  {abilityOptions.map((ability) => (
                                    <option key={`${kind}-available-${ability.id}`} value={ability.id}>
                                      {abilityLabel(ability)}
                                    </option>
                                  ))}
                                </select>
                                <button onClick={() => handleAbilityAdd(kind)} disabled={!canEdit || busy}>
                                  Add
                                </button>
                              </div>
                              <p className="muted editor-help">
                                Add core tree entries from the dropdown. Unlocked trees show addable and removable abilities below.
                              </p>
                              <div className="ability-preview scroll-region">
                                {groupedAbilities(
                                  kind,
                                  treeAbilities,
                                  visibleAbilities(kind, availableAbilities[kind]),
                                ).map((group) => (
                                  <div key={`${kind}-${group.label}`} className="ability-group">
                                    <h4>{group.label}</h4>
                                    {group.abilities.map((ability) => {
                                      const selected = abilityDrafts[kind].some((entry) => entry.id === ability.id);
                                      const locked = selected && abilityIsLocked(kind, ability.id);
                                      return (
                                        <div key={`${kind}-${ability.id}`} className="ability-entry">
                                          <div className="ability-entry-header">
                                            <strong>{ability.name ?? `Ability ${ability.id}`}</strong>
                                            {selected ? (
                                              <button
                                                onClick={() => handleAbilityRemove(kind, ability.id)}
                                                disabled={!canEdit || busy || locked}
                                              >
                                                Remove
                                              </button>
                                            ) : (
                                              <button
                                                onClick={() => handleVisibleAbilityAdd(kind, ability.id)}
                                                disabled={!canEdit || busy}
                                              >
                                                Add
                                              </button>
                                            )}
                                          </div>
                                          <span>{abilityLabel(ability)}</span>
                                          {locked ? (
                                            <span className="muted">Required by another selected ability.</span>
                                          ) : null}
                                        </div>
                                      );
                                    })}
                                  </div>
                                ))}
                              </div>
                            </div>
                            );
                          })}
                        </div>
                      </>
                    ) : null}

                    {characterTab === "equipment" ? (
                      <div className="character-equipment-layout">
                        {renderItemList()}
                        <div className="equipment-detail scroll-panel">
                          {renderItemEditor({ allowRemove: false })}
                        </div>
                      </div>
                    ) : null}
                  </>
                ) : <p className="muted">Choose a character to edit.</p>}
                </div>
              </div>
            </section>
          ) : null}

          {summary && section === "inventory" ? (
            <section className="split-section inventory-layout">
              <div className="panel list-panel">
                <h2>Inventory</h2>
                <div className="inventory-money-panel">
                  <h3>Money</h3>
                  <div className="inline-form">
                    <input value={moneyDraft} onChange={(event) => setMoneyDraft(event.target.value)} />
                  </div>
                  <div className="button-row">
                    <button onClick={() => void handleMoneyApply()} disabled={!canEdit || busy}>Apply</button>
                    <button onClick={() => resetMoneyDraft()} disabled={!canEdit || busy}>Reset</button>
                  </div>
                </div>
                <h3>Backpack</h3>
                {renderItemList()}
              </div>
              <div className="panel detail-panel scroll-panel">
                {renderItemEditor({ allowRemove: true })}
              </div>
            </section>
          ) : null}

          {summary && section === "recipes" ? (
            <section className="panel detail-panel scroll-panel">
              <div className="panel-heading">
                <h2>Crafting Recipes</h2>
                <div className="button-row">
                  <button onClick={() => void handleCraftingRecipesApply()} disabled={!canEdit || busy}>Apply</button>
                  <button onClick={() => resetCraftingRecipeDrafts()} disabled={!canEdit || busy}>Reset</button>
                </div>
              </div>
              <div className="panel-scroll-body">
                <div className="recipe-checklist scroll-region">
                  {sortedRecipeChecklistIds().length > 0 ? (
                    groupedRecipeChecklistIds().map((group) => (
                      <div key={`recipe-group-${group.category}`} className="recipe-group">
                        <h3>{group.category}</h3>
                        {group.ids.map((recipeId) => (
                          <label key={`recipe-${recipeId}`} className="check-row">
                            <input
                              type="checkbox"
                              checked={craftingRecipeDrafts.includes(recipeId)}
                              onChange={(event) => handleCraftingRecipeToggle(recipeId, event.target.checked)}
                              disabled={!canEdit || busy || !recipeIsKnown(recipeId)}
                            />
                            <span>{recipeLabel(recipeId)}{recipeIsKnown(recipeId) ? "" : " (unknown, preserved)"}</span>
                          </label>
                        ))}
                      </div>
                    ))
                  ) : (
                    <p className="muted">No recipe catalog entries are available for this save.</p>
                  )}
                </div>
              </div>
            </section>
          ) : null}

          {summary && section === "plot_flags" ? (
            <section className="panel detail-panel scroll-panel">
              <div className="panel-heading">
                <h2>DA2 Plot Flags</h2>
                <div className="button-row">
                  <button
                    onClick={() => void handlePlotFlagsApply()}
                    disabled={!canEdit || busy || summary.preferred_game !== "da2"}
                  >
                    Apply
                  </button>
                  <button
                    onClick={() => resetPlotFlagDrafts()}
                    disabled={!canEdit || busy || summary.preferred_game !== "da2"}
                  >
                    Reset
                  </button>
                </div>
              </div>
              <div className="panel-scroll-body">
                {summary.preferred_game === "da2" ? (
                  <div className="plot-flags-layout">
                    <div className="plot-choice-list">
                      <h3>Choices</h3>
                      {groupedPlotIntegers().map((group) => (
                        <div key={`plot-int-group-${group.category}`} className="plot-group">
                          <h4>{group.category}</h4>
                          {group.flags.map((flag) => (
                            <fieldset key={`plot-int-${flag.id}`} className="plot-radio-group">
                              <legend>{flag.description}</legend>
                              {flag.options.map((option) => (
                                <label key={`plot-int-${flag.id}-${option.value}`} className="radio-row">
                                  <input
                                    type="radio"
                                    name={`plot-int-${flag.id}`}
                                    checked={plotIntegerDrafts[flag.id] === option.value}
                                    onChange={() => handlePlotIntegerChange(flag.id, option.value)}
                                    disabled={!canEdit || busy}
                                  />
                                  <span>{option.label}</span>
                                </label>
                              ))}
                              <span className="plot-flag-code">{flag.name} ({flag.id})</span>
                            </fieldset>
                          ))}
                        </div>
                      ))}
                    </div>
                    <div className="plot-boolean-list">
                      <h3>Booleans</h3>
                      {groupedPlotBooleans().map((group) => (
                        <div key={`plot-bool-group-${group.category}`} className="plot-group">
                          <h4>{group.category}</h4>
                          {group.flags.map((flag) => (
                            <label key={`plot-bool-${flag.id}`} className="check-row">
                              <input
                                type="checkbox"
                                checked={Boolean(plotBooleanDrafts[flag.id])}
                                onChange={(event) => handlePlotBooleanToggle(flag.id, event.target.checked)}
                                disabled={!canEdit || busy}
                              />
                              <span>
                                {flag.description}
                                <small className="plot-flag-code">{flag.name} ({flag.id})</small>
                              </span>
                            </label>
                          ))}
                        </div>
                      ))}
                    </div>
                  </div>
                ) : (
                  <p className="muted">Plot flag editing is available for DA2 saves.</p>
                )}
              </div>
            </section>
          ) : null}
        </main>
      </div>

      <footer className="statusbar">
        <span>{summary ? `Path: ${summary.source_path}` : "No document loaded"}</span>
        <span>{summary ? gameLabel(summary.preferred_game) : "Idle"}</span>
        <span>{summary ? (summary.dirty ? "Dirty" : "Saved") : "Idle"}</span>
        <span>{summary ? "Ready" : "No Save Loaded"}</span>
      </footer>

      {error ? (
        <div className="error-banner">
          <span>{error}</span>
          <button className="dismiss-button" onClick={() => setError(null)}>Dismiss</button>
        </div>
      ) : null}
    </div>
  );
}

export default App;
