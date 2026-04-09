use crate::domain::ability::{AbilityKind, AbilityRef};
use crate::domain::character::Character;
use crate::domain::gamedata::{GameDataLookup, GameId};
use crate::domain::item::{Item, ItemProperty, MaterialProfile};
use crate::domain::save::{ExtractError, SaveGame};
use crate::domain::stats::{CoreStat, CoreStatsPatch, PointPoolsPatch};
use crate::gff4::fields::{
    ITEM_COST, SAVEGAME_BACKPACK, SAVEGAME_EQUIPMENT_ITEMS, SAVEGAME_ITEM_MATERIALTYPE,
    SAVEGAME_MONEY, SAVEGAME_OBJECT_PLOT, SAVEGAME_PARTYLIST, SAVEGAME_SKILLLIST,
    SAVEGAME_SPELLLIST, SAVEGAME_TALENTLIST,
};
use crate::gff4::{FieldValue, GffFile, GffStruct, Value};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::io;
use std::path::Path;

const SAVEGAME_PLAYERCHAR_NAME: &str = "SAVEGAME_PLAYERCHAR";
const SAVEGAME_PLAYERCHAR_CHAR_NAME: &str = "SAVEGAME_PLAYERCHAR_CHAR";
const SAVEGAME_PARTYPOOLMEMBERS_NAME: &str = "SAVEGAME_PARTYPOOLMEMBERS";
const SAVEGAME_CREATURE_STATS_NAME: &str = "SAVEGAME_CREATURE_STATS";
const SAVEGAME_STATLIST_NAME: &str = "SAVEGAME_STATLIST";
const SAVEGAME_STATPROPERTY_INDEX_NAME: &str = "SAVEGAME_STATPROPERTY_INDEX";
const SAVEGAME_STATPROPERTY_BASE_NAME: &str = "SAVEGAME_STATPROPERTY_BASE";
const ITEM_PROPERTIES_NAME: &str = "ITEM_PROPERTIES";
const ITEM_PROPERTY_POWERS_NAME: &str = "ITEM_PROPERTY_POWERS";
const SAVEGAME_PARTY_APPROVAL_LIST_NAME: &str = "SAVEGAME_PARTY_APPROVAL_LIST";
const SAVEGAME_PARTY_APPROVAL_LEVEL_NAME: &str = "SAVEGAME_PARTY_APPROVAL_LEVEL";
const SAVEGAME_ABILITYLIST_NAME: &str = "SAVEGAME_ABILITYLIST";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterTarget {
    MainCharacter,
    Companion(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterSummary {
    pub target: CharacterTarget,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityListKind {
    Skills,
    Talents,
    Spells,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryContainer {
    Backpack,
    Equipment { target: CharacterTarget },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ItemMetadataPatch {
    pub item_cost: Option<u32>,
    pub material: Option<u32>,
    pub item_level: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackpackItemReplacement {
    pub resref: String,
    pub item_cost: Option<u32>,
    pub material: Option<u32>,
    pub item_level: Option<u8>,
}

#[derive(Debug)]
pub struct SaveEditor {
    raw: GffFile,
    save: SaveGame,
}

#[derive(Debug)]
pub enum EditError {
    Extract(ExtractError),
    Io(io::Error),
    InvalidTarget { target: CharacterTarget },
    MissingField {
        path: String,
    },
    TypeMismatch {
        path: String,
        expected: &'static str,
        actual: &'static str,
    },
    MissingStatRow {
        target: CharacterTarget,
        stat_id: u32,
    },
    UnsupportedNumericValue {
        path: String,
        actual: &'static str,
    },
    NumericRange {
        path: String,
        detail: String,
    },
    LookupFailed {
        path: String,
        detail: String,
    },
    UnknownAbility {
        ability_id: u32,
    },
    InvalidAbilityKind {
        ability_id: u32,
        expected: AbilityListKind,
        actual: AbilityKind,
    },
    MissingCoreAbility {
        target: CharacterTarget,
        list: AbilityListKind,
        required_id: u32,
    },
    InvalidItemIndex {
        container: InventoryContainer,
        index: usize,
    },
    MissingItemResref {
        container: InventoryContainer,
        index: usize,
    },
    BackpackResrefMismatch {
        index: usize,
        expected: String,
        actual: String,
    },
    InvalidPropertyIndex {
        container: InventoryContainer,
        item_index: usize,
        property_index: usize,
    },
    InvalidPropertyArrayParity {
        container: InventoryContainer,
        item_index: usize,
        ids_len: usize,
        powers_len: usize,
    },
}

impl fmt::Display for EditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditError::Extract(err) => write!(f, "{err}"),
            EditError::Io(err) => write!(f, "{err}"),
            EditError::InvalidTarget { target } => write!(f, "invalid character target: {target:?}"),
            EditError::MissingField { path } => write!(f, "missing field at {path}"),
            EditError::TypeMismatch {
                path,
                expected,
                actual,
            } => write!(f, "type mismatch at {path}: expected {expected}, found {actual}"),
            EditError::MissingStatRow { target, stat_id } => {
                write!(f, "missing stat row {stat_id} for target {target:?}")
            }
            EditError::UnsupportedNumericValue { path, actual } => {
                write!(f, "unsupported numeric value at {path}: {actual}")
            }
            EditError::NumericRange { path, detail } => {
                write!(f, "numeric range error at {path}: {detail}")
            }
            EditError::LookupFailed { path, detail } => write!(f, "lookup failed at {path}: {detail}"),
            EditError::UnknownAbility { ability_id } => write!(f, "unknown ability id {ability_id}"),
            EditError::InvalidAbilityKind {
                ability_id,
                expected,
                actual,
            } => write!(
                f,
                "ability {ability_id} has invalid kind for {expected:?}: {actual:?}"
            ),
            EditError::MissingCoreAbility {
                target,
                list,
                required_id,
            } => write!(
                f,
                "editing {list:?} for {target:?} would remove required core ability {required_id}"
            ),
            EditError::InvalidItemIndex { container, index } => {
                write!(f, "invalid item index {index} in {container:?}")
            }
            EditError::MissingItemResref { container, index } => {
                write!(f, "missing item resref at index {index} in {container:?}")
            }
            EditError::BackpackResrefMismatch {
                index,
                expected,
                actual,
            } => write!(
                f,
                "backpack replacement at index {index} must keep resref {expected}, found {actual}"
            ),
            EditError::InvalidPropertyIndex {
                container,
                item_index,
                property_index,
            } => write!(
                f,
                "invalid property index {property_index} for item {item_index} in {container:?}"
            ),
            EditError::InvalidPropertyArrayParity {
                container,
                item_index,
                ids_len,
                powers_len,
            } => write!(
                f,
                "invalid property array parity for item {item_index} in {container:?}: ITEM_PROPERTIES has {ids_len}, ITEM_PROPERTY_POWERS has {powers_len}"
            ),
        }
    }
}

impl Error for EditError {}

impl From<ExtractError> for EditError {
    fn from(value: ExtractError) -> Self {
        Self::Extract(value)
    }
}

impl From<io::Error> for EditError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
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
        set_character_stat_row_value(raw_character, 15, level, target)?;
        self.character_mut(target)?.level = Some(level);
        Ok(())
    }

    pub fn patch_character_point_pools(
        &mut self,
        target: CharacterTarget,
        patch: PointPoolsPatch,
    ) -> Result<(), EditError> {
        if let Some(value) = patch.attribute_points {
            self.set_character_point_pool(target, 34, value)?;
        }
        if let Some(value) = patch.skill_points {
            self.set_character_point_pool(target, 35, value)?;
        }
        if let Some(value) = patch.talent_points {
            self.set_character_point_pool(target, 36, value)?;
        }
        if let Some(value) = patch.specialization_points {
            self.set_character_point_pool(target, 38, value)?;
        }
        Ok(())
    }

    pub fn set_character_approval(
        &mut self,
        target: CharacterTarget,
        approval: i32,
    ) -> Result<(), EditError> {
        let CharacterTarget::Companion(index) = target else {
            return Err(EditError::InvalidTarget { target });
        };
        let party = raw_party_mut(&mut self.raw)?;
        let approvals = party
            .get_list_mut_by_name(SAVEGAME_PARTY_APPROVAL_LIST_NAME)
            .ok_or_else(|| EditError::MissingField {
                path: "root.SAVEGAME_PARTYLIST.SAVEGAME_PARTY_APPROVAL_LIST".to_string(),
            })?;
        let approval_row = nth_struct_mut(approvals, index).ok_or(EditError::InvalidTarget { target })?;
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
        let current_ids = self
            .character(target)?
            .ability_list(list)
            .iter()
            .map(|ability| ability.id)
            .collect::<BTreeSet<_>>();
        let replacement = load_validated_abilities(
            target,
            list,
            ability_ids,
            &current_ids,
            lookup,
            self.save.preferred_game,
        )?;
        let uses_da2_ability_list = uses_da2_combined_ability_list(&self.raw, target, self.save.preferred_game)?;
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

    pub fn patch_item_metadata(
        &mut self,
        container: InventoryContainer,
        index: usize,
        patch: ItemMetadataPatch,
    ) -> Result<(), EditError> {
        let raw_item = raw_item_mut(&mut self.raw, container, index)?;
        apply_item_metadata_patch_to_struct(raw_item, patch)?;
        apply_item_metadata_patch_to_domain(item_mut(&mut self.save, container, index)?, patch);
        Ok(())
    }

    pub fn refresh_item_material_info(
        &mut self,
        container: InventoryContainer,
        index: usize,
        lookup: Option<&dyn GameDataLookup>,
        preferred_game: Option<GameId>,
    ) -> Result<(), EditError> {
        let item = item_mut(&mut self.save, container, index)?;
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
        let items = party
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
        let expected = current
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
        let raw_item = raw_item_mut(&mut self.raw, container, index)?;
        let property_name = if let Some(lookup) = lookup {
            lookup
                .item_property_name(property_id)
                .map_err(|err| EditError::LookupFailed {
                    path: "item.ITEM_PROPERTIES".to_string(),
                    detail: err.to_string(),
                })?
        } else {
            None
        };
        let (property_ids, property_powers) =
            ensure_property_lists_mut(container, index, raw_item)?;
        append_numeric_value(property_ids, property_id, "item.ITEM_PROPERTIES")?;
        append_float_value(property_powers, power, "item.ITEM_PROPERTY_POWERS")?;
        item_mut(&mut self.save, container, index)?.properties.push(ItemProperty {
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
        let (property_ids, property_powers) = property_lists_mut(raw_item, container, index)?;
        if property_index >= property_ids.len() || property_index >= property_powers.len() {
            return Err(EditError::InvalidPropertyIndex {
                container,
                item_index: index,
                property_index,
            });
        }
        property_ids.remove(property_index);
        property_powers.remove(property_index);

        let properties = &mut item_mut(&mut self.save, container, index)?.properties;
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
        if property_index >= item_mut(&mut self.save, container, index)?.properties.len() {
            return Err(EditError::InvalidPropertyIndex {
                container,
                item_index: index,
                property_index,
            });
        }
        let raw_item = raw_item_mut(&mut self.raw, container, index)?;
        let (_, property_powers) = property_lists_mut(raw_item, container, index)?;
        let value = property_powers.get_mut(property_index).ok_or(EditError::InvalidPropertyIndex {
            container,
            item_index: index,
            property_index,
        })?;
        set_float_value(value, power, "item.ITEM_PROPERTY_POWERS")?;
        let properties = &mut item_mut(&mut self.save, container, index)?.properties;
        let property = properties.get_mut(property_index).ok_or(EditError::InvalidPropertyIndex {
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
        if property_index >= item_mut(&mut self.save, container, index)?.properties.len() {
            return Err(EditError::InvalidPropertyIndex {
                container,
                item_index: index,
                property_index,
            });
        }
        let raw_item = raw_item_mut(&mut self.raw, container, index)?;
        let (property_ids, _) = property_lists_mut(raw_item, container, index)?;
        let value = property_ids.get_mut(property_index).ok_or(EditError::InvalidPropertyIndex {
            container,
            item_index: index,
            property_index,
        })?;
        set_numeric_value(value, property_id, "item.ITEM_PROPERTIES")?;
        let property = item_mut(&mut self.save, container, index)?
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
                .item_property_name(property_id)
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
        match target {
            CharacterTarget::MainCharacter => Ok(&self.save.main_character),
            CharacterTarget::Companion(index) => self
                .save
                .companions
                .get(index)
                .ok_or(EditError::InvalidTarget { target }),
        }
    }

    fn character_mut(&mut self, target: CharacterTarget) -> Result<&mut Character, EditError> {
        match target {
            CharacterTarget::MainCharacter => Ok(&mut self.save.main_character),
            CharacterTarget::Companion(index) => self
                .save
                .companions
                .get_mut(index)
                .ok_or(EditError::InvalidTarget { target }),
        }
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
        stat_id: u32,
        value: u32,
    ) -> Result<(), EditError> {
        let raw_character = raw_character_mut(&mut self.raw, target)?;
        set_character_stat_row_value(raw_character, stat_id, value, target)?;
        let point_pools = &mut self.character_mut(target)?.point_pools;
        match stat_id {
            34 => point_pools.attribute_points = Some(value),
            35 => point_pools.skill_points = Some(value),
            36 => point_pools.talent_points = Some(value),
            38 => point_pools.specialization_points = Some(value),
            _ => {}
        }
        Ok(())
    }
}

fn raw_party_mut(raw: &mut GffFile) -> Result<&mut GffStruct, EditError> {
    raw.root_mut()
        .get_struct_mut(SAVEGAME_PARTYLIST)
        .ok_or_else(|| EditError::MissingField {
            path: "root.SAVEGAME_PARTYLIST".to_string(),
        })
}

fn raw_character(raw: &GffFile, target: CharacterTarget) -> Result<&GffStruct, EditError> {
    match target {
        CharacterTarget::MainCharacter => {
            let player = raw
                .root()
                .get_struct_by_name(SAVEGAME_PLAYERCHAR_NAME)
                .ok_or_else(|| EditError::MissingField {
                    path: "root.SAVEGAME_PLAYERCHAR".to_string(),
                })?;
            player
                .get_struct_by_name(SAVEGAME_PLAYERCHAR_CHAR_NAME)
                .ok_or_else(|| EditError::MissingField {
                    path: "root.SAVEGAME_PLAYERCHAR.SAVEGAME_PLAYERCHAR_CHAR".to_string(),
                })
        }
        CharacterTarget::Companion(index) => {
            let party = raw
                .root()
                .get_struct(SAVEGAME_PARTYLIST)
                .ok_or_else(|| EditError::MissingField {
                    path: "root.SAVEGAME_PARTYLIST".to_string(),
                })?;
            let companions = party
                .get_list_by_name(SAVEGAME_PARTYPOOLMEMBERS_NAME)
                .ok_or_else(|| EditError::MissingField {
                    path: "root.SAVEGAME_PARTYLIST.SAVEGAME_PARTYPOOLMEMBERS".to_string(),
                })?;

            companions
                .iter()
                .filter_map(Value::as_struct)
                .nth(index)
                .ok_or(EditError::InvalidTarget { target })
        }
    }
}

fn raw_character_mut(raw: &mut GffFile, target: CharacterTarget) -> Result<&mut GffStruct, EditError> {
    match target {
        CharacterTarget::MainCharacter => {
            let player = raw
                .root_mut()
                .get_struct_mut_by_name(SAVEGAME_PLAYERCHAR_NAME)
                .ok_or_else(|| EditError::MissingField {
                    path: "root.SAVEGAME_PLAYERCHAR".to_string(),
                })?;
            player
                .get_struct_mut_by_name(SAVEGAME_PLAYERCHAR_CHAR_NAME)
                .ok_or_else(|| EditError::MissingField {
                    path: "root.SAVEGAME_PLAYERCHAR.SAVEGAME_PLAYERCHAR_CHAR".to_string(),
                })
        }
        CharacterTarget::Companion(index) => {
            let party = raw_party_mut(raw)?;
            let companions = party
                .get_list_mut_by_name(SAVEGAME_PARTYPOOLMEMBERS_NAME)
                .ok_or_else(|| EditError::MissingField {
                    path: "root.SAVEGAME_PARTYLIST.SAVEGAME_PARTYPOOLMEMBERS".to_string(),
                })?;

            let mut current_struct_index = 0usize;
            for value in companions {
                if let Value::Struct(structure) = value {
                    if current_struct_index == index {
                        return Ok(structure.as_mut());
                    }
                    current_struct_index += 1;
                }
            }

            Err(EditError::InvalidTarget { target })
        }
    }
}

fn raw_item_mut(
    raw: &mut GffFile,
    container: InventoryContainer,
    index: usize,
) -> Result<&mut GffStruct, EditError> {
    let items = match container {
        InventoryContainer::Backpack => raw_party_mut(raw)?
            .get_list_mut(SAVEGAME_BACKPACK)
            .ok_or_else(|| EditError::MissingField {
                path: "root.SAVEGAME_PARTYLIST.SAVEGAME_BACKPACK".to_string(),
            })?,
        InventoryContainer::Equipment { target } => raw_character_mut(raw, target)?
            .get_list_mut(SAVEGAME_EQUIPMENT_ITEMS)
            .ok_or_else(|| EditError::MissingField {
                path: "character.SAVEGAME_EQUIPMENT_ITEMS".to_string(),
            })?,
    };
    let raw_index = nth_struct_index(items, index).ok_or(EditError::InvalidItemIndex {
        container,
        index,
    })?;
    items[raw_index]
        .as_struct_mut()
        .ok_or(EditError::InvalidItemIndex { container, index })
}

fn nth_struct_mut(values: &mut [Value], index: usize) -> Option<&mut GffStruct> {
    let mut current = 0usize;
    for value in values {
        if let Value::Struct(structure) = value {
            if current == index {
                return Some(structure.as_mut());
            }
            current += 1;
        }
    }
    None
}

fn item_mut(
    save: &mut SaveGame,
    container: InventoryContainer,
    index: usize,
) -> Result<&mut Item, EditError> {
    match container {
        InventoryContainer::Backpack => save
            .backpack
            .get_mut(index)
            .ok_or(EditError::InvalidItemIndex { container, index }),
        InventoryContainer::Equipment { target } => match target {
            CharacterTarget::MainCharacter => save
                .main_character
                .equipment
                .get_mut(index)
                .ok_or(EditError::InvalidItemIndex { container, index }),
            CharacterTarget::Companion(companion_index) => save
                .companions
                .get_mut(companion_index)
                .ok_or(EditError::InvalidTarget { target })?
                .equipment
                .get_mut(index)
                .ok_or(EditError::InvalidItemIndex { container, index }),
        },
    }
}

fn load_validated_abilities(
    target: CharacterTarget,
    list: AbilityListKind,
    ability_ids: &[u32],
    current_ids: &BTreeSet<u32>,
    lookup: &dyn GameDataLookup,
    preferred_game: Option<GameId>,
) -> Result<Vec<AbilityRef>, EditError> {
    let mut abilities = Vec::with_capacity(ability_ids.len());
    let mut replacement_ids = BTreeSet::new();
    let expected_kind = expected_ability_kind(list);

    for &ability_id in ability_ids {
        let ability = lookup
            .ability(ability_id, preferred_game)
            .map_err(|err| EditError::LookupFailed {
                path: "character.ability_list".to_string(),
                detail: err.to_string(),
            })?
            .ok_or(EditError::UnknownAbility { ability_id })?;
        if ability.kind != expected_kind {
            return Err(EditError::InvalidAbilityKind {
                ability_id,
                expected: list,
                actual: ability.kind,
            });
        }
        replacement_ids.insert(ability.id);
        abilities.push(ability);
    }

    let mut required_core_ids = BTreeSet::new();
    for ability in &abilities {
        for &core_id in &ability.core_ids {
            let Some(core_ability) = lookup
                .ability(core_id, preferred_game)
                .map_err(|err| EditError::LookupFailed {
                    path: "character.ability_list".to_string(),
                    detail: err.to_string(),
                })?
            else {
                continue;
            };
            if matches!(
                core_ability.ability_type.as_deref().map(str::trim),
                Some("Class") | Some("Specialization")
            ) {
                continue;
            }
            if core_ability.kind == expected_kind {
                required_core_ids.insert(core_id);
            }
        }
    }
    for &core_id in &required_core_ids {
        if !replacement_ids.contains(&core_id) {
            return Err(EditError::MissingCoreAbility {
                target,
                list,
                required_id: core_id,
            });
        }
    }

    let mut currently_owned_cores = BTreeSet::new();
    for &ability_id in current_ids {
        let Some(ability) = lookup
            .ability(ability_id, preferred_game)
            .map_err(|err| EditError::LookupFailed {
                path: "character.ability_list".to_string(),
                detail: err.to_string(),
            })?
        else {
            continue;
        };
        for &core_id in &ability.core_ids {
            let Some(core_ability) = lookup
                .ability(core_id, preferred_game)
                .map_err(|err| EditError::LookupFailed {
                    path: "character.ability_list".to_string(),
                    detail: err.to_string(),
                })?
            else {
                continue;
            };
            if matches!(
                core_ability.ability_type.as_deref().map(str::trim),
                Some("Class") | Some("Specialization")
            ) {
                continue;
            }
            if core_ability.kind != expected_kind {
                continue;
            }
            if current_ids.contains(&core_id) {
                currently_owned_cores.insert(core_id);
            }
        }
    }
    for core_id in currently_owned_cores {
        if !replacement_ids.contains(&core_id) {
            return Err(EditError::MissingCoreAbility {
                target,
                list,
                required_id: core_id,
            });
        }
    }

    Ok(abilities)
}

fn expected_ability_kind(list: AbilityListKind) -> AbilityKind {
    match list {
        AbilityListKind::Skills => AbilityKind::Skill,
        AbilityListKind::Talents => AbilityKind::Talent,
        AbilityListKind::Spells => AbilityKind::Spell,
    }
}

fn merged_da2_ability_ids(
    character: &Character,
    replaced_list: AbilityListKind,
    replacement_ids: &[u32],
) -> Vec<u32> {
    let mut merged = Vec::new();

    let mut push_existing = |list_kind: AbilityListKind, abilities: &[AbilityRef]| {
        if list_kind == replaced_list {
            merged.extend_from_slice(replacement_ids);
        } else {
            merged.extend(abilities.iter().map(|ability| ability.id));
        }
    };

    push_existing(AbilityListKind::Skills, &character.skills);
    push_existing(AbilityListKind::Talents, &character.talents);
    push_existing(AbilityListKind::Spells, &character.spells);

    merged
}

fn uses_da2_combined_ability_list(
    raw: &GffFile,
    target: CharacterTarget,
    preferred_game: Option<GameId>,
) -> Result<bool, EditError> {
    if preferred_game != Some(GameId::Da2) {
        return Ok(false);
    }

    let raw_character = raw_character(raw, target)?;
    let Some(stats) = raw_character.get_struct_by_name(SAVEGAME_CREATURE_STATS_NAME) else {
        return Ok(false);
    };

    Ok(stats.get_by_name(SAVEGAME_ABILITYLIST_NAME).is_some())
}

fn ability_list_path(list: AbilityListKind) -> &'static str {
    match list {
        AbilityListKind::Skills => "character.SAVEGAME_CREATURE_STATS.SAVEGAME_SKILLLIST",
        AbilityListKind::Talents => "character.SAVEGAME_CREATURE_STATS.SAVEGAME_TALENTLIST",
        AbilityListKind::Spells => "character.SAVEGAME_CREATURE_STATS.SAVEGAME_SPELLLIST",
    }
}

fn ability_list_label(list: AbilityListKind) -> u32 {
    match list {
        AbilityListKind::Skills => SAVEGAME_SKILLLIST,
        AbilityListKind::Talents => SAVEGAME_TALENTLIST,
        AbilityListKind::Spells => SAVEGAME_SPELLLIST,
    }
}

fn nth_struct_index(values: &[Value], target_index: usize) -> Option<usize> {
    let mut struct_index = 0usize;
    for (index, value) in values.iter().enumerate() {
        if matches!(value, Value::Struct(_)) {
            if struct_index == target_index {
                return Some(index);
            }
            struct_index += 1;
        }
    }
    None
}

fn apply_item_metadata_patch_to_struct(
    item: &mut GffStruct,
    patch: ItemMetadataPatch,
) -> Result<(), EditError> {
    if let Some(item_cost) = patch.item_cost {
        let value = item
            .get_mut(ITEM_COST)
            .ok_or_else(|| EditError::MissingField {
                path: "item.ITEM_COST".to_string(),
            })?;
        set_numeric_value(value, item_cost, "item.ITEM_COST")?;
    }
    if let Some(material) = patch.material {
        let value = item
            .get_mut(SAVEGAME_ITEM_MATERIALTYPE)
            .ok_or_else(|| EditError::MissingField {
                path: "item.SAVEGAME_ITEM_MATERIALTYPE".to_string(),
            })?;
        set_numeric_value(value, material, "item.SAVEGAME_ITEM_MATERIALTYPE")?;
    }
    if let Some(item_level) = patch.item_level {
        let value = item
            .get_mut(SAVEGAME_OBJECT_PLOT)
            .ok_or_else(|| EditError::MissingField {
                path: "item.SAVEGAME_OBJECT_PLOT".to_string(),
            })?;
        set_numeric_value(value, item_level as u32, "item.SAVEGAME_OBJECT_PLOT")?;
    }
    Ok(())
}

fn apply_item_metadata_patch_to_domain(item: &mut Item, patch: ItemMetadataPatch) {
    if let Some(item_cost) = patch.item_cost {
        item.item_cost = Some(item_cost);
    }
    if let Some(material) = patch.material {
        item.material = Some(material);
    }
    if let Some(item_level) = patch.item_level {
        item.item_level = Some(item_level);
    }
}

fn property_lists_mut(
    item: &mut GffStruct,
    container: InventoryContainer,
    item_index: usize,
) -> Result<(&mut Vec<Value>, &mut Vec<Value>), EditError> {
    let ids_label = crate::gff4::fields::field_id_by_name(ITEM_PROPERTIES_NAME).ok_or_else(|| {
        EditError::MissingField {
            path: "item.ITEM_PROPERTIES".to_string(),
        }
    })?;
    let powers_label =
        crate::gff4::fields::field_id_by_name(ITEM_PROPERTY_POWERS_NAME).ok_or_else(|| {
            EditError::MissingField {
                path: "item.ITEM_PROPERTY_POWERS".to_string(),
            }
        })?;
    let ids_index = item.fields.iter().position(|field| field.label == ids_label);
    let powers_index = item.fields.iter().position(|field| field.label == powers_label);
    let (Some(ids_index), Some(powers_index)) = (ids_index, powers_index) else {
        return Err(EditError::InvalidPropertyArrayParity {
            container,
            item_index,
            ids_len: usize::from(ids_index.is_some()),
            powers_len: usize::from(powers_index.is_some()),
        });
    };
    if ids_index == powers_index {
        return Err(EditError::MissingField {
            path: "item.ITEM_PROPERTIES".to_string(),
        });
    }
    let (first_index, second_index, ids_first) = if ids_index < powers_index {
        (ids_index, powers_index, true)
    } else {
        (powers_index, ids_index, false)
    };
    let (left, right) = item.fields.split_at_mut(second_index);
    let first = &mut left[first_index].value;
    let second = &mut right[0].value;
    let first_type = first.type_name();
    let second_type = second.type_name();
    let first_list = first.as_list_mut().ok_or_else(|| EditError::TypeMismatch {
        path: if ids_first {
            "item.ITEM_PROPERTIES".to_string()
        } else {
            "item.ITEM_PROPERTY_POWERS".to_string()
        },
        expected: "List",
        actual: first_type,
    })?;
    let second_list = second.as_list_mut().ok_or_else(|| EditError::TypeMismatch {
        path: if ids_first {
            "item.ITEM_PROPERTY_POWERS".to_string()
        } else {
            "item.ITEM_PROPERTIES".to_string()
        },
        expected: "List",
        actual: second_type,
    })?;
    let (property_ids, property_powers) = if ids_first {
        (first_list, second_list)
    } else {
        (second_list, first_list)
    };

    if property_ids.len() != property_powers.len() {
        return Err(EditError::InvalidPropertyArrayParity {
            container,
            item_index,
            ids_len: property_ids.len(),
            powers_len: property_powers.len(),
        });
    }

    Ok((property_ids, property_powers))
}

fn ensure_property_lists_mut(
    container: InventoryContainer,
    item_index: usize,
    item: &mut GffStruct,
) -> Result<(&mut Vec<Value>, &mut Vec<Value>), EditError> {
    let ids_exists = item.get_by_name(ITEM_PROPERTIES_NAME).is_some();
    let powers_exists = item.get_by_name(ITEM_PROPERTY_POWERS_NAME).is_some();

    match (ids_exists, powers_exists) {
        (false, false) => {}
        (true, true) => return property_lists_mut(item, container, item_index),
        _ => {
            return Err(EditError::InvalidPropertyArrayParity {
                container,
                item_index,
                ids_len: if ids_exists { 1 } else { 0 },
                powers_len: if powers_exists { 1 } else { 0 },
            })
        }
    }

    if item.get_list_by_name(ITEM_PROPERTIES_NAME).is_none() {
        let label = crate::gff4::fields::field_id_by_name(ITEM_PROPERTIES_NAME).ok_or_else(|| {
            EditError::MissingField {
                path: "item.ITEM_PROPERTIES".to_string(),
            }
        })?;
        item.fields.push(FieldValue {
            label,
            value: Value::List(Vec::new()),
        });
    }
    if item.get_list_by_name(ITEM_PROPERTY_POWERS_NAME).is_none() {
        let label = crate::gff4::fields::field_id_by_name(ITEM_PROPERTY_POWERS_NAME).ok_or_else(|| {
            EditError::MissingField {
                path: "item.ITEM_PROPERTY_POWERS".to_string(),
            }
        })?;
        item.fields.push(FieldValue {
            label,
            value: Value::List(Vec::new()),
        });
    }
    property_lists_mut(item, container, item_index)
}

fn append_numeric_value(values: &mut Vec<Value>, new_value: u32, path: &str) -> Result<(), EditError> {
    let kind = values
        .iter()
        .find_map(NumericValueKind::from_value)
        .unwrap_or(NumericValueKind::UInt32);
    values.push(kind.build_value(new_value, path)?);
    Ok(())
}

fn append_float_value(values: &mut Vec<Value>, new_value: f32, path: &str) -> Result<(), EditError> {
    let kind = values
        .iter()
        .find_map(FloatValueKind::from_value)
        .unwrap_or(FloatValueKind::Float32);
    values.push(kind.build_value(new_value));
    let last = values.last_mut().expect("just pushed");
    set_float_value(last, new_value, path)?;
    Ok(())
}

fn replace_numeric_list(values: &mut Vec<Value>, new_values: &[u32], path: &str) -> Result<(), EditError> {
    let kind = values
        .iter()
        .find_map(NumericValueKind::from_value)
        .unwrap_or(NumericValueKind::UInt32);
    let mut rebuilt = Vec::with_capacity(new_values.len());
    for &new_value in new_values {
        rebuilt.push(kind.build_value(new_value, path)?);
    }
    *values = rebuilt;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum NumericValueKind {
    UInt8,
    Int8,
    UInt16,
    Int16,
    UInt32,
    Int32,
    Float32,
    Float64,
}

impl NumericValueKind {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::UInt8(_) => Some(Self::UInt8),
            Value::Int8(_) => Some(Self::Int8),
            Value::UInt16(_) => Some(Self::UInt16),
            Value::Int16(_) => Some(Self::Int16),
            Value::UInt32(_) => Some(Self::UInt32),
            Value::Int32(_) => Some(Self::Int32),
            Value::Float32(_) => Some(Self::Float32),
            Value::Float64(_) => Some(Self::Float64),
            _ => None,
        }
    }

    fn build_value(self, new_value: u32, path: &str) -> Result<Value, EditError> {
        let mut value = match self {
            Self::UInt8 => Value::UInt8(0),
            Self::Int8 => Value::Int8(0),
            Self::UInt16 => Value::UInt16(0),
            Self::Int16 => Value::Int16(0),
            Self::UInt32 => Value::UInt32(0),
            Self::Int32 => Value::Int32(0),
            Self::Float32 => Value::Float32(0.0),
            Self::Float64 => Value::Float64(0.0),
        };
        set_numeric_value(&mut value, new_value, path)?;
        Ok(value)
    }
}

#[derive(Debug, Clone, Copy)]
enum FloatValueKind {
    Float32,
    Float64,
    UInt8,
    Int8,
    UInt16,
    Int16,
    UInt32,
    Int32,
}

impl FloatValueKind {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::Float32(_) => Some(Self::Float32),
            Value::Float64(_) => Some(Self::Float64),
            Value::UInt8(_) => Some(Self::UInt8),
            Value::Int8(_) => Some(Self::Int8),
            Value::UInt16(_) => Some(Self::UInt16),
            Value::Int16(_) => Some(Self::Int16),
            Value::UInt32(_) => Some(Self::UInt32),
            Value::Int32(_) => Some(Self::Int32),
            _ => None,
        }
    }

    fn build_value(self, new_value: f32) -> Value {
        match self {
            Self::Float32 => Value::Float32(new_value),
            Self::Float64 => Value::Float64(new_value as f64),
            Self::UInt8 => Value::UInt8(new_value as u8),
            Self::Int8 => Value::Int8(new_value as i8),
            Self::UInt16 => Value::UInt16(new_value as u16),
            Self::Int16 => Value::Int16(new_value as i16),
            Self::UInt32 => Value::UInt32(new_value as u32),
            Self::Int32 => Value::Int32(new_value as i32),
        }
    }
}

fn set_character_stat_row_value(
    character: &mut GffStruct,
    stat_id: u32,
    new_value: u32,
    target: CharacterTarget,
) -> Result<(), EditError> {
    let stats = character
        .get_struct_mut_by_name(SAVEGAME_CREATURE_STATS_NAME)
        .ok_or_else(|| EditError::MissingField {
            path: "character.SAVEGAME_CREATURE_STATS".to_string(),
        })?;
    let stat_list = stats
        .get_list_mut_by_name(SAVEGAME_STATLIST_NAME)
        .ok_or_else(|| EditError::MissingField {
            path: "character.SAVEGAME_STATLIST".to_string(),
        })?;

    for stat_row in stat_list {
        let Some(row) = stat_row.as_struct_mut() else {
            continue;
        };
        let row_id = row
            .get_by_name(SAVEGAME_STATPROPERTY_INDEX_NAME)
            .and_then(value_to_u32)
            .ok_or_else(|| EditError::TypeMismatch {
                path: "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_INDEX".to_string(),
                expected: "UInt32-compatible number",
                actual: row
                    .get_by_name(SAVEGAME_STATPROPERTY_INDEX_NAME)
                    .map(Value::type_name)
                    .unwrap_or("Missing"),
            })?;

        if row_id == stat_id {
            let value = row
                .get_mut_by_name(SAVEGAME_STATPROPERTY_BASE_NAME)
                .ok_or_else(|| EditError::MissingField {
                    path: "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_BASE".to_string(),
                })?;
            return set_numeric_value(
                value,
                new_value,
                "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_BASE",
            );
        }
    }

    Err(EditError::MissingStatRow { target, stat_id })
}

fn set_numeric_value(value: &mut Value, new_value: u32, path: &str) -> Result<(), EditError> {
    match value {
        Value::UInt8(existing) => {
            *existing = u8::try_from(new_value).map_err(|_| EditError::NumericRange {
                path: path.to_string(),
                detail: format!("{new_value} does not fit into u8"),
            })?;
            Ok(())
        }
        Value::Int8(existing) => {
            *existing = i8::try_from(new_value).map_err(|_| EditError::NumericRange {
                path: path.to_string(),
                detail: format!("{new_value} does not fit into i8"),
            })?;
            Ok(())
        }
        Value::UInt16(existing) => {
            *existing = u16::try_from(new_value).map_err(|_| EditError::NumericRange {
                path: path.to_string(),
                detail: format!("{new_value} does not fit into u16"),
            })?;
            Ok(())
        }
        Value::Int16(existing) => {
            *existing = i16::try_from(new_value).map_err(|_| EditError::NumericRange {
                path: path.to_string(),
                detail: format!("{new_value} does not fit into i16"),
            })?;
            Ok(())
        }
        Value::UInt32(existing) => {
            *existing = new_value;
            Ok(())
        }
        Value::Int32(existing) => {
            *existing = i32::try_from(new_value).map_err(|_| EditError::NumericRange {
                path: path.to_string(),
                detail: format!("{new_value} does not fit into i32"),
            })?;
            Ok(())
        }
        Value::Float32(existing) => {
            *existing = new_value as f32;
            Ok(())
        }
        Value::Float64(existing) => {
            *existing = new_value as f64;
            Ok(())
        }
        other => Err(EditError::UnsupportedNumericValue {
            path: path.to_string(),
            actual: other.type_name(),
        }),
    }
}

fn set_signed_numeric_value(value: &mut Value, new_value: i32, path: &str) -> Result<(), EditError> {
    match value {
        Value::UInt8(existing) => {
            *existing = u8::try_from(new_value).map_err(|_| EditError::NumericRange {
                path: path.to_string(),
                detail: format!("{new_value} does not fit into u8"),
            })?;
            Ok(())
        }
        Value::Int8(existing) => {
            *existing = i8::try_from(new_value).map_err(|_| EditError::NumericRange {
                path: path.to_string(),
                detail: format!("{new_value} does not fit into i8"),
            })?;
            Ok(())
        }
        Value::UInt16(existing) => {
            *existing = u16::try_from(new_value).map_err(|_| EditError::NumericRange {
                path: path.to_string(),
                detail: format!("{new_value} does not fit into u16"),
            })?;
            Ok(())
        }
        Value::Int16(existing) => {
            *existing = i16::try_from(new_value).map_err(|_| EditError::NumericRange {
                path: path.to_string(),
                detail: format!("{new_value} does not fit into i16"),
            })?;
            Ok(())
        }
        Value::UInt32(existing) => {
            *existing = u32::try_from(new_value).map_err(|_| EditError::NumericRange {
                path: path.to_string(),
                detail: format!("{new_value} does not fit into u32"),
            })?;
            Ok(())
        }
        Value::Int32(existing) => {
            *existing = new_value;
            Ok(())
        }
        Value::Float32(existing) => {
            *existing = new_value as f32;
            Ok(())
        }
        Value::Float64(existing) => {
            *existing = new_value as f64;
            Ok(())
        }
        other => Err(EditError::UnsupportedNumericValue {
            path: path.to_string(),
            actual: other.type_name(),
        }),
    }
}

fn set_float_value(value: &mut Value, new_value: f32, path: &str) -> Result<(), EditError> {
    match value {
        Value::Float32(existing) => {
            *existing = new_value;
            Ok(())
        }
        Value::Float64(existing) => {
            *existing = new_value as f64;
            Ok(())
        }
        Value::UInt8(existing) => {
            if new_value.is_finite() && new_value >= 0.0 && new_value <= u8::MAX as f32 {
                *existing = new_value as u8;
                Ok(())
            } else {
                Err(EditError::NumericRange {
                    path: path.to_string(),
                    detail: format!("{new_value} does not fit into u8"),
                })
            }
        }
        Value::Int8(existing) => {
            if new_value.is_finite() && new_value >= i8::MIN as f32 && new_value <= i8::MAX as f32 {
                *existing = new_value as i8;
                Ok(())
            } else {
                Err(EditError::NumericRange {
                    path: path.to_string(),
                    detail: format!("{new_value} does not fit into i8"),
                })
            }
        }
        Value::UInt16(existing) => {
            if new_value.is_finite() && new_value >= 0.0 && new_value <= u16::MAX as f32 {
                *existing = new_value as u16;
                Ok(())
            } else {
                Err(EditError::NumericRange {
                    path: path.to_string(),
                    detail: format!("{new_value} does not fit into u16"),
                })
            }
        }
        Value::Int16(existing) => {
            if new_value.is_finite() && new_value >= i16::MIN as f32 && new_value <= i16::MAX as f32 {
                *existing = new_value as i16;
                Ok(())
            } else {
                Err(EditError::NumericRange {
                    path: path.to_string(),
                    detail: format!("{new_value} does not fit into i16"),
                })
            }
        }
        Value::UInt32(existing) => {
            if new_value.is_finite() && new_value >= 0.0 && new_value <= u32::MAX as f32 {
                *existing = new_value as u32;
                Ok(())
            } else {
                Err(EditError::NumericRange {
                    path: path.to_string(),
                    detail: format!("{new_value} does not fit into u32"),
                })
            }
        }
        Value::Int32(existing) => {
            if new_value.is_finite() && new_value >= i32::MIN as f32 && new_value <= i32::MAX as f32 {
                *existing = new_value as i32;
                Ok(())
            } else {
                Err(EditError::NumericRange {
                    path: path.to_string(),
                    detail: format!("{new_value} does not fit into i32"),
                })
            }
        }
        other => Err(EditError::UnsupportedNumericValue {
            path: path.to_string(),
            actual: other.type_name(),
        }),
    }
}

fn value_to_u32(value: &Value) -> Option<u32> {
    match value {
        Value::UInt8(v) => Some(*v as u32),
        Value::Int8(v) if *v >= 0 => Some(*v as u32),
        Value::UInt16(v) => Some(*v as u32),
        Value::Int16(v) if *v >= 0 => Some(*v as u32),
        Value::UInt32(v) => Some(*v),
        Value::Int32(v) if *v >= 0 => Some(*v as u32),
        Value::Float32(v) if v.is_finite() && *v >= 0.0 => Some(*v as u32),
        Value::Float64(v) if v.is_finite() && *v >= 0.0 => Some(*v as u32),
        _ => None,
    }
}

fn core_stat_id(stat: CoreStat) -> u32 {
    match stat {
        CoreStat::Strength => 1,
        CoreStat::Dexterity => 2,
        CoreStat::Willpower => 3,
        CoreStat::Magic => 4,
        CoreStat::Cunning => 5,
        CoreStat::Constitution => 6,
    }
}

fn clean_resref(value: &str) -> String {
    value.trim_end_matches('\0').trim().to_ascii_lowercase()
}

trait CharacterAbilityAccess {
    fn ability_list(&self, list: AbilityListKind) -> &[AbilityRef];
}

impl CharacterAbilityAccess for Character {
    fn ability_list(&self, list: AbilityListKind) -> &[AbilityRef] {
        match list {
            AbilityListKind::Skills => &self.skills,
            AbilityListKind::Talents => &self.talents,
            AbilityListKind::Spells => &self.spells,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AbilityListKind, BackpackItemReplacement, CharacterTarget, EditError, InventoryContainer,
        ItemMetadataPatch, SaveEditor,
    };
    use crate::domain::gamedata::{GameDataLookup, GameId, SqliteGameData, DEFAULT_GAME_DATA_PATH};
    use crate::domain::save::SaveGame;
    use crate::domain::stats::{CoreStat, CoreStatsPatch};
    use crate::gff4::fields::{SAVEGAME_MONEY, SAVEGAME_PARTYLIST};
    use crate::gff4::GffFile;
    use crate::test_support::{da2_save_path, dao_save_path};
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn lists_characters_with_stable_targets() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let editor =
            SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
        let characters = editor.list_characters();
        assert_eq!(characters[0].target, CharacterTarget::MainCharacter);
        assert_eq!(characters[1].target, CharacterTarget::Companion(0));
    }

    #[test]
    fn setting_money_updates_expected_field() {
        let mut editor = SaveEditor::from_path(dao_save_path()).unwrap();
        editor.set_money(424242).unwrap();
        let party = editor.raw().root.get_struct(SAVEGAME_PARTYLIST).unwrap();
        assert_eq!(
            party.get(SAVEGAME_MONEY).and_then(super::value_to_u32),
            Some(424242)
        );
    }

    #[test]
    fn editing_companion_stats_targets_by_index() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let mut editor =
            SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
        let original_second = editor.save().companions[1].core_stats.magic;
        editor
            .patch_character_core_stats(
                CharacterTarget::Companion(0),
                CoreStatsPatch {
                    magic: Some(77),
                    ..CoreStatsPatch::default()
                },
            )
            .unwrap();
        assert_eq!(editor.save().companions[0].core_stats.magic, 77);
        assert_eq!(editor.save().companions[1].core_stats.magic, original_second);
    }

    #[test]
    fn missing_stat_row_returns_explicit_error() {
        let mut editor = SaveEditor::from_path(dao_save_path()).unwrap();
        remove_stat_row(&mut editor, CharacterTarget::MainCharacter, 1);
        let error = editor
            .set_character_stat(CharacterTarget::MainCharacter, CoreStat::Strength, 88)
            .unwrap_err();
        match error {
            EditError::MissingStatRow {
                target: CharacterTarget::MainCharacter,
                stat_id: 1,
            } => {}
            other => panic!("unexpected error: {other}"),
        }
    }

    #[test]
    fn replaces_skill_list_and_preserves_order() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let mut editor =
            SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
        let original = editor
            .save()
            .main_character
            .skills
            .iter()
            .map(|ability| ability.id)
            .collect::<Vec<_>>();
        let replacement = vec![original[1], original[0]];
        editor
            .replace_character_abilities(
                CharacterTarget::MainCharacter,
                AbilityListKind::Skills,
                &replacement,
                &lookup,
            )
            .unwrap();
        assert_eq!(
            editor
                .save()
                .main_character
                .skills
                .iter()
                .map(|ability| ability.id)
                .collect::<Vec<_>>(),
            replacement
        );
    }

    #[test]
    fn rejects_cross_type_ability_replacement() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let mut editor =
            SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
        let spell_id = editor
            .save()
            .main_character
            .talents
            .first()
            .map(|ability| ability.id)
            .unwrap();
        let error = editor
            .replace_character_abilities(
                CharacterTarget::MainCharacter,
                AbilityListKind::Skills,
                &[spell_id],
                &lookup,
            )
            .unwrap_err();
        match error {
            EditError::InvalidAbilityKind { .. } => {}
            other => panic!("unexpected error: {other}"),
        }
    }

    #[test]
    fn replaces_da2_talent_list_with_valid_da2_ids() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let replacement = vec![101000, 101001, 101010];
        let abilities = super::load_validated_abilities(
            CharacterTarget::MainCharacter,
            AbilityListKind::Talents,
            &replacement,
            &std::collections::BTreeSet::new(),
            &lookup,
            Some(crate::domain::gamedata::GameId::Da2),
        )
        .unwrap();

        assert_eq!(
            abilities.iter().map(|ability| ability.id).collect::<Vec<_>>(),
            replacement
        );
    }

    #[test]
    fn class_core_dependencies_do_not_block_talent_replacement() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let current_ids = std::collections::BTreeSet::from([23_u32]);
        let abilities = super::load_validated_abilities(
            CharacterTarget::MainCharacter,
            AbilityListKind::Talents,
            &[23],
            &current_ids,
            &lookup,
            Some(crate::domain::gamedata::GameId::Dao),
        )
        .unwrap();

        assert_eq!(abilities.len(), 1);
        assert_eq!(abilities[0].id, 23);
    }

    #[test]
    fn rejects_dao_talent_id_when_editing_da2_talents() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let mut editor =
            SaveEditor::from_path_with_lookup(da2_save_path(), Some(&lookup), None).unwrap();

        let error = editor
            .replace_character_abilities(
                CharacterTarget::MainCharacter,
                AbilityListKind::Talents,
                &[23],
                &lookup,
            )
            .unwrap_err();

        match error {
            EditError::UnknownAbility { ability_id: 23 } => {}
            other => panic!("unexpected error: {other}"),
        }
    }

    #[test]
    fn replacing_da2_talents_preserves_existing_spells() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let mut editor =
            SaveEditor::from_path_with_lookup(da2_save_path(), Some(&lookup), None).unwrap();
        let original_spells = editor
            .save()
            .main_character
            .spells
            .iter()
            .map(|ability| ability.id)
            .collect::<Vec<_>>();
        let mut replacement = editor
            .save()
            .main_character
            .talents
            .iter()
            .map(|ability| ability.id)
            .filter(|id| {
                lookup
                    .ability(*id, Some(GameId::Da2))
                    .unwrap()
                    .is_some()
            })
            .collect::<Vec<_>>();
        replacement.reverse();

        editor
            .replace_character_abilities(
                CharacterTarget::MainCharacter,
                AbilityListKind::Talents,
                &replacement,
                &lookup,
            )
            .unwrap();

        assert_eq!(
            editor
                .save()
                .main_character
                .spells
                .iter()
                .map(|ability| ability.id)
                .collect::<Vec<_>>(),
            original_spells
        );
    }

    #[test]
    fn patches_backpack_item_metadata() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let mut editor =
            SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
        editor
            .patch_item_metadata(
                InventoryContainer::Backpack,
                0,
                ItemMetadataPatch {
                    item_cost: Some(1234),
                    material: Some(5),
                    item_level: Some(2),
                },
            )
            .unwrap();
        let item = &editor.save().backpack[0];
        assert_eq!(item.item_cost, Some(1234));
        assert_eq!(item.material, Some(5));
        assert_eq!(item.item_level, Some(2));
    }

    #[test]
    fn removes_backpack_item() {
        let mut editor = SaveEditor::from_path(dao_save_path()).unwrap();
        let original_len = editor.save().backpack.len();
        editor.remove_backpack_item(0).unwrap();
        assert_eq!(editor.save().backpack.len(), original_len - 1);
    }

    #[test]
    fn allows_same_resref_backpack_replacement() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let mut editor =
            SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
        let resref = editor.save().backpack[0].resref.clone().unwrap();
        editor
            .replace_backpack_item(
                0,
                BackpackItemReplacement {
                    resref,
                    item_cost: Some(2222),
                    material: Some(3),
                    item_level: Some(4),
                },
            )
            .unwrap();
        assert_eq!(editor.save().backpack[0].item_cost, Some(2222));
    }

    #[test]
    fn rejects_backpack_replacement_with_different_resref() {
        let mut editor = SaveEditor::from_path(dao_save_path()).unwrap();
        let error = editor
            .replace_backpack_item(
                0,
                BackpackItemReplacement {
                    resref: "totally_different_item".to_string(),
                    item_cost: Some(1),
                    material: None,
                    item_level: None,
                },
            )
            .unwrap_err();
        match error {
            EditError::BackpackResrefMismatch { .. } => {}
            other => panic!("unexpected error: {other}"),
        }
    }

    #[test]
    fn patches_equipped_item_metadata_in_place() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let mut editor =
            SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
        editor
            .patch_item_metadata(
                InventoryContainer::Equipment {
                    target: CharacterTarget::MainCharacter,
                },
                0,
                ItemMetadataPatch {
                    item_cost: Some(555),
                    material: Some(7),
                    item_level: Some(9),
                },
            )
            .unwrap();
        let item = &editor
            .equipment_items(CharacterTarget::MainCharacter)
            .unwrap()[0];
        assert_eq!(item.item_cost, Some(555));
    }

    #[test]
    fn adds_item_property_to_backpack_item() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let mut editor =
            SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
        let index = first_backpack_item_with_properties(&editor).unwrap_or(0);
        let original_len = editor.save().backpack[index].properties.len();
        editor
            .add_item_property(InventoryContainer::Backpack, index, 3011, 12.5, Some(&lookup))
            .unwrap();
        let properties = &editor.save().backpack[index].properties;
        assert_eq!(properties.len(), original_len + 1);
        assert_eq!(properties.last().unwrap().id, 3011);
        assert_eq!(properties.last().unwrap().power, 12.5);
    }

    #[test]
    fn updates_item_property_power() {
        let mut editor = SaveEditor::from_path(dao_save_path()).unwrap();
        let index = first_backpack_item_with_properties(&editor).unwrap();
        editor
            .set_item_property_power(InventoryContainer::Backpack, index, 0, 33.0)
            .unwrap();
        assert_eq!(editor.save().backpack[index].properties[0].power, 33.0);
    }

    #[test]
    fn removes_item_property() {
        let mut editor = SaveEditor::from_path(dao_save_path()).unwrap();
        let index = first_backpack_item_with_properties(&editor).unwrap();
        let original_len = editor.save().backpack[index].properties.len();
        editor
            .remove_item_property(InventoryContainer::Backpack, index, 0)
            .unwrap();
        assert_eq!(editor.save().backpack[index].properties.len(), original_len - 1);
    }

    #[test]
    fn replaces_item_property_id_in_place() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let mut editor =
            SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
        let index = first_backpack_item_with_properties(&editor).unwrap();
        editor
            .set_item_property_id(InventoryContainer::Backpack, index, 0, 3011, Some(&lookup))
            .unwrap();
        assert_eq!(editor.save().backpack[index].properties[0].id, 3011);
    }

    #[test]
    fn rejects_property_edits_when_raw_arrays_are_mismatched() {
        let mut editor = SaveEditor::from_path(dao_save_path()).unwrap();
        let index = first_backpack_item_with_properties(&editor).unwrap();
        corrupt_first_backpack_property_power_list(&mut editor);

        let error = editor
            .set_item_property_power(InventoryContainer::Backpack, index, 0, 10.0)
            .unwrap_err();

        match error {
            EditError::InvalidPropertyArrayParity { .. } => {}
            other => panic!("unexpected error: {other}"),
        }
    }

    #[test]
    fn write_reload_main_character_level_edit() {
        let input = dao_save_path();
        let output = test_output_path("main-level-edit.das");
        let mut editor = SaveEditor::from_path(&input).unwrap();
        editor
            .set_character_level(CharacterTarget::MainCharacter, 31)
            .unwrap();
        editor.write_to_path(&output).unwrap();
        let reloaded = GffFile::from_path(&output).unwrap();
        let save = SaveGame::from_gff(&reloaded).unwrap();
        assert_eq!(save.main_character.level, Some(31));
    }

    #[test]
    fn write_reload_ability_edit() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let input = dao_save_path();
        let output = test_output_path("ability-edit.das");
        let mut editor =
            SaveEditor::from_path_with_lookup(&input, Some(&lookup), None).unwrap();
        let original = editor
            .save()
            .main_character
            .skills
            .iter()
            .map(|ability| ability.id)
            .collect::<Vec<_>>();
        let replacement = vec![original[1], original[0]];
        editor
            .replace_character_abilities(
                CharacterTarget::MainCharacter,
                AbilityListKind::Skills,
                &replacement,
                &lookup,
            )
            .unwrap();
        editor.write_to_path(&output).unwrap();
        let reloaded = GffFile::from_path(&output).unwrap();
        let save = SaveGame::from_gff_with_lookup(&reloaded, Some(&lookup), None).unwrap();
        assert_eq!(
            save.main_character
                .skills
                .iter()
                .map(|ability| ability.id)
                .collect::<Vec<_>>(),
            replacement
        );
    }

    #[test]
    fn write_reload_backpack_metadata_edit() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let input = dao_save_path();
        let output = test_output_path("backpack-metadata-edit.das");
        let mut editor =
            SaveEditor::from_path_with_lookup(&input, Some(&lookup), None).unwrap();
        editor
            .patch_item_metadata(
                InventoryContainer::Backpack,
                0,
                ItemMetadataPatch {
                    item_cost: Some(6060),
                    material: Some(4),
                    item_level: Some(2),
                },
            )
            .unwrap();
        editor.write_to_path(&output).unwrap();
        let reloaded = GffFile::from_path(&output).unwrap();
        let save = SaveGame::from_gff_with_lookup(&reloaded, Some(&lookup), None).unwrap();
        assert_eq!(save.backpack[0].item_cost, Some(6060));
    }

    #[test]
    fn write_reload_item_property_edit() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let input = dao_save_path();
        let output = test_output_path("item-property-edit.das");
        let mut editor =
            SaveEditor::from_path_with_lookup(&input, Some(&lookup), None).unwrap();
        let index = first_backpack_item_with_properties(&editor).unwrap();
        editor
            .set_item_property_power(InventoryContainer::Backpack, index, 0, 21.0)
            .unwrap();
        editor
            .add_item_property(InventoryContainer::Backpack, index, 3011, 9.0, Some(&lookup))
            .unwrap();
        editor.write_to_path(&output).unwrap();
        let reloaded = GffFile::from_path(&output).unwrap();
        let save = SaveGame::from_gff_with_lookup(&reloaded, Some(&lookup), None).unwrap();
        assert_eq!(save.backpack[index].properties[0].power, 21.0);
        assert_eq!(save.backpack[index].properties.last().unwrap().id, 3011);
    }

    #[test]
    fn write_reload_item_property_id_edit() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let input = dao_save_path();
        let output = test_output_path("item-property-id-edit.das");
        let mut editor =
            SaveEditor::from_path_with_lookup(&input, Some(&lookup), None).unwrap();
        let index = first_backpack_item_with_properties(&editor).unwrap();
        editor
            .set_item_property_id(InventoryContainer::Backpack, index, 0, 3011, Some(&lookup))
            .unwrap();
        editor.write_to_path(&output).unwrap();
        let reloaded = GffFile::from_path(&output).unwrap();
        let save = SaveGame::from_gff_with_lookup(&reloaded, Some(&lookup), None).unwrap();
        assert_eq!(save.backpack[index].properties[0].id, 3011);
    }

    fn remove_stat_row(editor: &mut SaveEditor, target: CharacterTarget, stat_id: u32) {
        let character = super::raw_character_mut(&mut editor.raw, target).unwrap();
        let stats = character
            .get_struct_mut_by_name(super::SAVEGAME_CREATURE_STATS_NAME)
            .unwrap();
        let stat_list = stats
            .get_list_mut_by_name(super::SAVEGAME_STATLIST_NAME)
            .unwrap();
        stat_list.retain(|value| {
            value
                .as_struct()
                .and_then(|row| row.get_by_name(super::SAVEGAME_STATPROPERTY_INDEX_NAME))
                .and_then(super::value_to_u32)
                != Some(stat_id)
        });
    }

    fn test_output_path(name: &str) -> PathBuf {
        let dir = PathBuf::from("target").join("test-output");
        fs::create_dir_all(&dir).unwrap();
        dir.join(name)
    }

    fn first_backpack_item_with_properties(editor: &SaveEditor) -> Option<usize> {
        editor
            .save()
            .backpack
            .iter()
            .position(|item| !item.properties.is_empty())
    }

    fn corrupt_first_backpack_property_power_list(editor: &mut SaveEditor) {
        let party = editor.raw.root_mut().get_struct_mut(SAVEGAME_PARTYLIST).unwrap();
        let items = party.get_list_mut(super::SAVEGAME_BACKPACK).unwrap();
        for value in items {
            let Some(item) = value.as_struct_mut() else {
                continue;
            };
            let Some(powers) = item.get_list_mut_by_name(super::ITEM_PROPERTY_POWERS_NAME) else {
                continue;
            };
            if !powers.is_empty() {
                powers.pop();
                return;
            }
        }
        panic!("expected backpack item with property powers");
    }
}
