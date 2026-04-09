use crate::domain::ability::{AbilityKind, AbilityRef};
use crate::domain::character::Character;
use crate::domain::gamedata::{GameDataLookup, GameId, LookupError};
use crate::domain::item::{Item, ItemProperty, MaterialProfile};
use crate::domain::stats::{CoreStats, PointPools};
use crate::gff4::fields::{
    ITEM_COST, ITEM_STACKSIZE, OBJECT_ID, SAVEGAME_BACKPACK, SAVEGAME_CREATURE_STATS,
    SAVEGAME_EQUIPMENT_ITEMS, SAVEGAME_ITEM_MATERIALTYPE, SAVEGAME_MONEY, SAVEGAME_OBJECT_NAME,
    SAVEGAME_OBJECT_PLOT, SAVEGAME_PARTYLIST, SAVEGAME_PARTYPOOLMEMBERS, SAVEGAME_SKILLLIST,
    SAVEGAME_SPELLLIST, SAVEGAME_STATLIST, SAVEGAME_TALENTLIST, TEMPLATERESREF,
    field_id_by_name,
};
use crate::gff4::{GffFile, GffStruct, Value};
use std::error::Error;
use std::fmt;

const ITEM_PROPERTIES_NAME: &str = "ITEM_PROPERTIES";
const ITEM_PROPERTY_POWERS_NAME: &str = "ITEM_PROPERTY_POWERS";
const SAVEGAME_EQUIPMENTSET_SLOT_NAME: &str = "SAVEGAME_EQUIPMENTSET_SLOT";
const SAVEGAME_PLAYERCHAR_NAME: &str = "SAVEGAME_PLAYERCHAR";
const SAVEGAME_PLAYERCHAR_CHAR_NAME: &str = "SAVEGAME_PLAYERCHAR_CHAR";
const SAVEGAME_STATPROPERTY_INDEX_NAME: &str = "SAVEGAME_STATPROPERTY_INDEX";
const SAVEGAME_STATPROPERTY_BASE_NAME: &str = "SAVEGAME_STATPROPERTY_BASE";
const SAVEGAME_PARTY_APPROVAL_LIST_NAME: &str = "SAVEGAME_PARTY_APPROVAL_LIST";
const SAVEGAME_PARTY_APPROVAL_LEVEL_NAME: &str = "SAVEGAME_PARTY_APPROVAL_LEVEL";
const SAVEGAME_ABILITYLIST_NAME: &str = "SAVEGAME_ABILITYLIST";

#[derive(Debug, Clone, PartialEq)]
pub struct SaveGame {
    pub preferred_game: Option<GameId>,
    pub money: u32,
    pub main_character: Character,
    pub companions: Vec<Character>,
    pub backpack: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtractError {
    MissingField {
        path: String,
    },
    TypeMismatch {
        path: String,
        expected: &'static str,
        actual: &'static str,
    },
    InvalidValue {
        path: String,
        detail: String,
    },
    Lookup {
        path: String,
        detail: String,
    },
}

impl fmt::Display for ExtractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtractError::MissingField { path } => write!(f, "missing field at {path}"),
            ExtractError::TypeMismatch {
                path,
                expected,
                actual,
            } => write!(
                f,
                "type mismatch at {path}: expected {expected}, found {actual}"
            ),
            ExtractError::InvalidValue { path, detail } => {
                write!(f, "invalid value at {path}: {detail}")
            }
            ExtractError::Lookup { path, detail } => {
                write!(f, "lookup failed at {path}: {detail}")
            }
        }
    }
}

impl Error for ExtractError {}

impl SaveGame {
    pub fn from_gff(file: &GffFile) -> Result<Self, ExtractError> {
        Self::from_gff_with_lookup(file, None, None)
    }

