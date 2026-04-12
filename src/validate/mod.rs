use crate::gff4::fields::{
    SAVEGAME_BACKPACK, SAVEGAME_CREATURE_STATS, SAVEGAME_EQUIPMENT_ITEMS, SAVEGAME_MONEY,
    SAVEGAME_PARTYLIST, SAVEGAME_PARTYPOOLMEMBERS, SAVEGAME_SKILLLIST, SAVEGAME_SPELLLIST,
    SAVEGAME_STATLIST, SAVEGAME_TALENTLIST,
};
use crate::gff4::{GffFile, GffStruct, Value};
use serde::{Deserialize, Serialize};

const ITEM_PROPERTIES_NAME: &str = "ITEM_PROPERTIES";
const ITEM_PROPERTY_POWERS_NAME: &str = "ITEM_PROPERTY_POWERS";
const SAVEGAME_ABILITYLIST_NAME: &str = "SAVEGAME_ABILITYLIST";
const SAVEGAME_PLAYERCHAR_NAME: &str = "SAVEGAME_PLAYERCHAR";
const SAVEGAME_PLAYERCHAR_CHAR_NAME: &str = "SAVEGAME_PLAYERCHAR_CHAR";
const SAVEGAME_STATPROPERTY_INDEX_NAME: &str = "SAVEGAME_STATPROPERTY_INDEX";
const SAVEGAME_STATPROPERTY_BASE_NAME: &str = "SAVEGAME_STATPROPERTY_BASE";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AbilityStorageKind {
    DaoSplitLists,
    Da2CombinedList,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSeverity {
    Error,
    Warning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationCode {
    MissingField,
    TypeMismatch,
    InvalidNumericValue,
    InvalidListEntry,
    InvalidPropertyArrayParity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    pub severity: ValidationSeverity,
    pub code: ValidationCode,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationReport {
    pub findings: Vec<ValidationFinding>,
}

impl ValidationReport {
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == ValidationSeverity::Error)
    }
}

pub fn validate_gff(file: &GffFile) -> ValidationReport {
    let mut report = ValidationReport::default();
    let root = &file.root;
    let ability_storage = infer_ability_storage(file);

    let Some(party) = require_struct(
        root,
        SAVEGAME_PARTYLIST,
        "root.SAVEGAME_PARTYLIST",
        &mut report,
    ) else {
        return report;
    };
    let Some(player) = require_struct_by_name(
        root,
        SAVEGAME_PLAYERCHAR_NAME,
        "root.SAVEGAME_PLAYERCHAR",
        &mut report,
    ) else {
        return report;
    };
    let Some(player_char) = require_struct_by_name(
        player,
        SAVEGAME_PLAYERCHAR_CHAR_NAME,
        "root.SAVEGAME_PLAYERCHAR.SAVEGAME_PLAYERCHAR_CHAR",
        &mut report,
    ) else {
        return report;
    };

    require_numeric(
        party,
        SAVEGAME_MONEY,
        "root.SAVEGAME_PARTYLIST.SAVEGAME_MONEY",
        &mut report,
    );
    validate_character(
        player_char,
        "root.SAVEGAME_PLAYERCHAR.SAVEGAME_PLAYERCHAR_CHAR",
        ability_storage,
        &mut report,
    );
    validate_item_list(
        party,
        SAVEGAME_BACKPACK,
        "root.SAVEGAME_PARTYLIST.SAVEGAME_BACKPACK",
        &mut report,
    );

    if let Some(companions) = require_list(
        party,
        SAVEGAME_PARTYPOOLMEMBERS,
        "root.SAVEGAME_PARTYLIST.SAVEGAME_PARTYPOOLMEMBERS",
        &mut report,
    ) {
        for (index, value) in companions.iter().enumerate() {
            match value {
                Value::Struct(structure) => validate_character(
                    structure,
                    &format!("root.SAVEGAME_PARTYLIST.SAVEGAME_PARTYPOOLMEMBERS[{index}]"),
                    ability_storage,
                    &mut report,
                ),
                Value::Null => {}
                other => push(
                    &mut report,
                    ValidationCode::InvalidListEntry,
                    format!("root.SAVEGAME_PARTYLIST.SAVEGAME_PARTYPOOLMEMBERS[{index}]"),
                    format!("expected Struct or Null, found {}", other.type_name()),
                ),
            }
        }
    }

    report
}

fn validate_character(
    source: &GffStruct,
    path: &str,
    ability_storage: AbilityStorageKind,
    report: &mut ValidationReport,
) {
    validate_optional_item_list(
        source,
        SAVEGAME_EQUIPMENT_ITEMS,
        &format!("{path}.SAVEGAME_EQUIPMENT_ITEMS"),
        report,
    );

    let Some(stats) = require_struct(
        source,
        SAVEGAME_CREATURE_STATS,
        &format!("{path}.SAVEGAME_CREATURE_STATS"),
        report,
    ) else {
        return;
    };

    if let Some(stat_list) = require_list(
        stats,
        SAVEGAME_STATLIST,
        &format!("{path}.SAVEGAME_CREATURE_STATS.SAVEGAME_STATLIST"),
        report,
    ) {
        for (index, value) in stat_list.iter().enumerate() {
            match value {
                Value::Struct(structure) => {
                    require_numeric_by_name(
                        structure,
                        SAVEGAME_STATPROPERTY_INDEX_NAME,
                        &format!(
                            "{path}.SAVEGAME_CREATURE_STATS.SAVEGAME_STATLIST[{index}].SAVEGAME_STATPROPERTY_INDEX"
                        ),
                        report,
                    );
                    require_numeric_by_name(
                        structure,
                        SAVEGAME_STATPROPERTY_BASE_NAME,
                        &format!(
                            "{path}.SAVEGAME_CREATURE_STATS.SAVEGAME_STATLIST[{index}].SAVEGAME_STATPROPERTY_BASE"
                        ),
                        report,
                    );
                }
                Value::Null => {}
                other => push(
                    report,
                    ValidationCode::InvalidListEntry,
                    format!("{path}.SAVEGAME_CREATURE_STATS.SAVEGAME_STATLIST[{index}]"),
                    format!("expected Struct or Null, found {}", other.type_name()),
                ),
            }
        }
    }

    validate_ability_storage(stats, path, ability_storage, report);
}

fn validate_ability_storage(
    stats: &GffStruct,
    path: &str,
    ability_storage: AbilityStorageKind,
    report: &mut ValidationReport,
) {
    match ability_storage {
        AbilityStorageKind::DaoSplitLists => {
            validate_optional_numeric_list(
                stats,
                SAVEGAME_SKILLLIST,
                &format!("{path}.SAVEGAME_CREATURE_STATS.SAVEGAME_SKILLLIST"),
                report,
            );
            validate_optional_numeric_list(
                stats,
                SAVEGAME_TALENTLIST,
                &format!("{path}.SAVEGAME_CREATURE_STATS.SAVEGAME_TALENTLIST"),
                report,
            );
            validate_optional_numeric_list(
                stats,
                SAVEGAME_SPELLLIST,
                &format!("{path}.SAVEGAME_CREATURE_STATS.SAVEGAME_SPELLLIST"),
                report,
            );
        }
        AbilityStorageKind::Da2CombinedList => {
            validate_optional_numeric_list_by_name(
                stats,
                SAVEGAME_ABILITYLIST_NAME,
                &format!("{path}.SAVEGAME_CREATURE_STATS.SAVEGAME_ABILITYLIST"),
                report,
            );
        }
    }
}

fn validate_item_list(source: &GffStruct, label: u32, path: &str, report: &mut ValidationReport) {
    if let Some(items) = require_list(source, label, path, report) {
        for (index, value) in items.iter().enumerate() {
            match value {
                Value::Struct(structure) => {
                    validate_item(structure, &format!("{path}[{index}]"), report)
                }
                Value::Null => {}
                other => push(
                    report,
                    ValidationCode::InvalidListEntry,
                    format!("{path}[{index}]"),
                    format!("expected Struct or Null, found {}", other.type_name()),
                ),
            }
        }
    }
}

fn validate_optional_item_list(
    source: &GffStruct,
    label: u32,
    path: &str,
    report: &mut ValidationReport,
) {
    let Some(value) = source.get(label) else {
        return;
    };
    match value {
        Value::List(items) => {
            for (index, value) in items.iter().enumerate() {
                match value {
                    Value::Struct(structure) => {
                        validate_item(structure, &format!("{path}[{index}]"), report)
                    }
                    Value::Null => {}
                    other => push(
                        report,
                        ValidationCode::InvalidListEntry,
                        format!("{path}[{index}]"),
                        format!("expected Struct or Null, found {}", other.type_name()),
                    ),
                }
            }
        }
        other => push(
            report,
            ValidationCode::TypeMismatch,
            path.to_string(),
            format!("expected List, found {}", other.type_name()),
        ),
    }
}

fn validate_item(source: &GffStruct, path: &str, report: &mut ValidationReport) {
    let property_ids = optional_list_by_name(
        source,
        ITEM_PROPERTIES_NAME,
        &format!("{path}.ITEM_PROPERTIES"),
        report,
    );
    let property_powers = optional_list_by_name(
        source,
        ITEM_PROPERTY_POWERS_NAME,
        &format!("{path}.ITEM_PROPERTY_POWERS"),
        report,
    );

    match (property_ids, property_powers) {
        (None, None) => {}
        (Some(ids), Some(powers)) => {
            if ids.len() != powers.len() {
                push(
                    report,
                    ValidationCode::InvalidPropertyArrayParity,
                    path.to_string(),
                    format!(
                        "ITEM_PROPERTIES has {} entries but ITEM_PROPERTY_POWERS has {}",
                        ids.len(),
                        powers.len()
                    ),
                );
            }
        }
        _ => push(
            report,
            ValidationCode::InvalidPropertyArrayParity,
            path.to_string(),
            "ITEM_PROPERTIES and ITEM_PROPERTY_POWERS must both exist or both be absent"
                .to_string(),
        ),
    }
}

fn require_struct<'a>(
    source: &'a GffStruct,
    label: u32,
    path: &str,
    report: &mut ValidationReport,
) -> Option<&'a GffStruct> {
    let value = source.get(label).or_else(|| {
        push(
            report,
            ValidationCode::MissingField,
            path.to_string(),
            "missing field".to_string(),
        );
        None
    })?;
    match value {
        Value::Struct(structure) => Some(structure),
        other => {
            push(
                report,
                ValidationCode::TypeMismatch,
                path.to_string(),
                format!("expected Struct, found {}", other.type_name()),
            );
            None
        }
    }
}

