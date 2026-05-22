use super::{
    AbilityListKind, BackpackItemReplacement, CharacterSummary, CharacterTarget, EditError,
    InventoryContainer, ItemMetadataPatch, PlotBooleanPatch, PlotIntegerPatch,
};
use crate::domain::ability::AbilityRef;
use crate::domain::character::Character;
use crate::domain::game::GameBehavior;
use crate::domain::gamedata::{GameDataLookup, GameId};
use crate::domain::item::{Item, ItemProperty, MaterialProfile};
use crate::domain::save::{
    SaveGame, WORLD_VAULT_BOOLEANS_LABEL, WORLD_VAULT_INTS_LABEL, WORLD_VAULT_LABEL,
};
use crate::domain::stats::{
    CoreStat, CoreStatsPatch, PointPoolKind, PointPoolsPatch, core_stat_id, experience_stat_id,
    level_stat_id, point_pool_stat_id,
};
use crate::edit::internal::*;
use crate::edit::targets::{
    DomainSaveTargets, domain_character_mut, domain_item_mut, nth_struct_index, raw_character,
    raw_character_mut, raw_item, raw_item_mut,
};
#[cfg(test)]
use crate::gff4::fields::{ITEM_STACKSIZE, SAVEGAME_WORLDDATABASE};
use crate::gff4::fields::{
    OBJECT_ID, SAVEGAME_BACKPACK, SAVEGAME_CRAFTING_RECIPE_LIST, SAVEGAME_MONEY,
};
use crate::gff4::{GffFile, Value};
use std::path::Path;

const SAVEGAME_CREATURE_STATS_NAME: &str = "SAVEGAME_CREATURE_STATS";
const SAVEGAME_PARTY_APPROVAL_LIST_NAME: &str = "SAVEGAME_PARTY_APPROVAL_LIST";
const SAVEGAME_PARTY_APPROVAL_ID_NAME: &str = "SAVEGAME_PARTY_APPROVAL_ID";
const SAVEGAME_PARTY_APPROVAL_LEVEL_NAME: &str = "SAVEGAME_PARTY_APPROVAL_LEVEL";
const SAVEGAME_ABILITYLIST_NAME: &str = "SAVEGAME_ABILITYLIST";
const MAX_ITEM_STACK_SIZE: u32 = 99;

#[derive(Debug, Clone)]
pub struct SaveEditor {
    raw: GffFile,
    save: SaveGame,
}

impl SaveEditor {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, EditError> {
        Self::from_path_with_lookup(path, None, None)
    }

    pub fn from_path_with_lookup(
        path: impl AsRef<Path>,
        lookup: Option<&dyn GameDataLookup>,
        preferred_game: Option<GameId>,
    ) -> Result<Self, EditError> {
        let raw = GffFile::from_path(path)?;
        Self::from_gff_with_lookup(raw, lookup, preferred_game)
    }

    pub fn from_gff(raw: GffFile) -> Result<Self, EditError> {
        Self::from_gff_with_lookup(raw, None, None)
    }

    pub fn from_gff_with_lookup(
        raw: GffFile,
        lookup: Option<&dyn GameDataLookup>,
        preferred_game: Option<GameId>,
    ) -> Result<Self, EditError> {
        let save = SaveGame::from_gff_with_lookup(&raw, lookup, preferred_game)?;
        Ok(Self { raw, save })
    }

    pub fn save(&self) -> &SaveGame {
        &self.save
    }

    pub fn raw(&self) -> &GffFile {
        &self.raw
    }

    pub fn into_raw(self) -> GffFile {
        self.raw
    }

    pub fn write_to_path(&self, path: impl AsRef<Path>) -> Result<(), EditError> {
        self.raw.write_to_path(path)?;
        Ok(())
    }

    pub fn list_characters(&self) -> Vec<CharacterSummary> {
        let mut characters = Vec::with_capacity(1 + self.save.companions.len());
        characters.push(CharacterSummary {
            target: CharacterTarget::MainCharacter,
            name: self.save.main_character.name.clone(),
        });
        characters.extend(
            self.save
                .companions
                .iter()
                .enumerate()
                .map(|(index, character)| CharacterSummary {
                    target: CharacterTarget::Companion(index),
                    name: character.name.clone(),
                }),
        );
        characters
    }

