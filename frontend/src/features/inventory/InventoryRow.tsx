import type { KeyboardEvent, MouseEvent } from "react";
import { itemLabel } from "../../lib/itemUtils";
import type { IndexedItem } from "../../types";

type InventoryRowProps = {
  entry: IndexedItem;
  expanded: boolean;
  selected: boolean;
  onToggle: (index: number) => void;
  hideMaterial?: boolean;
};

function displayValue(value: string | number | null | undefined) {
  if (value === null || value === undefined || value === "") {
    return "-";
  }
  return value;
}

export function InventoryRow({ entry, expanded, selected, onToggle, hideMaterial }: InventoryRowProps) {
  const { item } = entry;
  const amount = item.item_stacksize && item.item_stacksize > 1 ? `x${item.item_stacksize}` : "-";
  const material = item.material_info?.name ?? (item.material ? `Tier ${item.material}` : null);
  const handleToggle = () => onToggle(entry.index);
  const handleRowClick = (event: MouseEvent<HTMLTableRowElement>) => {
    if (event.target instanceof Element && event.target.closest(".inv-expand, .inline-item-editor")) {
      return;
    }
    handleToggle();
  };
  const handleRowKeyDown = (event: KeyboardEvent<HTMLTableRowElement>) => {
    if (event.target !== event.currentTarget || (event.key !== "Enter" && event.key !== " ")) {
      return;
    }
    event.preventDefault();
    handleToggle();
  };

  return (
    <tr
      className={["inventory-row", expanded ? "is-expanded" : "", selected ? "is-selected" : ""]
        .filter(Boolean)
        .join(" ")}
      tabIndex={0}
      onClick={handleRowClick}
      onKeyDown={handleRowKeyDown}
    >
      <td>
        <button
          className="inventory-expand-button"
          type="button"
          aria-expanded={expanded}
        >
          <span>
            <strong>{itemLabel(item, entry.index)}</strong>
            <small>{item.resref ?? `Item ${entry.index}`}</small>
          </span>
        </button>
      </td>
      <td>{item.category.label}</td>
      <td>{amount}</td>
      {!hideMaterial ? <td>{displayValue(material)}</td> : null}
      <td>{displayValue(item.item_level)}</td>
      <td>{item.properties.length}</td>
    </tr>
  );
}
