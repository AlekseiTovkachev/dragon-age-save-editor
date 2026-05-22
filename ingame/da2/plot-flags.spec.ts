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
];

test.describe("DA2 Plot flags", () => {
  test.beforeEach(backupSave);
  test.afterEach(restoreSave);

  test("set the imported Warden to a female City Elf and ally the werewolves", async ({ page }, testInfo) => {
    await ensurePrerequisites(testInfo, prerequisites);
    await openSave(page);

    await page.getByRole("button", { name: "Plot Flags" }).click();
    await expect(page.getByRole("heading", { name: "Plot Flags" })).toBeVisible();

    // ORDER MATTERS. handleExclusiveSelect runs applyImplications on every
    // exclusive-group click. A prince-consort outcome (flag 2024) carries an
    // implication that forces the Warden back to a human noble. It must be
    // cleared FIRST — otherwise the later identity edits get reverted the
    // moment the next exclusive group (Origin) is touched while 2024 is true.
    await page
      .getByRole("region", { name: "Landsmeet" })
      .getByRole("radiogroup", { name: "Who Rules Ferelden" })
      .getByRole("radio", { name: "Alistair and Anora (co-rulers)" })
      .click();

    // Warden identity — integer flags render as radio groups labelled by description.
    const wardenSection = page.getByRole("region", { name: "Warden" });
    await wardenSection
      .getByRole("radiogroup", { name: "Hero gender" })
      .getByRole("radio", { name: "Female", exact: true })
      .click();
    await wardenSection
      .getByRole("radiogroup", { name: "Hero race" })
      .getByRole("radio", { name: "Elf", exact: true })
      .click();
    // Origin must agree with race — the editor does not yet sync them, so set
    // City Elf explicitly to keep the worldstate internally consistent.
    await wardenSection
      .getByRole("radiogroup", { name: "Origin" })
      .getByRole("radio", { name: "City Elf", exact: true })
      .click();

    // Nature of the Beast — exclusive group; selecting one option clears the
    // contradicting flags (2015 / 2017) automatically.
    await page
      .getByRole("region", { name: "Nature of the Beast" })
      .getByRole("radiogroup", { name: "Sided with" })
      .getByRole("radio", { name: "Werewolves (elves killed)" })
      .click();

    await applyAndSave(page);

    const savedFlags = readSaveJson<{
      booleans: Array<{ id: number; value: boolean }>;
      integers: Array<{ id: number; value: number }>;
    }>({ command: "list_plot_flags" });
    const bools = new Map(savedFlags.booleans.map((flag) => [flag.id, flag.value]));
    const ints = new Map(savedFlags.integers.map((flag) => [flag.id, flag.value]));
    expect(ints.get(1000), "DAO_HERO_GENDER should be Female").toBe(2);
    expect(ints.get(1001), "DAO_HERO_RACE should be Elf").toBe(2);
    expect(bools.get(2003), "GEN_BACK_CITY should be true").toBe(true);
    expect(bools.get(2005), "GEN_BACK_HUMAN_NOBLE should be false").toBe(false);
    expect(bools.get(2016), "NTB_MAIN_WEREWOLVES_PROMISED_ALLIANCE should be true").toBe(true);
    expect(bools.get(2015), "NTB_MAIN_ELVES_PROMISED_ALLIANCE should be false").toBe(false);
    expect(bools.get(2017), "NTB_MAIN_ZATHRIAN_SACRIFICES_HIMSELF should be false").toBe(false);

    const passed = await verifyInGame(page, [
      "Imported worldstate shows the Warden as Female",
      "Imported worldstate shows the Warden as an Elf with a City Elf origin",
      "Landsmeet resolved as Alistair and Anora co-ruling (no Warden marriage)",
      "Nature of the Beast resolved as Werewolves allied (Dalish elves killed)",
    ]);

    expect(passed, "In-game verification failed").toBe(true);
  });
});
