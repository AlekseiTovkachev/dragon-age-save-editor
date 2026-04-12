use crate::domain::gamedata::LookupError;
use crate::edit::EditError;
use serde::{Deserialize, Serialize};

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

impl CommandError {
    pub(super) fn from_lookup(error: LookupError) -> Self {
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
