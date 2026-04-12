use crate::domain::ability::AbilityRef;
use crate::domain::character::Character;
use crate::domain::gamedata::{
    DEFAULT_GAME_DATA_PATH, GameDataLookup, LookupError, SqliteGameData,
};
use crate::domain::item::{
    Item, ItemCategory, ItemProperty, MaterialFamily, MaterialInfo, MaterialProfile, MaterialTarget,
};
use crate::domain::stats::{CoreStats, CoreStatsPatch, PointPools, PointPoolsPatch};
use crate::edit::{
    AbilityListKind, BackpackItemReplacement, CharacterTarget, EditError, InventoryContainer,
    ItemMetadataPatch, PlotBooleanPatch, PlotIntegerPatch, SaveEditor,
};
use crate::gff4::GffFile;
use crate::validate::{
    ValidationCode, ValidationFinding, ValidationReport, ValidationSeverity, validate_gff,
};
use base64::Engine;
use image::DynamicImage;
use image_dds::{ddsfile::Dds, image_from_dds};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Cursor;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SaveDocument {
    source_path: PathBuf,
    raw: GffFile,
    editor: Option<SaveEditor>,
    load_error: Option<CommandError>,
    lookup: Option<SqliteGameData>,
    dirty: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandErrorCode {
    InvalidTarget,
    MissingField,
    TypeMismatch,
    MissingStatRow,
    UnsupportedNumericValue,
    NumericRange,
    LookupFailed,
    UnknownAbility,
    InvalidAbilityKind,
    MissingCoreAbility,
    InvalidItemIndex,
    MissingItemResref,
    BackpackResrefMismatch,
    InvalidPropertyIndex,
    InvalidPropertyArrayParity,
    UnsupportedGameForClone,
    ItemIsStackable,
    ItemIsNotStackable,
    InvalidStackSize,
    UnsupportedPlotFlags,
    InvalidSaveState,
    Io,
    Extract,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandError {
    pub code: CommandErrorCode,
    pub message: String,
}

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

impl SaveDocument {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, CommandError> {
        let source_path = path.as_ref().to_path_buf();
        let raw = GffFile::from_path(&source_path).map_err(|err| CommandError {
            code: CommandErrorCode::Io,
            message: err.to_string(),
        })?;
        Ok(Self::from_gff(source_path, raw)?)
    }

    pub fn from_gff(source_path: impl Into<PathBuf>, raw: GffFile) -> Result<Self, CommandError> {
        let source_path = source_path.into();
        let lookup = resolve_game_data_path()
            .map(|path| SqliteGameData::open(path).map_err(CommandError::from_lookup))
            .transpose()?;
        let editor_result =
            SaveEditor::from_gff_with_lookup(raw.clone(), lookup.as_ref().map(|db| db as _), None);
        let (editor, load_error) = match editor_result {
            Ok(editor) => (Some(editor), None),
            Err(err) => (None, Some(CommandError::from(err))),
        };
        Ok(Self {
            source_path,
            raw,
            editor,
            load_error,
            lookup,
            dirty: false,
        })
    }

    pub fn summary(&self) -> SaveSummaryDto {
        let save = self.editor.as_ref().map(SaveEditor::save);
        SaveSummaryDto {
            source_path: self.source_path.display().to_string(),
            dirty: self.dirty,
            preferred_game: save
                .and_then(|save| save.preferred_game)
                .map(GameIdDto::from),
            money: save.map(|save| save.money).unwrap_or_default(),
            main_character_name: save
                .map(|save| save.main_character.name.clone())
                .unwrap_or_else(|| "<unavailable>".to_string()),
            companion_count: save.map(|save| save.companions.len()).unwrap_or_default(),
            backpack_count: save.map(|save| save.backpack.len()).unwrap_or_default(),
        }
    }

    pub fn execute(&mut self, command: SaveCommand) -> Result<SaveCommandResult, CommandError> {
        match command {
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
                if self.lookup.is_none() {
                    return Err(CommandError {
                        code: CommandErrorCode::LookupFailed,
                        message: "ability editing requires data/gamedata.db".to_string(),
                    });
                }
                let lookup = self.lookup.take();
                let result = self
                    .editor_mut()?
                    .replace_character_abilities(
                        target,
                        AbilityListKind::from(list),
                        &ability_ids,
                        lookup
                            .as_ref()
                            .map(|db| db as &dyn crate::domain::gamedata::GameDataLookup)
                            .unwrap(),
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

    fn editor(&self) -> Result<&SaveEditor, CommandError> {
        self.editor.as_ref().ok_or_else(|| {
            self.load_error.clone().unwrap_or_else(|| CommandError {
                code: CommandErrorCode::InvalidSaveState,
                message: "save is not available for editing".to_string(),
            })
        })
    }

    fn editor_mut(&mut self) -> Result<&mut SaveEditor, CommandError> {
        self.editor.as_mut().ok_or_else(|| {
            self.load_error.clone().unwrap_or_else(|| CommandError {
                code: CommandErrorCode::InvalidSaveState,
                message: "save is not available for editing".to_string(),
            })
        })
    }

    fn character_dto(&self, target: CharacterTarget) -> Result<CharacterDto, CommandError> {
        let save = self.editor()?.save();
        let character = match target {
            CharacterTarget::MainCharacter => &save.main_character,
            CharacterTarget::Companion(index) => {
                save.companions.get(index).ok_or_else(|| CommandError {
                    code: CommandErrorCode::InvalidTarget,
                    message: format!("invalid character target: {target:?}"),
                })?
            }
        };
        Ok(CharacterDto {
            name: character.name.clone(),
            template_resref: character.template_resref.clone(),
            approval: character.approval,
            level: character.level,
            experience: character.experience,
            core_stats: CoreStatsDto::from(character.core_stats),
            point_pools: PointPoolsDto::from(character.point_pools),
            equipment: character
                .equipment
                .iter()
                .cloned()
                .map(|item| self.item_to_dto(item))
                .collect::<Result<Vec<_>, CommandError>>()?,
            skills: character
                .skills
                .iter()
                .cloned()
                .map(AbilityDto::from)
                .collect(),
            talents: character
                .talents
                .iter()
                .cloned()
                .map(AbilityDto::from)
                .collect(),
            spells: character
                .spells
                .iter()
                .cloned()
                .map(AbilityDto::from)
                .collect(),
        })
    }

    fn item_dto(
        &self,
        container: InventoryContainer,
        index: usize,
    ) -> Result<ItemDto, CommandError> {
        let save = self.editor()?.save();
        let item = match container {
            InventoryContainer::Backpack => save.backpack.get(index),
            InventoryContainer::Equipment { target } => match target {
                CharacterTarget::MainCharacter => save.main_character.equipment.get(index),
                CharacterTarget::Companion(companion_index) => save
                    .companions
                    .get(companion_index)
                    .and_then(|character| character.equipment.get(index)),
            },
        }
        .ok_or_else(|| CommandError {
            code: CommandErrorCode::InvalidItemIndex,
            message: format!("invalid item index {index} in {container:?}"),
        })?;
        self.item_to_dto(item.clone())
    }

    fn screenshot_data_url(&self) -> Result<Option<String>, CommandError> {
        let Some(screenshot_path) = self
            .source_path
            .parent()
            .map(|parent| parent.join("screen.dds"))
            .filter(|path| path.exists())
        else {
            return Ok(None);
        };

        let mut file = File::open(&screenshot_path).map_err(|err| CommandError {
            code: CommandErrorCode::Io,
            message: format!(
                "failed to open screenshot {}: {err}",
                screenshot_path.display()
            ),
        })?;
        let dds = Dds::read(&mut file).map_err(|err| CommandError {
            code: CommandErrorCode::Io,
            message: format!(
                "failed to read DDS screenshot {}: {err}",
                screenshot_path.display()
            ),
        })?;
        let image = image_from_dds(&dds, 0).map_err(|err| CommandError {
            code: CommandErrorCode::Io,
            message: format!(
                "failed to decode DDS screenshot {}: {err}",
                screenshot_path.display()
            ),
        })?;

        let mut png_bytes = Cursor::new(Vec::new());
        DynamicImage::ImageRgba8(image)
            .write_to(&mut png_bytes, image::ImageFormat::Png)
            .map_err(|err| CommandError {
                code: CommandErrorCode::Io,
                message: format!(
                    "failed to encode screenshot {} as PNG: {err}",
                    screenshot_path.display()
                ),
            })?;

        let encoded = base64::engine::general_purpose::STANDARD.encode(png_bytes.into_inner());
        Ok(Some(format!("data:image/png;base64,{encoded}")))
    }

    fn preferred_game(&self) -> Option<crate::domain::gamedata::GameId> {
        self.editor
            .as_ref()
            .and_then(|editor| editor.save().preferred_game)
    }

    fn item_to_dto(&self, value: Item) -> Result<ItemDto, CommandError> {
        let material_profile = value.material_profile.clone();
        let material_options = if let (Some(profile), Some(lookup)) =
            (material_profile.as_ref(), self.lookup.as_ref())
        {
            lookup
                .material_options(profile.family, profile.target, self.preferred_game())
                .map_err(CommandError::from_lookup)?
                .into_iter()
                .map(MaterialInfoDto::from)
                .collect()
        } else {
            Vec::new()
        };

        Ok(ItemDto {
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
            material_profile: material_profile.map(MaterialProfileDto::from),
            material_info: value.material_info.map(MaterialInfoDto::from),
            material_options,
            properties: value
                .properties
                .into_iter()
                .map(ItemPropertyDto::from)
                .collect(),
        })
    }
}

impl CommandError {
    fn from_lookup(error: LookupError) -> Self {
        Self {
            code: CommandErrorCode::LookupFailed,
            message: error.to_string(),
        }
    }
}

impl From<EditError> for CommandError {
    fn from(value: EditError) -> Self {
        let code = match value {
            EditError::InvalidTarget { .. } => CommandErrorCode::InvalidTarget,
            EditError::MissingField { .. } => CommandErrorCode::MissingField,
            EditError::TypeMismatch { .. } => CommandErrorCode::TypeMismatch,
            EditError::MissingStatRow { .. } => CommandErrorCode::MissingStatRow,
            EditError::UnsupportedNumericValue { .. } => CommandErrorCode::UnsupportedNumericValue,
            EditError::NumericRange { .. } => CommandErrorCode::NumericRange,
            EditError::LookupFailed { .. } => CommandErrorCode::LookupFailed,
            EditError::UnknownAbility { .. } => CommandErrorCode::UnknownAbility,
            EditError::InvalidAbilityKind { .. } => CommandErrorCode::InvalidAbilityKind,
            EditError::MissingCoreAbility { .. } => CommandErrorCode::MissingCoreAbility,
            EditError::InvalidItemIndex { .. } => CommandErrorCode::InvalidItemIndex,
            EditError::MissingItemResref { .. } => CommandErrorCode::MissingItemResref,
            EditError::BackpackResrefMismatch { .. } => CommandErrorCode::BackpackResrefMismatch,
            EditError::InvalidPropertyIndex { .. } => CommandErrorCode::InvalidPropertyIndex,
            EditError::InvalidPropertyArrayParity { .. } => {
                CommandErrorCode::InvalidPropertyArrayParity
            }
            EditError::UnsupportedGameForClone { .. } => CommandErrorCode::UnsupportedGameForClone,
            EditError::ItemIsStackable { .. } => CommandErrorCode::ItemIsStackable,
            EditError::ItemIsNotStackable { .. } => CommandErrorCode::ItemIsNotStackable,
            EditError::InvalidStackSize { .. } => CommandErrorCode::InvalidStackSize,
            EditError::UnsupportedPlotFlags { .. } => CommandErrorCode::UnsupportedPlotFlags,
            EditError::Io(_) => CommandErrorCode::Io,
            EditError::Extract(_) => CommandErrorCode::Extract,
        };
        Self {
            code,
            message: value.to_string(),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::{
        AbilityListKindDto, BackpackItemReplacementDto, CharacterTargetDto, ItemMetadataPatchDto,
        SaveCommand, SaveCommandResult, SaveDocument,
    };
    use crate::gff4::GffFile;
    use crate::gff4::fields::{SAVEGAME_BACKPACK, SAVEGAME_PARTYLIST};
    use crate::test_support::{da2_save_path, dao_save_path, flat_sample_save_path};
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn serializes_and_deserializes_command_dtos() {
        let command = SaveCommand::ReplaceAbilityList {
            target: CharacterTargetDto::Companion { index: 0 },
            list: AbilityListKindDto::Talents,
            ability_ids: vec![100100, 100200],
        };

        let json = serde_json::to_string(&command).unwrap();
        let decoded: SaveCommand = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded, command);
    }

    #[test]
    fn command_execution_returns_updated_summary() {
        let mut document = SaveDocument::open(dao_save_path()).unwrap();

        let response = document
            .execute(SaveCommand::SetMoney { money: 321321 })
            .unwrap();

        match response {
            SaveCommandResult::Summary { summary } => {
                assert_eq!(summary.money, 321321);
                assert!(summary.dirty);
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[test]
    fn validate_command_reports_healthy_save_without_dirtying_document() {
        let mut document = SaveDocument::open(dao_save_path()).unwrap();

        let response = document.execute(SaveCommand::Validate).unwrap();

        match response {
            SaveCommandResult::Validation { report } => {
                assert!(report.is_valid);
            }
            other => panic!("unexpected response: {other:?}"),
        }
        let summary = document.summary();
        assert!(!summary.dirty);
    }

    #[test]
    fn list_equipment_items_returns_items() {
        let mut document = SaveDocument::open(dao_save_path()).unwrap();

        let response = document
            .execute(SaveCommand::ListEquipmentItems {
                target: CharacterTargetDto::MainCharacter,
            })
            .unwrap();

        match response {
            SaveCommandResult::Items { items } => {
                assert!(!items.is_empty());
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[test]
    fn da2_talent_browsing_is_scoped_to_da2_rows() {
        let mut document = SaveDocument::open(da2_save_path()).unwrap();

        let response = document
            .execute(SaveCommand::ListAvailableAbilities {
                list: AbilityListKindDto::Talents,
            })
            .unwrap();

        match response {
            SaveCommandResult::AvailableAbilities { abilities, .. } => {
                assert!(abilities.iter().any(|ability| ability.id == 101000));
                assert!(!abilities.iter().any(|ability| ability.id == 23));
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[test]
    fn da2_spell_browsing_uses_da2_spell_rows() {
        let mut document = SaveDocument::open(da2_save_path()).unwrap();

        let response = document
            .execute(SaveCommand::ListAvailableAbilities {
                list: AbilityListKindDto::Spells,
            })
            .unwrap();

        match response {
            SaveCommandResult::AvailableAbilities { abilities, .. } => {
                assert!(abilities.iter().any(|ability| ability.id == 301000));
                assert!(!abilities.iter().any(|ability| ability.id == 101000));
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[test]
    fn dao_spell_browsing_includes_mage_specialization_unlocks() {
        let mut document = SaveDocument::open(dao_save_path()).unwrap();

        let response = document
            .execute(SaveCommand::ListAvailableAbilities {
                list: AbilityListKindDto::Spells,
            })
            .unwrap();

        match response {
            SaveCommandResult::AvailableAbilities { abilities, .. } => {
                for ability_id in [4012_u32, 4017, 4018, 4025] {
                    assert!(abilities.iter().any(|ability| ability.id == ability_id));
                }
                assert!(!abilities.iter().any(|ability| ability.id >= 400_000));
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[test]
    fn awakening_spell_browsing_includes_awakening_mage_specializations() {
        let Some(path) = flat_sample_save_path("testingawakening.das") else {
            return;
        };
        let mut document = SaveDocument::open(path).unwrap();

        let response = document
            .execute(SaveCommand::ListAvailableAbilities {
                list: AbilityListKindDto::Spells,
            })
            .unwrap();

        match response {
            SaveCommandResult::AvailableAbilities { abilities, .. } => {
                for ability_id in [401002_u32, 401003] {
                    assert!(abilities.iter().any(|ability| ability.id == ability_id));
                }
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[test]
    fn da2_character_fetch_returns_loaded_talents_and_spells() {
        let mut document = SaveDocument::open(da2_save_path()).unwrap();

        let response = document
            .execute(SaveCommand::GetCharacter {
                target: CharacterTargetDto::MainCharacter,
            })
            .unwrap();

        match response {
            SaveCommandResult::Character { character, .. } => {
                assert!(!character.talents.is_empty());
                assert!(
                    !character.skills.is_empty()
                        || !character.talents.is_empty()
                        || !character.spells.is_empty()
                );
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[test]
    fn da2_plot_flags_include_full_article_30_catalog() {
        let mut document = SaveDocument::open(da2_save_path()).unwrap();

        let response = document
            .execute(SaveCommand::ListAvailablePlotFlags)
            .unwrap();

        match response {
            SaveCommandResult::AvailablePlotFlags { booleans, integers } => {
                assert_eq!(
                    integers.iter().map(|flag| flag.id).collect::<Vec<_>>(),
                    vec![1000, 1001]
                );
                assert!(integers.iter().all(|flag| flag.category == "Hero"));
                assert!(integers.iter().all(|flag| !flag.description.is_empty()));

                assert!(
                    booleans
                        .iter()
                        .any(|flag| flag.id == 2007 && flag.description == "Connor lives")
                );
                assert!(booleans.iter().any(|flag| flag.id == 2108));
                assert_eq!(booleans.len(), 109);
                assert_eq!(
                    booleans.iter().map(|flag| flag.id).collect::<Vec<_>>(),
                    (2000_u16..=2108).collect::<Vec<_>>()
                );
                assert!(
                    booleans
                        .iter()
                        .any(|flag| flag.id == 2072 && flag.category == "Return to Ostagar")
                );
                assert!(booleans.iter().any(|flag| flag.id == 2076
                    && flag.description == "Shale was recruited and survived"));
                assert!(
                    booleans
                        .iter()
                        .any(|flag| flag.id == 2078 && flag.category == "Witch Hunt")
                );
                assert!(
                    booleans
                        .iter()
                        .any(|flag| flag.id == 2103 && flag.category == "Golems of Amgarrak")
                );
                assert!(booleans.iter().all(|flag| !flag.description.is_empty()));
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[test]
    fn save_as_writes_new_file_and_keeps_original_unchanged() {
        let input = dao_save_path();
        let original = fs::read(&input).unwrap();
        let output = test_output_path("document-save-as.das");
        let mut document = SaveDocument::open(&input).unwrap();

        document
            .execute(SaveCommand::PatchItemMetadata {
                container: super::InventoryContainerDto::Backpack,
                index: 0,
                patch: ItemMetadataPatchDto {
                    item_cost: Some(777),
                    material: None,
                    item_level: None,
                },
            })
            .unwrap();
        let response = document
            .execute(SaveCommand::SaveAs {
                output_path: output.display().to_string(),
            })
            .unwrap();

        match response {
            SaveCommandResult::Saved {
                output_path,
                summary,
            } => {
                assert_eq!(PathBuf::from(output_path), output);
                assert!(!summary.dirty);
            }
            other => panic!("unexpected response: {other:?}"),
        }
        assert_eq!(fs::read(&input).unwrap(), original);
        assert!(output.exists());
    }

    #[test]
    fn replace_backpack_item_command_uses_same_resref_policy() {
        let mut document = SaveDocument::open(dao_save_path()).unwrap();

        let error = document
            .execute(SaveCommand::ReplaceBackpackItem {
                index: 0,
                replacement: BackpackItemReplacementDto {
                    resref: "different_item".to_string(),
                    item_cost: Some(1),
                    material: None,
                    item_level: None,
                },
            })
            .unwrap_err();

        assert_eq!(error.code, super::CommandErrorCode::BackpackResrefMismatch);
    }

    #[test]
    fn stack_size_command_returns_updated_item_snapshot() {
        let mut document = SaveDocument::open(dao_save_path()).unwrap();
        let response = document.execute(SaveCommand::ListBackpackItems).unwrap();
        let selected_index = match response {
            SaveCommandResult::Items { items } => items
                .iter()
                .find(|entry| entry.item.stackable)
                .map(|entry| entry.index)
                .expect("expected stackable DAO backpack item"),
            other => panic!("unexpected response: {other:?}"),
        };

        let response = document
            .execute(SaveCommand::SetBackpackItemStackSize {
                index: selected_index,
                stack_size: 2,
            })
            .unwrap();

        match response {
            SaveCommandResult::Item {
                container,
                index,
                item,
            } => {
                assert_eq!(container, super::InventoryContainerDto::Backpack);
                assert_eq!(index, selected_index);
                assert_eq!(item.item_stacksize, Some(2));
                assert!(!item.category.value.is_empty());
                assert!(!item.category.label.is_empty());
            }
            other => panic!("unexpected response: {other:?}"),
        }
        assert!(document.summary().dirty);
    }

    #[test]
    fn crafting_recipe_command_updates_recipe_ids() {
        let mut document = SaveDocument::open(dao_save_path()).unwrap();

        let response = document
            .execute(SaveCommand::ReplaceCraftingRecipeList {
                recipe_ids: vec![2, 11, 2, 20019],
            })
            .unwrap();

        match response {
            SaveCommandResult::CraftingRecipes { recipe_ids } => {
                assert_eq!(recipe_ids, vec![2, 11, 20019]);
            }
            other => panic!("unexpected response: {other:?}"),
        }
        assert!(document.summary().dirty);
    }

    #[test]
    fn da2_available_crafting_recipes_are_named() {
        let mut document = SaveDocument::open(da2_save_path()).unwrap();

        let response = document
            .execute(SaveCommand::ListAvailableCraftingRecipes)
            .unwrap();

        match response {
            SaveCommandResult::AvailableCraftingRecipes { recipes } => {
                assert!(recipes.iter().any(|recipe| {
                    recipe.id == 10000
                        && recipe.name == "Elfroot Potion"
                        && recipe.category == "Potions"
                }));
                assert!(recipes.iter().any(|recipe| {
                    recipe.id == 31007
                        && recipe.name == "Devastation"
                        && recipe.category == "Weapon Runes"
                }));
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[test]
    fn dao_available_crafting_recipes_are_named() {
        let mut document = SaveDocument::open(dao_save_path()).unwrap();

        let response = document
            .execute(SaveCommand::ListAvailableCraftingRecipes)
            .unwrap();

        match response {
            SaveCommandResult::AvailableCraftingRecipes { recipes } => {
                assert!(recipes.iter().any(|recipe| {
                    recipe.id == 2
                        && recipe.name == "Lesser Health Poultice Recipe"
                        && recipe.category == "Herbalism"
                }));
                assert!(recipes.iter().any(|recipe| {
                    recipe.id == 78
                        && recipe.name == "Shock Trap Plans"
                        && recipe.category == "Trap-Making"
                }));
                assert!(recipes.iter().any(|recipe| {
                    recipe.id == 57
                        && recipe.name == "Fleshrot Recipe"
                        && recipe.category == "Poison-Making"
                }));
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[test]
    fn dao_clone_backpack_item_command_returns_new_item_snapshot() {
        let mut document = SaveDocument::open(dao_save_path()).unwrap();
        let response = document.execute(SaveCommand::ListBackpackItems).unwrap();
        let (index, original_resref) = match response {
            SaveCommandResult::Items { items } => items
                .iter()
                .find(|entry| !entry.item.stackable)
                .map(|entry| (entry.index, entry.item.resref.clone()))
                .expect("expected non-stackable DAO backpack item"),
            other => panic!("unexpected response: {other:?}"),
        };

        let response = document
            .execute(SaveCommand::CloneBackpackItem { index })
            .unwrap();

        match response {
            SaveCommandResult::Item {
                container,
                index: cloned_index,
                item,
            } => {
                assert_eq!(container, super::InventoryContainerDto::Backpack);
                assert!(cloned_index > index);
                assert_eq!(item.resref, original_resref);
            }
            other => panic!("unexpected response: {other:?}"),
        }
        assert!(document.summary().dirty);
    }

    #[test]
    fn da2_clone_backpack_item_command_returns_new_item_snapshot() {
        let mut document = SaveDocument::open(da2_save_path()).unwrap();
        let response = document.execute(SaveCommand::ListBackpackItems).unwrap();
        let (index, original_resref) = match response {
            SaveCommandResult::Items { items } => items
                .iter()
                .find(|entry| !entry.item.stackable)
                .map(|entry| (entry.index, entry.item.resref.clone()))
                .expect("expected non-stackable DA2 backpack item"),
            other => panic!("unexpected response: {other:?}"),
        };

        let response = document
            .execute(SaveCommand::CloneBackpackItem { index })
            .unwrap();

        match response {
            SaveCommandResult::Item {
                container,
                index: cloned_index,
                item,
            } => {
                assert_eq!(container, super::InventoryContainerDto::Backpack);
                assert!(cloned_index > index);
                assert_eq!(item.resref, original_resref);
            }
            other => panic!("unexpected response: {other:?}"),
        }
        assert!(document.summary().dirty);
    }

    #[test]
    fn item_property_commands_update_document() {
        let mut document = SaveDocument::open(dao_save_path()).unwrap();
        let response = document.execute(SaveCommand::ListBackpackItems).unwrap();
        let index = match response {
            SaveCommandResult::Items { items } => items
                .iter()
                .position(|item| !item.item.properties.is_empty())
                .unwrap(),
            other => panic!("unexpected response: {other:?}"),
        };

        document
            .execute(SaveCommand::SetItemPropertyPower {
                container: super::InventoryContainerDto::Backpack,
                index,
                property_index: 0,
                power: 18.0,
            })
            .unwrap();
        document
            .execute(SaveCommand::AddItemProperty {
                container: super::InventoryContainerDto::Backpack,
                index,
                property_id: 3011,
                power: 7.5,
            })
            .unwrap();
        let response = document.execute(SaveCommand::ListBackpackItems).unwrap();

        match response {
            SaveCommandResult::Items { items } => {
                assert_eq!(items[index].item.properties[0].power, 18.0);
                assert_eq!(items[index].item.properties.last().unwrap().id, 3011);
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[test]
    fn item_property_id_command_returns_updated_item_snapshot() {
        let mut document = SaveDocument::open(dao_save_path()).unwrap();
        let response = document.execute(SaveCommand::ListBackpackItems).unwrap();
        let index = match response {
            SaveCommandResult::Items { items } => items
                .iter()
                .position(|item| !item.item.properties.is_empty())
                .unwrap(),
            other => panic!("unexpected response: {other:?}"),
        };

        let response = document
            .execute(SaveCommand::SetItemPropertyId {
                container: super::InventoryContainerDto::Backpack,
                index,
                property_index: 0,
                property_id: 3011,
            })
            .unwrap();

        match response {
            SaveCommandResult::Item { item, .. } => {
                assert_eq!(item.properties[0].id, 3011);
            }
            other => panic!("unexpected response: {other:?}"),
        }
        assert!(document.summary().dirty);
    }

    #[test]
    fn validate_command_works_even_when_editor_cannot_be_built() {
        let mut raw = GffFile::from_path(dao_save_path()).unwrap();
        corrupt_first_backpack_property_power_list(&mut raw);
        let mut document = SaveDocument::from_gff("broken.das", raw).unwrap();

        let response = document.execute(SaveCommand::Validate).unwrap();

        match response {
            SaveCommandResult::Validation { report } => {
                assert!(!report.is_valid);
                assert!(report.findings.iter().any(|finding| {
                    finding.code == super::ValidationCodeDto::InvalidPropertyArrayParity
                }));
            }
            other => panic!("unexpected response: {other:?}"),
        }

        let error = document
            .execute(SaveCommand::ListBackpackItems)
            .unwrap_err();
        assert_eq!(error.code, super::CommandErrorCode::Extract);
    }

    #[test]
    fn document_assets_include_decoded_screenshot_when_available() {
        let mut document = SaveDocument::open(dao_save_path()).unwrap();
        let response = document.execute(SaveCommand::GetDocumentAssets).unwrap();

        match response {
            SaveCommandResult::DocumentAssets { assets } => {
                assert!(
                    assets
                        .screenshot_data_url
                        .as_deref()
                        .is_some_and(|value| value.starts_with("data:image/png;base64,"))
                );
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    fn test_output_path(name: &str) -> PathBuf {
        let dir = PathBuf::from("target").join("test-output");
        fs::create_dir_all(&dir).unwrap();
        dir.join(name)
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

struct StaticPlotBooleanFlag {
    id: u16,
    name: &'static str,
    description: &'static str,
    category: &'static str,
}

struct StaticPlotIntegerFlag {
    id: u16,
    name: &'static str,
    description: &'static str,
    category: &'static str,
    options: &'static [StaticPlotIntegerOption],
}

struct StaticPlotIntegerOption {
    value: i32,
    label: &'static str,
}

const HERO_GENDER_OPTIONS: &[StaticPlotIntegerOption] = &[
    StaticPlotIntegerOption {
        value: 1,
        label: "Male",
    },
    StaticPlotIntegerOption {
        value: 2,
        label: "Female",
    },
];

const HERO_RACE_OPTIONS: &[StaticPlotIntegerOption] = &[
    StaticPlotIntegerOption {
        value: 1,
        label: "Dwarf",
    },
    StaticPlotIntegerOption {
        value: 2,
        label: "Elf",
    },
    StaticPlotIntegerOption {
        value: 3,
        label: "Human",
    },
];

const DA2_PLOT_INTEGER_FLAGS: &[StaticPlotIntegerFlag] = &[
    StaticPlotIntegerFlag {
        id: 1000,
        name: "DAO_HERO_GENDER",
        description: "Hero gender",
        category: "Hero",
        options: HERO_GENDER_OPTIONS,
    },
    StaticPlotIntegerFlag {
        id: 1001,
        name: "DAO_HERO_RACE",
        description: "Hero race",
        category: "Hero",
        options: HERO_RACE_OPTIONS,
    },
];

macro_rules! da2_plot_bool {
    ($id:expr, $name:expr, $description:expr, $category:expr) => {
        StaticPlotBooleanFlag {
            id: $id,
            name: $name,
            description: $description,
            category: $category,
        }
    };
}

const DA2_PLOT_BOOLEAN_FLAGS: &[StaticPlotBooleanFlag] = &[
    da2_plot_bool!(2000, "GEN_BACK_CIRCLE", "Origin: Circle mage", "Hero"),
    da2_plot_bool!(
        2001,
        "GEN_BACK_DWARF_COMMONER",
        "Origin: Dwarf commoner",
        "Hero"
    ),
    da2_plot_bool!(2002, "GEN_BACK_DWARF_NOBLE", "Origin: Dwarf noble", "Hero"),
    da2_plot_bool!(2003, "GEN_BACK_ELF_CITY", "Origin: City elf", "Hero"),
    da2_plot_bool!(2004, "GEN_BACK_ELF_DALISH", "Origin: Dalish elf", "Hero"),
    da2_plot_bool!(2005, "GEN_BACK_HUMAN_NOBLE", "Origin: Human noble", "Hero"),
    da2_plot_bool!(
        2006,
        "ARL_REMOVE_DEMON_ISOLDE_KILLS_CONNOR",
        "Isolde kills Connor",
        "Redcliffe"
    ),
    da2_plot_bool!(
        2007,
        "ARL_REMOVE_DEMON_CONNOR_FREED",
        "Connor lives",
        "Redcliffe"
    ),
    da2_plot_bool!(
        2008,
        "ARL_REMOVE_DEMON_ISOLDE_KNOCKED_OUT",
        "Connor dies",
        "Redcliffe"
    ),
    da2_plot_bool!(
        2009,
        "ARL_REMOVE_DEMON_PC_REFUSED_TO_LET_ISOLDE_KILL_CONNOR",
        "Warden refused to let Isolde kill Connor",
        "Redcliffe"
    ),
    da2_plot_bool!(
        2010,
        "ARL_FADE_DEMON_INTIMIDATED",
        "Demon was intimidated in the Fade",
        "Redcliffe"
    ),
    da2_plot_bool!(
        2011,
        "ARL_FADE_OFFER_ACCEPTED_FROM_DEMON",
        "Demon's offer was accepted in the Fade",
        "Redcliffe"
    ),
    da2_plot_bool!(2012, "MAGES_IN_ARMY", "Mages recruited", "Broken Circle"),
    da2_plot_bool!(
        2013,
        "TEMPLARS_IN_ARMY",
        "Templars recruited",
        "Broken Circle"
    ),
    da2_plot_bool!(
        2014,
        "GENITIVI_RETURNS_TO_DENERIM",
        "Andraste's ashes revealed",
        "Urn of Sacred Ashes"
    ),
    da2_plot_bool!(
        2015,
        "NTB_MAIN_ELVES_PROMISED_ALLIANCE",
        "Dalish recruited",
        "Nature of the Beast"
    ),
    da2_plot_bool!(
        2016,
        "NTB_MAIN_WEREWOLVES_PROMISED_ALLIANCE",
        "Werewolves recruited",
        "Nature of the Beast"
    ),
    da2_plot_bool!(
        2017,
        "NTB_MAIN_ZATHRIAN_SACRIFICES_HIMSELF",
        "Zathrian sacrificed himself",
        "Nature of the Beast"
    ),
    da2_plot_bool!(
        2018,
        "ORZ_MAIN___PLOT_04_COMPLETED_KING_IS_BHELEN",
        "Bhelen crowned king",
        "Orzammar"
    ),
    da2_plot_bool!(
        2019,
        "ORZ_MAIN___PLOT_04_COMPLETED_KING_IS_HARROWMONT",
        "Harrowmont crowned king",
        "Orzammar"
    ),
    da2_plot_bool!(
        2020,
        "LANDSMEET_ALISTAIR_ENGAGED_TO_ANORA",
        "Alistair and Anora rule together",
        "Landsmeet"
    ),
    da2_plot_bool!(
        2021,
        "LANDSMEET_ALISTAIR_IS_KING",
        "Alistair rules",
        "Landsmeet"
    ),
    da2_plot_bool!(
        2022,
        "LANDSMEET_ALISTAIR_LEAVES_FOREVER",
        "Alistair is exiled",
        "Landsmeet"
    ),
    da2_plot_bool!(
        2023,
        "LANDSMEET_ALISTAIR_KILLED",
        "Alistair was executed",
        "Landsmeet"
    ),
    da2_plot_bool!(
        2024,
        "LANDSMEET_PLAYER_IS_KING",
        "Warden becomes king",
        "Landsmeet"
    ),
    da2_plot_bool!(
        2025,
        "LANDSMEET_LOGHAIN_KILLED",
        "Loghain dies",
        "Landsmeet"
    ),
    da2_plot_bool!(
        2026,
        "LANDSMEET_ALISTAIR_ENGAGED_TO_PLAYER",
        "Alistair rules with the Warden",
        "Landsmeet"
    ),
    da2_plot_bool!(2027, "LANDSMEET_ANORA_IS_QUEEN", "Anora rules", "Landsmeet"),
    da2_plot_bool!(
        2028,
        "CLIMAX_ARCHDEMON_ALISTAIR_KILLS_ARCHDEMON",
        "Alistair kills the Archdemon",
        "Finale"
    ),
    da2_plot_bool!(
        2029,
        "CLIMAX_ARCHDEMON_LOGHAIN_KILLS_ARCHDEMON",
        "Loghain kills the Archdemon",
        "Finale"
    ),
    da2_plot_bool!(
        2030,
        "CLIMAX_ARCHDEMON_PC_KILLS_ARCHDEMON",
        "Warden kills the Archdemon",
        "Finale"
    ),
    da2_plot_bool!(
        2031,
        "EPI_REWARD_CHANCELLOR",
        "Warden became chancellor",
        "Epilogue"
    ),
    da2_plot_bool!(
        2032,
        "EPI_REWARD_CIRCLE",
        "Circle received the boon",
        "Epilogue"
    ),
    da2_plot_bool!(
        2033,
        "EPI_REWARD_DALISH",
        "Dalish received the land boon",
        "Epilogue"
    ),
    da2_plot_bool!(
        2034,
        "POST_MORRIGAN_RITUAL_WITH_ALISTAIR",
        "Morrigan's ritual was performed with Alistair",
        "Morrigan Ritual"
    ),
    da2_plot_bool!(
        2035,
        "POST_MORRIGAN_RITUAL_WITH_PLAYER",
        "Morrigan's ritual was performed with the Warden",
        "Morrigan Ritual"
    ),
    da2_plot_bool!(
        2036,
        "POST_MORRIGAN_RITUAL_WITH_LOGHAIN",
        "Morrigan's ritual was performed with Loghain",
        "Morrigan Ritual"
    ),
    da2_plot_bool!(2037, "GEN_DOG_RECRUITED", "Dog was recruited", "Companions"),
    da2_plot_bool!(
        2038,
        "GEN_LELIANA_RECRUITED",
        "Leliana was recruited",
        "Companions"
    ),
    da2_plot_bool!(
        2039,
        "GEN_ZEVRAN_RECRUITED",
        "Zevran was recruited",
        "Companions"
    ),
    da2_plot_bool!(
        2040,
        "APP_ALISTAIR_ROMANCE_ACTIVE",
        "Alistair romance active",
        "Romance"
    ),
    da2_plot_bool!(
        2041,
        "APP_ALISTAIR_MAKE_LOVE",
        "Alistair slept with the Warden",
        "Romance"
    ),
    da2_plot_bool!(
        2042,
        "APP_LELIANA_ROMANCE_ACTIVE",
        "Leliana romance active",
        "Romance"
    ),
    da2_plot_bool!(
        2043,
        "APP_LELIANA_MAKE_LOVE",
        "Leliana slept with the Warden",
        "Romance"
    ),
    da2_plot_bool!(
        2044,
        "LELIANA_MAIN_LEAVES_LOTHERING_FOREVER",
        "Leliana was not recruited in Lothering",
        "Companions"
    ),
    da2_plot_bool!(
        2045,
        "LELIANA_MAIN_ATTACKS_PC",
        "Leliana attacked the Warden",
        "Companions"
    ),
    da2_plot_bool!(
        2046,
        "APP_MORRIGAN_ROMANCE_ACTIVE",
        "Morrigan romance active",
        "Romance"
    ),
    da2_plot_bool!(
        2047,
        "APP_MORRIGAN_MAKE_LOVE",
        "Morrigan slept with the Warden",
        "Romance"
    ),
    da2_plot_bool!(
        2048,
        "APP_ZEVRAN_ROMANCE_ACTIVE",
        "Zevran romance active",
        "Romance"
    ),
    da2_plot_bool!(
        2049,
        "APP_ZEVRAN_MAKE_LOVE",
        "Zevran slept with the Warden",
        "Romance"
    ),
    da2_plot_bool!(
        2050,
        "ZEVRAN_MAIN_GOES_HOSTILE",
        "Zevran turned hostile",
        "Companions"
    ),
    da2_plot_bool!(
        2051,
        "ZEVRAN_MAIN_LEAVES_PARTY_AND_GOES",
        "Zevran left the party",
        "Companions"
    ),
    da2_plot_bool!(
        2052,
        "ISABELA_AND_ALISTAIR_THREESOME",
        "Isabela and Alistair had a threesome",
        "Romance"
    ),
    da2_plot_bool!(
        2053,
        "ISABELA_AND_LELIANA_THREESOME",
        "Isabela and Leliana had a threesome",
        "Romance"
    ),
    da2_plot_bool!(
        2054,
        "ISABELA_AND_ZEVRAN_THREESOME",
        "Isabela and Zevran had a threesome",
        "Romance"
    ),
    da2_plot_bool!(
        2055,
        "ISABELA_IN_FOURSOME",
        "Isabela joined a foursome",
        "Romance"
    ),
    da2_plot_bool!(
        2056,
        "ISABELA_SLEPT_WITH",
        "Isabela slept with the Warden",
        "Romance"
    ),
    da2_plot_bool!(
        2057,
        "GXA_BACK_ORLESIAN_WARDEN",
        "Orlesian Warden-Commander",
        "Awakening"
    ),
    da2_plot_bool!(
        2058,
        "VGK_SIEGE___PLOT_09_COMPLETED",
        "Vigil's Keep saved",
        "Awakening"
    ),
    da2_plot_bool!(
        2059,
        "AOA_SIEGE_AMARANTHINE_SAVED",
        "Amaranthine saved",
        "Awakening"
    ),
    da2_plot_bool!(
        2060,
        "VGK_DEFENSES_ROADS_CHOSEN",
        "Protected the trade routes",
        "Awakening"
    ),
    da2_plot_bool!(
        2061,
        "VGK_DEFENSES_FARMS_CHOSEN",
        "Protected the farms",
        "Awakening"
    ),
    da2_plot_bool!(
        2062,
        "VGK_HERREN_COMPLETED_SILVERITE",
        "Vigil's Keep had upgraded silverite walls",
        "Awakening"
    ),
    da2_plot_bool!(
        2063,
        "LTM_MAIN___PLOT_03_KILLED_ARCHITECT",
        "Architect dies",
        "Awakening"
    ),
    da2_plot_bool!(
        2064,
        "GXA_ANDERS_RECRUITED",
        "Anders was recruited",
        "Awakening"
    ),
    da2_plot_bool!(
        2065,
        "GXA_NATHANIEL_RECRUITED",
        "Nathaniel was recruited",
        "Awakening"
    ),
    da2_plot_bool!(
        2066,
        "COD_CHA_ANDERS_DIED_IN_VGK_SIEGE",
        "Anders died during the siege of Vigil's Keep",
        "Awakening"
    ),
    da2_plot_bool!(
        2067,
        "GWB_MAIN_SOPHIA_KILLED",
        "Sophia was killed",
        "Warden's Keep"
    ),
    da2_plot_bool!(
        2068,
        "GWB_MAIN_AVERNUS_KILLED",
        "Avernus was killed",
        "Warden's Keep"
    ),
    da2_plot_bool!(
        2069,
        "GWB_MAIN_COMPLETED",
        "Warden's Keep completed",
        "Warden's Keep"
    ),
    da2_plot_bool!(
        2070,
        "GWB_AVERNUS_DOING_BAD_EXPERIMENTS",
        "Avernus continued unethical experiments",
        "Warden's Keep"
    ),
    da2_plot_bool!(
        2071,
        "GWB_AVERNUS_DOING_GOOD_EXPERIMENTS",
        "Avernus continued ethical research",
        "Warden's Keep"
    ),
    da2_plot_bool!(
        2072,
        "KCC_CAILAN_BURNT",
        "Cailan's corpse was burned",
        "Return to Ostagar"
    ),
    da2_plot_bool!(
        2073,
        "KCC_CAILAN_LEFT",
        "Cailan's corpse was left to the darkspawn",
        "Return to Ostagar"
    ),
    da2_plot_bool!(
        2074,
        "KCC_CAILAN_WOLVES",
        "Cailan's corpse was fed to the wolves",
        "Return to Ostagar"
    ),
    da2_plot_bool!(
        2075,
        "SHL_LEAVES",
        "Shale left the party",
        "The Stone Prisoner"
    ),
    da2_plot_bool!(
        2076,
        "SHL_FOLLOWS",
        "Shale was recruited and survived",
        "The Stone Prisoner"
    ),
    da2_plot_bool!(
        2077,
        "SHL_ATTACKS",
        "Shale attacked the Warden",
        "The Stone Prisoner"
    ),
    da2_plot_bool!(
        2078,
        "STR_MORRIGAN_LEFT",
        "Witch Hunt: let Morrigan go through the Eluvian",
        "Witch Hunt"
    ),
    da2_plot_bool!(
        2079,
        "STR_MORRIGAN_FOLLOWED",
        "Witch Hunt: followed Morrigan through the Eluvian",
        "Witch Hunt"
    ),
    da2_plot_bool!(
        2080,
        "STR_MORRIGAN_STABBED",
        "Witch Hunt: stabbed Morrigan",
        "Witch Hunt"
    ),
    da2_plot_bool!(
        2081,
        "ORZ_ANVIL___PLOT_08_COMPLETED_BRANKA_ALIVE",
        "Support Branka, Anvil saved",
        "Orzammar"
    ),
    da2_plot_bool!(
        2082,
        "ORZ_ANVIL___PLOT_08_COMPLETED_BRANKA_SUICIDES",
        "Support Branka, Anvil destroyed",
        "Orzammar"
    ),
    da2_plot_bool!(
        2083,
        "ORZ_ANVIL___PLOT_08_COMPLETED_CARIDIN",
        "Support Caridin, Anvil destroyed",
        "Orzammar"
    ),
    da2_plot_bool!(
        2084,
        "COD_CHA_NATHANIEL_DIED_IN_VGK_SIEGE",
        "Nathaniel died during the siege of Vigil's Keep",
        "Awakening"
    ),
    da2_plot_bool!(
        2085,
        "GXA_APP_NATHANIEL_FRIENDLY_ELIGIBLE",
        "Nathaniel remained friendly with the Warden-Commander",
        "Awakening"
    ),
    da2_plot_bool!(
        2086,
        "ARL_REMOVE_DEMON_JOWAN_DOES_RITUAL",
        "Jowan performed the ritual to enter the Fade",
        "Redcliffe"
    ),
    da2_plot_bool!(
        2087,
        "ARL_REMOVE_DEMON_CIRCLE_DOES_RITUAL",
        "Circle mages performed the ritual to enter the Fade",
        "Redcliffe"
    ),
    da2_plot_bool!(2088, "ORIGINS_STARTED", "Origins started", "Campaigns"),
    da2_plot_bool!(2089, "ORIGINS_COMPLETED", "Origins completed", "Campaigns"),
    da2_plot_bool!(2090, "AWAKENING_STARTED", "Awakening started", "Campaigns"),
    da2_plot_bool!(
        2091,
        "AWAKENING_COMPLETED",
        "Awakening completed",
        "Awakening"
    ),
    da2_plot_bool!(2092, "STR_STARTED", "Witch Hunt started", "Witch Hunt"),
    da2_plot_bool!(2093, "STR_COMPLETED", "Witch Hunt completed", "Witch Hunt"),
    da2_plot_bool!(
        2094,
        "GWB_STARTED",
        "Warden's Keep started",
        "Warden's Keep"
    ),
    da2_plot_bool!(
        2095,
        "GWB_COMPLETED",
        "Warden's Keep add-in completed",
        "Warden's Keep"
    ),
    da2_plot_bool!(
        2096,
        "LTM_MAIN___PLOT_03_DEAL_WITH_ARCHITECT",
        "Architect lives",
        "Awakening"
    ),
    da2_plot_bool!(
        2097,
        "LANDSMEET_LOGHAIN_LIVES",
        "Loghain lives",
        "Landsmeet"
    ),
    da2_plot_bool!(
        2098,
        "ARL_SIEGE_SIEGE_OVER",
        "Redcliffe siege completed",
        "Redcliffe"
    ),
    da2_plot_bool!(
        2099,
        "ARL_SIEGE_PREP_VILLAGE_ABANDONED",
        "Redcliffe village was abandoned",
        "Redcliffe"
    ),
    da2_plot_bool!(
        2100,
        "MORRIGAN_RITUAL_WITH_ALISTAIR",
        "Morrigan's ritual was performed with Alistair",
        "Morrigan Ritual"
    ),
    da2_plot_bool!(
        2101,
        "MORRIGAN_RITUAL_WITH_LOGHAIN",
        "Morrigan's ritual was performed with Loghain",
        "Morrigan Ritual"
    ),
    da2_plot_bool!(
        2102,
        "MORRIGAN_RITUAL_WITH_PLAYER",
        "Morrigan's ritual was performed with the Warden",
        "Morrigan Ritual"
    ),
    da2_plot_bool!(
        2103,
        "GIB_STARTED",
        "Golems of Amgarrak started",
        "Golems of Amgarrak"
    ),
    da2_plot_bool!(
        2104,
        "RITUAL_PERFORMED",
        "Morrigan's ritual accepted",
        "Morrigan Ritual"
    ),
    da2_plot_bool!(
        2105,
        "ZEVRAN_MAIN_START_AMBUSH_FIGHT_ZEVRAN_ENEMY",
        "Zevran fought the Warden at the ambush",
        "Companions"
    ),
    da2_plot_bool!(
        2106,
        "ZEVRAN_MAIN_KILLED_BEFORE_INTRODUCTION",
        "Zevran died before recruitment",
        "Companions"
    ),
    da2_plot_bool!(
        2107,
        "ZEVRAN_MAIN_LEAVES_AFTER_KISSING_FAREWELL",
        "Zevran left after kissing the Warden goodbye",
        "Companions"
    ),
    da2_plot_bool!(
        2108,
        "ZEVRAN_MAIN_LEAVES_FOR_GOOD",
        "Zevran left for good",
        "Companions"
    ),
];

struct StaticCraftingRecipe {
    id: u32,
    name: &'static str,
    category: &'static str,
}

fn available_crafting_recipes(
    game: Option<crate::domain::gamedata::GameId>,
) -> &'static [StaticCraftingRecipe] {
    match game {
        Some(
            crate::domain::gamedata::GameId::Dao | crate::domain::gamedata::GameId::DaoAwakening,
        ) => DAO_CRAFTING_RECIPES,
        Some(crate::domain::gamedata::GameId::Da2) => DA2_CRAFTING_RECIPES,
        _ => &[],
    }
}

const DAO_CRAFTING_RECIPES: &[StaticCraftingRecipe] = &[
    StaticCraftingRecipe {
        id: 2,
        name: "Lesser Health Poultice Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 7,
        name: "Health Poultice Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 18,
        name: "Greater Health Poultice Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 90,
        name: "Potent Health Poultice Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 11,
        name: "Lesser Lyrium Potion Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 22,
        name: "Lyrium Potion Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 31,
        name: "Greater Lyrium Potion Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 91,
        name: "Potent Lyrium Potion Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 1,
        name: "Mabari Crunch Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 6,
        name: "Double-Baked Mabari Crunch Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 14,
        name: "Lesser Elixir of Grounding Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 34,
        name: "Greater Elixir of Grounding Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 12,
        name: "Lesser Ice Salve Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 32,
        name: "Greater Ice Salve Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 15,
        name: "Lesser Nature Salve Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 35,
        name: "Greater Nature Salve Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 27,
        name: "Lesser Spirit Balm Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 36,
        name: "Greater Spirit Balm Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 13,
        name: "Lesser Warmth Balm Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 33,
        name: "Greater Warmth Balm Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 16,
        name: "Lesser Injury Kit Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 28,
        name: "Injury Kit Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 37,
        name: "Greater Injury Kit Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 89,
        name: "Dwarven Regicide Antidote Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 5,
        name: "Incense of Awareness Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 4,
        name: "Rock Salve Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 17,
        name: "Swift Salve Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 79,
        name: "Acidic Grease Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 68,
        name: "Acidic Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 85,
        name: "Soulrot Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 72,
        name: "Mild Lure Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 82,
        name: "Interesting Lure Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 87,
        name: "Overpowering Lure Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 67,
        name: "Spring Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 80,
        name: "Poisoned Caltrop Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 65,
        name: "Small Caltrop Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 66,
        name: "Small Claw Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 64,
        name: "Small Grease Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 73,
        name: "Small Shrapnel Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 70,
        name: "Large Caltrop Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 74,
        name: "Large Claw Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 69,
        name: "Large Grease Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 83,
        name: "Large Shrapnel Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 71,
        name: "Mild Choking Powder Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 86,
        name: "Choking Powder Cloud Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 81,
        name: "Choking Powder Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 75,
        name: "Mild Sleeping Gas Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 84,
        name: "Sleeping Gas Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 88,
        name: "Sleeping Gas Cloud Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 76,
        name: "Fire Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 77,
        name: "Freeze Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 78,
        name: "Shock Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 41,
        name: "Acid Flask Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 42,
        name: "Acidic Coating Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 39,
        name: "Venom Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 43,
        name: "Concentrated Venom Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 54,
        name: "Adder's Kiss Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 63,
        name: "Quiet Death Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 44,
        name: "Crow Poison Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 56,
        name: "Concentrated Crow Poison Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 40,
        name: "Deathroot Extract Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 45,
        name: "Concentrated Deathroot Extract Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 55,
        name: "Demonic Poison Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 62,
        name: "Concentrated Demonic Poison Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 47,
        name: "Magebane Poison Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 59,
        name: "Concentrated Magebane Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 46,
        name: "Soldier's Bane Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 58,
        name: "Concentrated Soldier's Bane Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 48,
        name: "Fire Bomb Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 51,
        name: "Flaming Coating Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 49,
        name: "Freeze Bomb Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 52,
        name: "Freezing Coating Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 50,
        name: "Shock Bomb Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 53,
        name: "Shock Coating Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 60,
        name: "Soulrot Bomb Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 61,
        name: "Soulrot Coating Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 57,
        name: "Fleshrot Recipe",
        category: "Poison-Making",
    },
];

const DA2_CRAFTING_RECIPES: &[StaticCraftingRecipe] = &[
    StaticCraftingRecipe {
        id: 10000,
        name: "Elfroot Potion",
        category: "Potions",
    },
    StaticCraftingRecipe {
        id: 10001,
        name: "Elixir of Purity",
        category: "Potions",
    },
    StaticCraftingRecipe {
        id: 10003,
        name: "Restoration Potion",
        category: "Potions",
    },
    StaticCraftingRecipe {
        id: 10004,
        name: "Life Ward Potion",
        category: "Potions",
    },
    StaticCraftingRecipe {
        id: 10005,
        name: "Rock Armor Potion",
        category: "Potions",
    },
    StaticCraftingRecipe {
        id: 10006,
        name: "Mighty Offense Potion",
        category: "Potions",
    },
    StaticCraftingRecipe {
        id: 10007,
        name: "Elixir of Heroism",
        category: "Potions",
    },
    StaticCraftingRecipe {
        id: 20000,
        name: "Combustion Grenade",
        category: "Grenades",
    },
    StaticCraftingRecipe {
        id: 20001,
        name: "Tar Bomb",
        category: "Grenades",
    },
    StaticCraftingRecipe {
        id: 20002,
        name: "Mythal's Favor",
        category: "Grenades",
    },
    StaticCraftingRecipe {
        id: 20003,
        name: "Fell Grenade",
        category: "Grenades",
    },
    StaticCraftingRecipe {
        id: 21000,
        name: "Debilitating Poison",
        category: "Poisons",
    },
    StaticCraftingRecipe {
        id: 21001,
        name: "Crow Venom",
        category: "Poisons",
    },
    StaticCraftingRecipe {
        id: 21002,
        name: "Deathroot Toxin",
        category: "Poisons",
    },
    StaticCraftingRecipe {
        id: 21003,
        name: "Arcane Poison",
        category: "Poisons",
    },
    StaticCraftingRecipe {
        id: 21004,
        name: "Fell Poison",
        category: "Poisons",
    },
    StaticCraftingRecipe {
        id: 30000,
        name: "Protection",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 30001,
        name: "Fortune",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 30002,
        name: "Frost Warding",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 30003,
        name: "Fire Warding",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 30004,
        name: "Lightning Warding",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 30005,
        name: "Nature Warding",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 30006,
        name: "Spirit Warding",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 30007,
        name: "Defense",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 30008,
        name: "Valiance",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 31000,
        name: "Impact",
        category: "Weapon Runes",
    },
    StaticCraftingRecipe {
        id: 31001,
        name: "Frost",
        category: "Weapon Runes",
    },
    StaticCraftingRecipe {
        id: 31002,
        name: "Fire",
        category: "Weapon Runes",
    },
    StaticCraftingRecipe {
        id: 31003,
        name: "Lightning",
        category: "Weapon Runes",
    },
    StaticCraftingRecipe {
        id: 31004,
        name: "Nature",
        category: "Weapon Runes",
    },
    StaticCraftingRecipe {
        id: 31005,
        name: "Spirit",
        category: "Weapon Runes",
    },
    StaticCraftingRecipe {
        id: 31006,
        name: "Striking",
        category: "Weapon Runes",
    },
    StaticCraftingRecipe {
        id: 31007,
        name: "Devastation",
        category: "Weapon Runes",
    },
];

fn resolve_game_data_path() -> Option<PathBuf> {
    let mut candidates = vec![
        PathBuf::from(DEFAULT_GAME_DATA_PATH),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(DEFAULT_GAME_DATA_PATH),
    ];

    if let Ok(current_dir) = env::current_dir() {
        candidates.push(current_dir.join(DEFAULT_GAME_DATA_PATH));
    }

    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            candidates.push(exe_dir.join(DEFAULT_GAME_DATA_PATH));
            if let Some(parent) = exe_dir.parent() {
                candidates.push(parent.join(DEFAULT_GAME_DATA_PATH));
                if let Some(grandparent) = parent.parent() {
                    candidates.push(grandparent.join(DEFAULT_GAME_DATA_PATH));
                }
            }
        }
    }

    candidates.into_iter().find(|path| path.exists())
}
