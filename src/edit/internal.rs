use super::{
    AbilityListKind, CharacterTarget, EditError, InventoryContainer, ItemMetadataPatch,
    PlotBooleanPatch, PlotIntegerPatch,
};
use crate::domain::ability::{AbilityKind, AbilityRef};
use crate::domain::character::Character;
use crate::domain::game::{AbilityListStyle, GameBehavior, PropertyPowerEncoding};
use crate::domain::gamedata::{GameDataLookup, GameId};
use crate::domain::item::Item;
use crate::domain::save::{WORLD_VAULT_ID_LABEL, WORLD_VAULT_VALUE_LABEL};
use crate::edit::targets::RawSaveTargets;
use crate::gff4::fields::{
    ITEM_COST, ITEM_STACKSIZE, OBJECT_ID, SAVEGAME_ITEM_MATERIALTYPE, SAVEGAME_OBJECT_PLOT,
    SAVEGAME_PARTYLIST, SAVEGAME_SKILLLIST, SAVEGAME_SPELLLIST, SAVEGAME_TALENTLIST,
    SAVEGAME_WORLDDATABASE,
};
use crate::gff4::numeric::{self, NumericWriteError};
use crate::gff4::{FieldValue, GffFile, GffStruct, Value};
use std::collections::BTreeSet;

pub(super) const SAVEGAME_CREATURE_STATS_NAME: &str = "SAVEGAME_CREATURE_STATS";
pub(super) const SAVEGAME_STATLIST_NAME: &str = "SAVEGAME_STATLIST";
pub(super) const SAVEGAME_STATPROPERTY_INDEX_NAME: &str = "SAVEGAME_STATPROPERTY_INDEX";
pub(super) const SAVEGAME_STATPROPERTY_BASE_NAME: &str = "SAVEGAME_STATPROPERTY_BASE";
pub(super) const ITEM_PROPERTIES_NAME: &str = "ITEM_PROPERTIES";
pub(super) const ITEM_PROPERTY_POWERS_NAME: &str = "ITEM_PROPERTY_POWERS";
const SAVEGAME_ABILITYLIST_NAME: &str = "SAVEGAME_ABILITYLIST";
pub(super) const SAVEGAME_WORLDDB_LASTID: u32 = 16502;

pub(super) fn raw_party_mut(raw: &mut GffFile) -> Result<&mut GffStruct, EditError> {
    raw.root_mut()
        .get_struct_mut(SAVEGAME_PARTYLIST)
        .ok_or_else(|| EditError::MissingField {
            path: "root.SAVEGAME_PARTYLIST".to_string(),
        })
}

pub(super) fn set_or_insert_world_vault_bool(
    values: &mut Vec<Value>,
    patch: PlotBooleanPatch,
) -> Result<(), EditError> {
    if let Some(entry) = values
        .iter_mut()
        .filter_map(Value::as_struct_mut)
        .find(|entry| entry.get(WORLD_VAULT_ID_LABEL).and_then(value_to_u16) == Some(patch.id))
    {
        let value =
            entry
                .get_mut(WORLD_VAULT_VALUE_LABEL)
                .ok_or_else(|| EditError::MissingField {
                    path: format!("root.WVLT.WVB1[{}].value", patch.id),
                })?;
        set_world_vault_bool_value(value, patch.value)?;
        return Ok(());
    }

    let struct_index = world_vault_struct_index(values).unwrap_or(65);
    values.push(Value::Struct(Box::new(GffStruct {
        struct_index,
        fields: vec![
            FieldValue {
                label: WORLD_VAULT_ID_LABEL,
                value: Value::UInt16(patch.id),
            },
            FieldValue {
                label: WORLD_VAULT_VALUE_LABEL,
                value: Value::UInt8(u8::from(patch.value)),
            },
        ],
    })));
    Ok(())
}

