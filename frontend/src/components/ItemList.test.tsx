import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ItemList } from "./ItemList";
import type { IndexedItem, Item } from "../types";

const item: Item = {
  resref: "gen_swd",
  name: "Sword",
  wiki_url: null,
  category: { value: "weapon", label: "Weapon" },
  stackable: false,
  object_id: null,
  equipment_slot: null,
  item_cost: null,
  item_stacksize: null,
  item_level: null,
  material: null,
  material_profile: null,
  material_info: null,
  material_options: [],
  properties: [{ id: 1, name: "Damage", power: 1 }],
};

describe("ItemList", () => {
  it("renders item labels and emits selection", () => {
    const onSelect = vi.fn();
    const items: IndexedItem[] = [{ index: 4, item: { ...item, item_stacksize: 3 } }];

    render(<ItemList items={items} selectedIndex={4} onSelect={onSelect} />);

    expect(screen.getByText("Sword")).toBeInTheDocument();
    expect(screen.getByText("x3")).toBeInTheDocument();
    expect(screen.queryByText("gen_swd")).not.toBeInTheDocument();
    expect(screen.queryByText("1 Property")).not.toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /Sword/ }));
    expect(onSelect).toHaveBeenCalledWith(4);
  });

  it("does not show item amount when the stack size is missing, zero, or one", () => {
    const items: IndexedItem[] = [
      { index: 4, item: { ...item, item_stacksize: 0 } },
      { index: 5, item: { ...item, item_stacksize: 1 } },
    ];
    render(<ItemList items={items} selectedIndex={4} onSelect={vi.fn()} />);

    expect(screen.queryByText(/x0/)).not.toBeInTheDocument();
    expect(screen.queryByText(/x1/)).not.toBeInTheDocument();
  });
});