    pub fn from_gff_with_lookup(
        file: &GffFile,
        lookup: Option<&dyn GameDataLookup>,
        preferred_game: Option<GameId>,
    ) -> Result<Self, ExtractError> {
        let preferred_game = preferred_game.or_else(|| infer_game(file));
        let root = &file.root;
        let party = require_struct(root, SAVEGAME_PARTYLIST, "root.SAVEGAME_PARTYLIST")?;
        let player =
            require_struct_by_name(root, SAVEGAME_PLAYERCHAR_NAME, "root.SAVEGAME_PLAYERCHAR")?;
        let player_char = require_struct_by_name(
            player,
            SAVEGAME_PLAYERCHAR_CHAR_NAME,
            "root.SAVEGAME_PLAYERCHAR.SAVEGAME_PLAYERCHAR_CHAR",
        )?;

        let money = require_u32(
            party,
            SAVEGAME_MONEY,
            "root.SAVEGAME_PARTYLIST.SAVEGAME_MONEY",
        )?;

        let main_character = extract_character(player_char, true, None, lookup, preferred_game)?;
        let approvals = extract_companion_approvals(party)?;
        let companions = extract_character_list(
            party,
            SAVEGAME_PARTYPOOLMEMBERS,
            "root.SAVEGAME_PARTYLIST.SAVEGAME_PARTYPOOLMEMBERS",
            false,
            Some(&approvals),
            lookup,
            preferred_game,
        )?;
        let backpack = extract_item_list(
            party,
            SAVEGAME_BACKPACK,
            "root.SAVEGAME_PARTYLIST.SAVEGAME_BACKPACK",
            lookup,
            preferred_game,
        )?;

        Ok(Self {
            preferred_game,
            money,
            main_character,
            companions,
            backpack,
        })
    }
}

fn extract_character_list(
    source: &GffStruct,
    label: u32,
    path: &str,
    main_character: bool,
    approvals: Option<&[Option<i32>]>,
    lookup: Option<&dyn GameDataLookup>,
    preferred_game: Option<GameId>,
) -> Result<Vec<Character>, ExtractError> {
    let values = require_list(source, label, path)?;
    let mut result = Vec::with_capacity(values.len());

    for (index, value) in values.iter().enumerate() {
        match value {
            Value::Struct(structure) => result.push(extract_character(
                structure,
                main_character && index == 0,
                approvals.and_then(|values| values.get(index).copied().flatten()),
                lookup,
                preferred_game,
            )?),
            Value::Null => {}
            other => {
                return Err(ExtractError::TypeMismatch {
                    path: format!("{path}[{index}]"),
                    expected: "Struct",
                    actual: other.type_name(),
                });
            }
        }
    }

    Ok(result)
}

fn extract_character(
    source: &GffStruct,
    main_character: bool,
    approval: Option<i32>,
    lookup: Option<&dyn GameDataLookup>,
    preferred_game: Option<GameId>,
) -> Result<Character, ExtractError> {
    let name = extract_character_name(source, main_character, lookup, preferred_game)?;
    let template_resref = optional_string(source, TEMPLATERESREF);

    let stats_source = require_struct(
        source,
        SAVEGAME_CREATURE_STATS,
        "character.SAVEGAME_CREATURE_STATS",
    )?;
    let equipment = extract_item_list(
        source,
        SAVEGAME_EQUIPMENT_ITEMS,
        "character.SAVEGAME_EQUIPMENT_ITEMS",
        lookup,
        preferred_game,
    )
    .unwrap_or_default();

    let (core_stats, level, point_pools) = extract_stats(stats_source)?;
    let (skills, talents, spells) = extract_character_abilities(stats_source, lookup, preferred_game)?;

    Ok(Character {
        name,
        template_resref,
        approval,
        level,
        core_stats,
        point_pools,
        equipment,
        skills,
        talents,
        spells,
    })
}