pub(super) fn set_or_insert_world_vault_int(
    values: &mut Vec<Value>,
    patch: PlotIntegerPatch,
) -> Result<(), EditError> {
    if let Some(entry) = values
        .iter_mut()
        .filter_map(Value::as_struct_mut)
        .find(|entry| entry.get(WORLD_VAULT_ID_LABEL).and_then(value_to_u16) == Some(patch.id))
    {
        let value =
            entry
                .get_mut(WORLD_VAULT_VALUE_LABEL)
                .ok_or_else(|| EditError::MissingField {
                    path: format!("root.WVLT.WVI1[{}].value", patch.id),
                })?;
        set_signed_numeric_value(value, patch.value, "root.WVLT.WVI1.value")?;
        return Ok(());
    }

    let struct_index = world_vault_struct_index(values).unwrap_or(67);
    values.push(Value::Struct(Box::new(GffStruct {
        struct_index,
        fields: vec![
            FieldValue {
                label: WORLD_VAULT_ID_LABEL,
                value: Value::UInt16(patch.id),
            },
            FieldValue {
                label: WORLD_VAULT_VALUE_LABEL,
                value: Value::Int32(patch.value),
            },
        ],
    })));
    Ok(())
}

pub(super) fn world_vault_struct_index(values: &[Value]) -> Option<usize> {
    values.iter().find_map(|value| match value {
        Value::Struct(structure) => Some(structure.struct_index),
        _ => None,
    })
}

pub(super) fn set_world_vault_bool_value(
    value: &mut Value,
    new_value: bool,
) -> Result<(), EditError> {
    match value {
        Value::UInt8(value) => *value = u8::from(new_value),
        Value::Int8(value) => *value = if new_value { 1 } else { 0 },
        Value::UInt16(value) => *value = if new_value { 1 } else { 0 },
        Value::Int16(value) => *value = if new_value { 1 } else { 0 },
        Value::UInt32(value) => *value = if new_value { 1 } else { 0 },
        Value::Int32(value) => *value = if new_value { 1 } else { 0 },
        other => {
            return Err(EditError::UnsupportedNumericValue {
                path: "root.WVLT.WVB1.value".to_string(),
                actual: other.type_name(),
            });
        }
    }
    Ok(())
}

pub(super) fn load_validated_abilities(
    target: CharacterTarget,
    list: AbilityListKind,
    ability_ids: &[u32],
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

    for ability in &abilities {
        let mut enforceable_core_ids = Vec::new();
        for &core_id in &ability.core_ids {
            let Some(core_ability) =
                lookup
                    .ability(core_id, preferred_game)
                    .map_err(|err| EditError::LookupFailed {
                        path: "character.ability_list".to_string(),
                        detail: err.to_string(),
                    })?
            else {
                continue;
            };
            if should_enforce_core_ability(&core_ability, expected_kind) {
                enforceable_core_ids.push(core_id);
            }
        }
        if !enforceable_core_ids.is_empty()
            && !enforceable_core_ids
                .iter()
                .any(|core_id| replacement_ids.contains(core_id))
        {
            return Err(EditError::MissingCoreAbility {
                target,
                list,
                required_id: enforceable_core_ids[0],
            });
        }
    }

    Ok(abilities)
}

pub(super) fn should_enforce_core_ability(
    core_ability: &AbilityRef,
    expected_kind: AbilityKind,
) -> bool {
    core_ability.kind == expected_kind
}

pub(super) fn expected_ability_kind(list: AbilityListKind) -> AbilityKind {
    match list {
        AbilityListKind::Skills => AbilityKind::Skill,
        AbilityListKind::Talents => AbilityKind::Talent,
        AbilityListKind::Spells => AbilityKind::Spell,
    }
}

pub(super) fn merged_da2_ability_ids(
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

pub(super) fn uses_combined_ability_list(
    raw: &GffFile,
    target: CharacterTarget,
    preferred_game: Option<GameId>,
) -> Result<bool, EditError> {
    if preferred_game.ability_list_style() != AbilityListStyle::Combined {
        return Ok(false);
    }

    let raw_targets = RawSaveTargets::new(raw);
    let raw_character = raw_targets.character(target)?;
    let Some(stats) = raw_character.get_struct_by_name(SAVEGAME_CREATURE_STATS_NAME) else {
        return Ok(false);
    };

    Ok(stats.get_by_name(SAVEGAME_ABILITYLIST_NAME).is_some())
}

pub(super) fn ability_list_path(list: AbilityListKind) -> &'static str {
    match list {
        AbilityListKind::Skills => "character.SAVEGAME_CREATURE_STATS.SAVEGAME_SKILLLIST",
        AbilityListKind::Talents => "character.SAVEGAME_CREATURE_STATS.SAVEGAME_TALENTLIST",
        AbilityListKind::Spells => "character.SAVEGAME_CREATURE_STATS.SAVEGAME_SPELLLIST",
    }
}

