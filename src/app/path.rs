use crate::domain::gamedata::DEFAULT_GAME_DATA_PATH;
use std::env;
use std::path::PathBuf;

pub(super) fn resolve_game_data_path() -> Option<PathBuf> {
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
