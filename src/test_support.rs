#[cfg(test)]
use std::path::{Path, PathBuf};

#[cfg(test)]
const SAMPLE_SAVES_DIR: &str = "sample_saves";

#[cfg(test)]
pub fn dao_save_path() -> PathBuf {
    first_matching_save("DAO")
}

#[cfg(test)]
pub fn da2_save_path() -> PathBuf {
    first_matching_save("DA2")
}

#[cfg(test)]
pub fn camp_save_path() -> PathBuf {
    let flat = PathBuf::from(SAMPLE_SAVES_DIR).join("Camptesting2.das");
    if flat.exists() {
        flat
    } else {
        dao_save_path()
    }
}

#[cfg(test)]
fn first_matching_save(game_folder: &str) -> PathBuf {
    let base = Path::new(SAMPLE_SAVES_DIR).join(game_folder);
    if base.exists() {
        if let Some(path) = first_slot_save(&base) {
            return path;
        }
    }

    let flat = Path::new(SAMPLE_SAVES_DIR).join(format!("{game_folder}.das"));
    if flat.exists() {
        return flat;
    }

    panic!("no sample save found for {game_folder}");
}

#[cfg(test)]
fn first_slot_save(base: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(base).ok()?;
    let mut slots = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    slots.sort();

    for slot in slots {
        let files = std::fs::read_dir(&slot).ok()?;
        let mut saves = files
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("das"))
            .collect::<Vec<_>>();
        saves.sort();
        if let Some(save) = saves.into_iter().next() {
            return Some(save);
        }
    }

    None
}
