/// CLI tool for in-game testing: applies a JSON SaveCommand to a save file.
///
/// Usage:
///   apply_edit <save_path> <command_json>          -- runs command, does NOT save
///   apply_edit <save_path> <command_json> --save   -- runs command, saves in-place
///   apply_edit <save_path> <command_json_file_path>  -- reads command from file if path exists
use dragon_age_save_editor::app::{SaveCommand, SaveDocument};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: apply_edit <save_path> <command_json_or_@file> [--save]");
        std::process::exit(1);
    }

    let save_path = &args[1];
    let save_in_place = args.iter().any(|a| a == "--save");

    let command_json = if args[2] == "-" {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf
    } else if std::path::Path::new(&args[2]).exists() {
        std::fs::read_to_string(&args[2])?
    } else {
        args[2].clone()
    };

    let command: SaveCommand = serde_json::from_str(&command_json)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, format!("invalid JSON: {e}")))?;

    let mut document = SaveDocument::open(save_path)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.message))?;

    let result = document
        .execute(command)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.message))?;

    if save_in_place {
        document
            .execute(SaveCommand::SaveAs {
                output_path: save_path.clone(),
            })
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.message))?;
    }

    let output = serde_json::to_string_pretty(&result)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    println!("{output}");

    Ok(())
}
