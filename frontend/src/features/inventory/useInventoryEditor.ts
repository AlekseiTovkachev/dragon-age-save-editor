/* eslint-disable react-hooks/set-state-in-effect -- Inventory form drafts intentionally mirror the selected item and summary snapshots. */
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { openUrl } from "@tauri-apps/plugin-opener";
import { executeCommand, expectResult } from "../../api";
import { useDraftCheckpoint } from "../../hooks/useDraftCheckpoint";
import { MAIN_TARGET, toItemPropertyDrafts } from "../../lib/itemUtils";
import { parseNumber } from "../../lib/format";
import type { ItemPropertyDraft } from "../../lib/itemUtils";
import type { IndexedItem, InventoryContainer, Item, SaveSummary, SelectableItemProperty } from "../../types";
import type { AsyncRun } from "../shared/types";
import { planInventoryDraftCommands } from "./inventoryCommandPlanner";

type UseInventoryEditorOptions = {
  summary: SaveSummary | null;
  container: InventoryContainer;
  isBackpackInventory: boolean;
  run: AsyncRun;
  setError: (message: string | null) => void;
  refreshSummary: () => Promise<unknown>;
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
};

export type InventoryItemDraft = {
  itemMetadataDraft: ItemMetadataDraft;
  itemPropertiesDraft: ItemPropertyDraft[];
  propertyDraft: PropertyDraft;
};

type DraftItemEntry = {
  index: number;
  item: Item;
  draft: InventoryItemDraft;
};

const cloneItemPropertiesDraft = (properties: ItemPropertyDraft[]) => properties.map((property) => ({ ...property }));

const cloneInventoryItemDraft = (draft: InventoryItemDraft): InventoryItemDraft => ({
  itemMetadataDraft: { ...draft.itemMetadataDraft },
  itemPropertiesDraft: cloneItemPropertiesDraft(draft.itemPropertiesDraft),
  propertyDraft: { ...draft.propertyDraft },
});

const cloneInventoryCheckpoint = (checkpoint: InventoryDraftCheckpoint): InventoryDraftCheckpoint => ({
  moneyDraft: checkpoint.moneyDraft,
  itemDrafts: Object.fromEntries(
    Object.entries(checkpoint.itemDrafts).map(([key, draft]) => [key, cloneInventoryItemDraft(draft)]),
  ),
});

