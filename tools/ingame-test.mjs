/**
 * In-game test runner for Dragon Age: Origins saves.
 *
 * Workflow per scenario:
 *   1. Back up the save file
 *   2. Apply specific edits via apply_edit binary (no UI needed)
 *   3. Print a checklist of what to verify in-game
 *   4. Wait for the user to load the save, test, and report
 *   5. Restore the backup before the next scenario (always, even on error)
 *
 * Usage:
 *   node tools/ingame-test.mjs [save_path]
 *
 * Default save: QuickSave_1 for Aedan. Override via first arg or SAVE_PATH env.
 */

import { spawnSync } from "child_process";
import { copyFileSync, existsSync, rmSync } from "fs";
import * as path from "path";
import * as readline from "readline";

// ── Config ────────────────────────────────────────────────────────────────────

const DEFAULT_SAVE =
  "C:/Users/atovk/Documents/BioWare/Dragon Age/Characters/Aedan/Saves/QuickSave_1/savegame.das";
const SAVE_PATH = process.argv[2] ?? process.env.SAVE_PATH ?? DEFAULT_SAVE;
const BINARY = path.resolve("target/debug/apply_edit.exe");
const BACKUP_PATH = SAVE_PATH + ".ingame-test-backup";

// ── I/O helpers ───────────────────────────────────────────────────────────────

const rl = readline.createInterface({ input: process.stdin, output: process.stdout });

let rlClosed = false;
rl.once("close", () => { rlClosed = true; });

function ask(question) {
  if (rlClosed) return Promise.resolve("");
  return new Promise((resolve) => {
    try {
      rl.question(question, (answer) => resolve(answer ?? ""));
    } catch {
      resolve("");
    }
  });
}

// ── Save helpers ──────────────────────────────────────────────────────────────

function applyEdit(commandObj, saveInPlace = false) {
  const args = [SAVE_PATH, JSON.stringify(commandObj)];
  if (saveInPlace) args.push("--save");
  const result = spawnSync(BINARY, args, { encoding: "utf8" });
  if (result.status !== 0) {
    throw new Error(result.stderr?.trim() || `apply_edit exited with code ${result.status}`);
  }
  return JSON.parse(result.stdout);
}

function backup() {
  copyFileSync(SAVE_PATH, BACKUP_PATH);
  console.log(`  [backup]  ${path.basename(SAVE_PATH)} → ${path.basename(BACKUP_PATH)}`);
}

function restore() {
  copyFileSync(BACKUP_PATH, SAVE_PATH);
  console.log(`  [restore] Original save restored.`);
}

function cleanupBackup() {
  if (existsSync(BACKUP_PATH)) {
    rmSync(BACKUP_PATH);
  }
}

// ── Scenario runner ───────────────────────────────────────────────────────────

async function runScenario(title, commandFn, checks) {
  const line = "─".repeat(60);
  console.log(`\n${line}`);
  console.log(`  SCENARIO: ${title}`);
  console.log(`${line}`);

  backup();

  try {
    applyEdit(commandFn(), true);
    console.log("  [edit]    Changes applied successfully.\n");
  } catch (err) {
    console.error(`  [error]   ${err.message}`);
    restore();
    return { title, status: "ERROR" };
  }

  try {
    console.log("Verify in-game:");
    checks.forEach((c, i) => console.log(`  ${i + 1}. ${c}`));
    console.log("\n  Load the save in Dragon Age: Origins now, then return here.");

    const answer = (await ask("\nDid all checks pass? (y/n/skip): ")).trim().toLowerCase();
    const status = answer === "y" ? "PASS" : answer === "skip" ? "SKIPPED" : "FAIL";
    console.log(`  → ${status}`);

    if (status === "FAIL") {
      const notes = await ask("  Notes (optional, press Enter to skip): ");
      if (notes.trim()) console.log(`  Notes: ${notes.trim()}`);
    }

    return { title, status };
  } finally {
    restore();
  }
}

// ── Scenario definitions ──────────────────────────────────────────────────────

function cmdStatsAndLevel() {
  return {
    command: "apply_batch",
    commands: [
      { command: "set_money", money: 999900 },
      { command: "set_level", target: { main_character: {} }, level: 10 },
      { command: "set_experience", target: { main_character: {} }, experience: 62500 },
      {
        command: "patch_core_stats",
        target: { main_character: {} },
        patch: {
          strength: 55,
          dexterity: null,
          willpower: null,
          magic: null,
          cunning: null,
          constitution: null,
        },
      },
    ],
  };
}

function cmdAddSkills(existingSkillIds) {
  // Append the Coercion chain to whatever skills the character already has.
  const coercionChain = [4001, 100011, 100012, 100013, 100014];
  const merged = [...new Set([...existingSkillIds, ...coercionChain])];
  return {
    command: "replace_ability_list",
    target: { main_character: {} },
    list: "skills",
    ability_ids: merged,
  };
}

function cmdRemoveSkills(existingSkillIds) {
  // Remove Expert and Master Coercion to verify removal.
  const trimmed = existingSkillIds.filter((id) => id !== 100013 && id !== 100014);
  return {
    command: "replace_ability_list",
    target: { main_character: {} },
    list: "skills",
    ability_ids: trimmed,
  };
}