pub(super) fn ability_list_label(list: AbilityListKind) -> u32 {
    match list {
        AbilityListKind::Skills => SAVEGAME_SKILLLIST,
        AbilityListKind::Talents => SAVEGAME_TALENTLIST,
        AbilityListKind::Spells => SAVEGAME_SPELLLIST,
    }
}

pub(super) fn next_object_id(raw: &GffFile) -> Result<u32, EditError> {
    let max_seen_object_id = max_object_id_in_struct(raw.root()).unwrap_or_default();
    let worlddb_last_id = raw
        .root()
        .get(SAVEGAME_WORLDDATABASE)
        .and_then(|value| find_field_value(value, SAVEGAME_WORLDDB_LASTID))
        .and_then(value_to_u32)
        .unwrap_or_default();
    max_seen_object_id
        .max(worlddb_last_id)
        .checked_add(1)
        .ok_or_else(|| EditError::NumericRange {
            path: "SAVEGAME_WORLDDATABASE.SAVEGAME_WORLDDB_LASTID".to_string(),
            detail: "next OBJECT_ID would overflow u32".to_string(),
        })
}

pub(super) fn update_worlddb_last_id(
    raw: &mut GffFile,
    new_object_id: u32,
) -> Result<(), EditError> {
    let worlddb = raw
        .root_mut()
        .get_mut(SAVEGAME_WORLDDATABASE)
        .ok_or_else(|| EditError::MissingField {
            path: "root.SAVEGAME_WORLDDATABASE".to_string(),
        })?;
    let value = find_field_value_mut(worlddb, SAVEGAME_WORLDDB_LASTID).ok_or_else(|| {
        EditError::MissingField {
            path: "root.SAVEGAME_WORLDDATABASE.SAVEGAME_WORLDDB_LASTID".to_string(),
        }
    })?;
    set_numeric_value(
        value,
        new_object_id,
        "root.SAVEGAME_WORLDDATABASE.SAVEGAME_WORLDDB_LASTID",
    )
}

pub(super) fn set_object_id(item: &mut GffStruct, object_id: u32) -> Result<(), EditError> {
    let value = item
        .get_mut(OBJECT_ID)
        .ok_or_else(|| EditError::MissingField {
            path: "item.OBJECT_ID".to_string(),
        })?;
    set_numeric_value(value, object_id, "item.OBJECT_ID")
}

pub(super) fn set_or_insert_stack_size(
    item: &mut GffStruct,
    stack_size: u32,
) -> Result<(), EditError> {
    if let Some(value) = item.get_mut(ITEM_STACKSIZE) {
        return set_numeric_value(value, stack_size, "item.ITEM_STACKSIZE");
    }
    item.fields.push(FieldValue {
        label: ITEM_STACKSIZE,
        value: Value::UInt32(stack_size),
    });
    Ok(())
}

pub(super) fn find_field_value(value: &Value, label: u32) -> Option<&Value> {
    match value {
        Value::Struct(structure) => find_field_value_in_struct(structure, label),
        Value::List(values) => values
            .iter()
            .find_map(|value| find_field_value(value, label)),
        _ => None,
    }
}

pub(super) fn find_field_value_in_struct(structure: &GffStruct, label: u32) -> Option<&Value> {
    if let Some(value) = structure.get(label) {
        return Some(value);
    }
    structure
        .fields
        .iter()
        .find_map(|field| find_field_value(&field.value, label))
}

pub(super) fn find_field_value_mut(value: &mut Value, label: u32) -> Option<&mut Value> {
    match value {
        Value::Struct(structure) => find_field_value_mut_in_struct(structure, label),
        Value::List(values) => values
            .iter_mut()
            .find_map(|value| find_field_value_mut(value, label)),
        _ => None,
    }
}

