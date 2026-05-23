import { test, expect } from "@playwright/test";
import { openSave, applyAndSave, verifyInGame, backupSave, restoreSave, ensurePrerequisites, prereq } from "../helpers";

const prerequisites = [
  prereq.daoFamilySave(),
  prereq.mainEquipmentItem(/Rose.s Thorn/i),
];

test.describe("Item properties", () => {
  test.beforeEach(backupSave);
  test.afterEach(restoreSave);

  test("add a property to The Rose's Thorn (currently has 5)", async ({ page }, testInfo) => {
    await ensurePrerequisites(testInfo, prerequisites);
    await openSave(page);

    // Navigate to Characters > Equipment tab
    await page.getByRole("button", { name: "Characters" }).click();
    await page.locator('nav[aria-label="Character sections"]').getByRole("button", { name: "Equipment" }).click();

    // Open The Rose's Thorn inline editor
    await page.getByRole("row", { name: /Rose.s Thorn/i }).click();

    // Read the current property count from the section header: "Properties (N)"
    const beforeText = (await page.locator(".prop-chips-label").textContent()) ?? "";
    const beforeCount = Number(beforeText.match(/\((\d+)\)/)?.[1] ?? "0");

    // Reveal the property add form, pick a property, set a distinct power, click Add
    await page.getByRole("button", { name: /add property/i }).click();
    const propSelect = page.locator(".prop-add-form select");
    await propSelect.selectOption({ index: 0 });
    const propertyName = (await propSelect.locator("option:checked").textContent())?.trim() ?? "(unknown)";
    await page.locator(".prop-add-form input").fill("9");
    await page.locator(".prop-add-form").getByRole("button", { name: "Add" }).click();

    // Sanity-check that the editor's count went up by one before we save
    await expect(page.locator(".prop-chips-label")).toContainText(`(${beforeCount + 1})`);

    await applyAndSave(page);

    const passed = await verifyInGame(page, [
      `The Rose's Thorn now has ${beforeCount + 1} properties (was ${beforeCount})`,
      `New property "${propertyName}" (+9) appears on the weapon's tooltip in-game`,
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });
});