function cmdItemLevel() {
  return {
    command: "patch_item_metadata",
    container: { equipment: { target: { main_character: {} } } },
    index: 0,
    patch: { item_level: 5, item_cost: null, material: null },
  };
}

function cmdStackSize(index) {
  return { command: "set_backpack_item_stack_size", index, stack_size: 50 };
}

// ── Main ──────────────────────────────────────────────────────────────────────

async function main() {
  // Guards
  if (!existsSync(SAVE_PATH)) {
    console.error(`Save not found: ${SAVE_PATH}`);
    console.error("Pass the save path as first argument or set SAVE_PATH env.");
    process.exit(1);
  }
  if (!existsSync(BINARY)) {
    console.error(`apply_edit binary not found: ${BINARY}`);
    console.error("Run: cargo build --bin apply_edit");
    process.exit(1);
  }

  // Clean up any leftover backup from a previous aborted run.
  cleanupBackup();

  // Read current state to build additive edits.
  console.log("Reading current save state...");
  const charResult = applyEdit({ command: "get_character", target: { main_character: {} } });
  const char = charResult.character;
  // Exclude unknown IDs (name === null means not in gamedata.db; replace_ability_list rejects them).
  const existingSkillIds = char.skills.filter((s) => s.name !== null).map((s) => s.id);
  const afterAddSkillIds = [...new Set([...existingSkillIds, 4001, 100011, 100012, 100013, 100014])];

  const bpResult = applyEdit({ command: "list_backpack_items" });
  const stackableIdx = bpResult.items.findIndex((it) => it.item.stackable);
  const stackableItem = stackableIdx >= 0 ? bpResult.items[stackableIdx].item : null;

  const mainHandName = char.equipment[0]?.name ?? "(no main-hand item)";

  console.log(`  Character : ${char.name}, level ${char.level}, str ${char.core_stats.strength}`);
  console.log(`  Skills    : ${char.skills.map((s) => s.name ?? `#${s.id}`).join(", ")}`);
  console.log(`  Main hand : ${mainHandName}`);
  console.log(`  Stackable : ${stackableItem?.name ?? "none"} (index ${stackableIdx})`);

  console.log("\n=== Dragon Age: Origins In-Game Test Runner ===");
  console.log(`Save : ${SAVE_PATH}`);
  console.log("Each scenario modifies the save, you load it in game, verify it,");
  console.log("then the save is restored to its original state before the next scenario.");

  const results = [];

  // ── Scenario 1: Stats & Level ────────────────────────────────────────────
  results.push(
    await runScenario("Stats & Level", cmdStatsAndLevel, [
      "Character is level 10 (character sheet)",
      "Strength is 55 (character sheet > Attributes)",
      "Money shows 9999 gold (inventory / wallet)",
      "Experience bar reflects ~62,500 XP",
    ]),
  );

  // ── Scenario 2: Add Skills ───────────────────────────────────────────────
  results.push(
    await runScenario("Add Coercion Skills", () => cmdAddSkills(existingSkillIds), [
      "Skills tab shows Coercion",
      "Skills tab shows Improved Coercion",
      "Skills tab shows Expert Coercion",
      "Skills tab shows Master Coercion",
      "All previously existing skills are still there",
    ]),
  );

  // ── Scenario 3: Remove Skills ─────────────────────────────────────────────
  results.push(
    await runScenario("Remove Skills (Expert + Master Coercion)", () => cmdRemoveSkills(afterAddSkillIds), [
      "Skills tab shows Coercion and Improved Coercion",
      "Expert Coercion is ABSENT",
      "Master Coercion is ABSENT",
      "All other skills are unchanged",
    ]),
  );

  // ── Scenario 4: Equipped Item Level ──────────────────────────────────────
  results.push(
    await runScenario(`Equipped Item Level (${mainHandName})`, cmdItemLevel, [
      `Main-hand weapon (${mainHandName}) has visibly better stats`,
      "Item tooltip reflects item level 5 (DAO: item_level affects damage/armour tier)",
      "(Note: item level = OBJECT_RANK; higher rank = better base stats on weapon)",
    ]),
  );

  // ── Scenario 5: Backpack Stack Size ──────────────────────────────────────
  if (stackableItem) {
    results.push(
      await runScenario(`Backpack Stack Size (${stackableItem.name})`, () => cmdStackSize(stackableIdx), [
        `${stackableItem.name} shows quantity 50 in inventory (was ${stackableItem.item_stacksize})`,
        "Item is otherwise identical",
      ]),
    );
  }

  // ── Summary ───────────────────────────────────────────────────────────────
  console.log("\n" + "═".repeat(60));
  console.log("  RESULTS");
  console.log("═".repeat(60));
  for (const r of results.filter(Boolean)) {
    const icon = { PASS: "✓", FAIL: "✗", SKIPPED: "○", ERROR: "!" }[r.status] ?? "?";
    console.log(`  ${icon}  ${r.title}: ${r.status}`);
  }
  console.log("═".repeat(60));

  cleanupBackup();
  rl.close();
}

main().catch((err) => {
  console.error("\n[fatal]", err.message ?? err);
  // Best-effort restore on unexpected crash.
  if (existsSync(BACKUP_PATH)) {
    copyFileSync(BACKUP_PATH, SAVE_PATH);
    console.error("[fatal] Restored backup due to crash.");
    cleanupBackup();
  }
  rl.close();
  process.exit(1);
});
