# Manual QA

Manual QA is not a substitute for `npm run verify` or the `verification/ingame/` suite. Use it only to check the packaged Windows app boundary that automated tests do not cover.

## v0.9 Packaging Smoke

Use a copied save only.

1. Install either:
   - `src-tauri/target/release/bundle/msi/Dragon Age Save Editor_0.9.0_x64_en-US.msi`
   - `src-tauri/target/release/bundle/nsis/Dragon Age Save Editor_0.9.0_x64-setup.exe`
2. Launch Dragon Age Save Editor from the installed app, not from the repo.
3. Open a copied `.das` save.
4. Make one harmless edit, such as money or a stack size.
5. Click **Apply Drafts**.
6. Click **Save As...** and write a new `.das` file.
7. Confirm the output file exists and the original save folder is unchanged.
8. If a game install is available, load the edited copy in game once.

Record the installer used, save family, and result in the release notes or handoff.
