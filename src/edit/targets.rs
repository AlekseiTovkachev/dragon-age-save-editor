use super::{CharacterTarget, EditError, InventoryContainer};
use crate::domain::character::Character;
use crate::domain::item::Item;
use crate::domain::save::SaveGame;
use crate::gff4::fields::{SAVEGAME_BACKPACK, SAVEGAME_EQUIPMENT_ITEMS, SAVEGAME_PARTYLIST};
use crate::gff4::{GffFile, GffStruct, Value};

const SAVEGAME_PLAYERCHAR_NAME: &str = "SAVEGAME_PLAYERCHAR";
const SAVEGAME_PLAYERCHAR_CHAR_NAME: &str = "SAVEGAME_PLAYERCHAR_CHAR";
const SAVEGAME_PARTYPOOLMEMBERS_NAME: &str = "SAVEGAME_PARTYPOOLMEMBERS";

pub(super) struct RawSaveTargets<'a> {
    raw: &'a GffFile,
}

impl<'a> RawSaveTargets<'a> {
    pub(super) fn new(raw: &'a GffFile) -> Self {
        Self { raw }
    }

    pub(super) fn character(&self, target: CharacterTarget) -> Result<&'a GffStruct, EditError> {
        raw_character(self.raw, target)
    }
}

pub(super) struct DomainSaveTargets<'a> {
    save: &'a SaveGame,
}

impl<'a> DomainSaveTargets<'a> {
    pub(super) fn new(save: &'a SaveGame) -> Self {
        Self { save }
    }

    pub(super) fn character(&self, target: CharacterTarget) -> Result<&'a Character, EditError> {
        domain_character(self.save, target)
    }
}

pub(super) fn raw_character(
    raw: &GffFile,
    target: CharacterTarget,
) -> Result<&GffStruct, EditError> {
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
            let party = raw.root().get_struct(SAVEGAME_PARTYLIST).ok_or_else(|| {
                EditError::MissingField {
                    path: "root.SAVEGAME_PARTYLIST".to_string(),
                }
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

pub(super) fn raw_character_mut(
    raw: &mut GffFile,
    target: CharacterTarget,
) -> Result<&mut GffStruct, EditError> {
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
            let companions = raw
                .root_mut()
                .get_struct_mut(SAVEGAME_PARTYLIST)
                .ok_or_else(|| EditError::MissingField {
                    path: "root.SAVEGAME_PARTYLIST".to_string(),
                })?
                .get_list_mut_by_name(SAVEGAME_PARTYPOOLMEMBERS_NAME)
                .ok_or_else(|| EditError::MissingField {
                    path: "root.SAVEGAME_PARTYLIST.SAVEGAME_PARTYPOOLMEMBERS".to_string(),
                })?;

            companions
                .iter_mut()
                .filter_map(Value::as_struct_mut)
                .nth(index)
                .ok_or(EditError::InvalidTarget { target })
        }
    }
}

pub(super) fn raw_item(
    raw: &GffFile,
    container: InventoryContainer,
    index: usize,
) -> Result<&GffStruct, EditError> {
    let items = match container {
        InventoryContainer::Backpack => raw
            .root()
            .get_struct(SAVEGAME_PARTYLIST)
            .ok_or_else(|| EditError::MissingField {
                path: "root.SAVEGAME_PARTYLIST".to_string(),
            })?
            .get_list(SAVEGAME_BACKPACK)
            .ok_or_else(|| EditError::MissingField {
                path: "root.SAVEGAME_PARTYLIST.SAVEGAME_BACKPACK".to_string(),
            })?,
        InventoryContainer::Equipment { target } => raw_character(raw, target)?
            .get_list(SAVEGAME_EQUIPMENT_ITEMS)
            .ok_or_else(|| EditError::MissingField {
                path: "character.SAVEGAME_EQUIPMENT_ITEMS".to_string(),
            })?,
    };
    let raw_index =
        nth_struct_index(items, index).ok_or(EditError::InvalidItemIndex { container, index })?;
    items[raw_index]
        .as_struct()
        .ok_or(EditError::InvalidItemIndex { container, index })
}

pub(super) fn raw_item_mut(
    raw: &mut GffFile,
    container: InventoryContainer,
    index: usize,
) -> Result<&mut GffStruct, EditError> {
    let items = match container {
        InventoryContainer::Backpack => raw
            .root_mut()
            .get_struct_mut(SAVEGAME_PARTYLIST)
            .ok_or_else(|| EditError::MissingField {
                path: "root.SAVEGAME_PARTYLIST".to_string(),
            })?
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
    let raw_index =
        nth_struct_index(items, index).ok_or(EditError::InvalidItemIndex { container, index })?;
    items[raw_index]
        .as_struct_mut()
        .ok_or(EditError::InvalidItemIndex { container, index })
}

pub(super) fn domain_character(
    save: &SaveGame,
    target: CharacterTarget,
) -> Result<&Character, EditError> {
    match target {
        CharacterTarget::MainCharacter => Ok(&save.main_character),
        CharacterTarget::Companion(index) => save
            .companions
            .get(index)
            .ok_or(EditError::InvalidTarget { target }),
    }
}

pub(super) fn domain_character_mut(
    save: &mut SaveGame,
    target: CharacterTarget,
) -> Result<&mut Character, EditError> {
    match target {
        CharacterTarget::MainCharacter => Ok(&mut save.main_character),
        CharacterTarget::Companion(index) => save
            .companions
            .get_mut(index)
            .ok_or(EditError::InvalidTarget { target }),
    }
}

pub(super) fn domain_item_mut(
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

pub(super) fn nth_struct_index(values: &[Value], target_index: usize) -> Option<usize> {
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