pub(super) fn find_field_value_mut_in_struct(
    structure: &mut GffStruct,
    label: u32,
) -> Option<&mut Value> {
    if let Some(index) = structure
        .fields
        .iter()
        .position(|field| field.label == label)
    {
        return Some(&mut structure.fields[index].value);
    }
    structure
        .fields
        .iter_mut()
        .find_map(|field| find_field_value_mut(&mut field.value, label))
}

pub(super) fn max_object_id_in_struct(structure: &GffStruct) -> Option<u32> {
    let mut max_id = structure
        .get(OBJECT_ID)
        .and_then(value_to_i32)
        .and_then(|id| u32::try_from(id).ok());

    for field in &structure.fields {
        max_id = max_id.max(max_object_id_in_value(&field.value));
    }

    max_id
}

pub(super) fn max_object_id_in_value(value: &Value) -> Option<u32> {
    match value {
        Value::Struct(structure) => max_object_id_in_struct(structure),
        Value::List(values) => values.iter().filter_map(max_object_id_in_value).max(),
        _ => None,
    }
}

pub(super) fn apply_item_metadata_patch_to_struct(
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
        let value =
            item.get_mut(SAVEGAME_ITEM_MATERIALTYPE)
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

pub(super) fn apply_item_metadata_patch_to_domain(item: &mut Item, patch: ItemMetadataPatch) {
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

pub(super) struct ItemProperties<'a> {
    ids: &'a mut Vec<Value>,
    powers: &'a mut Vec<Value>,
    container: InventoryContainer,
    item_index: usize,
    preferred_game: Option<GameId>,
}

impl<'a> ItemProperties<'a> {
    pub(super) fn from_item(
        item: &'a mut GffStruct,
        container: InventoryContainer,
        item_index: usize,
        preferred_game: Option<GameId>,
    ) -> Result<Self, EditError> {
        let (ids, powers) = property_lists_mut(item, container, item_index)?;
        Ok(Self {
            ids,
            powers,
            container,
            item_index,
            preferred_game,
        })
    }

    pub(super) fn from_item_or_create(
        item: &'a mut GffStruct,
        container: InventoryContainer,
        item_index: usize,
        preferred_game: Option<GameId>,
    ) -> Result<Self, EditError> {
        let (ids, powers) = ensure_property_lists_mut(container, item_index, item)?;
        Ok(Self {
            ids,
            powers,
            container,
            item_index,
            preferred_game,
        })
    }

    pub(super) fn push(&mut self, property_id: u32, power: f32) -> Result<(), EditError> {
        append_property_id_value(
            self.ids,
            property_id,
            self.preferred_game,
            "item.ITEM_PROPERTIES",
        )?;
        append_property_power_value(
            self.powers,
            power,
            self.preferred_game,
            "item.ITEM_PROPERTY_POWERS",
        )
    }

    pub(super) fn remove(&mut self, index: usize) -> Result<(), EditError> {
        self.validate_index(index)?;
        self.ids.remove(index);
        self.powers.remove(index);
        Ok(())
    }

    pub(super) fn set_id(&mut self, index: usize, property_id: u32) -> Result<(), EditError> {
        self.validate_index(index)?;
        let value = &mut self.ids[index];
        set_numeric_value(value, property_id, "item.ITEM_PROPERTIES")
    }

    pub(super) fn set_power(&mut self, index: usize, power: f32) -> Result<(), EditError> {
        self.validate_index(index)?;
        let value = &mut self.powers[index];
        set_property_power_value(
            value,
            power,
            self.preferred_game,
            "item.ITEM_PROPERTY_POWERS",
        )
    }

    pub(super) fn len(&self) -> usize {
        self.ids.len()
    }

    fn validate_index(&self, property_index: usize) -> Result<(), EditError> {
        if property_index >= self.len() {
            return Err(EditError::InvalidPropertyIndex {
                container: self.container,
                item_index: self.item_index,
                property_index,
            });
        }
        Ok(())
    }
}

fn property_lists_mut(
    item: &mut GffStruct,
    container: InventoryContainer,
    item_index: usize,
) -> Result<(&mut Vec<Value>, &mut Vec<Value>), EditError> {
    let ids_label =
        crate::gff4::fields::field_id_by_name(ITEM_PROPERTIES_NAME).ok_or_else(|| {
            EditError::MissingField {
                path: "item.ITEM_PROPERTIES".to_string(),
            }
        })?;
    let powers_label = crate::gff4::fields::field_id_by_name(ITEM_PROPERTY_POWERS_NAME)
        .ok_or_else(|| EditError::MissingField {
            path: "item.ITEM_PROPERTY_POWERS".to_string(),
        })?;
    let ids_index = item
        .fields
        .iter()
        .position(|field| field.label == ids_label);
    let powers_index = item
        .fields
        .iter()
        .position(|field| field.label == powers_label);
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
    let second_list = second
        .as_list_mut()
        .ok_or_else(|| EditError::TypeMismatch {
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
            });
        }
    }

    if item.get_list_by_name(ITEM_PROPERTIES_NAME).is_none() {
        let label =
            crate::gff4::fields::field_id_by_name(ITEM_PROPERTIES_NAME).ok_or_else(|| {
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
        let label =
            crate::gff4::fields::field_id_by_name(ITEM_PROPERTY_POWERS_NAME).ok_or_else(|| {
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

fn append_property_id_value(
    values: &mut Vec<Value>,
    new_value: u32,
    preferred_game: Option<GameId>,
    path: &str,
) -> Result<(), EditError> {
    let kind = values
        .iter()
        .find_map(NumericValueKind::from_value)
        .unwrap_or(match preferred_game {
            Some(GameId::Da2) => NumericValueKind::Float32,
            _ => NumericValueKind::UInt32,
        });
    values.push(kind.build_value(new_value, path)?);
    Ok(())
}

fn append_float_value(
    values: &mut Vec<Value>,
    new_value: f32,
    path: &str,
) -> Result<(), EditError> {
    let kind = values
        .iter()
        .find_map(FloatValueKind::from_value)
        .unwrap_or(FloatValueKind::Float32);
    let mut value = kind.build_value(new_value);
    set_float_value(&mut value, new_value, path)?;
    values.push(value);
    Ok(())
}

fn append_property_power_value(
    values: &mut Vec<Value>,
    new_value: f32,
    preferred_game: Option<GameId>,
    path: &str,
) -> Result<(), EditError> {
    match preferred_game.property_power_encoding() {
        PropertyPowerEncoding::Float => append_float_value(values, new_value, path),
        PropertyPowerEncoding::Da2Bitcast => {
            let kind = values
                .iter()
                .find_map(NumericValueKind::from_value)
                .unwrap_or(NumericValueKind::UInt32);
            let mut value = match kind {
                NumericValueKind::UInt8 => Value::UInt8(0),
                NumericValueKind::Int8 => Value::Int8(0),
                NumericValueKind::UInt16 => Value::UInt16(0),
                NumericValueKind::Int16 => Value::Int16(0),
                NumericValueKind::UInt32 => Value::UInt32(0),
                NumericValueKind::Int32 => Value::Int32(0),
                NumericValueKind::Float32 => Value::Float32(0.0),
                NumericValueKind::Float64 => Value::Float64(0.0),
            };
            set_property_power_value(&mut value, new_value, preferred_game, path)?;
            values.push(value);
            Ok(())
        }
    }
}

pub(super) fn replace_numeric_list(
    values: &mut Vec<Value>,
    new_values: &[u32],
    path: &str,
) -> Result<(), EditError> {
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
pub(super) enum NumericValueKind {
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
pub(super) enum FloatValueKind {
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

pub(super) fn set_character_stat_row_value(
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

pub(super) fn set_or_insert_character_stat_row_value(
    character: &mut GffStruct,
    stat_id: u32,
    new_value: u32,
    target: CharacterTarget,
) -> Result<(), EditError> {
    match set_character_stat_row_value(character, stat_id, new_value, target) {
        Ok(()) => Ok(()),
        Err(EditError::MissingStatRow {
            stat_id: missing_stat_id,
            ..
        }) if missing_stat_id == stat_id => {
            insert_character_stat_row_value(character, stat_id, new_value, target)
        }
        Err(err) => Err(err),
    }
}

pub(super) fn insert_character_stat_row_value(
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
    let template = stat_list
        .iter()
        .find_map(Value::as_struct)
        .ok_or(EditError::NoStatRowTemplate { target })?;
    let mut row = template.clone();
    let index_value = row
        .get_mut_by_name(SAVEGAME_STATPROPERTY_INDEX_NAME)
        .ok_or_else(|| EditError::MissingField {
            path: "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_INDEX".to_string(),
        })?;
    set_numeric_value(
        index_value,
        stat_id,
        "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_INDEX",
    )?;
    let base_value = row
        .get_mut_by_name(SAVEGAME_STATPROPERTY_BASE_NAME)
        .ok_or_else(|| EditError::MissingField {
            path: "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_BASE".to_string(),
        })?;
    set_numeric_value(
        base_value,
        new_value,
        "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_BASE",
    )?;
    stat_list.push(Value::Struct(Box::new(row)));
    Ok(())
}

pub(super) fn set_numeric_value(
    value: &mut Value,
    new_value: u32,
    path: &str,
) -> Result<(), EditError> {
    numeric::set_u32_compatible(value, new_value).map_err(|err| numeric_write_error(err, path))
}

pub(super) fn set_signed_numeric_value(
    value: &mut Value,
    new_value: i32,
    path: &str,
) -> Result<(), EditError> {
    numeric::set_i32_compatible(value, new_value).map_err(|err| numeric_write_error(err, path))
}

pub(super) fn set_float_value(
    value: &mut Value,
    new_value: f32,
    path: &str,
) -> Result<(), EditError> {
    numeric::set_f32_compatible(value, new_value).map_err(|err| numeric_write_error(err, path))
}

pub(super) fn set_property_power_value(
    value: &mut Value,
    new_value: f32,
    preferred_game: Option<GameId>,
    path: &str,
) -> Result<(), EditError> {
    match preferred_game.property_power_encoding() {
        PropertyPowerEncoding::Float => set_float_value(value, new_value, path),
        PropertyPowerEncoding::Da2Bitcast => numeric::set_da2_property_power(value, new_value)
            .map_err(|err| numeric_write_error(err, path)),
    }
}

fn numeric_write_error(err: NumericWriteError, path: &str) -> EditError {
    match err {
        NumericWriteError::Unsupported { actual } => EditError::UnsupportedNumericValue {
            path: path.to_string(),
            actual,
        },
        NumericWriteError::Range { detail } => EditError::NumericRange {
            path: path.to_string(),
            detail,
        },
    }
}

pub(super) fn value_to_u32(value: &Value) -> Option<u32> {
    value.to_u32_compatible()
}

pub(super) fn value_to_u16(value: &Value) -> Option<u16> {
    value.to_u16_compatible()
}

pub(super) fn value_to_i32(value: &Value) -> Option<i32> {
    value.to_i32_compatible()
}

pub(super) fn clean_resref(value: &str) -> String {
    value.trim_end_matches('\0').trim().to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::{
        EditError, FieldValue, GameId, GffStruct, ITEM_PROPERTIES_NAME, ITEM_PROPERTY_POWERS_NAME,
        InventoryContainer, ItemProperties, Value,
    };
    use crate::gff4::fields::field_id_by_name;

    fn field(name: &str, value: Value) -> FieldValue {
        FieldValue {
            label: field_id_by_name(name).unwrap(),
            value,
        }
    }

    fn list_values<'a>(item: &'a GffStruct, name: &str) -> &'a [Value] {
        item.get_list_by_name(name).unwrap()
    }

    #[test]
    fn item_properties_reads_arrays_in_either_field_order() {
        let mut item = GffStruct {
            struct_index: 0,
            fields: vec![
                field(
                    ITEM_PROPERTY_POWERS_NAME,
                    Value::List(vec![Value::Float32(1.0)]),
                ),
                field(ITEM_PROPERTIES_NAME, Value::List(vec![Value::UInt32(3011)])),
            ],
        };

        let properties = ItemProperties::from_item(
            &mut item,
            InventoryContainer::Backpack,
            0,
            Some(GameId::Dao),
        )
        .unwrap();

        assert_eq!(properties.len(), 1);
    }

    #[test]
    fn item_properties_rejects_missing_or_mismatched_parallel_arrays() {
        let mut missing_side = GffStruct {
            struct_index: 0,
            fields: vec![field(
                ITEM_PROPERTIES_NAME,
                Value::List(vec![Value::UInt32(3011)]),
            )],
        };
        assert!(matches!(
            ItemProperties::from_item(
                &mut missing_side,
                InventoryContainer::Backpack,
                0,
                Some(GameId::Dao)
            ),
            Err(EditError::InvalidPropertyArrayParity { .. })
        ));

        let mut mismatched = GffStruct {
            struct_index: 0,
            fields: vec![
                field(
                    ITEM_PROPERTIES_NAME,
                    Value::List(vec![Value::UInt32(3011), Value::UInt32(3012)]),
                ),
                field(
                    ITEM_PROPERTY_POWERS_NAME,
                    Value::List(vec![Value::Float32(1.0)]),
                ),
            ],
        };
        assert!(matches!(
            ItemProperties::from_item(
                &mut mismatched,
                InventoryContainer::Backpack,
                0,
                Some(GameId::Dao)
            ),
            Err(EditError::InvalidPropertyArrayParity { .. })
        ));
    }

    #[test]
    fn item_properties_creates_absent_arrays_and_pushes_dao_values() {
        let mut item = GffStruct {
            struct_index: 0,
            fields: Vec::new(),
        };
        {
            let mut properties = ItemProperties::from_item_or_create(
                &mut item,
                InventoryContainer::Backpack,
                0,
                Some(GameId::Dao),
            )
            .unwrap();
            properties.push(3011, 12.5).unwrap();
        }

        assert_eq!(
            list_values(&item, ITEM_PROPERTIES_NAME),
            &[Value::UInt32(3011)]
        );
        assert_eq!(
            list_values(&item, ITEM_PROPERTY_POWERS_NAME),
            &[Value::Float32(12.5)]
        );
    }

    #[test]
    fn item_properties_pushes_da2_property_bits() {
        let mut item = GffStruct {
            struct_index: 0,
            fields: Vec::new(),
        };
        {
            let mut properties = ItemProperties::from_item_or_create(
                &mut item,
                InventoryContainer::Backpack,
                0,
                Some(GameId::Da2),
            )
            .unwrap();
            properties.push(1000, 1.0).unwrap();
        }

        assert_eq!(
            list_values(&item, ITEM_PROPERTIES_NAME),
            &[Value::Float32(1000.0)]
        );
        assert_eq!(
            list_values(&item, ITEM_PROPERTY_POWERS_NAME),
            &[Value::UInt32(1.0f32.to_bits())]
        );
    }

    #[test]
    fn item_properties_setters_and_remove_keep_arrays_in_lockstep() {
        let mut item = GffStruct {
            struct_index: 0,
            fields: vec![
                field(
                    ITEM_PROPERTIES_NAME,
                    Value::List(vec![Value::UInt32(3011), Value::UInt32(3012)]),
                ),
                field(
                    ITEM_PROPERTY_POWERS_NAME,
                    Value::List(vec![Value::Float32(1.0), Value::Float32(2.0)]),
                ),
            ],
        };
        {
            let mut properties = ItemProperties::from_item(
                &mut item,
                InventoryContainer::Backpack,
                0,
                Some(GameId::Dao),
            )
            .unwrap();
            properties.set_id(1, 3013).unwrap();
            properties.set_power(1, 3.5).unwrap();
            properties.remove(0).unwrap();
        }

        assert_eq!(
            list_values(&item, ITEM_PROPERTIES_NAME),
            &[Value::UInt32(3013)]
        );
        assert_eq!(
            list_values(&item, ITEM_PROPERTY_POWERS_NAME),
            &[Value::Float32(3.5)]
        );
    }
}
