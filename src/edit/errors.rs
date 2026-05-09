use crate::domain::ability::AbilityKind;
use crate::domain::gamedata::GameId;
use crate::domain::save::ExtractError;
use std::io;

use super::{AbilityListKind, CharacterTarget, InventoryContainer};

const MAX_ITEM_STACK_SIZE: u32 = 99;

#[derive(Debug, thiserror::Error)]
pub enum EditError {
    #[error("{0}")]
    Extract(#[from] ExtractError),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("invalid character target: {target:?}")]
    InvalidTarget { target: CharacterTarget },
    #[error("missing field at {path}")]
    MissingField { path: String },
    #[error("type mismatch at {path}: expected {expected}, found {actual}")]
    TypeMismatch {
        path: String,
        expected: &'static str,
        actual: &'static str,
    },
    #[error("missing stat row {stat_id} for target {target:?}")]
    MissingStatRow {
        target: CharacterTarget,
        stat_id: u32,
    },
    #[error("cannot insert stat row for {target:?}: no stat row template exists")]
    NoStatRowTemplate { target: CharacterTarget },
    #[error("unsupported numeric value at {path}: {actual}")]
    UnsupportedNumericValue { path: String, actual: &'static str },
    #[error("numeric range error at {path}: {detail}")]
    NumericRange { path: String, detail: String },
    #[error("lookup failed at {path}: {detail}")]
    LookupFailed { path: String, detail: String },
    #[error("unknown ability id {ability_id}")]
    UnknownAbility { ability_id: u32 },
    #[error("ability {ability_id} has invalid kind for {expected:?}: {actual:?}")]
    InvalidAbilityKind {
        ability_id: u32,
        expected: AbilityListKind,
        actual: AbilityKind,
    },
    #[error("editing {list:?} for {target:?} would remove required core ability {required_id}")]
    MissingCoreAbility {
        target: CharacterTarget,
        list: AbilityListKind,
        required_id: u32,
    },
    #[error("invalid item index {index} in {container:?}")]
    InvalidItemIndex {
        container: InventoryContainer,
        index: usize,
    },
    #[error("missing item resref at index {index} in {container:?}")]
    MissingItemResref {
        container: InventoryContainer,
        index: usize,
    },
    #[error("backpack replacement at index {index} must keep resref {expected}, found {actual}")]
    BackpackResrefMismatch {
        index: usize,
        expected: String,
        actual: String,
    },
    #[error("invalid property index {property_index} for item {item_index} in {container:?}")]
    InvalidPropertyIndex {
        container: InventoryContainer,
        item_index: usize,
        property_index: usize,
    },
    #[error(
        "invalid property array parity for item {item_index} in {container:?}: ITEM_PROPERTIES has {ids_len}, ITEM_PROPERTY_POWERS has {powers_len}"
    )]
    InvalidPropertyArrayParity {
        container: InventoryContainer,
        item_index: usize,
        ids_len: usize,
        powers_len: usize,
    },
    #[error("backpack item cloning is not supported for {game:?}")]
    UnsupportedGameForClone { game: Option<GameId> },
    #[error("backpack item {index} is stackable and cannot be cloned")]
    ItemIsStackable { index: usize },
    #[error("backpack item {index} is not stackable")]
    ItemIsNotStackable { index: usize },
    #[error(
        "invalid stack size {stack_size}; stack size must be between 1 and {MAX_ITEM_STACK_SIZE}"
    )]
    InvalidStackSize { stack_size: u32 },
    #[error("plot flag editing is not supported for {game:?}")]
    UnsupportedPlotFlags { game: Option<GameId> },
}