fn extract_character_name(
    source: &GffStruct,
    main_character: bool,
    lookup: Option<&dyn GameDataLookup>,
    preferred_game: Option<GameId>,
) -> Result<String, ExtractError> {
    if !main_character {
        if let Some(resref) = optional_string(source, TEMPLATERESREF) {
            if let Some(lookup) = lookup {
                if let Some(name) = map_lookup_error(
                    lookup.item_name(&resref, preferred_game),
                    "character.TEMPLATERESREF",
                )? {
                    return Ok(name);
                }
            }
        }
    }

    if let Some(value) = source.get(SAVEGAME_OBJECT_NAME) {
        if let Some(name) = value_to_text(value) {
            return Ok(clean_string(name));
        }
    }

    if let Some(resref) = optional_string(source, TEMPLATERESREF) {
        return Ok(clean_string(resref).to_ascii_lowercase());
    }

    Ok("<unknown>".to_string())
}

fn extract_stats(source: &GffStruct) -> Result<(CoreStats, Option<u32>, PointPools), ExtractError> {
    let mut core_stats = CoreStats::default();
    let mut level = None;
    let mut point_pools = PointPools::default();

    let stat_list = require_list(source, SAVEGAME_STATLIST, "character.SAVEGAME_STATLIST")?;

    for (index, value) in stat_list.iter().enumerate() {
        let stat = match value {
            Value::Struct(stat) => stat,
            Value::Null => continue,
            other => {
                return Err(ExtractError::TypeMismatch {
                    path: format!("character.SAVEGAME_STATLIST[{index}]"),
                    expected: "Struct",
                    actual: other.type_name(),
                });
            }
        };

        let stat_id = require_u32_by_name(
            stat,
            SAVEGAME_STATPROPERTY_INDEX_NAME,
            "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_INDEX",
        )?;
        if let Some(core_stat) = core_stat_from_id(stat_id) {
            let base = require_u32_by_name(
                stat,
                SAVEGAME_STATPROPERTY_BASE_NAME,
                "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_BASE",
            )?;
            core_stats.set(core_stat, base);
        } else if stat_id == 15 {
            let base = require_u32_by_name(
                stat,
                SAVEGAME_STATPROPERTY_BASE_NAME,
                "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_BASE",
            )?;
            level = Some(base);
        } else if stat_id == 34 {
            point_pools.attribute_points = Some(require_u32_by_name(
                stat,
                SAVEGAME_STATPROPERTY_BASE_NAME,
                "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_BASE",
            )?);
        } else if stat_id == 35 {
            point_pools.skill_points = Some(require_u32_by_name(
                stat,
                SAVEGAME_STATPROPERTY_BASE_NAME,
                "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_BASE",
            )?);
        } else if stat_id == 36 {
            point_pools.talent_points = Some(require_u32_by_name(
                stat,
                SAVEGAME_STATPROPERTY_BASE_NAME,
                "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_BASE",
            )?);
        } else if stat_id == 38 {
            point_pools.specialization_points = Some(require_u32_by_name(
                stat,
                SAVEGAME_STATPROPERTY_BASE_NAME,
                "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_BASE",
            )?);
        }
    }

    Ok((core_stats, level, point_pools))
}

fn extract_companion_approvals(source: &GffStruct) -> Result<Vec<Option<i32>>, ExtractError> {
    let Some(values) = source.get_by_name(SAVEGAME_PARTY_APPROVAL_LIST_NAME) else {
        return Ok(Vec::new());
    };
    let Value::List(entries) = values else {
        return Err(ExtractError::TypeMismatch {
            path: "root.SAVEGAME_PARTYLIST.SAVEGAME_PARTY_APPROVAL_LIST".to_string(),
            expected: "List",
            actual: values.type_name(),
        });
    };

    let mut approvals = Vec::with_capacity(entries.len());
    for (index, value) in entries.iter().enumerate() {
        match value {
            Value::Struct(structure) => approvals.push(Some(require_i32_by_name(
                structure,
                SAVEGAME_PARTY_APPROVAL_LEVEL_NAME,
                &format!(
                    "root.SAVEGAME_PARTYLIST.SAVEGAME_PARTY_APPROVAL_LIST[{index}].SAVEGAME_PARTY_APPROVAL_LEVEL"
                ),
            )?)),
            Value::Null => approvals.push(None),
            other => {
                return Err(ExtractError::TypeMismatch {
                    path: format!("root.SAVEGAME_PARTYLIST.SAVEGAME_PARTY_APPROVAL_LIST[{index}]"),
                    expected: "Struct",
                    actual: other.type_name(),
                });
            }
        }
    }
    Ok(approvals)
}

