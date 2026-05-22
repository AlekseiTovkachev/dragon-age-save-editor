/* eslint-disable react-hooks/set-state-in-effect -- Inventory form drafts intentionally mirror the selected item and summary snapshots. */
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { openUrl } from "@tauri-apps/plugin-opener";
import { executeCommand, expectResult } from "../../api";
import { useDraftCheckpoint } from "../../hooks/useDraftCheckpoint";
import { MAIN_TARGET, toItemPropertyDrafts } from "../../lib/itemUtils";
import { parseNumber } from "../../lib/format";
import type { ItemPropertyDraft } from "../../lib/itemUtils";
import type { IndexedItem, InventoryContainer, Item, SaveCommand, SaveSummary, SelectableItemProperty } from "../../types";
import type { AsyncRun } from "../shared/types";
import { planInventoryDraftCommands } from "./inventoryCommandPlanner";

type UseInventoryEditorOptions = {
  summary: SaveSummary | null;
  container: InventoryContainer;
  isBackpackInventory: boolean;
  run: AsyncRun;
  setError: (message: string | null) => void;
};

export type ItemMetadataDraft = {
  material: string;
  item_level: string;
  stack_size: string;
};

export type PropertyDraft = {
  property_id: string;
  power: string;
};

type InventoryDraftCheckpoint = {
  moneyDraft: string;
  itemDrafts: Record<string, InventoryItemDraft>;
  removedBackpackIndexes: number[];
  clonedBackpackItems: PendingBackpackClone[];
};

export type InventoryItemDraft = {
  itemMetadataDraft: ItemMetadataDraft;
  itemPropertiesDraft: ItemPropertyDraft[];
  propertyDraft: PropertyDraft;
};

type PendingBackpackClone = {
  tempIndex: number;
  sourceIndex: number;
  item: Item;
};

export type InventoryCloneSpec = Pick<PendingBackpackClone, "tempIndex" | "sourceIndex"> & {
  batch: SaveCommand[];
};

type InventoryCommandPlan = {
  clones?: InventoryCloneSpec[];
  removes?: number[];
  batch: SaveCommand[];
};

type DraftItemEntry = {
  index: number;
  item: Item;
  draft: InventoryItemDraft;
};

type LoadedInventoryContainer = {
  container: InventoryContainer;
  items: IndexedItem[];
};

const cloneItemPropertiesDraft = (properties: ItemPropertyDraft[]) => properties.map((property) => ({ ...property }));

const cloneInventoryItemDraft = (draft: InventoryItemDraft): InventoryItemDraft => ({
  itemMetadataDraft: { ...draft.itemMetadataDraft },
  itemPropertiesDraft: cloneItemPropertiesDraft(draft.itemPropertiesDraft),
  propertyDraft: { ...draft.propertyDraft },
});

const cloneItem = (item: Item): Item => ({
  ...item,
  category: { ...item.category },
  material_profile: item.material_profile ? { ...item.material_profile } : null,
  material_info: item.material_info ? { ...item.material_info } : null,
  material_options: item.material_options.map((option) => ({ ...option })),
  properties: item.properties.map((property) => ({ ...property })),
});

const clonePendingBackpackClone = (entry: PendingBackpackClone): PendingBackpackClone => ({
  tempIndex: entry.tempIndex,
  sourceIndex: entry.sourceIndex,
  item: cloneItem(entry.item),
});

const cloneInventoryCheckpoint = (checkpoint: InventoryDraftCheckpoint): InventoryDraftCheckpoint => ({
  moneyDraft: checkpoint.moneyDraft,
  itemDrafts: Object.fromEntries(
    Object.entries(checkpoint.itemDrafts).map(([key, draft]) => [key, cloneInventoryItemDraft(draft)]),
  ),
  removedBackpackIndexes: [...checkpoint.removedBackpackIndexes],
  clonedBackpackItems: checkpoint.clonedBackpackItems.map(clonePendingBackpackClone),
});

function withoutDraftsForContainer(
  drafts: Record<string, InventoryItemDraft>,
  containerKey: string,
): Record<string, InventoryItemDraft> {
  return Object.fromEntries(
    Object.entries(drafts).filter(([key]) => !key.startsWith(`${containerKey}:`)),
  );
}

function keyForContainer(container: InventoryContainer) {
  if (container === "backpack") {
    return "backpack";
  }
  const target = container.equipment.target;
  return target === "main_character" ? "equipment:main" : `equipment:companion:${target.companion.index}`;
}

