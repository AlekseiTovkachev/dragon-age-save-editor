use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharacterTargetDto {
    MainCharacter,
    Companion { index: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AbilityListKindDto {
    Skills,
    Talents,
    Spells,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameIdDto {
    Dao,
    DaoAwakening,
    Da2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InventoryContainerDto {
    Backpack,
    Equipment { target: CharacterTargetDto },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CoreStatsPatchDto {
    pub strength: Option<u32>,
    pub dexterity: Option<u32>,
    pub willpower: Option<u32>,
    pub magic: Option<u32>,
    pub cunning: Option<u32>,
    pub constitution: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ItemMetadataPatchDto {
    pub item_cost: Option<u32>,
    pub material: Option<u32>,
    pub item_level: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackpackItemReplacementDto {
    pub resref: String,
    pub item_cost: Option<u32>,
    pub material: Option<u32>,
    pub item_level: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveSummaryDto {
    pub source_path: String,
    pub dirty: bool,
    pub preferred_game: Option<GameIdDto>,
    pub money: u32,
    pub main_character_name: String,
    pub companion_count: usize,
    pub backpack_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentAssetsDto {
    pub screenshot_data_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationReportDto {
    pub is_valid: bool,
    pub findings: Vec<ValidationFindingDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFindingDto {
    pub severity: ValidationSeverityDto,
    pub code: ValidationCodeDto,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSeverityDto {
    Error,
    Warning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationCodeDto {
    MissingField,
    TypeMismatch,
    InvalidNumericValue,
    InvalidListEntry,
    InvalidPropertyArrayParity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterSummaryDto {
    pub target: CharacterTargetDto,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterDto {
    pub name: String,
    pub template_resref: Option<String>,
    pub approval: Option<i32>,
    pub level: Option<u32>,
    pub experience: Option<u32>,
    pub core_stats: CoreStatsDto,
    pub point_pools: PointPoolsDto,
    pub equipment: Vec<ItemDto>,
    pub skills: Vec<AbilityDto>,
    pub talents: Vec<AbilityDto>,
    pub spells: Vec<AbilityDto>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreStatsDto {
    pub strength: u32,
    pub dexterity: u32,
    pub willpower: u32,
    pub magic: u32,
    pub cunning: u32,
    pub constitution: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PointPoolsPatchDto {
    pub attribute_points: Option<u32>,
    pub skill_points: Option<u32>,
    pub talent_points: Option<u32>,
    pub specialization_points: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PointPoolsDto {
    pub attribute_points: Option<u32>,
    pub skill_points: Option<u32>,
    pub talent_points: Option<u32>,
    pub specialization_points: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbilityDto {
    pub id: u32,
    pub name: Option<String>,
    pub tree: Option<String>,
    pub ability_type: Option<String>,
    pub core_ids: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexedItemDto {
    pub index: usize,
    pub item: ItemDto,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemDto {
    pub resref: Option<String>,
    pub name: Option<String>,
    pub wiki_url: Option<String>,
    pub category: ItemCategoryDto,
    pub stackable: bool,
    pub object_id: Option<i32>,
    pub equipment_slot: Option<u32>,
    pub item_cost: Option<u32>,
    pub item_stacksize: Option<u32>,
    pub item_level: Option<u8>,
    pub material: Option<u32>,
    pub material_profile: Option<MaterialProfileDto>,
    pub material_info: Option<MaterialInfoDto>,
    pub material_options: Vec<MaterialInfoDto>,
    pub properties: Vec<ItemPropertyDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemCategoryDto {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialInfoDto {
    pub code: u32,
    pub tier: u8,
    pub name: String,
    pub family: MaterialFamilyDto,
    pub target: MaterialTargetDto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterialFamilyDto {
    Metal,
    Wood,
    Leather,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterialTargetDto {
    Armor,
    Weapon,
    Shield,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialProfileDto {
    pub family: MaterialFamilyDto,
    pub target: MaterialTargetDto,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemPropertyDto {
    pub id: u32,
    pub name: Option<String>,
    pub power: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectableItemPropertyDto {
    pub id: u32,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CraftingRecipeDto {
    pub id: u32,
    pub name: String,
    pub category: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlotBooleanValueDto {
    pub id: u16,
    pub value: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlotIntegerValueDto {
    pub id: u16,
    pub value: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlotBooleanFlagDto {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub category: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlotIntegerFlagDto {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub category: String,
    pub options: Vec<PlotIntegerOptionDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlotIntegerOptionDto {
    pub value: i32,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlotFlagWarningDto {
    pub section: String,
    pub message: String,
}