    pub fn backpack_items(&self) -> &[Item] {
        &self.save.backpack
    }

    pub fn crafting_recipes(&self) -> &[u32] {
        &self.save.crafting_recipes
    }

    pub fn equipment_items(&self, target: CharacterTarget) -> Result<&[Item], EditError> {
        Ok(&self.character(target)?.equipment)
    }

    pub fn set_money(&mut self, money: u32) -> Result<(), EditError> {
        let party = raw_party_mut(&mut self.raw)?;
        let value = party
            .get_mut(SAVEGAME_MONEY)
            .ok_or_else(|| EditError::MissingField {
                path: "root.SAVEGAME_PARTYLIST.SAVEGAME_MONEY".to_string(),
            })?;
        set_numeric_value(value, money, "root.SAVEGAME_PARTYLIST.SAVEGAME_MONEY")?;

        self.save.money = money;
        Ok(())
    }

    pub fn patch_character_core_stats(
        &mut self,
        target: CharacterTarget,
        patch: CoreStatsPatch,
    ) -> Result<(), EditError> {
        if let Some(value) = patch.strength {
            self.set_character_stat(target, CoreStat::Strength, value)?;
        }
        if let Some(value) = patch.dexterity {
            self.set_character_stat(target, CoreStat::Dexterity, value)?;
        }
        if let Some(value) = patch.willpower {
            self.set_character_stat(target, CoreStat::Willpower, value)?;
        }
        if let Some(value) = patch.magic {
            self.set_character_stat(target, CoreStat::Magic, value)?;
        }
        if let Some(value) = patch.cunning {
            self.set_character_stat(target, CoreStat::Cunning, value)?;
        }
        if let Some(value) = patch.constitution {
            self.set_character_stat(target, CoreStat::Constitution, value)?;
        }

        Ok(())
    }

    pub fn set_character_stat(
        &mut self,
        target: CharacterTarget,
        stat: CoreStat,
        value: u32,
    ) -> Result<(), EditError> {
        let stat_id = core_stat_id(stat);
        let raw_character = raw_character_mut(&mut self.raw, target)?;
        set_character_stat_row_value(raw_character, stat_id, value, target)?;
        self.character_mut(target)?.core_stats.set(stat, value);
        Ok(())
    }

    pub fn set_character_level(
        &mut self,
        target: CharacterTarget,
        level: u32,
    ) -> Result<(), EditError> {
        let raw_character = raw_character_mut(&mut self.raw, target)?;
        set_character_stat_row_value(
            raw_character,
            level_stat_id(self.save.preferred_game),
            level,
            target,
        )?;
        self.character_mut(target)?.level = Some(level);
        Ok(())
    }

    pub fn set_character_experience(
        &mut self,
        target: CharacterTarget,
        experience: u32,
    ) -> Result<(), EditError> {
        let raw_character = raw_character_mut(&mut self.raw, target)?;
        set_or_insert_character_stat_row_value(
            raw_character,
            experience_stat_id(self.save.preferred_game),
            experience,
            target,
        )?;
        self.character_mut(target)?.experience = Some(experience);
        Ok(())
    }

    pub fn patch_character_point_pools(
        &mut self,
        target: CharacterTarget,
        patch: PointPoolsPatch,
    ) -> Result<(), EditError> {
        if let Some(value) = patch.attribute_points {
            self.set_character_point_pool(target, PointPoolKind::Attribute, value)?;
        }
        if let Some(value) = patch.skill_points {
            self.set_character_point_pool(target, PointPoolKind::Skill, value)?;
        }
        if let Some(value) = patch.talent_points {
            self.set_character_point_pool(target, PointPoolKind::Talent, value)?;
        }
        if let Some(value) = patch.specialization_points {
            self.set_character_point_pool(target, PointPoolKind::Specialization, value)?;
        }
        Ok(())
    }

