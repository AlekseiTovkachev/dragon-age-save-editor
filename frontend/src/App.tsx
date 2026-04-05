import { useEffect, useMemo, useState } from "react";
import { open, save } from "@tauri-apps/plugin-dialog";
import { executeCommand, hasDocument, openDocument, toErrorMessage } from "./api";
import type {
  Ability,
  AbilityListKind,
  Character,
  CharacterSummary,
  CharacterTarget,
  IndexedItem,
  InventoryContainer,
  Item,
  ItemProperty,
  SelectableItemProperty,
  SaveSummary,
} from "./types";

type Section = "overview" | "characters" | "abilities" | "inventory";

const MAIN_TARGET: CharacterTarget = "main_character";
const SECTIONS: Section[] = ["overview", "characters", "abilities", "inventory"];
const SECTION_TITLES: Record<Section, string> = {
  overview: "Overview",
  characters: "Characters",
  abilities: "Abilities",
  inventory: "Inventory",
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

function parseSignedNumber(value: string): number | null {
  if (value.trim() === "") {
    return null;
  }
  const parsed = Number(value);
  return Number.isInteger(parsed) ? parsed : null;
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

function cloneAbilities(abilities: Ability[]): Ability[] {
  return abilities.map((ability) => ({ ...ability, core_ids: [...ability.core_ids] }));
}

function cloneItemProperties(properties: ItemProperty[]): ItemProperty[] {
  return properties.map((property) => ({ ...property }));
}

function App() {
  const [section, setSection] = useState<Section>("overview");
  const [summary, setSummary] = useState<SaveSummary | null>(null);
  const [screenshotDataUrl, setScreenshotDataUrl] = useState<string | null>(null);
  const [characters, setCharacters] = useState<CharacterSummary[]>([]);
  const [characterKey, setCharacterKey] = useState("main");
  const [inventoryMode, setInventoryMode] = useState<"backpack" | "equipment">("backpack");
  const [equipmentCharacterKey, setEquipmentCharacterKey] = useState("main");
  const [character, setCharacter] = useState<Character | null>(null);
  const [items, setItems] = useState<IndexedItem[]>([]);
  const [itemIndex, setItemIndex] = useState<number | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [moneyDraft, setMoneyDraft] = useState("");
  const [statsDraft, setStatsDraft] = useState<Record<string, string>>({});
  const [levelDraft, setLevelDraft] = useState("");
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
  const [itemMetadataDraft, setItemMetadataDraft] = useState({
    item_cost: "",
    material: "",
    item_level: "",
  });
  const [itemPropertiesDraft, setItemPropertiesDraft] = useState<ItemProperty[]>([]);
  const [propertyDraft, setPropertyDraft] = useState({ property_id: "", power: "" });
  const [replacementDraft, setReplacementDraft] = useState({
    resref: "",
    item_cost: "",
    material: "",
    item_level: "",
  });

  const selectedCharacterTarget = useMemo(
    () => characters.find((entry) => targetKey(entry.target) === characterKey)?.target ?? MAIN_TARGET,
    [characters, characterKey],
  );
  const selectedInventoryContainer = useMemo<InventoryContainer>(() => {
    if (inventoryMode === "backpack") {
      return "backpack";
    }
    const target =
      characters.find((entry) => targetKey(entry.target) === equipmentCharacterKey)?.target ?? MAIN_TARGET;
    return { equipment: { target } };
  }, [characters, equipmentCharacterKey, inventoryMode]);
  const selectedItem = useMemo(
    () => items.find((entry) => entry.index === itemIndex)?.item ?? null,
    [itemIndex, items],
  );
  const canEdit = Boolean(summary);

  useEffect(() => {
    void hasDocument().then(async (present) => {
      if (present) {
        await refreshSummary();
        await refreshDocumentAssets();
        await refreshCharacters();
        await refreshAvailableAbilities();
        await refreshAvailableItemProperties();
      }
    });
  }, []);

  useEffect(() => {
    if (summary) {
      setMoneyDraft(summary.money.toString());
    }
  }, [summary]);

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
      item_cost: selectedItem.item_cost?.toString() ?? "",
      material: selectedItem.material?.toString() ?? "",
      item_level: selectedItem.item_level?.toString() ?? "",
    });
    setReplacementDraft({
      resref: selectedItem.resref ?? "",
      item_cost: selectedItem.item_cost?.toString() ?? "",
      material: selectedItem.material?.toString() ?? "",
      item_level: selectedItem.item_level?.toString() ?? "",
    });
    setItemPropertiesDraft(cloneItemProperties(selectedItem.properties));
  }, [selectedItem]);

  useEffect(() => {
    if (summary) {
      void loadCharacter(selectedCharacterTarget);
    }
  }, [selectedCharacterTarget, summary]);

  useEffect(() => {
    if (summary) {
      void refreshItems();
    }
  }, [equipmentCharacterKey, inventoryMode, summary]);

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
        setEquipmentCharacterKey((current) =>
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
        setAvailableAbilities((current) => ({ ...current, [list]: response.abilities }));
        setSelectedAbilityToAdd((current) => ({
          ...current,
          [list]: response.abilities[0] ? response.abilities[0].id.toString() : "",
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

  async function loadCharacter(target: CharacterTarget) {
    const response = await executeCommand({ command: "get_character", target });
    if (response.result === "character") {
      setCharacter(response.character);
    }
  }

  async function refreshItems() {
    const equipmentTarget =
      selectedInventoryContainer === "backpack" ? null : selectedInventoryContainer.equipment.target;
    const response = await (inventoryMode === "backpack"
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
      setSection("overview");
    });
  }

  async function handleSaveAs() {
    if (!summary) {
      return;
    }
    const path = await save({
      title: "Save Edited File As",
      defaultPath: summary.source_path.replace(/\.das$/i, "-edited.das"),
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
      if (selectedCharacterTarget !== "main_character") {
        const approval = parseSignedNumber(approvalDraft);
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
    return abilityDrafts[list].some(
      (ability) => ability.id !== abilityId && ability.core_ids.includes(abilityId),
    );
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
    const selectedId = Number(selectedAbilityToAdd[list]);
    const selected = availableAbilities[list].find((ability) => ability.id === selectedId);
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
      { id: propertyId, name: selectedProperty?.name ?? null, power },
    ]);
    setPropertyDraft((current) => ({ ...current, power: "" }));
  }

  function handlePropertyRemoveDraft(propertyIndex: number) {
    setItemPropertiesDraft((current) => current.filter((_, index) => index !== propertyIndex));
  }

  function handlePropertyUpdateDraft(kind: "id" | "power", propertyIndex: number, raw: string) {
    const value = parseNumber(raw);
    if (value === null) {
      setError("Property value must be a valid number.");
      return;
    }
    setItemPropertiesDraft((current) =>
      current.map((property, index) => {
        if (index !== propertyIndex) {
          return property;
        }
        if (kind === "id") {
          const selectedProperty = availableItemProperties.find((entry) => entry.id === value);
          return { ...property, id: value, name: selectedProperty?.name ?? null };
        }
        return { ...property, power: value };
      }),
    );
  }

  async function handleInventoryApply() {
    if (itemIndex === null) {
      return;
    }
    await run(async () => {
      let response = await executeCommand({
        command: "patch_item_metadata",
        container: selectedInventoryContainer,
        index: itemIndex,
        patch: {
          item_cost: parseNumber(itemMetadataDraft.item_cost),
          material: parseNumber(itemMetadataDraft.material),
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
        if (sourceProperties[index].id !== draftProperties[index].id) {
          response = await executeCommand({
            command: "set_item_property_id",
            container: selectedInventoryContainer,
            index: itemIndex,
            property_index: index,
            property_id: draftProperties[index].id,
          });
        }
        if (sourceProperties[index].power !== draftProperties[index].power) {
          response = await executeCommand({
            command: "set_item_property_power",
            container: selectedInventoryContainer,
            index: itemIndex,
            property_index: index,
            power: draftProperties[index].power,
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
          response = await executeCommand({
            command: "add_item_property",
            container: selectedInventoryContainer,
            index: itemIndex,
            property_id: draftProperties[index].id,
            power: draftProperties[index].power,
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
      item_cost: selectedItem.item_cost?.toString() ?? "",
      material: selectedItem.material?.toString() ?? "",
      item_level: selectedItem.item_level?.toString() ?? "",
    });
    setItemPropertiesDraft(cloneItemProperties(selectedItem.properties));
    setPropertyDraft((current) => ({ ...current, power: "" }));
  }

  async function handleBackpackRemove() {
    if (inventoryMode !== "backpack" || itemIndex === null) {
      return;
    }
    await run(async () => {
      await executeCommand({ command: "remove_backpack_item", index: itemIndex });
      await refreshSummary();
      await refreshItems();
    });
  }

  async function handleBackpackReplace() {
    if (inventoryMode !== "backpack" || itemIndex === null) {
      return;
    }
    await run(async () => {
      const response = await executeCommand({
        command: "replace_backpack_item",
        index: itemIndex,
        replacement: {
          resref: replacementDraft.resref,
          item_cost: parseNumber(replacementDraft.item_cost),
          material: parseNumber(replacementDraft.material),
          item_level: parseNumber(replacementDraft.item_level),
        },
      });
      if (response.result === "item") {
        updateVisibleItem(response.index, response.item);
      }
      await refreshSummary();
    });
  }

  return (
    <div className="app-shell">
      <header className="topbar">
        <div>
          <h1>Dragon Age Save Editor</h1>
          <p>Open, edit, and save as a new file.</p>
        </div>
        <div className="toolbar">
          <button onClick={() => void handleOpen()} disabled={busy}>Open Save</button>
          <button onClick={() => void handleSaveAs()} disabled={busy || !summary}>Save As</button>
        </div>
      </header>

      <div className="workspace">
        <aside className="sidebar">
          {SECTIONS.map((entry) => (
            <button
              key={entry}
              className={section === entry ? "nav-link active" : "nav-link"}
              onClick={() => setSection(entry)}
            >
              {SECTION_TITLES[entry]}
            </button>
          ))}
        </aside>

        <main className="content">
          {!summary ? (
            <section className="panel empty-state">
              <h2>No save open</h2>
              <p>Use Open Save to start a single-document editing session.</p>
            </section>
          ) : null}

          {summary && section === "overview" ? (
            <section className="panel overview-grid">
              <div className="metric-card"><span>Main Character</span><strong>{summary.main_character_name}</strong></div>
              <div className="metric-card"><span>Companions</span><strong>{summary.companion_count}</strong></div>
              <div className="metric-card"><span>Backpack Items</span><strong>{summary.backpack_count}</strong></div>
              <div className="metric-card"><span>Status</span><strong>{summary.dirty ? "Unsaved Changes" : "Saved"}</strong></div>
              <div className="section-panel preview-panel scroll-region">
                <h2>Save Preview</h2>
                {screenshotDataUrl ? (
                  <div className="save-preview-frame">
                    <img className="save-preview" src={screenshotDataUrl} alt="Save screenshot" />
                  </div>
                ) : (
                  <p className="muted">No save screenshot was found next to this save file.</p>
                )}
              </div>
              <div className="section-panel">
                <h2>Money</h2>
                <div className="inline-form">
                  <input value={moneyDraft} onChange={(event) => setMoneyDraft(event.target.value)} />
                </div>
                <div className="button-row">
                  <button onClick={() => void handleMoneyApply()} disabled={!canEdit || busy}>Apply</button>
                  <button onClick={() => resetMoneyDraft()} disabled={!canEdit || busy}>Reset</button>
                </div>
              </div>
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
                <h2>{character?.name ?? "Character"}</h2>
                <div className="panel-scroll-body">
                {character ? (
                  <>
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
                      <label>
                        <span>Level</span>
                        <input value={levelDraft} onChange={(event) => setLevelDraft(event.target.value)} disabled={!canEdit || busy} />
                      </label>
                      <label>
                        <span>Attribute Points</span>
                        <input
                          value={pointPoolsDraft.attribute_points ?? ""}
                          onChange={(event) =>
                            setPointPoolsDraft((current) => ({ ...current, attribute_points: event.target.value }))
                          }
                          disabled={!canEdit || busy}
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
                        />
                      </label>
                      {selectedCharacterTarget !== "main_character" ? (
                        <label>
                          <span>Approval</span>
                          <input
                            value={approvalDraft}
                            onChange={(event) => setApprovalDraft(event.target.value)}
                            disabled={!canEdit || busy}
                          />
                        </label>
                      ) : null}
                    </div>
                    <div className="button-row">
                      <button onClick={() => void handleCharacterApply()} disabled={!canEdit || busy}>Apply</button>
                      <button onClick={() => resetCharacterDraft()} disabled={!canEdit || busy}>Reset</button>
                    </div>
                  </>
                ) : <p className="muted">Choose a character to edit.</p>}
                </div>
              </div>
            </section>
          ) : null}

          {summary && section === "abilities" ? (
            <section className="split-section">
              <div className="panel list-panel">
                <h2>Characters</h2>
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
                <div className="panel-heading">
                  <h2>Ability Lists</h2>
                  <div className="button-row">
                    <button onClick={() => void handleAbilityApplyAll()} disabled={!canEdit || busy}>Apply</button>
                    <button onClick={() => resetAbilityDrafts()} disabled={!canEdit || busy}>Reset</button>
                  </div>
                </div>
                <div className="ability-grid panel-scroll-body">
                  {(["skills", "talents", "spells"] as AbilityListKind[]).map((kind) => (
                    <div key={kind} className="ability-panel">
                      <div className="panel-heading">
                        <h3>{titleCase(kind)}</h3>
                      </div>
                      <div className="ability-add-row">
                        <select
                          value={selectedAbilityToAdd[kind]}
                          onChange={(event) =>
                            setSelectedAbilityToAdd((current) => ({ ...current, [kind]: event.target.value }))
                          }
                          disabled={!canEdit || busy}
                        >
                          {availableAbilities[kind].map((ability) => (
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
                        Remove entries directly from the current list. Entries with dependent children stay disabled.
                      </p>
                      <div className="ability-preview scroll-region">
                        {abilityDrafts[kind].map((ability) => (
                          <div key={`${kind}-${ability.id}`} className="ability-entry">
                            <div className="ability-entry-header">
                              <strong>{ability.name ?? `Ability ${ability.id}`}</strong>
                              <button
                                onClick={() => handleAbilityRemove(kind, ability.id)}
                                disabled={!canEdit || busy || abilityIsLocked(kind, ability.id)}
                              >
                                Delete
                              </button>
                            </div>
                            <span>{abilityLabel(ability)}</span>
                            {abilityIsLocked(kind, ability.id) ? (
                              <span className="muted">Required by another selected ability.</span>
                            ) : null}
                          </div>
                        ))}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </section>
          ) : null}

          {summary && section === "inventory" ? (
            <section className="split-section inventory-layout">
              <div className="panel list-panel">
                <h2>Inventory</h2>
                <div className="inventory-switcher">
                  <button className={inventoryMode === "backpack" ? "nav-link active" : "nav-link"} onClick={() => setInventoryMode("backpack")}>Backpack</button>
                  <button className={inventoryMode === "equipment" ? "nav-link active" : "nav-link"} onClick={() => setInventoryMode("equipment")}>Equipment</button>
                </div>
                {inventoryMode === "equipment" ? (
                  <select value={equipmentCharacterKey} onChange={(event) => setEquipmentCharacterKey(event.target.value)}>
                    {characters.map((entry) => (
                      <option key={targetKey(entry.target)} value={targetKey(entry.target)}>{entry.name}</option>
                    ))}
                  </select>
                ) : null}
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
              </div>
              <div className="panel detail-panel scroll-panel">
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
                      <label><span>Item Cost</span><input value={itemMetadataDraft.item_cost} onChange={(event) => setItemMetadataDraft((current) => ({ ...current, item_cost: event.target.value }))} disabled={!canEdit || busy} /></label>
                      <label><span>Material</span><input value={itemMetadataDraft.material} onChange={(event) => setItemMetadataDraft((current) => ({ ...current, material: event.target.value }))} disabled={!canEdit || busy} /></label>
                      <label><span>Item Level</span><input value={itemMetadataDraft.item_level} onChange={(event) => setItemMetadataDraft((current) => ({ ...current, item_level: event.target.value }))} disabled={!canEdit || busy} /></label>
                      <label><span>Equipment Slot</span><input value={selectedItem.equipment_slot ?? ""} disabled /></label>
                    </div>
                    <div className="button-row">
                      {inventoryMode === "backpack" ? (
                        <>
                          <button onClick={() => void handleBackpackRemove()} disabled={!canEdit || busy}>Remove Item</button>
                          <button onClick={() => void handleBackpackReplace()} disabled={!canEdit || busy}>Same Resref Replace</button>
                        </>
                      ) : null}
                    </div>
                    {inventoryMode === "backpack" ? (
                      <div className="replacement-box">
                        <h3>Backpack Replacement</h3>
                        <div className="field-grid">
                          <label><span>Resref</span><input value={replacementDraft.resref} onChange={(event) => setReplacementDraft((current) => ({ ...current, resref: event.target.value }))} disabled={!canEdit || busy} /></label>
                          <label><span>Item Cost</span><input value={replacementDraft.item_cost} onChange={(event) => setReplacementDraft((current) => ({ ...current, item_cost: event.target.value }))} disabled={!canEdit || busy} /></label>
                          <label><span>Material</span><input value={replacementDraft.material} onChange={(event) => setReplacementDraft((current) => ({ ...current, material: event.target.value }))} disabled={!canEdit || busy} /></label>
                          <label><span>Item Level</span><input value={replacementDraft.item_level} onChange={(event) => setReplacementDraft((current) => ({ ...current, item_level: event.target.value }))} disabled={!canEdit || busy} /></label>
                        </div>
                      </div>
                    ) : null}
                    <div className="properties-section">
                      <div className="panel-heading"><h3>Properties</h3></div>
                      <div className="property-list scroll-region">
                        {itemPropertiesDraft.map((property, propertyIndex) => (
                          <div key={`${property.id}-${propertyIndex}`} className="property-row">
                            <input
                              value={property.id}
                              onChange={(event) => handlePropertyUpdateDraft("id", propertyIndex, event.target.value)}
                              disabled={!canEdit || busy}
                            />
                            <input value={property.name ?? `Property ${property.id}`} disabled />
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
                        <button onClick={() => handlePropertyAddDraft()} disabled={!canEdit || busy}>Add Property</button>
                      </div>
                    </div>
                  </>
                ) : <p className="muted">Select an item to edit metadata and properties.</p>}
                </div>
              </div>
            </section>
          ) : null}
        </main>
      </div>

      <footer className="statusbar">
        <span>{summary ? `Path: ${summary.source_path}` : "No document loaded"}</span>
        <span>{summary ? (summary.dirty ? "Dirty" : "Saved") : "Idle"}</span>
        <span>{summary ? "Ready" : "No Save Loaded"}</span>
      </footer>

      {error ? <div className="error-banner">{error}</div> : null}
    </div>
  );
}

export default App;
