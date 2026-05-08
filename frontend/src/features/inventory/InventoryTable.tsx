import { Fragment, useMemo, useState, type ReactNode } from "react";
import { itemLabel } from "../../lib/itemUtils";
import type { IndexedItem } from "../../types";
import { InventoryRow } from "./InventoryRow";

type SortKey = "index" | "name" | "category" | "stack" | "material" | "level" | "properties";
type SortDirection = "asc" | "desc";

type InventoryTableProps = {
  items: IndexedItem[];
  selectedIndex: number | null;
  onSelect: (index: number) => void;
  renderInlineEditor: () => ReactNode;
};

const collator = new Intl.Collator(undefined, { numeric: true, sensitivity: "base" });

function sortValue(entry: IndexedItem, key: SortKey): string | number {
  const { item } = entry;
  switch (key) {
    case "name":
      return itemLabel(item, entry.index);
    case "category":
      return item.category.label;
    case "stack":
      return item.item_stacksize ?? 0;
    case "material":
      return item.material_info?.name ?? item.material ?? "";
    case "level":
      return item.item_level ?? 0;
    case "properties":
      return item.properties.length;
    case "index":
      return entry.index;
  }
}

function compareEntries(a: IndexedItem, b: IndexedItem, key: SortKey, direction: SortDirection) {
  const aValue = sortValue(a, key);
  const bValue = sortValue(b, key);
  const comparison =
    typeof aValue === "number" && typeof bValue === "number"
      ? aValue - bValue
      : collator.compare(String(aValue), String(bValue));
  return direction === "asc" ? comparison : -comparison;
}

type SortHeaderProps = {
  label: string;
  column: SortKey;
  sortKey: SortKey;
  sortDirection: SortDirection;
  onSort: (column: SortKey) => void;
};

function SortHeader({ label, column, sortKey, sortDirection, onSort }: SortHeaderProps) {
  const active = sortKey === column;
  const nextDirection = active && sortDirection === "asc" ? "descending" : "ascending";

  return (
    <button
      className={active ? "inventory-sort is-active" : "inventory-sort"}
      type="button"
      aria-label={`Sort by ${label} ${nextDirection}`}
      aria-sort={active ? (sortDirection === "asc" ? "ascending" : "descending") : "none"}
      onClick={() => onSort(column)}
    >
      <span>{label}</span>
      <span aria-hidden="true">{active ? sortDirection : "--"}</span>
    </button>
  );
}

export function InventoryTable({ items, selectedIndex, onSelect, renderInlineEditor }: InventoryTableProps) {
  const [sortKey, setSortKey] = useState<SortKey>("index");
  const [sortDirection, setSortDirection] = useState<SortDirection>("asc");
  const [expandedIndex, setExpandedIndex] = useState<number | null>(null);
  const sortedItems = useMemo(
    () => [...items].sort((a, b) => compareEntries(a, b, sortKey, sortDirection)),
    [items, sortDirection, sortKey],
  );

  const handleToggle = (index: number) => {
    setExpandedIndex((current) => {
      if (current === index) {
        return null;
      }
      onSelect(index);
      return index;
    });
  };

  const handleSort = (column: SortKey) => {
    if (column === sortKey) {
      setSortDirection((current) => (current === "asc" ? "desc" : "asc"));
      return;
    }
    setSortKey(column);
    setSortDirection("asc");
  };

  return (
    <div className="inventory-table-shell">
      <table className="inventory-table">
        <thead>
          <tr>
            <th scope="col">
              <SortHeader
                label="Item"
                column="name"
                sortKey={sortKey}
                sortDirection={sortDirection}
                onSort={handleSort}
              />
            </th>
            <th scope="col">
              <SortHeader
                label="Category"
                column="category"
                sortKey={sortKey}
                sortDirection={sortDirection}
                onSort={handleSort}
              />
            </th>
            <th scope="col">
              <SortHeader
                label="Qty"
                column="stack"
                sortKey={sortKey}
                sortDirection={sortDirection}
                onSort={handleSort}
              />
            </th>
            <th scope="col">
              <SortHeader
                label="Material"
                column="material"
                sortKey={sortKey}
                sortDirection={sortDirection}
                onSort={handleSort}
              />
            </th>
            <th scope="col">
              <SortHeader
                label="Level"
                column="level"
                sortKey={sortKey}
                sortDirection={sortDirection}
                onSort={handleSort}
              />
            </th>
            <th scope="col">
              <SortHeader
                label="Props"
                column="properties"
                sortKey={sortKey}
                sortDirection={sortDirection}
                onSort={handleSort}
              />
            </th>
          </tr>
        </thead>
        <tbody>
          {sortedItems.map((entry) => {
            const expanded = entry.index === expandedIndex;
            return (
              <Fragment key={entry.index}>
                <InventoryRow
                  key={`item-${entry.index}`}
                  entry={entry}
                  expanded={expanded}
                  selected={entry.index === selectedIndex}
                  onToggle={handleToggle}
                />
                {expanded ? (
                  <tr key={`editor-${entry.index}`} className="inventory-inline-row inv-expand">
                    <td className="inv-expand" colSpan={6}>
                      {renderInlineEditor()}
                    </td>
                  </tr>
                ) : null}
              </Fragment>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
