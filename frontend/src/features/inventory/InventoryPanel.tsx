import { useMemo, useState, type Dispatch, type SetStateAction } from "react";
import type { ItemPropertyDraft } from "../../lib/itemUtils";
import type { IndexedItem, Item, SelectableItemProperty } from "../../types";
import { InlineItemEditor } from "./InlineItemEditor";
import { InventoryTable } from "./InventoryTable";
import { InventoryToolbar } from "./InventoryToolbar";
import type { ItemMetadataDraft, PropertyDraft } from "./useInventoryEditor";

type InventoryPanelProps = {
  state: InventoryPanelState;
  actions: InventoryPanelActions;
  canEdit: boolean;
  busy: boolean;
  hideMaterial?: boolean;
};

export type InventoryPanelState = {
  moneyDraft: string;
  items: IndexedItem[];
  itemIndex: number | null;
  selectedItem: Item | null;
  canEditStackSize: boolean;
  canCloneBackpackItem: boolean;
  canEditMaterial: boolean;
  itemMetadataDraft: ItemMetadataDraft;
  propertyDraft: PropertyDraft;
  itemPropertiesDraft: ItemPropertyDraft[];
  availableItemProperties: SelectableItemProperty[];
};

export type InventoryPanelActions = {
  setMoneyDraft: (value: string) => void;
  setItemIndex: (index: number) => void;
  setItemMetadataDraft: Dispatch<SetStateAction<ItemMetadataDraft>>;
  setPropertyDraft: Dispatch<SetStateAction<PropertyDraft>>;
  handlePropertyAddDraft: () => void;
  handlePropertyRemoveDraft: (propertyIndex: number) => void;
  handlePropertyUpdateDraft: (kind: "id" | "power", propertyIndex: number, raw: string) => void;
  handleBackpackRemove: () => Promise<void>;
  handleBackpackClone: () => Promise<void>;
  handleWikiOpen: (url: string) => Promise<void>;
};

export type InventoryCategoryFilter = {
  value: string;
  label: string;
};

const ALL_CATEGORY: InventoryCategoryFilter = { value: "__all__", label: "All" };

function normalizedText(value: string | number | null | undefined) {
  return value === null || value === undefined ? "" : String(value).toLowerCase();
}

function itemMatchesSearch({ item }: IndexedItem, query: string) {
  if (!query) {
    return true;
  }

  const searchableValues = [
    item.name,
    item.resref,
    item.category.label,
    item.category.value,
    item.material_info?.name,
    item.material_info?.code,
    item.material,
    ...item.properties.flatMap((property) => [property.name, property.id]),
  ];

  return searchableValues.some((value) => normalizedText(value).includes(query));
}

export function inventoryCategories(items: IndexedItem[]): InventoryCategoryFilter[] {
  const categories = new Map<string, string>();
  for (const { item } of items) {
    if (!categories.has(item.category.value)) {
      categories.set(item.category.value, item.category.label);
    }
  }

  return [
    ALL_CATEGORY,
    ...Array.from(categories, ([value, label]) => ({ value, label })).sort((a, b) =>
      a.label.localeCompare(b.label, undefined, { sensitivity: "base" }),
    ),
  ];
}

export function filterInventoryItems(items: IndexedItem[], category: string, search: string): IndexedItem[] {
  const query = search.trim().toLowerCase();
  return items.filter((entry) => {
    const matchesCategory = category === ALL_CATEGORY.value || entry.item.category.value === category;
    return matchesCategory && itemMatchesSearch(entry, query);
  });
}

export function InventoryPanel({ state, actions, canEdit, busy, hideMaterial }: InventoryPanelProps) {
  const [search, setSearch] = useState("");
  const [category, setCategory] = useState(ALL_CATEGORY.value);
  const categories = useMemo(() => inventoryCategories(state.items), [state.items]);
  const filteredItems = useMemo(() => filterInventoryItems(state.items, category, search), [category, search, state.items]);

  return (
    <section className="inventory-panel">
      <InventoryToolbar
        itemCount={filteredItems.length}
        totalItemCount={state.items.length}
        categories={categories}
        category={category}
        search={search}
        moneyDraft={state.moneyDraft}
        canEdit={canEdit}
        busy={busy}
        onCategoryChange={setCategory}
        onSearchChange={setSearch}
        onMoneyChange={actions.setMoneyDraft}
      />
      <InventoryTable
        items={filteredItems}
        selectedIndex={state.itemIndex}
        onSelect={actions.setItemIndex}
        hideMaterial={hideMaterial}
        renderInlineEditor={() => (
          <InlineItemEditor state={state} actions={actions} canEdit={canEdit} busy={busy} />
        )}
      />
    </section>
  );
}
