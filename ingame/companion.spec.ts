import { test, expect } from "@playwright/test";
import { openSave, applyAndSave, verifyInGame, backupSave, restoreSave, ensurePrerequisites, prereq } from "./helpers";

const prerequisites = [
  prereq.daoFamilySave(),
  prereq.companion("Morrigan"),
  prereq.companionApproval("Morrigan"),
  prereq.companionDoesNotHaveAbility("Morrigan", "spells", "Blood Mage"),
  prereq.companionDoesNotHaveAbility("Morrigan", "spells", "Blood Magic"),
];

test.describe("Companion-focused edits", () => {
  test.beforeEach(backupSave);
  test.afterEach(restoreSave);

  test("give Morrigan the Blood Mage specialization + a spell, set approval", async ({ page }, testInfo) => {
    await ensurePrerequisites(testInfo, prerequisites);
    await openSave(page);

    await page.getByRole("button", { name: "Characters" }).click();

    // Select Morrigan in the party rail
    const partyRail = page.locator('aside[aria-label="Party members"]');
    await partyRail.getByRole("button", { name: /Morrigan/i }).click();

    // Overview tab — set approval to 25
    await page.getByLabel("Approval").fill("25");

    // Go to Abilities
    await page.locator('nav[aria-label="Character sections"]').getByRole("button", { name: "Abilities" }).click();

    // Spells → "Mage Specialization" tree → add Blood Mage
    await page.locator('nav[aria-label="Ability lists"]').getByRole("button", { name: /^Spells/ }).click();
    await page.getByRole("option", { name: /Mage Specialization/i }).click();
    await page.getByRole("button", { name: /Add Blood Mage/i }).click();

    // Spells → "Blood Mage Spells" tree → add Blood Magic (the toggle that turns the spec on)
    await page.getByRole("option", { name: /Blood Mage Spells/i }).click();
    await page.getByRole("button", { name: /Add Blood Magic/i }).click();

    await applyAndSave(page);

    const passed = await verifyInGame(page, [
      "Morrigan's approval is 25",
      "Morrigan has Blood Mage as an active specialization (spell list / spec slot)",
      "Morrigan has Blood Magic (toggle spell) available in her spell book",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });
});
