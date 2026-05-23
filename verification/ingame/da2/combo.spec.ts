import { test, expect } from "@playwright/test";
import {
  openSave,
  applyDrafts,
  saveAs,
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
  prereq.mainHasEquippedItemWithProperties(),
];

test.describe("DA2 Multi-feature combo", () => {
  test.beforeEach(backupSave);
  test.afterEach(restoreSave);

  test("MC stats + equipped item property + party gold + plot flag in one save", async ({ page }, testInfo) => {
    const snapshot = await ensurePrerequisites(testInfo, prerequisites);
    await openSave(page);

    const mainEquipment = snapshot.characterDetails.get("main")?.equipment ?? [];
    const target = mainEquipment.find((item) => item.properties.length > 0)!;
    const targetName = target.name ?? target.resref ?? "(unnamed item)";

    // PHASE 1: Main character stats
    await page.getByRole("button", { name: "Characters" }).click();
    await page.getByLabel("Level").fill("18");
    await page.getByLabel("Constitution").fill("55");
    await applyDrafts(page);

    // PHASE 2: Equipped item property — add a property with a fractional power
    await page.locator('nav[aria-label="Character sections"]').getByRole("button", { name: "Equipment" }).click();
    await page.getByRole("row", { name: new RegExp(targetName.slice(0, Math.min(targetName.length, 16)), "i") }).click();
    const beforeText = (await page.locator(".prop-chips-label").textContent()) ?? "";
    const beforeCount = Number(beforeText.match(/\((\d+)\)/)?.[1] ?? "0");
    await page.getByRole("button", { name: /add property/i }).click();
    const propSelect = page.locator(".prop-add-form select");
    await propSelect.selectOption({ index: 0 });
    const propertyName = (await propSelect.locator("option:checked").textContent())?.trim() ?? "(unknown)";
    await page.locator(".prop-add-form input").fill("20");
    await page.locator(".prop-add-form").getByRole("button", { name: "Add" }).click();
    await applyDrafts(page);

    // PHASE 3: Party gold
    await page.getByRole("button", { name: "Inventory" }).click();
    await page.getByLabel("Party gold").fill("180000");
    await applyDrafts(page);

    // PHASE 4: Plot flag toggle
    await page.getByRole("button", { name: "Plot Flags" }).click();
    await page.getByLabel("Plot flag categories").getByRole("button", { name: "Arl of Redcliffe" }).click();
    await page
      .getByRole("radiogroup", { name: "Andraste's ashes revealed" })
      .getByRole("radio", { name: "No" })
      .click();
    await applyDrafts(page);

    await saveAs(page);

    const savedFlags = readSaveJson<{ booleans: Array<{ id: number; value: boolean }> }>({ command: "list_plot_flags" });
    const bools = new Map(savedFlags.booleans.map((flag) => [flag.id, flag.value]));
    expect(bools.get(2014), "URN_ASHES_REVEALED_TO_WORLD should be false").toBe(false);

    const passed = await verifyInGame(page, [
      "Hawke is level 18 with Constitution 55 (character sheet > Attributes)",
      `"${targetName}" has ${beforeCount + 1} properties, including a new "${propertyName}" bonus (DA2 rescales the raw power, so the number won't be 20)`,
      "Money reflects the edited value (inventory/wallet)",
      "Plot flag 'Andraste's ashes revealed' is set to No",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });
});
