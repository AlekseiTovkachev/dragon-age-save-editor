import { test, expect } from "@playwright/test";
import { openSave, applyAndSave, verifyInGame, backupSave, restoreSave, DAO_SAVE, setOutputPath } from "./helpers";

test.describe("Abilities", () => {
  test.beforeEach(backupSave);
  test.afterEach(restoreSave);

  test("add Coercion skill chain", async ({ page }) => {
    await openSave(page);
    await setOutputPath(page, DAO_SAVE);

    await page.getByRole("button", { name: "Characters" }).click();
    // Click the "Abilities" character subtab
    await page.getByRole("button", { name: "Abilities" }).click();

    // Select the Skills kind tab (button text is "Skills <count>")
    await page.getByRole("button", { name: /^skills/i }).click();

    // Coercion belongs to the "Skill Tree" group in the TreeList
    await page.getByRole("option", { name: "Skill Tree" }).click();

    // The RankLadder now shows all Skill Tree abilities — add Coercion specifically
    await page.getByRole("button", { name: "Add Coercion" }).click();

    await applyAndSave(page);

    const passed = await verifyInGame(page, [
      "Skills tab shows Coercion",
      "Coercion appears in the character's skill list in-game",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });

  test("remove a skill", async ({ page }) => {
    await openSave(page);
    await setOutputPath(page, DAO_SAVE);

    await page.getByRole("button", { name: "Characters" }).click();
    await page.getByRole("button", { name: "Abilities" }).click();
    await page.getByRole("button", { name: /^skills/i }).click();

    // Remove the first skill that has a remove button (not locked)
    // Remove buttons have aria-label="Remove {ability.name}" (locked ones say "Required {name}")
    const removeButtons = page.getByRole("button", { name: /^remove /i });
    const count = await removeButtons.count();
    if (count > 0) {
      await removeButtons.first().click();
    }

    await applyAndSave(page);

    const passed = await verifyInGame(page, [
      "The removed skill is absent from the character's skill list in-game",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });
});