    pub fn set_character_approval(
        &mut self,
        target: CharacterTarget,
        approval: i32,
    ) -> Result<(), EditError> {
        let CharacterTarget::Companion(_) = target else {
            return Err(EditError::InvalidTarget { target });
        };
        let object_id = raw_character(&self.raw, target)?
            .get(OBJECT_ID)
            .and_then(value_to_i32)
            .ok_or_else(|| EditError::MissingField {
                path: "character.OBJECT_ID".to_string(),
            })?;
        let party = raw_party_mut(&mut self.raw)?;
        let approvals = party
            .get_list_mut_by_name(SAVEGAME_PARTY_APPROVAL_LIST_NAME)
            .ok_or_else(|| EditError::MissingField {
                path: "root.SAVEGAME_PARTYLIST.SAVEGAME_PARTY_APPROVAL_LIST".to_string(),
            })?;
        let approval_row = approvals
            .iter_mut()
            .filter_map(Value::as_struct_mut)
            .find(|row| {
                row.get_by_name(SAVEGAME_PARTY_APPROVAL_ID_NAME)
                    .and_then(value_to_i32)
                    == Some(object_id)
            })
            .ok_or_else(|| EditError::MissingField {
                path: format!(
                    "root.SAVEGAME_PARTYLIST.SAVEGAME_PARTY_APPROVAL_LIST[OBJECT_ID={object_id}]"
                ),
            })?;
        let value = approval_row
            .get_mut_by_name(SAVEGAME_PARTY_APPROVAL_LEVEL_NAME)
            .ok_or_else(|| EditError::MissingField {
                path: "root.SAVEGAME_PARTYLIST.SAVEGAME_PARTY_APPROVAL_LIST[].SAVEGAME_PARTY_APPROVAL_LEVEL".to_string(),
            })?;
        set_signed_numeric_value(
            value,
            approval,
            "root.SAVEGAME_PARTYLIST.SAVEGAME_PARTY_APPROVAL_LIST[].SAVEGAME_PARTY_APPROVAL_LEVEL",
        )?;
        self.character_mut(target)?.approval = Some(approval);
        Ok(())
    }

    pub fn replace_character_abilities(
        &mut self,
        target: CharacterTarget,
        list: AbilityListKind,
        ability_ids: &[u32],
        lookup: &dyn GameDataLookup,
    ) -> Result<(), EditError> {
        let character = self.character(target)?;
        let current_abilities = match list {
            AbilityListKind::Skills => &character.skills,
            AbilityListKind::Talents => &character.talents,
            AbilityListKind::Spells => &character.spells,
        };
        let preserved_existing_ids = current_abilities
            .iter()
            .map(|ability| ability.id)
            .collect();
        let replacement = load_validated_abilities(
            target,
            list,
            ability_ids,
            lookup,
            self.save.preferred_game,
            &preserved_existing_ids,
        )?;
        let uses_da2_ability_list =
            uses_combined_ability_list(&self.raw, target, self.save.preferred_game)?;
        if uses_da2_ability_list {
            let merged_ids = merged_da2_ability_ids(self.character(target)?, list, ability_ids);
            let raw_character = raw_character_mut(&mut self.raw, target)?;
            let stats = raw_character
                .get_struct_mut_by_name(SAVEGAME_CREATURE_STATS_NAME)
                .ok_or_else(|| EditError::MissingField {
                    path: "character.SAVEGAME_CREATURE_STATS".to_string(),
                })?;
            let values = stats
                .get_list_mut_by_name(SAVEGAME_ABILITYLIST_NAME)
                .ok_or_else(|| EditError::MissingField {
                    path: "character.SAVEGAME_ABILITYLIST".to_string(),
                })?;
            replace_numeric_list(values, &merged_ids, "character.SAVEGAME_ABILITYLIST")?;
        } else {
            let raw_character = raw_character_mut(&mut self.raw, target)?;
            let stats = raw_character
                .get_struct_mut_by_name(SAVEGAME_CREATURE_STATS_NAME)
                .ok_or_else(|| EditError::MissingField {
                    path: "character.SAVEGAME_CREATURE_STATS".to_string(),
                })?;
            let values = stats
                .get_list_mut(ability_list_label(list))
                .ok_or_else(|| EditError::MissingField {
                    path: ability_list_path(list).to_string(),
                })?;
            replace_numeric_list(values, ability_ids, ability_list_path(list))?;
        }

        *self.character_ability_list_mut(target, list)? = replacement;
        Ok(())
    }

