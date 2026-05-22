import { test, expect } from "@playwright/test";
import { openSave, applyAndSave, verifyInGame, backupSave, restoreSave, ensurePrerequisites, prereq } from "../helpers";

const prerequisites = [
  prereq.daoFamilySave(),
  prereq.mainCharacter(),
  prereq.mainHasAbility("skills", "Master Coercion"),
  prereq.mainDoesNotHaveAbility("talents", "Dual-Weapon Mastery"),
];

test.describe("Abilities", () => {
  test.beforeEach(backupSave);
  test.afterEach(restoreSave);

  test("remove Master Coercion and add Dual-Weapon Mastery", async ({ page }, testInfo) => {
    await ensurePrerequisites(testInfo, prerequisites);
    await openSave(page);

    await page.getByRole("button", { name: "Characters" }).click();
    await page.locator('nav[aria-label="Character sections"]').getByRole("button", { name: "Abilities" }).click();

    // Kind tab: Skills
    await page.locator('nav[aria-label="Ability lists"]').getByRole("button", { name: /^Skills/ }).click();
    // Select "Skill Tree" tree, then remove Master Coercion
    await page.getByRole("option", { name: /Skill Tree/ }).click();
    await page.getByRole("button", { name: /Remove Master Coercion/i }).click();

    // Kind tab: Talents
    await page.locator('nav[aria-label="Ability lists"]').getByRole("button", { name: /^Talents/ }).click();
    // Select "Dual Weapons" tree, then add Dual-Weapon Mastery
    await page.getByRole("option", { name: /Dual Weapon Talents/ }).click();
    await page.getByRole("button", { name: /Add Dual.Weapon Mastery/i }).click();

    await applyAndSave(page);

    const passed = await verifyInGame(page, [
      "Master Coercion is gone from the skills list",
      "Dual-Weapon Mastery appears in the talents list",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });
});
