use dragon_age_save_editor::app::{
    CommandError, CommandErrorCode, SaveCommand, SaveCommandResult, SaveDocument, SaveSummaryDto,
};
use std::sync::Mutex;

struct AppState {
    document: Mutex<Option<SaveDocument>>,
}

#[tauri::command]
fn open_document(
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<SaveSummaryDto, CommandError> {
    let document = SaveDocument::open(&path)?;
    let summary = document.summary();
    *state.document.lock().expect("app state lock poisoned") = Some(document);
    Ok(summary)
}

#[tauri::command]
fn has_document(state: tauri::State<'_, AppState>) -> bool {
    state
        .document
        .lock()
        .expect("app state lock poisoned")
        .is_some()
}

#[tauri::command]
fn execute_save_command(
    command: SaveCommand,
    state: tauri::State<'_, AppState>,
) -> Result<SaveCommandResult, CommandError> {
    let mut guard = state.document.lock().expect("app state lock poisoned");
    let document = guard.as_mut().ok_or_else(|| CommandError {
        code: CommandErrorCode::InvalidSaveState,
        message: "no save is currently open".to_string(),
    })?;
    document.execute(command)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            document: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            open_document,
            has_document,
            execute_save_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use dragon_age_save_editor::app::{
        InventoryContainerDto, ItemMetadataPatchDto, SaveCommandResult,
    };
    use std::path::{Path, PathBuf};

    #[test]
    fn opens_dao_save_for_tauri_state_flow() {
        let document = SaveDocument::open(relative_sample_save("DAO")).unwrap();
        let summary = document.summary();
        assert!(!summary.main_character_name.is_empty());
        assert!(summary.backpack_count > 0);
    }

    #[test]
    fn opens_da2_save_for_tauri_state_flow() {
        let document = SaveDocument::open(relative_sample_save("DA2")).unwrap();
        let summary = document.summary();
        assert!(summary.companion_count > 0);
    }

    #[test]
    fn mutation_commands_update_summary() {
        let mut document = SaveDocument::open(relative_sample_save("DAO")).unwrap();
        let response = document
            .execute(SaveCommand::SetMoney { money: 456_789 })
            .unwrap();

        match response {
            SaveCommandResult::Summary { summary } => {
                assert_eq!(summary.money, 456_789);
                assert!(summary.dirty);
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[test]
    fn validate_is_available_for_open_document_flow() {
        let mut document = SaveDocument::open(relative_sample_save("DAO")).unwrap();
        let response = document.execute(SaveCommand::Validate).unwrap();

        match response {
            SaveCommandResult::Validation { report } => assert!(report.is_valid),
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[test]
    fn representative_item_command_runs_through_document_interface() {
        let mut document = SaveDocument::open(relative_sample_save("DAO")).unwrap();
        let response = document.execute(SaveCommand::ListBackpackItems).unwrap();
        let index = match response {
            SaveCommandResult::Items { items } => items
                .iter()
                .position(|item| item.item.item_cost.is_some())
                .unwrap_or(0),
            other => panic!("unexpected response: {other:?}"),
        };

        let response = document
            .execute(SaveCommand::PatchItemMetadata {
                container: InventoryContainerDto::Backpack,
                index,
                patch: ItemMetadataPatchDto {
                    item_cost: Some(999),
                    material: None,
                    item_level: None,
                },
            })
            .unwrap();

        match response {
            SaveCommandResult::Item { item, .. } => {
                assert_eq!(item.item_cost, Some(999));
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    fn relative_sample_save(game_folder: &str) -> PathBuf {
        let base = Path::new("..").join("sample_saves").join(game_folder);
        let slots = std::fs::read_dir(base)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_dir())
            .collect::<Vec<_>>();

        let mut slot_saves = slots
            .into_iter()
            .flat_map(|slot| {
                std::fs::read_dir(slot)
                    .into_iter()
                    .flatten()
                    .filter_map(|entry| entry.ok())
                    .map(|entry| entry.path())
                    .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("das"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        slot_saves.sort();
        slot_saves
            .into_iter()
            .next()
            .expect("sample save should exist")
    }
}
