use crate::domain::ability::{AbilityKind, AbilityRef};
use crate::domain::character::Character;
use crate::domain::game::{AbilityListStyle, GameBehavior, PropertyPowerEncoding};
use crate::domain::gamedata::{GameDataLookup, GameId, LookupError};
use crate::domain::item::{Item, ItemCategory, ItemProperty, MaterialProfile};
use crate::domain::stats::{
    CoreStats, PointPoolKind, PointPools, core_stat_from_id, experience_stat_id, level_stat_id,
    point_pool_stat_id,
};
use crate::gff4::fields::{
    ITEM_COST, ITEM_STACKSIZE, OBJECT_ID, SAVEGAME_BACKPACK, SAVEGAME_CAMPAIGN,
    SAVEGAME_CRAFTING_RECIPE_LIST, SAVEGAME_CREATURE_STATS, SAVEGAME_EQUIPMENT_ITEMS,
    SAVEGAME_ITEM_LEVEL, SAVEGAME_ITEM_MATERIALTYPE, SAVEGAME_MONEY, SAVEGAME_OBJECT_NAME,
    SAVEGAME_PARTYLIST, SAVEGAME_PARTYPOOLMEMBERS, SAVEGAME_SKILLLIST, SAVEGAME_SPELLLIST,
    SAVEGAME_STATLIST, SAVEGAME_TALENTLIST, TEMPLATERESREF, field_id_by_name,
};
use crate::gff4::numeric;
use crate::gff4::{GffFile, GffStruct, Value};
use std::collections::BTreeMap;
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
const SAVEGAME_PARTY_APPROVAL_ID_NAME: &str = "SAVEGAME_PARTY_APPROVAL_ID";
const SAVEGAME_PARTY_APPROVAL_LEVEL_NAME: &str = "SAVEGAME_PARTY_APPROVAL_LEVEL";
const SAVEGAME_ABILITYLIST_NAME: &str = "SAVEGAME_ABILITYLIST";
pub const WORLD_VAULT_LABEL: u32 = 16024;
pub const WORLD_VAULT_ID_LABEL: u32 = 17601;
pub const WORLD_VAULT_VALUE_LABEL: u32 = 17602;
pub const WORLD_VAULT_INTS_LABEL: u32 = 17603;
pub const WORLD_VAULT_BOOLEANS_LABEL: u32 = 17607;

#[derive(Debug, Clone, PartialEq)]
pub struct SaveGame {
    pub preferred_game: Option<GameId>,
    pub money: u32,
    pub main_character: Character,
    pub companions: Vec<Character>,
    pub backpack: Vec<Item>,
    pub crafting_recipes: Vec<u32>,
    pub plot_flags: PlotFlags,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlotFlags {
    pub booleans: BTreeMap<u16, bool>,
    pub integers: BTreeMap<u16, i32>,
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
        let crafting_recipes = extract_u32_list(party, SAVEGAME_CRAFTING_RECIPE_LIST);
        let plot_flags = extract_plot_flags(root);

        Ok(Self {
            preferred_game,
            money,
            main_character,
            companions,
            backpack,
            crafting_recipes,
            plot_flags,
        })
    }
}

