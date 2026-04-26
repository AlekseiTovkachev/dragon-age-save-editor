import { itemLabel } from "../lib/itemUtils";
import type { IndexedItem } from "../types";
import { ListRow, ScrollRegion } from "./ui";

type ItemListProps = {
  items: IndexedItem[];
  selectedIndex: number | null;
  onSelect: (index: number) => void;
};

export function ItemList({ items, selectedIndex, onSelect }: ItemListProps) {
  return (
    <ScrollRegion className="item-list">
      {items.map((entry) => {
        const amount = entry.item.item_stacksize && entry.item.item_stacksize > 1 ? entry.item.item_stacksize : null;
        return (
          <ListRow
            key={entry.index}
            active={entry.index === selectedIndex}
            onClick={() => onSelect(entry.index)}
          >
            <strong>{itemLabel(entry.item, entry.index)}</strong>
            {amount ? <span className="item-amount">x{amount}</span> : null}
          </ListRow>
        );
      })}
    </ScrollRegion>
  );
}