    pub fn replace_crafting_recipes(&mut self, recipe_ids: &[u32]) -> Result<(), EditError> {
        let party = raw_party_mut(&mut self.raw)?;
        let values = party
            .get_list_mut(SAVEGAME_CRAFTING_RECIPE_LIST)
            .ok_or_else(|| EditError::MissingField {
                path: "root.SAVEGAME_PARTYLIST.SAVEGAME_CRAFTING_RECIPE_LIST".to_string(),
            })?;
        replace_numeric_list(
            values,
            recipe_ids,
            "root.SAVEGAME_PARTYLIST.SAVEGAME_CRAFTING_RECIPE_LIST",
        )?;
        self.save.crafting_recipes = recipe_ids.to_vec();
        Ok(())
    }

    pub fn patch_plot_flags(
        &mut self,
        booleans: &[PlotBooleanPatch],
        integers: &[PlotIntegerPatch],
    ) -> Result<(), EditError> {
        if !self.save.preferred_game.supports_plot_flags() {
            return Err(EditError::UnsupportedPlotFlags {
                game: self.save.preferred_game,
            });
        }

        let world_vault = self
            .raw
            .root_mut()
            .get_struct_mut(WORLD_VAULT_LABEL)
            .ok_or_else(|| EditError::MissingField {
                path: "root.WVLT".to_string(),
            })?;

        if !booleans.is_empty() {
            let values = world_vault
                .get_list_mut(WORLD_VAULT_BOOLEANS_LABEL)
                .ok_or_else(|| EditError::MissingField {
                    path: "root.WVLT.WVB1".to_string(),
                })?;
            for patch in booleans {
                set_or_insert_world_vault_bool(values, *patch)?;
                self.save.plot_flags.booleans.insert(patch.id, patch.value);
            }
        }

        if !integers.is_empty() {
            let values = world_vault
                .get_list_mut(WORLD_VAULT_INTS_LABEL)
                .ok_or_else(|| EditError::MissingField {
                    path: "root.WVLT.WVI1".to_string(),
                })?;
            for patch in integers {
                set_or_insert_world_vault_int(values, *patch)?;
                self.save.plot_flags.integers.insert(patch.id, patch.value);
            }
        }

        Ok(())
    }

    pub fn patch_item_metadata(
        &mut self,
        container: InventoryContainer,
        index: usize,
        patch: ItemMetadataPatch,
    ) -> Result<(), EditError> {
        let raw_item = raw_item_mut(&mut self.raw, container, index)?;
        apply_item_metadata_patch_to_struct(raw_item, patch)?;
        apply_item_metadata_patch_to_domain(
            domain_item_mut(&mut self.save, container, index)?,
            patch,
        );
        Ok(())
    }

    pub fn refresh_item_material_info(
        &mut self,
        container: InventoryContainer,
        index: usize,
        lookup: Option<&dyn GameDataLookup>,
        preferred_game: Option<GameId>,
    ) -> Result<(), EditError> {
        let item = domain_item_mut(&mut self.save, container, index)?;
        item.material_info = match (lookup, item.material) {
            (Some(lookup), Some(material_code)) => lookup
                .material_info(material_code, preferred_game)
                .map_err(|err| EditError::LookupFailed {
                    path: "item.SAVEGAME_ITEM_MATERIALTYPE".to_string(),
                    detail: err.to_string(),
                })?,
            _ => None,
        };
        if item.material_profile.is_none() {
            item.material_profile = item.material_info.as_ref().map(|info| MaterialProfile {
                family: info.family,
                target: info.target,
            });
        }
        Ok(())
    }

    pub fn remove_backpack_item(&mut self, index: usize) -> Result<(), EditError> {
        let party = raw_party_mut(&mut self.raw)?;
        let items =
            party
                .get_list_mut(SAVEGAME_BACKPACK)
                .ok_or_else(|| EditError::MissingField {
                    path: "root.SAVEGAME_PARTYLIST.SAVEGAME_BACKPACK".to_string(),
                })?;
        let raw_index = nth_struct_index(items, index).ok_or(EditError::InvalidItemIndex {
            container: InventoryContainer::Backpack,
            index,
        })?;
        items.remove(raw_index);
        self.save.backpack.remove(index);
        Ok(())
    }