fn extract_item_list(
    source: &GffStruct,
    label: u32,
    path: &str,
    lookup: Option<&dyn GameDataLookup>,
    preferred_game: Option<GameId>,
) -> Result<Vec<Item>, ExtractError> {
    let values = require_list(source, label, path)?;
    let mut result = Vec::with_capacity(values.len());

    for (index, value) in values.iter().enumerate() {
        match value {
            Value::Struct(structure) => {
                result.push(extract_item(structure, lookup, preferred_game)?)
            }
            Value::Null => {}
            other => {
                return Err(ExtractError::TypeMismatch {
                    path: format!("{path}[{index}]"),
                    expected: "Struct",
                    actual: other.type_name(),
                });
            }
        }
    }

    Ok(result)
}

fn extract_item(
    source: &GffStruct,
    lookup: Option<&dyn GameDataLookup>,
    preferred_game: Option<GameId>,
) -> Result<Item, ExtractError> {
    let property_ids = optional_typed_list_by_name(
        source,
        ITEM_PROPERTIES_NAME,
        "item.ITEM_PROPERTIES",
    )?
        .map(extract_u32_values)
        .unwrap_or_default();
    let property_powers = optional_typed_list_by_name(
        source,
        ITEM_PROPERTY_POWERS_NAME,
        "item.ITEM_PROPERTY_POWERS",
    )?
        .map(extract_f32_values)
        .unwrap_or_default();

    if property_ids.len() != property_powers.len() {
        return Err(ExtractError::InvalidValue {
            path: "item".to_string(),
            detail: format!(
                "ITEM_PROPERTIES has {} entries but ITEM_PROPERTY_POWERS has {}",
                property_ids.len(),
                property_powers.len()
            ),
        });
    }

    let mut properties = Vec::new();
    for (id, power) in property_ids.into_iter().zip(property_powers.into_iter()) {
        let name = if let Some(lookup) = lookup {
            map_lookup_error(lookup.item_property_name(id), "item.ITEM_PROPERTIES")?
        } else {
            None
        };
        properties.push(ItemProperty { id, name, power });
    }

    let resref = optional_string(source, TEMPLATERESREF);
    let object_name = source.get(SAVEGAME_OBJECT_NAME).and_then(value_to_text);
    let name = if let Some(display_name) = object_name {
        Some(clean_string(display_name))
    } else if let (Some(lookup), Some(resref)) = (lookup, resref.as_deref()) {
        let resolved = map_lookup_error(
            lookup.item_name(resref, preferred_game),
            "item.TEMPLATERESREF",
        )?;
        resolved.or_else(|| Some(clean_string(resref.to_string()).to_ascii_lowercase()))
    } else if let Some(resref) = resref.as_deref() {
        Some(clean_string(resref.to_string()).to_ascii_lowercase())
    } else {
        None
    };

    let material = optional_u32(source, SAVEGAME_ITEM_MATERIALTYPE);
    let material_profile = if let (Some(lookup), Some(resref)) = (lookup, resref.as_deref()) {
        map_lookup_error(
            lookup.item_material_profile(resref, preferred_game),
            "item.TEMPLATERESREF",
        )?
    } else {
        None
    };
    let material_info = if let (Some(lookup), Some(material)) = (lookup, material) {
        map_lookup_error(
            lookup.material_info(material, preferred_game),
            "item.SAVEGAME_ITEM_MATERIALTYPE",
        )?
    } else {
        None
    };
    let material_profile = material_profile.or_else(|| {
        material_info.as_ref().map(|info| MaterialProfile {
            family: info.family,
            target: info.target,
        })
    });

    Ok(Item {
        resref,
        name,
        object_id: optional_i32(source, OBJECT_ID),
        equipment_slot: optional_u32_by_name(source, SAVEGAME_EQUIPMENTSET_SLOT_NAME),
        item_cost: optional_u32(source, ITEM_COST),
        item_stacksize: optional_u32(source, ITEM_STACKSIZE),
        item_level: optional_u8(source, SAVEGAME_OBJECT_PLOT),
        material,
        material_profile,
        material_info,
        properties,
    })
}