fn require_struct_by_name<'a>(
    source: &'a GffStruct,
    name: &str,
    path: &str,
    report: &mut ValidationReport,
) -> Option<&'a GffStruct> {
    let value = source.get_by_name(name).or_else(|| {
        push(
            report,
            ValidationCode::MissingField,
            path.to_string(),
            "missing field".to_string(),
        );
        None
    })?;
    match value {
        Value::Struct(structure) => Some(structure),
        other => {
            push(
                report,
                ValidationCode::TypeMismatch,
                path.to_string(),
                format!("expected Struct, found {}", other.type_name()),
            );
            None
        }
    }
}

fn require_list<'a>(
    source: &'a GffStruct,
    label: u32,
    path: &str,
    report: &mut ValidationReport,
) -> Option<&'a [Value]> {
    let value = source.get(label).or_else(|| {
        push(
            report,
            ValidationCode::MissingField,
            path.to_string(),
            "missing field".to_string(),
        );
        None
    })?;
    match value {
        Value::List(items) => Some(items),
        other => {
            push(
                report,
                ValidationCode::TypeMismatch,
                path.to_string(),
                format!("expected List, found {}", other.type_name()),
            );
            None
        }
    }
}

fn optional_list_by_name<'a>(
    source: &'a GffStruct,
    name: &str,
    path: &str,
    report: &mut ValidationReport,
) -> Option<&'a [Value]> {
    let value = source.get_by_name(name)?;
    match value {
        Value::List(items) => Some(items),
        other => {
            push(
                report,
                ValidationCode::TypeMismatch,
                path.to_string(),
                format!("expected List, found {}", other.type_name()),
            );
            None
        }
    }
}