fn extract_character_list(
    source: &GffStruct,
    label: u32,
    path: &str,
    main_character: bool,
    approvals: Option<&BTreeMap<i32, i32>>,
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
                optional_i32(structure, OBJECT_ID).and_then(|object_id| {
                    approvals.and_then(|values| values.get(&object_id).copied())
                }),
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

    let (core_stats, level, experience, point_pools) = extract_stats(stats_source, preferred_game)?;
    let abilities = extract_character_abilities(stats_source, lookup, preferred_game)?;

    Ok(Character {
        name,
        template_resref,
        approval,
        level,
        experience,
        core_stats,
        point_pools,
        equipment,
        skills: abilities.skills,
        talents: abilities.talents,
        spells: abilities.spells,
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

fn extract_stats(
    source: &GffStruct,
    preferred_game: Option<GameId>,
) -> Result<(CoreStats, Option<u32>, Option<u32>, PointPools), ExtractError> {
    let mut core_stats = CoreStats::default();
    let mut level = None;
    let mut experience = None;
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
        } else if stat_id == level_stat_id(preferred_game) {
            let base = require_u32_by_name(
                stat,
                SAVEGAME_STATPROPERTY_BASE_NAME,
                "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_BASE",
            )?;
            level = Some(base);
        } else if stat_id == experience_stat_id(preferred_game) {
            let base = require_u32_by_name(
                stat,
                SAVEGAME_STATPROPERTY_BASE_NAME,
                "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_BASE",
            )?;
            experience = Some(base);
        } else if Some(stat_id) == point_pool_stat_id(PointPoolKind::Attribute, preferred_game) {
            point_pools.attribute_points = Some(require_u32_by_name(
                stat,
                SAVEGAME_STATPROPERTY_BASE_NAME,
                "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_BASE",
            )?);
        } else if Some(stat_id) == point_pool_stat_id(PointPoolKind::Skill, preferred_game) {
            point_pools.skill_points = Some(require_u32_by_name(
                stat,
                SAVEGAME_STATPROPERTY_BASE_NAME,
                "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_BASE",
            )?);
        } else if Some(stat_id) == point_pool_stat_id(PointPoolKind::Talent, preferred_game) {
            point_pools.talent_points = Some(require_u32_by_name(
                stat,
                SAVEGAME_STATPROPERTY_BASE_NAME,
                "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_BASE",
            )?);
        } else if Some(stat_id) == point_pool_stat_id(PointPoolKind::Specialization, preferred_game)
        {
            point_pools.specialization_points = Some(require_u32_by_name(
                stat,
                SAVEGAME_STATPROPERTY_BASE_NAME,
                "character.SAVEGAME_STATLIST[].SAVEGAME_STATPROPERTY_BASE",
            )?);
        }
    }

    Ok((core_stats, level, experience, point_pools))
}

fn extract_companion_approvals(source: &GffStruct) -> Result<BTreeMap<i32, i32>, ExtractError> {
    let Some(values) = source.get_by_name(SAVEGAME_PARTY_APPROVAL_LIST_NAME) else {
        return Ok(BTreeMap::new());
    };
    let Value::List(entries) = values else {
        return Err(ExtractError::TypeMismatch {
            path: "root.SAVEGAME_PARTYLIST.SAVEGAME_PARTY_APPROVAL_LIST".to_string(),
            expected: "List",
            actual: values.type_name(),
        });
    };

    let mut approvals = BTreeMap::new();
    for (index, value) in entries.iter().enumerate() {
        match value {
            Value::Struct(structure) => {
                let object_id = require_i32_by_name(
                    structure,
                    SAVEGAME_PARTY_APPROVAL_ID_NAME,
                    &format!(
                        "root.SAVEGAME_PARTYLIST.SAVEGAME_PARTY_APPROVAL_LIST[{index}].SAVEGAME_PARTY_APPROVAL_ID"
                    ),
                )?;
                let approval = require_i32_by_name(
                    structure,
                    SAVEGAME_PARTY_APPROVAL_LEVEL_NAME,
                    &format!(
                        "root.SAVEGAME_PARTYLIST.SAVEGAME_PARTY_APPROVAL_LIST[{index}].SAVEGAME_PARTY_APPROVAL_LEVEL"
                    ),
                )?;
                approvals.insert(object_id, approval);
            }
            Value::Null => {}
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
    let property_ids =
        optional_typed_list_by_name(source, ITEM_PROPERTIES_NAME, "item.ITEM_PROPERTIES")?
            .map(extract_u32_values)
            .unwrap_or_default();
    let property_powers = optional_typed_list_by_name(
        source,
        ITEM_PROPERTY_POWERS_NAME,
        "item.ITEM_PROPERTY_POWERS",
    )?
    .map(|values| extract_property_power_values(values, preferred_game))
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
            map_lookup_error(
                lookup.item_property_name(id, preferred_game),
                "item.ITEM_PROPERTIES",
            )?
        } else {
            None
        };
        properties.push(ItemProperty { id, name, power });
    }

    let resref = optional_string(source, TEMPLATERESREF);
    let object_name = source.get(SAVEGAME_OBJECT_NAME).and_then(value_to_text);
    let catalog_item = if let (Some(lookup), Some(resref)) = (lookup, resref.as_deref()) {
        map_lookup_error(
            lookup.item_metadata(resref, preferred_game),
            "item.TEMPLATERESREF",
        )?
    } else {
        None
    };
    let name = object_name
        .map(clean_string)
        .or_else(|| catalog_item.as_ref().and_then(|item| item.name.clone()))
        .or_else(|| {
            resref
                .as_deref()
                .map(|resref| clean_string(resref.to_string()).to_ascii_lowercase())
        });

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
        wiki_url: catalog_item.as_ref().and_then(|item| item.wiki_url.clone()),
        category: catalog_item
            .as_ref()
            .map(|item| item.category)
            .unwrap_or(ItemCategory::Uncategorized),
        stackable: catalog_item
            .as_ref()
            .map(|item| item.stackable)
            .unwrap_or(false),
        object_id: optional_i32(source, OBJECT_ID),
        equipment_slot: optional_u32_by_name(source, SAVEGAME_EQUIPMENTSET_SLOT_NAME),
        item_cost: optional_u32(source, ITEM_COST),
        item_stacksize: optional_u32(source, ITEM_STACKSIZE),
        item_level: optional_i32(source, SAVEGAME_ITEM_LEVEL).and_then(|v| u8::try_from(v).ok()),
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
    numeric::to_u32_compatible(value).ok_or_else(|| ExtractError::TypeMismatch {
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
    numeric::to_u32_compatible(value).ok_or_else(|| ExtractError::TypeMismatch {
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
    numeric::to_i32_compatible(value).ok_or_else(|| ExtractError::TypeMismatch {
        path: path.to_string(),
        expected: "Int32-compatible number",
        actual: value.type_name(),
    })
}

fn optional_u32(source: &GffStruct, label: u32) -> Option<u32> {
    source.get(label).and_then(numeric::to_u32_compatible)
}

fn optional_u32_by_name(source: &GffStruct, name: &str) -> Option<u32> {
    source
        .get_by_name(name)
        .and_then(numeric::to_u32_compatible)
}

fn optional_i32(source: &GffStruct, label: u32) -> Option<i32> {
    source.get(label).and_then(|value| match value {
        Value::Int32(v) => Some(*v),
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
    items
        .iter()
        .filter_map(numeric::to_u32_compatible)
        .collect()
}

fn extract_plot_flags(root: &GffStruct) -> PlotFlags {
    let Some(world_vault) = root.get_struct(WORLD_VAULT_LABEL) else {
        return PlotFlags::default();
    };
    PlotFlags {
        booleans: extract_world_vault_bools(world_vault),
        integers: extract_world_vault_ints(world_vault),
    }
}

fn extract_world_vault_bools(world_vault: &GffStruct) -> BTreeMap<u16, bool> {
    let mut values = BTreeMap::new();
    let Some(entries) = world_vault.get_list(WORLD_VAULT_BOOLEANS_LABEL) else {
        return values;
    };
    for entry in entries.iter().filter_map(Value::as_struct) {
        let Some(id) = entry
            .get(WORLD_VAULT_ID_LABEL)
            .and_then(numeric::to_u16_compatible)
        else {
            continue;
        };
        let Some(value) = entry
            .get(WORLD_VAULT_VALUE_LABEL)
            .and_then(numeric::to_u32_compatible)
        else {
            continue;
        };
        values.insert(id, value != 0);
    }
    values
}

fn extract_world_vault_ints(world_vault: &GffStruct) -> BTreeMap<u16, i32> {
    let mut values = BTreeMap::new();
    let Some(entries) = world_vault.get_list(WORLD_VAULT_INTS_LABEL) else {
        return values;
    };
    for entry in entries.iter().filter_map(Value::as_struct) {
        let Some(id) = entry
            .get(WORLD_VAULT_ID_LABEL)
            .and_then(numeric::to_u16_compatible)
        else {
            continue;
        };
        let Some(value) = entry
            .get(WORLD_VAULT_VALUE_LABEL)
            .and_then(numeric::to_i32_compatible)
        else {
            continue;
        };
        values.insert(id, value);
    }
    values
}

fn extract_property_power_values(items: &[Value], preferred_game: Option<GameId>) -> Vec<f32> {
    match preferred_game.property_power_encoding() {
        PropertyPowerEncoding::Float => extract_f32_values(items),
        PropertyPowerEncoding::Da2Bitcast => items
            .iter()
            .filter_map(numeric::to_da2_property_power)
            .collect(),
    }
}

fn extract_f32_values(items: &[Value]) -> Vec<f32> {
    items
        .iter()
        .filter_map(numeric::to_f32_compatible)
        .collect()
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
        Value::TlkString {
            text: Some(text), ..
        } => Some(text.clone()),
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
        b"V1.1" => Some(infer_dao_campaign(file)),
        b"V2.0" => Some(GameId::Da2),
        _ => None,
    }
}

fn infer_dao_campaign(file: &GffFile) -> GameId {
    let campaign_resource = file
        .root
        .get_struct(SAVEGAME_CAMPAIGN)
        .and_then(|campaign| campaign.get(field_id_by_name("SAVEGAME_CAMPAIGN_RESOURCE")?))
        .and_then(value_to_display_string)
        .map(clean_string)
        .map(|value| value.to_ascii_uppercase());

    match campaign_resource.as_deref() {
        Some("DAO_PRC_EP_1" | "DAO_PRC_STR" | "DAO_PRC_GIB") => GameId::DaoAwakening,
        _ => GameId::Dao,
    }
}

fn clean_string(value: String) -> String {
    value.trim_end_matches('\0').to_string()
}

#[cfg(test)]
mod tests;

#[derive(Debug, Default)]
struct CharacterAbilities {
    skills: Vec<AbilityRef>,
    talents: Vec<AbilityRef>,
    spells: Vec<AbilityRef>,
}

fn extract_character_abilities(
    source: &GffStruct,
    lookup: Option<&dyn GameDataLookup>,
    preferred_game: Option<GameId>,
) -> Result<CharacterAbilities, ExtractError> {
    if preferred_game.ability_list_style() == AbilityListStyle::Combined
        && source.get_by_name(SAVEGAME_ABILITYLIST_NAME).is_some()
    {
        let mut abilities = CharacterAbilities::default();

        for ability in
            extract_abilities_by_name(source, SAVEGAME_ABILITYLIST_NAME, lookup, preferred_game)?
        {
            match ability.kind {
                AbilityKind::Skill => abilities.skills.push(ability),
                AbilityKind::Spell => abilities.spells.push(ability),
                AbilityKind::Talent | AbilityKind::Unknown => abilities.talents.push(ability),
            }
        }

        Ok(abilities)
    } else {
        Ok(CharacterAbilities {
            skills: extract_abilities(
                source,
                SAVEGAME_SKILLLIST,
                "character.SAVEGAME_CREATURE_STATS.SAVEGAME_SKILLLIST",
                lookup,
                preferred_game,
            )?,
            talents: extract_abilities(
                source,
                SAVEGAME_TALENTLIST,
                "character.SAVEGAME_CREATURE_STATS.SAVEGAME_TALENTLIST",
                lookup,
                preferred_game,
            )?,
            spells: extract_abilities(
                source,
                SAVEGAME_SPELLLIST,
                "character.SAVEGAME_CREATURE_STATS.SAVEGAME_SPELLLIST",
                lookup,
                preferred_game,
            )?,
        })
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
        if id == 0 {
            continue;
        }
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
