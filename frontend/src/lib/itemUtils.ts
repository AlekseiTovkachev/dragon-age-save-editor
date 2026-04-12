import type { CharacterTarget, Item, ItemProperty } from "../types";

export type ItemPropertyDraft = {
  id: number;
  name: string | null;
  power: string;
};

export const MAIN_TARGET: CharacterTarget = "main_character";

export function targetKey(target: CharacterTarget): string {
  return target === "main_character" ? "main" : `companion:${target.companion.index}`;
}

export function itemLabel(item: Item, index: number): string {
  if (item.name) {
    return item.name;
  }
  if (item.resref) {
    return `<${item.resref}>`;
  }
  return `Item ${index}`;
}

export function toItemPropertyDrafts(properties: ItemProperty[]): ItemPropertyDraft[] {
  return properties.map((property) => ({
    id: property.id,
    name: property.name,
    power: property.power.toString(),
  }));
}