fn require_numeric(source: &GffStruct, label: u32, path: &str, report: &mut ValidationReport) {
    let Some(value) = source.get(label) else {
        push(
            report,
            ValidationCode::MissingField,
            path.to_string(),
            "missing field".to_string(),
        );
        return;
    };
    if !is_numeric(value) {
        push(
            report,
            ValidationCode::InvalidNumericValue,
            path.to_string(),
            format!("expected numeric value, found {}", value.type_name()),
        );
    }
}

fn require_numeric_by_name(
    source: &GffStruct,
    name: &str,
    path: &str,
    report: &mut ValidationReport,
) {
    let Some(value) = source.get_by_name(name) else {
        push(
            report,
            ValidationCode::MissingField,
            path.to_string(),
            "missing field".to_string(),
        );
        return;
    };
    if !is_numeric(value) {
        push(
            report,
            ValidationCode::InvalidNumericValue,
            path.to_string(),
            format!("expected numeric value, found {}", value.type_name()),
        );
    }
}

fn validate_optional_numeric_list(
    source: &GffStruct,
    label: u32,
    path: &str,
    report: &mut ValidationReport,
) {
    let Some(value) = source.get(label) else {
        return;
    };
    validate_numeric_list_value(value, path, report);
}

fn validate_optional_numeric_list_by_name(
    source: &GffStruct,
    name: &str,
    path: &str,
    report: &mut ValidationReport,
) {
    let Some(value) = source.get_by_name(name) else {
        return;
    };
    validate_numeric_list_value(value, path, report);
}

