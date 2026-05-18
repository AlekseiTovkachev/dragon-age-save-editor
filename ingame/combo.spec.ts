import { test, expect } from "@playwright/test";
import {
  openSave,
  applyDrafts,
  saveAs,
  verifyInGame,
  backupSave,
  restoreSave,
  ensurePrerequisites,
  prereq,
  readSaveJson,
} from "./helpers";

const prerequisites = [
  prereq.daoFamilySave(),
  prereq.mainCharacter(),
  prereq.companion("Alistair"),
  prereq.companionApproval("Alistair"),
  prereq.companionArmorWithMaterialOption("Alistair", 6, "Silverite"),
];

test.describe("Multi-feature combo", () => {
  test.beforeEach(backupSave);
  test.afterEach(restoreSave);

  test("MC stats + Alistair approval/armor + party gold in one save", async ({ page }, testInfo) => {
    await ensurePrerequisites(testInfo, prerequisites);
    await openSave(page);

    // PHASE 1: Main character stats
    await page.getByRole("button", { name: "Characters" }).click();
    await page.getByLabel("Level").fill("12");
    await page.getByLabel("Constitution").fill("50");
    // Commit MC drafts before switching characters (character drafts are single-buffer).
    await applyDrafts(page);

    // PHASE 2: Alistair's approval
    const partyRail = page.locator('aside[aria-label="Party members"]');
    await partyRail.getByRole("button", { name: /Alistair/i }).click();
    await page.getByLabel("Approval").fill("-10");
    await applyDrafts(page);

    // PHASE 3: Alistair's armor to Silverite (tier 6). Inventory drafts are per-container,
    // so we must apply here before leaving Equipment.
    await page.locator('nav[aria-label="Character sections"]').getByRole("button", { name: "Equipment" }).click();
    const rows = page.locator("tr.inventory-row");
    await rows.first().waitFor();
    const total = await rows.count();
    let changedCount = 0;
    for (let i = 0; i < total; i++) {
      const row = rows.nth(i);
      const categoryText = (await row.locator("td").nth(1).textContent())?.trim() ?? "";
      if (!/^Armor\b/.test(categoryText)) continue;
      await row.click();
      const materialSelect = page.getByRole("combobox", { name: /^Material/ });
      const silveriteValue = await materialSelect
        .locator("option")
        .filter({ hasText: /^Tier 6 - Silverite/i })
        .first()
        .getAttribute("value");
      if (silveriteValue) {
        await materialSelect.selectOption(silveriteValue);
        changedCount++;
      }
    }
    expect(changedCount, "Alistair should have at least one editable armor piece with a Silverite option").toBeGreaterThan(0);
    await applyDrafts(page);

    // PHASE 4: Party gold
    await page.getByRole("button", { name: "Inventory" }).click();
    await page.getByLabel("Party gold").fill("500000");
    await applyDrafts(page);

    // Single Save As at the end — all edits are now committed to working state.
    await saveAs(page);

    const characters = readSaveJson({ command: "list_characters" }).characters;
    const alistair = characters.find((entry: { name: string }) => entry.name === "Alistair");
    const mainCharacter = readSaveJson({ command: "get_character", target: "main_character" }).character;
    const alistairCharacter = readSaveJson({ command: "get_character", target: alistair.target }).character;
    const summary = readSaveJson({ command: "get_summary" }).summary;
    const silveriteArmor = alistairCharacter.equipment.filter(
      (item: { category: { label: string }; material: number }) =>
        item.category.label.startsWith("Armor") && item.material === 3,
    );

    expect(mainCharacter.level).toBe(12);
    expect(mainCharacter.core_stats.constitution).toBe(50);
    expect(alistairCharacter.approval).toBe(-10);
    expect(silveriteArmor).toHaveLength(changedCount);
    expect(summary.money).toBe(500000);

    const passed = await verifyInGame(page, [
      "Main character is level 12",
      "Main character constitution is 50",
      "Alistair's approval is -10",
      `Alistair has ${changedCount} armor piece(s) now Silverite (tier 6)`,
      "Party gold shows 50 gold",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });
});
