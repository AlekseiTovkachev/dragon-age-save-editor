use crate::domain::gamedata::{GameDataLookup, SqliteGameData};
use crate::domain::item::Item;
use crate::edit::{CharacterTarget, InventoryContainer, SaveEditor};
use crate::gff4::GffFile;
use base64::Engine;
use image::DynamicImage;
use image_dds::{ddsfile::Dds, image_from_dds};
use std::fs::File;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use super::dto::*;
use super::errors::{CommandError, CommandErrorCode};
use super::path::resolve_game_data_path;

#[derive(Debug)]
pub struct SaveDocument {
    pub(super) source_path: PathBuf,
    pub(super) raw: GffFile,
    pub(super) editor: Option<SaveEditor>,
    pub(super) load_error: Option<CommandError>,
    pub(super) lookup: Option<SqliteGameData>,
    pub(super) dirty: bool,
}

impl SaveDocument {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, CommandError> {
        let source_path = path.as_ref().to_path_buf();
        let raw = GffFile::from_path(&source_path).map_err(|err| CommandError {
            code: CommandErrorCode::Io,
            message: err.to_string(),
        })?;
        Self::from_gff(source_path, raw)
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

    pub(super) fn editor(&self) -> Result<&SaveEditor, CommandError> {
        self.editor.as_ref().ok_or_else(|| {
            self.load_error.clone().unwrap_or_else(|| CommandError {
                code: CommandErrorCode::InvalidSaveState,
                message: "save is not available for editing".to_string(),
            })
        })
    }

    pub(super) fn editor_mut(&mut self) -> Result<&mut SaveEditor, CommandError> {
        self.editor.as_mut().ok_or_else(|| {
            self.load_error.clone().unwrap_or_else(|| CommandError {
                code: CommandErrorCode::InvalidSaveState,
                message: "save is not available for editing".to_string(),
            })
        })
    }

    pub(super) fn character_dto(
        &self,
        target: CharacterTarget,
    ) -> Result<CharacterDto, CommandError> {
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

    pub(super) fn item_dto(
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

    pub(super) fn screenshot_data_url(&self) -> Result<Option<String>, CommandError> {
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

    pub(super) fn preferred_game(&self) -> Option<crate::domain::gamedata::GameId> {
        self.editor
            .as_ref()
            .and_then(|editor| editor.save().preferred_game)
    }

    pub(super) fn item_to_dto(&self, value: Item) -> Result<ItemDto, CommandError> {
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
