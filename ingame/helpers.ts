import { copyFileSync, existsSync, rmSync } from "fs";
import type { Page } from "@playwright/test";

export const DAO_SAVE =
  process.env.DAO_SAVE ??
  "C:/Users/atovk/Documents/BioWare/Dragon Age/Characters/Aedan/Saves/QuickSave_1/savegame.das";

export function backupSave() {
  copyFileSync(DAO_SAVE, DAO_SAVE + ".ingame-backup");
}

export function restoreSave() {
  if (existsSync(DAO_SAVE + ".ingame-backup")) {
    copyFileSync(DAO_SAVE + ".ingame-backup", DAO_SAVE);
    rmSync(DAO_SAVE + ".ingame-backup");
  }
}

// Open the save in the editor via the UI
export async function openSave(page: Page) {
  await page.addInitScript((savePath) => {
    localStorage.setItem("ingameTestSave", savePath);
  }, DAO_SAVE);
  await page.goto("/");
  await page.getByRole("button", { name: /open save/i }).click();
  await page.waitForSelector('[aria-label="Save editor navigation"]');
}

// Set the output path for Save As
export async function setOutputPath(page: Page, outputPath: string) {
  await page.evaluate((p) => localStorage.setItem("ingameTestSaveOutput", p), outputPath);
}

// Apply drafts then save as to the same file (overwrite)
export async function applyAndSave(page: Page) {
  await page.getByRole("button", { name: /apply drafts/i }).click();
  await page.waitForSelector("text=Unsaved changes");
  await page.getByRole("button", { name: /save as/i }).click();
  await page.waitForSelector("text=Saved copy ready");
}

// Inject an in-page verification panel and wait for pass/fail click
// Returns true if passed.
export async function verifyInGame(page: Page, checklist: string[]): Promise<boolean> {
  await page.evaluate((items: string[]) => {
    const el = document.createElement("div");
    el.id = "__verify_panel";
    el.style.cssText =
      "position:fixed;inset:0;z-index:9999;background:rgba(0,0,0,0.88);display:flex;align-items:center;justify-content:center;";
    el.innerHTML = `
      <div style="background:#14100c;border:1px solid #c79a48;border-radius:6px;padding:2rem;max-width:520px;color:#f3ead6;font-family:monospace;font-size:0.85rem;">
        <div style="font-size:0.6rem;letter-spacing:0.18em;color:#888;text-transform:uppercase;margin-bottom:0.75rem">In-Game Verification</div>
        <ol style="margin:0 0 1.25rem;padding:0 0 0 1.25rem;line-height:1.9">
          ${items.map((i) => `<li>${i}</li>`).join("")}
        </ol>
        <p style="color:#666;font-size:0.72rem;margin-bottom:1rem">Load the save in Dragon Age: Origins, verify the items above, then return here.</p>
        <div style="display:flex;gap:0.75rem">
          <button id="__vpass" style="flex:1;padding:0.65rem;background:rgba(42,100,42,0.3);border:1px solid rgba(42,150,42,0.6);color:#5fdf5f;cursor:pointer;font-family:monospace;font-size:0.8rem;border-radius:4px">&#10003;  PASS &mdash; all checks good</button>
          <button id="__vfail" style="flex:1;padding:0.65rem;background:rgba(100,28,28,0.3);border:1px solid rgba(150,42,42,0.6);color:#df5f5f;cursor:pointer;font-family:monospace;font-size:0.8rem;border-radius:4px">&#10007;  FAIL &mdash; something wrong</button>
        </div>
      </div>`;
    document.body.appendChild(el);
    document.getElementById("__vpass")!.onclick = () => {
      (window as unknown as Record<string, unknown>)["__verify_result"] = "pass";
    };
    document.getElementById("__vfail")!.onclick = () => {
      (window as unknown as Record<string, unknown>)["__verify_result"] = "fail";
    };
  }, checklist);

  // No timeout — wait indefinitely for the user
  await page.waitForFunction(() => (window as unknown as Record<string, unknown>)["__verify_result"] !== undefined, { timeout: 0 });
  const result = await page.evaluate(() => (window as unknown as Record<string, unknown>)["__verify_result"] as string);
  await page.evaluate(() => document.getElementById("__verify_panel")?.remove());
  return result === "pass";
}
