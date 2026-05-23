import { test, expect } from "@playwright/test";
import { openSave, applyAndSave, verifyInGame, backupSave, restoreSave, ensurePrerequisites, prereq } from "../helpers";

const prerequisites = [
  prereq.da2Save(),
  prereq.mainCharacter(),
  prereq.mainHasEquippedItemWithProperties(),
];

test.describe("DA2 Item properties", () => {
  test.beforeEach(backupSave);
  test.afterEach(restoreSave);

  test("add a property to the first equipped item with properties (float-bitcast power roundtrip)", async ({ page }, testInfo) => {
    const snapshot = await ensurePrerequisites(testInfo, prerequisites);
    await openSave(page);

    const mainEquipment = snapshot.characterDetails.get("main")?.equipment ?? [];
    const target = mainEquipment.find((item) => item.properties.length > 0)!;
    const targetName = target.name ?? target.resref ?? "(unnamed item)";

    await page.getByRole("button", { name: "Characters" }).click();
    await page.locator('nav[aria-label="Character sections"]').getByRole("button", { name: "Equipment" }).click();

    // Open the target equipped item by name fragment (first chars are stable enough)
    await page.getByRole("row", { name: new RegExp(targetName.slice(0, Math.min(targetName.length, 16)), "i") }).click();

    const beforeText = (await page.locator(".prop-chips-label").textContent()) ?? "";
    const beforeCount = Number(beforeText.match(/\((\d+)\)/)?.[1] ?? "0");

    await page.getByRole("button", { name: /add property/i }).click();
    const propSelect = page.locator(".prop-add-form select");
    await propSelect.selectOption({ index: 0 });
    const propertyName = (await propSelect.locator("option:checked").textContent())?.trim() ?? "(unknown)";
    await page.locator(".prop-add-form input").fill("25");
    await page.locator(".prop-add-form").getByRole("button", { name: "Add" }).click();

    await expect(page.locator(".prop-chips-label")).toContainText(`(${beforeCount + 1})`);

    await applyAndSave(page);

    const passed = await verifyInGame(page, [
      `"${targetName}" now has ${beforeCount + 1} properties (was ${beforeCount})`,
      `A new "${propertyName}" bonus appears on the item tooltip in-game`,
      "(DA2 rescales the raw power value, so the displayed number will not be 25 — just confirm the property is present)",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });
});
