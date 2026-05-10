import { test, expect } from "@playwright/test";
import { openSave, applyAndSave, verifyInGame, backupSave, restoreSave, DAO_SAVE, setOutputPath } from "./helpers";

test.describe("Stats & Level", () => {
  test.beforeEach(backupSave);
  test.afterEach(restoreSave);

  test("set level, strength, and money", async ({ page }) => {
    await openSave(page);
    await setOutputPath(page, DAO_SAVE);

    // Navigate to the main character (already on Characters > Overview by default)
    await page.getByRole("button", { name: "Characters" }).click();

    // Set level
    await page.getByLabel("Level").fill("10");

    // Set strength — rendered via titleCase("strength") = "Strength"
    await page.getByLabel("Strength").fill("55");

    // Navigate to Inventory for money
    await page.getByRole("button", { name: "Inventory" }).click();
    // Money input has aria-label="Party gold"
    await page.getByLabel("Party gold").fill("999900");

    await applyAndSave(page);

    const passed = await verifyInGame(page, [
      "Character is level 10 (character sheet)",
      "Strength is 55 (character sheet > Attributes)",
      "Money shows 9999 gold (inventory/wallet)",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });
});
