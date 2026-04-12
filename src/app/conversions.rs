use crate::domain::ability::AbilityRef;
use crate::domain::character::Character;
use crate::domain::item::{
    Item, ItemCategory, ItemProperty, MaterialFamily, MaterialInfo, MaterialProfile, MaterialTarget,
};
use crate::domain::stats::{CoreStats, CoreStatsPatch, PointPools, PointPoolsPatch};
use crate::edit::{
    AbilityListKind, BackpackItemReplacement, CharacterTarget, InventoryContainer,
    ItemMetadataPatch, PlotBooleanPatch, PlotIntegerPatch,
};
use crate::validate::{ValidationCode, ValidationFinding, ValidationReport, ValidationSeverity};

use super::dto::*;

impl From<crate::edit::CharacterSummary> for CharacterSummaryDto {
    fn from(value: crate::edit::CharacterSummary) -> Self {
        Self {
            target: CharacterTargetDto::from(value.target),
            name: value.name,
        }
    }
}

impl From<CharacterTarget> for CharacterTargetDto {
    fn from(value: CharacterTarget) -> Self {
        match value {
            CharacterTarget::MainCharacter => Self::MainCharacter,
            CharacterTarget::Companion(index) => Self::Companion { index },
        }
    }
}

impl From<CharacterTargetDto> for CharacterTarget {
    fn from(value: CharacterTargetDto) -> Self {
        match value {
            CharacterTargetDto::MainCharacter => Self::MainCharacter,
            CharacterTargetDto::Companion { index } => Self::Companion(index),
        }
    }
}

impl From<crate::domain::gamedata::GameId> for GameIdDto {
    fn from(value: crate::domain::gamedata::GameId) -> Self {
        match value {
            crate::domain::gamedata::GameId::Dao => Self::Dao,
            crate::domain::gamedata::GameId::DaoAwakening => Self::DaoAwakening,
            crate::domain::gamedata::GameId::Da2 => Self::Da2,
        }
    }
}

impl From<AbilityListKindDto> for AbilityListKind {
    fn from(value: AbilityListKindDto) -> Self {
        match value {
            AbilityListKindDto::Skills => Self::Skills,
            AbilityListKindDto::Talents => Self::Talents,
            AbilityListKindDto::Spells => Self::Spells,
        }
    }
}

impl From<InventoryContainerDto> for InventoryContainer {
    fn from(value: InventoryContainerDto) -> Self {
        match value {
            InventoryContainerDto::Backpack => Self::Backpack,
            InventoryContainerDto::Equipment { target } => Self::Equipment {
                target: CharacterTarget::from(target),
            },
        }
    }
}

impl From<CoreStatsPatchDto> for CoreStatsPatch {
    fn from(value: CoreStatsPatchDto) -> Self {
        Self {
            strength: value.strength,
            dexterity: value.dexterity,
            willpower: value.willpower,
            magic: value.magic,
            cunning: value.cunning,
            constitution: value.constitution,
        }
    }
}

impl From<PointPoolsPatchDto> for PointPoolsPatch {
    fn from(value: PointPoolsPatchDto) -> Self {
        Self {
            attribute_points: value.attribute_points,
            skill_points: value.skill_points,
            talent_points: value.talent_points,
            specialization_points: value.specialization_points,
        }
    }
}

impl From<ItemMetadataPatchDto> for ItemMetadataPatch {
    fn from(value: ItemMetadataPatchDto) -> Self {
        Self {
            item_cost: value.item_cost,
            material: value.material,
            item_level: value.item_level,
        }
    }
}

impl From<BackpackItemReplacementDto> for BackpackItemReplacement {
    fn from(value: BackpackItemReplacementDto) -> Self {
        Self {
            resref: value.resref,
            item_cost: value.item_cost,
            material: value.material,
            item_level: value.item_level,
        }
    }
}

impl From<PlotBooleanValueDto> for PlotBooleanPatch {
    fn from(value: PlotBooleanValueDto) -> Self {
        Self {
            id: value.id,
            value: value.value,
        }
    }
}

impl From<PlotIntegerValueDto> for PlotIntegerPatch {
    fn from(value: PlotIntegerValueDto) -> Self {
        Self {
            id: value.id,
            value: value.value,
        }
    }
}

impl From<ItemProperty> for ItemPropertyDto {
    fn from(value: ItemProperty) -> Self {
        Self {
            id: value.id,
            name: value.name,
            power: value.power,
        }
    }
}

