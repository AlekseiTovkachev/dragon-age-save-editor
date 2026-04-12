use crate::domain::ability::AbilityKind;
use crate::domain::gamedata::GameId;
use crate::domain::save::ExtractError;
use std::error::Error;
use std::fmt;
use std::io;

use super::{AbilityListKind, CharacterTarget, InventoryContainer};

const MAX_ITEM_STACK_SIZE: u32 = 99;

#[derive(Debug)]
pub enum EditError {
    Extract(ExtractError),
    Io(io::Error),
    InvalidTarget {
        target: CharacterTarget,
    },
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
    UnsupportedGameForClone {
        game: Option<GameId>,
    },
    ItemIsStackable {
        index: usize,
    },
    ItemIsNotStackable {
        index: usize,
    },
    InvalidStackSize {
        stack_size: u32,
    },
    UnsupportedPlotFlags {
        game: Option<GameId>,
    },
}

impl fmt::Display for EditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditError::Extract(err) => write!(f, "{err}"),
            EditError::Io(err) => write!(f, "{err}"),
            EditError::InvalidTarget { target } => {
                write!(f, "invalid character target: {target:?}")
            }
            EditError::MissingField { path } => write!(f, "missing field at {path}"),
            EditError::TypeMismatch {
                path,
                expected,
                actual,
            } => write!(
                f,
                "type mismatch at {path}: expected {expected}, found {actual}"
            ),
            EditError::MissingStatRow { target, stat_id } => {
                write!(f, "missing stat row {stat_id} for target {target:?}")
            }
            EditError::UnsupportedNumericValue { path, actual } => {
                write!(f, "unsupported numeric value at {path}: {actual}")
            }
            EditError::NumericRange { path, detail } => {
                write!(f, "numeric range error at {path}: {detail}")
            }
            EditError::LookupFailed { path, detail } => {
                write!(f, "lookup failed at {path}: {detail}")
            }
            EditError::UnknownAbility { ability_id } => {
                write!(f, "unknown ability id {ability_id}")
            }
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
            EditError::UnsupportedGameForClone { game } => {
                write!(f, "backpack item cloning is not supported for {game:?}")
            }
            EditError::ItemIsStackable { index } => {
                write!(f, "backpack item {index} is stackable and cannot be cloned")
            }
            EditError::ItemIsNotStackable { index } => {
                write!(f, "backpack item {index} is not stackable")
            }
            EditError::InvalidStackSize { stack_size } => write!(
                f,
                "invalid stack size {stack_size}; stack size must be between 1 and {MAX_ITEM_STACK_SIZE}"
            ),
            EditError::UnsupportedPlotFlags { game } => {
                write!(f, "plot flag editing is not supported for {game:?}")
            }
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
