import { describe, expect, it } from "vitest";
import { itemLabel, targetKey, toItemPropertyDrafts } from "./itemUtils";
import type { Item } from "../types";

const baseItem: Item = {
  resref: null,
  name: null,
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
  properties: [],
};

describe("itemUtils", () => {
  it("builds stable labels and target keys", () => {
    expect(itemLabel({ ...baseItem, name: "Sword" }, 3)).toBe("Sword");
    expect(itemLabel({ ...baseItem, resref: "gen_swd" }, 3)).toBe("<gen_swd>");
    expect(itemLabel(baseItem, 3)).toBe("Item 3");
    expect(targetKey("main_character")).toBe("main");
    expect(targetKey({ companion: { index: 2 } })).toBe("companion:2");
  });

  it("copies item properties into editable string-power drafts", () => {
    expect(toItemPropertyDrafts([{ id: 7, name: "Damage", power: 1.5 }])).toEqual([
      { id: 7, name: "Damage", power: "1.5", sourceIndex: 0 },
    ]);
  });
});
