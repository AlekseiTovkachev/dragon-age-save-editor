# Dragon Age Save Editor

Dragon Age Save Editor is a local Windows desktop app for editing save files from Dragon Age: Origins, Awakening-style DAO campaigns, and Dragon Age II.

## Supported Saves

- Dragon Age: Origins vanilla saves.
- Awakening-style DAO saves, including Awakening, Witch Hunt, and Golems of Amgarrak.
- Dragon Age II saves.

## Safety First

Back up your save folder before editing. Keep the original `.das`, `.das.met`, and `screen.dds` files until the edited save has loaded successfully in game.

## Basic Workflow

1. Back up the save folder you want to edit.
2. Open the `.das` file in Dragon Age Save Editor.
3. Make edits in the character, inventory, crafting, or plot flag panels.
4. Click **Apply Drafts** to commit pending changes inside the editor.
5. Click **Save As...** and write the edited `.das` file. You can save to a new file first, or replace the original `.das` if you have a backup.
6. Launch the game and verify the edited save before deleting your backup.

## Known Limitations

- v0.9 is Windows-only and unsigned. Windows SmartScreen may warn that the app is from an unknown publisher.
- The app writes through **Save As...**. It does not autosave over the opened file.
- DAO-family in-game verification is currently tied to specific save prerequisites, so arbitrary Awakening-style saves may need manual checking.
- DA2 plot flag edits expose warnings for contradictory imported-worldstate choices. Resolve warnings before applying drafts.

## Troubleshooting

- **SmartScreen warning:** Choose **More info** and **Run anyway** only if you downloaded the release from this repository.
- **The game does not show my edit:** Confirm the edited `.das` is in the save slot the game is loading, and keep the matching `.das.met` / `screen.dds` files from the same save folder.
- **Apply Drafts is disabled:** Resolve visible validation warnings, especially DA2 plot flag contradictions.
- **An item, ability, or plot value is missing:** File an issue with the game, save type, and what you expected to edit.

## Development

Run the full verification gate before release work:

```bash
npm run verify
```

Other useful commands:

```bash
npm run dev
npm run check
npm run smoke
npm run data:build
npm run data:verify
cargo test
cargo check
```

## Issues

File bugs and feature requests on GitHub:

https://github.com/AlekseiTovkachev/dragon-age-save-editor/issues

## Acknowledgments

- Mephales' pyGFF / Dragon Age save editor (https://www.nexusmods.com/dragonage/mods/4512)
- sapphim's NEW Import Vault Fixes and Editable Vault (https://www.nexusmods.com/dragonage2/mods/4608)
- Dragon Age Wiki / Fandom (https://dragonage.fandom.com/)
- BioWare / Electronic Arts

Without them this project would not exist.

This is an unofficial fan-made tool and does not bundle Dragon Age game assets.

## License

MIT. See `LICENSE`.
