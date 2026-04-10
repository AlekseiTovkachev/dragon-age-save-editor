use dragon_age_save_editor::domain::gamedata::{DEFAULT_GAME_DATA_PATH, SqliteGameData};
use dragon_age_save_editor::domain::save::SaveGame;
use dragon_age_save_editor::gff4::GffFile;
use std::env;
use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    let input = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(default_sample_save);
    let save = GffFile::from_path(&input)?;
    let db_path = DEFAULT_GAME_DATA_PATH;
    let lookup = if Path::new(db_path).exists() {
        Some(
            SqliteGameData::open(db_path)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?,
        )
    } else {
        None
    };
    let domain_save =
        SaveGame::from_gff_with_lookup(&save, lookup.as_ref().map(|db| db as _), None)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;

    println!("file type: {:?}", save.header.file_type);
    println!("input path: {}", input.display());
    println!("root struct index: {}", save.root.struct_index);
    println!("money: {}", domain_save.money);
    println!("main character: {}", domain_save.main_character.name);
    println!("companions: {}", domain_save.companions.len());
    println!("backpack items: {}", domain_save.backpack.len());

    Ok(())
}

fn default_sample_save() -> PathBuf {
    // let nested = Path::new("sample_saves").join("DAO");
    // if let Ok(entries) = std::fs::read_dir(&nested) {
    //     let mut saves = entries
    //         .filter_map(|entry| entry.ok())
    //         .map(|entry| entry.path())
    //         .filter(|path| path.is_dir())
    //         .flat_map(|slot| {
    //             std::fs::read_dir(slot)
    //                 .into_iter()
    //                 .flatten()
    //                 .filter_map(|entry| entry.ok())
    //                 .map(|entry| entry.path())
    //                 .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("das"))
    //                 .collect::<Vec<_>>()
    //         })
    //         .collect::<Vec<_>>();
    //     saves.sort();
    //     if let Some(path) = saves.into_iter().next() {
    //         return path;
    //     }
    // }

    PathBuf::from("sample_saves")
        .join("DA2")
        .join("Slot_18")
        .join("GallowsCourtyard.das")
}
