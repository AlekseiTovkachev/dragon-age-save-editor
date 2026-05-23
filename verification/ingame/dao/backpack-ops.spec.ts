import { test, expect } from "@playwright/test";
import { openSave, applyAndSave, verifyInGame, backupSave, restoreSave, ensurePrerequisites, prereq } from "../helpers";

const prerequisites = [
  prereq.daoFamilySave(),
  prereq.nonStackableBackpackItem(/Ring of Ages/i),
  prereq.backpackHasItemNotMatching(/Ring of Ages/i),
];

test.describe("Backpack operations", () => {
  test.beforeEach(backupSave);
  test.afterEach(restoreSave);

  test("clone Ring of Ages twice and remove the original", async ({ page }, testInfo) => {
    await ensurePrerequisites(testInfo, prerequisites);
    await openSave(page);

    await page.getByRole("button", { name: "Inventory" }).click();

    const ringRows = () => page.locator("tr.inventory-row").filter({ hasText: /Ring of Ages/i });
    const nonRingRows = () => page.locator("tr.inventory-row").filter({ hasNotText: /Ring of Ages/i });

    // Wait for the inventory to actually finish loading before reading the count.
    await ringRows().first().waitFor();
    const beforeCount = await ringRows().count();

    // Expand the original (first Ring of Ages by insertion order). The inline editor opens beneath it.
    // After Clone, the editor stays on the original source item so a second clone queues from the
    // same source without re-clicking the row (re-clicking would just collapse it).
    await ringRows().first().click();
    await page.getByRole("button", { name: /^Clone$/ }).click();
    await expect(ringRows()).toHaveCount(beforeCount + 1);
    await page.getByRole("button", { name: /^Clone$/ }).click();
    await expect(ringRows()).toHaveCount(beforeCount + 2);

    // To remove the ORIGINAL, we need itemIndex pointing to it (Remove uses itemIndex, not the row).
    // 1) Click a non-Ring row to collapse the current expansion.
    await nonRingRows().first().click();
    // 2) Click the first Ring of Ages again — that re-expands it and selects its index.
    await ringRows().first().click();
    await page.getByRole("button", { name: /^Remove$/ }).click();
    await expect(ringRows()).toHaveCount(beforeCount + 1);

    await applyAndSave(page);

    const passed = await verifyInGame(page, [
      `Backpack contains ${beforeCount + 1} Ring of Ages (was ${beforeCount})`,
      "The original Ring of Ages is gone; only the two clones remain",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });
});
