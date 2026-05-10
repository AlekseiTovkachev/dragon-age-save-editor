import { test, expect } from "@playwright/test";
import { openSave, applyAndSave, verifyInGame, backupSave, restoreSave, DAO_SAVE, setOutputPath } from "./helpers";

test.describe("Inventory", () => {
  test.beforeEach(backupSave);
  test.afterEach(restoreSave);

  test("set item level on main-hand weapon", async ({ page }) => {
    await openSave(page);
    await setOutputPath(page, DAO_SAVE);

    // Navigate to Characters > Equipment tab
    await page.getByRole("button", { name: "Characters" }).click();
    await page.getByRole("button", { name: "Equipment" }).click();

    // Click the first item row to expand its inline editor
    // InventoryRow renders as <tr class="inventory-row"> with a button inside
    await page.locator("tr.inventory-row").first().click();

    // Fill Item Level in the inline editor
    await page.getByLabel("Item Level").fill("5");

    await applyAndSave(page);

    const passed = await verifyInGame(page, [
      "Main-hand weapon has item level 5 (better base stats than original)",
      "(In DAO, item level = OBJECT_RANK — affects damage/armour tier)",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });

  test("increase stack size of a backpack item", async ({ page }) => {
    await openSave(page);
    await setOutputPath(page, DAO_SAVE);

    // Navigate to the Inventory section
    await page.getByRole("button", { name: "Inventory" }).click();

    // Click through item rows until we find one with a Stack Size field
    const items = page.locator("tr.inventory-row");
    const count = await items.count();
    let found = false;
    for (let i = 0; i < count; i++) {
      await items.nth(i).click();
      const stackField = page.getByLabel("Stack Size");
      if (await stackField.isVisible()) {
        await stackField.fill("50");
        found = true;
        break;
      }
    }

    // Only save if we found a stackable item
    if (found) {
      await applyAndSave(page);
    }

    const passed = await verifyInGame(page, [
      found
        ? "A stackable item in the backpack shows quantity 50"
        : "No stackable items found in backpack — test skipped",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });
});