fn validate_numeric_list_value(value: &Value, path: &str, report: &mut ValidationReport) {
    match value {
        Value::List(items) => {
            for (index, value) in items.iter().enumerate() {
                if !is_numeric(value) {
                    push(
                        report,
                        ValidationCode::InvalidNumericValue,
                        format!("{path}[{index}]"),
                        format!("expected numeric value, found {}", value.type_name()),
                    );
                }
            }
        }
        other => push(
            report,
            ValidationCode::TypeMismatch,
            path.to_string(),
            format!("expected List, found {}", other.type_name()),
        ),
    }
}

fn infer_ability_storage(file: &GffFile) -> AbilityStorageKind {
    match &file.header.file_version {
        b"V2.0" => AbilityStorageKind::Da2CombinedList,
        _ => AbilityStorageKind::DaoSplitLists,
    }
}

fn is_numeric(value: &Value) -> bool {
    matches!(
        value,
        Value::UInt8(_)
            | Value::Int8(_)
            | Value::UInt16(_)
            | Value::Int16(_)
            | Value::UInt32(_)
            | Value::Int32(_)
            | Value::UInt64(_)
            | Value::Int64(_)
            | Value::Float32(_)
            | Value::Float64(_)
    )
}

fn push(report: &mut ValidationReport, code: ValidationCode, path: String, message: String) {
    report.findings.push(ValidationFinding {
        severity: ValidationSeverity::Error,
        code,
        path,
        message,
    });
}

#[cfg(test)]
mod tests {
    use super::{ValidationCode, validate_gff};
    use crate::gff4::GffFile;
    use crate::gff4::fields::{SAVEGAME_BACKPACK, SAVEGAME_PARTYLIST};
    use crate::test_support::{da2_save_path, dao_save_path};

    #[test]
    fn validates_dao_save() {
        let gff = GffFile::from_path(dao_save_path()).unwrap();
        let report = validate_gff(&gff);
        assert!(report.is_valid());
        assert!(
            !report
                .findings
                .iter()
                .any(|finding| finding.path.contains("SAVEGAME_ABILITYLIST"))
        );
    }

    #[test]
    fn validates_da2_save() {
        let gff = GffFile::from_path(da2_save_path()).unwrap();
        let report = validate_gff(&gff);
        assert!(report.is_valid());
        assert!(
            !report
                .findings
                .iter()
                .any(|finding| finding.path.contains("SAVEGAME_SKILLLIST")
                    || finding.path.contains("SAVEGAME_TALENTLIST")
                    || finding.path.contains("SAVEGAME_SPELLLIST"))
        );
    }

    #[test]
    fn reports_invalid_property_array_parity() {
        let mut gff = GffFile::from_path(dao_save_path()).unwrap();
        corrupt_first_backpack_property_power_list(&mut gff);

        let report = validate_gff(&gff);

        assert!(!report.is_valid());
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.code == ValidationCode::InvalidPropertyArrayParity)
        );
    }

    #[test]
    fn reports_missing_backpack_field() {
        let mut gff = GffFile::from_path(dao_save_path()).unwrap();
        let party = gff.root_mut().get_struct_mut(SAVEGAME_PARTYLIST).unwrap();
        party
            .fields
            .retain(|field| field.label != SAVEGAME_BACKPACK);

        let report = validate_gff(&gff);

        assert!(!report.is_valid());
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.code == ValidationCode::MissingField
                    && finding.path == "root.SAVEGAME_PARTYLIST.SAVEGAME_BACKPACK")
        );
    }

    fn corrupt_first_backpack_property_power_list(gff: &mut GffFile) {
        let party = gff.root_mut().get_struct_mut(SAVEGAME_PARTYLIST).unwrap();
        let items = party.get_list_mut(SAVEGAME_BACKPACK).unwrap();
        for value in items {
            let Some(item) = value.as_struct_mut() else {
                continue;
            };
            let Some(powers) = item.get_list_mut_by_name("ITEM_PROPERTY_POWERS") else {
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
