import { test, expect } from "@playwright/test";
import { openSave, applyAndSave, verifyInGame, backupSave, restoreSave, ensurePrerequisites, prereq } from "../helpers";

const prerequisites = [
  prereq.da2Save(),
  prereq.mainCharacter(),
];

test.describe("DA2 Stats & Level", () => {
  test.beforeEach(backupSave);
  test.afterEach(restoreSave);

  test("set level, a core stat, and money on Hawke", async ({ page }, testInfo) => {
    await ensurePrerequisites(testInfo, prerequisites);
    await openSave(page);

    await page.getByRole("button", { name: "Characters" }).click();
    await page.getByLabel("Level").fill("15");
    await page.getByLabel("Strength").fill("48");

    await page.getByRole("button", { name: "Inventory" }).click();
    await page.getByLabel("Party gold").fill("250000");

    await applyAndSave(page);

    const passed = await verifyInGame(page, [
      "Hawke is level 15 (character sheet)",
      "Strength is 48 (character sheet > Attributes)",
      "Money reflects the edited value (inventory/wallet — DA2 displays gold from copper)",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });
});
