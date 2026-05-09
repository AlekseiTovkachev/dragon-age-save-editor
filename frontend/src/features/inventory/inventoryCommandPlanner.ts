import { parseNumber } from "../../lib/format";
import type { InventoryContainer, Item, SaveCommand } from "../../types";
import type { InventoryItemDraft } from "./useInventoryEditor";

type DraftItemEntry = {
  index: number;
  item: Item;
  draft: InventoryItemDraft;
};

type InventoryCommandPlanInput = {
  container: InventoryContainer;
  entries: DraftItemEntry[];
};

export type InventoryDraftCommand = Extract<
  SaveCommand,
  | { command: "set_backpack_item_stack_size" }
  | { command: "patch_item_metadata" }
  | { command: "set_item_property_id" }
  | { command: "set_item_property_power" }
  | { command: "remove_item_property" }
  | { command: "add_item_property" }
>;

function parseRequiredNumber(raw: string, message: string) {
  const value = parseNumber(raw);
  if (value === null) {
    throw new Error(message);
  }
  return value;
}

function parseStackSize(raw: string) {
  const value = parseRequiredNumber(raw, "Stack size must be a whole number from 1 to 99.");
  if (!Number.isInteger(value) || value < 1 || value > 99) {
    throw new Error("Stack size must be a whole number from 1 to 99.");
  }
  return value;
}

export function planInventoryDraftCommands({
  container,
  entries,
}: InventoryCommandPlanInput): InventoryDraftCommand[] {
  const commands: InventoryDraftCommand[] = [];

  for (const { index: itemIndex, item, draft } of entries) {
    const itemCanEditStackSize = container === "backpack" && item.stackable;
    const itemCanEditMaterial = Boolean(item.material_profile && item.material_options.length > 0);

    if (itemCanEditStackSize) {
      const stackSize = parseStackSize(draft.itemMetadataDraft.stack_size);
      if (item.item_stacksize !== stackSize) {
        commands.push({
          command: "set_backpack_item_stack_size",
          index: itemIndex,
          stack_size: stackSize,
        });
      }
    }

    const metadataPatch: Extract<InventoryDraftCommand, { command: "patch_item_metadata" }>["patch"] = {};
    if (itemCanEditMaterial) {
      const material = parseNumber(draft.itemMetadataDraft.material);
      if (item.material !== material) {
        metadataPatch.material = material;
      }
    }
    const itemLevel = parseNumber(draft.itemMetadataDraft.item_level);
    if (item.item_level !== itemLevel) {
      metadataPatch.item_level = itemLevel;
    }
    if (Object.keys(metadataPatch).length > 0) {
      commands.push({
        command: "patch_item_metadata",
        container,
        index: itemIndex,
        patch: metadataPatch,
      });
    }

    const sourceProperties = item.properties;
    const draftProperties = draft.itemPropertiesDraft;

    for (const draftProperty of draftProperties.filter((property) => property.sourceIndex !== null)) {
      const propertyIndex = draftProperty.sourceIndex!;
      const sourceProperty = sourceProperties[propertyIndex];
      if (!sourceProperty) {
        continue;
      }
      const parsedPower = parseRequiredNumber(
        draftProperty.power,
        `Property ${propertyIndex + 1} power must be a valid number.`,
      );
      if (sourceProperty.id !== draftProperty.id) {
        commands.push({
          command: "set_item_property_id",
          container,
          index: itemIndex,
          property_index: propertyIndex,
          property_id: draftProperty.id,
        });
      }
      if (sourceProperty.power !== parsedPower) {
        commands.push({
          command: "set_item_property_power",
          container,
          index: itemIndex,
          property_index: propertyIndex,
          power: parsedPower,
        });
      }
    }

    const retainedSourceIndexes = new Set(
      draftProperties
        .map((property) => property.sourceIndex)
        .filter((sourceIndex): sourceIndex is number => sourceIndex !== null),
    );
    for (let propertyIndex = sourceProperties.length - 1; propertyIndex >= 0; propertyIndex -= 1) {
      if (!retainedSourceIndexes.has(propertyIndex)) {
        commands.push({
          command: "remove_item_property",
          container,
          index: itemIndex,
          property_index: propertyIndex,
        });
      }
    }

    for (const draftProperty of draftProperties.filter((property) => property.sourceIndex === null)) {
      const parsedPower = parseRequiredNumber(draftProperty.power, "New property power must be a valid number.");
      commands.push({
        command: "add_item_property",
        container,
        index: itemIndex,
        property_id: draftProperty.id,
        power: parsedPower,
      });
    }
  }

  return commands;
}