fn extract_abilities(
    source: &GffStruct,
    label: u32,
    path: &str,
    lookup: Option<&dyn GameDataLookup>,
    preferred_game: Option<GameId>,
) -> Result<Vec<AbilityRef>, ExtractError> {
    let ids = extract_u32_list(source, label);
    hydrate_abilities(ids, path, lookup, preferred_game)
}

fn require_struct<'a>(
    source: &'a GffStruct,
    label: u32,
    path: &str,
) -> Result<&'a GffStruct, ExtractError> {
    let value = source
        .get(label)
        .ok_or_else(|| ExtractError::MissingField {
            path: path.to_string(),
        })?;
    require_struct_value(value, path)
}

fn require_struct_by_name<'a>(
    source: &'a GffStruct,
    name: &str,
    path: &str,
) -> Result<&'a GffStruct, ExtractError> {
    let label = field_id_by_name(name).ok_or_else(|| ExtractError::InvalidValue {
        path: path.to_string(),
        detail: format!("unknown field name {name}"),
    })?;
    require_struct(source, label, path)
}

fn require_struct_value<'a>(value: &'a Value, path: &str) -> Result<&'a GffStruct, ExtractError> {
    match value {
        Value::Struct(structure) => Ok(structure),
        other => Err(ExtractError::TypeMismatch {
            path: path.to_string(),
            expected: "Struct",
            actual: other.type_name(),
        }),
    }
}

fn require_list<'a>(
    source: &'a GffStruct,
    label: u32,
    path: &str,
) -> Result<&'a [Value], ExtractError> {
    let value = source
        .get(label)
        .ok_or_else(|| ExtractError::MissingField {
            path: path.to_string(),
        })?;
    match value {
        Value::List(items) => Ok(items),
        other => Err(ExtractError::TypeMismatch {
            path: path.to_string(),
            expected: "List",
            actual: other.type_name(),
        }),
    }
}

fn optional_typed_list_by_name<'a>(
    source: &'a GffStruct,
    name: &str,
    path: &str,
) -> Result<Option<&'a [Value]>, ExtractError> {
    let Some(value) = source.get_by_name(name) else {
        return Ok(None);
    };
    match value {
        Value::List(items) => Ok(Some(items.as_slice())),
        other => Err(ExtractError::TypeMismatch {
            path: path.to_string(),
            expected: "List",
            actual: other.type_name(),
        }),
    }
}

fn require_u32(source: &GffStruct, label: u32, path: &str) -> Result<u32, ExtractError> {
    let value = source
        .get(label)
        .ok_or_else(|| ExtractError::MissingField {
            path: path.to_string(),
        })?;
    value_to_u32(value).ok_or_else(|| ExtractError::TypeMismatch {
        path: path.to_string(),
        expected: "UInt32-compatible number",
        actual: value.type_name(),
    })
}

fn require_u32_by_name(source: &GffStruct, name: &str, path: &str) -> Result<u32, ExtractError> {
    let value = source
        .get_by_name(name)
        .ok_or_else(|| ExtractError::MissingField {
            path: path.to_string(),
        })?;
    value_to_u32(value).ok_or_else(|| ExtractError::TypeMismatch {
        path: path.to_string(),
        expected: "UInt32-compatible number",
        actual: value.type_name(),
    })
}