function parseDraftItemKey(key: string): { containerKey: string; index: number } | null {
  const separatorIndex = key.lastIndexOf(":");
  if (separatorIndex < 0) {
    return null;
  }
  const index = Number(key.slice(separatorIndex + 1));
  if (!Number.isFinite(index)) {
    return null;
  }
  return { containerKey: key.slice(0, separatorIndex), index };
}

export function useInventoryEditor({
  summary,
  container,
  isBackpackInventory,
  run,
  setError,
}: UseInventoryEditorOptions) {
  const [items, setItems] = useState<IndexedItem[]>([]);
  const [loadedContainerKey, setLoadedContainerKey] = useState<string | null>(null);
  const [itemIndex, setItemIndex] = useState<number | null>(null);
  const [removedBackpackIndexes, setRemovedBackpackIndexes] = useState<number[]>([]);
  const [clonedBackpackItems, setClonedBackpackItems] = useState<PendingBackpackClone[]>([]);
  const [availableItemProperties, setAvailableItemProperties] = useState<SelectableItemProperty[]>([]);
  const [moneyDraft, setMoneyDraft] = useState("");
  const [itemMetadataDraft, setItemMetadataDraft] = useState<ItemMetadataDraft>({
    material: "",
    item_level: "",
    stack_size: "",
  });
  const [itemPropertiesDraft, setItemPropertiesDraft] = useState<ItemPropertyDraft[]>([]);
  const [propertyDraft, setPropertyDraft] = useState<PropertyDraft>({ property_id: "", power: "" });
  const draftCheckpoint = useDraftCheckpoint<InventoryDraftCheckpoint>({ clone: cloneInventoryCheckpoint });
  const itemDrafts = useRef<Record<string, InventoryItemDraft>>({});
  const currentItemDraftKey = useRef<string | null>(null);
  const itemIndexRef = useRef<number | null>(null);
  const moneyDraftRef = useRef(moneyDraft);
  const itemMetadataDraftRef = useRef(itemMetadataDraft);
  const itemPropertiesDraftRef = useRef(itemPropertiesDraft);
  const propertyDraftRef = useRef(propertyDraft);
  const selectedItemRef = useRef<Item | null>(null);
  const loadedSummaryKey = useRef<string | null>(null);
  const loadedItemsByContainer = useRef<Record<string, LoadedInventoryContainer>>({});
  const nextTemporaryBackpackIndex = useRef(-1);

  const containerKey = useMemo(() => keyForContainer(container), [container]);
  const visibleItems = useMemo(() => {
    if (loadedContainerKey !== containerKey) {
      return [];
    }
    if (container !== "backpack") {
      return items;
    }
    const removed = new Set(removedBackpackIndexes);
    return [
      ...items.filter((entry) => !removed.has(entry.index)),
      ...clonedBackpackItems.map((entry) => ({ index: entry.tempIndex, item: entry.item })),
    ];
  }, [clonedBackpackItems, container, containerKey, items, loadedContainerKey, removedBackpackIndexes]);

  const selectedItem = useMemo(
    () => visibleItems.find((entry) => entry.index === itemIndex)?.item ?? null,
    [itemIndex, visibleItems],
  );

  const selectedItemDraftKey = selectedItem && itemIndex !== null ? `${containerKey}:${itemIndex}` : null;

  useEffect(() => {
    moneyDraftRef.current = moneyDraft;
    itemIndexRef.current = itemIndex;
    itemMetadataDraftRef.current = itemMetadataDraft;
    itemPropertiesDraftRef.current = itemPropertiesDraft;
    propertyDraftRef.current = propertyDraft;
    selectedItemRef.current = selectedItem;
  }, [itemIndex, itemMetadataDraft, itemPropertiesDraft, moneyDraft, propertyDraft, selectedItem]);

  const storeCurrentItemDraft = useCallback(() => {
    if (!currentItemDraftKey.current) {
      return;
    }
    itemDrafts.current[currentItemDraftKey.current] = {
      itemMetadataDraft: { ...itemMetadataDraftRef.current },
      itemPropertiesDraft: cloneItemPropertiesDraft(itemPropertiesDraftRef.current),
      propertyDraft: { ...propertyDraftRef.current },
    };
  }, []);

  const draftFromItem = useCallback((item: Item): InventoryItemDraft => ({
    itemMetadataDraft: {
      material: item.material?.toString() ?? "",
      item_level: item.item_level?.toString() ?? "",
      stack_size: item.item_stacksize?.toString() ?? "1",
    },
    itemPropertiesDraft: toItemPropertyDrafts(item.properties),
    propertyDraft: { property_id: propertyDraftRef.current.property_id, power: "" },
  }), []);

  const applyItemDraft = useCallback((draft: InventoryItemDraft) => {
    setItemMetadataDraft({ ...draft.itemMetadataDraft });
    setItemPropertiesDraft(cloneItemPropertiesDraft(draft.itemPropertiesDraft));
    setPropertyDraft({ ...draft.propertyDraft });
  }, []);

  const canEditStackSize = Boolean(selectedItem && isBackpackInventory && selectedItem.stackable);
  const canCloneBackpackItem = Boolean(
    selectedItem &&
      itemIndex !== null &&
      itemIndex >= 0 &&
      isBackpackInventory &&
      !selectedItem.stackable &&
      (summary?.preferred_game === "dao" ||
        summary?.preferred_game === "dao_awakening" ||
        summary?.preferred_game === "da2"),
  );
  const canEditMaterial = Boolean(selectedItem?.material_profile && selectedItem.material_options.length > 0);
  const canEditItemLevel = summary?.preferred_game === "da2";

  useEffect(() => {
    if (summary) {
      const summaryKey = `${summary.source_path}:${summary.preferred_game}`;
      if (loadedSummaryKey.current === summaryKey) {
        return;
      }
      loadedSummaryKey.current = summaryKey;
      const nextMoneyDraft = summary.money.toString();
      setMoneyDraft(nextMoneyDraft);
      draftCheckpoint.checkpoint({
        moneyDraft: nextMoneyDraft,
        itemDrafts: draftCheckpoint.current?.itemDrafts ?? {},
        removedBackpackIndexes: [],
        clonedBackpackItems: [],
      });
    }
  }, [draftCheckpoint, summary]);

  useEffect(() => {
    storeCurrentItemDraft();
    currentItemDraftKey.current = selectedItemDraftKey;
    if (!selectedItem || !selectedItemDraftKey) {
      return;
    }
    const nextDraft = itemDrafts.current[selectedItemDraftKey] ?? draftFromItem(selectedItem);
    itemDrafts.current[selectedItemDraftKey] = {
      itemMetadataDraft: { ...nextDraft.itemMetadataDraft },
      itemPropertiesDraft: cloneItemPropertiesDraft(nextDraft.itemPropertiesDraft),
      propertyDraft: { ...nextDraft.propertyDraft },
    };
    if (!draftCheckpoint.current?.itemDrafts[selectedItemDraftKey]) {
      draftCheckpoint.checkpoint({
        moneyDraft: draftCheckpoint.current?.moneyDraft ?? moneyDraftRef.current,
        itemDrafts: {
          ...(draftCheckpoint.current?.itemDrafts ?? {}),
          [selectedItemDraftKey]: cloneInventoryItemDraft(nextDraft),
        },
        removedBackpackIndexes: draftCheckpoint.current?.removedBackpackIndexes ?? [],
        clonedBackpackItems: draftCheckpoint.current?.clonedBackpackItems ?? [],
      });
    }
    applyItemDraft(nextDraft);
  }, [applyItemDraft, draftCheckpoint, draftFromItem, selectedItem, selectedItemDraftKey, storeCurrentItemDraft]);

  const refreshAvailableItemProperties = useCallback(async () => {
    const response = expectResult(
      await executeCommand({ command: "list_available_item_properties" }),
      "available_item_properties",
    );
    setAvailableItemProperties(response.properties);
    setPropertyDraft((current) => ({
      ...current,
      property_id: current.property_id || response.properties[0]?.id.toString() || "",
    }));
  }, []);

  const loadItemsForContainer = useCallback(async (targetContainer: InventoryContainer) => {
    const targetContainerKey = keyForContainer(targetContainer);
    const response = expectResult(
      await (targetContainer === "backpack"
        ? executeCommand({ command: "list_backpack_items" })
        : executeCommand({
            command: "list_equipment_items",
            target: targetContainer.equipment.target ?? MAIN_TARGET,
          })),
      "items",
    );
    loadedItemsByContainer.current[targetContainerKey] = {
      container: targetContainer,
      items: response.items,
    };
    return { containerKey: targetContainerKey, items: response.items };
  }, []);

  const refreshItems = useCallback(async () => {
    const response = await loadItemsForContainer(container);
    setItems(response.items);
    setLoadedContainerKey(containerKey);
    setItemIndex((current) =>
      current !== null && response.items.some((entry) => entry.index === current)
        ? current
        : response.items[0]?.index ?? null,
    );
  }, [container, containerKey, loadItemsForContainer]);

  const refreshLoadedItems = useCallback(async () => {
    const loadedContainers = Object.values(loadedItemsByContainer.current);
    for (const loaded of loadedContainers) {
      const response = await loadItemsForContainer(loaded.container);
      if (response.containerKey === containerKey) {
        setItems(response.items);
        setLoadedContainerKey(containerKey);
      }
    }
  }, [containerKey, loadItemsForContainer]);

  const handlePropertyAddDraft = useCallback(() => {
    const propertyId = parseNumber(propertyDraft.property_id);
    const power = parseNumber(propertyDraft.power);
    if (propertyId === null || power === null) {
      setError("Property ID and power must be valid numbers.");
      return;
    }
    const selectedProperty = availableItemProperties.find((property) => property.id === propertyId);
    setItemPropertiesDraft((current) => [
      ...current,
      { id: propertyId, name: selectedProperty?.name ?? null, power: propertyDraft.power.trim(), sourceIndex: null },
    ]);
    setPropertyDraft((current) => ({ ...current, power: "" }));
  }, [availableItemProperties, propertyDraft, setError]);

  const handlePropertyRemoveDraft = useCallback((propertyIndex: number) => {
    setItemPropertiesDraft((current) => current.filter((_, index) => index !== propertyIndex));
  }, []);

  const handlePropertyUpdateDraft = useCallback(
    (kind: "id" | "power", propertyIndex: number, raw: string) => {
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
    },
    [availableItemProperties],
  );

  const planCommands = useCallback((): InventoryCommandPlan => {
    storeCurrentItemDraft();
    const batch: SaveCommand[] = [];
    const money = parseNumber(moneyDraftRef.current);
    if (money === null) {
      throw new Error("Money must be a valid number.");
    }
    if (summary && summary.money !== money) {
      batch.push({ command: "set_money", money });
    }

    const removed = new Set(removedBackpackIndexes);
    const entriesByContainer = new Map<string, DraftItemEntry[]>();
    for (const [key, draft] of Object.entries(itemDrafts.current)) {
      const parsed = parseDraftItemKey(key);
      if (!parsed || parsed.index < 0 || (parsed.containerKey === "backpack" && removed.has(parsed.index))) {
        continue;
      }
      const loaded = loadedItemsByContainer.current[parsed.containerKey];
      if (!loaded) {
        throw new Error(`Inventory draft exists for unloaded container ${parsed.containerKey}.`);
      }
      const item = loaded.items.find((entry) => entry.index === parsed.index)?.item;
      if (!item) {
        continue;
      }
      const entries = entriesByContainer.get(parsed.containerKey) ?? [];
      entries.push({ index: parsed.index, item, draft });
      entriesByContainer.set(parsed.containerKey, entries);
    }

    for (const [key, entries] of entriesByContainer) {
      const loaded = loadedItemsByContainer.current[key];
      if (loaded) {
        batch.push(...planInventoryDraftCommands({ container: loaded.container, entries }));
      }
    }

    return {
      clones: clonedBackpackItems.map(({ tempIndex, sourceIndex, item }) => {
        const draft = itemDrafts.current[`backpack:${tempIndex}`];
        return {
          tempIndex,
          sourceIndex,
          batch: draft
            ? planInventoryDraftCommands({ container: "backpack", entries: [{ index: tempIndex, item, draft }] })
            : [],
        };
      }),
      removes: [...removedBackpackIndexes].sort((a, b) => b - a),
      batch,
    };
  }, [
    clonedBackpackItems,
    removedBackpackIndexes,
    storeCurrentItemDraft,
    summary,
  ]);

  const checkpointDrafts = useCallback(() => {
    storeCurrentItemDraft();
    const nextItemDrafts = Object.fromEntries(
      Object.entries(itemDrafts.current).map(([key, draft]) => [
      key,
        cloneInventoryItemDraft(draft),
      ]),
    );
    draftCheckpoint.checkpoint({
      moneyDraft,
      itemDrafts: nextItemDrafts,
      removedBackpackIndexes: [],
      clonedBackpackItems: [],
    });
  }, [draftCheckpoint, moneyDraft, storeCurrentItemDraft]);

  const markDraftsCommitted = useCallback((clearBackpackStructureDrafts = false) => {
    setRemovedBackpackIndexes([]);
    setClonedBackpackItems([]);
    nextTemporaryBackpackIndex.current = -1;
    if (clearBackpackStructureDrafts) {
      itemDrafts.current = withoutDraftsForContainer(itemDrafts.current, "backpack");
      currentItemDraftKey.current = null;
    }
    checkpointDrafts();
  }, [checkpointDrafts]);

  const resetToCommittedDrafts = useCallback(() => {
    const checkpoint = draftCheckpoint.reset();
    if (!checkpoint) {
      return;
    }
    setMoneyDraft(checkpoint.moneyDraft);
    itemDrafts.current = checkpoint.itemDrafts;
    setRemovedBackpackIndexes(checkpoint.removedBackpackIndexes);
    setClonedBackpackItems(checkpoint.clonedBackpackItems);
    if (currentItemDraftKey.current && itemDrafts.current[currentItemDraftKey.current]) {
      applyItemDraft(itemDrafts.current[currentItemDraftKey.current]);
    }
  }, [applyItemDraft, draftCheckpoint]);

  const handleBackpackRemove = useCallback(async () => {
    if (container !== "backpack" || itemIndex === null) {
      return;
    }
    storeCurrentItemDraft();
    delete itemDrafts.current[`${containerKey}:${itemIndex}`];
    if (itemIndex < 0) {
      setClonedBackpackItems((current) => current.filter((entry) => entry.tempIndex !== itemIndex));
    } else {
      setRemovedBackpackIndexes((current) => current.includes(itemIndex) ? current : [...current, itemIndex]);
    }
    setItemIndex(visibleItems.find((entry) => entry.index !== itemIndex)?.index ?? null);
  }, [container, containerKey, itemIndex, storeCurrentItemDraft, visibleItems]);

  const handleBackpackClone = useCallback(async () => {
    if (!canCloneBackpackItem || itemIndex === null || !selectedItem) {
      return;
    }
    storeCurrentItemDraft();
    const tempIndex = nextTemporaryBackpackIndex.current;
    nextTemporaryBackpackIndex.current -= 1;
    const currentDraft = currentItemDraftKey.current
      ? itemDrafts.current[currentItemDraftKey.current]
      : draftFromItem(selectedItem);
    itemDrafts.current[`${containerKey}:${tempIndex}`] = cloneInventoryItemDraft(currentDraft);
    setClonedBackpackItems((current) => [
      ...current,
      { tempIndex, sourceIndex: itemIndex, item: cloneItem(selectedItem) },
    ]);
  }, [canCloneBackpackItem, containerKey, draftFromItem, itemIndex, selectedItem, storeCurrentItemDraft]);

  const handleWikiOpen = useCallback(
    async (url: string) => {
      await run(async () => {
        await openUrl(url);
      });
    },
    [run],
  );

  const clear = useCallback(() => {
    setItems([]);
    setLoadedContainerKey(null);
    setItemIndex(null);
    setRemovedBackpackIndexes([]);
    setClonedBackpackItems([]);
    setMoneyDraft("");
    setAvailableItemProperties([]);
    setItemMetadataDraft({ material: "", item_level: "", stack_size: "" });
    setItemPropertiesDraft([]);
    setPropertyDraft({ property_id: "", power: "" });
    itemDrafts.current = {};
    currentItemDraftKey.current = null;
    loadedItemsByContainer.current = {};
    loadedSummaryKey.current = null;
    nextTemporaryBackpackIndex.current = -1;
    draftCheckpoint.clear();
  }, [draftCheckpoint]);

  return {
    items: visibleItems,
    itemIndex,
    setItemIndex,
    selectedItem,
    moneyDraft,
    setMoneyDraft,
    itemMetadataDraft,
    setItemMetadataDraft,
    itemPropertiesDraft,
    propertyDraft,
    setPropertyDraft,
    availableItemProperties,
    canEditStackSize,
    canCloneBackpackItem,
    canEditMaterial,
    canEditItemLevel,
    refreshAvailableItemProperties,
    refreshItems,
    refreshLoadedItems,
    handlePropertyAddDraft,
    handlePropertyRemoveDraft,
    handlePropertyUpdateDraft,
    handleBackpackRemove,
    handleBackpackClone,
    handleWikiOpen,
    planCommands,
    markDraftsCommitted,
    resetToCommittedDrafts,
    clear,
  };
}