export function useInventoryEditor({
  summary,
  container,
  isBackpackInventory,
  run,
  setError,
  refreshSummary,
}: UseInventoryEditorOptions) {
  const [items, setItems] = useState<IndexedItem[]>([]);
  const [itemIndex, setItemIndex] = useState<number | null>(null);
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

  const selectedItem = useMemo(
    () => items.find((entry) => entry.index === itemIndex)?.item ?? null,
    [itemIndex, items],
  );

  const containerKey = useMemo(() => {
    if (container === "backpack") {
      return "backpack";
    }
    const target = container.equipment.target;
    return target === "main_character" ? "equipment:main" : `equipment:companion:${target.companion.index}`;
  }, [container]);

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
      isBackpackInventory &&
      !selectedItem.stackable &&
      (summary?.preferred_game === "dao" ||
        summary?.preferred_game === "dao_awakening" ||
        summary?.preferred_game === "da2"),
  );
  const canEditMaterial = Boolean(selectedItem?.material_profile && selectedItem.material_options.length > 0);

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

  const refreshItems = useCallback(async () => {
    const response = expectResult(
      await (container === "backpack"
        ? executeCommand({ command: "list_backpack_items" })
        : executeCommand({
            command: "list_equipment_items",
            target: container.equipment.target ?? MAIN_TARGET,
          })),
      "items",
    );
    setItems(response.items);
    setItemIndex((current) =>
      current !== null && response.items.some((entry) => entry.index === current)
        ? current
        : response.items[0]?.index ?? null,
    );
  }, [container]);

  const commitMoneyDraft = useCallback(async () => {
    const money = parseNumber(moneyDraftRef.current);
    if (money === null) {
      setError("Money must be a valid number.");
      return false;
    }
    return run(async () => {
      const response = expectResult(await executeCommand({ command: "set_money", money }), "summary");
      await refreshSummary();
      setMoneyDraft(response.summary.money.toString());
    });
  }, [refreshSummary, run, setError]);

  const resetMoneyDraftToLoaded = useCallback(() => {
    if (summary) {
      setMoneyDraft(summary.money.toString());
    }
  }, [summary]);

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

  const commitInventoryItemDrafts = useCallback(async () => {
    storeCurrentItemDraft();
    return run(async () => {
      const draftEntries = Object.entries(itemDrafts.current)
        .filter(([key]) => key.startsWith(`${containerKey}:`))
        .map(([key, draft]) => {
          const index = Number(key.slice(containerKey.length + 1));
          const item = items.find((entry) => entry.index === index)?.item;
          return item ? { index, item, draft } : null;
        })
        .filter((entry): entry is DraftItemEntry => entry !== null);

      const commands = planInventoryDraftCommands({ container, entries: draftEntries });
      if (commands.length > 0) {
        await executeCommand({ command: "apply_batch", commands });
      }
      await refreshItems();
      await refreshSummary();
    });
  }, [
    container,
    containerKey,
    refreshItems,
    refreshSummary,
    run,
    storeCurrentItemDraft,
    items,
  ]);

  const resetInventoryDraftToLoaded = useCallback(() => {
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
  }, [selectedItem]);

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
    });
  }, [draftCheckpoint, moneyDraft, storeCurrentItemDraft]);

  const commitDrafts = useCallback(async () => {
    if (!await commitMoneyDraft()) {
      return false;
    }
    if (!await commitInventoryItemDrafts()) {
      return false;
    }
    checkpointDrafts();
    return true;
  }, [checkpointDrafts, commitInventoryItemDrafts, commitMoneyDraft]);

  const resetToCommittedDrafts = useCallback(() => {
    const checkpoint = draftCheckpoint.reset();
    if (!checkpoint) {
      return;
    }
    setMoneyDraft(checkpoint.moneyDraft);
    itemDrafts.current = checkpoint.itemDrafts;
    if (currentItemDraftKey.current && itemDrafts.current[currentItemDraftKey.current]) {
      applyItemDraft(itemDrafts.current[currentItemDraftKey.current]);
    }
  }, [applyItemDraft, draftCheckpoint]);

  const handleBackpackRemove = useCallback(async () => {
    if (container !== "backpack" || itemIndex === null) {
      return;
    }
    await run(async () => {
      await executeCommand({ command: "remove_backpack_item", index: itemIndex });
      await refreshSummary();
      await refreshItems();
    });
  }, [container, itemIndex, refreshItems, refreshSummary, run]);

  const handleBackpackClone = useCallback(async () => {
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
  }, [canCloneBackpackItem, itemIndex, refreshItems, refreshSummary, run]);

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
    setItemIndex(null);
    setMoneyDraft("");
    setAvailableItemProperties([]);
    setItemMetadataDraft({ material: "", item_level: "", stack_size: "" });
    setItemPropertiesDraft([]);
    setPropertyDraft({ property_id: "", power: "" });
    itemDrafts.current = {};
    currentItemDraftKey.current = null;
    loadedSummaryKey.current = null;
    draftCheckpoint.clear();
  }, [draftCheckpoint]);

  return {
    items,
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
    refreshAvailableItemProperties,
    refreshItems,
    commitMoneyDraft,
    resetMoneyDraftToLoaded,
    handlePropertyAddDraft,
    handlePropertyRemoveDraft,
    handlePropertyUpdateDraft,
    commitInventoryItemDrafts,
    resetInventoryDraftToLoaded,
    handleBackpackRemove,
    handleBackpackClone,
    handleWikiOpen,
    commitDrafts,
    resetToCommittedDrafts,
    clear,
  };
}
