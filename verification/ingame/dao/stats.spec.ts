import { test, expect } from "@playwright/test";
import { openSave, applyAndSave, verifyInGame, backupSave, restoreSave, ensurePrerequisites, prereq } from "../helpers";

const prerequisites = [
  prereq.daoFamilySave(),
  prereq.mainCharacter(),
];

test.describe("Stats & Level", () => {
  test.beforeEach(backupSave);
  test.afterEach(restoreSave);

  test("set level, strength, and money", async ({ page }, testInfo) => {
    await ensurePrerequisites(testInfo, prerequisites);
    await openSave(page);

    // Navigate to the main character (already on Characters > Overview by default)
    await page.getByRole("button", { name: "Characters" }).click();

    // Set level
    await page.getByLabel("Level").fill("10");

    // Set strength — rendered via titleCase("strength") = "Strength"
    await page.getByLabel("Strength").fill("55");

    // Navigate to Inventory for money (DAO: 1 gold = 10000 copper, 1 silver = 100 copper)
    await page.getByRole("button", { name: "Inventory" }).click();
    await page.getByLabel("Party gold").fill("420000");

    await applyAndSave(page);

    const passed = await verifyInGame(page, [
      "Character is level 10 (character sheet)",
      "Strength is 55 (character sheet > Attributes)",
      "Money shows 42 gold 0 silver (inventory/wallet)",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });
});
