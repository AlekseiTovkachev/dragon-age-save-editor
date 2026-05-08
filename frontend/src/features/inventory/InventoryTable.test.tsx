import { fireEvent, render, screen, within } from "@testing-library/react";
import { useState } from "react";
import { describe, expect, it } from "vitest";
import { indexedItem } from "../../test/factories";
import { filterInventoryItems, inventoryCategories } from "./InventoryPanel";
import { InventoryTable } from "./InventoryTable";
import { InventoryToolbar } from "./InventoryToolbar";

const items = [
  indexedItem(0, {
    name: "Sword",
    resref: "gen_swd",
    category: { value: "weapon", label: "Weapon" },
    material_info: { code: 1, tier: 1, name: "Iron", family: "metal", target: "weapon" },
    properties: [{ id: 7, name: "Damage", power: 1 }],
  }),
  indexedItem(1, {
    name: "Amulet",
    resref: "gen_amu",
    category: { value: "jewelry", label: "Jewelry" },
    material: null,
    properties: [{ id: 42, name: "Spirit ward", power: 2 }],
  }),
];

function firstCellText() {
  return screen
    .getAllByRole("row")
    .slice(1)
    .filter((row) => within(row).queryAllByRole("cell").length > 1)
    .map((row) => within(row).getAllByRole("cell")[0].textContent ?? "");
}

function InventoryTableHarness() {
  const [selectedIndex, setSelectedIndex] = useState<number | null>(null);

  return (
    <InventoryTable
      items={items}
      selectedIndex={selectedIndex}
      onSelect={setSelectedIndex}
      renderInlineEditor={() => <div>Inline editor</div>}
    />
  );
}

function InventoryHarness() {
  const [selectedIndex, setSelectedIndex] = useState<number | null>(null);
  const [search, setSearch] = useState("");
  const [category, setCategory] = useState("__all__");
  const filteredItems = filterInventoryItems(items, category, search);

  return (
    <section>
      <InventoryToolbar
        itemCount={filteredItems.length}
        totalItemCount={items.length}
        categories={inventoryCategories(items)}
        category={category}
        search={search}
        moneyDraft="100"
        canEdit
        busy={false}
        onCategoryChange={setCategory}
        onSearchChange={setSearch}
        onMoneyChange={() => undefined}
      />
      <InventoryTable
        items={filteredItems}
        selectedIndex={selectedIndex}
        onSelect={setSelectedIndex}
        renderInlineEditor={() => <div className="inline-item-editor">Inline editor</div>}
      />
    </section>
  );
}

describe("InventoryTable", () => {
  it("filters by search", () => {
    render(<InventoryHarness />);

    fireEvent.change(screen.getByRole("searchbox", { name: "Search inventory" }), { target: { value: "spirit" } });

    expect(screen.getByText("Amulet")).toBeInTheDocument();
    expect(screen.queryByText("Sword")).not.toBeInTheDocument();
    expect(screen.getByText("1 of 2 items")).toBeInTheDocument();
  });

  it("filters by category chip", () => {
    render(<InventoryHarness />);

    fireEvent.click(screen.getByRole("button", { name: "Jewelry" }));

    expect(screen.getByText("Amulet")).toBeInTheDocument();
    expect(screen.queryByText("Sword")).not.toBeInTheDocument();
    expect(screen.getByText("1 of 2 items")).toBeInTheDocument();
  });

  it("expands on click and collapses on second click", () => {
    render(<InventoryTableHarness />);

    expect(screen.getByText("Sword")).toBeInTheDocument();
    expect(screen.getByText("Amulet")).toBeInTheDocument();
    expect(screen.queryByText("Inline editor")).not.toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /Sword/ }));
    expect(screen.getByText("Inline editor")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /Sword/ }));
    expect(screen.queryByText("Inline editor")).not.toBeInTheDocument();
  });

  it("toggles sort direction", () => {
    render(<InventoryTableHarness />);

    fireEvent.click(screen.getByRole("button", { name: /Sort by Item ascending/ }));
    expect(firstCellText()[0]).toContain("Amulet");

    fireEvent.click(screen.getByRole("button", { name: /Sort by Item descending/ }));
    expect(firstCellText()[0]).toContain("Sword");
  });
});