impl From<Item> for ItemDto {
    fn from(value: Item) -> Self {
        Self {
            resref: value.resref,
            name: value.name,
            wiki_url: value.wiki_url,
            category: ItemCategoryDto::from(value.category),
            stackable: value.stackable,
            object_id: value.object_id,
            equipment_slot: value.equipment_slot,
            item_cost: value.item_cost,
            item_stacksize: value.item_stacksize,
            item_level: value.item_level,
            material: value.material,
            material_profile: value.material_profile.map(MaterialProfileDto::from),
            material_info: value.material_info.map(MaterialInfoDto::from),
            material_options: Vec::new(),
            properties: value
                .properties
                .into_iter()
                .map(ItemPropertyDto::from)
                .collect(),
        }
    }
}

impl From<ItemCategory> for ItemCategoryDto {
    fn from(value: ItemCategory) -> Self {
        Self {
            value: value.as_db_value().to_string(),
            label: value.label().to_string(),
        }
    }
}

impl From<MaterialInfo> for MaterialInfoDto {
    fn from(value: MaterialInfo) -> Self {
        Self {
            code: value.code,
            tier: value.tier,
            name: value.name,
            family: MaterialFamilyDto::from(value.family),
            target: MaterialTargetDto::from(value.target),
        }
    }
}

impl From<MaterialFamily> for MaterialFamilyDto {
    fn from(value: MaterialFamily) -> Self {
        match value {
            MaterialFamily::Metal => Self::Metal,
            MaterialFamily::Wood => Self::Wood,
            MaterialFamily::Leather => Self::Leather,
        }
    }
}

impl From<MaterialTarget> for MaterialTargetDto {
    fn from(value: MaterialTarget) -> Self {
        match value {
            MaterialTarget::Armor => Self::Armor,
            MaterialTarget::Weapon => Self::Weapon,
            MaterialTarget::Shield => Self::Shield,
        }
    }
}

impl From<MaterialProfile> for MaterialProfileDto {
    fn from(value: MaterialProfile) -> Self {
        Self {
            family: MaterialFamilyDto::from(value.family),
            target: MaterialTargetDto::from(value.target),
        }
    }
}

impl From<ValidationReport> for ValidationReportDto {
    fn from(value: ValidationReport) -> Self {
        Self {
            is_valid: value.is_valid(),
            findings: value
                .findings
                .into_iter()
                .map(ValidationFindingDto::from)
                .collect(),
        }
    }
}

impl From<ValidationFinding> for ValidationFindingDto {
    fn from(value: ValidationFinding) -> Self {
        Self {
            severity: ValidationSeverityDto::from(value.severity),
            code: ValidationCodeDto::from(value.code),
            path: value.path,
            message: value.message,
        }
    }
}

impl From<ValidationSeverity> for ValidationSeverityDto {
    fn from(value: ValidationSeverity) -> Self {
        match value {
            ValidationSeverity::Error => Self::Error,
            ValidationSeverity::Warning => Self::Warning,
        }
    }
}

impl From<ValidationCode> for ValidationCodeDto {
    fn from(value: ValidationCode) -> Self {
        match value {
            ValidationCode::MissingField => Self::MissingField,
            ValidationCode::TypeMismatch => Self::TypeMismatch,
            ValidationCode::InvalidNumericValue => Self::InvalidNumericValue,
            ValidationCode::InvalidListEntry => Self::InvalidListEntry,
            ValidationCode::InvalidPropertyArrayParity => Self::InvalidPropertyArrayParity,
        }
    }
}

impl From<CoreStats> for CoreStatsDto {
    fn from(value: CoreStats) -> Self {
        Self {
            strength: value.strength,
            dexterity: value.dexterity,
            willpower: value.willpower,
            magic: value.magic,
            cunning: value.cunning,
            constitution: value.constitution,
        }
    }
}

impl From<PointPools> for PointPoolsDto {
    fn from(value: PointPools) -> Self {
        Self {
            attribute_points: value.attribute_points,
            skill_points: value.skill_points,
            talent_points: value.talent_points,
            specialization_points: value.specialization_points,
        }
    }
}

impl From<AbilityRef> for AbilityDto {
    fn from(value: AbilityRef) -> Self {
        Self {
            id: value.id,
            name: value.name,
            tree: value.tree,
            ability_type: value.ability_type,
            core_ids: value.core_ids,
        }
    }
}

impl From<Character> for CharacterDto {
    fn from(value: Character) -> Self {
        Self {
            name: value.name,
            template_resref: value.template_resref,
            approval: value.approval,
            level: value.level,
            experience: value.experience,
            core_stats: CoreStatsDto::from(value.core_stats),
            point_pools: PointPoolsDto::from(value.point_pools),
            equipment: value.equipment.into_iter().map(ItemDto::from).collect(),
            skills: value.skills.into_iter().map(AbilityDto::from).collect(),
            talents: value.talents.into_iter().map(AbilityDto::from).collect(),
            spells: value.spells.into_iter().map(AbilityDto::from).collect(),
        }
    }
}
