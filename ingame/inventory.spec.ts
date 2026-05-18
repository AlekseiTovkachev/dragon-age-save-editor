import { test, expect } from "@playwright/test";
import { openSave, applyAndSave, verifyInGame, backupSave, restoreSave, ensurePrerequisites, prereq } from "./helpers";

const roseThornPrerequisites = [
  prereq.daoFamilySave(),
  prereq.mainEquipmentItemWithMaterialOption(/Rose.s Thorn/i, 1, "Iron"),
];

const healthPoulticePrerequisites = [
  prereq.daoFamilySave(),
  prereq.stackableBackpackItem(/Health Poultice/i),
];

test.describe("Inventory", () => {
  test.beforeEach(backupSave);
  test.afterEach(restoreSave);

  test("downgrade The Rose's Thorn material to tier 1", async ({ page }, testInfo) => {
    await ensurePrerequisites(testInfo, roseThornPrerequisites);
    await openSave(page);

    // Navigate to Characters > Equipment tab
    await page.getByRole("button", { name: "Characters" }).click();
    await page.locator('nav[aria-label="Character sections"]').getByRole("button", { name: "Equipment" }).click();

    // Click The Rose's Thorn row to expand its inline editor
    await page.getByRole("row", { name: /Rose.s Thorn/i }).click();

    // Material options are formatted "Tier N - <name>"; pick the tier-1 option by its value attr
    const materialSelect = page.getByRole("combobox", { name: /^Material/ });
    const tierOneValue = await materialSelect
      .locator("option")
      .filter({ hasText: /^Tier 1 - / })
      .first()
      .getAttribute("value");
    await materialSelect.selectOption(tierOneValue!);

    await applyAndSave(page);

    const passed = await verifyInGame(page, [
      "The Rose's Thorn now has tier-1 material (much weaker base stats)",
      "(In DAO, item power scales with material tier — OBJECT_MATERIAL)",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });

  test("set Health Poultice stack size to 50", async ({ page }, testInfo) => {
    await ensurePrerequisites(testInfo, healthPoulticePrerequisites);
    await openSave(page);

    // Navigate to the Inventory section
    await page.getByRole("button", { name: "Inventory" }).click();

    // Click the Health Poultice row to expand its inline editor
    await page.getByRole("row", { name: /Health Poultice/i }).first().click();

    // Fill Stack Size to 50
    await page.getByLabel("Stack Size").fill("50");

    await applyAndSave(page);

    const passed = await verifyInGame(page, [
      "Health Poultice in the backpack shows quantity 50",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });
});