fn require_i32_by_name(source: &GffStruct, name: &str, path: &str) -> Result<i32, ExtractError> {
    let value = source
        .get_by_name(name)
        .ok_or_else(|| ExtractError::MissingField {
            path: path.to_string(),
        })?;
    value_to_i32(value).ok_or_else(|| ExtractError::TypeMismatch {
        path: path.to_string(),
        expected: "Int32-compatible number",
        actual: value.type_name(),
    })
}

fn optional_u32(source: &GffStruct, label: u32) -> Option<u32> {
    source.get(label).and_then(value_to_u32)
}

fn optional_u32_by_name(source: &GffStruct, name: &str) -> Option<u32> {
    source.get_by_name(name).and_then(value_to_u32)
}

fn optional_i32(source: &GffStruct, label: u32) -> Option<i32> {
    source.get(label).and_then(|value| match value {
        Value::Int32(v) => Some(*v),
        _ => None,
    })
}

fn optional_u8(source: &GffStruct, label: u32) -> Option<u8> {
    source.get(label).and_then(|value| match value {
        Value::UInt8(v) => Some(*v),
        _ => None,
    })
}

fn optional_string(source: &GffStruct, label: u32) -> Option<String> {
    source
        .get(label)
        .and_then(value_to_display_string)
        .map(clean_string)
}

fn extract_u32_list(source: &GffStruct, label: u32) -> Vec<u32> {
    source
        .get(label)
        .and_then(|value| match value {
            Value::List(items) => Some(extract_u32_values(items)),
            _ => None,
        })
        .unwrap_or_default()
}

fn extract_u32_values(items: &[Value]) -> Vec<u32> {
    items.iter().filter_map(value_to_u32).collect()
}

fn extract_f32_values(items: &[Value]) -> Vec<f32> {
    items.iter().filter_map(value_to_f32).collect()
}

fn value_to_u32(value: &Value) -> Option<u32> {
    match value {
        Value::UInt8(v) => Some(*v as u32),
        Value::UInt16(v) => Some(*v as u32),
        Value::UInt32(v) => Some(*v),
        Value::Int8(v) if *v >= 0 => Some(*v as u32),
        Value::Int16(v) if *v >= 0 => Some(*v as u32),
        Value::Int32(v) if *v >= 0 => Some(*v as u32),
        Value::Float32(v) if v.is_finite() && *v >= 0.0 => Some(*v as u32),
        Value::Float64(v) if v.is_finite() && *v >= 0.0 => Some(*v as u32),
        _ => None,
    }
}

fn value_to_f32(value: &Value) -> Option<f32> {
    match value {
        Value::Float32(v) => Some(*v),
        Value::Float64(v) => Some(*v as f32),
        Value::UInt8(v) => Some(*v as f32),
        Value::UInt16(v) => Some(*v as f32),
        Value::UInt32(v) => Some(*v as f32),
        Value::Int8(v) => Some(*v as f32),
        Value::Int16(v) => Some(*v as f32),
        Value::Int32(v) => Some(*v as f32),
        _ => None,
    }
}

fn value_to_i32(value: &Value) -> Option<i32> {
    match value {
        Value::UInt8(v) => Some(*v as i32),
        Value::UInt16(v) => Some(*v as i32),
        Value::UInt32(v) => i32::try_from(*v).ok(),
        Value::Int8(v) => Some(*v as i32),
        Value::Int16(v) => Some(*v as i32),
        Value::Int32(v) => Some(*v),
        Value::Float32(v) if v.is_finite() => Some(*v as i32),
        Value::Float64(v) if v.is_finite() => Some(*v as i32),
        _ => None,
    }
}

