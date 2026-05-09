import type { InventoryCategoryFilter } from "./InventoryPanel";

type InventoryToolbarProps = {
  itemCount: number;
  totalItemCount: number;
  categories: InventoryCategoryFilter[];
  category: string;
  search: string;
  moneyDraft: string;
  canEdit: boolean;
  busy: boolean;
  onCategoryChange: (value: string) => void;
  onSearchChange: (value: string) => void;
  onMoneyChange: (value: string) => void;
};

export function InventoryToolbar({
  itemCount,
  totalItemCount,
  categories,
  category,
  search,
  moneyDraft,
  canEdit,
  busy,
  onCategoryChange,
  onSearchChange,
  onMoneyChange,
}: InventoryToolbarProps) {
  const countLabel =
    itemCount === totalItemCount
      ? itemCount === 1
        ? "1 item"
        : `${itemCount} items`
      : `${itemCount} of ${totalItemCount} items`;

  return (
    <div className="inventory-toolbar">
      <div className="inventory-toolbar-summary">
        <div>
          <h2>Inventory</h2>
          <p>{countLabel}</p>
        </div>
        <input
          className="search-input inventory-search"
          type="search"
          value={search}
          placeholder="Search inventory"
          aria-label="Search inventory"
          onChange={(event) => onSearchChange(event.target.value)}
        />
        <div className="inventory-chip-row" aria-label="Inventory categories">
          {categories.map((entry) => (
            <button
              className={["cat-chip", entry.value === category ? "is-active" : ""].filter(Boolean).join(" ")}
              type="button"
              key={entry.value}
              aria-pressed={entry.value === category}
              onClick={() => onCategoryChange(entry.value)}
            >
              {entry.label}
            </button>
          ))}
        </div>
      </div>
      <div className="gold-pill">
        <svg width="13" height="13" viewBox="0 0 13 13" fill="none" aria-hidden="true">
          <circle cx="6.5" cy="6.5" r="5.5" stroke="currentColor" strokeWidth="1.2"/>
          <text x="6.5" y="10" textAnchor="middle" fontSize="7" fill="currentColor" fontFamily="serif">g</text>
        </svg>
        <span>Money</span>
        <input
          className="amt-input"
          type="number"
          min={0}
          value={moneyDraft}
          disabled={!canEdit || busy}
          onChange={(event) => onMoneyChange(event.target.value)}
          onBlur={(event) => {
            const n = Number(event.target.value);
            if (isNaN(n) || n < 0) onMoneyChange("0");
          }}
          aria-label="Party gold"
        />
      </div>
    </div>
  );
}
