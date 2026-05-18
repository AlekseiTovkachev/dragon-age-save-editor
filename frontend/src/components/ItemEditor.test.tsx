import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ItemEditor } from "./ItemEditor";
import type { Item } from "../types";

const item: Item = {
  resref: "gen_swd",
  name: "Sword",
  wiki_url: "https://example.test/item",
  category: { value: "weapon", label: "Weapon" },
  stackable: true,
  object_id: null,
  equipment_slot: null,
  item_cost: null,
  item_stacksize: 3,
  item_level: 1,
  material: 1,
  material_profile: { family: "metal", target: "weapon" },
  material_info: null,
  material_options: [{ code: 1, tier: 1, name: "Iron", family: "metal", target: "weapon" }],
  properties: [{ id: 7, name: "Damage", power: 1 }],
};

function renderEditor(overrides = {}) {
  const props = {
    item,
    itemIndex: 0,
    canEdit: true,
    busy: false,
    allowRemove: true,
    canEditStackSize: true,
    canCloneBackpackItem: false,
    canEditMaterial: true,
    canEditItemLevel: true,
    metadataDraft: { material: "1", item_level: "1", stack_size: "3" },
    propertyDraft: { property_id: "7", power: "" },
    itemPropertiesDraft: [{ id: 7, name: "Damage", power: "1", sourceIndex: 0 }],
    availableItemProperties: [{ id: 7, name: "Damage" }],
    onMetadataChange: vi.fn(),
    onPropertyDraftChange: vi.fn(),
    onPropertyAdd: vi.fn(),
    onPropertyRemove: vi.fn(),
    onPropertyUpdate: vi.fn(),
    onRemove: vi.fn(),
    onClone: vi.fn(),
    onWikiOpen: vi.fn(),
    ...overrides,
  };
  render(<ItemEditor {...props} />);
  return props;
}

describe("ItemEditor", () => {
  it("edits metadata draft", () => {
    const props = renderEditor();

    const stackInput = screen.getByDisplayValue("3");
    fireEvent.change(stackInput, { target: { value: "5" } });
    expect(props.onMetadataChange).toHaveBeenCalledWith({ stack_size: "5" });
  });

  it("edits property draft after opening add form", () => {
    const props = renderEditor();

    fireEvent.click(screen.getByRole("button", { name: "+ add property" }));

    fireEvent.change(screen.getByPlaceholderText("Power"), { target: { value: "2" } });
    expect(props.onPropertyDraftChange).toHaveBeenCalledWith({ power: "2" });
  });

  it("rejects invalid numeric metadata input", () => {
    const props = renderEditor();

    const stackInput = screen.getByDisplayValue("3");
    fireEvent.change(stackInput, { target: { value: "abc" } });
    fireEvent.change(stackInput, { target: { value: "-1" } });
    fireEvent.change(stackInput, { target: { value: "100" } });

    expect(props.onMetadataChange).not.toHaveBeenCalled();
  });

  it("rejects invalid property power input after opening add form", () => {
    const props = renderEditor();

    fireEvent.click(screen.getByRole("button", { name: "+ add property" }));

    const powerInput = screen.getByPlaceholderText("Power");
    fireEvent.change(powerInput, { target: { value: "x" } });
    fireEvent.change(powerInput, { target: { value: "-2" } });

    expect(props.onPropertyDraftChange).not.toHaveBeenCalled();
  });

  it("does not render an apply button", () => {
    renderEditor({ itemIndex: null });

    expect(screen.queryByRole("button", { name: "Apply" })).not.toBeInTheDocument();
  });

  it("renders item name and chip-style properties", () => {
    renderEditor();

    expect(screen.getByRole("link", { name: "open wiki page →" })).toBeInTheDocument();
    expect(screen.getByText("Properties (1)")).toBeInTheDocument();
    expect(screen.queryByDisplayValue("Sword")).not.toBeInTheDocument();
    expect(screen.queryByDisplayValue("Yes")).not.toBeInTheDocument();
  });

  it("renders property chips for existing properties", () => {
    renderEditor();

    expect(screen.getByText("Damage")).toBeInTheDocument();
  });

  it("calls onPropertyRemove when chip remove is clicked", () => {
    const props = renderEditor();

    fireEvent.click(screen.getByRole("button", { name: "Remove Damage" }));
    expect(props.onPropertyRemove).toHaveBeenCalledWith(0);
  });

  it("shows add form when + add property is clicked and hides on cancel", () => {
    renderEditor();

    expect(screen.queryByPlaceholderText("Power")).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "+ add property" }));
    expect(screen.getByPlaceholderText("Power")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
    expect(screen.queryByPlaceholderText("Power")).not.toBeInTheDocument();
  });

  it("calls onPropertyAdd and closes form when Add is clicked", () => {
    const props = renderEditor();

    fireEvent.click(screen.getByRole("button", { name: "+ add property" }));
    fireEvent.click(screen.getByRole("button", { name: "Add" }));

    expect(props.onPropertyAdd).toHaveBeenCalledOnce();
    expect(screen.queryByPlaceholderText("Power")).not.toBeInTheDocument();
  });
});