fn value_to_display_string(value: &Value) -> Option<String> {
    match value {
        Value::ECString(text) => Some(text.clone()),
        Value::TlkString {
            label: _,
            text: Some(text),
            ..
        } => Some(text.clone()),
        Value::TlkString {
            label, text: None, ..
        } => Some(format!("<tlk:{label}>")),
        _ => None,
    }
}

fn value_to_text(value: &Value) -> Option<String> {
    match value {
        Value::ECString(text) => Some(text.clone()),
        Value::TlkString { text: Some(text), .. } => Some(text.clone()),
        _ => None,
    }
}

fn map_lookup_error<T>(result: Result<T, LookupError>, path: &str) -> Result<T, ExtractError> {
    result.map_err(|err| ExtractError::Lookup {
        path: path.to_string(),
        detail: err.to_string(),
    })
}

fn infer_game(file: &GffFile) -> Option<GameId> {
    match &file.header.file_version {
        b"V1.1" => Some(GameId::Dao),
        b"V2.0" => Some(GameId::Da2),
        _ => None,
    }
}

fn clean_string(value: String) -> String {
    value.trim_end_matches('\0').to_string()
}

fn core_stat_from_id(stat_id: u32) -> Option<crate::domain::stats::CoreStat> {
    match stat_id {
        1 => Some(crate::domain::stats::CoreStat::Strength),
        2 => Some(crate::domain::stats::CoreStat::Dexterity),
        3 => Some(crate::domain::stats::CoreStat::Willpower),
        4 => Some(crate::domain::stats::CoreStat::Magic),
        5 => Some(crate::domain::stats::CoreStat::Cunning),
        6 => Some(crate::domain::stats::CoreStat::Constitution),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::SaveGame;
    use crate::domain::gamedata::{GameId, SqliteGameData, DEFAULT_GAME_DATA_PATH};
    use crate::gff4::GffFile;
    use crate::test_support::{da2_save_path, dao_save_path};

    #[test]
    fn extracts_dao_read_only_summary() {
        let gff = GffFile::from_path(dao_save_path()).unwrap();
        let save = SaveGame::from_gff(&gff).unwrap();

        assert!(!save.main_character.name.is_empty());
        assert!(!save.companions.is_empty());
        assert!(save.main_character.core_stats.strength > 0);
        assert!(save.main_character.level.is_some());
        assert!(save.companions[0].core_stats.magic > 0);
    }

    #[test]
    fn extracts_da2_read_only_summary() {
        let gff = GffFile::from_path(da2_save_path()).unwrap();
        let save = SaveGame::from_gff(&gff).unwrap();

        assert!(!save.main_character.name.is_empty());
        assert!(save.money <= u32::MAX);
        assert!(!save.companions.is_empty());
        assert!(save.main_character.core_stats.strength > 0);
        assert!(save.main_character.core_stats.dexterity > 0);
    }

    #[test]
    fn enriches_domain_with_db_lookups() {
        let gff = GffFile::from_path(dao_save_path()).unwrap();
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let save = SaveGame::from_gff_with_lookup(&gff, Some(&lookup), None).unwrap();

        assert!(!save.main_character.skills.is_empty());
        assert!(
            save.main_character
                .skills
                .iter()
                .any(|ability| ability.name.is_some())
        );
        assert_eq!(save.preferred_game, Some(GameId::Dao));
        assert!(save
            .companions
            .iter()
            .all(|character| !character.name.trim().is_empty()));
        assert!(
            save.backpack
                .iter()
                .any(|item| item.name.is_some())
        );
        assert!(
            save.backpack
                .iter()
                .any(|item| item
                    .properties
                    .iter()
                    .any(|property| property.name.is_some()))
        );
    }

    #[test]
    fn enriches_da2_names_with_inferred_game() {
        let gff = GffFile::from_path(da2_save_path()).unwrap();
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let save = SaveGame::from_gff_with_lookup(&gff, Some(&lookup), None).unwrap();

        assert_eq!(save.preferred_game, Some(GameId::Da2));
        assert!(save
            .companions
            .iter()
            .all(|character| !character.name.trim().is_empty()));
        assert!(
            save.backpack
                .iter()
                .any(|item| item.name.is_some())
        );
    }

    #[test]
    fn loads_existing_da2_abilities_from_combined_ability_list() {
        let gff = GffFile::from_path(da2_save_path()).unwrap();
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let save = SaveGame::from_gff_with_lookup(&gff, Some(&lookup), None).unwrap();

        assert!(!save.main_character.talents.is_empty());
        assert!(
            save.main_character
                .talents
                .iter()
                .any(|ability| ability.name.is_some())
        );
        assert!(
            save.main_character
                .talents
                .iter()
                .any(|ability| ability.id == 200000 || ability.id == 201000 || ability.id == 201001)
        );
    }
}
fn extract_character_abilities(
    source: &GffStruct,
    lookup: Option<&dyn GameDataLookup>,
    preferred_game: Option<GameId>,
) -> Result<(Vec<AbilityRef>, Vec<AbilityRef>, Vec<AbilityRef>), ExtractError> {
    if preferred_game == Some(GameId::Da2) && source.get_by_name(SAVEGAME_ABILITYLIST_NAME).is_some() {
        let mut skills = Vec::new();
        let mut talents = Vec::new();
        let mut spells = Vec::new();

        for ability in extract_abilities_by_name(
            source,
            SAVEGAME_ABILITYLIST_NAME,
            lookup,
            preferred_game,
        )? {
            match ability.kind {
                AbilityKind::Skill => skills.push(ability),
                AbilityKind::Spell => spells.push(ability),
                AbilityKind::Talent | AbilityKind::Unknown => talents.push(ability),
            }
        }

        Ok((skills, talents, spells))
    } else {
        Ok((
            extract_abilities(
                source,
                SAVEGAME_SKILLLIST,
                "character.SAVEGAME_CREATURE_STATS.SAVEGAME_SKILLLIST",
                lookup,
                preferred_game,
            )?,
            extract_abilities(
                source,
                SAVEGAME_TALENTLIST,
                "character.SAVEGAME_CREATURE_STATS.SAVEGAME_TALENTLIST",
                lookup,
                preferred_game,
            )?,
            extract_abilities(
                source,
                SAVEGAME_SPELLLIST,
                "character.SAVEGAME_CREATURE_STATS.SAVEGAME_SPELLLIST",
                lookup,
                preferred_game,
            )?,
        ))
    }
}

fn extract_abilities_by_name(
    source: &GffStruct,
    name: &str,
    lookup: Option<&dyn GameDataLookup>,
    preferred_game: Option<GameId>,
) -> Result<Vec<AbilityRef>, ExtractError> {
    let ids = source
        .get_by_name(name)
        .and_then(|value| match value {
            Value::List(items) => Some(extract_u32_values(items)),
            _ => None,
        })
        .unwrap_or_default();

    hydrate_abilities(
        ids,
        &format!("character.SAVEGAME_CREATURE_STATS.{name}"),
        lookup,
        preferred_game,
    )
}

fn hydrate_abilities(
    ids: Vec<u32>,
    path: &str,
    lookup: Option<&dyn GameDataLookup>,
    preferred_game: Option<GameId>,
) -> Result<Vec<AbilityRef>, ExtractError> {
    let mut abilities = Vec::with_capacity(ids.len());
    for id in ids {
        let ability = if let Some(lookup) = lookup {
            map_lookup_error(lookup.ability(id, preferred_game), path)?.unwrap_or(AbilityRef {
                id,
                name: None,
                tree: None,
                ability_type: None,
                kind: AbilityKind::Unknown,
                core_ids: Vec::new(),
            })
        } else {
            AbilityRef {
                id,
                name: None,
                tree: None,
                ability_type: None,
                kind: AbilityKind::Unknown,
                core_ids: Vec::new(),
            }
        };
        abilities.push(ability);
    }

    Ok(abilities)
}