    pub fn clone_backpack_item(&mut self, index: usize) -> Result<usize, EditError> {
        if !self
            .save
            .preferred_game
            .is_some_and(|game| game.is_dao_family() || game.is_da2())
        {
            return Err(EditError::UnsupportedGameForClone {
                game: self.save.preferred_game,
            });
        }

        let source_item = self
            .save
            .backpack
            .get(index)
            .ok_or(EditError::InvalidItemIndex {
                container: InventoryContainer::Backpack,
                index,
            })?;
        if source_item.stackable {
            return Err(EditError::ItemIsStackable { index });
        }
        let mut cloned_item = source_item.clone();

        let new_object_id = next_object_id(&self.raw)?;
        let mut cloned_raw = raw_item(&self.raw, InventoryContainer::Backpack, index)?.clone();
        set_object_id(&mut cloned_raw, new_object_id)?;
        update_worlddb_last_id(&mut self.raw, new_object_id)?;

        let party = raw_party_mut(&mut self.raw)?;
        let items =
            party
                .get_list_mut(SAVEGAME_BACKPACK)
                .ok_or_else(|| EditError::MissingField {
                    path: "root.SAVEGAME_PARTYLIST.SAVEGAME_BACKPACK".to_string(),
                })?;
        items.push(Value::Struct(Box::new(cloned_raw)));

        cloned_item.object_id =
            Some(
                i32::try_from(new_object_id).map_err(|_| EditError::NumericRange {
                    path: "item.OBJECT_ID".to_string(),
                    detail: format!("{new_object_id} does not fit into i32"),
                })?,
            );
        self.save.backpack.push(cloned_item);
        Ok(self.save.backpack.len() - 1)
    }

    pub fn set_backpack_item_stack_size(
        &mut self,
        index: usize,
        stack_size: u32,
    ) -> Result<(), EditError> {
        if stack_size == 0 || stack_size > MAX_ITEM_STACK_SIZE {
            return Err(EditError::InvalidStackSize { stack_size });
        }

        let item = self
            .save
            .backpack
            .get(index)
            .ok_or(EditError::InvalidItemIndex {
                container: InventoryContainer::Backpack,
                index,
            })?;
        if !item.stackable {
            return Err(EditError::ItemIsNotStackable { index });
        }

        let raw_item = raw_item_mut(&mut self.raw, InventoryContainer::Backpack, index)?;
        set_or_insert_stack_size(raw_item, stack_size)?;
        self.save.backpack[index].item_stacksize = Some(stack_size);
        Ok(())
    }

    pub fn replace_backpack_item(
        &mut self,
        index: usize,
        replacement: BackpackItemReplacement,
    ) -> Result<(), EditError> {
        let current = self
            .save
            .backpack
            .get(index)
            .ok_or(EditError::InvalidItemIndex {
                container: InventoryContainer::Backpack,
                index,
            })?;
        let expected =
            current
                .resref
                .as_deref()
                .map(clean_resref)
                .ok_or(EditError::MissingItemResref {
                    container: InventoryContainer::Backpack,
                    index,
                })?;
        let actual = clean_resref(&replacement.resref);
        if expected != actual {
            return Err(EditError::BackpackResrefMismatch {
                index,
                expected,
                actual,
            });
        }

        self.patch_item_metadata(
            InventoryContainer::Backpack,
            index,
            ItemMetadataPatch {
                item_cost: replacement.item_cost,
                material: replacement.material,
                item_level: replacement.item_level,
            },
        )
    }

    pub fn add_item_property(
        &mut self,
        container: InventoryContainer,
        index: usize,
        property_id: u32,
        power: f32,
        lookup: Option<&dyn GameDataLookup>,
    ) -> Result<(), EditError> {
        let preferred_game = self.save.preferred_game;
        let raw_item = raw_item_mut(&mut self.raw, container, index)?;
        let property_name = if let Some(lookup) = lookup {
            lookup
                .item_property_name(property_id, preferred_game)
                .map_err(|err| EditError::LookupFailed {
                    path: "item.ITEM_PROPERTIES".to_string(),
                    detail: err.to_string(),
                })?
        } else {
            None
        };
        let mut properties = ItemProperties::from_item_or_create(
            raw_item,
            container,
            index,
            self.save.preferred_game,
        )?;
        properties.push(property_id, power)?;
        domain_item_mut(&mut self.save, container, index)?
            .properties
            .push(ItemProperty {
                id: property_id,
                name: property_name,
                power,
            });
        Ok(())
    }

