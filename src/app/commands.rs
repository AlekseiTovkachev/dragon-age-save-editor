use crate::domain::gamedata::GameDataLookup;
use crate::domain::stats::{CoreStatsPatch, PointPoolsPatch};
use crate::edit::{
    AbilityListKind, BackpackItemReplacement, CharacterTarget, InventoryContainer,
    ItemMetadataPatch, PlotBooleanPatch, PlotIntegerPatch,
};
use crate::validate::validate_gff;
use serde::{Deserialize, Serialize};

use super::catalogs::{DA2_PLOT_BOOLEAN_FLAGS, DA2_PLOT_INTEGER_FLAGS, available_crafting_recipes};
use super::document::SaveDocument;
use super::dto::*;
use super::errors::{CommandError, CommandErrorCode};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum SaveCommand {
    Validate,
    GetSummary,
    GetDocumentAssets,
    GetCharacter {
        target: CharacterTargetDto,
    },
    ListAvailableAbilities {
        list: AbilityListKindDto,
    },
    ListAvailableItemProperties,
    ListAvailableCraftingRecipes,
    ListAvailablePlotFlags,
    ListCharacters,
    ListBackpackItems,
    ListEquipmentItems {
        target: CharacterTargetDto,
    },
    ListCraftingRecipes,
    ListPlotFlags,
    SetMoney {
        money: u32,
    },
    PatchCoreStats {
        target: CharacterTargetDto,
        patch: CoreStatsPatchDto,
    },
    PatchPointPools {
        target: CharacterTargetDto,
        patch: PointPoolsPatchDto,
    },
    SetLevel {
        target: CharacterTargetDto,
        level: u32,
    },
    SetExperience {
        target: CharacterTargetDto,
        experience: u32,
    },
    SetApproval {
        target: CharacterTargetDto,
        approval: i32,
    },
    ReplaceAbilityList {
        target: CharacterTargetDto,
        list: AbilityListKindDto,
        ability_ids: Vec<u32>,
    },
    ReplaceCraftingRecipeList {
        recipe_ids: Vec<u32>,
    },
    PatchPlotFlags {
        booleans: Vec<PlotBooleanValueDto>,
        integers: Vec<PlotIntegerValueDto>,
    },
    PatchItemMetadata {
        container: InventoryContainerDto,
        index: usize,
        patch: ItemMetadataPatchDto,
    },
    RemoveBackpackItem {
        index: usize,
    },
    CloneBackpackItem {
        index: usize,
    },
    SetBackpackItemStackSize {
        index: usize,
        stack_size: u32,
    },
    ReplaceBackpackItem {
        index: usize,
        replacement: BackpackItemReplacementDto,
    },
    AddItemProperty {
        container: InventoryContainerDto,
        index: usize,
        property_id: u32,
        power: f32,
    },
    RemoveItemProperty {
        container: InventoryContainerDto,
        index: usize,
        property_index: usize,
    },
    SetItemPropertyPower {
        container: InventoryContainerDto,
        index: usize,
        property_index: usize,
        power: f32,
    },
    SetItemPropertyId {
        container: InventoryContainerDto,
        index: usize,
        property_index: usize,
        property_id: u32,
    },
    ApplyBatch {
        commands: Vec<SaveCommand>,
    },
    SaveAs {
        output_path: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum SaveCommandResult {
    Validation {
        report: ValidationReportDto,
    },
    Summary {
        summary: SaveSummaryDto,
    },
    DocumentAssets {
        assets: DocumentAssetsDto,
    },
    AvailableAbilities {
        list: AbilityListKindDto,
        abilities: Vec<AbilityDto>,
    },
    AvailableItemProperties {
        properties: Vec<SelectableItemPropertyDto>,
    },
    AvailableCraftingRecipes {
        recipes: Vec<CraftingRecipeDto>,
    },
    AvailablePlotFlags {
        booleans: Vec<PlotBooleanFlagDto>,
        integers: Vec<PlotIntegerFlagDto>,
    },
    Characters {
        characters: Vec<CharacterSummaryDto>,
    },
    Items {
        items: Vec<IndexedItemDto>,
    },
    CraftingRecipes {
        recipe_ids: Vec<u32>,
    },
    PlotFlags {
        booleans: Vec<PlotBooleanValueDto>,
        integers: Vec<PlotIntegerValueDto>,
    },
    Character {
        target: CharacterTargetDto,
        character: CharacterDto,
    },
    Item {
        container: InventoryContainerDto,
        index: usize,
        item: ItemDto,
    },
    Saved {
        output_path: String,
        summary: SaveSummaryDto,
    },
}

impl SaveDocument {
    pub fn execute(&mut self, command: SaveCommand) -> Result<SaveCommandResult, CommandError> {
        match command {
            SaveCommand::ApplyBatch { commands } => self.execute_batch(commands),
            SaveCommand::Validate => Ok(SaveCommandResult::Validation {
                report: ValidationReportDto::from(validate_gff(&self.raw)),
            }),
            SaveCommand::GetSummary => Ok(SaveCommandResult::Summary {
                summary: self.summary(),
            }),
            SaveCommand::GetDocumentAssets => Ok(SaveCommandResult::DocumentAssets {
                assets: DocumentAssetsDto {
                    screenshot_data_url: self.screenshot_data_url()?,
                },
            }),
            SaveCommand::GetCharacter { target } => {
                let target = CharacterTarget::from(target);
                Ok(SaveCommandResult::Character {
                    target: CharacterTargetDto::from(target),
                    character: self.character_dto(target)?,
                })
            }
            SaveCommand::ListAvailableAbilities { list } => {
                let lookup = self.lookup.as_ref().ok_or_else(|| CommandError {
                    code: CommandErrorCode::LookupFailed,
                    message: "ability browsing requires data/gamedata.db".to_string(),
                })?;
                let abilities = lookup
                    .abilities_by_kind(
                        expected_ability_kind(AbilityListKind::from(list.clone())),
                        self.preferred_game(),
                    )
                    .map_err(CommandError::from_lookup)?
                    .into_iter()
                    .map(AbilityDto::from)
                    .collect();
                Ok(SaveCommandResult::AvailableAbilities { list, abilities })
            }
            SaveCommand::ListAvailableItemProperties => {
                let lookup = self.lookup.as_ref().ok_or_else(|| CommandError {
                    code: CommandErrorCode::LookupFailed,
                    message: "item property browsing requires data/gamedata.db".to_string(),
                })?;
                let properties = lookup
                    .item_properties(self.preferred_game())
                    .map_err(CommandError::from_lookup)?
                    .into_iter()
                    .map(|(id, name)| SelectableItemPropertyDto { id, name })
                    .collect();
                Ok(SaveCommandResult::AvailableItemProperties { properties })
            }
            SaveCommand::ListAvailableCraftingRecipes => {
                Ok(SaveCommandResult::AvailableCraftingRecipes {
                    recipes: available_crafting_recipes(self.preferred_game())
                        .iter()
                        .map(|recipe| CraftingRecipeDto {
                            id: recipe.id,
                            name: recipe.name.to_string(),
                            category: recipe.category.to_string(),
                        })
                        .collect(),
                })
            }
            SaveCommand::ListAvailablePlotFlags => Ok(SaveCommandResult::AvailablePlotFlags {
                booleans: DA2_PLOT_BOOLEAN_FLAGS
                    .iter()
                    .map(|flag| PlotBooleanFlagDto {
                        id: flag.id,
                        name: flag.name.to_string(),
                        description: flag.description.to_string(),
                        category: flag.category.to_string(),
                    })
                    .collect(),
                integers: DA2_PLOT_INTEGER_FLAGS
                    .iter()
                    .map(|flag| PlotIntegerFlagDto {
                        id: flag.id,
                        name: flag.name.to_string(),
                        description: flag.description.to_string(),
                        category: flag.category.to_string(),
                        options: flag
                            .options
                            .iter()
                            .map(|option| PlotIntegerOptionDto {
                                value: option.value,
                                label: option.label.to_string(),
                            })
                            .collect(),
                    })
                    .collect(),
            }),
            SaveCommand::ListCharacters => Ok(SaveCommandResult::Characters {
                characters: self
                    .editor()?
                    .list_characters()
                    .into_iter()
                    .map(CharacterSummaryDto::from)
                    .collect(),
            }),
            SaveCommand::ListBackpackItems => Ok(SaveCommandResult::Items {
                items: self
                    .editor()?
                    .backpack_items()
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(index, item)| {
                        Ok(IndexedItemDto {
                            index,
                            item: self.item_to_dto(item)?,
                        })
                    })
                    .collect::<Result<Vec<_>, CommandError>>()?,
            }),
            SaveCommand::ListEquipmentItems { target } => {
                let target = CharacterTarget::from(target);
                Ok(SaveCommandResult::Items {
                    items: self
                        .editor()?
                        .equipment_items(target)
                        .map_err(CommandError::from)?
                        .iter()
                        .cloned()
                        .enumerate()
                        .map(|(index, item)| {
                            Ok(IndexedItemDto {
                                index,
                                item: self.item_to_dto(item)?,
                            })
                        })
                        .collect::<Result<Vec<_>, CommandError>>()?,
                })
            }
            SaveCommand::ListCraftingRecipes => Ok(SaveCommandResult::CraftingRecipes {
                recipe_ids: self.editor()?.crafting_recipes().to_vec(),
            }),
            SaveCommand::ListPlotFlags => {
                let plot_flags = &self.editor()?.save().plot_flags;
                Ok(SaveCommandResult::PlotFlags {
                    booleans: plot_flags
                        .booleans
                        .iter()
                        .map(|(id, value)| PlotBooleanValueDto {
                            id: *id,
                            value: *value,
                        })
                        .collect(),
                    integers: plot_flags
                        .integers
                        .iter()
                        .map(|(id, value)| PlotIntegerValueDto {
                            id: *id,
                            value: *value,
                        })
                        .collect(),
                })
            }
            SaveCommand::SetMoney { money } => {
                self.editor_mut()?
                    .set_money(money)
                    .map_err(CommandError::from)?;
                self.dirty = true;
                Ok(SaveCommandResult::Summary {
                    summary: self.summary(),
                })
            }
            SaveCommand::PatchCoreStats { target, patch } => {
                let target = CharacterTarget::from(target);
                self.editor_mut()?
                    .patch_character_core_stats(target, CoreStatsPatch::from(patch))
                    .map_err(CommandError::from)?;
                self.dirty = true;
                Ok(SaveCommandResult::Character {
                    target: CharacterTargetDto::from(target),
                    character: self.character_dto(target)?,
                })
            }
            SaveCommand::PatchPointPools { target, patch } => {
                let target = CharacterTarget::from(target);
                self.editor_mut()?
                    .patch_character_point_pools(target, PointPoolsPatch::from(patch))
                    .map_err(CommandError::from)?;
                self.dirty = true;
                Ok(SaveCommandResult::Character {
                    target: CharacterTargetDto::from(target),
                    character: self.character_dto(target)?,
                })
            }
            SaveCommand::SetLevel { target, level } => {
                let target = CharacterTarget::from(target);
                self.editor_mut()?
                    .set_character_level(target, level)
                    .map_err(CommandError::from)?;
                self.dirty = true;
                Ok(SaveCommandResult::Character {
                    target: CharacterTargetDto::from(target),
                    character: self.character_dto(target)?,
                })
            }
            SaveCommand::SetExperience { target, experience } => {
                let target = CharacterTarget::from(target);
                self.editor_mut()?
                    .set_character_experience(target, experience)
                    .map_err(CommandError::from)?;
                self.dirty = true;
                Ok(SaveCommandResult::Character {
                    target: CharacterTargetDto::from(target),
                    character: self.character_dto(target)?,
                })
            }
            SaveCommand::SetApproval { target, approval } => {
                let target = CharacterTarget::from(target);
                self.editor_mut()?
                    .set_character_approval(target, approval)
                    .map_err(CommandError::from)?;
                self.dirty = true;
                Ok(SaveCommandResult::Character {
                    target: CharacterTargetDto::from(target),
                    character: self.character_dto(target)?,
                })
            }
            SaveCommand::ReplaceAbilityList {
                target,
                list,
                ability_ids,
            } => {
                let target = CharacterTarget::from(target);
                let missing_lookup = || CommandError {
                    code: CommandErrorCode::LookupFailed,
                    message: "ability editing requires data/gamedata.db".to_string(),
                };
                let lookup = self.lookup.take();
                let Some(lookup_ref) = lookup
                    .as_ref()
                    .map(|db| db as &dyn crate::domain::gamedata::GameDataLookup)
                else {
                    self.lookup = lookup;
                    return Err(missing_lookup());
                };
                let result = self
                    .editor_mut()?
                    .replace_character_abilities(
                        target,
                        AbilityListKind::from(list),
                        &ability_ids,
                        lookup_ref,
                    )
                    .map_err(CommandError::from);
                self.lookup = lookup;
                result?;
                self.dirty = true;
                Ok(SaveCommandResult::Character {
                    target: CharacterTargetDto::from(target),
                    character: self.character_dto(target)?,
                })
            }
            SaveCommand::ReplaceCraftingRecipeList { recipe_ids } => {
                self.editor_mut()?
                    .replace_crafting_recipes(&dedupe_preserving_order(recipe_ids))
                    .map_err(CommandError::from)?;
                self.dirty = true;
                Ok(SaveCommandResult::CraftingRecipes {
                    recipe_ids: self.editor()?.crafting_recipes().to_vec(),
                })
            }
            SaveCommand::PatchPlotFlags { booleans, integers } => {
                self.editor_mut()?
                    .patch_plot_flags(
                        &booleans
                            .into_iter()
                            .map(PlotBooleanPatch::from)
                            .collect::<Vec<_>>(),
                        &integers
                            .into_iter()
                            .map(PlotIntegerPatch::from)
                            .collect::<Vec<_>>(),
                    )
                    .map_err(CommandError::from)?;
                self.dirty = true;
                let plot_flags = &self.editor()?.save().plot_flags;
                Ok(SaveCommandResult::PlotFlags {
                    booleans: plot_flags
                        .booleans
                        .iter()
                        .map(|(id, value)| PlotBooleanValueDto {
                            id: *id,
                            value: *value,
                        })
                        .collect(),
                    integers: plot_flags
                        .integers
                        .iter()
                        .map(|(id, value)| PlotIntegerValueDto {
                            id: *id,
                            value: *value,
                        })
                        .collect(),
                })
            }
            SaveCommand::PatchItemMetadata {
                container,
                index,
                patch,
            } => {
                let container_enum = InventoryContainer::from(container.clone());
                self.editor_mut()?
                    .patch_item_metadata(container_enum, index, ItemMetadataPatch::from(patch))
                    .map_err(CommandError::from)?;
                let lookup = self.lookup.take();
                let preferred_game = self.preferred_game();
                self.editor_mut()?
                    .refresh_item_material_info(
                        container_enum,
                        index,
                        lookup
                            .as_ref()
                            .map(|db| db as &dyn crate::domain::gamedata::GameDataLookup),
                        preferred_game,
                    )
                    .map_err(CommandError::from)?;
                self.lookup = lookup;
                self.dirty = true;
                Ok(SaveCommandResult::Item {
                    container,
                    index,
                    item: self.item_dto(container_enum, index)?,
                })
            }
            SaveCommand::RemoveBackpackItem { index } => {
                self.editor_mut()?
                    .remove_backpack_item(index)
                    .map_err(CommandError::from)?;
                self.dirty = true;
                Ok(SaveCommandResult::Summary {
                    summary: self.summary(),
                })
            }
            SaveCommand::CloneBackpackItem { index } => {
                let new_index = self
                    .editor_mut()?
                    .clone_backpack_item(index)
                    .map_err(CommandError::from)?;
                self.dirty = true;
                Ok(SaveCommandResult::Item {
                    container: InventoryContainerDto::Backpack,
                    index: new_index,
                    item: self.item_dto(InventoryContainer::Backpack, new_index)?,
                })
            }
            SaveCommand::SetBackpackItemStackSize { index, stack_size } => {
                self.editor_mut()?
                    .set_backpack_item_stack_size(index, stack_size)
                    .map_err(CommandError::from)?;
                self.dirty = true;
                Ok(SaveCommandResult::Item {
                    container: InventoryContainerDto::Backpack,
                    index,
                    item: self.item_dto(InventoryContainer::Backpack, index)?,
                })
            }
            SaveCommand::ReplaceBackpackItem { index, replacement } => {
                self.editor_mut()?
                    .replace_backpack_item(index, BackpackItemReplacement::from(replacement))
                    .map_err(CommandError::from)?;
                self.dirty = true;
                Ok(SaveCommandResult::Item {
                    container: InventoryContainerDto::Backpack,
                    index,
                    item: self.item_dto(InventoryContainer::Backpack, index)?,
                })
            }
            SaveCommand::AddItemProperty {
                container,
                index,
                property_id,
                power,
            } => {
                let container_enum = InventoryContainer::from(container.clone());
                let lookup = self.lookup.take();
                let result = self
                    .editor_mut()?
                    .add_item_property(
                        container_enum,
                        index,
                        property_id,
                        power,
                        lookup
                            .as_ref()
                            .map(|db| db as &dyn crate::domain::gamedata::GameDataLookup),
                    )
                    .map_err(CommandError::from);
                self.lookup = lookup;
                result?;
                self.dirty = true;
                Ok(SaveCommandResult::Item {
                    container,
                    index,
                    item: self.item_dto(container_enum, index)?,
                })
            }
            SaveCommand::RemoveItemProperty {
                container,
                index,
                property_index,
            } => {
                let container_enum = InventoryContainer::from(container.clone());
                self.editor_mut()?
                    .remove_item_property(container_enum, index, property_index)
                    .map_err(CommandError::from)?;
                self.dirty = true;
                Ok(SaveCommandResult::Item {
                    container,
                    index,
                    item: self.item_dto(container_enum, index)?,
                })
            }
            SaveCommand::SetItemPropertyPower {
                container,
                index,
                property_index,
                power,
            } => {
                let container_enum = InventoryContainer::from(container.clone());
                self.editor_mut()?
                    .set_item_property_power(container_enum, index, property_index, power)
                    .map_err(CommandError::from)?;
                self.dirty = true;
                Ok(SaveCommandResult::Item {
                    container,
                    index,
                    item: self.item_dto(container_enum, index)?,
                })
            }
            SaveCommand::SetItemPropertyId {
                container,
                index,
                property_index,
                property_id,
            } => {
                let container_enum = InventoryContainer::from(container.clone());
                let lookup = self.lookup.take();
                let result = self
                    .editor_mut()?
                    .set_item_property_id(
                        container_enum,
                        index,
                        property_index,
                        property_id,
                        lookup
                            .as_ref()
                            .map(|db| db as &dyn crate::domain::gamedata::GameDataLookup),
                    )
                    .map_err(CommandError::from);
                self.lookup = lookup;
                result?;
                self.dirty = true;
                Ok(SaveCommandResult::Item {
                    container,
                    index,
                    item: self.item_dto(container_enum, index)?,
                })
            }
            SaveCommand::SaveAs { output_path } => {
                self.editor()?
                    .write_to_path(&output_path)
                    .map_err(CommandError::from)?;
                self.dirty = false;
                Ok(SaveCommandResult::Saved {
                    output_path,
                    summary: self.summary(),
                })
            }
        }
    }

    fn execute_batch(
        &mut self,
        commands: Vec<SaveCommand>,
    ) -> Result<SaveCommandResult, CommandError> {
        if let Some(command) = commands
            .iter()
            .find(|command| !command.is_batch_edit_command())
        {
            return Err(CommandError {
                code: CommandErrorCode::InvalidSaveState,
                message: format!("command {command:?} cannot be used in apply_batch"),
            });
        }

        let editor_backup = self.editor.clone();
        let dirty_backup = self.dirty;
        for command in commands {
            if let Err(err) = self.execute(command) {
                self.editor = editor_backup;
                self.dirty = dirty_backup;
                return Err(err);
            }
        }
        Ok(SaveCommandResult::Summary {
            summary: self.summary(),
        })
    }
}

impl SaveCommand {
    fn is_batch_edit_command(&self) -> bool {
        matches!(
            self,
            SaveCommand::SetMoney { .. }
                | SaveCommand::PatchCoreStats { .. }
                | SaveCommand::PatchPointPools { .. }
                | SaveCommand::SetLevel { .. }
                | SaveCommand::SetExperience { .. }
                | SaveCommand::SetApproval { .. }
                | SaveCommand::ReplaceAbilityList { .. }
                | SaveCommand::ReplaceCraftingRecipeList { .. }
                | SaveCommand::PatchPlotFlags { .. }
                | SaveCommand::PatchItemMetadata { .. }
                | SaveCommand::SetBackpackItemStackSize { .. }
                | SaveCommand::AddItemProperty { .. }
                | SaveCommand::RemoveItemProperty { .. }
                | SaveCommand::SetItemPropertyPower { .. }
                | SaveCommand::SetItemPropertyId { .. }
        )
    }
}

fn expected_ability_kind(list: AbilityListKind) -> crate::domain::ability::AbilityKind {
    match list {
        AbilityListKind::Skills => crate::domain::ability::AbilityKind::Skill,
        AbilityListKind::Talents => crate::domain::ability::AbilityKind::Talent,
        AbilityListKind::Spells => crate::domain::ability::AbilityKind::Spell,
    }
}

fn dedupe_preserving_order(values: Vec<u32>) -> Vec<u32> {
    let mut seen = std::collections::BTreeSet::new();
    values
        .into_iter()
        .filter(|value| seen.insert(*value))
        .collect()
}
