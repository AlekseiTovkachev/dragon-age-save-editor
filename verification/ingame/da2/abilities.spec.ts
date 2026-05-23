import { test, expect } from "@playwright/test";
import {
  openSave,
  applyAndSave,
  verifyInGame,
  backupSave,
  restoreSave,
  ensurePrerequisites,
  prereq,
  readSaveJson,
} from "../helpers";

const prerequisites = [
  prereq.da2Save(),
  prereq.mainCharacter(),
  prereq.mainDoesNotHaveAbility("talents", "Lacerate"),
  prereq.mainDoesNotHaveAbility("talents", "Murder"),
  prereq.companion("Anders"),
  prereq.companionApproval("Anders"),
  prereq.companionDoesNotHaveAbility("Anders", "spells", "Walking Bomb"),
  prereq.companionDoesNotHaveAbility("Anders", "spells", "Death Vortex"),
];

test.describe("DA2 ability edits", () => {
  test.beforeEach(backupSave);
  test.afterEach(restoreSave);

  test("add abilities to Hawke and Anders, then set Anders approval", async ({ page }, testInfo) => {
    const snapshot = await ensurePrerequisites(testInfo, prerequisites);
    const anders = snapshot.characters.find((character) => character.name === "Anders");
    expect(anders, "Anders should be present in the save").toBeTruthy();

    await openSave(page);
    await page.getByRole("button", { name: "Characters" }).click();
    await page.locator('nav[aria-label="Character sections"]').getByRole("button", { name: "Abilities" }).click();

    await page.locator('nav[aria-label="Ability lists"]').getByRole("button", { name: /^Talents/ }).click();
    await page.getByRole("option", { name: /Dual Weapon/ }).click();
    await page.getByRole("button", { name: /Add Lacerate/i }).click();
    await expect(page.getByRole("button", { name: /Remove Lacerate/i })).toBeVisible();
    await page.getByRole("button", { name: /Add Murder/i }).click();
    await expect(page.getByRole("button", { name: /Remove Murder/i })).toBeVisible();
    await applyAndSave(page);

    const hawkeAfterAbilitySave = readSaveJson<{
      character: { talents: Array<{ id: number; name: string | null }> };
    }>({ command: "get_character", target: "main_character" }).character;
    expect(hawkeAfterAbilitySave.talents.map((ability) => ability.id)).toEqual(expect.arrayContaining([201040, 201012]));

    await openSave(page);

    const partyRail = page.locator('aside[aria-label="Party members"]');
    await partyRail.getByRole("button", { name: /Anders/i }).click();
    await page.locator('nav[aria-label="Character sections"]').getByRole("button", { name: "Overview" }).click();
    await page.getByLabel("Approval").fill("-10");
    await page.locator('nav[aria-label="Character sections"]').getByRole("button", { name: "Abilities" }).click();
    await page.locator('nav[aria-label="Ability lists"]').getByRole("button", { name: /^Spells/ }).click();
    await page.getByRole("option", { name: /Spirit Spells/ }).click();
    await page.getByRole("button", { name: /Add Walking Bomb/i }).click();
    await expect(page.getByRole("button", { name: /Remove Walking Bomb/i })).toBeVisible();
    await page.getByRole("button", { name: /Add Death Vortex/i }).click();
    await expect(page.getByRole("button", { name: /Remove Death Vortex/i })).toBeVisible();
    await applyAndSave(page);

    const hawkeAfter = readSaveJson<{
      character: { talents: Array<{ id: number; name: string | null }> };
    }>({ command: "get_character", target: "main_character" }).character;
    const andersAfter = readSaveJson<{
      character: { approval: number | null; spells: Array<{ id: number; name: string | null }> };
    }>({ command: "get_character", target: anders!.target }).character;

    expect(hawkeAfter.talents.map((ability) => ability.id)).toEqual(expect.arrayContaining([201040, 201012]));
    expect(andersAfter.spells.map((ability) => ability.id)).toEqual(expect.arrayContaining([303030, 303041]));
    expect(andersAfter.approval).toBe(-10);

    const passed = await verifyInGame(page, [
      "Hawke has Lacerate and Murder in the Dual Weapon talent tree",
      "Anders has Walking Bomb and Death Vortex in the Spirit spell tree",
      "Anders approval is -10",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });
});
