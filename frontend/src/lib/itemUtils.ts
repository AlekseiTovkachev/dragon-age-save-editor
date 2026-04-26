import type { CharacterTarget, Item, ItemProperty, SelectableItemProperty } from "../types";

export type ItemPropertyDraft = {
  id: number;
  name: string | null;
  power: string;
  sourceIndex: number | null;
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
  return properties.map((property, sourceIndex) => ({
    id: property.id,
    name: property.name,
    power: property.power.toString(),
    sourceIndex,
  }));
}

export function isEditableItemPropertyName(name: string | null): boolean {
  if (!name) {
    return true;
  }
  const normalized = name.toLowerCase();
  if (normalized.includes("[internal]")) {
    return false;
  }
  return ![
    "(base item)",
    "(damage type)",
    "(heraldry)",
    "(item set)",
    "(restriction)",
    "(treasure)",
  ].some((prefix) => normalized.startsWith(prefix));
}

export function editableItemProperties<T extends ItemPropertyDraft | SelectableItemProperty>(properties: T[]): T[] {
  return properties.filter((property) => isEditableItemPropertyName(property.name));
}