    pub fn remove_item_property(
        &mut self,
        container: InventoryContainer,
        index: usize,
        property_index: usize,
    ) -> Result<(), EditError> {
        let raw_item = raw_item_mut(&mut self.raw, container, index)?;
        let mut raw_properties =
            ItemProperties::from_item(raw_item, container, index, self.save.preferred_game)?;
        raw_properties.remove(property_index)?;

        let properties = &mut domain_item_mut(&mut self.save, container, index)?.properties;
        if property_index >= properties.len() {
            return Err(EditError::InvalidPropertyIndex {
                container,
                item_index: index,
                property_index,
            });
        }
        properties.remove(property_index);
        Ok(())
    }

    pub fn set_item_property_power(
        &mut self,
        container: InventoryContainer,
        index: usize,
        property_index: usize,
        power: f32,
    ) -> Result<(), EditError> {
        if property_index
            >= domain_item_mut(&mut self.save, container, index)?
                .properties
                .len()
        {
            return Err(EditError::InvalidPropertyIndex {
                container,
                item_index: index,
                property_index,
            });
        }
        let raw_item = raw_item_mut(&mut self.raw, container, index)?;
        let mut raw_properties =
            ItemProperties::from_item(raw_item, container, index, self.save.preferred_game)?;
        raw_properties.set_power(property_index, power)?;
        let properties = &mut domain_item_mut(&mut self.save, container, index)?.properties;
        let property =
            properties
                .get_mut(property_index)
                .ok_or(EditError::InvalidPropertyIndex {
                    container,
                    item_index: index,
                    property_index,
                })?;
        property.power = power;
        Ok(())
    }

    pub fn set_item_property_id(
        &mut self,
        container: InventoryContainer,
        index: usize,
        property_index: usize,
        property_id: u32,
        lookup: Option<&dyn GameDataLookup>,
    ) -> Result<(), EditError> {
        let preferred_game = self.save.preferred_game;
        if property_index
            >= domain_item_mut(&mut self.save, container, index)?
                .properties
                .len()
        {
            return Err(EditError::InvalidPropertyIndex {
                container,
                item_index: index,
                property_index,
            });
        }
        let raw_item = raw_item_mut(&mut self.raw, container, index)?;
        let mut raw_properties =
            ItemProperties::from_item(raw_item, container, index, self.save.preferred_game)?;
        raw_properties.set_id(property_index, property_id)?;
        let property = domain_item_mut(&mut self.save, container, index)?
            .properties
            .get_mut(property_index)
            .ok_or(EditError::InvalidPropertyIndex {
                container,
                item_index: index,
                property_index,
            })?;
        property.id = property_id;
        property.name = if let Some(lookup) = lookup {
            lookup
                .item_property_name(property_id, preferred_game)
                .map_err(|err| EditError::LookupFailed {
                    path: "item.ITEM_PROPERTIES".to_string(),
                    detail: err.to_string(),
                })?
        } else {
            None
        };
        Ok(())
    }

    fn character(&self, target: CharacterTarget) -> Result<&Character, EditError> {
        DomainSaveTargets::new(&self.save).character(target)
    }

    fn character_mut(&mut self, target: CharacterTarget) -> Result<&mut Character, EditError> {
        domain_character_mut(&mut self.save, target)
    }

    fn character_ability_list_mut(
        &mut self,
        target: CharacterTarget,
        list: AbilityListKind,
    ) -> Result<&mut Vec<AbilityRef>, EditError> {
        let character = self.character_mut(target)?;
        Ok(match list {
            AbilityListKind::Skills => &mut character.skills,
            AbilityListKind::Talents => &mut character.talents,
            AbilityListKind::Spells => &mut character.spells,
        })
    }

    fn set_character_point_pool(
        &mut self,
        target: CharacterTarget,
        kind: PointPoolKind,
        value: u32,
    ) -> Result<(), EditError> {
        let Some(stat_id) = point_pool_stat_id(kind, self.save.preferred_game) else {
            return Ok(());
        };
        let raw_character = raw_character_mut(&mut self.raw, target)?;
        set_or_insert_character_stat_row_value(raw_character, stat_id, value, target)?;
        let point_pools = &mut self.character_mut(target)?.point_pools;
        match kind {
            PointPoolKind::Attribute => point_pools.attribute_points = Some(value),
            PointPoolKind::Skill => point_pools.skill_points = Some(value),
            PointPoolKind::Talent => point_pools.talent_points = Some(value),
            PointPoolKind::Specialization => point_pools.specialization_points = Some(value),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
